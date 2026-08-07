#!/usr/bin/env python3
"""Global cross-crate router recursion detector (R34, robust).

Builds a registry of every function that returns an `axum::Router` across all
crates (keyed by `crate::fnname`), then for each such function scans its body
for calls to other registry members (same-crate self/helper calls, `crate::X`,
or `X::router`/`X::routes` forms). Builds the call graph and reports any cycle.

This catches the mechanical-transform-induced infinite recursion that overflows
`create_app` at runtime (and would crash the real server at startup).
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

FN_DEF_RE = re.compile(r"\b(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
ROUTER_RET_RE = re.compile(r"->\s*(?:axum::)?Router\b")
# call: IDENT ( pool  OR  IDENT :: IDENT ( pool
CALL_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)(?:::([A-Za-z_][A-Za-z0-9_]*))?\s*\(\s*pool\b")


def list_crates():
    return sorted(d for d in os.listdir(CRATES) if os.path.isdir(os.path.join(CRATES, d)))


def read(p):
    try:
        with open(p, "r", encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return ""


def split_fns(src):
    fns = []
    for m in FN_DEF_RE.finditer(src):
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


def build_registry():
    """Return dict: 'crate::fnname' -> body, for fns returning Router."""
    registry = {}
    for crate in list_crates():
        for fn in ("lib.rs", "routes.rs"):
            p = os.path.join(CRATES, crate, "src", fn)
            if not os.path.exists(p):
                continue
            src = read(p)
            for name, body in split_fns(src):
                # check return type: from fn signature start to first {
                sig_start = src.find("fn " + name)
                sig = src[sig_start:src.find("{", sig_start)]
                if ROUTER_RET_RE.search(sig):
                    registry[f"{crate}::{name}"] = body
    # shared::router::router lives in crates/shared/src/router.rs
    p = os.path.join(CRATES, "shared", "src", "router.rs")
    if os.path.exists(p):
        src = read(p)
        for name, body in split_fns(src):
            sig_start = src.find("fn " + name)
            sig = src[sig_start:src.find("{", sig_start)]
            if ROUTER_RET_RE.search(sig):
                registry[f"shared::{name}"] = body
    return registry


def resolve_call(crate, callee_base, callee_method, registry):
    """Resolve a call to a registry key."""
    # X::router  -> crate X, fn router
    if callee_method and callee_base in set(k.split("::")[0] for k in registry):
        key = f"{callee_base}::{callee_method}"
        if key in registry:
            return key
    # plain IDENT in same crate, possibly a router fn
    if callee_base in set(k.split("::")[1] for k in registry if k.startswith(crate + "::")):
        # find exact crate::fn matching base name within this crate
        for k in registry:
            if k.startswith(crate + "::") and k.split("::")[1] == callee_base:
                return k
    # crate::X
    key = f"{crate}::{callee_base}"
    if key in registry:
        return key
    return None


def detect():
    registry = build_registry()
    edges = {k: set() for k in registry}
    for key, body in registry.items():
        crate = key.split("::")[0]
        for m in CALL_RE.finditer(body):
            base, method = m.group(1), m.group(2)
            tgt = resolve_call(crate, base, method, registry)
            if tgt and tgt != key:
                edges[key].add(tgt)
    # DFS for cycle
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {k: WHITE for k in registry}
    cyc = []

    def dfs(u, stack):
        color[u] = GRAY
        stack.append(u)
        for v in edges.get(u, ()):
            if color.get(v, WHITE) == GRAY:
                idx = stack.index(v)
                cyc.append(stack[idx:] + [v])
                return True
            if color.get(v, WHITE) == WHITE:
                if dfs(v, stack):
                    return True
        stack.pop()
        color[u] = BLACK
        return False

    for k in registry:
        if color[k] == WHITE:
            if dfs(k, []):
                break
    return cyc, registry


if __name__ == "__main__":
    cyc, registry = detect()
    if cyc:
        print("[CYCLE] " + " -> ".join(cyc[0]))
        sys.exit(1)
    print(f"NO_CYCLE ({len(registry)} router-returning fns analyzed)")
    sys.exit(0)
