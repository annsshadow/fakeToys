# R405 PC 语句设计器写链 200+type:error 误报成功 甄别报告

> 角色：qa-e2e（缺陷审计与浏览器实测）｜轮次：R405 / 任务 t237｜性质：只读甄别（未改任何业务代码）
> 测试环境：1440x900 headless Chromium 针对已部署测试服的真实页面流程；非物理真机，截图均来自该 headless 会话。
> 破坏性真写（建/改/删真语句）未执行，以假 ID + 写响应不拦截的真后端决定性对照替代。

## 一、结论（三分判定）

判定：1 个用户可达的 P1 真缺陷（三全满足）。

- 缺陷：QueryStatementDesigner.vue 的 saveByFlag（:1048-1062）通过真实 UI 按Flag保存按钮（模板:16）调用 PUT /api/query/assemble/designer/statement/{flag}，在后端返回 HTTP 200 + type=error（statement not found）时，于 :1057 无条件 toast.success（语句已按 flag…保存）并刷新列表，不读取响应 type。
- 三全：真 UI 可复现（真点按钮 + prompt 填 flag）+ 真请求响应（真后端 200 error 响应体，写路由全程不拦截）+ 源码定位（:1053 await 后 :1057 裸成功提示，catch :1059 仅覆盖非 2xx）。
- 连带影响：失败仍触发 queryClient.invalidateQueries（:1058）刷新列表，用户误以为保存成功且列表已更新。

## 二、源码甄别（逐函数分类）

| 函数 | 入口 | 分类 | 判定依据 |
| --- | --- | --- | --- |
| saveByFlag :1048-1062 | 模板:16 真实 @click 按Flag保存，需 currentStatement 非空 | 真实可达写（P1） | :1053 await PUT 后 :1057 裸 toast.success，不读 type；catch 仅非 2xx |
| deleteStatement :1083-1086 | 模板:63 行内删除按钮 | 真实可达写，正确模式不判缺陷 | useMutation delM onSuccess 仅刷新、onError toast.error（:1075-1082），无成功 toast；confirmMsg 守卫 :1084 |
| saveStatement / modalSave :1034-1074 | 模板:15 保存、弹窗:136 | 真实可达写，正确模式不判缺陷 | saveM onError toast.error、onSuccess 仅刷新关弹窗（:1036/:1042-1045），无成功 toast |
| executeBulkDelete :2024-2040 | 无外部入口：showBulkDelete 唯一置 true 在弹窗内部 confirmBulkDelete（:2021-2022），bulkIds/toggleBulk 无 UI 绑定 | 死代码，不判缺陷 | DOM 审计确认删除按钮命中 0；弹窗无法被打开，形成自循环 |
| doImport :1977-2004 | showExportImport 无打开入口（仅 ref 定义 :1948） | 死代码 + 计数消息（非裸 toast.success），不判缺陷 | 用 importMsg.txt 计数文案，无成功 toast |
| runBatch :1345-1362 | 模板:25 批量 → :236 开始执行 可达 | 计数变体，L1 观察非本任务缺陷 | batchResults.push（success）按 try/catch 计数，2xx+error 不 reject 会误计 success，但无操作成功 toast，属执行结果计数非成功 toast 误报范围 |
| executeSQL :1088-1110 | 模板:14 执行 | 正确模式，不判缺陷 | 结果写 resultData、catch 写 execError + toast.error，无成功 toast |
| loadStatements / selectStatement / loadSchema 等 | 只读 | 只读探测，剔除 | 无写请求 |

说明：队长静态侦察将 executeBulkDelete 列为强候选，本轮据模板与真 DOM 审计纠正为死代码；其 Promise 循环 try/catch 计数变体口径在 runBatch 中仍以 L1 观察记录。

## 三、后端与 SDK 信封分析

- 后端 query_assemble_designer/src/u2_closures.rs statement_edit（:292-346）：admin 门通过（:299-302），alias 查重命中→:321 ActionResult::error（statement alias already exists）；UPDATE x_query_statement WHERE id=$4（:325-331）影响 0 行→:333-335 Ok（Json ActionResult::error statement not found），HTTP 200。成功才返回 success + id/updated。
- 路由 lib.rs:915：/api/query/assemble/designer/statement/{flag} 的 PUT 绑定 statement_edit，与前端 saveByFlag 的 api.put 精确命中。
- SDK frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 仅解析 JSON 不检查 type；:296 附近仅在非 2xx 抛 ApiError。故 200+type=error 原样 resolve，await 不 reject，前端走进无条件成功分支。
- ActionResult::error 走 HTTP 200 是 o2server 兼容既有设计（同批次 R390/R392/R398/R403 一致），非本轮引入。

