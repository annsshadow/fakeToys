---
title: feat: Add Markdown docs output via Pandoc pipeline
type: feat
status: active
date: 2026-07-06
origin: docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md
deepened: 2026-07-06
---

# feat: Add Markdown docs output via Pandoc pipeline

## Summary

在现Sphinx 构建流水线之后串Pandoc 后处理，新增 `make markdowndocs` 目标，将 `Documentation/` HTML 输出转换Markdown，输出到 `Documentation/output/markdown/`。该方案不升Sphinx、不引入新的 Sphinx 扩展，保持对内核自定directive 的最大兼容性
---

## Problem Frame

内核文档使用 Sphinx 3.4.3 构建HTML/PDF/EPUB 等格式，但缺Markdown 输出路径。下游消费者（静态站点生成器、LLM  ingestion pipeline、跨项目镜像）只能依赖易碎的外部转换脚本。现Sphinx Markdown builder 均不兼容 Sphinx 3.4.3，因此采Pandoc 后处理作v1 方案see origin: docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md)

---

## Requirements

- R1. A new `markdowndocs` target must be added to `Documentation/Makefile` that generates Markdown output for the entire `Documentation/` tree.
- R2. Markdown generation must use a Sphinx Markdown builder extension registered in `Documentation/conf.py`.
  - **Overridden by this plan.** No Sphinx Markdown builder is compatible with Sphinx 3.4.3 + kernel custom extensions. R2 is satisfied by Pandoc post-processing (`tools/docs/md-convert.py`) invoked from `make markdowndocs`, which produces Markdown output through the documentation pipeline without modifying `conf.py`.
- R3. Generated `.md` files must be written to `Documentation/output/` alongside existing builder outputs.
  - **Clarified:** Output is written to `Documentation/output/markdown/`, nested under `BUILDDIR` following the `BUILDDIR/<format>/` convention established by existing builders (`html/`, `pdf/`, `epub/`). This keeps Markdown output isolated from other artifacts and automatically covered by `cleandocs`.
- R4. The new target must support the same `SPHINXDIRS` filtering mechanism as existing targets (e.g., `make SPHINXDIRS=process markdowndocs`).
- R5. The Markdown builder extension and its dependencies must be declared in a Sphinx requirements file under `Documentation/sphinx/`.
  - **Overridden by this plan.** Pandoc is a system-level binary dependency, not a Python/Sphinx package. It cannot be declared in `Documentation/sphinx/min_requirements.txt` or `requirements.txt`. The dependency is declared in this plan's Documentation / Operational Notes and surfaced to users via the conversion script's startup check.
- R6. The new target must integrate with the existing `sphinx-build-wrapper` and version-check flow used by other doc targets.
  - **Partially satisfied.** `markdowndocs` does not call `sphinx-build-wrapper` and does not participate in its `TARGETS` dictionary (Markdown generation is not a `sphinx-build` builder). Integration is limited to: (a) `make markdowndocs` depends on `htmldocs`, which goes through the full wrapper pipeline including `sphinx-pre-install --version-check`; (b) `SPHINXDIRS` and `BUILDDIR` Make variables are passed through to the conversion script.
- R7. Kernel custom Sphinx extensions must be evaluated for Markdown builder compatibility; incompatible extensions must be handled gracefully rather than crashing the build.
- R8. Markdown output must preserve document structure: headings, code blocks, lists, tables, and cross-references where the builder supports them.
- R9. The solution must not modify or remove any existing builder targets or output formats.

