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
| 12 | TEST（契约守卫） | TEST | **新增 HTTP 方法契约守卫**（commit `2e28a827`）：既有端点守卫只验路径不看方法（轮10 的 405 错配因此漏网）；route-methods.test.ts 扫前端 (api\|mapi).{method}('/api/…') 对撞 backend-route-methods.fixture（3981 路径→方法集，gen_mcp_tools 重生成不手写），方法不在集内即失败；轮10 修后现状全绿，此后方法漂移 CI 即拦。门禁：route-methods 1/1、vitest 976/976 |
| 13 | FIX（后端工具链） | FIX | **openapi 生成器同款多方法链 bug + 追平 611 条漂移**（commit `5504f1393`）：`gen_openapi_paths.py` 与轮10 的 gen_mcp_tools 同一 bug（三方法 CRUD 丢 PUT/DELETE），同法修正；重生成暴露产物严重滞后——已提交 openapi 仅 4045 条 /api path，真实 4656 条（含轮2/3 新增路由），文档漂移 611 条本轮追平。校验：openapi cargo check 0、安全方案守卫 rc=0、4666 path items 与 MCP 表同步 |
| 14 | FIX（后端 i18n/健壮性） | FIX | **中文文件名下载 panic 根治**（commit `ec1c04466`）：file_assemble_control 6 处下载 handler 把 DB 原始文件名塞进 `filename="{name}"`，中文/特殊字符使 HeaderValue 构造失败→`.body().unwrap()` panic（O2OA 文档普遍中文命名=中文附件下载即崩）；新增 attachment_disposition（ASCII 回退+RFC5987 `filename*=UTF-8''`）统一改用+2 单测。门禁：file 89/89+clippy 0+fmt 0 |
| 15 | FIX（后端，跨 crate 收口） | FIX | **同款 panic 面全消除 + helper 上提 shared**（commit `4986b32b4`）：排查其余下载 crate——message（sanitize_filename 只去路径不去 CJK）、processplatform（25980 builder+unwrap 仅去引号，CJK 必 panic）同款；attachment_disposition 上提 shared::response 单一事实源，file 删本地副本，message+process 三处统一改用，shared 补 2 单测。门禁：shared 87/file 102/message 601/process 143 全绿、clippy 0、fmt 0 |
| 16 | FEAT+PERF（desktop 功能补全+按需打包） | FEAT | **查询统计「图表类型」死配置落地 + echarts 按需模块化**（commit `f9f16067e`）：QueryManagerDeep 统计分析让用户选柱/饼/折线/表格，但结果恒渲染键值列表、chartType 形同虚设；仓内完整 EChartsView.vue 却零引用（孤儿组件，全仓 grep 0 importer）。本轮接线：非表格时用 EChartsView 渲染 statResult（{标签:数值}→name/value 行集），表格模式回退键值列表，statConfig.chartType 收紧联合类型对齐组件 props。同步把 EChartsView 全量 barrel `import * as echarts from 'echarts'` 改为按需模块化（echarts/core + Bar/Line/PieChart + Grid/Tooltip + CanvasRenderer + echarts.use），新接入的 echarts 以懒加载异步块落地（534KB/gz 182KB，仅统计视图渲图时拉取，不入口打包）。踩坑：biome 2.5.11 不识别本文件 Vue 模板组件用法（scratch 实证与组件名无关），按仓内既有约定加 `biome-ignore noUnusedImports` 注释。门禁：QueryManagerDeep 11/11（新增 stat-chart 4 例）、biome 0、tsc 0、desktop build ✓ |
| 17 | FEAT（desktop 功能补全） | FEAT | **表格设计 tableConfig 四项死配置全部落地**（commit `fc5c7f613`）：QueryManagerDeep 表格设计模式 4 个控件（主题/可排序/可筛选/行选择）此前全为 v-model 收集却零消费的死配置（同轮16 chartType 病灶）。全部接线：主题→default/striped/bordered 表格 CSS 类；可排序→点表头切列排序（数值差/中文 localeCompare）带 ▲▼ 指示；可筛选→快速全列过滤输入；行选择→按行内容 JSON 签名稳定选中并计数（排序后不错位）。以 `displayRows` 计算属性叠加在 viewRows（视图投影）之上，**导出仍走 viewRows/viewHeaders 保持所见即所得口径不被交互层污染**（轮5 契约保留）。踩坑：改表格渲染源 viewRows→displayRows 触发轮5 view-config.test 网格断言回归，同步更新；biome 把长 filter 链重排多行→测试 needle 改用抗格式化片段。门禁：QueryManagerDeep 17/17（新增 table-config 6 例）、biome 0、tsc 0、desktop build ✓ |
| 18 | FIX（后端数据完整性+性能） | FIX | **CMS 权限集合替换写入原子化**（commit `32ff2b1f3`）：`u2_save`/`u3_save_scope_permissions` 两个权限替换 handler 此前对「DELETE 旧权限 + 循环 INSERT 逐人重授」用裸 client 逐条执行，**非原子**——循环中途任一 INSERT 失败即留下半删半授的越权/漏权中间态（数据完整性缺陷），且 N 次独立往返。改用 `client.transaction()` 包裹（与 ai/portal/processplatform designer 既有事务惯例一致，AGENTS 规则10 遵从），要么全成要么整体回滚，顺带减少往返。验证：cargo check + clippy `-D warnings` 0 + **cms 331/331（含多例 real_db 实跑本地 PG:5432）** + fmt --check 0 |
| 19 | FIX（后端数据完整性） | FIX | **CMS 批量文档关联创建/更新写入原子化**（commit `dc3a76199`）：`correlation_create_u3`/`correlation_update_u3` 对每个 relatedDocId 逐对「DELETE 旧关联 + INSERT 新关联」裸 client 循环执行，同轮18 非原子——批量中途失败留下部分关联的半成品集合。改 `client.transaction()` 整体包裹，整批成功或整体回滚。验证：clippy `-D warnings` 0 + **cms 331/331（含 `u3_correlation_update_upsert_real_db`+`test_correlation_update_u3` 实跑本地 PG）** + fmt --check 0 |
| 20 | FIX（后端数据完整性）+ 中型门禁 | FIX | **CMS 批量文档加密写入原子化 + 轮20 中型门禁**（commit `e2f485757`）：`u3_cipher_upsert` 逐 doc UPSERT x_cms_document_cipher 裸 client 循环，非原子——中途失败留下部分文档已加密、其余明文的半加密集合；改 `client.transaction()` 整体包裹，**收口 cms 全部 4 处非事务批量写**。中型门禁全绿：**desktop vitest 986/986**（较 976 +10=轮16/17 新测）、**mobile vitest 102/102**、**desktop build ✓**、**mobile build(H5) ✓**、cms 331/331（含 cipher real_db）、clippy 0、fmt 0 |
| 21 | FIX（后端数据完整性，跨 crate） | FIX | **考勤组明细重算写入原子化**（commit `88b95002f`）：`v2_group_rebuild_detail_group_date` 对考勤组全体参与人逐人「DELETE 旧明细 + INSERT 新明细」裸 client 循环，非原子——中途失败留部分人已重算、其余旧明细残留的不一致考勤集合。改 `client.transaction()` 整体包裹（cms 事务化跨 crate 续扫，attendance）。验证：clippy `-D warnings` 0 + fmt 0 + 默认 41/41 + ignored real_db 11/13；**诚实记录**：余 2 失败为 rule/list·rule/toggle 既有陈旧 `#[ignore]` 测试，其「无 DB→500」假设被本地 PG:5432 真连破坏，与本轮 group_rebuild 无关、不入常规门禁 |
| 24 | FIX（desktop 错误处理，批量收口） | FIX | **desktop 真空 catch 全清零**（commit `3c6bc6cbf`）：5 处批量导入/批删同款病灶（FormApp/QueryDesigner/QueryManagerDeep/QueryStatementDesigner 的 doImport + QueryStatementDesigner 的 executeBulkDelete）——循环内逐项请求有 try/catch 但无论成败恒报假成功。统一改逐项 await + 成败计数（失败报「导入完成：成功 X / 失败 Y」/批删失败 `toast.error`，QueryStatementDesigner 补 toast import）；QueryManager `runQ` 空 catch 吞执行错误 → toast.error+清空结果；OrgViewer 搜索 best-effort catch 补意图注释；ProcessDesigner `onMapDrop` 未消费 `data` 变量删除（目标侧功能未实现，注释声明）。**desktop 全目录真空 catch{} 10→0**。门禁：biome 7 文件 0、tsc 0、**vitest 989/989**（+3=轮23）、desktop build ✓ |
| 25 | FEAT（desktop 可用性） | FEAT | **导航后 document.title 同步路由 meta.title**（commit `7a63c4ef7`）：86 条路由全部声明 meta.title 却无任何 document.title 同步——每个页面浏览器标签恒显示「OA4Rust」，多标签页/历史记录无法辨识所在页（meta 设计意图落空）。补 `router.afterEach` 同步（有 title → `{title} · OA4Rust`），main.test 补 1 例守卫。**本轮排除项记录**：cms 139 处 `list_from_table_filtered_legacy` 全量 SELECT * 无 LIMIT 是 O2OA 行为对齐契约（size=0 全量，parity 依赖），加防御 LIMIT 会破契约——不改，记档。门禁：main 37/37、**全量 vitest 990/990**、biome 0、tsc 0 |
| 26 | FEAT（mobile 可用性） | FEAT | **16 页面 navigationBarTitleText 全补齐**（commit `ef0682ce7`）：mobile 全部 16 页此前无页面级标题（仅轮8 的 bbs 论坛页有）——系统导航栏/H5 document.title 恒显示 globalStyle 兜底的「OA4Rust」，微信小程序原生导航栏无标题。按业务域逐页补齐（工作台/消息/即时通讯/审批/发起流程/考勤打卡/文档/通讯录/会议/日历/浏览/搜索/回收站/我的/登录），与 tabBar 文案对齐。与轮25 desktop title 同步同病灶跨端修复。门禁：biome 0、mobile vitest 102/102、H5 build ✓ |
| 27 | FIX（mobile 跨端稳定性） | FIX | **login 页 onShow 自 reLaunch 循环根治**（commit `8735726c6`）：login 页 onShow 调 `ensureAuthenticated()`，未登录时守卫会 `uni.reLaunch` 到**登录页自身**——H5 上 router replace 同路由是 no-op 掩盖了问题，但**微信小程序 reLaunch 当前页会销毁重建页面并重触发 onShow → 无限重载循环**（真跨端缺陷，GUI 验证只在 H5 做过故未暴露）。改 login 页直接 `session.init()`+`isAuthenticated` 判断后 navigateHome，不经守卫重定向分支；auth-guard 语义不变供受保护页使用。新增 login-guard.test.ts 2 例钉死「login 页不得调 ensureAuthenticated」约束。门禁：mobile vitest **104/104**、biome 0、H5 build ✓ |
| 22 | FIX（后端数据完整性，跨 crate） | FIX | **人员级联删除 + 身份重排写入原子化**（commit `cbf08c27c`）：organization 两处非事务多写——`person_reserve_delete`（级联删身份/组成员/属性 3 表 + 软删人员共 4 写，中途失败留人员仍"存在"却丢失全部身份/成员/属性的孤儿损坏态，**严重完整性缺陷**）；`identity_flag_order_before_followFlag`（整表重排 order_number 逐行 UPDATE 循环，中途失败留半排序错乱/重号）。均改 `client.transaction()` 包裹（读在事务前、写在事务内）。验证：clippy `-D warnings` 0 + fmt 0 + **organization 147/147（0 ignored，含 real_db 实跑本地 PG）** |
| 23 | FIX（desktop 错误处理/健壮性） | FIX | **配置设计器错误吞没修复**（commit `074bc8a02`）：ConfigDesignerApp 两处 `catch{}` 吞错——`deleteItem` 删除失败被静默吞掉却仍从本地列表移除（UI 与后端不一致，用户误以为删成功）→改 catch 时 `toast.error` + return 不移除；`importConfigs` 的 `api.post` **未 await**（fire-and-forget，内层 catch 根本捕不到异步失败）且无论成败恒报"成功导入 N 项"误导用户 →改 async + 逐项 await + 成败计数（"导入完成：成功 X / 失败 Y"，匹配仓内既有导入反馈约定）。全文件真空 catch 清零。门禁：ConfigDesignerApp 3/3（新增 error-handling 3 例）、biome 0、tsc 0、desktop build ✓ |

