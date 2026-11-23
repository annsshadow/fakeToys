# R430 PC 查询设计器静默刷新吞错写链甄别记录

> 取证环境：部署测试服 + Playwright Chromium headless 1440x900（非物理真机），写请求全程走真后端不拦截，破坏性真写未执行，用假 ID 触发 0 行 UPDATE（无副作用）替代。
> 敏感信息：本文所有主机/域名均以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，真值仅存在于 gitignored 证据目录，不入库、不回显。

## 一、结论（三分判定）

- P1（1 个用户可达真缺陷，三全满足）：saveQuery 静默吞错新维度——真后端返回 HTTP 200 + type:error（query design not found）时，前端不读 type，直接关闭 modal 并刷新列表，用户看到「保存成功」的视觉信号（弹窗消失），实际一条记录都没存下。
- 0 缺陷（独立观察项，不属本维度）：deleteQuery 写链完全不可达——源码未导入 confirmMsg，点击删除触发 Vue runtime-5 ReferenceError，确认框不出现，DELETE 写请求 0 次。
- 剔除只读：runQuery(:613-633) execute 查询执行只读展示；openEdit(:565-584) 回读；loadQueries(:308) 列表加载。

## 二、甄别范围与源码定位

- 文件：frontend/apps/desktop/src/views/QueryDesigner.vue（约 942 行）；路由 main.ts:230-232 path=query-designer，:520 redirect，实测访问 /app/admin/query-designer。
- 门控：EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 全为 0，无生产门控；E2E DOM：title=true / createBtns=1 / editBtns=1 / fold=0。
- saveQuery(:586-611)：模板第 124 行 modal 内保存按钮真实 @click；:599-602 依 editingQuery.id 走 api.put(save/{id}) 或 api.post(create)，接响应但不读 type/success；:604 showModal=false、:605 loadQueries()；catch(:606) 仅非 2xx 弹 toast.error。saveSqlQuery(:651-659) 复用同一函数。
- deleteQuery(:635-644)：:636 confirmMsg 守卫、:638 api.delete(delete/{id})、:639-640 本地清选中并 filter 移除行；但 imports 段(:274-280)未导入 confirmMsg（对比 ViewApp.vue 标准 import { confirmMsg, toast }）。
- 后端 query_assemble_designer/src/lib.rs：save_designer(:607-645) :614 require_admin_opt，:621-628 UPDATE x_query_design WHERE id=$4 AND deleted_at IS NULL，:630-631 result==0 返回 ActionResult::error(query design not found) 即 HTTP 200 + type:error，:634 成功才 success；delete_designer(:648-679) :665-669 0 行返回 error(query design not found or already deleted)，:671 成功。路由 :888 DELETE /delete/{id}、:889 PUT /save/{id}。ActionResult error HTTP 200 为 o2server 兼容约定。
- SDK sdk/src/api.ts：parseJsonEnvelope(:82-88) 仅 JSON.parse 不查 type；request():275 仅非 2xx 抛错；put/post(:303-308) 不查 type。故 200 + type:error 走 .then 不抛错，前端不读 type 即静默吞错。

## 三、决定性对照与响应体铁证

- 真后端假 ID 探针：saveFakeId_authed = HTTP 200 / type:error / query design not found；deleteFakeId_authed = HTTP 200 / type:error / query design not found or already deleted；无会话均 401。0 行 UPDATE 无副作用。
- 决定性对照（写响应全程不拦截、page.on(response) 抓 newWrites）：注入列表 1 行 R430测试查询（假 ID R430-FAKE-ROW-*）→ 真点编辑 → modal 改名 → 真点保存 → 真 PUT /api/query/assemble/designer/save/{fake} 走真后端 = HTTP 200，响应体 {type:error, message:query design not found, prompt:ExceptionEntityNotExist}，writesDelta=1；结果 modalAfter=0（弹窗关闭）、列表行未变、无成功 toast——视觉成功但实际未保存，静默吞错 P1 三全坐实。
- 对照非 2xx：注入 save 端点 500 → 红 toast「✗保存失败: : HTTP 500: Internal Server Error」且 modalAfter=1（modal 保持打开）；403 → 红 toast「✗保存失败: : 没有权限执行该操作」且 modalAfter=1。非 2xx 由 SDK 抛错进 catch，路径正确，与静默吞错形成鲜明对照。

## 四、deleteQuery 不可达独立观察

