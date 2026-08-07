#!/usr/bin/env python3
"""Comprehensively fix `router(pool)` inner-call signatures across all crates.

Handles three mismatch classes discovered during the mechanical realization:

  (A) E0425 - inner router fn defined at crate root (lib.rs) but not imported in
      the file where `router(pool)` lives (usually routes.rs).
      Fix: add `use crate::<fn>;` import.

  (B) E0308 - inner router fn takes `Option<Pool>` (defensive jpush-style
      handlers) but the entry passes a plain `Pool`.
      Fix: wrap the argument as `Some(pool)`.

  (C) 0-arg inner router fn (already handled by fix_router_arity.py; this
      script is idempotent and will not touch already-wrapped calls).

Detection is scoped to the broken form `INNER(pool)` inside a
`pub fn router(pool: deadpool_postgres::Pool)` body. Already-correct or already
wrapped calls are left untouched.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

ROUTER_ENTRY_RE = re.compile(r"pub fn router\(pool: deadpool_postgres::Pool\) -> axum::Router \{")
FN_DEF_RE = re.compile(r"fn\s+([A-Za-z_]\w*)\s*\(([^)]*)\)")
CALL_POOL_RE = re.compile(r"([A-Za-z_]\w*(?:::[A-Za-z_]\w*)*)\(pool\)")
USE_LINE_RE = re.compile(r"^\s*use\s+")


def def_args_for(crate_defs, name):
    # crate_defs: dict name -> args string (first seen)
    if name in crate_defs:
        return crate_defs[name]
    return None


def is_in_scope(src, name):
    if re.search(r"fn\s+" + re.escape(name) + r"\s*\(", src):
        return True
    # imported as `use crate::name;` or inside a `use crate::{... name ...};`
    if re.search(r"use\s+crate::" + re.escape(name) + r"\s*;", src):
        return True
    if re.search(r"use\s+crate::\{[^}]*\b" + re.escape(name) + r"\b[^}]*\}\s*;", src):
        return True
    return False


def add_import(path, src, name):
    lines = src.split("\n")
    last_use = -1
    for i, ln in enumerate(lines):
        if USE_LINE_RE.match(ln):
            last_use = i
    insert = f"use crate::{name};"
    if last_use >= 0:
        lines.insert(last_use + 1, insert)
    else:
        lines.insert(0, insert)
    return "\n".join(lines)


def fix_crate(crate):
    cdir = os.path.join(CRATES, crate)
    srcdir = os.path.join(cdir, "src")
    if not os.path.isdir(srcdir):
        return []
    files = {}
    for fn in os.listdir(srcdir):
        if fn.endswith(".rs"):
            p = os.path.join(srcdir, fn)
            with open(p, "r", encoding="utf-8", errors="replace") as f:
                files[fn] = f.read()
    # collect all fn defs in crate (name -> args)
    crate_defs = {}
    for src in files.values():
        for m in FN_DEF_RE.finditer(src):
            crate_defs.setdefault(m.group(1), m.group(2))

    changes = []
    for fn, src in files.items():
        m = ROUTER_ENTRY_RE.search(src)
        if not m:
            continue
        body = src[m.end():m.end() + 600]
        cm = CALL_POOL_RE.search(body)
        if not cm:
            continue
        full = cm.group(1)          # e.g. "query_assemble_designer_router" or "routes::mind_routes"
        inner = full.split("::")[-1]
        args = def_args_for(crate_defs, inner)
        if args is None:
            changes.append(f"{crate}/{fn}: INNER {inner} undefined (skip)")
            continue
        argstart = m.end() + cm.start()
        argend = m.end() + cm.end()

        if args.strip() == "":
            # 0-arg inner -> wrap with Extension layer (pattern C)
            repl = f"{full}().layer(axum::extract::Extension(pool))"
            changes.append(f"{crate}/{fn}: wrap 0-arg {full}() with Extension layer")
        elif "Option<" in args and "Pool" in args:
            # Option<Pool> inner -> pass Some(pool) (pattern B)
            repl = f"{full}(Some(pool))"
            changes.append(f"{crate}/{fn}: wrap arg as Some(pool) for {full}")
        elif "Pool" in args:
            # Pool (1-arg) inner; needs to be in scope (pattern A)
            if is_in_scope(src, inner):
                continue  # already correct
            new_src = add_import(os.path.join(srcdir, fn), src, inner)
            with open(os.path.join(srcdir, fn), "w", encoding="utf-8") as f:
                f.write(new_src)
            changes.append(f"{crate}/{fn}: add import use crate::{inner};")
            continue  # call itself is already correct; only import was missing
        else:
            continue

        new_src = src[:argstart] + repl + src[argend:]
        with open(os.path.join(srcdir, fn), "w", encoding="utf-8") as f:
            f.write(new_src)
        # refresh src for subsequent processing of same file
        files[fn] = new_src
    return changes


def main():
    all_changes = []
    for crate in sorted(os.listdir(CRATES)):
        if not os.path.isdir(os.path.join(CRATES, crate)):
            continue
        all_changes.extend(fix_crate(crate))
    if not all_changes:
        print("No inner-router call mismatches found.")
    else:
        print(f"Applied {len(all_changes)} fix(es):")
        for c in all_changes:
            print("  " + c)


if __name__ == "__main__":
    main()