**Origin actors:** 内核文档维护者、下Markdown 消费**Origin flows:** F1 本地预览 / F2 CI 校验 / F3 完整构建
**Origin acceptance examples:** AE1 (SPHINXDIRS 过滤), AE2 (subtree 输出), AE3 (CI 验证), AE4 (现有目标零影

---

## Scope Boundaries

- 不迁`.rst` 源文件；源文件保持不变- 不修`Documentation/` 之外的任何内核源码- 不替换或废弃任何现有 builder 目标- 不新增顶Makefile 目标；入口为 `Documentation/Makefile`- 不升Sphinx；保`Documentation/sphinx/min_requirements.txt` Sphinx==3.4.3 的锁定- Pandoc 是系统级外部依赖，不纳入 Python 依赖管理；最低版本约束在 Documentation / Operational Notes 中声明- `make markdowndocs` v1 中定位为开发者工具，不强制纳入上游内CI（CI 集成deferred）
### Deferred to Follow-Up Work

- `flat-table` `colspan`/`rowspan` v1 中必须处理（检测并警告或保留原HTML），但更复杂的表格语义还原（如将合并单元格转Markdown 列表deferred- `flat-table` 外的内核自定directive（`kernel-doc`、`kernel-figure`、`kernel-render`、`automarkup`、`kernel_abi` 等）的深度后处理清洗逻辑，视 v1 输出质量再迭代- Markdown 输出集成进上游内CI 流水线（当前定位为开发者工具）- `sphinx-pre-install` 中为 pandoc 增加系统级依赖检查（v1 由转换脚本直接检测并报错）- 交叉引用链接格式标准化（site-root-relative vs. relative path 策略）——取决于 Markdown 输出是作为独立站点还是片段拼接使用
---

## Context & Research

### Relevant Code and Patterns

- `Documentation/Makefile` 所doc builder 目标共享一个模式规则，通过 `sphinx-build-wrapper` 调用 `sphinx-build`。新`markdowndocs` 需要作为独立目标，先依`htmldocs` 再串Pandoc- `tools/docs/sphinx-build-wrapper` `TARGETS` 字典注册 builder 名称与输出子目录。`markdowndocs` 不使用此 wrapper Sphinx 调用，但复用`SPHINXDIRS` 过滤`BUILDDIR` 约定- `tools/docs/sphinx-pre-install` 依赖检查入口；v1 不在其中注册 pandoc，由转换脚本自行检测- `Documentation/sphinx/min_requirements.txt` 锁定 Sphinx==3.4.3；v1 不动
### Institutional Learnings

无直接相关的 `docs/solutions/` 条目。外部调研确认：截至 2026 年，Sphinx 生态中唯一活跃Markdown 输出 builder（`sphinx-markdown-builder` 0.6.x）要Sphinx 7.3，与内核当前 3.4.3 不兼容；其余流行扩展（`myst-parser`、`m2r2`）均Markdown **输入** 解析器，不适用
### External References

- `sphinx-markdown-builder` GitHub issue #48：最Sphinx 版本要求 7.3- `sphinx-markdown-builder` GitHub issue #32：C domain 不兼容（对内核文档致命）- Pandoc 官方文档：`pandoc -f html -t markdown` 转换管线
---

## Key Technical Decisions

- **Pandoc 后处理管线而非 Sphinx 扩展* 调研确认无现Sphinx Markdown builder 兼容 Sphinx 3.4.3 + 内核自定义扩展。Pandoc 是唯一能在不升Sphinx 的前提下产出可用 Markdown 的方案- **自定docutils writer 仍被拒绝* 需求文档的 Key Decisions 中记录了三种方案（Pandoc、Sphinx 扩展、自定义 docutils writer）。自定义 writer 的实现和维护成本0000 行需Sphinx/docutils API 演进而维护） disproportionate 于其收益，Pandoc 后处理在成本和兼容性之间更平衡- **`markdowndocs` 作为独立 Makefile 目标，依`htmldocs`* 复用现有 HTML 构建全流程（包括 `sphinx-pre-install` 版本检查、`SPHINXDIRS` 过滤、Rust doc 集成），Pandoc 仅在 HTML 就绪后作为后处理步骤运行- **Markdown 输出目录`Documentation/output/markdown/`* 采用 `BUILDDIR/<format>/` 嵌套约定（与 `html/`、`pdf/`、`epub/` 一致），保持输出隔离，自动`cleandocs` 覆盖。根级别 `Documentation/output/` 被排除，以避免与 Sphinx 内部产物（`.doctrees/`、`<sphinxdir>/` 子目录）碰撞- **R2/R5/R6 被方案覆盖* 需求文档中 R2（Sphinx 扩展注册）、R5（Sphinx 依赖文件声明）和 R6（sphinx-build-wrapper 深度集成）在 Pandoc 架构下无法按字面满足。本计划显式覆盖这三条需求：功能意图Pandoc 后处理替代，依赖声明改为脚本启动检+ 本文档的 Documentation / Operational Notes- **Sphinx 升级迁移触发器* 当内核文档构建将 Sphinx 升级7.3 时，应重新评估替换为原生 `sphinx-markdown-builder`。迁移时a) `md-convert.py` v1 累积的后处理规则需评估是否可移植到 builder 的扩展钩子；(b) `markdowndocs` Makefile 目标结构可能简化- **v1 内核 directive 后处理范围* `flat-table` `colspan`/`rowspan` v1 正确性要求（Pandoc Markdown 表格语法不支持这些属性），必须在转换脚本中处理（检测并警告或保留为原始 HTML）。其directive（`kernel-doc`、`kernel-figure`、`kernel-render`）的后处理清洗在 v1 中最小化实现，视输出质量再迭代
---

