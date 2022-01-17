//! Positions
//!
//! This module is responsible for keeping track of open positions and observing their debt healthiness.
use crate::{
    bindings::Codex,
    // bindings::IMulticall2, bindings::IMulticall2Call,
    bindings::PositionIdType,
    bindings::TokenIdType,
    bindings::VaultIdType,
    Result,
};
use core::cmp::Ordering;
use ethers::prelude::*;
use futures_util::try_join;

// use futures_util::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{
    debug,
    debug_span,
    instrument, // info, trace, warn
};

fn compute_position_id(vault: H160, token_id: U256, user: H160) -> PositionIdType {
    ethers::utils::keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Address(vault.clone()),
        ethers::abi::Token::Uint(token_id.into()),
        ethers::abi::Token::Address(user.clone()),
    ]))
}

pub type UpdateIdType = [u8; 32];

pub type PositionMap = HashMap<PositionIdType, Position>;

#[derive(Clone)]
pub struct PositionsWatcher<M> {
    // pub liquidator: Witch<M>,
    pub codex: Codex<M>,

    /// Mapping of the addresses that have taken loans from the system and might
    /// be susceptible to liquidations
    pub positions: PositionMap,
    // We use multicall to batch together calls and have reduced stress on
    // our RPC endpoint
    // multicall2: IMulticall2<M>,
    // multicall_batch_size: usize,

    // instance_name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
/// A position's details
pub struct Position {
    pub position_id: PositionIdType,
    pub under_auction: bool,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub collateral: U256,
    pub normal_debt: U256,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Ord)]
pub struct PositionUpdate {
    pub block_number: U64,
    pub transaction_index: U64,
    pub position_id: PositionIdType,
    pub vault_id: VaultIdType,
    pub token_id: TokenIdType,
    pub user: H160,
    pub delta_collateral: I256,
    pub delta_normal_debt: I256,
}

impl PartialOrd for PositionUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.block_number == other.block_number {
            return Some(self.transaction_index.cmp(&other.transaction_index));
        }
        Some(self.block_number.cmp(&other.block_number))
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Ord)]
pub struct RateUpdate {
    pub block_number: U64,
    pub transaction_index: U64,
    pub rate_id: PositionIdType,
    pub value: U256,
}

impl PartialOrd for RateUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.block_number == other.block_number {
            return Some(self.transaction_index.cmp(&other.transaction_index));
        }
        Some(self.block_number.cmp(&other.block_number))
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, Ord)]
pub struct Update<T> {
    pub update_id: UpdateIdType, 
    pub block_number: U64,
    pub transaction_index: U64,
    pub data: T,
}

impl<T> PartialEq for Update<T> {
    fn eq(&self, other: &Self) -> bool {
        self.update_id == other.update_id
    }
}

impl<T> PartialOrd for Update<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.block_number == other.block_number {
            return Some(self.transaction_index.cmp(&other.transaction_index));
        }
        Some(self.block_number.cmp(&other.block_number))
    }
}

enum UpdateTypes {
    PositionUpdate(PositionUpdate),
    RateUpdate(RateUpdate),
}

impl<M: Middleware> PositionsWatcher<M> {
    /// Constructor
    pub async fn new(
        // liquidator: Address,
        codex_: Address,
        _multicall2: Address,
        _multicall_batch_size: usize,
        client: Arc<M>,
        positions: HashMap<PositionIdType, Position>,
        _instance_name: String,
    ) -> Self {
        // let multicall2 = IMulticall2::new(multicall2, client.clone());
        PositionsWatcher {
            // cauldron: Cauldron::new(cauldron, client.clone()),
            // liquidator: Witch::new(liquidator, client),
            codex: Codex::new(codex_, client),
            positions,
            //     multicall2,
            //     multicall_batch_size,
            //     instance_name,
        }
    }

