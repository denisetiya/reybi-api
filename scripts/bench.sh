#!/usr/bin/env bash
# reybi-api benchmark
# Usage:
#   ./bench.sh                       # via host (compose must publish 3000)
#   ./bench.sh 127.0.0.1 3000        # direct host/port
# For sidecar mode (host can't reach container), use bench-sidecar.sh
set -euo pipefail

HOST="${1:-127.0.0.1}"
PORT="${2:-3000}"
BASE="http://${HOST}:${PORT}/v1"

# color helpers
g() { printf '\033[1;32m%s\033[0m\n' "$*"; }
y() { printf '\033[1;33m%s\033[0m\n' "$*"; }
b() { printf '\033[1;34m%s\033[0m\n' "$*"; }

# extract ms from curl -w output
timing() {
  local url="$1" label="$2" etag="${3:-}"
  local hdr=()
  [[ -n "$etag" ]] && hdr=(-H "If-None-Match: $etag")
  # 1 warmup + 5 measured
  curl -s -o /dev/null "${hdr[@]}" "$url" || true
  local total=0 min=999 max=0 n=0
  for i in 1 2 3 4 5; do
    local t
    t=$(curl -s -o /dev/null "${hdr[@]}" -w '%{time_total}' "$url")
    t_ms=$(awk -v t="$t" 'BEGIN{printf "%.2f", t*1000}')
    total=$(awk -v a="$total" -v b="$t_ms" 'BEGIN{print a+b}')
    awk -v m="$min" -v b="$t_ms" 'BEGIN{exit !(b<m)' && min="$t_ms"
    awk -v m="$max" -v b="$t_ms" 'BEGIN{exit !(b>m)' && max="$t_ms"
    n=$((n+1))
  done
  local avg
  avg=$(awk -v t="$total" -v n="$n" 'BEGIN{printf "%.2f", t/n}')
  printf "  %-30s avg=%4sms  min=%4sms  max=%4sms\n" "$label" "$avg" "$min" "$max"
}

echo
b "=== reybi-api benchmark ==="
echo "target: $BASE"
echo

# ----- COLD vs HOT cache -----
b "[A] Products (cold vs hot cache)"
echo "  -- cold (first request, populates cache) --"
curl -s -o /dev/null -w '  cold   %{time_total}s  http=%{http_code}\n' "${BASE}/products"
echo "  -- hot (subsequent, cached) --"
timing "${BASE}/products" "products  (hot)"

b "[B] Banners"
timing "${BASE}/banners" "banners"

b "[C] Articles"
timing "${BASE}/articles" "articles  (list)"

# ----- ETag / 304 -----
b "[D] ETag negotiation"
ETAG=$(curl -sI "${BASE}/banners" | awk -F': ' 'tolower($1)=="etag"{gsub(/\r/,"",$2);print $2}')
if [[ -n "$ETAG" ]]; then
  y "  ETag: $ETAG"
  echo "  -- with If-None-Match --"
  curl -s -o /dev/null -w '  304 check  %{time_total}s  http=%{http_code}\n' \
    -H "If-None-Match: $ETAG" "${BASE}/banners"
  timing "${BASE}/banners" "banners (304)" "$ETAG"
else
  y "  (no ETag header returned)"
fi

# ----- Latency under small sequential load -----
b "[E] Sequential load (50 reqs, banners)"
T1=$(date +%s%N)
for i in $(seq 1 50); do curl -s -o /dev/null "${BASE}/banners"; done
T2=$(date +%s%N)
DUR_MS=$(( (T2 - T1) / 1000000 ))
RPS=$(( 50000 / DUR_MS ))
printf "  50 reqs in %sms  →  %s req/s\n" "$DUR_MS" "$RPS"

# ----- Redis cache keys -----
b "[F] Redis cache keys"
if command -v redis-cli >/dev/null 2>&1; then
  REDIS_HOST="${REDIS_HOST:-infra-redis}"
  REDIS_PORT="${REDIS_PORT:-6379}"
  if [[ -n "${REDIS_PASSWORD:-}" ]]; then
    keys=$(redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" -a "$REDIS_PASSWORD" \
      --no-auth-warning KEYS 'reybi:v1:*' 2>/dev/null | head -20)
  else
    keys=$(redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" KEYS 'reybi:v1:*' 2>/dev/null | head -20)
  fi
  if [[ -n "$keys" ]]; then
    echo "$keys" | sed 's/^/  /'
  else
    echo "  (no cache keys — cache disabled or cold)"
  fi
else
  echo "  (redis-cli not installed)"
fi

echo
g "=== done ==="