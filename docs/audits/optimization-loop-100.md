<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# 100 轮审计优化循环账本（2026-09-27）

> 起点：`b1546fdcf`（501 桩全仓清零之后）。每轮 = 真实扫描 → 处置 → 验证 → 记账。
> 节奏：轻量检查为主；每 10 轮中型门禁（biome/build）；第 50/100 轮完整门禁
> （clippy/build/vitest/tsc/parity/reconcile）。结果三类：**FIX**（发现并修复）、
> **CLEAN**（扫描通过零发现）、**IMPROVE**（不改行为的质量提升）。

## 状态：进行中

| 轮 | 维度 | 命令/手段 | 结果 | 处置 |
|---|---|---|---|---|
| 1 | Rust 编译回归 | `cargo check --workspace` | CLEAN | 零 error/warning |
| 2 | 格式漂移 | `cargo fmt --check` | FIX | 32 文件 rustfmt 标准化（commit 04a2f0e 段） |
| 3 | 依赖 advisory | `cargo deny check` | SKIP(env) | 本机 git 传输故障无法拉库 |
| 4 | RustSec | `cargo audit` | SKIP(env) | 同上（网络受限） |
| 5 | unwrap/expect 面 | grep + 抽样 10 处人工核 | CLEAN | 均为 row.get/Option 惯用法，非请求路径 panic |
| 6 | panic!/unreachable! | grep 逐条核 | IMPROVE | oauth 2/3 处改 BadRequest；parity/u2_render 为测试；storage 启动 fail-fast 设计保留 |
| 7 | async 阻塞 IO | std::fs/Command 扫描 | FIX | preview LibreOfficePreview 改 tokio::fs + tokio::process |
| 8 | N+1 循环查询 | for 内 query 扫描 | CLEAN | 零命中 |
| 9 | 无界全表读 | SELECT 无 LIMIT | CLEAN | 116 条均为配置小表合理语义（前轮已裁定） |
| 10 | 无 WHERE UPDATE/DELETE | 合并字符串字面量复扫 | CLEAN | 仅 4 条 o2 既有全表刷新语义（touch/x_query） |
| 11 | SQL 注入面 | format! 值位拼接扫描 | FIX | cms 2 处 Path 参数 `id='{}'` → $1 参数化；attendance 表名为字面量白名单 |
| 12 | 语义回归 | 受影响 crate 全测 | CLEAN | cms 331/331 attendance 41/41 auth 93/93 preview 4/4 |
| 13 | 语义回归 | cms/attendance 全测 | CLEAN | cms 331/331 attendance 41/41 |
| 14 | TODO/FIXME | grep 复扫 | CLEAN | 0 基线维持 |
| 15 | 敏感硬编码 | password/secret/token 模式 | CLEAN | 零命中（.env 外置纪律有效） |
| 16 | SPDX 头覆盖 | 生成物扫描 | FIX | 5 缺失（3 生成器 + parity lib/behavior）→ 生成器补头重新生成，缺失清零 |
| 17 | console 残留 | grep | CLEAN | 3 处均为 PWA 提示/设计器字面量，有用途 |
| 18 | XSS 面 | v-html/innerHTML 扫描 | CLEAN | 零命中 |
| 19 | localStorage 敏感 | token/password 模式 | CLEAN | 零命中 |
| 20 | biome 门禁 | `pnpm lint` | FIX | **根除 autocrlf 门禁炸弹**：.gitattributes 强制 15 类文本 LF + 166 文件本地转 LF + biome format/check --write 全仓（48 文件格式化）+ mobile meeting.vue implicit any 1 处类型修复；234 文件 0 error |
| 21 | npm advisory | `pnpm audit --prod` | SKIP(documented) | file-type GHSA-5v7r-6r5c-r473 moderate = 既有 TD-2（ESM 不兼容无法升级，已记档） |
| 22 | tsc | `pnpm typecheck` | CLEAN | desktop+mobile 0 错误 |
| 23 | vitest desktop | `pnpm test` | CLEAN | 953/953 |
| 24 | vitest mobile | `pnpm test:mobile` | CLEAN | 101/101 |
| 25 | desktop build | `pnpm build` | CLEAN | 3026 modules, 3.31s |
| 26 | bundle 体积 | assets 总量 | CLEAN | 2.16MB / 96 分片（与基线持平） |
| 27 | reconcile gate | `compare.py --gate` | CLEAN | shadow/405/404 = 0 PASS |
| 28 | 契约守卫 | contracts + autoquery vitest | CLEAN | 20/20 |
| 29 | 仓库垃圾 | untracked 盘点 | CLEAN | oa4rust 范围内无残留（augmentor/laiyipao 未跟踪文件属并行工作线，不越界处置） |
