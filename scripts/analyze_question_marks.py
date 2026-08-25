#!/usr/bin/env python3
"""Classify '?' occurrences in a translated md: legit / code-symbol / corrupted-CJK."""
import sys
from pathlib import Path

path = Path(sys.argv[1])
lines = path.read_text(encoding="utf-8", errors="replace").split("\n")
in_code = False
suspect, code_sym, legit = [], 0, []
for i, line in enumerate(lines, 1):
    if line.strip().startswith("```"):
        in_code = not in_code
        continue
    for m in __import__("re").finditer(r"\?", line):
        pos = m.start()
        before = line[pos - 1] if pos > 0 else " "
        if in_code:
            code_sym += 1
        elif ord(before) > 0x2E7F:  # CJK range
            suspect.append((i, line.strip()[:80]))
        else:
            legit.append((i, line.strip()[:60]))

print(f"code-block symbols: {code_sym}")
print(f"legit: {len(legit)}")
print(f"suspect corrupted: {len(suspect)}")
for ln, t in suspect[:15]:
    print(f"  L{ln}: {t}")
