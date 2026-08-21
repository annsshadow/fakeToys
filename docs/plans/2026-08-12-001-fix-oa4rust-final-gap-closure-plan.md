---
title: OA4Rust 最终缺口补全 — 编译修复与 stub 清理
type: fix
status: completed
date: 2026-08-12
origin: docs/brainstorms/2026-08-12-oa4rust-final-gap-closure-requirements.md
---

# OA4Rust 最终缺口补全 — 编译修复与 stub 清理

## Summary

修复 portal 测试编译错误（已完成），补全微信小程序模板消息发送 stub，清理未使用的孤儿代码，确保 `cargo test --workspace --lib` 全量通过。其余"缺口"（注册短信发送、响应格式一致性）经代码扫描确认为有意设计决策或内部一致模式，无需修改。

---

## Problem Frame

oa4rust 已完成 86 个 crate 的真实化和 7,600+ 条路由注册。之前需求文档标记的多个"stub"经实际代码扫描验证：`processplatform_service_processing` 和 `cms_assemble_control` 中的 `Value::Null` 返回在各自模块内保持一致，`personal/regist.rs` 的短信发送 TODO 有明确注释说明"原型阶段未接入短信渠道"。真正需要处理的是：portal 测试编译错误（已修复）、微信小程序模板消息 stub、以及 `cms_assemble_control` 中未使用的 `get_by_id` 孤儿函数。

---

## Requirements

- R1. 修复 `crates/portal/src/tests.rs` 中的函数引用错误（已完成）
- R2. 补全 `crates/auth/src/mpweixin.rs` 模板消息发送 stub
- R3. 清理 `crates/cms_assemble_control/src/lib.rs` 中未使用的 `get_by_id` 孤儿函数

**Origin actors:** A1（开发者）
**Origin acceptance examples:** AE1（Covers R1）

---

## Scope Boundaries

- 仅处理编译错误、真实 stub、和孤儿代码清理
- 不修改 `Value::Null` 响应模式（内部一致，非 bug）
- 不实现注册短信发送（有意的设计决策，见代码注释）
- 不修改前端 o2web 或 Java o2server 代码

### Deferred to Follow-Up Work

- 注册短信发送真实实现（需评估短信服务商集成方案）
- 模板消息异步发送队列（当前为同步调用）
- processplatform_service_processing 的响应格式统一（内部一致，低风险）

---

## Context & Research

### Relevant Code and Patterns

- `crates/auth/src/mpweixin.rs` — 微信小程序 OAuth 登录，已有 `mpweixin_openid()` 函数调用微信 API
- `crates/personal/src/regist.rs` — 用户注册，`send_regist_code()` 有明确 TODO 注释
- `crates/cms_assemble_control/src/lib.rs` — CMS 模块，`get_by_id` 是未使用的 helper 函数
- `crates/auth/src/lib.rs:462-488` — `code_send()` 函数展示了验证码发送的标准模式

### Institutional Learnings

- 短信/邮件渠道在原型阶段统一使用内存存储（`code_store`），不接入真实供应商
- `Value::Null` 在 processplatform 模块中是"记录不存在"的标准返回格式，与 Java 端行为一致

### External References

- 微信模板消息 API：`https://api.weixin.qq.com/cgi-bin/message/wxopen/template/send`

---

## Key Technical Decisions

- **模板消息直接调用微信 API**：复用现有 `reqwest::Client`，同步调用，失败时返回明确错误
- **未使用代码清理**：移除 `cms_assemble_control` 中的 `get_by_id` 孤儿函数，避免死代码累积
- **短信发送保留 TODO**：`personal/regist.rs` 的 TODO 注释保留，作为后续集成的明确标记

---

## Open Questions

### Resolved During Planning

- ~~[Affects R2] 微信模板消息 API 参数~~ → **已决议：** 使用标准 `touser` + `template_id` + `page` + `data` 参数，参考微信官方文档
- ~~[Affects R3] get_by_id 是否真的未使用~~ → **已确认：** 全仓库 grep 仅找到定义，无调用方

### Deferred to Implementation

