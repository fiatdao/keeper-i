//! Auctions Module
//!
//! This module is responsible for triggering and managing Auction's
use crate::{
    bindings::{AuctionIdType, Limes, NoLossCollateralAuction, PositionIdType},
    escalator::GeometricGasPrice,
    watcher::{Auction, DiscountRateMap, Position, SpotMap, VaultMap},
    Result,
};

use decimal_rs::Decimal;
use ethers::{prelude::*, utils::parse_units, utils::WEI_IN_ETHER};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Div, Mul},
    sync::Arc,
    time::{Instant, SystemTime},
};
use tracing::{debug, error, info, instrument, trace, warn};

// Map of cached positions
pub type CachedPositionMap = HashMap<PositionIdType, CachedPosition>;

#[derive(Clone)]
pub struct Liquidator<M> {
    limes: Limes<M>,
    collateral_auction: NoLossCollateralAuction<M>,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    _multicall: Multicall<M>,

    // extra gas to use for txs, as percent of estimated gas cost
    gas_boost: u16,

    pending_liquidations: HashMap<PositionIdType, PendingTransaction>,
    pending_auctions: HashMap<PositionIdType, PendingTransaction>,
    gas_escalator: GeometricGasPrice,
    bump_gas_delay: u64,

    pub cached_positions: HashMap<PositionIdType, CachedPosition>,

    instance_name: String,
}

/// Tx / Hash/ Submitted at time
type PendingTransaction = (TypedTransaction, TxHash, Instant);

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// Cached Position
pub struct CachedPosition {
    position_id: PositionIdType,
    vault: H160,
    token_id: U256,
    owner: H160,
    collateral_value: U256,
    debt: U256,
    health_factor: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TxType {
    Auction,
    Liquidation,
}

impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TxType::Auction => "auction",
            TxType::Liquidation => "liquidation",
        };
        write!(f, "{}", string)
    }
}

impl<M: Middleware> Liquidator<M> {
    /// Constructor
    pub async fn new(
        limes: Address,
        collateral_auction: Address,
        multicall: Option<Address>,
        gas_boost: u16,
        client: Arc<M>,
        gas_escalator: GeometricGasPrice,
        bump_gas_delay: u64,
        instance_name: String,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("could not initialize multicall");

        Self {
            limes: Limes::new(limes, client.clone()),
            collateral_auction: NoLossCollateralAuction::new(collateral_auction, client.clone()),
            _multicall: multicall,
            gas_boost: gas_boost,
            pending_liquidations: HashMap::new(),
            pending_auctions: HashMap::new(),
            cached_positions: HashMap::new(),
            gas_escalator,
            bump_gas_delay,
            instance_name,
        }
    }

    /// Checks if any transactions which have been submitted are mined, removes
    /// them if they were successful, otherwise bumps their gas price
    #[instrument(skip(self), fields(self.instance_name))]
    pub async fn remove_or_bump(&mut self) -> Result<(), M> {
        let now = Instant::now();

        let client = self.limes.client();
        // Check all the pending liquidations
        Liquidator::remove_or_bump_inner(
            now,
            client,
            &self.gas_escalator,
            &mut self.pending_liquidations,
            "liquidations",
            self.instance_name.as_ref(),
            self.bump_gas_delay,
        )
        .await?;
        Liquidator::remove_or_bump_inner(
            now,
            client,
            &self.gas_escalator,
            &mut self.pending_auctions,
            "auctions",
            self.instance_name.as_ref(),
            self.bump_gas_delay,
        )
        .await?;

        Ok(())
    }

