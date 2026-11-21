# R316 5域低覆盖业务闭环深度功能审计

## 来源与边界
- 任务：t89 [R316] 深度功能审计（BAM 周期明细 / 通知已读流转 / 工作流已办 / 日程新建 / SearchApp loadSearchTwin 死代码观察），kind=work（只读审计）。
- 审计人/身份：qa-e2e（agent-2）；时间：2026-11-21。
- 分支：opt1000/oa4rust-pc-ai-refactor（禁 main）；起始 HEAD：9808b42d95a6980930825261c11b791d5aa288ef。
- 测试服：systemctl is-active oa4rust=active；PC 构建入口 index-yrBKdt7I.js / CSS index-BMFkaD73.css（已含 R315 搜索 D1 修复与 R314 已办 D1 修复）。
- 测试环境：Playwright Chromium headless，1440x900；**本地无头浏览器渲染，非物理真机**。
- 登录：真实 UI 登录（getByPlaceholder 用户名/密码 + 登录按钮），凭据运行期读取，不回显不入库。
- 域名占位：〈YOUR_TEST_DOMAIN〉，0 真值（无域名/IP/口令/会话数据入库）。
- 范围：只读审计，不改源码/契约/后端；唯一入库物为本文档；证据 gitignored 落 oa4rust/test-results/r316-deep-audit/。
- 方法：每域真请求 + 真响应 + 真截图 + 源码定位四件套；找不到如实写「未复现/取证限制」，不臆造缺陷。

## 逐闭环判定表
主 E2E（R316_FLOW_DONE：5 闭环，pageErrors=0，API 调用 78 次）+ 补测（R316_SUPP_DONE：3 闭环，pageErrors=0）：

| # | 域/闭环 | 主判定 | 补测/深挖 | 最终结论 |
|---|---|---|---|---|
| L1 | BAM 周期明细 | FAIL 按钮=0 / period文本=false / period接口200=false | S1：fold=0、toggle=0；EndpointFold 生产构建 DEV=false 整块不渲染；统计卡 98/0/27/10 由 state/summary 200 真数据派生 | 取证限制（非缺陷）：周期明细按钮仅 DEV 可见，生产不可点 |
| L2 | 通知/消息已读流转 | FAIL 未读bar=0 / badge=0 / readAPI=0 | S3：通知公告列表 1 条可见（instant/list/currentperson/count/20/desc 200），会话 unread 分布空、全部已读按钮=0 | 取证限制（非缺陷）：当前账号无未读会话，已读流转无法触发 |
| L3 | 工作流已办闭环 | FAIL 脚本判定「条目 8→3」 | S2：已办 API200、DOM 条目=3、响应 count=3 且含刚审批 task id 5fce276b…；approvedCalls POST /api/task/…/complete 200 | 修复生效（非缺陷）：R314 D1 已办同步后端补写后审批即入已办；「8→3」系脚本跨时点计数口径误判 |
| L4 | 日程新建 | PASS 日历数=7 / 表单=true / 创建200 / 回显=1 / 刷新后=1 | — | 通过：calendar/event/create 200，刷新持久化 |
| L5 | 搜索 loadSearchTwin 死代码观察 | PASS 孪生按钮=0 / 孪生API=0 / 结果=3 / 空态=0 / 初始态=0 | — | 通过：R315 D1 搜索信封修复后返回 3 条结果；loadSearchTwin 无调用点=死代码，生产不触发 |

## 关键观察与源码定位
1. BAM 周期明细 gating（L1/S1）
   - 组件 oa4rust/frontend/apps/desktop/src/components/admin/EndpointFold.vue:6 v-if=devToolsEnabled、:55 devToolsEnabled 默认 import.meta.env.DEV。
   - 消费点 oa4rust/frontend/apps/desktop/src/views/BamApp.vue:9 EndpointFold test-id=bam-endpoint-fold、:82 foldGroups；周期明细按钮（:11 周期统计/:17 维度周期统计/:18 计数聚合/:19 周期多维矩阵）全在折叠面板内。
   - 统计卡 BamApp.vue:44-48 由 stats 派生，:606 state/summary，:614 stats.value=「total=totalWork+totalTask, active=pendingTask+processingTask, completed=completedWork, failed=expiredTask」。
   - 实测响应：totalTask=48 / totalWork=48 / completedTask=26 / pendingWork=22 / expiredTask=10（主 E2E 14:01:17）；补测时点统计卡显示 98/0/27/10——两次采样时点不同，审计期间数据自然演进，非缺陷。
