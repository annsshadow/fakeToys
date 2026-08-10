# OA4Rust 剩余工作量清单

**更新时间：** 2026-08-10（已更新）
**总览：** 81 个 crate 全部完成真实化，SeaORM 迁移完成

---

## 已完成真实化的 crate

以下 crate 均已完成 PostgreSQL 真实查询实现，handler 中包含真实业务逻辑：

| crate 名称 | 路由数 | 核心端点 |
|------------|--------|----------|
| ai | 6 | model/config/app/conversation 等真实查询 |
| ai_core_entity | 3 | app/model/conversation 列表真实查询 |
| cms_express | 3 | uuid/template/view 查询全部真实 |
| correlation_core_entity | 2 | 关联关系 list/list_by_source 真实查询 |
| file_core_entity | 4 | folder/file list/complex 查询全部真实 |
| organization_core_entity | 6 | definition/group/identity/person/custom/bind 列表全部真实 |
| program_center_core_entity | 39 | application/script/invoke/agent/structure CRUD 全部真实 ✅ |
| query_express | 1 | query list 真实查询 |

---

## 已完成的后续工作（2026-08-09 ~ 2026-08-10）

- [x] SeaORM 全量迁移（81 个 crate，`feat/seaorm-migration` 分支，7 个 commit）
- [x] 写操作补齐（19 个 core_entity crate，~76 个 POST/PUT/DELETE handler）
- [x] program_center_core_entity 写操作（application/script/invoke/agent/structure CRUD）
- [x] IDOR 安全修复（`require_owner` 检查，commit `869188d9`）
- [x] creator_person 字段注入（migration 012）
- [x] 输入验证（validate_name/validate_text 助手，commit `0f66c101`）
- [x] nested tokio runtime panic 修复（`catch_unwind` 降级，commit `ba8d1368`）
- [x] 计划文档状态清理（completed/superseded）
- [x] docs/oa/ 文档完善（55 模块卡片 + 86 组件卡片 + 58 API 文档）

---

## 当前剩余工作

| 工作项 | 优先级 | 说明 |
|--------|--------|------|
| SQLx 完全移除 | 低 | ORM 为默认路径，复杂查询可保留 SQLx 并存 |
| 数据库连接池优化 | 低 | Deadpool 参数调优 |
| ORM 层支持多数据库后端 | 低 | 规划中 |
| 前端兼容性端到端验证 | 中 | 需与 o2web 联调 |
| behavior_compare 测试全量覆盖 | 中 | 当前 ~79/7624 端点，需扩展至全量 |
| `openapi` crate 端点注解补全 | 低 | 当前 14 个占位，7624 端点缺失 |
| `docs/oa/modules/o2web/` 组件卡片 Responsibility 填充 | 已完成 | ✅ 2026-08-10 |
| `docs/oa/reference/data-models.md` 实体关系图完善 | 已完成 | ✅ 2026-08-10 |

---

## 参考

- **迁移状态：** `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`
- **计划文档：** `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md`
- **生产就绪：** `docs/plans/2026-08-10-001-prod-readiness-plan.md`
