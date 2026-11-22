# R366 PC 程序中心写操作链信封与 UI 反馈一致性甄别（只读）

- 轮次/任务：R366 / t179（qa-e2e 只读甄别，attempt 83a18a19-cfbb-4fa7-b276-bc16d26ea44e）
- 日期：2026-11-22
- 对象：oa4rust/frontend/apps/desktop/src/views/ProgramCenterApp.vue（2503 行）；路由 oa4rust/frontend/apps/desktop/src/main.ts:170-173（path=program，name=ProgramCenterApp）
- 环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；域名/主机统一以占位符 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 替代；非物理真机（headless 浏览器自动化，不代表真实用户设备）。
- 性质：只读审计，未改任何源码/契约/后端/SDK；破坏性真状态翻转与真删未授权，未执行；changePassword/adminLogout 等敏感写仅验取消守卫，未真执行。

## 1. 覆盖甄别

docs/ops 全量 grep（programcenter / program_center 写 / R366）命中 0 处；R359（H5 meeting 五写端点 200+success+bool:false 误报成功）、R362（PC MeetingApp 写链 200+success+bool:false）、R365（PC ProcessWork 写链，假ID多 404）均非本页专项。本轮首次真浏览器专测程序中心 PC 写链。

## 2. 源码甄别（先按 R364/R365 教训过滤死代码）

- 真实 @click 入口（非空代码）：agent tab :45 toggleAgent / :46 executeAgent / :47 editAgent / :48 deleteAgent；application :65 deleteApp / :66 compareApp；script :83 runScript / :84 deleteScript；dict :104 deleteDict / :103 openDictData；invoke :140 createInvoke / :151 deleteInvoke；config 写区 :181-262 pcStyleErase×7 / pcCollectDelete / pcInvokeSaveById / pcInvokeDeleteById / pcConfigSave×2 / pcTokenThreshold / pcU3×多 / pcU4Write×多 / pcU5Write×多 / pcU6×多 / pcU7×多 / pcU19×3 / pcOpenPlatform×多（含 changePassword:251、tokenPut:252、adminLogout:262）。
- 全文件无 defineExpose、无 window 挂载（grep 仅 import confirmMsg,toast at :401），确认上述函数为生产可用入口，非 R364/R365 的死代码调试函数。
- 裸 toast.success（await 后完全不看响应）：toggleAgent(:909-918，:912 GET agent/{flag}/disable|enable→:913 toast 已启用/已禁用)；executeAgent(:919-927，:922 GET agent/{flag}/execute→:923 toast 已触发执行)；saveAgentEdit(:1024-1038，:1029 POST agent/save/{id}→:1030 toast 已保存)；saveDictData(:1072-1093，:1085 POST dict/{flag}/data→:1086 toast 已保存)；saveScript(:1118-1133，:1124 POST script/{flag}→:1125 toast 脚本已保存)；pcStyleErase(:1406-1420，:1416)；pcCollectDelete(:1421-1430，:1426)；pcInvokeSaveById(:1431-1440，:1436)；pcInvokeDeleteById(:1441-1450，:1446)；pcConfigSave(:1451-1466，:1462)；pcTokenThreshold(:1467-1475，:1471)；pcU3(:1556-1593，:1589)；pcU4Write(:1595-1627，:1623)；pcU5Write(:1748-1768，:1764)；pcU6(:1770-1822，:1818)；pcU7(:1824-1839，:1835)；pcU19(:2009-2040，:2021/2032/2035)。
- 对比组（已部分判 type）：pcOpenPlatform(:1482-1554) 中 changePassword(:1493-1498) 判 r?.success===false 或 r?.type 为 error 时 toast.error；invokeExecGet(:1510-1512)、invokeTokenPost(:1514-1516)、adminLogout(:1547-1549) 判 type；但 tokenPut(:1502-1503)、thresholdPut(:1506-1507)、invokeTokenExec(:1529-1530)、moduleComparePost(:1532-1533)、moduleComparePut(:1535-1536)、avatarPut(:1539-1540)、avatarEraseGet(:1543-1544) 仍裸 toast.success。
- useMutation onSuccess 同型风险：createAgentM(:929-939→:935)、deleteAgentM(:950-957→:955)、deleteAppM(:958-965→:963)、deleteScriptM(:966-973→:971)、deleteDictM(:974-981→:979)、createDictM(:997-1007→:1003)、compareM(:1159-1167→:1161)、runScriptM(:1170-1179→:1172)、eraseImageM(:1202-1211→:1205) 一律 2xx resolve 即触发 onSuccess，未读 type。

## 3. 契约与 SDK 证据链

- 后端：oa4rust/backend/crates/shared/src/response.rs:149-164 ActionResult 含 data/type/message/date/spent/size/count/position/prompt；:170-182 success() 设 type=success；:188-200 error() 设 type=error/message/prompt（prompt=com.x.base.core.project.exception.ExceptionEntityNotExist）。即业务失败以 HTTP200+type:error 返回，而非非 2xx。
- program_center/src/lib.rs 全量 grep not-found 128 处；典型 :472 config not found、:522/:564/:606/:648/:687/:861/:882/:942 等均 None 时 Ok(Json(ActionResult::error(...)))，HTTP200。agent 三写 handler：:10455 flag_disable / :10465 enable → u3_agent_set_enable(:10445 if n==0 return error(agent not found))；:10475 execute（:10484 UPDATE x_program_agent，:10489 if n==0 return error(agent not found)，:10501 success executed:true）。
- SDK：oa4rust/frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 仅 await resp.json()，不检查 type；:275-291 仅非 resp.ok 才抛错（403→PermissionError、404→ApiError、其它→httpStatusError）；:293-296 discardResponse 或 204 返回 success:true。故 2xx+type:error 原样 resolve，前端裸 toast.success 误判成功。

