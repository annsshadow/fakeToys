---
title: feat: Comprehensive oa4rust advancement
type: feat
status: superseded
date: 2026-08-05
<!-- Superseded by: docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md -->
origin: docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md
---

# oa4rust 全面推进计划

## Summary

全面推进 oa4rust Rust 后端迁移：按价值优先的执行顺序将所有 80 个 workspace crate 接入 `main.rs` 建立完整路由框架（桩代码端点标记 TODO，仍为桩、不宣称完整能力），将组织控制、个人信息与系统初始化模块的核心业务端点完善为真实数据库实现，并完善认证模块（验证码、OAuth、认证与限流中间件），使前端可经对应前缀调用已接入模块的端点。

---

## Problem Frame

O2OA 后端当前 100% 基于 Java（Maven 55+ 模块），长期技术栈锁定在 Java 生态。oa4rust 项目已将全部 55 个 Java 模块映射为 80 个 Rust crate，编译通过且测试通过，但只有 4 个 crate 接入运行时入口（`main.rs`）。其余 76 个 crate 中仅部分（如 control、personal_extend）已实现真实业务逻辑并经过测试，大部分仍为返回占位数据或空列表的桩代码，均未接入运行时，前端无法调用、无法验证行为一致性、无法追踪迁移进度。认证模块存在占位实现（验证码返回占位图、OAuth 返回示例 URL），CMS 控制模块仅返回空列表。团队无法在 Rust 后端上推进任何实际工作，迁移停滞的代价是持续维护 Java 运行时和技术栈锁定。

---

## Requirements

**路由框架接入**
- R1. 所有 80 个 workspace crate 分两阶段接入 `main.rs`：第一阶段接入已有真实实现的 crate（control、personal_extend 等），第二阶段接入其余桩代码 crate
- R2. 桩代码端点（返回占位数据或空列表）必须在代码中标记 `TODO: [module] - real implementation needed`，所有 TODO 标记在 sprint 规划中 review 并分配优先级
- R3. 已存在真实实现的 crate（control、personal_extend 等）优先接入，桩代码端点不得阻塞已实现端点的暴露

**核心业务端点真实化**
- R4. 组织控制模块（人员、单位、角色、用户组）的 CRUD 端点返回真实数据库数据
- R5. 个人信息模块（个人信息查询/更新、密码管理、头像管理）的端点返回真实数据
- R6. 系统初始化模块（密码设置、初始化检查）的端点返回真实数据

**认证模块完善**
- R7. 验证码端点返回真正的验证码图片（本地生成，集成 captcha 库）
- R8. OAuth 端点实现第三方登录对接（微信、钉钉）
- R9. 认证流程完整可用（登录 → 会话 → 登出 → 刷新令牌）

**安全需求**
- R12. 所有端点强制认证（登录用户方可访问），健康检查端点及认证前置端点除外：`/jaxrs/authentication`（POST 登录 / DELETE 登出 / GET 当前用户）、`/jaxrs/authentication/captcha`、`/jaxrs/authentication/captcha/width/{width}/height/{height}`（验证码）、`/jaxrs/authentication/code`、`/jaxrs/authentication/code/credential/{credential}`（短信验证码发送与登录）、`/jaxrs/authentication/bind`、`/jaxrs/authentication/bind/meta/{meta}`（扫码登录）、`/jaxrs/authentication/oauth/list`、`/jaxrs/authentication/oauth/qywx/config`、`/jaxrs/authentication/oauth/dingding/config`、`/jaxrs/authentication/oauth/name/{name}`、`/jaxrs/authentication/oauth/login/qywx/code/{code}`、`/jaxrs/authentication/oauth/login/dingding/code/{code}`、`/jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}`、`/jaxrs/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}`（OAuth 授权与回调）、`/jaxrs/secret/captcha/verify`（验证码校验）、`/jaxrs/reset/check/credential/{credential}`、`/jaxrs/reset/check/password/{password}`、`/jaxrs/reset/code/credential/{credential}`、`/jaxrs/reset`、`/jaxrs/reset/password/anonymous`（密码重置流程）、`/jaxrs/secret/set`（仅系统未初始化时）；豁免按精确路径匹配，不使用 `*` 前缀通配（OAuth 与重置端点逐条精确列举）
- R13. 所有输入端点进行参数验证（类型、长度、格式），拒绝无效输入
- R14. 认证接口速率限制（10次/分钟/IP），普通接口速率限制（100次/分钟/IP）
- R15. 所有响应强制 HTTPS（TLS 1.2+），生产环境返回安全的响应头（HSTS, X-Content-Type-Options）

**迁移策略**
- R10. 沿用 Strangler Fig 渐进式迁移策略：Rust 与 Java 并行运行，通过 nginx 反向代理按 URL 前缀路由，逐步切换流量
- R11. 迁移进度通过 `docs/brainstorms/oa4rust-migration-status.md` 模块跟踪清单持续反映，每个模块标记为待迁移 / 已接入（桩代码） / 真实化中 / 已完成

**Actor context（自源文档内容推导，源文档无独立 Actors 章节）:** A1（开发者，单人）、A2（现有 Java 后端）、A3（前端 o2web）
**Flow context（自源文档内容推导）:** F1（模块梳理与优先级排序）、F2（Rust 服务独立开发与测试）、F3（数据迁移与流量切换）
**Origin acceptance examples:** AE1（Covers R4, R5, R6 — CRUD 端点返回真实数据库数据而非占位响应）、AE2（Covers R7 — 验证码端点返回生成的验证码图片而非占位数据）、AE3（Covers R8 — OAuth 端点返回有效的第三方授权 URL 而非示例 URL）

---

## Scope Boundaries

- 不修改前端 `o2web` 的任何代码，仅通过 URL 前缀路由适配后端切换
- 不在改写期间实现 Java ↔ Rust 的实时数据同步，仅依赖一次性迁移窗口
- 不拆分为微服务，Rust 侧始终以单一进程单体服务运行
- 不包含 Rust 性能压测或与 Java 的基准对比
- 不迁移 `o2web` 前端，该部分保持现状
- 不处理 Java 服务的下线和回滚脚本（后续阶段）
- 不进行数据库 schema 变更或迁移脚本编写（沿用现有计划中的 schema；例外：本计划新增 migration 005，在 U1 创建并应用，补齐 U1/U3/U4 所需的 `auth_group` 表及 `deleted_at`、`avatar`、`icon` 列）

