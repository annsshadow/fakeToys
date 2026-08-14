---
date: 2026-07-06
topic: project-overview-markdown
---

# Project Overview Markdown Generation

## Summary

从 Linux 内核源码直接生成一份综合 Markdown 项目总览文档 `docs/PROJECT.md`，内容涵盖目录树、Kconfig 配置项摘要、Makefile 目标列表、各子系统 README/头文件注释。不依赖 `Documentation/` 下已有的 `.rst` 文件。

---

## Problem Frame

Linux 内核项目规模庞大（~36,681 个 C 文件、~26,666 个头文件、42.6M 行代码）， newcomers 或下游消费者难以快速建立对项目结构的整体认知。`Documentation/` 下的文档虽然详尽，但以 reStructuredText 为主，且分散在数十个子目录中，不便快速浏览。一份从源码直接生成的 Markdown 总览文档能降低入门门槛，支持 LLM ingestion、静态站点生成和跨项目镜像。

---

## Requirements

- R1. 必须从源码直接提取信息，不依赖 `Documentation/` 下已有的 `.rst` 文件作为信息来源。
- R2. 输出文件为 `docs/PROJECT.md`（Markdown 格式）。
- R3. 文档必须包含目录树：项目顶层和关键子目录（`arch/`、`drivers/`、`fs/`、`kernel/`、`mm/`、`net/`、`include/`、`lib/`、`scripts/`、`tools/` 等）的递归结构，每个目录附带用途说明（从 README、Makefile 注释、或目录名推断）。
- R4. 文档必须包含 Kconfig 配置项摘要：从根 `Kconfig` 和关键子系统 Kconfig 中提取主要配置选项（架构、驱动、文件系统、网络等），以 Markdown 表格或列表形式呈现。
- R5. 文档必须包含 Makefile 目标列表：从顶层 `Makefile` 和 `Documentation/Makefile` 中提取主要构建目标（`all`、`modules`、`clean`、`htmldocs`、`markdowndocs` 等），附带简短说明。
- R6. 文档必须包含各子系统的 README/头文件注释摘要：扫描每个关键子目录下的 `README*` 文件和核心头文件开头的注释块，提取子系统描述。
- R7. 生成过程必须是可重复的：重新运行生成脚本应产生一致的输出。
- R8. 生成脚本必须不修改任何源码文件，仅读取。

---

## Success Criteria

- `docs/PROJECT.md` 生成成功，文件大小合理（KB 级，非 MB 级）。
- 目录树覆盖项目关键子目录，每个目录有用途说明。
- Kconfig 摘要覆盖主要配置类别（架构、驱动、文件系统、网络、安全等）。
- Makefile 目标列表包含至少 10 个常用目标及其说明。
- 至少 5 个子系统的 README/头文件注释被提取并总结。
- 生成脚本可在干净源码树上独立运行，不依赖外部网络或数据库。

---

## Scope Boundaries

- 不提取 kernel-doc 注释（`/** ... */`）到 PROJECT.md；kernel-doc 的输出留给现有工具链处理。
- 不生成 API 级别的详细函数/结构体文档；PROJECT.md 是项目总览，不是 API 参考。
- 不修改任何源码或 `Documentation/` 下的文件。
- 不替代 `Documentation/` 的现有文档体系；PROJECT.md 是入口级总览，不是完整文档。

---

## Key Decisions

- **输出定位：项目总览而非 API 参考。** 源码级 API 文档留给 `tools/docs/kernel-doc` 和 `make markdowndocs`。PROJECT.md 的目标是让读者在 5 分钟内理解项目结构。
- **信息来源：源码文件头注释 + Kconfig + Makefile。** 不依赖 `Documentation/` 的 `.rst` 文件，确保信息来源独立。
- **输出位置：`docs/PROJECT.md`。** 与现有的 `docs/brainstorms/` 和 `docs/plans/` 平级，作为 workspace 级别的项目总览。

---

## Dependencies / Assumptions

- 项目关键子目录（`arch/`、`drivers/`、`fs/`、`kernel/`、`mm/`、`net/`、`include/`、`lib/`、`scripts/`、`tools/`）下存在 `README*` 文件或具有自解释性的 Makefile 注释。
- 根 `Kconfig` 和主要子系统 Kconfig 中包含可解析的 `config` / `menuconfig` 条目。
- 顶层 `Makefile` 包含可识别的目标定义（`xxx:` 模式）。
