# R390 PC 查询视图设计器写操作链失败信封甄别报告

- 轮次：R390（qa-e2e 只读甄别，不改码）
- 甄别对象：PC 端 \`QueryViewApp.vue\` 查询视图/设计器写操作链在后端 HTTP 200 业务失败信封下的 UI 反馈一致性
- 实测环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；**非物理真机**。测试域名/主机均为占位符 \`<YOUR_TEST_DOMAIN>\` / \`<YOUR_TEST_HOST>\`，真值不入库。
- git 基线：\`b5411dbf811c127cf3610910628b1512f371e162\`（HEAD 与上游一致，工作树干净）

## 一、任务与范围

甄别查询视图写链是否存在与 R366/R368/R370/R373/R381 同型的缺陷：后端返回 HTTP 200 + type:error 业务失败时，前端 await 后无条件弹成功 toast，误导用户。判真缺陷需三全：真 UI 可复现 + 真请求响应 + 源码定位。破坏性真状态翻转/清空表/删真行/建表写未授权不执行，用假 ID + 不拦截写响应走真后端替代。

覆盖甄别：docs/ops 命中 r344-queryview-audit、r345-queryview-search-envelope-fix、r306/r332 视觉与默认插槽修复等，均为视觉/搜索信封/插槽，从未专项甄别写链失败信封 UI 反馈，本轮首次专测有效增量。

## 二、源码甄别（真实入口 / 死代码 / 只读探测三分）

文件相对路径 \`frontend/apps/desktop/src/views/QueryViewApp.vue\`（919 行）；路由 \`main.ts:176-180\` path=queryview/name=QueryViewApp，页面标题「查询视图」。无 EndpointFold / devToolsEnabled 门控（与 R372/R388 门控型不同），写按钮在测试服生产构建真渲染：DOM 审计 75 个按钮，含「删行」「相似检索」「设计器改行」「设计器删行」「设计器清表」「设计器建表」各 1 个，fold=0。

**裸成功提示且有真实 @click 入口的四个写函数（均 await 后无条件 toast.success，不读 type / 权威 bool）**：
- qvWrite（:547-597，12 分支，:593 toast「queryview 操作已提交」，删除/清空分支 :558/:562/:587 有 confirmMsg 守卫，其余 prompt 取参；含相似检索 :591 POST queryview/morelikethis、删行 :559、清空 :563、执行视图 :572）
- qvMore（:599-695，约 24 分支，:691 toast「queryview 操作已提交」，prompt 取参；含视图执行/打包/Excel/统计执行/语句执行/设计器语句执行/设计器导入模型权限等）
- qvRows（:697-727，:723 toast「数据表行操作已提交」，:708/:713 confirmMsg 守卫；含表行插入/清空/删行/部分更新）
- qvDesignerRows（:729-789，模板 :61-66 六个真实按钮；insert :743→:744 toast「设计器表行已插入」、update :757→:758「设计器表行已更新」、save :770→:771「设计器表行已保存」、build :774→:775「数据表构建已触发」、deleteAll :779→:780「设计器表已清空」（confirmMsg :778）、delete :785→:786「设计器表行已删除」（confirmMsg :784））

四个函数 catch（:595/:693/:725/:789）只在 SDK 抛错（非 2xx）时 toast.error，不覆盖 200 + type:error。prompt() 为原生 window.prompt，confirmMsg 为 #oa4-confirm-overlay 自定义 DOM 弹窗（utils/toast.ts:182 导出）。

**只读探测（不判缺陷）**：模板 :15-30 的 doSearch/loadViews/loadQueryList/loadQueryByFlag/loadQvDetails/loadQueryTwin/2/3/loadStatementStat/loadImportModels/loadTables/loadTableRowsCursor/loadQueryViewExtras/loadQueryViewCursors/loadQueryViewDeep/loadQueryViewMore 等 load*/query* 系列；其 toast「查询孪生端点 N 条已提交」（:415/:914/:950）属读操作反馈。

SDK 铁证：frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 仅 JSON.parse 不查 type；:275-296 仅 !resp.ok 抛错（非 2xx 进 catch），2xx + type:error 原样 resolve。

## 三、真后端信封实测（假 ID / 缺参探针，不注入）

在测试服真浏览器内 fetch 直打（API base \`/oa4rust-api\`），坐实 query 系列后端信封形状（7 个探针）：
- morelikethis 空 key → 200 + type:error「key is required」；有关键词无匹配 → 200 + type:success + count:0（正常无数据）
- surface 删行 / 删导入记录假 ID → 200 + type:error「row not found」/「record not found」
- designer 改行 / 删行假 ID → 200 + type:error「row not found」
- view execute 假 ID → 200 + type:error「view not found」

后端源码佐证：query_assemble_surface/src/u2_closures.rs morelikethis_post:667-680 空 key→ActionResult::error、table_row_delete:721-746 0 行→error、importmodel_record_delete:818-843 0 行→error；query_assemble_designer/src/u2_closures.rs table_tableFlag_row_update:1193-1229 0 行→error、table_tableFlag_row_delete:1233-1271 0 行→error。shared/response.rs ActionResult::error 为 HTTP 200 + type:error + size:-1 + prompt=ExceptionEntityNotExist，是 o2server 兼容既有设计。

## 四、E2E 决定性对照（写响应不拦截，走真后端）

1440x900 headless，4 个真实 UI 点击 + prompt 填假 ID，写响应完全走真后端不拦截，逐个捕获真请求响应体：
1. 点「相似检索」填空关键词 → POST queryview/morelikethis 真响应 200 type:error「key is required」→ UI 弹绿色「✓queryview 操作已提交」（误报）
2. 点「设计器改行」填假表 flag/行 ID/JSON → PUT designer/table/row/update/假/假 真响应 200 type:error「row not found」→ UI 弹「✓设计器表行已更新」（误报）
3. 点「删行」填假表 flag/行 ID + confirm 确认 → DELETE queryview/table/row/delete/假/假 真响应 200 type:error「row not found」→ UI 弹「✓queryview 操作已提交」（误报）
4. 点「设计器删行」填假 flag/ID + confirm 确认 → DELETE designer/table/row/delete/假/假 真响应 200 type:error「row not found」→ UI 弹「✓设计器表行已删除」（误报）

每次 writesDelta=1，newWrites 响应体为铁证；截图 qv-decisive-morelike.png / qv-decisive-des-update.png 等目视确认绿 toast 与 200+error 并存（已排除人造注入伪造质疑）。

## 五、通过项 / 取证限制 / 产品语义差异

通过项：confirm 取消守卫（点删行填值后点取消）writesBefore=writesAfter、writesDelta=0、无 toast；非 2xx 注入 500→「✗操作失败: HTTP 500: Internal Server Error」、403→「✗操作失败: 没有权限执行该操作」正确 toast.error；列表 500→「查询视图加载失败」+ 重试按钮 1，空 data→「暂无查询视图」，失败/空态可区分；0 pageerror；totalWrites=13。

取证限制：破坏性真状态翻转写（build 建表、清空表、删真行、designer save/insert INSERT）未授权未执行，用假 ID + 不拦截真后端写响应替代；写分支按代表性覆盖（4 个真 UI 点击 + 7 个探针 + handler 源码），全量约 50 分支未逐一 UI 触发。

产品语义差异：ActionResult::error 用 HTTP 200 是 o2server 兼容既有设计（非本轮改）；morelikethis 有关键词但无匹配返回 200+success+count:0 属正常无数据；表行 INSERT 类（insert/save）为新增行，后端不产生 0 行失败信封，与 update/delete 失败口径不同属既有实现差异。

## 六、三分判定与最小修复方向

判定：**1 个 P1 真缺陷**——四个有真实 @click 入口的写函数（qvWrite/qvMore/qvRows/qvDesignerRows）在后端 HTTP 200 + type:error 业务失败下无条件弹成功 toast，三全满足（真 UI 可复现 + 真请求响应 + 源码定位）。

最小修复方向（对齐 R363 方案 B，本轮不改码）：新增 queryViewWriteSucceeded(resp) 判 type 为 success（或 type 缺省且 success 非 false）为主、对带权威 bool/flag 的端点附加校验；四个写函数 await 后先判响应再提示，失败时 toast.error 透传 resp.message，不再无条件 toast.success。

## 七、交付证据

- 文档：\`docs/ops/oa4rust-r390-pc-queryview-write-audit.md\`（本文件），脱敏占位 0 真值、相对路径、非物理真机标注。
- E2E 脚本与证据（gitignored）：\`test-results/r390-pc-queryview-write.cjs\`、\`test-results/r390-pc-queryview-write/evidence.json\` 与截图 qv-dom-audit / qv-probe-done / qv-decisive-morelike / qv-decisive-des-update / qv-decisive-row-delete / qv-decisive-des-delete / qv-cancel-guard / qv-http500 / qv-http403 / qv-fail-state / qv-empty-state.png。
