#!/usr/bin/env python3
"""Format consistency audit for docs/linux-7.1.3/**/*.md.

Checks (read-only over the docs tree):
  1. YAML frontmatter usage: coverage ratio + field-name frequency.
  2. RST toctree remnants (``.. toctree::`` directives) and TOC style split
     (markdown-list TOC vs no TOC structure).
  3. Heading issues: level jumps (e.g. h1 -> h3) and duplicate h1.

Outputs (names prefixed format-consistency-* to avoid clashing with
parallel link_audit.py outputs):
  - format-consistency-report.json  (full machine-readable data)
  - format-consistency-report.md    (human-readable summary)

Stdlib only, single process. Never writes inside docs/linux-7.1.3/.
"""

import json
import re
import time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

AUDIT_DIR = Path(__file__).resolve().parent
DOCS_ROOT = AUDIT_DIR.parent / "linux-7.1.3"

HEADING_RE = re.compile(r"^ {0,3}(#{1,6})\s+\S")
FENCE_RE = re.compile(r"^ {0,3}(`{3,}|~{3,})")
LIST_LINK_RE = re.compile(r"^ {0,3}[-*+]\s+\S*?\([^()]+\)")
TOCTREE_DIRECTIVE_RE = re.compile(r"^ {0,3}\.\.\s+toctree::")
TOCTREE_KEYWORD_RE = re.compile(r"\btoctree\b", re.IGNORECASE)
FIELD_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_-]*)\s*:")

MAX_FM_LINES = 100


def parse_frontmatter(text):
    """Return list of top-level field names if file starts with YAML
    frontmatter (first line exactly ``---``), else None."""
    if text.startswith("\ufeff"):
        text = text[1:]
    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        return None
    fields = []
    for line in lines[1 : 1 + MAX_FM_LINES]:
        stripped = line.strip()
        if stripped in ("---", "..."):
            return fields
        m = FIELD_RE.match(line)
        if m:
            fields.append(m.group(1))
    return None  # unterminated frontmatter -> treat as absent


def analyze_file(path):
    """Analyze one markdown file; return dict of findings."""
    text = path.read_bytes().decode("utf-8", errors="replace")

    fm_fields = parse_frontmatter(text)
    lines = text.splitlines()
    if fm_fields is not None:
        # Skip past the closing delimiter so its content is not scanned.
        for i in range(1, min(len(lines), 1 + MAX_FM_LINES)):
            if lines[i].strip() in ("---", "..."):
                lines = lines[i + 1 :]
                break

    headings = []  # ATX heading levels, in order
    directive_hits = 0
    keyword_hits = 0
    toc_run = 0
    max_toc_run = 0
    toc_items = 0
    in_fence = False
    fence_char = ""

    for line in lines:
        fence = FENCE_RE.match(line)
        if fence:
            marker = fence.group(1)
            if not in_fence:
                in_fence = True
                fence_char = marker[0]
            elif marker[0] == fence_char:
                in_fence = False
            continue
        if in_fence:
            continue
        if TOCTREE_DIRECTIVE_RE.match(line):
            directive_hits += 1
        if TOCTREE_KEYWORD_RE.search(line):
            keyword_hits += 1
        hm = HEADING_RE.match(line)
        if hm:
            headings.append(len(hm.group(1)))
            toc_run = 0
        elif LIST_LINK_RE.match(line):
            toc_run += 1
            toc_items += 1
            max_toc_run = max(max_toc_run, toc_run)
        else:
            toc_run = 0

    jumps = [
        {"from": a, "to": b}
        for a, b in zip(headings, headings[1:])
        if b > a + 1
    ]
    h1_count = headings.count(1)
    if max_toc_run >= 2:
        toc_type = "md_list"
    elif toc_items >= 1:
        toc_type = "single_link_item"
    else:
        toc_type = "none"

    return {
        "has_frontmatter": fm_fields is not None,
        "frontmatter_fields": fm_fields or [],
        "toc_type": toc_type,
        "toc_link_items": toc_items,
        "heading_count": len(headings),
        "h1_count": h1_count,
        "duplicate_h1": h1_count > 1,
        "level_jumps": jumps,
        "h1_to_deep_jump": any(j["from"] == 1 and j["to"] >= 3 for j in jumps),
        "toctree_directive_hits": directive_hits,
        "toctree_keyword_hits": keyword_hits,
    }


