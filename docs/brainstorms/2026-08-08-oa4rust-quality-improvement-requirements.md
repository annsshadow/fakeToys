# OA4Rust 质量改进与合并准入

**日期：** 2026-08-08
**依据：** `oa4rust-code-review-2026-08-08.md`（全量代码审查报告）
**迁移状态：** 81 个 crate 全部已完成真实化，7624 个路由已注册

---

## Summary

在 81 个 crate 全部迁移完成的背景下，按代码审查报告优先级全量推进质量改进工作，7 个 P0 问题全部修复作为合并准入硬条件，同时补齐测试覆盖、修复安全漏洞、消除虚假信心断言，确保 oa4rust 达到可合并、可上线标准。

---

## Problem Frame

O2OA 的 Java 后端正在迁移到 Rust（oa4rust）。2026-08-08 的全量代码审查确认所有 81 个 crate 已完成真实化，但暴露了 67 个问题：7 个 P0 阻断级（安全风险、数据不一致、部署隐患）、16 个 P1 高优先级、20+ 个 P2 中优先级、10+ 个 P3 低优先级。当前状态是"功能完成但质量未达标"——如果带着这些缺陷合并，将面临安全漏洞、部署失败、测试虚假信心等风险。

---

## Requirements

**[P0 阻断级修复 — 合并硬准入]**
- R1. 修复 `query_service` ON CONFLICT 部分唯一索引缺失问题（`crates/query_service/src/lib.rs:116`），确保部署后不报错
- R2. 为 `chat_delete` 添加事务保护，防止双表 DELETE 失败导致数据不一致（`crates/ai/src/lib.rs:305`）
- R3. 修复 AI 模型 API key 明文泄露：`config_list_model_paging` 需对 API key 字段脱敏（`crates/ai/src/lib.rs:116`）
- R4. 修复 `create_file_entity` 的 creator 字段伪造漏洞，移除用户可控的 creator 字段，改用认证上下文获取（`crates/file_assemble_control/src/lib.rs:506`）
- R5. 修复 `processing_execute` 的 SQL 注入风险：禁止将用户原始 SQL 存入数据库，需改为参数化查询或存储过程调用（`crates/query_service/src/lib.rs:100`）
- R6. 修复迁移编号冲突：`migrations/008_person_group_tables.sql` 与现有 `008_cleanup_duplicates.sql` 冲突，需重命名（`migrations/` 目录）
- R7. 修复 `ADMIN_WRITE_PREFIXES` 静默扩展问题：从 4 个扩展到 15 个属于破坏性变更，需文档化并通知集成方，或回滚为最小集合并逐步扩展

**[P1 高优先级修复]**
- R8. 修复 `update_file_entity` 的 COALESCE 逻辑错误：size=0 时会被错误覆盖（`crates/file_assemble_control/src/lib.rs:540`）
- R9. 修复分页 offset 计算中 i32 转 i64 的溢出风险（`crates/ai/src/lib.rs:91`）
- R10. 修复 `modules_all` 的 entityCount 语义变更：字段数 ≠ 实体数，需恢复原语义（`crates/program_center/src/lib.rs:118`）
- R11. 为 `chat_delete`/`file_delete` 添加所有权检查，防止跨用户删除（`crates/ai/src/lib.rs:338`）
- R12. 实现 `PermissionLevel::Owner` 分支，当前为空实现永不执行（`crates/shared/src/middleware.rs:727`）
- R13. 为 `person_has_group` 添加软删/禁用过滤（`person_has_role` 已有但 `group` 缺失）（`crates/shared/src/middleware.rs:536`）
- R14. 修复所有断言 INTERNAL_SERVER_ERROR 的路由存在性测试，替换为 mock-based 测试（`crates/ai/src/tests.rs:34` 及 6+ 个 crate 同类问题）
- R15. 修复 `MockControlClient::ctrl_query` 对 Rows 变体返回空 vec 的 mock 失效问题（`crates/file_assemble_control/src/tests.rs:103`）
- R16. 修复 DB 宕机时授权检查降级为 deny 的问题，添加 graceful fallback（`crates/shared/src/middleware.rs:334`）
- R17. 修复新模块未注册时继承 Authenticated 而非 Admin 的权限降级问题（`crates/shared/src/middleware.rs:193`）
- R18. 修复 `ON CONFLICT DO UPDATE` 返回新生成 UUID 但实际 DB 行保留旧 ID 的语义错误（`crates/query_service/src/lib.rs:109`）
- R19. 优化授权中间件性能：每请求 1-3 次 DB 查询，需添加缓存或批量查询（`crates/shared/src/middleware.rs:677`）
- R20. 消除 `is_admin` 重复调用（最多 5 次/请求），添加请求级缓存（`crates/shared/src/middleware.rs:693`）
- R21. 修复 `ApplicationCreateRequest` 字段命名不一致：请求 snake_case，响应 camelCase（`crates/program_center/src/lib.rs:2520`）
- R22. 修复 `list_enable_model` 响应语义：name 返回内部 xname 而非显示名（`crates/ai/src/lib.rs:485`）
- R23. 为 ActionResult 错误响应添加结构化错误码，支持 agent 程序化决策（`crates/ai/src/lib.rs`）
- R24. 修复 403 响应提供缺失角色信息，便于 agent 感知（`crates/shared/src/middleware.rs:777`）

