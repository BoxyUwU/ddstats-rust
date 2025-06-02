FROM rustembedded/cross:x86_64-unknown-linux-gnu

RUN apt-get update && \
    apt-get install libx11-xcb-dev libgl1-mesa-dev xorg-dev openssl libssl-dev pkg-config python3
    # apt list --installed | grep ssl && \
    # openssl version -d && \
    # pkg-config --modversion libssl && \
    # pkg-config --modversion openssl