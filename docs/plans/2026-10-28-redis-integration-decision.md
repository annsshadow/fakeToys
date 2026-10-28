# Redis 集成处置 · 落地规划

- **登记日**：2026-10-28
- **范围**：`oa4rust/backend`（全仓唯一使用 Redis 的子项目；`laiyipao` / `augmentor` / `auto-checkin` / `cool` / `llm-lab` 零引用）
- **目标**：把「Redis 集成代码全在、路径全断」这件事从口头结论变成**可复核的处置计划**：先钉事实，再给判据，最后给三条互斥路线与各自的落地步骤。
- **方法**：只读勘察 → 判据 → 最小改动 → 红→绿验证 → 台账。每条结论都能指回 `file:line`。
- **不写什么**：不写 secrets、不写真实服务器地址、不在文档里回显本机绝对路径。

---

## 0. 与既有文档的关系

| 既有文档 | 本文档承接 |
| --- | --- |
| `docs/plans/2026-10-27-comprehensive-test-coverage.md` §4.4 的 **D5** | D5 只登记了「`main.rs:77` 短路导致 Redis 未初始化」；本文档把它扩成完整事实（含消息总线无订阅方），并给出处置路线 |
| `docs/ops/redis-cache-strategy.md` | 该文 2026-10-14 注记称「会话存储与分布式限流**已默认走 Redis**」——与实况不符，本文档 §1.3 列偏差 |
| `docs/oa4rust/05-environment.md:34` | 把 `REDIS_URL` 标为「推荐」；本文档给出处置后应改成的措辞 |
| `oa4rust/backend/.env` / `.env.example` | 都带 `REDIS_URL=redis://127.0.0.1:6379`，注释称「默认 session 存储」 |

---

## 1. 事实基线（2026-10-28 逐条验证）

### 1.1 使用面

| 能力 | 代码位置 | 机制 |
| --- | --- | --- |
| 会话存储 L2 缓存 | `crates/shared/src/session.rs` | 内存 `HashMap` → Redis → PostgreSQL 三层。key = `oa4rust:session:` + **HMAC-SHA256 签名后的 token**；`SET EX` / `GET` / `DEL` / `KEYS` 前缀扫描 |
| 分布式限流 | `crates/shared/src/rate_limit.rs` + `crates/shared/src/middleware/rate_limit_distributed.rs` | INCR+EXPIRE 滑动窗口，key = `rate:{client_ip}:{window_secs}`；运行期失败降级进程内存；`WindowCounter` trait 留了测试注入点 |
| 跨实例令牌吊销 | `crates/shared/src/messaging.rs::RedisPubSubBus` | Redis Pub/Sub，topic = `token-threshold`，由 `SessionManager::broadcast_logout` 发布 |
| 连接池工厂 | `crates/shared/src/redis.rs` | `from_url` / `from_url_with_retry` / `with_connection` / `is_healthy` / `reconnect` / `redis_url_from_env` |

规模：Redis 相关行约 208 行（`redis.rs` 33 / `messaging.rs` 40 / `session.rs` 37 / `rate_limit.rs` 66 / `rate_limit_distributed.rs` 32）。

### 1.2 路径断裂的五条证据

| # | 证据 | 后果 |
| --- | --- | --- |
| E1 | `src/main.rs:77`：`let redis_available = session_manager.init_redis() && rate_limiter.init_redis();` —— `init_redis` 在检测到已在 tokio 运行时内时直接 `return false`（`session.rs:151` / `rate_limit.rs:81`），而 `async fn main` 必然如此 | `&&` **短路** ⇒ 第二个永不执行 ⇒ `redis_available` 恒 `false`，两个 pool 都是 `None` |
| E2 | `init_redis_async`（唯一能在运行时内用的入口）在生产代码**零调用**（`grep init_redis_async` 只命中测试） | 没有替代路径 |
| E3 | `RedisPubSubBus` / `InMemoryBus` **零构造点**（只出现在 `crates/shared/src/lib.rs:43` 的 re-export）；`SessionManager.message_bus` 恒为 `None`（`session.rs:71/94`），字段虽 `pub` 但无人赋值 | `broadcast_logout` 里 `if let Some(ref bus) = self.message_bus` 分支永不执行 |
| E4 | `MessageBus::subscribe` 在生产代码**零调用**（唯一的 `.subscribe(` 命中是 `crates/realtime/src/lib.rs:229` 的 tokio broadcast，与消息总线无关） | 即使补上 publish 侧，也**没有订阅方**，消息进黑洞 |
| E5 | `from_url_with_retry` / `with_connection` / `is_healthy` / `reconnect` 全仓零调用；CI 不起 Redis（`oa4rust-ci.yml:146` 只有一条注释：某测试在无 Redis 时会等 TCP 超时 60s+） | 这些 API 从未被执行过，行为未被验证 |

**结论**：运行时永远是「内存 session + PG 持久化 + 进程内限流」。而启动日志每次都打
`Redis unreachable; session store and rate limiter using in-memory fallback` ——
运维看到会去查 Redis，但 **Redis 从未被尝试连接**（短路发生在建连之前）。

