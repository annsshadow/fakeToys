#!/usr/bin/env python3
"""Internal link integrity audit for docs/linux-7.1.3.

Scans inline links [text](path) and reference-style definitions [text]: path,
resolves relative targets (with URL decoding and #anchor stripping), and
classifies them into: ok-file / ok-directory / anchor-only / external-http(s)
/ other-scheme / broken.

Outputs:
  docs/audits/link-integrity-report.json  (full broken-link detail)
  docs/audits/link-integrity-report.md    (human-readable summary)

Read-only w.r.t. docs/linux-7.1.3/.
"""

import json
import re
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import unquote

REPO_ROOT = Path(__file__).resolve().parents[2]
DOC_ROOT = REPO_ROOT / "docs" / "linux-7.1.3"
OUT_JSON = REPO_ROOT / "docs" / "audits" / "link-integrity-report.json"
OUT_MD = REPO_ROOT / "docs" / "audits" / "link-integrity-report.md"

INLINE_LINK_RE = re.compile(
    r"\[([^\]\n]*)\]\(\s*((?:[^()\n]|\([^()\n]*\))*)\s*\)"
)
REFDEF_RE = re.compile(
    r"^[ \t]{0,3}\[([^\]\n]+)\]:[ \t]*(?:<([^>\n]+)>|(\S+))"
    r"[ \t]*(?:\"[^\"]*\"|'[^']*'|\\([^\\)]*\\))?[ \t]*$",
    re.MULTILINE,
)
SCHEME_RE = re.compile(r"^[a-zA-Z][a-zA-Z0-9+.\-]*:")


def build_existence_set(root: Path) -> set[str]:
    """All file/dir paths under root, as posix relative strings."""
    known: set[str] = set()
    for p in root.rglob("*"):
        known.add(p.relative_to(root).as_posix())
    return known


def split_inline_target(raw: str) -> str:
    """Extract the URL part from inline paren content.

    Handles both `path "title"` and Sphinx-style leftover `label </real/path>`.
    """
    raw = raw.strip()
    if "<" in raw and raw.endswith(">"):
        start = raw.index("<")
        return raw[start + 1 : -1]
    return raw.split()[0] if raw.split() else ""


def classify_target(raw: str) -> str:
    """Coarse classification before filesystem resolution."""
    t = raw.strip()
    if not t or t.startswith("#"):
        return "anchor_only"
    if t.lower().startswith(("http://", "https://")):
        return "external_http"
    if SCHEME_RE.match(t):
        return "other_scheme"
    return "internal"


def resolve_internal(target_no_anchor: str, source_rel_dir: str, known: set[str]):
    """Resolve an internal target. Returns (status, normalized_target).

    status: 'file_ok' | 'dir_ok' | ('broken', reason)
    """
    decoded = unquote(target_no_anchor.strip())
    if not decoded:
        return ("broken", "empty_target", "")
    # Strip leading ./ for normalization; keep track of root-relative form.
    if decoded.startswith("/"):
        rel = decoded.lstrip("/")
    else:
        base = source_rel_dir
        rel = f"{base}/{decoded}" if base else decoded
    # Normalize .. and . lexically.
    parts: list[str] = []
    for seg in rel.split("/"):
        if seg in ("", "."):
            continue
        if seg == "..":
            if parts:
                parts.pop()
            else:
                return ("broken", "escapes_doc_root", decoded)
        else:
            parts.append(seg)
    norm = "/".join(parts)
    if not norm:
        return ("dir_ok", ".", "")
    if norm in known:
        return ("dir_ok" if (DOC_ROOT / norm).is_dir() else "file_ok", norm, "")
    if f"{norm}.md" in known:
        return ("file_ok", f"{norm}.md", "")
    if f"{norm}/index.md" in known:
        return ("file_ok", f"{norm}/index.md", "")
    return ("broken", "target_not_found", decoded)


