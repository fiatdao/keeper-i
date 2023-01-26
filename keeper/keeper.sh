#!/usr/bin/env bash
set -o errexit
set -o pipefail

if [ -f /usr/bin/.env ]; then
  set -o allexport; source /usr/bin/.env; set +o allexport
fi

if [ -z "$ALCHEMY_API_KEY" ]; then
  echo "ALCHEMY_API_KEY is undefined in .env";
  exit 1;
fi

if [ -z "$NETWORK" ]; then
  echo "NETWORK is undefined in .env";
  exit 1;
fi

set -o nounset

export RUST_BACKTRACE=1
export RUST_LOG=info

# Run it with
exec /usr/bin/main \
    --config /usr/bin/deployment-$NETWORK.json \
    --private-key /usr/bin/private_key \
    --url wss://eth-$NETWORK.alchemyapi.io/v2/$ALCHEMY_API_KEY \
    --chain-id 5 \
    --interval 7000 \
    --start-block 6142980 \
    --file /usr/bin/data/state.json \
    --instance-name $NETWORK \
