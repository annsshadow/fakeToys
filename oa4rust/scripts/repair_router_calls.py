#!/usr/bin/env python3
"""Repair `router(pool)` inner calls by fully-qualifying them and removing the
fragile `use crate::...` imports the earlier scanner added.

Strategy (conflict-free):
  * Determine where the inner router fn `NAME` is defined for the crate:
      - crate root (lib.rs top-level fn)  -> callable as `crate::NAME`
      - re-exported (`pub use routes::NAME` in lib.rs) -> `crate::NAME`
      - submodule (routes.rs top-level fn) -> `crate::routes::NAME`
  * Rewrite the call `NAME(pool)` inside `router(pool)` to the qualified path.
  * Remove any single-line `use crate::NAME;` / `use crate::routes::NAME;`
    imports in that file (these were added by the earlier scanner and cause
    E0252/E0432 conflicts). Pre-existing multi-item `use crate::{...}` blocks
    are left untouched.

Only names ending in `_router` / `_routes` are considered, so the
`Extension(pool)` wrapper used by the arity-fixed crates is never matched.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

ROUTER_ENTRY_RE = re.compile(r"pub fn router\(pool: deadpool_postgres::Pool\) -> axum::Router \{")
FN_DEF_RE = re.compile(r"^\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)\s*\(")
PUB_USE_RE = re.compile(r"^\s*pub\s+use\s+routes::([A-Za-z_]\w*)\s*;")
CALL_POOL_RE = re.compile(r"([A-Za-z_]\w*(?:::[A-Za-z_]\w*)*_(?:router|routes))\(pool\)")
SINGLE_IMPORT_RE = re.compile(r"^\s*use\s+crate::(?:routes::)?([A-Za-z_]\w*)\s*;\s*$")


def collect(crate):
    srcdir = os.path.join(CRATES, crate, "src")
    lib = os.path.join(srcdir, "lib.rs")
    rts = os.path.join(srcdir, "routes.rs")
    root_fns = set()
    root_reexports = set()
    if os.path.exists(lib):
        with open(lib, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = FN_DEF_RE.match(line)
                if m:
                    root_fns.add(m.group(1))
                m = PUB_USE_RE.match(line)
                if m:
                    root_reexports.add(m.group(1))
    routes_fns = set()
    if os.path.exists(rts):
        with open(rts, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = FN_DEF_RE.match(line)
                if m:
                    routes_fns.add(m.group(1))
    return root_fns, root_reexports, routes_fns


def qualified_for(name, root_fns, root_reexports, routes_fns):
    if name in root_fns or name in root_reexports:
        return f"crate::{name}"
    if name in routes_fns:
        return f"crate::routes::{name}"
    # fallback: assume crate root
    return f"crate::{name}"


def repair_file(path, root_fns, root_reexports, routes_fns):
    with open(path, encoding="utf-8", errors="replace") as f:
        lines = f.readlines()
    src = "".join(lines)
    m = ROUTER_ENTRY_RE.search(src)
    if not m:
        return False
    body = src[m.end():m.end() + 600]
    cm = CALL_POOL_RE.search(body)
    if not cm:
        return False
    full = cm.group(1)
    name = full.split("::")[-1]
    qualified = qualified_for(name, root_fns, root_reexports, routes_fns)
    argstart = m.end() + cm.start()
    argend = m.end() + cm.end()
    new_src = src[:argstart] + f"{qualified}(pool)" + src[argend:]

    # remove single-line added imports for this name
    out_lines = []
    removed = 0
    for ln in new_src.split("\n"):
        sm = SINGLE_IMPORT_RE.match(ln)
        if sm and sm.group(1) == name:
            removed += 1
            continue
        out_lines.append(ln)
    new_src = "\n".join(out_lines)

    with open(path, "w", encoding="utf-8") as f:
        f.write(new_src)
    return True


def main():
    changed = []
    for crate in sorted(os.listdir(CRATES)):
        cdir = os.path.join(CRATES, crate)
        srcdir = os.path.join(cdir, "src")
        if not os.path.isdir(srcdir):
            continue
        root_fns, root_reexports, routes_fns = collect(crate)
        for fn in sorted(os.listdir(srcdir)):
            if not fn.endswith(".rs"):
                continue
            p = os.path.join(srcdir, fn)
            if repair_file(p, root_fns, root_reexports, routes_fns):
                changed.append(f"{crate}/{fn}")
    if not changed:
        print("No router calls needed repair.")
    else:
        print(f"Repaired {len(changed)} file(s):")
        for c in changed:
            print("  " + c)


if __name__ == "__main__":
    main()