### Deferred to Follow-Up Work

- Java 服务的下线和完全切换（后续阶段）
- 性能压测与基准对比
- 文件存储（本地/NAS/对象存储）的迁移方案
- 定时任务/批处理框架的 Rust 迁移

---

## Context & Research

### Relevant Code and Patterns

- **当前 main.rs 接线**：`oa4rust/src/main.rs` 已接入 4 个 crate：`shared`、`auth`、`personal`、`cms_control`，使用 Axum Router 组装
- **已实现真实逻辑的 crate**：`control`（含 group.rs、person.rs、role.rs、unit.rs，均含数据库查询）、`personal_extend`（含 avatar.rs、password.rs、personal.rs）、`personal`（含 password.rs、reset.rs）、`auth`（含完整的 session 管理、登录/登出逻辑）
- **桩代码 crate**：`cms_control`（仅返回空列表）、`message`（返回硬编码测试数据）、`portal`（返回硬编码测试数据）、`express`（返回占位数据）、`program_init`（secret 端点使用内存状态而非数据库）
- **共享基础设施**：`shared` crate 提供 `ActionResult<T>` 响应格式（9 个字段：data, type, message, date, spent, size, count, position, prompt）、`AppError` 错误类型、trace 中间件、数据库连接池
- **技术栈**：Axum 0.7.9（Cargo.lock 当前锁定，仅支持 `:param`；约 19 个 crate 的存量路由仍使用 `:param` 语法，本计划 U1 阶段升级至 Axum 0.8 并将全部路由统一转换为 `{param}` 语法，保证参数化路由在 0.8 下可匹配）+ SQLx 0.7 + PostgreSQL + deadpool-postgres + sqlx-cli + tokio + tracing
- **Cargo workspace**：80 个 crate 成员，`rust-version = "1.75"`，`edition = "2021"`

### Institutional Learnings

- 现有计划 `docs/plans/2026-08-03-001-refactor-o2server-rust-rewrite-plan.md` 提供了 auth 模块迁移的详细设计，可作为参考
- 密钥迁移计划 `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md` 提供了幂等迁移模式参考
- 仓库中无 `docs/solutions/` 目录

### External References

- Axum 官方示例：https://github.com/tokio-rs/axum/tree/main/examples
- Cargo workspace 文档：https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
- SQLx + deadpool-postgres 集成：https://docs.rs/sqlx/latest/sqlx/postgres/struct.Pool.html

---

## Key Technical Decisions

- **执行顺序（价值优先）**：U1 → U3 → U2 → U4/U5 → U6 → U7；单人开发下按依赖图串行推进。U2 提前至 U4 之前（U4 依赖 U2 完成 `program_init` 接入 `main.rs`，否则 U4 的 `GET /jaxrs/secret/check`/`POST /jaxrs/secret/set` 无法端到端验证；U4 不再依赖 U3 的查询模式，改为独立实现 personal 查询）
- **桩代码标记 TODO 而非删除**：保留桩代码作为后续实现的明确占位，避免遗漏端点（see origin）
- **已实现模块优先接入**：control、personal_extend 等已有真实实现的 crate 优先接入 main.rs，让团队尽快看到可用 API（see origin）
- **Rust 为唯一技术选项**：无替代方案（Go 等），迁移必须使用 Rust（see origin）
- **沿用 Strangler Fig 迁移策略**：双轨运行、按模块切换、灰度验证，已在前期计划中验证（see origin）
- **响应格式兼容**：Rust 侧必须输出与 Java `ActionResult<T>` 完全一致的 JSON 结构（9 个字段），因为前端 `action.js` 直接提取 `json.data` 字段。这是前端零改动的硬性前提
- **认证端点路径保持 `/jaxrs/` 前缀**：与 Java 侧完全一致，确保前端无需改动。认证端点以 Java `x_organization_assemble_authentication` 的 `AuthenticationAction`/`OauthAction` 及前端 `xAction` 服务契约为准（登录/登出/当前用户均为 `POST`/`DELETE`/`GET /jaxrs/authentication`，验证码为 `captcha/width/{width}/height/{height}`，OAuth 为 `/jaxrs/authentication/oauth/...`），Rust 侧现有 `login`/`logout`/`who`/`refresh` 等自造路径需在 U1/U5 中对齐为上述契约路径
- **会话互认**：双轨共存期间 Rust 与 Java 的会话 token 需可互认（共享 JWT 密钥或共享会话存储），避免用户经 Rust 登录后访问未迁移模块时收到 401；或在认证模块完全切换前将登录保留在 Java 侧。决策截止点：U5（认证模块真实化完成）前必须确定（三选一：共享 JWT 密钥 / 共享会话存储 / 登录保留在 Java 侧），并在 U6 验证；在确定前，Rust 侧 `validate_session` 仅信任 Rust 自身签发的会话，Java 登录用户访问 Rust 前缀模块的互通价值取决于该决策，U1-U5 期间不得宣称该互通可用
- **认证与限流中间件在 U1 挂载**：认证与速率限制中间件随 U1 的路由结构一并实现并挂载，R12 从首个端点暴露起生效，U6 仅做加固与专项验证（避免端点先于认证暴露）
- **axum 升级至 0.8**：Cargo.lock 当前锁定 axum 0.7.9（仅支持 `:param`，`{param}` 路由不匹配返回 404）；存量约 19 个 crate 的路由仍使用 `:param` 语法。U1 前统一升级 workspace 至 axum 0.8 并 `cargo update`，同时将全部存量路由的 `:param` 转换为 `{param}` 语法，U1 冒烟测试覆盖参数化端点，避免路由静默 404

---

## Open Questions

### Resolved During Planning

- [Affects R1] 76 个未集成 crate 是否一次性全部接入 `main.rs`，还是按批次分批接入？→ 已改为分阶段接入：第一阶段接入已有真实实现的 crate，第二阶段接入其余桩代码 crate
- [Affects R7] 验证码图片生成采用第三方服务集成还是本地实现？→ 已指定本地生成（集成 captcha 库）
- [Affects R8] OAuth 第三方登录支持哪些平台？→ 已指定微信和钉钉
- [Affects R4-R6] 组织控制模块（control）的 20 个 CRUD 端点中，哪些在 Java 侧有复杂业务逻辑需要在 Rust 侧还原？→ 已在 U3 中识别，优先实现人员 CRUD（list/create/get/update/delete），单位和角色的 CRUD 同步实现
- [Affects R10] 双轨运行期间 Rust 与 Java 模块间的跨服务调用如何处理？→ 暂不处理跨服务调用，Rust 服务独立运行，Java 服务继续处理未迁移模块的请求
- [Affects R11] 模块跟踪清单的格式和更新频率如何定义？→ 在 U7 中定义跟踪清单格式，每个模块标记为待迁移 / 已接入（桩代码） / 真实化中 / 已完成

