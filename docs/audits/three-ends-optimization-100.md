<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# oa4rust 三端全面优化 100 轮账本（2026-09-28 启动）

> 目标：oa4rust 三端（Rust 后端 + desktop + mobile）的**功能扩展**与**性能质量提升**全面优化 100 轮。
> 每轮 = 真实扫描 → 处置 → 验证 → 记账。结果三类：**FEAT**（功能扩展）、**FIX**（发现并修复）、
> **PERF/IMPROVE**（性能/不改行为的质量提升）。
> 节奏：每轮轻量门禁（biome lint + tsc + 受影响测试）；每 10 轮中型门禁（全量 vitest + 双端 build）；
> 第 50/100 轮完整门禁（另加 cargo fmt/clippy --all-targets -D warnings/parity/reconcile）。

## 前置资产（如实记档，不计入本轮 100 轮）

- **审计优化循环 100 轮**（`docs/audits/optimization-loop-100.md`，2026-09-27 收官）：FIX 14 /
  IMPROVE 3 / CLEAN 其余，新增真实现 html→image，501 桩全仓清零，终态门禁 8/8 绿。
- **2026-09-27 审计批次** 7 提交：契约物漂移清零、63 表 xid 索引、42 缺列 + 50 个 500 修复、zip×3 真实现。
- **2026-09-28 性能增量** 7 提交：`[profile.release]` thin-LTO/strip、发布档/延迟 A/B 基准脚本与实测、
  CPU 微基准、clippy 门禁（--all-targets -D warnings）落地、前端 lint 收紧 --error-on-warnings。
- 真实消费收官（`docs/audits/three-ends-2026-09-20/REAL_CONSUMPTION_CLOSURE.md`）：216 条未消费路由
  三类裁决（116 仅运行时可验证 / 73 仅真实用户写动作 / 27 结构不可计入），名义接桩已被用户否决并 revert。

## 状态：进行中（轮 1/100）