## 四、真浏览器 E2E 决定性对照（1440x900 headless，非物理真机）

脚本与证据（均 gitignored）：test-results/r405-pc-querystatement-write.cjs、test-results/r405-pc-querystatement-write/run.sh、test-results/r405-pc-querystatement-write/evidence.json + 8 张截图。

1. S0 DOM 审计：按钮 62、页面标题 SQL语句设计器 命中、按Flag保存 1、确认删除 0（坐实批量删除死代码）、EndpointFold 0（无门控、生产真渲染）、空态文案 暂无语句定义 可见。
2. S1 真后端假 ID 探针（fetch 直打，不触发 UI 破坏性写）：
   - statementEditFake（PUT statement/{假ID}）= 200 type:error statement not found，
   - statementDeleteFake（DELETE）= 200 type:error statement not found，
   - designerDeleteFake（DELETE designer/delete/{假ID}）= 200 type:error query design not found or already deleted，
   - listAll = 200 success + data 数组（list 成功信封，非 error）。
3. S2 决定性对照（注入 list/all 仅 1 行 R405测试语句 + R405-NOSUCH 假 flag；写路由 PUT 全程不拦截走真后端，排除注入伪造质疑）：
   - 选行后 .stmt-name=R405测试语句、按Flag保存按钮 enabled 可点；
   - 真 UI 点按Flag保存 + prompt 填假 flag → 捕获真 PUT /designer/statement/{假ID} = 200 type:error statement not found（newWrites 真响应体），writesDelta=1，仍弹绿 toast 语句已按 flag R405-NOSUCH 保存，误报成功坐实；
   - prompt 取消 → dialogs=[prompt]、writesDelta=0、无新写（取消零写守卫成立）；
   - 截图 r405-decisive-saveflag.png sha256=76d79832e8a5e87b8d943a8683df97a1f01e433b86c8bae7e07f6349ad12a3a0，肉眼确认绿 toast 与注入行并存。
4. S3 非 2xx：500 注入 → 红 toast 按 flag 保存失败 HTTP 500 Internal Server Error（截图 r405-500.png sha256=76bd219ec139bd68f63094f37e06e2456fc951d855cd6f9d69fb2bc82ad8e10e）；403 注入 → 红 toast 按 flag 保存失败 没有权限执行该操作。均走 catch 正确 toast.error。
5. S4 失败/空态（新 context 规避 useQuery 缓存）：空 data → 暂无语句定义 空态可见、rows=0（r405-error-state.png sha256=1e07cbaf332355ac3667a5dcba3bf6d5fe5d3607f730bc3082b9689db9acb9a8 目视确认可区分）。
6. pageErrors=[]，totalWrites=6。

## 五、通过项

- 500/403 非 2xx → catch → toast.error 正确，不弹成功；
- prompt 取消零写（writesDelta=0）；
- 空态 暂无语句定义 可见、与有数据态可区分；
- deleteStatement / saveStatement useMutation 正确模式无误报；
- 无 EndpointFold 门控，真实 UI 入口可达；
- 全程 0 pageerror。

## 六、取证限制与产品语义差异

1. 破坏性真写（改真语句 flag 内容、真删语句）未授权执行，以假 ID + 写响应不拦截的真后端决定性对照替代；假 ID 只在 marker 层制造 200 error，未触碰真实数据。
2. 失败态文案未坐实：注入 list/all 500（type:error）后列表进入 暂无语句定义 空态、未出现 语句列表加载失败 提示、retryBtns=0；据实记录为取证限制，不虚报失败态通过（空态可区分成立）。
3. executeBulkDelete / doImport 据模板与 DOM 审计判为死代码，未做 UI 触发；runBatch 计数变体按代表性覆盖，未做 2xx+error 专项注入（属 L1 观察非本任务范围）。
4. ActionResult::error 走 HTTP 200 是 o2server 兼容既有设计；SDK 不查 type 是同型批次共性，非本轮引入。

## 七、最小修复建议（不在本轮实施）

对标 R363/R390/R402/R404 既有范式，新增 queryStatementWriteSucceeded（resp:any）：boolean（type 为主、不强求权威 bool：type=error 或 success=false→false，type=success 或 type 缺省且 success 非 false→true，非对象→false）。saveByFlag 接住 api.put 响应，成功提示前统一守卫：失败时 toast.error（resp.message 或 操作未生效请刷新后重试）并 return，不弹成功、不 invalidateQueries；成功分支才提示与刷新。死代码 executeBulkDelete / doImport 若后续启用需同口径处理计数变体。

> 证据与截图均为 headless Chromium 1440x900 测试服实跑，来源如实标注，非物理真机。
