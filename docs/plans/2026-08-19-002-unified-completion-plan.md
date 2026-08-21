---
title: OA4Rust 全面收尾与生产就绪计划
type: plan
status: active
date: 2026-08-19
origin: docs/plans/2026-08-13-003-oa4rust-completion-plan.md
---

# OA4Rust 全面收尾与生产就绪计划

> 统一承接 `docs/plans/2026-08-13-003-oa4rust-completion-plan.md` 与 `docs/plans/2026-08-19-001-fix-close-all-blocking-gaps-plan.md`，将两份计划中未完成项、测试缺口、生产就绪项、o2server 缺失功能整合为可执行的新计划。

---

## 0. 目标与出关判定

**目标**：使 oa4rust 在测试通过率、功能完整度、生产可运维性三个维度达到可接管 oa/o2server 的程度。

**出关条件（DoD）**：
1. `cargo test --workspace --lib -- --test-threads=1` 通过率 ≥ 99%（仅保留极少数已知外部依赖导致的 skip）
2. `Value::Null` 归零；CMS 核心路径 stub 归零
3. 集成测试在 PostgreSQL + MySQL 双库下均可运行
4. 生产基础设施就绪：Redis 会话存储、Tantivy 全文检索、性能基线达标
5. 核心 o2server 功能缺口补齐：IM 实时消息、PDF 签章完整链路

---

## 1. 现状审计摘要

### 1.1 已完成的里程碑

| 里程碑 | 状态 | 关键交付 |
|--------|------|----------|
| Phase 0 可构建 | ✅ | `cargo build --workspace` 零 error |
| Phase 1 真实测试基座 | ✅ | `is_db_available()` 守卫、67 个 auth 连库测试 |
| Phase 2 U2.1-U2.4 | ✅ | LDAP 接入、OIDC 实现、gateway_fork + timer 持久化 |
| Phase 2-4 blocking gaps U1-U10 | ✅ | 10 个 blocking gap 已关闭（commit `7be346d4`） |
| 认证生态 | ✅ | LDAP + OIDC + 企业微信/钉钉 + 2FA |
| BPMN 执行语义 | ✅ | gateway_fork、TimerRegistry 持久化、tokio interval scanner |

### 1.2 未完成/部分完成项（本次计划覆盖）

| 类别 | 未完成项 | 数量 | 严重程度 |
|------|----------|------|----------|
| 测试缺口 | parity 行为契约失败 | 49 | P0 |
| 静默空数据 | 剩余 `Value::Null` | 100 | P1 |
| CMS 真实化 | 剩余 `Value::Bool(true)` stub | 96 | P1 |
| 生产就绪 | Tantivy 集成（网络阻塞） | 1 | P1 |
| 生产就绪 | Redis 会话存储未接入生产 | 1 | P1 |
| 生产就绪 | MySQL 集成测试仅 skip | 1 | P2 |
| 功能缺口 | IM 实时消息端点 | 33 | P2 |
| 功能缺口 | PDF 签章完整链路 | 1 | P2 |
| 性能 | Login P99 延迟（已优化，待压测验证） | 1 | P2 |

### 1.3 计划间的依赖关系

```
本次计划
├── Phase A：测试基线修复（4.1-4.3）
│   ├── U-A1: Value::Null 归零（Top 10 crate）
│   ├── U-A2: parity 49 失败修复
│   └── U-A3: CMS 96 stub 真实化
│
├── Phase B：生产就绪基础设施（4.4-4.6）
│   ├── U-B1: Tantivy 全文检索替换 PG to_tsvector
│   ├── U-B2: Redis 会话存储接入生产
│   └── U-B3: MySQL 集成测试真正跑通
│
└── Phase C：o2server 缺失功能补齐（4.7-4.9）
    ├── U-C1: IM 实时消息 33 端点
    ├── U-C2: PDF 签章完整链路
    └── U-C3: 其他缺失功能补齐
```

---

## 2. 实现单元详述

### Phase A：测试基线修复

#### U-A1. Value::Null 归零（Top 10 crate）

**目标**：将剩余 100 处 `Value::Null` 归零，统一用 `Value::Object` 省略 None 字段。

**范围**：按频次最高的 Top 10 crate 分批：
1. `cms_core_entity`（7 处）
2. `organization_core_entity`（12 处）
3. `meeting_core_entity`（14 处）
4. `calendar_core_entity`（13 处）
5. `meeting`（11 处）
6. `portal_assemble_designer`（10 处）
7. `process_designer`（9 处）
8. `auth`（剩余 3 处）
9. `bbs_assemble_control`（剩余 3 处）
10. 其余 16 个 crate 的剩余部分

