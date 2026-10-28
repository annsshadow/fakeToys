# Redis 集成处置 · 决策与落地规划

- **登记日**：2026-10-28
- **决策日**：2026-10-28
- **决策：走路线 B —— 删除 Redis 集成。** 部署形态确认为**单实例**，不引入 Redis。
- **范围**：`oa4rust/backend`（全仓唯一使用 Redis 的子项目；`laiyipao` / `augmentor` / `auto-checkin` / `cool` / `llm-lab` 零引用）
- **方法**：只读勘察 → 判据 → 最小改动 → 红→绿验证 → 台账。每条结论都能指回 `file:line`。
- **不写什么**：不写 secrets、不写真实服务器地址、不在文档里回显本机绝对路径。

---

## 0. 决策摘要

| 项 | 结论 |
| --- | --- |
| 部署形态 | 单实例 |
| 处置 | **B：删除 Redis 集成**（会话用内存 + PG，限流用进程内存） |
| 被否 | A 接线（无多实例需求，接线只是增加运行期依赖与故障面）、C 保留不宣传（债务与误导留存） |
| 删除规模 | 整文件 3 个共 **785 行**（`redis.rs` 150 / `messaging.rs` 373 / `middleware/rate_limit_distributed.rs` 262）+ `session.rs`、`rate_limit.rs` 内的 Redis 分支约 350 行 |
| 行为变化 | **零功能变化**（当前 Redis 路径本就不可达，见 §2）；变的是启动日志不再谎称「Redis unreachable」 |
| 退出判据 | `cargo test -p shared --lib` 全绿、`cargo clippy --workspace --all-targets` 零告警、`shared` 行覆盖率上升、文档三处措辞同步 |

---

## 1. 与既有文档的关系

| 既有文档 | 本文档承接 |
| --- | --- |
| `docs/plans/2026-10-27-comprehensive-test-coverage.md` §4.4 **D5** | D5 只登记「`main.rs:77` 短路导致 Redis 未初始化」；本文档扩查成完整事实（含消息总线无订阅方）并给出决策 |
| `docs/ops/redis-cache-strategy.md` | 2026-10-14 注记称「会话存储与分布式限流**已默认走 Redis**」——不成立；执行 B 时须改 |
| `docs/oa4rust/05-environment.md:34` | 把 `REDIS_URL` 标为「推荐」；执行 B 时删除该行 |
| `oa4rust/backend/.env` / `.env.example:4-6` | 都带 `REDIS_URL` 且注释称「默认 session 存储」；执行 B 时删除 |

---

## 2. 事实基线（2026-10-28 逐条验证）

### 2.1 使用面

| 能力 | 代码位置 | 机制 |
| --- | --- | --- |
| 会话存储 L2 缓存 | `crates/shared/src/session.rs` | 内存 `HashMap` → Redis → PostgreSQL 三层。key = `oa4rust:session:` + **HMAC-SHA256 签名后的 token** |
| 分布式限流 | `crates/shared/src/rate_limit.rs` + `middleware/rate_limit_distributed.rs` | INCR+EXPIRE 滑动窗口，key = `rate:{client_ip}:{window_secs}` |
| 跨实例令牌吊销 | `crates/shared/src/messaging.rs::RedisPubSubBus` | Redis Pub/Sub，topic = `token-threshold` |
| 连接池工厂 | `crates/shared/src/redis.rs` | `from_url` / `from_url_with_retry` / `with_connection` / `is_healthy` / `reconnect` / `redis_url_from_env` |

依赖声明：`crates/shared/Cargo.toml:44` `redis.workspace = true`；workspace 定义在根 `Cargo.toml:318`。**全仓只有 `shared` 用这个 crate。**

### 2.2 路径断裂的五条证据

