# final coverage sweep — plan002 U2 收官验证

- 生成时间：2026-08-24 15:02　|　清单：`java-endpoint-inventory.json`（generated_at=2026-08-23T13:57:31）
- 扫描对象：`crates/*/src/**/*.rs` 共 4152 条 `.route(` 注册（唯一归一化路径 3784 条），覆盖单行/多行注册、`&fmt()+JAVA_BASE` 间接寻址、链式多方法。
- 匹配口径：路径参数归一化为 `{}`；exact（method+全路径，允许 Rust 侧更长前缀）∪ casefold 计入覆盖；verb_mismatch / literal_shift 仅诊断不计入（影子副本会真实 404，见 alignment-reconciliation.md §2.3）。
- 留档排除项对照 `docs/plans/2026-08-21-002` 台账 U2 行。

## 一、结论速览

| 指标 | 数值 |
|------|------|
| 有端点的 Java 模块组 | 30 / 55 |
| Java 唯一端点（模块内去重合计，台账口径） | 3092（清单 totals=3092） |
| **总覆盖端点数（同口径）** | **2626** |
| **总覆盖率** | **84.93%** |
| 严格全局并集（跨模块去重后） | 2421/2868 = 84.41%（跨模块重叠 224 对，如 query designer/surface 共享 statement 族） |
| 模块内口径合计 | 2626/3092 = 84.9% |
| **达到 100% 的模块数** | **11** |
| 未覆盖端点总数 | 466 |
| 其中 axum 平台限制留档 | 4 |
| 其中 cms 语义不匹配留档 | 0 |
| **排除留档后剩余缺口** | **462** |

## 二、模块覆盖明细

| 模块 | 唯一端点 | 已覆盖 | 覆盖率 | 状态 |
|------|---------:|-------:|-------:|------|
| `x_processplatform_assemble_bam` | 45 | 3 | 6.7% | ⚠️ 缺口 |
| `x_mind_assemble_control` | 23 | 5 | 21.7% | ⚠️ 缺口 |
| `x_jpush_assemble_control` | 9 | 2 | 22.2% | ⚠️ 缺口 |
| `x_query_assemble_surface` | 70 | 20 | 28.6% | ⚠️ 缺口 |
| `x_portal_assemble_surface` | 38 | 11 | 28.9% | ⚠️ 缺口 |
| `x_calendar_assemble_control` | 31 | 9 | 29.0% | ⚠️ 缺口 |
| `x_hotpic_assemble_control` | 12 | 5 | 41.7% | ⚠️ 缺口 |
| `x_query_assemble_designer` | 90 | 38 | 42.2% | ⚠️ 缺口 |
| `x_component_assemble_control` | 7 | 3 | 42.9% | ⚠️ 缺口 |
| `x_program_center` | 252 | 133 | 52.8% | ⚠️ 缺口 |
| `x_general_assemble_control` | 46 | 25 | 54.3% | ⚠️ 缺口 |
| `x_processplatform_assemble_designer` | 117 | 66 | 56.4% | ⚠️ 缺口 |
| `x_portal_assemble_designer` | 64 | 41 | 64.1% | ⚠️ 缺口 |
| `x_organization_assemble_authentication` | 53 | 43 | 81.1% | ⚠️ 缺口 |
| `x_base_core_project` | 8 | 7 | 87.5% | ⚠️ 缺口 |
| `x_message_assemble_communicate` | 64 | 62 | 96.9% | ⚠️ 缺口 |
| `x_organization_assemble_personal` | 76 | 75 | 98.7% | ⚠️ 缺口 |
| `x_processplatform_assemble_surface` | 659 | 651 | 98.8% | ⚠️ 缺口 |
| `x_bbs_assemble_control` | 106 | 105 | 99.1% | ⚠️ 缺口 |
| `x_ai_assemble_control` | 33 | 33 | 100.0% | ✅ 100% |
| `x_attendance_assemble_control` | 180 | 180 | 100.0% | ✅ 100% |
| `x_cms_assemble_control` | 437 | 437 | 100.0% | ✅ 100% |
| `x_correlation_service_processing` | 12 | 12 | 100.0% | ✅ 100% |
| `x_file_assemble_control` | 105 | 105 | 100.0% | ✅ 100% |
| `x_meeting_assemble_control` | 76 | 76 | 100.0% | ✅ 100% |
| `x_organization_assemble_control` | 187 | 187 | 100.0% | ✅ 100% |
| `x_organization_assemble_express` | 132 | 132 | 100.0% | ✅ 100% |
| `x_processplatform_service_processing` | 121 | 121 | 100.0% | ✅ 100% |
| `x_program_init` | 15 | 15 | 100.0% | ✅ 100% |
| `x_query_service_processing` | 24 | 24 | 100.0% | ✅ 100% |

> 无 JAXRS 端点的模块（25 个，不计入分母）：`x_ai_core_entity`、`x_attendance_core_entity`、`x_bbs_core_entity`、`x_calendar_core_entity`、`x_cms_core_entity`、`x_cms_core_express`、`x_component_core_entity`、`x_console`、`x_correlation_core_entity`、`x_correlation_core_express`、`x_file_core_entity`、`x_general_core_entity`、`x_hotpic_core_entity`、`x_jpush_core_entity`、`x_meeting_core_entity`、`x_message_core_entity`、`x_mind_core_entity`、`x_organization_core_entity`、`x_organization_core_express`、`x_portal_core_entity`、`x_processplatform_core_entity`、`x_processplatform_core_express`、`x_program_center_core_entity`、`x_query_core_entity`、`x_query_core_express`

## 三、未覆盖端点（按模块分组，标注排除类别）

