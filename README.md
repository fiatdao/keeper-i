# Keeper Service

Triggers and maintains auctions for liquidated positions.

This service altruistically calls the `Limes.liquidate` function for any
position that is undercollateralized, triggering an auction for that position.

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
  -s, --start-block START-BLOCK
                             the block to start watching from
```

Your contracts' `--config` file should be in the following format where:
 * `Codex` is the address of Codex (see fiatdao/fiat)
 * `CollateralAuction` is the address of CollateralAuction / NoLossCollateralAuction (see fiatdao/fiat)
 * `Collybus` is the address of Collybus (see fiatdao/fiat)
 * `Limes` is the address of Limes (see fiatdao/fiat)
 * `Multicall` is the address of Multicall (https://github.com/makerdao/multicall)

The `--private-key` _must not_ have a `0x` prefix. Set the `interval` to 15s for mainnet.

## Building and Running

```sh
# development
./dev.sh
# release locally
./start.sh
```

## How it Works

On each block:
1. Bumps the gas price of all of our pending transactions
2. Updates our dataset of positions & liquidation auctions with the new block's data
3. Trigger the auction for any undercollateralized positions
