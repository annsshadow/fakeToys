#!/usr/bin/env python3
"""Build a unique method+path endpoint inventory from o2server JAX-RS sources.

Scans every x_* module under the o2server source root for `jaxrs` directories,
parses class-level @Path plus method-level @Path/@GET/@POST/@PUT/@DELETE on
each resource class, joins them, normalizes path params to `{}`, dedupes, and
writes docs/audits/java-endpoint-inventory.json grouped by module.

Usage (from repo root):
    python scripts/build_java_endpoint_inventory.py
    python scripts/build_java_endpoint_inventory.py --source-root oa/o2server --out docs/audits/java-endpoint-inventory.json
"""

import argparse
import datetime
import json
import os
import re
import sys

VERBS = ("GET", "POST", "PUT", "DELETE")
ANN_NAME_RE = re.compile(r"@[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)*")
CLASS_DECL_RE = re.compile(r"\b(?:public\s+|final\s+|abstract\s+)*class\s+(\w+)")
PATH_ARG_RE = re.compile(r'@Path\s*\(\s*"([^"]*)"')
VERB_COUNT_RE = re.compile(r"@(?:GET|POST|PUT|DELETE)\b")
PARAM_NORM_RE = re.compile(r"\{[^/{}]*\}")


def strip_comments(src):
    """Remove // and /* */ comments while preserving string/char literals."""
    out = []
    i, n = 0, len(src)
    state = None
    while i < n:
        ch = src[i]
        if state:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(src[i + 1])
                i += 2
                continue
            if ch == state:
                state = None
            i += 1
        elif ch in ('"', "'"):
            state = ch
            out.append(ch)
            i += 1
        elif ch == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            i = n if j < 0 else j
        elif ch == "/" and i + 1 < n and src[i + 1] == "*":
            j = src.find("*/", i + 2)
            out.append(" ")
            i = n if j < 0 else j + 2
        else:
            out.append(ch)
            i += 1
    return "".join(out)


def match_paren(text, open_pos):
    """Index of the ')' matching the '(' at open_pos; -1 if unbalanced."""
    depth = 0
    i, n = open_pos, len(text)
    state = None
    while i < n:
        ch = text[i]
        if state:
            if ch == "\\":
                i += 2
                continue
            if ch == state:
                state = None
            i += 1
            continue
        if ch in ('"', "'"):
            state = ch
        elif ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def scan_annotations(text):
    """Sorted list of (start, end, name); end covers parenthesized args."""
    anns = []
    for m in ANN_NAME_RE.finditer(text):
        name = m.group(0)[1:].rsplit(".", 1)[-1]
        p = m.end()
        while p < len(text) and text[p] in " \t\r\n":
            p += 1
        if p < len(text) and text[p] == "(":
            q = match_paren(text, p)
            end = q + 1 if q >= 0 else len(text)
        else:
            end = m.end()
        anns.append((m.start(), end, name))
    anns.sort(key=lambda a: a[0])
    return anns


def gap_ws(text, prev_end, nxt_start):
    return prev_end <= nxt_start and text[prev_end:nxt_start].strip() == ""


def parse_file(text):
    """Yield (class_name, class_path, verb, method_path_raw) tuples."""
    anns = scan_annotations(text)

    # Class declarations with their annotation headers and first '{'.
    classes = []  # (decl_start, header_lo, header_hi, name, class_path)
    for m in CLASS_DECL_RE.finditer(text):
        header_lo = m.start()
        changed = True
        while changed:
            changed = False
            for a in anns:
                if a[1] <= header_lo and gap_ws(text, a[1], header_lo):
                    header_lo = a[0]
                    changed = True
                    break
        brace = text.find("{", m.end())
        if brace < 0:
            continue
        header = text[header_lo:brace]
        paths = PATH_ARG_RE.findall(header)
        classes.append((m.start(), header_lo, brace, m.group(1), paths[-1] if paths else ""))
    classes.sort()

    import bisect

    results = []
    anomaly = []
    for i, a in enumerate(anns):
        if a[2] not in VERBS:
            continue
        lo = hi = i
        while lo > 0 and gap_ws(text, anns[lo - 1][1], anns[lo][0]):
            lo -= 1
        while hi + 1 < len(anns) and gap_ws(text, anns[hi][1], anns[hi + 1][0]):
            hi += 1
        # sanity: the block must be followed by a method signature keyword
        tail = text[anns[hi][1]:]
        sig = re.match(r"\s*(?:public|protected|private)\b", tail)
        if not sig:
            anomaly.append(("no-signature-after-verb", a[2]))
            continue
        ci = bisect.bisect_right([c[0] for c in classes], a[0]) - 1
        if ci < 0:
            anomaly.append(("no-class-for-verb", a[2]))
            continue
        _, _, _, cname, cpath = classes[ci]
        block = text[anns[lo][0]:anns[hi][1]]
        mpaths = PATH_ARG_RE.findall(block)
        mpath = mpaths[-1] if mpaths else ""
        # adjacent verbs chained into one block share the method; emit once
        verbs_in_block = [x[2] for x in anns[lo:hi + 1] if x[2] in VERBS]
        if verbs_in_block[0] != a[2]:
            continue
        for v in verbs_in_block:
            results.append((cname, cpath, v, mpath))
    return results, anomaly


