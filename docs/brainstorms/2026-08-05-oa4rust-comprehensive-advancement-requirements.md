---
date: 2026-08-08
topic: oa4rust-comprehensive-advancement
---

# oa4rust 全面推进：需求文档

## Summary

继续推进 oa4rust 后端 Rust 迁移：当前 81 个 crate 中 73 个已完成真实化，剩余 8 个无数据库查询的 crate 需从零实现 PostgreSQL 真实业务逻辑，最终使 Rust 后端达到可完全替代 Java 运行的完整度。

---

## Problem Frame

O2OA 后端当前 100% 基于 Java（Maven 55+ 模块），长期技术栈锁定在 Java 生态。oa4rust 项目已将全部 55 个 Java 模块映射为 81 个 Rust crate，编译通过且测试通过。目前全部 81 个 crate 已注册到 workspace 并在 `main.rs` 中挂载路由（7,618 个路由注册）。其中 73 个 crate 已完成真实化，handler 中包含 PostgreSQL 真实查询；剩余 8 个 crate（`ai`、`ai_core_entity`、`cms_express`、`correlation_core_entity`、`file_core_entity`、`organization_core_entity`、`program_center_core_entity`、`query_express`）已注册路由但 handler 中完全没有 PostgreSQL 查询调用，需从零实现真实业务逻辑。团队无法在 Rust 后端上推进这些模块的实际工作，迁移停滞的代价是持续维护 Java 运行时和技术栈锁定。本次迁移要求剩余 8 个 crate 的每一个端点都必须实现真实业务逻辑，无任何桩代码残留。

---

## Requirements

**路由框架接入**
- R1. 全部 80 个 workspace crate 的路由已注册到 `main.rs`，需验证所有路由正确暴露且无冲突。对于尚未完全接入的 crate，完成中间件配置（认证、CORS、限流等）
- R2. 桩代码端点必须在代码中标记 `TODO: [module] - real implementation needed`，所有 TODO 标记在 sprint 规划中 review 并分配优先级
- R3. 已存在真实实现的 crate（control、personal_extend 等）优先接入，桩代码端点不得阻塞已实现端点的暴露

**全模块全功能全路由真实化**
- R4. 全部 80 个 crate 的每一个端点都必须接入 PostgreSQL 真实业务逻辑，无任何永久性桩代码残留。实施时按业务关键性和依赖关系分优先级推进，优先完成覆盖核心用户工作流且业务逻辑相对简单的 crate，再逐步推进其余 crate
- R5. 每个 crate 的全量路由按对应 Java 模块的 JAX-RS 控制器端点逐一实现，功能上与 Java 端等效（字段名、类型、非空约束、分页/游标/排序语义保持一致），允许 Rust 实现在不破坏前端契约的前提下修复错误、简化逻辑和改进性能
- R6. 组织控制模块（人员、单位、角色、用户组）的 CRUD 端点返回真实数据库数据
- R7. 个人信息模块（个人信息查询/更新、密码管理、头像管理）的端点返回真实数据
- R8. 文件管理模块（上传/下载/目录管理/权限）的端点返回真实数据
- R9. 消息服务模块（消息消费/创建/已读/未读）的端点返回真实数据
- R10. 会议管理模块（会议创建/查询/参与人/日程）的端点返回真实数据
- R11. 考勤模块（打卡/统计/排班/请假）的端点返回真实数据
- R12. CMS 模块（栏目/文章/字典/索引）的端点返回真实数据
- R13. 查询/报表模块（视图/统计/导入/导出）的端点返回真实数据
- R14. 门户模块（页面/部件/脚本/字典）的端点返回真实数据
- R15. 流程引擎模块（应用/表单/流程/任务/工作/快照/签名）的端点返回真实数据
- R16. BBS/论坛模块（分类/文章/版主/搜索）的端点返回真实数据
- R17. 日历模块（日程/事件/共享）的端点返回真实数据
- R18. 组件管理模块（应用中心/应用市场/部署）的端点返回真实数据
- R19. AI 引擎模块（模型/推理/训练）的端点返回真实数据
- R20. 思维导图模块（创建/编辑/共享）的端点返回真实数据
- R21. 推送服务模块（消息推送/设备管理）的端点返回真实数据
- R22. 热点图片模块（轮播/推荐）的端点返回真实数据
- R23. 程序中心模块（应用管理/配置）的端点返回真实数据
- R24. 控制台模块（命令行/日志/监控）的端点返回真实数据
- R25. 通用服务模块（序列号/权限/配置）的端点返回真实数据
- R26. 关联关系模块（数据关联/引用）的端点返回真实数据
- R27. 表达式引擎模块（组织/流程/CMS/查询脚本）的端点返回真实数据
- R28. BAM 监控模块（业务活动监控）的端点返回真实数据
- R29. 系统初始化模块（密码设置、初始化检查）的端点返回真实数据
- R30. 快递查询模块（物流追踪）的端点返回真实数据

