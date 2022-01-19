//! Auctions Module
//!
//! This module is responsible for triggering and participating in a Auction's
//! dutch auction
use crate::{
    bindings::{CollateralAuction, Limes, PositionIdType},
    escalator::GeometricGasPrice,
    watcher::Position,
    Result,
};

use ethers_core::types::transaction::eip2718::TypedTransaction;

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, sync::Arc, time::Instant};
use tracing::{debug, error, info, instrument, trace, warn};

pub type AuctionMap = HashMap<PositionIdType, bool>;

#[derive(Clone)]
pub struct Liquidator<M> {
    limes: Limes<M>,
    _collateral_auction: CollateralAuction<M>,
    /// The currently active auctions
    pub auctions: AuctionMap,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    _multicall: Multicall<M>,

    /// The minimum ratio (collateral/debt) to trigger liquidation
    _min_ratio: u16,

    // extra gas to use for txs, as percent of estimated gas cost
    _gas_boost: u16,

    pending_liquidations: HashMap<PositionIdType, PendingTransaction>,
    pending_auctions: HashMap<PositionIdType, PendingTransaction>,
    gas_escalator: GeometricGasPrice,
    bump_gas_delay: u64,

    instance_name: String,
}

/// Tx / Hash/ Submitted at time
type PendingTransaction = (TypedTransaction, TxHash, Instant);

/// An initiated auction
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Auction {
    /// The start time of the auction
    started: u32,
    under_auction: bool,
    /// The debt which can be repaid
    debt: u128,

    ratio_pct: u16,
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
        min_ratio: u16,
        gas_boost: u16,
        client: Arc<M>,
        auctions: AuctionMap,
        gas_escalator: GeometricGasPrice,
        bump_gas_delay: u64,
        instance_name: String,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("could not initialize multicall");

        Self {
            limes: Limes::new(limes, client.clone()),
            _collateral_auction: CollateralAuction::new(collateral_auction, client.clone()),
            // flash_liquidator: FlashLiquidator::new(flashloan, client.clone()),
            _multicall: multicall,
            _min_ratio: min_ratio,
            _gas_boost: gas_boost,
            // target_collateral_offer,
            auctions,

            pending_liquidations: HashMap::new(),
            pending_auctions: HashMap::new(),
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
                  status = status, tx_type, instance_name, "confirmed");
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

    pub fn is_collateralized(&self, _position_id: &PositionIdType, _position: &Position) -> bool {
        return true;
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    #[instrument(skip(self, positions), fields(self.instance_name))]
    pub async fn start_auctions(
        &mut self,
        positions: impl Iterator<Item = (&PositionIdType, &Position)>,
        gas_price: U256,
    ) -> Result<(), M> {
        debug!("checking for undercollateralized positions...");

        let now = Instant::now();

        for (position_id, position) in positions {
            //   if !position.is_initialized {
            //       trace!(position_id = ?hex::encode(position_id), "Position is not initialized yet, skipping");
            //       continue;
            //   }
            // only iterate over positions that do not have pending liquidations
            if let Some(pending_tx) = self.pending_liquidations.get(position_id) {
                trace!(tx_hash = ?pending_tx.1, position_id = ?hex::encode(position_id), "liquidation not confirmed yet");
                continue;
            }

            if !self.is_collateralized(position_id, position) {
                if position.under_liquidation {
                    debug!(position_id = ?hex::encode(position_id), details = ?position, "found position under auction, ignoring it");
                    continue;
                }
                info!(
                    position_id = ?hex::encode(position_id), details = ?position, gas_price=?gas_price,
                    instance_name=self.instance_name.as_str(),
                    "found an undercollateralized position. starting an auction",
                );

                // Send the tx and track it
                let call = self
                    .limes
                    .liquidate(
                        position.vault_id.into(),
                        position.token_id.into(),
                        position.user.into(),
                        Address::zero(),
                    )
                    .gas_price(gas_price);
                let tx = call.tx.clone();
                match call.send().await {
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
                          calldata=?call.calldata(),
                          "Can't start the auction");
                    }
                };
            } else {
                debug!(position_id=?hex::encode(position_id), "position is collateralized");
            }
        }
        Ok(())
    }

    // fn current_offer(&self, now: u64, auction_start: u64, duration: u64, initial_offer: u64) -> Result<u16, M> {
    //     if now < auction_start.into() {
    //         return Err(ContractError::ConstructorError{});
    //     }
    //     let one = 10u64.pow(18);
    //     if initial_offer > one {
    //         error!(initial_offer, "initialOffer > 1");
    //         return Err(ContractError::ConstructorError{});
    //     }
    //     let initial_offer_pct = initial_offer / 10u64.pow(16); // 0-100

    //     let time_since_auction_start: u64 = now - auction_start;
    //     if time_since_auction_start >= duration {
    //         Ok(100)
    //     } else {
    //         // time_since_auction_start / duration * (1 - initial_offer) + initial_offer
    //         Ok((time_since_auction_start * (100 - initial_offer_pct) / duration + initial_offer_pct).try_into().unwrap())
    //     }
    // }

    // async fn get_auction(&mut self, position_id: PositionIdType) -> Result<Auction, M> {
    //     let (_, _, ilk_id) = self.cauldron.positions(position_id).call().await?;
    //     let balances_fn = self.cauldron.balances(position_id);
    //     let auction_fn = self.liquidator.auctions(position_id);

    //     trace!(
    //         position_id=?hex::encode(position_id),
    //         "Fetching auction details"
    //     );

    //     let multicall = self
    //         .multicall
    //         .clear_calls()
    //         .add_call(balances_fn)
    //         .add_call(auction_fn)
    //         .add_call(self.liquidator.ilks(ilk_id))
    //         .add_call(self.flash_liquidator.collateral_to_debt_ratio(position_id));

    //     let ((art, _), (auction_owner, auction_start), (duration, initial_offer), ratio_u256): (
    //         (u128, u128),
    //         (Address, u32),
    //         (u32, u64),
    //         U256,
    //     ) = multicall.call().await?;

    //     let current_offer: u16 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    //         Ok(x) => self
    //             .current_offer(
    //                 x.as_secs(),
    //                 u64::from(auction_start),
    //                 u64::from(duration),
    //                 initial_offer,
    //             )
    //             .unwrap_or(0),
    //         Err(x) => {
    //             error!("Failed to get system time: {}", x);
    //             0u16
    //         }
    //     };

    //     trace!(
    //         position_id=?hex::encode(position_id),
    //         debt=?art,
    //         ratio=?ratio_u256,
    //         current_offer=current_offer,
    //         "Fetched auction details"
    //     );

    //     let ratio_pct_u256 = ratio_u256 / U256::exp10(16);
    //     let ratio_pct: u16 = {
    //         if ratio_pct_u256 > U256::from(u16::MAX) {
    //             error!(position_id=?position_id, ratio_pct_u256=?ratio_pct_u256, "Ratio is too big");
    //             0
    //         } else {
    //             (ratio_pct_u256.as_u64()) as u16
    //         }
    //     };

    //     Ok(Auction {
    //         under_auction: (auction_owner != Address::zero()),
    //         started: auction_start,
    //         debt: art,
    //         ratio_pct: ratio_pct,
    //         // collateral_offer_is_good_enough: current_offer >= self.target_collateral_offer,
    //     })
    // }
}
