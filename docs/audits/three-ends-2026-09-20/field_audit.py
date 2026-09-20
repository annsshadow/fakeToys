#!/usr/bin/env python3
"""字段一致性审计：
1) 构建后端"JSON 键词表"——所有 handler 实际产出的键名（json! 字面量键、Map 键、
   serde camelCase 结构体字段、row_to_json 的原样列名）。
2) 提取前端视图声明的响应字段（interface/type 字面量）。
3) 报告"前端声明但后端从不产出"的字段。
"""
import json
import os
import re

def find_repo_root(start):
    """向上查找同时包含 oa4rust 与 oa4rust-web 的目录，使脚本可任意位置运行。"""
    cur = os.path.abspath(start)
    while True:
        if os.path.isdir(os.path.join(cur, "oa4rust")) and os.path.isdir(os.path.join(cur, "oa4rust-web")):
            return cur
        parent = os.path.dirname(cur)
        if parent == cur:
            raise SystemExit("未找到仓库根（需同时存在 oa4rust 与 oa4rust-web）")
        cur = parent


ROOT = find_repo_root(os.path.dirname(os.path.abspath(__file__)))
RUST = os.path.join(ROOT, "oa4rust")
WEB = os.path.join(ROOT, "oa4rust-web")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "field_audit.json")

KEY_JSON = re.compile(r'"([A-Za-z_][A-Za-z0-9_]*)"\s*:')
KEY_MAP = re.compile(r'\(\s*"([A-Za-z_][A-Za-z0-9_]*)"\s*\.to_string\(\)\s*,')
KEY_MAP2 = re.compile(r'\(\s*"([A-Za-z_][A-Za-z0-9_]*)"\.into\(\)\s*,')
SERDE_STRUCT = re.compile(r'#\[serde\(rename_all\s*=\s*"(\w+)"\)\]\s*(?:#\[[^\]]*\]\s*)*pub struct \w+\s*\{([^}]*)\}', re.DOTALL)
PLAIN_STRUCT = re.compile(r'pub struct \w+\s*\{([^}]*)\}', re.DOTALL)


def snake_to_camel(s):
    parts = s.split("_")
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])


def backend_vocab():
    vocab = {}
    for base in (os.path.join(RUST, "crates"), os.path.join(RUST, "src")):
        for dp, _d, fs in os.walk(base):
            if os.sep + "target" in dp:
                continue
            for f in fs:
                if not f.endswith(".rs"):
                    continue
                p = os.path.join(dp, f)
                rel = os.path.relpath(p, RUST).replace("\\", "/")
                src = open(p, encoding="utf-8", errors="replace").read()
                for rx in (KEY_JSON, KEY_MAP, KEY_MAP2):
                    for m in rx.finditer(src):
                        vocab.setdefault(m.group(1), set()).add(rel)
                for m in SERDE_STRUCT.finditer(src):
                    style = m.group(1)
                    for fm in re.finditer(r'\bpub\s+(\w+)\s*:', m.group(2)):
                        name = fm.group(1)
                        if style == "camelCase":
                            name = snake_to_camel(name)
                        vocab.setdefault(name, set()).add(rel)
    return vocab


def extract_iface_fields(src):
    """提取 interface/type 字面量里的顶层字段名。"""
    fields = {}
    for m in re.finditer(r'\b(?:interface|type)\s+([A-Za-z_]\w*)[^={]*[={]\s*\{', src):
        name = m.group(1)
        start = src.index("{", m.end() - 1)
        depth, i = 0, start
        while i < len(src):
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        body = src[start + 1 : i]
        # 只取顶层（深度 0）的 key:
        top, d = [], 0
        for line in body.split("\n"):
            stripped = line.strip()
            if d == 0:
                fm = re.match(r'([A-Za-z_$][\w$]*)\s*\??\s*:', stripped)
                if fm:
                    top.append(fm.group(1))
            d += line.count("{") + line.count("[") - line.count("}") - line.count("]")
        fields[name] = top
    return fields


def main():
    vocab = backend_vocab()
    print(f"后端 JSON 键词表规模: {len(vocab)}")

    report = {}
    for app, sub in (("desktop", "desktop"), ("mobile", "mobile")):
        rows = []
        base = os.path.join(WEB, "apps", sub, "src")
        for dp, _d, fs in os.walk(base):
            if "node_modules" in dp:
                continue
            for f in fs:
                if not f.endswith((".vue", ".ts")):
                    continue
                if f.endswith((".test.ts", ".spec.ts")):
                    continue
                p = os.path.join(dp, f)
                rel = os.path.relpath(p, WEB).replace("\\", "/")
                src = open(p, encoding="utf-8", errors="replace").read()
                for iface, fields in extract_iface_fields(src).items():
                    missing = [x for x in fields if x not in vocab]
                    if missing:
                        rows.append({"file": rel, "iface": iface, "declared": len(fields), "missing": missing})
        report[app] = rows

    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump({"vocab_size": len(vocab), "report": report}, fh, ensure_ascii=False, indent=1)

    for app, rows in report.items():
        tot_missing = sum(len(r["missing"]) for r in rows)
        print(f"\n=== {app}: {len(rows)} 个接口存在'后端无此键'字段，共 {tot_missing} 个字段 ===")
        for r in rows:
            print(f"  {r['file']}  <{r['iface']}> ({len(r['missing'])}/{r['declared']}): {', '.join(r['missing'][:14])}")


if __name__ == "__main__":
    main()
