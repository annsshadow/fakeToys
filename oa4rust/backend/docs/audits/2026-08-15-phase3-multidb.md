# OA4RUST Phase 3 — 多数据库（第二后端）可行性审计

**Date:** 2026-08-15
**Scope:** 只读可行性评估。未修改任何 `.rs` / `Cargo.toml`；未运行 `cargo build` / `cargo test`。
**Question:** 当前数据访问架构能否支撑第二个数据库后端（MySQL / SQLite / Oracle）？需要做什么、成本与风险如何？

---

## 0. TL;DR（结论）

当前代码**已经存在双连接池**（raw-SQL 的 `deadpool_postgres::Pool` 与 SeaORM 的 `sea_orm::DatabaseConnection` 同时在 `src/main.rs` 注入），但**两者都指向 Postgres**，且两条路径的"后端可移植性"完全不同：约 60 个 crate 走的是**手写 Postgres 原生 SQL**（含 `$N` 占位符、`NOW()`、`::text`、`jsonb`、`uuid-ossp`、`ON CONFLICT` 等），基本不可移植；仅约 22 个 `*_core_entity` crate 走 SeaORM（后端无关，但 SeaORM 1.x **不支持 Oracle**，且共享 helper 层是空桩）。

**结论：短期支持 MySQL/SQLite 现实可行（以 SeaORM 为轴心 + 迁移框架改造），但要把占主导的 raw-SQL 路径整体移植是接近"重写主导代码路径"的量级；Oracle 在当前技术选型下不可行。** 详见 §3–§5。

---

## 1. 现状（基于 evidence，`file:line` 引用）

### 1.1 双池已在 `src/main.rs` 注入，但都是 Postgres

`src/main.rs`：

- `src/main.rs:114` — `let pool = create_pool().await` 创建 **`deadpool_postgres::Pool`**（raw SQL 路径）。
- `src/main.rs:117` — `shared::migrate::run_migrations(&pool)` 用该池跑迁移（见 §1.2）。
- `src/main.rs:348` — `app = app.layer(axum::extract::Extension(pool.clone()))`，为所有提取 `Extension<Pool>` 的 handler 提供 raw-SQL 池。
- `src/main.rs:353-355` — `if let Ok(sea_db) = shared::db::create_sea_orm_pool().await { app = app.layer(Extension(sea_db)); }`，为所有 `*_core_entity` 提取 `Extension<DatabaseConnection>` 的 handler 提供 SeaORM 池。

`crates/shared/src/db.rs` 中两个池构造器均为 Postgres 写死：

- `shared/src/db.rs:21` — `pub async fn create_pool() -> Result<Pool, DbError>`，内部用 `deadpool_postgres::Config` + `NoTls`，默认 `postgres://o2server:password@localhost:5432/oa4rust`。
- `shared/src/db.rs:55` — `pub async fn create_sea_orm_pool() -> Result<DatabaseConnection, DbError>`，用 `sea_orm::Database::connect(ConnectOptions)`，但其底层 feature 为 `sqlx-postgres`（见 `Cargo.toml:233`）。

**关键事实：** 架构上已是"双池并存"，但**没有任何后端抽象层**来选择/切换后端——两个池都只能连 Postgres。

### 1.2 路径 A — raw-SQL（`deadpool_postgres` / `tokio_postgres`）— 主导路径

