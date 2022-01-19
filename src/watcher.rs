//! Positions
//!
//! This module is responsible for keeping track of open positions and observing their debt healthiness.
use crate::{
    bindings::AuctionIdType,
    bindings::Codex,
    bindings::Collybus,
    // bindings::IMulticall2, bindings::IMulticall2Call,
    bindings::Limes,
    bindings::LiquidationIdType,
    bindings::PositionIdType,
    bindings::RateIdType,
    bindings::SpotIdType,
    bindings::TokenIdType,
    bindings::UpdateIdType,
    bindings::VaultEPT,
    bindings::VaultIdType,
    Result,
};
use ethers::prelude::*;
use futures_util::try_join;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{
    debug,
    debug_span,
    info,
    instrument, // trace, warn
};

/// Map of vaults
pub type VaultMap = HashMap<VaultIdType, Vault>;
/// Map of positions
pub type PositionMap = HashMap<PositionIdType, Position>;
/// Map of discount rates
pub type RateMap = HashMap<RateIdType, Rate>;
/// Map of underlier spot prices
pub type SpotMap = HashMap<SpotIdType, Spot>;
// / Map of auctions
// pub type AuctionMap = HashMap<AuctionIdType, Auction>;

#[derive(Clone)]
pub struct Watcher<M> {
    // pub liquidator: Witch<M>,
    /// Codex contract
    pub codex: Codex<M>,
    /// Collybus contract
    pub collybus: Collybus<M>,
    /// Base Vault contract (to be instantiated when required)
    pub base_vault: VaultEPT<M>,
    /// Limes contract
    pub limes: Limes<M>,
    /// Mapping of vault address
    pub vaults: VaultMap,
    /// Mapping of the addresses that have taken loans from the system and might be susceptible to liquidations
    pub positions: PositionMap,
    /// Mapping of the of discount rates used to determine the value of a positions collateral
    pub rates: RateMap,
    /// Mapping of the of spot prices used to determine the value of a positions collateral
    pub spots: SpotMap,
    // We use multicall to batch together calls and have reduced stress on
    // our RPC endpoint
    // multicall2: IMulticall2<M>,
    // multicall_batch_size: usize,
    instance_name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Vault {
    pub vault_id: VaultIdType,
    pub token: H160,
    pub underlier: H160,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Position {
    pub position_id: PositionIdType,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub collateral: U256,
    pub normal_debt: U256,
    pub under_liquidation: bool,
    pub auction_id: AuctionIdType,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Rate {
    pub rate_id: RateIdType,
    pub rate: U256,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Spot {
    pub spot_id: SpotIdType,
    pub spot: U256,
}

#[derive(Debug, Eq)]
pub struct GenericUpdate<T> {
    pub update_id: UpdateIdType,
    pub block_number: U64,
    pub transaction_index: U64,
    pub data: T,
}

impl<T: Eq> PartialEq for GenericUpdate<T> {
    fn eq(&self, other: &Self) -> bool {
        self.update_id == other.update_id
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PositionUpdate {
    pub position_id: PositionIdType,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub delta_collateral: I256,
    pub delta_normal_debt: I256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RateUpdate {
    pub rate_id: RateIdType,
    pub rate: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpotUpdate {
    pub spot_id: SpotIdType,
    pub spot: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LiquidationUpdate {
    pub liquidation_id: LiquidationIdType,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub collateral: U256,
    pub normal_debt: U256,
    pub due: U256,
    pub collateral_auction: H160,
    pub auction_id: AuctionIdType,
}

#[derive(Debug, PartialEq, Eq)]
enum Update {
    PositionUpdate(GenericUpdate<PositionUpdate>),
    RateUpdate(GenericUpdate<RateUpdate>),
    SpotUpdate(GenericUpdate<SpotUpdate>),
    LiquidationUpdate(GenericUpdate<LiquidationUpdate>),
}

pub fn compute_update_id(
    block_number: U64,
    transaction_index: U64,
    content_hash: [u8; 32],
) -> UpdateIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Uint(block_number.as_u64().into()),
        ethers::abi::Token::Uint(transaction_index.as_u64().into()),
        ethers::abi::Token::FixedBytes(content_hash.into()),
    ]))
}

fn compute_position_update_id(vault: H160, token_id: U256, user: H160) -> PositionIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Address(vault.clone()),
        ethers::abi::Token::Uint(token_id.into()),
        ethers::abi::Token::Address(user.clone()),
    ]))
}

fn compute_rate_update_id(rate_id: U256, rate: U256) -> PositionIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Uint(rate_id.into()),
        ethers::abi::Token::Uint(rate.into()),
    ]))
}

