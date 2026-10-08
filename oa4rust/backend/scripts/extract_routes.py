"""Extract all axum route registrations from the oa4rust workspace.

Walks crates/ (*.rs, excluding target), finds `.route("...", get/post/...)` and
`.nest("...", ...)` registrations, and emits a categorized JSON of endpoints so
the parity probe can enumerate the full surface area.

Usage:
    python scripts/extract_routes.py > routes.json
"""
import os
import re
import json
import sys

CRATES_DIR = "crates"

route_open_re = re.compile(r'\.route\(')
builder_re = re.compile(
    r'\.(get|post|put|delete|patch|head|options)\(\s*'
    r'("(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\')\s*,'
)
METHOD_RE = re.compile(r'\b(get|post|put|delete|patch|head|options)\s*\(', re.I)
nest_re = re.compile(r'\.nest\(\s*("(?:[^"\\]|\\.)*")\s*,')


def extract_balanced(s, paren_start):
    """Return the content inside the parenthesis at s[paren_start] (which is '(').

    String literals are skipped so parens inside paths/handlers are ignored.
    """
    depth = 0
    i = paren_start
    n = len(s)
    while i < n:
        c = s[i]
        if c == '(':
            depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0:
                return s[paren_start + 1:i]
        elif c == '"' or c == "'":
            quote = c
            i += 1
            while i < n and s[i] != quote:
                if s[i] == '\\':
                    i += 1
                i += 1
        i += 1
    return s[paren_start + 1:]


results = []  # (file, method, path)

for root, dirs, files in os.walk(CRATES_DIR):
    if "target" in root:
        continue
    for fn in files:
        if not fn.endswith(".rs"):
            continue
        path = os.path.join(root, fn)
        try:
            txt = open(path, encoding="utf-8", errors="replace").read()
        except Exception:
            continue
        for m in nest_re.finditer(txt):
            results.append((path, "NEST", m.group(1).strip('"')))
        # `.route("p", get(a).put(b))` —— 解析整段并提取所有 method
        for m in route_open_re.finditer(txt):
            paren_start = m.end() - 1
            body = extract_balanced(txt, paren_start)
            qm = re.match(r'\s*("(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\')', body)
            if not qm:
                continue
            rpath = qm.group(1).strip('"\'')
            rest = body[qm.end():]
            for mm in METHOD_RE.finditer(rest):
                results.append((path, mm.group(1).upper(), rpath))
        # 构建式 `.get("p", h)` / `.post("p", h)` 兜底
        for m in builder_re.finditer(txt):
            rpath = m.group(2).strip('"\'')
            if rpath.startswith('/'):
                results.append((path, m.group(1).upper(), rpath))

gets = [(f, m, p) for (f, m, p) in results if m == "GET"]
posts = [(f, m, p) for (f, m, p) in results if m == "POST"]
others = [(f, m, p) for (f, m, p) in results if m not in ("GET", "POST", "NEST")]
nests = [(f, m, p) for (f, m, p) in results if m == "NEST"]

get_paths = sorted(set(p for (_, _, p) in gets))
list_like = [
    p for p in get_paths
    if re.search(r"/(list|paging|all|get|count|query|search)", p)
]
proble = [
    p for p in list_like
    if "{" in p and not re.search(r"\{page\}/size/\{size\}", p)
]

out = {
    "total": len(results),
    "gets": len(gets),
    "posts": len(posts),
    "others": len(others),
    "nests": len(nests),
    "get_paths": get_paths,
    "list_like": list_like,
    "proble": proble,
    "nests_paths": sorted(set(p for (_, _, p) in nests)),
}

if __name__ == "__main__":
    if "--quiet" not in sys.argv:
        print(json.dumps({
            "total": out["total"],
            "gets": out["gets"],
            "posts": out["posts"],
            "others": out["others"],
            "nests": out["nests"],
            "unique_get": len(get_paths),
            "list_like": len(list_like),
            "proble": len(proble),
        }, indent=2))
    out_path = os.path.join(os.path.dirname(__file__), "routes.json")
    json.dump(out, open(out_path, "w"), ensure_ascii=False, indent=1)
    if "--quiet" in sys.argv:
        print(out_path)
