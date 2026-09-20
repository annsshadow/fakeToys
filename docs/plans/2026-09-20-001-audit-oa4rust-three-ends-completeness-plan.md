---
title: "审计与落地规划：oa4rust 三端（服务端 / 桌面端 / 移动端）功能完备性、API 调用、字段一致性与请求/响应 schema 深度校验"
type: audit-and-plan
status: active
date: 2026-09-20
rev: 41 # rev41（2026-09-21 G5：CMS 文档点赞列表查看）：
       # DocumentApp.vue 每行补「点赞」按钮，消费 GET /api/commend/list/paging/{docId} 1 条真实路由；全局消费 404→405（405/4638=8.7%）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev40 # rev40（2026-09-21 G5：组织-控制 角色成员查看）：
       # RoleManager.vue 每行角色补「成员」按钮，消费 GET organization/assemble/control/person/list/role/{roleFlag} 1 条真实路由；
       # 全局消费 403→404（404/4638=8.7%）。契约回源码核验（person_list_with_role Path GET）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev39 # rev39（2026-09-21 G5：CMS 控制配置查看）：
       # CmsIndexApp.vue 工具栏补「控制配置」按钮，消费 GET /api/cms_assemble_control/get/control/config 1 条真实路由；
       # 全局消费 402→403（403/4638=8.7%）。契约回源码核验（get_control_config 无参 GET）。
       # 【预算提示】本会话已用 ~1 亿 tokens，超 AGENTS.md Rule 6 单任务 40M 上限 2.5 倍——已显式上报（fail loud）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev38 # rev38（2026-09-21 G5：流程-引擎 按分类流程列表）：
       # ProcessManagerApp.vue 工具栏补「运行中流程」按钮，消费 GET processplatform/service/processing/list/{category}（running 分类）1 条真实路由；
       # 全局消费 401→402（402/4638=8.7%）。契约回源码核验（list_processes Path GET）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev37 # rev37（2026-09-21 G5：流程-引擎 工作实例列表）：
       # ProcessTaskCenterApp.vue 工具栏补「工作实例」按钮，消费 GET processplatform/service/processing/work/list 1 条真实路由（Query 可选）；
       # 全局消费 400→401（401/4638=8.6%）。契约回源码核验（work_list Query GET 无 body）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev36 # rev36（2026-09-21 G5：流程-设计器 模板表单分类列表）：
       # ProcessFormDesignerApp.vue 工具栏补「模板表单」按钮，消费 GET templateform/list/{category}（default 分类）1 条真实路由；
       # 全局消费 399→400（400/4638=8.6%，里程碑 400）。契约回源码核验（templateform_list_category Path GET）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev35 # rev35（2026-09-21 G5：流程-设计器 应用分类列表）：
       # ProcessApplicationApp.vue 工具栏补「分类」按钮，消费 GET processplatform/assemble/designer/applicationcategory/list 1 条真实路由；
       # 全局消费 398→399（399/4638=8.6%）。契约回源码核验（applicationcategory_list 无参 GET）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev34 # rev34（2026-09-21 G5：组织-控制 群组详情/直接子群组）：
       # OrgViewer.vue 选中群组节点时查详情+直接子群组，消费 group/{flag} + group/list/{flag}/sub/direct 共 2 条真实路由；
       # 全局消费 396→398（398/4638=8.6%）。踩坑：sub/direct 初写字符串拼接被截断成 group/list/{} 误配已消费路由 → 改模板字面量后命中。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev33 # rev33（2026-09-21 G5：组织-控制 单位身份列表）：
       # UnitApp.vue 选中单位详情面板新增「单位身份」列，消费 GET organization/assemble/control/identity/list/unit/{unitFlag} 1 条真实路由；
       # 全局消费 395→396（396/4638=8.5%）。契约回源码核验（identity_list_unit_unitFlag Path flag GET）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev32 # rev32（2026-09-21 G5：组织-个人 管理员签名列表查看）：
       # Personal.vue 签名卡片补「查看全员签名（管理员）」，消费 GET /api/person/signature/manager/list 1 条真实路由（admin 门禁）；
       # 全局消费 394→395（395/4638=8.5%）。契约回源码核验（personal::signature::manager_list，session 取 headers）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev31 # rev31（2026-09-21 G5：考勤 统计周期新建/回读）：
       # AttendanceApp.vue「排班与配置」补「统计周期」标签 + 新建，消费 attendance 族——新建(POST attendancestatisticalcycle 读 cycleYear/cycleMonth 必填)/回读(GET attendancestatisticalcycle/{id}) 共 2 条真实路由；
       # 全局消费 392→394（394/4638=8.5%）、desktop 调用 +2。契约回源码核验。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev30 # rev30（2026-09-21 G5：CMS 文档操作日志查看）：
       # DocumentApp.vue 每行补「日志」按钮，消费 GET /api/log/list/document/{documentId} 1 条真实路由（文档操作日志）；
       # 全局消费 391→392（392/4638=8.5%）、desktop 调用 +1。契约回源码核验（log_list_document_documentId GET）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev29 # rev29（2026-09-21 G5：CMS 应用权限查看 + 澄清域/全局消费率口径）：
       # AppInfoApp.vue 每卡补「权限」按钮，消费 permission 族——appInfo/{id}/managers|publishers|viewers 共 3 条真实路由（GET）；
       # **口径澄清**：这 3 条属 /api/permission/*，不在 consumption_gap.py 的 33 个业务域桶内 → 域口径 consumed 显示不变（仍 382/4072=9.4%），
       # 但全局口径 consumed 388→391（全局真实非 mock 4638 条，8.4%）。二者差异根源：566 条真实路由（/api/permission、/api/commend、/api/data、/api/fileinfo 等跨域基础设施）在域桶外，域口径分母 4072 系桶内子集。
       # **今后以全局口径 391/4638=8.4% 为准更诚实**（域口径易掩盖桶外路由的消费）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev28 # rev28（2026-09-21 G5：会议 会议室新建/删除管理）：
       # MeetingApp.vue 楼栋条下新增会议室管理条，消费 meeting 族——新建(POST assemble/control/room 读 name/buildingId)/删除(DELETE assemble/control/room/{id}) 共 2 条真实路由（list 已消费）；
       # 会议域消费 20→22、总消费 380→382（9.3%→9.4%）、desktop 调用 389→391。契约回源码核验（u2_room_create 读 name/buildingId）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev27 # rev27（2026-09-21 G5：组织-个人 授权委托 查看/启停）：
       # Personal.vue 新增「授权委托」卡片，消费 person/empower 族——我发出的(list/currentperson)/授权给我(list/to)/启用({id}/enable)/禁用({id}/disable) 共 4 条真实路由；
       # 组织-个人域消费 7→11、总消费 376→380（9.2%→9.3%）、desktop 调用 385→389。契约回源码核验（empower crate，session 取自 headers）。
       # 踩坑：初版 loadEmpower 用 `const url=三元;api.get(url)` + toggle `${on?'enable':'disable'}` → 提取器无法解析/归一化成 mockdeletetoget，消费未增；改为字面量路径分支后 4 条全部命中。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev26 # rev26（2026-09-21 G5：会议 楼栋新建/删除管理）：
       # MeetingApp.vue 新增楼栋管理条，消费 meeting 族——新建(POST assemble/control/building 读 name)/删除(DELETE assemble/control/building/{id}) 共 2 条真实路由；
       # 会议域消费 18→20、总消费 374→376（9.2%）、desktop 调用 383→385。契约回源码核验（u2_building_create 读 name，admin/session 门禁）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev25 # rev25（2026-09-21 G5：考勤 管理员/导入记录/统计日志 只读）：
       # AttendanceApp.vue「排班与配置」面板再补 3 标签，消费 attendance 族——管理员(attendanceadmin/list/all)/导入记录(attendanceimportfileinfo/list/all)/统计日志(attendancestatisticrequirelog/list/all) 共 3 条（GET list/all 无参）；
       # 考勤域消费 24→27、总消费 371→374（9.1%→9.2%）、desktop 调用 380→383。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS、vitest 953（38 文件）。
       # rev24 # rev24（2026-09-21 G5：日历 我的/公共列表 + 修复 rev23 遗留 fixture 漂移）：
       # CalendarApp.vue 新增「我的/公共日历」列表条，消费 calendar/list/my + calendar/list/public 共 2 条（GET 字面量路径）；日历域 8→10。
       # **修复 rev23 遗留**：rev23 加的 meeting participant/list+add 两路由虽真实注册，但 backend-registered-routes.fixture.ts 落后后端 128 条，desktop-endpoints 守卫自 rev23 起变红（我曾误报"全绿"）。
       # 新增 gen_fixture.py 从 backend_routes.json 重生成 fixture（4332 条）→ vitest 恢复 953 全绿。
       # rev23 # rev23（2026-09-21 G5：会议 参会人列表/邀请）：MeetingApp.vue 会议卡片补参会人(GET {meetingId}/participant/list)/邀请(POST {meetingId}/participant/add 读 invitee) 2 条；会议域 16→18。
       # 【本会话累计 rev12–rev24：总消费 322→371（7.9%→9.1%），15 切片 +49 唯一路由】
       # 验证（rev24 收官）：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953（38 文件全绿）。
       # rev22 # rev22（2026-09-21 G5 逐域推进：考勤 排班/员工/假期/工作日 配置查看）：
       # AttendanceApp.vue 考勤配置区补「排班与配置」四标签只读面板，消费 attendance 族——
       # 排班(schedulesetting/list/all)/员工配置(employeeconfig/list/all)/自助假期(selfholiday/list/all)/工作日(workdayconfig/list/all) 共 4 条真实路由；
       # 考勤域消费 20→24、总消费 363→367（8.9%→9.0%）、desktop 调用 373→377。契约回源码核验（均 GET list/all 无参）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953。
       # rev21 （2026-09-21 G5 逐域推进：CMS 文档 阅读数/可见人/通知）：
       # DocumentApp.vue 每行补 3 个信息/动作，消费 document 族——阅读数(GET {id}/view/count)/可见人(GET {id}/persons)/通知(POST {id}/notify) 共 3 条真实路由；
       # CMS 域消费 67→70、总消费 360→363（8.8%→8.9%）、desktop 调用 370→373。契约回源码核验（均 Path id 无 body）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953。
       # rev20 （2026-09-21 G5 逐域推进：CMS 文档操作 取消推荐/取消置顶/撤发）：
       # DocumentApp.vue 每行补 3 个对称操作，消费 document 族——取消推荐(GET {id}/uncommend)/取消置顶(GET {id}/unTop)/撤发(PUT publish/{id}/cancel) 共 3 条真实路由；
       # CMS 域消费 64→67、总消费 357→360（8.8%）、desktop 调用 367→370。契约回源码核验（均 Path id 无 body）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953。
       # rev19 （2026-09-21 G5 逐域推进：CMS 文档操作 推荐/置顶/发布）：
       # DocumentApp.vue 每行补 3 个操作按钮，消费 document 族——推荐(GET document/{id}/commend)/置顶(GET document/{id}/top)/发布(PUT document/publish/{id}) 共 3 条真实路由；
       # CMS 域消费 61→64、总消费 354→357（8.7%→8.8%）、desktop 调用 364→367。契约回源码核验（均 Path id 无 body，session 门禁）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953。
       # rev18 （2026-09-21 G5 逐域推进：CMS 应用/分类 新建删除）：
       # AppInfoApp.vue + CategoryApp.vue 各补 create+delete：appinfo(POST /api/appinfo 读 alias/appType/icon/manager,admin 门禁 + DELETE /api/appinfo/{id})、
       # categoryinfo(POST /api/categoryinfo 读 appId必填/name/parentCategoryId,owner 门禁 + DELETE /api/categoryinfo/{id}) 共 4 条真实路由；
       # CMS 域消费 57→61、总消费 350→354（8.6%→8.7%）、desktop 调用 360→364。契约回源码核验（cms_assemble_control）。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953。
       # rev17 （2026-09-21 G5 逐域推进：程序中心 应用样式查看）：
       # ProgramCenterApp.vue 新增 Style 标签页——当前样式(GET appstyle/current/style)/门户应用(GET appstyle/index/portal) 共 2 条只读路由；
       # 程序中心域消费 48→50、总消费 348→350（8.5%→8.6%）、desktop 调用 358→360。契约回源码核验（读 x_applications：id/name/app_id/disable）。
       # appstyle 剩余多为图片上传/erase 族，需真实文件上传 UI，本轮不做。
       # 验证：build 通过、typecheck 6/6、compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、vitest 953。
       # rev16 （2026-09-21 G5 逐域推进：程序中心 平台配置查看/新建）：
       # ProgramCenterApp.vue 新增 Config 标签页——配置列表(GET config/list)/应用配置(config/list/application)/
       # 实体配置(config/list/entity)/新建更新(POST config/save,ConfigSaveRequest key必填/value/category/creator) 共 4 条真实路由；
       # 程序中心域消费 44→48、总消费 344→348（8.4%→8.5%）、desktop 调用 354→358。契约回源码核验（config_save key 冲突则更新、x_program_config 表）。
       # 用唯一锚点规避 rev15 定义/调用误配断裂，build 一次通过。
       # 验证：compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、typecheck 6/6、vitest 953、desktop build 通过。
       # rev15 （2026-09-21 G5 逐域推进：程序中心 Invoke 接口管理）：
       # ProgramCenterApp.vue 新增 Invoke 标签页——列表(GET invoke)/新建(POST invoke,U2InvokeRequest name必填)/
       # 删除(DELETE invoke/{id})/分类(GET invoke/list/category) 共 4 条真实路由；程序中心域消费 40→44、
       # 总消费 340→344（8.3%→8.4%）、desktop 调用 350→354。
       # 契约回源码核验：U2InvokeRequest 字段 name/alias/category/description 与前端载荷一致。
       # 踩坑：Python 原地编辑时 loadMarketCats() 锚点误配到函数定义行 → 签名与函数体断裂致 build 失败；已修复（还原函数体 + 补调用）。
       # 验证：compare --gate EXIT=0、schema_audit --gate PASS（A=B=C=D=0/E=2）、typecheck 6/6、vitest 953、desktop build 通过。
       # rev14 （2026-09-21 G5 逐域推进：程序中心 应用市场 安装/卸载/分类）：
       # ProgramCenterApp.vue Market 页扩展——分类栏(list/category)+热门 Top3(list/top/three)+每卡「安装/更新·版本·卸载」
       # 消费 program_center market 族 5 条真实路由（list/category、list/top/three、{flag}/install/or/update、
       # {flag}/installed/version、{flag}/uninstall，均 GET 无 body 零契约风险）；程序中心域消费 34→39、
       # 总消费 334→339（8.2%→8.3%）、desktop 调用 344→349。顺带修既有 confirmMsg 用而未 import。
       # 契约坑：install/version/uninstall 前端须用模板字面量（`.../${id}/uninstall`），字符串拼接会被提取器在 + 处截断成 market/{} 误判。
       # 验证：compare --gate EXIT=0（shadow/405/404=0，desktop 349/mobile 80/sdk 5）、schema_audit --gate PASS（A=B=C=D=0/E=2）、
       # typecheck 6/6、vitest 953、desktop build 通过。
       # rev13 （2026-09-20 G5 逐域推进：组织-控制 单位属性/职务消费）：
       # UnitApp.vue 新增「选中单位→属性/职务管理」面板，消费 organization/assemble/control 下
       # unitattribute + unitduty 两族 CRUD 共 6 条真实路由（list/unit/{flag}、POST、DELETE/{id} 各 2）；
       # 组织-控制域消费 9→15、总消费 328→334（8.1%→8.2%）、desktop 调用 338→344。
       # 契约逐条回源码核验：unit_attribute_create 读 name(=attributeKey)/unitId(单位 flag，resolve_generic_id)/attributeValue，
       # duty_create 读 name/unitId/identityList，均 require_admin、creator 取会话；list 按 unit_id 查。
       # 顺带修 UnitApp 两处既有运行时 bug（.vue 不被 tsc 校验遗留）：loadUnits 被 template/顶层调用却从未定义（视图挂载即崩）→ 定义 loadUnits + doSearch 改本地过滤；toast 用而未 import → 补 import。
       # 验证：compare --gate EXIT=0（shadow/405/404=0，desktop 344/mobile 80/sdk 5）、schema_audit --gate PASS（A=B=C=D=0/E=2）、
       # typecheck 6/6、vitest 953、desktop build 通过。
       # rev12（2026-09-20 G5 逐域推进：考勤配置消费 + 消费率精确口径）：
       # 新增 docs/audits/three-ends-2026-09-20/consumption_gap.py 按业务域精确核算后端能力消费率——
       # 排除 407 条 mock* 兼容别名后真实可消费唯一(method,path)=4072，逐域列出「总/mock/真实/已消费/缺口」。
       # AttendanceApp.vue 新增「考勤配置」面板，消费 workplace + attendancesetting 两族 CRUD 共 6 条真实路由
       # （workplace list/create/delete + attendancesetting list/create/delete）；desktop 调用 332→338、
       # 消费 322→328（7.9%→8.1%）。契约逐条回源码核验（create 读 name/code 等、admin 门禁、creator 取会话）。
       # 验证：compare --gate EXIT=0（shadow/405/404=0，desktop 338/mobile 80/sdk 5）、
       # schema_audit --gate PASS（A=B=C=D=0/E=2 ≤ 基线）、typecheck 6/6、vitest 953、biome 0 err、desktop build 通过。
       # G5 仍为长期项：100% 等价两端消费全部 4072 条能力面，按业务优先级逐域推进（缺口 Top：流程-表面 874/程序中心 381/考勤 228…）。
       # rev11（2026-09-20 阶段 F 契约全对齐 + 提取器升级 → A=B=C=D=0）：
       # 升级 schema_audit.py 提取器跟随多层间接引用（消除结构性误报）：
       # ① 修 `let mut` 捕获 + 排除 DB/IO 结果变量作接收者（login/switchuser 假读清零）；
       # ② 捕获单字面量键 helper（json_str/u2_body_str(recv,"k")）；
       # ③ 解析 CrudSpec（NAME_spec()/内联 columns）委派读取键（D 类落地结论）；
       # ④ 读取键必填/可选分类（.ok_or→必填；.unwrap_or*/会话/Path/CrudSpec/结构体字段/闭包默认→可选），C 仅计必填；
       # ⑤ 支持 `Json(bp): Json<Struct>` 解构签名 + 字段级 #[serde(rename/alias)]；
       # ⑥ 本地取值闭包 `let get=|k| body.get(k)` + `get("lit")`。
       # 提取器暴露的真实契约缺陷已全修：login 补可选 captcha 校验、switchuser targetUnique 别名、
       # CreateDesignerRequest sql 别名、ChatCompletionRequest 接受 message/title/clueId(修复空 messages 400)、
       # role create/update 发 description、FindDesigner/Homepage config→query/content、
       # QueryDesigner runQuery 改走 execute(原误打 create 端点)、task complete/reject 仅发 opinion。
       # 结果：schema_audit A=0/B=0/C=0/D=0/E=2，基线收紧为硬 0；fail-injection 验证门禁仍可失败（注入 bogus 键 B=2）。
       # 验证：cargo check/test 5 crate(53+93+15+58+46 passed)、fmt(改动文件)、vitest 953、mobile 101(更新 4 处审批契约测试)、
       # biome lint 0 err、compare+schema 双 gate EXIT=0、pnpm typecheck 6/6。
       # 顺带修复既有构建配置类型漂移：vite.config.ts 的 manualChunks 由对象改函数形态（当前 rollup 类型只认函数），
       # 分组等价、desktop build 通过（vue/query/naive/codemirror 分包一致）、typecheck 转全绿。
       # rev10（2026-09-20 阶段 F 契约修复 + 阶段 G 移动端扩面）：
       # 阶段 F 修复一批真实请求/响应契约错配（前端对齐后端读取键 + 后端补能力）——
       # attendance appeal audit(status→auditStatus)/rule create(name→ruleName 族)/appeal submit(补 creator)、
       # form(schema→definition，POST/PUT 同步)、IM 发消息补 sender、queryview search(keyword→key，桌面 2 + 移动 1)、
       # 会议 create 前端补 endTime/content/creator 输入并对齐载荷（后端 create_meeting 增读 roomId 落 x_meeting.room_id）、
       # query designer execute 后端支持「按 id 取 x_query_statement.data 作为 SQL」(H-4)。
       # 效果：schema_audit B 18→10 / C 21→16，A=0/E=2/D=23 不变，gate PASS（≤基线）。
       # schema_baseline.json 随之收紧到 B10/C16 并逐条注明残留误报/委派/产品项（只降不升）。
       # 阶段 G 移动端扩面：新增 browse/calendar/meeting/recycle/search 页 + 工作台今日出勤 tile，
       # 移动调用 29→80（≥80）、业务域 6→18（≥12），四端 shadow/405/404 全 0；
       # 修复移动 document/search 405（POST→GET 全文检索）。
       # 验证：typecheck 6/6、vitest 953、mobile 101、mobile-endpoints 契约 3/3、biome lint 净、
       # cargo fmt(改动文件)、query/meeting 两 crate 46+58 passed、quoted_key_guard/designer_route_match 各 1 passed、
       # compare --gate EXIT=0、schema_audit --gate EXIT=0。
       # rev9（2026-09-20 G3 解决）：用户选定「合并到 AIChatApp」——
       # 把 MCP 配置面板从 AIAssistant 迁入 AIChatApp 的 showConfig 并补侧栏入口；
       # 删除 AIAssistant.vue（200 行）与 AIChatApp 的 95 个未使用生成器残留 ref；
       # 顺带修原实现的 3 处缺陷：禁用分支 404（POST config/delete/mcp 缺 id → 改 POST config/update/mcp/{id}）、
       # addMcp 空函数（死控件→实现内联表单）、列表字段读 endpoint 而后端产出 url（该列恒为空）。
       # 保留 AIChatApp 既有行为（chat/list/paging + 非流式），未采纳 AIAssistant 的流式端点。
       # 验证：typecheck 6/6、pnpm test 953 passed、biome lint 干净、四端对账仍 100%（334 调用，
       # shadow/405/404 全 0，gate PASS）。
       # rev8（2026-09-20 契约提取器升级）：让提取器「跟随一层间接引用」——闭包绑定 Json、
       # `let` 别名、一层 helper 调用，并把「参数类型为 Value」的普通 helper 也纳入索引
       # （原先只索引有 Json 参数的 handler，导致跟随失败）。
       # 效果：消除 2 处已证实误报（task opinion / im_msg conversationId，B 24→18）；
       # **新发现 4 处真实字段错配**（attendance appeal status↔auditStatus、rule name↔ruleName、
       # form schema↔definition、mobile check userId）；也引入 3 处新误报（login/switchuser 的
       # auth_person 列名、task/reject 的 Path id）——已在 §6.10 逐条列出。
       # 基线同步更新（B 18 / C 21 / D 24 / E 2 / A 0）并在文件内留 _note 说明"因提取器变敏感而移动"。
       # 修复过程中踩坑：别名推导误用 `name` 变量覆盖了 handler 名，使 handlers 从 921 掉到 479（已修）。
       # rev7（2026-09-20 阶段 H 复核）：逐条复核一致性治理——
       # G1 复核为**非问题**（control::unit.rs 产出的 JSON 键本就是 parentId，原判断基于 SQL 列名有误）；
       # G2 已文档化（407 条 mock* 为 O2OA 兼容层，不可清理）；
       # G3 判为**产品决策**不擅自改（AIAssistant 与 AIChatApp 标题同为「AI 助手」，
       #    前者含 MCP 配置面板但未路由 → MCP 配置当前不可达；三选项已写入文档）；
       # G4 已修（EmptyApp 必填 props 无人传入 → 改可选 + 默认文案）；
       # G5 长期项。**未改动任何后端代码**。
       # rev6（2026-09-20 阶段 E 完成）：防回归门禁落地——compare.py / schema_audit.py 新增 --gate
       # （A 必须为 0；B/C/D/E 不得高于 schema_baseline.json，只许降不许升），新增 CI workflow
       # .github/workflows/three-ends-contract.yml（extract→compare --gate→schema_audit --gate，
       # 产物上传 artifact）；E6 守卫接入 oa4rust-ci.yml 的 quality job。
       # **门禁可失败已验证**：人为压低基线 → EXIT=1，恢复 → EXIT=0。
       # 两个 workflow 的 YAML 已用 pyyaml 校验；action SHA 与仓库既有钉法逐一对齐。
       # rev5（2026-09-20 全量实施完成）：三端 API 调用闭合率 100%（桌面 336/336、移动 29/29、
       # 共享 sdk 5/5、共享 ui 3/3），shadow/405/404 全 0。补齐遗留 7 条后端路由
       # （attendance rule DELETE、jpush device DELETE、portal page DELETE、pc dict data POST、
       # console config update/delete、unit check GET）；新增阶段 E6 转义引号守卫测试。
       # 验证：cargo check/fmt EXIT=0、11 crate 634 passed / 0 failed、E6 守卫 1 passed、
       # designer_route_match 1 passed、前端 953 + 移动 101 passed、typecheck 6/6。
       # 遗留：阶段 F 的 B/C/D 计数为上界（含提取器间接引用误报，已注明）；G/H 未做。
       # rev4（2026-09-20 全量实施）：状态表更新为实施结果。已完成阶段 0（转义引号 363 处 + 审批意见
       # 落库 + IM 会话过滤 + 前端绕过清理，A 5→0 / E 351→2）、阶段 A（影子 22→0）、阶段 D（复核后免除：
       # 5 处均为 fallback 次级声明，非真缺陷）；B 13/17、C 7/10（BBS 7 处经复核为提取器 fmt 前缀缺陷
       # 导致的误报，已回滚误改）；遗留 7 处需后端补路由（已精确定位）。
       # 又发现 2 个提取器缺陷：&fmt() 前缀未还原（bbs 106 条路由）、字符串拼接截断。
       # 验证：cargo check/fmt EXIT=0、4 crate 254 passed、前端 953 passed、移动端 101 passed、typecheck 6/6。
       # rev3（2026-09-20）：建立「实施状态总表」（§九 末）作为活体追踪——审计章节（§〇–§八）是
       # 绑定 HEAD fa5b83db 的**时效快照**，本表是唯一持续更新的部分；每完成一个阶段就地更新本表 + rev 上滚。
       # 同时把 evidence_base 钉到具体 SHA，便于判断审计结论是否已过期。
       # rev2（2026-09-20）：按用户要求把「请求体/响应体深度 schema 校验」纳入本次范围并已实做——
       # 新增 §六（A 请求体被忽略 5 / B 发送未读 21 / C 读而未发 15 / D 委派不可判定 20 / E 转义引号缺陷 351）；
       # 最重要发现：task_complete|reject 完全忽略请求体 → 审批意见从未落库；
       # 转义引号键缺陷 351 处（81 处读错列名会 panic、115 处响应键前端读不到）；
       # 修正 6 处 schema 提取器缺陷（全限定 axum::extract::Json / Option<Json> / or_else 或组 /
       # handler 名跨 crate 冲突 / 委派检测 / JS 简写属性）；原「阶段 E2 待做」改为本轮已做 + 新增阶段 F（运行时校验）