2. 工作流已办修复证据（L3/S2）
   - 前端 oa4rust/frontend/apps/desktop/src/views/ProcessWork.vue:286 completed=/api/processplatform/assemble/surface/taskcompleted/list/my/paging/0/size/20、:298-299 queryFn、:83 work-completed-list。
   - 后端修复见 oa4rust/docs/ops/oa4rust-r314-taskcompleted-sync-fix.md（PP_C_TASKCOMPLETED 补写）。
   - 真响应：count=3，ids 含 5fce276b-440d-467e-9e66-bf702a07a03d（本轮刚审批 task），DOM 3 条「已办」标签，截图 S2-work-completed.png 目视确认。
3. 通知已读能力（L2/S3）
   - 前端 IMChat.vue:37 im-mark-all-read、:531-547 markConversationRead/markAllRead 调 POST /api/message/assemble/communicate/mark_read/{convId}。
   - 后端真实注册 oa4rust/backend/crates/message_assemble_communicate/src/routes.rs:44 mark_read POST 路由。
   - Dashboard 通知公告 Dashboard.vue:101-118，loadRecent GET instant/list/currentperson/count/20/desc。
   - 当前账号未读=0，能力存在但无可触发数据，取证限制。
4. 日程新建（L4）
   - CalendarApp.vue:49-53 calendar-create-entry、:130-135 calendar-event-form、createEvent :467-490 POST /api/calendar/event/create。
   - API 链 calendar/calendar/list/my → calendar/event/create → event/list/{id} 均 200，刷新后条目仍在。
5. 搜索 loadSearchTwin 死代码（L5）
   - SearchApp.vue:111-119 loadSearchTwin（Promise.all 调 api.put /api/ftsearch/save/0 + api.delete /api/ftsearch/delete/0），全文件唯一出现=无调用点。
   - 后端路由真实存在 oa4rust/backend/crates/search/src/lib.rs:423 ftsearch/list、:425 ftsearch/create；generated_routes.rs:38883-38923 ftsearch/delete/{id}/list/create/save/{id}。
   - 契约 oa4rust/tests/contracts/backend-route-methods.fixture.ts:1044/1046、backend-registered-routes.fixture.ts:1044/1046。
   - 观察：死代码，生产不触发 ftsearch 写入；R315 D1 搜索信封修复后 doSearch 正常渲染 3 条结果。
6. R315 D1 搜索修复回归确认（L5）
   - 关键词 a 返回 results=3、empty=0、initial=0（修复前 R315 为 results=0 / initial=1）。
   - SearchApp.vue:85-109 doSearch 已按分组信封 designList/viewList 合并扁平数组。

## 取证限制（不计业务缺陷）
1. BAM 周期明细按钮：EndpointFold 生产构建 DEV=false 整块不渲染，需 DEV 环境或显式 defaultOpen 才可点。
2. 通知已读流转：当前账号会话 unread=0、无未读徽标，mark_read 无法触发。
3. 已办「8→3」脚本口径：L3 脚本按跨时点列表长度差判定 FAIL，补测按响应 count + 新 task id 精确复核后确认修复生效。
4. 统计数据动态：审计期间自发起流程，BAM 统计卡两次采样时点数字不同，属数据演进非缺陷。

## 真实缺陷
本轮未发现新增真实功能缺陷。5 域低覆盖闭环中，2 项直接 PASS（日程、搜索），3 项初判 FAIL 经补测深挖均归因为取证限制或脚本口径误判；R314 D1（已办同步）与 R315 D1（搜索信封）两项历史缺陷的修复在真浏览器中均得到正向证据。

## 证据索引（均 gitignored，本地无头浏览器非物理真机）
- 目录：oa4rust/test-results/r316-deep-audit/
- evidence.json（主 E2E 5 闭环，78 次 API 调用）、supplement-evidence.json（补测 3 闭环）
- 截图：L1-bam-period.png / L2-notice-read.png / L3-work-completed.png / L4-calendar-event.png / L5-search-twin.png / S1-bam-fold.png / S2-work-completed.png / S3-notice-read.png
- 脚本：r316-flow.cjs / r316-flow-run.sh / r316-supplement.cjs / r316-supplement-run.sh / r316-analyze.cjs / r316-detail.cjs / r316-detail2.cjs / r316-l3-detail.cjs

## 结论
R316 5 域低覆盖业务闭环真浏览器审计完成：日程新建、搜索修复回归两项直接通过；BAM 周期明细（DEV gating）、通知已读（无未读数据）、工作流已办（脚本口径误判，实际修复生效）三项初判 FAIL 经补测深挖排除真实缺陷。0 新增真实缺陷、0 pageerror。项目按 ≥1000 轮持续迭代，本轮为其中一环，非终态。