### 1.3 成熟度分级（这是路线选择的关键）

三项能力**不在同一成熟度**，不能一起处置：

| 能力 | 代码完整度 | 缺什么 | 接线成本 |
| --- | --- | --- | --- |
| 会话 L2 缓存 | 完整（create / validate / remove / remove_by_person / cleanup 都有 Redis 分支） | 仅缺初始化 | 小 |
| 分布式限流 | 完整（INCR+EXPIRE + 失败降级 + 决策函数） | 仅缺初始化 | 小 |
| 跨实例令牌吊销 | **半成品**：publish 侧有，subscribe 侧没有 | 需**新写订阅循环** + 事件处理语义 | 大 |

### 1.4 文档与事实的偏差

- `docs/ops/redis-cache-strategy.md`：注记「会话存储与分布式限流已默认走 Redis」→ 不成立（E1）。
- `docs/oa4rust/05-environment.md:34`：`REDIS_URL` 标「推荐」→ 实际填不填都一样。
- `.env` / `.env.example:4-6`：注释「Redis（Phase B-U-B2: 默认 session 存储）」→ 与实现不符。

---

## 2. 判据：怎样才算「Redis 真的在用」

不满足以下四条中的任何一条，就不能说「已启用 Redis」：

| 编号 | 判据 | 当前 |
| --- | --- | --- |
| R1 | 启动日志能区分三种状态：**未配置** / **配置了但连不上** / **已连上** | ✗ 三者都打同一句「unreachable」 |
| R2 | Redis 断连后可降级到内存，且降级**可观测**（有 warn 且带原因） | 代码里有，但从未被执行（E1/E5） |
| R3 | 多实例语义可验证：两个 `SessionManager` 共用一个 Redis，A 建的会话 B 能查到 | ✗ 无测试、无 Redis 容器 |
| R4 | Redis 分支要么有测试，要么在覆盖率里显式豁免并写明理由 | ✗ 这些分支正是 `shared` 剩余缺口主体（`session.rs` 52.56% / `messaging.rs` 51.76% / `redis.rs` 65.56%） |

---

## 3. 三条互斥路线

| 维度 | **A 接线** | **B 删除** | **C 保留但不宣传** |
| --- | --- | --- | --- |
| 改动面 | `main.rs` 1 行 + 日志分级；消息总线需新增订阅循环 | 删 `redis.rs`、`messaging.rs` 的 Redis 实现、`session`/`rate_limit` 的 Redis 分支 | 只改文档与 `.env` 注释 |
| 风险 | 会话路径改为「先 Redis」，需重测登录/登出/并发 | 若将来要横向扩容需重写 | 零风险，但债务留存 |
| 验证成本 | 需起 Redis 容器 + 多实例用例 | 删完跑全量即可 | 无 |
| 对覆盖率 | 分母变大（要补 Redis 分支测试），短期分数下降 | 分母变干净，分数上升 | 不变 |
| 适合什么 | 确实要跑多实例 | 单实例部署是既定事实 | 还没想清楚 |

**推荐：先做 A-core（会话 + 限流接线），消息总线单独立项。** 理由见 §4 / §5。

---

## 4. 推荐路线 A-core 的落地步骤

> 前提：确认部署形态确实需要 Redis（多实例或重启不丢会话）。若只跑单实例，直接走 B。

### A1 初始化改为 async 且不短路

`src/main.rs:76-84`：

```rust
// before
let redis_available = session_manager.init_redis() && rate_limiter.init_redis();

// after
let redis_available = session_manager.init_redis_async().await
    & rate_limiter.init_redis_async().await; // `&` 非短路：两者都要尝试
```

注意用 `&` 而不是 `&&`——`&&` 会在第一个失败时跳过第二个，正是当前 bug 的成因。

### A2 日志分级（R1）

把单条 warn 拆成三态，让运维能区分「没配」与「配了连不上」：

```rust
match (redis_url_from_env().is_some(), redis_available) {
    (false, _) => tracing::info!("REDIS_URL 未配置：使用内存 session + 进程内限流"),
    (true, true) => tracing::info!("Redis 已连接：session 与限流走分布式后端"),
    (true, false) => tracing::warn!("REDIS_URL 已配置但连接失败：降级内存 + PG"),
}
```

### A3 补 R3 的多实例用例（这是 A-core 的退出判据）

新增 `crates/shared/src/session.rs` 内的 `#[tokio::test]`：

1. 从 `REDIS_URL` 建一个 `RedisPool`，分别注入两个 `SessionManager`（`with_pool_and_config` 之后直接写 `redis_pool` 字段）；
2. 实例 A `create_session`；
3. 实例 B `validate_session(同一 token)` 必须命中——**证明会话真的过了 Redis 而不是各存各的内存**；
4. A `remove_session`，B 再查必须 miss。
5. Redis 不可达时（把 URL 指向关闭端口）两个实例都必须降级且不 panic。