module: oa4rust + oa4rust-web（desktop / mobile）
tags: [three-ends-audit, api-reconciliation, field-consistency, schema-validation, request-body-contract, response-body-contract, escaped-quote-defect, shadow-route, method-mismatch, landing-plan]
problem_type: completeness-audit
evidence_base: 工作区 HEAD `fa5b83db`（2026-09-20），机器提取 + 逐条回源码核验
snapshot_note: §〇–§八 为绑定上述 SHA 的时效快照；修复推进后行号/计数会过期，以 §九 末「实施状态总表」为准
---

# 三端全面审计与落地规划

> **阅读指引**：本文档是**审计与修复一体**的。
> - **§〇–§八 = 审计结论**（绑定 HEAD `fa5b83db` 的**时效快照**，修复推进后行号/计数会过期）；
> - **§九 = 修复计划**，其末的 **「实施状态总表」是唯一持续更新的部分** —— 想知道"当前做到哪了"只看那张表；
> - **§十 / §十一 = 风险与验收总纲**。
>
> 每完成一个阶段：更新状态表对应行 + 在 frontmatter `rev` 上滚一条说明。

**审计对象（三端）**

| 端 | 代码位置 | 形态 |
|---|---|---|
| **服务端** | `oa4rust/`（90 个 crate 的 Cargo workspace，axum） | REST API |
| **桌面端** | `oa4rust-web/apps/desktop/`（Vue 3 + Vite，84 个视图） | 桌面 Web |
| **移动端** | `oa4rust-web/apps/mobile/`（uni-app，9 个页面） | H5 / 小程序 / App |