> 判定图例：🔴 缺失＝任何形态均无注册；🔵 动词差＝路径已有但缺该 HTTP 方法变体；🟣 形变疑云＝存在同段数形变候选（影子副本会真实 404，不计入覆盖）；🟠 平台限制＝axum 无法表达（单段多参数）；🟡 语义留档＝台账记录的语义不匹配。
### x_program_center（缺 119 / 252）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/agent` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/agent/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE） |
| PUT | `/agent/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：DELETE） |
| GET | `/agent/{}/disable` | 🔴 缺失 |  |
| GET | `/agent/{}/enable` | 🔴 缺失 |  |
| GET | `/agent/{}/execute` | 🔴 缺失 |  |
| PUT | `/agent/{}/file` | 🔴 缺失 |  |
| PUT | `/appstyle` | 🔴 缺失 |  |
| GET | `/appstyle/current/update` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST/PUT） |
| PUT | `/appstyle/image/application/top` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/appstyle/image/application/top/erase` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE） |
| PUT | `/appstyle/image/launch/logo` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| PUT | `/appstyle/image/login/avatar` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| PUT | `/appstyle/image/menu/logo/blur` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/appstyle/image/menu/logo/blur/erase` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE） |
| PUT | `/appstyle/image/menu/logo/focus` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/appstyle/image/menu/logo/focus/erase` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE） |
| PUT | `/appstyle/image/process/default` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/appstyle/image/process/default/erase` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE） |
| PUT | `/appstyle/image/setup/about/logo` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/appstyle/image/setup/about/logo/erase` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE） |
| GET | `/bar/create/mass/{}/{}` | 🔴 缺失 |  |
| GET | `/bar/select1/field/{}/value/{}/count/{}` | 🔴 缺失 |  |
| GET | `/bar/select2/count/{}` | 🔴 缺失 |  |
| GET | `/bar/select3/field/{}/value/{}/count/{}` | 🔴 缺失 |  |
| GET | `/bar/select4/field/{}/value/{}/count/{}` | 🔴 缺失 |  |
| GET | `/captcha/v2/create/width/{}/height/{}` | 🔴 缺失 |  |
| GET | `/captcha/{}/validate/answer/{}` | 🔴 缺失 |  |
| GET | `/code/create/mobile/{}` | 🔴 缺失 |  |
| POST | `/code/list/paging/{}/size/{}` | 🔴 缺失 |  |
| GET | `/code/validate/mobile/{}/answer/{}` | 🔴 缺失 |  |
| GET | `/code/validate/mobile/{}/answer/{}/cascade` | 🔴 缺失 |  |
| POST | `/collect` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| PUT | `/collect` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/collect/code/mobile/{}` | 🔴 缺失 |  |
| GET | `/collect/controllermobile/name/{}/mobile/{}` | 🔴 缺失 |  |
| GET | `/collect/name/{}/exist` | 🔴 缺失 |  |
| DELETE | `/collect/name/{}/mobile/{}/code/{}` | 🔴 缺失 |  |
| PUT | `/collect/resetpassword` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| PUT | `/collect/urlMapping` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/config-open/get/disable/export/enable` | 🔴 缺失 |  |
| POST | `/config/change/password` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| PUT | `/config/collect` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| POST | `/config/open` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| POST | `/config/open/run/time/config` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| PUT | `/config/portal` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| PUT | `/config/proxy` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| POST | `/config/ternary/management` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| POST | `/deploy/list/paging/{}/size/{}` | 🔴 缺失 |  |
| POST | `/deploy/server/resource` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| POST | `/deploy/web/resource/as/new/{}` | 🔴 缺失 |  |
| GET | `/deploy/{}` | 🔴 缺失 |  |
| POST | `/dict/list/paging/{}/size/{}` | 🔴 缺失 |  |
| PUT | `/dict/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：DELETE/GET） |
| GET | `/dict/{}/data` | 🔴 缺失 |  |
| GET | `/dict/{}/{}/data` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE/POST/PUT） |
| GET | `/dict/{}/{}/data/mockdeletetoget` | 🔴 缺失 |  |
| POST | `/dict/{}/{}/data/mockputtopost` | 🔴 缺失 |  |
| GET | `/dingding/sync/organization/register/callback/{}` | 🔴 缺失 |  |
| GET | `/distribute/assemble/source/{}` | 🔴 缺失 |  |
| GET | `/distribute/webserver/assemble/source/{}` | 🔴 缺失 |  |
| GET | `/foo/create/mass/{}/{}` | 🔴 缺失 |  |
| GET | `/invoke` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| GET | `/invoke/list/with/category/{}` | 🔴 缺失 |  |
| POST | `/invoke/{}/client/{}/token/{}/execute` | 🔴 缺失 |  |
| POST | `/invoke/{}/execute` | 🔴 缺失 |  |
| GET | `/invoke/{}/execute/get` | 🔴 缺失 |  |
| PUT | `/invoke/{}/file` | 🔴 缺失 |  |
| GET | `/jest/clear/cache/{}` | 🔴 缺失 |  |
| POST | `/market/list/install/log/paging/{}/size/{}` | 🔴 缺失 |  |
| POST | `/market/list/paging/{}/size/{}` | 🔴 缺失 |  |
| GET | `/market/list/paging/{}/size/{}/category/{}` | 🔴 缺失 |  |
| GET | `/market/{}` | 🔴 缺失 |  |
| GET | `/market/{}/cover/pic` | 🔴 缺失 |  |
| GET | `/market/{}/download` | 🔴 缺失 |  |
| GET | `/market/{}/install/log` | 🔴 缺失 |  |
| GET | `/market/{}/install/or/update` | 🔴 缺失 |  |
| GET | `/market/{}/installed/version` | 🔴 缺失 |  |
| GET | `/market/{}/uninstall` | 🔴 缺失 |  |
| PUT | `/module/compare/upload` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：POST） |
| PUT | `/module/list` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| PUT | `/module/output` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：POST） |
| GET | `/module/output/structure` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| GET | `/module/output/{}/file` | 🔴 缺失 |  |
| DELETE | `/module/remove/structure/{}` | 🔴 缺失 |  |
| PUT | `/module/write/{}` | 🔴 缺失 |  |
| GET | `/module/{}/compare` | 🔴 缺失 |  |
| POST | `/mpweixin/check` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/mpweixin/menu/create/to/weixin` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| DELETE | `/mpweixin/menu/delete/{}` | 🔴 缺失 |  |
| POST | `/mpweixin/menu/update/{}` | 🔴 缺失 |  |
| POST | `/mpweixin/message/template/send` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/output/{}/select/file` | 🔴 缺失 |  |
| POST | `/prompterrorlog` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/next/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/next/{}/date/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/next/{}/exceptionclass/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/next/{}/loggername/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/prev/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/prev/{}/date/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/prev/{}/exceptionclass/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/list/{}/prev/{}/loggername/{}` | 🔴 缺失 |  |
| GET | `/prompterrorlog/{}` | 🔴 缺失 |  |
| GET | `/qiyeweixin` | 🔴 缺失 |  |
| POST | `/qiyeweixin` | 🔴 缺失 |  |
| POST | `/qiyeweixin/request/pull/sync` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/schedule/list/schedulelog/application/{}` | 🔴 缺失 |  |
| POST | `/schedule/schedule/fire` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| POST | `/script/name/{}` | 🔴 缺失 |  |
| GET | `/script/name/{}/imported` | 🔴 缺失 |  |
| POST | `/unexpectederrorlog` | 🔴 缺失 |  |
| GET | `/unexpectederrorlog/list/{}/next/{}` | 🔴 缺失 |  |
| GET | `/unexpectederrorlog/list/{}/next/{}/date/{}` | 🔴 缺失 |  |
| GET | `/unexpectederrorlog/list/{}/prev/{}` | 🔴 缺失 |  |
| GET | `/unexpectederrorlog/list/{}/prev/{}/date/{}` | 🔴 缺失 |  |
| GET | `/unexpectederrorlog/{}` | 🔴 缺失 |  |
| GET | `/validation/timeout/{}` | 🔴 缺失 |  |
| GET | `/welink/pull/sync` | 🔴 缺失 |  |
| POST | `/welink/request/pull/sync` | 🔴 缺失 |  |

### x_query_assemble_designer（缺 52 / 90）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/importmodel/list/query/{}` | 🔴 缺失 |  |
| DELETE | `/importmodel/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：POST） |
| GET | `/importmodel/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| PUT | `/importmodel/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：POST） |
| POST | `/importmodel/{}/permission` | 🔴 缺失 |  |
| DELETE | `/neural/model/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/neural/model/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/neural/model/{}/reset/status` | 🔴 缺失 |  |
| GET | `/output/{}/select/file` | 🔴 缺失 |  |
| POST | `/query` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/query/entity/{}/category/{}/properties` | 🔴 缺失 |  |
| GET | `/query/list/all` | 🔴 缺失 |  |
| GET | `/query/list/querycategory/{}` | 🔴 缺失 |  |
| GET | `/query/list/summary` | 🔴 缺失 |  |
| GET | `/query/list/summary/querycategory/{}` | 🔴 缺失 |  |
| GET | `/query/querycategory/list` | 🔴 缺失 |  |
| DELETE | `/query/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET/POST） |
| PUT | `/query/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET/POST） |
| PUT | `/query/{}/icon` | 🔴 缺失 |  |
| POST | `/query/{}/permission` | 🔴 缺失 |  |
| GET | `/stat/list/{}/next/{}` | 🔴 缺失 |  |
| DELETE | `/stat/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/stat/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| POST | `/stat/{}/permission` | 🔴 缺失 |  |
| PUT | `/stat/{}/simulate` | 🔴 缺失 |  |
| POST | `/statement/{}/execute/mode/{}/page/{}/size/{}` | 🔴 缺失 |  |
| POST | `/statement/{}/execute/page/{}/size/{}` | 🔴 缺失 |  |
| POST | `/statement/{}/permission` | 🔴 缺失 |  |
| GET | `/table/export/{}/count/{}` | 🔴 缺失 |  |
| GET | `/table/list/query/{}` | 🔴 缺失 |  |
| GET | `/table/list/{}/row/select/where/{}` | 🔴 缺失 |  |
| GET | `/table/list/{}/row/{}/next/{}` | 🔴 缺失 |  |
| GET | `/table/list/{}/row/{}/prev/{}` | 🔴 缺失 |  |
| GET | `/table/query/{}/build` | 🔴 缺失 |  |
| DELETE | `/table/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/table/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/table/{}/build/dispatch` | 🔴 缺失 |  |
| POST | `/table/{}/execute` | 🔴 缺失 |  |
| POST | `/table/{}/permission` | 🔴 缺失 |  |
| POST | `/table/{}/row` | 🔴 缺失 |  |
| GET | `/table/{}/row/count/where/{}` | 🔴 缺失 |  |
| DELETE | `/table/{}/row/delete/all` | 🔴 缺失 |  |
| POST | `/table/{}/row/save` | 🔴 缺失 |  |
| DELETE | `/table/{}/row/{}` | 🔴 缺失 |  |
| GET | `/table/{}/row/{}` | 🔴 缺失 |  |
| PUT | `/table/{}/row/{}` | 🔴 缺失 |  |
| GET | `/table/{}/status/build` | 🔴 缺失 |  |
| GET | `/table/{}/status/draft` | 🔴 缺失 |  |
| GET | `/view/list/{}/next/{}` | 🔴 缺失 |  |
| PUT | `/view/{}/bundle` | 🔴 缺失 |  |
| POST | `/view/{}/permission` | 🔴 缺失 |  |
| PUT | `/view/{}/simulate` | 🔴 缺失 |  |

### x_processplatform_assemble_designer（缺 51 / 117）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| PUT | `/application/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET/POST） |
| PUT | `/application/{}/icon` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| POST | `/application/{}/permission` | 🔴 缺失 |  |
| GET | `/applicationcategory/list` | 🔴 缺失 |  |
| POST | `/applicationdict` | 🔴 缺失 |  |
| POST | `/applicationdict/list/paging/{}/size/{}` | 🔴 缺失 |  |
| DELETE | `/applicationdict/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET/PUT） |
| GET | `/elementtool/applicationdict/orphan` | 🔴 缺失 |  |
| GET | `/elementtool/form/orphan` | 🔴 缺失 |  |
| GET | `/elementtool/process/orphan` | 🔴 缺失 |  |
| GET | `/elementtool/script/orphan` | 🔴 缺失 |  |
| GET | `/file/{}/application/{}` | 🔴 缺失 |  |
| GET | `/form/list/{}/next/{}` | 🔴 缺失 |  |
| GET | `/form/list/{}/prev/{}` | 🔴 缺失 |  |
| POST | `/item-access` | 🔴 缺失 |  |
| POST | `/item-access/bach/save` | 🔴 缺失 |  |
| DELETE | `/item-access/delete/process/{}/path/{}` | 🔴 缺失 |  |
| GET | `/item-access/path/{}` | 🔴 缺失 |  |
| GET | `/item-access/process/{}` | 🔴 缺失 |  |
| GET | `/item-access/process/{}/path/{}` | 🔴 缺失 |  |
| GET | `/item-access/{}` | 🔴 缺失 |  |
| POST | `/mapping` | 🔴 缺失 |  |
| GET | `/mapping/list/{}/next/{}` | 🔴 缺失 |  |
| GET | `/mapping/list/{}/prev/{}` | 🔴 缺失 |  |
| DELETE | `/mapping/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/mapping/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/mapping/{}/execute` | 🔴 缺失 |  |
| POST | `/mergeitemplan` | 🔴 缺失 |  |
| POST | `/mergeitemplan/estimate` | 🔴 缺失 |  |
| GET | `/mergeitemplan/list/application/{}/paging/{}/size/{}` | 🔴 缺失 |  |
| GET | `/mergeitemplan/list/paging/{}/size/{}` | 🔴 缺失 |  |
| DELETE | `/mergeitemplan/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：POST） |
| GET | `/mergeitemplan/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| PUT | `/mergeitemplan/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：POST） |
| GET | `/process/application/{}/disable/edition` | 🔴 缺失 |  |
| GET | `/process/application/{}/edition/{}` | 🔴 缺失 |  |
| GET | `/process/upgrade/all` | 🔴 缺失 |  |
| PUT | `/process/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET/POST） |
| GET | `/process/{}/disable` | 🔴 缺失 |  |
| GET | `/process/{}/enable` | 🔴 缺失 |  |
| GET | `/process/{}/enabled` | 🔴 缺失 |  |
| POST | `/process/{}/execute/projection` | 🔴 缺失 |  |
| GET | `/process/{}/lead/out` | 🔴 缺失 |  |
| POST | `/process/{}/list/element` | 🔴 缺失 |  |
| POST | `/process/{}/permission` | 🔴 缺失 |  |
| GET | `/process/{}/process` | 🔴 缺失 |  |
| POST | `/process/{}/upgrade` | 🔴 缺失 |  |
| DELETE | `/process/{}/{}/edition` | 🔴 缺失 |  |
| GET | `/script/application/{}/name/{}` | 🔴 缺失 |  |
| GET | `/workcompleted/application/{}/merge/data` | 🔴 缺失 |  |
| GET | `/workcompleted/process/{}/merge/data` | 🔴 缺失 |  |

### x_query_assemble_surface（缺 50 / 70）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/importmodel/flag/{}/query/{}` | 🔴 缺失 |  |
| GET | `/importmodel/list/query/{}` | 🔴 缺失 |  |
| POST | `/importmodel/list/record/item/paging/{}/size/{}` | 🔴 缺失 |  |
| POST | `/importmodel/list/record/paging/{}/size/{}` | 🔴 缺失 |  |
| DELETE | `/importmodel/record/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：POST） |
| GET | `/importmodel/record/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| GET | `/importmodel/record/{}/mockdeletetoget` | 🔴 缺失 |  |
| GET | `/importmodel/record/{}/status` | 🔴 缺失 |  |
| GET | `/importmodel/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| POST | `/importmodel/{}/execute` | 🔴 缺失 |  |
| GET | `/query/list/key/{}` | 🔴 缺失 |  |
| PUT | `/stat/flag/{}/query/{}/execute` | 🔴 缺失 |  |
| POST | `/stat/flag/{}/query/{}/execute/mockputtopost` | 🔴 缺失 |  |
| POST | `/statement/{}/execute/mode/{}/page/{}/size/{}` | 🔴 缺失 |  |
| POST | `/statement/{}/execute/page/{}/size/{}` | 🔴 缺失 |  |
| POST | `/table/list/paging/{}/size/{}` | 🔴 缺失 |  |
| POST | `/table/list/table/{}/row/paging/{}/size/{}` | 🔴 缺失 |  |
| GET | `/table/list/{}/next/{}` | 🔴 缺失 |  |
| POST | `/table/list/{}/row/select` | 🔴 缺失 |  |
| GET | `/table/list/{}/row/select/where/{}` | 🔴 缺失 |  |
| GET | `/table/list/{}/row/{}/next/{}` | 🔴 缺失 |  |
| GET | `/table/list/{}/row/{}/prev/{}` | 🔴 缺失 |  |
| POST | `/table/{}/row` | 🔴 缺失 |  |
| GET | `/table/{}/row/count/where/{}` | 🔴 缺失 |  |
| DELETE | `/table/{}/row/delete/all` | 🔴 缺失 |  |
| GET | `/table/{}/row/delete/all/mockdeletetoget` | 🔴 缺失 |  |
| POST | `/table/{}/row/one` | 🔴 缺失 |  |
| DELETE | `/table/{}/row/{}` | 🔴 缺失 |  |
| GET | `/table/{}/row/{}` | 🔴 缺失 |  |
| PUT | `/table/{}/row/{}` | 🔴 缺失 |  |
| GET | `/table/{}/row/{}/mockdeletetoget` | 🔴 缺失 |  |
| POST | `/table/{}/row/{}/mockputtopost` | 🔴 缺失 |  |
| POST | `/table/{}/row/{}/part/update` | 🔴 缺失 |  |
| GET | `/view/excel/result/{}` | 🔴 缺失 |  |
| GET | `/view/flag/{}/query/{}` | 🔴 缺失 |  |
| PUT | `/view/flag/{}/query/{}/bundle` | 🔴 缺失 |  |
| POST | `/view/flag/{}/query/{}/bundle/mockputtopost` | 🔴 缺失 |  |
| PUT | `/view/flag/{}/query/{}/excel` | 🔴 缺失 |  |
| POST | `/view/flag/{}/query/{}/excel/mockputtopost` | 🔴 缺失 |  |
| PUT | `/view/flag/{}/query/{}/execute` | 🔴 缺失 |  |
| POST | `/view/flag/{}/query/{}/execute/mockputtopost` | 🔴 缺失 |  |
| POST | `/view/flag/{}/query/{}/execute/v2/page/{}/size/{}` | 🔴 缺失 |  |
| PUT | `/view/{}/bundle` | 🔴 缺失 |  |
| POST | `/view/{}/bundle/mockputtopost` | 🔴 缺失 |  |
| POST | `/view/{}/bundle/v2` | 🔴 缺失 |  |
| PUT | `/view/{}/excel` | 🔴 缺失 |  |
| POST | `/view/{}/excel/mockputtopost` | 🔴 缺失 |  |
| PUT | `/view/{}/execute` | 🔴 缺失 |  |
| POST | `/view/{}/execute/mockputtopost` | 🔴 缺失 |  |
| POST | `/view/{}/execute/v2/page/{}/size/{}` | 🔴 缺失 |  |

### x_processplatform_assemble_bam（缺 42 / 45）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/period/list/completed/task/applicationstubs` | 🔴 缺失 |  |
| GET | `/period/list/completed/task/unitstubs` | 🔴 缺失 |  |
| GET | `/period/list/completed/work/applicationstubs` | 🔴 缺失 |  |
| GET | `/period/list/completed/work/unitstubs` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/task/application/{}/process/{}/activity/{}/by/unit` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/task/application/{}/process/{}/activity/{}/unit/{}/person/{}` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/task/application/{}/process/{}/unit/{}/person/{}/by/activity` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/task/application/{}/unit/{}/person/{}/by/process` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/task/unit/{}/person/{}/by/application` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/work/application/{}/process/{}/by/unit` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/work/application/{}/process/{}/unit/{}/person/{}` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/work/application/{}/unit/{}/person/{}/by/process` | 🔴 缺失 |  |
| GET | `/period/list/count/completed/work/unit/{}/person/{}/by/application` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/task/application/{}/process/{}/activity/{}/by/unit` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/task/application/{}/process/{}/activity/{}/unit/{}/person/{}` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/task/application/{}/process/{}/unit/{}/person/{}/by/activity` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/task/application/{}/unit/{}/person/{}/by/process` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/task/unit/{}/person/{}/by/application` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/work/application/{}/process/{}/by/unit` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/work/application/{}/process/{}/unit/{}/person/{}` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/work/application/{}/unit/{}/person/{}/by/process` | 🔴 缺失 |  |
| GET | `/period/list/count/expired/work/unit/{}/person/{}/by/application` | 🔴 缺失 |  |
| GET | `/period/list/count/start/task/application/{}/process/{}/activity/{}/by/unit` | 🔴 缺失 |  |
| GET | `/period/list/count/start/task/application/{}/process/{}/activity/{}/unit/{}/person/{}` | 🔴 缺失 |  |
| GET | `/period/list/count/start/task/application/{}/process/{}/unit/{}/person/{}/by/activity` | 🔴 缺失 |  |
| GET | `/period/list/count/start/task/application/{}/unit/{}/person/{}/by/process` | 🔴 缺失 |  |
| GET | `/period/list/count/start/task/unit/{}/person/{}/by/application` | 🔴 缺失 |  |
| GET | `/period/list/count/start/work/application/{}/process/{}/by/unit` | 🔴 缺失 |  |
| GET | `/period/list/count/start/work/application/{}/process/{}/unit/{}/person/{}` | 🔴 缺失 |  |
| GET | `/period/list/count/start/work/application/{}/unit/{}/person/{}/by/process` | 🔴 缺失 |  |
| GET | `/period/list/count/start/work/unit/{}/person/{}/by/application` | 🔴 缺失 |  |
| GET | `/period/list/expired/task/applicationstubs` | 🔴 缺失 |  |
| GET | `/period/list/expired/task/unitstubs` | 🔴 缺失 |  |
| GET | `/period/list/expired/work/applicationstubs` | 🔴 缺失 |  |
| GET | `/period/list/expired/work/unitstubs` | 🔴 缺失 |  |
| GET | `/period/list/start/task/applicationstubs` | 🔴 缺失 |  |
| GET | `/period/list/start/task/unitstubs` | 🔴 缺失 |  |
| GET | `/period/list/start/work/applicationstubs` | 🔴 缺失 |  |
| GET | `/period/list/start/work/unitstubs` | 🔴 缺失 |  |
| GET | `/state/applicationtstubs/trigger` | 🔴 缺失 |  |
| GET | `/state/category` | 🔴 缺失 |  |
| GET | `/state/category/trigger` | 🔴 缺失 |  |

