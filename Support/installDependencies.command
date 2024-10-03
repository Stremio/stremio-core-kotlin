#!/bin/bash
# make sure that you installed brew and rustup

cargo install cargo-lipo
rustup toolchain install nightly
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
rustup component add rust-src --toolchain nightly-aarch64-apple-darwin
brew install cbindgen
