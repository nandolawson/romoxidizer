FROM rust:slim

# Install neded components
RUN rustup component add clippy rustfmt && \
    rustup target add x86_64-pc-windows-gnu && \
    cargo install cargo-edit just

# Install git & gpg
RUN apt update -y && \
    apt install -y git gpg mingw-w64