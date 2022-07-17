# syntax=docker/dockerfile:1
FROM rust:slim-bullseye as builder
WORKDIR /build
COPY ./ ./
RUN apt-get update && \
apt-get install --yes libssl-dev pkg-config && \
cargo build --release

FROM debian:bullseye-slim
LABEL project.repo="https://github.com/wlieberz/release-notifier-rust"
WORKDIR /app
COPY --from=builder /build/target/release/release-notifier-rust ./

RUN apt-get update && \
apt-get upgrade --yes && \
groupadd worker && \
useradd --create-home --shell /bin/bash -g worker worker && \
apt-get clean && \
chmod 775 /app/release-notifier-rust

USER worker

ENTRYPOINT ["./release-notifier-rust"]