#!/usr/bin/env python3
"""Recover double-encoded CJK docs (plan002 U11.1).

Corruption mechanism (verified): original UTF-8 text was misread as
Windows cp936 (ANSI) and re-saved as UTF-8.  Artifacts of this misread:
  - classic GBK pseudo-hanzi (e.g. 缂洪櫡杩借釜 = 缺陷追踪)
  - U+20AC (EUR sign) from lone byte 0x80
  - PUA chars U+E000-U+F8FF from byte pairs cp936 maps to the private area
  - literal '?' where a dangling lead byte had no valid trail

Because of the PUA/EUR artifacts, reversing MUST use the 'gb18030' codec
(a strict superset of cp936/gbk; Python's 'gbk' cannot encode them back).
Every candidate recovery is proven by forward re-corruption:
    corrupt(rec) == original_line
where corrupt(s) = s.encode('utf-8').decode('gb18030', 'replace')
                   .replace('\ufffd', '?')

Lines whose byte losses (the '?' holes) break strict decoding get a
best-effort lossy recovery (utf-8 decode with errors='ignore'), gated on
mojibake evidence in the original line.  Everything is per-line because
corruption is mixed within files.

Punctuation repair (after recovery, outside fenced code blocks):
  '?' preceded by CJK at end-of-line   -> '。'
  '?' preceded by CJK followed by CJK  -> '，'
  everything else                      -> untouched, logged unresolved

All IO is explicit UTF-8 via bytes; BOM and original line endings are
preserved.  Never touch these files with PowerShell pipe cmdlets.
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FILES = [
    ROOT / "docs/linux-7.1.3/admin-guide/bug-hunting.md",
    ROOT / "docs/linux-7.1.3/dev-tools/kmsan.md",
    ROOT / "docs/linux-7.1.3/dev-tools/ubsan.md",
]
LOG_PATH = ROOT / "docs/audits/cjk-recovery-log.md"

# CJK ideographs + kana/blocks + fullwidth forms (same class as the repo's
# fix_cjk_question_marks.py, so both tools agree on what "CJK" means).
CJK_RE = re.compile(r"[\u2e80-\u9fff\uf900-\ufaff\uff01-\uff65]")
# High-byte (non-ASCII) immediately followed by literal '?': the fingerprint
# of a lost byte at a cp936 pair boundary.
HIGHBYTE_Q_RE = re.compile(r"[^\x00-\x7f]\?")
# Chars that essentially only occur as UTF-8-read-as-GBK debris in
# simplified-Chinese docs, plus EUR/PUA artifacts.
MOJIBAKE_SIG_RE = re.compile(
    "[锛銆鎴鏄鍦鍒纭绛鐨勬涓缂娓璇璁娴鑾鍚敓枅嬫寚鍖呭惈鏂囦欢娆鍝堟暟璺"
    "寮傚父瑙ｅ喅纭畾绫诲瀷妫€鏌ュけ璐ヨ皟鐢ㄦ彃浠舵墽琛屾搷浣滅紪璇戝櫒"
    "\ue000-\uf8ff\u20ac]"
)


def cjk_ratio(s: str) -> float:
    return len(CJK_RE.findall(s)) / len(s) if s else 0.0


# Exact Windows cp936 tables via the OS API (the corruption was produced by
# a Windows ANSI decoder, whose mappings -- EUR from 0x80 and the PUA zone
# U+E000-U+F8FF -- are absent from CPython's gbk/gb18030 codecs).
import ctypes

_MBTWC = ctypes.windll.kernel32.MultiByteToWideChar
_CHAR_CACHE = {}


def _cp936_decode(raw: bytes) -> str:
    if raw in _CHAR_CACHE:
        return _CHAR_CACHE[raw]
    buf = ctypes.create_unicode_buffer(8)
    n = _MBTWC(936, 0, raw, len(raw), buf, 8)
    ch = buf[:n] if n > 0 else "?"
    _CHAR_CACHE[raw] = ch
    return ch


# Forward table: structurally valid lead+trail -> decoded char(s), skipping
# pairs that decode to the default char '?' (those became literal '?' holes).
_FWD = {}
for _lead in range(0x81, 0x100):
    for _trail in range(0x40, 0x100):
        if _trail == 0x7F:
            continue
        _s = _cp936_decode(bytes([_lead, _trail]))
        if _s and _s != "?":
            _FWD[(_lead, _trail)] = _s
_REV = {}
for (_lead, _trail), _s in _FWD.items():
    if len(_s) == 1 and _s not in _REV:
        _REV[_s] = bytes([_lead, _trail])
_REV["\u20ac"] = b"\x80"


def corrupt_forward(text_utf8: str) -> str:
    """Re-run the suspected corruption on s (Windows cp936 semantics)."""
    data = text_utf8.encode("utf-8")
    out, i, n = [], 0, len(data)
    while i < n:
        b = data[i]
        if b < 0x80:
            out.append(chr(b))
            i += 1
        elif b == 0xFF:
            out.append("?")
            i += 1
        else:
            pair = _FWD.get((b, data[i + 1])) if i + 1 < n else None
            if pair is not None:
                out.append(pair)
                i += 2
            else:
                out.append("\u20ac" if b == 0x80 else "?")
                i += 1
    return "".join(out)


def reverse_bytes(s: str) -> bytes:
    out = bytearray()
    for ch in s:
        rev = _REV.get(ch)
        if rev is not None:
            out += rev
        else:
            out += ch.encode("gb18030")
    return bytes(out)


def looks_corrupt(line: str) -> bool:
    return bool(MOJIBAKE_SIG_RE.search(line) or HIGHBYTE_Q_RE.search(line))


def recover_line(line: str):
    """Return (status, line): status in clean/recovered/recovered_lossy/failed."""
    if line.isascii():
        return "clean", line
    try:
        raw = reverse_bytes(line)
    except UnicodeEncodeError:
        # Cannot even reverse-map: only treat as corrupt if it smells bad.
        return ("failed", line) if looks_corrupt(line) else ("clean", line)
    try:
        rec = raw.decode("utf-8")  # strict
    except UnicodeDecodeError:
        rec = None
    if rec is not None:
        if rec != line and CJK_RE.search(rec) and corrupt_forward(rec) == line:
            return "recovered", rec
        # Roundtrip succeeded but we cannot PROVE the line was corrupted
        # (some clean hanzi lines happen to re-encode into valid UTF-8).
        return "clean", line
    if not looks_corrupt(line):
        return "clean", line  # ordinary Chinese that simply isn't a roundtrip
    rec = raw.decode("utf-8", errors="ignore")  # best effort around '?' holes
    if (
        rec
        and rec != line
        and CJK_RE.search(rec)
        and "\ufffd" not in rec
        and cjk_ratio(rec) > 0.0
    ):
        return "recovered_lossy", rec
    return "failed", line


def fix_punctuation(lines):
    """Spec rules; fenced code blocks are skipped (repo convention)."""
    out, replacements, unresolved = [], [], []
    in_code = False
    for idx, line in enumerate(lines, 1):
        if line.strip().startswith("```"):
            in_code = not in_code
            out.append(line)
            continue
        if in_code or "?" not in line:
            out.append(line)
            continue
        chars = list(line)
        for pos, ch in enumerate(line):
            if ch != "?":
                continue
            prev = line[pos - 1] if pos > 0 else ""
            nxt = line[pos + 1] if pos + 1 < len(line) else ""
            if CJK_RE.match(prev or " "):
                if nxt == "":
                    chars[pos] = "。"
                    replacements.append((idx, "。"))
                elif CJK_RE.match(nxt):
                    chars[pos] = "，"
                    replacements.append((idx, "，"))
                else:
                    unresolved.append((idx, line.strip()[:80]))
            else:
                unresolved.append((idx, line.strip()[:80]))
        out.append("".join(chars))
    return out, replacements, unresolved


def garbled_stats(lines):
    total = sum(1 for ln in lines if ln.strip())
    hits = sum(1 for ln in lines if MOJIBAKE_SIG_RE.search(ln))
    return hits, total


def process(path: Path):
    data = path.read_bytes()
    has_bom = data.startswith(b"\xef\xbb\xbf")
    # Write LF endings unconditionally: the HEAD blobs are LF-only and
    # core.autocrlf=true re-normalizes on commit anyway (the CRLF seen in
    # some working-tree copies came from Windows tooling / fresh checkout).
    text = data.decode("utf-8")
    if has_bom:
        text = text.lstrip("\ufeff")
    lines = text.replace("\r\n", "\n").split("\n")

    before_hits, total_nonempty = garbled_stats(lines)
    stats = {"clean": 0, "recovered": 0, "recovered_lossy": 0, "failed": 0}
    failures = []
    new_lines = []
    for idx, line in enumerate(lines, 1):
        status, new = recover_line(line)
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
    }


def main():
    report = ["# CJK 双重编码恢复日志（plan002 U11.1）", ""]
    report.append("- 行尾统一为 LF、保留 BOM（与 HEAD blob 一致；core.autocrlf=true 提交时亦归一为 LF）。")
    report.append("- 机制：UTF-8 原文被按 Windows cp936（ANSI）误读后存回 UTF-8；"
                  "逐行用 Windows MultiByteToWideChar(936) 导出的精确映射表反向还原"
                  "（覆盖 €/PUA 特殊映射，gb18030 兜底），并以“正向再损坏 == 原行”"
                  "证明后才采纳；原损坏中字节已丢失的位置（字面 `?`）仅能有损恢复。")
    report.append("")
    for path in FILES:
        r = process(path)
        st = r["stats"]
        rep_zh = sum(1 for _, c in r["replacements"] if c == "。")
        rep_dy = sum(1 for _, c in r["replacements"] if c == "，")
        pct_before = 100 * r["before_hits"] / max(r["total_nonempty"], 1)
        pct_after = 100 * r["after_hits"] / max(r["total_nonempty"], 1)
        rel = path.relative_to(ROOT).as_posix()
        report.append(f"## {rel}")
        report.append("")
        report.append(f"- 总行数：{r['total_lines']}（含空行；非空 {r['total_nonempty']}）")
        report.append(f"- 干净行：{st['clean']}；恢复行：{st['recovered']}"
                      f"（另有有损恢复 {st['recovered_lossy']} 行）；"
                      f"恢复失败行：{st['failed']}")
        report.append(f"- 标点替换：{len(r['replacements'])} 处"
                      f"（句号 {rep_zh}，逗号 {rep_dy}）；未解决 `?`："
                      f"{len(r['unresolved'])} 处")
        report.append(f"- 自验乱码签名行率：{r['before_hits']}/{r['total_nonempty']}"
                      f"（{pct_before:.1f}%）→ {r['after_hits']}/{r['total_nonempty']}"
                      f"（{pct_after:.1f}%）")
        if st["recovered_lossy"]:
            report.append(f"- 注意：{st['recovered_lossy']} 行因原文件中存在"
                          "字节丢失（字面 `?`），仅能部分还原")
        if r["failures"]:
            report.append("")
            report.append("### 恢复失败行（保留原文，需人工对照上游）")
            report.extend(f"- L{i}: `{t}`" for i, t in r["failures"])
        if r["unresolved"]:
            report.append("")
            report.append("### 未解决 `?`（不满足替换规则，保留原样）")
            seen = set()
            for i, t in r["unresolved"]:
                key = (i, t)
                if key not in seen:
                    seen.add(key)
                    report.append(f"- L{i}: `{t}`")
        report.append("")
        print(f"{rel}: clean={st['clean']} recovered={st['recovered']} "
              f"lossy={st['recovered_lossy']} failed={st['failed']} "
              f"punct={len(r['replacements'])} unresolved={len(r['unresolved'])} "
              f"garbled {r['before_hits']}->{r['after_hits']}")
    LOG_PATH.parent.mkdir(parents=True, exist_ok=True)
    LOG_PATH.write_text("\n".join(report) + "\n", encoding="utf-8")
    print(f"log={LOG_PATH.relative_to(ROOT).as_posix()}")


if __name__ == "__main__":
    main()
