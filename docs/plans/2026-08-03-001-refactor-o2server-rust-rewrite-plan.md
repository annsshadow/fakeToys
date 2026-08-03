---
title: refactor: O2OA 后端 Rust 改写
type: refactor
status: active
date: 2026-08-03
origin: docs/brainstorms/2026-08-03-o2server-rust-rewrite-requirements.md
---

# O2OA 后端 Rust 改写

## Summary

以认证模块（`x_organization_assemble_authentication`）为第一个迁移试点，搭建 Rust 单体服务骨架，实现认证模块的 Rust 版本，完成数据迁移和 nginx 路由切换，验证 Rust 替换 Java 后端的端到端可行性。前端通过 nginx 前缀路由保持 URL 不变，零改动接入。

---

## Problem Frame

`o2server` 是 O2OA 的 Java 后端，57+ Maven 模块运行在 Jetty + OpenJPA 栈上，需要持续维护且技术栈锁定在 Java 生态。本次改写以「边运行边替换」策略逐步将模块迁移到 Rust 单体服务，第一个目标是认证模块——它是所有业务模块的基础依赖，替换成功后为后续模块提供可复用的 Rust 侧认证基础设施。

---

## Requirements

**来自上游需求文档的溯源：**

- **R1** — 扫描 `o2server/` 下全部 Maven 模块的 `pom.xml`，产出完整的模块依赖图谱（见 U1）
- **R2** — 基于依赖图谱进行优先级排序，形成模块替换路线图（见 U1）
- **R3** — 每个模块在落地文档中包含标准化卡片（见 U1）
- **R4** — Rust 后端编译为单一进程单体服务（见 U2）
- **R5** — 单体服务内部按模块边界组织代码，具体 crate 划分根据模块依赖分析结果确定（见 U2、U3）
- **R6** — Rust 服务需支持 HTTP 框架、类型安全的数据访问、连接池管理和 schema 迁移管理（见 U2、U3）
- **R7** — Rust 项目采用 Cargo workspace 组织（见 U2）
- **R8** — Rust 服务与 Java 服务独立部署、独立进程（见 U6）
- **R9** — 前端通过 nginx 或等价反向代理按 URL 前缀路由请求，前端无需改动代码（见 U6）
- **R10** — 迁移前后前端感知到的 URL 路径保持不变（见 U6）
- **R11** — Rust 侧使用独立 PostgreSQL 实例，与 Java 侧的 MySQL/H2 物理隔离，运行时无数据互通（见 U2、U5）
- **R12** — 认证模块切换前需有可执行的数据迁移方案（见 U5）
- **R13** — 数据迁移脚本需在切换窗口期执行（切换窗口期定义为：低峰期时间段，具体时段根据业务特性确定），迁移完成后进行数据一致性校验，校验通过后才允许切换流量（见 U5）
- **R14** — 每个模块的替换遵循四步流程，四步之间允许回滚（见 U5、U6）。四步流程定义为：1）数据迁移（U5 执行迁移脚本）→ 2）Rust 服务部署（U2/U3/U4 部署验证）→ 3）nginx 切流（U6 切换路由）→ 4）验证观察（U6 观察期）。
- **R15** — 在 Rust 服务运行期间，Java 服务持续正常运行（见 U6）
- **R16** — `docs/brainstorms/o2server-module-index.md` 作为跟踪清单（见 U1）

**Origin actors:** A1（开发者，单人）、A2（现有 Java 后端）、A3（前端 o2web）

**Origin flows:** F1（模块梳理与优先级排序）、F2（Rust 服务独立开发与测试）、F3（数据迁移与流量切换）

**Origin acceptance examples:** AE1（模块梳理文档产出）、AE2（URL 路径保持不变，nginx 内部路由）、AE3（数据迁移与回滚）

---

## Scope Boundaries