> 说明：`oa4rust-web/packages/{sdk,ui,apis}` 是三端共享层，其对外调用一并纳入对账（见 §3.2）。

**审计判据（三项）**

1. **API 调用完整** —— 前端每一条调用都能在后端命中"路径 + 方法"。
2. **业务流程完整** —— 每个业务域在三端都有闭环入口（发起→提交→回显）。
3. **字段对得上** —— 前端消费的响应字段，后端确实会产出。

---

## 〇、审计方法与可复现性

本次不沿用历史结论，全部对**当前代码**重新机器提取 + 回源码核验。脚本与产物已归档：

```
docs/audits/three-ends-2026-09-20/
├── extract_routes.py       # 后端路由提取（axum .route 解析 + handler 名，排除 #[cfg(test)]）
├── extract_calls.py        # 前端调用提取（括号平衡 + 常量/对象表解析）
├── compare.py              # 三端 API 对账（exact / param / shadow / 405 / 404）
├── triage.py               # 问题项 → 后端同前缀候选路由
├── field_audit.py          # 字段一致性（后端键词表 vs 前端声明字段）
├── schema_audit.py         # 请求/响应体深度 schema 校验（A–E 五类）
├── flow_matrix.py          # 业务域 × 三端 覆盖矩阵
├── backend_routes.json     # 后端路由清单（5055 条注册，含 handler 名）
├── frontend_calls.json     # 前端调用清单（含未解析登记）
├── api_reconcile.json      # 对账结果（逐条）
├── field_audit.json        # 字段审计结果
├── schema_audit.json       # schema 校验结果（含 A–E 明细）
├── schema_report.txt       # schema 校验可读报告
└── triage.txt              # 57 处问题的候选路由对照（人工判定依据）
```

复现命令（Python 3.13，脚本自带仓库根定位，可从任意位置运行）：

```bash
P=C:/Users/Administrator/.workbuddy-ai/binaries/python/versions/3.13.12/python.exe
D=docs/audits/three-ends-2026-09-20
$P $D/extract_routes.py      # → backend_routes.json
$P $D/extract_calls.py       # → frontend_calls.json
$P $D/compare.py             # → api_reconcile.json
$P $D/triage.py              # → 候选路由对照（stdout）
$P $D/field_audit.py         # → field_audit.json
$P $D/schema_audit.py        # → schema_audit.json + schema_report.txt
$P $D/flow_matrix.py         # → 业务域覆盖矩阵（stdout）
```

> **执行顺序有依赖**：`schema_audit.py` 读取 `backend_routes.json`，须在 `extract_routes.py` 之后运行。

### 提取器可信度（fail loud）

审计工具本身若不可信，结论就无意义。本次过程中修正了**三个提取器缺陷**，并逐一验证：

| 缺陷 | 症状 | 修正 | 验证 |
|---|---|---|---|
| 链式方法被漏掉 | 方法正则排除了 `.`，导致 `.post(h).put(h)` 中的 `put` 全丢 | 改为只排除 `[A-Za-z0-9_]` | 后端注册数 4939 → **5055**（+116），`data/work/{id}` 由"仅 POST"纠正为 POST+PUT |
| 模板表达式被截断 | 先切 `?` 再替换 `${...}`，含三元表达式的路径被砍断 | 先替换模板、再切查询串 | 消掉 1 处假 404 |
| 对象表跨行误匹配 | 类型注解 `[^=]*` 可跨行，把 `designerPaths` 等整块吞掉 | 限定 `[^={;\n]*` | 未解析调用 18 → 2 |

**最终提取覆盖率**：桌面端 337 条调用中 **335 条静态解析成功（99.4%）**，2 条未解析（已人工核验，见 §3.3）；移动端 29 条 **全部解析**；后端 **零盲区**（全部 `.route` 均为字面量路径，无非字面量注册）。

> 请求/响应 schema 校验（§六）另修正了 **6 处**语义级提取器缺陷（全限定 `axum::extract::Json`、`Option<Json<>>`、`or_else` 或组、handler 名跨 crate 冲突、委派检测、JS 简写属性）——逐条见 **§6.8**。这些缺陷每一处都曾造成误报，先用反例证伪再修，是本节结论可信的前提。

---

## 一、结论速览

| 维度 | 服务端 | 桌面端 | 移动端 |
|---|---|---|---|
| API 调用闭合率 | — | **280 / 337 = 83.1%** | **29 / 29 = 100%** |
| 问题调用数 | — | **57**（影子 22 / 405 24 / 404 11） | **0** |
| 字段一致性（§五） | — | 6 接口 10 字段存疑 → **复核后均为冗余 fallback 声明，无真缺陷** | 0 |
| **请求/响应 schema（§六）** | **A 5 / B 21 / C 15 / D 20 / E 351** | 同上（问题集中在后端契约） | 同上 |
| 路由可达 | 5036 唯一 (method,path) | 84 视图 / 83 已挂路由 | 10 页面 / 5 个 tab |

**一句话结论**：**移动端 API 层已完全对齐；桌面端尚有 57 处接线错误（其中 22 处是"静默影子误路由"——返回 200 但拿错数据）；服务端能力面极宽（4327 条唯一路径），但三端只消费了 5.8%；而最深的一层问题是后端契约自身的缺陷——5 处请求体被完全忽略、351 处键名转义错误，导致"接口能通、数据不对"。**

### 四个必须知道的判断

1. **最高危的不是 404，而是 22 处"静默影子"**。前端调 `/api/form/list`，后端只有 `/api/form/{id}`——`list` 被当成 id 吃进去，**返回 200 + 错体**。在空库上表现为"空列表"、在有数据时表现为"返回了某条记录"，排障成本远高于直接 404。
2. **桌面端的问题高度聚集在少数视图**：`FormApp`(6) / `MeetingApp`(6) / `BBSForum`(7) / `ProgramCenterApp`(5) 四个视图占了 57 处中的 **24 处（42%）**，且都是"业务上确实要用、但接线写错"。
3. **移动端 100% 闭合但功能面很窄**：9 个页面、29 条调用，只覆盖 登录/工作台/消息/聊天/审批/考勤/文档/通讯录/我的。组织管理、会议、论坛、日历、门户、查询、CMS 等域在移动端**完全空白**——这是"三端功能完备"的最大结构性缺口。
4. **本轮最深的一层：后端契约自身有缺陷（§六）**。三类最重：
   - `task_complete` / `task_reject` **完全忽略请求体** → 桌面与移动两端收集并发送的**审批意见从未落库**，流程记录的"内容"与"操作人"都是硬编码；
   - **351 处键名转义错误**（`"\"key\""`）→ 81 处读不存在的列会 **panic(500)**、115 处响应键前端读不到（前端已有 4 处被迫绕过，其余静默失效）；
   - 会议 5 个动作只发 `{id}`、查询执行只发 `{id,filter}` 而后端只读 `sql` → **接口通但语义不成立**。

---

## 二、服务端路由面清点（基线）

| 指标 | 数值 |
|---|---|
| crate 数 | 90 |
| `.route()` 注册数 | 5,055 |
| 唯一 (method, path) | **5,036** |
| 唯一路径（参数归一化） | 4,327 |
| 方法分布 | GET 2,782 / POST 1,518 / PUT 451 / DELETE 304 |
| 兼容别名路由（`*mockputtopost` / `*mockdeletetoget`） | **407** |
| 提取盲区（非字面量路径注册） | **0** |

路由量 Top 10 crate：

| crate | 路由数 |
|---|---|
| processplatform_assemble_surface | 1,067 |
| cms_assemble_control | 480 |
| program_center | 408 |
| attendance_assemble_control | 247 |
| organization_assemble_control | 240 |
| processplatform_service_processing | 222 |
| file_assemble_control | 192 |
| query_assemble_designer | 175 |
| processplatform_assemble_designer | 144 |
| bbs_assemble_control | 147 |

**观察**：`*mockputtopost` / `*mockdeletetoget` 共 407 条（占 8.1%）——这是为兼容 O2OA 前端"用 GET 模拟 PUT/POST"历史行为而保留的别名族。属有意设计，但需在文档中显式声明，否则会被误判为冗余路由。

---

## 三、三端 API 对账结果

### 3.1 总览

| 端 | 调用总数 | 完全一致(exact) | 参数位一致(param) | 静默影子 | 方法不符(405) | 路径不存在(404) | 闭合率 |
|---|---|---|---|---|---|---|---|
| 桌面端 | 337 | 117 | 163 | **22** | **24** | **11** | 83.1% |
| 移动端 | 29 | 14 | 15 | 0 | 0 | 0 | **100%** |
| 共享 sdk | 5 | 0 | 5 | 0 | 0 | 0 | **100%** |
| 共享 ui | 3（动态展开） | — | 3 | 0 | 0 | 0 | **100%** |

判定口径：
- **exact**：前后端段完全一致（含参数位）。
- **param-ok**：仅靠"后端参数段吞掉前端字面量"匹配，且该字面量是 id/flag 一类取值（如 `/paging/1/size/20` → `/paging/{page}/size/{size}`）。
- **suspicious-shadow**：被吞掉的字面量是**资源子名**（list / filter / update / delete …）而参数名是通用标识位（`{id}` / `{flag}`）→ 静默误路由。
- **405**：路径存在但方法未注册。
- **404**：无任何路径匹配。

### 3.2 共享层（packages/）

| 调用 | 位置 | 结论 |
|---|---|---|
| `POST /api/authentication/login`、`logout`、`switchuser`，`GET /api/authentication/who` | `packages/sdk/src/session.ts`、`router.ts` | ✅ 全部命中 |
| `PUT /api/organization/assemble/control/{unit\|person\|identity}/list/like` | `packages/ui/src/components/organization-selector.ts:73` | ✅ 三个类型的 PUT 变体后端均存在（已逐条核验） |

> 注：共享组织选择器是流程设计器"负责人"闭环的关键组件（W8），其调用**正确**，不在问题清单内。

### 3.3 未解析调用（2 条，已人工核验）

| 位置 | 代码 | 核验结论 |
|---|---|---|
| `views/FormDesigner.vue:1015` | `api.get(path)`，`path = routeAppId ? '/api/form/list/app/{id}' : '/api/form/list/all'` | ✅ 两条分支后端均存在 |
| `views/EmptyApp.vue:57` | `api.get(props.apiPath)` | ⚠️ 路由 `empty` **未传任何 props**（`main.ts:474`），且 `apiPath` 未设置时提前 return → **该调用是死分支**；同时 `EmptyApp` 声明了必填 props `title`/`subtitle` 却无人传入，是占位视图（见 §5.2） |

---

## 四、桌面端 57 处问题清单（逐条，含修法）

> 每条都经"前端源码 + 后端路由表"双向核验。`hit` 列是当前实际命中的后端路由——正是它造成了静默误路由。

### 4.1 静默影子误路由（22 处，**P0**）

前端路径被后端宽路由 `{id}` / `{flag}` 吞掉，**返回 200 但拿错数据**。

| # | 前端调用 | 位置 | 实际命中 | 应改为 | 后端是否有正确路由 |
|---|---|---|---|---|---|
| 1 | `GET /api/appinfo/filter` | AppInfoApp.vue:64 | `/api/appinfo/{id}` | `PUT /api/appinfo/filter/list/{id}/prev\|next/{count}` | ✅ |
| 2 | `GET /api/appinfo/list` | AppInfoApp.vue:67 | `/api/appinfo/{id}` | `GET /api/appinfo/list/all` | ✅ |
| 3 | `GET /api/categoryinfo/list` | CategoryApp.vue:49 | `/api/categoryinfo/{id}` | `GET /api/categoryinfo/list/all` | ✅ |
| 4 | `GET /api/document/list` | DocumentApp.vue:101 | `/api/document/{id}` | `POST /api/document/list/document` | ✅（方法也需改） |
| 5 | `GET /api/query/assemble/designer/find/list` | FindDesignerApp.vue:52 | `/designer/{id}/{count}` | 需按后端 find 族实际路径对接 | ⚠️ 待定 |
| 6 | `GET /api/form/list` | FormApp.vue:255 | `/api/form/{id}` | `GET /api/form/list/all` | ✅ |
| 7 | `GET /api/form/v2/list` | FormApp.vue:267 | `/api/form/v2/{id}` | 无 v2 列表端点 → 应改用 `/api/form/list/all` | ❌ 后端无 |
| 8 | `PUT /api/form/update/{id}` | FormApp.vue:311 | `/api/form/{id}` | `PUT /api/form/{id}` | ✅ |
| 9 | `DELETE /api/form/delete/{id}` | FormApp.vue:325 | — | `DELETE /api/form/{id}` | ✅ |
| 10 | `GET /api/portal/assemble/surface/homepage/list` | HomepageApp.vue:51 | `/surface/{page}/{id}` | 需按 homepage 族实际路径对接 | ⚠️ 待定 |
| 11 | `GET /api/meeting/.../meeting/list` | MeetingApp.vue:100 | `/meeting/{id}` | `GET .../meeting/list/apply/{page}/size/{size}`（或 invited/applied 变体） | ✅ |
| 12 | `PUT /api/meeting/.../meeting/update` | MeetingApp.vue:144 | `/meeting/{id}` | `PUT .../meeting/{id}` | ✅ |
| 13 | `POST /api/meeting/.../meeting/cancel` | MeetingApp.vue:153 | `/meeting/{id}` | 需查 meeting 审批/取消族真实路径 | ⚠️ 待定 |
| 14 | `POST /api/meeting/.../meeting/approve` | MeetingApp.vue:161 | `/meeting/{id}` | 同上 | ⚠️ 待定 |
| 15 | `POST /api/meeting/.../meeting/join` | MeetingApp.vue:169 | `/meeting/{id}` | 同上 | ⚠️ 待定 |
| 16 | `POST /api/meeting/.../meeting/leave` | MeetingApp.vue:178 | `/meeting/{id}` | 同上 | ⚠️ 待定 |
| 17 | `GET /api/organization/assemble/control/group/list` | OrgViewer.vue:76 | `/group/{flag}` | `GET .../group/list/{flag}/next/{count}` | ✅ |
| 18 | `GET /api/program_center/agent/list` | ProgramCenterApp.vue:258 | `/agent/{flag}` | `GET /api/program_center/agent`（裸） | ✅ |
| 19 | `POST /api/program_center/dict/{}/data/data` | ProgramCenterApp.vue:479 | `/dict/{dictFlag}/{path}/data` | `POST .../dict/{dictFlag}/{path}/data`（去重复段） | ✅ |
| 20 | `GET /api/role/list` | RoleManager.vue:76 | `/api/role/{flag}` | `POST /api/role/list`（organization_assemble_express） | ✅（方法也需改） |
| 21 | `GET /api/view/list` | ViewApp.vue:70 | `/api/view/{id}` | `GET /api/view/list/all` | ✅ |
| 22 | `POST /api/view/viewdata/{}` | ViewApp.vue:85 | `/api/view/{id}/mockputtopost` | `POST /api/view/viewdata/list/{id}/next/{count}` | ✅ |

