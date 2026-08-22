---
title: feat: Generate PROJECT.md from kernel source
type: feat
status: active
date: 2026-07-06
origin: docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md
deepened: 2026-07-06
---

# feat: Generate PROJECT.md from kernel source

## Summary

编写一Python 脚本 `tools/docs/gen-project-md.py`，从 Linux 内核源码直接提取信息生成 `docs/PROJECT.md`，内容包括目录树（带用途说明）、Kconfig 配置项摘要、Makefile 目标列表、各子系README/头文件注释摘要。信息来源为源码文件本身，不依赖 `Documentation/` 下已有的 `.rst` 文件
---

## Problem Frame

Linux 内核项目规模庞大（~36,681 C 文件、~26,666 个头文件）， newcomers 或下游消费者难以快速建立对项目结构的整体认知。`Documentation/` 下的文档详尽但分散，且以 reStructuredText 为主。一份从源码直接生成Markdown 总览文档能降低入门门槛，支持 LLM ingestion、静态站点生成和跨项目镜像see origin: docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md)

---

## Requirements

- R1. 必须从源码直接提取信息，不依`Documentation/` 下已有的 `.rst` 文件作为信息来源- R2. 输出文件`docs/PROJECT.md`（Markdown 格式）- R3. 文档必须包含目录树：项目顶层和关键子目录的递归结构，每个目录附带用途说明（README、Makefile 注释、或目录名推断）- R4. 文档必须包含 Kconfig 配置项摘要：从根 `Kconfig` 和关键子系统 Kconfig 中提取主要配置选项，以 Markdown 表格或列表形式呈现- R5. 文档必须包含 Makefile 目标列表：从顶层 `Makefile` `Documentation/Makefile` 中提取主要构建目标，附带简短说明- R6. 文档必须包含各子系统README/头文件注释摘要：扫描每个关键子目录下`README*` 文件和核心头文件开头的注释块，提取子系统描述- R7. 生成过程必须是可重复的：重新运行生成脚本应产生一致的输出- R8. 生成脚本必须不修改任何源码文件，仅读取
**Origin actors:** 内核 newcomers、下游消费者、LLM ingestion 管道
**Origin flows:** F1 本地项目总览 / F2 CI 生成 / F3 跨项目镜
---

## Scope Boundaries

- 不提kernel-doc 注释（`/** ... */`）到 PROJECT.md；kernel-doc 的输出留给现有工具链处理- 不生API 级别的详细函结构体文档；PROJECT.md 是项目总览，不API 参考- 不修改任何源码或 `Documentation/` 下的文件- 不替`Documentation/` 的现有文档体系；PROJECT.md 是入口级总览，不是完整文档- 不递归扫描所~6,143 个目录；聚焦顶层和关键子目录（约 15-20 个）
---

## Context & Research

### Relevant Code and Patterns

- `tools/docs/kernel-doc` 现有 Python 脚本，从 C/H 文件提取 kernel-doc 注释，输ReST man page。其架构（解析器 + 输出格式类）可作gen-project-md.py 的参考模式- `tools/docs/md-convert.py` 新建Markdown 生成脚本（本项目同期实施），使用标准+ subprocess，无第三方依赖，风格可借鉴- `tools/docs/sphinx-pre-install` 依赖检查风格参考- `tools/lib/python/kdoc/kdoc_output.py` kernel-doc 的输出格式基`OutputFormat` `RestFormat`/`ManFormat` 实现。`MarkdownFormat` 可以作为 future work 加入此体系- `Kconfig` 根配置文件，34 行，通过 `source` 指令引用子目Kconfig，语法为 `config`/`menuconfig`/`choice`/`endchoice`/`endif`- `Makefile` 2307 行，目标定义模式`<target>:` `PHONY += <target>`，目标分散在多处- `README` 根目README 和多个子目录 README 文件存在，但关键子系统（`arch/`、`drivers/`、`fs/` 等）没有 README
### Institutional Learnings

无直接相关的 `docs/solutions/` 条目
### External References

- Kconfig 语法文档：`Documentation/kbuild/kconfig-language.rst`（本脚本不读取此文件，仅作为外部参考理解语法）

---

## Key Technical Decisions

