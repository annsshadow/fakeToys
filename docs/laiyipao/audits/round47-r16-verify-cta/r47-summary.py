import json, pathlib
D=pathlib.Path("docs/laiyipao/audits/round47-r16-verify-cta/data")
def load(n):
    s=(D/n).read_text(encoding='utf-8').strip()
    d=json.loads(s)
    if isinstance(d,str): d=json.loads(d)
    return d
for n in ["list-probe.json","cta-before.json","cta-after.json"]:
    try:
        d=load(n)
    except Exception as e:
        print(n,"PARSE_ERR",e); continue
    print("---",n)
    for k in ["href","isLogin","drawerOpen","drawerScrollTop","drawerScrollH","drawerClientH","windowScrollY","ctaText","ctaCursor","ctaRect","verifyRowInViewport","verifyRows","hashes","inputs","verifyResultCount","timelineText","bodyLen"]:
        print(" ",k,"=",d.get(k))
