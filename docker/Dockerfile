FROM rust    

RUN rustup target add thumbv7em-none-eabihf
RUN cargo install cargo-binutils
RUN rustup component add llvm-tools-preview     

WORKDIR /build

ENTRYPOINT ["./hardware-test.sh"]