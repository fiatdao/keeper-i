use crate::{
    escalator::GeometricGasPrice,
    liquidator::{
        Liquidator, CachedPositionMap
    },
    watcher::{
        AuctionMap, DiscountRateMap, PositionMap, ProcessedUpdateMap, SpotMap, VaultMap, Watcher
    },
    Result,
};

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{
    collections::HashMap, io::Write, path::PathBuf, sync::Arc, time::SystemTime, time::UNIX_EPOCH,
};
use tokio::time::{sleep, Duration};
use tracing::{debug_span, info, instrument, trace};

#[serde_as]
#[derive(Serialize, Deserialize, Default)]
/// The state which is stored in our logs
pub struct State {
    /// The vaults being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    vaults: VaultMap,
    /// The positions being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    positions: PositionMap,
    /// The rates being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    rates: DiscountRateMap,
    /// The spot prices being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    spots: SpotMap,
    /// The auctions being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    auctions: AuctionMap,
    /// Processed updates (used for avoiding reorgs)
    #[serde_as(as = "Vec<(_, _)>")]
    processed_updates: ProcessedUpdateMap,
    /// Cached positions
    #[serde_as(as = "Vec<(_, _)>")]
    cached_positions: CachedPositionMap,    
    /// The last observed block
    last_block: u64,
}

#[derive(Clone)]
/// The keeper monitors the chain for both liquidation opportunities and for
/// participation in auctions using Uniswap as a liquidity source
pub struct Keeper<M> {
    client: Arc<M>,
    last_block: U64,

    watcher: Watcher<M>,
    liquidator: Liquidator<M>,
    instance_name: String,
}

impl<M: Middleware> Keeper<M> {
    /// Instantiates the keeper. `state` should be passed if there is previous
    /// data which should be taken into account from a previous run
    pub async fn new(
        client: Arc<M>,
        codex: Address,
        collateral_auction: Address,
        collybus: Address,
        limes: Address,
        multicall2: Address,
        multicall_batch_size: usize,
        gas_boost: u16,
        gas_escalator: GeometricGasPrice,
        bump_gas_delay: u64,
        state: Option<State>,
        instance_name: String,
    ) -> Result<Keeper<M>, M> {
        let (vaults, positions, rates, spots, auctions, processed_updates, last_block) = match state
        {
            Some(state) => (
                state.vaults,
                state.positions,
                state.rates,
                state.spots,
                state.auctions,
                state.processed_updates,
                state.last_block.into(),
            ),
            None => (
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                0.into(),
            ),
        };
        let watcher = Watcher::new(
            codex,
            collateral_auction,
            collybus,
            limes,
            multicall2,
            multicall_batch_size,
            client.clone(),
            auctions,
            vaults,
            positions,
            rates,
            spots,
            processed_updates,
            instance_name.clone(),
        )
        .await;
        let liquidator = Liquidator::new(
            limes,
            collateral_auction,
            Some(multicall2),
            gas_boost,
            client.clone(),
            gas_escalator,
            bump_gas_delay,
            instance_name.clone(),
        )
        .await;

        Ok(Self {
            client,
            watcher,
            liquidator,
            last_block,
            instance_name: instance_name.clone(),
        })
    }

