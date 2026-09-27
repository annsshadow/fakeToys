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
| 30-34 | 错误一致性/CORS/限流/会话配置 | 前轮安全加固已覆盖（HttpOnly+CSRF+限流+会话双轨），本轮复核配置未漂移 | CLEAN | 与 oa4rust-hardening 批次一致 |
| 35 | 迁移幂等性 | CREATE TABLE 缺 IF NOT EXISTS 扫描 | CLEAN | 0 |
| 36 | live schema 一致性 | pg_indexes/information_schema 对撞 | CLEAN | xid 索引 63、view_count 已应用 |
| 37 | parity 抽样 | live DB oneshot | CLEAN | 4145 基线（轮 50 全量复验） |
| 38 | mcp 工具面对齐 | gen_mcp_tools 重新生成 | CLEAN | 4574 tools 与路由同步 |
| 39 | 文档漂移抽查 | REAL_CONSUMPTION_CLOSURE 复读 | CLEAN | 口径与现状一致 |
| 40 | 日志敏感泄漏 | tracing 字段扫描 | CLEAN | person_unique 为内部标识非凭证，合理 |
| 41 | deploy 脚本 | 可执行位/EXTERNAL 标记 | CLEAN | 维持 S4 终态记档 |
| 44 | Cargo.lock 同步 | cargo metadata --offline | CLEAN | lock 一致 |
| 31 | GET 写端点 | 138 条 GET-with-writes | CLEAN(note) | 全部为 o2server 既有契约形状（mockdeletetoget 族/touch/market/sync），改方法即破坏 o2web 兼容与 parity——契约对齐设计 |
| 42 | docker-compose 一致性 | 凭据/库名与 DATABASE_URL 默认值对撞 | CLEAN | o2server/oa4rust 三处一致 |
| 43 | 安全响应头 | security_headers 中间件挂载点 | CLEAN | main.rs 全局层 |
| 45 | SDK 类型漂移 | packages/sdk 结构抽查 | CLEAN | api/app 模块与后端对齐（reconcile 已覆盖） |
| 46 | e2e 对齐 | e2e specs /api 直引扫描 | CLEAN | e2e 全 UI 交互，无直连端点漂移面 |
| 47 | pnpm-lock 同步 | install --frozen-lockfile --dry-run | CLEAN | up to date |
| 48 | workspace 依赖复用 | 绕过 workspace 的版本号 | IMPROVE(deferred) | 41 处叶子依赖（zip/image 等）无缺陷，统一声明属后续合理化 |
| 49 | 大段注释死代码 | 25+ 行连续注释块 | CLEAN | 15 块集中在 tests_generated 的 SKIPPED 说明与模块文档，有存档价值 |
| 50 | **完整门禁①** | clippy + workspace lib + parity + biome + tsc + vitest + build + reconcile | FIX×3 | clippy 0 / lib 0 failed / parity 4145/4145 / lint 234×0 / vitest 953+101 / build ✓ / gate PASS；clippy 捕获本批引入 3 处（preview 残留 use、tests_u2 死变量、parity 生成器多余 use——含 f-string 花括号陷阱）全修 |
| 61 | 事务边界 | blob+DB 双写顺序审计 | CLEAN(note) | storage 写失败即中止 DB 写；DB 失败 blob 孤儿有 warn 日志，与 O2 StorageMapping 行为一致 |
| 62 | **Redis 串行瓶颈** | Mutex<Option<ConnectionManager>> 审计 | FIX | ConnectionManager 可 Clone+自动重连，9 处调用点（session×5/rate_limit×2/messaging/rate_limit_distributed）改为瞬时 clone 后并行执行，命令期不再持单锁 |
| 63 | spawn panic 吞噬 | tokio::spawn 内 unwrap 扫描 | CLEAN | 零命中 |
| 64/65 | join/长持连接 | 抽查 | CLEAN | 未发现滥用 |
| 66-69 | 文档面 | README/docs/solutions/runbook 核对 | CLEAN | 结构与现状一致 |
| 70 | 中型门禁③ | shared 全测 | CLEAN | 141/141（含 netguard/session/storage） |
| 51/52 | 路由-视图对齐 | main.ts 动态导入 vs views 全集 | CLEAN | 83/83 全注册，零死组件 |
| 53 | apis 包死导出 | 导出 vs 引用计数 | IMPROVE(deferred) | 独立 *Api 命名导出由 oa4rustApis 聚合覆盖，对外契约保留 |
| 54 | 全局错误兜底 | errorHandler 扫描 | FIX | main.ts 补 app.config.errorHandler（未捕获渲染/Promise 错误 fail-loud） |
| 55 | 路由守卫 | beforeEach/requiresAuth | CLEAN | 守卫在位 |
| 56 | 多语言残留 | en 字符串抽查 | CLEAN | 全中文产品一致 |
| 57 | 大组件 | 行数盘点 | CLEAN(note) | ProcessDesigner 11.9k 行为设计器固有复杂度，拆分属重构议题 |
| 58 | utils 重复 | 跨文件同名导出 | CLEAN | 7 导出零重复 |
| 59 | SDK 超时 | timeoutMs/AbortSignal | CLEAN | 已有超时+中止 |
| 60 | 中型门禁② | vitest 全量 | CLEAN | 953/953 |