def join_and_normalize(class_path, method_path):
    parts = []
    for seg in (class_path, method_path):
        seg = seg.strip().strip("/")
        if seg:
            parts.append(seg)
    p = "/" + "/".join(parts) if parts else "/"
    p = PARAM_NORM_RE.sub("{}", p)
    p = re.sub("/{2,}", "/", p)
    if len(p) > 1:
        p = p.rstrip("/")
    return p if p else "/"


def iter_jaxrs_files(module_dir):
    for dirpath, _dirnames, filenames in os.walk(module_dir):
        rel = os.path.relpath(dirpath, module_dir)
        parts = rel.split(os.sep)
        # only canonical sources; skip mirrors like webapp/describe/sources
        if parts[:3] != ["src", "main", "java"]:
            continue
        if "jaxrs" not in parts:
            continue
        for fn in filenames:
            if fn.endswith(".java"):
                yield os.path.join(dirpath, fn)


def build_inventory(source_root):
    modules = {}
    skipped_modules = []
    for entry in sorted(os.listdir(source_root)):
        mdir = os.path.join(source_root, entry)
        if not entry.startswith("x_") or not os.path.isdir(mdir):
            continue
        endpoints = {}
        verb_total = 0
        records = 0
        files_n = 0
        anomalies = []
        for fpath in iter_jaxrs_files(mdir):
            try:
                raw = open(fpath, encoding="utf-8", errors="replace").read()
            except OSError as exc:
                anomalies.append(("unreadable-file", str(exc)))
                continue
            files_n += 1
            verb_total += len(VERB_COUNT_RE.findall(raw))
            text = strip_comments(raw)
            try:
                found, anoms = parse_file(text)
            except Exception as exc:  # fail loud per file, keep going
                anomalies.append((os.path.relpath(fpath, source_root), repr(exc)))
                continue
            anomalies.extend(anoms)
            for _cname, cpath, verb, mpath in found:
                records += 1
                key = (verb, join_and_normalize(cpath, mpath))
                endpoints.setdefault(key, 0)
                endpoints[key] += 1
        mod_entry = {
            "java_files_scanned": files_n,
            "verb_annotations": verb_total,
            "endpoint_records": records,
            "unique_endpoints": len(endpoints),
            "duplicate_collisions": records - len(endpoints),
            "endpoints": [
                {"method": v, "path": p}
                for (v, p) in sorted(endpoints, key=lambda k: (k[1], k[0]))
            ],
        }
        if anomalies:
            mod_entry["anomalies"] = anomalies[:20]
            mod_entry["anomaly_count"] = len(anomalies)
        modules[entry] = mod_entry
        if files_n == 0:
            skipped_modules.append(entry)

    totals = {
        "modules_with_jaxrs_files": sum(1 for m in modules.values() if m["java_files_scanned"] > 0),
        "modules_without_jaxrs": len(skipped_modules),
        "verb_annotations": sum(m["verb_annotations"] for m in modules.values()),
        "endpoint_records": sum(m["endpoint_records"] for m in modules.values()),
        "unique_endpoints": sum(m["unique_endpoints"] for m in modules.values()),
        "duplicate_collisions": sum(m["duplicate_collisions"] for m in modules.values()),
    }
    return {
        "generated_at": datetime.datetime.now().isoformat(timespec="seconds"),
        "source_root": source_root.replace("\\", "/"),
        "normalization": {
            "path_param": "{}",
            "rules": [
                "only src/main/java trees scanned (webapp/describe/sources mirrors excluded)",
                "class @Path + method @Path joined with '/'",
                "path params {x} (incl. {x:regex}) normalized to {}",
                "duplicate slashes collapsed, trailing slash stripped",
                "dedup key = (HTTP method, normalized path)",
            ],
        },
        "modules": modules,
        "totals": totals,
    }


def main():
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--source-root",
        default=os.path.join(repo_root, "oa", "o2server"),
        help="path to o2server source root (default: %(default)s)",
    )
    ap.add_argument(
        "--out",
        default=os.path.join(repo_root, "docs", "audits", "java-endpoint-inventory.json"),
        help="output json path (default: %(default)s)",
    )
    args = ap.parse_args()
    if not os.path.isdir(args.source_root):
        print(f"ERROR: source root not found: {args.source_root}", file=sys.stderr)
        return 2
    inv = build_inventory(args.source_root)
    os.makedirs(os.path.dirname(os.path.abspath(args.out)), exist_ok=True)
    with open(args.out, "w", encoding="utf-8") as fh:
        json.dump(inv, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    t = inv["totals"]
    print(f"inventory written: {args.out}")
    print(
        f"modules scanned={t['modules_with_jaxrs_files']} "
        f"(no-jaxrs={t['modules_without_jaxrs']}) | "
        f"legacy verb annotations={t['verb_annotations']} | "
        f"unique endpoints={t['unique_endpoints']} | "
        f"records={t['endpoint_records']} | dup-collisions={t['duplicate_collisions']}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