**测试要求**
- R31. 每个 crate 必须包含单元测试，覆盖核心业务逻辑
- R32. 核心业务流程必须包含集成测试，核心业务流程集成测试覆盖率 ≥ 80%
- R33. 必须建立 Rust 端点与 Java 端点的行为对比测试机制，确保功能等效性

**剩余 8 个无数据库查询 crate 的真实化**
- R54. `ai` crate（26 个路由）实现 AI 模型列表/详情、对话历史、模型配置等真实 PostgreSQL 查询或外部 API 调用
- R55. `ai_core_entity` crate（17 个路由）实现 AI 核心实体（模型/对话/消息）的 CRUD 操作
- R56. `cms_express` crate（11 个路由）实现 CMS 内容发布/下架、内容审核、缓存刷新等真实逻辑
- R57. `correlation_core_entity` crate（14 个路由）实现关联关系实体 CRUD 及数据关联/引用管理
- R58. `file_core_entity` crate（27 个路由）实现文件实体 CRUD、文件版本管理、文件权限控制（需与 `file` 和 `file_assemble_control` 明确职责边界，避免重复）
- R59. `organization_core_entity` crate（31 个路由）实现组织核心实体（人员/单位/角色/用户组）CRUD（需与 `control` 和 `organization_assemble_control` 明确职责边界，避免重复）
- R60. `program_center_core_entity` crate（39 个路由）实现程序中心核心实体（应用/脚本/数据结构）的 CRUD
- R61. `query_express` crate（6 个路由）实现查询执行引擎、动态 SQL 解析与执行

**端点清单与优先级**
- R34. 实施前必须建立端点清单文档，明确每个 crate 对应的 Java JAX-RS 端点列表、业务优先级和实现顺序
- R35. 优先实现高业务价值、低复杂度的 crate（建议首批 20 个 crate 覆盖 80% 核心用户工作流），验证通过后再推进其余 crate

**认证模块完善**
- R36. 验证码端点返回真正的验证码图片（本地生成，集成 captcha 库）
- R37. OAuth 端点实现第三方登录对接（微信、钉钉）
- R38. 认证流程完整可用（登录 → 会话 → 登出 → 刷新令牌）

**安全需求**
- R39. 除健康检查及认证入口端点（登录、验证码、OAuth 授权、令牌刷新等）外，所有端点强制认证（登录用户方可访问）
- R40. 除 R39  exempted 的未认证端点外，每个端点必须实施基于用户角色、用户组和资源所有者的访问控制，明确每个模块的权限边界（谁能读取/更新哪些资源）
- R41. 所有输入端点进行参数验证（类型、长度、格式），拒绝无效输入
- R42. 认证接口速率限制（10次/分钟/IP），普通接口速率限制（100次/分钟/IP）
- R43. 所有响应强制 HTTPS（TLS 1.2+），生产环境返回安全的响应头（HSTS, X-Content-Type-Options）
- R44. 敏感数据（PII、凭证、组织数据）必须加密存储，传输使用 TLS，日志中禁止记录敏感字段
- R45. OAuth 客户端密钥和 API Key 必须存储在环境特定的密钥管理器中，禁止硬编码，支持密钥轮换和环境隔离
- R46. OAuth 集成必须验证 state 参数、使用 PKCE（若提供者支持）、验证提供者签名，并具备备用认证方案
- R47. 验证码库集成必须验证来源完整性，优雅处理生成失败，不在错误信息中泄露实现细节
- R48. 会话令牌必须加密签名、设置短期过期时间；刷新令牌必须安全存储（HttpOnly、Secure cookies）；令牌在刷新时必须轮换；登出时立即失效；禁止使用 localStorage 存储令牌

**迁移策略**
- R49. 沿用 Strangler Fig 渐进式迁移策略：Rust 与 Java 并行运行，通过 nginx 反向代理按 URL 前缀路由，逐步切换流量
- R50. 迁移进度通过 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 模块跟踪清单持续反映，每个模块标记为待迁移 / 迁移中 / 已完成
- R51. 必须制定回滚计划：定义触发回滚的条件（数据损坏、性能下降超过阈值等）、回滚流程（切回 Java）、以及用于即时切换的特性开关
- R52. 双轨运行期间必须进行数据库访问模式分析：记录事务隔离级别、识别并发写入风险、对正在迁移的表实施数据校验或禁用 Java 写入
- R53. 迁移前必须对已实现真实业务逻辑的 4 个 crate（auth、personal、personal_extend、control）进行行为测试，确认与 Java 后端一致后再作为其余 76 个 crate 的参考基准

