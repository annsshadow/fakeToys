#!/usr/bin/env python3
"""Compute Java-Rust endpoint alignment.

Default (new) denominator: unique method+path endpoints from
docs/audits/java-endpoint-inventory.json (built by
scripts/build_java_endpoint_inventory.py).

--legacy switches the denominator to the raw verb-annotation count
(@GET/@POST/@PUT/@DELETE), reproducing the old methodology. Note: the
historical figure 4386 included double-counting from webapp/describe/sources
source mirrors; this script reports both for reference.

Numerator either way: unique Rust endpoints from
oa4rust/tests/behavior_comparison/endpoints.rs, grouped into module families.

Usage (from repo root):
    python scripts/calc_alignment.py            # new methodology
    python scripts/calc_alignment.py --legacy   # verb-annotation denominator
"""

import argparse
import collections
import json
import os
import re
import sys

HISTORICAL_LEGACY_DENOMINATOR = 4386  # cited by plan002 U2; includes mirror copies

PARAM_NORM_RE = re.compile(r"\{[^/{}]*\}")
SPLIT_RE = re.compile(r"EndpointDef\s*\{")
FIELD_RE = {
    "crate_name": re.compile(r'crate_name:\s*"([^"]*)"'),
    "method": re.compile(r'method:\s*"([^"]*)"'),
    "rust_path": re.compile(r'rust_path:\s*"([^"]*)"'),
}

CORE_SUFFIX_RE = re.compile(r"_core_(entity|express)$")


def family(name):
    """Collapse module/crate names to their business-module family."""
    n = name[2:] if name.startswith("x_") else name
    if "_assemble_" in n:
        return n.split("_assemble_", 1)[0]
    if n.endswith("_service_processing"):
        return n[: -len("_service_processing")]
    if CORE_SUFFIX_RE.search(n):
        return CORE_SUFFIX_RE.sub("", n)
    if "_core_" in n:
        return n.split("_core_", 1)[0]
    return n


def normalize_rust_path(p):
    p = PARAM_NORM_RE.sub("{}", p)
    if not p.startswith("/"):
        p = "/" + p
    p = re.sub("/{2,}", "/", p)
    if len(p) > 1:
        p = p.rstrip("/")
    return p or "/"


def load_rust_endpoints(path):
    text = open(path, encoding="utf-8", errors="replace").read()
    per_crate = collections.defaultdict(set)
    total_defs = 0
    segments = SPLIT_RE.split(text)[1:]
    for seg in segments:
        crate = FIELD_RE["crate_name"].search(seg)
        method = FIELD_RE["method"].search(seg)
        rpath = FIELD_RE["rust_path"].search(seg)
        if not (crate and method and rpath):
            continue
        total_defs += 1
        per_crate[crate.group(1)].add(
            (method.group(1).upper(), normalize_rust_path(rpath.group(1)))
        )
    return per_crate, total_defs


def pct(num, den):
    return f"{(num / den * 100):.1f}%" if den else "n/a"