### x_portal_assemble_surface（缺 27 / 38）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/dict/{}/portal/{}` | 🔴 缺失 |  |
| GET | `/dict/{}/portal/{}/data` | 🔴 缺失 |  |
| DELETE | `/dict/{}/portal/{}/{}/data` | 🔴 缺失 |  |
| GET | `/dict/{}/portal/{}/{}/data` | 🔴 缺失 |  |
| POST | `/dict/{}/portal/{}/{}/data` | 🔴 缺失 |  |
| PUT | `/dict/{}/portal/{}/{}/data` | 🔴 缺失 |  |
| GET | `/dict/{}/portal/{}/{}/data/mockdeletetoget` | 🔴 缺失 |  |
| POST | `/dict/{}/portal/{}/{}/data/mockputtopost` | 🔴 缺失 |  |
| GET | `/file/{}/portal/{}/content` | 🔴 缺失 |  |
| GET | `/file/{}/portal/{}/download` | 🔴 缺失 |  |
| GET | `/page/list/portal/{}` | 🔴 缺失 |  |
| GET | `/page/v2/{}` | 🔴 缺失 |  |
| GET | `/page/v2/{}/mobile` | 🔴 缺失 |  |
| GET | `/page/v2/{}/portal/{}` | 🔴 缺失 |  |
| GET | `/page/v2/{}/portal/{}/mobile` | 🔴 缺失 |  |
| GET | `/page/{}/mobile` | 🔴 缺失 |  |
| GET | `/page/{}/portal/{}` | 🔴 缺失 |  |
| GET | `/page/{}/portal/{}/mobile` | 🔴 缺失 |  |
| GET | `/portal/list/mobile` | 🔴 缺失 |  |
| GET | `/portal/{}/corner/mark` | 🔴 缺失 |  |
| GET | `/portal/{}/icon` | 🔴 缺失 |  |
| GET | `/portal/{}/icon/base64` | 🔴 缺失 |  |
| POST | `/script/portal/{}/name/{}` | 🔴 缺失 |  |
| GET | `/script/portal/{}/name/{}/imported` | 🔴 缺失 |  |
| GET | `/widget/{}/mobile` | 🔴 缺失 |  |
| GET | `/widget/{}/portal/{}` | 🔴 缺失 |  |
| GET | `/widget/{}/portal/{}/mobile` | 🔴 缺失 |  |

