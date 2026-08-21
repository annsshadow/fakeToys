---
title: feat: oa4rust 与 Linux 文档剩余工作汇总计划
type: feat
status: active
date: 2026-08-21
origin: docs/brainstorms/2026-08-21-plans-status-audit-and-consolidation-requirements.md
---

# OA4Rust 与 Linux 文档剩余工作汇总计划

## Summary

承接 2026-08-21 全面状态审计的结论，将 18 份历史计划中全部未实现功能单元、未完成 Deferred 项与 `docs/todo.md` 未完成项去重后收敛为 11 个可执行实施单元，按优先级分组；每个条目注明来源计划与原始单元编号，作为后续工作的唯一真相源。

---

## Problem Frame

审计发现各计划的遗留工作散落在 18 份文档的 Deferred 小节与未竟单元中，且存在大量跨计划重复条目（如 SQLx 移除出现在 5 份计划、Office 预览出现在 3 份）。没有统一视图时，后续规划要么重复审计要么遗漏。本计划是 2026-08-21 审计（见各计划"实现情况"小节）的直接产物。

---

## Requirements

- R1. 完整收录三批审计发现的全部未完成/unverified 条目，无遗漏（对应 origin R6/R7）
- R2. 每个条目注明来源计划路径与原始单元编号（origin R7）
- R3. 收录 `docs/todo.md` 的 4 个 Linux 文档待办项（origin R8）
- R4. 跨计划重复条目去重合并，保留全部来源引用
- R5. 本计划成为剩余工作唯一真相源；`docs/todo.md` 同步清空（origin R9）

---

## 图例与口径

- **P0** 测试基线/核心对齐缺口 —— 阻塞"可替代"判定
- **P1** 高价值功能或生产切换 —— 显著推进替代目标
- **P2** 增强/运维完善
- **P3** 低频/待核验小项
- 状态基线：工作树 HEAD 314c7a75（2026-08-21 审计）

## 剩余工作总览

| # | 单元 | 优先级 | 类型 |
|---|------|--------|------|
| U1 | Value::Null 与 CMS stub 清零 | P0 | oa4rust |
| U2 | Java-Rust 端点对齐度 ≥70% | P0 | oa4rust |
| U3 | 影子流量灰度验证与切流 | P1 | oa4rust |
| U4 | Tantivy 全文检索集成 | P1 | oa4rust |
| U5 | query/portal 深度审计 | P1 | oa4rust |
| U6 | 存储后端三项（文件/BBS 附件/Office 格式） | P2 | oa4rust |
| U7 | 认证与消息增强三项 | P2 | oa4rust |
| U8 | 接口规范与依赖清理两项 | P2 | oa4rust |
| U9 | 测试体系增强四项 | P2 | oa4rust |
| U10 | 待核验小项五条 | P3 | oa4rust |
| U11 | Linux 文档精修四项 | P2 | linux-docs |
| U12 | 模块卡片文档深度填充 | P3 | oa-docs |

---

## Implementation Units

### U1. Value::Null 与 CMS stub 清零（P0）

**来源：** `docs/plans/2026-08-19-001-fix-close-all-blocking-gaps-plan.md` U2/U5；`docs/plans/2026-08-19-002-unified-completion-plan.md` U-A1/U-A3；`docs/plans/2026-08-13-003-oa4rust-completion-plan.md` Phase 1

**现状：** Value::Null 由 201 降至 15 处；CMS `Value::Bool(true)` 由 96 降至 17 处。两份前序计划的验收标准均为归零，未达成。

**Approach:** 沿用 Top-crate 分批模式清零剩余 15+17 处；无法真实化的改为显式 404/501 响应并记日志。

**Verification:** `rg -n "Value::Null" oa4rust/crates/` 与 `rg -n "Value::Bool\(true\)" oa4rust/crates/cms_assemble_control/src/` 双双零命中。

---

### U2. Java-Rust 端点对齐度提升至 ≥70%（P0）

**来源：** `docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md` U4（唯一未竟单元）；`docs/plans/2026-08-10-001-feat-oa4rust-gap-audit-plan.md` Deferred「流程平台深度功能」

**现状：** 最新提交实测约 36.6%（314c7a75），距 08-20-001 U4 验证标准 ≥70% 尚远。

**Approach:** 按 08-20-001 U4 既有方法：静态分析 Java `@Path` 注解 × Rust endpoints.rs 交叉比对，优先补齐认证安全、个人中心、流程平台核心模块缺失端点；新增端点纳入行为对比清单。