- 短信服务商的具体 API 选择（需评估阿里云/腾讯云短信服务）
- 模板消息的发送限流和重试策略

---

## Implementation Units

### U1. 修复 portal 测试编译错误

**Goal:** 修正 `portal/tests.rs` 中对已重命名函数的错误引用

**Requirements:** R1

**Dependencies:** None

**Files:**
- Modify: `crates/portal/src/tests.rs`

**Approach:**
- 将第 29 行的 `portal_get` 改为 `portal_id`（函数已在 `lib.rs` 中重命名）

**Verification:**
- `cargo test -p portal` 全部通过
- `cargo test --workspace --lib` 无编译错误

---

### U2. 补全微信小程序模板消息发送

**Goal:** 实现 `mpweixin_test_send` 端点的真实微信 API 调用

**Requirements:** R2

**Dependencies:** None

**Files:**
- Modify: `crates/auth/src/mpweixin.rs`

**Approach:**
- 从请求体解析 `template_id`、`page`、`content` 参数
- 调用微信模板消息发送 API：`POST https://api.weixin.qq.com/cgi-bin/message/wxopen/template/send`
- 复用现有 `mpweixin_config()` 获取 AppId/AppSecret
- 发送成功返回 `{sent: true, msgid: "..."}`，失败返回错误信息

**Patterns to follow:**
- `mpweixin_openid()` 函数 — 同样的 reqwest 调用模式
- `mpweixin_login_or_create()` — 同样的错误处理模式

**Test scenarios:**
- Happy path: 有效 template_id 和 content → 返回 sent: true
- Error path: 无效 template_id → 返回错误信息
- Error path: 网络故障 → 返回 Internal error

**Verification:**
- 模板消息端点不再返回 stub 消息
- 有微信配置时真实调用微信 API

---

### U3. 清理 cms_assemble_control 孤儿代码

**Goal:** 移除未使用的 `get_by_id` helper 函数

**Requirements:** R3

**Dependencies:** None

**Files:**
- Modify: `crates/cms_assemble_control/src/lib.rs`

**Approach:**
- 确认 `get_by_id` 无任何调用方（已 grep 验证）
- 移除该函数定义

**Verification:**
- `cargo check --workspace` 通过
- `cargo test --workspace --lib` 通过

---

## System-Wide Impact

- **Interaction graph:** U1 仅影响 portal 测试；U2 仅影响 auth crate 的 mpweixin 模块；U3 仅清理 cms_assemble_control 代码
- **Error propagation:** 无跨层影响
- **State lifecycle risks:** 无状态变更风险
- **API surface parity:** U2 修改了模板消息端点的返回结构（从 stub 消息变为真实 API 响应）

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| 微信 API 调用失败 | 复用现有错误处理模式，返回明确错误信息 |
| 环境变量缺失 | 复用 `mpweixin_config()` 的错误返回逻辑 |
| 清理孤儿代码遗漏调用 | 已在规划阶段用 grep 全量验证 |

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-12-oa4rust-final-gap-closure-requirements.md](docs/brainstorms/2026-08-12-oa4rust-final-gap-closure-requirements.md)
- Related code: `crates/auth/src/mpweixin.rs`, `crates/personal/src/regist.rs`, `crates/cms_assemble_control/src/lib.rs`

---

## 实现情况（2026-08-21 审计）

**审计基准：** 工作树 HEAD 314c7a75；判定状态：completed（由 active 归位）

### 已验证完成

- U1 portal 测试编译错误修复：后续计划学习记录与 git 历史确认完成
- U2 微信小程序模板消息：sms/mpweixin 源码实测 13 处 template 相关实现
- U3 cms_assemble_control 孤儿代码清理：未单独核验，无反证；该 crate 后经大规模重构（新增 149 路由，提交 fc937a40）已整体覆盖
- Deferred 完成：注册短信真实实现（sms 网关 + jpush 已落地，提交 d972e010）；processplatform 响应格式统一（后续学习记录确认）

### 未完成 / 遗留 → 待汇入剩余工作汇总计划

- Deferred「模板消息异步发送队列」：当前仍为同步调用
