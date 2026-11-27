# R102-B 首页五处 CTA 键盘可达性部署后真浏览器复查报告

- 轮次：R102-B（t274 B5 部署后复查）
- 前置：t269 侦察 → t270 规范 RED（5 fail / 4 pass）→ t271 实施 GREEN（9/9）→ t272 QA PASS → t273 终审 verdict=pass 六维全绿 8 条 low
- 基线 HEAD：`5ec099e5508b853a4cc9b8936d09d76ec1020589`
- 视角：生产构建 H5 首页五处 CTA 键盘可达性（真实静态服务，非 dev server）
- 报告：`docs/laiyipao/audits/round102-b-index-cta-postdeploy/REPORT.md`

## §1 部署与方法

- 静态服务：`python -m http.server 5201 --bind 127.0.0.1` 宿主 `dist/build/h5`（HTTP=200），全程未起后端 8080。
- H5 产物重建：`pnpm build:h5` RC0；主 bundle `index-U0L6c5Yx.js`（`index.html` 引用 `assets/index-U0L6c5Yx.js`），sha256 `a776331eecb0ac5f5519c6e5ea296df6bbbe93f31521ddb803d0c04b0cd7dd26`。后端/admin 无变化沿用不回退。
- 浏览器：真 Chromium（agent-browser 0.41.1），三视口 414×896 / 820×1180 / 1440×900，正常态 / 满值态 / 错误态三态。
- 注入：`r102-init.js`（正常态）、`r102-init-full.js`（满值态）、`r102-init-err.js`（错误态，令牌过期至 2000 年 + 首次 guest 500），占位令牌 `r102-qa-placeholder-token`（仅内存注入、零真值），XHR + fetch 双 patch 合成 config/公告/战报。QA 的 dev-server init-script 在生产静态构建同样生效。
- 探针：DOM role/tabIndex/cursor/vis/hOverflow；AX snapshot 读 button；document capture 监听 keydown 记录 Enter 送达；stub `uni.navigateTo`/`redirectTo` 记录 URL；截图 sha256sum 逐张核对并亲读 3 张。
- 防干扰：Tab/连续 Enter 用单 session 独占 driver；干净 Enter 改为每处 CTA 独立 session（每处重新 `open --init-script`），避免前一处 Enter 导航/失焦污染后续探针。

## §2 三视口 DOM/AX/Tab 证据

正常态三视口 bodyLen=824、insight=补强、isError=false、iw=cw=sw=视口宽、零横溢；错误态三视口 bodyLen=82、isError=true、retry 可达、零横溢。五处 CTA（`:40` 重试 / `:53` first-run-guide / `:78` 去补强 / `:92` 去战斗 / `:109` ann-all）及两处不变量（dim-hint / ann-item）均 `role=button`、`tabIndex=0`、`cursor=pointer`、`vis=true`。

- 真 Tab 序列：每视口 total=18、ctaHits=10，依次命中 first-run-guide → ai-cta → ann-all → ann-item×2 → dim-hint×5；另有 action-card / grid-item×15 在序列中。
- AX：正常态 snapshot buttons=34，含 `button "新玩家先打第1关…"`；错误态 snapshot 含 `- button "重试" [ref=e1]`。
- 五处 CTA 改前基线 role=null/tabIndex=null/cursor=auto、不在键盘序列 —— 本轮全部修复，与 QA 判定一致。

## §3 Enter 送达（干净独立会话 18/18）

每处 CTA 独立会话：重新注入 init-script → 装载 observer-stub → JS focus 该 CTA → 真 press Enter → 读 keydown 记录。三视口 × 六处（first / ai / annall / dimhint / annitem / retry）共 18 组，`focus=FOCUS:true|role=button|tab=0`、`lastKey=Enter`、`isCta=true` 全绿；retry 组 `guestCount` 1→2，坐实重发 `POST /auth/guest`。

