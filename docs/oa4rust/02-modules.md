# 02 · 模块划分与核心功能

> crate 全量清单与迁移状态以 `oa4rust/backend/docs/brainstorms/oa4rust-endpoint-inventory.md`（脚本自动生成）为准，本册给业务视角的分组导读。

## 1. 业务域分组（U1–U7 波次）

### 认证与组织（U1–U4 首批试点）

| 模块 | crate | 功能要点 |
| --- | --- | --- |
| 认证 | `auth` | 46 条路由 / 51 handler：密码登录、登出、whoami、token 刷新、验证码、SSO（加密票据）、OAuth、OIDC、LDAP、企业微信（`qiyeweixin`）、微信小程序（`mpweixin`）、welink、政务钉钉（`zhengwudingding`）、短信登录、双因素（`two_factor`）、切换用户（`switch_user`）、安全登出（`safe_logout`）、AndFx |
| 个人设置 | `personal` / `personal_extend` | 个人资料、偏好；扩展属性 |
| 权限授予 | `empower` | 资源授权 |
| 控制/初始化 | `control` / `program_init` | 系统控制面、程序初始化 |

### 协作办公（U5）

| 模块 | crate | 功能要点 |
| --- | --- | --- |
| 考勤 | `attendance` + `attendance_assemble_control` | 考勤打卡、统计、配置（`attendance_assemble_control` 89 handler） |
| 日历 | `calendar` + `calendar_assemble_control` | 日程、日历共享 |
| 文件 | `file` + `file_assemble_control` + `file_core_entity` | 网盘、分享、回收站（94 handler） |
| 通用组件 | `general` + `general_assemble_control` | 通用数据/字典（61 handler） |

### 内容与流程（U6）

| 模块 | crate | 功能要点 |
| --- | --- | --- |
| CMS 内容 | `cms_control` / `cms_express` / `cms_assemble_control`（312 handler）/ `cms_core_*` | 栏目、文档、表单/视图/脚本设计器数据 |
| 会议 | `meeting` + `meeting_assemble_control` | 会议室、预约、签到 |
| 消息 | `message` + `message_assemble_communicate` | 站内消息、IM 会话收发族 |
| 门户 | `portal` + `portal_assemble_designer/surface` | 门户设计器与渲染 |
| 流程平台 | `processplatform_*`（assemble_surface 487 handler、designer 96、service_processing 113、bam 50、core_*） | 流程引擎核心：待办/已办/草稿、task complete/reject、表单/路由设计、BAM 监控 |
| 数据平台 | `query_assemble_designer/surface`、`query_service(_processing)`、`query_core_*` | 查询设计器、统计视图、自定义查询执行 |

### 社区与增值（U7）

| 模块 | crate | 功能要点 |
| --- | --- | --- |
| AI | `ai` + `ai_assemble_control` + `ai_core_entity` | AI 助手、ANN 全文检索（迁移 096 建 ANN/FTS 表） |
| 论坛 | `bbs` + `bbs_assemble_control` + `bbs_core_entity` | 版块、发帖、回复 |
| 组件 | `component` + `component_assemble_control` | 可插拔组件 |
| 云控制台 | `console` | 运维控制台 |
| 关联 | `correlation` 族 | 数据关联 |
| 热图/热搜 | `hotpic` 族 | 热点图片 |
| 推送 | `jpush` 族 | 极光推送对接 |
| 脑图 | `mind` 族 | 思维导图 |
| 组织 | `organization_assemble_control`（105 handler）等 | 人员/部门/身份/群组管理 |
| 程序中心 | `program_center`（205 handler）+ `program_center_core_entity` | 服务注册、invoke 别名（唯一索引见迁移 105） |

### 基础设施 crate（不计入迁移统计）

| crate | 职责 |
| --- | --- |
| `shared` | 全部横切能力（见下节） |
| `orm` | 早期 ORM 抽象（大部分业务 crate 已直用 tokio-postgres） |
| `mcp_server` | MCP 工具服务：`tool_bridge`（JSON-RPC 2.0：initialize / tools/list / tools/call，协议版本 2024-11-05）+ 生成路由清单 |
| `openapi` | utoipa `ApiDoc`，输出 `/openapi.json` |
| `realtime` | WebSocket 实时协议：`/ws/realtime`、`/ws/realtime/room/{room_id}`、`/ws/realtime/room/{room_id}/stats`；`RealtimeManager` 房间管理 |
| `search` | 全文/ANN 检索服务 |
| `preview` | 文档预览 |
| `signature` | PDF/文档签名（lopdf 0.42 解析签名） |
| `parity` | 与 Java 基准的行为对齐测试集（4143 用例） |
| `ldap` | LDAP 认证 + 定时同步 worker（`LDAP_SYNC_ENABLE` 门控，幂等） |
| `sms` / `captcha_store` | 短信与验证码存储适配（待迁移项） |

## 2. `shared` crate —— 横切能力总览

路径：`oa4rust/backend/crates/shared/src/`

