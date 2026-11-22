# R374 PC IMChat sendFileMessage 写链失败信封误报成功 P1 修复

## 1. 问题与依据

前端即时通讯页 \`sendFileMessage\`（\`frontend/apps/desktop/src/views/IMChat.vue\`）在文件上传请求返回 HTTP 200 时，不检查响应信封 \`type\`，也不校验权威 \`fileUrl/url\`，即无条件 \`toast.success('文件消息已发送')\`。后端统一信封 \`error()\` 以 HTTP 200 + \`type:'error'\` 返回（兼容旧端），SDK 传输层只按 HTTP 状态码抛错、不检查 \`type\`，故 \`type:'error'\` 或成功但缺 \`fileUrl\` 时仍误报成功。

审计任务坐实：向真实上传端点发假 ID → HTTP 200 + \`type:'error' not a conversation member\`，UI 仍弹「✓文件消息已发送」；403 → 正确错误提示，500 → 正确错误提示。

## 2. 修复方案（type 判定为主，不强制 bool）

在 \`IMChat.vue\` 新增本地 helper：

\`\`\`ts
function imUploadSucceeded(resp: any): boolean {
  if (!resp || typeof resp !== 'object') return false
  if (resp.type === 'error' || resp.success === false) return false
  if (!(resp.type === 'success' || (resp.type === undefined && resp.success !== false))) return false
  const data = resp.data
  return !!(data && (data.fileUrl || data.url))
}
\`\`\`

- \`type:'error'\` 或 \`success:false\` 判失败。
- \`type:'success'\`（含 \`type\` 缺省且 \`success\` 非 false）为成功的必要条件，但还须 \`data.fileUrl/url\` 非空——上传确实拿到下载地址才算成功。
- 后端 \`im_msg_upload\` 成功信封带 \`fileUrl\`、无权威 bool，故不照搬严格 \`bool===true\`，避免假阴性。

\`sendFileMessage\` 中 \`await api.upload(...)\` 后先判 \`imUploadSucceeded(r)\`：失败则 \`toast.error(r?.message || '文件发送失败，请重试')\` 并 \`return\`（不刷新、不落态）；成功仍带文件名 \`toast.success\` 并 \`refreshFirstPage\`。

## 3. 硬边界

- 只改 \`sendFileMessage\` 的成功判定；非 2xx 的 \`catch\`（403/500）文案不变。
- 端点路径、payload、multipart 字段、模板唯一 \`@click\` 入口（发送文件按钮）不变。
- 无 UI 入口的死代码群、后端 crates、SDK \`api.ts\` 传输层、其它页面、migrations 零改。

## 4. 验证

- 契约 \`frontend/apps/desktop/src/views/IMChat.writeEnvelope.contract.test.ts\`：
  \`@vitest-environment node\`，\`readFileSync\` + 花括号配对提取函数体 + \`stripTs\` 剥 TS 标注 + \`new Function\` 真执行，注入 \`api.upload/toast/refresh/document/FormData\` 桩，非文本断言；12 it（helper 6 + 两态 6）。父版 RED 5 failed / 7 passed → 修复后 GREEN 12/12。
- 根全量 \`pnpm exec vitest run\`：281 files / 2442 tests，0 failed。
- \`pnpm exec tsc --noEmit\`（desktop）：exit 0。
- PC 构建 \`pnpm exec vite build\`：入口 \`index-DukolC6h.js\`，整目录原子替换部署到测试服 \`<YOUR_TEST_HOST>\` 的 web 目录（脚本见 \`test-results/deploy-pc-r374.sh\`，真值由部署环境注入，不入库）。
- 真浏览器对照：Playwright Chromium headless 1440x900，运行在部署后的测试服 \`<YOUR_TEST_DOMAIN>\`。注入一条会话并选中，拦截上传端点覆盖五种返回：\`type:error\`（透传后端 message、无误报、0 落态）、\`fileUrl\` 缺失（透传 message）、HTTP 500、HTTP 403（均正确错误提示）、\`type:success + fileUrl\`（仍成功）。8 条断言全部为 true，0 pageerror。
- 证据与截图落 \`test-results/r374-pc-imchat-write-fix/\`（已被 gitignore）：\`evidence.json\`、\`01-typeerror-fixed.png\`、\`02-missing-url-fixed.png\`、\`03-trueSuccess.png\`。截图来源为 headless 浏览器实测，**非物理真机**。

## 5. 范围

变更文件：
- \`frontend/apps/desktop/src/views/IMChat.vue\`
- \`frontend/apps/desktop/src/views/IMChat.writeEnvelope.contract.test.ts\`（新增）
- \`docs/ops/oa4rust-r374-imchat-write-envelope-fix.md\`（本文）