    async fn remove_or_bump_inner<K: Clone + Eq + ::std::hash::Hash + std::fmt::Debug>(
        now: Instant,
        client: &M,
        gas_escalator: &GeometricGasPrice,
        pending_txs: &mut HashMap<K, PendingTransaction>,
        tx_type: &str,
        instance_name: &str,
        bump_gas_delay: u64,
    ) -> Result<(), M> {
        for (addr, (pending_tx_wrapper, tx_hash, instant)) in pending_txs.clone().into_iter() {
            let pending_tx = match pending_tx_wrapper {
                TypedTransaction::Eip1559(x) => x,
                _ => panic!("Non-Eip1559 transactions are not supported yet"),
            };

            // get the receipt and check inclusion, or bump its gas price
            let receipt = client
                .get_transaction_receipt(tx_hash)
                .await
                .map_err(ContractError::MiddlewareError)?;
            if let Some(receipt) = receipt {
                pending_txs.remove(&addr);
                let status = if receipt.status == Some(1.into()) {
                    "success"
                } else {
                    "fail"
                };
                info!(tx_hash = ?tx_hash, gas_used = %receipt.gas_used.unwrap_or_default(), user = ?addr,
                  status = status, tx_type, instance_name, "Confirmed");
            } else {
                let time_since = now.duration_since(instant).as_secs();
                if time_since > bump_gas_delay {
                    info!(tx_hash = ?tx_hash, "Bumping gas");
                    // Get the new gas price based on how much time passed since the
                    // tx was last broadcast
                    let new_gas_price = gas_escalator.get_gas_price(
                        pending_tx
                            .max_fee_per_gas
                            .expect("max_fee_per_gas price must be set"),
                        now.duration_since(instant).as_secs(),
                    );

                    let replacement_tx = pending_txs
                        .get_mut(&addr)
                        .expect("tx will always be found since we're iterating over the map");

                    // bump the gas price
                    if let TypedTransaction::Eip1559(x) = &mut replacement_tx.0 {
                        // it should be reversed:
                        // - max_fee_per_gas has to be constant
                        // - max_priority_fee_per_gas needs to be bumped
                        x.max_fee_per_gas = Some(new_gas_price);
                        x.max_priority_fee_per_gas = Some(U256::from(2000000000));
                    // 2 gwei
                    } else {
                        panic!("Non-Eip1559 transactions are not supported yet");
                    }

                    // rebroadcast
                    match client
                        .send_transaction(replacement_tx.0.clone(), None)
                        .await
                    {
                        Ok(tx) => {
                            replacement_tx.1 = *tx;
                        }
                        Err(x) => {
                            error!(tx=?replacement_tx, err=?x, "Failed to replace transaction: dropping it");
                            pending_txs.remove(&addr);
                        }
                    }

                    info!(tx_hash = ?tx_hash, new_gas_price = %new_gas_price, user = ?addr,
                      tx_type, instance_name, "Bumping gas: done");
                } else {
                    info!(tx_hash = ?tx_hash, time_since, bump_gas_delay, instance_name, "Bumping gas: too early");
                }
            }
        }

        Ok(())
    }

