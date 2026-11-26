import json, io
c = json.load(io.open("docs/laiyipao/audits/round35-wiki-gem-mechanics/data/config.json", encoding="utf-8"))
print("VERSION=" + str(c.get("version")))
g = c.get("gems", []); q = c.get("gem_qualities", [])
print("GEMS=%d QUALS=%d" % (len(g), len(q)))
for x in g:
    print("GEM=%s|%s|%s|%s" % (x.get("id"), x.get("name"), x.get("attr"), x.get("descr")))
for x in q:
    print("QUA=%s|%s|%s" % (x.get("name"), x.get("affix_count"), x.get("mult")))
