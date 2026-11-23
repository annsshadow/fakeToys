# R416 PC 流程实例管理（ProcessManagerApp）写链 200+type:error 误报成功甄别报告

## 一、审计范围与目标
- 目标文件：frontend/apps/desktop/src/views/ProcessManagerApp.vue（1223 行 / 73956 字节）；路由 frontend/apps/desktop/src/main.ts:313-318（path=process-manager / name=ProcessManagerApp / meta.title=流程实例管理），页面标题"流程实例管理"，实际落点 /app/admin/process-manager。
- 任务性质：只读甄别，不改码；判定用户可达真缺陷 / 隐患 L1 / 正确模式三分。
- 实测环境：测试服生产构建，1440x900 headless Chromium，非物理真机；凭据与域名用占位符，不入库真值。

## 二、源码甄别
- 门控（决定性）：模板第 17 行 <EndpointFold label="高级操作" :groups="foldGroups" :total-count="endpointCount" test-id="pm-endpoint-fold">，第 136 行闭合；组件 frontend/apps/desktop/src/components/admin/EndpointFold.vue 第 6 行 <section v-if="devToolsEnabled" class="endpoint-fold" :data-testid="testId">，第 55 行 devToolsEnabled 默认取 import.meta.env.DEV，第 63-64 行注释明确"仅开发模式显示调试面板；生产/测试构建不渲染触发器与插槽内容"。即生产构建下整个 section v-if=false，group-write 写按钮插槽根本不渲染。
- 模板结构：第 19-38 行 group-read 全为 load* 只读按钮（loadData / loadRunningProcesses / loadManagedApps / loadAppProcesses / loadOrphans / loadProcessDetails / loadMappingAccess / loadDesignerFile 等），剔除；第 39 行 <template #group-write> 内第 41-135 行为大量写按钮：pdCreate（五分支）/ ppFileUpload / pdEdit（五分支）/ pdDelete（六分支）/ pdPerm / psSerial / psHandover / psSign / psDraft / psModeDelete / psCompleted（十一分支）/ pdMisc（八分支）/ pdEngine（十四分支）/ dataWrite（九分支）/ pdDesigner2 / pdDesigner3；第 115-135 行 group-config 含只读 GET 与写 POST 混合。
- 写函数裸 toast.success（全部不读响应 type、catch 仅非 2xx）：ppFileUpload(:484 流程资源内容已上传) / pdCreate(:501 已创建) / pdEdit(:523 已更新) / pdDelete(:540 已删除) / pdPerm(:552 权限已设置) / psSerial(:564,:569) / psHandover(:586,:592) / psSign(:610) / psDraft(:621,:625,:629,:634) / psModeDelete(:645 confirmMsg 守卫) / psCompleted(:677 管理操作已提交) / pdMisc(:708 设计器操作已提交) / pdEngine(:755 引擎操作已提交) / dataWrite(:792 数据操作已提交) / pdDesigner2(:857) / pdDesigner3(:880)。
- 剔除项：展示类 toast（:219/:233/:248/:258/:309/:1160/:1189/:1196）与全部 load* 只读加载不判缺陷；confirmMsg 来自 frontend/apps/desktop/src/utils/toast.ts。
- SDK frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 不查 type，仅非 2xx 抛错——误报前提（200+type:error 不 reject）成立。

## 三、后端铁证（失败信封成立，但入口不可达）
- shared/response.rs 中 ActionResult::error 恒以 HTTP200 返回（o2server 兼容既有设计）。
- processplatform_assemble_designer/src/lib.rs：:334/:369 process definition not found、:638/:669/:704/:724 application not found、:842 applicationdict not found、:1074/:1109/:1144/:1178/:1216/:1248 file not found、:1853 item_access not found、:2007/:2042 mapping not found、:2166 mergeitemplan not found。
- 代表性写 handler：file_id_upload（:1239-1256）UPDATE PP_E_FILE SET xcontent WHERE xid=$2，命中 0 行时 :1247-1248 返回 ActionResult::error(file not found)；application_id_update（:625-640）查不到时 :638 返回 error(application not found)。routes.rs 均注册可达（:37 applicationdict POST / :38 DELETE / :53 file/upload/{id} POST 等）。