**验收**：`grep -rn "Value::Null" crates/ | wc -l` 输出 0

---

#### U-A2. parity 49 失败修复

**目标**：将 parity 测试从 49 失败降至 0。

**策略**：
1. 对每个失败测试，读取 `crates/parity/src/lib.rs` 中的行为契约定义
2. 修复对应 handler 使其返回契约期望的响应体结构
3. 对无法修复的（外部依赖不可用），标记为 `#[ignore]` 并记录原因

**验收**：`cargo test -p parity --lib -- --test-threads=1` 全绿

---

#### U-A3. CMS 96 stub 真实化

**目标**：将 `crates/cms_assemble_control/src/lib.rs` 中 96 个 `Value::Bool(true)` 替换为真实查询。

**策略**：
1. 按业务域分批：document → fileinfo → form → surface → template → appinfo → categoryinfo
2. 每批 15-20 个 handler，复用 `row_to_json` 过渡
3. 每批补充集成测试

**验收**：`grep -c "Value::Bool(true)" crates/cms_assemble_control/src/lib.rs` 输出 0

---

### Phase B：生产就绪基础设施

#### U-B1. Tantivy 全文检索集成

**目标**：替换 `crates/search/src/lib.rs` 中的 PostgreSQL `to_tsvector` 为 Tantivy 本地索引。

**阻塞解除条件**：网络恢复后可从 crates.io 下载 `tantivy` crate。

**验收**：
- `cargo test -p search --lib` 全绿
- 三个 search 端点（documents/subjects/messages）返回 Tantivy 检索结果

---

#### U-B2. Redis 会话存储接入生产

**目标**：将 `SessionManager` 的 Redis 后端从"可选初始化"变为"生产默认启用"。

**现状**：`crates/shared/src/session.rs` 已有 Redis 连接池 + fallback 逻辑，但：
1. `src/main.rs` 仅在 `REDIS_URL` 存在时初始化 Redis
2. 未配置 Redis 时使用纯内存存储，多实例部署下会话不一致

**实现**：
1. 在 `SessionManager::new()` 中默认尝试连接 Redis（超时 2s）
2. Redis 不可达时记录 warn 并 fallback 到内存 + DB 持久化
3. 在 `docker-compose.yml` 添加 Redis service
4. 在 CI 的 integration-tests job 中添加 Redis service

**验收**：
- 多实例部署下会话失效一致
- CI integration-tests job 包含 Redis service

---

#### U-B3. MySQL 集成测试真正跑通

**目标**：使 CI 的 `DATABASE_DIALECT=mysql` 集成测试真正执行，而非 skip。

**现状**：`tests/integration_runner.rs` 在 MySQL 模式下直接 return。

**实现**：
1. 审计所有 migration SQL，确保 MySQL 兼容
2. 修复所有 `$N` 参数为 `?`（`rewrite_pg_to_mysql` 已实现）
3. 修复 `jsonb` → `json`、`SERIAL` → `INT AUTO_INCREMENT` 等类型差异
4. 在 CI matrix 中运行 MySQL 集成测试

**验收**：`DATABASE_DIALECT=mysql cargo test --test integration_runner` 通过

---

### Phase C：o2server 缺失功能补齐

#### U-C1. IM 实时消息 33 端点

**目标**：实现 o2server `ImAction` 下的 33 个消息端点。

**现状**：`crates/realtime` 已有 WebSocket 基础框架，但仅支持通用 room broadcast。

**实现**：
1. 对每个 ImAction @Path，在 `crates/message` 或 `crates/realtime` 中添加对应 handler
2. 支持单聊、群聊、已读回执、消息撤回、文件消息
3. 消息持久化到 `x_message` 表

**验收**：parity 测试覆盖全部 33 个 IM 端点

---

#### U-C2. PDF 签章完整链路

**目标**：完成 `crates/signature` 的 PDF 签章完整业务流程。

**现状**：`crates/signature/src/lib.rs` 已有 RSA 签名 + lopdf 嵌入框架，但：
1. `SignatureInfo` 缺少 `signer_name` 字段
2. `embed_signature` 中 ByteRange 计算不完整
3. 缺少证书链验证

