#!/usr/bin/env python3
"""提取 oa4rust-web 桌面端 / 移动端对后端的 API 调用路径。

增强点：
- 括号平衡扫描（正确处理嵌套泛型 get<Record<string,unknown>>(）
- 解析文件内 `const NAME = '...'` 常量
- 解析对象字面量映射 `obj = { key: '...', fn: (a) => `...` }`
- 未解析成功的动态调用单独登记（fail loud，不静默丢弃）
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


WEB = os.path.join(find_repo_root(os.path.dirname(os.path.abspath(__file__))), "oa4rust-web")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "frontend_calls.json")

CLIENTS = ("api", "mapi", "http", "client", "req")
VERBS = ("get", "post", "put", "delete", "patch", "upload", "download")
ENTRY_RE = re.compile(r"\b(" + "|".join(CLIENTS) + r")\s*\.\s*(" + "|".join(VERBS) + r")\b")
CONST_RE = re.compile(r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^=\n]*)?=\s*([`'\"])(.*?)\2")
OBJ_STR_RE = re.compile(r"\b([A-Za-z_$][\w$]*)\s*:\s*([`'\"])(.*?)\2")
OBJ_FN_RE = re.compile(r"\b([A-Za-z_$][\w$]*)\s*:\s*(?:async\s*)?\([^)]*\)\s*=>\s*([`'\"])(.*?)\2")
OBJ_BLOCK_RE = re.compile(r"\b([A-Za-z_$][\w$]*)\s*(?::[^={;\n]*)?=\s*\{", re.DOTALL)


def skip_ws(s, i):
    while i < len(s) and s[i] in " \t\r\n":
        i += 1
    return i


def skip_angle(s, i):
    depth = 0
    while i < len(s):
        if s[i] == "<":
            depth += 1
        elif s[i] == ">":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return i


def read_first_string(s, i):
    i = skip_ws(s, i)
    if i >= len(s) or s[i] not in "`'\"":
        return None
    q = s[i]
    j = i + 1
    buf = []
    while j < len(s):
        c = s[j]
        if c == "\\":
            buf.append(s[j : j + 2])
            j += 2
            continue
        if c == q:
            return "".join(buf)
        buf.append(c)
        j += 1
    return None


def normalize(p):
    # 先替换模板表达式（其中可能含 '?' 三元运算符），再去查询串 —— 顺序不可颠倒
    p = re.sub(r"\$\{[^}]*\}", "{}", p)
    p = p.split("?")[0]
    p = re.sub(r"\{[^}]*\}", "{}", p)
    segs = p.split("/")
    segs = ["{}" if s.startswith(":") and len(s) > 1 else s for s in segs]
    while segs and segs[-1] == "":
        segs.pop()
    return "/" + "/".join(segs[1:]) if segs and segs[0] == "" else p


def build_maps(src):
    """返回 (constmap, objmap, objvals)。objvals[name] = 该对象字面量的全部字符串值。"""
    constmap, objmap, objvals = {}, {}, {}
    for m in CONST_RE.finditer(src):
        constmap[m.group(1)] = m.group(3)
    for ob in OBJ_BLOCK_RE.finditer(src):
        start = ob.end()
        depth = 1
        i = start
        while i < len(src) and depth > 0 and i - start < 20000:
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
            i += 1
        body = src[start:i]
        name = ob.group(1)
        for m in OBJ_FN_RE.finditer(body):
            objmap[name + "." + m.group(1)] = m.group(3)
            objvals.setdefault(name, []).append(m.group(3))
        for m in OBJ_STR_RE.finditer(body):
            key = name + "." + m.group(1)
            objmap.setdefault(key, m.group(3))
            objvals.setdefault(name, []).append(m.group(3))
    return constmap, objmap, objvals


def resolve(src, i, constmap, objmap, objvals):
    """i 指向 '(' 之后；返回 (raw_list, via) 或 (None, reason)。raw_list 支持展开。"""
    j = skip_ws(src, i)
    lit = read_first_string(src, j)
    if lit is not None:
        return [lit], "literal"
    m = re.match(r"([A-Za-z_$][\w$]*)", src[j:])
    if m:
        name = m.group(1)
        after = skip_ws(src, j + len(name))
        if after < len(src) and src[after] == "[":
            vals = objvals.get(name)
            if vals:
                return list(dict.fromkeys(vals)), "expand:" + name
            return None, "unresolved-index:" + name
        if after < len(src) and src[after] == ".":
            m2 = re.match(r"\.\s*([A-Za-z_$][\w$]*)", src[after:])
            if m2:
                key = name + "." + m2.group(1)
                k2 = skip_ws(src, after + len(m2.group(0)))
                if k2 < len(src) and src[k2] == "(":  # 函数调用形态
                    if key in objmap:
                        return [objmap[key]], "obj-fn:" + key
                    return None, "unresolved-objfn:" + key
                if key in objmap:
                    return [objmap[key]], "obj:" + key
                return None, "unresolved-obj:" + key
        if name in constmap:
            return [constmap[name]], "const:" + name
        return None, "unresolved-var:" + name
    return None, "dynamic"


def load_global_maps(app_dir):
    """合并 contracts/ 下导出的端点表（如 designerPaths），供跨文件引用解析。"""
    constmap, objmap, objvals = {}, {}, {}
    for dirpath, _dirs, files in os.walk(app_dir):
        if "contracts" not in dirpath and "utils" not in dirpath and "constants" not in dirpath:
            continue
        for f in files:
            if not f.endswith(".ts"):
                continue
            src = open(os.path.join(dirpath, f), encoding="utf-8", errors="replace").read()
            c, o, ov = build_maps(src)
            constmap.update(c)
            objmap.update(o)
            for k, v in ov.items():
                objvals.setdefault(k, []).extend(v)
    return constmap, objmap, objvals


def scan(app_dir, label):
    rows, unresolved = [], []
    gconst, gobj, gvals = load_global_maps(app_dir)
    for dirpath, _dirs, files in os.walk(app_dir):
        if "node_modules" in dirpath or os.sep + "dist" in dirpath:
            continue
        for f in files:
            if not f.endswith((".ts", ".vue", ".js", ".tsx")):
                continue
            if f.endswith((".test.ts", ".spec.ts")):
                continue
            full = os.path.join(dirpath, f)
            rel = os.path.relpath(full, WEB).replace("\\", "/")
            src = open(full, encoding="utf-8", errors="replace").read()
            constmap, objmap, objvals = build_maps(src)
            for k, v in gconst.items():
                constmap.setdefault(k, v)
            for k, v in gobj.items():
                objmap.setdefault(k, v)
            for k, v in gvals.items():
                objvals.setdefault(k, v)
            for m in ENTRY_RE.finditer(src):
                verb = m.group(2).upper()
                i = skip_ws(src, m.end())
                if i < len(src) and src[i] == "<":
                    i = skip_ws(src, skip_angle(src, i))
                if i >= len(src) or src[i] != "(":
                    continue
                raws, via = resolve(src, i + 1, constmap, objmap, objvals)
                line = src[: m.start()].count("\n") + 1
                if raws is None:
                    unresolved.append(
                        {
                            "app": label,
                            "file": rel,
                            "line": line,
                            "method": verb,
                            "reason": via,
                            "snippet": src[m.start() : m.start() + 110].replace("\n", " "),
                        }
                    )
                    continue
                for raw in raws:
                    if not raw.startswith("/"):
                        continue
                    rows.append(
                        {
                            "app": label,
                            "file": rel,
                            "line": line,
                            "method": verb,
                            "raw": raw,
                            "path": normalize(raw),
                            "via": via,
                            # 动态索引展开（如 endpoints[tab]）无法判定分支，方法视为不确定
                            "ambiguous": via.startswith("expand:"),
                        }
                    )
    return rows, unresolved


def main():
    data, unres = {}, {}
    targets = (
        ("desktop", os.path.join(WEB, "apps", "desktop", "src")),
        ("mobile", os.path.join(WEB, "apps", "mobile", "src")),
        ("shared-ui", os.path.join(WEB, "packages", "ui", "src")),
        ("shared-sdk", os.path.join(WEB, "packages", "sdk", "src")),
    )
    for key, path in targets:
        rows, u = scan(path, key)
        data[key] = rows
        unres[key] = u
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump({"calls": data, "unresolved": unres}, fh, ensure_ascii=False, indent=1)
    for k, v in data.items():
        uniq = {(r["method"], r["path"]) for r in v}
        print(
            f"{k}: calls={len(v)} files={len({r['file'] for r in v})} unique(method,path)={len(uniq)} "
            f"unresolved={len(unres[k])}"
        )
    print("written:", OUT)


if __name__ == "__main__":
    main()
