FROM rust    

RUN rustup target add thumbv7em-none-eabihf && rustup component add llvm-tools-preview
RUN apt-get update && apt-get install -y jq

WORKDIR /build

ENTRYPOINT ["./hardware-test.sh"]