- **单脚本实现* 所有四个输出组件（目录树、Kconfig、Makefile、README）由一Python 脚本 `tools/docs/gen-project-md.py` 生成，避免多脚本协调成本- **Kconfig 解析采用正则表达式而非完整词法分析器* Kconfig 语法简单（`config`/`menuconfig`/`source`/`comment`/`endmenu`/`endif`），正则表达式足以提取配置项名称和描述，且保持脚本轻量- **目录树深度限制为 2 层* 递归扫描全部 ~6,143 个目录会产生 MB 级输出，违背"KB 成功标准。v1 扫描顶层 + 关键子目录（`arch/`、`drivers/`、`fs/`、`kernel/`、`mm/`、`net/`、`include/`、`lib/`、`scripts/`、`tools/`、`security/`、`crypto/`、`sound/`、`virt/`、`io_uring/`、`ipc/`、`samples/`、`rust/`），每个子目录下仅列出第一级子目录- **Makefile 目标提取兼顾顶层 Makefile Documentation/Makefile* 顶层 Makefile 有构建目标，Documentation/Makefile 有文档目标，两者互补- **README 提取聚焦关键子目录* 仅在 `arch/<arch>/`、`drivers/<driver>/` 等关键子目录下搜`README*` 文件，不全树扫描- **输出确定性* 所有目录遍历使`sorted()` 排序，确保多次运行输出一致（满足 R7）
---

## Open Questions

### Resolved During Planning

- **输出格式* 选定单文`docs/PROJECT.md`，而非多文件- **目录树深度：** 限制2 层（顶层 + 关键子目录的第一级），保持输KB 级- **Kconfig 解析策略* 正则表达式提`config`/`menuconfig` 条目名称和帮助文本，不追求完整词法分析
### Deferred to Implementation

- 部分子目录（`drivers/` 下有数百个驱动子目录）的用途说明可能需要人工标注或`Kconfig` 中推断，实现时需确认推断准确度- Makefile 目标说明文本的来源——有些目标有注释，有些没有，需要实现时决定是否从目标名推断或留空
---

## Implementation Units

### U1. 创建项目结构扫描
**Goal:** 实现目录树扫描逻辑，递归遍历项目顶层和关键子目录，提取每个目录的用途说明
**Requirements:** R1, R3, R7, R8

**Dependencies:** 鏃。
**Files:**
- Create: `tools/docs/gen-project-md.py`

**Approach:**
- 定义 KEY_DIRS 列表，包含需要扫描的关键子目录（`arch/`、`drivers/`、`fs/`、`kernel/`、`mm/`、`net/`、`include/`、`lib/`、`scripts/`、`tools/` 等）- 对每个关键子目录，递归扫描其第一级子目录（深度限制为 2）- 用途说明提取优先级：README 文件第一> Makefile 顶部的注释块 > 目录名推断- 输出Python 数据结构（嵌套字典），供后续单元组合Markdown- 所有目录遍历使`sorted()` 确保确定性输出
**Patterns to follow:**
- `tools/docs/kernel-doc` 的模块化设计（扫描器 + 输出器分离）- `tools/docs/md-convert.py` 的标准库优先、无第三方依赖风格
**Test scenarios:**
- Happy path: 扫描关键子目录，正确提取目录名和用途说明- README: 目录下无 README 时，回退Makefile 注释或目录名- 深度限制: `drivers/` 下仅列出第一级子目录（如 `drivers/net/`、`drivers/block/`），不递归`drivers/net/wireless/`- 确定 连续运行两次，目录树顺序完全一致- 非源码修 脚本运行期间不修改任何文件
**Verification:**
- 生成的目录树数据结构包含所KEY_DIRS，每个目录有名称和用途说明字段- 输出顺序确定性验证通过
---

### U2. 实现 Kconfig 配置项摘要提取器

