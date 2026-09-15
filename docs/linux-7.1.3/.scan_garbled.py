import os
import re

# Find all garbled Chinese phrases across all files
garbled_phrases = set()
garbled_contexts = []

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

        lines = content.split('\n')
        for i, line in enumerate(lines):
            # Find lines with Chinese characters that might be garbled
            if re.search(r'[\u4e00-\u9fff]', line):
                # Check if the line has garbled characters (outside normal CJK range)
                non_cjk = re.findall(r'[^\u0000-\u9fff\s]', line)
                # Also check for sequences of Chinese that seem wrong
                chinese_words = re.findall(r'[\u4e00-\u9fff]+', line)
                for word in chinese_words:
                    if len(word) >= 2:
                        # Check if this might be garbled by looking at byte-level representation
                        encoded = word.encode('utf-8')
                        # Valid UTF-8 Chinese characters have specific byte patterns
                        garbled_phrases.add(word)
                        garbled_contexts.append((fpath, i+1, word, line[:120]))

# Now check which Chinese sequences might be garbled
# by checking if they contain characters outside common usage
suspect_garbled = []
for word in sorted(garbled_phrases, key=len, reverse=True):
    # Check if any character in this word is rare/unusual
    for ch in word:
        cp = ord(ch)
        # CJK Unified Ideographs: U+4E00 to U+9FFF
        # CJK Extension A: U+3400 to U+4DBF
        # Rare/old characters often outside these ranges
        if cp > 0xA000 or (0x3400 <= cp <= 0x4DFF):
            suspect_garbled.append(word)
            break

# Deduplicate and show context
seen = set()
print("=== SUSPECT GARBLED CHINESE PHRASES ===")
for word in sorted(set(suspect_garbled)):
    contexts = [c for c in garbled_contexts if c[2] == word]
    if contexts[:3]:
        for fpath, line_num, _, context in contexts[:3]:
                if fpath not in seen:
                    print(f"\n'{word}' in {fpath}:{line_num}")
                    print(f"  Context: {context}")
                    seen.add(fpath)