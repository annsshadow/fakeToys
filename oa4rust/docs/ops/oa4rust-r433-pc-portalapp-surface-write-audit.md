# R433 PC 门户管理 surfaceOps 写链误报成功甄别记录

> 取证环境：部署测试服 + Playwright Chromium headless 1440x900（非物理真机），写请求全程走真后端不拦截，破坏性真写未执行，用假 ID 触发 0 行 UPDATE（无副作用）替代；scriptByName 首次真创建 1 条假 portal/name 测试数据据实披露。
> 敏感信息：本文所有主机/域名均以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，真值仅存在于 gitignored 证据目录，不入库、不回显。

## 一、结论（三分判定）

- P1（1 个用户可达真缺陷，三全满足）：surfaceOps 写链统一裸 toast.success(门户表面操作已提交) 不读响应 type——真后端返回 HTTP 200 + type:error 时（surface not found / dict not found / script already exists），用户看到绿色「已提交」成功信号，实际操作未生效。8 分支共用一个成功 toast，3 条代表性写分支决定性对照全部坐实。
- 剔除只读/无失败前提分支：portalPage(:437-438)、pageById(:439-441)、dictData(:442-447) 为 api.get 只读展示剔除；create(:430-432) 后端恒 success（create_surface lib.rs:182-216）；dictDel(:456-463) 删字典 0 行仍 200 success(deleted:0)（lib.rs:2762-2784 无 error 分支），均不构成 200+error 误报前提。
- 无额外 L1：surfaceOps 本身即 P1；未发现其它需登记的静默吞错隐患。

## 二、甄别范围与源码定位

- 文件：frontend/apps/desktop/src/views/PortalApp.vue；路由 main.ts 门户管理，实测访问 /app/admin/portal。
- 模板：:15-20 / :26-27 普通 <details class="endpoint-group"> 内 8 个 @click="surfaceOps(...)" 写/读按钮，生产可见真实可达；非 EndpointFold 门控。门控 grep（EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose）全为 0。
- E2E DOM：url=/oa4rust/app/admin/portal / title=门户管理=true / details=4 / fold=0 / 发布门户·存字典数据·删字典数据·按名建脚本按钮各 1。
- surfaceOps(:428-476)：:430-471 八分支 await 后 :472 无条件 toast.success(门户表面操作已提交)，不读响应 type/success；catch(:473-474) 仅非 2xx。dictDel 分支 :460 confirmMsg 守卫（本页 import { confirmMsg, toast } 已正确导入 :101）。
- 后端 portal_assemble_surface/src/lib.rs：publish_surface(:339-391) :356-357 UPDATE 0 行 → Ok(Json(ActionResult::error("surface not found"))) HTTP200；dict_dictFlag_portal_portalFlag_path_data_post(:2787-2857) :2810-2811 UPDATE 0 行 → error("dict not found") HTTP200；script_portal_portal_name_post(:2953+) :2970-2971 同名查重 → error("script already exists") HTTP200；create_surface(:182-216) 恒 success；dict_..._delete(:2762-2784) 0 行仍 success(deleted:0)。路由同 lib.rs:578 POST /surface/create、:581 POST /surface/publish/{id}、:626 DELETE /dict/.../data、:628 POST /dict/.../data、:647 POST /script/portal/{portal}/name/{name}。ActionResult error HTTP200 为 o2server 兼容约定。
- SDK sdk/src/api.ts：parseJsonEnvelope(:82-88) 仅 JSON.parse 不查 type；request():275 仅非 2xx 抛错（401 refresh / 403 PermissionError / 其余 httpStatusError），2xx 原样返回。故 200+type:error 走 await 不抛错，前端不读 type 即误报成功。

## 三、真后端假数据探针

- 证据 gitignored probe.json（apiMasked=<HOST>）。publishFake = HTTP200 / type:error / surface not found；dictSetFake = HTTP200 / type:error / dict not found；dictDelFake = HTTP200 / type:success / deleted:0；scriptByNameFirst = HTTP200 / type:success（真创建假 portal/name 1 条）；scriptByNameDup = HTTP200 / type:error / script already exists；publishNoSession = HTTP401 / unauthorized。三条代表性写分支误报前提坐实，无副作用 UPDATE 0 行。

## 四、决定性对照与响应体铁证

- 全程不拦截写响应，page.on(response) 抓 newWrites 铁证（排除注入伪造），writesDelta 各 1。
- 发布门户：真点「发布门户」→ prompt 填假 ID → 真 POST /api/portal/assemble/surface/publish/{fake} = HTTP200，响应体 {type:error, message:surface not found, prompt:ExceptionEntityNotExist}；toasts 含 status|✓门户表面操作已提交。误报坐实。
- 存字典数据：真点「存字典数据」→ 真 POST /api/portal/assemble/surface/dict/{fake}/portal/.../root/data = HTTP200 {type:error, message:dict not found}，仍弹 ✓门户表面操作已提交。误报坐实。
- 按名建脚本：同 portal/name 连点两次——首次 200 type:success（真写入 1 条假数据），第二次真 POST 同端点 = HTTP200 {type:error, message:script already exists}，仍弹 ✓门户表面操作已提交。误报坐实。
- 截图 publish/dictSet 两张 sha256 相同（同时机画面：绿 toast + 暂无门户页面），两条独立写请求的响应体铁证在 evidence.json.writes 分别留存（surface not found / dict not found），据实记录。

