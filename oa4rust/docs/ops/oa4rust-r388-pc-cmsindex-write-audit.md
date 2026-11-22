# R388 PC CMS索引设计写操作链失败信封甄别报告

- 轮次：R388（qa-e2e 只读甄别，不改码）
- 甄别对象：PC 端 `CmsIndexApp.vue` CMS 写操作链在后端 HTTP 200 业务失败信封下的 UI 反馈一致性
- 实测环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；**非物理真机**。测试域名/主机均为占位符，真值不入库。
- git 基线：`58ea9a6cf5f9927dd9faf73ffbb799a3dd00988f`（HEAD 与上游一致，工作树干净）

## 一、任务与范围

甄别 CMS 写操作是否存在与 R366/R368/R370/R373/R381 同型的缺陷：后端返回 HTTP 200 + type:error（或 200 + type:success + 权威布尔为 false）业务失败时，前端 await 后无条件弹成功 toast，误导用户。判真缺陷需三全：真 UI 可复现 + 真请求响应 + 源码定位。破坏性真状态翻转写未授权不执行，用假 ID + 只注入列表行走真后端不拦截写响应替代。

## 二、源码甄别（真实入口 / 门控隐藏 / 只读探测三分）

文件相对路径 `frontend/apps/desktop/src/views/CmsIndexApp.vue`（919 行）；路由 `main.ts:302-304` path=cms-index/name=CmsIndexApp，重定向 `main.ts:484` → /app/admin/cms-index。

**大量裸成功提示的写函数群（被生产构建门控隐藏，用户不可达）**：
- createCmsCategory（:386-396，:392 toast「分类已创建」）、createCmsArticle（:397-408，:404）、toggleViewPublish（:409-426，:418/:421）、cmsDelete（:429-444，:440 含 confirmMsg 守卫）、cmsSaveConfig（:445-453，:449）、cmsUpdateDocument（:454-464，:460）、cmsRoot（:466-513，:509，约 23 个 op 分支含 confirmMsg）、cmsWrite2（:562-608，:604，含 confirmMsg）、cmsWrite3（:610-630，:626）——全部 await 后无条件 toast.success，不读 type/权威 bool。
- 这些按钮全部位于模板 `#group-write` / `#group-config`（模板 :60-113）的 `<EndpointFold>` 内。组件相对路径 `frontend/apps/desktop/src/components/admin/EndpointFold.vue`：第 6 行 `<section v-if="devToolsEnabled">`，第 55 行 `devToolsEnabled: import.meta.env.DEV`。测试服为生产构建（DEV=false），整段高级操作面板不渲染。
- 真 DOM 审计坐实：`[data-testid=cms-index-endpoint-fold]` 命中 0、`.endpoint-fold` 命中 0，10 个写按钮文案（新建分类/新建文章/发布视图/删字典/更新文档/存控制配置/批量删文档等）命中全 0。与 R372（ProcessManagerApp EndpointFold 门控）同型。

**唯一真实可达的写（列表 CRUD，在 EndpointFold 外）**：
- 模板 :11 openCreate → 新建弹窗 → :147 saveItem；:132 editItem / :133 deleteItem；:139 弹窗。
- saveItem（:738-742）→ saveM（:726-737，POST index/create 或 PUT index/save/{id}）；deleteItem（:751-756，confirmMsg 守卫 :753）→ delM（:744-750，POST index/delete/{id}）。
- 二者 onSuccess 仅 `qc.invalidateQueries` 刷新列表、无成功 toast；onError 走 `toast.error(mutationErrorMessage(e))`。即成功/业务失败都不弹成功提示。

**只读探测（不判缺陷）**：loadCmsConfig/Overview/Express/Details/FormDetails/AliasForm/ViewRecords/AppReads/AppReads2/DeepReads、cmsQuery（:515-532）、cmsQuery3（:534-560）、loadCmsTwin/2/3 —— 均为读端点（GET/PUT/POST 查询），不在本轮写链甄别范围；其 toast「查询已提交」属读操作反馈。

## 三、真后端信封实测（假 ID 探针，不注入）

在测试服真浏览器内 fetch 直打（API base `/oa4rust-api`，域名占位省略），坐实 CMS 后端信封形状：

| 探针 | 方法/路径 | HTTP | 响应信封 |
| --- | --- | --- | --- |
| catCreateMissing | POST /api/cms/category/create | 200 | type:success + data.id（name-only 创建成功） |
| viewPublishFake | POST /api/cms/view/publish/{假ID} | 200 | type:error，message=cms view not found，size=-1，prompt=ExceptionEntityNotExist |
| cmsIndexCreate | POST /api/cms/core/entity/index/create | 200 | type:success + data.created:true + data.id |
| cmsIndexDeleteFake | POST /api/cms/core/entity/index/delete/{假ID} | 200 | type:success + data.deleted:false |
| dictDeleteFake | DELETE /api/cms/assemble/control/dict/delete/{假ID} | 200 | type:success + data.deleted:false |
| docUpdateFake | POST /api/document/{假ID}/update | 200 | type:error，message=document not found |