**Goal:** 解析`Kconfig` 和关键子系统 Kconfig 文件，提取主要配置项（名称、类型、帮助文本），生Markdown 表格
**Requirements:** R1, R4, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/docs/gen-project-md.py`（同一脚本的新模块
**Approach:**
- 正则表达式匹Kconfig 语法  - `config <NAME>` / `menuconfig <NAME>` 配置项入  - `bool/string/int/hex` 类型
  - `help` / `---help---` 帮助文本开始标  - `source "<path>"` 递归解析引用Kconfig 文件
  - `comment` / `menu` / `endmenu` / `endif` 结构标记，跳- 提取配置项名称、类型、帮助文本（截断至合理长度）- **按子系统分组* 根据 `source "<path>"` 中的路径推断子系统归属。映射规则为：`arch/` 架构、`drivers/` 驱动、`fs/` 文件系统、`net/` 网络、`security/` 安全、`crypto/` 加密、`sound/` 声音、`lib/` 库、`kernel/` 内核核心、`mm/` 内存管理、`fs/` 文件系统、`Documentation/` 文档。路径前缀不匹配时归入 "Other"- 生成 Markdown 表格：`| 配置| 类型 | 说明 |`
**Patterns to follow:**
- `tools/lib/python/kdoc/kdoc_re.py` `KernRe` 正则封装模式
**Test scenarios:**
- Happy path: 解析Kconfig source 引用，正确提10+ 个配置项- help 文本提取: 多行 help 文本被正确拼接为单行摘要- 嵌套结构: `menu`/`endmenu` 内的配置项被正确提取，不被跳过- source 递归: 解析 `fs/Kconfig` 引用的文件系统配置项- 确定 相同输入产生相同输出顺序
**Verification:**
- Kconfig 提取结果包含至少 5 个主要类别的配置项（架构、驱动、文件系统、网络、安全）- Markdown 表格格式正确，无语法错误
---

### U3. 实现 Makefile 目标提取
**Goal:** 解析顶层 `Makefile` `Documentation/Makefile`，提取主要构建目标及其说明
**Requirements:** R1, R5, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/docs/gen-project-md.py`（同一脚本的新模块
**Approach:**
- 正则表达式匹Makefile 目标定义  - `^<target>:` 标准目标（排除以 `#` 开头的注释行）
  - `^PHONY += <target>` PHONY 目标
  - `^# <comment>` 紧邻目标上方的注释作为说  - Pattern rules（如 `%.o: %.c`）v1 跳过，不纳入 PROJECT.md
- 从顶Makefile 提取常见目标：`all`、`modules`、`clean`、`mrproper`、`distclean`、`help`、`defconfig`、`menuconfig`、`O=` 相关说明等- `Documentation/Makefile` 提取文档目标：`htmldocs`、`pdfdocs`、`epubdocs`、`markdowndocs` 等- 目标说明提取优先级：目标上方注释 > 从目标名推断 > 空- 按目标类别分组（构建目标、配置目标、清理目标、文档目标）
**Patterns to follow:**
- `tools/docs/kernel-doc` 的过滤模式（仅提取需要的符号）
**Test scenarios:**
- Happy path: 提取顶层 Makefile 10+ 个主要目标- 注释提取: 目标上方 3 行内的注释被正确捕获作为说明- PHONY 目标: `PHONY +=` 声明的目标被正确识别- 文档 Makefile: `Documentation/Makefile` 中的 `htmldocs`、`pdfdocs` 等目标被提取- 确定 相同输入产生相同输出顺序
**Verification:**
- 提取的目标列表包含至10 个常用目标及其说明- Markdown 列表格式正确
---

### U4. 实现 README/头文件注释提取器