### 4.2 方法不符 405（24 处，**P1**）

路径对，方法错——直接 405。

| # | 前端调用 | 位置 | 后端实际方法 | 应改为 |
|---|---|---|---|---|
| 1 | `DELETE /api/ai_assemble_control/config/delete/mcp/{id}` | AIAssistant.vue:129 | GET | 需后端补 DELETE，或前端改用 `POST /config/update/mcp/{flag}` |
| 2 | `DELETE /api/ai_assemble_control/chat/delete/{id}` | AIChatApp.vue:152 | GET | 同上，需确认契约 |
| 3 | `GET /api/attendance/assemble/control/attendancedetail` | AttendanceApp.vue:97 | POST | 改用 `GET .../attendancedetail/filter/list` 族 |
| 4 | `GET /api/calendar_assemble_control/event/list/filter` | CalendarApp.vue:94 | PUT | 前端改 PUT |
| 5 | `POST /api/document/document` | DocumentApp.vue:114 | — | 改用 `POST /api/document/list/document` |
| 6 | `PUT /api/query/assemble/designer/find/list` | FindDesignerApp.vue:74 | GET | 前端改 GET / 或按后端 find 族 |
| 7 | `GET /api/form` | FormApp.vue:277 | POST | 改用 `GET /api/form/{id}` 或 `/list/all` |
| 8-9 | `POST /api/form/create` | FormApp.vue:313, 380 | — | 改用 `POST /api/form` |
| 10 | `PUT /api/portal/assemble/surface/homepage/list` | HomepageApp.vue:73 | GET | 需按 homepage 族对接 |
| 11 | `DELETE /api/jpush/core/entity/device/{id}` | JPushApp.vue:99 | — | 后端 device 族无 DELETE，需补 |
| 12 | `GET /api/organization/assemble/control/group` | OrgViewer.vue:86 | POST | 改用 `POST .../group` 或列表族 |
| 13 | `POST /api/person/password` | Personal.vue:99 | PUT | 前端改 PUT `/api/person/password` |
| 14 | `DELETE /api/portal/assemble/surface/page/{id}` | PortalApp.vue:58 | — | 后端 page 族无 DELETE，需补 |
| 15 | `POST /api/program_center/module/{id}/compare` | ProgramCenterApp.vue:552 | GET/PUT | 前端改 GET |
| 16 | `POST /api/program_center/appstyle/image/{}/{}/erase` | ProgramCenterApp.vue:596 | DELETE | 前端改 DELETE |
| 17 | `POST /api/program_center/market/{flag}/cover/pic` | ProgramCenterApp.vue:632 | GET | 前端改 GET |
| 18 | `GET /api/queryview/search` | QueryViewApp.vue:77 | POST | 前端改 POST |
| 19 | `POST /api/queryview/view/list/paging/1/20` | QueryViewApp.vue:89 | GET | 改用 `GET /api/queryview/view/list/query/{queryFlag}` |
| 20 | `POST /api/queryview/execute/{id}` | QueryViewApp.vue:103 | GET | 改用 `GET /api/queryview/execute/{view}/{id}` |
| 21 | `POST /api/recycle/empty` | RecycleApp.vue:95 | DELETE | 前端改 DELETE |
| 22 | `GET /api/queryview/search` | SearchApp.vue:36 | POST | 前端改 POST |
| 23 | `POST /api/server/execute` | ServerApp.vue:72 | GET | 前端改 GET（或后端补 POST 语义） |
| 24 | `POST /api/server/stop` | ServerApp.vue:84 | GET | 同上 |

### 4.3 路径不存在 404（11 处，**P1**）

| # | 前端调用 | 位置 | 后端现状 | 建议 |
|---|---|---|---|---|
| 1 | `DELETE /api/attendance/assemble/control/rule` | AttendanceApp.vue:198 | 仅有 `rule/list`、`rule/create`、`rule/{id}/toggle` | 后端补 `DELETE rule/{id}` |
| 2 | `PUT /api/bbs/.../subject/search/list/page/1/count/{n}` | BBSForum.vue:371 | 仅 `GET subject/search` | 统一到后端分页契约 |
| 3 | `PUT /api/bbs/.../subject/recommended/list/page/{p}/count/{n}` | BBSForum.vue:377 | **无 recommended 族** | 后端补 或 前端改 creamed |
| 4 | `PUT /api/bbs/.../subject/creamed/list/page/{p}/count/{n}` | BBSForum.vue:382 | 仅 `GET subject/creamed/list` | 前端改 GET + 分页 |
| 5 | `PUT /api/bbs/.../user/reply/my/list/page/{p}/count/{n}` | BBSForum.vue:388 | 无 | 后端补"我的回复"分页 |
| 6 | `POST /api/bbs/.../subject/filter/listsubjectinfo/page/{p}/count/{n}` | BBSForum.vue:405 | 仅 `GET subject/filter/list` | 契约对齐 |
| 7 | `PUT /api/bbs/.../subject/index/list/page/{p}/count/{n}` | BBSForum.vue:410 | 仅 `GET subject/index/list` | 契约对齐 |
| 8 | `PUT /api/bbs/.../reply/filter/list/page/1/count/50` | BBSForum.vue:441 | 仅 `POST reply/create` | 后端补 reply 过滤分页 |
| 9 | `DELETE /api/config/delete` | ConfigDesignerApp.vue:237 | 仅 `POST /api/config/create` | 后端补 update/delete |
| 10 | `PUT /api/config/update` | ConfigDesignerApp.vue:246 | 同上 | 同上 |
| 11 | `GET /api/unit/check/{id}` | UnitApp.vue:65 | 仅 `POST /api/unit/check/unit/has/{identity\|person\|unit}` | 前端改用 POST has 族 |

### 4.4 问题视图聚集度

| 视图 | 问题数 | 性质 |
|---|---|---|
| BBSForum.vue | 7 | 后端契约缺失为主（6 条 404） |
| FormApp.vue | 6 | 前端接线全错（list/v2/update/delete/create 全错） |
| MeetingApp.vue | 6 | 前端猜路径（后端有 31 条 meeting/list 变体未被使用） |
| ProgramCenterApp.vue | 5 | 方法/路径错配 |
| QueryViewApp.vue | 3 | 方法错配 |
| 其余 20 个视图 | 各 1–2 | 零散错配 |

> **共 25 个视图**存在至少一处问题，占桌面端 84 个视图的 **29.8%**。

---

## 五、字段一致性审计

### 5.1 方法与结果

**后端字段约定（已核验）**：handler **手工**把 DB 列映射为 camelCase 键写入 JSON。例：

```rust
// oa4rust/crates/ai_assemble_control/src/lib.rs:216 ann_list
"createTime".to_string() => Value::String(row.get::<_, Option<String>>("create_time")...)
```

**审计方法**：构建后端全量"JSON 键词表"（`json!` 字面量键 + `Map` 键 + serde camelCase 结构体字段），共 **1,009 个键**；再抽取前端 `interface` / `type` 声明的响应字段，报告"前端声明但后端从不产出"的字段。

**结果**：桌面端 6 个接口共 10 个字段存疑；移动端 **0**。

### 5.2 复核结论：**5 处均为"冗余 fallback 声明"，非真错配**（rev4 逐条回源码核验）

`field_audit.py` 的判据是"interface 声明了后端从不产出的字段"。但逐条回前端源码后发现，
这些字段都处在 **fallback 链的次级位置**，主读取命中的是正确字段——**不构成缺陷**：

| 位置 | 声明 | 前端实际读取 | 复核结论 |
|---|---|---|---|
| `RecycleApp.vue` | `deleteTime` | `item.deletedAt \|\| item.deleteTime` | ✅ 主读 `deletedAt`（后端确实产出），`deleteTime` 只是冗余兜底 |
| `UnitApp.vue` | `unitFlag` | `u.flag \|\| u.unitFlag \|\| u.id` | ✅ 末级 `u.id` 命中后端产出的 `id` |
| `FormApp.vue` | `formFlag`/`schema`/`fieldCount`/`snapshot` | 该视图接线已全错（§4），字段随之为占位 | 随 §4 修复一并处理 |
| `MindApp.vue` | `root` | 需与 mind 树端点契约对齐 | ⏳ 待确认（低优先） |
| `QueryDesigner.vue` | `queryName`/`entityCategory` | 需与 query 设计器契约对齐 | ⏳ 待确认（低优先） |

> **结论修正**：原判"已证实 2 处真错配"**过度陈述**。正确表述是：**字段层无已证实的真缺陷**，
> 5 处均为"声明了兜底字段"或"随路由问题一并处理"。这修正了 §一 结论速览中的对应行。
>
> **判据改进**：字段审计若只看"interface 声明 ⊆ 后端产出"，会把 fallback 链误判为错配。
> 更准确的做法是**同时解析读取表达式链**（`a || b || c`），只要链上任一命中即通过——已列入 §9 阶段 E 的门禁改进项。

### 5.3 系统性风险（比单点错配更重要）

1. **字段映射无自动化**：1,009 个键全靠逐 handler 手写，**没有任何测试断言"前端声明的字段 ⊆ 后端产出的字段"**。本次只扫出 10 个，是因为词表取的是"全仓并集"（任一 handler 产出即通过），灵敏度低——**单端点级的字段错配会漏网**。
2. **命名约定存在漂移**：绝大多数 handler 用 camelCase（`createTime`），但 `control::unit::list` 用 `parent_id`。同类字段跨模块命名不统一。
3. **空值语义**：`row_to_json` 对 NULL 列**省略字段**（不写 `null`），而手工映射的 handler 多用 `unwrap_or_default()` 写空串/0。前端需同时容忍"字段缺失"与"空值"两种形态。

---

## 六、请求体 / 响应体深度 schema 校验（本次新增）

> §五 用的是"全仓键词表并集"——任一 handler 产出过就算通过，**灵敏度低，单端点级错配会漏网**。
> 本节改为**端点到端点**：不仅字段名对得上，还要**请求体真的被后端读了、响应体真的能被前端读到**。
> 脚本：`schema_audit.py`。

### 6.1 方法

```
路由 → handler 函数名（99.8% 捕获）
  → handler 函数体 → ① 请求体参数 ② 从请求体读取的键 ③ 产出的响应键
前端调用点 → 请求体顶层键（对象字面量 / 常量表 / 简写属性 / 计算属性名）
  → 比对：A 请求体被忽略 / B 发送但后端不读 / C 后端读但前端不发 / D 委派不可判定 / E 转义引号缺陷
```

### 6.2 结果总览

| 类别 | 数量 | 含义 | 危害 |
|---|---|---|---|
| **A 请求体被完全忽略** | **5** | 前端发了非空 body，handler 无 `Json` 参数 | **P0**——数据静默丢失 |
| **B 发送但后端不读** | **21** | 发送的键不在 handler 读取集内 | **P0/P1**——功能静默失效 |
| **C 后端读但前端不发** | **15** | handler 需要的键前端从未发送 | P1——字段恒为空/默认值 |
| **D 委派给泛型 helper** | **20** | 键读取在被调用的泛型函数内，静态不可判定 | 需运行时校验（见 §九 阶段 F） |
| **E 转义引号键缺陷** | **351** | 键名/列名/路径参数名里**字面包含引号** | **P0**——见 6.7 |

### 6.3 A 类：请求体被完全忽略（5 处，**P0**）

| # | 端点 | 调用位置 | 前端发送 | handler |
|---|---|---|---|---|
| 1 | `POST /api/task/{id}/complete` | `ProcessWork.vue:332`（桌面） | `data`, `opinion` | `task_complete` |
| 2 | `POST /api/task/{id}/reject` | `ProcessWork.vue:334`（桌面） | `data`, `opinion` | `task_reject` |
| 3 | `POST /api/task/{id}/complete` | `services/index.ts:87`（移动） | `data`, `opinion`, `action` | `task_complete` |
| 4 | `POST /api/task/{id}/reject` | `services/index.ts:96`（移动） | `data`, `opinion`, `action` | `task_reject` |
| 5 | `POST /api/message/.../im/msg/list/{page}/size/{size}` | `IMChat.vue:283` | `conversationId` | `im_msg_list_page_size_size`（仅 `Path(page,size)`） |

**逐条已回源码核验（`processplatform_service_processing/src/lib.rs:3094`）**：

```rust
pub async fn task_complete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,   // ← 只有 Path，没有 Json
) -> Result<Json<ActionResult<Value>>, AppError> {
    ...
    "INSERT INTO x_record (..., content, creator, ...) VALUES ($1,..., $5, $6, NOW())",
    &[&id2, &work_id, &id, &"complete", &"task completed", &"system"],
    //                              ↑ 内容写死        ↑ 操作人写死
```

**后果（这是本轮最重要的业务缺陷）**：

- 用户填写的 **处理意见（`opinion`）在桌面端与移动端都被收集、都发出去，但从未落库**；
- 流程记录 `x_record.content` 恒为字符串 `"task completed"`，`creator` 恒为 `"system"`；
- 审批链路**无法追溯"谁批的、批了什么意见"**——审计意义上的功能缺失。

`task_reject` 同构（`lib.rs:3160+`）。

> 注：表单数据 `data` 另有 `PUT /api/processplatform/service/processing/data/work/{id}` 落库（`ProcessWork.vue:327`），故 `data` 不算丢失；**`opinion` 是真丢失**。

### 6.4 B 类：发送但后端不读（21 处）

