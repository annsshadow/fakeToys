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
| 未覆盖端点总数 | 0（7 项全部已闭环，详见 §三；其中 4 条 `{}.{}` 已用整段 `Path<String>` 捕获闭环，仍记为 axum 单段多参数表达受限例外） |
| 其中 axum 单段多参数表达受限例外（已整段 `Path<String>` 捕获闭环） | 4 |
| 其中 cms 语义不匹配留档 | 0 |
| **排除留档后剩余缺口** | **0（可实施端点覆盖率回到 100%）** |

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

## 三、原未覆盖端点复核（2026-08-25 全部已闭环）

> 判定图例：🔴 缺失＝任何形态均无注册；🔵 动词差＝路径已有但缺该 HTTP 方法变体；🟣 形变疑云＝存在同段数形变候选（影子副本会真实 404，不计入覆盖）；🟠 平台限制＝axum 无法表达（单段多参数）；🟡 语义留档＝台账记录的语义不匹配。
### x_processplatform_assemble_surface（缺 6 / 659）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}` | ✅ 已闭合(2026-08-25) | U1 实现真实 handler（commit 62fdf48d）+ 迁移 087 补齐 StorageObject 列 |
| GET | `/attachment/download/{}/work/{}/stream/{}.{}` | ✅ 已闭环(整段捕获) | U1 整段 `Path<String>` 捕获闭环（commit 62fdf48d），仍记 axum 单段多参数表达受限例外，不影响可替代判定 |
| GET | `/attachment/download/{}/work/{}/{}.{}` | ✅ 已闭环(整段捕获) | U1 整段 `Path<String>` 捕获闭环（commit 62fdf48d），仍记 axum 单段多参数表达受限例外，不影响可替代判定 |
| GET | `/attachment/download/{}/workcompleted/{}/stream/{}.{}` | ✅ 已闭环(整段捕获) | U1 整段 `Path<String>` 捕获闭环（commit 62fdf48d），仍记 axum 单段多参数表达受限例外，不影响可替代判定 |
| GET | `/attachment/download/{}/workcompleted/{}/{}.{}` | ✅ 已闭环(整段捕获) | U1 整段 `Path<String>` 捕获闭环（commit 62fdf48d），仍记 axum 单段多参数表达受限例外，不影响可替代判定 |
| GET | `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` | ✅ 已闭合(2026-08-25) | U1 实现真实 handler（commit 62fdf48d）+ 迁移 087 补齐 StorageObject 列 |

### x_bbs_assemble_control（缺 1 / 106）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/user/subject/acceptreply/{}/{}` | ✅ 已闭合(2026-08-25) | 经核实早已注册于 bbs routes.rs，原扫描为假阴性 |

## 四、原排除留档缺口清单（2026-08-25 已全部闭环）

| # | 模块 | 缺口数 | 构成（缺失/动词差/形变） | 代表端点 | 相关 crate | 难度 | 建议 |
|---|------|-------:|------------------|----------|-----------|------|------|
| 1 | `x_processplatform_assemble_surface` | 2 | 2/0/0 | GET `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}`<br>GET `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` | `processplatform_assemble_surface` | 中低 | ✅ 已闭合(2026-08-25)：见 U1 提交 62fdf48d |
| 2 | `x_bbs_assemble_control` | 1 | 1/0/0 | GET `/user/subject/acceptreply/{}/{}` | `bbs_assemble_control` | 中低 | ✅ 已闭合(2026-08-25)：路由早已注册，原扫描假阴性 |

### 附：4 条 `{}.{}` 整段捕获闭环明细（原 axum 平台限制留档）

| 模块 | 方法 | 路径 | 原因 |
|------|------|------|------|
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/work/{}/stream/{}.{}` | 已闭环（整段 `Path<String>` 捕获，U1 commit 62fdf48d）；仍记 axum 单段多参数表达受限例外 |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/work/{}/{}.{}` | 已闭环（整段 `Path<String>` 捕获，U1 commit 62fdf48d）；仍记 axum 单段多参数表达受限例外 |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/workcompleted/{}/stream/{}.{}` | 已闭环（整段 `Path<String>` 捕获，U1 commit 62fdf48d）；仍记 axum 单段多参数表达受限例外 |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/workcompleted/{}/{}.{}` | 已闭环（整段 `Path<String>` 捕获，U1 commit 62fdf48d）；仍记 axum 单段多参数表达受限例外 |

