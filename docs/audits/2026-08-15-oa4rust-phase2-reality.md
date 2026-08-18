# OA4Rust Phase 2 现实盘点（纠正 002 分析文档的过时前提）

- **日期**：2026-08-15
- **依据规划**：`docs/plans/2026-08-13-003-oa4rust-completion-plan.md`
- **纠正对象**：`docs/plans/2026-08-13-002-oa4rust-o2server-parity-analysis.md`（已严重过时）

## 0. 结论摘要

规划文档的前提（依赖 002）已失真，必须用真实测量数据替换：

1. **Phase 0 / Phase 1 出关判定已达成**（前序会话验证）：
   - `cargo build --workspace` 零 error（仅 2 warning）。
   - `cargo test --workspace --lib` 全绿（前序 1185 passed；本次复跑 exit 0，无回归）。
   - `cargo test --test integration_runner -- --ignored` 全绿（本次新增 CMS CRUD 场景后仍 1 passed）。
   - 迁移自举、`gen_inventory` 报 83/83 done / 0 stub / 0 静默 null。

2. **002 声称的"201 处 Value::Null"已不复存在**：全仓 0 `Value::Null`、0 stub。**真正的缺口不是字面空数据，而是"已定义 handler 是否可达 + 是否真正查询数据库"**。

3. **真实口径（新测量脚本）**：
   - **已定义 handler 2633 个，仅 1085 个被路由（41.2%）**。剩余 ~1548 个为**未路由的死代码**，集中在少数巨型 crate。
   - **已定义 handler 中 78.2% 触及数据库**；但这是"已定义"口径，真正决定可测试性的是**已路由**集合。
   - `ldap` crate：**库已实现完整**（`LdapConfig` + `LdapAuthenticator` + `ldap3` 绑定），但 **0 路由、未合并进 root router** → U2.3 的真实缺口是"集成进认证流 + 测试 DB 回退契约"，而非"从零实现"。
   - `processplatform_*`（BPMN）：已定义 handler **100% 触及数据库**，但每个巨型 crate 仅路由 6–12 个端点 → 缺口是"路由接线"，不是"执行语义缺失"。

## 1. 测量方法

新增两个脚本（不污染源码，纯分析）：

- `scripts/measure_db_touch.py`：逐 crate 统计 `pub async fn` handler 数与其函数体内是否出现 DB 访问模式（`.query`/`.execute`/`Entity::find`/`SELECT`/`deadpool_postgres` 等），得 DB 接触率。
- `scripts/measure_routing.py`：逐 crate 统计 `pub async fn` 定义数与 `.route(` 注册数，得"路由率"，暴露未接线死代码。

## 2. 路由率（已定义 vs 已路由）— 核心发现

```
crate                                     defined  routed   route%
cms_assemble_control                        311       7     2.3%   <-- 死代码集中
program_center                             205       7     3.4%   <-- 死代码集中
processplatform_assemble_surface           487       7     1.4%   <-- 死代码集中
processplatform_assemble_designer           96       6     6.2%
processplatform_service_processing         100      12    12.0%
query_assemble_designer                     67       5     7.5%
query_assemble_surface                      59       9    15.3%
file_assemble_control                       94      12    12.8%
portal_assemble_designer                    56      12    21.4%
...
TOTAL                                     2633    1085    41.2%
```

**含义**："2600 handler" 的数字被少数巨型 crate 的未接线死代码严重放大。真正的可测试/可交付表面是 **~1085 个已路由端点**。Phase 2 的务实口径应从"已路由端点"出发，而非"已定义函数"。

## 3. DB 接触率（已定义口径）

整体 2633 定义 / 2060 触库 = **78.2%**。低接触率 crate（需重点核实"已路由部分"是否真的触库）：

```
crate                           handlers   db    rate
cms_assemble_control              311      19    6.1%   <-- 但仅 7 路由，壳在死代码区
auth                              45      19   42.2%
ldap                               1       0    0.0%   <-- 库在实现侧，0 路由
program_center                   205      69   33.7%
control                           25      17   68.0%
file_assemble_control             94      65   69.1%
attendance_assemble_control       89      88   98.9%
processplatform_* (全)          100%/100%  DB 接触率高（已定义口径）
```

