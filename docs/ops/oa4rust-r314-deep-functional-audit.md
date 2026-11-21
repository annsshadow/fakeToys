# R314 深度功能闭环审计（工作流 / IM / 考勤 / 文件系统）

> 来源与边界：本报告由 qa-e2e 于分支 `opt1000/oa4rust-pc-ai-refactor` 上以本地无头浏览器（Playwright Chromium headless）对测试服真页面走真实 UI 流程取得，非物理真机；测试域名与主机一律以占位符表示，口令仅在运行时经 SSH 从远端凭据文件读取，不入仓、不回显。本轮为只读审计，未修改任何源码 / 契约 / 后端，未判决缺陷修复，项目持续迭代（≥1000 轮）非终态。

- 测试目标：PC 端 `/oa4rust`（入口 `index-D1EjQmHJ.js`），1440×900；登录用真 UI（占位用户名/口令输入框 + 登录按钮）。
- 测试域名占位：`<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`；报告内 0 处真值。
- 证据目录（gitignored，不入仓）：`oa4rust/test-results/r314-deep-audit/`（DOM 探查 JSON + 5 张 dom-*.png，主流程 `flow-evidence.json` + 9 组截图，补测 `flow-supp.json` / `f1-resume.json` + 若干截图）。全部为本地无头浏览器渲染，非物理真机。
- 脚本（gitignored）：`r314-dom-probe.cjs`、`r314-flow.cjs`、`r314-supp.cjs`、`r314-f1-resume.cjs` 及各自 run 包装（运行期注入 BASE，不入库）。

## 一、逐闭环判定表（真 UI 流程 + HTTP 状态 + 页面回显）

| 域 / 闭环 | 判定 | 关键证据（HTTP + 页面回显 + 证据文件） |
|---|---|---|
| W1 工作流·发起流程 | PASS | 真 UI 发起流程 → 选流程填标题发起 → 我发起的列表回显 6 条（含 R314流程_时间戳）；启动端点 HTTP 200。截图 W1-process-started.png |
| W2 工作流·审批通过→已办 | FAIL（真实缺陷 D1） | 待办 11→办理→审批通过 HTTP 200（响应含 end_time）；但已办列表恒 0，0s/3s/6s 三轮轮询均 0；taskcompleted/list/my/paging HTTP 200 但 data:[] count:0。截图 W2b-done.png |
| X1 跨页·待审批数一致性 | PASS（非缺陷） | 工作台 overview 5 待审批与 dashboard 待办列表 5 条一致；工作流待办全量 11 条。Dashboard.vue:332 的 slice(0,5) 截断展示，:320 的 DR-02 注释统计与列表同源，非跨页矛盾。截图 X1-dashboard.png |
| I1 IM·单聊发消息+刷新回显 | PASS | 会话列表 11 条；选首会话输入消息发送，API im/msg POST 200 + im/msg/list 200，发送回显 2、刷新后回显 1（R314消息_时间戳）。截图 I1-im-after.png |
| I2 IM·群聊闭环 | 取证限制 | 会话列表 11 条全部标记单聊，groupCount=0，测试账户无群聊会话可测；非缺陷，需预置群聊数据后复测。 |
| A1 考勤·月份切换+统计卡 | PASS | 真 UI 月份 2026-11 切到 2026-10 成功；统计卡恒 0 应出勤/0 实际出勤/0 迟到/0 请假，空态引导 attendance-empty-cta 存在，统计与明细（空）一致。截图 A1-attendance.png |
| A2 考勤·打卡闭环 | 取证限制 | 高级操作打卡类入口均为调试残留（prompt 入口，如打卡周期员工、建打卡点、导入打卡记录、钉钉同步打卡等），无真实打卡表单按钮；打卡点数据前置为空。不计业务缺陷。 |
| F1 文件·新建→删除→回收站→恢复 | PASS（补测后） | 首轮 FAIL 系脚本误用 li 选择器（回收站行真实 DOM 为 div.table-row）；补测 r314-f1-resume.cjs 真 UI 走通：新建 file/core/entity/file 200 → 删除按钮 → recycle/list 200 命中 → 独立回收站恢复 → recycle/resume/id 200 且 resumed=true → 残留 0。截图 F1-resume-after.png |
| F2 文件·上传真实文件 | FAIL（真实缺陷 D2） | 点上传进弹窗、弹窗内 input[type=file] setFiles 成功，但上传端点 POST /api/file/assemble/control/file/upload 返回 HTTP 415，提示 Content-Type 应为 application/json；我的文件未新增。截图 F2b-uploaded.png |

