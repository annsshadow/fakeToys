# R314 契约健康度静态审计（过时契约 / API 漂移 / 白名单）

## 来源与边界

- 任务：t80（R314），工作分支 opt1000/oa4rust-pc-ai-refactor，本轮**纯静态审计**——不部署测试服、不跑真浏览器、不改任何源码/契约/后端。
- 立项背景：R311 终审曾发现 3 个过时契约（calWriteCE 误列黑名单、TDZ 朴素 indexOf、白名单漏登新端点）；R313 发现 4 处 API 封装路径/方法漂移，且既有覆盖率测试「401/404 也算通过」不校验路径。本轮系统性排查同类问题是否仍潜伏。
- 审计输入（均为只读）：
  - 前端封装：`frontend/packages/apis/src/index.ts`（1462 行，revision 78de778c…）
  - 覆盖率测试：`frontend/packages/apis/src/api-coverage-generated.test.ts`
  - 端点守卫：`tests/contracts/desktop-endpoints.test.ts`、`tests/contracts/mobile-endpoints.test.ts`、`tests/contracts/route-methods.test.ts`
  - 后端路由快照：`tests/contracts/backend-registered-routes.fixture.ts`（4177 行）、`tests/contracts/backend-route-methods.fixture.ts`（4266 行，path→methods，程序解析得 2618 条）
  - 后端真实源码：各 `backend/crates/<crate>/src/routes.rs` 与部分 `lib.rs` 的 `.route(...)` 注册
- 方法：程序化提取 index.ts 全部 `api.get/post/put/delete/upload('…')` 调用（共 200 处），路径归一（`${}`→`{}`、去尾斜杠、小写）后与 `BACKEND_ROUTE_METHODS` 对照；对每条不一致**再回到后端真实 `.route` 源码交叉验证**，不臆造。
- 脱敏：本轮不接触测试服，文档不含任何域名/IP/主机/口令真值，统一以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位；无截图（纯静态审计，无真浏览器取证）。

## 逐域对照表（方法漂移核对）

对照口径：前端封装声明的方法 vs 方法级 fixture（`BACKEND_ROUTE_METHODS`）+ 后端真实 `.route` 注册。仅列「无 @dead 标注、声明为已对齐」且存在差异者；R313 已修三封装单列 pass。

| 封装 | 源码定位 | 前端声明 | fixture / 后端真实注册 | 判定 |
| --- | --- | --- | --- | --- |
| generalApi.dictItemCreate | index.ts:284 | POST `/api/general/dict/item/{dictId}` | fixture 仅 GET `/api/general/dict/item/{}`；后端真实 `general_core_entity/src/lib.rs:1078` 为 **POST /api/general/dict/item/create** | 路径+方法双漂移 |
| jpushApi.templateUpdate | index.ts:355 | PUT `/api/jpush/core/entity/template/{id}` | fixture 仅 GET；后端 `jpush_core_entity/src/lib.rs:274`、`jpush/src/routes.rs:16` 均只 **GET**（template_get），无 PUT | 方法漂移（405 风险） |
| queryViewApi.search | index.ts:495 | GET `/api/queryview/search` | fixture ['POST']；后端 `query_assemble_surface/src/lib.rs:617` 为 **POST**（u2::search_post） | 方法漂移（405 风险） |
| queryViewApi.importModelList | index.ts:491 | GET `/api/queryview/importmodel/list/{query}/{flag}` | fixture ['POST']；后端 `query_assemble_surface/src/lib.rs:404` + mcp generated_routes 确认 **POST** | 方法漂移（405 风险） |
| programCenterApi.moduleInvoke | index.ts:538 | POST `/api/program_center/invoke/{flag}` | fixture ['DELETE','GET','PUT']；后端 `program_center/src/routes.rs:468` 为 **GET+PUT+DELETE**，POST 只在 `/invoke/{flag}/execute`（routes.rs:632） | 方法漂移（405 风险） |
| documentApi.create | index.ts:603 | POST `/api/document` | 后端 `cms_assemble_control/routes.rs:526` POST | ✅ R313 已修 pass |
| documentApi.update | index.ts:604 | POST `/api/document/{id}/update` | 后端 routes.rs:527 POST | ✅ R313 已修 pass |
| programCenterApi.dictCreate | index.ts:529 | POST `/api/program_center/dict` | 后端 program_center/routes.rs:566 POST | ✅ R313 已修 pass |
| apis.general.dictCreate | index.ts:280 | POST `/api/general/dict/create` | 后端 general_core_entity/lib.rs:1073 POST（不同命名空间，非同源） | ✅ R313 已核 pass |