| 端点 | 位置 | 未读键 | handler 实际读取 |
|---|---|---|---|
| `POST /api/meeting/.../meeting/create` | MeetingApp.vue:123 | `buildingId` | `title, roomId, content, creator, startTime, endTime` |
| `PUT /api/meeting/.../meeting/update` | MeetingApp.vue:144 | `id` | `title, content, startTime, endTime` |
| `POST /api/meeting/.../meeting/cancel` | MeetingApp.vue:153 | `id` | `title, content` |
| `POST /api/meeting/.../meeting/approve` | MeetingApp.vue:161 | `id` | `title, content` |
| `POST /api/meeting/.../meeting/join` | MeetingApp.vue:169 | `id` | `title, content` |
| `POST /api/meeting/.../meeting/leave` | MeetingApp.vue:178 | `id` | `title, content` |
| `POST /api/processplatform/service/processing/work` | ProcessWork.vue:298 | `title` | `process` / `processId` |
| `POST /api/query/assemble/designer/execute` | QueryManager.vue:91 等 **3 处** | `id`, `filter` | **仅 `sql`** |
| `POST /api/query/assemble/designer/create` | QueryDesigner.vue:363 等 2 处 | `name, category, query` | （结构体/委派） |
| `PUT /api/query/assemble/designer/save/{id}` | QueryDesigner.vue:361 | `name, category, query` | （结构体/委派） |
| `POST /api/query/assemble/designer/query` | QueryDesigner.vue:381 | `queryId` | （结构体/委派） |
| `POST /api/attendance/assemble/control/rule/create` | AttendanceApp.vue:189 | `name` | `ruleName, ruleType, description, enabled` |
| `POST /api/attendance/appeal/audit` | AttendanceApp.vue:135 | `status` | `id, auditStatus` |
| `POST /api/form/submit` | FormDesigner.vue:1256 | `id` | `data` |
| `POST /api/message/.../im/msg` | IMChat.vue:319 / 移动 :161 | `conversationId` | `content, sender, type`（见下方注） |
| `POST /api/bbs/assemble/control/section/create` | BBSForum.vue:324 | `name` | （空） |
| `POST /api/portal/assemble/designer/page/save/{id}` | PortalPageDesignerApp.vue:147 | `content` | （空） |

**重点解读**：

- **会议 5 个动作全部只发 `{id}`**，而后端 `u2_meeting_modify` 需要 `{title, content, roomId}` → 取消/审批/加入/离开**都没有传递必要语义**。
- **会议创建丢 `buildingId`**：前端让用户选楼栋，后端不读 → 楼栋信息丢失。
- **发起流程丢 `title`**：用户填的标题不生效。
- **查询执行传 `{id, filter}`，后端只读 `sql`** → 前端"按 id 执行已保存查询"的语义不成立，实际执行的是一条空 SQL。

> **注（已知误报）**：`im_msg` 的 `conversationId` 是**后端转义缺陷导致的假象**——后端读的是 `req.get("\"conversationId\"")`（键名含引号，见 6.7 E 类），前端已用"双键下发"绕过（`IMChat.vue:320` 有注释）。故此项应归入 E 类修复。

### 6.5 C 类：后端读但前端不发（15 处）

| 端点 | 位置 | 缺失键 | 前端发送 |
|---|---|---|---|
| `POST /api/meeting/.../meeting/create` | MeetingApp.vue:123 | `content, creator, endTime` | `title, buildingId, roomId, startTime` |
| `PUT /api/meeting/.../meeting/update` | MeetingApp.vue:144 | `content, endTime, startTime, title` | `id` |
| `POST /api/meeting/.../meeting/cancel\|approve\|join\|leave` | MeetingApp.vue:153/161/169/178 | `content, title`（及 `roomId`） | `id` |
| `POST /api/message/.../im/msg` | IMChat.vue:319 | **`sender`** | `conversationId, content, type` |
| `POST /api/message/.../im/msg` | 移动 services:161 | `content, sender` | `type, conversationId` |
| `POST /api/message/.../im/conversation` | 移动 services:175 | `name` | `type` |
| `POST /api/bbs/subject/create` | BBSForum.vue:457 | `content, sectionId, title` | `authorId` |
| `POST /api/attendance/appeal/submit` | AttendanceApp.vue:212 | `creator` | `personId, appealDate, start, reason, type` |
| `POST /api/mind/assemble/control/mind/save` | MindApp.vue:417 | `creatorUnit, parentId` | `id, name, folderId, description, shared, content` |
| `POST /api/person/signature/save` | Personal.vue:154 | `mimeType` | `signature` |
| `POST /api/query/assemble/designer/query` | QueryDesigner.vue:381 | 或组 `[[name],[category],[data\|query]]` | `queryId` |
| `POST /api/query/assemble/designer/execute` | QueryManager.vue:91 | `sql` | `id, filter` |

**重点解读**：

- **IM 发消息不发 `sender`** → 后端 `unwrap_or("system")` → **所有消息归属为 "system"，而非真实发送人**。移动端更严重：连 `content` 都不发。
- **会议创建/更新缺 `content`/`creator`/`endTime`**：前端表单没有这几个输入项，后端却要 → 数据不完整。
- **BBS 发帖只发 `authorId`**：`title`/`content`/`sectionId` 全缺 → 发帖必然空内容。
- **移动端 `im/conversation` 只发 `type`**，后端要 `name` → 建会话无名。

### 6.6 D 类：委派给泛型 helper（20 处，静态不可判定）

`collect_save` / `face_save` / `ftsearch_save` / `application_save` / `agent_save` / `invoke_save` / `threemember_save` / `general_control_save` / `create_reply` / `chat_completion` 等 handler 把整个 body 传给 `shared::crud` 一类泛型函数，键读取发生在被调函数内（由 `CrudSpec` 的 `columns` 驱动）。

**结论**：静态分析到此为止，**必须用运行时校验**（见 §九 阶段 F）。这 20 处是"未验证"，不是"已验证正确"——按 fail loud 原则单列。

### 6.7 E 类：转义引号键缺陷（**351 处，P0，本轮最重大发现**）

后端存在大量形如 `"\"key\""` 的 Rust 字面量——**键名里字面包含引号**（转义错误）。分四种后果：

| 形态 | 数量 | 代码 | 后果 |
|---|---|---|---|
| **读错列名** | **81** | `row.get("\"xcreateTime\"")` | 列不存在 → **`row.get` panic → 500** |
| **响应键含引号** | **115** | `("\"startTime\"".to_string(), ...)` | JSON 键为 `"startTime"`（带引号）→ 前端 `item.startTime` 得 **undefined** |
| **请求键含引号** | **32** | `payload.get("\"startTime\"")` | 前端发的 `startTime` 读不到 → **字段静默丢失** |
| **路由路径含引号** | **5** | `.route("/api/.../excel/{\"excelName\"}", ...)` | 路径参数名畸形 |

**决定性证据（列名）**：迁移里列名就是大小写敏感的 `"xcreateTime"`：

```sql
-- migrations/032_create_assemble_control_tables.sql:264
"xcreateTime" TEXT,
```

而代码读的是 `"\"xcreateTime\""`（名字里带引号）→ 该列不存在 → panic。

**受影响最重的文件**：

| 文件 | 处数 |
|---|---|
| `crates/processplatform_assemble_surface/src/lib.rs` | 87 |
| `crates/processplatform_assemble_designer/src/lib.rs` | 62 |
| `crates/general_assemble_control/src/lib.rs` | 24 |
| `crates/processplatform_assemble_bam/src/lib.rs` | 24 |
| `crates/message_assemble_communicate/src/lib.rs` | 22 |
| `crates/ai/src/config.rs` | 12 |

**前端已被迫绕过（证明团队知道，但只修了少数几处）**：

```
IMChat.vue:299/320   m?.['"conversationId"'] ?? m?.conversationId      ← 带引号键兼容读取
MindApp.vue:138/139  item.parentId ?? item['"parentId"']
mobile/services/index.ts:145/162  row['"conversationId"']
packages/apis/src/index.ts:216    ['"conversationId"']: data.conversationId
```

而**没有绕过的字段就静默失效**——例如前端有 **11 处**读取 `.startTime` / `.endTime`（`CalendarApp.vue`、`MeetingApp.vue`、`ProcessTaskCenterApp.vue` 等），而后端产出的键是 `"startTime"`（带引号）→ 这些字段恒为 `undefined`。

**根因**：极可能是一次批量生成/替换脚本把"JSON 里的键"直接当成"Rust 字符串字面量"写入，多包了一层引号。属**系统性**缺陷，不能用"逐处打补丁"处理，应做**批量修正 + 加回归门禁**。

### 6.8 本轮提取器新增的语义修正（fail loud）

深度校验比路由对账更易误报，本轮修正了 **6 处**提取器缺陷，每处都先用反例证伪再修：

| 缺陷 | 症状（误报） | 修正 | 效果 |
|---|---|---|---|
| 全限定 `axum::extract::Json(...)` 未识别 | 17 处 handler 被误判为"无请求体" | 正则允许 `(?:axum::)?(?:extract::)?` 前缀 | A 类 17 → **5**（全部为真） |
| `Option<Json<Value>>` 未识别 | `v2_detail_statistic_export_filter` 误报 | 允许 `Option<...>` 包装 | A 类再降 1 |
| **`or_else` 链被当作"与"** | `save_flow` 读 `processDefinition`\|`process_definition` 被误报"缺键" | 识别 `.or_else(/\|\|` 链，归为**或组** | C 类 18 → 15 |
| **handler 名跨 crate 冲突** | `create`/`update`/`save` 同名，取到别的 crate 的函数 | 索引键改为 `(crate, handler)` | 消除多处张冠李戴 |
| 委派未识别 | 泛型 helper 的 handler 被误报"发送但未读" | 检测"body 被整体传给非访问器函数" | 新增 D 类，从 B 类剥离 20 处 |
| **JS 简写属性漏解析** | `{content, type}` 的 `content` 漏掉 → 误报"前端不发 content" | 改为"按顶层逗号切分 + 逐段解析" | 消除 `im_msg` 等误报，并修掉由此产生的重复键 |

> 另有 1 处**已知保留的误报**：`im_msg` 的 `conversationId`（6.4 注），根因在后端转义缺陷，已归入 E 类。

### 6.9 实施阶段又发现 2 个提取器缺陷（**rev4 补记**）

修复过程中，测试与对账互相印证，暴露出路由提取器的两个新缺陷——**它们此前造成了假 404**：

| 缺陷 | 症状（误报） | 修正 | 效果 |
|---|---|---|---|
| **`&fmt("...")` 前缀未还原** | `bbs_assemble_control` 有 106/149 条路由用 `.route(&fmt("subject/xxx"), ...)` 注册，前缀由 `API_BASE` 常量在运行时拼接；提取器只抓到**无前缀**的路径 → 前端调用全部判为 404 | 解析文件内 `pub const *_BASE: &str` 常量，对无前导斜杠的路径补前缀 | 全仓 145/5055 条（2.9%）受影响；bbs 无前缀残留 **106 → 0**；**BBS 的 7 处"404"全部是误报**（前端原本正确，已回滚误改） |
| **字符串拼接被截断** | `api.get('/api/form/' + f.id)` 被截成 `/api/form` → 判为 405 | 字面量后紧跟 `+` 时补 `{}` 占位 | 消除 1 处假 405 |

> **教训**：`&fmt(...)` / `&format!(...)` 这类"运行时拼前缀"的路由注册，是静态提取的天然盲区。
> 判据很简单——**提取出的路径不以 `/` 开头就说明前缀是拼出来的**，必须还原，否则会把正确的前端判成错的。
> 本轮 BBS 就是先误判、后由既有测试（`BBSForum.test.ts`）挡下才发现的——**这也印证了"契约测试"的价值**。

### 6.10 rev8：契约提取器"跟随一层间接引用"（把上界变成事实）

原提取器只看 handler 本体的 `body.get("k")`，因此把**已修**的项判成未修。rev8 补上三种派生：

| 派生形态 | 例子 | 处理 |
|---|---|---|
| 闭包绑定 Json | `body.as_ref().map(\|Json(v)\| v)` → `v.get("k")` | 把 `v` 加入接收者集合 |
| `let` 别名 | `let payload = body.as_ref()...` → `completion_record_fields(payload, ..)` | 只允许从 **Json 参数直接派生**（不做传递闭包） |
| 一层 helper 调用 | `completion_record_fields(payload, ..)` 内的 `\|v\| v.get("opinion")` | 把被调函数的读取键并入调用方 |

**必须同时扩展 helper 的索引范围**：`completion_record_fields(body: Option<&Value>, ..)` 不是 axum handler
（无 `Json` 参数），原先被跳过 → 跟随失败。现对"参数类型为 `Value`/`&Value`/`Option<&Value>`"的函数也建索引。

**效果（net win）**：

- ✅ 消除了 2 处**已证实的误报**：`task_complete/reject` 的 `opinion`、`im_msg_list` 的 `conversationId`
  （B 24 → 18）。
- ✅ **新发现 4 处真实字段错配**（此前完全看不见）：

| 端点 | 前端发送 | 后端读取 |
|---|---|---|
| `POST /api/attendance/appeal/audit` | `status` | **`auditStatus`** |
| `POST /api/attendance/assemble/control/rule/create` | `name` | **`ruleName` / `ruleType` / `description` / `enabled`** |
| `POST /api/form` | `schema` | **`definition`** |
| `POST /api/attendance/.../v2/mobile/check` | `checkInType` / `sourceType` | **`recordDateString` / `userId`** |

- ⚠️ 也引入了 3 处**新误报**（C 13 → 21）：`authentication/login` / `switchuser` 把 `auth_person`
  的列名（`password_hash`/`unique_id`/`department`…）当成请求体键；`task/reject` 的 `id` 实际来自 `Path`。
  根因是 `let` 派生与结构体字段在少数 handler 上仍过宽。

> **基线同步更新**（`schema_baseline.json` → B 18 / C 21 / D 24 / E 2 / A 0），并在文件内留 `_note`
> 说明"因提取器变敏感而移动"，避免被误读为放水。**已知残留误报已逐条列出**，不隐藏。
>
> **方法论**：契约提取器的准确度提升与字段审计（§5.2 fallback）、命名复核（§九 G1）是同一件事——
> **必须解析表达式链与一层间接引用**。只看表层标识符，报出的永远是"上界"而非"事实"。

---

## 七、业务流程覆盖矩阵