## 4. 真浏览器实测（3 轮，1440x900 headless，非物理真机）

- 第 1 轮（r366-pc-programcenter-write.cjs）：①真后端假ID探针（fetch 直打，R366-NOSUCH-*）9 端点——agentDisable/agentEnable/agentExecute/agentSave(POST)、dictDelete/appDelete/invokeDelete/styleErase(DELETE) 全 status200+type:error+message=not found+prompt=ExceptionEntityNotExist；collectDelete=status404。②注入列表 1 行 R366 注入 Agent + 注入写响应：type:error 场景点禁用→toast「✓已禁用」误报、点执行→「✓已触发执行」误报；bool:false 场景→「✓已禁用」误报；http500 场景→「✗HTTP 500: Internal Server Error」正确。0 pageerror。截图 program-dom-audit.png / program-injected-agent.png / program-write-toasts.png。
- 第 2 轮（r366-pc-programcenter-write-v2.cjs，决定性对照）：只注入列表行，写响应完全走真后端（不拦截写 route）。rows=1；点禁用→toast「✓已禁用」，真响应 realResps 含 agent/R366-INJECT-1/disable status200 body type:error agent not found、execute status200 type:error agent not found；点执行→「✓已触发执行」。真后端 200+type:error 下 UI 仍误报成功，排除注入伪造质疑。取消守卫：点删除弹「确定删除该Agent？」确认框，dismiss 取消后 deletesBefore=0/deletesAfter=0、zeroWrites=true，取消 0 写请求。0 pageerror。截图 program-v2-real-backend-toasts.png。
- 第 3 轮（r366-pc-programcenter-write-v3.cjs）：干净 context 分别注入列表端点 500 与空数组。失败态→「智能体列表加载失败：HTTP 500: Internal Server Error」+重试按钮（failRows=0）；空态→「暂无Agent」（emptyRows=0）；distinguishable=true，0 pageerror。截图 program-v3-fail-state.png / program-v3-empty-state.png。

## 5. 三分判定

### 真缺陷（1 个 P1）
ProgramCenterApp.vue 大量有真实 @click 入口的裸 toast.success 写函数，在后端 200+type:error（真后端假ID可复现，第 2 轮真响应坐实）下无条件弹成功 toast。三全满足：真 UI 可复现（注入行+真后端写响应）+ 真请求响应（realResps status200 type:error）+ 源码定位（:909-927 等函数 await 后直接 toast.success）。影响函数群：toggleAgent、executeAgent、saveAgentEdit、saveDictData、saveScript、pcStyleErase、pcCollectDelete、pcInvokeSaveById、pcInvokeDeleteById、pcConfigSave、pcTokenThreshold、pcU3/U4Write/U5Write/U6/U7/U19、pcOpenPlatform 部分分支，useMutation onSuccess 类同型。用户后果：操作实际未生效（如 agent not found）却提示成功，掩盖失败、误导运维。
最小修复方向（对齐 R363 方案 B，本轮不改码）：新增统一写响应成功判定（如 meetingWriteSucceeded 思路）：typeOk 为 type 等于 success 或（type 缺省且 success 不为 false）；权威 bool 取 resp?.data?.[flagKey] 回退 resp?.[flagKey]；仅 typeOk 且 flag 严格等于 true 才 toast.success，否则 toast.error(r?.message||操作未生效，请刷新后重试)。各写函数 await 后先判再提示；useMutation onSuccess 回调内同样判 type。R363 参考 oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:634-640。

### 取证限制
- 破坏性真状态翻转/真删未授权未执行，用假ID探针+route注入对照替代；changePassword/adminLogout 敏感写仅验取消守卫未真执行。
- 全量约 65 处 toast.success 未逐一 UI 触发，按裸 toast / 已判 type / onSuccess 三类源码归类 + 代表性端点实测覆盖。

### 产品语义差异（非缺陷）
- 后端 ActionResult::error 用 HTTP200+type:error 是 o2server 兼容既有设计（本轮不改后端）。
- collectDelete 走 404 与其它程序中心端点 200+type:error 口径不同，属既有实现差异。
- useMutation onSuccess 仅在 HTTP 非 2xx reject 时跳过，设计如此；缺陷在于 onSuccess 内未判 type，同型归 P1 修复范围。

## 6. 通过项

- 非 2xx（403/500）正确 toast.error，未误报成功。
- confirmMsg 取消守卫 0 写请求（第 2 轮 zeroWrites=true）。
- 列表失败态与空态文案可区分、可重试（第 3 轮 distinguishable=true）。
- 三轮 0 pageerror；pcOpenPlatform 已判 type 的 changePassword/invokeExec/invokeToken/adminLogout 分支正确。

## 7. 证据清单（gitignored，不入库）

- 目录：oa4rust/test-results/r366-pc-programcenter-write/
- 数据：evidence.json（假ID探针+注入toast）、evidence-v2.json（真后端响应对照）、evidence-v3.json（失败/空态）。
- 截图：program-dom-audit.png、program-injected-agent.png、program-probe-done.png、program-write-toasts.png、program-v2-real-backend-toasts.png、program-v3-fail-state.png、program-v3-empty-state.png。
- 脚本：r366-pc-programcenter-write.cjs / -v2.cjs / -v3.cjs 及对应 run.sh。

注：本报告为 headless 浏览器自动化实测，非物理真机；域名与主机均以占位符表示，源码路径为仓库相对路径。