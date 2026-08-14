import json, re, os, sys

BASE = "D:/WORKSPACE/linux-7.1.3/docs/系统文档"
d = json.load(open(os.path.join(BASE, ".translate_dispatch_cold2.json")))
chunk = set(p.replace("\\","/") for p in d["chunks"][0])
zh_re = re.compile(r'[\u3400-\u4dbf\u4e00-\u9fff]')
en_re = re.compile(r'[A-Za-z]+')

def strip_code(text):
    out=[]; inf=False
    for line in text.splitlines():
        if line.strip().startswith("```"):
            inf=not inf; continue
        if inf: continue
        line=re.sub(r'`[^`]*`',' ',line)
        out.append(line)
    return "\n".join(out)

def selfcheck(target):
    text=open(target,encoding="utf-8",errors="replace").read()
    fences=text.count("```")
    if fences%2!=0:
        return False, f"odd fences={fences}"
    prose=strip_code(text)
    zh=len(zh_re.findall(prose)); en=len(en_re.findall(prose))
    ratio=zh/(zh+en) if (zh+en) else 0
    if ratio<0.03 and (zh+en)>8:
        return False, f"zh_ratio={ratio:.3f}"
    return True, f"fences={fences} zh_ratio={ratio:.3f}"

# commit all .tmp whose target is in chunk list
tmp_files=[]
for dp,_,fns in os.walk(BASE):
    for fn in fns:
        if fn.endswith(".tmp"):
            tmp=os.path.join(dp,fn)
            target=tmp[:-4]
            if target in chunk:
                tmp_files.append((tmp,target))

print(f"committing {len(tmp_files)} tmp files")
results=[]
for tmp,target in tmp_files:
    try:
        os.replace(tmp,target)
        ok,msg=selfcheck(target)
        results.append((target, ok, msg))
        print(("PASS" if ok else "FAIL"), msg, "->", target.split("/")[-1])
    except Exception as e:
        results.append((target, False, f"replace-error:{e}"))
        print("FAIL replace-error:",e, target)

fails=[r for r in results if not r[1]]
print(f"\nDONE commit: {len(results)-len(fails)} ok, {len(fails)} fail")
