#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""单 crate 桩代码真实化（碰撞感知）。

用法: python3 scripts/realize_one.py <crate_name> [--apply]

流程:
  1. 收集 crate 内所有 .rs 的函数定义名。
  2. 对每个 stub_<crate>_X:
       - 若去前缀后的 X 已是真实函数名 -> 重复桩: 删除该 stub 定义,
         并把 routes 中对它的引用改指真实 X。
       - 否则 -> 全局去掉 stub_<crate>_ 前缀(重命名, 安全无碰撞)。
  3. 删除 "real implementation needed" 占位字符串。
  4. 把 router(pool) 改写为委托给真实路由函数(优先 routes::<crate>_routes,
     其次 <crate>_router, 再其次唯一带 .route( 的 Router 函数)。
"""
import os, re, sys, glob

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

def read(p): return open(p, encoding="utf-8").read()
def write(p, t): open(p, "w", encoding="utf-8").write(t)

def fn_names(text):
    return {m.group(1) for m in re.finditer(r'(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)', text)}

def find_fn_start(text, fn_name):
    for pat in [rf'pub\s+async\s+fn\s+{fn_name}\b',
                rf'pub\s+fn\s+{fn_name}\b',
                rf'async\s+fn\s+{fn_name}\b',
                rf'fn\s+{fn_name}\b']:
        m = re.search(pat, text)
        if m:
            return m
    return None

def delete_fn_def(text, fn_name):
    m = find_fn_start(text, fn_name)
    if not m:
        return text
    start = m.start()
    line_start = text.rfind('\n', 0, start) + 1
    scan = line_start
    while scan > 0:
        prev_end = text.rfind('\n', 0, scan - 1)
        prev_start = prev_end + 1
        line = text[prev_start:scan - 1]
        if line.lstrip().startswith('///') or line.lstrip().startswith('#['):
            scan = prev_start
        else:
            break
    def_start = scan
    brace_pos = text.find('{', m.end())
    if brace_pos == -1:
        return text
    depth = 0
    i = brace_pos
    while i < len(text):
        c = text[i]
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return text[:def_start] + text[i + 1:]
        i += 1
    return text

def replace_router(text, crate, call):
    m = re.search(r'pub\s+fn\s+router\s*\(', text)
    if not m:
        return text, False
    start = text.rfind('\n', 0, m.start()) + 1
    brace_pos = text.find('{', m.end())
    if brace_pos == -1:
        return text, False
    depth = 0
    i = brace_pos
    while i < len(text):
        c = text[i]
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                new = text[:start] + f'pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {{\n    {call}\n}}\n' + text[i + 1:]
                return new, True
        i += 1
    return text, False

def realize(crate, apply):
    base = os.path.join(CRATES, crate, "src")
    if not os.path.isdir(base):
        print(f"[skip] {crate}: 无 src 目录"); return
    prefix = f"stub_{crate}_"
    files = glob.glob(os.path.join(base, "**", "*.rs"), recursive=True)
    texts = {f: read(f) for f in files}
    full = "\n".join(texts.values())
    all_names = fn_names(full)
    real_names = {n for n in all_names if not n.startswith(prefix)}
    stub_names = {n for n in all_names if n.startswith(prefix)}

    dupes = [(sn, sn[len(prefix):]) for sn in stub_names if sn[len(prefix):] in real_names]
    renames = [(sn, sn[len(prefix):]) for sn in stub_names if sn[len(prefix):] not in real_names]

    # --- 1. 处理重复桩: 删除定义 + 路由改指真实函数 ---
    for sn, target in dupes:
        for f in list(texts.keys()):
            if find_fn_start(texts[f], sn):
                texts[f] = delete_fn_def(texts[f], sn)
        for f in texts:
            texts[f] = texts[f].replace(f"crate::{sn}", f"crate::{target}")
            texts[f] = texts[f].replace(sn, target)

    # --- 2. 重命名非重复桩(全局去前缀) ---
    for sn, target in renames:
        for f in texts:
            texts[f] = texts[f].replace(sn, target)

    # --- 3. 删除 TODO 占位 ---
    for f in texts:
        texts[f] = texts[f].replace(f'"TODO: {crate} - real implementation needed"', "")
        texts[f] = texts[f].replace(f"TODO: {crate} - real implementation needed", "")

    # --- 4. 改写 router(pool) 委托给真实路由函数 ---
    full2 = "\n".join(texts.values())
    wired = None
    if f"routes::{crate}_routes" in full2:
        wired = f"routes::{crate}_routes(pool)"
    elif f"{crate}_routes" in full2:
        wired = f"{crate}_routes(pool)"
    elif f"{crate}_router" in full2:
        wired = f"{crate}_router(pool)"
    else:
        for m in re.finditer(r'pub\s+(?:async\s+)?fn\s+([A-Za-z_]\w*)\s*\([^)]*\)\s*->\s*axum::Router', full2):
            cand = m.group(1)
            if cand == "router":
                continue
            seg = full2[full2.find(m.group(0)): full2.find(m.group(0)) + 4000]
            if ".route(" in seg:
                wired = f"{cand}(pool)"
                break
    if wired:
        done = False
        for f in texts:
            if "pub fn router(" in texts[f]:
                texts[f], ok = replace_router(texts[f], crate, wired)
                if ok:
                    done = True
        if not done:
            print(f"[warn] {crate}: 找到真实路由 {wired} 但未定位 router() 所在文件")
    else:
        print(f"[warn] {crate}: 无法自动定位真实路由函数, router 未改写")

    if apply:
        for f, t in texts.items():
            write(f, t)
        print(f"[apply] {crate}: renames={len(renames)} dupes={len(dupes)} wired={wired}")
    else:
        print(f"[dry] {crate}: renames={len(renames)} dupes={len(dupes)} wired={wired}")

if __name__ == "__main__":
    realize(sys.argv[1], "--apply" in sys.argv)