说明：另有 131 条「路径未在 fixture 命中」（归一化后），集中在 programCenterApi(34)、mindApi(20)、orgApi(18)、queryViewApi(13)、correlationApi(12) 等，多为动态路径模式与快照占位不一致，或已标 `@dead` 的遗留 o2web 表面。它们不构成「方法漂移」证据，作为「fixture 覆盖局限」记录，不臆断为缺陷。

## 已坐实缺陷清单（静态可证，无需真浏览器）

### F1（结构性，high）packages/apis 不在任何端点守卫扫描范围

- 证据：
  - `tests/contracts/desktop-endpoints.test.ts:37` `desktopSrcRoot = resolve(..., '../../frontend/apps/desktop/src')`，`collectPaths()` 只 walk 该根（:49）。
  - `tests/contracts/route-methods.test.ts:22-25` `SRC_ROOTS = [frontend/apps/desktop/src, mobile/src]`，`collectCalls()` 只遍历这两个根（:66）。
  - `tests/contracts/mobile-endpoints.test.ts` 扫描移动端源码 + 内联白名单。
  - 三者均**未包含** `frontend/packages/apis/src`。
- 事实：R313 的三处漂移（documentApi.create/update、programCenterApi.dictCreate）全部位于 `packages/apis/src/index.ts`，三个守卫都没有拦住，只能靠临时新增的 `apiWrapperDrift.R313.contract.test.ts` 专门契约才发现——这直接证明了盲区存在。
- 结论：`packages/apis` 作为独立发布包（`package.json` name `@oa4rust/apis`），其封装路径/方法 correctness 目前无 CI 系统性守卫，任何新增/回归漂移都可能静默潜伏。属静态坐实的结构性缺口。
- 建议修复：将 `packages/apis/src` 纳入 `route-methods.test.ts` 的 `SRC_ROOTS`（或新增同型守卫），使 `api.get/post/put/delete('…')` 调用同样受 `BACKEND_ROUTE_METHODS` 方法校验；对历史存量按基线收敛，不一次性失败。

### F2（覆盖率测试，medium）api-coverage-generated.test.ts 无路径真断言，401/404 也算通过

- 证据：`frontend/packages/apis/src/api-coverage-generated.test.ts` 全文统一模式——
  ```ts
  const fn = module?.login
  expect(fn).toBeDefined()
  expect(typeof fn).toBe('function')
  try { await fn?.() } catch (e) { /* 401/404 也是有效覆盖 */ }
  ```
  （如 :34-46 authApi.login，其余数百用例同构）。
- 事实：仅断言「函数存在且可调用」，异常被吞；只要函数抛出（含路径错误导致的 404、方法错误导致的 405）一律算通过。它无法发现任何路径/方法漂移——R313 漂移能潜伏与该测试口径直接相关。这与 R313 文档 §漂移根因 的记载一致。
- 结论：坐实的测试有效性缺陷（覆盖率驱动而非路径正确性驱动）。
- 建议：R313 已用 `apiWrapperDrift.R313.contract.test.ts`（vi.mock + `toHaveBeenCalledWith` 真断言方法+URL）为 3 个封装打了样；应对其余关键封装按同型补真路径断言，或至少对 F1 纳入守卫后由守卫兜底。属长期收敛项，非本轮源码改动范围。

### F3（静态不一致，high；线上影响待确认）5 处封装方法/路径与后端注册不符

