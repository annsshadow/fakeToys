#!/usr/bin/env python3
"""Binary-search the create_app merge chain to find the crate whose router()
overflows the stack (or whose merge triggers unbounded recursion).

Patches src/lib.rs (via bisect_lib2 logic) to merge only the first K routers,
runs the integration test, and reports whether it overflowed. Binary-searches
K in [1, total] to find the smallest K that overflows.
"""
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIB = os.path.join(ROOT, "src", "lib.rs")
CANONICAL = os.path.join(ROOT, "scripts", "_lib_canonical.rs")
CARGO = r"C:\Users\Administrator\.cargo\bin\cargo.exe"

SRC = open(CANONICAL, "r", encoding="utf-8").read()


def parse_merges(src):
    merges = []
    i = 0
    chain_end = src.find(".layer(")
    if chain_end == -1:
        chain_end = len(src)
    seg = src[:chain_end]
    while True:
        j = seg.find(".merge(", i)
        if j == -1:
            break
        depth = 0
        k = j + len(".merge(")
        start = k
        while k < len(seg):
            c = seg[k]
            if c == "(":
                depth += 1
            elif c == ")":
                if depth == 0:
                    merges.append(seg[start:k].strip())
                    break
                depth -= 1
            k += 1
        i = k + 1
    return merges


MERGES = parse_merges(SRC)
TOTAL = len(MERGES)


def crate_name(expr):
    # expr like `auth::router(pool...)` or `shared::router::router()`
    return expr.split("::")[0]


def patch_k(k):
    chosen = MERGES[:k]
    start = SRC.find("Router::new()")
    end = SRC.find("let app = app")
    nl = SRC.rfind("\n", 0, end)
    nl = 0 if nl == -1 else nl + 1
    head = SRC[:start] + "Router::new()\n"
    tail = SRC[nl:]
    merges_str = "".join(f"        .merge({m})\n" for m in chosen).rstrip("\n") + ";\n"
    new_src = head + merges_str + tail
    with open(LIB, "w", encoding="utf-8") as f:
        f.write(new_src)


def run_test():
    env = dict(os.environ)
    env["PATH"] = r"C:\Users\Administrator\.cargo\bin" + os.pathsep + env.get("PATH", "")
    env["CARGO_INCREMENTAL"] = "0"
    p = subprocess.run(
        [CARGO, "test", "--test", "integration_tests",
         "test_all_routes_merge_without_panic"],
        cwd=ROOT, env=env,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
    )
    out = p.stdout.decode("utf-8", "replace")
    overflow = "overflowed its stack" in out
    compile_err = "could not compile" in out
    return overflow, compile_err, p.returncode, out


def check(k):
    patch_k(k)
    overflow, compile_err, rc, out = run_test()
    if compile_err:
        print(f"  K={k:2d} ({crate_name(MERGES[k-1]) if k>0 else '-'}): "
              f"COMPILE_ERR (rc={rc}) -- aborting, patch broken")
        raise SystemExit(2)
    print(f"  K={k:2d} ({crate_name(MERGES[k-1]) if k>0 else '-'}): "
          f"{'OVERFLOW' if overflow else 'ok'} (rc={rc})")
    return overflow


def main():
    # endpoints
    print(f"total merges = {TOTAL}")
    lo, hi = 1, TOTAL
    # confirm hi overflows
    if not check(hi):
        print("K=all did NOT overflow -> nothing to bisect. Exiting.")
        patch_k("all") if False else patch_k(TOTAL)
        return
    if check(lo):
        culprit = lo
    else:
        # binary search smallest K that overflows
        while hi - lo > 1:
            mid = (lo + hi) // 2
            if check(mid):
                hi = mid
            else:
                lo = mid
        culprit = hi
    print(f"\nCULPRIT K={culprit} crate={crate_name(MERGES[culprit-1])}")
    print(f"  merge expr: {MERGES[culprit-1]}")
    # restore canonical
    patch_k(TOTAL)


if __name__ == "__main__":
    main()
