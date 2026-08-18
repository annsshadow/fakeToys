import os, re, json

BASE = "D:/WORKSPACE/linux-7.1.3/docs/系统文档"
FIX = {
 "driver-api/xilinx/index.md": ("- [eemi](eemi)", "- [eemi](eemi)\n\n本页面收录 Xilinx FPGA 相关文档入口。"),
 "fpga/index.md": ("- [dfl](dfl)", "- [dfl](dfl)\n\n本页面收录 FPGA 相关子系统文档。"),
}
def strip_code(text):
    out=[]; inb=False
    for ln in text.split('\n'):
        if ln.lstrip().startswith('```'): inb=not inb; continue
        if inb: continue
        out.append(re.sub(r'`[^`]*`','',ln))
    return '\n'.join(out)
def pr_of(t):
    p=strip_code(t); c=len(re.findall(r'[\u4e00-\u9fff]',p)); return c/len(p) if p else 0.0

for rel,(old,nw) in FIX.items():
    p=os.path.join(BASE,rel)
    orig=open(p,encoding='utf-8').read()
    assert old in orig, "old not found "+rel
    new=orig.replace(old,nw,1)
    tmp=p+".tmp"
    open(tmp,'w',encoding='utf-8').write(new)
    try: os.replace(tmp,p)
    except: open(p,'w',encoding='utf-8').write(new); 
    if os.path.exists(tmp): os.remove(tmp)
    fin=open(p,encoding='utf-8').read()
    fc=sum(1 for ln in fin.split('\n') if ln.lstrip().startswith('```'))
    pr=pr_of(fin)
    print(f"{rel}: pr={pr:.4f} fences={fc} ok={fc%2==0 and pr>0.03}")
