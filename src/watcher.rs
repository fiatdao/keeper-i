//! Positions
//!
//! This module is responsible for keeping track of open positions and observing their debt healthiness.
use crate::{
    bindings::AuctionIdType,
    bindings::Codex,
    bindings::CollateralAuction,
    bindings::Collybus,
    bindings::DiscountRateIdType,
    bindings::IVault,
    // bindings::IMulticall2, bindings::IMulticall2Call,
    bindings::Limes,
    bindings::PositionIdType,
    bindings::SpotIdType,
    bindings::TokenIdType,
    bindings::UpdateIdType,
    bindings::VaultIdType,
    Result,
};
use ethers::prelude::*;
use futures_util::try_join;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, debug_span, info, instrument};

pub type ProcessedUpdateMap = HashMap<UpdateIdType, bool>;
/// Map of vaults
pub type VaultMap = HashMap<VaultIdType, Vault>;
/// Map of positions
pub type PositionMap = HashMap<PositionIdType, Position>;
/// Map of discount rates
pub type DiscountRateMap = HashMap<DiscountRateIdType, DiscountRate>;
/// Map of underlier spot prices
pub type SpotMap = HashMap<SpotIdType, Spot>;
// / Map of auctions
pub type AuctionMap = HashMap<AuctionIdType, Auction>;
// / Map of maturities
pub type MaturityMap = HashMap<TokenIdType, U256>;
// / Map of maturities
pub type DiscountRateIdMap = HashMap<TokenIdType, U256>;