fn compute_spot_update_id(spot_id: H160, spot: U256) -> PositionIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Address(spot_id.clone()),
        ethers::abi::Token::Uint(spot.into()),
    ]))
}

fn compute_liquidation_update_id(
    vault: H160,
    token_id: U256,
    user: H160,
    due: U256,
) -> LiquidationIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Address(vault.clone()),
        ethers::abi::Token::Uint(token_id.into()),
        ethers::abi::Token::Address(user.clone()),
        ethers::abi::Token::Uint(due.into()),
    ]))
}

impl<M: Middleware> Watcher<M> {
    /// Constructor
    pub async fn new(
        codex: Address,
        collybus: Address,
        limes: Address,
        _multicall2: Address,
        _multicall_batch_size: usize,
        client: Arc<M>,
        vaults: HashMap<VaultIdType, Vault>,
        positions: HashMap<PositionIdType, Position>,
        rates: HashMap<RateIdType, Rate>,
        spots: HashMap<SpotIdType, Spot>,
        instance_name: String,
    ) -> Self {
        // let multicall2 = IMulticall2::new(multicall2, client.clone());
        Watcher {
            codex: Codex::new(codex, client.clone()),
            collybus: Collybus::new(collybus, client.clone()),
            base_vault: VaultEPT::new(Address::zero(), client.clone()),
            limes: Limes::new(limes, client.clone()),
            vaults,
            positions,
            rates,
            spots,
            //     multicall2,
            //     multicall_batch_size,
            instance_name,
        }
    }