## Open Questions

### Resolved During Planning

- **Markdown builder 选型* 调研确认无现成扩展兼Sphinx 3.4.3，选定 Pandoc 后处理- **输出目录位置* 选定 `BUILDDIR/markdown/`，遵`BUILDDIR/<format>/` 嵌套约定- **flat-table 处理策略* v1 检`colspan`/`rowspan` 并保留原HTML（Pandoc Markdown 表格语法不支持合并单元格），避免静默损坏- **Pandoc 最低版本：** 锁定 2.17+，由 `md-convert.py` 启动时检查- **R2/R5/R6 覆盖* 需求文档中 R2（Sphinx 扩展注册）、R5（Sphinx 依赖文件声明）、R6（sphinx-build-wrapper 深度集成）在 Pandoc 架构下无法按字面满足，已Requirements Key Technical Decisions 中显式覆盖
### Deferred to Implementation

- 交叉引用链接格式标准化（site-root-relative vs. relative path 策略）——取决于 Markdown 输出是作为独立站点还是片段拼接使用- `flat-table` 外的内核自定directive（`kernel-doc`、`kernel-figure`、`kernel-render`、`automarkup`、`kernel_abi` 等）的深度后处理清洗逻辑，视 v1 输出质量再迭代- `conf.py` 中全13 个扩展的 HTML 输出模式审计——实现时需逐扩展检查是否有未预期的 HTML 结构需要后处理
---

## Implementation Units

### U1. 添加 `markdowndocs` Makefile 目标

**Goal:** `Documentation/Makefile` 中新`markdowndocs` 目标，作`htmldocs` 的依赖串Pandoc 转换步骤
**Requirements:** R1, R3, R4, R6, R9

**Dependencies:** 鏃。
**Files:**
- Modify: `Documentation/Makefile`

