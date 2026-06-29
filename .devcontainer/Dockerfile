FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       jq unzip zip python3 python3-pip shellcheck git-lfs \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-nextest cargo-deny cargo-audit cargo-llvm-cov cargo-machete --locked || true
