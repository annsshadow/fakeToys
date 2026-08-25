# final coverage sweep — plan002 U2 收官验证

- 生成时间：2026-08-24 23:17　|　清单：`java-endpoint-inventory.json`（generated_at=2026-08-23T13:57:31）
- 扫描对象：`crates/*/src/**/*.rs` 共 4573 条 `.route(` 注册（唯一归一化路径 4155 条），覆盖单行/多行注册、`&fmt()+JAVA_BASE` 间接寻址、链式多方法。
- 匹配口径：路径参数归一化为 `{}`；exact（method+全路径，允许 Rust 侧更长前缀）∪ casefold 计入覆盖；verb_mismatch / literal_shift 仅诊断不计入（影子副本会真实 404，见 alignment-reconciliation.md §2.3）。
- 留档排除项对照 `docs/plans/2026-08-21-002` 台账 U2 行。

## 一、结论速览

| 指标 | 数值 |
|------|------|
| 有端点的 Java 模块组 | 30 / 55 |
| Java 唯一端点（模块内去重合计，台账口径） | 3092（清单 totals=3092） |
| **总覆盖端点数（同口径）** | **3085** |
| **总覆盖率** | **99.77%** |
| 严格全局并集（跨模块去重后） | 2861/2868 = 99.76%（跨模块重叠 224 对，如 query designer/surface 共享 statement 族） |
| 模块内口径合计 | 3085/3092 = 99.8% |
| **达到 100% 的模块数** | **28** |
| 未覆盖端点总数 | 7 |
| 其中 axum 平台限制留档 | 4 |
| 其中 cms 语义不匹配留档 | 0 |
| **排除留档后剩余缺口** | **3** |

## 二、模块覆盖明细

| 模块 | 唯一端点 | 已覆盖 | 覆盖率 | 状态 |
|------|---------:|-------:|-------:|------|
| `x_bbs_assemble_control` | 106 | 105 | 99.1% | ⚠️ 缺口 |
| `x_processplatform_assemble_surface` | 659 | 653 | 99.1% | ⚠️ 缺口 |
| `x_ai_assemble_control` | 33 | 33 | 100.0% | ✅ 100% |
| `x_attendance_assemble_control` | 180 | 180 | 100.0% | ✅ 100% |
| `x_base_core_project` | 8 | 8 | 100.0% | ✅ 100% |
| `x_calendar_assemble_control` | 31 | 31 | 100.0% | ✅ 100% |
| `x_cms_assemble_control` | 437 | 437 | 100.0% | ✅ 100% |
| `x_component_assemble_control` | 7 | 7 | 100.0% | ✅ 100% |
| `x_correlation_service_processing` | 12 | 12 | 100.0% | ✅ 100% |
| `x_file_assemble_control` | 105 | 105 | 100.0% | ✅ 100% |
| `x_general_assemble_control` | 46 | 46 | 100.0% | ✅ 100% |
| `x_hotpic_assemble_control` | 12 | 12 | 100.0% | ✅ 100% |
| `x_jpush_assemble_control` | 9 | 9 | 100.0% | ✅ 100% |
| `x_meeting_assemble_control` | 76 | 76 | 100.0% | ✅ 100% |
| `x_message_assemble_communicate` | 64 | 64 | 100.0% | ✅ 100% |
| `x_mind_assemble_control` | 23 | 23 | 100.0% | ✅ 100% |
| `x_organization_assemble_authentication` | 53 | 53 | 100.0% | ✅ 100% |
| `x_organization_assemble_control` | 187 | 187 | 100.0% | ✅ 100% |
| `x_organization_assemble_express` | 132 | 132 | 100.0% | ✅ 100% |
| `x_organization_assemble_personal` | 76 | 76 | 100.0% | ✅ 100% |
| `x_portal_assemble_designer` | 64 | 64 | 100.0% | ✅ 100% |
| `x_portal_assemble_surface` | 38 | 38 | 100.0% | ✅ 100% |
| `x_processplatform_assemble_bam` | 45 | 45 | 100.0% | ✅ 100% |
| `x_processplatform_assemble_designer` | 117 | 117 | 100.0% | ✅ 100% |
| `x_processplatform_service_processing` | 121 | 121 | 100.0% | ✅ 100% |
| `x_program_center` | 252 | 252 | 100.0% | ✅ 100% |
| `x_program_init` | 15 | 15 | 100.0% | ✅ 100% |
| `x_query_assemble_designer` | 90 | 90 | 100.0% | ✅ 100% |
| `x_query_assemble_surface` | 70 | 70 | 100.0% | ✅ 100% |
| `x_query_service_processing` | 24 | 24 | 100.0% | ✅ 100% |

