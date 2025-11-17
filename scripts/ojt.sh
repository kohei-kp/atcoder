#!/bin/bash
set -eu

if [ $# -ne 2 ]; then
    echo "Usage: ojt.sh <contest_number> <problem_letter>"
    exit 1
fi

DIR=$(cd $(dirname "$0") && pwd)

NUM=$1
LETTER=$2
CONTEST="abc${NUM}"
BIN="${CONTEST}/src/bin/${LETTER}.rs"
TESTDIR="${CONTEST}/test/${LETTER}"

# サンプルが存在しない場合、自動でダウンロード
if [ ! -d "$TESTDIR" ] || [ -z "$(ls -A "$TESTDIR")" ]; then
    echo "[INFO] Samples not found, downloading..."
    "$DIR/ojd.sh" "$NUM" "$LETTER"
fi

echo "[INFO] Testing $BIN"
PYTHONWARNINGS="ignore" oj t -d "$TESTDIR" -c "cargo run -p ${CONTEST} --bin ${LETTER}"
