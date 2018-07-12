#!/bin/sh

# Install form, svd2rust, rustfmt-nightly

rm -rf src

svd2rust --nightly -i resources/STM32F40x.svd

# Reformat compiler attributes removing unnecessary spaces
# Remove spaces from # [ attribute ] => #[attribute] and add \n
sed -i 's/\s*# \[ \([^]]*\) \]/\n#[\1]/g' lib.rs
# Remove spaces from # ! [ attribute ] and add \n
sed -i 's/\s*# ! \[ \([^]]*\) \]/#![\1]\n/g' lib.rs
sed -i 's/ \([()]\) */\1/g' lib.rs

form -i lib.rs -o src/ && rm lib.rs

# Use rustfmt to reformat to human readable format
#rustfmt src/*.rs
cargo fmt

# Test that build succeeds for target platform (ARM Cortex-M4)
xargo check --target thumbv7em-none-eabihf