## 五、非 2xx / 取消 / 失败空态 / 0pageerror

- 非 2xx：注入发布端点 500 → 红 toast「✗操作失败: HTTP 500: Internal Server Error」；403 → 红 toast「✗操作失败: 没有权限执行该操作」。非 2xx 由 SDK 抛错进 catch，路径正确，与 200+error 误报形成鲜明对照。
- 取消零写：删字典数据 confirm 确认框 overlay=1，点「取消」writesDelta=0，无成功 toast；发布门户 prompt 直接 dismiss，writesDelta=0（未发请求，红 toast「请求的资源不存在或已被删除」系 dismiss 后空 ID 走真后端的独立副作用，零写成立）。
- 失败/空态：errorState hasFail=false / retryBtns=0（列表注入 500 未生效，据实列取证限制），emptyState emptyHint=true（暂无门户页面，可区分）。P1 判定不依赖此项。
- pageErrors=0，totalWrites=6。

## 六、三分判定与最小修复建议

- P1（可达缺陷）：surfaceOps(:472) 在真后端 200+type:error 下统一弹 ✓门户表面操作已提交，三全=真 UI 可复现（details 内按钮生产可见）+ 真请求响应体（3 分支 200+error）+ 源码定位（:472 裸成功 toast）。
- 剔除：3 条只读 GET 分支；create 恒 success；dictDel 0 行恒 success(deleted:0)无 error 前提，confirm 取消零写已坐实。
- 正确模式：非 2xx → SDK 抛错 → catch 红 toast；confirm/prompt 取消零写。
- 最小修复：新增 portalSurfaceWriteSucceeded(resp)（type 为主、不强求 bool），surfaceOps 各写分支 await 接响应守卫，失败 toast.error 透传 message 并 return 不弹成功，仅成功才走 :472 toast.success；保持只读分支与 confirm 守卫不变。
- 产品语义差异：ActionResult::error 以 HTTP 200 返回是 o2server 兼容约定，前端必须按 body.type 而非 HTTP 状态判断写结果。

## 七、证据与取证限制

- 证据目录（gitignored）：test-results/r433-pc-portalapp-surface-write/，含 evidence.json、probe.json、dom-probe.json 与截图；真域名仅留该目录占位为 <HOST>，不入库。
- 截图 sha256（命令行 sha256sum 原始值，1440x900 headless 非物理真机）：
  - r433-decisive-publish.png = b7ce69ed3b85fd891e927afb5d0828680fd25f2b106faada28d8d10b64f29b9b（绿 toast 门户表面操作已提交 + 真后端 200 error，已目视）
  - r433-decisive-dictset.png = b7ce69ed3b85fd891e927afb5d0828680fd25f2b106faada28d8d10b64f29b9b（与上同时机画面，响应体铁证在 evidence 区分，已目视）
  - r433-decisive-script.png = f47c0a60c919d997e9468e45ac6616b878f82bcd6a2229f68d745fcbe145c456（两次绿 toast + script already exists，已目视）
  - r433-500.png = 22fb4821c21ed5d52f06a42b4df4704b5342000d2642b7b580aac7502aea8913
  - r433-403.png = b38311dc97375377bab0e82fce4b55c11fbba5757968a369ac7beb205e7338fd
  - r433-confirm-cancel.png = 22491895b1073a475ad89c770063913fae5d3d933e82aaf48f983d663a9a8ed2
  - r433-error-state.png = 026c6c134ae974484dc34425bb2434975f6fa9a794d8d045fddcdd83ef9de864
  - r433-empty-state.png = 22491895b1073a475ad89c770063913fae5d3d933e82aaf48f983d663a9a8ed2
  - r433-dom-audit.png = 22491895b1073a475ad89c770063913fae5d3d933e82aaf48f983d663a9a8ed2
- 取证限制：破坏性真写未执行用假 ID 触发 0 行 UPDATE 替代；scriptByName 首次真创建 1 条假 portal/name 测试数据据实披露；失败态列表注入未生效据实记录，P1 判定不依赖；8 分支按代表性覆盖（3 条写分支决定性对照，3 条只读 GET + create/dictDel 按后端语义剔除）；截图 publish/dictSet sha 相同系同时机画面。
- 提交：仅本文一个文档精确 git add，不 -A/.；push 单次即止（系统级 TLS 故障按队长口径不重试），本地 commit 落地回报 SHA 与 AHEAD。