def main():
    t0 = time.perf_counter()

    files = sorted(
        p for p in DOCS_ROOT.rglob("*") if p.is_file() and p.suffix.lower() == ".md"
    )
    per_file = []
    field_counter = Counter()
    toc_counter = Counter()
    jump_files, deep_jump_files, dup_h1_files = [], [], []
    directive_files, keyword_files = [], []
    with_fm = 0

    for path in files:
        rel = path.relative_to(DOCS_ROOT).as_posix()
        res = analyze_file(path)
        res_out = {"path": rel}
        res_out.update(res)
        per_file.append(res_out)

        if res["has_frontmatter"]:
            with_fm += 1
            field_counter.update(res["frontmatter_fields"])
        toc_counter[res["toc_type"]] += 1
        if res["level_jumps"]:
            jump_files.append(rel)
        if res["h1_to_deep_jump"]:
            deep_jump_files.append(rel)
        if res["duplicate_h1"]:
            dup_h1_files.append(rel)
        if res["toctree_directive_hits"]:
            directive_files.append(rel)
        if res["toctree_keyword_hits"]:
            keyword_files.append(rel)

    total = len(files)
    duration = round(time.perf_counter() - t0, 2)

    report = {
        "meta": {
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "root": DOCS_ROOT.as_posix(),
            "total_files": total,
            "duration_seconds": duration,
            "script": Path(__file__).name,
        },
        "frontmatter": {
            "with_frontmatter": with_fm,
            "without_frontmatter": total - with_fm,
            "coverage_pct": round(with_fm / total * 100, 2) if total else 0.0,
            "field_frequency": dict(field_counter.most_common()),
            "files_with_frontmatter": [
                f["path"] for f in per_file if f["has_frontmatter"]
            ],
        },
        "toctree": {
            "rst_directive_remnant_files": directive_files,
            "rst_directive_remnant_count": sum(
                f["toctree_directive_hits"] for f in per_file
            ),
            "keyword_mention_files": keyword_files,
            "toc_style_counts": dict(toc_counter),
            "markdown_list_toc_files": toc_counter.get("md_list", 0),
            "no_toc_structure_files": toc_counter.get("none", 0),
            "single_link_item_files": toc_counter.get("single_link_item", 0),
        },
        "headings": {
            "level_jump_files": jump_files,
            "level_jump_file_count": len(jump_files),
            "h1_to_h3plus_files": deep_jump_files,
            "h1_to_h3plus_file_count": len(deep_jump_files),
            "duplicate_h1_files": dup_h1_files,
            "duplicate_h1_file_count": len(dup_h1_files),
        },
        "per_file": per_file,
    }

    json_path = AUDIT_DIR / "format-consistency-report.json"
    md_path = AUDIT_DIR / "format-consistency-report.md"
    json_path.write_text(
        json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8"
    )

    pct = lambda n: f"{n / total * 100:.1f}%" if total else "n/a"
    ex = lambda lst: "\n".join(f"- `{p}`" for p in lst[:5]) or "-（无）"
    fm_top = (
        "\n".join(f"- `{k}`: {v}" for k, v in field_counter.most_common(10))
        or "-（无字段）"
    )

    md = f"""# 格式一致性检查报告（format-consistency）

- 生成时间：{report['meta']['generated_at']}
- 扫描根目录：`{report['meta']['root']}`
- 文件总数：{total}　耗时：{duration}s　脚本：`{report['meta']['script']}`

## 1. YAML Frontmatter

| 指标 | 数量 | 占比 |
| --- | --- | --- |
| 有 frontmatter | {with_fm} | {pct(with_fm)} |
| 无 frontmatter | {total - with_fm} | {pct(total - with_fm)} |

出现过的字段及频次（Top 10）：

{fm_top}

## 2. Toctree 残留与目录形式

- `.. toctree::` RST 指令残留：**{len(directive_files)}** 个文件（共 {report['toctree']['rst_directive_remnant_count']} 处）
- 含 `toctree` 关键字（含正文提及，如 doc-guide/sphinx.md）：{len(keyword_files)} 个文件
- 目录形式分布：
  - Markdown 列表目录（连续 ≥2 个列表链接项）：**{toc_counter.get('md_list', 0)}**（{pct(toc_counter.get('md_list', 0))}）
  - 仅单个列表链接项：{toc_counter.get('single_link_item', 0)}
  - 无目录结构：**{toc_counter.get('none', 0)}**（{pct(toc_counter.get('none', 0))}）

## 3. 标题层级问题

- 层级跳跃文件（任一相邻标题跳级，如 h1→h3、h2→h4）：**{len(jump_files)}**（{pct(len(jump_files))}）
- 其中 h1 直接到 h3 及更深：**{len(deep_jump_files)}**
- 重复 h1 文件：**{len(dup_h1_files)}**（{pct(len(dup_h1_files))}）

## 示例文件（各 5 个）

### toctree 指令残留
{ex(directive_files)}

### Markdown 列表目录
{ex([f['path'] for f in per_file if f['toc_type'] == 'md_list'])}

### 无目录结构
{ex([f['path'] for f in per_file if f['toc_type'] == 'none'])}

### 标题层级跳跃
{ex(jump_files)}

### 重复 h1
{ex(dup_h1_files)}

> 方法说明：仅识别 ATX 标题（`#` 前缀）；忽略代码块与 frontmatter 内内容；
> 目录判定为连续 ≥2 个含 `(...)` 链接目标的列表项（部分文件因编码问题缺失 `]`，
> 故采用宽松匹配）。全量明细见 `format-consistency-report.json`。
"""
    md_path.write_text(md, encoding="utf-8")

    print(f"scanned={total} with_fm={with_fm} ({report['frontmatter']['coverage_pct']}%)")
    print(
        f"toctree_directive_files={len(directive_files)} "
        f"md_list_toc={toc_counter.get('md_list', 0)} "
        f"no_toc={toc_counter.get('none', 0)}"
    )
    print(
        f"jump_files={len(jump_files)} h1_to_h3plus={len(deep_jump_files)} "
        f"dup_h1_files={len(dup_h1_files)}"
    )
    print(f"reports: {json_path.name}, {md_path.name} ({duration}s)")


if __name__ == "__main__":
    main()
