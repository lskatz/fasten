FROM rust:1.59.0-alpine3.14 as builder 

ARG SOFTWARE_VER="0.4"

LABEL base.image="alpine-3.14"
LABEL dockerfile.version="1"
LABEL software="Fasten"
LABEL software.version=$SOFTWARENAME_VER
LABEL description="Fastq file manipulation suite"
LABEL website="https://github.com/lskatz/fasten"
LABEL license="https://github.com/lskatz/fasten/LICENSE"
LABEL maintainer="Lee Katz"
LABEL maintainer.email="gzu2@cdc.gov"
LABEL maintainer2="John Phan"

RUN apk update
RUN apk add --no-cache \
        ca-certificates \
        build-base \
        linux-headers \
        git \
        openssl-dev \
        util-linux-dev \
        libseccomp-dev 

RUN mkdir -p /usr/src/app \
    && cd /usr/src/app \
    && git clone https://github.com/lskatz/fasten \
    && cd /usr/src/app/fasten \
RUN cd /usr/src/app/fasten && cargo build --release


# build final container

FROM alpine:3.14

COPY --from=builder /usr/src/app/fasten/target/release /usr/local/bin

