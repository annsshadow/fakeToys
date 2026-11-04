# O23 盘点：`AppError::Internal` 丢 source / 保留例外

生成时间：2026-11-05 02:28（本地工作树）

## 概览

- 原丢 source 写法 `.map_err(|_| AppError::Internal)`：**3895 处 / 137 个 .rs 文件**。
- 其中 **7 处**属外部连接器缺环境变量（andfx 2、zhengwudingding 3、mpweixin 2），已改为 `AppError::missing_config(<项名>)`（503）。
- 其余 **3888 处**统一替换为 `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`。
- 另手工补充上下文 **18 处**（连接器上游 errcode/缺字段、express 配置读取），当前 `AppError::internal_with_source` 调用共 **3906 处**。
- 剩余直接 `AppError::Internal`：**125 处 / 36 个 .rs 文件**（逐处见文末），全部落入下列例外类别。

## 例外与理由

1. **测试断言/模式匹配**：`tests*.rs`、`#[cfg(test)]` 内 `Err(AppError::Internal)` 是断言目标，不产生错误，保留。
2. **无底层错误对象的不变量缺失**：`.ok_or(AppError::Internal)` / `return Err(AppError::Internal)` 表示
   「上游响应缺字段」或「内部状态不可能为空」，没有可携带的 source；
   连接器相关的已补 `internal_with_source(ctx, "<说明>")`，其余保留并在清单标注。
3. **非运行时代码**：`backend/add_stubs.py`、`backend/apply_routes.js`、`backend/add_routes.js`
   是生成器脚本，文本会被重新生成覆盖，不参与运行时错误路径，未改。

## 按文件命中数（`internal_with_source` 调用行）

| 文件 | 命中数 |
| --- | --- |
| backend/crates/processplatform_assemble_surface/src/lib.rs | 702 |
| backend/crates/program_center/src/lib.rs | 286 |
| backend/crates/cms_assemble_control/src/lib.rs | 233 |
| backend/crates/processplatform_service_processing/src/lib.rs | 214 |
| backend/crates/attendance_assemble_control/src/lib.rs | 197 |
| backend/crates/processplatform_assemble_designer/src/lib.rs | 135 |
| backend/crates/organization_assemble_control/src/lib.rs | 108 |
| backend/crates/processplatform_service_processing/src/u2.rs | 107 |
| backend/crates/meeting_assemble_control/src/lib.rs | 93 |
| backend/crates/message_assemble_communicate/src/lib.rs | 87 |
| backend/crates/query_assemble_designer/src/lib.rs | 86 |
| backend/crates/bbs_assemble_control/src/u2.rs | 76 |
| backend/crates/portal_assemble_designer/src/lib.rs | 74 |
| backend/crates/file_assemble_control/src/lib.rs | 70 |
| backend/crates/query_assemble_designer/src/u2_closures.rs | 69 |
| backend/crates/general_assemble_control/src/lib.rs | 65 |
| backend/crates/portal_assemble_surface/src/lib.rs | 65 |
| backend/crates/query_assemble_surface/src/lib.rs | 65 |
| backend/crates/bbs_assemble_control/src/lib.rs | 64 |
| backend/crates/ai_assemble_control/src/lib.rs | 57 |
| backend/crates/processplatform_assemble_bam/src/lib.rs | 55 |
| backend/crates/organization_assemble_control/src/u2_org.rs | 37 |
| backend/crates/calendar_assemble_control/src/u2.rs | 34 |
| backend/crates/query_assemble_surface/src/u2_closures.rs | 34 |
| backend/crates/organization_assemble_control/src/u2_person.rs | 30 |
| backend/crates/empower/src/lib.rs | 26 |
| backend/crates/program_init/src/lib.rs | 26 |
| backend/crates/general_core_entity/src/lib.rs | 25 |
| backend/crates/mind_assemble_control/src/u2.rs | 24 |
| backend/crates/personal/src/u2.rs | 24 |
| backend/crates/organization_assemble_authentication/src/lib.rs | 23 |
| backend/crates/hotpic_assemble_control/src/lib.rs | 22 |
| backend/crates/bbs_core_entity/src/lib.rs | 20 |
| backend/crates/auth/src/qiyeweixin.rs | 19 |
| backend/crates/jpush_assemble_control/src/lib.rs | 19 |
| backend/crates/organization_core_entity/src/lib.rs | 18 |
| backend/crates/organization_assemble_express/src/endpoints.rs | 16 |
| backend/crates/organization_assemble_express/src/endpoints_org.rs | 15 |
| backend/crates/organization_assemble_express/src/endpoints_person2.rs | 15 |
| backend/crates/attendance_core_entity/src/lib.rs | 14 |
| backend/crates/calendar_core_entity/src/lib.rs | 14 |
| backend/crates/organization_assemble_express/src/endpoints_unit2.rs | 14 |
| backend/crates/auth/src/mpweixin.rs | 13 |
| backend/crates/organization_assemble_control/src/u2_misc.rs | 13 |
| backend/crates/query_service_processing/src/u2.rs | 13 |
| backend/crates/meeting_core_entity/src/lib.rs | 12 |
| backend/crates/file/src/lib.rs | 11 |
| backend/crates/mind_assemble_control/src/lib.rs | 11 |
| backend/crates/organization_assemble_express/src/endpoints_attr.rs | 11 |
| backend/crates/portal/src/lib.rs | 11 |
| backend/crates/attendance/src/lib.rs | 10 |
| backend/crates/auth/src/oauth.rs | 10 |
| backend/crates/cms_core_entity/src/lib.rs | 10 |
| backend/crates/correlation_service_processing/src/lib.rs | 10 |
| backend/crates/express/src/batch_query.rs | 10 |
| backend/crates/mind/src/lib.rs | 10 |
| backend/crates/calendar/src/lib.rs | 9 |
| backend/crates/component_assemble_control/src/lib.rs | 9 |
| backend/crates/console/src/lib.rs | 9 |
| backend/crates/meeting/src/lib.rs | 9 |
| backend/crates/organization_assemble_express/src/endpoints_group2.rs | 9 |
| backend/crates/auth/src/welink.rs | 8 |
| backend/crates/correlation_service_processing/src/u2.rs | 8 |
| backend/crates/organization_assemble_control/src/u2_helpers.rs | 8 |
| backend/crates/ai/src/chat.rs | 7 |
| backend/crates/auth/src/zhengwudingding.rs | 7 |
| backend/crates/control/src/unit.rs | 7 |
| backend/crates/message/src/lib.rs | 7 |
| backend/crates/mind_core_entity/src/lib.rs | 7 |
| backend/crates/portal_core_entity/src/lib.rs | 7 |
| backend/crates/processplatform_core_express/src/lib.rs | 7 |
| backend/crates/shared/src/error.rs | 7 |
| backend/crates/ai/src/config.rs | 6 |
| backend/crates/auth/src/oidc.rs | 6 |
| backend/crates/control/src/group.rs | 6 |
| backend/crates/control/src/person.rs | 6 |
| backend/crates/control/src/role.rs | 6 |
| backend/crates/file_core_entity/src/lib.rs | 6 |
| backend/crates/jpush_core_entity/src/lib.rs | 6 |
| backend/crates/organization_assemble_express/src/endpoints_duty2.rs | 6 |
| backend/crates/organization_assemble_express/src/endpoints_org2.rs | 6 |
| backend/crates/personal/src/regist.rs | 6 |
| backend/crates/portal_assemble_designer/src/u2_script.rs | 6 |
| backend/crates/processplatform_assemble_designer/src/u2_script.rs | 6 |
| backend/crates/processplatform_assemble_surface/src/u2_render.rs | 6 |
| backend/crates/processplatform_core_entity/src/lib.rs | 6 |
| backend/crates/query_service_processing/src/lib.rs | 6 |
| backend/crates/shared/src/crud.rs | 6 |
| backend/crates/auth/src/lib.rs | 5 |
| backend/crates/bbs/src/subject.rs | 5 |
| backend/crates/hotpic_core_entity/src/lib.rs | 5 |
| backend/crates/jpush/src/lib.rs | 5 |
| backend/crates/organization_core_express/src/lib.rs | 5 |
| backend/crates/personal_extend/src/avatar.rs | 5 |
| backend/crates/personal_extend/src/personal.rs | 5 |
| backend/crates/query_core_entity/src/lib.rs | 5 |
| backend/crates/query_core_express/src/lib.rs | 5 |
| backend/crates/query_service/src/lib.rs | 5 |
| backend/crates/ai/src/app.rs | 4 |
| backend/crates/ai/src/file.rs | 4 |
| backend/crates/correlation_core_entity/src/lib.rs | 4 |
| backend/crates/personal/src/reset.rs | 4 |
| backend/crates/personal/src/signature.rs | 4 |
| backend/crates/process_designer/src/lib.rs | 4 |
| backend/crates/search/src/lib.rs | 4 |
| backend/crates/ai/src/index.rs | 3 |
| backend/crates/ai_core_entity/src/lib.rs | 3 |
| backend/crates/calendar_assemble_control/src/lib.rs | 3 |
| backend/crates/component/src/lib.rs | 3 |
| backend/crates/component_core_entity/src/lib.rs | 3 |
| backend/crates/correlation/src/routes.rs | 3 |
| backend/crates/express/src/lib.rs | 3 |
| backend/crates/general/src/lib.rs | 3 |
| backend/crates/hotpic/src/lib.rs | 3 |
| backend/crates/message_core_entity/src/lib.rs | 3 |
| backend/crates/organization_assemble_express/src/lib.rs | 3 |
| backend/crates/organization_assemble_personal/src/lib.rs | 3 |
| backend/crates/process_bam/src/lib.rs | 3 |
| backend/crates/program_center_core_entity/src/handlers/agent.rs | 3 |
| backend/crates/program_center_core_entity/src/handlers/application.rs | 3 |
| backend/crates/program_center_core_entity/src/handlers/invoke.rs | 3 |
| backend/crates/program_center_core_entity/src/handlers/script.rs | 3 |
| backend/crates/program_center_core_entity/src/handlers/structure.rs | 3 |
| backend/crates/bbs/src/forum.rs | 2 |
| backend/crates/bbs/src/section.rs | 2 |
| backend/crates/cms_control/src/lib.rs | 2 |
| backend/crates/cms_express/src/lib.rs | 2 |
| backend/crates/cms_express/src/routes.rs | 2 |
| backend/crates/correlation_core_express/src/lib.rs | 2 |
| backend/crates/organization_assemble_authentication/src/u2.rs | 2 |
| backend/crates/personal/src/icon.rs | 2 |
| backend/crates/query_express/src/lib.rs | 2 |
| backend/crates/auth/src/andfx.rs | 1 |
| backend/crates/auth/src/sso.rs | 1 |
| backend/crates/base/src/lib.rs | 1 |
| backend/crates/cms_core_express/src/lib.rs | 1 |
| backend/crates/orm/src/lib.rs | 1 |
| backend/crates/personal/src/lib.rs | 1 |
| backend/crates/personal/src/password.rs | 1 |

