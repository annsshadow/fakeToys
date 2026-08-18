---
date: 2026-08-10
topic: oa4rust-production-readiness-and-docs-cleanup
---

# OA4Rust 生产就绪与文档清理

## Summary

清理已过期计划的 status，补全 `program_center_core_entity` 缺失的写操作，以及完善 `docs/oa/` 文档体系中空白的人工作业内容，使 oa4rust 达到生产就绪状态且团队文档完整可用。

---

## Problem Frame

oa4rust 项目在 2026-08-09 已完成全部 81 个 crate 的真实化和 SeaORM 迁移，`cargo test --workspace --lib` 全部通过。但存在三个维度的生产就绪缺口：

**计划目录混乱。** `docs/plans/` 中存在 3 个已被后续计划完全取代的旧计划（2026-08-05 comprehensive-advancement、2026-08-06 full-realization、2026-08-03 o2server-rust-rewrite）仍标记为 `status: active`，以及 2 个已完成但未更新 status 的计划（2026-08-07 4-wave-realization、2026-08-09 orm-migration），造成规划追踪失真。

**写操作不完整。** 8 个 `core_entity` crate 中 7 个已补全 POST/PUT/DELETE 写操作，但 `program_center_core_entity` 仅有 5 个 GET 查询端点（application/script/invoke/agent/structure list），缺少完整的 CRUD 能力。这意味着程序中心模块在前端只能查看不能编辑。

**文档骨架完整但内容空白。** `docs/oa/` 目录已建立完整的脚手架结构（README、架构、模块卡片、API 文档、部署指南、业务功能指南），Python 生成脚本也已就位。但 55 张 o2server 模块卡片的 "Responsibility" 字段全部为空（脚本仅生成骨架），API 自动提取仅覆盖 14/55+ 模块，新成员无法通过文档快速了解系统。

---

## Actors

- **A1（开发者，单人）：** 负责写操作实现、文档内容填充、计划清理
- **A2（新加入开发者）：** 文档的目标读者，需要通过文档快速建立系统心智模型
- **A3（维护者）：** 后续负责文档持续更新

---

## Requirements

**计划清理**
- R1. 将 `docs/plans/2026-08-07-001-feat-oa4rust-4wave-realization-plan.md` 和 `docs/plans/2026-08-09-001-refact-oa4rust-orm-migration-plan.md` 的 `status` 从 `active` 更新为 `completed`
- R2. 将 `docs/plans/2026-08-05-001-feat-oa4rust-comprehensive-advancement-plan.md`、`docs/plans/2026-08-06-001-feat-oa4rust-full-realization-plan.md`、`docs/plans/2026-08-03-001-refactor-o2server-rust-rewrite-plan.md` 的 `status` 更新为 `superseded`，并在文件头部添加注释说明被哪个计划取代
- R3. 将 `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md` 的 `status` 更新为 `completed`（delivery.md 已确认所有单元完成）

**program_center_core_entity 写操作补全**
- R4. 为 `program_center_core_entity` crate 的 5 个实体（application、script、invoke、agent、structure）补全 POST/PUT/DELETE 写操作端点
- R5. 每个写操作端点包含参数验证（必填字段、类型检查）和软删除过滤（`deleted_at IS NULL`）
- R6. 写操作响应遵循 `ActionResult<T>` 9 字段结构，与现有端点保持一致
- R7. 为新增写操作端点添加集成测试，覆盖 happy path 和 error path
- R8. `cargo test --workspace --lib` 全部通过

**OA 文档内容完善**
- R9. 为 `docs/oa/modules/o2server/` 下所有 55 张模块卡片的 "Responsibility" 字段填充人类编写的内容（每卡 1-3 句话描述模块职责）
- R10. 为有 REST 端点的模块卡片填充 "REST Endpoints" 字段（基于 `oa/o2server/o2_core/o2/xAction/services/` 中的 action JSON 文件）
- R11. 扩展 `docs/oa/scripts/generate_api_docs.py` 覆盖全部 55+ 个 o2server 模块的 API 文档生成
- R12. 验证 `docs/oa/` 下所有链接可正常解析，README 目录与现有文件一致