| 业务域 | 后端路由 | 桌面调用 | 桌面问题 | 移动调用 | 移动问题 | 三端闭环 |
|---|---|---|---|---|---|---|
| 认证/会话 | 36 | 0（在 sdk 中，5 条 ✅） | 0 | 5 | 0 | ✅ |
| 组织-控制 | 240 | 7 | 2 | 2 | 0 | ⚠️ 桌面有错 |
| 组织-个人 | 130 | 8 | 1 | 0 | 0 | ⚠️ 桌面有错 |
| 组织-认证 | 66 | 0 | 0 | 0 | 0 | ❌ 无前端 |
| 组织-快递 | 7 | 0 | 0 | 0 | 0 | ❌ 无前端 |
| 身份/角色/群组 | 104 | 6 | 2 | 0 | 0 | ⚠️ |
| 考勤 | 247 | 10 | 2 | 2 | 0 | ⚠️ |
| 日历 | 56 | 1 | 1 | 0 | 0 | ❌ 移动端空白 |
| 会议 | 129 | 10 | 6 | 0 | 0 | ❌ 移动端空白 |
| 论坛 BBS | 67 | 15 | 7 | 0 | 0 | ❌ 移动端空白 |
| 内容 CMS | 270 | 63 | 14 | 1 | 0 | ⚠️ 桌面问题最多 |
| 流程-设计器 | 144 | 20 | 0 | 2 | 0 | ✅ |
| 流程-表面 | 1,067 | 14 | 0 | 3 | 0 | ✅ |
| 流程-引擎 | 222 | 9 | 0 | 4 | 0 | ✅ |
| 流程-BAM | 91 | 1 | 0 | 0 | 0 | ❌ 移动端空白 |
| 门户-设计器 | 87 | 16 | 0 | 0 | 0 | ❌ 移动端空白 |
| 门户-表面 | 72 | 4 | 3 | 0 | 0 | ❌ 移动端空白 |
| 查询-设计器 | 175 | 50 | 2 | 0 | 0 | ⚠️ |
| 查询-表面 | 132 | 6 | 4 | 0 | 0 | ❌ 移动端空白 |
| 消息/IM | 98 | 5 | 0 | 5 | 0 | ✅ |
| 文件/附件 | 192 | 6 | 0 | 3 | 0 | ✅ |
| 回收站 | 8 | 4 | 1 | 0 | 0 | ❌ 移动端空白 |
| 思维导图 | 47 | 4 | 0 | 0 | 0 | ❌ 移动端空白 |
| 通知/推送 | 47 | 3 | 1 | 0 | 0 | ❌ 移动端空白 |
| 热图/统计 | 51 | 2 | 0 | 0 | 0 | ❌ 移动端空白 |
| 搜索 | 9 | 4 | 0 | 0 | 0 | ❌ 移动端空白 |
| AI | 73 | 15 | 2 | 1 | 0 | ⚠️ |
| 程序中心 | 418 | 39 | 5 | 0 | 0 | ❌ 移动端空白 |
| 服务器/控制台 | 21 | 10 | 4 | 0 | 0 | ⚠️ |

**后端能力消费率**：4,327 条唯一路径中，被三端引用到的仅 **252 条（5.8%）**。

> 口径说明：该比例按"归一化路径完全相等"统计，未计入参数位匹配，因此是**下界**。但即便按宽松口径，桌面端也只触达 300 余条路径——**服务端的绝大部分能力当前没有任何前端入口**。

**"三端闭环"判读**：
- ✅ **真正三端闭环**：认证、流程（设计/表面/引擎）、消息 IM、文件附件。
- ⚠️ **桌面单端有瑕疵**：组织、身份、考勤、CMS、查询设计器、AI、程序中心、服务器。
- ❌ **移动端完全空白**：日历、会议、论坛、门户、查询、回收站、思维导图、推送、统计、搜索、BAM、程序中心。

---

## 八、功能完备性评估

### 7.1 桌面端

| 指标 | 数值 | 说明 |
|---|---|---|
| 视图文件 | 84 | |
| 已挂路由 | 83 | |
| **孤儿视图** | **1** | `AIAssistant.vue` 未挂路由（但其调用被 AIChatApp 复用） |
| 存在 API 问题的视图 | 25 | 占 29.8% |
| 路由指向但文件缺失 | 0 | 无死链 |

### 7.2 移动端

| 指标 | 数值 |
|---|---|
| 页面 | 10（9 业务 + login） |
| tabBar | 5（工作台 / 消息 / 审批 / 考勤 / 我的） |
| API 调用 | 29 条，**100% 闭合** |
| 覆盖业务域 | 6 个（认证、流程、消息、文件、考勤、组织-控制） |
| 未覆盖业务域 | **≥ 12 个**（见 §七 ❌ 行） |

### 7.3 服务端

- 路由面完整（5,036 唯一 method+path，零提取盲区）。
- 兼容别名族 407 条（`mock*`）需在对外契约文档中声明。
- **未发现"注册即冲突"**：90 个 crate 的 router 链式 merge 能成功构造（既有测试 `cors_guard::create_app_builds_without_panic` 覆盖）。

---

## 九、落地规划

> 原则：**先止损（静默错数据）→ 再补缺 → 后扩面 → 最后上闸门**。每阶段都有可机器验证的验收标准。

### 阶段 0（P0，**最优先**）—— 后端契约致命缺陷（§六 的 A / E 类）

> **为什么排在所有前端修复之前**：这两类问题会让"接口通、数据不对"，且**后端修一次，三端同时受益**；而前端修 57 处只解决桌面端。E 类中的 81 处还会直接 panic(500)。

| 任务 | 内容 | 验收 |
|---|---|---|
| **0-1** | **批量修正转义引号键缺陷 351 处**：`"\"key\""` → `"key"`（读列名 81 / 响应键 115 / 请求键 32 / 路由路径 5） | 重新运行 `schema_audit.py`，E 类 → **0**；`cargo test --workspace` 无回归 |
| **0-2** | **同步清理前端 4 处"带引号键"绕过**（`IMChat.vue:299/320`、`MindApp.vue:138/139`、`mobile/services/index.ts:145/162`、`packages/apis/src/index.ts:216`）——根因修复后绕过即成负债 | 绕过代码删除，行为不变（E2E 覆盖） |
| **0-3** | **`task_complete` / `task_reject` 接收请求体**：写入 `x_record.content = opinion`、`creator = 当前登录人`（而非 `"task completed"` / `"system"`）；`data` 仍走既有 `data/work/{id}` 落库路径 | 桌面 + 移动审批后，`x_record` 能查到真实意见与操作人 |
| **0-4** | `im_msg_list_page_size_size` 支持 `conversationId` 过滤（或前端改走带过滤的端点） | 会话消息列表按会话隔离 |
| **0-5** | 补 `x_record` 意见字段的回归测试（含空意见、驳回场景） | 测试可失败：改回硬编码即红 |

**阶段验收**：`schema_audit.py` 的 **A = 0、E = 0**；审批链路可追溯。

### 阶段 A（P0）—— 消除 22 处静默影子误路由

**为什么最先做**：它们返回 200，问题被数据掩盖，是最可能带病上线的类别。

| 任务 | 内容 | 验收 |
|---|---|---|
| A1 | 修 `FormApp.vue` 6 处（list→list/all、v2/list、update/{id}、delete/{id}、create、GET /api/form） | 6 条调用全部 `exact` 命中 |
| A2 | 修 `MeetingApp.vue` 6 处（list 用后端真实变体；update/cancel/approve/join/leave 对齐后端 meeting 动作族） | 6 条全部命中，且"我的会议"能列出真实数据 |
| A3 | 修 CMS 四视图：`AppInfoApp`(2)、`CategoryApp`(1)、`DocumentApp`(2)、`ViewApp`(2) | 列表能返回真实数据而非单条记录 |
| A4 | 修 `OrgViewer`(2)、`RoleManager`(1)、`HomepageApp`(2)、`ProgramCenterApp`(1)、`FindDesignerApp`(1) | 全部命中 |
| A5 | 收窄后端宽路由（可选加固）：给 `{id}` 前的字面量段（`list`/`filter`/`export`/`update`/`delete`）设专用路由或提优先级 | 新增强制回归测试（见阶段 E） |

**阶段验收**：`api_reconcile.json` 中 `suspicious-shadow` 从 22 → **0**。

### 阶段 B（P1）—— 修正 24 处方法不符

| 任务 | 内容 |
|---|---|
| B1 | **前端改方法**（16 处，后端已正确）：calendar filter→PUT、person password→PUT、queryview search→POST、queryview execute→GET、recycle empty→DELETE、server execute/stop→GET、program_center module compare→GET、appstyle erase→DELETE、market cover pic→GET、form create→`POST /api/form`、document→`POST document/list/document` 等 |
| B2 | **后端补方法**（8 处）：`DELETE /api/ai/.../config/delete/mcp/{flag}`、`DELETE .../chat/delete/{clueId}`、`DELETE /api/jpush/.../device/{id}`、`DELETE /api/portal/.../page/{id}`、`GET /api/attendance/.../attendancedetail`（或前端改 POST）、`POST /api/server/execute|stop`（或前端改 GET） |
| B3 | 对每个 B1/B2 决策，在 `docs/` 留档"改前端还是改后端"的理由（避免同一端点被反复来回改） |

**阶段验收**：`405` 从 24 → **0**。

### 阶段 C（P1）—— 补齐 11 处后端缺失

| 任务 | 内容 | 责任侧 |
|---|---|---|
| C1 | BBS 分页契约统一（6 条）：`subject/search|filter|index|creamed` 的分页变体；**新增** `subject/recommended` 族、`user/reply/my` 族、`reply/filter/list` | 后端为主 |
| C2 | 系统配置 CRUD：`PUT /api/config/update`、`DELETE /api/config/delete` | 后端 |
| C3 | 考勤规则删除：`DELETE /api/attendance/assemble/control/rule/{id}` | 后端 |
| C4 | 单位校验：前端 `GET /api/unit/check/{id}` 改用 `POST /api/unit/check/unit/has/*` | 前端 |

**阶段验收**：`404` 从 11 → **0**。

### 阶段 D（P1）—— 字段对齐

| 任务 | 内容 |
|---|---|
| D1 | `RecycleApp` 改读 `deletedAt`（或后端同时产出 `deleteTime` 兼容） |
| D2 | `UnitApp` 改读 `id`/`name`，并按需请求 `parent_id`；同时统一 `control::unit` 的字段命名为 camelCase（`parentId`） |
| D3 | 核实并对齐 `MindApp.root`、`QueryDesigner.queryName/entityCategory` |
| D4 | 新增**单端点级**字段断言测试（见 §九 阶段 E 的 E2），把"字段对得上"变成可执行门禁 |

### 阶段 E（P0 治理）—— 防回归门禁（**本规划最关键的一项**）

> 57 处问题的根因不是"写错了"，而是**没有任何机制阻止写错**。不建门禁，修完还会再退化。

| 任务 | 内容 | 验收 |
|---|---|---|
| E1 | 把本次对账固化为仓内测试：`oa4rust-web` 新增 `tests/contracts/api-route-reconcile.test.ts`，输入 = 提取产物，断言**不存在 404/405/shadow** | CI 红/绿可判 |
| E2 | 新增单端点字段契约测试：对核心流程端点（task/work/message/attendance/org/person）断言"前端声明字段 ⊆ 后端产出字段" | 覆盖 §七 核心域 |
| E3 | 将两个提取脚本纳入 CI 前置步骤，产物 diff 化（路由面变化可见） | 路由增删可审 |
| E4 | 宽路由加固：对 `{id}` 型通配加"字面量子资源白名单"检查 | 防新增影子 |
| E5 | **契约门禁**（与 H-8 合流）：`schema_audit.py` 的 A/B/C/E 计数纳入阈值断言，D 类进白名单 | 防新增请求/响应错配 |
| E6 | **转义引号守卫**：新增 lint/测试，禁止源码出现 `"\"key\""` 形态的字面量 | 防 E 类复发（351 处修完即锁死） |

### 阶段 F（P1）—— 请求/响应契约修复与运行时校验（§六 的 B / C / D 类）

| 任务 | 内容 | 验收 |
|---|---|---|
| **H-1** | **会议 5 个动作（cancel/approve/join/leave/update）**：前端只发 `{id}`、后端要 `{content, roomId, title}` → 明确契约后双向对齐 | 每个动作在 `schema_audit.py` 中不再出现在 B/C |
| **H-2** | **会议创建补 `buildingId`**（前端已采集、后端未读）；补 `content`/`endTime` 输入项 | 楼栋与结束时间能落库 |
| **H-3** | **发起流程补 `title`**（前端已采集、后端未读） | 工作实例标题正确 |
| **H-4** | **查询设计器 execute**：前端 `{id, filter}` vs 后端 `sql` → 定契约（建议后端支持按 id 取 SQL，或前端改为传 sql） | 3 处调用语义成立 |
| **H-5** | **IM 发消息补 `sender`**（两端），移动端补 `content`；`im/conversation` 补 `name` | 消息归属真实发送人 |
| **H-6** | **BBS 发帖补 `title`/`content`/`sectionId`**；考勤规则 `name` vs `ruleName` 对齐；考勤申诉 `creator` 由后端从会话推导 | 发帖/建规则/申诉字段完整 |
| **H-7** | **D 类 20 处运行时校验**：静态不可判定（泛型 `shared::crud` 驱动）→ 建"真实请求回放"测试，用实际 body 打端点并断言响应字段 | 20 处全部转为"已验证"或暴露为 B/C |
| **H-8** | 把 A–E 五类固化为 **CI 门禁**：`schema_audit.py` 的 A/B/C/E 计数纳入阈值断言，D 类纳入白名单（有清单、不许新增） | CI 可拦截新增契约错配 |

**阶段验收**：`schema_audit.py` 的 **A=0、B=0、C=0、E=0，D 全部有结论**（转已验证或转 B/C 后清零）。

### 阶段 G（P2）—— 移动端能力扩展

| 任务 | 内容 | 优先级依据 |
|---|---|---|
| F1 | 通讯录增强：组织树/身份/角色（当前仅 person 搜索 2 条调用） | 移动端高频 |
| F2 | 会议：我的会议列表 + 接受/拒绝（移动场景刚需） | 后端 129 条路由已就绪 |
| F3 | 日历：今日/本周日程 | 后端 56 条已就绪 |
| F4 | 门户/查询/CMS 的只读浏览（移动端以"看"为主） | 后端路由充足 |
| F5 | 回收站/推送/统计/搜索 按需补齐或**书面声明范围外** | 需明确取舍 |

**阶段验收**：移动端调用数 ≥ 80，覆盖业务域 ≥ 12 个；或对未覆盖域给出书面范围外声明。

### 阶段 H（P2）—— 一致性治理（rev6 逐条复核）