## 逐处清单（替换后上下文）


### backend/crates/ai/src/app.rs

- L17: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L45: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L86: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L119: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/ai/src/chat.rs

- L24: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L33: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L78: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L87: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L137: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L155: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/ai/src/config.rs

- L22: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L122: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L171: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L239: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L306: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/ai/src/file.rs

- L25: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L75: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L133: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/ai/src/index.rs

- L22: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L46: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L84: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/ai_assemble_control/src/lib.rs

- L68: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L198: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L412: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L430: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L455: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L524: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L623: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L700: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L726: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L752: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L806: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L859: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L917: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L989: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1052: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1173: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1278: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1325: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1350: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1417: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1571: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1622: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1683: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1729: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1763: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1806: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1849: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1859: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1887: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1946: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1966: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1985: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2032: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2059: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2082: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2219: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2270: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2394: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2435: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2491: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2544: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2626: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/ai_core_entity/src/lib.rs

- L49: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L90: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/attendance/src/lib.rs

- L72: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L90: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L153: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L240: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L377: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L416: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L494: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L540: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L577: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/attendance_assemble_control/src/lib.rs

- L59: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L175: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L223: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L253: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L350: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L440: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L513: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L522: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L535: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L571: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L613: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L655: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L784: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L837: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L875: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L902: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L969: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L996: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1030: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1056: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1083: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1130: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1257: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1310: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1509: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1621: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1758: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1811: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1843: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1866: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1909: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1958: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2000: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2048: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2089: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2135: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2181: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2231: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2271: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2368: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2468: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2524: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2601: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2642: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2685: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2778: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2824: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2875: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2920: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2966: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3006: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3073: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3096: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3146: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3288: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3335: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3382: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3482: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3533: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3584: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3632: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3678: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3724: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3817: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3863: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3913: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3959: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4005: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4057: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4165: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4316: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4344: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4407: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4447: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4486: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4529: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4577: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4620: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4658: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4698: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4774: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4810: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5021: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5241: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5278: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5363: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5514: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5584: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5631: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5666: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5715: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5759: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5772: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5898: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5925: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5959: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5970: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5982: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6009: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6043: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6054: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6066: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6095: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L6167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6241: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L6254: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L6441: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6486: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6509: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6596: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6666: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6690: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6727: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6749: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6784: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6815: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7006: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7016: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L7196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7293: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L7306: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7357: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7384: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7415: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7468: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7504: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7548: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7594: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7604: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7628: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7646: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7706: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7805: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7848: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7885: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7951: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7968: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7992: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8039: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8204: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8340: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8362: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8475: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L8484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L8559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8604: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8611: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8743: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8797: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8920: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L8944: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8962: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8995: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9069: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/attendance_core_entity/src/lib.rs

- L39: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L85: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L156: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L216: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L275: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L280: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L356: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/andfx.rs

- L80: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/lib.rs

- L494: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L731: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L867: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L904: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L937: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/mpweixin.rs

- L139: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L142: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L146: `return Err(AppError::internal_with_source(`
- L155: `AppError::internal_with_source(`
- L196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L203: `return Err(AppError::internal_with_source(`
- L214: `AppError::internal_with_source(`
- L245: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L323: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L354: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L424: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L427: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/oauth.rs

- L297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L319: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L322: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L349: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L352: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L374: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L377: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L447: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L465: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/oidc.rs

- L117: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L122: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L205: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L209: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L259: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/qiyeweixin.rs