- 注入面极广：约 **95 处** `.layer(Extension(pool))` 注入点（`grep "Extension(pool)"` 命中 ~95 文件）。
- 获取连接：约 **110 个文件**调用 `pool.get()` 后直接 `client.query / client.execute`，且 SQL 普遍使用 Postgres 的 `$N` 位置占位符。
- `grep '\$\d'`（`**/*.rs`）命中 **~75 文件**——`$N` 位置参数几乎遍布所有业务 crate。
- `grep "NOW()|uuid_generate_v4|gen_random_uuid|RETURNING|ON CONFLICT|ILIKE|::jsonb|::text|::uuid|TIMESTAMPTZ|BYTEA|SERIAL|ARRAY[]|pg_catalog|jsonb"`（`**/*.rs`）命中 **~75 文件**——Postgres 专有关键字/函数/类型转换无处不在。
- 迁移 DDL 全部是 Postgres 方言：`migrations/` 下 **81 个 `.sql` 文件**（正向迁移），其中 `034_create_x_schema_tables.sql`（73 处方言命中）、`036_create_additional_tables.sql`（59 处）等大量使用 `TIMESTAMPTZ`、`SERIAL`、`BYTEA`、`jsonb`、`::` 强转、`uuid-ossp`/`uuid_generate_v4`、`ON CONFLICT`、`RETURNING`、`ARRAY[]`、`ILIKE`、`pg_catalog`。
- 具体耦合示例：
  - `crates/control/src/person.rs:40-45` — `person_flag_clause` 拼接 `((id = ${i}) OR (unique_id = ${i}) OR (name = ${i}))`，直接生成 `$N` 位置参数 SQL。
  - `crates/shared/src/migrate.rs:67` — `CREATE TABLE ... applied_at TIMESTAMPTZ NOT NULL DEFAULT now()`；`migrate.rs:109` — `WHERE version = $1`；`migrate.rs:70` — `INTEGER`。
  - `crates/organization_assemble_control/src/lib.rs:3293` 等巨型 raw-SQL 路由（单文件 22+ 条 `client.query`）。

**抽象边界：** `crates/shared/src/lib.rs` 已有 `RowGet`（`lib.rs:42`）、`ControlClient`（`lib.rs:69`，方法 `ctrl_query/ctrl_query_one/ctrl_query_opt/ctrl_execute`）、`ControlPool`（`lib.rs:163`）、`DynControlPool`（`lib.rs:183`）等 trait。但这些都是**为单测 mock 注入**而设，**类型仍绑定 `tokio_postgres::Row` 与 `&(dyn ToSql + Sync)`**（`lib.rs:50` `impl RowGet for deadpool_postgres::tokio_postgres::Row`；`lib.rs:74` `ToSql` 约束）。**不是后端无关抽象。**

### 1.3 路径 B — SeaORM（`DatabaseConnection` / `Entity::find`）— 部分采用（~22 crate）

- 依赖声明：22 个 `Cargo.toml` 含 `sea-orm`；根 `Cargo.toml:233` 集中 `sea-orm = { version = "1.0", features = ["runtime-tokio-rustls", "sqlx-postgres", "macros", "with-chrono", "with-uuid"] }`。注意 **feature 写死 `sqlx-postgres`**。
- 使用面：`DatabaseConnection` 出现在 **~30 个 `.rs` 文件 / ~22 个 crate**（全部 `*_core_entity` + `program_center_core_entity` + `orm`）；`Entity::find | EntityTrait | ActiveModelTrait | ActiveValue::Set` 命中 ~30 文件。
- handler 写法：`Extension<DatabaseConnection>` + `Entity::find()`（`crates/general_core_entity/src/lib.rs:51,83,115…`），实体用 `#[derive(DeriveEntityModel)] #[sea_orm(table_name = "...")]`（`crates/general_core_entity/src/entities/general_application_dict.rs:3-15`）。
- **不一致点：** `*_core_entity` crate 仍携带 `deadpool_postgres::Pool` 的 router 签名，但参数未使用——`general_core_entity/src/lib.rs:727` `general_core_entity_router(_pool: Pool)`、`lib.rs:758` `pub fn router(pool: deadpool_postgres::Pool)`（`_pool` 带下划线，未使用）。其 SeaORM handler 实际依赖外层 `src/main.rs:354` 注入的 `DatabaseConnection`。即 SeaORM 采用是"实体 + handler 层"，却仍挂在 raw-SQL 的 `Pool` router 签名上，属于半成品迁移。
- **共享 helper 是空桩：** `crates/orm/src/helpers.rs:14` `count_active` 直接 `Ok(0)` 占位；`crates/orm/src/pagination.rs:9` `cursor_list` 返回空。说明 SeaORM 共享层**未落地**，各 crate 仍在手写查询。
- **重复建池：** `program_center_core_entity/src/lib.rs:17` 自行 `let db = shared::db::create_sea_orm_pool().await.ok();`，并在 5 个 handler 内 `router.layer(Extension(conn))`（`agent.rs:186`、`structure.rs:187`、`script.rs:158`、`invoke.rs:169`、`application.rs:173`）。与 `src/main.rs:354` 外层注入重复（内层优先，无害但冗余）。