> 这条用例只有在「两个实例的内存表互不可见」时才有意义，所以必须断言 `B.sessions` 里原本没有该 token（先 `assert!(B.sessions.read().await.is_empty())`）。

### A4 文档同步

- `docs/oa4rust/05-environment.md:34`：把「推荐」改成「可选；配置后 session/限流走分布式，未配置或连不上自动降级」；
- `docs/ops/redis-cache-strategy.md` 的 2026-10-14 注记：改为「会话与限流已接线（2026-10-28）；跨实例令牌吊销仍未实现（见 §5）」；
- `.env.example:4-6` 注释同步。

### A5 回滚

单 commit 可回退；回退后行为等价于现状（内存 + PG）。A1 那一行是唯一的行为开关。

---

## 5. A-plus（跨实例令牌吊销）——单独立项，不在 A-core 里做

现状是**半成品**：publish 侧写完，subscribe 侧不存在（E4）。要让「管理员在实例 A 禁用某人 → 实例 B 立刻踢掉其会话」真正生效，至少需要：

1. **订阅循环**：启动时 `bus.subscribe("token-threshold")`，收到 `TokenThresholdEvent` 后按 `person_unique` 调 `remove_sessions_by_person`（注意**不能**再调 `broadcast_logout`，否则会回环发布）；
2. **幂等与自发布过滤**：本实例发布的自己也会收到（Redis Pub/Sub 不区分来源），需要按 `expired_tokens` 交集判断或加 instance id；
3. **重连**：`RedisPubSubBus::subscribe` 内部已有重连循环（`messaging.rs:240-280`），但要验证断连恢复后不会丢订阅；
4. **测试**：两个 `InMemoryBus` 共享同一个 bus 实例模拟跨实例；真实 Redis 用容器起两个 manager。

工作量：新代码约 40-60 行 + 2 条集成用例。**建议先落 A-core，观察是否真有多实例需求再决定。**

---

## 6. 若选 B（删除）的落地要点

1. 删 `crates/shared/src/redis.rs` 与 `lib.rs` 的 `pub mod redis`；
2. 删 `messaging.rs` 的 `RedisPubSubBus`（保留 `InMemoryBus`？——它也没有构造点，一并删更干净，但 `MessageBus` trait 可留作扩展点）；
3. `session.rs` / `rate_limit.rs` 删 Redis 分支，保留 `Option<Pool>` 的 PG 路径；
4. `Cargo.toml` 去掉 `redis` 依赖（连带减少 supply-chain 面）；
5. 文档：`.env*` 删 `REDIS_URL`，两份 docs 改成「不提供 Redis 集成」；
6. 验证：`cargo test -p shared --lib` 全绿 + `cargo clippy --workspace` 零告警（workspace lints 是 deny，删代码可能暴露未用 import）。

**收益**：`shared` 行覆盖会从 67.45% 直接升（分母少 ~208 行未覆盖代码），且不再有「日志说 Redis 挂了」的误导。

---

## 7. 若选 C（保留但不宣传）

只改三处措辞（§1.4 三条），并在 `redis-cache-strategy.md` 顶部加一句：

> 本能力**当前未接线**（`main.rs` 的初始化在 async 上下文内短路，见 `docs/plans/2026-10-28-redis-integration-decision.md`），仅作为预留实现保留。

零风险，但 D5 的误导仍在（启动日志照旧）。

---

## 8. 验证命令（A-core 落地后）

```bash
# 起 Redis（与 PG 同样走 Docker；本机引擎需先启 com.docker.service）
docker run -d --name fakeToys-redis -p 16379:6379 redis:7-alpine

# 指向它并跑 shared 全量
cd oa4rust/backend
REDIS_URL=redis://127.0.0.1:16379 cargo test -p shared --lib -- --nocapture

# 覆盖率（Redis 分支应明显上升）
REDIS_URL=redis://127.0.0.1:16379 cargo llvm-cov -p shared --lib --summary-only

# 降级路径：指向关闭端口，必须 warn 且不 panic
REDIS_URL=redis://127.0.0.1:1 cargo test -p shared --lib rate_limit::tests
```

---

## 9. 风险与未决

| # | 风险 / 未决 | 说明 |
| --- | --- | --- |
| 1 | **启动变慢** | Redis 不可达时每个 `init_redis_async` 最多等 2s（`tokio::time::timeout`），A1 里两次即 4s。可接受，但要写进部署文档 |
| 2 | **会话双写一致性** | 接线后 session 同时写 Redis 与 PG；Redis 成功、PG 失败时两者不一致。当前实现是 best-effort（`let _ =`），需确认这是否符合预期 |
| 3 | **CI 是否需要 Redis** | 若 A3 用例依赖真实 Redis，CI 要加 service 容器；否则该用例只能本地跑（那就等于没门禁） |
| 4 | 部署形态未知 | 是否真的要多实例？这决定 A 还是 B —— **需要产品/运维确认** |
| 5 | 与覆盖率计划的耦合 | 选 B 则 `shared` 剩余缺口自动消失一部分；选 A 则要先补 Redis 分支测试。**建议先定路线再继续提 `shared` 覆盖率** |
