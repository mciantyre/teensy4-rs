#!/usr/bin/env bash
#
# Script to auto-generate the imxrt1062 super PAC. This might
# take some time to complete.

set -e
rm -Rf imxrt1062
svd2rust -i MIMXRT1062.xml
cargo new --lib imxrt1062 --vcs none
mv build.rs imxrt1062
mv device.x imxrt1062
form -i lib.rs -o imxrt1062/src/ && rm lib.rs

cd imxrt1062
cat >> Cargo.toml <<EOF
bare-metal = "0.2.0"
cortex-m = "0.5.8"
vcell = "0.1.0"
EOF
cargo fmt --all
cd ..