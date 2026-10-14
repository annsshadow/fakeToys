---
title: "RustSec 供应链门禁：pinned PR 扫描 + floating 日扫双轨与例外治理"
date: 2026-09-09
last_refreshed: 2026-10-14
category: security-issues
module: oa4rust/backend（Cargo workspace）
problem_type: supply_chain_governance
component: ci
severity: high
symptoms:
  - "cargo-deny 已配置但工具版本、toolchain、advisory DB 均未固定，扫描结果不可复现"
  - "cargo-audit 与 cargo-deny 双 advisory DB、双 ignore、双失败源，口径互相矛盾"
  - "本地基线命中 lopdf/rsa/tantivy-lru advisory 与 license gate 失败，靠 continue-on-error 粉饰"
  - "例外无 owner/期限，过期后无人发现"
root_cause: unpinned_toolchain_and_dual_gate
resolution_type: process_and_ci_design
tags: [cargo-audit, cargo-deny, rustsec, supply-chain, pinned-db, exception-governance, ci]
applies_when:
  - "为 Rust workspace 建立可复现的漏洞扫描 PR 门禁"
  - "处置无 patched 版本的 advisory（例外治理）"
  - "拆分 cargo-audit 与 cargo-deny 职责"
related_components:
  - oa4rust/backend/security/rustsec-db.rev
  - oa4rust/backend/security/rustsec-exceptions.toml
  - oa4rust/backend/.cargo/audit.toml
  - .github/workflows/oa4rust-supply-chain.yml
  - oa4rust/backend/deny.toml
---

# RustSec 供应链门禁：pinned PR 扫描 + floating 日扫双轨与例外治理

## Problem

供应链扫描要成为硬门禁，必须先回答三个问题：工具版本是什么、advisory DB 是什么、已知失败如何诚实处置。固定不住这三样，门禁要么不可复现，要么靠 `continue-on-error` 变绿——后者比没有门禁更糟，因为它看起来像好消息。

## Resolution

### 职责单一化

- **cargo-audit 独占 RustSec advisory**（vulnerability/unsound/yanked 硬失败）；`--deny unsound --deny yanked`，禁 `--stale`、`continue-on-error`。
- **cargo-deny 只做 bans/licenses/sources**，删除其 `[advisories]` 职责，避免双 DB 双 ignore。CI 只跑 `cargo deny --locked --workspace check bans licenses sources`。

### pinned + floating 双轨

| 轨道 | 触发 | DB | 语义 |
|------|------|----|----|
| PR/push gate | 修改 Cargo.lock 必触发 | checkout `security/rustsec-db.rev` 的 40 位 commit，`--no-fetch` | 可复现：今天红明天必红 |
| daily schedule | 每日 | floating（在线拉最新） | 新鲜度：发现新 advisory |

- cache key 必须含 DB SHA，恢复后验证 remote/HEAD/git object 完整性；**网络获取失败硬失败，不回退旧 cache 冒充最新**。
- 扫描日志打印 scanner 版本、DB HEAD、lockfile hash 三元组，事后可审计。
- 工具版本在线核验后固定（本仓：cargo-audit 0.22.2 / cargo-deny 0.20.2），不把"本机候选"误写成"远端最新"。

### 例外治理（fail loud）

`security/rustsec-exceptions.toml`，每条例外必填：`advisory / package / version / dependency_path / owner / tracking_url / created / expires / reason / compensating_control / approved_by`。

- **30 天到期硬失败**：CI 校验 expires，过期即红，迫使再分类（renew / 修复 / 下线）。
- 2026-10-09 实际再分类样例：`rsa 0.9.10`（RUSTSEC-2023-0071）因 sqlx-mysql 0.8.x 仍拉 rsa 0.9.x 且上游无 patched 版，确认仅存在于 dev-dependency MySQL 测试驱动（生产 OIDC/PDF 签名已迁离），续期 30 天至 2026-11-08。
- 无 patched 版时的正当路径：**缩小暴露面**（私钥签名迁 HSM/ring、把依赖打进 dev-dependencies）+ 补偿控制 + 限时例外，而不是永久 ignore。

### advisory 修复的取舍

- lopdf：解析/签名链升 0.42（patched）并补畸形/深嵌套 PDF 测试；生成链 genpdf 仍锁 0.26 时，论证"genpdf 无 load 入口、不可达"后建例外（RUSTSEC-2026-0187，到期 2026-10-28 强制再分类）。
- tantivy/lru：升级链不可控时，**整体替换实现**（PostgreSQL 全文检索）比强行 patch 兼容 lru 更干净。
- 每个依赖升级独立提交，lockfile diff 只含预期解析变化。

## Why This Matters

"门禁全绿"只有在「工具版本 × DB 版本 × lockfile」三元组可复现、已知失败有期限有 owner 时才有意义。双轨设计同时满足"PR 可复现"与"每日新鲜度"两个互相冲突的目标；30 天例外到期把"临时放行"变成"定期再决策"。

## When to Apply

- Rust workspace 的 CI 供应链门禁设计
- 任何"无 patched 版本"advisory 的处置决策
- monorepo 中多扫描器职责划分

## Related

- 计划：`docs/archived-plans/2026-09-09-001-refactor-httponly-cookie-cargo-audit-rustsec-db-biome-2-5-11-plan.md`（U6-U8、Completion Evidence、2026-10-14 定期复核）
- 能力清单/版本单一来源思想同 `../best-practices/test-hygiene-ast-triage-and-coverage-traps.md`（F10 版本号漂移）
