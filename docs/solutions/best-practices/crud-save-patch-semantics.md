---
title: "crud_save 只写载荷里出现的列（PATCH 语义）+ 归属列的 save 窄白名单"
date: 2026-10-19
last_refreshed: 2026-10-19
category: best-practices
module: oa4rust/backend/crates/{shared,program_center,portal_assemble_designer,query_assemble_designer,personal_extend}
problem_type: data_loss_on_partial_write
component: crud_helper
severity: high
symptoms:
  - "局部 save（请求只带要改的字段）把未提交的列写成空串；归属列被抹后属主自己的 save/delete 全部 403"
  - "x_person_face（face_save）与 x_program_collect（collect_save）实测：改个名字就把 person_id 抹成空串"
  - "关联列同理：app_id / portal_id / xapplication 被抹后，行从所属应用/门户里掉出去"
root_cause: full_column_update_semantics
resolution_type: code_fix
tags: [crud, patch-semantics, partial-update, ownership, data-loss, update-spec]
applies_when:
  - "新增或审阅 *_save handler：判断该把哪些列列进 spec"
  - "排查『保存后归属/关联列为空』类缺陷"
  - "要在通用写助手里收口、还是在 per-handler 窄 spec 里收口之间做取舍"
---

# crud_save 只写载荷里出现的列（PATCH 语义）

## 症状与根因

`shared::crud::crud_save` 原来是**全列写**：遍历 `spec.columns`，每个 db 列都生成 `SET col=$N`，
取值 `payload.get(json_key).as_str().unwrap_or("")` —— **缺键写空串**。

于是「局部保存」（客户端只提交改动的字段）会把未提交的列清空。落在归属/关联列上就是数据损坏：

- `x_person_face`：属主改 `faceName` ⇒ `person_id` 变 `""` ⇒ 之后本人 `save`/`delete` 全 403（第一轮实测复现）；
- `x_program_collect`：收藏改标题 ⇒ `person_id` 变 `""` ⇒ 收藏从属主列表里消失、本人也改不动；
- 关联列同理：`app_id` / `portal_id` / `xapplication` 被抹后，行从所属应用/门户里掉出去。

判据：**save 是"改我给的那几格"，不是"整行重写"**。整行重建的入口是 create。

## 修法（两层）

### 1）助手层：缺键 = 不写（根因收口）

`shared/src/crud.rs` 的 `crud_save` 只对**载荷里出现**的键生成 `SET col=$N`：

- 缺键 → 不落笔（保持原值）；
- 显式 `""` / `null` → 写 `""`（清空要走显式值）；
- 载荷里一个可写列都没有 → 不发空 `SET`（那是语法错），改查 `SELECT 1 … WHERE id=$1`，
  只回答"行是否存在"（保住 not-found 语义与 `*_owned` 门禁链）。

一处改动覆盖全部 21 个 `crud_save` 调用点。测试：`shared` 三条实库用例（DB 不可达则跳过，CI 的 unit-tests
job 带 PostgreSQL 会真跑）：

- `test_crud_save_partial_payload_preserves_unlisted_columns_live`
- `test_crud_save_empty_payload_is_noop_but_reports_existence_live`
- `test_crud_save_explicit_empty_string_clears_live`

### 2）归属层：save 窄白名单（防显式改写）

助手层只解决"没提交的列被抹"，不解决"客户端**显式**把 `creator`/`personId` 塞进 save 改写归属"。
凡 spec 含归属列者，另开 `*_update_spec()`（只列业务列），save 走它：

| handler | 归属列 | 处理 |
|---|---|---|
| `program_center::collect_save` | `person_id` | 新增 `collect_update_spec`（title/url）；顺带删掉原先"剥离 personId"的补丁（窄 spec 已覆盖该意图） |
| `portal_assemble_designer::dict_save` | `creator` | 新增 `dict_update_spec` |
| `portal_assemble_designer::widget_save` | `creator` | 新增 `widget_update_spec` |
| `query_assemble_designer::importer_save` | `creator` | 新增 `importer_update_spec` |
| `query_assemble_designer::stat_save` | `creator` | 新增 `stat_update_spec` |
| `personal_extend::face_save` | `person_id` | 上一轮已加 `face_update_spec`（同口径，本轮仅更新注释） |
| `bbs_assemble_control::section_save` | `creator` | 仓库最早先例 `section_update_spec` |

**判定标准**：归属/审计列（`person_id`、`creator`）= create-only；
**关联/可编辑列**（`app_id`、`portal_id`、`xapplication`、`category_id`、`unit_id`）保持可写——
它们可能是合法编辑（三员管理改 `unit_id`、视图换分类），且助手层已保证"不提交就不动"。

## 审计表：21 个 save 调用点

| crate | 表 | save spec 列 | 判定 |
|---|---|---|---|
| ai_assemble_control | x_ai_ann | title, content, category, status | 无归属列 ✓ |
| bbs_assemble_control | x_bbs_…_section | name | 窄 spec 先例 ✓ |
| cms_assemble_control ×5 | x_cms_surface_appdict / x_cms_form / x_cms_form_v2（与 form 同表）/ x_cms_view | app_id, name, definition, status, view_config, category_id… | 关联列保留可写；助手层防误抹 ✓ |
| console | x_program_deploy_server | app_id, app_name, application, category, disable | 同上 ✓ |
| general_assemble_control | x_general_assemble_control_config | system_name, maintenance_mode, allow_registration, version | 配置行，无归属 ✓ |
| organization_assemble_control | x_org_person | name, mobile, email, unit_id, icon, status, status_des | `unit_id` 是三员管理的合法编辑项 ⇒ 保留（靠助手层防误抹） |
| personal_extend | x_person_face | face_name, face_type, feature, status | 窄 spec ✓ |
| portal_assemble_designer ×2 | x_portal_dict / x_portal_widget | name, app_name, app_data / portal_id, category, config | **本轮窄化 creator** |
| processplatform_assemble_designer ×3 | pp_e_applicationdict / pp_e_form / x_pp_xform | xname, xapplication / name, definition, status | 关联列保留；助手层防误抹 ✓ |
| program_center ×2 | x_program_collect / x_program_invoke | title, url / name, alias, category | **本轮窄化 person_id** |
| query_assemble_designer ×2 | x_query_import_model / x_query_stat | name, model_flag, query_flag, content / name, query_flag, stat_type | **本轮窄化 creator** |
| search | x_ftsearch_document | title, body, source | 无归属 ✓ |

## 遗留（明确不动/待办）

1. **create 侧的归属仍取载荷**：`dict_create`/`widget_create`/`importer_create`/`stat_create` 的 `creator`
   由客户端给定（能造假）。对齐 Java o2server 口径并改成"会话为事实源"属于"归属只由会话定"的更大一轮，本轮不动。
2. **通用 save/delete 无 owner 谓词**：除 personal_extend（`crud_save_owned`）与 program_center collect（手写门禁）外，
   其余 save 依赖 crate 级 admin 门。是否全面补 `*_owned` 属于安全审计既有待办（见 IDOR 文档）。
3. 将来若某 handler 确需"整行重写"，应显式走 create 或自写 SQL，**不要**把助手改回全列写。

## 相关

- [IDOR Vulnerability in Write Handlers](../security-issues/idor-vulnerability-write-handlers.md)
- [main 既有红全量修复（2026-10-19）](../development-workflow/main-existing-reds-fix-2026-10-19.md)（§2.3 首次定性，§6 本轮收口）
- **实现**：`oa4rust/backend/crates/shared/src/crud.rs`