### Deferred to Implementation

- 认证模块的 JPA 实体到 Rust SQL schema 的具体映射关系（需在 U5 实施时确定）
- 数据一致性校验的具体方案（行数对比 vs 业务逻辑校验，需结合认证模块数据特性决定）
- 认证模块的定时任务（如有）如何迁移（Scope Boundaries 中已排除，延后处理）

---

## Output Structure

    oa4rust/
    ├── Cargo.toml                 # workspace 根（已有）
    ├── src/
    │   └── main.rs                # 入口，组装所有 crate 路由（需更新）
    ├── crates/
    │   ├── shared/                # 共享基础设施（已有）
    │   ├── auth/                  # 认证模块（已有，需完善）
    │   ├── personal/              # 个人信息（已有，需完善）
    │   ├── personal_extend/       # 个人扩展（已有，需完善）
    │   ├── control/               # 组织控制（已有，需完善）
    │   ├── cms_control/           # CMS 控制（已有，桩代码）
    │   ├── program_init/          # 系统初始化（已有，桩代码）
    │   ├── message/               # 消息（已有，桩代码）
    │   ├── portal/                # 门户（已有，桩代码）
    │   ├── express/               # 快递（已有，桩代码）
    │   ├── ...                    # 其余 70+ 个 crate（已有，桩代码）
    │   └── ...
    ├── docs/
    │   └── brainstorms/
    │       └── oa4rust-migration-status.md  # 迁移进度跟踪清单（需创建）
    └── ...

---

## Implementation Units

### U1. 框架接入：已有真实实现的 crate 接入 main.rs

**Goal:** 将所有已有真实实现的 crate（control、personal_extend、personal、auth）接入 `main.rs`，建立完整的路由框架，使前端可通过对应前缀调用已实现的 API。

**Requirements:** R1, R3

**Dependencies:** None

**Files:**
- Modify: `oa4rust/src/main.rs`
- Modify: `oa4rust/crates/shared/src/middleware.rs`（新增认证与速率限制中间件）
- Modify: `oa4rust/crates/shared/src/router.rs`（挂载认证与限流中间件）
- Modify: `oa4rust/crates/auth/src/lib.rs`（router 函数签名改造为接收注入的 `SessionManager`/`RateLimiter`，支持单一实例）
- Modify: `oa4rust/crates/personal_extend/src/routes.rs`（同上，router 签名改造）
- Modify: `oa4rust/crates/control/src/routes.rs`（移除重复 `/health`；person/unit/role/group 路由契约对齐在 U3，本单元仅处理去重）
- Create: `oa4rust/migrations/005_org_tables.sql`（新增 `auth_group` 表，为 `auth_person`/`auth_unit`/`auth_role` 补充 `deleted_at` 列、为 `auth_person` 补充 `avatar`、`icon` 列；原属 U3，因已接线端点依赖该 schema 而移入 U1）
- Test: `oa4rust/src/main.rs`（集成测试）

**Approach:**
- 将 `control`、`personal_extend`、`personal`、`auth` 四个已实现真实逻辑的 crate 全部接入 `main.rs`
- 保持现有 `shared`、`auth`、`personal`、`cms_control` 的接线不变
- 新增 `control` 和 `personal_extend` 的路由挂载
- 所有路由保持 `/jaxrs/` 前缀，与 Java 侧一致
- 路由去重：`control` 与 `auth` 重复注册的 `/jaxrs/person/list`、`/jaxrs/unit/list`、`/jaxrs/role/list`、`/jaxrs/group/list` 统一由 `control` 提供（移除 `auth` 侧重复）；`GET /jaxrs/person/{flag}`（`auth` 注册）与 `GET /jaxrs/person/{id}`（`control` 注册）规范化路径相同（axum 0.8 `{param}` 仅按位置匹配，参数名不影响匹配，合并时同 path+method 冲突 panic），统一由 `control` 提供并移除 `auth` 侧重复；`control` 自身重复注册的 `/health` 仅保留 `shared::router()` 一处；`/jaxrs/secret/check` 与 `/jaxrs/secret/set` 唯一归属 `program_init`（真实化在 U4），U2 接入 `express`、`program_init` 时不再在 `auth` 侧注册重复路由
- 认证与速率限制中间件随 U1 实现并挂载到全部路由（R12 从首个端点暴露起生效），`main.rs` 构造单一 `SessionManager`/`RateLimiter` 实例注入各 router，避免认证与限流状态分裂
- 声明 schema 迁移执行步骤：U1 完成后执行 `sqlx migrate run`，按序应用 migration 001-005
- 核对所有已接入端点的路径参数语法：axum 0.8 使用 `{param}`（升级前为 `:param`），参数化端点纳入冒烟测试，避免路由静默 404
- 启动服务后验证所有已接入端点可正常响应（含启动冒烟测试，确认无 axum 路由冲突 panic）

**Patterns to follow:**
- 现有 `main.rs` 的路由组装模式：`Router::new().merge(...).merge(...)`
- `shared::router()` 提供基础中间件和健康检查

**Test scenarios:**
- Happy path: `cargo build` 成功编译，`cargo run` 启动服务，所有已接入端点返回 200
- Happy path: GET `/jaxrs/unit/list` 返回 `ActionResult` 结构（200 + 9 字段），真实数据断言在 U3 验证
- Happy path: GET `/jaxrs/person/{flag}` 参数化路由可匹配并返回 `ActionResult` 而非 404（验证 axum 路径参数语法）
- Happy path: PUT `/jaxrs/person/password` 请求可到达处理器并返回 `ActionResult`
- Integration: 所有已接入端点的响应 JSON 结构与 Java 侧 `ActionResult<T>` 一致
- Integration: 启动冒烟覆盖认证中间件豁免端点（`POST /jaxrs/authentication`、`GET /jaxrs/authentication/captcha/width/{width}/height/{height}`、`GET /jaxrs/reset/code/credential/{credential}` 等）可绕过认证正常访问