- 导航类（first/ai/annall）Enter 送达 CTA，目标 URL 由 QA 阶段 stub `uni.navigateTo` 坐实（stage?level=1 / equipment / announcements），本轮聚焦送达不重复跳转断言。
- 不变量（dim-hint / annitem）Enter 送达 CTA，守 R99/R101 不回退；annitem 展开态由 R101 静态契约保证。
- 首轮连续会话 driver 中，first 的 Enter 触发导航后同页后续 CTA 失焦（`focus=NO`，`isCta=false`），属同会话连续 Enter 干扰；已由每处独立会话补跑消除，不作为能力缺陷。

## §4 部署后截图（sha256sum 实物，UNIQUE 6/6 且全 ≠ QA 6 图）

| 文件 | sha256 |
|---|---|
| pd102-index-414.png | e26c3335a3f26a87584881a703d48a800ebadd7a0fc4eecf3f69b3ede51312be |
| pd102-index-820.png | 26c939273402fc36debfeb0bc26716c5e02f3396a30f44d9a323fff314099ff7 |
| pd102-index-1440.png | 5130397796a1c5ee216e85f62fb75e0de991ac05d379e39c67647c19f6378278 |
| pd102-err-414.png | ce5f403e2dddc4e9c6edf5882be374d6d53b92d4ab5aa1707fb3fab0abd2e32e |
| pd102-err-820.png | f215e3c96b668d26ee1cf64d2018931368fc4d219eefb3a184f091fda1582c39 |
| pd102-err-1440.png | 7ba4c60a977fdf0dd4ce079fe3080fa57deb9e2ccadcf37bc14df7015af656a5 |

亲读 3 张非登录页：`pd102-index-414`（首页 QA玩家/游客账号资源栏、关卡卡片、九宫格入口）、`pd102-err-414`（无法连接服务器 / 请求失败 HTTP 500 + 橙红重试按钮）、`pd102-index-1440`（桌面首页关卡卡片 + 九宫格）。6 图 sha 独立重算 UNIQUE 6/6，且与 QA 6 图逐一比对全部不同（部署后独立截图，非复用 QA 图）。

## §5 门禁与产物

- QA 门禁 G1-G10 全 RC0：契约 9/9、r60 6/6、r99 24/24、r100 12/12、r101 9/9、index.test 27/27、全量 234 files passed(1 skip)/2150 tests passed(2 skip)、vue-tsc、build:h5、git diff --check。
- Go 守卫：`TestNoInfraIdentifiersInRepo` PASS 8.22s(cached)；`TestNoRealSecretsInRepo` **SKIP 非 PASS**（`secrets_leak_test.go:182` 读不到 `.local.keys` —— 真值扫描这条守卫**没有执行**，不是「通过了」）。
- H5 bundle sha 与本报告 §1 逐字一致；后端/admin 沿用不回退。

## §6 诚实边界

- 后端 8080 未起，UI 数据由 init-script 合成零真值；坐实的是客户端 DOM/AX/键盘因果链，非服务端真实结果。
- 错误态由令牌置 2000 年 + 首次 guest 500 合成，非真实服务故障。
- Enter 用 stub 拦截导航记录目标，未做跳转后页面 DOM 断言；retry 走 `await store.login()` 而非导航，属 handler 语义差异。
- 焦点环：JS `element.focus()` 不触发 `:focus-visible`，探针 `outlineStyle=none`；焦点环样式由 `common.css:914-919` 静态保证，非本轮新增。
- first 连续会话 Enter 导航导致后续 CTA 失焦的干扰已由独立会话补跑隔离，不作为能力缺陷。
- 全量 vitest 1 file / 2 tests skipped 为仓库既存，非本轮引入。

## §7 结论与 hygiene

判定 PASS。生产构建三视口坐实五处 CTA role=button/tabIndex=0/AX button/真 Tab 命中/Enter 送达（独立会话 18/18），错误态重试恢复路径可达，两处不变量未回退，零横溢；6 张部署后截图 UNIQUE 且全 ≠ QA 图，亲读 3 张非登录页。

hygiene：本轮 postdeploy 自身 tracked data=0（data/ 由 `.gitignore:653-656` 拦）；占位令牌仅内存注入零真值；auth 六路径（`.env.local` / `.env.local.keys` / `laiyipao/miniapp/.env.local` / `laiyipao/admin/.env.local.keys` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.local.keys`）全 ABSENT；静态服务与浏览器 job 收尾清理。