### 1.4 路径 C — `sqlx`

- `sqlx` 仅在 `crates/shared/Cargo.toml:10`（`features = ["postgres", ...]`）与 `crates/orm/Cargo.toml:8` 声明，且是 **SeaORM 的底层引擎**（`sea-orm` 经 `sqlx` 连库）。**没有任何 crate 直接使用 raw `sqlx` 查询。** 因此 `sqlx` 不是独立路径，而是 SeaORM 的实现细节。

### 1.5 当前抽象边界小结

| 边界 | 位置 | 后端无关？ |
|---|---|---|
| `RowGet` / `ControlClient` / `ControlPool` | `shared/src/lib.rs:42,69,163` | ❌ 绑定 `tokio_postgres::Row` / `ToSql` |
| `Extension<Pool>` vs `Extension<DatabaseConnection>` | 各 handler | ❌ 两种并行 extractor，无统一 repository trait |
| `migrate::run_migrations` | `shared/src/migrate.rs:54` | ❌ 写死 `deadpool_postgres::Pool` + 原始 `.sql` |
| `create_pool` / `create_sea_orm_pool` | `shared/src/db.rs:21,55` | ❌ 均为 Postgres |
| `orm` helpers | `orm/src/helpers.rs:14`, `orm/src/pagination.rs:9` | ⚠️ 空桩，无实现 |

**结论：** 今天**没有任何后端无关的抽象层 / 特性开关 / repository trait**。双池只是"两个 Postgres 连接"，而非"可切换后端"。

---

## 2. 要支持第二后端需要做什么

### 2a. 路径 A（raw-SQL / `deadpool_postgres`）—— 基本不可移植

- **81 个手写 `.sql` 迁移全部是 Postgres 方言**：`TIMESTAMPTZ`/`SERIAL`/`BYTEA`/`jsonb`/`::` 强转/`uuid-ossp`/`ON CONFLICT`/`RETURNING`/`ARRAY[]` 等。换 MySQL/SQLite/Oracle 需整库重移植，并配齐各自迁移框架。
- **~75 个文件的内联 `$N` SQL + Postgres 函数**：
  - 占位符：Postgres `$1` vs MySQL/SQLite `?` vs Oracle `:1`。
  - 函数差异：`NOW()` vs `CURRENT_TIMESTAMP`/`SYSDATE`；`uuid_generate_v4()`/`gen_random_uuid()` vs 原生 `UUID()`；字符串连接 `||` vs `CONCAT()`；`ILIKE` 仅 PG；`RETURNING` MySQL 不支持（需插入后 `SELECT`）；`ON CONFLICT` vs `ON DUPLICATE KEY`/`MERGE`。
  - 类型：`jsonb` vs `JSON`；`::text`/`::uuid` 强转 Oracle/MySQL 无。
- **行映射耦合**：`RowGet` 仅实现于 `tokio_postgres::Row`（每后端需另写实现）。
- **客户端类型耦合**：`tokio_postgres` 是 PG 专有；`deadpool` 虽有多后端（`deadpool-mysql`/`deadpool-sqlite` 经 `sqlx`），但要把 ~110 个 `pool.get()` 调用背后的 `Object`/`Client` 类型整体替换，等于重写主导路径。
- **评估：raw-SQL 路径是最大阻塞点。整体移植近似"重写主导代码路径"。**

### 2b. 路径 B（SeaORM）—— 原则上可移植，但覆盖有限

