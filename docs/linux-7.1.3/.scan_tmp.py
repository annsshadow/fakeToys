import json, os, re
d = json.load(open('.translate_dispatch_cold2.json', encoding='utf-8'))
chunk = d['chunks'][0]

done = {
 'arch/arm64/silicon-errata.md','bpf/libbpf/program_types.md','filesystems/nfs/nfs41-server.md',
 'userspace-api/media/v4l/capture.c.md','admin-guide/media/gspca-cardlist.md',
 'admin-guide/media/saa7134-cardlist.md','admin-guide/media/bttv-cardlist.md',
 'admin-guide/media/em28xx-cardlist.md','admin-guide/media/frontend-cardlist.md',
 'dev-tools/kunit/run_wrapper.md','networking/device_drivers/ethernet/netronome/nfp.md',
 'driver-api/clk.md','PCI/msi-howto.md','admin-guide/hw-vuln/rsb.md','admin-guide/parport.md',
}

for p in chunk:
    path = p.replace('\\', '/')
    rel = path.split('系统文档/',1)[1]
    if rel in done:
        continue
    try:
        lines = open(path, encoding='utf-8', errors='ignore').read().split('\n')
    except Exception as e:
        print('ERR', rel, e); continue
    # first non-empty line
    first = ''
    for ln in lines:
        if ln.strip():
            first = ln.strip()
            break
    zh_total = len(re.findall(r'[一-鿿]', '\n'.join(lines)))
    has_cn_first = bool(re.search(r'[一-鿿]', first))
    print(f"{'CN' if has_cn_first else 'EN':3} zh={zh_total:5}  {rel}")
