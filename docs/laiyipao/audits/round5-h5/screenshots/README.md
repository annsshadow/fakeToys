# Round5 H5 五页视觉改造截图说明

- 采集工具：agent-browser 0.41.1（桌面 Chromium 模拟移动视口），视口 390 × 844。
- 设备性质：**非物理真机**；所有页面经本地回环 `http://127.0.0.1` 访问，未使用任何外网真值。
- 页面路由：`announcements` / `login` / `signin` / `stage` / `verify`。

## after（当前工作树，5174）

`VITE_API_BASE=http://127.0.0.1:8080/api/v1`，游客态有效。已逐张读图确认：公告、签到、关卡、验真四页均非登录页，并可见本轮新增的眉题、KPI 带与 AI 线索卡；login 路由本身展示登录表单及本机缓存提示。

| 文件 | 页面 | sha256 |
| --- | --- | --- |
| after-announcements.png | 公告 | b7d5f337e34f7f62707835293099d3513bfa077c5dff1f21163b0f7adba90eb6 |
| after-login.png | 登录 | edcce5588a7fc05817ca626594a9ffca2fe92f0ab7dbb7aaf87c8a0a2212ebfc |
| after-signin.png | 签到 | f1e456160f05494002920acb3d17b93b28cb84d4cdfdc03792c715e386dab86b |
| after-stage.png | 关卡 | 6bb24737f2617165bfaa2c9d00d1ece600a0cc451cbeaa7f0c45111c93182f87 |
| after-verify.png | 验真 | db8e5ee6bd29f2ceb11447379fd4e119030e583d787401d85c47e147aa8941b0 |

## before（基线工作树，5175）——已知采集遗留

基线服务来自另一工作树，其认证/API 状态与本轮工作树不同：五张 `before-*.png` 实际落在登录页或「公告加载失败」空态（五张登录态截图 sha256 均为 `9ef68430...`），**不能作为有效的改造前视觉对照**。该限制已如实上报队长；不把无效基线包装成有效证据。

| 文件 | 实测内容 | sha256 |
| --- | --- | --- |
| before-announcements.png | 登录页 | 9ef68430b126f7bd913379d7a1d696ff9105066b115d99369446decd5a527317 |
| before-login.png | 登录页 | 同上 |
| before-signin.png | 登录页 | 同上 |
| before-stage.png | 登录页 | 同上 |
| before-verify.png | 登录页 | 同上 |
| probe-before.png | 公告加载失败空态 | 83d11af243cb3f16ef0efc7f2fc248b4b16ee511ed8bffed04f21ac53626b6b6 |

## 验证

- `pnpm exec vitest run --config vitest.config.ts src/styles/r5H5Pages.contract.test.ts`：11/11 passed（含 index 反应伤害文案 RED→GREEN 契约）。
- `pnpm test`：171 files passed | 1 skipped；1619 tests passed | 2 skipped。
- `pnpm type-check`：exit 0。
- `pnpm build:h5`：DONE Build complete。