**Verification:**
- `cargo build` 无错误
- `cargo test` 所有测试通过
- `sqlx migrate run` 成功应用 migration 001-005
- 启动服务后 curl 调用已接入端点返回 `ActionResult` 响应，参数化端点（`/jaxrs/person/{flag}`）非 404
- 响应 JSON 结构与 Java 侧一致（9 个字段的 ActionResult）

---

### U2. 框架接入：桩代码 crate 接入 main.rs 并标记 TODO

**Goal:** 将 U1 接入后剩余的桩代码 crate（基线 76 个，U1 已接入 control、personal_extend，实际剩余 74 个）接入 `main.rs`，所有桩代码端点标记 `TODO: [module] - real implementation needed`，建立完整的路由可见性。

**Requirements:** R1, R2, R3

**Dependencies:** U1

**Files:**
- Modify: `oa4rust/src/main.rs`
- Modify: 74 个桩代码 crate 的 `lib.rs` 或 `routes.rs`（添加 TODO 标记）
- Modify: `oa4rust/crates/cms_express/src/routes.rs`（移除与 `cms_control` 重复的 `GET /jaxrs/cms/view/list/all` 注册）
- Modify: 使用 `:param` 旧语法的约 19 个 crate 的路由文件（统一转换为 axum 0.8 `{param}` 语法）

**Approach:**
- 按业务域分组接入桩代码 crate：
  - 消息域：`message`、`message_core_entity`、`message_assemble_communicate`
  - 门户域：`portal`、`portal_assemble_surface`、`portal_assemble_designer`、`portal_core_entity`
  - 快递域：`express`
  - 系统初始化：`program_init`
  - CMS 域：`cms_express`、`cms_assemble_control`、`cms_core_entity`、`cms_core_express`
  - 流程域：`process_express`、`process_surface`、`process_bam`、`process_designer`、`processplatform_*`
  - 查询域：`query_express`、`query_service`、`query_service_processing`、`query_assemble_*`、`query_core_*`
  - 文件域：`file`、`file_assemble_control`、`file_core_entity`
  - AI 域：`ai`、`ai_assemble_control`、`ai_core_entity`
  - 考勤域：`attendance`、`attendance_assemble_control`、`attendance_core_entity`
  - 关联域：`correlation`、`correlation_core_*`、`correlation_service_processing`
  - 通用域：`general`、`general_assemble_control`、`general_core_entity`
  - 热图域：`hotpic`、`hotpic_assemble_control`、`hotpic_core_entity`
  - 推送域：`jpush`、`jpush_assemble_control`、`jpush_core_entity`
  - 会议域：`meeting`、`meeting_assemble_control`、`meeting_core_entity`
  - 思维域：`mind`、`mind_assemble_control`、`mind_core_entity`
  - 组织域：`organization_assemble_express`、`organization_core_*`
  - 组件域：`component`、`component_assemble_control`、`component_core_entity`
  - 日历域：`calendar`、`calendar_assemble_control`、`calendar_core_entity`
  - 论坛域：`bbs`、`bbs_assemble_control`、`bbs_core_entity`
  - 程序中心：`program_center`、`program_center_core_entity`
  - 控制台：`console`
  - 基础域：`base`
- 以上分组为业务域示意并覆盖剩余 74 个待接入的桩代码 crate（以 Cargo workspace 成员清单为最终依据）
- 已接入的桩代码 crate（如 `cms_control`）同步补充 TODO 标记，保持全仓标记一致
- 每个桩代码端点添加 `TODO: [module] - real implementation needed` 注释
- 桩代码端点返回空的 `ActionResult::success(Value::Null)` 或空列表
- 路径参数语法核对与转换：全仓搜索 `:param` 旧语法（grep 验证约 19 个 crate 的存量路由仍使用 `:param`，见 Context & Research 技术栈条目），统一转换为 `{param}` 或纳入升级步骤，避免 axum 0.8 下 `:param` 路由静默 404
- 全量路由冲突扫描：U2 完成后执行启动冒烟与路由扫描，确认无同 path+method 的重复注册；当前代码中 `GET /jaxrs/cms/view/list/all` 由 `cms_control` 与 `cms_express` 重复注册（axum 合并 panic），须在接入时移除 `cms_express` 侧注册（cms_express 路由文件列入本单元 Files 修改清单），其余冲突以 U1 去重规则为准

**Patterns to follow:**
- 现有桩代码 crate 的模式（如 `cms_control`、`message`、`portal`）
- TODO 标记格式：`// TODO: [module] - real implementation needed`

**Test scenarios:**
- Happy path: `cargo build` 成功编译所有 80 个 crate
- Happy path: 启动服务后，所有桩代码端点返回 `ActionResult` 格式响应（data 为 null 或空数组）
- Happy path: 源码层验证——每个桩代码 crate 的端点代码包含 `TODO: [module] - real implementation needed` 注释（grep 检查）
- Integration: 已实现端点（来自 U1）不受桩代码接入影响，仍返回真实数据

**Verification:**
- `cargo build` 无错误，所有 80 个 crate 编译通过
- `cargo test` 所有测试通过
- 启动服务后，桩代码端点返回 `ActionResult` 格式响应
- 每个桩代码端点代码中包含 TODO 标记

---

### U3. 核心业务端点真实化：组织控制模块

**Goal:** 完善组织控制模块（control）已实现的 CRUD 端点，补齐分页、参数验证与错误处理，确保返回真实数据库数据且与 Java 侧行为一致。

**Requirements:** R4

**Dependencies:** U1

**Files:**
- Modify: `oa4rust/crates/control/src/person.rs`
- Modify: `oa4rust/crates/control/src/group.rs`
- Modify: `oa4rust/crates/control/src/role.rs`
- Modify: `oa4rust/crates/control/src/unit.rs`
- Test: `oa4rust/crates/control/src/tests.rs`