#[derive(Clone)]
pub struct Watcher<M> {
    // pub liquidator: Witch<M>,
    /// Codex contract
    pub codex: Codex<M>,
    /// CollateralAuction contract
    pub collateral_auction: CollateralAuction<M>,
    /// Collybus contract
    pub collybus: Collybus<M>,
    /// Base Vault contract (to be instantiated when required)
    pub base_vault: IVault<M>,
    /// Limes contract
    pub limes: Limes<M>,
    /// Mapping of auction address
    pub auctions: AuctionMap,
    /// Mapping of vault address
    pub vaults: VaultMap,
    /// Mapping of the addresses that have taken loans from the system and might be susceptible to liquidations
    pub positions: PositionMap,
    /// Mapping of the of discount rates used to determine the value of a positions collateral
    pub rates: DiscountRateMap,
    /// Mapping of the of spot prices used to determine the value of a positions collateral
    pub spots: SpotMap,
    /// Mapping of processed updates (used as reorg protection)
    pub processed_updates: ProcessedUpdateMap,
    // We use multicall to batch together calls and have reduced stress on
    // our RPC endpoint
    // multicall2: IMulticall2<M>,
    // multicall_batch_size: usize,
    instance_name: String,
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Vault {
    pub vault_id: VaultIdType,
    pub token: H160,
    pub underlier: H160,
    pub liquidation_ratio: U256,
    pub default_rate_id: U256,
    pub rate: U256,

    #[serde_as(as = "Vec<(_, _)>")]
    pub rate_ids: DiscountRateIdMap,

    #[serde_as(as = "Vec<(_, _)>")]
    pub maturities: MaturityMap,
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
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct DiscountRate {
    pub rate_id: DiscountRateIdType,
    pub rate: U256,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Spot {
    pub spot_id: SpotIdType,
    pub spot: U256,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// An auction's details
pub struct Auction {
    pub auction_id: AuctionIdType,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub start_price: U256,
    pub debt: U256,
    pub collateral_to_sell: U256,
}

#[derive(Debug, Eq)]
pub struct GenericUpdate<T> {
    pub update_id: UpdateIdType,
    pub block_number: U64,
    pub transaction_index: U64,
    pub log_index: U256,
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
pub struct RateIdUpdate {
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub rate_id: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DiscountRateUpdate {
    pub rate_id: DiscountRateIdType,
    pub rate: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpotUpdate {
    pub spot_id: SpotIdType,
    pub spot: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LiquidationUpdate {
    pub position_id: PositionIdType,
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
pub struct AuctionUpdate {
    pub auction_id: AuctionIdType,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub start_price: U256,
    pub debt: U256,
    pub collateral_to_sell: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RateUpdate {
    pub vault_id: VaultIdType,
    pub delta_rate: I256,
}

#[derive(Debug, PartialEq, Eq)]
enum Update {
    PositionUpdate(GenericUpdate<PositionUpdate>),
    RateIdUpdate(GenericUpdate<RateIdUpdate>),
    DiscountRateUpdate(GenericUpdate<DiscountRateUpdate>),
    SpotUpdate(GenericUpdate<SpotUpdate>),
    LiquidationUpdate(GenericUpdate<LiquidationUpdate>),
    AuctionUpdate(GenericUpdate<AuctionUpdate>),
    RateUpdate(GenericUpdate<RateUpdate>),
}

pub fn compute_update_id(
    block_number: U64,
    transaction_index: U64,
    log_index: U256,
) -> UpdateIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Uint(block_number.as_u64().into()),
        ethers::abi::Token::Uint(transaction_index.as_u64().into()),
        ethers::abi::Token::Uint(log_index),
    ]))
}

fn compute_position_id(vault: H160, token_id: U256, user: H160) -> PositionIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Address(vault.clone()),
        ethers::abi::Token::Uint(token_id.into()),
        ethers::abi::Token::Address(user.clone()),
    ]))
}

impl<M: Middleware> Watcher<M> {
    /// Constructor
    pub async fn new(
        codex: Address,
        collateral_auction: Address,
        collybus: Address,
        limes: Address,
        _multicall2: Address,
        _multicall_batch_size: usize,
        client: Arc<M>,
        auctions: AuctionMap,
        vaults: VaultMap,
        positions: PositionMap,
        rates: DiscountRateMap,
        spots: SpotMap,
        processed_updates: ProcessedUpdateMap,
        instance_name: String,
    ) -> Self {
        // let multicall2 = IMulticall2::new(multicall2, client.clone());
        Watcher {
            codex: Codex::new(codex, client.clone()),
            collateral_auction: CollateralAuction::new(collateral_auction, client.clone()),
            collybus: Collybus::new(collybus, client.clone()),
            base_vault: IVault::new(Address::zero(), client.clone()),
            limes: Limes::new(limes, client.clone()),
            auctions,
            vaults,
            positions,
            rates,
            spots,
            processed_updates,
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
        let modify_rate_query = self
            .codex
            .modify_rate_filter()
            .from_block(from_block)
            .to_block(to_block);

        // Collybus
        let update_rate_id_query = self
            .collybus
            .set_param_filter()
            .from_block(from_block)
            .to_block(to_block);
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

        // CollateralAuction
        let start_auction_query = self
            .collateral_auction
            .start_auction_filter()
            .from_block(from_block)
            .to_block(to_block);
        let redo_auction_query = self
            .collateral_auction
            .redo_auction_filter()
            .from_block(from_block)
            .to_block(to_block);
        let stop_auction_query = self
            .collateral_auction
            .stop_auction_filter()
            .from_block(from_block)
            .to_block(to_block);

        // query events in parallel
        let (
            modify_collateral_and_debt_events,
            transfer_collateral_and_debt_events,
            confiscate_collateral_and_debt_events,
            modify_rate_events,
            update_rate_id_events,
            update_discount_rate_events,
            update_spot_events,
            liquidate_events,
            start_auction_events,
            redo_auction_events,
            stop_auction_events,
        ) = try_join!(
            modify_collateral_and_debt_query.query_with_meta(),
            transfer_collateral_and_debt_query.query_with_meta(),
            confiscate_collateral_and_debt_query.query_with_meta(),
            modify_rate_query.query_with_meta(),
            update_rate_id_query.query_with_meta(),
            update_discount_rate_query.query_with_meta(),
            update_spot_query.query_with_meta(),
            liquidate_query.query_with_meta(),
            start_auction_query.query_with_meta(),
            redo_auction_query.query_with_meta(),
            stop_auction_query.query_with_meta(),
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
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: PositionUpdate {
                        position_id: compute_position_id(x.vault, x.token_id, x.user),
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
                                meta.log_index,
                            ),
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            log_index: meta.log_index,
                            data: PositionUpdate {
                                position_id: compute_position_id(x.vault, x.token_id, x.dst),
                                vault_id: x.vault.into(),
                                token_id: x.token_id.into(),
                                user: x.dst,
                                delta_collateral: x.delta_collateral,
                                delta_normal_debt: x.delta_normal_debt,
                            },
                        }),
                        Update::PositionUpdate(GenericUpdate::<PositionUpdate> {
                            update_id: compute_update_id(
                                meta.block_number,
                                meta.transaction_index,
                                meta.log_index,
                            ),
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            log_index: meta.log_index,
                            data: PositionUpdate {
                                position_id: compute_position_id(x.vault, x.token_id, x.src),
                                vault_id: x.vault.into(),
                                token_id: x.token_id.into(),
                                user: x.src,
                                delta_collateral: -x.delta_collateral,
                                delta_normal_debt: -x.delta_normal_debt,
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
                            meta.log_index,
                        ),
                        block_number: meta.block_number,
                        transaction_index: meta.transaction_index,
                        log_index: meta.log_index,
                        data: PositionUpdate {
                            position_id: compute_position_id(x.vault, x.token_id, x.user),
                            vault_id: x.vault.into(),
                            token_id: x.token_id.into(),
                            user: x.user,
                            delta_collateral: x.delta_collateral,
                            delta_normal_debt: x.delta_normal_debt,
                        },
                    })
                })
                .collect();
        let mut modify_rate_updates: Vec<Update> = modify_rate_events
            .into_iter()
            .map(|(x, meta)| {
                Update::RateUpdate(GenericUpdate::<RateUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: RateUpdate {
                        vault_id: x.vault.into(),
                        delta_rate: x.delta_rate,
                    },
                })
            })
            .collect();

