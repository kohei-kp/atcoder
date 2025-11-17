#!/bin/bash
set -eu

if [ $# -ne 2 ]; then
    echo "Usage: ojs.sh <contest_number> <problem_letter>"
    exit 1
fi

NUM=$1
LETTER=$2
CONTEST="abc${NUM}"
SRC="${CONTEST}/src/bin/${LETTER}.rs"
URL="https://atcoder.jp/contests/${CONTEST}/tasks/${CONTEST}_${LETTER}"

echo "[INFO] Submitting $SRC"
PYTHONWARNINGS="ignore" oj s "$URL" "$SRC"