注意：上述为"已定义"口径。由于巨型 crate 的死代码 handler 多为壳，其"已定义低接触率"主要反映死代码，而非已路由端点的真实质量。下一步应测量**已路由端点**的触库率（见第 6 节待办）。

## 4. 本会话已完成的真实化（可验证）

**U2.1 CMS 文档 CRUD 核心链路（端到端连库验证通过）**：

- 在 `crates/cms_assemble_control/src/lib.rs` 新增 4 个真实 DB handler（替换原先 `{"success":true}` 壳）：
  - `data_document_create`（POST，INSERT + `publish_time=NOW()`，缺省 uuid 生成）
  - `data_document_id_get`（GET by id，`deleted_at IS NULL` 过滤）
  - `data_document_id_update`（POST，`COALESCE` 仅更新非空字段）
  - `data_document_id_delete`（POST，软删除置 `deleted_at`）
- 复用既有真实 DB 助手 `list_from_table_filtered` 作为列表端点（原 `data_document_id` 壳虽触库但**未路由**，本次一并接线）。
- 在 `routes.rs` 接线 5 条端点（`/jaxrs/cms_assemble_control/data/document[/create|/{id}|/{id}/update|/{id}/delete]`）。
- 新增集成测试 `tests/integration_tests/scenarios/cms_document.rs`：`create→list(可见)→get(真实内容)→update→软删(列表消失 + get 404)`，**连真实 PostgreSQL 跑通**。
- 验证：`cargo test --test integration_runner -- --ignored` → `integration_scenarios ... ok`（1 passed）。

## 5. 修订后的 Phase 2 任务拆解（基于真实数据，不再套用 002）

| 单元 | 真实状态 | 本会话 | 下一步 |
|------|----------|--------|--------|
| U2.1 CMS 文档 CRUD 核心 | 壳/未路由 | **已完成并连库验证** | 扩展 appinfo/categoryinfo/comment/file/form/view 的增删改真实化 + 接线 |
| U2.1 CMS 其余 304 未路由 handler | 死代码 | 未动 | 按 o2server `@Path` 优先级分批接线 + 真实化（最大工程量） |
| U2.2 BPMN 工作流执行语义 | 已定义 100% 触库，但每 crate 仅路由 6–12 | 未动（待核实路由后语义） | 先接线核心流程端点，再补 agree/return/transfer/网关/定时器语义 + 端到端集成测试 |
| U2.3 LDAP | 库完整，0 路由/未合并 | 未动 | 接线进 auth 登录流（LDAP 优先、失败回退 DB）+ DB 回退契约测试 |
| U2.3b 企微/钉钉/验证码/OAuth2/SSO | 源码疑似存在（auth 42% 触库） | 未动 | 核实各端点是否路由 + 连库验证 |
| U2.4 program_center | 205 定义/7 路由（33.7% 触库） | 未动 | 接线 + 真实化核心实体 |
| U2.5 query/file/portal 已路由端点 | 路由率 12–21% | 未动 | 核实已路由端点触库质量，补真实化 |
| U2.6 查询/报表/门户深度验证 | 见上 | 未动 | 抽样审计 |

## 6. 待办（下一步优先）

1. **测量"已路由端点"的 DB 接触率**（修正 measure_routing 以映射 route→handler，或直接对 root router 做运行时路由遍历），得到真正决定覆盖率的口径。
2. 按表推进 U2.1 扩展 → U2.3 LDAP 集成 → U2.2 BPMN 接线与语义 → program_center/query/file 真实化。
3. Phase 3-4（多库/分布式/全文检索/IM/预览签章、parity 回归/灰度）维持"按需求裁剪"的务实最低可行替代点策略。

## 7. 风险登记更新

- 原风险"进度口径失真"**已坐实并量化**：gen_inventory 的 handler 计数包含未路由死代码，掩盖了真实可交付表面的缺口。本报告的路由率测量是纠偏工具，应纳入 CI 防回退。
- 新增风险：巨型 crate 的大量未路由 handler 若被当作"已实现"，会严重误导接管判定。建议以"已路由端点数 + 已路由触库率"作为接管核心指标。