| # | 证据 | 后果 |
| --- | --- | --- |
| E1 | `src/main.rs:77`：`let redis_available = session_manager.init_redis() && rate_limiter.init_redis();` —— `init_redis` 检测到已在 tokio 运行时内时直接 `return false`（`session.rs:151` / `rate_limit.rs:81`），而 `async fn main` 必然如此 | `&&` **短路** ⇒ 第二个永不执行 ⇒ `redis_available` 恒 `false`，两个 pool 都是 `None` |
| E2 | `init_redis_async` 在生产代码**零调用**（只命中测试） | 没有替代路径 |
| E3 | `RedisPubSubBus` / `InMemoryBus` **零构造点**（只在 `lib.rs:43` re-export）；`SessionManager.message_bus` 恒 `None`（`session.rs:71/94`），字段虽 `pub` 但无人赋值 | `broadcast_logout` 的 `if let Some(ref bus)` 分支永不执行 |
| E4 | `MessageBus::subscribe` 在生产代码**零调用**（唯一 `.subscribe(` 命中是 `crates/realtime/src/lib.rs:229` 的 tokio broadcast，无关） | 跨实例吊销是**半成品**：publish 侧有、subscribe 侧不存在 |
| E5 | `from_url_with_retry` / `with_connection` / `is_healthy` / `reconnect` 全仓零调用；CI 不起 Redis（`oa4rust-ci.yml:146` 只有注释：某测试无 Redis 时会等 TCP 超时 60s+） | 这些 API 从未被执行，行为未验证 |

**结论**：运行时一直是「内存 session + PG 持久化 + 进程内限流」。启动日志却每次都打
`Redis unreachable; session store and rate limiter using in-memory fallback` ——
运维会去查 Redis，而 **Redis 从未被尝试连接**（短路发生在建连之前）。

### 2.3 删除边界（已核实，避免删多/删少）

| 符号 | 外部使用者 | 处置 |
| --- | --- | --- |
| `MessageBus` / `TokenThresholdEvent` | 仅 `session.rs:17/46/643`（`message_bus` 字段与发布分支） | 随字段一起删 |
| `Envelope` / `InMemoryBus` | **零外部引用** | 随 `messaging.rs` 整文件删 |
| `SessionManager::sign_token` / `verify_and_extract` | 同时被 **PG 路径**使用（`persist_to_db`、`validate_session` 的 PG 分支） | **保留** |
| `SessionManager::broadcast_logout` | `crates/auth/src/safe_logout.rs:52` | **保留函数**，删掉其中的 `bus.publish` 分支（退化为「只做本地清理」） |
| `check_token_threshold` | `create_session` 内部 | 保留（PG 阈值逻辑与 Redis 无关） |
| `WindowCounter` trait | `rate_limit.rs` 的注入点 + 测试 | 若 `RedisWindowCounter` 删除后无实现者，则 trait 与注入点一并删；`window_decision` / `distributed_rate_key` 无使用者则删 |

---

## 3. 判据：B 完成后怎样算「收干净」

| 编号 | 判据 | 验证方式 |
| --- | --- | --- |
| R1 | 启动日志**不再出现**「Redis unreachable」，改为明确的单实例说明 | 读 `main.rs` + 启动一次 |
| R2 | 全仓 `grep -ri redis --include=*.rs` **零命中**（除文档/历史账本） | 命令核验 |
| R3 | `cargo test -p shared --lib` 全绿；`cargo clippy --workspace --all-targets` 零告警（workspace lints 是 `deny`，删代码最容易暴露未用 import / 未用变量） | CI 同款命令 |
| R4 | `Cargo.lock` 里 `redis` 及其传递依赖消失；supply-chain 面减少 | `cargo tree -i redis` 应报 not found |
| R5 | `shared` 行覆盖率**上升**（分母少了未覆盖代码），且不出现新的未覆盖分支 | `cargo llvm-cov -p shared --lib --summary-only` |
| R6 | 文档三处措辞同步（§1 表格里的三份） | 人工核对 |

---

## 4. 决策记录

**选 B 的理由**

