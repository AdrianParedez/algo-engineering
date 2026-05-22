ARG RUST_VERSION=1.95.0

FROM rust:${RUST_VERSION}-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        bash \
        build-essential \
        ca-certificates \
        jq \
        linux-perf \
        procps \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

ENV CARGO_HOME=/usr/local/cargo \
    CARGO_TERM_COLOR=always

CMD ["bash"]
