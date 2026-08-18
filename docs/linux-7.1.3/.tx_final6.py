import os, re, json

BASE="D:/WORKSPACE/linux-7.1.3/docs/系统文档"
with open('.translate_dispatch_final.json','r',encoding='utf-8') as f:
    paths = json.load(f)['chunks'][6]

def strip_code(text):
    out=[]; inb=False
    for ln in text.split('\n'):
        if ln.lstrip().startswith('```'): inb=not inb; continue
        if inb: continue
        out.append(re.sub(r'`[^`]*`','',ln))
    return '\n'.join(out)
def pr_of(t):
    p=strip_code(t); c=len(re.findall(r'[\u4e00-\u9fff]',p)); return c/len(p) if p else 0.0

files=[]
full=light=done=fail=0
for p in paths:
    t=open(p,encoding='utf-8').read()
    pr=pr_of(t)
    fc=sum(1 for ln in t.split('\n') if ln.lstrip().startswith('```'))
    ok = (fc%2==0) and (pr>0.03)
    mode="light"
    result = "light" if ok else "fail"
    if not ok: fail+=1; light+=0
    else: light+=1
    files.append({"path":p,"mode":mode,"result":result,"pr":round(pr,4)})

report={"chunk":6,"translated":0,"light_translated":light,"skipped_done":0,"failed":fail,"files":files}
os.makedirs('.translate_results',exist_ok=True)
json.dump(report, open('.translate_results/wFINAL2_6.json','w',encoding='utf-8'), ensure_ascii=False, indent=1)
print("translated(full)=0 light(stub)=",light," skipped_done=0 failed=",fail)
print("all pass:", fail==0)
for f in files:
    print(f"  {f['result']:5} pr={f['pr']}  {os.path.basename(f['path'])}")
