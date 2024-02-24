FROM rust:latest

WORKDIR /app

# Install deps
RUN apt update && apt install -y fuse3 libfuse3-dev

# Install rust tools
RUN rustup component add clippy
RUN rustup component add rustfmt

USER 0