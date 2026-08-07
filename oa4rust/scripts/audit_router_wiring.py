#!/usr/bin/env python3
"""Audit router(pool) wiring across all crates.

For every crate, find `pub fn router(pool: Pool)` (or any fn named router that
takes a Pool). Determine which inner function it calls. Then check whether that
inner function's definition accepts a Pool argument. Report mismatches.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

def read(p):
    with open(p, "r", encoding="utf-8", errors="replace") as f:
        return f.read()

# Match a function definition: `fn NAME ( ARGS ) -> RET {`
FN_DEF = re.compile(r"fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)")
# Match `pub fn router(pool: ...)` anywhere
ROUTER_DEF = re.compile(r"fn\s+router\s*\(([^)]*)\)")
# Match a call inside router body: `SOMEFN(pool)` or `SOMEFN()` or `mod::SOMEFN(pool)`
CALL = re.compile(r"([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)\s*\(([^)]*)\)")

def def_takes_pool(args_str):
    # definition args look like "pool: deadpool_postgres::Pool" or "pool: Pool"
    if not args_str.strip():
        return False
    return bool(re.search(r"pool\s*:|deadpool_postgres::Pool|:\s*Pool\b", args_str))

def call_passes_pool(args_str):
    # call args look like "pool" / "pool.clone()" / "pool, x"
    a = args_str.strip()
    if not a:
        return False
    return "pool" in a

def find_fn_defs(src):
    defs = {}
    for m in FN_DEF.finditer(src):
        name = m.group(1)
        args = m.group(2)
        # only record top-level-ish fns (heuristic: not inside a fn body already accounted)
        defs.setdefault(name, args)
    return defs

problems = []
checked = 0
for crate in sorted(os.listdir(CRATES)):
    cdir = os.path.join(CRATES, crate)
    if not os.path.isdir(cdir):
        continue
    srcdir = os.path.join(cdir, "src")
    if not os.path.isdir(srcdir):
        continue
    files = []
    for fn in os.listdir(srcdir):
        if fn.endswith(".rs"):
            files.append(os.path.join(srcdir, fn))
    if not files:
        continue
    allsrc = {os.path.basename(f): read(f) for f in files}
    combined = "\n".join(allsrc.values())
    # find router defs
    router_found = False
    for fname, src in allsrc.items():
        for m in ROUTER_DEF.finditer(src):
                    args = m.group(1)
                    if not def_takes_pool(args):
                        continue
                    router_found = True
                    # find the router body call: take text after this def up to next fn def at same level
                    start = m.end()
                    body = src[start:start+800]
                    # find first call that is a lowercase router fn (skip constructors/type::new)
                    called = None
                    for cm in CALL.finditer(body):
                        cname = cm.group(1)
                        cargs = cm.group(2)
                        if cname[0:1].isupper():
                            # PascalCase -> constructor/type, not the inner router fn
                            continue
                        if cname in ("if", "match", "while", "for", "some", "ok", "err", "return"):
                            continue
                        called = (cname, cargs)
                        break
                    if called is None:
                        # router body may be a direct Router::new() chain (pool used via merge/route)
                        problems.append((crate, fname, "router body does not call an inner router fn (direct Router::new chain?)", args))
                        continue
                    cname, cargs = called
                    # resolve definition across files
                    cdef_args = None
                    if "::" in cname:
                        mod, fn = cname.split("::", 1)
                        for mf in allsrc:
                            if fn in find_fn_defs(allsrc[mf]):
                                cdef_args = find_fn_defs(allsrc[mf])[fn]
                                break
                    else:
                        for mf, s in allsrc.items():
                            defs = find_fn_defs(s)
                            if cname in defs:
                                cdef_args = defs[cname]
                                break
                    if cdef_args is None:
                        problems.append((crate, fname, f"router calls {cname}(...) which is UNDEFINED", args))
                    else:
                        # router passes pool; does called fn accept pool?
                        if call_passes_pool(cargs):
                            if not def_takes_pool(cdef_args):
                                problems.append((crate, fname, f"router passes pool to {cname}() but {cname} defined with 0 args: ({cdef_args})", args))
                        else:
                            if def_takes_pool(cdef_args):
                                pass  # pool unused warning only
    if router_found:
        checked += 1

print(f"Checked {checked} crates with a router(pool) entry point.")
print(f"Found {len(problems)} wiring problems:\n")
for crate, fname, msg, args in sorted(problems):
    print(f"  [{crate}] {fname}: {msg}")