- 不修改前端 `o2web` 的任何代码，仅通过 nginx 前缀路由适配后端切换
- 不在改写期间实现 Java ↔ Rust 的实时数据同步，仅依赖一次性迁移窗口
- 不拆分为微服务，Rust 侧始终以单一进程单体服务运行
- 不包含 Rust 性能压测或与 Java 的基准对比
- 不迁移 `o2web` 前端
- 认证模块首批替换范围限定于 HTTP 接口层，定时任务（Quartz）和后台作业的迁移延后到后续模块

### Deferred to Follow-Up Work

- 非认证模块的 Rust 实现（考勤、流程、CMS 等）
- 定时任务/批处理框架的 Rust 迁移
- 文件存储（本地/NAS/对象存储）的迁移方案
- 前端 `o2web` 现代化
- 性能压测与基准对比

---

## Context & Research

### Relevant Code and Patterns

- **模块依赖结构**：`oa/o2server/pom.xml` 声明 57 个 Maven 模块，按 `x_base_core_project` → `x_*_core_entity` → `x_*_core_express` → `x_*_assemble_control` → `x_*_assemble_surface` → `x_*_service_processing` 的层级排列。认证模块（`x_organization_assemble_control`、`x_organization_assemble_authentication`）位于中层，依赖 `x_organization_core_entity` 和 `x_base_core_project`。
- **前端 API 调用模式**：`oa/o2web/source/x_init/src/common/action.js` 定义通用 `get()` / `post()` 方法，统一从响应 JSON 中提取 `json.data` 字段。前端硬编码的认证相关路径包括 `/jaxrs/secret/check`（密码验证，来自 `x_program_init` 模块）和 `/jaxrs/person/*`（人员信息，来自 `x_organization_assemble_control` 模块）。这意味着 Rust 侧必须保持与 Java 侧完全一致的 JSON 响应结构——Java `ActionResult<T>` 实际包含 9 个字段：`data, type, message, date, spent, size, count, position, prompt`。
- **Java Action 基类**：`oa/o2server/x_base_core_project/src/main/java/com/x/base/core/project/jaxrs/StandardJaxrsAction.java` 定义了统一的异常处理和响应包装逻辑，Rust 侧需要在中间件层复现这一行为。
- **认证模块结构**：`x_organization_assemble_control` 包含 `jaxrs/` 子目录下的多个 Action 类（如 `PersonAction`），使用 `@Path`、`@GET`、`@POST` 注解，继承 `StandardJaxrsAction`。
- **部署方式**：当前系统通过 `oa/o2server/start_linux.sh` 启动，使用 bundled JRE，默认 4GB 堆内存，端口 20020。无 Docker 或 K8s 配置，原生裸机 + systemd 部署。
- **Swagger 覆盖率**：约 20%（11/55 模块），API 文档主要依赖源码阅读和 action JSON 文件。

### Institutional Learnings

仓库中无 `docs/solutions/` 目录。相关历史文档包括：
- `docs/brainstorms/2026-08-03-oa-project-documentation-requirements.md` — OA 项目文档建设需求
- `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md` — 密钥迁移计划（环境变量注入模式可作为参考）

### External References

无外部研究需求。技术选型（Axum + SQLx + PostgreSQL）已在 Key Decisions 中确定，基于仓库内已有文档和代码模式的充分调研。

---

## Key Technical Decisions

- **PostgreSQL 作为 Rust 侧数据库**：与 Java 侧的 MySQL/H2 物理隔离，消除 schema 冲突风险。Rust 侧通过 SQLx 直写 SQL，编译时检查类型安全。
- **响应格式兼容**：Rust 侧必须输出与 Java `ActionResult<T>` 完全一致的 JSON 结构（9 个字段：`data, type, message, date, spent, size, count, position, prompt`），因为前端 `action.js` 直接提取 `json.data` 字段。这是前端零改动的硬性前提。完整字段列表需在 U3 中实现。
- **认证端点优先**：`/jaxrs/secret/check` 和 `/jaxrs/person/*` 是前端硬编码的关键认证路径，首批实现必须覆盖这些端点。
- **Crate 初始划分**：`shared`（公共类型、响应格式、中间件、数据库连接池）+ `auth`（认证模块业务逻辑），预留后续业务域 crate 的扩展空间。
- **迁移策略**：一次性迁移 + nginx 切流回滚，不在过渡期双写。数据迁移脚本在模块梳理完成后、Rust 实现之前开发，与 Rust 实现并行推进。

