# Rust as the base image
FROM rust:1.67

# 1. Create a new empty shell project
RUN USER=root cargo new --bin substreams
WORKDIR /usr/src/app

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --target wasm32-unknown-unknown --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./ ./

# 5. Build for release.
RUN rm ./target/release/deps/substreams*
RUN make build

CMD ["make gui"]
