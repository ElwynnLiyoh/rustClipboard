#!/bin/bash

cd "$(dirname "$0")"
cargo build --release
./target/release/clipboard