---

## Open Questions

### Resolved During Planning

- 认证模块是否作为第一个被替换的模块：**是**（用户确认选择 A）
- 技术栈选型：Axum + SQLx + PostgreSQL + deadpool-postgres + sqlx-cli（需求文档 Key Decisions）
- 单体服务架构：Cargo workspace 组织，crate 按业务域划分，运行时单一二进制

### Resolved Before Implementation

- `/jaxrs/secret/*` 端点归属：**纳入首批**（选择 1-A）。`x_program_init` 模块的 `/jaxrs/secret/*` 端点作为认证模块首批的一部分，一并迁移到 Rust。
- ActionResult JSON 字段完整性：**全部实现 9 个字段**（选择 2-A）。Rust 侧 `ActionResult<T>` 包含 `data, type, message, date, spent, size, count, position, prompt`，与 Java 侧完全一致。
- 首批端点范围：**完整覆盖 30+ 个端点**（选择 3-A）。包括 `/jaxrs/authentication/*`（15+ 个）、`/jaxrs/person/*`（30+ 个）、`/jaxrs/secret/*`（来自 x_program_init）。
- 认证端点访问控制策略：**速率限制 + 账户锁定**（选择 4-A）。每 IP 每分钟最多 5 次失败尝试，超过后临时锁定。
- 密码迁移哈希算法兼容：**在 Rust 侧重新实现兼容的哈希算法**（选择 5-B）。分析 Java 的 Crypto 工具类逻辑，在 Rust 侧实现相同的哈希验证。
- 回滚时数据 diverged 处理：**双写过渡**（选择 6-B）。切换初期双写两边，验证后再完全切到 Rust。
- 会话管理策略：**Rust 侧独立实现**（选择 7-B）。不依赖 Java 的 session 机制，Rust 侧实现独立的会话管理。
- 迁移脚本幂等策略：**INSERT ON CONFLICT**（选择 8-B）。基于主键或唯一键去重，增量更新，支持幂等重跑。

### Deferred to Implementation

- 认证模块的 JPA 实体到 Rust SQL schema 的具体映射关系（需在 U1 模块梳理后确定）
- 数据一致性校验的具体方案（行数对比 vs 业务逻辑校验，需结合认证模块数据特性决定）
- 认证模块的定时任务（如有）如何迁移（Scope Boundaries 中已排除，延后处理）

---

## Output Structure

    o2server-rust/
    ├── Cargo.toml                 # workspace 根
    ├── crates/
    │   ├── shared/
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       ├── error.rs       # 统一错误类型
    │   │       ├── response.rs    # ActionResult<T> 序列化
    │   │       ├── middleware.rs  # 异常处理中间件
    │   │       └── db.rs          # 数据库连接池、迁移管理
    │   └── auth/
    │       ├── Cargo.toml
    │       └── src/
    │           ├── lib.rs
    │           ├── secret.rs      # /jaxrs/secret/* 端点
    │           ├── person.rs      # /jaxrs/person/* 端点
    │           └── model.rs       # 认证相关数据模型
    ├── src/
    │   └── main.rs                # 入口，组装路由
    ├── migrations/
    │   └── ...                    # sqlx-cli 迁移文件
    ├── scripts/
    │   ├── migrate_auth_data.py   # 数据迁移脚本
    │   └── verify_auth_data.py    # 一致性校验脚本
    ├── deploy/
    │   └── nginx-auth-routes.conf # nginx 路由配置
    ├── .env.example
    └── README.md

---

## Implementation Units

### U1. 模块梳理与落地文档