- 证据：见「逐域对照表」5 行，每行都有前端源码定位 + fixture + 后端真实 `.route` 源码三方对照。这 5 处**无 `@dead` 标注**，即前端声明为「已对齐真实路由」，但实际不一致——静态事实坐实。
- 影响面（如实披露）：在 `frontend/apps/desktop/src` 与 `mobile/src` 中 grep 这 5 个封装名**无调用方命中**；在 `packages/apis/src` 内仅被覆盖率测试调用。这与 index.ts 头部注释「本包当前无视图消费方」一致。故其**线上故障影响当前未被证实**，不臆断为已在生产触发 405/404。
- 判定：方法/路径不一致是静态坐实缺陷；「是否造成线上 405」列为待真浏览器/接入后确认候选（见下节）。
- 建议修复（后续轮次，本轮不改）：
  - jpushApi.templateUpdate：后端若确无 PUT，改为前端对齐已有方法或补 `@dead` 标注说明；
  - queryViewApi.search / importModelList：前端改 GET→POST（后端真实为 POST）；
  - programCenterApi.moduleInvoke：对齐 `/invoke/{flag}` 的 GET/PUT/DELETE，或改打 `/invoke/{flag}/execute` POST；
  - generalApi.dictItemCreate：路径 `item/{dictId}`→后端 `item/create`（并核对 dictId 传参语义）。

## 待确认候选（需真浏览器 / 接入后验证，不臆造）

1. F3 五处的线上影响：当前无视图消费方，若未来被桌面/H5 视图调用，GET→POST 类错配将返回 405、路径错配返回 404。需在真实接入路径上以真浏览器抓包确认 status + contentType，再决定是否修前端或补后端方法别名。
2. 131 条未注册路径：应在后续轮次逐条区分「fixture 快照模式未覆盖的动态路径」（正常）与「真实写向未注册路径」（缺陷），以真浏览器请求结果为准，不据静态字符串直接定性。
3. 覆盖率测试补真断言后是否会暴露更多历史漂移：F1 守卫纳入 `packages/apis` 后再评估，属于下一轮回归噪声，本轮不预估数量。

## 已复核无残留项（pass）

- R311 三过时契约均已修复到位，无回退：`CalendarApp.business.contract.test.ts:54-59`（calWriteCE 移出黑名单、改 `\b` 词边界、新增 `data-testid="calendar-create-entry"` 正向断言）；`CalendarApp.tdz.contract.test.ts:135`（花括号栈深度判定，消除函数内同名调用误抓）；`tests/contracts/mobile-endpoints.test.ts:95`（补登 `/api/calendar_assemble_control/calendar` 等 4 条）。
- 契约假断言扫描：181 个 `*.contract.test.ts` 中 `expect(true)` / `it.skip` / `describe.skip` / `.only` 命中 **0**。
- 朴素 indexOf 使用：桌面契约中确实存在大量 `source.indexOf(...)`（ScriptWorkbench.detailRace、XformRuntime.requiredA11y、main.adminGuard、aiDesignTokens、colorScheme 等），但均为「函数体源码切片定位」的合法顺序断言，不属于 R311 那类「顶层调用位置被函数内同名调用误抓」的 TDZ 假阳性模式；不列为缺陷。
- R313 三封装路径/方法修复已在 commit 77603c529 完成并经终审 t78 pass，本轮再次核对与后端注册一致。

## 结论

- 本轮静态审计达成三项任务：①完成 `packages/apis` 全部 200 处封装调用与后端方法注册的程序化对照；②扫描三类端点守卫与契约过时模式；③抽查覆盖率驱动测试路径断言必要性。
- 产出 **3 条静态坐实缺陷**：F1 守卫盲区（high，结构性）、F2 覆盖率测试无路径真断言（medium）、F3 五处封装方法/路径漂移（high，静态不一致，线上影响待确认）；另有 3 项待确认候选与 4 类 pass 项。
- 未发现「已在线触发的 405/404 故障」证据——5 处漂移当前无视图消费方；本轮保守定性，不臆造线上事故。
- 本轮为纯静态审计，仅交付本文档，未改任何源码/契约/后端；建议后续轮次按 F1→F3→F2 优先级修复，每处修复走 RED→GREEN 并以真浏览器抓包确认。
- 项目 ≥1000 轮持续迭代，本轮为其中一环，非终态。
