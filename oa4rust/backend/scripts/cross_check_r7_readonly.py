#!/usr/bin/env python3
"""R7 只读挂写方法交叉验证：对 write_routes_readonly_on_writemethod.tsv 中
R7 桶的全部 handler 提取函数体，剥离注释/字符串后检查写 SQL 关键字
（INSERT/UPDATE/DELETE + sea_orm ORM 写）。输出疑似含写但被尺子判只读的候选，
供人工复核。这是对尺子 writeEvidence 的独立交叉验证（简化判据）。
"""
import re
import sys
from pathlib import Path

BACKEND = Path(__file__).resolve().parents[1]
CRATES = BACKEND / "crates"
ARTIFACT = BACKEND / "docs/audits/artifacts/write_routes_readonly_on_writemethod.tsv"

WRITE_RE = re.compile(
    r"\b(INSERT\s+INTO|DELETE\s+FROM|UPDATE\s+(?:\\?[\"`]?[A-Za-z_]\w*|\{))",
    re.IGNORECASE,
)
ENTITY_WRITE_RE = re.compile(
    r"\bEntity::(insert|insert_many|update_many|delete_many|delete_by_id)\s*\("
)
ACTIVE_MODEL_RE = re.compile(
    r"let\s+(?:mut\s+)?([a-z_]\w*)\s*:\s*(?:[\w:]+::)?ActiveModel\s*="
)


def strip_rust(src: str) -> str:
    out = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            while i < n and src[i] != "\n":
                i += 1
        elif c == "/" and i + 1 < n and src[i + 1] == "*":
            i += 2
            while i + 1 < n and not (src[i] == "*" and src[i + 1] == "/"):
                i += 1
            i += 1
        elif c == '"':
            out.append(c)
            i += 1
            while i < n:
                out.append(src[i])
                if src[i] == "\\":
                    i += 1
                    if i < n:
                        out.append(src[i])
                        i += 1
                    continue
                if src[i] == '"':
                    i += 1
                    break
                i += 1
        elif c == "'":
            out.append(c)
            i += 1
            while i < n:
                out.append(src[i])
                if src[i] == "\\":
                    i += 1
                    if i < n:
                        out.append(src[i])
                        i += 1
                    continue
                if src[i] == "'":
                    i += 1
                    break
                i += 1
        else:
            out.append(c)
        i += 1
    return "".join(out)


def find_fn_body(text: str, name: str) -> str | None:
    needle = f"fn {name}("
    pos = 0
    while True:
        rel = text.find(needle, pos)
        if rel < 0:
            return None
        # 边界：fn 前不能是标识符字符
        if rel > 0 and (text[rel - 1].isalnum() or text[rel - 1] in "_:"):
            pos = rel + len(needle)
            continue
        start = rel + len(needle)  # '(' 之后
        depth = 1
        i = start
        in_str = in_char = esc = False
        while i < len(text) and depth > 0:
            c = text[i]
            if in_str:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == '"':
                    in_str = False
            elif in_char:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == "'":
                    in_char = False
            else:
                if c == '"':
                    in_str = True
                elif c == "'":
                    in_char = True
                elif c == "(":
                    depth += 1
                elif c == ")":
                    depth -= 1
                elif c == "/" and i + 1 < len(text) and text[i + 1] == "/":
                    while i < len(text) and text[i] != "\n":
                        i += 1
                elif c == "/" and i + 1 < len(text) and text[i + 1] == "*":
                    i += 2
                    while i + 1 < len(text) and not (text[i] == "*" and text[i + 1] == "/"):
                        i += 1
            i += 1
        if depth != 0:
            pos = rel + len(needle)
            continue
        # 参数结束，找 '{'
        j = i
        while j < len(text):
            c = text[j]
            if c == "/" and j + 1 < len(text) and text[j + 1] == "/":
                while j < len(text) and text[j] != "\n":
                    j += 1
            elif c == "/" and j + 1 < len(text) and text[j + 1] == "*":
                j += 2
                while j + 1 < len(text) and not (text[j] == "*" and text[j + 1] == "/"):
                    j += 1
            elif c == "{":
                break
            elif c == "}":
                pos = rel + len(needle)
                break
            j += 1
        if j >= len(text) or text[j] != "{":
            pos = rel + len(needle)
            continue
        # 括号配平提取 body
        depth = 0
        k = j
        in_str = in_char = esc = False
        while k < len(text):
            c = text[k]
            if in_str:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == '"':
                    in_str = False
            elif in_char:
                if esc:
                    esc = False
                elif c == "\\":
                    esc = True
                elif c == "'":
                    in_char = False
            else:
                if c == '"':
                    in_str = True
                elif c == "'":
                    in_char = True
                elif c == "{":
                    depth += 1
                elif c == "}":
                    depth -= 1
                    if depth == 0:
                        return text[rel : k + 1]
                elif c == "/" and k + 1 < len(text) and text[k + 1] == "/":
                    while k < len(text) and text[k] != "\n":
                        k += 1
                elif c == "/" and k + 1 < len(text) and text[k + 1] == "*":
                    k += 2
                    while k + 1 < len(text) and not (text[k] == "*" and text[k + 1] == "/"):
                        k += 1
            k += 1
        pos = rel + len(needle)