**Goal:** 扫描 `oa/o2server/` 下全部 Maven 模块的 `pom.xml`，产出 `docs/brainstorms/o2server-module-index.md`，包含模块依赖图谱、优先级排序和认证模块迁移策略。

**Requirements:** R1, R2, R3, R16

**Dependencies:** 无

**Files:**
- Create: `docs/brainstorms/o2server-module-index.md`

**Approach:**
- 解析 `oa/o2server/pom.xml` 中的 `<modules>` 声明，提取全部 57+ 个模块名
- 对每个模块读取其 `pom.xml`，提取 `<dependencies>` 中的模块依赖关系
- 构建有向依赖图：节点为模块，边为依赖方向
- 按三个维度评分：依赖少（入度低）、业务清晰（已有文档或接口明确）、替换杠杆高（被依赖多）
- 认证模块（`x_organization_*`）作为首批试点，在卡片中明确标注

**Test scenarios:**
- Happy path: 扫描完成所有 57+ 个模块的 `pom.xml`，落地文档包含 57+ 个模块条目，每个条目标注依赖列表和优先级评分
- Edge case: 遇到条件编译或父 POM 继承的依赖时，正确解析并记录

**Verification:**
- 新加入的开发者阅读 `docs/brainstorms/o2server-module-index.md` 后，能理解替换路线图和认证模块的定位

---

### U2. Rust 项目骨架初始化

**Goal:** 创建 Cargo workspace 项目结构，配置 Axum + SQLx + deadpool-postgres + sqlx-cli，建立开发环境。

**Requirements:** R4, R5, R6, R7, R8, R11

**Dependencies:** 无

**Files:**
- Create: `o2server-rust/Cargo.toml`（workspace 根）
- Create: `o2server-rust/crates/shared/Cargo.toml`
- Create: `o2server-rust/crates/auth/Cargo.toml`
- Create: `o2server-rust/src/main.rs`
- Create: `o2server-rust/.env.example`
- Create: `o2server-rust/README.md`
- Create: `o2server-rust/migrations/`（sqlx-cli 初始化）

**Approach:**
- workspace 根 `Cargo.toml` 声明 `shared` 和 `auth` 两个成员 crate
- `main.rs` 作为单一二进制入口，组装各 crate 的路由
- 使用 `dotenvy` 加载环境变量，数据库连接信息从环境变量注入
- `sqlx-cli` 初始化迁移目录，创建初始迁移文件
- 开发环境使用本地 PostgreSQL 实例，连接字符串通过 `.env` 配置

**Technical design:**
```
main.rs
  ├── shared::middleware::trace
  ├── shared::response::wrap ActionResult
  ├── auth::secret::router()
  └── auth::person::router()
```

**Patterns to follow:**
- Cargo workspace 官方文档：https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
- Axum 官方示例：https://github.com/tokio-rs/axum/tree/main/examples

**Test scenarios:**
- Happy path: `cargo build` 成功编译为单一二进制，`cargo run` 启动服务，健康检查端点返回 200
- Edge case: 缺少环境变量时服务启动失败，输出明确错误信息

**Verification:**
- `cargo build` 无错误，输出单一二进制文件
- `cargo test` 所有测试通过
- 服务启动后可通过 curl 访问健康检查端点

---

### U3. 共享基础设施层

**Goal:** 实现 `shared` crate，包含统一响应格式、错误处理中间件、数据库连接池和迁移管理，为后续所有业务 crate 提供基础能力。

**Requirements:** R6, R7

**Dependencies:** U2

**Files:**
- Create: `o2server-rust/crates/shared/src/response.rs`
- Create: `o2server-rust/crates/shared/src/error.rs`
- Create: `o2server-rust/crates/shared/src/middleware.rs`
- Create: `o2server-rust/crates/shared/src/db.rs`
- Test: `o2server-rust/crates/shared/src/response.rs`（同一文件内测试）

