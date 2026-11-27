import json,sys
rows=[l.rstrip("\n") for l in sys.stdin if l.startswith("CAP|")]
out=[]; naked=0; ring=0
for l in rows:
  p=l.split("|",4)
  vp,pg,n,raw=p[1],p[2],p[3],p[4]
  try:
    s=raw.strip()
    if s.startswith(chr(34)): s=json.loads(s)
    o=json.loads(s) if isinstance(s,str) else s
  except Exception as e:
    out.append((vp,pg,n,"PARSE_ERR",raw[:60]));continue
  if o.get("tag")=="BODY": naked+=1; continue
  ov=o.get("ov",""); bs=o.get("bs","")
  parts=ov.split("|")
  has_ring = len(parts)>=2 and parts[0]=="solid" and parts[1] not in ("0px","") and "rgba(0, 0, 0, 0)" not in ov
  has_shadow = bs not in ("none","")
  if has_ring or has_shadow: ring+=1
  out.append((vp,pg,n,o.get("tag"),o.get("cls","")[:24],o.get("txt","")[:18],"FV="+str(o.get("fv")),ov,o.get("ooff"),("SHADOW" if has_shadow else "-"),("RING" if has_ring else "NO_RING")))
print("CAPROWS",len(rows),"BODY",naked,"FOCUS",len(out)-out.count(None),"RING_OR_SHADOW",ring)
for r in out: print(" | ".join(str(x) for x in r))