    #[instrument(skip(self, position_id, position, vaults, rates, spots), fields(self.instance_name))]
    pub fn is_collateralized(
        &mut self,
        position_id: &PositionIdType,
        position: &Position,
        vaults: &VaultMap,
        rates: &DiscountRateMap,
        spots: &SpotMap,
    ) -> bool {
        let vault = (*vaults).get(&position.vault_id).unwrap();
        let underlier = (*vault).underlier;
        let discount_rate = match (*rates).get(&position.token_id) {
            Some(rate) => rate.rate,
            None => vault.default_rate_id,
        };
        let spot = (*spots).get::<[u8; 20]>(&underlier.into()).unwrap().spot;
        let block_timestamp = U256::from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let maturity = vault.maturities.get(&position.token_id).unwrap().clone();
        let liquidation_ratio = vault.liquidation_ratio;

        let mut price;
        if !discount_rate.is_zero() && maturity.gt(&block_timestamp) {
            let base = discount_rate.add(WEI_IN_ETHER);
            let max_base = WEI_IN_ETHER.add(parse_units(2, 10).unwrap());
            if base.ge(&max_base) {
                return false;
            }
            let left = maturity.checked_sub(block_timestamp).unwrap();
            let rate = U256::from(
                Decimal::from_parts(base.as_u128(), 18, false)
                    .unwrap()
                    .checked_pow(&Decimal::from_parts(left.as_u128(), 0, false).unwrap())
                    .unwrap()
                    .round(18)
                    .into_parts()
                    .0,
            );
            price = WEI_IN_ETHER
                .checked_mul(WEI_IN_ETHER.into())
                .unwrap()
                .checked_div(rate)
                .unwrap();
        } else {
            price = WEI_IN_ETHER;
        }
        price = price
            .checked_mul(spot)
            .unwrap()
            .checked_div(WEI_IN_ETHER)
            .unwrap();
        price = price
            .checked_mul(WEI_IN_ETHER)
            .unwrap()
            .checked_div(liquidation_ratio)
            .unwrap();

        let collateral_value = position
            .collateral
            .checked_mul(price)
            .unwrap()
            .checked_div(WEI_IN_ETHER)
            .unwrap();

        let debt = position
            .normal_debt
            .checked_mul(vault.rate)
            .unwrap()
            .checked_div(WEI_IN_ETHER)
            .unwrap();

        let health_factor = match debt.eq(&U256::zero()) {
            true => U256::MAX,
            false => collateral_value
                .checked_mul(WEI_IN_ETHER)
                .unwrap()
                .checked_div(debt)
                .unwrap()
        };

        self.cached_positions.insert(
            position_id.clone(),
            CachedPosition {
                position_id: position_id.clone(),
                vault: position.vault_id.into(),
                token_id: position.token_id.into(),
                owner: position.user,
                collateral_value,
                debt,
                health_factor,
            },
        );

        if collateral_value.ge(&debt) {
            return true;
        }

        debug!(
            position_id = ?hex::encode(position_id),
            collateral_value = ?collateral_value,
            debt_value = ?debt,
            fair_price = ?price,
            spot = ?spot,
            discount_rate = ?discount_rate,
        );

        return false;
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    #[instrument(skip(self, _auctions, positions, vaults, rates, spots, gas_price), fields(self.instance_name))]
    pub async fn start_auctions(
        &mut self,
        _auctions: impl Iterator<Item = (&AuctionIdType, &Auction)>,
        positions: impl Iterator<Item = (&PositionIdType, &Position)>,
        vaults: &VaultMap,
        rates: &DiscountRateMap,
        spots: &SpotMap,
        gas_price: U256,
    ) -> Result<(), M> {
        debug!("Checking for undercollateralized positions...");

        let now = Instant::now();

        for (position_id, position) in positions {
            // only iterate over positions that do not have pending liquidations
            if let Some(pending_tx) = self.pending_liquidations.get(position_id) {
                trace!(
                    tx_hash = ?pending_tx.1,
                    position_id = ?hex::encode(position_id),
                    "Liquidation not confirmed yet"
                );
                continue;
            }

            if !self.is_collateralized(position_id, position, vaults, rates, spots) {
                info!(
                    position_id = ?hex::encode(position_id),
                    vault = ?H160::from(position.vault_id),
                    token_id = ?U256::from(position.token_id),
                    user = ?position.user,
                    collateral = ?position.collateral,
                    normal_debt = ?position.normal_debt,
                    gas_price=?gas_price,
                    instance_name=self.instance_name.as_str(),
                    "Found an undercollateralized position. starting an auction",
                );

                // Send the tx and track it
                let client = self.limes.client();
                let mut tx = self
                    .limes
                    .liquidate(
                        position.vault_id.into(),
                        position.token_id.into(),
                        position.user.into(),
                        Address::zero(),
                    )
                    .tx;

                match client.estimate_gas(&tx).await {
                    Ok(gas_estimation) => {
                        let sender = client.default_sender().unwrap();
                        let nonce = client.get_transaction_count(sender, None).await.unwrap();
                        let gas = gas_estimation
                            .mul(U256::from(self.gas_boost + 100))
                            .div(100);

                        tx.set_gas_price(gas_price);
                        tx.set_gas(gas);
                        tx.set_nonce(nonce);

                        let tx_signed = tx.rlp_signed(
                            client.get_chainid().await.unwrap().as_u64(),
                            &client.sign_transaction(&tx, sender).await.unwrap(),
                        );

                        match client.send_raw_transaction(tx_signed).await {
                            Ok(tx_hash) => {
                                info!(tx_hash = ?tx_hash,
                                position_id = ?hex::encode(position_id),
                                instance_name=self.instance_name.as_str(), "Submitted liquidation");
                                self.pending_liquidations
                                    .entry(*position_id)
                                    .or_insert((tx, *tx_hash, now));
                            }
                            Err(x) => {
                                warn!(
                                position_id = ?hex::encode(position_id),
                                error=?x,
                                calldata=?tx.data(),
                                "Can't start the auction. Transaction reverted."
                                );
                            }
                        };
                    }
                    Err(x) => {
                        warn!(
                            position_id = ?hex::encode(position_id),
                            error=?x,
                            "Can't start the auction. Gas estimation failed."
                        );
                    }
                }
            } else {
                debug!(
                    position_id=?hex::encode(position_id),
                    vault=?H160::from(position.vault_id),
                    token_id=?U256::from(position.token_id),
                    user=?position.user,
                    "Position is collateralized"
                );
            }
        }

        Ok(())
    }

    #[instrument(skip(self, auction_id), fields(self.instance_name))]
    pub async fn needs_redo(&self, auction_id: &AuctionIdType) -> bool {
        let status = match self
            .collateral_auction
            .get_status(auction_id.into())
            .call()
            .await {
                Ok(status) => status,
                Err(_) => (false, U256::MAX, U256::MAX, U256::MAX)
            };
        

        if status.0 == true {
            return true;
        }

        debug!(
            auction_id = ?hex::encode(auction_id),
        );

        return false;
    }

    /// Triggers redo for any active auctions which has exipired or where the floor price was met
    /// controller
    #[instrument(skip(self, auctions, gas_price), fields(self.instance_name))]
    pub async fn redo_auctions(
        &mut self,
        auctions: impl Iterator<Item = (&AuctionIdType, &Auction)>,
        gas_price: U256,
    ) -> Result<(), M> {
        debug!("Checking for auctions up for redo...");

        let now = Instant::now();

        for (auction_id, _auction) in auctions {
            // only iterate over auctions that do not have pending redo
            if let Some(pending_tx) = self.pending_auctions.get(auction_id) {
                trace!(
                    tx_hash = ?pending_tx.1,
                    auction_id = ?hex::encode(auction_id),
                    "Redo not confirmed yet"
                );
                continue;
            }

            if self.needs_redo(auction_id).await {
                info!(
                    auction_id = ?hex::encode(auction_id),
                    gas_price=?gas_price,
                    instance_name=self.instance_name.as_str(),
                    "Found an auction up for redo. redoing an auction",
                );

                // Send the tx and track it
                let client = self.limes.client();
                let mut tx = self
                    .collateral_auction
                    .redo_auction(auction_id.into(), Address::zero())
                    .tx;

                match client.estimate_gas(&tx).await {
                    Ok(gas_estimation) => {
                        let sender = client.default_sender().unwrap();
                        let nonce = client.get_transaction_count(sender, None).await.unwrap();
                        let gas = gas_estimation
                            .mul(U256::from(self.gas_boost + 100))
                            .div(100);

                        tx.set_gas_price(gas_price);
                        tx.set_gas(gas);
                        tx.set_nonce(nonce);

                        let tx_signed = tx.rlp_signed(
                            client.get_chainid().await.unwrap().as_u64(),
                            &client.sign_transaction(&tx, sender).await.unwrap(),
                        );

                        match client.send_raw_transaction(tx_signed).await {
                            Ok(tx_hash) => {
                                info!(tx_hash = ?tx_hash,
                                auction_id = ?hex::encode(auction_id),
                                instance_name=self.instance_name.as_str(), "Submitted Auction Redo");
                                self.pending_auctions
                                    .entry(*auction_id)
                                    .or_insert((tx, *tx_hash, now));
                            }
                            Err(x) => {
                                warn!(
                                auction_id = ?hex::encode(auction_id),
                                error=?x,
                                calldata=?tx.data(),
                                "Can't redo the auction. Transaction reverted."
                                );
                            }
                        };
                    }
                    Err(x) => {
                        warn!(
                            auction_id = ?hex::encode(auction_id),
                            error=?x,
                            "Can't redo the auction. Gas estimation failed."
                        );
                    }
                }
            } else {
                debug!(
                    auction_id=?hex::encode(auction_id),
                    "Auction is not up for redo"
                );
            }
        }

        Ok(())
    }
}
