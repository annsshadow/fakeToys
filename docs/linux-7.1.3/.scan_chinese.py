import os
import re

# Find all garbled Chinese patterns
garbled_dict = {}

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

        # Find all Chinese characters that are unusual/garbled
        # Look for sequences of Chinese chars that might be garbled
        chinese_seqs = re.findall(r'[\u4e00-\u9fff]+', content)
        for seq in chinese_seqs:
            if len(seq) >= 2:
                # Check if this sequence appears elsewhere as clean Chinese
                if seq not in garbled_dict:
                    garbled_dict[seq] = fpath

# Print all unique Chinese sequences and where they appear
print("=== UNIQUE CHINESE SEQUENCES ===")
for seq, fpath in sorted(garbled_dict.items(), key=lambda x: len(x[0])):
    # Check if this might be garbled (contains unusual characters)
    unusual = any(ord(c) > 0x9FFF or ord(c) < 0x4E00 for c in seq)
    print(f"  '{seq}' ({len(seq)}) in {fpath}")
