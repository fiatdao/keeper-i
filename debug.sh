#!/usr/bin/env bash
set -o errexit
set -o pipefail

if [ -f .env ]; then
  set -o allexport; source .env; set +o allexport
fi

if [ -z "$ALCHEMY_API_KEY" ]; then
  echo "ALCHEMY_API_KEY is undefined in .env";
  exit 1;
fi

set -o nounset

RUST_BACKTRACE=1 RUST_LOG="main,keeper=debug" cargo run -- \
    --config ./addrs.json \
    --private-key ./private_key \
    --url wss://eth-goerli.alchemyapi.io/v2/$ALCHEMY_API_KEY \
    --interval 7000 \
    --start-block 6142980 \
    --file state.json \
    --instance-name goerli \