### x_portal_assemble_designer（缺 23 / 64）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/dict/list/paging/{}/size/{}` | 🔴 缺失 |  |
| PUT | `/dict/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：DELETE/GET） |
| GET | `/output/{}/select/file` | 🔴 缺失 |  |
| POST | `/page` | 🔴 缺失 |  |
| GET | `/page/list/portal/{}` | 🔴 缺失 |  |
| DELETE | `/page/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/page/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| GET | `/pageversion/list/page/{}` | 🔴 缺失 |  |
| POST | `/portal` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/portal/list/summary` | 🔴 缺失 |  |
| POST | `/portal/list/summary/v2` | 🔴 缺失 |  |
| DELETE | `/portal/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/portal/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| PUT | `/portal/{}/icon` | 🔴 缺失 |  |
| POST | `/portal/{}/permission` | 🔴 缺失 |  |
| POST | `/templatepage` | 🔴 缺失 |  |
| GET | `/templatepage/list` | 🔴 缺失 |  |
| GET | `/templatepage/list/category` | 🔴 缺失 |  |
| PUT | `/templatepage/list/category` | 🔴 缺失 |  |
| DELETE | `/templatepage/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| POST | `/widget` | 🔴 缺失 |  |
| DELETE | `/widget/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/widget/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |

### x_calendar_assemble_control（缺 22 / 31）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/calendar` | 🔴 缺失 |  |
| GET | `/calendar/follow/{}` | 🔴 缺失 |  |
| GET | `/calendar/follow/{}/cancel` | 🔴 缺失 |  |
| GET | `/calendar/ismanager/calendar/{}` | 🔴 缺失 |  |
| PUT | `/calendar/list/filter` | 🔴 缺失 |  |
| GET | `/calendar/manager/list/with/person/{}` | 🔴 缺失 |  |
| DELETE | `/calendar/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| POST | `/event` | 🔴 缺失 |  |
| DELETE | `/event/after/{}` | 🔴 缺失 |  |
| DELETE | `/event/all/{}` | 🔴 缺失 |  |
| PUT | `/event/list/filter` | 🔴 缺失 |  |
| PUT | `/event/list/filter/sample` | 🔴 缺失 |  |
| POST | `/event/list/filter/sample/manager` | 🔴 缺失 |  |
| POST | `/event/manage` | 🔴 缺失 |  |
| GET | `/event/rfc/{}` | 🔴 缺失 |  |
| DELETE | `/event/single/{}` | 🔴 缺失 |  |
| PUT | `/event/update/after/{}` | 🔴 缺失 |  |
| PUT | `/event/update/all/{}` | 🔴 缺失 |  |
| PUT | `/event/update/single/{}` | 🔴 缺失 |  |
| POST | `/setting` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET/PUT） |
| GET | `/setting/code/{}` | 🔴 缺失 |  |
| GET | `/test/1` | 🔴 缺失 |  |

### x_general_assemble_control（缺 21 / 46）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/ecnet/check` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| POST | `/excel/excelName/{}` | 🔴 缺失 |  |
| POST | `/excel/excelName/{}/sheetList` | 🔴 缺失 |  |
| GET | `/excel/result/{}` | 🔴 缺失 |  |
| POST | `/generalfile` | 🔴 缺失 |  |
| GET | `/invoice/delete/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：DELETE/POST） |
| POST | `/invoice/list/paging/{}/size/{}` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| POST | `/office` | 🔴 缺失 |  |
| GET | `/office/html/to/word/result/{}` | 🔴 缺失 |  |
| POST | `/qrcode` | 🔴 缺失 |  |
| GET | `/qrcode/width/{}/height/{}/text/{}` | 🔵 动词差 | 路径已存在，缺 GET 变体（现有：POST） |
| GET | `/securityclearance` | 🔴 缺失 |  |
| GET | `/worktime/betweenholidaycount/start/{}/end/{}` | 🔴 缺失 |  |
| GET | `/worktime/betweenminutes/start/{}/end/{}` | 🔴 缺失 |  |
| GET | `/worktime/forwarddays/start/{}/days/{}` | 🔴 缺失 |  |
| GET | `/worktime/forwardminutes/start/{}/minutes/{}` | 🔴 缺失 |  |
| GET | `/worktime/indefinedholiday/{}` | 🔴 缺失 |  |
| GET | `/worktime/indefinedworkday/{}` | 🔴 缺失 |  |
| GET | `/worktime/isholiday/{}` | 🔴 缺失 |  |
| GET | `/worktime/isworktime/{}` | 🔴 缺失 |  |
| GET | `/worktime/minutesofworkday` | 🔴 缺失 |  |

### x_mind_assemble_control（缺 18 / 23）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| PUT | `/folder/move/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：POST） |
| DELETE | `/folder/{}/force` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：POST） |
| PUT | `/mind/filter/list/{}/next/{}` | 🔴 缺失 |  |
| PUT | `/mind/filter/recived/{}/next/{}` | 🔴 缺失 |  |
| PUT | `/mind/filter/recycle/{}/next/{}` | 🔴 缺失 |  |
| PUT | `/mind/filter/shared/{}/next/{}` | 🔴 缺失 |  |
| GET | `/mind/list/{}/shareRecords` | 🔴 缺失 |  |
| DELETE | `/mind/recycle/{}` | 🔴 缺失 |  |
| GET | `/mind/restore/{}` | 🔴 缺失 |  |
| POST | `/mind/save` | 🔴 缺失 |  |
| PUT | `/mind/share/{}` | 🔴 缺失 |  |
| PUT | `/mind/share/{}/cancel` | 🔴 缺失 |  |
| GET | `/mind/version/{}` | 🔴 缺失 |  |
| GET | `/mind/view/{}` | 🔴 缺失 |  |
| DELETE | `/mind/{}/destorymind` | 🔴 缺失 |  |
| DELETE | `/mind/{}/destoryrecycle` | 🔴 缺失 |  |
| GET | `/mind/{}/icon` | 🔴 缺失 |  |
| POST | `/mind/{}/icon/size/{}` | 🔴 缺失 |  |