        let mut update_rate_id_updates: Vec<Update> = update_rate_id_events
            .into_iter()
            .map(|(x, meta)| {
                Update::RateIdUpdate(GenericUpdate::<RateIdUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: RateIdUpdate {
                        vault_id: x.vault.into(),
                        token_id: x.token_id.into(),
                        rate_id: x.data.into(),
                    },
                })
            })
            .collect();
        let mut update_discount_rate_updates: Vec<Update> = update_discount_rate_events
            .into_iter()
            .map(|(x, meta)| {
                Update::DiscountRateUpdate(GenericUpdate::<DiscountRateUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: DiscountRateUpdate {
                        rate_id: x.rate_id.into(),
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
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
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
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: LiquidationUpdate {
                        position_id: compute_position_id(
                            x.vault, x.token_id, x.position,
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

        let mut start_auction_updates: Vec<Update> = start_auction_events
            .into_iter()
            .map(|(x, meta)| {
                Update::AuctionUpdate(GenericUpdate::<AuctionUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        x.auction_id.into(),
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: AuctionUpdate {
                        auction_id: x.auction_id.into(),
                        vault_id: x.vault.into(),
                        token_id: x.token_id.into(),
                        user: x.user.into(),
                        start_price: x.start_price,
                        debt: x.debt,
                        collateral_to_sell: x.collateral_to_sell.into(),
                    },
                })
            })
            .collect();
        let mut redo_auction_updates: Vec<Update> = redo_auction_events
            .into_iter()
            .map(|(x, meta)| {
                Update::AuctionUpdate(GenericUpdate::<AuctionUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: AuctionUpdate {
                        auction_id: x.auction_id.into(),
                        vault_id: x.vault.into(),
                        token_id: x.token_id.into(),
                        user: x.user.into(),
                        start_price: x.start_price,
                        debt: x.debt,
                        collateral_to_sell: x.collateral_to_sell.into(),
                    },
                })
            })
            .collect();
        let mut stop_auction_updates: Vec<Update> = stop_auction_events
            .into_iter()
            .map(|(x, meta)| {
                Update::AuctionUpdate(GenericUpdate::<AuctionUpdate> {
                    update_id: compute_update_id(
                        meta.block_number,
                        meta.transaction_index,
                        meta.log_index,
                    ),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    log_index: meta.log_index,
                    data: AuctionUpdate {
                        auction_id: x.auction_id.into(),
                        vault_id: Default::default(),
                        token_id: Default::default(),
                        user: Default::default(),
                        start_price: Default::default(),
                        debt: Default::default(),
                        collateral_to_sell: Default::default(),
                    },
                })
            })
            .collect();

        // consolidate all update events
        let mut all_updates: Vec<Update> = Vec::new();
        all_updates.append(&mut modify_collateral_and_debt_updates);
        all_updates.append(&mut transfer_collateral_and_debt_updates);
        all_updates.append(&mut confiscate_collateral_and_debt_updates);
        all_updates.append(&mut modify_rate_updates);
        all_updates.append(&mut update_rate_id_updates);
        all_updates.append(&mut update_discount_rate_updates);
        all_updates.append(&mut update_spot_updates);
        all_updates.append(&mut liquidate_updates);
        all_updates.append(&mut start_auction_updates);
        all_updates.append(&mut redo_auction_updates);
        all_updates.append(&mut stop_auction_updates);

        // sort them by 1. block_number, 2. transaction_index
        all_updates.sort_by(|a, b| {
            let (block_number_a, transaction_index_a, log_index_a) = match a {
                Update::PositionUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::RateIdUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::DiscountRateUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::SpotUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::LiquidationUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::AuctionUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::RateUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
            };
            let (block_number_b, transaction_index_b, log_index_b) = match b {
                Update::PositionUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::RateIdUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::DiscountRateUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::SpotUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::LiquidationUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::AuctionUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
                Update::RateUpdate(u) => (u.block_number, u.transaction_index, u.log_index),
            };

            if block_number_a == block_number_b {
                if transaction_index_a == transaction_index_b {
                    return log_index_a.cmp(&log_index_b);
                }
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
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    let _ = self.update_vault(update.data.vault_id).await;
                    let _ = self.update_maturity_for_token_id(update.data.vault_id, update.data.token_id).await;
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        vault=?H160::from(update.data.vault_id),
                        token_id=?U256::from(update.data.token_id),
                        user=?update.data.user,
                        delta_collateral=?update.data.delta_collateral,
                        delta_normal_debt=?update.data.delta_normal_debt,
                        "Position updated",
                    );
                    let _ = self.update_collateral_and_debt(
                        update.data.position_id,
                        update.data.vault_id,
                        update.data.token_id,
                        update.data.user,
                        update.data.delta_collateral,
                        update.data.delta_normal_debt,
                    );
                    self.processed_updates.insert(update.update_id, true);
                }
                Update::RateIdUpdate(update) => {
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        vault=?H160::from(update.data.vault_id),
                        token_id=?U256::from(update.data.token_id),
                        rate_id=?U256::from(update.data.rate_id),
                        "RateId updated",
                    );
                    let _ = self.update_rate_id(
                        update.data.vault_id,
                        update.data.token_id,
                        update.data.rate_id,
                    );
                    self.processed_updates.insert(update.update_id, true);
                }
                Update::DiscountRateUpdate(update) => {
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        rate_id=?U256::from(update.data.rate_id),
                        discount_rate=?U256::from(update.data.rate),
                        "DiscountRate updated",
                    );
                    let _ = self.update_discount_rate(update.data.rate_id, update.data.rate);
                    self.processed_updates.insert(update.update_id, true);
                }
                Update::SpotUpdate(update) => {
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        spot_id=?H160::from(update.data.spot_id),
                        spot=?U256::from(update.data.spot),
                        "Spot updated",
                    );
                    let _ = self.update_spot(update.data.spot_id, update.data.spot);
                    self.processed_updates.insert(update.update_id, true);
                }
                Update::LiquidationUpdate(update) => {
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        vault=?H160::from(update.data.vault_id),
                        token_id=?U256::from(update.data.token_id),
                        user=?update.data.user,
                        "Liquidation updated",
                    );
                    // let _ = self.update_liquidate(
                    //     update.data.position_id,
                    //     update.data.vault_id,
                    //     update.data.token_id,
                    //     update.data.user,
                    //     update.data.due,
                    //     update.data.collateral,
                    //     update.data.normal_debt,
                    //     update.data.collateral_auction,
                    //     update.data.auction_id,
                    // );
                    self.processed_updates.insert(update.update_id, true);
                }
                Update::AuctionUpdate(update) => {
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        auction_id=?U256::from(update.data.auction_id),
                        vault=?H160::from(update.data.vault_id),
                        token_id=?U256::from(update.data.token_id),
                        user=?update.data.user,
                        "Auction updated",
                    );
                    let _ = self.update_auction(
                        update.data.auction_id,
                        update.data.vault_id,
                        update.data.token_id,
                        update.data.user,
                        update.data.start_price,
                        update.data.debt,
                        update.data.collateral_to_sell,
                    );
                    self.processed_updates.insert(update.update_id, true);
                }
                Update::RateUpdate(update) => {
                    if self.processed_updates.contains_key(&update.update_id) {
                        continue;
                    }
                    debug!(
                        transaction_index=?update.transaction_index,
                        log_index=?update.log_index,
                        vault=?H160::from(update.data.vault_id),
                        delta_rate=?I256::from(update.data.delta_rate),
                        "Rate updated",
                    );
                    let _ = self.update_rate(update.data.vault_id, update.data.delta_rate);
                    self.processed_updates.insert(update.update_id, true);
                }
            }
        }

        Ok(())
    }

    pub async fn update_vault(&mut self, vault_id: VaultIdType) -> Result<&mut Vault, M> {
        let exists = self.vaults.contains_key(&vault_id);

        let vault = match exists {
            true => self.vaults.get_mut(&vault_id).unwrap(),
            false => {
                let vc = IVault::new(vault_id.clone(), self.base_vault.client().into());
                // let vault_type = vc.vault_type().call().await.unwrap();
                let underlier = match vc.underlier_token().call().await {
                    Ok(underlier_token) => match underlier_token.is_zero() {
                        false => underlier_token,
                        true => vc.token().call().await.unwrap(),
                    },
                    _ => vc.token().call().await.unwrap(),
                };
                let v = self.vaults.entry(vault_id).or_insert(Vault {
                    vault_id: vault_id,
                    token: vault_id.into(),
                    underlier,
                    liquidation_ratio: self
                        .collybus
                        .vaults(vault_id.into())
                        .call()
                        .await
                        .unwrap()
                        .0
                        .into(),
                    default_rate_id: self
                        .collybus
                        .vaults(vault_id.into())
                        .call()
                        .await
                        .unwrap()
                        .1
                        .into(),
                    rate: self
                        .codex
                        .vaults(vault_id.into())
                        .call()
                        .await
                        .unwrap()
                        .1
                        .into(),
                    rate_ids: HashMap::new(),
                    maturities: HashMap::new(),
                });

                // Maturity for TokenId 0
                v.maturities.insert(
                    U256::zero().into(),
                    vc.maturity(U256::zero()).call().await.unwrap(),
                );
                debug!(
                    vault=?H160::from(v.vault_id),
                    token=?H160::from(v.token),
                    underlier=?H160::from(v.underlier),
                    "Added Vault",
                );
                v
            }
        };

        Ok(vault)
    }

    pub async fn update_rate_id(
        &mut self,
        vault_id: VaultIdType,
        token_id: TokenIdType,
        rate_id: U256,
    ) -> Result<&Vault, M> {
        let vault = self.update_vault(vault_id).await.unwrap();

        vault.rate_ids.insert(token_id, rate_id);

        Ok(vault)
    }

    pub async fn update_maturity_for_token_id(
        &mut self,
        vault_id: VaultIdType,
        token_id: TokenIdType
    ) -> Result<(), M> {
        if !self.update_vault(vault_id).await.unwrap().maturities.contains_key(&token_id) {
            let client = self.base_vault.client().clone();
            let vc = IVault::new(vault_id.clone(), client.into());
            let maturity = vc.maturity(token_id.into()).call().await.unwrap();
            let vault = self.update_vault(vault_id).await.unwrap();
            vault.maturities.insert(token_id,maturity);
        }

        Ok(())
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
            position_id,
            vault_id,
            token_id,
            user,
            collateral: U256::zero(),
            normal_debt: U256::zero(),
        });

        match delta_collateral.into_sign_and_abs() {
            (Sign::Positive, abs) => position.collateral += abs,
            (Sign::Negative, abs) => {
                position.collateral -= if abs.gt(&position.collateral) {
                    position.collateral
                } else {
                    abs
                }
            }
        }
        match delta_normal_debt.into_sign_and_abs() {
            (Sign::Positive, abs) => position.normal_debt += abs,
            (Sign::Negative, abs) => {
                position.normal_debt -= if abs.gt(&position.normal_debt) {
                    position.normal_debt
                } else {
                    abs
                }
            }
        }

        Ok(position)
    }

    pub fn update_discount_rate(
        &mut self,
        rate_id: DiscountRateIdType,
        value: U256,
    ) -> Result<&DiscountRate, M> {
        let rate = self.rates.entry(rate_id).or_insert(DiscountRate {
            rate_id: rate_id,
            rate: U256::zero(),
        });

        rate.rate = value;

        Ok(rate)
    }

    pub fn update_spot(&mut self, spot_id: SpotIdType, value: U256) -> Result<&Spot, M> {
        let spot = self.spots.entry(spot_id).or_insert(Spot {
            spot_id,
            spot: U256::zero(),
        });

        spot.spot = value;

        Ok(spot)
    }

    // pub fn update_liquidate(
    //     &mut self,
    //     position_id: PositionIdType,
    //     vault_id: VaultIdType,
    //     token_id: TokenIdType,
    //     user: H160,
    //     _due: U256,
    //     _collateral: U256,
    //     _normal_debt: U256,
    //     _collateral_auction: H160,
    //     _auction_id: AuctionIdType,
    // ) -> Result<&Position, M> {
    //     let position = self.positions.entry(position_id).or_insert(Position {
    //         position_id,
    //         vault_id,
    //         token_id,
    //         user,
    //         collateral: U256::zero(),
    //         normal_debt: U256::zero(),
    //     });

    //     Ok(position)
    // }

    pub fn update_auction(
        &mut self,
        auction_id: AuctionIdType,
        vault_id: VaultIdType,
        token_id: TokenIdType,
        user: H160,
        start_price: U256,
        debt: U256,
        collateral_to_sell: U256,
    ) -> Result<&Auction, M> {
        let auction = self.auctions.entry(auction_id).or_insert(Auction {
            auction_id,
            vault_id,
            token_id,
            user,
            start_price: Default::default(),
            debt: Default::default(),
            collateral_to_sell: Default::default(),
        });

        auction.start_price = start_price;
        auction.debt = debt;
        auction.collateral_to_sell = collateral_to_sell;

        Ok(auction)
    }

    pub async fn update_rate(
        &mut self,
        vault_id: VaultIdType,
        delta_rate: I256,
    ) -> Result<&Vault, M> {
        let vault = self.update_vault(vault_id).await.unwrap();

        match delta_rate.into_sign_and_abs() {
            (Sign::Positive, abs) => vault.rate += abs,
            (Sign::Negative, abs) => {
                vault.rate -= if abs.gt(&(*vault).rate) {
                    vault.rate
                } else {
                    abs
                }
            }
        }

        Ok(vault)
    }
}
