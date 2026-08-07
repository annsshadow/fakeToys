#!/usr/bin/env python3
"""Cross-crate router recursion detector (R34 diagnostic).

create_app merges 80 crate routers; each crate's root `router(pool)` is
evaluated eagerly during the merge. If crate A's router calls crate B's router
which (transitively) calls back into A, the merge stack-overflows -- and the
real server would crash identically at startup.

We build a GLOBAL call graph over crates:
  - The root router fn of a crate = the `pub fn router` that `crate::router`
    resolves to (lib.rs `router`, or `routes::router` re-exported at root).
  - Inside that fn body we find calls to other crates' router fns:
        crate::<other>_router(pool)
        <other>::router(pool)
        <other>::routes(pool)
    mapping the prefix to a crate name.
  - Edge: crate -> called crate. Detect any cycle.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

FN_DEF_RE = re.compile(r"\b(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
ROUTER_NAME_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*(?:_router|_routes|router|routes)")


def read(p):
    try:
        with open(p, "r", encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return ""


def list_crates():
    out = {}
    for name in sorted(os.listdir(CRATES)):
        d = os.path.join(CRATES, name)
        if os.path.isdir(d):
            out[name] = d
    return out


def split_fns(src):
    fns = []
    for m in FN_DEF_RE.finditer(src):
        name = m.group(1)
        start = m.end()
        i = src.find("{", start)
        if i == -1:
            continue
        depth = 0
        j = i
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


def root_router_body(crate_dir):
    """Return the body of the crate-root `router` fn (lib.rs `router`, else routes::router)."""
    lib = read(os.path.join(crate_dir, "src", "lib.rs"))
    lib_fns = {n: b for n, b in split_fns(lib)}
    if "router" in lib_fns:
        return lib_fns["router"], "lib"
    routes = read(os.path.join(crate_dir, "src", "routes.rs"))
    routes_fns = {n: b for n, b in split_fns(routes)}
    if "router" in routes_fns:
        return routes_fns["router"], "routes"
    return None, None


def detect():
    crates = list_crates()
    # map crate name -> root router body
    bodies = {}
    for name, d in crates.items():
        body, _ = root_router_body(d)
        if body is not None:
            bodies[name] = body

    edges = {name: set() for name in bodies}
    for name, body in bodies.items():
        # find calls to OTHER crates' routers
        # pattern: crate::<other>_router  OR  <other>::router / <other>::routes
        for m in re.finditer(r"(?:crate::)?([A-Za-z_][A-Za-z0-9_]*)(?:::([A-Za-z_][A-Za-z0-9_]*))?\(pool", body):
            prefix = m.group(1)
            method = m.group(2)
            # candidate called crate
            called = None
            if prefix in bodies and prefix != name:
                called = prefix
            elif method in ("router", "routes") and prefix in bodies and prefix != name:
                called = prefix
            if called:
                edges[name].add(called)

    # DFS cycle detection over crate graph
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {n: WHITE for n in bodies}
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

    for n in bodies:
        if color[n] == WHITE:
            if dfs(n, []):
                break

    return cyc


if __name__ == "__main__":
    cyc = detect()
    if not cyc:
        print("NO_CROSS_CRATE_RECURSION")
        sys.exit(0)
    print("[CYCLE] " + " -> ".join(cyc[0]))
    sys.exit(1)