**Verification:** 对齐度统计 ≥70%；新增端点通过 `BEHAVIOR_COMPARE=1` 行为对比。

---

### U3. 影子流量灰度验证与切流执行（P1）

**来源：** `docs/plans/2026-08-13-003-oa4rust-completion-plan.md` U4.3；`docs/plans/2026-08-13-001-feat-oa4rust-seaorm-entity-parity.md` Remaining Work #6

**现状：** 灰度脚手架全部就绪（`deploy/nginx.conf`、`toggle_module.sh`、`shadow-traffic.sh`、`rollback-playbook.md`、`gray-release-playbook.md`），但影子流量观察与切流从未实际执行。

**Approach:** 按既有 playbook 执行模块级灰度 + 影子流量比对，观察期 ≥2 周无差异后切流。

**Verification:** 影子流量比对报告归档；切流后核心链路监控无回归。

---

### U4. Tantivy 全文检索集成（P1）

**来源：** `docs/plans/2026-08-19-002-unified-completion-plan.md` U-B1

**现状：** 全仓 0 处 tantivy 引用；search crate 仍为 PG `to_tsvector`（6 处）；前序因 crates.io 网络阻塞搁置。

**Approach:** 网络可达后引入 tantivy，替换 search 三端点（documents/subjects/messages）的检索实现，保留 PG to_tsvector 为 fallback。

**Verification:** `cargo test -p search --lib` 全绿；三端点返回 Tantivy 结果。

---

### U5. query/portal 抽样深度审计与浅层补齐（P1）

**来源：** `docs/plans/2026-08-13-003-oa4rust-completion-plan.md` U2.6

**现状：** 未逐项核验。query(67+59)、portal(56+48) handler 的真实度未经系统审计。

**Approach:** 抽样审计 statement/stat/view 核心 handler 的查库真实性，浅层者按 U1 模式补齐或显式降级。

**Verification:** 审计报告归档；核心 statement/stat/view 端点连库返回真实数据。

---

### U6. 存储后端三项（P2）

**来源：**
- 文件实际物理存储后端（文件系统/对象存储）：`docs/plans/2026-08-11-002-feat-oa4rust-core-modules-gap-closure-plan.md` Deferred
- BBS 图片附件完整文件存储：同上 + `docs/plans/2026-08-11-003-feat-oa4rust-api-gap-closure-plan.md` Deferred
- Office 预览 .xlsx/.pptx 支持：`docs/plans/2026-08-11-004-feat-oa4rust-remaining-gap-closure-plan.md` Deferred（.docx HTML 已落地）

**Approach:** 抽象存储 trait（本地盘/对象存储两实现）；BBS 附件与 Office 预览复用同一存储层。

**Verification:** 上传→存储→读取链路在两种后端下集成测试通过；xlsx/pptx 预览返回可用 HTML 或明确降级响应。

---

### U7. 认证与消息增强三项（P2）

**来源：**
- LDAP 用户自动同步与增量更新：`docs/plans/2026-08-11-002-...` Deferred（ldap crate 当前仅认证绑定）
- 微信模板消息异步发送队列：`docs/plans/2026-08-12-001-fix-oa4rust-final-gap-closure-plan.md` Deferred
- DB 池 multi-instance 分布式限流：`docs/plans/2026-08-10-002-fix-oa4rust-gap-closure-plan.md` Deferred

**Verification:** LDAP 目录变更增量同步测试；模板消息经队列异步发送；多实例部署限流一致。

---

### U8. 接口规范与依赖清理两项（P2）

**来源：**
- OpenAPI securitySchemes（JWT/OAuth 安全描述）：`docs/plans/2026-08-10-002-fix-...` Deferred（实测 openapi 源码 0 处）
- SQLx 完全替代（含 sea-orm 底层依赖）：`docs/plans/2026-08-10-001-...`、`2026-08-10-002-feat-...`、`2026-08-11-002-...`、`2026-08-11-004-...`、`2026-08-20-001-...` 五处 Deferred 合并（workspace 直接依赖已清零，余底层传递依赖）

**Verification:** `/openapi.json` 含 securitySchemes 定义；`cargo tree` 无 sqlx 节点。

---

### U9. 测试体系增强四项（P2）

