#!/usr/bin/env bash
# Run python bench via sidecar container.
# Usage: REDIS_PASSWORD=*** ./bench-sidecar.sh
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"

docker run --rm \
  --network db-shared \
  --network reybi-net \
  -e REDIS_PASSWORD="${REDIS_PASSWORD:-}" \
  -e BASE="http://reybi-api:3000/v1" \
  -v "$HERE/bench.py:/bench.py:ro" \
  python:3.12-alpine sh -c '
    apk add -q redis >/dev/null 2>&1
    pip install -q requests >/dev/null 2>&1 || true
    python3 /bench.py
  '