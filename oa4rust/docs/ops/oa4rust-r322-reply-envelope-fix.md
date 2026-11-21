# OA4Rust R322 D1 论坛详情回复双层信封解包修复

- 轮次：R322（PC 端论坛回复闭环补测发现并修复）
- 分支：`opt1000/oa4rust-pc-ai-refactor`（非 main）
- 相关任务：t108 审计（发现）→ t110 修复（本文）
- 变更性质：真 bug，test-first RED→GREEN

## 1. 问题与根因

t108 真浏览器审计坐实：论坛帖子详情里**回复已成功落库**（POST `reply/create` 返回 200，GET `reply/list/sub/{id}` 与 PUT `reply/filter/list/page/1/count/50` 均能读到 3 条回复），但页面显示「回复 ()」「暂无回复」及 `bbs-reply-undefined` 空卡。

根因（前端渲染层，两层信封只解一层）：

- `BBSForum.vue` 回复列表 `queryFn` 旧实现 `return ((resp as any)?.data ?? []) as Reply[]`。
- 真响应形态：`{ data: { data: [reply...], total: 3 }, type: "success" }`（双层信封）。
- 旧实现只解一层，`replies.value` 被赋为内层对象 `{ data: [...], total: 3 }`；
  模板 `replies.length` 为 `undefined`（显示「回复 ()」），`v-for` 遍历对象键得到 `data`/`total`，
  渲染出 `bbs-reply-undefined` 且无正文。

次生缺陷（同一可见性问题链上）：

- `enabled: computed(() => !!viewingTopic.value).value as any` 在 `setup` 时取静态快照 `false`，
  导致详情初次打开与刷新都不触发查询，只有提交回复后 `refetchReplies()` 手动触发才渲染。
- 后端 `PUT reply/filter/list` 只返回 `id/topicId/content/creator`，无 `author/createTime`；
  模板读 `reply.author`、`fmtTime(reply.createTime)`，作者/时间显示为空。

## 2. 修复（最小改动，仅 BBSForum.vue）

1. 双层信封归一：`return (((resp as any)?.data?.data ?? (resp as any)?.data ?? []) as Reply[])`
   —— 兼容双层/单层/空信封，统一归一为 `Reply[]`。
2. `enabled` 改响应式 getter：`enabled: () => !!viewingTopic.value`
   —— 详情打开/关闭/切换主题时重新判定，初次打开与刷新均能触发查询。
3. 写读字段兼容映射（watch `repliesData`）：`author = r.author ?? r.creator ?? ''`、
   `createTime = r.createTime ?? r.create_time ?? ''`。仅做兼容映射，缺字段不臆造。

未改：后端任何源码、`GET reply/list/sub/{id}`、`POST reply/create` 写链、R318/R320 既有修复、
`IMChat.vue`、移动端。

## 3. RED→GREEN 证据

新增契约 `frontend/apps/desktop/src/views/BBSForum.replyEnvelope.contract.test.ts`（6 it）：
从 `BBSForum.vue` 源码提取解包 `return` 表达式、剥离 TS 断言后用 `new Function` 真执行，
断言双层/单层/空信封归一为数组、字段正确、写读链与模板未回退、`enabled` 为响应式 getter、
作者兼容 `creator`。

- RED（修复前）：`pnpm exec vitest run frontend/apps/desktop/src/views/BBSForum.replyEnvelope.contract.test.ts`
  → 2 failed / 2 passed，报「replies 必须是数组，不能是内层信封对象」「expected { data: [], total: 0 } to deeply equal []」。
- GREEN（修复后）：同命令 6/6 passed。
- 全量回归：`pnpm exec vitest run --reporter=dot` → 265 files / 2139 tests，0 failed。
- 类型检查：`pnpm run typecheck` → exit 0（含 desktop `tsc --noEmit`）。

## 4. 真浏览器复验（部署测试服，Playwright Chromium headless 1440×900）

- 部署：前端 `pnpm --filter @oa4rust/desktop build`（入口 `index-rf91JkBU.js`），
  经原子替换脚本部署到 `<YOUR_TEST_DOMAIN>` 测试服，远端 grep 入口哈希校验通过。
- 流程：以真实账号登录 → `/app/bbs` →「我的」tab → 打开主题「R322闭环_176861」
  → 读取回复卡 → 真 UI 输入并提交新回复 → reload → 再次打开详情。
- 结果（evidence `test-results/r322-reply-envelope-fix/evidence.json`）：
  - 初次打开 5 条真实卡片，`data-testid=bbs-reply-{真实 uuid}`，无 `bbs-reply-undefined`；
  - 作者 `admin`、正文与后端落库一致，回复数「回复 (5)」与卡片数一致；
  - 新增回复后立即出现（6 条），刷新后仍为 6 条且含新正文；
  - `pageErrors=[]`、`apiFailures=[]`（无 ≥400 的 `/oa4rust-api/api/` 请求）。
- 截图：`01-detail-fixed.png`、`02-after-reply.png`、`03-persisted.png`（均 1440×900，
  位于 gitignored `test-results/r322-reply-envelope-fix/`）。
- 证据来源标注：**Playwright Chromium headless against deployed test server; not a physical device**
  （非物理真机；未冒充真机）。

## 5. 取证限制与遗留项

- **回复时间不显示**：后端 `reply/filter/list` 的 SQL 未 SELECT `create_time`、映射也无该字段，
  `GET reply/{id}` 详情同样无时间；前端在本任务 inScope 内不臆造时间，
  以 `createTime=''` 降级显示为空。属后端能力缺失，需后续版本在后端补齐字段（本任务 outOfScope）。
- 通知未读已读链路：t89/t108 已证实后端会话列表接口不返回 `unread` 字段，属既有取证限制，
  不重复计入本轮缺陷。

## 6. 提交范围

精确三文件（禁 `-A`/`.`）：

- `frontend/apps/desktop/src/views/BBSForum.vue`
- `frontend/apps/desktop/src/views/BBSForum.replyEnvelope.contract.test.ts`
- `docs/ops/oa4rust-r322-reply-envelope-fix.md`（本文）

本文所有服务器地址/凭据均为占位 `<YOUR_TEST_DOMAIN>`，无真实域名/IP/口令入库。
