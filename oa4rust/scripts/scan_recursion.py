#!/usr/bin/env python3
"""Find per-crate router recursion cycles introduced by the mechanical transform.

For each crate, collect every `-> Router` function in lib.rs and routes.rs
(keyed by (file, name)). Build the intra-crate call graph (resolving
`crate::X`, `routes::X`, and bare `X(pool)` to the matching fn) and report any
cycle. This catches the `router -> <crate>_router -> routes::router -> <crate>_router`
pattern that overflows create_app.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

FN_RE = re.compile(r"\b(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
ROUTER_RE = re.compile(r"->\s*(?:axum::)?Router\b")
# calls: crate::X ( pool | ...), routes::X ( ...), or X ( pool
CALL_RE = re.compile(
    r"\b(?:crate::|routes::)?([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*(?:pool|self|&pool)?"
)


def split_fns(src):
    fns = []
    for m in FN_RE.finditer(src):
        name = m.group(1)
        i = src.find("{", m.end())
        if i == -1:
            continue
        depth, j = 0, i
        while j < len(src):
            c = src[j]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        fns.append((name, src[i + 1 : j]))
    return fns


def sig_of(src, name):
    i = src.find("fn " + name)
    j = src.find("{", i)
    return src[i:j]


def main():
    cyclic = []
    for crate in sorted(d for d in os.listdir(CRATES) if os.path.isdir(os.path.join(CRATES, d))):
        fns = {}  # (file, name) -> body
        for fname in ("lib.rs", "routes.rs"):
            p = os.path.join(CRATES, crate, "src", fname)
            if not os.path.exists(p):
                continue
            src = open(p, "r", encoding="utf-8", errors="replace").read()
            for name, body in split_fns(src):
                if ROUTER_RE.search(sig_of(src, name)):
                    fns[(fname, name)] = body
        if not fns:
            continue
        # build edges
        edges = {k: set() for k in fns}
        for (fname, name), body in fns.items():
            for cm in CALL_RE.finditer(body):
                callee = cm.group(1)
                if callee in ("if", "for", "while", "match", "fn", "return", "Some", "Ok", "Err", "let"):
                    continue
                # resolve target within this crate
                for target in ((fname2, n2) for (fname2, n2) in fns if n2 == callee):
                    edges[(fname, name)].add(target)
        # cycle detection (DFS)
        color = {k: 0 for k in fns}
        cyc = []

        def dfs(u, stack):
            color[u] = 1
            stack.append(u)
            for v in edges.get(u, ()):
                if color.get(v, 0) == 1:
                    cyc.append(stack[stack.index(v):] + [v])
                    return True
                if color.get(v, 0) == 0 and dfs(v, stack):
                    return True
            stack.pop()
            color[u] = 2
            return False

        for k in fns:
            if color[k] == 0 and dfs(k, []):
                break
        if cyc:
            cyclic.append((crate, cyc[0]))
    if cyclic:
        print(f"FOUND {len(cyclic)} cyclic crates:")
        for crate, c in cyclic:
            chain = " -> ".join(f"{f}:{n}" for (f, n) in c)
            print(f"  {crate}: {chain}")
    else:
        print("NO per-crate router cycles found")


if __name__ == "__main__":
    main()
