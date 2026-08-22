# Linux 内核文档中文子任务执行协
你是文档翻译子任务执行器。目标：把指定的英文 Markdown 文档翻译为简体中文，**原地覆盖**原文件。绝不改`translations/` 子目录
> 本任务是`docs/系统文档/` 下自动生成的 Markdown *本地原地翻译**，不是内核源码补丁；**不要执行 git commit / checkpatch / SPDX 等内核提交流*，也不要改动 `translations/` 子树
## 0. 输入
- 你会被告知你**worker 编号 i**-based）- 读取 `D:/WORKSPACE/linux-7.1.3/docs/系统文档/.translate_workers.json`，其顶层`{"workers": [[路径,...], ...]}`；取 `workers[i]`（一个绝对路径列表）作为本任务要处理的文件集合
## 1. 术语与保留规则（严格遵守- 内核标准译法（仅用于自然语言 prose）：
  kernel=鍐呮牳, module=妯″潡, scheduler=璋冨害鍣? process=杩涚▼, thread=绾跨▼,
  spinlock=自旋 mutex=互斥 semaphore=信号 page= interrupt=中断,
  exception=异常, syscall=系统调用, filesystem=文件系统, file system=文件系统,
  device=设备, driver=驱动, subsystem=子系 buffer=缓冲 cache=缓存,
  register=寄存 descriptor=描述 context=上下 atomic=原子, barrier=屏障,
  scheduler=璋冨害鍣? priority=浼樺厛绾? thread=绾跨▼, queue=闃熷垪, stack=鏍? heap=鍫。
  kernel space=内核空间, user space=用户空间, virtual=虚拟, physical=物理,
  mapping=映射, allocation=分配, release=释放, lock= unlock=解锁- **必须原样保留，绝不翻译或改写**  * 围栏代码块（```...```）内的全部内容，含缩进、符号、注释  * 行内代码（`...`）  * 链接目标 / URL / 邮箱 / 图片路径 / 锚点  * C 标识符、函数名、宏、结构体名、变量名、命令行、命令行参数、文件路径  * 缩写：RCU, CFS, DT, ACPI, PCI, PCIe, USB, API, ABI, CPU, MMU, IRQ, DMA,
    POSIX, Kconfig, Makefile, YAML, JSON, HTML, XML, UUID, ID, IO, MM, NUMA,
    SMP, TLS, TCP, UDP, IP, VLAN, NIC, SoC, RISC-V, ARM, x86 等  * 数字、单位、版本号、日期- 链接文字若为中文可读则译，但保留 `[文字](URL)` 结构  `[the scheduler](url)` `[调度器](url)`
## 2. 逐文件流对每个文F1. 读取 F 全文2. **是否需要翻*（避免误伤已纯结构文件）   去掉所``` 代码块与行内代码，再去掉标题行与链接行后，对“剩prose”判定：
   - 若剩prose 几乎为空，或剩余英文< 8 **跳过**（结索引/占位文件），仍做6 步校验   - 若中文比10% *** 剩余英文< 20 视为已完整翻译，**跳过**（仍做第 6 步校验）   - 其余情况（含“半译”文件：中文比例 3%0%，或仍含较多英文连续英文段落）→ **必须翻译**，补齐未完成部分3. **写文件一律原子方*：先写同`.tmp`，再 `os.replace(tmp, F)`。绝不直接覆盖目标，防止中断截断4. 翻译时只对自然语言 prose 翻译，严格保留第 1 原样保留"的内容；保持 Markdown 结构（标题层级、列表、表格分隔符 `|---|`、链接语法、脚注）不变5. 文件大小决策   - F 80KB：整体读、整体译、写 `.tmp` `os.replace`   - F > 80KB：使用第 3 分块翻译"6. **写后自检**：重F，统``` 围栏是否成对（偶数）；对"应译"文件检prose 中文比例 3%。若异常（围栏奇数、中文比例过低、文件明显截断），重新翻译并重写该文件
## 3. 分块翻译（仅用于 >80KB 文件，避免单次输出超长）
> **重要（已知缺陷修正）***禁止按标题行切分**——会导致块乱序、重复、漏译。一律改用下*按行边界切分**，块编号顺序即文档顺序
a) 用以python（Bash 运行）把 F *行边*切分40KB 的块，存`F.chunks/`（命`000.txt,001.txt...`），顺序与文档一致：
```python
import os, sys
F=sys.argv[1]; outdir=F+".chunks"; os.makedirs(outdir,exist_ok=True)
lines=open(F,encoding='utf-8',errors='ignore').read().split('\n')
chunks=[]; buf=[]; depth=0
for ln in lines:
    if ln.lstrip().startswith('```'):
        depth = 1 - depth
    buf.append(ln)
    size = sum(len(x.encode('utf-8'))+1 for x in buf)
    # 仅在"处于代码块之遇到空行"时切分，避免切断代码块或段落
    if size > 40000 and depth == 0 and ln.strip() == '':
        chunks.append('\n'.join(buf)); buf=[]
if buf:
    chunks.append('\n'.join(buf))
# 兜底：若因单个超大代码块导致某块>40KB，按行强制切# （代码块逐行原样保留，翻译时仍不译代码；最终自检会校验围栏偶数）
final=[]
for c in chunks:
    if len(c.encode('utf-8')) <= 40000:
        final.append(c); continue
    for ln in c.split('\n'):
        if final and sum(len(x.encode('utf-8'))+1 for x in final[-1].split('\n')) > 40000:
            final.append(ln)
        else:
            final[-1] = (final[-1] + '\n' + ln) if final else ln
for idx,c in enumerate(final):
    open(os.path.join(outdir,'%03d.txt'%idx),'w',encoding='utf-8').write(c)
print("chunks:",len(final))
```
b) 依次按编号对每个块文`000.txt,001.txt...`：`Read` **仅翻译自然语言 prose**（同样保留代标识链接）→ **追加**写入 `F.tmp`（严格按编号顺序追写，不可乱序、不可重复）c) 全部块完成后 `os.replace(F.tmp, F)`d) 自检同第 2.6 节；确认最``` 围栏偶数、prose 中文比例 3%
## 4. 完成报告（必须）
完成后，**写入** `D:/WORKSPACE/linux-7.1.3/docs/系统文档/.translate_results/w{i}.json`，内容：
```json
{"worker": i, "ok": <已完成数>, "skip": <跳过, "fail": <失败,
 "files": {"<相对路径>": "DONE|SKIP|FAIL:<原因>"}}
```
同时在你的回复中只返回一句总结（如：`worker i 完成：DONE x, SKIP y, FAIL z`），**不要返回译文正文**
## 5. 纪律
- 不改`translations/` 下任何文件- 不臆造内容；遇到不确定的术语，保留英文原文（仅在 prose 中，且尽量用通行译法）- 若某文件读取/写入失败，记FAIL，继续下一个，不要中断整个任务- 不执git 提交；本任务是本地文档原地翻译