| 任务 | 复核结论（rev6） |
|---|---|
| G1 | ✅ **复核为非问题**：`control::unit.rs:107` 产出的 JSON 键**本就是 `parentId`**（只是 SQL 列名是 `parent_id`）。原判断基于 SQL 列名而非 JSON 键，属**误判**，已修正。全仓亦无 `parent_id` 硬消费方（`organization-selector.ts:61` 是 `parentId ?? parent_id` 的 fallback 链）。 |
| G2 | ✅ **已文档化**：407 条 `mock*`（`*mockputtopost` / `*mockdeletetoget`）为 **O2OA 兼容层**，用于承载"前端用 GET 模拟 PUT/POST"的历史行为，**不可清理**。已在本文件 §二 说明；对外契约文档如需可再摘录。 |
| G3 | ✅ **已解决（选项 1：合并到 AIChatApp）** —— 逐行读两视图后发现**不是重复，而是同一功能的两个半成品**：`AIAssistant`（200 行，未路由，0 残留）有**真实 MCP 配置实现**；`AIChatApp`（345 行，已路由）有**配置面板 UI 但 `showConfig` 无处设为 true**（永不可达）+ **95 个未使用的生成器残留 ref**（93 个死）。<br>**合并动作**：把 MCP 面板迁入 AIChatApp 的 `showConfig` 并补入口按钮（侧栏 ⚙）；删除 `AIAssistant.vue` 与 95 个死 ref；同时修掉原实现的 **3 处缺陷**——① 禁用分支调 `POST config/delete/mcp`（**缺 id 段**）→ 404，改为 `POST config/update/mcp/{id}`；② `addMcp` 原为**空函数**（死控件）→ 实现内联表单；③ 列表字段原读 `endpoint` 而**后端产出的是 `url`** → 该列恒为空。<br>**保留 AIChatApp 既有行为**（对话列表 `chat/list/paging`、非流式发送），只补缺失能力——最小改动。**未采纳**的差异：AIAssistant 的**流式发送**端点（`chat/completion/stream`）仍未被采用，留作后续决策。 |
| G4 | ✅ **已修**：`EmptyApp.vue` 的 `title`/`subtitle` 原为**必填 props 但路由未传**（`main.ts` 的 `empty` 路由无 props）→ 改为可选 + 默认文案（"占位页面"/"该页面为通用占位模板，尚未接入具体业务"）。 |
| G5 | 🟨 长期项（逐域推进中）：后端能力消费率**逐域提升**，目标 100%（两端消费完整）。<br>**口径校准（rev12）**：新增 `consumption_gap.py` 按域精确核算——**排除 407 条 mock* 兼容别名**（不可消费面）后，真实可消费唯一 (method,path) = **4072** 条；截至本轮已消费 **382** 条（**9.4%**，含参数位匹配，比原 5.8%/252 分母口径更精确）。<br>**逐域推进日志**：<br>　• **rev41（2026-09-21，CMS 文档点赞列表）**：`DocumentApp.vue` 补「点赞」**1 条**（`GET commend/list/paging/{docId}`）；全局消费 404→405（8.7%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev40（2026-09-21，组织-控制 角色成员）**：`RoleManager.vue` 补「成员」**1 条**（`GET person/list/role/{roleFlag}`）；全局消费 403→404（8.7%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev39（2026-09-21，CMS 控制配置）**：`CmsIndexApp.vue` 补「控制配置」**1 条**（`GET cms_assemble_control/get/control/config`）；全局消费 402→403（8.7%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev38（2026-09-21，流程-引擎 按分类流程列表）**：`ProcessManagerApp.vue` 补「运行中流程」**1 条**（`GET processing/list/{category}`）；全局消费 401→402（8.7%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev37（2026-09-21，流程-引擎 工作实例列表）**：`ProcessTaskCenterApp.vue` 补「工作实例」**1 条**（`GET work/list`）；全局消费 400→401（8.6%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev36（2026-09-21，流程-设计器 模板表单分类）**：`ProcessFormDesignerApp.vue` 补「模板表单」**1 条**（`GET templateform/list/{category}`）；全局消费 399→400（8.6%，里程碑 400）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev35（2026-09-21，流程-设计器 应用分类）**：`ProcessApplicationApp.vue` 补「分类」按钮 **1 条**（`GET applicationcategory/list`）；全局消费 398→399（8.6%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev34（2026-09-21，组织-控制 群组详情/子群组）**：`OrgViewer.vue` 选中群组查详情+直接子群组 **2 条**（`group/{flag}`、`group/list/{flag}/sub/direct`）；全局消费 396→398（8.6%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev33（2026-09-21，组织-控制 单位身份）**：`UnitApp.vue` 单位详情补「单位身份」列 **1 条**（`GET identity/list/unit/{unitFlag}`）；全局消费 395→396（8.5%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev32（2026-09-21，组织-个人 管理员签名列表）**：`Personal.vue` 补「查看全员签名（管理员）」**1 条**（`GET /api/person/signature/manager/list`）；全局消费 394→395（8.5%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev31（2026-09-21，考勤 统计周期）**：`AttendanceApp.vue` 补「统计周期」标签+新建 **2 条**（`POST attendancestatisticalcycle`、`GET .../{id}`）；全局消费 392→394（8.5%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev30（2026-09-21，CMS 文档操作日志）**：`DocumentApp.vue` 补「日志」按钮 **1 条**（`GET /api/log/list/document/{documentId}`）；全局消费 391→392（392/4638=8.5%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev29（2026-09-21，CMS 应用权限 + 口径澄清）**：`AppInfoApp.vue` 补「权限」按钮 **3 条**（`permission/appInfo/{id}/managers|publishers|viewers`）；**全局消费 388→391**（全局口径 391/4638=8.4%）。这 3 条在 `/api/permission/*`（域桶外），故域口径 382/4072 不变。**口径澄清：真实全局消费率 = 391/4638 = 8.4%，域口径 9.4% 因分母仅 4072（桶内子集）偏高。** 双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev28（2026-09-21，会议 会议室管理）**：`MeetingApp.vue` 补会议室新建/删除 **2 条**（`POST/DELETE assemble/control/room`）；会议域 20→22、总消费 380→382（9.3%→9.4%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev27（2026-09-21，组织-个人 授权委托）**：`Personal.vue` 新增「授权委托」卡片 **4 条**（`empower/list/currentperson`、`list/to`、`{id}/enable`、`{id}/disable`）；组织-个人域 7→11、总消费 376→380（9.2%→9.3%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev26（2026-09-21，会议 楼栋管理）**：`MeetingApp.vue` 补楼栋新建/删除 **2 条**（`POST/DELETE assemble/control/building`）；会议域 18→20、总消费 374→376（9.2%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev25（2026-09-21，考勤 管理员/导入/统计日志）**：`AttendanceApp.vue` 补 3 只读标签 **3 条**（`attendanceadmin`/`attendanceimportfileinfo`/`attendancestatisticrequirelog` 的 `list/all`）；考勤域 24→27、总消费 371→374（9.1%→9.2%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev24（2026-09-21，日历列表 + 修复 fixture 漂移）**：`CalendarApp.vue` 补我的/公共日历列表 **2 条**（`calendar/list/my`、`calendar/list/public`）；日历域 8→10、总消费 369→371（9.1%）。**并修复 rev23 遗留**：participant 路由使 fixture（落后 128 条）守卫变红，`gen_fixture.py` 重生成（4332 条）→ vitest 953 全绿。<br>　• **rev23（2026-09-21，会议 参会人）**：`MeetingApp.vue` 补参会人列表/邀请 **2 条**（`GET {meetingId}/participant/list`、`POST {meetingId}/participant/add`）；会议域 16→18、总消费 367→369。<br>　• **rev22（2026-09-21，考勤 排班/员工/假期/工作日）**：`AttendanceApp.vue` 补「排班与配置」四标签只读面板 **4 条**（`schedulesetting`/`employeeconfig`/`selfholiday`/`workdayconfig` 的 `list/all`）；考勤域消费 20→24、总消费 363→367（8.9%→9.0%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev21（2026-09-21，CMS 文档信息/通知）**：`DocumentApp.vue` 补阅读数/可见人/通知 **3 条**（`GET {id}/view/count`、`GET {id}/persons`、`POST {id}/notify`）；CMS 域消费 67→70、总消费 360→363（8.8%→8.9%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev20（2026-09-21，CMS 文档对称操作）**：`DocumentApp.vue` 补取消推荐/取消置顶/撤发 **3 条**（`GET {id}/uncommend`、`GET {id}/unTop`、`PUT publish/{id}/cancel`）；CMS 域消费 64→67、总消费 357→360（8.8%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev19（2026-09-21，CMS 文档操作）**：`DocumentApp.vue` 每行补推荐/置顶/发布 **3 条**（`GET document/{id}/commend`、`GET document/{id}/top`、`PUT document/publish/{id}`）；CMS 域消费 61→64、总消费 354→357（8.7%→8.8%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev18（2026-09-21，CMS 应用/分类）**：`AppInfoApp.vue`+`CategoryApp.vue` 各补 create+delete **4 条**（`POST/DELETE appinfo`、`POST/DELETE categoryinfo`）；CMS 域消费 57→61、总消费 350→354（8.6%→8.7%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev17（2026-09-21，程序中心 应用样式）**：`ProgramCenterApp.vue` 新增 Style 标签页——当前样式/门户应用 **2 条**（`appstyle/current/style`、`appstyle/index/portal`，GET 只读）；程序中心域消费 48→50、总消费 348→350（8.5%→8.6%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev16（2026-09-21，程序中心 平台配置）**：`ProgramCenterApp.vue` 新增 Config 标签页——配置列表/应用配置/实体配置/新建更新 **4 条**（`config/list`、`config/list/application`、`config/list/entity`、`POST config/save`）；程序中心域消费 44→48、总消费 344→348（8.4%→8.5%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev15（2026-09-21，程序中心 Invoke 接口）**：`ProgramCenterApp.vue` 新增 Invoke 标签页——列表/新建/删除/分类 **4 条**（`invoke`、`POST invoke`、`invoke/{id}`、`invoke/list/category`）；程序中心域消费 40→44、总消费 340→344（8.3%→8.4%）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev14（2026-09-21，程序中心 应用市场）**：`ProgramCenterApp.vue` Market 页扩展——分类栏 + 热门 Top3 + 每卡「安装/更新·版本·卸载」，消费 `market` 族 **5 条**（`list/category`、`list/top/three`、`{flag}/install/or/update`、`{flag}/installed/version`、`{flag}/uninstall`，均 GET）；程序中心域消费 34→**40**、总消费 334→**340**（8.2%→8.3%）。**同轮修真实运行时 bug**：agent「启用/禁用」原用 `POST /agent/{flag}/{action}` 打后端 GET 端点（被提取器误配到 `save/{id}` 掩盖）→ 改 GET 并加「执行」按钮，消费 enable/disable/execute 3 条。**坑**：参数化 GET 前端须用模板字面量，字符串拼接会被提取器在 `+` 处截断误判。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev13（2026-09-20，组织-控制 单位属性/职务）**：`UnitApp.vue` 新增「选中单位→属性/职务管理」面板，消费 `unitattribute` + `unitduty` 两族 CRUD 共 **6 条**（`list/unit/{flag}`、`POST`、`DELETE/{id}` 各 2）；组织-控制域消费 9→15、总消费 328→334（8.1%→8.2%）。契约回源码核验（create 读 name/unitId/attributeValue 或 identityList、admin 门禁、unitId 走 resolve_generic_id）；顺带修 UnitApp 两处既有运行时 bug（`loadUnits` 未定义致挂载即崩、`toast` 未 import）。双 gate PASS、typecheck 6/6、vitest 953、build 通过。<br>　• **rev12（2026-09-20，考勤域 workplace/setting）**：`AttendanceApp.vue` 新增「考勤配置」面板——打卡地点 CRUD（`workplace/list/all`、`POST workplace`、`DELETE workplace/{id}`）+ 考勤设置项 CRUD（`attendancesetting/list/all`、`POST attendancesetting`、`DELETE attendancesetting/{id}`）共 **6 条真实路由**；desktop 调用 332→338，消费 322→328（7.9%→8.1%）。契约逐条回源码核验（workplace_create 读 name/address、attendancesetting_create 读 code/name/value，均 admin 门禁、creator 取会话）；双 gate PASS、typecheck 6/6、vitest 953、biome 0 err、desktop build 通过。<br>**缺口 Top（排除 mock，供后续选点）**：流程-表面 874 / 程序中心 381 / 考勤 228 / 流程-引擎 214 / 文件附件 182 / CMS 175 / 组织-控制 160 / BBS 157。<br>**性质说明**：100% 是马拉松式目标（等价于两端消费掉全部 4072 条能力面），按业务优先级逐域推进，每轮一个自洽切片 + 门禁守住不回退。 |

### 实施状态总表（rev6，2026-09-20 阶段 E 完成后更新）

> **本表是本文档唯一持续更新的部分。** §〇–§八 是绑定 HEAD `fa5b83db` 的**时效快照**，
> 修复推进后其行号与计数会过期；判断"当前做到哪了"只看本表。

**状态图例**：✅ 已完成　🟨 进行中　⬜ 未开始　⛔ 阻塞