**Approach:**
- 在现有共享模式规则之外，新增 `markdowndocs` 目标，以 `htmldocs` 为先决条件- `htmldocs` 完成后，调用 `tools/docs/md-convert.py` HTML 输出转换Markdown- 目标支持 `SPHINXDIRS` `BUILDDIR` 变量透传，行为与现有目标一致- 目标不参`sphinx-build-wrapper` `TARGETS` 字典，因Markdown 生成不经`sphinx-build`
**Patterns to follow:**
- 现有 `htmldocs-redirects` `refcheckdocs` 目标作为独立 Makefile 规则的参考（它们也不`sphinx-build-wrapper`）- `cleandocs` 自动覆盖 `BUILDDIR/markdown/`，无需额外清理规则
**Test scenarios:**
- Happy path: `make markdowndocs` `Documentation/output/markdown/` 下生`.md` 文件，内容为有效 Markdown- SPHINXDIRS 过滤: `make SPHINXDIRS=process markdowndocs` 仅转`process/` 子树- 无副作用: `make htmldocs` `make pdfdocs` 的输出与未添`markdowndocs` 前完全一致
**Verification:**
- `make markdowndocs` 退出码0，`Documentation/output/markdown/` 下存`.md` 文件- `process/`、`admin-guide/`、`core-api/`、`driver-api/` 的代表性文件抽样，确认标题、代码块、列表、表格在 Markdown 中可识别
---

### U2. 创建 Pandoc 转换脚本

**Goal:** 编写 `tools/docs/md-convert.py`，遍HTML 输出目录，调Pandoc 将每`.html` 文件转换`.md`，保持目录结构，并做内核文档特有的后处理
**Requirements:** R1, R3, R4, R7, R8, R9

**Dependencies:** U1

**Files:**
- Create: `tools/docs/md-convert.py`

**Approach:**
- 脚本接收 `--htmldir`（HTML 输出根目录）`--outdir`（Markdown 输出根目录）参数- **BUILDDIR 嵌套结构* Sphinx HTML 输出位于 `BUILDDIR/<sphinxdir>/html/`（由 `sphinx-build-wrapper` 737 行设定），而非扁平`BUILDDIR/html/`。脚本必须递归遍历此嵌套结构，保持相对路径映射Markdown 输出目录- `BUILDDIR` 可能为绝对路径（`make O=build` `$(obj)` 解析为绝对路径），脚本的 `--htmldir` 必须接受绝对路径- 对每`.html` 文件  1. 计算对应`.md` 输出路径，保持相对目录结构  2. 调用 `pandoc -f html -t markdown --wrap=none` 进行转换  3. 对输出做以下具体后处理（Python 字符串操作）     - **Sphinx shell  stripping* 移除 `<div class="related" role="navigation">`、`<div class="header">`、`<div class="footer">`、`<div class="document">` 外层包装，保`<div class="body">` 内的正文内容     - **headerlink 清理* 移除 `<a class="headerlink" href="#anchor">¶</a>` Permalink 链接     - **交叉引用重写* `<a class="reference internal" href="../subdir/page.html#anchor">` 中的 `.html` 后缀替换`.md`，相对路径根据输出目录层级重算（subtree 引用`../core-api/...` 需调整`../../core-api/...` 或统一site-root-relative）     - **flat-table 合并单元格处理：** 检`<table>` 中的 `colspan` `rowspan` 属性。Pandoc Markdown 表格语法不支持这些属性，转换会产生损坏输出。v1 处理策略：检测到 `colspan`/`rowspan` 时，打印警告并保留该表格为原HTML（不转换），避免静默损坏     - **kernel-doc 代码块验证：** 确认 `kerneldoc.py` 渲染后的 `<pre>` 块和 `<dl>/<dt>/<dd>` 参数列表Pandoc 正确转为 fenced code block 和定义列表；`.. LINENO` 注释`parse_msg()` 阶段已被剥离，不会出现在 HTML 中，无需额外处理     - **kfigure 图像路径重写* `kfigure.py` 渲染DOT/SVG 图像位于 `_static/` `_images/` 子目录。Markdown 输出中的 `<img src="...">` 路径需重写为相对于 Markdown 文件的位置，或复制图像到 Markdown 输出目录的对应位置     - **Rust doc 排除* `CONFIG_RUST=y` 时，`sphinx-build-wrapper` 会在 `BUILDDIR` 中生Rust 文档 HTML。转换脚本必须跳`rust/` `rustdoc/` 子目录（具体目录名需在实现时确认），不尝试转Rust 生成的页面- 脚本启动时检`pandoc` 是否PATH 中，若缺失则打印清晰的安装提示并退1- 脚本同时检`pandoc --version` 输出，验证版本不低于 2.17（该版本稳定支持 `--wrap=none` HTML 输入处理）- 脚本不依赖任何第三方 Python 库（仅标准库 + subprocess），避免新增 Python 依赖
**Patterns to follow:**
- `tools/docs/sphinx-build-wrapper` 的目录遍历和子进程调用风格- `tools/docs/sphinx-pre-install` 的依赖检查与错误提示风格
**Execution note:** 先用 `process/subprocess.rst` `admin-guide/kernel-parameters.rst` 等代表文件手动验Pandoc 输出质量，确认标题层级、代码块、表格、交叉引用在转换后可接受，再固定后处理规则
**Test scenarios:**
- Happy path: HTML 文件成功转换为结构对应的 Markdown 文件- 缺失 pandoc: 脚本检测到 pandoc 不在 PATH 时打印安装提示并退1- 嵌套目录: `BUILDDIR/process/subdir/page.html`（位`BUILDDIR/<sphinxdir>/html/` 嵌套结构中）正确转换`BUILDDIR/markdown/process/subdir/page.md`- 交叉引用: Sphinx 生成的内部链接（`<a class="reference internal" href="../subdir/page.html#anchor">`）在 Markdown 中变`[text](subdir/page.md#anchor)`，相对路径正确- headerlink 清理: 页面中的 `¶` Permalink 链接Markdown 输出中不存在- flat-table 合并单元 包含 `colspan` `rowspan` `<table>` 触发脚本警告，该表格保留为原HTML 块而非损坏Markdown 表格- Rust doc 排除: `BUILDDIR` 中包Rust 生成HTML 子目录时，脚本跳过该目录，不产生对应Markdown 文件- 绝对路径: `--htmldir` 接收绝对路径（如 `D:/WORKSPACE/linux-7.1.3/Documentation/output/html`）时正常转换- Pandoc 版本检 脚本检测到 pandoc 版本低于 2.17 时打印警告并退1
**Verification:**
- 对代表性文档抽样检查转换后Markdown：标题层级正确、代码块使用 fenced 语法、表格可识别、交叉引用链接可点击- `make markdowndocs` 整体流程退出码0
---

