import os, re

BASE = os.path.dirname(os.path.abspath(__file__))

files = [
'hwmon/jc42.md','virt/acrn/io-request.md','driver-api/mailbox.md','driver-api/memory-devices/ti-gpmc.md',
'PCI/pci-iov-howto.md','admin-guide/cgroup-v1/rdma.md','userspace-api/media/drivers/camera-sensor.md','gpu/zynqmp.md',
'admin-guide/device-mapper/writecache.md','driver-api/acpi/scan_handlers.md','scheduler/sched-domains.md','driver-api/iio/buffers.md',
'arch/arm/sunxi.md','admin-guide/media/dvb-usb-dib0700-cardlist.md','admin-guide/media/cx23885-cardlist.md','userspace-api/media/v4l/vidioc-dbg-g-register.md',
'userspace-api/media/v4l/vidioc-s-hw-freq-seek.md','arch/s390/vfio-ap-locking.md','misc-devices/spear-pcie-gadget.md','admin-guide/thermal/intel_thermal_throttle.md',
'PCI/endpoint/pci-vntb-function.md','admin-guide/media/tuner-cardlist.md','hwmon/abituguru.md','block/pr.md',
'infiniband/tag_matching.md','networking/gen_stats.md','gpu/amdgpu/display/dc-glossary.md','arch/powerpc/htm.md',
'admin-guide/LSM/Yama.md','admin-guide/hw-vuln/cross-thread-rsb.md','hwmon/fam15h_power.md','usb/raw-gadget.md',
'arch/arm/setup.md','admin-guide/mm/damon/stat.md','hwmon/max16601.md','misc-devices/apds990x.md',
'admin-guide/sysctl/index.md','networking/mac80211-injection.md','hwmon/lm63.md','gpu/drm-ras.md',
'hwmon/max31760.md','driver-api/media/drivers/contributors.md','driver-api/soundwire/locking.md','arch/s390/mm.md',
'userspace-api/media/rc/lirc-get-features.md','driver-api/pwrseq.md','userspace-api/media/v4l/pixfmt-srggb8-pisp-comp.md','trace/events-power.md',
'gpu/imagination/uapi.md','locking/rt-mutex.md','i2c/i2c-address-translators.md','arch/arm/nwfpe/todo.md',
'networking/device_drivers/ethernet/ti/am65_nuss_cpsw_switchdev.md','crypto/sha3.md','admin-guide/media/avermedia.md'
]

cjk = re.compile(r'[\u4e00-\u9fff\u3400-\u4dbf]')
latin = re.compile(r'[A-Za-z]+')

def analyze(path):
    txt = open(path, encoding='utf-8', errors='ignore').read()
    lines = txt.split('\n')
    out = []
    depth = 0
    for ln in lines:
        s = ln.lstrip()
        if s.startswith('```'):
            depth = 1 - depth
            continue
        if depth == 0:
            out.append(ln)
    prose = '\n'.join(out)
    c = len(cjk.findall(prose))
    nonc = len(latin.findall(prose))
    ratio = c / max(1, (c + nonc))
    # english prose runs (lines mostly latin, no cjk)
    eng_lines = 0
    eng_words = 0
    for ln in out:
        c2 = len(cjk.findall(ln))
        n2 = len(latin.findall(ln))
        is_url = ('http' in ln or '://' in ln or ln.strip().startswith('|'))
        if n2 >= 4 and c2 == 0 and not is_url:
            eng_lines += 1
            eng_words += n2
    return c, nonc, ratio, eng_words

print("file | cjk | eng | ratio | eng_prose_words | verdict")
for f in files:
    p = os.path.join(BASE, f)
    c, nonc, ratio, ew = analyze(p)
    if c == 0 and nonc == 0:
        verdict = 'EMPTY?'
    elif ratio >= 0.10 and nonc < 20:
        verdict = 'SKIP(complete)'
    elif ratio < 0.10 and ew < 8:
        verdict = 'SKIP(struct/low)'
    elif ratio < 0.10:
        verdict = 'TRANSLATE'
    else:
        verdict = 'HALF-review'
    print('%s | %d | %d | %.2f | %d | %s' % (f, c, nonc, ratio, ew, verdict))