坐实：CMS 后端失败口径混合——找不到实体类走 200 + type:error（如 view publish、document update），幂等删除类走 200 + type:success + 权威布尔 deleted:false（如 index/dict delete），与共享 `response.rs:188-200`（ActionResult::error 为 HTTP 200）一致。SDK `api.ts:82-88` parseJsonEnvelope 不检查 type、:275-296 仅非 2xx 抛错，故两种失败形状都会原样 resolve。

## 四、决定性对照（真实 UI 点击 + 真请求响应，写响应不拦截）

注入列表 1 行假 ID 索引（R388 注入索引），写请求完全走真后端、不拦截写路由：

- **取消守卫**：点删除 → confirmMsg 弹窗「确定删除该索引？」→ 点取消 → 写请求数 6→6，zeroWrites=true，守卫有效（confirmMsg 为 `#oa4-confirm-overlay` 自定义 DOM 弹窗，非原生 dialog）。
- **真删决定性对照**：再次点删除 → 确认 → 真 POST index/delete/{假ID} = HTTP 200 + type:success + deleted:false → UI toast 列表为空（`toasts=[]`），列表静默刷新。即真实失败信封下 **未误报成功**（因列表 CRUD 本无成功 toast）。
- **非 2xx**：注入 delete 500 → toast「✗操作失败，请重试」正确错误。
- **空态**：注入 data:[] → 「🔖暂无索引」，与列表态可区分。
- **失败态**：注入 index/list 500 → 本轮未捕获「加载失败」提示（retryBtns=0），据实记为取证限制，不虚报通过（与 R375 同型的注入时机/缓存问题）。
- 全程 0 pageerror。截图（gitignored）：cms-dom-audit / cms-probe-done / cms-injected / cms-cancel-guard / cms-decisive-delete / cms-http500 / cms-empty-state / cms-fail-state。

## 五、三分判定

**1）真缺陷：0 个用户可达真缺陷。**
- 高级批量写函数群虽有裸 toast.success 误报形状，但全部位于生产构建被门控隐藏的 EndpointFold 内，真 DOM 审计写按钮命中 0，用户不可达，与 R372 同型，不判缺陷。
- 唯一可达的列表 CRUD 用 useMutation，onSuccess 静默刷新无成功 toast；决定性对照真删假ID=200 success deleted:false 下 toasts=[]，无误报成功，三全不满足。

**2）取证限制。**
- 破坏性真状态翻转/真删未授权不执行，用假 ID + 注入列表行走真后端不拦截写响应替代。
- 高级写函数群的误报形状仅在源码 + HTTP 探针层坐实（200+type:error、200+success+deleted:false 确实可达），未做 UI 触发（生产构建无入口）。
- 失败态注入未坐实（list 500 后未见失败提示、retryBtns=0），据实记录；空态已坐实。
- 探针 catCreateMissing / cmsIndexCreate 触发 2 个真创建（新增非破坏性写，返回真实 id），据实标注。
- 全量写分支按代表性覆盖，未逐一 UI 触发。

**3）产品语义差异。**
- 后端 ActionResult::error 使用 HTTP 200 是 o2server 兼容既有设计（共享 response.rs:188-200），非本轮改动。
- categoryinfo/index 的 name-only 假名也能创建属既有 handler 宽松校验，非本轮缺陷。
- EndpointFold 的 devToolsEnabled 门控是 R301 调试端点收纳的有意设计（生产隐藏）。

**L1 潜在隐患（非本轮缺陷，仅记录）**：高级写函数群裸 toast.success，若未来 devToolsEnabled 放开或入口暴露，会复现 R362/R366/R368/R370 同型 P1；列表 CRUD 在 deleted:false 时静默刷新、用户无明确失败反馈。最小修复方向对齐 R363 方案 B：新增写响应成功判定（以 type 为主 + 权威布尔严格 true，否则 error 透传 message），门控内写函数 await 后先判再提示，CRUD onSuccess 判权威布尔决定提示成功或失败。本轮不改码。

## 六、覆盖甄别

docs/ops 下 CMS 相关文档均为视觉/插槽/建表/版块跨表/写链无关主题，从未专项甄别 CMS 写链失败信封 UI 反馈，本轮首次专测，构成有效增量。
