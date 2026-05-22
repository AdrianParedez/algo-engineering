#!/usr/bin/env bash
set -euo pipefail

EVENTS="${PERF_EVENTS:-cycles,instructions,cache-references,cache-misses,branches,branch-misses}"
FILTER="${1:-}"

if ! command -v perf >/dev/null 2>&1; then
  echo "perf is required for counter collection" >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

perf stat -d -e "${EVENTS}" cargo bench --bench cache_locality ${FILTER}