**Approach:**
- `person.rs`：实现完整的人员 CRUD（create/get/update/delete）与游标分页列表，路径与 Java `PersonAction` 契约一致：`POST /jaxrs/person`（create）、`GET /jaxrs/person/{flag}`、`PUT /jaxrs/person/{flag}`、`DELETE /jaxrs/person/{flag}`、`GET /jaxrs/person/list/{flag}/next/{count}`、`GET /jaxrs/person/list/{flag}/prev/{count}`；当前 Rust 自造的 `/jaxrs/person/create`、`/jaxrs/person/{id}/update`、`/jaxrs/person/{id}/delete`、`/jaxrs/person/list` 路径需移除，由契约路径取代（前端 `xAction` 按契约调用，自造路径不可达）；所有查询操作针对 `auth_person` 表
- `group.rs`：实现用户组 CRUD（create/get/update/delete）与游标分页列表，路径对齐 Java `GroupAction`：`POST /jaxrs/group`、`GET/PUT/DELETE /jaxrs/group/{flag}`、`GET /jaxrs/group/list/{flag}/next/{count}`，针对 `auth_group` 表
- `role.rs`：实现角色 CRUD（create/get/update/delete）与游标分页列表，路径对齐 Java `RoleAction`：`POST /jaxrs/role`、`GET/PUT/DELETE /jaxrs/role/{flag}`、`GET /jaxrs/role/list/{flag}/next/{count}`，针对 `auth_role` 表
- `unit.rs`：实现单位 CRUD（create/get/update/delete）与游标分页列表，路径对齐 Java `UnitAction`：`POST /jaxrs/unit`、`GET/PUT/DELETE /jaxrs/unit/{flag}`、`GET /jaxrs/unit/list/{flag}/next/{count}`，针对 `auth_unit` 表；`POST /jaxrs/unit` 的路径模式与 `GET /jaxrs/unit/list` 不冲突（方法不同）；person 侧 `GET /jaxrs/person/list` 与 `GET /jaxrs/person` 需与 personal 模块注册协调，避免路由冲突
- 所有端点使用 `ActionResult<T>` 格式返回，与 Java 侧一致
- 错误契约与 Java 侧对齐：业务错误返回 HTTP 200 + `type=error` + 非空 `message`（前端 `action.js` 依赖），HTTP 状态码仅用于传输层错误（认证 401、限流 429 等）；实现时以实际 Java Action 行为为准
- 所有写操作（create/update/delete）包含参数验证（类型、长度、格式）
- 分页采用 Java 契约的游标分页（`{flag}/next/{count}`、`{flag}/prev/{count}`），参数 `flag` 为上一页末条标识、`count` 为返回条数
- 人员创建（`POST /jaxrs/person`）写入的密码哈希统一使用 U4 的双算法兼容方案（新写入 bcrypt 带方案前缀，兼容既有 MD5/DES 校验），禁止在 U3 单独引入 MD5 新写入（保持全仓密码哈希策略一致）

**Patterns to follow:**
- 现有 `control/src/person.rs` 的数据库查询模式（使用 `deadpool_postgres::Pool` 和 SQLx）
- `shared::response::ActionResult` 的成功/错误响应格式

**Test scenarios:**
- Happy path: GET `/jaxrs/person/list/{flag}/next/{count}` 返回数据库中对应位置的人员列表（游标分页）
- Happy path: POST `/jaxrs/person` 创建新人员后返回包含 id 的成功响应
- Happy path: GET `/jaxrs/person/{flag}` 返回指定人员的完整信息
- Happy path: PUT `/jaxrs/person/{flag}` 更新人员信息后返回成功
- Happy path: DELETE `/jaxrs/person/{flag}` 软删除人员后返回成功
- Edge case: GET `/jaxrs/person/{flag}` 对不存在的 flag 返回 HTTP 200 + `type=error`（与 Java `ActionResult` 错误响应一致）
- Edge case: POST `/jaxrs/person` 缺少必填字段返回 `type=error` 响应（含缺参提示）
- Edge case: POST `/jaxrs/person` 重复 unique_id 返回 `type=error` 响应（含重复提示）
- Happy path: GET `/jaxrs/group/list/{flag}/next/{count}` 返回用户组列表
- Happy path: GET `/jaxrs/role/list/{flag}/next/{count}` 返回所有角色列表
- Happy path: GET `/jaxrs/unit/list/{flag}/next/{count}` 返回所有单位列表
- Integration: 响应 JSON 的 `data` 字段被前端 `action.js` 正确提取

**Verification:**
- `cargo test -p control` 全部通过
- 启动服务后，curl 调用组织控制端点返回真实数据库数据
- 响应 JSON 结构与 Java 侧一致

---

### U4. 核心业务端点真实化：个人信息与系统初始化模块

**Goal:** 将个人信息模块和系统初始化模块的端点完善为真实数据库操作（部分端点已实现真实逻辑，本单元补齐路径契约、当前用户身份解析与持久化语义，非简单占位替换）。

**Requirements:** R5, R6

**Dependencies:** U1, U2（`program_init` 接入 `main.rs` 来自 U2）；control 的数据库查询模式仅作为 personal 查询的参考实现（U3 不阻塞 U4，U4 不声明依赖 U3）

**Files:**
- Modify: `oa4rust/crates/personal/src/lib.rs`
- Modify: `oa4rust/crates/personal/src/password.rs`
- Modify: `oa4rust/crates/personal/src/reset.rs`
- Modify: `oa4rust/crates/personal_extend/src/personal.rs`
- Modify: `oa4rust/crates/personal_extend/src/avatar.rs`
- Modify: `oa4rust/crates/personal_extend/src/password.rs`
- Modify: `oa4rust/crates/program_init/src/lib.rs`
- Test: `oa4rust/crates/personal/src/tests.rs`
- Test: `oa4rust/crates/personal_extend/src/tests.rs`