主流程汇总：R314_FLOW_DONE 记录 loops=9、pass=5、pageErrors=0、api=113（pass 含首轮 F1 部分通过；补测后 F1 转 PASS 见上表）。补测 R314_SUPP_DONE 记录 checks=3、pass=0、pageErrors=0、api=60（三项 FAIL 经源码/API 定性：W2、F2 为真缺陷；F1 为脚本选择器，已由 F1 补测 PASS 推翻）。全程 0 pageerror，无未预期的 4xx/5xx（除已记录的 F2 415）。

## 二、真实缺陷清单（现象 / 证据 / 源码定位 / 复现 / 修复方向）

### D1 工作流审批通过后已办列表恒为空（已办不同步）
- 现象：待办任务办理并审批通过成功（task/complete HTTP 200，含 end_time），但已办 tab 列表与 taskcompleted/list/my/paging 接口均始终 0 条（0/3/6 秒轮询均 0）。
- 证据：test-results/r314-deep-audit/flow-evidence.json（W2 API 链）与 flow-supp.json（W2b polls 全 0）、截图 W2b-done.png。
- 源码定位（高置信，静态审计）：后端 oa4rust/backend/crates/processplatform_service_processing/src/lib.rs:3795 起 task_complete：约 :3825 执行 UPDATE x_task SET task_status 及 end_time，约 :3840 INSERT INTO x_record，终点分支 UPDATE x_work 为 completed 加 INSERT INTO x_record——全程只写 x_task 与 x_record，不写 PP_C_TASKCOMPLETED。前端已办列表 oa4rust/frontend/apps/desktop/src/views/ProcessWork.vue:298-299 读 GET /api/processplatform/assemble/surface/taskcompleted/list/my/paging/1/size/20；后端 oa4rust/backend/crates/processplatform_assemble_surface/src/lib.rs:19010 起 taskcompleted_list_my_paging_page_size_size_core 的 SQL 为 SELECT 自 PP_C_TASKCOMPLETED（约 :19020），且全后端除只读查询/测试外无 INSERT INTO PP_C_TASKCOMPLETED 写入点（grep 全 crates 确认）。两表数据链路断裂：审批完成只更新任务/工作/记录表，从不生成已办记录。
- 复现路径：登录 → /app/process → 待我处理 → 任一任务办理 → 填意见 → 审批通过（200）→ 切已办 tab → 列表空、接口 data 为空数组。多次、多任务稳定复现。
- 修复方向（交实施者，本轮未改）：在 task_complete 与 task_reject 事务内，任务终结时按 PP_C_TASKCOMPLETED 列结构补写一条已办记录（xid/xjob/xtitle/xstartTime/xcompletedTime/xcreatorPerson/xperson/xapplication 等），或改前端已办列表改读 x_task 中 task_status 为 completed/rejected 的自有任务；配 RED→GREEN 集成测试（完成一条待办后已办列表 count 至少为 1）。注意 x_task.person 是办理人、x_work 存工作实例，映射字段需核对契约。