| 轮 | 维度 | 结果 | 处置 |
|---|---|---|---|
| 1 | FEAT+门禁修复（desktop） | FEAT | **CSV 导入从占位转真实现**（commit `bfad15676`）：`importData` 占位 toast → `utils/csv.ts` RFC4180 子集解析（引号/`""` 转义/CRLF·CR·LF/BOM/字面 `\t` 映射）+ 逐行 POST 真实表行插入端点 `/api/query/assemble/designer/table/{flag}/row`（handler `table_tableFlag_row_insert` 整包存 body 为行 data，实读核实）。四道闸：文件在位/Excel 拒收（accept 收窄 `.csv,.tsv,.txt`）/空解析拦截/500 行上限+截断披露；confirmMsg 确认 + 每 100 行进度 + 失败收集汇总。测试 11 例（csv 运行时 7 + 视图源码断言 4）。**顺修收紧门禁两处既有误伤**：vue override 关 `useVueMultiWordComponentNames`（页面级路由组件无模板标签冲突面）+ `deepen_form.cjs` 无用 `String.raw` 改普通模板字面量。门禁：vitest 964/964（953 基线 + 11 新增）、biome 238 文件 0 错、tsc 4 包 0 错、desktop build 2.63s |
| 2 | FEAT（后端+desktop） | FEAT | **系统参数读/写从 501 契约转真实现**（commit `6b5f98707`）：`config_system_config` 实装 x_system_config 未删行全量读（legacy_success 信封）；`u2_config_save_system_config` 实装按 name UPSERT（value 统一落 TEXT）；`u2_capability_unavailable` 失去最后调用点随之删除；新增前端契约形状 `GET /api/config/system`（原 404）+ 保留旧形状 `/api/config/system/config`；Settings.vue 预览横幅移除、onMounted 真装载（数字/布尔按表单类型还原）+ 真 POST 保存；契约测试改写（删 501 前提测试、补真 DB 往返三闸）。**坑：x_system_config 时间列为真 TIMESTAMP，to_char 文本必炸，统一 NOW()**。门禁：cargo 87/87、vitest 967/967、biome/tsc/fmt 全绿 |
| 3 | FEAT（后端，缺口清零） | FEAT | **KNOWN_BACKEND_GAPS 最后两条实装清零**（commit `e0fcb616f`）：`GET /api/users/list`（x_org_person 未删行清单）+ `GET /api/departments/tree`（x_org_unit 部门树，level 自底向上挂 children 防浅拷贝丢深层节点）；desktop-endpoints 守卫缺口声明清空——**至此全仓无已声明未实现端点**；两个真 DB 往返测试。门禁：org crate 147/147 + clippy 0、vitest 967/967、biome/tsc/fmt 全绿 |
| 4 | FIX（desktop，真 404 修复） | FIX | **查询视图 Excel 导出真契约化**（commit `4b7564f6`）：`exportExcel` 原调单段 `/api/queryview/excel/{flag}`（后端仅注册双段 `{view}/{id}`，运行时必 404；desktop-endpoints 守卫槽位宽松匹配漏放）→ 修为与 execute 同形状双段真实路由；消费真契约 `{id, viewFlag, excelData}`（o2 存量 excel_data）：base64 落 .xlsx / 文本落 .csv / 空值如实提示，不再等永不存在的 url 字段；顺修双冒号文案缺陷。**守卫教训：槽位数不同的路径被宽松匹配放过——守卫判定与运行时路由表仍有缝**。门禁：vitest 970/970、biome 240×0、tsc 6 包 Done |
| 5 | FEAT（desktop） | FEAT | **查询管理视图配置从空壳转真应用**（commit `6a9d2d6e0`）：`applyViewConfig` 空壳 → 校验视图列/排序列存在性 + 应用摘要；computed 活应用 viewHeaders（列投影）/viewRows（过滤→排序→分页链），网格实时跟随；过滤表达式纯客户端求值（field=value/!=/~子串/裸子串，不拼 SQL 无注入面）；导出跟随视图投影所见即所得。门禁：vitest 973/973、biome 241×0、tsc 6 包 Done、desktop build 2.31s |
| 6 | PERF（后端 DB） | PERF | **pp_c_* 流程平台表族热路径索引 38 条**（commit `789d0f268`）：证据化扫描（handler SQL 谓词频次 × live pg_indexes 对撞）——xid 批次只覆盖主键查找，待办/已办/已阅计数分页的 xperson/xwork/xjob/xprocess/xapplication 全为顺序扫描；38 条索引逐条对应真实 handler 查询（无投机索引）；EXPLAIN 实证待办计数 Index Only Scan；幂等验证过。门禁：processplatform_assemble_surface 601/601 |
| 7 | PERF（后端 DB） | PERF | **x_* 经典族热路径索引 11 条**（commit `fdcd18b6e`）：同法延展——x_record(work_id,record_type) 复合、x_read/readcompleted(person)、x_correlation(person)、x_work(process/creator)、x_task 待办池部分索引、x_meeting(start_time)/(status,invited)、x_bbs_reply(topic_id)；已覆盖列与低选择性布尔列不重复建。EXPLAIN：待阅计数 Index Only Scan、BBS 回复 Index Scan；x_record 复合在空表规模规划器正确判 Seq Scan（如实记档） |
| 8 | FEAT（mobile） | FEAT | **新增论坛页**（commit `7dae7c203`）：bbsApi.mobileViewAll() 消费 o2 移动契约 mobile/view/all（后端返回最新 20 条主题）；pages/bbs/bbs.vue 主题流（onShow+auth 守卫+下拉刷新+空态）；pages.json 注册第 16 页 + index 工作台入口；mobile-endpoints 手工白名单增补。门禁：vitest 973/973、biome 242×0、tsc 6、mobile H5 build DONE |
| 9 | QUALITY（mobile） | FIX | **mobile 补全局 errorHandler**（commit `6b59100d`）：与 desktop 同口径 `app.config.errorHandler`（[unhandled] console.error，未捕获渲染/Promise 错误不静默吞），desktop 审计轮54 已有、mobile 缺失；测试按视图源码断言惯例（mobile vitest 无 .vue 插件）。门禁：test:mobile 102/102、biome 243×0 |
| 10 | FIX（后端工具链+desktop，含中型门禁） | FIX | **MCP 路由生成器多方法链解析 bug + 两处真方法错配**（commit `17399cfa5`）：`gen_mcp_tools.py` 只登记链首方法——三方法 CRUD `get.put.delete` 丢 PUT/DELETE，MCP 工具面欠表达、契约扫描读到 64 假错配；改按括号配平切 handler 表达式枚举全部方法，重生成 4574→4666（+92 真实端点）。该 bug 掩盖的两处真前端 405 错配（64→2→0）：RecycleApp 永久删除打 `DELETE /api/recycle/{id}`（真路由 `/{id}/delete`）、ProgramCenterApp 孪生打 `POST /api/reset`（真别名 `/reset/mockputtopost`）。**中型门禁全绿**：mcp_server cargo check 0、vitest 973/973、test:mobile 102/102、biome 243×0、tsc 6、desktop build 3.75s、mobile H5 DONE |
| 11 | QUALITY（desktop） | FIX | **清两个空 @input 死钩子**（commit `d4e4a4df`）：DesignCenter `filterDesigners` 空函数+@input 删（检索本由 filteredDesigners computed 响应式驱动）；ConfigDesigner `onConfigChange` 空占位 → `configValid` computed 实时 JSON 合法性徽标（✓/✕/空态中性），假占位换真 UX。门禁：vitest 975/975、biome 244×0、tsc 6、desktop build 2.51s |