> **第五批小结（轮 16–20，2026-09-29，中型门禁已过）**：desktop 死配置/孤儿组件根治 3 轮（QueryManagerDeep 统计图表落地+echarts 按需模块化、表格设计 tableConfig 四项全接线）+ 后端数据完整性 3 轮（CMS 权限替换/文档关联/文档加密三类非事务批量写全部事务化收口）。**关键探明：arity-trap 深水区已确认闭环**（`_arity_traps_baseline.txt` 与 `_arity_unreachable_baseline.txt` 均 0 字节=探针零回归基线，2026-09-27 审计批已清零）；静态可扫缺陷面（SQL 注入/format!拼 SQL/下载 panic/unwrap/占位 handler/TODO）逐一核过均已加固——续轮价值转向**真实功能补全（死配置/孤儿组件接线）与后端数据完整性（非事务批量写）**。踩坑：biome 2.5.11 不识别 .vue 模板组件用法（仓内约定 `biome-ignore noUnusedImports`）；本地 PG:5432 可用使 cms real_db 测试实跑（is_db_available 为真）。**下批候选**：① 跨 crate 非事务批量写续扫（attendance/organization/processplatform）；② 更多 desktop/mobile 死配置或未完成流程；③ 性能面（读路径 N+1/慢查询）。

> **第三批小结（轮 11–13）**：desktop 死钩子清理 + 方法契约守卫 institutionalize + 两个路由生成器的同款多方法解析 bug 根治（MCP+openapi），连带追平 openapi 611 条文档漂移。**注：本分支与并行 augmentor 会话交错提交（共享 augmentor-opt100），每轮仅暂存本人文件**。下批候选：mobile 业务域续扩、前端未消费 useQuery 复扫、后端 arity-trap 探针（需 live oneshot）。

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
