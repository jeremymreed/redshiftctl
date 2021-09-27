#!/usr/bin/env bash
# v1.0.0

cargo build --release

strip target/release/redshiftctl
install --mode=700 target/release/redshiftctl ~/bin

