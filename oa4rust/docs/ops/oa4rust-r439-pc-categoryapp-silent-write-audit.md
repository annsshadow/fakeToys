# R439 PC 分类管理 CategoryApp 写链静默吞错甄别记录

> 性质：只读甄别（kind=work，不改任何业务代码）。三分判定：**2 个用户可达 P1 真缺陷**（零反馈静默吞错变体，均三全满足）。
> 本页从未被任何既有审计文档覆盖，属缺口页首次甄别。
> 环境声明：PC E2E 使用 Playwright Chromium headless 1440x900 部署测试服，**非物理真机**；测试服域名/凭据一律占位，真值仅留 gitignored 证据目录，不入库、不回显。

## 一、范围与结论

- 目标文件：frontend/apps/desktop/src/views/CategoryApp.vue（共 144 行）。
- 甄别对象：createCategory（模板 AdminToolbar @create，第 39 行）与 deleteCategory（模板 .btn-del @click，第 29 行）两条写链。
- 剔除：load（第 70-93 行，GET 列表展示）、各列渲染槽、const 占位引用（第 123-134 行，无写语义）、runQuery 类只读函数不在本页。
- 结论：两条写链均在真后端 200+type:error 失败信封下不读响应、直接 load() 静默刷新；由于 catch 仅捕获非 2xx，失败被完全吞掉（既无成功 toast，也无失败 toast），用户零反馈。判 2 个用户可达 P1。
- 与历史先例的形态差异（据实）：本页成功分支本身也不弹成功 toast（纯 load 刷新），故本缺陷是「完全静默/零反馈」变体，而非 R427/R433/R435/R437 那种「弹假成功 toast」变体；因 load() 从后端重拉，列表状态与后端真实状态一致，**无「视觉假成功」信号**。危害集中在失败原因不可见与操作结果无即时确认。减轻因素如实记录，不改变可达性判定。

## 二、源码甄别（逐函数可达性）

- 门控：对 EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 全文件 grep，结果为空——两写链无开发环境门控、无隐藏开关，生产可见。
- 依赖：第 48 行 import { confirmMsg, toast } from ../utils/toast，confirmMsg 已正确导入（区别于 R430 QueryDesigner 漏导入导致 delete 不可达的情形）。
- createCategory（第 95-106 行）：第 96 行 prompt 所属应用 ID，第 97 行空值 return，第 98 行 prompt 名称；第 101 行 await api.post(/api/categoryinfo, {appId, name}) **不接收响应、不读 type/success**；第 102 行直接 load()；第 103-105 行 catch 仅在非 2xx 抛错时 toast.error(新建失败: ...)。200+type:error 走 await 不抛错 → catch 不触发 → 失败零反馈。模板第 39 行 @create 真实可达。
- deleteCategory（第 107-115 行）：第 108 行 confirmMsg 守卫（取消则 return）；第 110 行 await api.delete(/api/categoryinfo/{id}) **不接收响应、不读 type/success**；第 111 行直接 load()；第 112-114 行 catch 仅非 2xx。200+type:error 同样被静默吞掉。模板第 29 行 @click 真实可达。
- load（第 70-93 行）：GET /api/categoryinfo/list/all，失败 describeListError(分类加载失败) 保留旧数据，空态「暂无分类数据」，为只读展示，剔除。

## 三、后端失败信封铁证（误报前提成立）

- crate = cms_assemble_control（队长备料泛称已精确定位到本 crate）。
- 建分类 categoryinfo_u2_create：backend/crates/cms_assemble_control/src/lib.rs 第 10301-10336 行。第 10306-10309 行 appId 空 → Err BadRequest(400) 非 2xx；第 10310-10312 行 gate_sql 查 x_cms_appinfo.owner；**第 10313 行 U2Gate::NotFound → Ok(Json(ActionResult::error(application not found))) = HTTP 200 + type:error**；第 10314 行 Forbidden → Err403；第 10315-10333 行 Allowed 才 INSERT x_cms_categoryinfo 并 success。
- 删分类 categoryinfo_u2_delete：同 lib.rs 第 10340-10347 行，第 10345 行 u2_gate_by_sql(U2_CATEGORY_OWNER_SQL, id, session.person_unique)，第 10346 行转 soft_delete_after_gate(..., category not found)。
- 软删统一形状 soft_delete_after_gate：同 lib.rs 第 551-568 行，**第 559 行 U2Gate::NotFound → Ok(Json(ActionResult::error(not_found))) = HTTP 200 + type:error**；第 560 行 Forbidden → Err403；第 561-565 行 Allowed → 软删后 success(deleted:true)。
- 路由：cms_assemble_control/src/routes.rs 第 574 行 POST /api/categoryinfo→create，第 575 行 DELETE /api/categoryinfo/{id}→delete，第 434 行 GET list/all。
- ActionResult 的 error 以 HTTP 200 返回（o2server 兼容产品语义），非 2xx 仅用于鉴权/参数错误——这是全项目「写链必须读 type」口径的根因。

## 四、SDK envelope 解析判定

- frontend/packages/sdk/src/api.ts 第 82-88 行 parseJsonEnvelope 仅 resp.json()，不查 type/success。
- 第 275-291 行 request()：仅非 2xx 抛错（401 refresh、403 PermissionError、其余 httpStatusError）；第 293-296 行 discardResponse/204→success:true，否则原样返回 envelope。
- 第 303-305 行 post / 第 311-312 行 delete 均不查 type。
- 结论：后端 200+type:error 在 SDK 层不抛错，前端 await 正常返回，若调用方不读 type 即静默吞错——本页两写链正是如此。