- SeaORM 后端无关：切换后端 = 改 `ConnectOptions` + 启用对应 feature（`sqlx-mysql` / `sqlx-sqlite` / `sqlx-postgres`）。**但 SeaORM 1.x 不支持 Oracle**——若 Oracle 是目标，此路不通（R7）。
- 仅 ~22/88 crate 用 SeaORM；其余仍是 raw-SQL（R4）。
- 表结构仍由 **raw `.sql` 迁移**通过 `deadpool` 池创建（§1.2），**不是** SeaORM 的 `SchemaManager::create_table`。即 SeaORM crate 也依赖 Postgres 迁移来建表。SeaORM 自带后端无关的迁移（`sea-orm-migration` / `SchemaManager`），但本项目**未使用**。
- 共享 helper 是空桩（R5），各 crate 自写查询，移植时需逐个核对方言差异（即便经过 SeaORM，复杂查询仍可能落到 `Statement::from_string` 等方言相关代码）。
- **评估：SeaORM 路径可移植，前提（i）切 feature、（ii）用 SeaORM 迁移替代 raw `.sql` 建表、（iii）补全空桩 helper；但仅覆盖 ~22 crate。**

### 2c. 路径 C（`sqlx`）—— 非独立，无需单独处理

`sqlx` 仅作为 SeaORM 引擎存在，无独立业务代码，不单独构成路径。

---

## 3. 推荐架构

**总策略：** 以 SeaORM 为"后端无关轴心"，把 raw-SQL 路径视为遗留代码，逐步收敛到统一抽象；过渡期保留双池，长期合并为单池。

1. **引入后端选择 trait + 特性开关**
   - 根 `Cargo.toml` 增加 `[features]`：`backend-postgres`（默认）/ `backend-mysql` / `backend-sqlite`，据此 gating `sea-orm` 的 `sqlx-*` feature 与 `deadpool` 后端。
   - 定义 `DbBackend` 枚举 + 一个**异步查询执行器 trait**（如 `QueryExecutor`：`query / execute / tx`），屏蔽连接类型；提供 PG（`tokio_postgres`）与 MySQL/SQLite（`sqlx`）两套 impl。

2. **迁移体系后端化**
   - 首选：用 `sea-orm-migration` 的 `SchemaManager` 把 81 个 `.sql` 重写为后端无关迁移（覆盖建表/索引/种子）。
   - 备选：按方言分目录 `migrations/postgres/`、`migrations/mysql/`、`migrations/sqlite/`，由 feature 选择；保留 `migrate::run_migrations` 的校验和幂等逻辑，但参数化 DDL 来源。

3. **消除 `$N` 原生 SQL 耦合**
   - 新代码一律走 SeaORM 或参数化 query builder（输出方言相关 SQL）。
   - 存量 raw-SQL crate 分模块迁移到 repository / SeaORM（见 §5 工作量）。

4. **落地 `orm` 共享层**
   - 把 `count_active` / `cursor_list` 等空桩实现为真正的泛型助手（count/filter/paginate），让各 crate 停止手写重复查询，降低后续后端切换面。

5. **清理双池不一致**
   - 去掉 `*_core_entity` 上未使用的 `deadpool_postgres::Pool` router 参数；统一由外层注入所需连接。
   - 移除 `program_center_core_entity` 内部自建 SeaORM 池，复用外层注入（消除 R9 冗余）。

6. **测试基设施后端化**
   - 现有 `MockRow` / `MockClient`（`shared/src/mock_client.rs`、`shared/src/testing.rs`）绑定 PG row 语义；需增加后端无关的 mock 行接口，使单测不依赖 PG 类型。

---

## 4. 风险登记表（Risk Register）