**实现**：
1. 补全 `ByteRange` 计算逻辑
2. 添加证书链验证
3. 添加签章状态查询端点

**验收**：端到端签章 + 验证测试通过

---

#### U-C3. 其他缺失功能补齐

**目标**：补齐 o2server 有但 oa4rust 缺失的功能。

**候选清单**：
1. `crates/captcha_store` 验证码生成/校验（已有框架，需接入 auth 登录）
2. `crates/sms` 短信网关抽象（已有框架，需接入验证码发送）
3. `crates/jpush` 推送通知完整实现
4. `crates/ai` AI 助手对话完整链路

**验收**：每个子项有至少 1 个端到端测试通过

---

## 3. 优先级与时间盒

| 阶段 | 内容 | 时间盒 | 优先级 |
|------|------|--------|--------|
| Phase A | 测试基线修复（U-A1/U-A2/U-A3） | 2-3 周 | P0 |
| Phase B | 生产就绪（U-B1/U-B2/U-B3） | 2-3 周 | P1 |
| Phase C | 缺失功能补齐（U-C1/U-C2/U-C3） | 3-4 周 | P2 |

**推荐执行顺序**：A → B → C（先让测试全绿，再确保生产可运维，最后补齐功能）

---

## 4. 关键约束与假设

1. **网络约束**：Tantivy 集成依赖 crates.io 可达，若持续不可达，可暂时保留 PG `to_tsvector` 作为 fallback
2. **外部服务**：Redis、MySQL 仅用于生产/CI，本地开发可 skip
3. **Scope 边界**：不重构 `cms_assemble_control` 单文件结构（作为后续架构债处理）
4. **IM 范围**：仅实现 ImAction 核心消息端点，不包含完整的即时通讯协议（XMPP/WebRTC）

---

## 5. 验收标准汇总

| 指标 | 当前值 | 目标值 |
|------|--------|--------|
| `cargo test` 通过率 | 94.7% (838/887) | ≥ 99% |
| `Value::Null` 数量 | 100 | 0 |
| CMS `Value::Bool(true)` | 96 | 0 |
| parity 失败数 | 49 | 0 |
| 集成测试覆盖数据库 | PostgreSQL only | PostgreSQL + MySQL |
| 会话存储 | 纯内存 + DB | Redis + DB（多实例一致） |
| 全文检索 | PG `to_tsvector` | Tantivy（网络恢复后） |
| IM 端点 | 0/33 | 33/33 |

---

## 6. 下一步动作

1. **立即**：创建 `docs/plans/2026-08-19-002-unified-completion-plan.md`（本计划）
2. **按优先级**：从 Phase A 开始执行，先修复测试基线
3. **每完成一个 Phase**：更新本计划的状态标记，记录实际耗时与偏差

---

## 实现情况（2026-08-21 审计）

**审计基准：** 工作树 HEAD 314c7a75；判定状态：active（补建 frontmatter；DoD 第 2 条未完全达成）

### 已验证完成

- U-B2 Redis 会话存储生产默认：提交 7710d8af（phase-b）
- U-B3 MySQL 集成测试跑通：提交 004e02d9、d972e010
- U-C1 IM 实时消息端点：提交 d972e010（implement IM endpoints）
- U-C2 PDF 签章完整链路：提交 ae911482（PDF cert chain verification）
- U-C3 captcha/sms/jpush/AI 链路：提交 ae911482（jpush push + AI SSE streaming + SMS gateway）
- U-A2 parity 失败修复：已由 phase-a 系列提交处理（d83a80e5，82 个文件），最终失败数未在静态审计中实测

### 未完成 / 遗留 → 待汇入剩余工作汇总计划

- U-A1 Value::Null 归零：实测仍残留 15 处（验收标准为 0）
- U-A3 CMS `Value::Bool(true)` stub 清零：实测仍残留 17 处（原 96）
- U-B1 Tantivy 全文检索集成：全仓 0 处 tantivy 引用，search crate 仍为 PG to_tsvector（6 处），网络阻塞未解除

### DoD 达成度

- 测试通过率 ≥99%：静态审计不运行测试，以提交 314c7a75"清零全部剩余测试失败"为旁证，未实测
- Value::Null 归零：未达成（15 残留）
- 双库集成测试：已达成（PG + MySQL 提交在档）
- 生产基础设施：Redis ✓ / Tantivy ✗ / 性能基线文档 ✓
- IM + PDF 签章：已达成
