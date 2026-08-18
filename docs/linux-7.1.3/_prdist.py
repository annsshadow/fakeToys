import os, re, json
BASE = 'D:/WORKSPACE/linux-7.1.3/docs/系统文档'
W = json.load(open(os.path.join(BASE, '.translate_workers.json'), encoding='utf-8'))['workers']
cjk = re.compile(r'[\u4e00-\u9fff]')

def pr_of(path):
    try:
        d = open(path, encoding='utf-8', errors='ignore').read()
    except Exception:
        return None
    lines = d.split('\n')
    depth = 0
    parts = []
    for l in lines:
        if l.strip().startswith('```'):
            depth = 1 - depth
            continue
        if depth == 0:
            parts.append(l)
    prose = '\n'.join(parts)
    prose = re.sub(r'`[^`]*`', '', prose)
    if not prose.strip():
        return 1.0
    return len(cjk.findall(prose)) / len(prose)

buckets = {'<0.05': 0, '0.05-0.10': 0, '0.10-0.30': 0, '0.30-0.60': 0, '>=0.60': 0}
marginal = []  # pr in (0.03, 0.10): most likely mechanical/low-quality
total = 0
for i, files in enumerate(W):
    for f in files:
        p = pr_of(f)
        if p is None:
            continue
        total += 1
        if p < 0.05:
            buckets['<0.05'] += 1
            marginal.append((round(p, 4), i, os.path.relpath(f, BASE)))
        elif p < 0.10:
            buckets['0.05-0.10'] += 1
            marginal.append((round(p, 4), i, os.path.relpath(f, BASE)))
        elif p < 0.30:
            buckets['0.10-0.30'] += 1
        elif p < 0.60:
            buckets['0.30-0.60'] += 1
        else:
            buckets['>=0.60'] += 1

marginal.sort()
print('total worker files scanned:', total)
print('pr distribution:')
for k, v in buckets.items():
    print('  %-12s %5d  (%.1f%%)' % (k, v, 100.0 * v / total))
print()
print('marginal files pr in (0.03,0.10):', len(marginal))
for p, i, rel in marginal[:40]:
    print('  %.4f  u%-3d  %s' % (p, i, rel))
print('  ... (total %d)' % len(marginal))
json.dump({'buckets': buckets, 'total': total,
           'marginal': [{'pr': p, 'unit': i, 'rel': rel} for p, i, rel in marginal]},
          open(os.path.join(BASE, '.translate_prdist.json'), 'w', encoding='utf-8'),
          ensure_ascii=False, indent=2)
print('\nwritten to .translate_prdist.json')
