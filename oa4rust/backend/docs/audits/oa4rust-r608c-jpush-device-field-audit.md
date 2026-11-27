# R608c JPush 设备列表端点错配字段零交集审计（验证型甄别）

- 结论：P2。JPushApp.vue 设备列表调用 /api/jpush_assemble_control/device/list/jpush，该端点返回 x_jpush 推送记录字段（id/title/content/target/creator/createTime），与前端消费的设备字段（isOnline/online/alias/regId/deviceId）零交集；在线数恒 0、图标恒离线、名称恒「未知设备」、Token 空。属用户可感知的局部展示失效，不丢数据、不越权、不崩溃、不阻断主流程。
- 派发真父 HEAD：7ebab5dd5477ad3384ac27adebddaedea6aed332（git rev-parse 核实；实际工作树 HEAD 与派发父一致，merge-base --is-ancestor 返回 0）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 临时路径，用后即删。

## 立项与范围
- 主对象 apps/desktop/src/views/JPushApp.vue:107-119 loadDevices()（默认 tab=ref<Tab>('device')，:93 首屏命中）。
- 供给端 A：backend/crates/jpush_assemble_control/src/lib.rs:595 device_list_pushType；供给端 B（兄弟对照）：backend/crates/jpush/src/lib.rs:40 device_list。
- 路由映射：backend/crates/jpush_assemble_control/src/routes.rs:48-50 /api/jpush_assemble_control/device/list/{pushType} → get(device_list_pushType)。
- 不扩范围：不修改任何源码/端点/路由/测试配置；仅甄别与取证。

## 机制锚点（真父源码逐行）
- 前端消费字段：JPushApp.vue:39 devices.filter(d=>d.isOnline).length 在线统计；:49 d.online?'📱':'⚫' 图标；:50 d.alias||d.regId||d.deviceId||'未知设备' 名称；:51 d.platform||'unknown'；:52 String(d.regId||'').slice(0,20) Token；:102-105 stats computed 同样 filter(d=>d.isOnline)。
- isOnline 与 online 两名在同一组件并存（:39 用 isOnline、:49 用 online），自证至少一名必然取不到值。
- loadDevices():111 固定请求 /api/jpush_assemble_control/device/list/jpush（末尾 jpush 为 pushType 字面量），:112 devices.value=r.data??[] 原样装载，无字段映射。
- 供给端 A device_list_pushType:602 SQL 为 SELECT id,title,content,target,creator,create_time FROM x_jpush WHERE target=$1 ——查推送记录而非设备；:608-641 每条 data 仅 id/title/content/target/creator/createTime；:643-648 legacy_success(Value::Array(data),count,0)。
- 供给端 B jpush::device_list:44 SQL 为 SELECT id,user_id,platform,token FROM x_jpush_device（真设备表）；:50-60 data 字段 id/userId/platform/token，同样无 isOnline/online/alias/regId/deviceId，但含 platform。
- 路由可达：frontend/apps/desktop/src/main.ts:146-149 path:'jpush' 动态 import JPushApp.vue；tests_generated.rs:168 标 'SKIPPED: device_list_pushType not accessible'，无现成后端测试覆盖。
- 旁路对照：JPushApp.vue:145-166 loadJpushEntities() 另经 :151 /api/jpush/device/list 取真设备清单，但 :169 entitiesText 仅展示计数（设备清单 N 条），不走 devices 卡片列表；主设备视图仍命中错配端点。

## 决定性实测（vitest 真执行）
- 探针 tests/r608c_probe.test.ts（临时，已删）：4 个 itest；命令 cd oa4rust && npx vitest run tests/r608c_probe.test.ts → EXIT 0，4 passed / 0 failed（vitest v5.0.5）。
- 用例 1：device_list_pushType 源码含推送记录 SQL 与六个输出字段，供给集 ∩ 消费集 = ∅。
- 用例 2：兄弟 jpush::device_list 含 platform/token，但与 isOnline/online/alias/regId/deviceId 仍零交集。
- 用例 3：从 Vue 源码抽取 loadDevices 函数体，经 new Function('api','devices','loadingD','deviceError','describeListError', ...) 真执行；注入一条推送记录形状数据，断言实际请求路径为错配端点、devices 装载后五个消费字段全为 undefined、platform 亦 undefined。
- 用例 4：在供给数据下真求值展示表达式，断言在线数=0、图标='⚫'、名称='未知设备'、Token=''；并校验四个表达式源码锚点存在。
- 决定性日志（gitignored 临时路径，用后即删）：<PROBE_LOG_PATH>，字节 sha256=1b6efef2fd7f8bc259dfbfe751764c49006836028fed880f3b914f23340b07b1。

## 三分裁决（据实，不夸大不降级）
1. 判 P2：首屏默认 tab 的设备卡片列表字段全量错位，在线数/图标/名称/Token 均落入兜底值，用户可直接感知展示失效。
2. 不判 P1：纯展示层，不写库、不越权、不外泄数据、不抛异常、不阻断主流程；旁路 loadJpushEntities 仍能经真设备端点拿到计数。
3. 不判 P3：非边缘文案，是默认首屏设备视图的核心展示，影响面明确。
4. 非安全族：与身份盲 IDOR/DEV 门控/写 envelope 假成功无关；根因是端点与字段契约错配。

## Fail Loud
- 未跑浏览器 GUI 真页面；以 vitest 真执行 + new Function 真调用 loadDevices 函数体 + 源码逐行锚点替代，未静默跳过。
- 真执行探针按注入的推送记录形状取证；真实库若有字段形态差异不改变「消费字段零交集」结论。
- 测试夹具与日志均在 gitignored 临时路径并已删除，未入库；未读取或回显任何密钥/口令/真实地址。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=ccf218faafd7c1712e00dbcbebbaf4576b1b4135b136f7a9a7eefcdaac621b4a
- meta={"task":"t191","round":"R608c","kind":"verification-work","target":"apps/desktop/src/views/JPushApp.vue device list field mismatch","verdict":"P2","evidence":"vitest 4passed + new Function real loadDevices exec + field zero-overlap","parent":"7ebab5dd5477ad3384ac27adebddaedea6aed332","head":"7ebab5dd5477ad3384ac27adebddaedea6aed332","logSha":"1b6efef2fd7f8bc259dfbfe751764c49006836028fed880f3b914f23340b07b1"}
- 证据日志字节 sha256=1b6efef2fd7f8bc259dfbfe751764c49006836028fed880f3b914f23340b07b1
- frontend/apps/desktop/src/views/JPushApp.vue=ac7e0cef3662c2a321feacb15dbedbce44e2f2a4f242d0905111676d6cf20d6e
- backend/crates/jpush_assemble_control/src/lib.rs=bd10bf124448d545e3c235494aa93dd51eb0f3ff351f883616fdaf035e942303
- backend/crates/jpush/src/lib.rs=968bcec5ed2a7ab833262fabfc9d92390bef17326396cb35a8ec86428df9b157
- backend/crates/jpush_assemble_control/src/routes.rs=2255b06fbb4b6f61a8196f24bb0634baf3c856d3464bb55d3543063fe0613d12
- frontend/apps/desktop/src/main.ts=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0

## 修复方向（不在本任务范围）
- 将 loadDevices 改调真设备端点（/api/jpush/device/list，字段 id/userId/platform/token）或扩充 device_list_pushType 返回设备字段；前端补字段映射并统一 isOnline/online 命名；补一条「设备卡片显示真实平台/名称/在线态」的前端回归测试。