    pub async fn run(&mut self, fname: PathBuf, start_block: Option<u64>) -> Result<(), M> {
        // Create the initial list of borrowers from the start_block, if provided
        if let Some(start_block) = start_block {
            self.last_block = start_block.into();
        }

        let client = self.client.clone();
        let mut filter_id = client
            .new_filter(FilterKind::NewBlocks)
            .await
            .map_err(ContractError::MiddlewareError)?;

        let mut err_count = 0;
        let mut file: Option<std::fs::File> = None;

        let mut maybe_last_block_number: Option<u64> = None;

        let span = debug_span!("run", instance_name = self.instance_name.as_str());
        let _enter = span.enter();
        loop {
            sleep(Duration::from_secs(30)).await; // don't spin
            match client
                .get_filter_changes::<_, ethers_core::types::H256>(filter_id)
                .await
            {
                Ok(_results) => {
                    err_count = 0;
                    let block_number = self
                        .client
                        .get_block_number()
                        .await
                        .map_err(ContractError::MiddlewareError)?;

                    if let Some(last_block_number) = maybe_last_block_number {
                        if last_block_number == block_number.as_u64() {
                            trace!(last_block_number, "Skipping previously seen block");
                            continue;
                        }
                    }

                    maybe_last_block_number = Some(block_number.as_u64());
                    match self
                        .client
                        .get_block(block_number)
                        .await
                        .map_err(ContractError::MiddlewareError)?
                    {
                        Some(block) => {
                            let block_timestamp = block.timestamp.as_u64() as i64;
                            match SystemTime::now().duration_since(UNIX_EPOCH) {
                                Ok(current_time) => {
                                    info!(
                                        block_number = block_number.as_u64(),
                                        timestamp = block_timestamp,
                                        delay_seconds =
                                            current_time.as_secs() as i64 - block_timestamp,
                                        instance_name = self.instance_name.as_str(),
                                        "New block"
                                    );
                                }
                                Err(_) => {
                                    info!(
                                        block_number = block_number.as_u64(),
                                        timestamp = block_timestamp,
                                        instance_name = self.instance_name.as_str(),
                                        "New block"
                                    );
                                }
                            }
                        }
                        None => {
                            info!(
                                block_number = block_number.as_u64(),
                                instance_name = self.instance_name.as_str(),
                                "New block"
                            );
                        }
                    }

                    if block_number % 5 == 0.into() {
                        // on each new block we open a new file handler to dump our state.
                        // we should just have a database connection instead here...
                        file = Some(
                            std::fs::OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open(&fname)
                                .unwrap(),
                        );
                    }

                    // run the logic for this block
                    self.on_block(block_number).await?;

                    // update our last block
                    self.last_block = block_number;

                    // Log once every 10 blocks
                    if let Some(file) = file.take() {
                        self.log(file);
                    }
                }
                Err(_x) => {
                    err_count += 1;
                    if err_count == 10 {
                        return Err(ContractError::ProviderError(ProviderError::CustomError(
                            String::from("can't query filter"),
                        )));
                    }
                    filter_id = client
                        .new_filter(FilterKind::NewBlocks)
                        .await
                        .map_err(ContractError::MiddlewareError)?;
                }
            }
        }
    }

    #[instrument(skip(self), fields(self.instance_name))]
    pub async fn one_shot(&mut self) -> Result<(), M> {
        let block_number = self
            .client
            .get_block_number()
            .await
            .map_err(ContractError::MiddlewareError)?;
        return self.on_block(block_number).await;
    }

    /// Runs the liquidation business logic for the specified block
    #[instrument(skip(self), fields(self.instance_name))]
    async fn on_block(&mut self, block_number: U64) -> Result<(), M> {
        // Get the gas price - TODO: Replace with gas price oracle
        let gas_price = self
            .client
            .get_gas_price()
            .await
            .map_err(ContractError::MiddlewareError)?;

        // 1. Check if our transactions have been mined
        self.liquidator.remove_or_bump().await?;

        // 2. update our dataset with the new block's data
        self.watcher.sync(self.last_block, block_number).await?;

        // 3. trigger the auction for any undercollateralized borrowers
        self.liquidator
            .start_auctions(
                self.watcher.auctions.iter(),
                self.watcher.positions.iter(),
                &self.watcher.vaults,
                &self.watcher.rates,
                &self.watcher.spots,
                gas_price,
            )
            .await?;

        // 4. trigger redo for any auctions up for redo
        self.liquidator
            .redo_auctions(self.watcher.auctions.iter(), gas_price)
            .await?;

        Ok(())
    }

    fn log<W: Write>(&self, w: W) {
        serde_json::to_writer(
            w,
            &State {
                vaults: self.watcher.vaults.clone(),
                positions: self.watcher.positions.clone(),
                rates: self.watcher.rates.clone(),
                spots: self.watcher.spots.clone(),
                auctions: self.watcher.auctions.clone(),
                processed_updates: self.watcher.processed_updates.clone(),
                cached_positions: self.liquidator.cached_positions.clone(),
                last_block: self.last_block.as_u64(),
            },
        )
        .unwrap();
    }
}
