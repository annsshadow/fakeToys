# R382 PC 热帖管理写链误报成功修复

## 问题
`HotpicApp.vue` 的 `hotpicWrite`、`hotpicMore` 与 `onDelete` 在后端返回 HTTP 200 + `type:error`（或权威 `deleted:false`）时仍提示成功，删除路径还会就地移除列表行，造成误报成功与错误落态。

## 修复
- 新增本地 helper `hotpicWriteSucceeded(resp, flagKey?)`：以 `type` 判定为主，`type:error`/`success:false` 为失败；提供权威 bool 字段时要求严格 `true`。
- `hotpicWrite` 的 `userDelete` 使用后端 `deleted` 权威字段；`hotpicMore` 的 `userDelete2` 使用 `deleted` 权威字段；其他分支按 type 判定。
- 写链失败时 `toast.error` 透传后端 message，直接返回：不弹成功、不刷新、不删行；成功保留原 toast 与原删行行为。
- 端点、payload、method、confirm 取消守卫、非 2xx catch 与其他端点逻辑均未改动。

## 验证
- 契约：`HotpicApp.writeEnvelope.contract.test.ts`，真父版本 9 failed/9 passed（RED），修复版 18/18 passed（GREEN）。
- 根 `vitest run`：285 files / 2522 tests 全绿；desktop `tsc --noEmit`：exit 0。
- 真浏览器：`test-results/r382-pc-hotpic-write-fix/`，Playwright Chromium headless 1440x900 访问 `<YOUR_TEST_DOMAIN>` 测试服，12 条断言全 true，0 pageerror；截图为真浏览器 headless 结果，非物理真机。

## 访问方式
测试入口与凭据从部署环境注入的 `<YOUR_TEST_HOST>` / 本地密钥读取，仓库内不保存真值。
