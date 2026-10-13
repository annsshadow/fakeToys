---
title: "assemble/control 管理写族 handler 内门禁模式（越权收口）"
date: 2026-10-13
last_refreshed: 2026-10-13
category: security-issues
module: oa4rust/crates/*_assemble_control
problem_type: security_issue
component: authorization
severity: high
symptoms:
  - "assemble/control 族写端点中间件兜底仅 Authenticated，任一登录用户可改全局配置/组织结构/门户内容"
  - "属主资源（invoice/qrcode/mind/calendar 等）update/delete 无 require_owner，可按任意 id 越权"
  - "create 的 creator 取请求体，可冒名归属"
root_cause: missing_permission
resolution_type: code_fix
tags: [authorization, admin-gate, require-owner, assemble-control, oa4rust, round2]
related_components:
  - oa4rust/backend/crates/shared/src/middleware/rbac.rs
  - oa4rust/backend/docs/audits/2026-10-13-security-audit-round2.md
  - docs/solutions/security-issues/idor-vulnerability-write-handlers.md
---

# assemble/control 管理写族 handler 内门禁模式

## 问题

`*_assemble_control` / `*_core_entity` 族 crate 的大量写端点，RBAC 中间件只兜底
`Authenticated`（`ADMIN_WRITE_PREFIXES` 仅 person/unit/role/group/organization 6 条，
且 `/api/portal`、`/api/program_center` 等已在 `ADMIN_WRITE_DEPRECATED_PREFIXES` 不再
强制 admin）。结果：任一登录用户即可改系统配置、增删组织结构、删他人属主资源。

## 解法（handler 内门禁，三选一）

1. **系统数据/无属主列 → `require_admin`**（crate-local helper，先例
   `meeting_assemble_control::u2_require_admin`、`program_center` 5fe82f1e7）：
   ```rust
   async fn require_admin(pool: &Pool, session: &shared::session::Session) -> Result<(), AppError> {
       if shared::middleware::is_admin(pool, &session.person_unique).await { Ok(()) }
       else { Err(AppError::Forbidden) }
   }
   ```
   handler 加 `session: Extension<shared::session::Session>`（非豁免路径会话必注入），
   函数体首行 `require_admin(&pool, &session).await?;`。
2. **属主资源（有 creator/createor 列）→ `require_owner`**（`shared::middleware`）：
   先查记录属主列，命中行才校验；行不存在则放行（后续 UPDATE/DELETE 命中 0 行，
   统一回 not-found，语义不变）。`create` 的属主列一律改取 `session.person_unique`，
   禁止取请求体（冒名归属面）。
3. **豁免前缀下的写（读须保持匿名）→ `Option<Extension<Session>>` + `require_admin_opt`**
   （未认证 401 / 非 admin 403，先例 program_center appstyle erase、本轮
   `meeting config_system_config_manage`）。

## 回归测试约定

- 无 DB 时 `is_admin` fail-closed 返回 false → 非 admin 会话 403 可**不依赖数据库**
  断言（`shared::testing::test_pool()` + `shared::testing::test_session(...)`）。
- 属主门（需查库取 creator）用 DB 门控（`is_db_available()`），种真实行断言
  非属主 403 / 属主放行，测试尾清理。
- 统一放各 crate `security_gate_tests.rs` 模块。

## 裁决边界（避免误升级）

- 维护作业/用户自服务（o2server 对齐、任意登录用户语义）只加**会话门**，不升级
  admin（先例 processplatform 7 个维护作业，一轮审计 V2）。
- ownerless 系统配置物理删维持 `require_admin` + 拒绝/幂等（idor 档案 2026-08-25
  裁决）；读端不改。
- 前端消费者若仅管理台调用（如 CommonApp），admin 门禁不破坏真实流程；OrgViewer
  等 autoquery 探针（id=0/空体）非功能消费。

## 已套用的 crate（2026-10-13 二轮）

meeting_assemble_control（S1）、general / organization（S2/S3）、
meeting_core_entity（S4）、mind / message / hotpic / calendar / jpush /
portal_assemble_surface / component_assemble_control / process_designer（S5）。
完整取证见 `oa4rust/backend/docs/audits/2026-10-13-security-audit-round2.md`。
