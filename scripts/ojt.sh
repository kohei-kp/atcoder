#!/bin/bash
set -eu

if [ $# -lt 2 ]; then
    echo "Usage: ojt.sh <contest_number> <problem_letter>"
    exit 1
fi

DIR=$(cd $(dirname "$0") && pwd)

NUM=$1
LETTER=$2
CONTEST="abc${NUM}"
BIN="${CONTEST}/src/bin/${LETTER}.rs"
TESTDIR="${CONTEST}/test/${LETTER}"

# 第3引数以降で -e フラグがあれば設定
E_OPT=""
for arg in "${@:3}"; do
    if [ "$arg" = "-e" ]; then
        E_OPT="-e 0.1e-5"
    fi
done

# サンプルが存在しない場合、自動でダウンロード
if [ ! -d "$TESTDIR" ] || [ -z "$(ls -A "$TESTDIR")" ]; then
    echo "[INFO] Samples not found, downloading..."
    "$DIR/ojd.sh" "$NUM" "$LETTER"
fi

echo "[INFO] Testing $BIN"
PYTHONWARNINGS="ignore" oj t $E_OPT -d "$TESTDIR" -c "cargo run -p ${CONTEST} --bin ${LETTER}"
