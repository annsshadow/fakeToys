#!/usr/bin/env python3
"""Break the mechanical-transform recursion in the 11 cyclic crates.

For each crate, routes.rs::router(pool) was generated as a passthrough that
calls back into `crate::<crate>_router(pool)`, forming an infinite cycle that
overflows create_app. This rewrites routes::router to actually wire up the
handlers that routes.rs already imports from `crate::{...}`, turning the
recursive stub into a real (non-recursive) router.

Method: every wired handler uses `get` (axum compiles fine; HTTP-method
semantics are a later realize refinement). Path: `/jaxrs/<crate>/<handler>`
with underscores replaced by '/', matching the convention used by sibling
crates (e.g. `config_get` -> `/jaxrs/ai/config/get`).
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

# The 10 crates whose routes::router back-calls <crate>_router (real cycles).
# attendance_assemble_control is a false positive (its routes::router is already
# a real, fully-wired router) and must NOT be touched.
CYCLIC = [
    "cms_control",
    "ai_assemble_control",
    "bbs_assemble_control",
    "calendar_assemble_control",
    "cms_assemble_control",
    "component_assemble_control",
    "file_assemble_control",
    "hotpic_assemble_control",
    "jpush_assemble_control",
    "organization_assemble_express",
]

IMPORT_RE = re.compile(r"use\s+crate::\{([^}]*)\}")
ROUTER_FN_RE = re.compile(r"pub\s+fn\s+router\s*\(\s*pool\s*[^)]*\)\s*->\s*axum::Router\s*\{(.*?)\n\}", re.S)


def split_fns(src):
    """Return list of (name, body_start, body_end, full_match) for fns."""
    out = []
    for m in re.finditer(r"pub\s+fn\s+([A-Za-z_]\w*)\s*\(", src):
        name = m.group(1)
        i = src.find("{", m.end())
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
        out.append((name, i + 1, j, m.start(), j + 1))
    return out


def main():
    for crate in CYCLIC:
        p = os.path.join(CRATES, crate, "src", "routes.rs")
        if not os.path.exists(p):
            print(f"[skip] {crate}: no routes.rs")
            continue
        src = open(p, "r", encoding="utf-8").read()
        # extract imported handlers
        handlers = []
        for m in IMPORT_RE.finditer(src):
            for part in m.group(1).split(","):
                part = part.strip()
                if part:
                    handlers.append(part)
        # find router fn
        fns = split_fns(src)
        router = next((f for f in fns if f[0] == "router"), None)
        if router is None:
            print(f"[skip] {crate}: no pub fn router")
            continue
        body = src[router[1]:router[2]].strip()
        # Always regenerate for the known back-call crates.
        if not re.search(r"crate::\w+_router\s*\(", body) and ".route(" not in body:
            print(f"[skip] {crate}: router body neither back-call nor empty: {body!r}")
            continue
        # build new body; exclude router-builder fns (names ending in `_router`)
        wired = [h for h in handlers if not h.endswith("_router")]
        lines = ["    Router::new()"]
        for h in wired:
            path = f"/jaxrs/{crate}/{h.replace('_', '/')}"
            lines.append(f'        .route("{path}", get({h}))')
        if wired:
            lines.append("        .layer(Extension(pool))")
        new_body = "\n".join(lines)
        new_src = src[:router[1]] + "\n" + new_body + "\n" + src[router[2]:]
        open(p, "w", encoding="utf-8").write(new_src)
        print(f"[fixed] {crate}: wired {len(wired)} handlers (excluded {len(handlers)-len(wired)} router-builder(s)), router no longer recurses")


if __name__ == "__main__":
    main()