**Approach:**
- `response.rs`：定义 `ActionResult<T>` 结构体，序列化为 `{data, type, message, count, size}`，与 Java 侧完全一致
- `error.rs`：定义统一错误类型，映射到适当的 HTTP 状态码
- `middleware.rs`：实现异常处理中间件，捕获 panic 和错误，包装为标准响应格式
- `db.rs`：使用 `deadpool-postgres` 管理连接池，使用 `sqlx::postgres::Postgres` 类型

**Technical design:**
```rust
// response.rs - 与 Java ActionResult<T> 兼容的序列化结构
pub struct ActionResult<T> {
    pub data: T,
    pub r#type: Option<String>,
    pub message: Option<String>,
    pub count: Option<i64>,
    pub size: Option<i64>,
}
```

**Patterns to follow:**
- Axum 中间件模式：https://docs.rs/axum/latest/axum/middleware/index.html
- SQLx + deadpool-postgres 集成：https://docs.rs/sqlx/latest/sqlx/postgres/struct.Pool.html

**Test scenarios:**
- Happy path: `ActionResult::success(data)` 序列化为 `{"data":...,"type":"success",...}`
- Edge case: `ActionResult::error(message)` 序列化时 `data` 为 `null`
- Error path: 中间件捕获 panic 时返回 500 响应，body 为标准错误格式
- Integration: 连接池成功从 PostgreSQL 获取连接并执行简单查询

**Verification:**
- `cargo test -p shared` 全部通过
- 序列化输出与 Java 侧 `ActionResult<T>` 的 JSON 结构一致

---

### U4. 认证模块 Rust 实现

**Goal:** 将 `x_organization_assemble_authentication` 和 `x_organization_assemble_control` 中的认证相关 Action（`/jaxrs/secret/check`、`/jaxrs/person/*`）用 Rust + Axum 重新实现。

**Requirements:** R4, R5

**Dependencies:** U2, U3

**Files:**
- Create: `o2server-rust/crates/auth/src/lib.rs`
- Create: `o2server-rust/crates/auth/src/secret.rs`
- Create: `o2server-rust/crates/auth/src/person.rs`
- Create: `o2server-rust/crates/auth/src/model.rs`
- Test: `o2server-rust/crates/auth/src/secret.rs`
- Test: `o2server-rust/crates/auth/src/person.rs`

**Approach:**
- `secret.rs`：实现 `/jaxrs/secret/check` 端点，接收前端密码验证请求，返回与 Java 侧一致的响应格式
- `person.rs`：实现 `/jaxrs/person/{flag}` 端点，提供人员信息查询
- `model.rs`：定义认证模块的数据模型，与 Java JPA 实体对应
- 路由挂载到 `main.rs`，路径前缀保持 `/jaxrs/` 不变
- 认证逻辑参照 `x_organization_assemble_control` 中的 Java Action 实现

**Technical design:**
```
Axum Router
  ├── POST /jaxrs/secret/check     → secret::check
  ├── GET  /jaxrs/person/{flag}    → person::get
  └── POST /jaxrs/person/{flag}    → person::save
```