    /// Gets any new borrowers which may have joined the system since we last
    /// made this call and then proceeds to get the latest account details for
    /// each user
    #[instrument(skip(self), fields(self.instance_name))]
    pub async fn sync(&mut self, from_block: U64, to_block: U64) -> Result<(), M> {
        let span = debug_span!("monitoring");
        let _enter = span.enter();

        // Codex
        let modify_collateral_and_debt_query = self
            .codex
            .modify_collateral_and_debt_filter()
            .from_block(from_block)
            .to_block(to_block);
        let transfer_collateral_and_debt_query = self
            .codex
            .transfer_collateral_and_debt_filter()
            .from_block(from_block)
            .to_block(to_block);
        let confiscate_collateral_and_debt_query = self
            .codex
            .confiscate_collateral_and_debt_filter()
            .from_block(from_block)
            .to_block(to_block);

        // Collybus
        let update_discount_rate_query = self
            .collybus
            .update_discount_rate_filter()
            .from_block(from_block)
            .to_block(to_block);
        let update_spot_query = self
            .collybus
            .update_spot_filter()
            .from_block(from_block)
            .to_block(to_block);

        // Limes
        let liquidate_query = self
            .limes
            .liquidate_filter()
            .from_block(from_block)
            .to_block(to_block);

        // query events in parallel
        let (
            modify_collateral_and_debt_events,
            transfer_collateral_and_debt_events,
            confiscate_collateral_and_debt_events,
            update_discount_rate_events,
            update_spot_events,
            liquidate_events,
        ) = try_join!(
            modify_collateral_and_debt_query.query_with_meta(),
            transfer_collateral_and_debt_query.query_with_meta(),
            confiscate_collateral_and_debt_query.query_with_meta(),
            update_discount_rate_query.query_with_meta(),
            update_spot_query.query_with_meta(),
            liquidate_query.query_with_meta(),
        )
        .unwrap();

        // create queue of update events
        let mut modify_collateral_and_debt_updates: Vec<Update> = modify_collateral_and_debt_events
            .into_iter()
            .map(|(x, meta)| {
                Update::PositionUpdate(GenericUpdate::<PositionUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        compute_position_update_id(x.vault, x.token_id, x.user),
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    data: PositionUpdate {
                        position_id: compute_position_update_id(x.vault, x.token_id, x.user),
                        vault_id: x.vault.into(),
                        token_id: x.token_id.into(),
                        user: x.user,
                        delta_collateral: x.delta_collateral,
                        delta_normal_debt: x.delta_normal_debt,
                    },
                })
            })
            .collect();
        let mut transfer_collateral_and_debt_updates: Vec<Update> =
            transfer_collateral_and_debt_events
                .into_iter()
                .map(|(x, meta)| {
                    [
                        Update::PositionUpdate(GenericUpdate::<PositionUpdate> {
                            update_id: compute_update_id(
                                meta.block_number,
                                meta.transaction_index,
                                compute_position_update_id(x.vault, x.token_id, x.src),
                            ),
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            data: PositionUpdate {
                                position_id: compute_position_update_id(x.vault, x.token_id, x.src),
                                vault_id: x.vault.into(),
                                token_id: x.token_id.into(),
                                user: x.src,
                                delta_collateral: -x.delta_collateral,
                                delta_normal_debt: -x.delta_normal_debt,
                            },
                        }),
                        Update::PositionUpdate(GenericUpdate::<PositionUpdate> {
                            update_id: compute_update_id(
                                meta.block_number,
                                meta.transaction_index,
                                compute_position_update_id(x.vault, x.token_id, x.dst),
                            ),
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            data: PositionUpdate {
                                position_id: compute_position_update_id(x.vault, x.token_id, x.dst),
                                vault_id: x.vault.into(),
                                token_id: x.token_id.into(),
                                user: x.dst,
                                delta_collateral: x.delta_collateral,
                                delta_normal_debt: x.delta_normal_debt,
                            },
                        }),
                    ]
                })
                .flatten()
                .collect();
        let mut confiscate_collateral_and_debt_updates: Vec<Update> =
            confiscate_collateral_and_debt_events
                .into_iter()
                .map(|(x, meta)| {
                    Update::PositionUpdate(GenericUpdate::<PositionUpdate> {
                        update_id: compute_update_id(
                            meta.block_number,
                            meta.transaction_index,
                            compute_position_update_id(x.vault, x.token_id, x.user),
                        ),
                        block_number: meta.block_number,
                        transaction_index: meta.transaction_index,
                        data: PositionUpdate {
                            position_id: compute_position_update_id(x.vault, x.token_id, x.user),
                            vault_id: x.vault.into(),
                            token_id: x.token_id.into(),
                            user: x.user,
                            delta_collateral: x.delta_collateral,
                            delta_normal_debt: x.delta_normal_debt,
                        },
                    })
                })
                .collect();

