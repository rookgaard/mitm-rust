#!/bin/sh

cargo build --example=$1 --release
strip -s target/release/examples/$1
