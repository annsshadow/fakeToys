import os, re, glob
os.chdir("laiyipao/miniapp/src/pages")
files = glob.glob("**/*.vue", recursive=True)
out = []
for f in files:
    s = open(f, encoding="utf-8").read().splitlines()
    for i, line in enumerate(s):
        if not re.search(r"class=\"[^\"]*\blink\b[^\"]*\"", line):
            continue
        j = i
        sbuf = ""
        while j >= 0 and j >= i-8:
            sbuf = s[j] + " " + sbuf
            if re.search(r"<(text|view)\b", s[j]): break
            j -= 1
        k = i
        abuf = ""
        while k < len(s) and k <= i+6:
            abuf += s[k] + " "
            if "@click" in s[k] or (">" in s[k] and k > i): break
            k += 1
        blob = sbuf + abuf
        if "@click" not in blob:
            continue
        role = "role=\"button\"" in blob
        tab = "tabindex=\"0\"" in blob
        key = "@keydown.enter" in blob
        txt = " ".join(x.strip() for x in s[j:k+1])[:140]
        out.append((f, j+1, role, tab, key, txt))
print("TOTAL_LINK_CLICK=" + str(len(out)))
for o in out:
    print(o[0] + ":" + str(o[1]) + "|role=" + str(o[2]) + "|tab=" + str(o[3]) + "|key=" + str(o[4]))
    print("    " + o[5])