def main():
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--inventory",
        default=os.path.join(repo_root, "docs", "audits", "java-endpoint-inventory.json"),
    )
    ap.add_argument(
        "--endpoints",
        default=os.path.join(repo_root, "oa4rust", "tests", "behavior_comparison", "endpoints.rs"),
    )
    ap.add_argument("--legacy", action="store_true", help="use raw verb annotations as denominator")
    ap.add_argument("--out-json", default=None, help="optionally save the report as json")
    args = ap.parse_args()

    try:
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except Exception:
        pass

    inv = json.load(open(args.inventory, encoding="utf-8"))
    per_crate, rust_defs = load_rust_endpoints(args.endpoints)

    java_fams = collections.defaultdict(lambda: {"unique": 0, "verbs": 0, "modules": []})
    for mod, m in inv["modules"].items():
        f = family(mod)
        java_fams[f]["unique"] += m["unique_endpoints"]
        java_fams[f]["verbs"] += m["verb_annotations"]
        java_fams[f]["modules"].append(mod)

    rust_fams = collections.defaultdict(lambda: {"unique": 0, "crates": []})
    unmapped_crates = {}
    for crate, eps in sorted(per_crate.items()):
        f = family(crate)
        rust_fams[f]["unique"] += len(eps)
        rust_fams[f]["crates"].append((crate, len(eps)))
        if f not in java_fams:
            unmapped_crates[crate] = len(eps)

    rust_total = sum(len(v) for v in per_crate.values())
    rust_mapped_total = sum(
        v["unique"] for f, v in rust_fams.items() if f in java_fams
    )

    rows = []
    for fam, jv in java_fams.items():
        covered = rust_fams.get(fam, {}).get("unique", 0)
        gap_denom = jv["verbs"] if args.legacy else jv["unique"]
        rows.append(
            {
                "family": fam,
                "java_unique": jv["unique"],
                "java_verbs": jv["verbs"],
                "covered": covered,
                "denominator": gap_denom,
                "gap": max(gap_denom - covered, 0),
                "ratio": (covered / gap_denom) if gap_denom else None,
                "crates": rust_fams.get(fam, {}).get("crates", []),
                "modules": jv["modules"],
            }
        )
    rows.sort(key=lambda r: (-r["gap"], -r["denominator"]))

    denom_total = sum(r["denominator"] for r in rows)
    global_ratio = rust_total / denom_total if denom_total else 0

    mode = "LEGACY (verb annotations)" if args.legacy else "UNIQUE ENDPOINTS"
    print(f"alignment report | mode: {mode} | rust defs parsed: {rust_defs} "
          f"-> unique rust endpoints: {rust_total} (mapped-to-java: {rust_mapped_total})")
    print(f"global alignment: {rust_total} / {denom_total} = {pct(rust_total, denom_total)}")
    if args.legacy:
        print(f"(historical legacy denominator incl. describe/sources mirrors: "
              f"{HISTORICAL_LEGACY_DENOMINATOR}; inventory src/main/java-only verbs: {inv['totals']['verb_annotations']})")
    print()
    print("| Module family | Java unique | Java verbs | Denominator used | Rust covered | Coverage | Gap | Crates |")
    print("|---|---|---|---|---|---|---|---|")
    for r in rows:
        crates = ",".join(f"{c}:{n}" for c, n in r["crates"]) or "-"
        print(
            f"| {r['family']} | {r['java_unique']} | {r['java_verbs']} | {r['denominator']} "
            f"| {r['covered']} | {pct(r['covered'], r['denominator'])} | {r['gap']} | {crates} |"
        )

    unmapped_sum = sum(unmapped_crates.values())
    if unmapped_crates:
        print()
        print(f"crates with no matching java family (excluded from table, "
              f"{unmapped_sum} endpoints, counted in global numerator):")
        for c, n in sorted(unmapped_crates.items(), key=lambda kv: -kv[1]):
            print(f"  - {c}: {n}")

    top = [r for r in rows if r["gap"] > 0][:10]
    if top:
        print()
        print("Top10 缺口模块（按缺口端点数降序）:")
        for r in top:
            print(
                f"  {r['family']:<28} denominator={r['denominator']:>5} "
                f"covered={r['covered']:>5} gap={r['gap']:>5} ({pct(r['covered'], r['denominator'])})"
            )

    if args.out_json:
        report = {
            "mode": "legacy-verb-annotations" if args.legacy else "unique-endpoints",
            "rust_total_unique": rust_total,
            "rust_total_defs": rust_defs,
            "rust_mapped_to_java": rust_mapped_total,
            "denominator_used_total": denom_total,
            "global_alignment": global_ratio,
            "historical_legacy_denominator": HISTORICAL_LEGACY_DENOMINATOR,
            "rows": rows,
            "unmapped_crates": unmapped_crates,
        }
        with open(args.out_json, "w", encoding="utf-8") as fh:
            json.dump(report, fh, ensure_ascii=False, indent=1)
            fh.write("\n")
        print(f"\nreport saved: {args.out_json}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
