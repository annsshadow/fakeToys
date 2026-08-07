#!/usr/bin/env python3
"""Fix `use crate::X;` imports (in any crate file, especially tests.rs) where X
is a router fn defined only inside `routes.rs` (a submodule) and therefore is
not reachable as `crate::X`. Such imports must be `crate::routes::X`.

Only names ending in `_router` / `_routes` are considered. Crate-root fns and
re-exported fns (`pub use routes::X` in lib.rs) are left untouched.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

FN_DEF_RE = re.compile(r"^\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)\s*\(")
PUB_USE_RE = re.compile(r"^\s*pub\s+use\s+routes::([A-Za-z_]\w*)\s*;")
IMPORT_RE = re.compile(r"^(?P<indent>\s*)use\s+crate::(?P<name>[A-Za-z_]\w*(?:_router|_routes))\s*;\s*$")


def main():
    changed = []
    for crate in sorted(os.listdir(CRATES)):
        cdir = os.path.join(CRATES, crate)
        srcdir = os.path.join(cdir, "src")
        if not os.path.isdir(srcdir):
            continue
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

        for fn in os.listdir(srcdir):
            if not fn.endswith(".rs"):
                continue
            p = os.path.join(srcdir, fn)
            with open(p, encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
            new_lines = []
            file_changed = False
            for line in lines:
                m = IMPORT_RE.match(line)
                if m:
                    name = m.group("name")
                    if name not in root_fns and name not in root_reexports and name in routes_fns:
                        indent = m.group("indent")
                        new_line = f"{indent}use crate::routes::{name};\n"
                        new_lines.append(new_line)
                        file_changed = True
                        changed.append(f"{crate}/{fn}: {name}")
                        continue
                new_lines.append(line)
            if file_changed:
                with open(p, "w", encoding="utf-8") as f:
                    f.writelines(new_lines)
    if not changed:
        print("No test/import paths needed fixing.")
    else:
        print(f"Fixed {len(changed)} import path(s):")
        for c in changed:
            print("  " + c)


if __name__ == "__main__":
    main()