| ID | 风险 | 严重度 | 说明 |
|---|---|---|---|
| R1 | raw-SQL 路径本质不可移植 | 高 | ~60 crate 手写 PG SQL，整体移植≈重写主导路径 |
| R2 | 81 个 Postgres-only 迁移文件 | 高 | DDL 方言差异大，需整库重移植 |
| R3 | `RowGet`/`ControlClient` 绑定 `tokio_postgres::Row` | 中 | 需每后端另写行实现 |
| R4 | SeaORM 仅覆盖 22/88 crate | 高 | 其余走 raw-SQL，后端无关化收益有限 |
| R5 | `orm` helper 为空桩 | 中 | 共享层未落地，crate 各自为政 |
| R6 | `*_core_entity` 仍带 `deadpool Postgres::Pool` 签名 | 中 | 半成品迁移，易误导与出错 |
| R7 | **SeaORM 1.x 不支持 Oracle** | 中 | 若 Oracle 是目标，此架构不可行 |
| R8 | `$N` 位置参数 + PG 函数遍布 ~75 文件 | 高 | 占位符/函数/类型均需方言化 |
| R9 | 双池重复/冗余建池（`program_center_core_entity`） | 低 | 无害但需清理 |
| R10 | 测试 mock 绑定 PG row 语义 | 中 | 后端无关化需改造测试基 |

---

## 5. 工作量估算（Effort Estimate）

> 单位：人周（1 工程师 ≈ 1 人周）。假设 1–2 名熟悉 Rust/SeaORM 的工程师。为**可行性量级**，非精确排期。

| 任务 | 涉及范围 | 估算 |
|---|---|---|
| 全量盘点 raw-SQL 查询与方言点 | ~75 文件 | 1–2 |
| 81 迁移转为 SeaORM 迁移 / 方言分目录 | `migrations/*` | 3–5 |
| 引入后端 trait + feature 开关 + 池选择 | `shared/src`、`Cargo.toml` | 2–3 |
| 迁移 raw-SQL crate（主导 ~60 crate）到 repository/SeaORM | ~60 crate | 12–20（最大头） |
| 落地 `orm` helper + 修复 `*_core_entity` 签名 | `orm`、`*_core_entity` | 2–3 |
| 双后端 CI（MySQL + SQLite 起容器）+ 测试改造 | CI / `testing.rs` | 2–4 |
| **合计** | | **≈ 24–37 人周（约 6–9 个月，1–2 人）** |

**分阶段建议：**
- **Phase 3a（约 6–10 人周）：** 确立 SeaORM 为**新代码标准**；落地 `orm` helper；引入 feature 开关与后端 trait；把现有 SeaORM crate 的建表迁移 SeaORM 化；CI 起 MySQL/SQLite 验证 SeaORM 路径。此阶段即可让 ~22 crate 跑在 MySQL/SQLite 上。
- **Phase 3b（约 18–27 人周）：** 按模块把 raw-SQL crate 逐个迁到 repository/SeaORM；迁移 81 个 `.sql`；清掉双池冗余。
- **Oracle：** 在当前 `sea-orm 1.0` 选型下**不支持**，需另评估 `sqlx` 直连或换 ORM，单独立项。

**总体判断：** 全程多后端是**长期工程**，不在一个 sprint 内完成；最低风险的切入点是先把 SeaORM 路径（~22 crate）在 MySQL/SQLite 上跑通，再渐进消化 raw-SQL 主导路径。

---

## 6. 关键证据索引

- 双池注入：`src/main.rs:114,117,348,353-355`
- 池构造器（均 Postgres）：`shared/src/db.rs:21,55`
- raw-SQL 抽象（绑定 PG）：`shared/src/lib.rs:42,50,69,74,163,183`
- 迁移 DDL（Postgres 方言）：`shared/src/migrate.rs:54,67,109`；`migrations/*.sql`（81 文件，`034_*` 73 方言命中、`036_*` 59 命中）
- `$N` / PG 函数耦合：`crates/control/src/person.rs:40-45`；`grep '\$\d'` ~75 文件；PG 方言 grep ~75 文件
- SeaORM 采用：`crates/general_core_entity/src/lib.rs:51,727,758`；实体 `crates/general_core_entity/src/entities/general_application_dict.rs:3-15`
- 空桩 helper：`crates/orm/src/helpers.rs:14`、`crates/orm/src/pagination.rs:9`
- 冗余建池：`crates/program_center_core_entity/src/lib.rs:17`；`agent.rs:186`、`structure.rs:187`、`script.rs:158`、`invoke.rs:169`、`application.rs:173`
- 根依赖（feature 写死 `sqlx-postgres`）：`Cargo.toml:233`