## 四、真浏览器实测（决定性对照：门控后不可达）
- 脚本（gitignored）：test-results/r416-probe.cjs + r416-buttons.cjs；证据目录 test-results/r416-pc-processmanager-write/。
- DOM 审计展开前 before：url=/oa4rust/app/admin/process-manager，标题命中，foldTrigger=0 / foldSection=0 / foldClass=0 / details=0 / btnCount=5；九个代表性写按钮（建应用字典 / 建映射 / 改应用 / 流程资源上传 / 删应用字典 / 建流程实例 / 执行实例 / 应用字典分页 / 删设计器对象）DOM count 全为 0。
- 展开后 after：真实尝试点击 [data-testid=pm-endpoint-fold-toggle]（不存在）并对所有 details.endpoint-group（0 个）置 open=true，全部审计值不变，写按钮仍全 0。
- 按钮文本审计：5 个按钮为 [深色模式, 主菜单, 搜索, 通知, 刷新]（☾深色模式 / ☰ / ⌘K / 🔔 / 刷新），roleButtons 全为导航与设置项，foldSection=0 / foldClass=0 / details=0，writeWordsInBody=[]（页面正文无任何建 / 改 / 删 / 保存 / 提交 / 上传 / 新建字样）。
- 三张截图（r416-dom-before.png / r416-dom-after.png / r416-probe-done.png）sha256 完全相同：1342db1646725e8b614ebe391f0a5f623636f1e93fe003fb7ffe88d9c9039fac（1440x900 非物理真机，页面无状态变化）；目视确认仅流程列表 + 搜索 + 刷新，无写入口、无高级操作折叠触发器。
- 真后端假 ID 探针（走真后端不拦截）：appUpdateFake PUT designer/application/{fake}=200 error(application not found)；procUpdateFake PUT designer/process/{fake}=200 error(process not found)；appPermFake POST designer/application/{fake}/permission=200 error；fileUploadFake POST designer/file/upload/{fake}=200 error(file not found)；engineExecFake POST service/processing/execute/{fake}=200 success(data.id / status=processing / taskId)属成功响应不误报；completedOpinionFake POST surface/taskcompleted/opinion/manage/{fake}=200 error(taskcompleted not found)。失败信封成立。
- pageErrors=[]，探针 totalWrites=6；写按钮不在 DOM，无法构造"UI 点击触发 200+error 仍弹成功 toast"的决定性对照——这正是判 0 缺陷的依据，而非取证不足。

## 五、三分判定
- 0 个用户可达真缺陷：生产构建下 EndpointFold 整段 v-if=false，所有 group-write 写按钮插槽不渲染；展开前后 DOM 写按钮均为 0、fold=0，命中队长判定口径④"门控后不可达"，同 R388 CmsIndexApp 先例。虽源码写函数存在裸 toast.success 且后端确实产生 200+type:error（误报前提成立），但用户无法在测试服/生产构建触达，不构成用户可达缺陷。
- L1 潜在隐患（据实记录，不计缺陷）：若 DEV 构建（import.meta.env.DEV=true）展开高级操作面板，上述 16 个写函数在 200+type:error 下会误报成功；建议后续统一加 writeSucceeded(type 为主不强求 bool) 守卫。
- 正确模式剔除：load* 只读按钮、展示类 toast、group-config 中纯 GET 探测均不判缺陷；engineExecFake 返回 200 success data 属产品语义差异，不误报。

## 六、取证限制与产品语义差异
- 破坏性真写未执行，全部用假 ID 探针替代（UPDATE/DELETE 命中 0 行，无副作用）。
- 非 2xx（500/403）、prompt/confirm 取消零写、失败/空态在"写按钮生产构建不可达"前提下不适用，据实标注；未伪造 UI 点击凑数。
- 产品语义差异：ActionResult error HTTP200 为 o2server 兼容；process/execute 假 ID 仍返回 success+processing 是真实成功语义（异步实例已受理），不属误报。

## 七、结论与最小修复建议
- 结论：判 0 个用户可达真缺陷；源码裸 toast.success 群列为 L1 隐患（仅 DEV 构建可达）。
- 建议（非本轮范围）：①若未来开放 DEV 高级操作面板，为上述写函数统一引入 writeSucceeded(resp)（type 为主不强求 bool）守卫，失败 toast.error 透传 message 并 return，成功才 toast.success；②保持 EndpointFold 生产门控（符合"调试面板仅开发模式"的设计意图）。
