# OA4Rust 剩余工作量清单

**更新时间：** 2026-08-08
**总览：** 81 个 crate 全部完成真实化

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
| program_center_core_entity | 5 | application/script/invoke/agent/structure 列表全部真实 |
| query_express | 1 | query list 真实查询 |

---

## 后续工作建议

虽然全部 81 个 crate 已完成基础真实化，但仍可继续完善：

1. **补充写操作（POST/PUT/DELETE）** — 当前 8 个 crate 仅有 GET 查询，如需支持数据修改，需补充 POST/PUT/DELETE 路由
2. **补充集成测试** — 部分 crate 测试较少或仅验证路由可达性，需补充真实数据库集成测试
3. **端点覆盖对齐 Java** — 对照 Java JAX-RS 端点清单，补充未覆盖的业务端点
4. **权限控制细化** — 为新增端点配置细粒度访问控制（基于角色、用户组、资源所有者）

---

## 下一步行动

- [x] 完成 8 个无数据库查询 crate 的真实化
- [x] 更新 `docs/brainstorms/oa4rust-migration-status-2026-08-08.md`
- [ ] 按需补充写操作（POST/PUT/DELETE）
- [ ] 补充集成测试覆盖
- [ ] 对齐 Java 端点清单，补充未覆盖端点
