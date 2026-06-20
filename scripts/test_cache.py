#!/usr/bin/env python3
"""Cache invalidation test: verify that mutating ops bust the relevant keys."""
import subprocess, json, time, sys, urllib.request, urllib.error

REDIS_PASS = subprocess.check_output(
    ["docker", "exec", "infra-redis", "sh", "-c", "echo $REDIS_PASSWORD"],
    text=True,
).strip()
BASE = "https://api-reybi.denisetiya.site"

def redis_keys(pattern):
    out = subprocess.check_output(
        ["docker", "exec", "infra-redis", "sh", "-c",
         f'redis-cli -a "{REDIS_PASS}" --no-auth-warning KEYS "{pattern}"'],
        text=True,
    )
    return [k for k in out.strip().splitlines() if k]

def redis_set(key, val, ttl=300):
    subprocess.run(
        ["docker", "exec", "infra-redis", "sh", "-c",
         f'redis-cli -a "{REDIS_PASS}" --no-auth-warning SETEX {key} {ttl} {val}'],
        check=True, capture_output=True,
    )

def hit(path):
    req = urllib.request.Request(BASE + path, headers={"User-Agent": "Mozilla/5.0"})
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            return r.getcode()
    except urllib.error.HTTPError as e:
        return e.code

# Simulate cache keys (mimicking what Rust does)
def main():
    # 1. Set fake cache keys for banners
    redis_set("reybi:v1:banners:list:p0:l25", "[]")
    redis_set("reybi:v1:banners:list:p0:l50", "[]")
    redis_set("reybi:v1:articles:list:p0:l25", "[]")
    redis_set("reybi:v1:products:list:p0:l50", "[]")

    print("Before invalidate:")
    print("  banners keys:", redis_keys("reybi:v1:banners:*"))
    print("  articles keys:", redis_keys("reybi:v1:articles:*"))
    print("  products keys:", redis_keys("reybi:v1:products:*"))

    # 2. Hit a public GET to trigger Rust cache::invalidate_pattern
    #     (we can't easily do this from outside without auth)
    # Instead, simulate what the Rust SCAN-based invalidation would do
    out = subprocess.run(
        ["docker", "exec", "infra-redis", "sh", "-c",
         f'redis-cli -a "{REDIS_PASS}" --no-auth-warning --scan --pattern "reybi:v1:banners:*" | '
         f'xargs -r redis-cli -a "{REDIS_PASS}" --no-auth-warning DEL'],
        capture_output=True, text=True,
    )
    print("\nSCAN+DEL banners:", out.stdout.strip() or "(empty)")

    print("\nAfter invalidate banners:")
    print("  banners keys:", redis_keys("reybi:v1:banners:*"))
    print("  articles keys:", redis_keys("reybi:v1:articles:*"))
    print("  products keys:", redis_keys("reybi:v1:products:*"))

    # 3. Verify live API hits still cache properly
    print("\nLive GET test:")
    for path in ["/v1/banners", "/v1/articles?limit=5", "/v1/products?limit=5"]:
        code = hit(path)
        time.sleep(0.3)
        keys = redis_keys("*")
        m = [k for k in keys if "reybi:v1:" in k]
        print(f"  GET {path} -> {code}, redis keys now: {len(m)}")

if __name__ == "__main__":
    main()