**Patterns to follow:**
- `oa/o2server/x_program_init/src/main/java/com/x/program/init/jaxrs/secret/SecretAction.java`（/jaxrs/secret/* 端点的实际位置）
- `oa/o2server/x_organization_assemble_control/src/main/java/com/x/organization/assemble/control/jaxrs/person/PersonAction.java`

**Test scenarios:**
- Happy path: POST `/jaxrs/secret/check` 返回与 Java 侧一致的 `ActionResult` 结构
- Happy path: GET `/jaxrs/person/{flag}` 返回正确的人员数据
- Edge case: 无效的 `{flag}` 参数返回与 Java 侧一致的错误响应
- Error path: 数据库连接失败时返回 500，body 为标准错误格式
- Integration: 响应 JSON 的 `data` 字段被前端 `action.js` 正确提取

**Verification:**
- `cargo test -p auth` 全部通过
- 启动服务后，curl 请求认证端点返回的 JSON 结构与 Java 侧一致
- 前端通过 nginx 路由到 Rust 服务后，登录和人员查询功能正常

---

### U5. 认证数据迁移脚本

**Goal:** 编写数据迁移脚本，将 Java 侧 MySQL 中认证模块相关数据（用户、角色、组织）转换导入 Rust 侧 PostgreSQL，并实现一致性校验。

**Requirements:** R11, R12, R13, R14

**Dependencies:** U1（依赖模块梳理确认数据表范围）、U4（依赖 Rust 侧 schema 定义）

**Files:**
- Create: `o2server-rust/migrations/001_create_auth_tables.sql`
- Create: `o2server-rust/migrations/002_seed_auth_data.sql`
- Create: `o2server-rust/scripts/migrate_auth_data.py`
- Create: `o2server-rust/scripts/verify_auth_data.py`

**Approach:**
- `001_create_auth_tables.sql`：定义 Rust 侧认证模块所需的数据库表（用户、角色、组织），与 Java 侧 JPA 实体对应
- `002_seed_auth_data.sql`：初始数据种子
- `migrate_auth_data.py`：从 MySQL 读取认证相关数据，转换为 PostgreSQL 格式，写入目标库。脚本支持幂等重跑（使用 INSERT ON CONFLICT 或先清空再插入）
- `verify_auth_data.py`：迁移后一致性校验，对比两库的行数、关键字段哈希

**Technical design:**
```
迁移流程:
  MySQL (Java 侧) → Python 脚本 (转换) → PostgreSQL (Rust 侧)
                                    ↓
                            verify_auth_data.py (校验)
```

**Patterns to follow:**
- `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md` 中的幂等迁移模式

**Test scenarios:**
- Happy path: 迁移脚本成功将 100 条用户记录从 MySQL 导入 PostgreSQL，校验脚本确认行数一致
- Edge case: 重复执行迁移脚本，结果与首次一致（幂等性）
- Error path: MySQL 连接失败时脚本明确报错退出
- Integration: 迁移完成后，Rust 服务能从 PostgreSQL 正确读取认证数据

**Verification:**
- 迁移脚本在测试数据集上执行成功，零数据丢失
- 一致性校验脚本输出通过
- 迁移结果在 Rust 服务中可正常查询

---

### U6. nginx 路由配置与流量切换

**Goal:** 配置 nginx 前缀路由，将认证模块的 `/jaxrs/*` 请求路由到 Rust 服务，其余请求继续指向 Java 服务，实现认证模块的流量切换和回滚。

**Requirements:** R9, R10, R14, R15

**Dependencies:** U4（认证模块上线）、U5（数据迁移完成）

**Files:**
- Create: `o2server-rust/deploy/nginx-auth-routes.conf`
- Modify: （系统 nginx 配置，路径取决于部署环境）

**Approach:**
- 在 nginx 配置中新增认证模块的路由规则：`/jaxrs/secret/*` 和 `/jaxrs/person/*` 转发到 Rust 服务端口
- 其余 `/jaxrs/*` 路径继续转发到 Java 服务（端口 20020）
- 配置健康检查：Rust 服务健康检查失败时自动回滚到 Java 服务
- 提供回滚脚本：一键将认证路径切回 Java 服务

**Technical design:**
```nginx
# 认证模块 → Rust 服务
location ~ ^/jaxrs/(secret|person)/ {
    proxy_pass http://127.0.0.1:RUST_PORT;
    proxy_set_header Host $host;
}

# 其他模块 → Java 服务
location /jaxrs/ {
    proxy_pass http://127.0.0.1:20020;
}
```

**Patterns to follow:**
- `docs/oa/deployment/linux.md` 中的 nginx 配置示例
- `oa/o2server/start_linux.sh` 中的系统启动脚本模式

**Test scenarios:**
- Happy path: 请求 `/jaxrs/secret/check` 被路由到 Rust 服务，返回正确响应
- Happy path: 请求 `/jaxrs/cms/*`（未迁移模块）被路由到 Java 服务，返回正确响应
- Error path: Rust 服务健康检查失败时，nginx 返回 502 或可配置的回退行为
- Integration: 前端通过浏览器访问认证页面，功能正常，URL 路径不变

**Verification:**
- nginx 配置加载成功，无语法错误
- 认证模块流量切到 Rust 服务后，前端登录和人员查询功能正常
- 回滚到 Java 服务后，功能恢复正常

---

## System-Wide Impact

- **Interaction graph:** Rust 认证服务接收前端直接发起的认证请求（登录、密码验证、人员查询）；Java 服务继续处理所有未迁移模块的请求。两个服务在认证模块切换期间互不调用。
- **Error propagation:** Rust 服务中间件层统一处理 panic 和错误，输出与 Java 侧一致的 `ActionResult` 格式，前端无需区分后端来源。
- **State lifecycle risks:** 认证模块数据在切换窗口期从 MySQL 迁移到 PostgreSQL，迁移完成后 Java 侧停止写入。回滚时通过 nginx 路由切回 Java，Java 侧数据保持原样。
- **API surface parity:** 认证模块首批覆盖的 API 路径（`/jaxrs/secret/check`、`/jaxrs/person/*`）必须与 Java 侧的请求方法、响应结构和错误码完全一致。
- **Unchanged invariants:** 未迁移模块的所有 API 路径和行为不受影响，继续由 Java 服务处理。

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| 前端 `action.js` 对响应格式有隐式假设（如 `data` 字段非 null 时的处理逻辑） | Medium | High | 在 U4 测试阶段用真实前端页面验证响应格式，与 Java 侧输出逐字段对比 |
| 认证模块依赖的数据库表涉及其他模块的外键约束，迁移时触发完整性错误 | Medium | High | 在 U1 模块梳理阶段识别认证模块的所有关联表和约束；U5 迁移脚本中处理外键顺序 |
| nginx 路由规则与现有配置冲突，导致认证模块或其他模块不可用 | Low | High | U6 实施前备份 nginx 配置，先在 staging 环境验证路由规则 |
| Rust 新手对 Axum + SQLx 的掌握不足，导致开发效率低于预期 | Medium | Medium | 参考 Axum 官方示例和仓库内已有的 Java Action 实现；先实现最简端点再逐步完善 |
| PostgreSQL schema 与 Java JPA 实体定义出现 drift | Medium | Medium | U1 模块梳理阶段提取 JPA 实体的字段定义；U5 迁移前对比两套 schema |
| Java 侧在迁移期间发生业务变更，导致 Rust 实现与 Java 行为不一致 | Low | Medium | 在 U4 开发阶段锁定 Java 源码版本，迁移完成后 Java 侧认证模块停止变更 |

---

## Documentation / Operational Notes

- 模块梳理文档（`docs/brainstorms/o2server-module-index.md`）是所有后续模块替换的跟踪依据，每完成一个模块后更新状态
- Rust 服务部署方式：独立 systemd service，启动脚本参考 `oa/o2server/start_linux.sh` 模式，但使用 `cargo run --release` 或预编译二进制
- 四步流程：1）数据迁移 → 2）Rust 服务部署 → 3）nginx 切流 → 4）验证观察。每步之间允许回滚。
- 切换窗口期：低峰期时间段（具体时段根据业务特性确定），操作流程为：低峰期 → 执行迁移脚本 → 一致性校验 → nginx 切流 → 观察 30 分钟 → 确认无误。
- 回滚流程：nginx 路由切回 Java 服务，Rust 服务停止接收流量但不终止（便于调试）。

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-03-o2server-rust-rewrite-requirements.md](../brainstorms/2026-08-03-o2server-rust-rewrite-requirements.md)
- Related code: `oa/o2server/pom.xml`, `oa/o2server/x_organization_assemble_control/`, `oa/o2web/source/x_init/src/common/action.js`
- Related docs: `docs/oa/architecture.md`, `docs/oa/deployment/linux.md`
- Related plans: `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md`
