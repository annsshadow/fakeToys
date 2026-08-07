#!/usr/bin/env python3
"""Bisect create_app router merges by K (robust, parses current lib.rs).

Usage: bisect_lib2.py K
Patches src/lib.rs so create_app merges only the first K router expressions
(from the canonical 80-merge chain), preserving the layer section. Restores
lib.rs to the full chain when K == 'all'.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIB = os.path.join(ROOT, "src", "lib.rs")
# Always restore from the canonical snapshot so a previously-corrupted lib.rs
# never feeds back into the next patch.
CANONICAL = os.path.join(ROOT, "scripts", "_lib_canonical.rs")

SRC = open(CANONICAL, "r", encoding="utf-8").read()


def parse_merges(src):
    """Return list of inner expressions of `.merge(<expr>)` occurrences
    within the create_app router chain (before the `.layer(...)` section)."""
    merges = []
    i = 0
    # only the router-chain merges: up to the first `.layer(`
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
                    inner = seg[start:k].strip()
                    merges.append(inner)
                    break
                depth -= 1
            k += 1
        i = k + 1
    return merges


def parse_layers(src):
    """Not used in current build(); kept for reference."""
    return ""


def build(k):
    merges = parse_merges(SRC)
    total = len(merges)
    if k != "all":
        k = int(k)
        assert 1 <= k <= total, f"K must be in [1,{total}] or 'all'"
        chosen = merges[:k]
    else:
        chosen = merges
    start = SRC.find("Router::new()")      # start of the merge-chain anchor line
    end = SRC.find("let app = app")        # start of the layer-adding block
    nl = SRC.rfind("\n", 0, end)            # back up to start of that line
    if nl == -1:
        nl = 0
    else:
        nl += 1
    # head: everything before "Router::new()" keeps the opening of the chain.
    head = SRC[:start] + "Router::new()\n"
    tail = SRC[nl:]                         # the layer block + Ok(app) + closing }
    merges_str = "".join(f"        .merge({m})\n" for m in chosen).rstrip("\n") + ";\n"
    new_src = head + merges_str + tail
    return new_src, total, len(chosen)


def main():
    if len(sys.argv) < 2:
        print("usage: bisect_lib2.py K|all")
        sys.exit(2)
    k = sys.argv[1]
    new_src, total, n = build(k)
    with open(LIB, "w", encoding="utf-8") as f:
        f.write(new_src)
    print(f"patched src/lib.rs: merged {n}/{total} routers (K={k})")


if __name__ == "__main__":
    main()
