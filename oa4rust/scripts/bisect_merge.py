#!/usr/bin/env python3
"""Rewrite create_app in src/main.rs to merge only the first K pool-backed
crate routers (plus the always-safe shared router), so we can binary-search
which crate triggers the stack overflow. Usage: bisect_merge.py K
K = number of pool-backed crate routers to keep (0..80)."""
import sys

MAIN = "src/main.rs"


def main():
    k = int(sys.argv[1])
    with open(MAIN, "r", encoding="utf-8") as f:
        lines = f.readlines()

    # find block: from "let app = Router::new()" to the line before "let app = app\n.layer("
    start = None
    end = None
    for i, ln in enumerate(lines):
        if ln.strip().startswith("let app = Router::new()"):
            start = i
        if ln.strip().startswith("let app = app"):
            end = i
            break
    if start is None or end is None:
        raise SystemExit("could not locate merge block")

    # collect merge lines between start and end
    block = lines[start:end]
    # The first merge is `Router::new().merge(shared...)` possibly on same line as Router::new.
    # We'll rebuild: keep line[start] as-is, then keep first (k+1) .merge(...) lines? 
    # Strategy: shared router is the first .merge(...). Keep it. Then keep k crate merges.
    merge_lines = [ln for ln in block if ".merge(" in ln]
    # line[start] is "let app = Router::new()" possibly with a trailing .merge(shared)
    head = lines[start]
    # ensure head ends with Router::new()
    head = "    let app = Router::new()\n"
    # shared merge
    shared_merge = [ln for ln in merge_lines if "shared::router::router" in ln]
    crate_merges = [ln for ln in merge_lines if "shared::router::router" not in ln]
    kept = shared_merge + crate_merges[:k]
    # reconstruct block
    new_block = [head] + kept
    # ensure last kept line ends with ";" 
    if kept:
        if not new_block[-1].rstrip().endswith(";"):
            new_block[-1] = new_block[-1].rstrip("\n") + ";\n"
        else:
            # already ends with ; (the original crate merge lines end with ;)
            pass
    lines[start:end] = new_block
    with open(MAIN, "w", encoding="utf-8") as f:
        f.writelines(lines)
    print(f"kept {len(kept)} merges (shared + {min(k, len(crate_merges))} crates)")


if __name__ == "__main__":
    main()
