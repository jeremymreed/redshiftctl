#!/usr/bin/env bash
# v1.0.0

cargo build --release
mv target/release/redshiftctl ~/bin

