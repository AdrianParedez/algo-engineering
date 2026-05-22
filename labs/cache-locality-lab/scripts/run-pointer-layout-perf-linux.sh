#!/usr/bin/env bash
set -euo pipefail

VARIANT="${1:-split_soa_index_chase}"
ROUNDS="${ROUNDS:-128}"
WARMUP_ROUNDS="${WARMUP_ROUNDS:-16}"
EVENTS="${PERF_EVENTS:-cycles,instructions,cache-references,cache-misses,branches,branch-misses}"
REPORT_DIR="${REPORT_DIR:-reports/pointer-layout}"
OUTPUT_PATH="${PERF_OUTPUT:-${REPORT_DIR}/${VARIANT}-perf-stat.txt}"

mkdir -p "${REPORT_DIR}"

cargo build --release --bin pointer_layout_probe

{
  echo "# perf stat"
  echo "variant=${VARIANT}"
  echo "rounds=${ROUNDS}"
  echo "warmup_rounds=${WARMUP_ROUNDS}"
  echo "events=${EVENTS}"
  echo
  perf stat -d -e "${EVENTS}" \
    target/release/pointer_layout_probe \
    --variant "${VARIANT}" \
    --rounds "${ROUNDS}" \
    --warmup-rounds "${WARMUP_ROUNDS}"
} 2>&1 | tee "${OUTPUT_PATH}"