---

## Acceptance Examples

- AE1. **Covers R4, R5, R6.** Given a seeded database with test data, when a CRUD endpoint is called, it returns data matching the Java backend's response schema (field names, types, non-null constraints) rather than empty or placeholder values.
- AE2. **Covers R36.** Given a captcha request, when the endpoint is called, it returns a generated captcha image (not a base64 placeholder or empty response).
- AE3. **Covers R37.** Given an OAuth request for a supported provider (WeChat or DingTalk), when the endpoint is called, it returns a valid authorization URL for the provider (not an example URL).
- AE4. **Covers R40.** Given an authenticated request with a valid session token, when the user attempts to access an admin-only endpoint without the admin role, the server returns 403 Forbidden rather than allowing access.
- AE5. **Covers R31, R32.** Given the codebase at any point during implementation, when `cargo test` is run, all tests pass and the integration test coverage report shows ≥ 80% coverage for core business flows.
- AE6. **Covers R34.** Given the start of implementation, when the team begins working on a crate, they reference the endpoint inventory document to identify the corresponding Java JAX-RS endpoints and their business priority.
- AE7. **Covers R51.** Given a production issue during parallel operation (e.g., data corruption detected), when the rollback procedure is executed, traffic is fully redirected to the Java backend within a predefined RTO (documented in the rollback plan) without data loss.
- AE8. **Covers R5.** Given a Java endpoint with a known bug (e.g., incorrect error message format), when the Rust implementation fixes the bug while maintaining the same response structure, the frontend continues to work correctly and the bug is resolved.
- AE9. **Covers R54-R61.** Given the remaining 8 crates with no database queries, when each handler is called, it returns real data from PostgreSQL queries rather than empty or placeholder values.

---

## Success Criteria

- 全部 81 个 crate 的路由在 `main.rs` 中可见，前端可通过对应前缀调用任意 API
- 全部 81 个 crate 的每一个端点均已接入 PostgreSQL 真实业务逻辑，无任何永久性桩代码残留
- 剩余 8 个无数据库查询的 crate（`ai`、`ai_core_entity`、`cms_express`、`correlation_core_entity`、`file_core_entity`、`organization_core_entity`、`program_center_core_entity`、`query_express`）实现真实业务逻辑并通过 `cargo test`
- 每个 crate 的全量路由按对应 Java 模块的 JAX-RS 控制器端点逐一实现并通过前端联调验证
- 认证模块完全可用，验证码和 OAuth 不再返回占位数据
- 迁移进度可追踪，团队能明确看到每个模块的全量端点实现状态
- Rust 服务通过反向代理与 Java 服务共存，前端无感知切换
- `cargo test` 全部通过，核心业务流程集成测试覆盖率 ≥ 80%
- 端点清单文档完整，每个 crate 的 Java 端点映射和实现优先级已定义
- 行为对比测试套件就绪，可自动化验证 Rust 端点与 Java 端点的功能等效性
- 回滚程序和特性开关已部署，可在出现问题时快速切回 Java

---

## Scope Boundaries

- 原则上不修改前端 `o2web` 的代码，仅通过 URL 前缀路由适配后端切换。若后端响应格式调整导致前端展示异常，允许在文档化并经前端团队确认后实施最小化前端适配，但核心业务逻辑和页面代码不得迁移
- 不在改写期间实现 Java ↔ Rust 的实时数据同步，仅依赖一次性迁移窗口
- 不拆分为微服务，Rust 侧始终以单一进程单体服务运行
- 不包含 Rust 性能压测或与 Java 的基准对比
- 不迁移 `o2web` 前端核心代码，该部分保持现状
- Java 服务的永久下线脚本属于后续阶段，但必须制定迁移期间的 rollback 程序和特性开关，确保出现问题时可快速切回 Java
- 不进行数据库 schema 变更或迁移脚本编写（沿用现有计划中的 schema）
- 全部 81 个 crate 的桩代码必须在真实化阶段全部清除，不允许任何 crate 以桩代码状态进入生产

---

## Key Decisions