### x_organization_assemble_authentication（缺 10 / 53）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/oauth/auth` | 🔴 缺失 |  |
| POST | `/oauth/generate/code` | 🔴 缺失 |  |
| GET | `/oauth/info` | 🔴 缺失 |  |
| POST | `/oauth/info` | 🔴 缺失 |  |
| GET | `/oauth/info/jira` | 🔴 缺失 |  |
| POST | `/oauth/info/jira` | 🔴 缺失 |  |
| GET | `/oauth/token` | 🔴 缺失 |  |
| POST | `/oauth/token` | 🔴 缺失 |  |
| POST | `/oauth/token/jira` | 🔴 缺失 |  |
| POST | `/qiyeweixin/info/sign` | 🔴 缺失 |  |

### x_processplatform_assemble_surface（缺 8 / 659）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}` | 🔴 缺失 |  |
| GET | `/attachment/download/{}/work/{}/stream/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/download/{}/work/{}/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/download/{}/workcompleted/{}/stream/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/download/{}/workcompleted/{}/{}.{}` | 🟠 平台限制 | axum 不支持单段多参数（如 `{}.{}` 段），留档不实现 |
| GET | `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` | 🔴 缺失 |  |
| GET | `/task/list/date/{}/hour/{}/exclude/draft/{}/manage` | 🔴 缺失 |  |
| GET | `/task/list/person/{}/exclude/draft/{}/manage` | 🔴 缺失 |  |

