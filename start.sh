cargo build --release

# Run it with 
./target/release/main \
    --config ./addrs.json \
    --private-key ./private_key \
    --url wss://eth-goerli.alchemyapi.io/v2/fe2m2g2zR0VSdfeDvvTlFCVgNycVTybN \
    --interval 7000 \
    --file state.json \