- **全量真实化**：全部 81 个 crate 的每一个端点都必须实现 PostgreSQL 真实业务逻辑，无任何永久性桩代码残留
- **分优先级推进**：按业务关键性和依赖关系分优先级，优先完成覆盖核心用户工作流且业务逻辑相对简单的 crate，再逐步推进其余 crate，避免"大爆炸"风险
- **三轨并行推进**：框架接入（所有 crate 接入 main.rs）、全模块真实化（全功能全路由）、认证完善（替换占位实现）三条轨道同时推进，最大化并行度
- **已实现模块优先接入**：control、personal_extend 等已有真实实现的 crate 优先接入 main.rs，让团队尽快看到可用 API
- **功能等效而非行为一致**：Rust 端点功能上与 Java 等效即可，允许在保持前端契约的前提下修复错误、简化逻辑和改进性能，建立行为兼容性测试套件而非逐字节一致性检查
- **前端容忍度**：允许存在少量无法避免的前端调整（如字段排序、错误消息格式），文档化这些例外，并尽早让前端团队参与验收测试
- **Rust 为唯一技术选项**：无替代方案（Go 等），迁移必须使用 Rust
- **沿用 Strangler Fig 迁移策略**：双轨运行、按模块切换、灰度验证，已在前期计划中验证
- **端点契约对齐**：每个 Rust 端点的请求/响应格式、字段名、类型、分页/游标/排序语义必须与对应 Java JAX-RS 端点保持一致，确保前端零改动
- **剩余 8 个 crate 按依赖关系排序**：先实现 `organization_core_entity`（组织实体是其他模块的基础），再实现 `file_core_entity`，最后处理 `ai`/`ai_core_entity`、`cms_express`、`correlation_core_entity`、`program_center_core_entity`、`query_express` 等复杂模块
- **职责边界明确**：`organization_core_entity` 需与 `control` 和 `organization_assemble_control` 明确边界；`file_core_entity` 需与 `file` 和 `file_assemble_control` 明确边界，避免功能重复

---

## Dependencies / Assumptions

- Java 服务在 Rust 改写期间持续稳定运行，是 Rust 模块功能验证前的可用参照物
- 76 个未集成 crate 的路由与 `main.rs` 中已有 crate 的路由无冲突（当前剩余 8 个 crate 需实现真实查询）
- auth 模块的验证码和 OAuth 集成有可行的第三方服务或自建方案
- 迁移窗口期的数据量在可接受范围内
- 反向代理（nginx）已存在或可部署，作为前缀路由层
- 已实现真实业务逻辑的 73 个 crate 与 Java 端行为一致（需前端联调验证）
- 已包含真实 PostgreSQL 查询代码的 crate 的真实查询逻辑可被复用，无需完全重写
- 每个 Java JAX-RS 端点的业务逻辑可被独立还原，无需依赖 Java 侧实时运行
- 全部 81 个 crate 的数据库表 schema 可通过现有迁移脚本或增量迁移补齐
- 前端 `o2web` 的 `action.js` 对 `ActionResult<T>` 9 字段 JSON 结构的解析逻辑在 Rust 侧完全兼容
- 团队具备足够的 Rust 开发能力和人员配置，能够在合理时间内完成剩余 8 个 crate 的真实化工作
- 并行运行期间数据库事务隔离级别足以防止 Rust 与 Java 之间的数据竞争

---

## Outstanding Questions

### Resolve Before Planning

（无，所有问题已在 brainstorm 阶段解决）

### Deferred to Planning

- [Affects R54-R61][Needs research] 剩余 8 个 crate 对应的 Java JAX-RS 端点业务逻辑细节，对照 Java 源码逐端点还原
- [Affects R58][Needs research] `file_core_entity` 与 `file`、`file_assemble_control` 的职责边界如何划分，避免功能重复
- [Affects R59][Needs research] `organization_core_entity` 与 `control`、`organization_assemble_control` 的职责边界如何划分，避免功能重复
- [Affects R54-R55][Technical] `ai` 和 `ai_core_entity` 的 AI 功能是需要调用外部 API 还是纯 PostgreSQL 查询，或是两者结合？
- [Affects R61][Technical] `query_express` 的查询执行引擎是调用其他 crate 的查询能力还是直接执行动态 SQL？
- [Affects R49][Technical] 双轨运行期间 Rust 与 Java 模块间的跨服务调用如何处理？
- [Affects R50][Needs research] 模块跟踪清单的格式和更新频率如何定义？
- [Affects R39-R48][Technical] 统一认证中间件、授权中间件、输入验证中间件、速率限制中间件的实现方案
- [Affects R51][Technical] 回滚程序的 RTO（恢复时间目标）是多少，特性开关的实现方式是什么？
- [Affects R52][Technical] 正在迁移的表的 Java 写入如何禁用或隔离，数据校验的具体机制是什么？
- [Affects R33][Needs research] 行为对比测试套件的具体实现方式：是通过快照对比、语义对比还是其他机制？
- [Affects R40][Needs research] 每个模块的具体权限边界如何定义（谁可以读取/更新哪些资源）？
- [Affects R45][Needs research] OAuth 客户端密钥和 API Key 的具体密钥管理器选型（环境变量、Vault、云服务等）？
- [Affects R48][Needs research] 会话令牌的具体格式（JWT、 opaque token）、过期时间、存储方式？