1. 部署形态是单实例——Redis 的全部价值（跨实例会话共享、跨进程限流计数、跨实例吊销）**前提都不存在**。
2. 当前 Redis 路径**一行都没被执行过**（E1–E5），删除属于「删死代码」而非「删功能」，回归风险极低。
3. 保留它有三个持续成本：每次启动都打误导性 warn（运维噪音）、`shared` 覆盖率分母被 ~800 行不可达代码拖住、供应链多一个 crate。
4. 将来若真要多实例，重建路径清晰：本文档 §7 保留了完整的 A 方案设计，且 git 历史里代码俱在。

**被否的 A**：接线后每实例启动多 2 秒超时窗口、会话变成双写（Redis + PG，一致性需重新论证），而收益为零——没有第二个实例来共享。**被否的 C**：只改文档不改代码，等于把「日志说谎」这件事固化成常态。

---

## 5. 执行计划（分批，每批可独立回退）

### B-1 删连接池与消息总线（整文件）

- 删 `crates/shared/src/redis.rs`（150 行）
- 删 `crates/shared/src/messaging.rs`（373 行）
- `crates/shared/src/lib.rs`：删 `pub mod redis;`、`pub mod messaging;` 与第 43 行的 re-export
- **退出判据**：`cargo check -p shared` 报错清单只剩 `session.rs` / `rate_limit.rs` / `testing.rs` 的引用（这些留给 B-2/B-3）

### B-2 会话层去 Redis

- `session.rs`：删 `redis_pool` 字段与两处构造赋值、`init_redis`、`init_redis_async`、`get_redis_pool`、`message_bus` 字段、`create_session` 的 Redis 写、`validate_session` 的 Redis 查、`remove_session` / `remove_sessions_by_person` / `cleanup_expired_sessions` 的 Redis 删
- `broadcast_logout`：保留「查阈值 → 删本地过期会话 → 记日志」，删 `bus.publish` 分支；函数名保留（改名的涟漪大于收益），在文档注释里注明「单实例：本地清理」
- **保留** `sign_token` / `verify_and_extract`（PG 路径在用）
- 同步删除我上一轮加的 Redis 相关用例（`init_redis_inside_runtime_declines` 等）
- **退出判据**：`cargo test -p shared --lib session::tests` 全绿

### B-3 限流层去 Redis

- `rate_limit.rs`：删 `redis_pool` 字段、`init_redis` / `init_redis_async` / `get_redis_pool`、`active_window_counter` 的 Redis 推导、`record_failure` / `reset` 的 Redis 分支
- `middleware/rate_limit_distributed.rs`：删 `RedisWindowCounter`；若 `WindowCounter` trait 已无实现者，连同 `set_window_counter_override_for_test` 与相关用例一起删
- `testing.rs:146` 的 `redis_url_from_env` 调用点删除
- **退出判据**：`cargo test -p shared --lib rate_limit` 全绿；`cargo clippy --workspace --all-targets` 零告警

### B-4 依赖与启动路径

- `crates/shared/Cargo.toml:44` 删 `redis.workspace = true`
- 根 `Cargo.toml:318` 删 workspace 依赖条目
- `src/main.rs:76-84`：把 `redis_available` 那三行换成一条明确日志（单实例：内存 session + PG 持久化 + 进程内限流）
- `.env` / `.env.example`：删 `REDIS_URL` 三行
- **退出判据**：`cargo tree -i redis` 报 not found；`cargo build` 通过

### B-5 文档与 CI 注释

- `docs/oa4rust/05-environment.md:34`：删 `REDIS_URL` 行
- `docs/ops/redis-cache-strategy.md`：顶部注记改为「本能力已于 2026-10-28 移除（单实例部署），保留本文作为未来多实例化的设计参考」
- `.github/workflows/oa4rust-ci.yml:146`：删那条关于 Redis 测试超时的注释（对应测试已不存在）
- **退出判据**：`grep -ri redis .github docs/oa4rust` 只剩历史账本与本决策文档

### B-6 复核覆盖率

- `cargo llvm-cov -p shared --lib --summary-only`，记录新的行覆盖率与各文件数字
- 更新 `docs/plans/2026-10-27-comprehensive-test-coverage.md`：`shared` 的基线与 D5/D8 相关表述

