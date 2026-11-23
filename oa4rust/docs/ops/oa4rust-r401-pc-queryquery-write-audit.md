# R401 PC 查询定义 QueryQueryApp.vue 表面查询写链甄别（失败信封误报成功）

## 1. 审计范围与方法

- 任务：只读甄别（不改码）PC 端 QueryQueryApp.vue「表面查询（surface）」写链，在后端 HTTP 200 + type:error 失败信封下是否误报成功。
- 方法：源码逐函数甄别 + 后端 handler 信封核对 + 真浏览器（Playwright Chromium headless，1440×900，非物理真机）真 UI 流程实测。
- 真缺陷三全判据：真 UI 可复现 + 真请求响应 + 源码定位，三者齐备才判真缺陷。
- 破坏性真写（新建真表面 / 修改真数据 / 删真行）未授权，一律不执行；用假 ID + 注入列表行 + 写响应不拦截走真后端的决定性对照替代。

## 2. 环境来源与真实性标注

- 测试服地址：由既有脚本读取，本文不记录真实域名/IP，统一以 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位。
- 登录凭据：本机 SSH 到测试机后由脚本从环境读取，本文不含任何口令/密钥真值。
- 截图来源：Playwright 无头 Chromium 对已部署测试服的真实页面截图，视口 1440×900，非物理真机、非模拟器冒充；证据与脚本位于被 gitignore 的 test-results/r401-pc-queryquery-write/。

## 3. 源码甄别（逐函数分类，文件 frontend/apps/desktop/src/views/QueryQueryApp.vue，共 373 行）

- 路由：main.ts:254-258，path=query-query，name=QueryQueryApp，标题「查询定义」；全文无 EndpointFold 门控，生产构建真渲染（E2E DOM 审计 fold=0）。
- 【P1 真缺陷】saveSurface(:303-327，模板第 90 行 @click「保存」)：:314 POST /api/query/assemble/surface/save/{id}（编辑）或 :317 POST /api/query/assemble/surface/create（新建），await 返回值未接、未读 type；:319 无条件 toast.success（表面已保存），:320 关弹窗，:321 刷新列表；catch(:322-324) 仅捕获非 2xx。后端 200+type:error 被 SDK 原样 resolve 进 try，仍弹成功。
- 【P1 真缺陷】deleteSurface(:328-338，模板第 74 行 @click「删除」，confirmMsg 守卫 :329)：:332 POST /api/query/assemble/surface/delete/{id}，await 后 :333 先本地 filter 删行、:334 无条件 toast.success（表面已删除）；catch(:335-337) 仅非 2xx。后端 200+type:error 时本地行已被移除且弹成功，状态与后端不一致。
- 【正确模式，不判缺陷】CRUD 主列表 saveItem(:215-219) → saveM(:198-214)：onSuccess 仅 qc.invalidateQueries + closeModal，无成功 toast；onError toast.error(mutationErrorMessage)。deleteItem(:228-233) → delM(:221-227)：onSuccess 仅 invalidateQueries，无成功 toast，onError toast.error。两者不误报，与表面查询写链分离。
- 【只读，剔除】loadSurfaces(:271-282，GET surface/list/{category})、viewSurface(:283-292，GET surface/get/{id})、loadByCategory(:240-253)、loadData(:234)/reload、loadData 主列表加载——均非写。
- 【死代码】无：模板所有写函数均有真实 @click 入口。
- SDK：frontend/packages/sdk/src/api.ts parseJsonEnvelope(:82-88) 不查 type，请求封装仅非 2xx 抛错，2xx+type:error 原样 resolve（是误报链路的共性前提）。
- confirmMsg：自定义 DOM 弹窗 #oa4-confirm-overlay，按钮「取消」/「确认」（utils/toast.ts:182/238/270），非原生 dialog。

## 4. 后端信封铁证（backend/crates/query_assemble_surface/src/lib.rs）

- save_surface(:269-307)：UPDATE x_query_surface 0 行时 :291-292 返回 Ok(Json(ActionResult::error("query surface not found")))，即 HTTP 200 + type:error；成功时返回 success + saved:count。
- delete_surface(:310-336)：DELETE 0 行时 :323-324 同样返回 200 + type:error；成功时 success + updated:count。
- create_surface(:127-161)：成功 success + id，无 error 分支。
- list_surfaces(:165-225)：用 legacy_success（data 数组 + count），与 save/delete 的标准 ActionResult 信封不同；E2E 注入按此形状构造。
- 路由 lib.rs:360-377：surface/get/create/list/save/delete/preview 均注册可达。
- ActionResult::error 走 HTTP 200 是 o2server 兼容的既有设计（共享 response.rs），非本轮缺陷；问题在前端不读 type。

