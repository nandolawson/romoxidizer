FROM rust:slim

# Install neded components
RUN rustup component add clippy rustfmt

# Install git & gpg
RUN apt update -y && \
    apt install -y git gpg