**Approach:**
- `personal` 模块：实现个人信息查询/更新（`GET`/`PUT /jaxrs/person`，当前登录用户）、密码修改（`PUT /jaxrs/person/password`，Java `PasswordAction` 契约；当前自造的 `/jaxrs/password/change`、`/jaxrs/reset/code|check|set` POST 路径需移除）、密码重置（`GET /jaxrs/reset/check/credential/{credential}`、`GET /jaxrs/reset/check/password/{password}`、`GET /jaxrs/reset/code/credential/{credential}`、`PUT /jaxrs/reset`、`POST /jaxrs/reset/password/anonymous`）的真实数据库操作；密码修改必须基于认证中间件注入的当前会话身份解析用户名/凭据（按登录态唯一标识查询），禁止按 `WHERE locked = false LIMIT 1` 操作首行（避免任意登录用户篡改他人/管理员密码）；重置验证码增加速率限制与尝试次数上限，校验通过后立即失效（一次性），过期与重发策略统一
- `personal_extend` 模块：实现个人信息详情查询/更新、头像上传/获取的真实数据库操作；头像路径对齐 Java 契约（`PUT /jaxrs/person/icon` 上传 formData、`GET /jaxrs/person/icon` 获取当前用户头像、`GET /jaxrs/icon/{person}` 获取指定用户头像；当前自造的 `/jaxrs/personal/avatar/{id}` 路径需移除或映射）；头像文件存储采用本地目录（如 `data/avatar/`），限制 5MB 大小上限与 MIME 白名单（jpeg/png/webp），文件名使用 UUID 且存储目录不映射为 web 可访问路径，不接入 file 模块，不承担文件存储迁移方案范围
- `program_init` 模块：实现密码设置（`POST /jaxrs/secret/set`）和初始化检查（`GET /jaxrs/secret/check`）的真实数据库操作，替换内存状态为数据库持久化；secret/check+set 的唯一实现归属为 `program_init`（U1 去重已确保 auth 侧不注册同名路由）；持久化的 secret 采用应用级加密（如 AES-GCM，密钥来自环境变量）存储，并定义轮换机制
- 所有端点使用 `ActionResult<T>` 格式返回，与 Java 侧一致
- 密码哈希采用双算法兼容方案：新写入使用 `bcrypt`（带方案前缀），校验路径同时支持 `bcrypt` 与既有 MD5/DES 哈希（Java 迁移兼容），确保修改密码后现有登录流程可验证新哈希、并可读取迁移的旧哈希；登录成功后对旧算法（MD5/DES）哈希自动 rehash 为 bcrypt，旧哈希淘汰截止点设为 R10 全量切换完成后

**Patterns to follow:**
- 现有 `personal` 和 `personal_extend` crate 的数据库查询模式
- `shared::response::ActionResult` 的成功/错误响应格式

**Test scenarios:**
- Happy path: GET `/jaxrs/person` 返回当前登录用户的信息
- Happy path: PUT `/jaxrs/person` 更新当前用户信息后返回更新后的数据
- Happy path: PUT `/jaxrs/person/password` 修改密码后返回成功
- Happy path: PUT `/jaxrs/reset` 重置密码后返回成功
- Happy path: GET `/jaxrs/reset/code/credential/{credential}` 发送重置验证码后返回成功
- Happy path: GET `/jaxrs/reset/check/credential/{credential}` 校验凭据后返回成功
- Happy path: POST `/jaxrs/reset/password/anonymous` 设置新密码后返回成功
- Happy path: PUT `/jaxrs/person/icon` 上传头像后返回成功
- Happy path: GET `/jaxrs/person/icon` 获取头像
- Happy path: GET `/jaxrs/secret/check` 返回初始化状态（从数据库读取而非内存）
- Happy path: POST `/jaxrs/secret/set` 设置密钥（持久化到数据库）
- Edge case: PUT `/jaxrs/person/icon` 超过 5MB 或非白名单 MIME 类型返回 `type=error` 响应
- Edge case: PUT `/jaxrs/person/password` 旧密码错误返回 `type=error` 响应（未认证 401 用例由 U6 覆盖）
- Integration: 密码修改后使用新密码可成功登录

**Verification:**
- `cargo test -p personal` 和 `cargo test -p personal_extend` 全部通过
- `cargo test -p program_init` 通过（U4 真实化的 program_init 端点在 U4 内验证，不需等待 U6）
- 启动服务后，curl 调用个人信息端点返回真实数据库数据
- 密码修改后新密码可成功验证
- 多用户会话并存下，修改密码仅作用于当前会话对应的用户（验证已消除 `LIMIT 1` 首行篡改）

---

### U5. 认证模块完善：验证码与 OAuth

**Goal:** 替换认证模块中的占位实现，验证码端点返回真正的验证码图片，OAuth 端点实现企业微信和钉钉的第三方登录对接。

**Requirements:** R7, R8, R9

**Dependencies:** U1

**Files:**
- Modify: `oa4rust/crates/auth/src/secret.rs`
- Modify: `oa4rust/crates/auth/src/lib.rs`
- Test: `oa4rust/crates/auth/src/tests.rs`

**Approach:**
- 验证码：在 workspace 根 `Cargo.toml` 添加 `captcha` 依赖，实现本地验证码图片生成；真实实现替换现有 `/jaxrs/authentication/captcha` 占位处理器（返回 base64 占位图）为 PNG 图片，并实现 Java 契约的参数化变体 `GET /jaxrs/authentication/captcha/width/{width}/height/{height}`（前端 `getLoginCaptcha` 实际调用此路径，无参数的自造路径前端不可达）；验证码校验独立为 `/jaxrs/secret/captcha/verify`（与初始化检查 `GET /jaxrs/secret/check` 区分）；验证码一次性使用、5 分钟内有效，校验失败可重试至尝试上限（与 U4 重置流程语义一致）
- OAuth 企业微信：实现企业微信 OAuth2.0 授权流程，`GET /jaxrs/authentication/oauth/qywx/config` 返回企业微信授权配置与 URL（Java `AuthenticationAction.oauth/qywx/config` 契约；当前自造的 `/jaxrs/oauth/wechat` 路径需移除）
- OAuth 钉钉：实现钉钉 OAuth2.0 授权流程，`GET /jaxrs/authentication/oauth/dingding/config` 返回钉钉授权配置与 URL（当前自造的 `/jaxrs/oauth/dingtalk` 路径需移除）
- 认证流程完整可用：登录（`POST /jaxrs/authentication`）→ 会话 → 登出（`DELETE /jaxrs/authentication`）→ 刷新令牌；当前自造的 `/jaxrs/authentication/login`、`/logout`、`/who`、`/refresh` 路径需在 U1/U5 中对齐为 Java 契约路径（`GET /jaxrs/authentication` 返回当前用户）
- 会话存储方案（进程内 `SessionManager` 重启失效 vs JWT 无状态）在 U5 实施时确定并记录；若采用进程内会话，须在 U5 测试覆盖服务重启后旧 token 失效场景
- OAuth 回调：实现 `GET /jaxrs/authentication/oauth/login/qywx/code/{code}`、`GET /jaxrs/authentication/oauth/login/dingding/code/{code}` 端点接收授权码，完成 code→token 交换、校验 state 参数与 redirect_uri 白名单，绑定或创建本地用户并签发会话 token；回调端点按具体路径列入 R12 认证豁免清单（不使用 `{provider}` 动态段）
- 扫码绑定登录端点处置（importante）：现有 `POST /jaxrs/authentication/bind` 直接按 `unique_id` 查询并签发会话，无任何密码/授权校验（可见 `SELECT id FROM auth_person WHERE unique_id = $1` 后 `create_session`），若保留将构成认证绕过；处置决策：对齐 Java 契约实现扫码登录完整流程（`GET /jaxrs/authentication/bind` 返回绑定二维码、`GET/POST /jaxrs/authentication/bind/meta/{meta}` 轮询确认），仅在被绑定用户已确认扫码授权后签发会话；在完整流程未实现前，从路由中移除自造的 `POST /jaxrs/authentication/bind`（拒绝静默 401 或绕过二选一，暴露前必须有明确授权语义）
- 所有 OAuth 端点使用环境变量存储第三方应用的 AppKey 和 AppSecret