| 模块 | 文件 | 说明 |
| --- | --- | --- |
| 错误 | `error.rs` | `AppError`：Database / Redis / Internal / InternalServerError / BadRequest / Unauthorized / NotFound / Forbidden / NotImplemented |
| 响应 | `response.rs` | `ActionResult<T>` 信封（对齐 o2server v9 Gson）、`row_to_json` 通用行转 JSON |
| 数据库 | `db/`（`mod.rs`、`dialect.rs`、`rewriter.rs`） | `create_pool()`；`SqlDialect` trait（PG/MySQL）；PG→MySQL SQL 重写 |
| 迁移 | `migrate.rs` | `run_migrations(&Pool)` 幂等执行；`MigrationReport { applied, skipped }`；`applied_versions()` |
| 会话 | `session.rs` | `SessionManager`：Redis 优先，降级内存+DB；HttpOnly Cookie；`SESSION_TTL_SECONDS` |
| 中间件 | `middleware/` | `constants.rs`（认证豁免路径、限流目标、管理写族前缀）、`security.rs`（CORS/安全头/trace）、`token.rs`（会话识别）、`rbac.rs`（`PermissionLevel` / `PermissionRegistry` / authorize）、`rate_limit_distributed.rs`（Redis 分布式限流） |
| 限流 | `rate_limit.rs` | `RateLimiter`（内存/Redis 双模） |
| CRUD | `crud.rs` | `CrudSpec` + `crud_create/save/delete` 通用真实化 handler |
| 校验 | `input_validation.rs` | 输入校验工具 |
| 存储 | `storage.rs` | `BlobStorage` trait：`FsBlobStorage` / `PgBlobStorage` / `DbBlobStorage`；`storage_from_env()` |
| 消息 | `messaging.rs` | `MessageBus`：`InMemoryBus` / `RedisPubSubBus`；`TokenThresholdEvent` |
| 调度 | `scheduler.rs` | 定时任务基座 |
| 网络防护 | `netguard.rs` | SSRF 等网络防护 |
| 配置 | `config.rs` | `AuthConfig::from_env()`（production 校验、public_origin、cookie_secure、session_ttl） |
| 测试 | `testing.rs` / `tests.rs` / `tests_generated.rs` | `test_pool()`、MockRow、生成测试 |
| 模拟 | `mock_client.rs` | 外部客户端 mock |
| 流程定义 | `process_definition.rs` | 流程定义共享模型 |

## 3. 桌面 Web 端模块（`frontend/apps/desktop/src/`）

- `views/`：98 个文件，覆盖 OA 全功能——流程（FormApp / FormDesigner / ProcessCenter 类）、CMS（CmsColumn/CmsForm/CmsView/CmsScript Designer）、数据平台（QueryManagerDeep / QueryView / QueryStatement Designer）、IM（IMChat）、AI（AIChatApp）、考勤（AttendanceApp）、日历、BBS（BBSForum）、会议、脑图、通讯录（UnitApp / PersonApp / RoleManager）、控制台（ControlPanelApp / ServerApp / DeploymentApp）、搜索（SearchApp / FtSearchApp）、SSO、人脸设置（FaceSetApp）等。
- `contracts/`：前端侧契约模块（`process-definition`、`xform`、`designer`），与 `tests/contracts/` 守卫联动。
- 组件与工具：`components/`、`utils/`；PWA：`registerSW.ts` + `manifest.json`。
- 依赖底座：`@oa4rust/sdk`（传输/状态）、`@oa4rust/ui`、naive-ui、pinia、vue-query。

## 4. 移动端模块（`mobile/src/`）

- **页面（16）**：`login`、`index`（工作台：待办计数 + 宫格入口）、`message`（会话列表）、`chat`（会话详情）、`process`（审批：待办/已办/我发起 + 通过/驳回）、`doc`（我的文件）、`contacts`（通讯录搜索）、`mine`、以及 `attendance`、`bbs`、`browse`、`calendar`、`collect`、`meeting`、`recycle`、`search`。
- **导航**：底部 tabBar 4 项（工作台 / 消息 / 审批 / 我的），路由配置在 `pages.json`。
- **传输层**：`services/http.ts`（`uni.request` / `uni.uploadFile`，语义对齐桌面 `ApiClient`：401 自动 refresh 重试、403 权限错误、统一 `ApiError`/`ApiResponse`）。
- **端点策略**：`services/index.ts` 只暴露移动端高频模块（auth / process / message / file / org / general），**端点全部对齐后端已注册 axum 路由**，不把全量路由打进移动包。
- **鉴权差异**：H5 同源/代理 → Cookie 直用；原生/小程序需 `setApiBase`（`VITE_OA_API_BASE`）指向后端绝对地址，且待后端 token 通道补齐后注入 `Authorization`（现状 Cookie 在原生容器不自动跨域携带，详见 mobile README）。

## 5. 质量与门禁设施

| 设施 | 位置 | 作用 |
| --- | --- | --- |
| 迁移状态跟踪 | `backend/docs/brainstorms/oa4rust-migration-status.md`、`oa4rust-endpoint-inventory.md` | 单一信息源：`regen_endpoints.py` / `extract_endpoints.py` 静态扫描生成 |
| 行为对齐测试 | `backend/tests/parity/`、`parity_suite.rs`、`behavior_comparison/` | 与 o2server v9 基准比对 |
| 生成的 handler 测试 | 各 crate `tests_generated.rs`；`backend/scripts/generate_handler_tests.py` | 覆盖全部 handler 的冒烟/分支 |
| 三端契约守卫 | `oa4rust/tests/contracts/` | desktop/mobile 端点 × 后端注册路由 fixture 对账、空 catch 守卫、生产序列化检查、schema 校验 |
| E2E | `oa4rust/e2e/`（Playwright） | app 覆盖、设计器往返、workflow 运行时、live 全流程 |
| 安全审计 | `backend/docs/audits/2026-10-*.md` | 依赖（RustSec advisory-db 全量比对）+ 代码级复验 |
| RustSec 例外 | `backend/security/rustsec-exceptions.toml` + `rustsec-db.rev` | 例外清单与 advisory-db 固定版本（审计可复现） |
| 性能基线 | `backend/docs/performance-baseline.md`、`tests/perf_baseline.rs`、压测脚本 | 端点延迟/发布 profile 基准 |
| 灰度与回滚 | `backend/deploy/`、`backend/docs/gray-release-playbook.md`、`ops/auth-rollback-drill-*.md` | 模块切流、回滚演练（RTO 5 分钟） |
