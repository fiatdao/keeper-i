# Keeper Service

Liquidates undercollateralized positions.

This service altruistically calls the `Limes.liquidate` function for any
position that is underwater, trigerring an auction for that position.

## CLI

```
Usage: ./keeper-service [OPTIONS]

Optional arguments:
  -h, --help
  -c, --config CONFIG        path to json file with the contract addresses
  -u, --url URL              the Ethereum node endpoint (HTTP or WS) (default: http://localhost:8545)
  -C, --chain-id CHAIN-ID    chain id (default: 1)
  -p, --private-key PRIVATE-KEY
                             path to your private key
  -i, --interval INTERVAL    polling interval (ms) (default: 1000)
  -f, --file FILE            the file to be used for persistence (default: data.json)
  -m, --min-ratio MIN-RATIO  the minimum ratio (collateral/debt) to trigger liquidation, percents (default: 110)
  -s, --start-block START-BLOCK
                             the block to start watching from
```

Your contracts' `--config` file should be in the following format where:
 * `Codex` is the address of the Codex
 * `Multicall` is the address of the Multicall (https://github.com/makerdao/multicall)

The `--private-key` _must not_ have a `0x` prefix. Set the `interval` to 15s for mainnet.

## Building and Running

```sh
# development
./debug.sh
# release
./start.sh
```

## How it Works

On each block:
1. Bumps the gas price of all of our pending transactions
2. Updates our dataset of positions & liquidation auctions with the new block's data
3. Trigger the auction for any undercollateralized positions

Take this keeper service for a spin by [running it in a test environment](TESTNET.md).