## 5. E2E 实测证据（job bash-83c09c13dffc4469，exit 0，0 pageerror，totalWrites=7）

- S0 DOM 审计：surface 标题 1、新建表面按钮 1、主列表新建按钮 1、表格 1、fold=0；页面标题查询定义（截图 queryquery-dom-audit.png）。
- S1 真后端假 ID 探针（fetch 直打，不注入，UPDATE/DELETE 0 行无副作用）：surfaceSaveFake=200 type:error「query surface not found」；surfaceDeleteFake=200 type:error 同文案；surfaceListProbe=200 type:success data:[]（截图 queryquery-probe-done.png）。
- S2 决定性对照（只注入 surface/list 列表 1 行 R401测试表面 / R401-FAKE-SURFACE，写响应完全不拦截、走真后端）：
  - 2a 取消守卫：点删除 → #oa4-confirm-overlay 点「取消」，writesDelta=0、无 toast、列表行仍在（queryquery-cancel-guard.png）。
  - 2b 编辑保存：真点编辑→保存，真 POST /api/query/assemble/surface/save/R401-FAKE-SURFACE = 200 type:error（响应体 query surface not found 已捕获），仍弹「✓表面已保存」（queryquery-decisive-save.png，sha256 e98de28aea570f63fa540b8230c8a4a70b2cfd9b10950d254cc219b022767b49）。
  - 2c 删除确认：真点删除→「确认」，真 POST /api/query/assemble/surface/delete/R401-FAKE-SURFACE = 200 type:error（响应体已捕获），仍弹「✓表面已删除」，且本地列表行被移除 rowStill=false（queryquery-decisive-delete.png）。
- S3 人造 200+type:error stub：编辑保存仍弹「✓表面已保存」（queryquery-stub-save.png）——仅佐证前端不读 type，真后端有此形状故 S2 已独立坐实，stub 不单独作为判据。
- S4 非 2xx：500 →「✗保存表面失败: HTTP 500: Internal Server Error」；403 →「✗保存表面失败: 没有权限执行该操作」（queryquery-http500.png / queryquery-http403.png），错误路径正确。
- S5 失败态/空态：列表 500 →「⚠️表面查询加载失败：HTTP 500: Internal Server Error」+ 重试按钮 1；空 data →「该分类暂无表面查询」，二者可区分（queryquery-fail-state.png / queryquery-empty-state.png）。
- 全程 pageErrors=[]。

## 6. 三分判定

- 结论：1 个 P1 真缺陷（用户可达、三全满足）。saveSurface / deleteSurface 两条真实 UI 写链在后端 HTTP 200 + type:error 失败信封下无条件弹成功 toast；deleteSurface 还在失败时错误地移除本地行。
- 通过项：CRUD 主列表 saveItem/delItem 正确模式不误报；confirm 取消零写；500/403 正确 toast.error；失败态与空态可区分；0 pageerror；无 EndpointFold 门控，写按钮真渲染。
- 取证限制：破坏性真写（新建/改真数据/删真行）未授权未执行，用假 ID + 注入列表 + 写响应不拦截的对照替代；新建分支 create 未单独真触发（走真后端会真实 INSERT，未授权）；按代表性覆盖两处裸 toast 写函数，未逐条穷举全部提示分支。
- 产品语义差异：ActionResult::error 走 HTTP 200 为 o2server 兼容既有设计；list_surfaces 用 legacy_success 而 save/delete 用标准信封，属既有实现差异，非本轮缺陷。

## 7. 最小修复建议（本轮不改码）

- 对齐既有范式（R363 方案 B / R397 appinfoWriteSucceeded）：新增 querySurfaceWriteSucceeded(resp:any)，以 resp?.type === error 为主判失败（不强求权威 bool，本端点失败信封无权威 bool）。
- saveSurface：接住 POST 响应，失败则 toast.error(透传 message) 并保留弹窗、不刷新；成功才 toast.success + 关弹窗 + 刷新。
- deleteSurface：接住 POST 响应，失败则 toast.error 且不删本地行；成功才 filter 删行 + toast.success。
