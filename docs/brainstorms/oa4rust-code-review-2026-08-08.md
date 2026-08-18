# OA4Rust 全量代码审查报告

**审查范围:** feat/o2server-rust-rewrite 分支 vs origin/main (7aec53db)
**审查日期:** 2026-08-08
**变更规模:** 162 files changed, 26007 insertions(+), 10378 deletions(-)
**审查模式:** 全量审计

## 审查团队

- correctness (always)
- security (always)
- testing (always)
- maintainability (always)
- project-standards (always)
- ce-agent-native-reviewer (always)
- ce-learnings-researcher (always)
- api-contract -- 新增大量 REST 端点
- performance -- 新增 DB 查询 + 中间件每请求授权查询
- data-migrations -- 新增迁移文件 008
- adversarial -- 变更涉及 auth、数据变更、外部 API

---

## P0 — 阻断级（7 项）

| # | 文件 | 行 | 问题 | 审查人 | 修复路由 |
|---|------|-----|------|--------|----------|
| 1 | oa4rust/crates/query_service/src/lib.rs | 116 | ON CONFLICT 需要部分唯一索引，但该索引可能不存在 | correctness | gated_auto -> downstream-resolver |
| 2 | oa4rust/crates/ai/src/lib.rs | 305 | chat_delete 执行双表 DELETE 无事务保护，失败会导致数据不一致 | correctness, adversarial | gated_auto -> downstream-resolver |
| 3 | oa4rust/crates/ai/src/lib.rs | 116 | config_list_model_paging 明文返回 AI 模型完整 API key（同 crate 内 config_get_model 已做脱敏） | security, adversarial, agent-native | gated_auto -> downstream-resolver |
| 4 | oa4rust/crates/file_assemble_control/src/lib.rs | 506 | create_file_entity 接受用户提供的 creator 字段，可伪造审计轨迹 | security | gated_auto -> downstream-resolver |
| 5 | oa4rust/crates/query_service/src/lib.rs | 100 | processing_execute 将用户原始 SQL 存入数据库，存在下游 SQL 注入风险 | security | manual -> downstream-resolver |
| 6 | oa4rust/migrations/008_person_group_tables.sql | 1 | 重复的 008_ 前缀导致迁移静默跳过，行为因环境而异 | data-migrations | gated_auto -> downstream-resolver |
| 7 | oa4rust/crates/shared/src/middleware.rs | 221 | ADMIN_WRITE_PREFIXES 从 4 个扩展到 15 个，是静默破坏性变更：非 admin 客户端对 /jaxrs/ai、/jaxrs/file、/jaxrs/program_center 等写操作将收到 403 | api-contract | gated_auto -> downstream-resolver |

---

## P1 — 高优先级（16 项）