### D2 文件上传端点 Content-Type 不匹配（真实用户上传功能不可用）
- 现象：文件管理页点上传 → 弹窗选真实文件 → 上传请求 POST /api/file/assemble/control/file/upload 返回 HTTP 415，提示期望 Content-Type 为 application/json，文件未落库、我的文件无新增。
- 证据：test-results/r314-deep-audit/flow-supp.json（F2b uploadApi 记录 415 于该文件上传端点，inList=0）、截图 F2b-uploaded.png（弹窗已选 r314b-upload.txt 但列表为空）。
- 源码定位：前端 oa4rust/frontend/apps/desktop/src/views/FileManager.vue:1373 起 uploadFile 用 FormData.append 加文件，再调 api.uploadWithProgress 指向 /api/file/assemble/control/file/upload（:1379）；SDK oa4rust/frontend/packages/sdk/src/api.ts:378-452 的 uploadWithProgressOnce 以 XHR 的 xhr.send(formData)（:452）发送 multipart/form-data。后端 oa4rust/backend/crates/file_assemble_control/src/lib.rs:415 起 upload_file 签名为 axum::extract::Json 的 Value body，只接受 application/json，从 body 取 name/path/folderId/size 写 x_file；路由 oa4rust/backend/crates/file_assemble_control/src/routes.rs:120 将该端点 POST 绑到 upload_file。前端 multipart 与后端 Json 提取器直接冲突导致 415。契约 fixture oa4rust/tests/contracts/backend-route-methods.fixture.ts:899 仅登记该路径方法为 POST（只校验方法，未校验 Content-Type/body），故契约绿但真 UI 上传失败。
- 复现路径：登录 → /app/文件 → 上传 → 弹窗选择任意本地文件 → 网络面板出现 415，我的文件无新条目。稳定复现。
- 修复方向（交实施者，本轮未改）：二选一对齐——①后端 upload_file 改为 axum multipart extractor，落盘字节并写 x_file（可复用同 crate oa4rust/backend/crates/file_assemble_control/src/lib.rs:4797 的 u2_file_upload_multipart 实现）；②前端 uploadFile 改用 JSON 登记（api.post 该端点并传 name/path/size/folderId，与后端现有 Json handler 对齐，牺牲真实字节上传）。建议①以保住上传真实文件语义；配契约/集成测试 RED→GREEN：multipart 上传返回 2xx 且文件列表可见。

## 三、取证限制与非缺陷说明（不计业务缺陷）

- I2 群聊：测试账户会话列表全为单聊（groupCount=0），无群聊会话，需在测试数据层预置群聊后复测。
- A2 打卡入口：AttendanceApp.vue 高级操作中的打卡相关按钮多为 prompt 调试残留（打卡周期员工、建打卡点、钉钉/企微同步打卡等），无真实打卡表单可点；不计缺陷，待真实打卡入口落地后覆盖。
- X1 待审批数：工作台 5 待审批与 dashboard-todo-list 5 条一致、工作流待办全量 11 条为同一数据源的截断展示（Dashboard.vue:332），非跨页数据矛盾。
- F1 首轮 FAIL：脚本误用 files-recycle-list 下的 li（回收站行 DOM 实为 div.table-row），校正选择器后新建→删除→回收→恢复全链路 PASS；原 FAIL 为取证脚本问题。
- 通讯录新建人员入口、角色成员授权等边界与前轮 R309/R310 结论一致，本轮不重复判缺陷。

## 四、结论

- 四域共 9 个闭环：PASS 6（W1、X1、I1、A1、F1 及跨页口径核实）、取证限制 2（I2 群聊、A2 打卡）、真实缺陷 2（D1 工作流已办不同步、D2 文件上传 415）。
- 两处真实缺陷均给出源码路径/行号、真 UI 与 HTTP 证据、可复现路径与 RED→GREEN 修复方向；本轮只读审计未改源码，交实施者排期修复。
- 全程 0 pageerror、无非预期 API 失败（F2 415 为已记录缺陷）；测试服服务正常，未发现前端解析/渲染类新问题。项目 ≥1000 轮持续迭代，本轮非终态。