---

## 6. 执行时的技术注意

1. **workspace lints 是 `deny`**：删代码最容易留下未用 `use` / 未用变量 / 未用 `pub` 项。每批结束必须跑 `cargo clippy --workspace --all-targets`，不能只看 `cargo build`。
2. **`Cargo.lock` 会变**：删依赖后 lock 要一起提交，否则 CI 的 `--locked`（若有）会红。
3. **别顺手删 `sign_token`**：它看着像 Redis 专用（key 里存签名 token），但 `persist_to_db` 与 PG 分支的 `verify_and_extract` 都依赖它。删了会让 PG 会话查不回来。
4. **`broadcast_logout` 的语义变化要写进注释**：单实例下它只清本地内存会话；名字里的 broadcast 已无实际含义，保留名字是为了不动 `auth::safe_logout` 的调用面。
5. **测试要一起删**：我上一轮加的 `init_redis_inside_runtime_declines`、`init_redis_async_is_idempotent_*`、`injected_counter_bypasses_the_redis_url_gate` 等在 B-2/B-3 后会编译失败，必须同批处理。

---

## 7. 若将来要多实例：重建路径（A 方案存档）

本节保留被否的 A 方案，供将来决策复用。

- **A1 初始化**：`main.rs` 改为
  ```rust
  let redis_available = session_manager.init_redis_async().await
      & rate_limiter.init_redis_async().await; // `&` 非短路：两者都要尝试
  ```
  必须用 `&` 而不是 `&&`——后者正是当年短路 bug 的成因。
- **A2 日志三态**：区分「未配置」「配置了连不上」「已连上」，让运维能定位。
- **A3 多实例用例**：两个 `SessionManager` 共用同一 Redis，A 建会话、B 能查到；A 删除、B 查不到。**断言前必须先确认 B 的内存表为空**，否则「命中」可能只是各查各的内存。
- **A4 消息总线**：需要新写订阅循环（约 40–60 行）+ 自发布过滤（Redis Pub/Sub 不区分来源，本实例会收到自己发的事件）+ 重连验证。
- 代码可从 git 历史取回（本决策提交之前的所有版本）。

---

## 8. 验证命令

```bash
cd oa4rust/backend

# 编译与门禁（workspace lints = deny，clippy 必须一起跑）
cargo clippy --workspace --all-targets -- -D warnings

# 单测（shared 需要 PG；用本机 15432 的 postgres:16 容器）
DATABASE_URL=postgres://o2server:password@localhost:15432/oa4rust?sslmode=disable \
  cargo test -p shared --lib

# 覆盖率
DATABASE_URL=... cargo llvm-cov -p shared --lib --summary-only

# 依赖面收干净
cargo tree -i redis   # 期望：package ID specification `redis` did not match any packages

# 全仓残留
grep -ri redis --include=*.rs crates src   # 期望：无输出
```

---

## 9. 风险与回滚

| # | 风险 | 应对 |
| --- | --- | --- |
| 1 | 误删 `sign_token`/`verify_and_extract` 导致 PG 会话失效 | §6.3 已点名；B-2 退出判据含 `session::tests` 全绿 |
| 2 | 删 `WindowCounter` 时连带删掉限流的**降级语义测试** | 降级测试（`test_degrade_to_memory_when_redis_unreachable`）随 Redis 一起失去意义，删除时在提交信息里写明 |
| 3 | `Cargo.lock` 未同步 | 每批 `cargo build` 后检查 `git status` 是否含 lock 变更 |
| 4 | 将来要多实例才发现删早了 | §7 存档 + git 历史；重建成本约 1–2 天（含测试） |
| 5 | 文档漏改导致下一个人以为还有 Redis | §3 R2/R6 用 grep 做机械核验，不靠记忆 |

**回滚粒度**：B-1..B-6 各自一个 commit，按逆序 `git revert` 即可；行为上回滚等价于恢复「死代码」状态，不影响线上。
