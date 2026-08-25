#!/usr/bin/env python3
"""Heuristic fixer for corrupted-CJK '?' in translated kernel docs.

Rules (conservative):
  - '?' preceded by CJK AND at end-of-line        -> '。'
  - '?' preceded by CJK AND followed by CJK/space+CJK -> '，'
  - everything else -> left untouched, logged as UNRESOLVED
Every replacement is recorded in a markdown log.
"""
import sys
import re
from pathlib import Path

src = Path(sys.argv[1])
log_path = Path(sys.argv[2])
text = src.read_text(encoding="utf-8", errors="replace")
lines = text.split("\n")

CJK = re.compile(r"[\u2e80-\u9fff\uff01-\uff65]")
in_code = False
out = []
fixed = []
unresolved = []

for idx, line in enumerate(lines, 1):
    if line.strip().startswith("```"):
        in_code = not in_code
        out.append(line)
        continue
    if in_code or "?" not in line:
        out.append(line)
        continue

    new_line = list(line)
    changed_positions = []
    for m in re.finditer(r"\?", line):
        pos = m.start()
        before = line[pos - 1] if pos > 0 else " "
        after = line[pos + 1] if pos + 1 < len(line) else ""
        if ord(before) <= 0x2E7F:
            continue  # not CJK-preceded -> legit English question
        nxt = after.lstrip()
        if nxt == "":
            repl = "。"
        elif CJK.match(nxt[0]):
            repl = "，"
        elif nxt[0] in "\"'“”‘’)]}":
            repl = "。"
        else:
            unresolved.append((idx, line.strip()[:80]))
            continue
        new_line[pos] = repl
        changed_positions.append((pos + 1, repl))
    if changed_positions:
        fixed.append((idx, line.strip()[:70], "".join(new_line).strip()[:70], changed_positions))
    out.append("".join(new_line))

src.write_text("\n".join(out), encoding="utf-8")

log_lines = [f"# 翻译损坏字符修复日志 — {src.name}", "",
             f"- 启发式修复 {len(fixed)} 处；未解决 {len(unresolved)} 处", ""]
for ln, old, new, chs in fixed:
    log_lines.append(f"- L{ln}: {len(chs)} 处替换 {'/'.join(c[1] for c in chs)}")
    log_lines.append(f"  - 原: {old}")
    log_lines.append(f"  - 新: {new}")
if unresolved:
    log_lines += ["", "## 未解决（需人工对照上游）"]
    for ln, t in unresolved:
        log_lines.append(f"- L{ln}: {t}")
log_path.write_text("\n".join(log_lines), encoding="utf-8")
total_repl = sum(len(f[3]) for f in fixed)
print(f"fixed={len(fixed)} lines, {total_repl} replacements")
print(f"unresolved={len(unresolved)}")
print(f"log={log_path}")
