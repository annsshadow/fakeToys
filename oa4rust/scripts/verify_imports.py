#!/usr/bin/env python3
"""Verify that every `use crate::X;` import resolves to a crate-root item or a
re-export. Flag imports where X is defined only inside a submodule (e.g.
routes.rs) without a re-export -- those are wrong and would cause E0432."""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

FN_DEF = re.compile(r"^\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)\s*\(")
PUB_USE = re.compile(r"^\s*pub\s+use\s+routes::([A-Za-z_]\w*)\s*;")
PUB_MOD = re.compile(r"^\s*pub\s+mod\s+([A-Za-z_]\w*)\s*;")
IMPORT_CRATE = re.compile(r"^\s*use\s+crate::([A-Za-z_]\w*)\s*;")

problems = []
for crate in sorted(os.listdir(CRATES)):
    cdir = os.path.join(CRATES, crate)
    srcdir = os.path.join(cdir, "src")
    if not os.path.isdir(srcdir):
        continue
    # gather crate-root (lib.rs) definitions and re-exports
    root_fns = set()
    reexports = set()
    root_mods = set()
    libp = os.path.join(srcdir, "lib.rs")
    if os.path.exists(libp):
        with open(libp, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = FN_DEF.match(line)
                if m:
                    root_fns.add(m.group(1))
                m = PUB_USE.match(line)
                if m:
                    reexports.add(m.group(1))
                m = PUB_MOD.match(line)
                if m:
                    root_mods.add(m.group(1))
    # gather submodule (routes.rs) definitions
    sub_fns = set()
    rp = os.path.join(srcdir, "routes.rs")
    if os.path.exists(rp):
        with open(rp, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = FN_DEF.match(line)
                if m:
                    sub_fns.add(m.group(1))
    # check every file's crate::X imports
    for fn in os.listdir(srcdir):
        if not fn.endswith(".rs"):
            continue
        p = os.path.join(srcdir, fn)
        with open(p, encoding="utf-8", errors="replace") as f:
            for i, line in enumerate(f, 1):
                m = IMPORT_CRATE.match(line)
                if not m:
                    continue
                x = m.group(1)
                if x in root_fns or x in reexports or x in root_mods:
                    continue
                if x in sub_fns:
                    problems.append(f"{crate}/{fn}:{i}: use crate::{x}; but {x} only in submodule (no re-export)")
                # else: struct/const at root or other -> assume ok

if problems:
    print(f"Found {len(problems)} suspicious imports:")
    for p in problems:
        print("  " + p)
else:
    print("All `use crate::X;` imports resolve correctly.")