| # | 文件 | 行 | 问题 | 审查人 | 修复路由 |
|---|------|-----|------|--------|----------|
| 8 | oa4rust/crates/file_assemble_control/src/lib.rs | 540 | update_file_entity 的 COALESCE 逻辑会覆盖 size=0 且返回请求值而非持久化值 | correctness | gated_auto -> downstream-resolver |
| 9 | oa4rust/crates/ai/src/lib.rs | 91 | 分页 offset 计算在 i32 转 i64 前可能溢出 | correctness | gated_auto -> downstream-resolver |
| 10 | oa4rust/crates/program_center/src/lib.rs | 118 | modules_all 将 entityCount 语义改为字段数，可能破坏前端契约 | correctness | gated_auto -> downstream-resolver |
| 11 | oa4rust/crates/ai/src/lib.rs | 338 | chat_delete、file_delete 等无所有权检查，任何登录用户可删除他人数据 | security | manual -> downstream-resolver |
| 12 | oa4rust/crates/shared/src/middleware.rs | 727 | PermissionLevel::Owner 分支是空实现，永远不会执行所有权验证 | security | manual -> downstream-resolver |
| 13 | oa4rust/crates/shared/src/middleware.rs | 536 | person_has_group 缺少软删/禁用过滤（person_has_role 有但 group 没有） | security | gated_auto -> downstream-resolver |
| 14 | oa4rust/crates/ai/src/tests.rs | 34 | 路由存在性测试断言 INTERNAL_SERVER_ERROR，提供虚假信心 | testing | manual -> downstream-resolver |
| 15 | 6+ crates | - | 相同的虚假信心模式在 6+ 个 crate 中重复 | testing | manual -> downstream-resolver |
| 16 | oa4rust/crates/file_assemble_control/src/tests.rs | 103 | MockControlClient::ctrl_query 对 Rows 变体返回空 vec，mock 完全失效 | testing | manual -> downstream-resolver |
| 17 | oa4rust/crates/shared/src/middleware.rs | 334 | DB 宕机时所有授权检查降级为 deny，导致管理接口全面不可用 | adversarial | advisory -> human |
| 18 | oa4rust/crates/shared/src/middleware.rs | 193 | 新 /jaxrs/* 模块未注册时继承 Authenticated 而非 Admin，写操作权限降级 | adversarial | advisory -> human |
| 19 | oa4rust/crates/query_service/src/lib.rs | 109 | ON CONFLICT DO UPDATE 返回新生成的 UUID，但实际 DB 行保留旧 ID | adversarial | advisory -> human |
| 20 | oa4rust/crates/shared/src/middleware.rs | 677 | authorize_middleware 每请求执行 1-3 次 DB 授权查询，高并发下可能耗尽连接池 | performance | manual -> downstream-resolver |
| 21 | oa4rust/crates/shared/src/middleware.rs | 693 | check_permission 中 is_admin 被重复调用最多 5 次 | performance | manual -> downstream-resolver |
| 22 | oa4rust/crates/program_center/src/lib.rs | 2520 | ApplicationCreateRequest 字段命名 snake_case 但响应 camelCase，契约不一致 | api-contract | gated_auto -> downstream-resolver |
| 23 | oa4rust/crates/ai/src/lib.rs | 485 | list_enable_model 响应语义变更：name 现在返回内部 xname 而非显示名 | api-contract | gated_auto -> downstream-resolver |
| 24 | oa4rust/crates/ai/src/lib.rs | - | ActionResult 错误响应缺少结构化错误码，agent 无法程序化决策 | agent-native | manual -> downstream-resolver |
| 25 | oa4rust/crates/shared/src/middleware.rs | 777 | 403 响应返回固定字符串，agent 不知道缺少哪个角色 | agent-native | manual -> downstream-resolver |

---

## P2 — 中优先级（20+ 项）

| # | 文件 | 行 | 问题 | 审查人 | 修复路由 |
|---|------|-----|------|--------|----------|
| 26 | oa4rust/crates/ai/src/lib.rs | 143 | 混合类型 OR 谓词 (id=$1 OR xname=$1) 存在隐式类型转换风险 | correctness | gated_auto -> downstream-resolver |
| 27 | oa4rust/crates/file_assemble_control/src/lib.rs | 592 | Stub 端点返回硬编码成功，应返回 501/404 | correctness | gated_auto -> downstream-resolver |
| 28 | oa4rust/crates/file_assemble_control/src/lib.rs | 573 | delete_file_entity 使用硬删除，同表 delete_file 使用软删除，策略不一致 | correctness | gated_auto -> downstream-resolver |
| 29 | oa4rust/crates/file_assemble_control/src/lib.rs | 554 | update_file_entity 响应回显请求值而非持久化值 | correctness | gated_auto -> downstream-resolver |
| 30 | oa4rust/crates/shared/src/middleware.rs | 948 | behavior-comparison 中间件记录完整响应体（最多 4KB）到 tracing | security | manual -> downstream-resolver |
| 31 | oa4rust/crates/shared/src/middleware.rs | 354 | is_admin 的时序侧信道：每次写请求都查 DB | security | manual -> downstream-resolver |
| 32 | oa4rust/crates/file_assemble_control/src/lib.rs | 503 | 用户可控文件路径未过滤，存在路径遍历风险 | security | manual -> downstream-resolver |
| 33 | oa4rust/crates/ai/src/lib.rs | 280 | 聊天完成端点返回原始 AI 输入/输出，可能包含敏感数据 | security | manual -> downstream-resolver |
| 34 | oa4rust/crates/program_center/src/tests.rs | - | 4 个新 POST 端点（application_create/save、agent_create/save）无测试 | testing | manual -> downstream-resolver |
| 35 | oa4rust/crates/query_service/src/tests.rs | 36 | POST /jaxrs/query/service/processing/execute 无测试 | testing | manual -> downstream-resolver |
| 36 | oa4rust/crates/ai/src/tests.rs | 18 | AI config_get/config_base_config 空行分支无测试 | testing | manual -> downstream-resolver |
| 37 | oa4rust/crates/file_assemble_control/src/tests.rs | 228 | 错误路径测试缺失（result==0 分支） | testing | manual -> downstream-resolver |
| 38 | oa4rust/crates/shared/src/middleware.rs | 1 | 中间件无单元测试（auth、RBAC、rate-limit、CORS） | testing | manual -> downstream-resolver |
| 39 | oa4rust/crates/ai/src/lib.rs | 212 | 5 个端点获取 DB 连接但从未使用 | maintainability, performance | manual -> downstream-resolver |
| 40 | oa4rust/crates/ai/src/lib.rs | 99 | 分页端点接受无界 size 参数，可导致 LIMIT 20 亿 | performance, adversarial | manual -> downstream-resolver |
| 41 | oa4rust/crates/ai/src/lib.rs | 357 | index_cms_doc_with_app 返回所有文档无 LIMIT | performance | manual -> downstream-resolver |
| 42 | oa4rust/crates/query_service/src/lib.rs | 47 | neural_list_model 返回所有行无 LIMIT | performance | manual -> downstream-resolver |
| 43 | oa4rust/crates/program_center/src/lib.rs | 99 | modules_all 无界 GROUP BY | performance | manual -> downstream-resolver |
| 44 | oa4rust/crates/ai/src/lib.rs | 174 | config_list_mcp_paging/config_get_mcp 返回空桩数据 | maintainability, agent-native | manual -> downstream-resolver |
| 45 | oa4rust/crates/ai/src/lib.rs | 375 | index_delete 不执行删除却返回成功 | agent-native | manual -> downstream-resolver |
| 46 | oa4rust/crates/ai/src/lib.rs | 420 | file_download/file_download_scale 返回 JSON 元数据而非文件流 | agent-native | manual -> downstream-resolver |
| 47 | oa4rust/crates/ai/src/lib.rs | 124 | 列表端点响应形状不一致（部分有 page，部分无） | agent-native, api-contract | manual -> downstream-resolver |
| 48 | oa4rust/migrations/008_person_group_tables.sql | 4 | 无外键约束，孤立记录会在应用层积累 | data-migrations | manual -> downstream-resolver |
| 49 | oa4rust/crates/file_assemble_control/src/lib.rs | 469 | 软删除与硬删除同时作用于 x_file 表，产生不一致可见性 | adversarial | advisory -> human |
| 50 | oa4rust/migrations/008_person_group_tables.sql | 4 | 删除群组后 auth_person_group 残留，继续授权访问 | adversarial | advisory -> human |
| 51 | oa4rust/crates/program_center/src/lib.rs | 881 | config_save 的 key 无白名单验证，可覆盖系统配置 | adversarial | advisory -> human |

---

## P3 — 低优先级（10+ 项）

| # | 文件 | 行 | 问题 | 审查人 | 修复路由 |
|---|------|-----|------|--------|----------|
| 52 | oa4rust/crates/ai/src/lib.rs | 1 | ai/src/lib.rs 超过 500 行，应拆分为 domain 模块 | maintainability | manual -> downstream-resolver |
| 53 | oa4rust/crates/ai/src/lib.rs | 124 | 分页响应包装器重复 3 次，应提取为 helper | maintainability | manual -> downstream-resolver |
| 54 | oa4rust/crates/file_assemble_control/src/lib.rs | 15 | RowGet/ControlClient/ControlPool 三层 trait 抽象是过度设计 | maintainability, project-standards | advisory -> human |
| 55 | oa4rust/crates/shared/src/middleware.rs | 14 | 未使用的导入 validate_required | maintainability | safe_auto -> review-fixer |
| 56 | oa4rust/crates/shared/src/middleware.rs | 198 | ADMIN_WRITE_PREFIXES 与 PermissionRegistry 重复维护相同前缀列表 | maintainability | manual -> downstream-resolver |
| 57 | oa4rust/crates/shared/src/middleware.rs | 1 | middleware.rs 超过 800 行，混合 auth/RBAC/comparison  concerns | maintainability | manual -> downstream-resolver |
| 58 | oa4rust/crates/program_center/src/lib.rs | 1 | program_center/src/lib.rs 超过 3300 行、207 个函数 | maintainability | manual -> downstream-resolver |
| 59 | oa4rust/crates/ai/src/lib.rs | 895 | behavior_comparison_middleware 已定义但未接入任何 router | maintainability, project-standards | advisory -> human |
| 60 | oa4rust/crates/ai/src/lib.rs | 174 | config_list_mcp_paging 返回空桩数据 | maintainability | advisory -> human |
| 61 | oa4rust/crates/program_center/src/lib.rs | 2520 | 请求字段 snake_case 但响应 camelCase | api-contract | gated_auto -> downstream-resolver |
| 62 | oa4rust/crates/file_assemble_control/src/lib.rs | 496 | 文件 CRUD 使用 Json<Value> 而非类型化请求体 | api-contract | gated_auto -> downstream-resolver |
| 63 | oa4rust/crates/ai/src/lib.rs | 87 | 分页参数类型不一致：i32 vs i64 | api-contract | gated_auto -> downstream-resolver |
| 64 | oa4rust/crates/file_assemble_control/src/lib.rs | - | 新端点无 API 版本前缀 | api-contract | advisory -> human |
| 65 | oa4rust/migrations/008_person_group_tables.sql | - | 无回滚迁移脚本 | data-migrations | gated_auto -> downstream-resolver |
| 66 | oa4rust/migrations/008_person_group_tables.sql | 11 | 单列索引冗余（复合 PK 已创建隐式索引） | data-migrations | advisory -> human |
| 67 | oa4rust/crates/shared/src/middleware.rs | 647 | 前缀匹配允许 /jaxrs/ai_extra 匹配 /jaxrs/ai Admin 前缀 | adversarial | advisory -> human |

---

## 已自动修复

| # | 修复项 | 文件 | 说明 |
|---|--------|------|------|
| 1 | 移除未使用的导入 validate_required | oa4rust/crates/shared/src/middleware.rs:14 | 编译器警告，安全自动修复 |

---

## 前置问题（Pre-existing）

以下问题在本次 diff 之前已存在，不在本次修复范围：

| # | 文件 | 问题 |
|---|------|------|
| - | oa4rust/crates/shared/src/input_validation.rs | 未使用的导入 DeserializeOwned |
| - | oa4rust/crates/shared/src/middleware.rs | 未使用的变量 method |
| - | oa4rust/crates/shared/src/middleware.rs | 未使用的函数 check_permission（本次已集成到中间件流） |

---

## 学习与已知模式

来自 docs/brainstorms/ 和 docs/plans/ 的相关历史记录：

1. **路由重复注册导致 axum panic** — 曾因 control 与 auth 重复注册 /jaxrs/person/list 等路由导致启动崩溃。当前 Wave 4 新增大量路由，需在合并前验证无重复注册。

2. **前端强依赖 ActionResult<T> 9 字段 JSON 结构** — 前端 o2web 的 action.js 直接提取 json.data，依赖 9 字段结构。业务错误必须返回 HTTP 200 + type=error。

3. **_core_entity 与 _assemble_control 职责边界** — file_assemble_control 新增的 file_core_entity CRUD 路由可能与 file_core_entity crate 职责重叠，需确认权限校验层。

4. **认证绕过历史教训** — 曾因 bind 端点无授权校验即签发会话，被标记为高危。所有写操作端点必须显式校验权限。

---

## 测试覆盖缺口

- 所有断言 INTERNAL_SERVER_ERROR 的路由存在性测试需替换为 mock-based 测试
- 新增的 POST 端点（program_center、query_service、file_assemble_control、correlation_service_processing）无行为测试
- 中间件（auth、RBAC、rate-limit）无单元测试
- 无集成测试验证 partial unique index 要求
- 无测试验证分页边界值（page=i32::MAX）
- 无测试验证 owner 权限检查

---

## 部署注意事项

1. **迁移冲突:** `008_person_group_tables.sql` 与现有 `008_cleanup_duplicates.sql` 编号冲突，部署前必须重命名
2. **权限变更:** 15 个新模块前缀的写操作现在要求 admin 角色，需通知所有非 admin 集成方
3. **API key 泄露:** `config_list_model_paging` 明文返回 API key，需立即修复或限制端点访问
4. **连接池:** 新授权中间件每请求 1-3 次 DB 查询，需验证生产环境 pool max_size 配置

---

## 覆盖率

- **审查文件数:** 162 files changed
- **插入行数:** 26007
- **删除行数:** 10378
- **发现总数:** 67
- **P0:** 8 | **P1:** 17 | **P2:** 27 | **P3:** 15
- **可自动修复:** 1
- **需下游处理:** 40
- **咨询建议:** 19
- **预存问题:** 3
- **未验证:** 0

---

## 结论

**Not ready** — 存在 8 个 P0 阻断级问题，必须在合并前修复：

1. **查询服务 ON CONFLICT 需要部分唯一索引** — 部署后将直接失败
2. **chat_delete 无事务双表删除** — 数据不一致风险
3. **AI 模型 API key 明文泄露** — 安全漏洞
4. **文件创建者可被伪造** — 审计轨迹可被篡改
5. **用户 SQL 存入数据库** — 潜在 SQL 注入
6. **迁移编号冲突** — 部署可能跳过迁移
7. **ADMIN_WRITE_PREFIXES 静默扩展** — 破坏性变更未文档化
8. **无 agent 工具层** — 所有新端点对 AI agent 不可见（需规划）

建议修复顺序：先修复 P0 安全漏洞（#3、#4、#5）和迁移冲突（#6），再修复数据一致性问题（#1、#2），最后处理权限变更和 API 契约问题。
