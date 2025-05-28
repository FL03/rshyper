# Author: FL03 <jo3mccain@icloud.com>
ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# STAGE 2: build
FROM builder-base AS builder
# declare some environment variables
ENV RUST_BACKTRACE=1 \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    CARGO_HOME=/usr/local/cargo
# setup the working directory
WORKDIR /app
# copy the source code
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --features full --workspace