### x_hotpic_assemble_control（缺 7 / 12）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| DELETE | `/cipher/hotpic/bbs/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| DELETE | `/cipher/hotpic/cms/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/cipher/hotpic/filter/list/page/{}/count/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| POST | `/user/hotpic` | 🔴 缺失 |  |
| PUT | `/user/hotpic/filter/list/page/{}/count/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |
| DELETE | `/user/hotpic/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| DELETE | `/user/hotpic/{}/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |

### x_jpush_assemble_control（缺 7 / 9）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/device/admin/unbind/all/person` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| GET | `/device/check/{}/{}/{}` | 🔴 缺失 |  |
| GET | `/device/list/{}` | 🔴 缺失 |  |
| GET | `/device/unbind/new/{}/{}/{}` | 🔴 缺失 |  |
| DELETE | `/device/unbind/{}/{}` | 🔴 缺失 |  |
| POST | `/message/send` | 🔴 缺失 |  |
| POST | `/message/test/send` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |

### x_component_assemble_control（缺 4 / 7）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/component` | 🔵 动词差 | 路径已存在，缺 POST 变体（现有：GET） |
| DELETE | `/component/delete/all` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：POST） |
| DELETE | `/component/{}` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |
| PUT | `/component/{}` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：GET） |

### x_message_assemble_communicate（缺 2 / 64）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| PUT | `/im/conversation` | 🔵 动词差 | 路径已存在，缺 PUT 变体（现有：POST） |
| DELETE | `/im/conversation/{}/group` | 🔵 动词差 | 路径已存在，缺 DELETE 变体（现有：GET） |

