# R372 PC 流程实例管理页写操作链甄别（只读审计）

- 轮次：R372 / 任务 t190；角色：qa-e2e（缺陷审计与浏览器实测）。
- 被测：`oa4rust/frontend/apps/desktop/src/views/ProcessManagerApp.vue`（路由 `main.ts:314` path `process-manager`）。
- 环境：部署测试服（域名占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，真值不入库）；Playwright Chromium headless 1440x900，**非物理真机**；凭据经 ssh 运行期读取、不回显。
- 方法：真浏览器登录后进入 `/app/admin/process-manager`，DOM 审计 + 真后端假 ID 探针 + 失败/空态注入；破坏性真状态翻转写未授权不执行。

## 1. 判定结论：0 个用户可达真缺陷（死代码潜在隐患另记）

按三全判据（真 UI 可复现 + 真请求响应 + 源码定位）逐项核对：本页写函数虽在源码中大量 `await` 后端响应后无条件 `toast.success`，但其唯一 UI 入口是 `EndpointFold` 高级操作面板，而该组件在生产/测试构建中 `import.meta.env.DEV` 为 false，`v-if="devToolsEnabled"` 使整个折叠面板（含全部写按钮）不渲染。真浏览器 DOM 审计：页面仅 22 个可交互按钮（导航 + 刷新），`[data-testid=pm-endpoint-fold]` 存在数=0，写按钮数=0。用户无法在测试服 UI 触达任何写函数，故不满足“真 UI 可复现”，不判真缺陷。

## 2. 源码甄别（只读）

- 写函数群（均为真实源码但无生产 UI 入口）：`pdCreate`(:492-505)、`pdEdit`(:506-527)、`pdDelete`(:528-544)、`pdPerm`(:545-556)、`psSerial`(:558-574)、`psHandover`(:575-597)、`psSign`(:598-614)、`psDraft`(:615-639)、`psModeDelete`(:640-649)、`psCompleted`(:651-681)、`pdMisc`(:683-712)、`pdEngine`(:714-759)、`dataWrite`(:761-796)、`pdDesigner2`(:798-861)、`pdDesigner3`(:863-884)、`ppFileUpload`(:~480)。
- 典型裸成功提示：`:501` `${kind} 已创建`、`:523` `${kind} 已更新`、`:540` `${kind} 已删除`、`:677` 管理操作已提交、`:708` 设计器操作已提交、`:755` 引擎操作已提交、`:792` 数据操作已提交、`:857`/`:880` 流程设计器操作已提交：均 `await api.*` 后直接调用，未读取响应 `type` / `data` 权威布尔。
- 守卫对照：`pdDelete`(:531) 用原生 `window.confirm`；多数删除分支用 `confirmMsg`（自定义 DOM 弹窗 `#oa4-confirm-overlay`）；伪取消路径会 `return` 不发请求。
- 入口甄别：`EndpointFold.vue:6` `v-if="devToolsEnabled"`，`:55` 默认 `import.meta.env.DEV`；`ProcessManagerApp.vue:17` 调用未传 `devToolsEnabled`，故生产构建隐藏整段。对比 R364/R365 的死代码甄别口径：无生产 UI 入口的代码不判用户可达缺陷。
- 后端契约（静态确证）：`processplatform_assemble_designer/src/lib.rs:50/334/369/638` 等失败统一 `Ok(Json(ActionResult::error(...)))`，即 HTTP 200 + `type: error`；`shared/src/response.rs:188-200` `ActionResult::error` 设 `type=error`、`message`、`size=-1`、`prompt=ExceptionEntityNotExist`，但 HTTP 状态仍 200。`frontend/packages/sdk/src/api.ts:275-296` 仅非 2xx 抛错，2xx 的 `type: error` 原样 resolve，调用方不检查 type 即误判成功。

## 3. 真浏览器实测（1440x900 headless，非物理真机）

- S0 DOM：进入 `/app/admin/process-manager`，标题「流程实例管理」可见；全部按钮 22 个，均为导航/刷新；展开折叠面板的定位器 `[data-testid=pm-endpoint-fold-toggle]` 与 `[data-testid=pm-endpoint-fold]` 均 0 命中（3 轮一致），写按钮 0 个。
- S1 真后端假 ID 探针（`R372-NOSUCH-*`，fetch 直打，9 端点）：`PUT designer/application/{fake}`=200 + `type:error` / application not found；`DELETE designer/mapping/{fake}`=200 + error；`DELETE designer/applicationdict/{fake}`=200 + error（两分支）；`DELETE surface/serialnumber/{fake}`=200 + error；`DELETE surface/taskcompleted/{fake}/manage`=200 + error；`POST service/processing/data/work/{fake}/delete`=200 + error；`GET designer/process/{fake}/disable`=200 + error；例外 `POST service/processing/cancel/{fake}`=404 + error。结论：与 R366/R368/R370 同型的 HTTP 200+type:error 业务失败信封在后端确实可达，但本页无 UI 入口触达。
- S2 决定性对照（不拦截写 route，真响应）：因写按钮不渲染，尝试点击「改应用/删映射/删应用字典」三轮均定位超时（clicked=false），无法形成“真 UI 点击 + 真请求响应 + 源码定位”三全；不判真缺陷。
- S3 取消守卫：原生 confirm 与 DOM `confirmMsg` 两种取消路径均未触发写请求（writesBefore=writesAfter，zeroWrites=true），但同样因按钮不可见未产生真实点击，仅作记录。
- S4 失败态/空态：列表 GET 注入 500 与空数组后，页面渲染与空态可区分（截图 `pm-fail-state.png` / `pm-empty-state.png`），0 pageerror。
- 证据：`test-results/r372-pc-processmanager-write/evidence.json`、`pm-dom-audit.png`、`pm-probe-done.png`、`pm-decisive-toasts.png`、`pm-cancel-guard.png`、`pm-fail-state.png`、`pm-empty-state.png`（gitignored，非物理真机）。

## 4. 三分判定

- 真缺陷：0 个。后端 200+error 信封可达且源码有无条件成功提示，但测试服生产构建无真实 UI 入口，三全不满足；注入伪造 200 响应不能替代真 UI 复现，不据以判缺陷。
- 死代码潜在隐患（L1，非本轮真缺陷）：上述写函数群若未来经 `devToolsEnabled=true` 或新入口暴露，会以 R362/R366/R368/R370 同型方式误报成功。最小修复方向对齐 R363 方案 B：新增写响应判定（判 `type` 为 success 且权威布尔严格为 true，否则 `toast.error(message)`），各写函数 `await` 后先判再提示。
- 取证限制：破坏性真状态翻转/真删未授权未执行，采用假 ID 探针；测试服生产构建隐藏调试面板，无法用真 UI 点击这些写端点；非 2xx 错误 toast 本轮仅 S0-S4 覆盖列表与探针，写端点错误路径因无入口未能 UI 触发。
- 产品语义差异：`EndpointFold` 显式 `v-if=import.meta.env.DEV` 收纳 API 调试按钮，生产隐藏属既有设计；假 ID 探针中 `service/processing/cancel` 走 404 而 designer/surface 多数走 200+type:error，属后端模块实现口径差异，非缺陷。

## 5. 覆盖甄别与交付

- `docs/ops` 命中 `oa4rust-r303-dr07-admin-heading.md`（视觉）、`oa4rust-r332-default-slot-audit.md:47-51`（插槽语义，注明未逐一展开写端点），均非写链失败信封专项；本轮首次真浏览器甄别，属有效增量。
- 本轮不改源码、不改契约、不改后端；仅交付本文档。截图与探针证据落 gitignored `test-results/`，不入库。
