FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1

RUN apt-get update && \
    apt-get install --assume-yes libx11-xcb-dev libgl1-mesa-dev xorg-dev openssl libssl-dev pkg-config python3 && \
    apt list --installed | grep ssl && \
    openssl version -d && \
    pkg-config --modversion libssl && \
    pkg-config --modversion openssl