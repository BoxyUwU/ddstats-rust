FROM ghcr.io/rust-cross/rust-musl-cross:x86_64-musl

RUN rustup update nightly && \
    rustup target add --toolchain nightly x86_64-unknown-linux-musl && \
    rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt

RUN apt-get update && \
    apt-get install --assume-yes libx11-xcb-dev libgl1-mesa-dev xorg-dev openssl libssl-dev pkg-config python3 libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

WORKDIR /src