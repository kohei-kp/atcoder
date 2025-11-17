#!/bin/bash
# scripts/ojd
set -eu

if [ $# -ne 2 ]; then
  echo "Usage: ojd <contest_number> <problem_letter>"
  exit 1
fi

NUM=$1
LETTER=$2
CONTEST="abc${NUM}"
DEST="${CONTEST}/test/${LETTER}"
mkdir -p "$DEST"
URL="https://atcoder.jp/contests/${CONTEST}/tasks/${CONTEST}_${LETTER}"
echo "[INFO] Downloading samples to $DEST"
oj d "$URL" -d "$DEST"
