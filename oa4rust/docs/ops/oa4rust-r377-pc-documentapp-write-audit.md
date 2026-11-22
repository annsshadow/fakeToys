# R377 PC 文档管理页写操作链失败信封 UI 反馈甄别

- 轮次：R377；任务来源：队长 m08197 / m08200 派发，attempt 2679b2ef 后续独立轮次。
- 角色：qa-e2e（只读甄别，不改源码/契约/后端/SDK）。
- 环境：部署测试服 <YOUR_TEST_DOMAIN>，Playwright Chromium headless 1440x900（非物理真机）；凭据经 SSH 运行期读取，不入库、不回显。
- 被测页面：PC 文档管理（路由 /app/admin/document，源码 frontend/apps/desktop/src/views/DocumentApp.vue）。

## 一、甄别范围与源码定位

模板真实 @click 写按钮：推荐（onCommend :39）、取消推荐（:40）、置顶（:41）、取消置顶（:42）、发布（:43）、撤发（:44）、密文发布（:45）、通知（:48）、删除（:53）。写函数群均为 await 后端响应后无条件 toast.success，未读取响应 type/bool：

- onCommend :275-276（GET document/{id}/commend → 已推荐）
- onTop :284-285（GET document/{id}/top → 已置顶）
- onPublish :293-294（PUT document/publish/{id} → 已发布）
- onUncommend :303-304（GET document/{id}/uncommend → 已取消推荐）
- onUnTop :312-313（GET document/{id}/unTop → 已取消置顶）
- onPublishCancel :321-322（PUT document/publish/{id}/cancel → 已撤销发布）
- onNotify :350-351（POST document/{id}/notify → 已发送通知）
- onCipherPublish :170-171、onRecordView :359-360（同为裸 toast.success）

后端 cms_assemble_control/src/lib.rs：document_u2_commend :8961-8976、document_u2_top :9021-9056、document_u2_publish :8911-8930、document_u2_publish_cancel :8936-8955、document_notify_u3 :11664-11690 在文档不存在时统一返回 ActionResult::error（HTTP 200 + type:error + message:document not found + size:-1 + prompt:ExceptionEntityNotExist，shared/response.rs :188-200）。SDK frontend/packages/sdk/src/api.ts :82-88 的 parseJsonEnvelope 仅 JSON.parse 不检查 type，:275-296 仅非 2xx 抛错，2xx type:error 被原样 resolve。

## 二、真浏览器实测（决定性证据）

1. 假 ID 探针（R377-NOSUCH-*，fetch 直打真后端）：commend/top/unTop/publish/publishCancel/notify 六个端点全部 status 200、type:error、message:document not found；uncommend 是幂等 DELETE，假 ID 返回 200 success removed:0（无成功 toast 误报语义，见通过项）。
2. 决定性对照（排除人造注入伪造）：仅注入文档列表 1 行假 ID 文档（R377-INJECT-*），写请求完全走真后端、不拦截写路由。真实 UI 点击后：
   - 推荐 → 真 GET document/{id}/commend 200 type:error → toast「✓已推荐」；
   - 置顶 → 真 GET document/{id}/top 200 type:error → toast「✓已置顶」；
   - 发布 → 真 PUT document/publish/{id} 200 type:error → toast「✓已发布」；
   - 通知 → 真 POST document/{id}/notify 200 type:error → toast「✓已发送通知」。
   截图 doc-decisive-toasts.png 肉眼可见 R377 注入行与上述误报 toast。三全满足：真 UI 可复现 + 真请求响应（200+type:error）+ 源码定位（:275-351）。
3. 非 2xx 对照：注入 500 → toast「✗操作失败: HTTP 500: Internal Server Error」；注入 403 → toast「✗操作失败: 没有权限执行该操作」，均正确错误提示。
4. confirm 取消守卫：点击删除弹出自定义 DOM 弹窗「确定删除文档 R377注入文档？」，点取消后 DELETE 增量 0，零写。
5. 列表失败/空态可区分：列表 500 →「文档列表加载失败：HTTP 500: Internal Server Error」+「重试」按钮（retryButtons=1）；空注入 → 仅表头无错误提示，两者可区分。
6. 全过程 0 pageerror。

## 三、三分判定

【真缺陷 1 个 P1】DocumentApp.vue 有真实 @click 入口的写函数（onCommend/onTop/onPublish/onNotify 等）在后端 HTTP 200 + type:error（假 ID 真后端可复现）下无条件 toast.success 误报成功；用户以为操作成功，实际后端未生效。最小修复方向对齐 R363 方案 B：新增写响应成功判定（type 为 success 且权威 bool 严格 true，否则 toast.error 透传 r.message），各写函数 await 后先判再提示。

【取证限制】破坏性真状态翻转/真删未授权未执行，故用假 ID + 仅注入列表行走真后端写响应的对照法；密文发布/记录浏览等其余写函数按代表性覆盖与源码归类，未逐一 UI 触发；撤发按钮文案为「撤发」（非脚本尝试的「撤销发布」），该单按钮未 UI 触发，源码 :321-322 同型已归类。

【产品语义差异】ActionResult::error 采用 HTTP 200 + type:error 是 o2server 兼容既有设计（非本轮改）；uncommend 假 ID 返回 success removed:0 是后端幂等删除既有口径，前端「已取消推荐」提示在该口径下语义可接受，不单列缺陷。

通过项：非 2xx（403/500）正确 toast.error；confirm 取消零写；列表失败态与空态可区分；0 pageerror。

## 四、覆盖甄别与证据

docs/ops 既往命中均为文档 CRUD/视图/列表解包相关，未专项甄别写操作链失败信封 UI 反馈，本轮首次专测有效增量。证据落 test-results/r377-pc-documentapp-write/（gitignored）：evidence.json、evidence-v2.json、截图 doc-dom-audit.png、doc-probe-done.png、doc-injected.png、doc-decisive-toasts.png、doc-v2-fail-state.png、doc-v2-empty-state.png；脚本 r377-pc-documentapp-write.cjs、-v2.cjs 及 run.sh。全部 1440x900 headless 非物理真机。
