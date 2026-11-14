<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- 归档自仓库根目录 MEMORY.md（2026-09-07 历史里程碑快照），2026-11-14 迁入 docs/ops/。 -->
<!-- 本页在 mkdocs.yml 的 exclude_docs 名单内，不发布到文档站。 -->

# 历史里程碑（2026-09-07）

> **归档说明**：原文件位于仓库根目录 `MEMORY.md`，只记录两条历史事实；它引用的原始文档
> 已在早期仓库清理中移除，链接失效，故此处保留事实、不再保留死链。
> 本页**不是项目状态入口**：当前各项目现量以 `docs/ops/current-state.md` 为准。

- **OA4Rust API 层优化**：删除 53 个重复属性 + 修复类型注解，零 TS 错误，前端 main bundle 1124KB→96KB（-91%），lazy load 85 views
- **OA4Rust 最终状态**：100% 路径覆盖率，TS 零错误，bundle 1124KB→96KB（-91%）

## 口径提醒

以上两条是 2026-09-07 的当时快照。其中「最终状态」「100% 路径覆盖率」是当时口径，
**不代表当前现量**；当前覆盖率、门禁读数与台账终态以 `docs/ops/current-state.md`
与各分册的实测记录为准。本页只增不改，后续里程碑另立新页，不覆写历史。
