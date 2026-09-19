import os
import re
from collections import Counter

# Find all Chinese sequences that look garbled
# by checking if they contain characters outside common ranges
garbled_map = {}

for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
        except:
            continue

        # Find all Chinese character sequences of 2+ chars
        for match in re.finditer(r'[\u4e00-\u9fff]{2,}', content):
            seq = match.group()
            # Check if any character is in a problematic range
            for ch in seq:
                cp = ord(ch)
                # CJK Extension B and beyond, or very rare characters
                if cp > 0x9FFF or (0x3400 <= cp <= 0x4DFF) or (0x20000 <= cp <= 0x2A6DF):
                    if seq not in garbled_map:
                        garbled_map[seq] = set()
                    garbled_map[seq].add(fpath)
                    break

# Also check for mixed ASCII-Chinese patterns that suggest garbling
for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
        except:
            continue

        # Look for Chinese sequences with embedded private use area chars
        for match in re.finditer(r'[\u4e00-\u9fff\U000F0000-\U000FFFFF]+', content):
            seq = match.group()
            if len(seq) >= 2:
                if seq not in garbled_map:
                    garbled_map[seq] = set()
                garbled_map[seq].add(fpath)

# Show results
print(f"Total garbled sequences: {len(garbled_map)}")
for seq, files in sorted(garbled_map.items(), key=lambda x: len(x[0])):
    file_list = ', '.join(list(files)[:3])
    print(f"  '{seq}' ({len(seq)} chars) in {file_list}")