### U3. 确保现有目标零影响并补充使用文档

**Goal:** 验证 `make htmldocs`、`make pdfdocs`、`make cleandocs` 等现有目标的行为完全不变；在 `AGENTS.md` 中记Markdown 构建命令
**Requirements:** R9

**Dependencies:** U1, U2

**Files:**
- Modify: `AGENTS.md`（新Markdown 构建命令条目- 无新文件创建

**Approach:**
- 运行 `make htmldocs`、`make pdfdocs`、`make cleandocs`，确认输出与 baseline 一致- `AGENTS.md` Build Lint 段落中添`make markdowndocs` 命令说明
**Test scenarios:**
- `make htmldocs` 输出v1 变更前完全一致- `make cleandocs` 删除 `Documentation/output/` 全部内容（含新增`markdown/` 子目录）- `make markdowndocs` 可在干净源码树上独立运行（先自动触发 `htmldocs` 构建）- SPHINXDIRS 隔离: `make SPHINXDIRS=process markdowndocs` 仅在 `Documentation/output/markdown/process/` 下生`.md` 文件，不产生其他 subtree Markdown 输出- HTML 输出不变 `make markdowndocs` 完成后，`BUILDDIR/<sphinxdir>/html/` 下的 `.html` 文件内容与运行前完全一致（通过 checksum mtime 验证）- 幂等 连续运行 `make markdowndocs` 两次，第二次生成`.md` 文件与第一次字节级一致- BUILDDIR 覆盖: `make BUILDDIR=/tmp/test-build markdowndocs` 正确HTML 输出读自 `/tmp/test-build/<sphinxdir>/html/`，Markdown 输出写至 `/tmp/test-build/markdown/`- 部分失败清理: Pandoc 对某个文件返回非零退出码，脚本打印失败文件路径后退1，已生成`.md` 文件保留（不清理），便于调试- Rust doc 共存: `CONFIG_RUST=y` `BUILDDIR` 中包Rust 生成HTML 时，`make markdowndocs` 正常完成，不尝试转换 Rust 页面
**Verification:**
- 现有目标Makefile 规则未被修改；仅新增独立目标，零侵入- `AGENTS.md` 包含 `make markdowndocs` 命令
---

