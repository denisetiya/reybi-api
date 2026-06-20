#!/usr/bin/env python3
"""reybi-api benchmark — minimal, accurate, docker-sidecar friendly."""
import json
import os
import statistics
import subprocess
import sys
import time
import urllib.request
from concurrent.futures import ThreadPoolExecutor

BASE = os.environ.get("BASE", "http://reybi-api:3000/v1")
REDIS_PASSWORD = os.environ.get("REDIS_PASSWORD", "")
N = 10  # samples per endpoint
CONCURRENCY = 5


def time_get(url, headers=None):
    t0 = time.perf_counter()
    req = urllib.request.Request(url, headers=headers or {})
    try:
        with urllib.request.urlopen(req, timeout=5) as r:
            r.read()
            return (time.perf_counter() - t0) * 1000, r.status
    except urllib.error.HTTPError as e:
        return (time.perf_counter() - t0) * 1000, e.code


def bench(path, label, etag=None, n=N):
    url = BASE + path
    hdrs = {"If-None-Match": etag} if etag else {}
    # warmup
    time_get(url, hdrs)
    # measure
    times = []
    for _ in range(n):
        ms, code = time_get(url, hdrs)
        times.append(ms)
    times.sort()
    p50 = statistics.median(times)
    p95 = times[int(n * 0.95) - 1]
    avg = statistics.mean(times)
    mn = times[0]
    mx = times[-1]
    print(f"  {label:30s} p50={p50:5.1f}ms  p95={p95:5.1f}ms  avg={avg:5.1f}ms  range={mn:4.1f}-{mx:5.1f}ms")


def seq_load(path, n=50):
    url = BASE + path
    t0 = time.perf_counter()
    for _ in range(n):
        try:
            urllib.request.urlopen(url, timeout=5).read()
        except Exception:
            pass
    dur = time.perf_counter() - t0
    print(f"  sequential {path:20s} {n} reqs in {dur*1000:.0f}ms  →  {n/dur:.0f} req/s")


def get_etag(path):
    req = urllib.request.Request(BASE + path, method="HEAD")
    try:
        with urllib.request.urlopen(req, timeout=5) as r:
            return r.headers.get("etag")
    except Exception:
        return None


def redis_keys():
    if not REDIS_PASSWORD:
        return
    cmd = [
        "redis-cli", "-h", "infra-redis", "-p", "6379",
        "-a", REDIS_PASSWORD, "--no-auth-warning",
        "KEYS", "reybi:v1:*",
    ]
    try:
        out = subprocess.check_output(cmd, timeout=5, stderr=subprocess.DEVNULL).decode()
        keys = [k for k in out.strip().split("\n") if k]
        print(f"\n[F] Redis cache keys ({len(keys)}):")
        for k in keys[:15]:
            print(f"    {k}")
        if len(keys) > 15:
            print(f"    ... +{len(keys)-15} more")
    except Exception as e:
        print(f"\n[F] redis-cli failed: {e}")


def main():
    print(f"=== reybi-api benchmark ===\nBASE={BASE}\n")

    print("[A] Products (cached)")
    bench("/products", "products (cache HIT)", n=N)

    print("\n[B] Banners (cached)")
    bench("/banners", "banners (cache HIT)", n=N)

    print("\n[C] Articles (cached)")
    bench("/articles", "articles (cache HIT)", n=N)

    print("\n[D] ETag negotiation")
    etag = get_etag("/banners")
    print(f"  ETag: {etag}")
    if etag:
        bench("/banners", "banners (304)", etag=etag, n=N)

    print("\n[E] Sequential load")
    seq_load("/banners", 100)
    seq_load("/products", 100)

    redis_keys()

    print("\n=== done ===")


if __name__ == "__main__":
    main()