**来源：**
- 行为对比测试 Java 侧服务容器化：`docs/plans/2026-08-20-001-...` Deferred
- handler 级测试覆盖率提升至 99% / 内部服务函数可测性改造：`docs/plans/2026-08-20-001-...` Deferred + `docs/plans/2026-08-13-001-feat-handler-test-coverage-99-plan.md` 后续建议（1,856 个未导出内部函数）
- 代码行覆盖率测量（grcov/tarpaulin）：`docs/plans/2026-08-12-002-...`、`2026-08-13-001-...` Deferred
- 并发/超时/数据完整性深度测试：`docs/plans/2026-08-12-002-...` Deferred

**Verification:** Java 容器化后 `BEHAVIOR_COMPARE=1` 全量对比可行；行覆盖率报告进 CI；深度测试场景入库。

---

### U10. 待核验小项五条（P3）

**来源与内容：**
- 批量操作端点（批量删除/导入）：`docs/plans/2026-08-10-001-...` Deferred（审计时未逐项核验）
- 程序中心分发组装接口：`docs/plans/2026-08-11-003-...` Deferred（未逐项核验）
- console 模块完整实现：`docs/plans/2026-08-10-002-feat-...` Deferred（仅 4 源文件，完整性未知）
- AI 模块 MCP/文件/索引端点细项：`docs/plans/2026-08-10-002-feat-...` Deferred（主体已落地，细项未核验）
- 递归导航 sub/nested/type 变体覆盖 + processplatform_assemble_bam 路由数差异（标称 27 vs 实测 5）：`docs/plans/2026-08-11-004-...`、`2026-08-11-002-...`

**Approach:** 先核验再决定补全或关闭；bam 差异需确认是否为度量口径问题。

**Verification:** 五条均有核验结论（补全 / 关闭 / 差异解释）。

---

### U11. Linux 文档精修四项（P2，独立域）

**来源：** `docs/todo.md`（2026-08-14 审计，提交 a5c82f3d），目标目录 `docs/linux-7.1.3/`

- L11.1 翻译文件 `?` 编码损坏校正：zh_CN 82 处、sp_SP 68、it_IT 67、zh_TW 33、pt_BR 7、ko_KR 6、ja_JP 3——逐文件对照 kernel.org 上游 RST 源确定正确中文标点；优先 zh_CN/zh_TW
- L11.2 非翻译目录残余 `?` 约 94 行疑似损坏人工复核（示例：`admin-guide/svga.md`、`power/freezing-of-tasks.md`、`scheduler/sched-deadline.md`、`cdrom/cdrom-standard.md`）
- L11.3 内部链接完整性审计：3943 个文件的 `[text](link)` 相对路径与外链可达性脚本化检查，产出断裂链接报告
- L11.4 格式与风格统一：toctree 指令的 Markdown 列表替代一致性、YAML frontmatter 使用一致性（CRLF/LF 由 git 自动转换，可接受）

**Verification:** L11.1/L11.2 按语言计数归零或复核完毕；L11.3 产出报告；L11.4 一致性清单过检。

---

### U12. 模块卡片文档深度填充（P3）

**来源：** `docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md` Deferred

**内容：**
- `docs/oa/modules/o2server/` 55 张模块卡片的 Key Flows、Dependencies 等字段深度填充（REST Endpoints 已于 62d67e1d 填充）
- `docs/oa/modules/o2web/` 组件卡片填充

**Verification:** 抽查卡片字段非空且与代码一致；`docs/oa/README.md` 链接可解析。

---

## Scope Boundaries

- 本计划只汇总与追踪，不重复承载各来源计划的完整上下文；执行时先读来源计划对应单元
- 不包含已完成工作的回溯描述（见各计划"实现情况（2026-08-21 审计）"小节）
- IM/XMPP/WebRTC 完整即时通讯协议不在范围内（沿用 08-19-002 约束）
- cms_assemble_control 单文件架构重构不在此列（08-19-002 定义的后续架构债）

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-21-plans-status-audit-and-consolidation-requirements.md](../brainstorms/2026-08-21-plans-status-audit-and-consolidation-requirements.md)
- **审计执行计划:** [docs/plans/2026-08-21-001-feat-plans-status-audit-plan.md](2026-08-21-001-feat-plans-status-audit-plan.md)
- 来源计划：`docs/plans/` 下 18 份（详见各条目标注）
- 相关经验：`docs/solutions/best-practices/single-source-of-truth-migration-status.md`
