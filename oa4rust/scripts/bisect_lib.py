#!/usr/bin/env python3
"""Rewrite lib.rs create_app to merge only shared + first K pool-backed crate
routers, keeping the middleware layer block verbatim, to binary-search which
crate triggers the (unbounded) stack overflow. Usage: bisect_lib.py K
"""
import sys

MAIN = "src/lib.rs"


def main():
    k = int(sys.argv[1])
    with open(MAIN, "r", encoding="utf-8") as f:
        src = f.readlines()

    sig = None
    for i, ln in enumerate(src):
        if ln.startswith("pub async fn create_app("):
            sig = i
            break
    if sig is None:
        raise SystemExit("create_app not found")

    ok = None
    for i in range(sig, len(src)):
        if src[i].strip() == "Ok(app)":
            ok = i
            break
    if ok is None:
        raise SystemExit("Ok(app) not found")

    block = src[sig:ok]
    # security_state block up to and including "let app = Router::new()"
    sec_end = None
    for j, ln in enumerate(block):
        if ln.strip().startswith("let app = Router::new()"):
            sec_end = j
            break
    sec_block = block[: sec_end + 1]

    merge_lines = [ln for ln in block if ".merge(" in ln]
    shared_merge = [ln for ln in merge_lines if "shared::router::router" in ln]
    crate_merges = [ln for ln in merge_lines if "shared::router::router" not in ln]
    kept = shared_merge + crate_merges[:k]
    # ensure last kept merge ends with ';'
    if kept:
        last = kept[-1].rstrip("\n")
        if not last.rstrip().endswith(";"):
            kept[-1] = last + ";\n"
        else:
            kept[-1] = last + "\n"

    # layer block: from first line containing "let app = app" to end of block
    layer_start = None
    for j, ln in enumerate(block):
        if "let app = app" in ln:
            layer_start = j
            break
    if layer_start is None:
        raise SystemExit("layer block not found")
    layer_block = block[layer_start:]

    new_block = "".join(sec_block) + "".join(kept) + "".join(layer_block) + "    Ok(app)\n"
    src[sig:ok] = [new_block]
    with open(MAIN, "w", encoding="utf-8") as f:
        f.writelines(src)
    print(f"kept shared + {min(k, len(crate_merges))} crates; layer block preserved")


if __name__ == "__main__":
    main()