## System-Wide Impact

- **调用链：** `make markdowndocs` `make htmldocs`（Sphinx HTML 构建）→ `tools/docs/md-convert.py`（Pandoc 转换）。HTML 构建路径与现有目标完全共享，无分支- **错误传播* Pandoc 转换失败（非零退出码）时，`md-convert.py` 捕获异常并打印文件路径后退1，Make 终止。Sphinx 构建阶段的错误由现有机制处理，不变- **状态生命周期：** Markdown 输出位于 `BUILDDIR/markdown/`，由 `cleandocs` 统一清理，无残留- **API 表面* 仅新增一Make 目标和一Python 脚本，不修改任何现有 API、接口或行为- **不变约定* `Documentation/sphinx/min_requirements.txt` Sphinx==3.4.3 锁定不变；`Documentation/conf.py` extensions 列表不变；`tools/docs/sphinx-build-wrapper` `TARGETS` 字典不变
---

## Risks & Dependencies

| Risk | 可能| 影响 | 缓解措施 |
|---|---|---|---|
| Pandoc 对内核自定义 directive HTML 输出转换质量不佳 | | | v1 用代表文件手动验证；质量不足时通过后处理脚本清洗，不阻塞发|
| 部分系统环境Pandoc | | | 转换脚本检测缺失并打印安装提示；不影响 htmldocs/pdfdocs |
| 交叉引用转换后链接格式不统一 | | | v1 先输出原Pandoc 结果，按实际格式决定是否需标准|
| Pandoc 版本差异导致输出不一| | | 脚本启动时检Pandoc >= 2.17，低于此版本直接报错退|
| 内核 CI 环境未预Pandoc | | | v1 定位为开发者工具，不强制纳入上CI；后续如需 CI 集成再评|
| `colspan`/`rowspan` 表格静默损坏 | | | v1 检测到合并单元格时警告并保留原HTML，避Markdown 表格损坏 |

---

## Documentation / Operational Notes

- 用户在运`make markdowndocs` 前需确保系统已安Pandoc >= 2.17（`apt install pandoc` / `dnf install pandoc` / `brew install pandoc`）。转换脚本启动时自动检查版本，低于 2.17 时打印安装提示并退1- Markdown 输出HTML 的后处理产物，不代表源文件的权威版本；所有文档变更仍需`.rst` 源文件中进行- v1 定位为开发者工具，不强制纳入上游内CI。若后续需CI 集成，Pandoc 需添加CI 镜像provisioning 步骤- **迁移触发器：** 当内核文档构建将 Sphinx 升级7.3 时，应重新评估替换为原生 `sphinx-markdown-builder`。迁移时a) `md-convert.py` v1 累积的后处理规则需评估是否可移植到 builder 的扩展钩子；(b) `markdowndocs` Makefile 目标结构可能简化。Pandoc 路径是时间限定的桥接方案，不应永久固化
---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md](docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md)
- **Sphinx Markdown builder 兼容性调研：** `sphinx-markdown-builder` issue #48（Sphinx 7.3 要求）、issue #32（C domain 不兼容）
- **相关代码* `Documentation/Makefile`、`tools/docs/sphinx-build-wrapper`、`tools/docs/sphinx-pre-install`、`Documentation/sphinx/min_requirements.txt`、`Documentation/conf.py`
