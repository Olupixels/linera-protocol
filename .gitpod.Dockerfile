FROM gitpod/workspace-full

# Install Rust and dependencies
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
 && apt-get update && apt-get install -y pkg-config libssl-dev rocksdb librocksdb-dev

ENV PATH="/home/gitpod/.cargo/bin:${PATH}"