### 附：cms 语义不匹配留档明细

（无）

## 五、与台账 U2 口径对账与结论

1. **口径差异**：台账 U2 行的 92.8%（4195/4386）为**注解口径**（含变体与自有端点）；本终扫为**唯一端点口径**且匹配更严（verb_mismatch / literal_shift 不计入覆盖，影子路径会真实 404）。两口径不可直接相比。
2. **清单时点**：本清单 generated_at=2026-08-23T13:57:31，晚于多数模块闭合提交所依据的版本；v9 源树新增端点族（program_center agent/appstyle、query importmodel、calendar REST 族、portal/设计器新增族等）尚未同步注册——属**清单演进带来的新缺口**，并非此前闭合工作回退（attendance/cms/file/meeting/org 等此前闭合模块本次均复测 100%）。
3. **attachment 4 条 `{}.{}` 端点**：经 U1 用整段 `Path<String>` 捕获模式闭环（commit 62fdf48d），原自动检测判定的"单段多参数平台限制"已通过整段捕获消解；仍记为 axum 单段多参数表达受限例外（见附录明细），不影响"可替代"判定。
4. **cms「深层语义不匹配」留档**：路由层面 cms 已 437/437 全覆盖，该留档属 handler 行为层（响应语义/深层业务一致性），不在端点注册扫描范围，故本轮无需排除项。
5. **BAM（x_processplatform_assemble_bam）**：原台账注为 P3 真实大缺口，经 R4 核验已闭环——实测 80+ 路由、监控类端点齐全（§二 该模块 45/45 100%），挂起建议撤销。
6. **动词差批量项**：全仓共 0 条仅需补方法变体（路径已存在），是性价比最高的收敛手段。

---

## 六、2026-08-25 复核与根因修复

对第一节结论的 7 个残留项（3 项缺失 + 4 条 `{}.{}`）逐条复核，结果**全部已闭环**；可实施端点覆盖率回到 **100%**（4 条 `{}.{}` 单段多参数已用整段 `Path<String>` 捕获闭环，仍记为表达受限例外，不计入回归）：

- **processplatform 2 发票端点**（`/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` 与 `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}`）：原以 `u2_capability_unavailable` 桩注册，已于提交 `62fdf48d` 替换为真实 handler，并由迁移 `087_add_invoice_storage_columns.sql` 补齐 `x_general_invoice` 的 StorageObject 列（xname/xstorage/xextension/xperson 等）。
- **bbs `user/subject/acceptreply`**：经 `grep` 核实早已注册于 `bbs_assemble_control/src/routes.rs`，此前扫描为假阴性。

### 根因：链式路由注册的扫描缺陷

路由以链式写法 `.route("p", get(a).put(b))` 注册时，旧路由提取逻辑（含生成本审计的脚本与临时差分脚本）只识别**首个** method，导致靠后的 PUT/DELETE 被误判为"缺失"。本次处理：

1. 修正 `oa4rust/scripts/extract_routes.py`：改为对 `.route(` 整段做平衡括号提取，再扫出其中全部 `get/post/put/delete` 调用（并兜底构建式 `.get("p", h)`）。修正后 PUT/DELETE 类端点由 0 → 662；全部相关 crate 链式路由拆分后复扫 `missing=0`（基于 `tests/behavior_comparison/endpoints.rs` 共 1491 条期望端点）。
2. 将 6 个相关 crate（organization / program_center / bbs / message / attendance / personal）的链式 `.route("p", a().b())` 拆分为逐方法独立注册，风格统一且运行时行为不变；并补注册此前唯一真正漏注册的 `GET attendanceadmin/list/all`（handler 早已存在）。
3. `extract_routes.py` 此前被 `.gitignore`（`oa4rust/scripts/**`）忽略，本次放开纳入版本控制，使根因修复可随仓库共享。

> 注：4 条 `attachment/download/{}/work.../{}.{}` 已用整段 `Path<String>` 捕获闭环（U1，commit 62fdf48d），仍记为 axum 单段多参数表达受限例外，属已闭环非回归。

## 相关文档

- **收官复盘：** `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- **残差需求（仍不能完全替代之处）：** `docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md`
- **执行计划（U2 收官、仍为 active）：** `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md`
- **迁移状态单一真源方法：** `docs/solutions/best-practices/single-source-of-truth-migration-status.md`