### x_base_core_project（缺 1 / 8）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/echo` | 🔴 缺失 |  |

### x_bbs_assemble_control（缺 1 / 106）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| GET | `/user/subject/acceptreply/{}/{}` | 🔴 缺失 |  |

### x_organization_assemble_personal（缺 1 / 76）

| 方法 | 归一化路径 | 判定 | 说明 |
|------|-----------|------|------|
| POST | `/custom/{}/mockputtopost` | 🔴 缺失 |  |

## 四、排除留档后剩余缺口 Top 清单（本轮只列清单不实现）

| # | 模块 | 缺口数 | 构成（缺失/动词差/形变） | 代表端点 | 相关 crate | 难度 | 建议 |
|---|------|-------:|------------------|----------|-----------|------|------|
| 1 | `x_program_center` | 119 | 79/40/0 | POST `/agent`<br>GET `/agent/{}` | `program_center`, `program_center_core_entity` | 高（大族缺失：新 handler + 路由成批补齐） | 按域分批补 handler + 注册；低频监控/管理域可挂起降级 |
| 2 | `x_query_assemble_designer` | 52 | 40/12/0 | GET `/importmodel/list/query/{}`<br>DELETE `/importmodel/{}` | `query_assemble_designer` | 高（大族缺失：新 handler + 路由成批补齐） | 按域分批补 handler + 注册；低频监控/管理域可挂起降级 |
| 3 | `x_processplatform_assemble_designer` | 51 | 42/9/0 | PUT `/application/{}`<br>PUT `/application/{}/icon` | `processplatform_assemble_designer` | 高（大族缺失：新 handler + 路由成批补齐） | 按域分批补 handler + 注册；低频监控/管理域可挂起降级 |
| 4 | `x_query_assemble_surface` | 50 | 47/3/0 | GET `/importmodel/flag/{}/query/{}`<br>GET `/importmodel/list/query/{}` | `query_assemble_surface` | 高（大族缺失：新 handler + 路由成批补齐） | 按域分批补 handler + 注册；低频监控/管理域可挂起降级 |
| 5 | `x_processplatform_assemble_bam` | 42 | 42/0/0 | GET `/period/list/completed/task/applicationstubs`<br>GET `/period/list/completed/task/unitstubs` | `processplatform_assemble_bam` | 高（大族缺失：新 handler + 路由成批补齐） | 按域分批补 handler + 注册；低频监控/管理域可挂起降级 |
| 6 | `x_portal_assemble_surface` | 27 | 27/0/0 | GET `/dict/{}/portal/{}`<br>GET `/dict/{}/portal/{}/data` | `portal_assemble_surface` | 中高（同域多族，需按域分批补 handler） | 同域小族成批补齐：仿同模块既有 u2 handler 模式 |
| 7 | `x_portal_assemble_designer` | 23 | 14/9/0 | POST `/dict/list/paging/{}/size/{}`<br>PUT `/dict/{}` | `portal_assemble_designer` | 中高（同域多族，需按域分批补 handler） | 同域小族成批补齐：仿同模块既有 u2 handler 模式 |
| 8 | `x_calendar_assemble_control` | 22 | 20/2/0 | POST `/calendar`<br>GET `/calendar/follow/{}` | `calendar_assemble_control` | 中高（同域多族，需按域分批补 handler） | 同域小族成批补齐：仿同模块既有 u2 handler 模式 |
| 9 | `x_general_assemble_control` | 21 | 17/4/0 | POST `/ecnet/check`<br>POST `/excel/excelName/{}` | `general_assemble_control` | 中高（同域多族，需按域分批补 handler） | 同域小族成批补齐：仿同模块既有 u2 handler 模式 |
| 10 | `x_mind_assemble_control` | 18 | 16/2/0 | PUT `/folder/move/{}`<br>DELETE `/folder/{}/force` | `mind_assemble_control` | 中高（同域多族，需按域分批补 handler） | 同域小族成批补齐：仿同模块既有 u2 handler 模式 |
| 11 | `x_organization_assemble_authentication` | 10 | 10/0/0 | GET `/oauth/auth`<br>POST `/oauth/generate/code` | `organization_assemble_authentication` | 中高（同域多族，需按域分批补 handler） | 同域小族成批补齐：仿同模块既有 u2 handler 模式 |
| 12 | `x_hotpic_assemble_control` | 7 | 1/6/0 | DELETE `/cipher/hotpic/bbs/{}`<br>DELETE `/cipher/hotpic/cms/{}` | `hotpic_assemble_control` | 中低（零星端点，逐条补齐） | 零星补齐：逐条仿既有 handler + 注册 |
| 13 | `x_jpush_assemble_control` | 7 | 5/2/0 | POST `/device/admin/unbind/all/person`<br>GET `/device/check/{}/{}/{}` | `jpush_assemble_control` | 中（小族缺失/形变，仿既有 u2 handler 模式补齐） | 零星补齐：逐条仿既有 handler + 注册 |
| 14 | `x_component_assemble_control` | 4 | 0/4/0 | POST `/component`<br>DELETE `/component/delete/all` | `component_assemble_control` | 低（纯方法变体，批量注册即可） | 批量补方法变体注册（沿用 gen_u2_putdelete 模式），不动 handler |
| 15 | `x_processplatform_assemble_surface` | 4 | 4/0/0 | GET `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}`<br>GET `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}` | `processplatform_assemble_surface` | 中（小族缺失/形变，仿既有 u2 handler 模式补齐） | 零星补齐：逐条仿既有 handler + 注册 |
| 16 | `x_message_assemble_communicate` | 2 | 0/2/0 | PUT `/im/conversation`<br>DELETE `/im/conversation/{}/group` | `message_assemble_communicate` | 低（纯方法变体，批量注册即可） | 批量补方法变体注册（沿用 gen_u2_putdelete 模式），不动 handler |
| 17 | `x_base_core_project` | 1 | 1/0/0 | GET `/echo` | `base` | 中低（零星端点，逐条补齐） | 零星补齐：逐条仿既有 handler + 注册 |
| 18 | `x_bbs_assemble_control` | 1 | 1/0/0 | GET `/user/subject/acceptreply/{}/{}` | `bbs_assemble_control` | 中低（零星端点，逐条补齐） | 零星补齐：逐条仿既有 handler + 注册 |
| 19 | `x_organization_assemble_personal` | 1 | 1/0/0 | POST `/custom/{}/mockputtopost` | `organization_assemble_personal` | 中低（零星端点，逐条补齐） | 零星补齐：逐条仿既有 handler + 注册 |

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
5. **BAM（x_processplatform_assemble_bam）**：台账已注明的 P3 真实大缺口，实测缺 42 条监控类低频端点，维持挂起建议。
6. **动词差批量项**：全仓共 95 条仅需补方法变体（路径已存在），是性价比最高的收敛手段。
