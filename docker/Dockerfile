FROM rust    

RUN rustup target add thumbv7em-none-eabihf
RUN apt-get update && apt-get install -y gcc-arm-none-eabi binutils-arm-none-eabi libnewlib-arm-none-eabi     

WORKDIR /build

ENTRYPOINT ["./hardware-test.sh"]