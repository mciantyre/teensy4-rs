# This provides a minimal Docker image that you can use for your
# Teensy 4 development. It should be useful with the teensy4-rs-template.

FROM rust    

RUN rustup target add thumbv7em-none-eabihf && \
    rustup component add llvm-tools-preview && \
    cargo install cargo-binutils

WORKDIR /build

ENTRYPOINT ["cargo objcopy --release -- -O ihex"]
