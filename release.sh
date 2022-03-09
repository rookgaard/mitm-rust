#!/bin/sh

cargo build --release
strip -s target/release/mitm-rust
upx -9 target/release/mitm-rust