def crate_files(crate: str):
    src = CRATES / crate / "src"
    if not src.is_dir():
        return
    for p in sorted(src.rglob("*.rs")):
        yield p


def all_fn_bodies(name: str):
    """全仓扫描同名 fn，返回 [(crate, body), ...]（按 crate 排序）。

    与尺子缺陷 30 对齐：R7 表里的 handler 可能是跨 crate 委托
    （如 organization_assemble_authentication 的 `auth::sso::*`），
    在路由所在 crate 内没有字面函数体。回退到全仓查找，但只接受
    **唯一同名命中**，避免同名碰撞导致用错函数体。
    """
    hits = []
    for crate_dir in sorted(CRATES.iterdir()):
        if not crate_dir.is_dir():
            continue
        src = crate_dir / "src"
        if not src.is_dir():
            continue
        for p in sorted(src.rglob("*.rs")):
            b = find_fn_body(p.read_text(encoding="utf-8", errors="replace"), name)
            if b:
                hits.append((crate_dir.name, b))
    return hits


def has_write(body: str) -> bool:
    code = strip_rust(body)
    if WRITE_RE.search(code):
        return True
    if ENTITY_WRITE_RE.search(code):
        return True
    for m in ACTIVE_MODEL_RE.finditer(code):
        var = m.group(1)
        pat = re.compile(rf"\b{var}\.(insert|update|delete|save)\s*\(")
        if pat.search(code):
            return True
    return False


def main():
    rows = []
    with open(ARTIFACT, encoding="utf-8") as f:
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if len(parts) == 5 and parts[4] == "R7":
                rows.append(parts)

    candidates = []
    checked = 0
    cross_crate = 0
    unresolved = []
    cache = {}
    for crate, method, path, handler, _bucket in rows:
        if (crate, handler) not in cache:
            body = None
            for p in crate_files(crate):
                txt = p.read_text(encoding="utf-8", errors="replace")
                b = find_fn_body(txt, handler)
                if b:
                    body = b
                    break
            if body is None:
                # 尺子缺陷 30 对齐：跨 crate 委托 handler 在路由 crate 内无函数体，
                # 全仓回退找唯一同名命中（同名碰撞时宁缺勿滥，记入 unresolved）。
                hits = all_fn_bodies(handler)
                if len(hits) == 1:
                    body = hits[0][1]
                    cross_crate += 1
            cache[(crate, handler)] = body
        body = cache[(crate, handler)]
        if body is None:
            unresolved.append((crate, method, path, handler))
            continue
        checked += 1
        if has_write(body):
            candidates.append((crate, method, path, handler))

    print(
        f"R7 只读挂写交叉验证：{len(rows)} 条，成功提取 {checked} 条函数体"
        f"（其中跨 crate 回退 {cross_crate} 条）"
    )
    if unresolved:
        print(f"未提取 {len(unresolved)} 条（同名碰撞或全仓无此函数）：")
        for u in unresolved:
            print("\t".join(u))
    if candidates:
        # 门禁判据（check-warnings.mjs）：输出含「疑似含写候选」即失败。
        print(f"疑似含写候选 {len(candidates)} 条（门禁失败）：")
        for c in candidates:
            print("\t".join(c))
        return 1
    print("通过：0 条疑似含写")
    return 0


if __name__ == "__main__":
    sys.exit(main())