import os
import re
from collections import Counter

# Scan for more error patterns
heading_pattern = Counter()
truncated_headings = []
garbled_patterns = Counter()

# Common truncated heading patterns (lines that look like headings but end mid-word)
for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8', errors='ignore') as f:
                lines = f.readlines()
        except:
            continue
        
        for i, line in enumerate(lines, 1):
            # Check for headings ending mid-word (Chinese)
            stripped = line.strip()
            if stripped.startswith('#'):
                # Heading line - check if it ends mid-sentence
                content = stripped.lstrip('#').strip()
                if content and not content.endswith(('。', '，', '、', '！', '？', '：', '》', '）', '」', '】', ':', '|', '))', '】')):
                    if len(content) > 3 and content[-1].isalpha() or content[-1] in '的一中在是本有来可以去可以能':
                        pass  # could be valid
                    if len(content) < 30 and content[-1] not in ('。', '，', '、', '！', '？', '：', '》', '）', '」', '】', ':', '|', '）', '》'):
                        heading_pattern[content[-1] if content else ''] += 1
                        if len(truncated_headings) < 200:
                            truncated_headings.append((fpath, i, content))

print("=== HEADING TERMINATION ANALYSIS ===")
for char, count in heading_pattern.most_common(30):
    print(f"  '{char}': {count}")

print("\n=== SAMPLE TRUNCATED HEADINGS ===")
for fpath, line_num, content in truncated_headings[:80]:
    print(f"  {fpath}:{line_num}: {content[:80]}")