## 五、决定性对照 E2E（真后端响应体铁证，非注入伪造）

- 手段：Playwright Chromium headless 1440x900，isMobile=false，非物理真机。两写链均通过**真实 UI 交互触发真实写请求且不拦截**，用 page.on(response) 捕获响应体作为铁证；注入列表行仅为制造可点击的假 categoryId（假 ID → U2Gate NotFound → 0 行写），写请求本身走真后端。
- 准备：注入列表 1 行（id=R439-FAKE-*，name=R439测试分类）。DOM 实测：url=/oa4rust/app/admin/category，标题命中，删除按钮 1、新建分类按钮 1、fold=0，写按钮真实可见。
- 决定性对照·新建：真点「+新建分类」→ prompt 填假 appId 与名称 → 真 POST /api/categoryinfo（不拦截）。捕获 **HTTP 200，body={type:error, message:application not found, prompt:com.x.base.core.project.exception.ExceptionEntityNotExist}**，writesDelta=1。前端无任何 toast（仅有路由切换提示），rowsAfter=1（未新增）——失败被完全静默吞掉。
- 决定性对照·删除：真点「删除」→ confirm 遮罩点确认（overlay=1）→ 真 DELETE /api/categoryinfo/{假id}（不拦截）。捕获 **HTTP 200，body={type:error, message:category not found, prompt:ExceptionEntityNotExist}**，writesDelta=1。前端无任何失败 toast，rowsAfter=1（注入行仍在）——失败被完全静默吞掉。
- 截图（命令行 sha256 原始值，非 read_image 重编码值）：r439-decisive-create.png=8045b82687739524a17bcb986ffd89365aa1d80f271c8bb93dd1c5784c7400aa；r439-decisive-delete.png=904fd700d8f189ecf94be051ae935cd511090254e5ad0d679a9e86d5cfd978dd。两图均已目视：列表仅「共 1 项」、注入行仍在、无任何结果提示。r439-dom-audit.png 与 decisive-create 同 sha（同时机画面，DOM 一致）。

## 六、非 2xx / 取消零写 / 失败空态 / 门控实测

- 非 2xx·新建 500：注入 POST 500 → 捕获红 toast「✗新建失败: HTTP 500: Internal Server Error」，writesDelta=1。
- 非 2xx·新建 403：注入 POST 403 → 捕获红 toast「✗新建失败: 没有权限执行该操作」，writesDelta=1。说明 catch 对非 2xx 工作正常，缺陷仅限 200+error 形状。
- 取消零写：新建 prompt 取消（dismiss）→ writesDelta=0；删除 confirm 取消（overlay=1）→ writesDelta=0。两写链确认守卫有效。
- 失败态：注入 list/all 500 → hasFail=true、retryBtns=1（「分类加载失败」+重试），可区分。
- 空态：注入 list/all 空 data → emptyHint=true（「暂无分类数据」）、retryBtns=0、hasFail=false，与失败态可区分。
- 门控：DOM 实测 fold=0，两写按钮生产可见。
- 0 pageerror：全程 pageErrors=[]，totalWrites=4（两条真实写 + 500 + 403），0 致命错误。
- 其余截图 sha256：r439-500.png=c756a2cc027225a0d450c9853b7acb90fd18f789684b82a496891588a20b07b1；r439-403.png=b329b292a62eec4f14dc02784c4931870785fd442c08d41b2aefef25386001f1；r439-error-state.png=94556ebfd48aa7ba1e5edf00a72f38b0412db5c2d3ca256df2eb573b79e3a2b3；r439-empty-state.png=28f9048f93b951e194b6232c44ec9d37192a923f0f62ac734ca5d2ced0611184。

## 七、三分判定、最小修复建议与取证限制

- 三分判定：**2 个用户可达 P1 真缺陷**（createCategory 第 95-106 行、deleteCategory 第 107-115 行），均满足「真 UI 可复现 + 真请求响应体 + 源码精确定位」三全。误报前提（真后端 200+type:error）已由真后端假数据探针 + E2E 决定性对照双重坐实，非 L1 潜在风险。
- 危害形态据实：完全静默/零反馈变体。失败时 catch 不触发、无成功 toast、无失败 toast；用户填表/确认后无法得到即时结果与原因。因 load() 从后端重拉，列表不会显示假成功（区别于乐观移除行/假成功 toast 变体），此减轻因素已如实记录，但「失败零反馈、原因不可见」仍是可达的核心写操作缺陷，需修。
- 最小修复建议（供后续修复轮，本轮不改码）：新增 categoryWriteSucceeded(type 为主，不强求 bool) 判定；两写链 const resp = await api.post/delete(...)，失败则 toast.error 透传 resp.message（如「所属应用不存在」/「分类不存在」）并 return（不 load，保留用户上下文），仅成功才 load()。confirm 守卫、prompt 输入、catch 非 2xx 分支与端点/payload/method/模板保持不变。
- 取证限制（据实披露）：① 破坏性真写未执行——用假 appId/假 categoryId 触发 U2Gate NotFound、0 行 gate，真后端探针+注入列表替代，无任何真实分类被新建或删除；② 两写链按代表性全覆盖（各一条决定性对照），列表其余只读列未逐一验证；③ 注入列表行是制造可点击假 ID 的手段，写请求响应体来自真后端而非注入伪造。

（证据文件：test-results/r439-pc-categoryapp-silent-write/probe.json 与 evidence.json，位于 gitignored 目录；真域名与凭据占位为 <YOUR_TEST_DOMAIN> / <TEST_HOST> / <HOST>，不入库。）