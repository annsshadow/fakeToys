#!/usr/bin/env python3
"""Fix router(pool) arity mismatch introduced by realize_one.py.

The mechanical realization rewired `pub fn router(pool: Pool)` to call an inner
routes function (e.g. `query_express_router(pool)`), but that inner function is
defined with 0 arguments. This script rewrites the call so the pool is supplied
via an `Extension` layer instead:

    router(pool) { inner(pool) }   ->   router(pool) { inner().layer(Extension(pool)) }

This is safe for both cases:
  * handlers that DO take Extension<Pool>  -> they receive the pool
  * handlers that do NOT use the pool       -> Extension layer is harmless

The inner function is left untouched (minimal, low-risk change).
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

ROUTER_RE = re.compile(r"pub fn router\(pool: deadpool_postgres::Pool\) -> axum::Router \{")
# a (lowercase_optional_module::)fn(pool) call, e.g. `console_router(pool)` or `routes::mind_routes(pool)`
CALL_RE = re.compile(r"(\b[a-zA-Z_]\w*(?:::[a-zA-Z_]\w*)*)\(pool\)")

def fix_file(path):
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        src = f.read()
    m = ROUTER_RE.search(src)
    if not m:
        return False
    body_start = m.end()
    window = src[body_start:body_start + 600]
    cm = CALL_RE.search(window)
    if not cm:
        return False
    # avoid double-fixing if already wrapped
    if "layer(axum::extract::Extension(pool))" in window[:cm.end()]:
        return False
    inner = cm.group(1)
    new_call = inner + "().layer(axum::extract::Extension(pool))"
    abs_start = body_start + cm.start()
    abs_end = body_start + cm.end()
    new_src = src[:abs_start] + new_call + src[abs_end:]
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_src)
    return True

def main():
    only = set(sys.argv[1:]) if len(sys.argv) > 1 else None
    fixed = []
    for crate in sorted(os.listdir(CRATES)):
        if only and crate not in only:
            continue
        cdir = os.path.join(CRATES, crate)
        srcdir = os.path.join(cdir, "src")
        if not os.path.isdir(srcdir):
            continue
        for fn in sorted(os.listdir(srcdir)):
            if not fn.endswith(".rs"):
                continue
            p = os.path.join(srcdir, fn)
            if fix_file(p):
                fixed.append(f"{crate}/src/{fn}")
    print(f"Fixed {len(fixed)} file(s):")
    for f in fixed:
        print("  " + f)

if __name__ == "__main__":
    main()
