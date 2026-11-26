# t33 审计：根治 admin 顶栏 API 地址泄露

## 环境与口径

- 环境：**非物理真机**（Windows Chrome headless via agent-browser 0.41.1），本地回环 127.0.0.1。
- 实际测试域名/口令真值一律从 laiyipao/deploy/.local.keys 读取，不入库/日志/截图/对话；本文档用 <YOUR_TEST_DOMAIN> 占位，真值文件仅用于截图二进制扫描的模式源。
- 场景一（回环豁免）：dev server :5175，未配置 VITE_API_BASE，默认回环值。
- 场景二（非回环脱敏）：dev server :5177，注入 VITE_API_BASE 为 RFC 保留的非真域名 example.invalid（仅用于验证渲染，非真实地址），用 fetch patch 让守卫通过以渲染后台壳。

## 缺陷与修复

- 缺陷：AdminLayout.vue 顶栏 .api-tag 直接渲染 VITE_API_BASE 的完整值，测试环境截图会 capture 到真实服务器地址，违反 AGENTS.md 合规红线。
- 修复：新增 src/utils/apiLabel.ts 纯函数集——isLoopbackApiBase 判定本地回环（127./localhost/::1/0.0.0.0），maskApiBase 对非回环地址只返回「API」占位，仅回环或显式调试开关 VITE_SHOW_API_HOST=1 才显示完整值。AdminLayout.vue 顶栏改消费 maskApiBase 结果。

## test-first RED → GREEN

- 改写 tests/AdminLayout.test.ts 中钉住泄露行为的旧用例为两条新契约：非回环地址不出现在 DOM、显式调试开关才显示完整值。
- 新建 tests/apiLabel.test.ts 七例纯函数契约：非回环域名/IP 脱敏、回环/localhost/IPv6 可见、未配置回退、调试开关。
- RED：2 files failed（模块不存在 + 旧用例断言真域名出现在 DOM）。GREEN：16 passed。

## 真浏览器实测（非物理真机）

- 回环场景：.api-tag 文本 = API 127.0.0.1:8080（豁免，便于本地调试）。
- 非回环场景：.api-tag 文本 = API（纯占位），body.innerText 不含 example.invalid / https://，标题 = 数据看板 · 来一炮 · 运营后台（确认进入后台壳而非登录页假阳性）。
- 二进制真值扫描：从 .local.keys 提取 5 条模式，对两张 PNG 做 grep -a -F 扫描，命中均为 0。

## 截图与 hash

| 文件 | sha256 | 说明 |
| --- | --- | --- |
| screenshots/loopback-visible.png | 0302324777e099975dc715be937236e74aef9415cacb95487f46246def8fe927 | 回环豁免：顶栏显示完整回环地址 |
| screenshots/nonloopback-masked.png | ab01f714e0ca19077e2c94a1b79c848288c2945c6880f6d25f85d2c71957a8fc | 非回环脱敏：顶栏仅显示 API |

## 门禁

- npm test / npx vue-tsc --noEmit / npm run build：见任务 update_task 的 commandsRun。