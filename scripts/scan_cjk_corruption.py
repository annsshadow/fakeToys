#!/usr/bin/env python3
"""Scan docs/linux-7.1.3 for double-encoded CJK corruption (plan002 U11.1).

Scan mode (default): report every .md whose mojibake-signature line rate
exceeds 5%, sorted descending, marked DONE/TODO against the audit log
(log headers "## docs/linux-7.1.3/<path>" define the processed set).

Recover mode (--recover N): run the recovery pipeline on the top-N
unprocessed files (rank: density desc, size desc, path asc) and APPEND
per-file sections plus a full inventory snapshot to the audit log.

Safety note: many flagged files are HALF-recovered (clean Chinese body +
residual PUA/EUR artifacts + '?' byte-loss holes).  The stock lossy branch
of recover_translated_docs would mangle such lines, so lossy candidates are
re-validated here: reverse_bytes(line) must fully parse as UTF-8 under the
grammar "literal byte, or '?' matching any single byte".  Half-recovered
lines (gb18030 re-encoding of clean hanzi) cannot parse and are kept as-is.
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))

from recover_translated_docs import (  # noqa: E402
    CJK_RE, MOJIBAKE_SIG_RE, corrupt_forward, fix_punctuation, garbled_stats,
    looks_corrupt, recover_line, reverse_bytes,
)

DOCS = ROOT / "docs/linux-7.1.3"
LOG_PATH = ROOT / "docs/audits/cjk-recovery-log.md"
THRESHOLD = 0.05


# ---------------------------------------------------------------- scanning

def scan():
    rows = []
    for p in DOCS.rglob("*.md"):
        text = p.read_bytes().decode("utf-8", errors="replace")
        lines = text.replace("\r\n", "\n").split("\n")
        total = sum(1 for ln in lines if ln.strip())
        hits = sum(1 for ln in lines if MOJIBAKE_SIG_RE.search(ln))
        rows.append((p.relative_to(ROOT).as_posix(), hits, total,
                     hits / total if total else 0.0))
    return rows


def sort_corrupted(rows):
    return sorted((x for x in rows if x[3] > THRESHOLD),
                  key=lambda x: (-x[3], -x[2], x[0]))


def processed_set():
    done = set()
    if LOG_PATH.exists():
        for ln in LOG_PATH.read_text(encoding="utf-8").splitlines():
            if ln.startswith("## docs/linux-7.1.3/"):
                done.add(ln[3:].strip())
    return done


def append_log(text):
    content = LOG_PATH.read_text(encoding="utf-8")
    if not content.endswith("\n"):
        content += "\n"
    LOG_PATH.write_text(content + text, encoding="utf-8")


# ------------------------------------------------- hardened lossy recovery

def _seq_len(b):
    if b < 0x80:
        return 1
    if 0xC2 <= b <= 0xDF:
        return 2
    if 0xE0 <= b <= 0xEF:
        return 3
    if 0xF0 <= b <= 0xF4:
        return 4
    return 0


def wildcard_parse(raw: bytes):
    """Decode raw as UTF-8 where b'?' matches any single byte (lost-byte
    hole).  Sequences touched by a hole are dropped, never guessed.
    Return None unless EVERY byte is covered by the grammar."""
    out, i, n = [], 0, len(raw)
    while i < n:
        b = raw[i]
        if b == 0x3F:
            i += 1
            continue
        ln = _seq_len(b)
        if ln == 1:
            out.append(chr(b))
            i += 1
            continue
        if ln == 0 or i + ln > n:
            return None
        seq = raw[i:i + ln]
        if any(c == 0x3F or not 0x80 <= c <= 0xBF for c in seq[1:]):
            if all(c == 0x3F or 0x80 <= c <= 0xBF for c in seq[1:]):
                i += ln  # hole-damaged sequence: consume, emit nothing
                continue
            return None
        try:
            out.append(seq.decode("utf-8"))
        except UnicodeDecodeError:
            return None
        i += ln
    return "".join(out)


def recover_line_hardened(line: str):
    """Return (status, line): clean / recovered / recovered_lossy /
    lossy_rejected / failed."""
    if line.isascii():
        return "clean", line
    status, new = recover_line(line)
    if status in ("clean", "recovered"):
        return status, new
    if not looks_corrupt(line):
        return "clean", line
    raw = reverse_bytes(line)
    rec = None
    try:
        strict = raw.decode("utf-8")
        if (strict != line and CJK_RE.search(strict)
                and corrupt_forward(strict) == line):
            rec = strict
    except UnicodeDecodeError:
        pass
    if rec is None:
        cand = wildcard_parse(raw)
        if (cand is not None and cand != line and CJK_RE.search(cand)
                and "\ufffd" not in cand
                and len(MOJIBAKE_SIG_RE.findall(cand))
                < len(MOJIBAKE_SIG_RE.findall(line))):
            rec = cand
    if rec is not None:
        return "recovered_lossy", rec
    return ("lossy_rejected" if status == "recovered_lossy" else "failed"), line


# ---------------------------------------------------------------- pipeline

def process(path: Path):
    data = path.read_bytes()
    has_bom = data.startswith(b"\xef\xbb\xbf")
    text = data.decode("utf-8")
    if has_bom:
        text = text.lstrip("\ufeff")
    lines = text.replace("\r\n", "\n").split("\n")

    before_hits, total_nonempty = garbled_stats(lines)
    nonascii_lines = sum(1 for ln in lines if not ln.isascii())
    stats = {"clean": 0, "recovered": 0, "recovered_lossy": 0,
             "lossy_rejected": 0, "failed": 0}
    failures, new_lines = [], []
    for idx, line in enumerate(lines, 1):
        status, new = recover_line_hardened(line)
        stats[status] += 1
        if status == "failed":
            failures.append((idx, line.strip()[:80]))
        new_lines.append(new)

    new_lines, replacements, unresolved = fix_punctuation(new_lines)
    after_hits, _ = garbled_stats(new_lines)

    out_text = ("\ufeff" if has_bom else "") + "\n".join(new_lines)
    path.write_bytes(out_text.encode("utf-8"))
    return {
        "stats": stats,
        "failures": failures,
        "replacements": replacements,
        "unresolved": unresolved,
        "before_hits": before_hits,
        "after_hits": after_hits,
        "total_nonempty": total_nonempty,
        "total_lines": len(lines),
        "nonascii_lines": nonascii_lines,
    }


def file_section(rel, r):
    st = r["stats"]
    provable = st["recovered"] + st["recovered_lossy"]
    rep_zh = sum(1 for _, c in r["replacements"] if c == "。")
    rep_dy = sum(1 for _, c in r["replacements"] if c == "，")
    pct_before = 100 * r["before_hits"] / max(r["total_nonempty"], 1)
    pct_after = 100 * r["after_hits"] / max(r["total_nonempty"], 1)
    out = [f"## {rel}", ""]
    out.append(f"- 检测结论：双重编码（非 ASCII 行 {r['nonascii_lines']}，"
               f"其中可证明损坏 {provable} 行）")
    out.append(f"- 总行数：{r['total_lines']}（含空行；非空 {r['total_nonempty']}）")
    out.append(f"- 干净行：{st['clean']}；严格恢复行：{st['recovered']}；"
               f"有损恢复行（通配符覆盖校验通过）：{st['recovered_lossy']}；"
               f"保留原样行：{st['lossy_rejected'] + st['failed']}"
               f"（其中有损校验拒绝 {st['lossy_rejected']}，"
               f"恢复失败 {st['failed']}）")
    out.append(f"- 标点替换：{len(r['replacements'])} 处"
               f"（句号 {rep_zh}，逗号 {rep_dy}）；未解决 `?`："
               f"{len(r['unresolved'])} 处")
    out.append(f"- 自验乱码签名行率：{r['before_hits']}/{r['total_nonempty']}"
               f"（{pct_before:.1f}%）→ {r['after_hits']}/{r['total_nonempty']}"
               f"（{pct_after:.1f}%）")
    if r["failures"]:
        out.append("")
        out.append("### 恢复失败行（保留原文，需人工对照上游）")
        out.extend(f"- L{i}: `{t}`" for i, t in r["failures"])
    if r["unresolved"]:
        out.append("")
        out.append("### 未解决 `?`（不满足替换规则，保留原样）")
        seen = set()
        for i, t in r["unresolved"]:
            key = (i, t)
            if key not in seen:
                seen.add(key)
                out.append(f"- L{i}: `{t}`")
    out.append("")
    return "\n".join(out)


# -------------------------------------------------------------------- main

def main():
    recover_n = 0
    if len(sys.argv) > 2 and sys.argv[1] == "--recover":
        recover_n = int(sys.argv[2])

    corrupted = sort_corrupted(scan())
    done = processed_set()

    if recover_n:
        todo = [x for x in corrupted if x[0] not in done][:recover_n]
        sections = []
        for rel, _h, _t, _d in todo:
            r = process(ROOT / rel)
            sections.append(file_section(rel, r))
            st = r["stats"]
            print(f"{rel}: clean={st['clean']} recovered={st['recovered']} "
                  f"lossy={st['recovered_lossy']} "
                  f"lossy_rejected={st['lossy_rejected']} "
                  f"failed={st['failed']} punct={len(r['replacements'])} "
                  f"unresolved={len(r['unresolved'])} "
                  f"garbled {r['before_hits']}->{r['after_hits']}")
        append_log("\n".join(sections) + "\n")
        rows = scan()
        corrupted = sort_corrupted(rows)
        done = processed_set()
        inv = ["## 受损文件清单（docs/linux-7.1.3 扫描，签名行率 >5%，按受损率降序）",
               ""]
        inv_rows = sorted((x for x in rows
                           if x[3] > THRESHOLD or x[0] in done),
                          key=lambda x: (-x[3], -x[2], x[0]))
        for rel, hits, total, dens in inv_rows:
            mark = "已处理" if rel in done else "未处理"
            note = "（已降至阈值以下）" if dens <= THRESHOLD else ""
            inv.append(f"- [{mark}]{note} {rel}"
                       f"（{hits}/{total} 行，{100 * dens:.1f}%）")
        inv.append("")
        append_log("\n".join(inv))
        print(f"log={LOG_PATH.relative_to(ROOT).as_posix()} (appended)")

    n_done = sum(1 for x in corrupted if x[0] in done)
    print(f"corrupted_total={len(corrupted)} processed={n_done} "
          f"pending={len(corrupted) - n_done}")
    for rel, hits, total, dens in corrupted:
        mark = "DONE" if rel in done else "TODO"
        print(f"{mark} {100 * dens:5.1f}% {hits}/{total} {rel}")


if __name__ == "__main__":
    main()