---

## Acceptance Examples

- AE1. **Covers R1, R2, R3.** 运行 `grep "status:" docs/plans/*.md` 后，无 `active` 状态的过期计划，已完成计划标记为 `completed`，已取代计划标记为 `superseded`。
- AE2. **Covers R4, R6.** 对 `program_center_core_entity` 的 application 实体发送 POST 请求创建新记录，响应包含 `type=success` 和创建后的 application 数据；发送 DELETE 请求后该记录在 list 查询中不再出现（软删除生效）。
- AE3. **Covers R7.** 对 `program_center_core_entity` 新增的写操作端点运行集成测试，全部通过。
- AE4. **Covers R9, R10.** 随机抽查 5 张模块卡片，每张的 "Responsibility" 字段包含非空的职责描述，有 REST 端点的卡片包含端点列表。
- AE5. **Covers R11, R12.** 运行 `docs/oa/scripts/generate_api_docs.py` 后 `docs/oa/api/auto/` 下生成 55+ 个模块的 API 文档；`docs/oa/README.md` 中所有链接可解析。

---

## Success Criteria

- `docs/plans/` 中无过期 `active` 状态的计划文件
- `program_center_core_entity` crate 包含完整的 CRUD 端点，`cargo test --workspace --lib` 通过
- `docs/oa/modules/o2server/` 中所有模块卡片的 Responsibility 字段非空
- `docs/oa/api/auto/` 覆盖全部 55+ 个 o2server 模块
- 新成员阅读 `docs/oa/README.md` 后能在 10 分钟内建立系统整体心智模型

---

## Scope Boundaries

- **包含：** 计划 status 更新与归档注释；`program_center_core_entity` 的写操作实现与测试；`docs/oa/` 模块卡片内容填充与 API 文档扩展
- **排除在外：** 其余 7 个 core_entity crate 的写操作（已补全）；Session 持久化（已实现）；全量行为对比测试扩展（已完成）；SQLx 完全移除（按计划策略保留并存）；`docs/oa/` 目录结构重建（已有脚手架）

### Deferred for later

- `docs/oa/modules/o2web/` 下 86 张组件卡片的 Responsibility 填充
- `openapi` crate 中 14 个未使用函数的清理
- CI 中集成文档生成脚本的定期刷新
- `docs/oa/reference/data-models.md` 中实体关系图的完善

---

## Key Decisions

- **只补全 program_center_core_entity 写操作：** 其余 7 个 core_entity crate 已有完整 CRUD，无需重复工作。program_center_core_entity 是唯一遗漏的。
- **文档填充优先 Responsibility 而非架构调整：** `docs/oa/` 的目录结构已完整，只需填充内容。不重新设计文档体系。
- **计划清理通过更新 frontmatter status 实现：** 不删除文件，保留历史追溯。已取代计划保留但标记 `superseded` 并说明被哪个计划取代。
- **API 文档使用 action JSON 作为数据源：** `generate_api_docs.py` 从 `oa/o2web/source/o2_core/o2/xAction/services/*.json` 提取端点信息，不依赖 Swagger（覆盖率不足）。

---

## Dependencies / Assumptions

- `program_center_core_entity` 的数据库表结构已随 SeaORM 迁移完成，entity 定义存在于 `crates/program_center_core_entity/src/entities/`
- `oa/o2server/o2_core/o2/xAction/services/` 中的 action JSON 文件可作为 API 文档的数据源
- 新成员的主要阅读路径是 `docs/oa/README.md` → 架构 → 模块卡片，因此模块卡片的 Responsibility 字段是最优先填充的内容

---

## Outstanding Questions

### Resolve Before Planning

- [Affects R4] `program_center_core_entity` 的写操作权限级别：参考 `control` crate 的模式，application/script 的写操作是否需要 Admin 权限，还是 Authenticated 即可？

### Deferred to Planning

- [Affects R9] 模块卡片 Responsibility 的具体内容深度：每卡 1-3 句还是更详细？
- [Affects R11] API 文档生成脚本是否需要支持增量刷新（仅更新有变化的模块）