> **第二批小结（轮 6–10）**：性能面 2 轮（pp_c_*/x_* 热路径索引共 49 条，EXPLAIN 实证）+ mobile 扩面 2 轮（论坛页/errorHandler）+ 工具链 1 轮（MCP 生成器 bug，连带清 2 真方法错配）。方法错配静态扫描已归零。**下批候选**：① 剩余 arity-trap 深水区（需 live axum oneshot 探针，静态不可靠）；② desktop 死钩子 2 个；③ 更多 mobile 业务域（recycle/search 页化）；④ 前端 bundle 分片。

> **会话小结（2026-09-28 首批 5 轮）**：功能扩展 5 连发全绿闭环。已探明面：mobile 占位文案 0、
> desktop 空函数仅剩 2 个无害 v-model 旁路钩子（ConfigDesignerApp.onConfigChange /
> DesignCenterApp.filterDesigners，v-model 已实时生效，属可留的死钩子）、501 契约仅剩结构性
> 4 处（审计轮83 终态）。**下一批候选方向**：① 后端 real-consumption 剩余 arity-trap 深水区
> （memory: arity-trap-real-consumption-2026-09-25，289 条最难族）；② 三端性能面（bundle 分片、
> 慢查询索引复扫）；③ desktop 死钩子清理或接线；④ mobile 业务域扩面。



## 记账纪律

- 提交信息带「（优化轮 N）」尾注；只暂存本轮文件（工作区有并行目标会话的 laiyipao 在途改动，绝不越界暂存）。
- 本仓工作树 CRLF/LF 混住、且存在工具回显损坏风险：**跨工具核实一律以 od/python 字节级输出为权威**，
  编辑用 `read_bytes`/`write_bytes`；Edit 工具的 old_string 必须先经字节级核实（本轮轮1 曾因一次
  空 Edit 误删 `}` 后换行造成 `}function` 粘连，靠 git diff 抓回）。
- 断言 needle 必须从真实源码字节抄，不凭记忆拼（轮1 的 `.xlsx|xls` needle 撞上 `\.(xlsx|xls)` 的括号假红一次）。
