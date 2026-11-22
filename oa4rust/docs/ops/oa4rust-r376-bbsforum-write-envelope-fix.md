# R376 PC BBSForum 写响应误报成功修复

> 实施人：page-core ｜ 轮次 R376 ｜ 任务 t195 ｜ 类型：P1 真缺陷修复
> 验证环境：Playwright Chromium headless 1440x900，**非物理真机**；测试服域名以 <YOUR_TEST_DOMAIN> 占位，测试主机以 <YOUR_TEST_HOST> 占位。所有路径均为相对仓库根目录。

## 1. 缺陷与修复范围

R375 审计确认 PC 论坛页 `saveSection / deleteSection / createTopic / submitReply` 在后端 HTTP 200 + `type:error`，或 HTTP 200 + `type:success` 却带 `saved:false` / `deleted:false` 时，仍无条件 `toast.success`，误导用户认为操作已生效。

本轮仅改 `frontend/apps/desktop/src/views/BBSForum.vue` 的上述真 `@click` 写链判定；端点路径、payload、`confirmMsg` 取消守卫、非 2xx catch、列表错误态/空态、死代码群、后端、SDK 传输层、其它 views 与 migrations 零改。

## 2. 修复方案

1. 新增本地 helper `bbsWriteSucceeded(resp: any, flagKey?: string): boolean`：
   - `resp` 非对象、`type:error` 或 `success:false` → 失败；
   - 仅 `type:success`（或 type 缺省且 success 非 false）视为业务成功；
   - 提供 `flagKey` 时要求 `resp.data?.[flagKey] === true`（严格 true），未提供 flagKey 的端点（如 subject/reply create 无权威 bool）按 type 判定，不强求 bool。
2. `saveSection`：edit 分支 `section/save/{id}` 的响应按 `bbsWriteSucceeded(resp,'saved')` 守卫，create 分支 `section/create` 按 type 守卫；失败 `toast.error` 透传后端 message 并 return，不刷新不落态；成功保留原 `toast.success`。
3. `deleteSection`：`section/delete/{id}` 响应按 `bbsWriteSucceeded(resp,'deleted')` 守卫，失败不清空选中、不刷新列表，仅 `toast.error`。
4. `createTopic`：`createMutation` 默认 `onSuccess(data)` 与 mutate options `onSuccess(data)` 均先 `bbsWriteSucceeded(data)`，失败静默/错误 toast，不关闭弹窗不清空表单；`submitReply`：`replyMutation` 默认与 options onSuccess 同型守卫，失败不刷新回复/主题、不清空输入。

## 3. 验证

- 契约：`frontend/apps/desktop/src/views/BBSForum.writeEnvelope.contract.test.ts` 用源码级函数体提取 + `new Function`/`new AsyncFn` 真执行注入 api.post、toast、confirmMsg、qc 桩；父版 10 failed / 9 passed（helper 缺失与四写链无守卫），修复版 19/19 通过。
- 命令：`pnpm exec vitest run frontend/apps/desktop/src/views/BBSForum.writeEnvelope.contract.test.ts`、`pnpm exec vitest run`、`pnpm exec tsc --noEmit`（desktop）均 exit 0；根全量 282 files / 2461 tests 全通过。
- 真浏览器（headless 1440x900，非物理真机）：见 `test-results/r376-pc-bbsforum-write-fix/evidence.json`，注入对照 saved:false / deleted:false / type:error / 403 / 500 / confirm 取消 / trueSuccess，断言全通过、0 pageerror。

## 4. 证据与脱敏

证据目录 `test-results/r376-pc-bbsforum-write-fix/` 已被 gitignore；真凭据仅从部署环境读取，不入库。文档中的测试域名、测试主机均为占位符，未写入任何真值。