        let mut update_discount_rate_updates: Vec<Update> = update_discount_rate_events
            .into_iter()
            .map(|(x, meta)| {
                Update::RateUpdate(GenericUpdate::<RateUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        compute_rate_update_id(x.token_id, x.rate),
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    data: RateUpdate {
                        rate_id: x.token_id.into(),
                        rate: x.rate,
                    },
                })
            })
            .collect();
        let mut update_spot_updates: Vec<Update> = update_spot_events
            .into_iter()
            .map(|(x, meta)| {
                Update::SpotUpdate(GenericUpdate::<SpotUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        compute_spot_update_id(x.token, x.spot),
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    data: SpotUpdate {
                        spot_id: x.token.into(),
                        spot: x.spot,
                    },
                })
            })
            .collect();

        let mut liquidate_updates: Vec<Update> = liquidate_events
            .into_iter()
            .map(|(x, meta)| {
                Update::LiquidationUpdate(GenericUpdate::<LiquidationUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        compute_liquidation_update_id(x.vault, x.token_id, x.position, x.due),
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    data: LiquidationUpdate {
                        liquidation_id: compute_liquidation_update_id(
                            x.vault, x.token_id, x.position, x.due,
                        ),
                        vault_id: x.vault.into(),
                        token_id: x.token_id.into(),
                        user: x.position.into(),
                        due: x.due,
                        collateral: x.collateral,
                        normal_debt: x.normal_debt,
                        collateral_auction: x.collateral_auction.into(),
                        auction_id: x.auction_id.into(),
                    },
                })
            })
            .collect();

        // consolidate all update events
        let mut all_updates: Vec<Update> = Vec::new();
        all_updates.append(&mut modify_collateral_and_debt_updates);
        all_updates.append(&mut transfer_collateral_and_debt_updates);
        all_updates.append(&mut confiscate_collateral_and_debt_updates);
        all_updates.append(&mut update_discount_rate_updates);
        all_updates.append(&mut update_spot_updates);
        all_updates.append(&mut liquidate_updates);

        // sort them by 1. block_number, 2. transaction_index
        all_updates.sort_by(|a, b| {
            let block_number_a = match a {
                Update::PositionUpdate(a) => a.block_number,
                Update::RateUpdate(a) => a.block_number,
                Update::SpotUpdate(a) => a.block_number,
                Update::LiquidationUpdate(a) => a.block_number,
            };
            let block_number_b = match b {
                Update::PositionUpdate(b) => b.block_number,
                Update::RateUpdate(b) => b.block_number,
                Update::SpotUpdate(b) => b.block_number,
                Update::LiquidationUpdate(b) => b.block_number,
            };

            if block_number_a == block_number_b {
                let transaction_index_a = match a {
                    Update::PositionUpdate(a) => a.transaction_index,
                    Update::RateUpdate(a) => a.transaction_index,
                    Update::SpotUpdate(a) => a.transaction_index,
                    Update::LiquidationUpdate(a) => a.transaction_index,
                };
                let transaction_index_b = match b {
                    Update::PositionUpdate(b) => b.transaction_index,
                    Update::RateUpdate(b) => b.transaction_index,
                    Update::SpotUpdate(b) => b.transaction_index,
                    Update::LiquidationUpdate(b) => b.transaction_index,
                };
                return transaction_index_a.cmp(&transaction_index_b);
            }

            block_number_a.cmp(&block_number_b)
        });

        info!(
            count = all_updates.len(),
            instance_name = self.instance_name.as_str(),
            "Updates collected"
        );

        // process update events
        for update in all_updates.iter() {
            match update {
                Update::PositionUpdate(update) => {
                    debug!(
                        "Position Update: Block: {}, TxIndex: {}",
                        update.block_number, update.transaction_index
                    );
                    let _ = self.update_vault(update.data.vault_id).await;
                    let _ = self.update_collateral_and_debt(
                        update.data.position_id,
                        update.data.vault_id,
                        update.data.token_id,
                        update.data.user,
                        update.data.delta_collateral,
                        update.data.delta_normal_debt,
                    );
                }
                Update::RateUpdate(update) => {
                    debug!(
                        "Rate Update: Block: {}, TxIndex: {}",
                        update.block_number, update.transaction_index
                    );
                    let _ = self.update_rate(update.data.rate_id, update.data.rate);
                }
                Update::SpotUpdate(update) => {
                    debug!(
                        "Spot Update: Block: {}, TxIndex: {}",
                        update.block_number, update.transaction_index
                    );
                    let _ = self.update_spot(update.data.spot_id, update.data.spot);
                }
                Update::LiquidationUpdate(update) => {
                    debug!(
                        "Liquidation Update: Block: {}, TxIndex: {}",
                        update.block_number, update.transaction_index
                    );
                    let _ = self.update_liquidate(
                        update.data.liquidation_id,
                        update.data.vault_id,
                        update.data.token_id,
                        update.data.user,
                        update.data.due,
                        update.data.collateral,
                        update.data.normal_debt,
                        update.data.collateral_auction,
                        update.data.auction_id,
                    );
                }
            }
        }

        Ok(())
    }

    pub async fn update_vault(&mut self, vault_id: VaultIdType) -> Result<&Vault, M> {
        let exists = self.vaults.contains_key(&vault_id);
        let vault = self.vaults.entry(vault_id).or_insert(Vault {
            vault_id: vault_id,
            token: vault_id.into(),
            underlier: VaultEPT::new(vault_id.clone(), self.base_vault.client().into())
                .token()
                .call()
                .await
                .unwrap(),
        });

        if !exists {
            debug!(
                "New Vault added: {:?}, Token: {:?}, Underlier: {:?}",
                vault_id, vault.token, vault.underlier
            );
        }

        Ok(vault)
    }

    pub fn update_collateral_and_debt(
        &mut self,
        position_id: PositionIdType,
        vault_id: VaultIdType,
        token_id: TokenIdType,
        user: H160,
        delta_collateral: I256,
        delta_normal_debt: I256,
    ) -> Result<&Position, M> {
        let position = self.positions.entry(position_id).or_insert(Position {
            position_id: position_id,
            vault_id: vault_id,
            token_id: token_id,
            user: user,
            collateral: U256::zero(),
            normal_debt: U256::zero(),
            under_liquidation: false,
            auction_id: Default::default(),
        });

        match delta_collateral.into_sign_and_abs() {
            (Sign::Positive, abs) => position.collateral += abs,
            (Sign::Negative, abs) => position.collateral -= abs,
        }
        match delta_normal_debt.into_sign_and_abs() {
            (Sign::Positive, abs) => position.normal_debt += abs,
            (Sign::Negative, abs) => position.normal_debt -= abs,
        }

        // info!(
        //     "vault: {:?}, tokenId: {:?}, user: {:?}, collateral: {:?}, normalDebt: {:?}",
        //     vault_id, token_id, position.user, position.collateral, position.normal_debt
        // );

        Ok(position)
    }

    pub fn update_rate(&mut self, rate_id: RateIdType, value: U256) -> Result<&Rate, M> {
        let rate = self.rates.entry(rate_id).or_insert(Rate {
            rate_id: rate_id,
            rate: U256::zero(),
        });

        rate.rate = value;

        // info!(
        //     "vault: {:?}, tokenId: {:?}, user: {:?}, collateral: {:?}, normalDebt: {:?}",
        //     vault_id, token_id, position.user, position.collateral, position.normal_debt
        // );

        Ok(rate)
    }

    pub fn update_spot(&mut self, spot_id: SpotIdType, value: U256) -> Result<&Spot, M> {
        let spot = self.spots.entry(spot_id).or_insert(Spot {
            spot_id: spot_id,
            spot: U256::zero(),
        });

        spot.spot = value;

        // info!(
        //     "vault: {:?}, tokenId: {:?}, user: {:?}, collateral: {:?}, normalDebt: {:?}",
        //     vault_id, token_id, position.user, position.collateral, position.normal_debt
        // );

        Ok(spot)
    }

    pub fn update_liquidate(
        &mut self,
        position_id: PositionIdType,
        vault_id: VaultIdType,
        token_id: TokenIdType,
        user: H160,
        _due: U256,
        _collateral: U256,
        _normal_debt: U256,
        _collateral_auction: H160,
        auction_id: AuctionIdType,
    ) -> Result<&Position, M> {
        let position = self.positions.entry(position_id).or_insert(Position {
            position_id: position_id,
            vault_id: vault_id,
            token_id: token_id,
            user: user,
            collateral: U256::zero(),
            normal_debt: U256::zero(),
            under_liquidation: false,
            auction_id: Default::default(),
        });

        position.under_liquidation = true;
        position.auction_id = auction_id.clone();

        // info!(
        //     "vault: {:?}, tokenId: {:?}, user: {:?}, collateral: {:?}, normalDebt: {:?}",
        //     vault_id, token_id, position.user, position.collateral, position.normal_debt
        // );

        Ok(position)
    }
}