    /// Gets any new borrowers which may have joined the system since we last
    /// made this call and then proceeds to get the latest account details for
    /// each user
    #[instrument(skip(self), fields(self.instance_name))]
    pub async fn update_positions(&mut self, from_block: U64, to_block: U64) -> Result<(), M> {
        let span = debug_span!("monitoring");
        let _enter = span.enter();

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

        let (
            modify_collateral_and_debt_events,
            transfer_collateral_and_debt_events,
            confiscate_collateral_and_debt_events,
        ) = try_join!(
            modify_collateral_and_debt_query.query_with_meta(),
            transfer_collateral_and_debt_query.query_with_meta(),
            confiscate_collateral_and_debt_query.query_with_meta()
        )
        .unwrap();

        let mut modify_collateral_and_debt_updates: Vec<Update<PositionUpdate>> =
            modify_collateral_and_debt_events
                .into_iter()
                .map(|(x, meta)| Update {
                    update_id: compute_position_id(x.vault, x.token_id, x.user),
                    block_number: meta.block_number,
                    transaction_index: meta.transaction_index,
                    data: PositionUpdate {
                        block_number: meta.block_number,
                        transaction_index: meta.transaction_index,
                        position_id: compute_position_id(x.vault, x.token_id, x.user),
                        vault_id: x.vault.into(),
                        token_id: x.token_id.into(),
                        user: x.user,
                        delta_collateral: x.delta_collateral,
                        delta_normal_debt: x.delta_normal_debt,
                    }
                })
                .collect();

        let mut transfer_collateral_and_debt_updates: Vec<Update<PositionUpdate>> =
            transfer_collateral_and_debt_events
                .into_iter()
                .map(|(x, meta)| {
                    [
                        Update {
                            update_id: compute_position_id(x.vault, x.token_id, x.src),
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            data: PositionUpdate {
                                block_number: meta.block_number,
                                transaction_index: meta.transaction_index,
                                position_id: compute_position_id(x.vault, x.token_id, x.src),
                                vault_id: x.vault.into(),
                                token_id: x.token_id.into(),
                                user: x.src,
                                delta_collateral: -x.delta_collateral,
                                delta_normal_debt: -x.delta_normal_debt,
                            },
                        },
                        Update {
                            update_id: compute_position_id(x.vault, x.token_id, x.dst),
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            data: PositionUpdate {
                                block_number: meta.block_number,
                                transaction_index: meta.transaction_index,
                                position_id: compute_position_id(x.vault, x.token_id, x.dst),
                                vault_id: x.vault.into(),
                                token_id: x.token_id.into(),
                                user: x.dst,
                                delta_collateral: x.delta_collateral,
                                delta_normal_debt: x.delta_normal_debt,
                            },
                        },
                    ]
                })
                .flatten()
                .collect();

                let mut confiscate_collateral_and_debt_updates: Vec<Update<PositionUpdate>> =
                confiscate_collateral_and_debt_events
                    .into_iter()
                    .map(|(x, meta)| Update {
                        update_id: compute_position_id(x.vault, x.token_id, x.user),
                        block_number: meta.block_number,
                        transaction_index: meta.transaction_index,
                        data: PositionUpdate {
                            block_number: meta.block_number,
                            transaction_index: meta.transaction_index,
                            position_id: compute_position_id(x.vault, x.token_id, x.user),
                            vault_id: x.vault.into(),
                            token_id: x.token_id.into(),
                            user: x.user,
                            delta_collateral: x.delta_collateral,
                            delta_normal_debt: x.delta_normal_debt,
                        }
                    })
                    .collect();

        let mut all_updates: Vec<Update<UpdateTypes>> = Vec::new();
        all_updates.append(&mut modify_collateral_and_debt_updates);
        all_updates.append(&mut transfer_collateral_and_debt_updates);
        all_updates.append(&mut confiscate_collateral_and_debt_updates);
        all_updates.sort();
        all_updates.into_iter().for_each(|position_update| {
            debug!(
                "{}, {}",
                position_update.block_number, position_update.transaction_index
            );

            let _ = self.update_position(
                position_update.position_id,
                position_update.vault_id,
                position_update.token_id,
                position_update.user,
                position_update.delta_collateral,
                position_update.delta_normal_debt,
            );
        });

        Ok(())
    }

    pub fn update_position(
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
            under_auction: false,
            vault_id: vault_id,
            token_id: token_id,
            user: user,
            collateral: U256::zero(),
            normal_debt: U256::zero(),
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
}
