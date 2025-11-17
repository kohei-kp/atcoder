#!/bin/bash
set -eu

if [ $# -ne 2 ]; then
    echo "Usage: ojt.sh <contest_number> <problem_letter>"
    exit 1
fi

NUM=$1
LETTER=$2
CONTEST="abc${NUM}"
BIN="${CONTEST}/src/bin/${LETTER}.rs"
TESTDIR="${CONTEST}/test/${LETTER}"

if [ ! -d "$TESTDIR" ]; then
    echo "[INFO] Samples not found, downloading..."
    ./ojd.sh "$NUM" "$LETTER"
fi

echo "[INFO] Testing $BIN"
PYTHONWARNINGS="ignore" oj t -d "$TESTDIR" -c "cargo run -p ${CONTEST} --bin ${LETTER}"