- L45: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L48: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L55: `return Err(AppError::internal_with_source(`
- L64: `AppError::internal_with_source(`
- L75: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L78: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L85: `return Err(AppError::internal_with_source(`
- L95: `AppError::internal_with_source(`
- L109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L112: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L118: `AppError::internal_with_source(`
- L129: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L132: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L139: `return Err(AppError::internal_with_source(`
- L176: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L267: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/sso.rs

- L132: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/welink.rs

- L76: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L79: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L85: `AppError::internal_with_source(`
- L102: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L105: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L111: `AppError::internal_with_source(`
- L131: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/auth/src/zhengwudingding.rs

- L55: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L58: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L65: `AppError::internal_with_source(`
- L78: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L81: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L88: `AppError::internal_with_source(`
- L106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/base/src/lib.rs

- L80: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/bbs/src/forum.rs

- L30: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L84: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/bbs/src/section.rs

- L36: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L97: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/bbs/src/subject.rs

- L45: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L119: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L337: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/bbs_assemble_control/src/lib.rs

- L182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L220: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L379: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L399: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L428: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L469: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L493: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L540: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L567: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L595: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L614: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L642: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L662: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L715: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L743: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L766: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L794: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L816: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L839: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L887: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L909: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L964: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L987: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1010: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1036: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1046: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1054: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1087: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1094: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1127: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1166: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1173: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1206: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1213: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1246: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1253: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1289: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1369: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1469: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1473: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1546: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1551: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1568: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1610: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1646: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1654: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1662: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1670: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1738: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1775: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1782: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1810: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1865: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1917: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1940: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1973: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2010: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2033: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/bbs_assemble_control/src/u2.rs

- L80: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L372: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L440: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L482: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L493: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L565: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L734: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L767: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L773: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L808: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L824: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L831: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L863: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L895: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L934: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L940: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L972: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L979: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1038: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1045: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1115: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1239: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1337: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1343: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1374: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1437: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1565: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1592: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1611: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1658: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1676: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1749: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1766: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1806: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1828: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1982: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2025: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2135: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2290: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2356: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2366: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2383: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2424: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2515: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2536: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2565: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2584: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2605: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2634: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2679: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2706: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2756: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2777: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2814: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2821: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2835: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2843: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2934: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2938: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/bbs_core_entity/src/lib.rs

- L48: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L86: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L252: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L307: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L332: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L344: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L389: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L412: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L457: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L487: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L499: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L547: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L619: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L652: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L664: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L754: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/calendar/src/lib.rs

- L154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L298: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L363: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L416: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L471: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L542: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L596: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L629: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/calendar_assemble_control/src/lib.rs

- L77: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L150: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/calendar_assemble_control/src/u2.rs

- L98: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L156: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L236: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L261: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L305: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L323: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L414: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L439: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L514: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L543: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L654: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L698: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L719: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L727: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L772: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L814: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L859: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L916: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L942: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L985: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1013: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1035: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1057: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1074: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1121: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1231: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1256: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/calendar_core_entity/src/lib.rs

- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L293: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L398: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L419: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L623: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L645: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/cms_assemble_control/src/lib.rs

