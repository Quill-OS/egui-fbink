#!/bin/bash

# https://users.rust-lang.org/t/how-to-install-armv7-unknown-linux-musleabihf/82395/12
# rustup target add armv7-unknown-linux-musleabihf
CC=clang cargo build -vv --release --target=armv7-unknown-linux-musleabihf

# It's in target/armv7-unknown-linux-musleabihf/release/egui_template, scp it and run

scp target/armv7-unknown-linux-musleabihf/release/egui_template root@192.168.2.2:/