import os
import re
from collections import Counter

error_types = Counter()
files_with_errors = Counter()
details = []

mu_ji_count = 0
truncated_count = 0
garbled_count = 0
total_files = 0

for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        total_files += 1
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            continue

        lines = content.split('\n')
        file_errors = []

        for i, line in enumerate(lines, 1):
            if '鈥?' in line:
                mu_ji_count += line.count('鈥?')
                if fpath not in [d[0] for d in file_errors]:
                    file_errors.append((fpath, 'mu_ji'))
            if '鍐呮牳' in line or '瀛愮郴缁熸' in line or '枃妗' in line:
                garbled_count += 1
                if fpath not in [d[0] for d in file_errors]:
                    file_errors.append((fpath, 'garbled'))
            if len(line) > 100 and line.rstrip().endswith(('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z')):
                pass

        for etype, count_local in [('mu_ji', mu_ji_count), ('garbled', garbled_count)]:
            pass

for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if not d.startswith('.')]
    for fname in files:
        if not (fname.endswith('.md') or fname.endswith('.rst') or fname.endswith('.txt')):
            continue
        fpath = os.path.join(root, fname)
        try:
            with open(fpath, 'r', encoding='utf-8') as f:
                content = f.read()
        except:
            continue
        mu_count = content.count('鈥?')
        garbled_count = len(re.findall(r'鍐呮牳|瀛愮郴缁熸|枃妗|瀛愭枃妗', content))
        if mu_count > 0:
            error_types['鈥? (mojibake)'] += mu_count
            files_with_errors[fpath] += mu_count
        if garbled_count > 0:
            error_types['garbled Chinese'] += garbled_count
            files_with_errors[fpath] += garbled_count

print("=== ERROR SUMMARY ===")
for k, v in error_types.most_common():
    print(f"{k}: {v}")
print(f"\nTotal MD/RST/TXT files: {total_files}")
print(f"\n=== TOP FILES WITH ERRORS ===")
for fpath, count in files_with_errors.most_common(30):
    print(f"{count:>4}  {fpath}")