- L45: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L88: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L137: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L338: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L425: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L470: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L488: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L507: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L604: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L658: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L716: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L827: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1361: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1409: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1451: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1507: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1571: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1672: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1679: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1759: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1903: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1957: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2052: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2263: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2271: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2362: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2369: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2441: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2531: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2551: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2627: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2669: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2734: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2755: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3041: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3138: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3205: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3293: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3365: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3456: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3463: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3516: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3523: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3626: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3633: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3716: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3780: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3851: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3908: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3947: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4007: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4039: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4056: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4128: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4285: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4357: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4415: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4493: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4533: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4587: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4627: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4705: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4782: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4819: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4846: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4863: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4930: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5010: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5082: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5149: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L5207: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L5215: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L5223: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L5251: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L5268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5270: `"id": row.try_get::<_, String>("id").map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L5271: `"appId": row.try_get::<_, String>("app_id").map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L5272: `"name": row.try_get::<_, Option<String>>("name").map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?.unwrap_or_default(),`
- L5297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5346: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5406: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5433: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5531: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5557: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5607: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5796: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5858: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6013: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6122: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6299: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6306: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6350: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6409: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6644: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6712: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6733: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6763: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L6898: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6981: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7100: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7240: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7403: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7465: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7639: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7652: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e));`
- L7704: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7720: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7746: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7888: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7899: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L8039: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8049: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8142: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8204: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8332: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8376: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8402: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8486: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8499: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8522: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8643: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8767: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8818: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8827: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8870: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8909: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8945: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9016: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9060: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9098: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9139: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L9277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9326: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9342: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9387: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9523: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9553: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9608: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9710: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9776: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9840: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9891: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9901: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9934: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9957: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9992: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10019: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10066: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10122: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10130: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L10150: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10186: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10257: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10359: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10379: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10390: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10474: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10531: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10557: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10615: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10627: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10655: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10664: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10713: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10743: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10825: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10860: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10888: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10914: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11030: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11181: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11206: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11240: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11259: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L11338: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L11351: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11464: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11508: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11593: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11612: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11696: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11739: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11785: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/cms_control/src/lib.rs

- L29: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L66: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/cms_core_entity/src/lib.rs

- L49: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L90: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L361: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L427: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L479: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L526: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/cms_core_express/src/lib.rs

- L30: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/cms_express/src/lib.rs

- L46: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L91: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/cms_express/src/routes.rs

- L37: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L72: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/component/src/lib.rs

- L29: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L177: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/component_assemble_control/src/lib.rs

- L109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L206: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L258: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L313: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L345: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L378: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L409: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/component_core_entity/src/lib.rs

- L33: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/console/src/lib.rs

- L87: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L133: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L189: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L217: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L241: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L473: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L492: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/control/src/group.rs

- L106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L116: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L248: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/control/src/person.rs

- L155: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L177: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L314: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L363: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L430: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/control/src/role.rs

- L106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L116: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L248: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/control/src/unit.rs

- L101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L179: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L270: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L356: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/correlation/src/routes.rs

- L45: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L116: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L187: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/correlation_core_entity/src/lib.rs

- L53: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L102: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/correlation_core_express/src/lib.rs

- L18: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L42: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/correlation_service_processing/src/lib.rs

- L59: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L87: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L185: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L248: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L278: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L305: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L340: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L461: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L489: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/correlation_service_processing/src/u2.rs

- L144: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L173: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L194: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L362: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L391: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L446: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L597: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/empower/src/lib.rs

- L103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L118: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L149: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L190: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L205: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L235: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L271: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L283: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L314: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L326: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L366: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L381: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L419: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L468: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L513: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L548: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L594: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L605: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L640: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L651: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L697: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/express/src/batch_query.rs

- L153: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L169: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L181: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L192: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L240: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L287: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L328: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L370: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L413: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L471: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/express/src/lib.rs

- L56: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L91: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/file/src/lib.rs

- L83: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L207: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L390: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L465: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L521: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L676: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L729: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L798: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/file_assemble_control/src/lib.rs

- L212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L293: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L359: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L441: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L495: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L520: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L533: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L585: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L614: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L645: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L661: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L669: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L677: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L685: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L693: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L701: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L744: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L758: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L787: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L838: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L911: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L984: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1056: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1316: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1349: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1423: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2131: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2508: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2740: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2827: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3522: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3648: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3681: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3850: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3935: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3946: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3973: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3996: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4098: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4136: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4171: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4581: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4585: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L4845: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4907: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4939: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4984: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5015: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5048: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5173: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5386: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5404: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5413: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5515: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5609: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/file_core_entity/src/lib.rs

- L86: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L141: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L191: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L359: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/general/src/lib.rs

- L19: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L63: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L85: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/general_assemble_control/src/lib.rs

- L69: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L150: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L322: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L395: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L504: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L535: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L563: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L587: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L660: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L733: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L806: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L920: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L948: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1049: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1076: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1100: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1256: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1318: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1385: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1483: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1592: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1619: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1726: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1811: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1863: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1906: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1943: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1980: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2020: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2043: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2092: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2122: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2184: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2256: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2296: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2307: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2324: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2372: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2474: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2562: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2592: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2638: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2659: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2687: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2720: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2786: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2846: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2898: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2950: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3001: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3052: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3116: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3143: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/general_core_entity/src/lib.rs

- L73: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L181: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L252: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L259: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L308: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L328: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L371: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L404: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L433: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L470: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L538: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L610: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L660: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L692: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L764: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L791: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L853: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L902: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L909: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/hotpic/src/lib.rs

- L33: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L63: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/hotpic_assemble_control/src/lib.rs

- L59: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L88: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L142: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L262: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L313: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L344: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L376: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L404: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L487: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L534: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L595: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L609: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L646: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L665: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L712: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L761: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L781: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L797: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L843: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/hotpic_core_entity/src/lib.rs

- L52: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L100: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L146: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/jpush/src/lib.rs

- L39: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L74: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L121: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L143: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L178: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/jpush_assemble_control/src/lib.rs

- L50: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L82: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L145: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L180: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L238: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L305: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L372: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L409: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L456: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L508: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L527: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L546: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L573: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L636: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L714: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1006: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/jpush_core_entity/src/lib.rs

- L55: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L88: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L138: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L193: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L237: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/meeting/src/lib.rs

- L99: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L218: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L296: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L338: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L393: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L474: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L512: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L572: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/meeting_assemble_control/src/lib.rs

- L101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L185: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L249: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L369: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L428: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L491: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L518: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L550: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L612: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L664: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L755: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L788: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L837: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L886: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L938: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1046: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1112: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1280: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1329: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1378: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1427: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1476: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1525: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1577: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1629: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1681: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1733: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1788: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1866: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1874: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1943: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1995: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2048: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2114: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2306: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2337: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2369: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2426: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2465: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2490: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2525: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2592: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2633: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2641: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2697: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2813: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2873: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2932: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2991: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3044: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3179: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3338: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3381: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3482: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3563: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3679: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3683: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3760: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3783: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3813: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3837: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3847: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3893: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3902: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3927: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3980: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3983: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4026: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4069: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4238: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L4271: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4292: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4322: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/meeting_core_entity/src/lib.rs

- L67: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L86: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L219: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L373: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L422: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L516: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L544: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L621: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/message/src/lib.rs

- L62: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L121: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L191: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/message_assemble_communicate/src/lib.rs

- L33: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L84: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L91: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L126: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L184: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L197: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L392: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L467: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L515: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L542: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L567: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L611: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L664: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L725: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L742: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L790: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L861: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L868: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L874: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L929: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L941: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L994: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1023: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1040: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1087: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1111: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1128: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1178: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1205: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1259: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1286: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1368: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1412: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1441: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1517: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1600: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1664: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1711: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1781: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1786: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1855: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1868: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1938: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1961: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2011: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2079: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L2103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2205: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2256: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2307: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2409: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2573: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2593: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2644: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2698: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2761: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2788: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2866: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2880: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2922: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2946: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2993: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3024: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3048: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3079: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3172: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3229: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3237: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3279: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3290: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3301: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3324: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/message_core_entity/src/lib.rs

- L30: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L71: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L111: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/mind/src/lib.rs

- L46: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L137: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L207: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L492: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L553: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L579: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/mind_assemble_control/src/lib.rs

- L120: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L272: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L301: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L325: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L363: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L371: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L380: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L462: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/mind_assemble_control/src/u2.rs

- L60: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L92: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L192: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L246: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L318: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L423: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L433: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L441: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L515: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L546: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L586: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L605: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L628: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L656: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L713: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/mind_core_entity/src/lib.rs

- L53: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L92: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L245: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L293: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L381: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L440: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_authentication/src/lib.rs

- L195: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L198: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L217: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L220: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L412: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L415: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L431: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L510: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L553: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L621: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L636: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L639: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L697: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L743: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L746: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L760: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L763: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L782: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_authentication/src/u2.rs

- L145: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L278: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_control/src/lib.rs

- L42: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L50: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L99: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L143: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L270: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L342: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L419: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L480: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L520: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L562: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L606: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L694: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L739: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L783: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L881: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L945: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1002: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1060: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1132: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1156: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1180: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1203: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1286: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1313: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1407: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1455: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1603: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1612: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1692: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1738: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1785: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1828: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1873: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1879: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1910: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1952: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2001: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2007: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2045: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2087: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2131: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2365: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2451: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2495: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2538: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2589: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2598: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2632: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2674: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2723: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2729: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2767: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2809: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2845: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2865: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2885: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2927: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3090: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3191: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3240: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3293: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3437: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3539: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3586: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3631: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3787: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3830: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3881: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3889: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3940: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3948: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4155: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4203: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4257: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4275: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L4286: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4319: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4338: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4501: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4550: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4599: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4807: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4862: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4917: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4971: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_control/src/u2_helpers.rs

- L150: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L185: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L237: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L264: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L331: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L353: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_control/src/u2_misc.rs

- L53: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L82: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L193: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L371: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L479: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L499: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L601: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L637: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L649: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_control/src/u2_org.rs

- L60: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L104: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L165: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L174: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L216: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L240: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L248: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L281: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L324: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L349: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L364: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L385: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L444: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L456: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L528: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L562: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L665: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L701: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L740: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L750: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L756: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L759: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L797: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))? as i64;`
- L844: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L917: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L951: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1000: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1003: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1074: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1178: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1192: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1224: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1263: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_control/src/u2_person.rs

- L63: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L88: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L145: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L234: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L241: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L270: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L279: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L355: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L372: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L382: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L406: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L422: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L433: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L463: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L494: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L602: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L623: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L644: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L665: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L690: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L723: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L766: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L780: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/endpoints.rs

- L144: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L237: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L246: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L257: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L301: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L332: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L393: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L412: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L442: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L463: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L540: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L552: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L586: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/endpoints_attr.rs

- L57: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L93: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L272: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L323: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L429: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L491: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L527: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/endpoints_duty2.rs

- L37: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L61: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L192: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L219: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/endpoints_group2.rs