**[P2 中优先级修复]**
- R25. 修复混合类型 OR 谓词隐式类型转换风险（`crates/ai/src/lib.rs:143`）
- R26. 将 Stub 端点从返回硬编码成功改为返回 501/404（`crates/file_assemble_control/src/lib.rs:592`）
- R27. 统一 `delete_file_entity` 与 `delete_file` 的删除策略（硬删除 vs 软删除）（`crates/file_assemble_control/src/lib.rs:573`）
- R28. 修复 `update_file_entity` 响应回显请求值而非持久化值（`crates/file_assemble_control/src/lib.rs:554`）
- R29. 限制 behavior-comparison 中间件记录完整响应体大小（当前最多 4KB）（`crates/shared/src/middleware.rs:948`）
- R30. 优化 is_admin 时序侧信道：写请求缓存结果避免每次查 DB（`crates/shared/src/middleware.rs:354`）
- R31. 修复用户可控文件路径未过滤的路径遍历风险（`crates/file_assemble_control/src/lib.rs:503`）
- R32. 限制聊天端点敏感数据暴露（`crates/ai/src/lib.rs:280`）
- R33. 补充新增 POST 端点的测试：program_center 4 个、query_service 1 个、file_assemble_control 1 个（`crates/*/src/tests.rs`）
- R34. 补充 AI 端点的空行分支测试（`crates/ai/src/tests.rs:18`）
- R35. 补充 file_assemble_control 错误路径测试（`crates/file_assemble_control/src/tests.rs:228`）
- R36. 为中间件添加单元测试：auth、RBAC、rate-limit、CORS（`crates/shared/src/middleware.rs:1`）
- R37. 移除 ai crate 中 5 个获取 DB 连接但未使用的冗余代码（`crates/ai/src/lib.rs:212`）
- R38. 为分页端点添加无界 size 参数上限（当前可导致 LIMIT 20 亿）（`crates/ai/src/lib.rs:99`）
- R39. 为 `index_cms_doc_with_app` 添加 LIMIT（`crates/ai/src/lib.rs:357`）
- R40. 为 `neural_list_model` 添加 LIMIT（`crates/query_service/src/lib.rs:47`）
- R41. 为 `modules_all` 的 GROUP BY 添加 LIMIT（`crates/program_center/src/lib.rs:99`）
- R42. 修复 config_save 的 key 无白名单验证，可覆盖系统配置（`crates/program_center/src/lib.rs:881`）
- R43. 为 migration 008 添加外键约束，防止孤立记录（`migrations/008_person_group_tables.sql:4`）
- R44. 修复删除群组后 auth_person_group 残留问题（`migrations/008_person_group_tables.sql`）

**[P3 低优先级改进]**
- R45. 拆分 `ai/src/lib.rs`（超 500 行）为 domain 模块
- R46. 提取分页响应包装器重复代码为 helper 函数
- R47. 移除 `file_assemble_control` 的三层 trait 抽象（RowGet/ControlClient/ControlPool），简化为两层
- R48. 消除 `middleware.rs` 中未使用的导入和变量
- R49. 消除 ADMIN_WRITE_PREFIXES 与 PermissionRegistry 的重复维护，统一为单一数据源
- R50. 拆分 `middleware.rs`（超 800 行）为 auth_middleware、rbac_middleware、rate_limit_middleware 等子模块
- R51. 拆分 `program_center/src/lib.rs`（超 3300 行、207 个函数）为 submodules
- R52. 接入已定义的 behavior_comparison_middleware 到 router（当前已定义但未使用）（`crates/ai/src/lib.rs:875`）
- R53. 修复 `config_list_mcp_paging` 返回空桩数据，实现真实查询
- R54. 修复 `index_delete` 不执行删除却返回成功的问题
- R55. 修复 `file_download`/`file_download_scale` 返回 JSON 元数据而非文件流的问题
- R56. 统一列表端点响应形状，部分有 page 部分无
- R57. 为 migration 008 添加回滚迁移脚本
- R58. 移除 migration 008 的冗余单列索引（复合 PK 已创建隐式索引）
- R59. 修复前缀匹配过宽问题：`/jaxrs/ai_extra` 不应匹配 `/jaxrs/ai` Admin 前缀（`crates/shared/src/middleware.rs:647`）
- R60. 为新增端点添加 API 版本前缀