> 无 JAXRS 端点的模块（25 个，不计入分母）：`x_ai_core_entity`、`x_attendance_core_entity`、`x_bbs_core_entity`、`x_calendar_core_entity`、`x_cms_core_entity`、`x_cms_core_express`、`x_component_core_entity`、`x_console`、`x_correlation_core_entity`、`x_correlation_core_express`、`x_file_core_entity`、`x_general_core_entity`、`x_hotpic_core_entity`、`x_jpush_core_entity`、`x_meeting_core_entity`、`x_message_core_entity`、`x_mind_core_entity`、`x_organization_core_entity`、`x_organization_core_express`、`x_portal_core_entity`、`x_processplatform_core_entity`、`x_processplatform_core_express`、`x_program_center_core_entity`、`x_query_core_entity`、`x_query_core_express`

## 三、未覆盖端点（按模块分组，标注排除类别）

> 判定图例：🔴 缺失＝任何形态均无注册；🔵 动词差＝路径已有但缺该 HTTP 方法变体；🟣 形变疑云＝存在同段数形变候选（影子副本会真实 404，不计入覆盖）；🟠 平台限制＝axum 无法表达（单段多参数）；🟡 语义留档＝台账记录的语义不匹配。
### x_processplatform_assemble_surface（缺 6 / 659）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}` | 🔴 缺失 |  |
| GET | `/attachment/download/{}/work/{}/stream/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/download/{}/work/{}/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/download/{}/workcompleted/{}/stream/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/download/{}/workcompleted/{}/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` | 🔴 缺失 |  |

### x_bbs_assemble_control（缺 1 / 106）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/user/subject/acceptreply/{}/{}` | 🔴 缺失 |  |

## 四、排除留档后剩余缺口 Top 清单（本轮只列清单不实现）

| # | 模块 | 缺口数 | 构成（缺失/动词差/形变） | 代表端点 | 相关 crate | 难度 | 建议 |
|---|------|-------:|------------------|----------|-----------|------|------|
| 1 | `x_processplatform_assemble_surface` | 2 | 2/0/0 | GET `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}`<br>GET `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` | `processplatform_assemble_surface` | 中低（零星端点，逐条补齐） | 零星补齐：逐条仿既有 handler + 注册 |
| 2 | `x_bbs_assemble_control` | 1 | 1/0/0 | GET `/user/subject/acceptreply/{}/{}` | `bbs_assemble_control` | 中低（零星端点，逐条补齐） | 零星补齐：逐条仿既有 handler + 注册 |

### 附：axum 平台限制留档明细

| 模块 | 方法 | 路径 | 原因 |
|------|------|------|------|
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/work/{}/stream/{}.{}` | axum 平台限制：单段多参数路由不可表达 |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/work/{}/{}.{}` | axum 平台限制：单段多参数路由不可表达 |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/workcompleted/{}/stream/{}.{}` | axum 平台限制：单段多参数路由不可表达 |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/workcompleted/{}/{}.{}` | axum 平台限制：单段多参数路由不可表达 |

### 附：cms 语义不匹配留档明细

（无）

## 五、与台账 U2 口径对账与结论

1. **口径差异**：台账 U2 行的 92.8%（4195/4386）为**注解口径**（含变体与自有端点）；本终扫为**唯一端点口径**且匹配更严（verb_mismatch / literal_shift 不计入覆盖，影子路径会真实 404）。两口径不可直接相比。
2. **清单时点**：本清单 generated_at=2026-08-23T13:57:31，晚于多数模块闭合提交所依据的版本；v9 源树新增端点族（program_center agent/appstyle、query importmodel、calendar REST 族、portal/设计器新增族等）尚未同步注册——属**清单演进带来的新缺口**，并非此前闭合工作回退（attendance/cms/file/meeting/org 等此前闭合模块本次均复测 100%）。
3. **attachment 4 条平台限制**：与本扫描的自动检测（单段多参数 `{}.{}` 段）逐条一致，见附录明细。
4. **cms「深层语义不匹配」留档**：路由层面 cms 已 437/437 全覆盖，该留档属 handler 行为层（响应语义/深层业务一致性），不在端点注册扫描范围，故本轮无需排除项。
5. **BAM（x_processplatform_assemble_bam）**：台账已注明的 P3 真实大缺口，实测缺 0 条监控类低频端点，维持挂起建议。
6. **动词差批量项**：全仓共 0 条仅需补方法变体（路径已存在），是性价比最高的收敛手段。

---

## 相关文档

- **收官复盘：** `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- **残差需求（仍不能完全替代之处）：** `docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md`
- **执行计划（U2 收官、仍为 active）：** `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md`
- **迁移状态单一真源方法：** `docs/solutions/best-practices/single-source-of-truth-migration-status.md`
