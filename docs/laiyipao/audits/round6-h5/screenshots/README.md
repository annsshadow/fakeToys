# R6 H5 数据驱动推荐卡 · 真浏览器截图记录

> ⚠️ **非物理真机**：以下截图由桌面 agent-browser（Chromium）以 **390×844 移动视口**访问本地回环 `http://127.0.0.1:5174/#/...` 采集，不是实体手机拍摄。

## 采集口径

- 浏览器：agent-browser 0.41.x，会话 `lyp-r6*`，视口 390×844。
- 前端：`pnpm dev:h5 --host 127.0.0.1 --port 5174`，`VITE_API_BASE=http://127.0.0.1:8080/api/v1`（本地回环，无真实域名/IP）。
- 后端：本机开发配置 API `:8080`（公开 dev 默认值，未使用 `.local.keys` 真值）。
- before 基线来源：`docs/laiyipao/audits/round3-h5/screenshots/after-*.png`，即 R3 完成、R6 开始前的真实状态；原样复制并保留 sha256，未做像素修饰。
- after：R6 实现后在同一视口、同一本地回环重新打开页面采集；已逐张目视读图。

## 文件与 sha256

| 页面 | before | sha256 | after | sha256 |
|---|---|---|---|---|
| equipment | before-equipment.png | 521f92bd431b2db18c45256ec924e212df552b2dca5cf89e8184105645efbb33 | after-equipment.png | 07ef05e871fc77242f3a9eee9eb5221c2f2217cdbdc9bf5b03cb01df03e93648 |
| reactions | before-reactions.png | 8dbd076c8c843055df3c9125e3461aafef4d99edcad66c6889c22a68c5a90b92 | after-reactions.png | 260ee056ed6f1ed1d9bf1b6dff63af9da31f545e2101776aa56472d4fa2b54f2 |
| skills | before-skills.png | 2cb5b3f73dbc85cd3e8657637a042049ba5e2f57be4b80b99b97e4af23131638 | after-skills.png | 11dc310383d8bdbba4d8b8e55e40a19105686f8fb2c2e3bb976d15419829d837 |

## 目视结论（非物理真机）

- 三页 before 均只有「待配置 / 本地规则」提示或裸进度，缺少接口驱动的结论、证据与 CTA。
- 三页 after 在 KPI 带下方新增独立 r6 推荐卡：可见 AI 徽章 + 真实结论（equipment「母巢碎片…优先刷宫」/ reactions「还差 4 条反应可达…」/ skills「重炮弹每热量 6 伤害…优先填空」）+ 依据数据 + ≥88rpx CTA，并明确标注「本地规则引擎」，未伪装成 GPT/LLM。
- 卡片使用 `--panel-2`、`--ai-accent`、`--ai-halo`、`--ai-glow`，与 R1–R5 视觉体系一致；没有用 .022/.06 低透明度纹理充数。
- 数据差异说明：before 截图采集时配置未下发；after 采集时本机 API 正常返回 `/config` 数据，因此 KPI 数值从 0 变为真实值，不是视觉包装造成的假数据。