**Goal:** 扫描关键子目录下README 文件和核心头文件开头的注释块，提取子系统描述
**Requirements:** R1, R6, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/docs/gen-project-md.py`（同一脚本的新模块
**Approach:**
- KEY_DIRS 中的每个关键子目录下搜索 `README*` 文件（`README`、`README.md`、`README.rst` 等）- 对找到的 README 文件，提取前 10 行或第一个空行前的文本作为摘要- 对没README 的关键子目录，扫描其核心头文件，提取文件开头注释块（`/*` `*/`）作为子系统描述。各子系统对应的核心头文件映射如下：
  - `kernel/` `include/linux/sched.h`、`kernel/sched/sched.h`
  - `mm/` `include/linux/mm.h`、`mm/mmap.c`（文件头注释  - `fs/` `include/linux/fs.h`
  - `net/` `include/linux/net.h`、`net/core/skbuff.h`
  - `arch/` 各架构的 `include/asm/entry-common.h` `arch/<arch>/kernel/` 下的核心文件头注  - `drivers/` 无统一头文件，回退到目录名 + `drivers/base/` 下的 `base.h`
  - `lib/` `include/linux/bitops.h`、`lib/radix-tree.c`（文件头注释  - `include/` `include/linux/printk.h`（文件头注释- 如果两者都不存在，使用目录+ 已知项目知识生成简要说明- 按子系统分组输出Markdown 列表或简短段落
**Patterns to follow:**
- `tools/docs/kernel-doc` 的注释提取模式（识别 `/**` 块）
**Test scenarios:**
- Happy path: `arch/x86/` 下的 README 被正确提取和摘要- README: `kernel/` 下无 README 时，从核心头文件提取注释- 头文件注 `include/linux/sched.h` 开头的注释块被正确提取- 回退: 既无 README 也无头文件注释时，生成基于目录名的简要说明- 数量: 至少提取 5 个子系统的描述
**Verification:**
- 至少 5 个子系统的描述被提取并包含在输出中- 提取的文本长度合理（不超500 字符/子系统）
---

### U5. 组装 Markdown 输出并创建生成入
**Goal:** 将四个提取器的输出组装为完整`docs/PROJECT.md`，并提供命令行入口
**Requirements:** R2, R7, R8

**Dependencies:** U1, U2, U3, U4

**Files:**
- Create: `tools/docs/gen-project-md.py`（组装和 CLI 入口- Create: `docs/PROJECT.md`（生成的目标文件
**Approach:**
- 定义 Markdown 模板结构  ```
  # Linux Kernel Project Overview
  
  ## Directory Structure
  
  ## Kconfig Summary
  
  ## Makefile Targets
  
  ## Subsystem Descriptions
  ```
- 组装四个提取器的输出，按模板结构排列- 添加 `if __name__ == "__main__"` 入口，支持命令行参数  - `--output` 指定输出文件路径（默`docs/PROJECT.md`  - `--srcdir` 指定源码根目录（默认当前目录- 脚本启动时验证源码根目录存在，若不存在则报错退出- 脚本运行后打印生成摘要（各板块条目数）
**Patterns to follow:**
- `tools/docs/kernel-doc` CLI 入口模式（argparse + main）- `tools/docs/md-convert.py` `if __name__ == "__main__"` 模式
**Test scenarios:**
- Happy path: 脚本在源码根目录运行，成功生`docs/PROJECT.md`- 自定义输 `--output /tmp/test.md` 将输出写入指定路径- 自定义源码目 `--srcdree /path/to/linux` 从指定目录读取源码- 可重复 连续运行两次，输出文件字节级一致- 非源码修 脚本运行期间不修改任何文件- 文件大小: 输出文件大小在合理范围内（KB 级，MB 级）
**Verification:**
- `docs/PROJECT.md` 成功生成，包含四个主要板块- 文件大小10KB - 500KB 之间- 连续运行两次，输出完全一致（`diff` 无差异）
---

## System-Wide Impact

- **调用链：** `python3 tools/docs/gen-project-md.py` 独立脚本，不调用 Sphinx、不调用 kernel-doc、不修改任何构建流程- **错误传播* 脚本遇到无法解析的文件时打印警告并跳过，不终止（`--strict` 模式可选，v1 默认宽松）- **状态生命周期：** `docs/PROJECT.md` 是静态产物，不参与内核构建流程- **API 表面* 新增一个独立脚本和一个输出文件，不修改任何现API、接口或行为- **不变约定* `Documentation/sphinx/min_requirements.txt`、`Documentation/conf.py`、`tools/docs/sphinx-build-wrapper`、`tools/docs/kernel-doc` 均不变
---

## Risks & Dependencies

| Risk | 可能| 影响 | 缓解措施 |
|---|---|---|---|
| 关键子目录无 README/注释，用途说明推断不| | | 使用目录名回退；人工审核后可微调脚本的目录名映射表 |
| Kconfig 语法复杂，正则解析漏提取 | | | v1 只提`config`/`menuconfig` 条目，跳`choice`/`comment` 等复杂结构；覆盖 80% 常用配置|
| Makefile 目标分散，提取不| | | v1 聚焦顶层 Makefile Documentation/Makefile；子目录 Makefile 目标 deferred |
| 输出文件过大（MB 级） | | | 目录树深度限制为 2 层；README 摘要截断500 字符/子系|
| 运行性能（全库扫描慢| | | 仅扫描关键子目录 + 第一级子目录，不递归全库 |

---

## Documentation / Operational Notes

- 运行方式：`python3 tools/docs/gen-project-md.py`，输出到 `docs/PROJECT.md`- 脚本无第三方依赖，仅使用 Python 标准库- `docs/PROJECT.md` 应加`.gitignore` 吗？不——它是从源码生成的产物，但代表项目状态快照，适合纳入版本控制（类`Documentation/output/` 的哲学但作为文档而非构建产物）。建议纳入版本控制，每次脚本更新时重新生成
---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md](docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md)
- **相关代码* `tools/docs/kernel-doc`、`tools/docs/md-convert.py`、`tools/lib/python/kdoc/kdoc_output.py`、`Kconfig`、`Makefile`
