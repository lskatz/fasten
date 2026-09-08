FROM rust:bookworm AS builder 

ARG SOFTWARE_VER="0.9.10"

LABEL base.image="debian:bookworm-slim"
LABEL dockerfile.version="1"
LABEL software="Fasten"
LABEL software.version="${SOFTWARE_VER}"
LABEL description="Fastq file manipulation suite"
LABEL website="https://github.com/lskatz/fasten"
LABEL license="https://github.com/lskatz/fasten/LICENSE"
LABEL maintainer="Lee Katz"
LABEL maintainer.email="gzu2@cdc.gov"
LABEL maintainer2="John Phan"
LABEL tag="${SOFTWARE_VER}"

RUN apt-get update && apt-get install -y \
        ca-certificates \
        git \
        bc \
        libcurl4-openssl-dev \
        libseccomp-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app/fasten
COPY . .

RUN cargo build && cargo build --release

RUN (set -ex; for i in tests/fasten*.sh; do bash $i; done;)

# build final container

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/fasten/target/release/fasten_* /usr/local/bin/