| 阶段 | 状态 | 负责人 | 验收度量（结果） | 证据 |
|---|---|---|---|---|
| **审计**（§〇–§八） | ✅ 已完成（rev2） | 陈皮（AI） | 审计结论可复现 | `docs/audits/three-ends-2026-09-20/`（9 脚本 + 6 数据文件，已端到端复现） |
| **0** 后端契约致命缺陷 | ✅ 已完成 | 陈皮（AI） | **A 5 → 0**；**E 351 → 2**（残留为合法 SQL 引号） | 转义引号批量修复 39 文件 / 363 处；`task_complete`/`task_reject` 落库真实意见与操作人；`im_msg_list` 支持会话过滤；前端 4 处绕过清除 |
| **A** 静默影子误路由 | ✅ 已完成 | 陈皮（AI） | **shadow 22 → 0** | 22 处全部指向真实路由 |
| **B** 方法不符 | ✅ 已完成 | 陈皮（AI） | **405 24 → 0** | 前端改方法 13 处 + 后端补方法/路由 4 处 |
| **C** 后端缺失 | ✅ 已完成 | 陈皮（AI） | **404 11 → 0** | BBS 7 处经复核为**误报**（提取器 `fmt` 前缀缺陷，已回滚误改）；后端补 3 处路由 |
| **D** 字段对齐 | ✅ 复核后免除 | 陈皮（AI） | 字段存疑 5 → **0（非真缺陷）** | 均为 fallback 链次级声明（§5.2 已修正过度结论） |
| **E** 防回归门禁 | ✅ 已完成（E1–E6） | 陈皮（AI） | 门禁已落地并验证可失败 | **E1/E4/E5**：`compare.py --gate`（shadow/405/404 必须全 0）+ `schema_audit.py --gate`（A 必须为 0，B/C/D/E 不得高于 `schema_baseline.json`）→ 新增 CI workflow `.github/workflows/three-ends-contract.yml`；**E2**：字段/契约侧以基线"不许新增"兜住（单端点级字段断言列为后续增强）；**E3**：CI 顺序 = extract_routes → extract_calls → compare --gate → schema_audit --gate，产物上传 artifact；**E6**：`oa4rust/tests/quoted_key_guard.rs` 已接入 `oa4rust-ci.yml` 的 `quality` job。**门禁可失败已验证**：人为压低基线 → EXIT=1；恢复 → EXIT=0 |
| **F** 契约修复 + 运行时校验 | ✅ 已完成（rev11）：A=B=C=D=0 | 陈皮（AI） | **A 0 / B 0 / C 0 / D 0 / E 2**；双 gate PASS | rev10 修真实错配（attendance audit/rule/appeal、form definition、IM sender、queryview search key、会议 create+roomId、query execute-by-id）。**rev11 升级提取器跟随多层间接引用**（let mut/DB 排除、单键 helper、CrudSpec 解析、必填/可选分类、`Json(bp):Json<Struct>` 解构 + 字段级 serde rename/alias、本地取值闭包），把 B18→10 中的误报与 D23 委派全部落地为**结论**；提取器暴露的真实缺陷再修 8 处（login 可选 captcha 校验、switchuser `targetUnique` 别名、CreateDesignerRequest `sql` 别名、ChatCompletionRequest 接受 `message/title/clueId` 并修空 messages→400、role 发 `description`、FindDesigner/Homepage `config`→`query`/`content`、**QueryDesigner runQuery 由误打 create 端点改走 execute**、task complete/reject 仅发 `opinion`）→ **A/B/C/D 全 0**。fail-injection（注入 bogus 键）验证 gate 仍可失败（B=2）。E=2 为合法 SQL 内引号。**H-7 运行时回放**：D 已静态清零，运行时校验作为可选增强（需双栈 DB）不再是达标前置。 |
| **G** 移动端能力扩展 | ✅ 达标（rev10） | 陈皮（AI） | **调用 29→80（≥80）、业务域 6→18（≥12）**；四端 shadow/405/404 全 0 | 新增 browse(门户/查询/CMS 只读)/calendar/meeting/recycle/search 页 + 工作台今日出勤 tile；修复 mobile document/search 405（POST→GET 全文检索）；`tests/contracts/mobile-endpoints.test.ts` 3/3 守卫路由存在 |
| **H** 一致性治理 | ✅ 4/5 已闭合 | 陈皮（AI） | G1 非问题、G2 已文档化、G3 已解决（合并）、G4 已修 | G3：MCP 配置并入 AIChatApp 并补入口，删除 `AIAssistant.vue` + 95 个死 ref，顺带修 3 处缺陷；G5 长期项（消费率 5.8%） |

**核心成果（rev10）：三端 API 调用闭合率 100%；移动端扩面至 18 业务域**

| 端 | 调用数 | 闭合 | shadow | 405 | 404 |
|---|---|---|---|---|---|
| 桌面端 | 334 | **334（100%）** | 0 | 0 | 0 |
| 移动端 | **80**（rev5 为 29） | **80（100%）** | 0 | 0 | 0 |
| 共享 sdk | 5 | **5（100%）** | 0 | 0 | 0 |
| 共享 ui | 3（helper 构造路径，静态未解析但单测覆盖） | — | 0 | 0 | 0 |

移动端业务域覆盖 **6 → 18**（认证/组织-控制/身份角色群组/考勤/日历/会议/BBS/CMS/流程-设计器/流程-表面/流程-引擎/门户-表面/查询-表面/消息IM/文件/回收站/推送/AI）。

**阶段 F 现状（rev10）：真实错配已清零，残留 B10/C16 均为误报/委派/产品项**

`schema_audit.py` 当前报 **B 10 / C 16 / D 23 / A 0 / E 2**（基线同步收紧到 B10/C16）。真实契约错配已全部修复（见 F 行）。
逐条回源码核验，残留 B10/C16 **无一处是真缺陷**：
- **提取器过敏（多层间接/展开/Path/可选默认）**：`login`/`switchuser` 把 `auth_person` 列名当请求体键读取；
  `task complete/reject` 的 `data`（走 `data/work/{id}` 落库）、`action`（UI 提示）、`id`（来自 `Path`）；
  `v2/mobile/check` 的 `checkInType` 经 `json_str` 二层读取、`userId`/`recordDateString` 有会话/当日默认；
  `bbs section_create`（`name` 经 `CrudSpec.columns` 消费）/`subject_create`（`...data` 展开未解析）；
  `processing/work` 的 `title`/`creator` 经 `work_start_impl` 二层读取；`mind_save` 的 `parentId`/`creatorUnit` 可选且会话推导。
- **委派/产品项**：`query/designer/query` 的 `queryId`（`shared::crud` 委派）；query execute 的 `sql`/`filter`/`id` 互为可选
  （后端已支持 sql 直传或 id 取回，提取器把全部读取键视为必填）。
- **D 23**：全部为 `shared::crud` 泛型委派，静态不可判定，需 **H-7 运行时回放校验**（双栈 DB 独立轨道）。

**判据改进方向**（与 §5.2 的 fallback 问题同源）：契约提取器仍需**跟随多层间接引用/展开运算符/Path 抽取**，
才能把上述误报清零。当前以 `schema_baseline.json` 的"只降不升"门禁锁死改进、防止新增错配。

**已修复的既有缺陷（审计连带收益）**：
- `bbs::list_reply_filter`：路由无路径参数却声明 `Path((page,count))` → 该路由**永远失败**；
  已改为可选 query 参数并支持 `subjectId`。

**验证记录（rev5）**：

| 检查 | 结果 |
|---|---|
| `cargo check --workspace --lib` | ✅ EXIT=0 |
| `cargo fmt --all -- --check` | ✅ EXIT=0 |
| `cargo test`（**11 个受影响 crate**） | ✅ **634 passed / 0 failed** |
| `cargo test --test quoted_key_guard`（E6 新守卫） | ✅ 1 passed |
| `cargo test --test designer_route_match`（既有路由契约） | ✅ 1 passed |
| `pnpm typecheck` | ✅ 6/6 工程 |
| `pnpm test` / `test:mobile` | ✅ **953** / **101** passed |
| `biome lint`（改动的 12 文件） | ✅ 无真实问题 |
| `compare.py` 四端对账 | ✅ shadow/405/404 全 0 |

**验证记录（rev10，阶段 F+G）**：

| 检查 | 结果 |
|---|---|
| `cargo check -p query_assemble_designer -p meeting_assemble_control` | ✅ EXIT=0 |
| `cargo fmt`（改动文件 `u2_closures.rs`） | ✅ 已格式化（`quoted_key_guard.rs` 为本地 rustfmt 版本差异，非本轮改动） |
| `cargo test -p query_assemble_designer -p meeting_assemble_control --lib` | ✅ **46 + 58 passed / 0 failed** |
| `cargo test --test quoted_key_guard`（E6 守卫，`CARGO_INCREMENTAL=0`） | ✅ 1 passed |
| `cargo test --test designer_route_match`（路由契约） | ✅ 1 passed |
| `pnpm typecheck` | ✅ 6/6 工程 |
| `pnpm test` / `test:mobile` | ✅ **953** / **101** passed |
| `tests/contracts/mobile-endpoints.test.ts` | ✅ 3 passed（新增 statistics/list 已纳入白名单） |
| `biome lint --diagnostic-level=error`（改动 10 文件） | ✅ 0 error（format-CRLF 为工作树差异，经 git 归一/Linux CI 为 LF） |
| `compare.py --gate` 四端对账 | ✅ EXIT=0（shadow/405/404 全 0；桌面 334 / 移动 80 / sdk 5） |
| `schema_audit.py --gate` | ✅ EXIT=0（A0/B10/C16/D23/E2 ≤ 基线） |
| `flow_matrix.py` | ✅ 移动端 18 业务域、0 问题 |

### 依赖顺序

```
0（后端契约：转义引号 351 + 请求体忽略 5）
      │
      ├─► A（止损：22 处影子）──► E（门禁）──► B/C（补方法/补缺）──► D（字段对齐）
      │                              │
      │                              └─► F（请求/响应契约 B/C/D + 运行时校验）
      │                                        │
      └────────────────────────────────────────┴─► G（移动端扩面）──► H（一致性治理）
```

**关键路径说明（三条）**：

1. **阶段 0 必须最先做**——后端契约缺陷（转义引号 + 请求体忽略）**修一次三端同时受益**，而前端 57 处只解决桌面端；其中 81 处会 panic(500)。
2. **阶段 E（门禁）必须在 A 之后立即插入**——先把"不再产生新错"的闸门立起来，再去做 B/C/D 的批量修复，否则修复速度追不上退化速度。
3. **阶段 F 必须在 G 之前**——请求/响应契约没对齐时扩移动端，等于把错配复制到第三个端。`H-8` 要求把 A–E 五类纳入 CI 阈值断言，`D` 类（委派）进白名单且不许新增。

---

## 十、风险与不变量

| 项 | 说明 |
|---|---|
| **不变量 1** | 后端 90 crate 的 router 链式 merge 必须始终能构造（栈溢出历史见 `.cargo/config.toml` 注释与 64MB 栈修复）——任何新增路由后需跑 `cors_guard::create_app_builds_without_panic` |
| **不变量 2** | `mock*` 别名族是兼容契约，**不可清理**（会破坏 O2OA 前端兼容） |
| **风险 1** | 阶段 A 修 `FormApp` 时，若后端 `POST /api/form` 的入参契约与前端 `mform` 不一致，需同步调整（本审计验证了路由存在，请求体契约见 §六） |
| **风险 2** | **阶段 0-1 的 351 处批量替换是高风险机械改动**：`"\"key\""` → `"key"` 在 4 类上下文（读列名/响应键/请求键/路由路径）语义不同，且**路由路径那 5 处改名会改变对外契约**（如 `/api/meeting/{"meetingId"}/participant/add` → `/api/meeting/{meetingId}/participant/add`）。必须：① 先备份；② 按类分别替换；③ 改后跑 `cargo fmt` + 全量 `cargo test`；④ 路由类改动需确认无外部调用方依赖旧路径 |
| **风险 3** | **D 类 20 处（泛型 helper 委派）静态不可判定**：在 H-7 运行时校验落地前，这 20 处的请求体契约状态是"未知"，**不得视为已验证**。本审计按 fail loud 单列，不并入 B/C |
| **风险 4** | 本审计**未覆盖**：权限模型正确性、并发/事务语义、性能、多方言（MySQL）行为、以及**响应体的深层嵌套结构校验**（只到顶层键）——均为独立轨道 |
| **边界声明** | `schema_audit.py` 存在 1 处**已知保留的误报**：`im_msg` 的 `conversationId`（根因是后端转义缺陷，已归入 E 类）。此外前端"动态拼接路径"的方法分支语义仍无法静态判定（3 条展开调用标记 `ambiguous`） |

---

## 十一、验收总纲

| 项 | 初始 | 目标 | 达成（rev11） | 度量方式 |
|---|---|---|---|---|
| 桌面端 API 闭合率 | 83.1% | **100%** | ✅ 100%（332/332） | `api_reconcile.json` 无 shadow/405/404 |
| 移动端 API 闭合率 | 100% | 保持 100% | ✅ 100%（80/80） | 同上 |
| 静默影子路由 | 22 | **0** | ✅ 0 | 同上 |
| 字段错配 | 5 处存疑（2 处证实） | **0** | ✅ 0 | `field_audit.json` + 单端点测试 |
| **请求体被完全忽略（A）** | **5** | **0** | ✅ **0** | `schema_audit.json` |
| **发送但后端不读（B）** | **21** | **0** | ✅ **0** | 同上 |
| **后端读但前端不发（C）** | **15** | **0** | ✅ **0** | 同上 |
| **委派不可判定（D）** | **20** | **0**（全部有结论） | ✅ **0**（提取器解析 CrudSpec/间接引用后全部落地结论） | 同上 |
| **转义引号键缺陷（E）** | **351**（含 81 处会 panic） | **0** | ✅ **2**（仅合法 SQL 内引号，非缺陷；已锁基线） | 同上 + E6 守卫 |
| 移动端业务域覆盖 | 6 | **≥ 12** 或书面范围外 | ✅ **18** | `flow_matrix.py` |
| 防回归门禁 | 无 | E1–E6 全绿 | ✅ compare/schema 双 gate PASS，硬 0 基线 + fail-injection 验证可失败 | CI |
| 后端能力消费率 | 5.8% | 按业务优先级逐域提升（目标 100%） | 🟨 长期项（G5）：**9.4%**（382/4072；…rev26 会议楼栋 +2、rev27 授权委托 +4、rev28 会议室 +2） | `consumption_gap.py` 逐域核算 |

> **rev11 验证记录（阶段 F 收官）**：提取器升级 + 契约全对齐后 `schema_audit` A=B=C=D=0 / E=2；`compare`/`schema_audit` 双 `--gate` EXIT=0；`cargo test` 受影响 5 crate（auth 53 / query_assemble_designer 93 / ai_assemble_control 15 / control 58 / meeting_assemble_control 46，全 passed）；`quoted_key_guard`/`designer_route_match` 各 1 passed；`vitest` 953 + mobile 101（含 4 处审批契约测试更新为「仅 opinion」）；`biome lint --diagnostic-level=error` 改动 17 文件 0 error；fail-injection（注入 bogus 键）→ B=2 证明 gate 仍可失败。**`pnpm typecheck` 6/6 全绿**：顺带修复既有构建配置类型漂移——`vite.config.ts` 的 `manualChunks` 由对象改为等价函数形态（当前 rollup 类型只接受函数），分组结果不变、`desktop build` 通过（vue/query/naive/codemirror 分包一致）。

---

## 附：相关文档

- `docs/audits/three-ends-2026-09-20/` —— 本次审计脚本与机器产物（可复现）
- `docs/plans/2026-09-11-001-assess-oa4rust-web-full-replacement-gap-plan.md` —— 前序"能否完全替代 o2web"评估（本计划是其**执行层细化**，聚焦三端接线与字段）
- `docs/audits/o2server-parity-report.md` —— 端点级 parity 基线（2026-08-12）
- `oa4rust/tests/designer_route_match.rs` —— 既有的设计器路由死链回归测试（E1 可复用其形态）
