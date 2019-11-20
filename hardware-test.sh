#!/usr/bin/env bash

# Pass in the example program to run from teensy4-examples.
# If the teensy_loader_cli is available, we will use it to
# automatically deploy the program to a connected Teensy4.

cargo build --release -p teensy4-examples --bin $1 && \
    mkdir -p out && \
    cp target/thumbv7em-none-eabihf/release/$1 out/$1 && \
    rust-objdump -d -S -C out/$1 > out/$1.lst && \
    rust-objdump -t -C out/$1 > out/$1.sym && \
    rust-objcopy -O ihex -R .eeprom out/$1 out/$1.hex

if [ -x "$(command -v teensy_loader_cli)" ]; then
    teensy_loader_cli --mcu=TEENSY40 -w -v out/$1.hex
fi