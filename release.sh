#!/bin/sh

cargo build --release
strip -s target/release/mitm-rust
