#!/bin/bash
# run.sh
# Usage: ./run.sh b
set -eu

CONTEST=$1
BIN=$2
cargo run -p $CONTEST --bin "$BIN"