- L67: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L128: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L178: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L217: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L287: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L335: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L342: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L349: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`

### backend/crates/organization_assemble_express/src/endpoints_org.rs

- L53: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L69: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L88: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L281: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L292: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L325: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L444: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L475: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L528: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L556: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/endpoints_org2.rs

- L73: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L123: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L202: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L248: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L272: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/endpoints_person2.rs

- L134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L139: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L351: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L366: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L487: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L540: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L734: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L739: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/organization_assemble_express/src/endpoints_unit2.rs

- L92: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L99: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L178: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L228: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L271: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L321: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L380: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L492: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L543: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L565: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L610: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L633: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_express/src/lib.rs

- L52: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L81: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L129: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_assemble_personal/src/lib.rs

- L49: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L86: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/organization_core_entity/src/lib.rs

- L134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L245: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L287: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L328: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L406: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L541: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L603: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L642: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L698: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L806: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L848: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L912: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L956: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/organization_core_express/src/lib.rs

- L20: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26: `AppError::internal_with_source(`
- L43: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L48: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L80: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/orm/src/lib.rs

- L44: `//!         .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/icon.rs

- L38: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L105: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/lib.rs

- L138: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/password.rs

- L86: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/regist.rs

- L76: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L89: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L158: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L176: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L194: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/reset.rs

- L177: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L303: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L315: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/signature.rs

- L87: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L120: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L197: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal/src/u2.rs

- L55: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L115: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L272: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L299: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L307: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L328: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L392: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L404: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L433: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L466: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L527: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L531: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L562: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L619: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L716: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L790: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L832: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L928: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L944: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1013: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1034: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1322: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal_extend/src/avatar.rs

- L121: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L191: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/personal_extend/src/personal.rs

- L115: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L218: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L234: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/portal/src/lib.rs

- L59: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L119: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L180: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L292: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L368: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L414: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L453: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L489: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L574: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/portal_assemble_designer/src/lib.rs

- L61: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L83: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L224: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L343: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L369: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L377: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L404: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L456: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L834: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L894: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L947: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1000: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1052: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1116: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1180: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1299: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1354: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1435: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1519: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1579: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1637: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1698: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1742: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1803: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1855: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1904: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1979: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2025: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2080: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2178: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2280: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2364: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2431: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2493: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2615: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2789: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2849: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2885: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2939: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3007: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3087: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3100: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3171: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3203: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3287: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3313: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3359: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3406: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3419: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3443: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3645: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3702: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/portal_assemble_designer/src/u2_script.rs

- L45: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L77: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L85: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L123: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L135: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/portal_assemble_surface/src/lib.rs

- L63: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L138: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L233: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L272: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L284: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L319: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L382: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L424: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L436: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L577: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L689: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L726: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L783: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L817: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L875: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L964: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1011: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1054: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L1063: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1075: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1127: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1243: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1375: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1477: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1535: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1590: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1649: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1705: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1763: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1802: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1857: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1908: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1960: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2010: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2042: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2071: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2161: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2213: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2385: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2444: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2568: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2640: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2663: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L2672: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L2745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2758: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2777: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2833: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2858: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/portal_core_entity/src/lib.rs

- L65: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L105: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L355: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/process_bam/src/lib.rs

- L28: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L72: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L136: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/process_designer/src/lib.rs

- L113: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L231: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L323: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_assemble_bam/src/lib.rs

- L40: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L97: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L123: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L340: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L398: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L454: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L498: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L593: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L624: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L717: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L768: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L795: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L830: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L872: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L925: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L964: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L995: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1042: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1088: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1139: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1166: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1243: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1365: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1413: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1512: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1539: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1575: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1665: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1716: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1772: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1816: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1873: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1924: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1981: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2025: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2079: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2089: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2099: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2218: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2294: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2331: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2394: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2405: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2576: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_assemble_designer/src/lib.rs

- L67: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L158: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L179: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L255: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L314: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L323: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L387: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L536: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L563: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L615: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L635: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L666: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L701: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L724: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L755: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L784: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L819: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L842: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L868: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L894: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L920: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L951: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L993: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1013: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1036: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1071: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1141: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1175: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1256: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1283: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1310: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1347: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1370: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1390: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1607: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1631: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1688: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1746: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1803: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1854: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1907: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1927: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1950: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2039: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2071: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2143: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2331: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2409: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2504: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2529: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2584: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2628: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2677: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2715: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2775: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2810: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2835: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2860: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2880: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2922: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2981: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3039: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3066: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3092: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3121: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3153: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3173: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3235: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3299: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3355: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3467: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3522: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3572: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3674: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3701: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3706: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3764: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3796: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3822: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3829: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3882: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3909: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3918: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3953: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3980: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4001: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4028: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4080: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))? as i64;`
- L4135: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))? as i64;`
- L4170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4223: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4310: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4332: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4412: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4436: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L4520: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L4564: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4640: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4670: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4718: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4768: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4793: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4800: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4826: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4858: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4887: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5021: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5156: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_assemble_designer/src/u2_script.rs

- L49: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L56: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L89: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L139: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_assemble_surface/src/lib.rs

