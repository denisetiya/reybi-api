#!/usr/bin/env python3
"""reybi-api full route smoke-test. Exit 0 if all expected codes match."""
import sys, json, time, urllib.request, urllib.error
from typing import Optional

BASE = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:3000"
STUB_TOKEN = "stub.bearer.token"
UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36"

# (method, path, payload, protected) — expected 200/200/200 if public GET; 401 if protected
ROUTES = [
    # public GETs
    ("GET", "/v1/products?limit=1", None, False),
    ("GET", "/v1/banners", None, False),
    ("GET", "/v1/articles?limit=1", None, False),
    # auth (public) — both need Firebase bearer, return 401 without
    ("POST", "/v1/auth/register", {"email": "x@x.com"}, False),
    ("POST", "/v1/auth", {"email": "x@x.com"}, False),
    ("POST", "/v1/auth/reset-password", {"email": "x@x.com"}, False),
    # user-protected writes
    ("POST", "/v1/products/create", {"title": "x", "price": 0, "stock": 0}, True),
    ("POST", "/v1/carts/user/u1", {"productId": "p1", "qty": 1}, True),
    ("DELETE", "/v1/carts/item/i1", None, True),
    ("POST", "/v1/orders/user/u1", {"items": []}, True),
    ("DELETE", "/v1/orders/o1", None, True),
    ("POST", "/v1/addresses/user/u1", {"line1": "x"}, True),
    ("PUT", "/v1/addresses/user/u1", {"line1": "x"}, True),
    ("POST", "/v1/reviews", {"productId": "p1", "rating": 5}, True),
    ("PUT", "/v1/reviews/r1", {"rating": 5}, True),
    ("POST", "/v1/deposites", {"amount": 100}, True),
    ("POST", "/v1/payments/snap", {"orderId": "o1"}, True),
    # user-protected reads
    ("GET", "/v1/profile/test@example.com", None, True),
    ("GET", "/v1/carts/user/u1", None, True),
    ("GET", "/v1/addresses/user/u1", None, True),
    ("GET", "/v1/orders/user/u1", None, True),
    ("GET", "/v1/deposites/user/u1", None, True),
    # admin-protected
    ("POST", "/v1/banners/create", {"image": "x", "title": "x", "type": "promo"}, True),
    ("PUT", "/v1/banners/b1", {"title": "x"}, True),
    ("DELETE", "/v1/banners/b1", None, True),
    ("POST", "/v1/articles/create", {"title": "x", "content": "x", "thumbnail": "x"}, True),
    ("PUT", "/v1/articles/a1", {"title": "x"}, True),
    ("DELETE", "/v1/articles/a1", None, True),
    ("POST", "/v1/products/variant/p1", {"name": "v", "price": 0}, True),
    ("PUT", "/v1/products/p1", {"title": "x"}, True),
    ("DELETE", "/v1/products/p1", None, True),
    ("POST", "/v1/landfills/create", {"name": "x", "lat": 0, "lng": 0}, True),
    ("PUT", "/v1/landfills/l1", {"name": "x"}, True),
    ("DELETE", "/v1/landfills/l1", None, True),
    ("POST", "/v1/trash/type", {"name": "x"}, True),
    ("PUT", "/v1/trash/type/t1", {"name": "x"}, True),
    ("DELETE", "/v1/trash/type/t1", None, True),
    ("GET", "/v1/landfills", None, True),
    ("GET", "/v1/orders", None, True),
    ("GET", "/v1/deposites", None, True),
    ("GET", "/v1/sallers/products/u1", None, True),
    ("GET", "/v1/trash/types", None, True),
]

def hit(method, path, payload, prot):
    url = BASE + path
    data = json.dumps(payload).encode() if payload else b""
    h = {"User-Agent": UA, "Content-Type": "application/json"}
    if prot: h["Authorization"] = "Bearer " + STUB_TOKEN
    req = urllib.request.Request(url, data=data if payload or method != "GET" else None, headers=h, method=method)
    t0 = time.perf_counter()
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            return r.getcode(), (time.perf_counter() - t0) * 1000, ""
    except urllib.error.HTTPError as e:
        return e.code, (time.perf_counter() - t0) * 1000, e.read(120).decode("utf-8", errors="replace")
    except Exception as e:
        return -1, (time.perf_counter() - t0) * 1000, repr(e)[:120]

def get_first_id(opener, path):
    try:
        with opener.open(BASE + path, timeout=10) as r:
            d = json.load(r)
        return d["content"][0]["id"] if d.get("content") else None
    except Exception:
        return None

def main():
    opener = urllib.request.build_opener()
    opener.addheaders = [("User-Agent", UA)]
    pid = get_first_id(opener, "/v1/products?limit=1")
    aid = get_first_id(opener, "/v1/articles?limit=1")
    print(f"base={BASE} pid={pid} aid={aid}\n")

    results = []
    for method, path, payload, prot in ROUTES:
        code, ms, body = hit(method, path, payload, prot)
        if prot:
            ok = code == 401
            exp = "401"
        elif method == "GET":
            ok = code == 200
            exp = "200"
        else:  # public POST
            ok = code in (200, 401, 422)
            exp = "200/401/422"
        mark = "OK " if ok else "FAIL"
        print(f"  [{mark}] {method:6s} {code:3d} {ms:6.1f}ms {path:50s} (exp {exp})")
        if not ok: print(f"           body: {body[:160]}")
        results.append(ok)

    # dynamic product/article detail
    if pid:
        code, ms, body = hit("GET", f"/v1/products/{pid}", None, False)
        ok = code == 200
        print(f"  [{'OK ' if ok else 'FAIL'}] GET     {code:3d} {ms:6.1f}ms /v1/products/{pid}")
        results.append(ok)
    if aid:
        code, ms, body = hit("GET", f"/v1/articles/{aid}", None, False)
        ok = code == 200
        print(f"  [{'OK ' if ok else 'FAIL'}] GET     {code:3d} {ms:6.1f}ms /v1/articles/{aid}")
        results.append(ok)

    total = len(results)
    passed = sum(results)
    print(f"\nPASS {passed}/{total}")
    sys.exit(0 if passed == total else 1)

if __name__ == "__main__":
    main()