**Patterns to follow:**
- 现有 `auth/src/lib.rs` 的会话管理模式（`SessionManager`）
- `shared::response::ActionResult` 的成功/错误响应格式

**Test scenarios:**
- Happy path: GET `/jaxrs/authentication/captcha/width/{width}/height/{height}` 返回 PNG 格式的验证码图片
- Happy path: GET `/jaxrs/authentication/oauth/qywx/config` 返回有效的企业微信授权 URL（包含正确的 redirect_uri 和 scope）
- Happy path: GET `/jaxrs/authentication/oauth/dingding/config` 返回有效的钉钉授权 URL（包含正确的 redirect_uri 和 scope）
- Happy path: GET `/jaxrs/authentication/oauth/login/qywx/code/{code}` 携带有效授权码完成登录并返回会话 token
- Happy path: POST `/jaxrs/secret/captcha/verify` 验证验证码后返回成功
- Happy path: POST `/jaxrs/authentication` 登录成功后返回包含 token 的 `LoginResponse`
- Happy path: GET `/jaxrs/authentication` 携带 token 返回当前用户（验证 token 有效性）
- Happy path: DELETE `/jaxrs/authentication` 登出后 token 失效
- Happy path: POST `/jaxrs/authentication/refresh` 刷新令牌后返回新的 token
- Edge case: GET `/jaxrs/authentication/captcha/width/{width}/height/{height}` 参数非法返回 400
- Edge case: POST `/jaxrs/authentication` 密码错误返回 401
- Edge case: GET `/jaxrs/authentication/oauth/qywx/config` 缺少企业微信配置返回 500
- Edge case: 扫码登录端点（`/jaxrs/authentication/bind`、`/jaxrs/authentication/bind/meta/{meta}`）在被绑定用户确认扫码授权前不得签发会话（验证无认证绕过）
- Integration: 登录后使用返回的 token 可访问受保护的端点

**Verification:**
- `cargo test -p auth` 全部通过
- 启动服务后，curl 调用验证码端点返回 PNG 图片（非占位数据）
- 启动服务后，curl 调用 OAuth 端点返回有效的授权 URL（非示例 URL）
- 完整的登录 → 会话 → 登出 → 刷新令牌流程可正常工作

---

### U6. 安全加固：认证、输入验证、速率限制、HTTPS

**Goal:** 为所有端点添加认证中间件、输入验证、速率限制和 HTTPS 支持，满足安全需求。

**Requirements:** R12, R13, R14, R15

**Dependencies:** U1, U3, U4, U5

**Files:**
- Modify: `oa4rust/crates/shared/src/middleware.rs`
- Modify: `oa4rust/crates/shared/src/router.rs`
- Modify: `oa4rust/crates/shared/src/response.rs`
- Modify: `oa4rust/src/main.rs`
- Test: `oa4rust/crates/shared/src/tests.rs`

**Approach:**
- 认证中间件：为所有非健康检查端点添加认证中间件，验证请求中的会话令牌（从 `Authorization` 头或 cookie 中提取）；中间件实现与挂载在 U1 完成，本单元对其进行加固与专项验证（401/429/豁免边界）；豁免端点与 R12 一致（健康检查、登录、验证码、验证码校验、刷新令牌、密码重置、OAuth 授权与回调、初始化设置且系统未初始化时），按精确路径匹配
- 授权检查：在认证中间件之上增加基于角色的授权检查，person/role/unit/group 的写操作（create/update/delete）仅允许管理员角色，与 Java 侧权限模型对齐
- 输入验证：为所有输入端点添加参数验证（类型、长度、格式），使用 `axum::extract` 的 `Json` 和 `Query` 进行验证
- 速率限制：为认证接口添加每 IP 每分钟 10 次限制的速率限制中间件，为普通接口添加每 IP 每分钟 100 次限制的速率限制中间件；密码重置端点（`/jaxrs/reset/check/credential/{credential}`、`/jaxrs/reset/check/password/{password}`、`/jaxrs/reset/code/credential/{credential}`、`/jaxrs/reset`、`/jaxrs/reset/password/anonymous`）计入认证限流（10 次/分钟/IP）；客户端 IP 提取仅信任来自 nginx 的 `X-Forwarded-For`（可信代理白名单），否则回退 socket 地址；替换 auth 现有 handler 级 RateLimiter（当前为 5 次/分钟且硬编码 `127.0.0.1`），统一由中间件限流
- HTTPS：在生产环境中强制 HTTPS，使用 TLS 1.2+，返回安全响应头（HSTS, X-Content-Type-Options）；TLS 由 nginx 终止（部署拓扑为反向代理），Rust 服务仅监听回环地址，HSTS 与安全头在 nginx 配置，Rust 侧设置应用层安全响应头并执行 HTTP→HTTPS 跳转
- 健康检查端点（`/health`）无需认证

**Patterns to follow:**
- 现有 `shared/src/middleware.rs` 的 trace 中间件模式
- Axum 中间件模式：https://docs.rs/axum/latest/axum/middleware/index.html

**Test scenarios:**
- Happy path: GET `/health` 无需认证返回 200
- Happy path: GET `/jaxrs/unit/list` 携带有效 token 返回 200 和数据
- Error path: GET `/jaxrs/unit/list` 缺少 token 返回 401
- Error path: POST `/jaxrs/person` 缺少必填字段返回 400
- Error path: POST `/jaxrs/person` 字段格式无效返回 400
- Rate limit: 同一 IP 1 分钟内超过 10 次认证请求返回 429
- Rate limit: 同一 IP 1 分钟内超过 100 次普通请求返回 429
- Rate limit: 两个不同的 X-Forwarded-For 客户端 IP 独立计数
- Error path: 非管理员调用 POST `/jaxrs/person` 返回 403
- Integration: 经 Rust 登录的 token 可访问 Java 侧未迁移模块（认证切换前登录保留在 Java 侧）
- Happy path: 生产环境 HTTPS 响应包含 HSTS 和 X-Content-Type-Options 头
- Edge case: 速率限制窗口重置后请求恢复正常

