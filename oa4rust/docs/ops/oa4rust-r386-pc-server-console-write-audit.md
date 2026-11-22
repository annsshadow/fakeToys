# R386 PC ServerApp 控制台写链失败信封与守卫甄别

- 轮次：R386，任务 t210（只读甄别，不改码）。
- 环境：Playwright Chromium headless 1440x900 对已部署测试服实测；**非物理真机**。
- 目标：甄别 ServerApp.vue 运维写链在后端失败/未生效信封下是否误报成功，以及确认守卫是否实际生效。
- 域名/主机：测试域名与凭据均不入库，统一用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位。

## 覆盖甄别

- 既有 docs/ops 中 R347 剩余页批量审计涉及 ServerApp，但仅验只读渲染（server/license 200）与「破坏性写未授权仅验守卫」，从未专项甄别控制台写链的失败信封 UI 反馈，本轮为首次专测，有效增量。

## 源码甄别（frontend/apps/desktop/src/views/ServerApp.vue）

- 路由：`frontend/apps/desktop/src/main.ts` path `server` / name `ServerApp`（后台路径 /app/admin/server）。
- 运维写操作真实 @click 入口（模板 :35-41）：清理缓存 cacheClear(:249 POST /api/console/cache/clear/{type})、控制台命令 cmdExecute(:253 POST /api/console/command/execute)、广播消息 sendMessage(:257 POST /api/console/send/message)、保存部署 deploySave(:261 PUT /api/server/deploy/save/{id})、删除部署 deployDelete(:266 POST /api/server/deploy/delete/{id})，均由 `serverConsoleOps(op)`(:244-276) 分发；停止服务器 stopServer(:231-240, GET /api/server/stop) 单独入口。均非死代码。
- serverConsoleOps 五个写分支一律 `await api.*` 后不接响应、不读 type/权威 bool，:272 无条件 `execOutput.value = 控制台操作已提交`；catch(:273-275) 仅覆盖非2xx。
- sysConfig(:268 GET /api/config/system/config) 为只读探测，execOutput 展示 JSON，本轮非写链对象。
- 对比组（正确处理）：executeCommand(:204-218) 展示 r.data 并在 catch 写 execError；loadExecuteStatus(:221-229) 同理；loadLicense(:190-202) 失败走 describeListError。
- 只读探测非写：loadSysStatus/loadGeneralMeta/loadGeneralMeta2/loadGeneralMeta3/loadBaseMeta/loadBaseMeta2/loadServerTwin/loadServerTwin2/loadServerTwin3 等，按 R342/R344/R347 口径非缺陷。

## 后端契约（真浏览器探针实测，非臆造）

- `backend/crates/console/src/lib.rs`：clear_cache(:203-229) `cleared = result>0`（删0行仍 `ActionResult::success`，HTTP200，cleared:false）；server_deploy_save(:580-597) `saved = crud_save(...)`（空 payload + 假ID走 `shared/crud.rs` :108-120 的 SELECT 存在性检查，0行→saved:false，HTTP200 success）；server_deploy_delete(:601-617) `deleted = crud_delete(...)`（假ID删0行→deleted:false，HTTP200 success）；send_message(:168-200) 非Admin→AppError::Forbidden(403)；execute_command(:275+) 非法命令→BadRequest(400)。
- `backend/crates/shared/src/response.rs`：ActionResult::success 置 type=success；`backend/crates/shared/src/error.rs` :178-204 AppError 走非2xx（BadRequest 400 / Forbidden 403）。
- SDK `frontend/packages/sdk/src/api.ts` :82-88 parseJsonEnvelope 仅 JSON.parse 不查 type；:275-296 仅非2xx抛错，2xx type:success+saved:false 原样 resolve。
- 路由注册 `backend/crates/console/src/routes.rs`：send/message :21 POST、cache/clear/{type} :22 POST、command/execute :24 POST、server/deploy/save/{id} :32 PUT/:33 POST、server/deploy/delete/{id} :36 DELETE/:38 POST。

## 真浏览器实测（1440x900 headless Chromium，非物理真机）

