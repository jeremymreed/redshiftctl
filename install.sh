#!/usr/bin/env bash
# v0.1.0

cargo build --release
mv target/release/redshiftctl ~/bin

