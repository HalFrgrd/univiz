# Stage 1: Build the Rust binary
FROM rust:1.95-slim AS builder

WORKDIR /usr/src/univiz

# Copy dependencies manifest
COPY Cargo.toml Cargo.lock ./

# Create dummy source and build dependencies to cache them
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -f target/release/deps/univiz*

# Copy actual source code
COPY src ./src

# Build the actual binary
RUN cargo build --release

# Stage 2: Export the binary
FROM scratch AS exporter
COPY --from=builder /usr/src/univiz/target/release/univiz /univiz