**Verification:**
- `cargo test -p shared` 全部通过
- 启动服务后，未认证请求到受保护端点返回 401
- 启动服务后，速率限制生效，超限请求返回 429
- 生产环境 HTTPS 响应包含安全头（经 nginx 终止层验证）

---

### U7. 迁移进度跟踪清单

**Goal:** 创建并维护 `docs/brainstorms/oa4rust-migration-status.md` 模块跟踪清单，持续反映迁移进度。

**Requirements:** R11

**Dependencies:** U1, U2

**Files:**
- Create: `docs/brainstorms/oa4rust-migration-status.md`

**Approach:**
- 跟踪清单列出所有 80 个 crate 的迁移状态：待迁移 / 已接入（桩代码） / 真实化中 / 已完成
- 每个条目包含：crate 名称、对应的 Java 模块、当前状态、接入 `main.rs` 的版本、已接入的端点列表
- 每次 sprint 结束时更新跟踪清单
- 跟踪清单作为团队追踪迁移进度的单一信息源

**Test scenarios:**
- Happy path: 跟踪清单包含所有 80 个 crate 的条目
- Happy path: 每个条目包含 crate 名称、Java 模块对应、当前状态
- Happy path: 状态为"已接入"或"真实化中"的条目包含已接入的端点列表

**Verification:**
- 跟踪清单包含所有 80 个 crate
- 每个 crate 的状态准确反映当前实现进度
- 团队可通过跟踪清单明确看到哪些模块已框架接入、哪些已有真实实现、哪些仍需填充

---

## System-Wide Impact

- **Interaction graph:** Rust 服务接收前端直接发起的所有 API 请求；Java 服务继续处理未迁移模块的请求。两个服务在迁移期间通过 nginx 前缀路由共存；nginx 前缀切换仅允许对已通过真实化验证的模块执行，且须在认证与限流中间件挂载（U1）并经 U6 专项验证之后，避免端点未受认证保护就暴露到前端；桩代码端点不进入前端可见的切换路径
- **Error propagation:** Rust 服务中间件层统一处理 panic 和错误，输出与 Java 侧一致的 `ActionResult` 格式，前端无需区分后端来源。
- **State lifecycle risks:** 认证模块数据在切换窗口期从 MySQL 迁移到 PostgreSQL，迁移完成后 Java 侧停止写入。回滚时通过 nginx 路由切回 Java，Java 侧数据保持原样。迁移窗口后仍由 Java 处理且会写 org/auth 表的模块须限定为只读或实施单向同步，避免双库数据分叉（Rust 侧新建人员在 Java 侧模块不可见）
- **API surface parity:** 所有已迁移模块的 API 路径（`/jaxrs/*`）必须与 Java 侧的请求方法、响应结构和错误码完全一致（以 Java Action 注解及前端 `xAction` 服务契约为准，非 Rust 自造路径）。
- **Unchanged invariants:** 未迁移模块的所有 API 路径和行为不受影响，继续由 Java 服务处理。
- **Switch acceptance criteria:** 终端用户侧验收须覆盖两个方向——Rust 登录用户访问 Java 侧未迁移模块、以及 Java 登录用户访问 Rust 侧已切换前缀的模块；后者仅在 U6 会话互认方案落地并验证后视为通过，在此之前该方向不列入发布门禁。

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| 桩代码 crate 接入后与已有实现 crate 的路由冲突 | Medium | High | U1 和 U2 严格按顺序执行，U1 先接入已实现 crate，U2 再接入桩代码 crate |
| `:param` 旧语法路由升级 axum 0.8 后静默 404（约 19 个 crate 存量） | High | High | U1 升级时全仓搜索并统一转换为 `{param}`，U1 冒烟覆盖参数化端点 |
| 桩代码 crate 的 TODO 标记遗漏 | Medium | Medium | U2 完成后进行代码审查，确保每个桩代码端点都包含 TODO 标记 |
| 认证模块的验证码和 OAuth 集成依赖第三方服务 | Medium | High | 本地生成验证码降低依赖；OAuth 使用环境变量存储配置，便于切换 |
| 双轨运行期间 Rust 与 Java 模块间的跨服务调用 | Low | Medium | 暂不处理跨服务调用，Rust 服务独立运行 |
| 会话互认方案未决造成切换期用户 401（Java 会话访问 Rust 前缀） | Medium | High | U5 前确定三选一决策并在 U6 验证；决策前不宣称互通可用 |
| Rust 端点路径与 Java 契约偏离导致前端不可达 | Medium | High | 以 Java Action 注解与前端 `xAction` 契约为准对齐路径；U3-U5 测试覆盖契约路径调用 |
| 数据一致性校验方案未确定 | Medium | Medium | 迁移窗口期前确定校验方案 |
| Rust 新手对 Axum + SQLx 的掌握不足 | Medium | Medium | 参考 Axum 官方示例和仓库内已有的 Java Action 实现 |

---

## Documentation / Operational Notes

- 迁移进度跟踪清单（`docs/brainstorms/oa4rust-migration-status.md`）是所有后续模块替换的跟踪依据，每完成一个模块后更新状态
- Rust 服务部署方式：独立 systemd service，启动脚本参考 `oa/o2server/start_linux.sh` 模式，但使用 `cargo run --release` 或预编译二进制
- 执行顺序：U1 → U3 → U2 → U4/U5 → U6 → U7；仅真实化验证通过的模块允许 nginx 前缀切换，认证切换前真实用户自 Java 携带的会话访问 Rust 前缀的互通依赖 U6 前确定的会话互认决策（见 Key Technical Decisions）
- 桩代码端点标记 TODO，保留作为后续实现的明确占位

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md](../brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md)
- Related plan: `docs/plans/2026-08-03-001-refactor-o2server-rust-rewrite-plan.md`
- Related code: `oa4rust/src/main.rs`, `oa4rust/crates/control/`, `oa4rust/crates/auth/`, `oa4rust/crates/personal/`, `oa4rust/crates/personal_extend/`
- Related docs: `docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md`
- External references: Axum 官方示例, Cargo workspace 文档, SQLx + deadpool-postgres 集成文档