- 真点列表删除按钮：确认框 #oa4-confirm-overlay 始终 overlay=0，行仍在（rowsAfter=1），新增写请求 0（writesAfter=1 与上一步 save 相同，无 DELETE），console 新增 error:[unhandled] runtime-5 ReferenceError: confirmMsg is not defined（构建产物 QueryDesigner-BZNRgnXl.js:1:11192），0 pageerror（Vue 将组件异常吞成 console error）。
- 结论：删除写链在当前构建完全不可达，不产生 200+error 误报也不产生任何写，故本维度判 0 缺陷；但「未导入 confirmMsg 致删除功能静默失效」本身是独立源码功能缺陷，建议单独立项修复（补 import）。

## 五、非 2xx / 取消 / 失败空态 / 0pageerror

- 非 2xx：500/403 均红 toast 且 modal 保留，已坐实（见第三节）。
- 取消零写：deleteQuery 确认框因 confirmMsg 未导入根本不出现，取消路径无法真 UI 触发；modal 保存的取消按钮为本地关闭，不产生写请求（本轮未单独计数，据实记录）。
- 失败/空态：errorState hasFail=false/retryBtns=0/items=0，emptyState emptyHint=true/items=0。错误态注入未生效（列表由真后端数据/时序覆盖），未坐实失败态文案；空态有无数据提示可区分。据实列为取证限制，P1 判定不依赖此项。
- pageerror=0（全程 0 pageerror；唯一 console error 为不可达删除的 ReferenceError 及注入 500/403 的期望资源加载失败）。totalWrites=3。

## 六、三分判定与最小修复建议

- P1（可达缺陷）：saveQuery(:604-605) 在真后端 200+type:error 下关 modal + 刷新列表，视觉成功但未生效。三全=真 UI 可复现 + 真请求响应体 + 源码定位。
- L1（隐患，不判缺陷）：无额外 L1——saveQuery 本身即 P1；deleteQuery 不可达属独立功能缺陷。
- 正确模式：非 2xx → SDK 抛错 → catch 红 toast 且 modal 保留；runQuery/openEdit/loadQueries 只读剔除。
- 最小修复：新增 queryDesignerWriteSucceeded(resp)（type 为主、不强求 bool），saveQuery 的 PUT/POST 接响应守卫，失败 toast.error 透传 message 且不关 modal、不刷新列表，成功才关 modal 并刷新；另补 import confirmMsg 修复删除不可达（建议独立变更）。
- 产品语义差异：ActionResult::error 以 HTTP 200 返回是 o2server 兼容约定，前端必须按 body.type 而非 HTTP 状态判断写结果。

## 七、证据与取证限制

- 证据目录（gitignored）：test-results/r430-pc-querydesigner-silent-write/，含 evidence.json、probe.json、诊断脚本与截图；真域名仅留该目录占位为 <HOST>，不入库。
- 截图 sha256（命令行 sha256sum 原始值，1440x900 headless 非物理真机）：
  - r430-decisive-save.png = 869e1f6b926ae4314874b879a36d8a8f0d4164a95aee6c850c3cda18c98be1bb（modal 关闭+列表行仍在+真后端 200 error，已目视）
  - r430-500.png = fba8304d1f9515e6557442d4565129ffaa71c60dd046948049c532b255981530（红 toast+modal 保留，已目视）
  - r430-403.png = 024eaa7244b9d5b455ef830b94bfffc559bc8b8b1fa3d68aa4eb2a717cc6a62c
  - r430-error-state.png = f5ae23f38f0b06a33ad3565f3be0685539d75db62bc0c10e94b43c23bcda67b7
  - r430-empty-state.png = b112c326264a5612a5d72b3f7c84c83338544a67c47c805e7162ef5275e85b6f
  - r430-dom-audit.png = 6b7b7327c8ee91a7c89b64bc64d6f75bc82f06adbea14bec617199b69a21109b
  - r430-delete-unreachable.png = c07ef25d72a721216cdd99ff8325e193ccccac804e4ab49682b226c3bddbee37
- 取证限制：破坏性真写未执行，用假 ID 触发 0 行 UPDATE 替代；失败态注入未生效据实记录，P1 判定不依赖；deleteQuery 确认框不可达无法测取消零写；全量写按钮按代表性覆盖。
- 提交：仅本文一个文档精确 git add，不 -A/.；push 单次即止（系统级 TLS 故障按队长口径不重试），本地 commit 落地回报 SHA 与 AHEAD。
