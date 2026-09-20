#!/usr/bin/env python3
"""修复「转义引号键缺陷」：Rust 源码里形如 "\"key\"" / {\"key\"} 的字面量。

背景（见 docs/plans/2026-09-20-001-... §6.7）：
  后端源码里大量出现 `"\"xcreateTime\""` —— 键名里字面包含引号。
  后果：row.get 读不存在的列（panic 500）、响应键前端读不到、请求键读不到、
        路由路径参数名畸形。

本脚本做两类替换（语义等价：把多包的一层引号去掉）：
  A. 路由路径参数：  {\"name\"}      ->  {name}
  B. 字符串字面量：  "\"name\""      ->  "name"

用法：
  python fix_quoted_keys.py --dry-run   # 只报告
  python fix_quoted_keys.py             # 实际写入
"""
import argparse
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def find_repo_root(start):
    cur = os.path.abspath(start)
    while True:
        if os.path.isdir(os.path.join(cur, "oa4rust")):
            return cur
        parent = os.path.dirname(cur)
        if parent == cur:
            raise SystemExit("未找到仓库根")
        cur = parent


ROOT = find_repo_root(HERE)
RUST = os.path.join(ROOT, "oa4rust")

# 路由路径参数名：{\"name\"}
RE_ROUTE_PARAM = re.compile(r'\{\\"([A-Za-z_][\w]*)\\"\}')
# 字符串字面量：\"name\" 包在引号里 —— 即源码文本 "\"name\""
RE_QUOTED_KEY = re.compile(r'"\\"([A-Za-z_][\w]*)\\""')

# 排除：SQL 标识符引用的合法用法（quote_ident 的返回值/断言）。
# 例：assert_eq!(d.quote_ident("clueId"), "\"clueId\"");  ← 这里的引号是 SQL 语义，正确
SKIP_LINE_MARKERS = ("quote_ident",)


def fix_line(line, stats):
    """对单行做替换；跳过合法 SQL 标识符引用行。"""
    if any(mk in line for mk in SKIP_LINE_MARKERS):
        stats["skipped"] += 1
        return line
    new, a = RE_ROUTE_PARAM.subn(r"{\1}", line)
    new, b = RE_QUOTED_KEY.subn(r'"\1"', new)
    stats["route"] += a
    stats["key"] += b
    return new


def targets():
    out = []
    for base in (os.path.join(RUST, "crates"), os.path.join(RUST, "src")):
        for dp, _d, fs in os.walk(base):
            if os.sep + "target" in dp:
                continue
            for f in fs:
                if f.endswith(".rs"):
                    out.append(os.path.join(dp, f))
    return sorted(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    total_a = total_b = total_skipped = 0
    changed_files = []
    for p in targets():
        src = open(p, encoding="utf-8", errors="replace").read()
        stats = {"route": 0, "key": 0, "skipped": 0}
        out_lines = [fix_line(l, stats) for l in src.split("\n")]
        total_skipped += stats["skipped"]
        if stats["route"] or stats["key"]:
            rel = os.path.relpath(p, RUST).replace("\\", "/")
            changed_files.append((rel, stats["route"], stats["key"]))
            total_a += stats["route"]
            total_b += stats["key"]
            if not args.dry_run:
                with open(p, "w", encoding="utf-8", newline="") as fh:
                    fh.write("\n".join(out_lines))

    print(f"{'[dry-run] ' if args.dry_run else ''}受影响文件 {len(changed_files)} 个")
    print(f"  路由参数名替换 {total_a} 处 / 字符串键替换 {total_b} 处 / 合计 {total_a + total_b}")
    print(f"  跳过的合法 SQL 标识符引用行 {total_skipped} 行")
    for rel, a, b in sorted(changed_files, key=lambda x: -(x[1] + x[2])):
        print(f"  {a + b:>4}  (route {a}, key {b})  {rel}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
