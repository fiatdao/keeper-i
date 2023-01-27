FROM amazonlinux:latest AS builder
# install OS deps
RUN yum install gcc openssl-devel -y
# install rust compiler
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# build dependencies - they're not changed frequently
WORKDIR /usr/src/
COPY Cargo.* ./
RUN mkdir -p src/bin \
    && echo "//" > src/lib.rs \
    && echo "fn main() {}" > src/bin/main.rs \
    && ~/.cargo/bin/cargo build --release

# copy sources and build
COPY abis abis
COPY build.rs .
COPY src src

RUN ~/.cargo/bin/cargo build --release

FROM amazonlinux:latest AS keeper-trigger-auctions
COPY --from=builder /usr/src/target/release/main /usr/bin
COPY keeper.sh /usr/bin
COPY .env /usr/bin
COPY private_key /usr/bin
COPY ./lib/changelog-i/deployment/deployment-goerli.json /usr/bin
COPY ./lib/changelog-i/deployment/deployment-mainnet.json /usr/bin
RUN mkdir -p /usr/bin/data
COPY state.json /usr/bin/data

CMD ["/usr/bin/keeper.sh"]
