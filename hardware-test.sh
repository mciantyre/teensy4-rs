#!/usr/bin/env bash

# Pass in the example program to run from teensy4-examples.
# If the teensy_loader_cli is available, we will use it to
# automatically deploy the program to a connected Teensy4.
#
# By default, build in release mode. Pass in --debug to build
# without optimizations.

set -e

BUILD_MODE_FLAG="--release"
BUILD_MODE="release"

if [ "$2" = "--debug" ]; then
    BUILD_MODE_FLAG=""
    BUILD_MODE="debug"
elif [ "$2" != "" ]; then 
    echo "Unknown option '$2'"
    exit 1
fi

cargo build $BUILD_MODE_FLAG -p teensy4-examples --bin $1
rm -Rf out
mkdir out
cp target/thumbv7em-none-eabihf/$BUILD_MODE/$1 out/$1
arm-none-eabi-objdump -d -S -C out/$1 > out/$1.lst
arm-none-eabi-objdump -t -C out/$1 > out/$1.sym
arm-none-eabi-objcopy -O ihex -R .eeprom out/$1 out/$1.hex

if [ -x "$(command -v teensy_loader_cli)" ]; then
    teensy_loader_cli --mcu=TEENSY40 -w -v out/$1.hex
fi