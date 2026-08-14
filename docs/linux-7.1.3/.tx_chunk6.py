import os, re, json

BASE = "D:/WORKSPACE/linux-7.1.3/docs/系统文档"

# (path, [(old, new), ...])  light translations
REPL = {
 "driver-api/media/v4l2-rect.md": [
    ("##### V4L2 rect helper functions", "##### V4L2 矩形辅助函数"),
 ],
 "gpu/xe/xe_map.md": [
    ("## Map Layer", "## 映射层"),
    ("   :doc: Map layer", "   :doc: 映射层"),
 ],
 "driver-api/media/v4l2-dv-timings.md": [
    ("##### V4L2 DV Timings functions", "##### V4L2 DV 时序函数"),
 ],
 "driver-api/xilinx/index.md": [
    ("## Xilinx FPGA", "## Xilinx FPGA"),
    ("- [eemi](eemi)", "- [eemi](eemi)\n\n本页面收录 Xilinx FPGA 相关文档入口。"),
 ],
 "driver-api/media/dtv-net.md": [
    ("### Digital TV Network kABI", "### 数字电视网络 kABI"),
 ],
 "userspace-api/media/dvb/dmx_types.md": [
    ("######## Demux Data Types", "######## Demux 数据类型"),
 ],
 "userspace-api/media/dvb/frontend-header.md": [
    ("## Frontend uAPI data types", "## 前端 uAPI 数据类型"),
 ],
 "driver-api/i3c/device-driver-api.md": [
    ("## I3C device driver API", "## I3C 设备驱动 API"),
 ],
 "mm/shmfs.md": [
    ("## Shared Memory Filesystem", "## 共享内存文件系统"),
 ],
 "userspace-api/media/dvb/net-types.md": [
    ("######## Net Data Types", "######## 网络数据类型"),
 ],
 "userspace-api/media/dvb/ca_data_types.md": [
    ("######## CA Data Types", "######## CA 数据类型"),
 ],
 "mm/oom.md": [
    ("## Out Of Memory Handling", "## 内存耗尽处理"),
 ],
 "sphinx-includes/subproject-index.md": [
    ("## Indices", "## 索引"),
    ("- genindex", "- genindex\n\n本页面提供文档索引入口。"),
 ],
 "fpga/index.md": [
    ("## FPGA", "## FPGA"),
    ("- [dfl](dfl)", "- [dfl](dfl)\n\n本页面收录 FPGA 相关子系统文档。"),
 ],
 "gpu/amdgpu/xgmi.md": [
    ("##  AMDGPU XGMI Support", "## AMDGPU XGMI 支持"),
 ],
 "crypto/libcrypto-utils.md": [
    ("## Utility functions", "## 工具函数"),
 ],
 "driver-api/media/v4l2-fwnode.md": [
    ("##### V4L2 fwnode kAPI", "##### V4L2 fwnode 内核 API"),
 ],
 "driver-api/media/v4l2-async.md": [
    ("##### V4L2 async kAPI", "##### V4L2 async 内核 API"),
 ],
 "driver-api/media/v4l2-cci.md": [
    ("##### V4L2 CCI kAPI", "##### V4L2 CCI 内核 API"),
 ],
 "mm/page_allocation.md": [
    ("## Page Allocation", "## 页分配"),
 ],
 "mm/page_reclaim.md": [
    ("## Page Reclaim", "## 页回收"),
 ],
 "gpu/xe/xe_debugging.md": [
    ("## Debugging", "## 调试"),
 ],
 "mm/bootmem.md": [
    ("## Boot Memory", "## 启动内存"),
 ],
 "mm/swap.md": [
    ("## Swap", "## 交换"),
 ],
}

def strip_code(text):
    lines = text.split('\n')
    out=[]; in_block=False
    for ln in lines:
        if ln.lstrip().startswith('```'):
            in_block = not in_block
            continue
        if in_block: continue
        ln = re.sub(r'`[^`]*`','',ln)
        out.append(ln)
    return '\n'.join(out)

def pr_of(text):
    prose = strip_code(text)
    cjk = len(re.findall(r'[\u4e00-\u9fff]', prose))
    total = len(prose)
    return (cjk/total) if total else 0.0

def fence_count(text):
    return sum(1 for ln in text.split('\n') if ln.lstrip().startswith('```'))

results=[]
for rel, reps in REPL.items():
    p = os.path.join(BASE, rel)
    orig = open(p, encoding='utf-8').read()
    new = orig
    for old, nw in reps:
        if old not in new:
            results.append({"path":rel,"mode":"light","result":"fail","pr":0.0,"note":"old not found: "+repr(old)})
            continue
        new = new.replace(old, nw, 1)
    # re-apply safety: ensure all reps applied
    for old, nw in reps:
        if old in new:
            results[-1] = {"path":rel,"mode":"light","result":"fail","pr":0.0,"note":"not replaced: "+repr(old)}
            break
    if any(r.get("path")==rel and r.get("result")=="fail" for r in results):
        continue
    # atomic write
    tmp = p + ".tmp"
    open(tmp, 'w', encoding='utf-8').write(new)
    try:
        os.replace(tmp, p)
    except Exception as e:
        open(p,'w',encoding='utf-8').write(new)
        if os.path.exists(tmp): os.remove(tmp)
    # self-check
    final = open(p, encoding='utf-8').read()
    fc = fence_count(final)
    pr = pr_of(final)
    ok = (fc % 2 == 0) and (pr > 0.03)
    res = "light" if ok else "fail"
    results.append({"path":rel,"mode":"light","result":res,"pr":round(pr,4),"fences":fc,"note":"" if ok else "pr<=0.03" if pr<=0.03 else "odd fence"})

json.dump(results, open('.tx_chunk6_results.json','w',encoding='utf-8'), ensure_ascii=False, indent=1)
for r in results:
    print(f"{r['result']:5} pr={r.get('pr')} fences={r.get('fences')} :: {r['path']} {r.get('note','')}")
