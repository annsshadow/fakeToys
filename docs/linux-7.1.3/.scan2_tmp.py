import json, re, os

BASE = "D:/WORKSPACE/linux-7.1.3/docs/系统文档"
d = json.load(open(os.path.join(BASE, ".translate_dispatch_cold2.json")))
ch = d["chunks"][0]

# Chinese char + CJK punctuation + fullwidth range
zh_re = re.compile(r'[\u3400-\u4dbf\u4e00-\u9fff\u3000-\u303f\uff00-\uffef]')
en_re = re.compile(r'[A-Za-z]+')

def strip_code(text):
    out = []
    infence = False
    for line in text.splitlines():
        if line.strip().startswith("```"):
            infence = not infence
            continue
        if infence:
            continue
        # strip inline code spans
        line = re.sub(r'`[^`]*`', ' ', line)
        out.append(line)
    return "\n".join(out)

results = []
for raw in ch:
    p = raw.replace("\\", "/")
    if not os.path.exists(p):
        results.append((p, "MISSING", 0, 0, 0.0, 0))
        continue
    size = os.path.getsize(p)
    text = open(p, encoding="utf-8", errors="replace").read()
    prose = strip_code(text)
    zh = len(zh_re.findall(prose))
    en = len(en_re.findall(prose))
    ratio = (zh / (zh + en)) if (zh+en) else 0.0
    # decision per protocol 2.2
    if zh + en < 8:
        dec = "SKIP(empty)"
    elif ratio >= 0.10 and en < 20:
        dec = "SKIP(done)"
    else:
        dec = "TR"
    results.append((p, dec, zh, en, round(ratio,3), size))

for p, dec, zh, en, ratio, size in results:
    name = p.split("/")[-1]
    print(f"{dec:12} zh={zh:5} en={en:5} r={ratio:.2f} {size:7}B  {name}")

tr = [r for r in results if r[1]=="TR"]
skip = [r for r in results if r[1].startswith("SKIP")]
missing = [r for r in results if r[1]=="MISSING"]
print(f"\nTOTAL={len(results)} TR={len(tr)} SKIP={len(skip)} MISSING={len(missing)}")
