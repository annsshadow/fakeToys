import os, re
BASE="D:/WORKSPACE/linux-7.1.3/docs/系统文档"
p=os.path.join(BASE,"sphinx-includes/subproject-index.md")
orig=open(p,encoding='utf-8').read()
print("BEFORE:",repr(orig))
new=orig.replace("## Indices","## 索引").replace("- genindex","- genindex\n\n本页面提供文档索引入口。",1)
tmp=p+".tmp"
open(tmp,'w',encoding='utf-8').write(new)
try: os.replace(tmp,p)
except: open(p,'w',encoding='utf-8').write(new)
if os.path.exists(tmp): os.remove(tmp)
fin=open(p,encoding='utf-8').read()
def pr_of(t):
    out=[]; inb=False
    for ln in t.split('\n'):
        if ln.lstrip().startswith('```'): inb=not inb; continue
        if inb: continue
        out.append(re.sub(r'`[^`]*`','',ln))
    p='\n'.join(out); c=len(re.findall(r'[\u4e00-\u9fff]',p)); return c/len(p) if p else 0.0
fc=sum(1 for ln in fin.split('\n') if ln.lstrip().startswith('```'))
print("AFTER pr=%.4f fences=%d ok=%s"%(pr_of(fin),fc,fc%2==0 and pr_of(fin)>0.03))
print(fin)