---

## Acceptance Examples

- AE1. **覆盖 R1, R5, R6.** 部署后运行 `cargo test --workspace --lib` 全部通过；`migrations/` 下无编号冲突；`query_service` 的 ON CONFLICT 在测试数据库中成功执行不报错。
- AE2. **覆盖 R3, R4.** `GET /jaxrs/ai/model/list` 返回的 API key 字段为 `***` 或 null；`POST /jaxrs/file/entity/create` 的 creator 字段来自认证上下文而非请求体。
- AE3. **覆盖 R14, R15, R33-R36.** 所有路由存在性测试使用 mock-based 断言成功响应（200 + type=success）；新增 POST 端点有对应的行为测试；中间件有独立的单元测试。
- AE4. **覆盖 R7.** `ADMIN_WRITE_PREFIXES` 列表在代码中明确注释来源和变更历史；前端或集成方收到变更通知。
- AE5. **覆盖 R11, R12.** 用户 A 尝试删除用户 B 的文件返回 403；`POST /jaxrs/person/permission/set` 使用 Owner 权限时正确执行所有权校验。
- AE6. **覆盖 R19, R20, R38-R41.** 授权中间件在高并发（100 QPS）下 P99 延迟 < 50ms；所有分页端点拒绝 size > 1000 的请求；无 LIMIT 的查询全部添加合理上限。

---

## Success Criteria

- **人工验收：** 7 个 P0 问题全部修复并通过代码审查复核；`cargo test --workspace` 全部通过且无 INTERNAL_SERVER_ERROR 断言；测试覆盖率提升至 80%+（当前估算约 45%）。
- **下游 agent 验收：** `docs/brainstorms/oa4rust-migration-status-2026-08-08.md` 更新为"合并就绪"；无 P0/P1 遗留问题；所有新增端点均有测试覆盖。

---

## Scope Boundaries

- **本次包含：** 代码审查 67 个发现的所有修复；测试覆盖补齐；API 契约一致性修复
- **排除在外（不在本次范围）：** Java 后端的修改；前端 o2web 的修改；新的业务功能开发；架构重设计（如微服务拆分）；数据库 schema 重新设计

---

## Key Decisions

- **P0 全量修复作为合并前提：** 不允许部分修复后合并，7 个 P0 必须全部解决才能进入合并流程
- **按审查优先级全量推进：** 不选择性跳过任何级别，P0→P1→P2→P3 依次修复，不跳过中间级别
- **测试覆盖补齐独立于功能修复：** 虚假信心断言（INTERNAL_SERVER_ERROR 断言）是独立工作项，即使功能代码正确也需修复
- **中间件安全优先于性能优化：** R16（DB 宕机 fallback）和 R17（权限降级）优先于 R19/R20（性能优化）

---

## Dependencies / Assumptions

- 所有修复基于 `oa4rust-code-review-2026-08-08.md` 审查报告，假设报告内容准确且覆盖完整
- PostgreSQL 连接池配置（max_size）需在生产环境验证以支持授权中间件的 DB 查询
- 前端 o2web 对 ActionResult 9 字段 JSON 结构的依赖保持不变，修复不影响前端

---

## Outstanding Questions

### Resolve Before Planning

- [Affects R6][User decision] 迁移 008 重命名方案：改为 `009_person_group_tables.sql` 还是 `008b_person_group_tables.sql`？需确认与现有迁移系统的兼容性
- [Affects R7][User decision] ADMIN_WRITE_PREFIXES 扩展策略：回滚到 4 个最小集合并文档化增量扩展计划，还是保留 15 个并通知所有集成方？

### Deferred to Planning

- [Affects R45, R50, R51][Technical] 大文件拆分的边界划分策略（按 domain 还是按 concern）
- [Affects R19, R20][Needs research] 授权中间件缓存策略：Redis 缓存还是内存 LRU？TTL 设置多少合适？
- [Affects R31][Needs research] 文件路径遍历攻击的实际风险评估（当前是否暴露在公网）