def main() -> int:
    if not DOC_ROOT.is_dir():
        print(f"ERROR: doc root not found: {DOC_ROOT}", file=sys.stderr)
        return 1

    md_files = sorted(DOC_ROOT.rglob("*.md"))
    known = build_existence_set(DOC_ROOT)

    cat_counter = Counter()
    broken: list[dict] = []
    broken_target_counter: Counter = Counter()
    broken_by_topdir: Counter = Counter()
    files_with_broken: set[str] = set()

    for md in md_files:
        rel = md.relative_to(DOC_ROOT).as_posix()
        rel_dir = Path(rel).parent.as_posix()
        rel_dir = "" if rel_dir == "." else rel_dir
        try:
            text = md.read_text(encoding="utf-8", errors="replace")
        except OSError as exc:
            print(f"WARN: cannot read {rel}: {exc}", file=sys.stderr)
            continue

        matches: list[tuple[int, str, str]] = []  # (line_no, link_text, raw_target)
        for m in INLINE_LINK_RE.finditer(text):
            line_no = text.count("\n", 0, m.start()) + 1
            matches.append((line_no, m.group(1), split_inline_target(m.group(2))))
            cat_counter["inline_links"] += 1
        for m in REFDEF_RE.finditer(text):
            line_no = text.count("\n", 0, m.start()) + 1
            raw = m.group(2) if m.group(2) is not None else m.group(3)
            matches.append((line_no, m.group(1), raw))
            cat_counter["reference_def_links"] += 1

        for line_no, link_text, raw_target in matches:
            cat = classify_target(raw_target)
            if cat != "internal":
                cat_counter[cat] += 1
                continue

            target_no_anchor = raw_target.split("#", 1)[0]
            status = resolve_internal(target_no_anchor, rel_dir, known)
            if status[0] == "broken":
                reason, decoded = status[1], status[2]
                cat_counter["internal_broken"] += 1
                entry = {
                    "source_file": f"docs/linux-7.1.3/{rel}",
                    "line": line_no,
                    "link_text": link_text,
                    "target": raw_target,
                    "reason": reason,
                }
                broken.append(entry)
                files_with_broken.add(rel)
                key = decoded if decoded else "(empty)"
                broken_target_counter[key] += 1
                top = rel.split("/", 1)[0] if "/" in rel else "(root)"
                broken_by_topdir[top] += 1
            elif status[0] == "dir_ok":
                cat_counter["internal_dir_ok"] += 1
            else:
                cat_counter["internal_file_ok"] += 1

    kind_keys = ("inline_links", "reference_def_links")
    total_links = sum(v for k, v in cat_counter.items() if k not in kind_keys)
    internal_total = (
        cat_counter["internal_file_ok"]
        + cat_counter["internal_dir_ok"]
        + len(broken)
    )
    broken_count = len(broken)
    rate_total = (broken_count / total_links * 100) if total_links else 0.0
    rate_internal = (broken_count / internal_total * 100) if internal_total else 0.0

    categories = {k: v for k, v in cat_counter.items() if k not in kind_keys}
    summary = {
        "files_scanned": len(md_files),
        "total_links": total_links,
        "inline_links": cat_counter.get("inline_links", 0),
        "reference_def_links": cat_counter.get("reference_def_links", 0),
        "categories": categories,
        "internal_links_total": internal_total,
        "broken_links": broken_count,
        "broken_rate_percent_of_all_links": round(rate_total, 3),
        "broken_rate_percent_of_internal_links": round(rate_internal, 3),
        "files_with_broken_links": len(files_with_broken),
    }

    OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "doc_root": "docs/linux-7.1.3",
        "summary": summary,
        "broken_links": broken,
    }
    OUT_JSON.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8"
    )

    top10 = broken_target_counter.most_common(10)
    lines: list[str] = []
    lines.append("# 链接完整性审计报告（link-integrity）")
    lines.append("")
    lines.append(f"- 生成时间：{payload['generated_at']}")
    lines.append(f"- 审计范围：`docs/linux-7.1.3/`")
    lines.append(f"- 扫描文件数：**{len(md_files)}**")
    lines.append(f"- 总链接数：**{total_links}**")
    lines.append(f"- 外部 http(s) 链接：{cat_counter.get('external_http', 0)}（只计数，未探测）")
    lines.append(f"- 纯锚点链接：{cat_counter.get('anchor_only', 0)}（跳过）")
    lines.append(f"- 其他协议链接：{cat_counter.get('other_scheme', 0)}（跳过）")
    lines.append(
        f"- 内部链接：{internal_total}"
        f"（指向文件 {cat_counter.get('internal_file_ok', 0)}，"
        f"指向目录 {cat_counter.get('internal_dir_ok', 0)}）"
    )
    lines.append(f"- **断链总数：{broken_count}**")
    lines.append(
        f"- **断链率：{rate_total:.2f}%**（占全部链接）；"
        f"占内部链接 {rate_internal:.2f}%"
    )
    lines.append(f"- 涉及断链的文件数：{len(files_with_broken)}")
    lines.append("")
    lines.append("## Top 10 最常被断链指向的目标")
    lines.append("")
    if top10:
        lines.append("| 次数 | 目标 |")
        lines.append("|---:|---|")
        for tgt, cnt in top10:
            lines.append(f"| {cnt} | `{tgt}` |")
    else:
        lines.append("无断链。")
    lines.append("")
    lines.append("## 断链按目录分布（按源文件一级子目录）")
    lines.append("")
    if broken_by_topdir:
        lines.append("| 目录 | 断链数 |")
        lines.append("|---|---:|")
        for d, cnt in sorted(broken_by_topdir.items(), key=lambda kv: (-kv[1], kv[0])):
            lines.append(f"| `{d}` | {cnt} |")
    else:
        lines.append("无断链。")
    lines.append("")
    lines.append("## 说明")
    lines.append("")
    lines.append("- 相对路径以所在文件目录为基解析；`/` 开头按文档树根解析。")
    lines.append("- 解析顺序：精确路径 → `<目标>.md` → `<目标>/index.md`。")
    lines.append("- 带 `#anchor` 的目标只验证文件部分；URL 编码（如 `%20`）已解码。")
    lines.append("- 明细见 `link-integrity-report.json`。")
    lines.append("")
    OUT_MD.write_text("\n".join(lines), encoding="utf-8")

    print(
        f"files={len(md_files)} total_links={total_links} "
        f"broken={broken_count} rate={rate_total:.2f}%"
    )
    print(f"json -> {OUT_JSON}")
    print(f"md   -> {OUT_MD}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