- L79: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L114: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L223: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L248: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L313: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L376: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L423: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L436: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L496: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L501: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L567: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L580: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L641: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L680: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L718: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L783: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L836: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L848: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L893: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L933: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L981: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1022: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1064: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1190: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1243: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1288: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1334: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1385: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1397: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1405: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1514: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1573: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1581: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1632: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1644: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1733: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1804: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1816: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1858: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1905: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1917: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1925: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1976: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1988: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2038: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2093: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2105: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2113: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2172: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2184: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2321: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2329: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2398: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2539: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2551: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2630: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2642: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2706: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2775: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2787: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2795: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2868: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2880: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2922: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2955: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2997: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3009: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3014: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3060: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3202: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3252: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3354: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3366: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3514: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3566: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3682: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3694: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3742: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3800: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3812: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3861: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3920: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3932: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3976: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4030: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4042: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4084: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4136: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4190: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4254: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4298: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4339: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4344: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4392: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4404: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4446: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4495: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4507: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4512: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4572: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4614: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4663: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4680: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4728: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4740: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4787: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4841: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4853: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4858: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4911: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4923: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4971: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5026: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5038: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5043: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5097: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5158: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5231: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5286: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5298: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5342: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5393: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5405: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5514: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5563: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5575: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5580: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5628: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5640: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5682: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5731: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5743: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5748: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5796: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5808: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5850: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5900: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5950: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6002: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6014: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6056: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6120: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6162: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6273: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6342: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6390: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6448: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6509: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6568: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6580: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6624: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6678: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6690: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6732: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6784: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6796: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6838: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6890: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6902: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6944: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6986: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7034: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7084: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7185: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7307: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7352: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7418: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7468: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7518: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7530: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7572: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7614: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7655: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7667: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7672: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7720: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7732: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7774: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7824: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7866: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7908: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7950: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7995: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L8004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8030: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L8049: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8091: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8133: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8175: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8217: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8259: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8301: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8343: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8388: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8454: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8498: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8510: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8566: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8608: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8692: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8734: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8776: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8809: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8821: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8854: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8893: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8952: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8964: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9008: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9013: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9059: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9251: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9390: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9474: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9516: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L9577: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L9625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9681: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9719: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L9741: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9815: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9862: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9920: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9973: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10042: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10150: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10250: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10350: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10400: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10450: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10500: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10550: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10600: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10644: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10664: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10715: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10781: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10831: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10884: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10950: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11000: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11050: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11092: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11176: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11217: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11229: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11234: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11294: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11381: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11393: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11435: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11477: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11522: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11534: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11576: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11663: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11675: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11717: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L11784: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11822: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L11844: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11856: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11918: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11965: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12023: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12076: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12145: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12211: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12261: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12311: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12361: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12461: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12611: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12661: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12707: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L12729: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12780: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12846: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12896: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12949: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13015: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13065: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13115: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13157: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13198: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13261: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13303: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13345: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13395: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13487: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13539: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13589: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13643: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13726: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13767: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13779: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13784: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13832: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13844: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13882: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L13905: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L13949: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L13996: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14060: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14168: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L14188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14239: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14305: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14355: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14479: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14537: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14587: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14637: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14679: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14721: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14763: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14804: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14816: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14821: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14864: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14912: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14924: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L14966: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15008: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15050: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15092: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15176: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15229: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15353: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15413: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15459: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L15471: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15483: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15528: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L15540: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15552: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15594: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15636: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15728: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15769: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15781: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15786: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15832: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15873: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15885: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15890: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L15932: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L15955: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L16020: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16058: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L16080: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16092: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16259: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16386: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16461: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16503: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16553: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16603: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16703: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16787: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16837: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16887: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16937: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L16987: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17029: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17071: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17115: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L17135: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17186: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17252: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17355: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17471: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17521: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17566: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17623: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17635: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17677: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17719: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17761: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17803: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17845: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17887: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17928: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17940: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17945: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L17993: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18005: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18047: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18092: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18104: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18146: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18275: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18287: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18329: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18371: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18413: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18458: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18470: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18512: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L18631: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18669: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L18691: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18703: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18765: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18812: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18870: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18923: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L18992: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19058: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19158: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19258: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19308: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19458: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19508: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19558: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19608: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19646: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L19668: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19719: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19785: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19835: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19888: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L19954: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20054: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20096: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20137: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20149: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20245: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20257: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20299: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20381: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20443: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20485: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L20524: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L20547: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L20570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20612: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L20672: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20746: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20809: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20874: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L20951: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21026: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21076: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21126: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21168: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21310: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21544: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21594: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21644: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21694: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21775: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21826: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21892: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21942: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L21992: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22042: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22084: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22129: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22141: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22227: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22239: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22281: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22325: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22337: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22379: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22423: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22435: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22477: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22519: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22603: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22645: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22687: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22729: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22771: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22813: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22855: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22897: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22938: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22950: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L22955: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23001: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23046: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23058: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23100: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23142: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23184: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23237: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23288: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23329: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23346: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23392: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23434: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23476: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23522: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L23544: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23556: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23618: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23683: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23747: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23822: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23872: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23922: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L23964: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24006: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24056: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24190: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24276: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24288: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24372: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24414: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24455: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24467: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24518: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24610: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24660: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24710: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24864: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24893: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24933: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24969: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L24972: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25004: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L25026: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L25117: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25241: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L25384: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25407: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25475: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L25487: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L25533: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L25620: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25650: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L25825: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L26264: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26280: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26504: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26550: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26567: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26589: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26669: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26706: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26760: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26814: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26817: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26821: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L26858: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L26900: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27072: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27180: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27218: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27239: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27296: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27336: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27562: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L27577: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27629: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27652: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27721: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27755: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L27931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L27947: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28008: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28030: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28084: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28095: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28174: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28197: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28272: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28364: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28389: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28398: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28405: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28464: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28564: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28598: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28699: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28739: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28762: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28813: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28871: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L28909: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_assemble_surface/src/u2_render.rs

- L74: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L217: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L283: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L377: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L405: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_core_entity/src/lib.rs

- L28: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L76: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L168: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L204: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_core_express/src/lib.rs

- L30: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L43: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L69: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L102: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L136: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L197: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_service_processing/src/lib.rs

- L36: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L168: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L238: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L283: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L380: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L389: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L447: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L517: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L546: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L580: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L588: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L595: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L602: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L633: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L639: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L647: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L663: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L669: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L676: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L683: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L734: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L779: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L788: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L794: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L837: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L843: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L850: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L889: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L907: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L939: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L981: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1003: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1026: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1053: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1060: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1080: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1147: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1155: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1174: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1181: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1189: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1219: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1251: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1258: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1278: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1285: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1305: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1329: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1346: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1366: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1373: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1394: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1421: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1428: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1457: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1479: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1496: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1508: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1527: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1552: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1559: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1596: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1635: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1657: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1686: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1694: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1713: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1744: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1765: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1775: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1782: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1795: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1816: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1823: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1841: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1862: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1885: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1893: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1900: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1907: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1929: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1937: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1954: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1963: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1980: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1987: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1994: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2028: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2060: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2090: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2099: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2116: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2133: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2150: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2159: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2202: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2209: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2228: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2276: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2303: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2367: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2380: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2420: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2429: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2458: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2467: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2477: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2494: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2528: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2592: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2620: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2636: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2645: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2683: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2705: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2750: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2772: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2794: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2816: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2833: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2847: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2886: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2898: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2936: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2951: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2975: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2992: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3009: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3031: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3053: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3070: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3080: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3105: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3126: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3207: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3229: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3256: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3278: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3307: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3338: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3354: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3380: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3388: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3391: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3428: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3443: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3473: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3496: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3503: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3568: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3594: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3625: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3633: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3682: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3705: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3720: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3730: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3738: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3760: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3769: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3790: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3813: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3827: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3835: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3843: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3852: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3872: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3896: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3905: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3930: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L3939: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3962: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3993: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4009: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4101: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L4111: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4147: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4166: `AppError::internal_with_source(concat!(file!(), ":", line!()), e)`
- L4186: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/processplatform_service_processing/src/u2.rs

- L103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L177: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L234: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L253: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L279: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L290: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L319: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L420: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L439: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L470: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L490: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L517: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L534: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L541: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L575: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L731: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L789: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L805: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L833: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L843: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L872: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L912: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L923: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L949: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L976: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L983: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1002: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1023: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1055: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1107: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1131: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1147: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1153: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1216: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1354: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1365: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1374: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1395: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1413: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L1455: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1524: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1549: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1594: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1605: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1613: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1621: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1635: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1651: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1733: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1758: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1777: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1821: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1845: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1865: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1886: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1918: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1953: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1988: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2013: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2037: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2063: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2075: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2086: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2133: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2167: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2190: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2197: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2255: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2288: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2296: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2305: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2324: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2390: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2401: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2411: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2437: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2460: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2472: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2506: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2523: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2549: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2560: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2583: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2595: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2623: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2632: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2655: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2668: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2671: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/program_center/src/lib.rs

