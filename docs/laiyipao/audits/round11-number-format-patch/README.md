# R11 补丁 · 推荐证据长浮点格式化 H5 界面审计

## 采集口径

- **非物理真机**：桌面 Chrome（agent-browser）模拟移动视口 **390×844** 与桌面视口 **1280×800**，均非物理手机。
- 本地回环 `http://127.0.0.1:5174`（H5 dev server）+ `http://127.0.0.1:8080`（API）。
- 本机无 PostgreSQL，API 无法启动；为避免"空态假阳性"，用 agent-browser `network route` 注入接口 mock（schema_version=rec-v1，context 按 skills/equipment/reactions 分别匹配），让三页真实渲染推荐证据区。
- mock 证据值：长浮点 `79.62962962962963`（%）、`0.30000000000000004`、字符串型配置版本 `cfg-sha-abc123`。

## 背景与修复

t105 修复了 5 处裸插值长浮点，但遗漏了 evidence 区与 r7BasisText 依据区 3 页共 5 处。本轮：

- skills.vue / equipment.vue：`r6-ev-val`、`r7BasisText` 走 `fmtEv()`（数字→`fmt()` 一位小数，字符串原样，`null→—`）。
- reactions.vue：新增同语义 `fmtEv()`，`r6-ev-val` 走格式化。
- 契约测试 RED（3 failed）→ GREEN（12 passed）：长浮点只显示一位小数，字符串型证据（配置版本 sha、分数形字符串）原样保留。

## 界面实测结果

三页 × 两视口，`LONGTAIL=false`（页面正文不含任何长浮点字面）；`r6-ev-val` 实测渲染为 `79.6% | 0.3 | cfg-sha-abc123`。
桌面视口 r7BasisText 实测渲染为「每波 +4.3 (+79.6%) · 战力 +0.3 · 评分 +4.0」，历史三星率 `33.3%`（stars=3 场数占比，不是胜率）。
字符串型配置版本 `cfg-sha-abc123` 原样保留，未被误格式化。

## 截图（非物理真机，桌面浏览器模拟移动/桌面视口，本地回环）

| 文件 | 视口 | sha256 |
| --- | --- | --- |
| after-skills-mobile.png | 390×844 | 02ef6db87d142bd4fe096ced01c6eb7d8c23c07f311bb3d94722db1eff3adec7 |
| after-skills-desktop.png | 1280×800 | 160b1e4d65a422c9fe7d041b444aaa60a2e6126273d1b059d340ee3610d3c109 |
| after-equipment-mobile.png | 390×844 | 554eec535d8d67d1126ac5bc06277870c2a6b198ac73fab56c3cf1cd4fdd8167 |
| after-equipment-desktop.png | 1280×800 | af7c8a260a6981948b8bc79229ee6bb0b9e236cf0f7ca61fafd38b2b38b9ddc0 |
| after-reactions-mobile.png | 390×844 | fd236348d1660a4f2b78a0e82eba67c62a195daf3c9d04afce7ecbf98569de7d |
| after-reactions-desktop.png | 1280×800 | 54b836564b9242ab6a7ec7a1dc914d4202c85126ef26361a178d8e2e1a63e2d2 |

## 验证

- 契约测试：RED 3 failed → GREEN 3 files / 12 passed。
- 全量 `pnpm test`、`pnpm type-check`、`pnpm type-check:test`、`pnpm build:h5`：见任务上报记录。
- 采集脚本：capture3.sh（context 分流 network route 注入）。

全程未执行任何 git 操作，未写入任何真值（域名/IP/口令/密钥）。
