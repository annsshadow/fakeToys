# R384 PC RecycleApp.vue 写链误报成功修复

- 轮次：R384（任务 t207）；对象：PC 回收站 `frontend/apps/desktop/src/views/RecycleApp.vue`。本轮只做前端写响应判定修复，不改后端、SDK、其它页面或 migrations。
- 根因：SDK 对 HTTP 200 + `type:error` 信封原样 resolve；`resume`、`permanentDelete`、`emptyRecycle` await 后不读响应即删行/清空；`batchResume`、`batchDelete` 用 `Promise.allSettled` 按 fulfilled 计 ok，失败信封被误计为成功。
- 修复：新增 `recycleWriteSucceeded(resp: any, flagKey?: string): boolean`：非对象、`type:error`、`success:false` 或 type 非 success/缺省均失败；传 `flagKey` 时要求 `resp.data?.[flagKey] ?? resp[flagKey]` 严格 `=== true`。
- 五写链接守卫：resume 传 `resumed`，permanentDelete 传 `deleted`，emptyRecycle 不传 flagKey；失败时透传后端 message 并保留行/不误报，成功路径保留原交互。批量写仅对 helper 判真的项计 ok、删行，失败项保留并汇总错误提示。
- 契约：新增 `frontend/apps/desktop/src/views/RecycleApp.writeEnvelope.contract.test.ts`，源码函数体真执行（注入 api/toast/confirmMsg/ref 桩）。真父版本 12 failed / 10 passed（RED），修复后 22/22 GREEN。
- 验证：根 vitest 全量 286 files / 2544 tests 全通过；desktop `pnpm exec tsc --noEmit` exit 0；PC 构建入口 `index-CvNBczcC.js`，整目录原子部署到测试服 `<YOUR_TEST_HOST>`（域名占位 `<YOUR_TEST_DOMAIN>`）。
- 真浏览器：Playwright Chromium headless 1440x900 对部署测试服做注入对照，**非物理真机**；覆盖单条恢复/永久删除、批量恢复/删除、清空，注入 type:error、权威 bool false、500/403、confirm 取消与真成功路径；0 pageerror。截图与 evidence 落 gitignored 目录 `test-results/r384-pc-recycle-write-fix/`，来源标注在 `evidence.json`。
- 脱敏：全文只用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位与相对路径，不写真实域名、IP、主机、凭据或个人环境信息。