- L55: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L98: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L141: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L193: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L264: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L376: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L399: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L458: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L500: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L542: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L584: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L623: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L668: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L691: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L737: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L783: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L841: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L864: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L922: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L945: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1003: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1026: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1084: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1107: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1165: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1188: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1246: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1269: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1350: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1449: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1502: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1555: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1608: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1659: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1715: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1733: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1763: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1809: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1870: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1886: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1943: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2000: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2022: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2044: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2093: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2144: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2296: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2347: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2399: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2451: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2500: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2551: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2602: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2704: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2755: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2806: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2857: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2908: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2982: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3030: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3078: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3216: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3270: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3324: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3378: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3578: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3624: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3670: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3730: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3752: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3807: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3853: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3919: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3972: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4030: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4083: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4129: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4196: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4219: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4265: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4316: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4387: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4477: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4510: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4575: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L4586: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L4595: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4622: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4677: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4712: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4737: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4773: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4804: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4822: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4875: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4928: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4986: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5037: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5088: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5141: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5194: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5351: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5395: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5454: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5479: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5488: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5514: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5523: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5547: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5605: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5629: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5694: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5743: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5761: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5814: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5877: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5930: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L5983: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6034: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6080: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6136: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6201: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6275: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6326: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6368: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6418: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6466: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6519: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6564: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6609: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6654: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6699: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6741: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6799: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6828: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6857: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6886: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6919: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6941: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L6969: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7027: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7085: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7165: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7207: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7275: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7335: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7395: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7455: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7521: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7581: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7641: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7701: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7753: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7788: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7814: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7854: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7896: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7921: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7956: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L7990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8015: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8137: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8160: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8220: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8288: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8349: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8415: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8491: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8549: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8601: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8705: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8764: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8774: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8798: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8864: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8924: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L8990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9050: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9127: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9151: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9180: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9218: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9362: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9393: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9483: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9523: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9585: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9611: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L9990: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10206: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10245: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10281: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10304: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10348: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10360: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10420: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10642: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10695: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10764: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10782: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10807: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10830: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10872: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10873: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10882: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10913: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10955: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10977: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L10978: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L10985: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11078: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11102: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11128: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11147: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11171: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11225: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11263: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11298: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11403: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11531: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11589: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11639: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11696: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11718: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11745: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11793: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11818: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11841: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11861: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11876: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L11982: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12003: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12023: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L12033: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L12048: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12078: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12131: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12177: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12205: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12222: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12241: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12312: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12337: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12359: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12436: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L12447: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L12456: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12494: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L12507: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12536: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12552: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12604: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12631: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12653: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12698: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L12714: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/program_center_core_entity/src/handlers/agent.rs

- L43: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L128: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/program_center_core_entity/src/handlers/application.rs

- L44: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L139: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L207: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/program_center_core_entity/src/handlers/invoke.rs

- L42: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L125: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L187: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/program_center_core_entity/src/handlers/script.rs

- L37: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L114: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/program_center_core_entity/src/handlers/structure.rs

- L44: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L148: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L226: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`

### backend/crates/program_init/src/lib.rs

- L75: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L83: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L99: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L112: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L134: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L140: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L173: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L192: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L220: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L306: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L327: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L374: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L487: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L553: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L562: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L595: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L598: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L611: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L634: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L705: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L766: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_assemble_designer/src/lib.rs

- L49: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L199: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))`
- L227: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L244: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L274: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L300: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L319: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L330: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L363: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L385: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L393: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L402: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L435: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L507: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L533: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L599: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L634: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L666: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L732: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L750: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L785: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1002: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1059: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1090: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1232: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1269: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1318: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1358: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1385: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1429: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1464: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1503: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1532: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1604: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1632: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1757: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1798: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1825: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1883: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1927: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1986: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2052: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2106: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2175: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2218: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2276: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2364: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2431: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2512: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2581: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2634: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2696: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2731: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2831: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2913: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2993: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3041: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3108: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3156: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3183: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3212: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3291: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3316: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3363: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3410: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3447: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3473: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3519: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3550: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3580: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3588: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3615: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3655: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3727: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3791: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3839: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3892: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3931: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3963: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3997: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L4145: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_assemble_designer/src/u2_closures.rs

- L214: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L320: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L332: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L375: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L399: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L419: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L444: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L474: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L482: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L547: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L570: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L637: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L657: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L690: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L736: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L777: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L810: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L855: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L868: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L880: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L904: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L914: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L959: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L995: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1012: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1038: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1083: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1113: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1121: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1149: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1157: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1202: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1247: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1257: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L1269: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1296: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?,`
- L1306: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1351: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1384: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1432: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1478: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1538: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1619: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1637: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1684: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1722: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1731: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1781: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1818: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1835: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1873: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1919: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1938: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1986: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2027: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2036: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2158: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2191: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2242: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2290: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2297: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2339: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2396: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2408: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2445: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2457: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L2473: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_assemble_surface/src/lib.rs

- L58: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L123: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L149: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L208: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L292: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L832: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L872: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L940: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1022: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1097: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1170: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1221: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1260: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1317: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1381: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1395: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1426: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1500: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1624: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1697: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1773: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1841: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1903: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1949: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1995: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2043: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2110: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2164: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2186: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2237: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2283: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2313: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2343: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2389: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2428: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2467: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2505: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2513: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2543: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2554: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2585: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2662: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2737: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2775: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2783: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2814: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2862: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2893: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2948: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2956: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L2988: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3046: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3101: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3154: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3192: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3200: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3230: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3269: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3317: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3347: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3414: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3422: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L3453: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_assemble_surface/src/u2_closures.rs

- L224: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L245: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L309: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L345: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L368: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L450: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L484: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L519: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L540: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L561: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L609: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L617: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L694: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L702: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L735: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L762: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L770: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L796: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L804: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L832: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L866: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L888: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L896: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L922: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1025: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1058: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1109: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1131: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1163: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e)),`
- L1275: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1295: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1329: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L1429: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_core_entity/src/lib.rs

- L66: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L130: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L224: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L243: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L302: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_core_express/src/lib.rs

- L211: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L280: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L311: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L322: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_express/src/lib.rs

- L37: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L94: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_service/src/lib.rs

- L40: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L65: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L169: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L182: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L197: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_service_processing/src/lib.rs

- L52: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L124: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L210: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L268: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L277: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L282: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/query_service_processing/src/u2.rs

- L127: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L215: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L341: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L355: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L366: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L469: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L499: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L511: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L523: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L566: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L579: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L606: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L683: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/search/src/lib.rs

- L64: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L103: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L141: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L299: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/shared/src/crud.rs

- L72: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L117: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L136: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L158: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`
- L190: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?`
- L314: `.map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;`

### backend/crates/shared/src/error.rs

- L104: `pub fn internal_with_source<C, E>(context: C, source: E) -> Self`
- L309: `/// O23 断言：`internal_with_source` 必须保留原始 source（根因可追溯）。`
- L312: `fn internal_with_source_keeps_root_cause() {`
- L313: `let err = AppError::internal_with_source(`
- L331: `async fn internal_with_source_maps_500_without_leaking() {`
- L332: `let resp = AppError::internal_with_source("unit:7", "secret-detail").into_response();`
- L367: `let err = AppError::internal_with_source("unit:9", "marker-source");`

## 剩余直接 `AppError::Internal` 逐处清单（例外）


### backend/crates/ai_assemble_control/src/lib.rs

- L1994: `.ok_or(AppError::Internal)`
- L2273: `return Err(AppError::Internal);`
- L2292: `yield Err(AppError::Internal);`

### backend/crates/attendance_core_entity/src/lib.rs

- L152: `.ok_or_else(|| AppError::Internal)?;`
- L197: `.ok_or_else(|| AppError::Internal)?;`
- L276: `.ok_or_else(|| AppError::Internal)?;`
- L337: `.ok_or_else(|| AppError::Internal)?;`

### backend/crates/auth/src/captcha.rs

