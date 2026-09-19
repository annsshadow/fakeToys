import os

remaining = 0
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
        if '鈥?' in content:
            c = content.count('鈥?')
            remaining += c
            print('REMAINING: {}: {} occurrences'.format(fpath, c))
print('Total remaining : {}'.format(remaining))