- 首轮脚本 `test-results/r386-pc-server-console-write.cjs`：DOM 审计 22 按钮含运维操作 6 写按钮（保存部署/删除部署/清理缓存/控制台命令/广播消息/停止服务器）与执行按钮；截图 srv-dom-audit.png。首轮探针 API base 误写为 `/oa4rust/api/` 全 405（fail-loud 如实记录，非产品缺陷），并意外发现 confirmMsg 未导入缺陷。
- v2 脚本 `test-results/r386-pc-server-console-write-v2.cjs`（修正 API base 为 SDK 实际前缀 `/oa4rust-api`，见 `frontend/apps/desktop/src/main.ts:30`）：
  - S1 真后端安全假ID探针：clearCacheFakeType → 200 type:success cleared:false；deploySaveFakeId(空payload) → 200 type:success saved:false；deployDeleteFakeId → 200 type:success deleted:false；cmdExecuteBadCmd → 400 type:error（非法命令白名单校验）。sendMessage 广播属高危未执行探针，据实跳过。
  - S2 决定性对照（只走真后端写响应，不拦截）：真UI点「保存部署」→ 原生 prompt 填 R386-NOSUCH-ID → 真 PUT /api/server/deploy/save/R386-NOSUCH-ID = 200 type:success saved:false，execOutput 仍显示「控制台操作已提交」，截图 srv-v2-decisive-deploy-save.png 目视绿色成功框坐实。
  - S3 confirmMsg 守卫复核：真UI点「删除部署」→ prompt 填假ID → 直接抛 `confirmMsg is not defined` 进 catch，execError 显示「操作失败: confirmMsg is not defined」，confirm 弹窗未出现（hasOverlay=false），截图 srv-v2-confirm-missing.png 目视红色错误框坐实。
  - S4 非2xx：注入 deploySave 500 → execError「操作失败: HTTP 500: Internal Server Error」，不误报成功。
  - S5 失败/空态（首轮）：license 500 → 「授权信息加载失败：HTTP 500: Internal Server Error」+重试；license data null → 「暂无授权信息」，可区分。
- 两轮 pageErrors 均为空。证据 JSON：evidence.json / evidence-v2.json；截图 srv-dom-audit.png、srv-v2-decisive-deploy-save.png、srv-v2-confirm-missing.png、srv-v2-http500.png、srv-fail-state.png、srv-empty-state.png（均在 gitignored test-results/r386-pc-server-console-write/）。

## 三分判定

### 真缺陷（2 个 P1，三全满足）

1. 【P1】控制台写链在后端返回 200+type:success+权威 bool:false（未生效）时误报成功。`serverConsoleOps`(:244-276) 五个写分支 await 后不读响应，:272 无条件写「控制台操作已提交」。三全：真UI点击保存部署 + 真请求 200 type:success saved:false + 源码 :261/:272。影响 cacheClear/deploySave/deployDelete（三者后端均 200+success+bool:false 可达）；sendMessage 亦同型（后端成功 sent:false 可达，但广播高危未真触发，按源码归类）。最小修复方向：对齐 R363 方案B——新增 consoleWriteSucceeded(resp, flagKey)，type 判定为主且权威 bool（cleared/sent/saved/deleted）严格 true 才算成功，否则 execError 透传 message/提示未生效，不写成功文案。
2. 【P1】确认守卫失效致三个高危操作完全无法执行。`ServerApp.vue` :85 仅 `import { toast }`，未在 script 引入 confirmMsg，却在 :232(stopServer)/:248(cacheClear)/:265(deployDelete) 调用 `confirmMsg(...)`。真UI点删除部署抛 `confirmMsg is not defined` 进 catch，用户无法完成清理缓存/删除部署/停止服务器，属功能阻断。三全：真UI点击 + 运行时 ReferenceError 证据 + 源码 :85/:265。最小修复方向：`utils/toast.ts` :182 已导出 confirmMsg，在 ServerApp.vue 的 import 中补入 `confirmMsg`（与 toast 同引自 ../utils/toast）即可恢复守卫。

### 取证限制

- 破坏性高危真写（清缓存真实类型、广播消息、命令真执行、部署真删、停服）未授权，未对真实数据执行；用假ID/假类型/空 payload 探针（删0行/查行无副作用）+ 真UI点击走真后端不拦截的对照替代。
- sendMessage 广播端点因高危未执行探针，仅以源码与后端成功分支 sent:false 归类，未单独 UI 触发坐实。
- stopServer 停服端点高危未真触发（会断开连接），confirmMsg 缺失缺陷以删除部署同因同构坐实，未逐一点停服。
- 首轮探针 API base 错误导致 405 已由 v2 修正复核，不作为产品缺陷。

### 产品语义差异

- console/deploy 写端点失败走 AppError 非2xx（400/403），但业务未生效（删0行/查无行）走 200+type:success+权威 bool:false，是本 crate 既有返回口径，与 o2server 兼容式 200+type:error 不同；两种口径下本页均误报成功，修复须同时覆盖。
- 空 payload 的 deploySave 在 shared/crud.rs 是「查行存在性」而非写入，假ID返回 saved:false 无副作用，用作安全探针。

## 结论

R386 ServerApp 控制台写链甄别发现 2 个 P1 真缺陷（失败信封误报成功、confirmMsg 未导入致守卫失效），均为用户可达真UI可复现，建议另派修复轮次按上述最小方向处理。本轮只读甄别，未改任何源码。