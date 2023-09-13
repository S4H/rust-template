# syntax=docker/dockerfile:experimental

FROM rust:1.72.0-slim as builder

RUN --mount=type=cache,target=/var/cache/apt \
    --mount=type=cache,target=/var/lib/apt \
    apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config libssl-dev

WORKDIR /repo

ARG CARGO_TARGET_DIR=/build

RUN --mount=type=bind,source=.,target=/repo \
    --mount=type=cache,target=/build \
    --mount=type=tmpfs,target=/tmp \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --all-targets \
    && cargo install --path .