- L61: `CaptchaError::NotFound => AppError::Internal,`
- L62: `_ => AppError::Internal,`

### backend/crates/auth/src/ldap_auth.rs

- L45: `Err(AppError::Internal)`

### backend/crates/auth/src/lib.rs

- L963: `let data = result.data.ok_or(AppError::Internal)?;`
- L967: `.ok_or(AppError::Internal)?`
- L972: `.ok_or(AppError::Internal)?`

### backend/crates/auth/src/oauth.rs

- L307: `return Err(AppError::Internal);`
- L312: `.ok_or(AppError::Internal)?;`
- L329: `return Err(AppError::Internal);`
- L335: `.ok_or(AppError::Internal)`
- L359: `return Err(AppError::Internal);`
- L364: `.ok_or(AppError::Internal)?;`
- L384: `return Err(AppError::Internal);`
- L391: `.ok_or(AppError::Internal)`
- L419: `let base = extract_scheme_host_port(&oauth_redirect_base()).ok_or(AppError::Internal)?;`

### backend/crates/auth/src/oidc.rs

- L219: `.ok_or(AppError::Internal)?;`
- L232: `.ok_or(AppError::Internal)`
- L237: `.ok_or(AppError::Internal)`

### backend/crates/auth/src/qiyeweixin.rs

- L342: `let ticket = get_jsapi_ticket().ok_or(AppError::Internal)?;`
- L349: `let agentid = get_qywx_agent_id().ok_or(AppError::Internal)?;`

### backend/crates/bbs_core_entity/src/lib.rs

- L722: `AppError::Internal`

### backend/crates/cms_assemble_control/src/tests_data_appdict.rs

- L230: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L248: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/cms_assemble_control/src/tests_u2.rs

- L282: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/cms_assemble_control/src/tests_u3.rs

- L341: `Err(AppError::Forbidden) | Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L355: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L419: `Err(AppError::Internal) | Err(AppError::Forbidden) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/control/src/tests.rs

- L67: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L91: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L111: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L135: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L159: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L320: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/correlation/src/tests.rs

- L38: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L60: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L82: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/empower/src/lib.rs

- L503: `.ok_or(AppError::Internal)?;`
- L549: `.ok_or(AppError::Internal)?;`
- L595: `.ok_or(AppError::Internal)?;`
- L641: `.ok_or(AppError::Internal)?;`
- L687: `.ok_or(AppError::Internal)?;`

### backend/crates/file_assemble_control/src/lib.rs

- L4640: `AppError::Internal`
- L4720: `AppError::Internal`

### backend/crates/meeting/src/security_gate_tests_b66.rs

- L13: `//!    `Err(AppError::Internal)`（会话放行后抵达 `pool.get()`，dead pool 取连接失败 → Internal），`

### backend/crates/meeting_assemble_control/src/lib.rs

- L3144: `AppError::Internal`
- L3149: `return Err(AppError::Internal);`
- L3278: `AppError::Internal`
- L3532: `AppError::Internal`
- L4339: `AppError::Internal`

### backend/crates/organization_assemble_authentication/src/lib.rs

- L205: `return Err(AppError::Internal);`
- L210: `.ok_or(AppError::Internal)?;`
- L227: `return Err(AppError::Internal);`
- L233: `.ok_or(AppError::Internal)`
- L419: `return Err(AppError::Internal);`
- L424: `.ok_or(AppError::Internal)?;`
- L441: `return Err(AppError::Internal);`
- L448: `.ok_or(AppError::Internal)`
- L627: `.ok_or(AppError::Internal)?`
- L645: `.ok_or(AppError::Internal)?`
- L752: `.ok_or(AppError::Internal)?`
- L769: `.ok_or(AppError::Internal)?`

### backend/crates/organization_assemble_control/src/lib.rs

- L832: `.map_err(|e| { eprintln!("DIAG org_sub_direct query err: {:?}", e); AppError::Internal })?;`

### backend/crates/organization_assemble_express/src/endpoints_person2.rs

- L69: `Err(_) => return Err(AppError::Internal),`

### backend/crates/personal/src/regist.rs

- L133: `AppError::Internal`

### backend/crates/portal/src/tests.rs

- L35: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`
- L50: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/processplatform_assemble_surface/src/lib.rs

- L25711: `AppError::Internal`
- L25790: `AppError::Internal`
- L26311: `AppError::Internal`

### backend/crates/processplatform_assemble_surface/src/security_gate_tests.rs

- L1654: `.filter(|(_, r)| matches!(r, Err(AppError::Internal)))`

### backend/crates/processplatform_assemble_surface/src/u2_render.rs

- L51: `AppError::Internal`
- L65: `AppError::Internal`
- L70: `AppError::Internal`
- L249: `AppError::Internal`
- L299: `AppError::Internal`

### backend/crates/program_center/src/lib.rs

- L9660: `AppError::Internal`
- L9673: `AppError::Internal`
- L9862: `AppError::Internal`
- L9891: `AppError::Internal`
- L9905: `AppError::Internal`
- L9961: `AppError::Internal`
- L10037: `AppError::Internal`
- L10071: `AppError::Internal`
- L10098: `AppError::Internal`
- L10122: `AppError::Internal`
- L10393: `AppError::Internal`
- L10469: `AppError::Internal`
- L11433: `AppError::Internal`
- L11921: `AppError::Internal`
- L11960: `AppError::Internal`

### backend/crates/program_center/src/tests.rs

- L53: `Err(AppError::Internal) | Err(AppError::Unavailable(_)) => {}`

### backend/crates/program_init/src/lib.rs

- L101: `return Err(AppError::Internal);`
- L659: `AppError::Internal`
- L669: `AppError::Internal`
- L686: `AppError::Internal`

### backend/crates/shared/src/db/mod.rs

- L344: `return AppError::Internal;`
- L356: `AppError::Internal`
- L386: `_ => AppError::Internal,`
- L395: `_ => AppError::Internal,`
- L951: `AppError::Internal`
- L1094: `matches!(err, AppError::Internal),`
- L1152: `status_of(AppError::Internal),`

### backend/crates/shared/src/error.rs

- L124: `AppError::InternalAnyhow(e) => format!("{e:?}"),`
- L174: `AppError::Internal => (`
- L178: `AppError::InternalAnyhow(_) => (`
- L310: `/// 破坏方式：换成 `AppError::Internal` 或把 cause 置空 → 此断言红。`
- L343: `/// 破坏方式：改回 `AppError::Internal` → 状态码断言红。`

### backend/crates/shared/src/lib.rs

- L233: `AppError::Internal`

### backend/crates/shared/src/panic_guard.rs

- L18: `//! 这一层把 panic 收敛成与 [`AppError::Internal`] 完全一致的 500 信封`
- L40: `AppError::Internal.into_response()`
- L71: `/// panic 必须变成 500 信封（与 `AppError::Internal` 同形状），`

### backend/crates/shared/src/response.rs

- L426: `(AppError::InternalAnyhow(anyhow::anyhow!("boom")), 500),`
- L427: `(AppError::Internal, 500),`

### backend/crates/shared/src/testing.rs

- L39: `/// DbErr → AppError::Internal（500），测试结果与环境无关。`

### backend/crates/shared/src/tests.rs

- L931: `let err = AppError::Internal;`
