# R102-B 首页五处 CTA 键盘可达性真浏览器 QA 报告

- 轮次：R102-B（t272 实施后独立 QA）
- 基线 HEAD：5ec099e5508b853a4cc9b8936d09d76ec1020589（全程未变）
- 视角：小程序 H5 首页（index.vue）五处 CTA 键盘可达性 + 错误态重试恢复路径
- 交付：docs/laiyipao/audits/round102-b-index-cta/REPORT.md
- 契约：laiyipao/miniapp/src/styles/r102IndexCtaKeyboard.contract.test.ts（9 it，t270 RED 5fail|4pass → t271 GREEN 9/9）

## §1 范围与方法

验证 t271 在 index.vue 五处用户可感知入口补 `role="button"` + `tabindex="0"` + `@keydown.enter` 同 handler：

| 行号 | 元素 | handler | Enter 预期结果 |
|---|---|---|---|
| :40 | view.btn.btn-primary 重试 | retry | 重发 guest 登录（错误态恢复） |
| :53 | view.card.first-run-guide 新手引导 | goStage | → /pages/stage/stage?level=1 |
| :78 | view.btn.ai-cta 去补强 | go(aiInsight.hint) | → /pages/equipment/equipment |
| :92 | view.btn.ai-cta 去战斗 | goStage | → /pages/stage/stage?level=1 |
| :109 | text.ann-all 全部 N 条 | goAnnouncements | → /pages/announcements/announcements |

方法：真 Chromium（agent-browser 0.41.1）三视口 390×844 / 768×1024 / 1280×800；init-script 注入合成后端响应（后端 8080 未起）；DOM 探针读 role/tabIndex/cursor/vis/hOverflow；AX snapshot 读 button 角色；真 press Tab 序列读焦点轨迹；document capture 监听 keydown 记录 Enter 送达，stub `uni.navigateTo`/`redirectTo` 记录目标 URL；截图 sha256sum 逐张核对并亲读 2 张。

## §2 三视口五处 CTA 证据（正常态 / 满值态）

正常态 mock（build_rating 五维全 0、有短板）bodyLen=824、insight=补强；满值态 mock（total=920 五维全满）bodyLen=746、insight=战斗。三视口均 iw=cw=sw=视口宽、hOverflow=false、isLogin=false。

| 视口 | CTA | role | tabIndex | cursor | vis | AX 角色 | Enter 送达(isCta) | 结果 |
|---|---|---|---|---|---|---|---|---|
| 390/768/1280 | 去第 1 关 first-run-guide | button | 0 | pointer | true | button | true | → stage?level=1 |
| 390/768/1280 | 去补强 ai-cta | button | 0 | pointer | true | button | true | → equipment |
| 390/768/1280 | 全部 2 条 ann-all | button | 0 | pointer | true | button | true | → announcements |
| 390/768/1280 | 去战斗 ai-cta（满值态） | button | 0 | pointer | true | button | true | → stage?level=1 |

真 Tab 序列（三视口一致）：按序命中 first-run-guide → ai-cta → ann-all → ann-item×2 → dim-hint×5 → mg-row×3 → action-card → grid-item。改前基线（ROUND102_B_INDEX_CTA_KEYBOARD_RECON.md）：三处 CTA role=null/tabIndex=null/cursor=auto、14 次真 Tab 全跳过、AX_TARGETS=0 —— 全部修复。

不变量未回退：:189-193 dim-hint link 仍 role=button/tabIndex=0/@keydown.enter=go(d.hint)，Enter → bag（三视口一致）；:114-120 ann-item 仍 role=button/tabIndex=0/@keydown.enter=toggleAnn(a.id)/:aria-expanded="String(expanded.has(a.id))"，AX 显示 `[expanded=false]`，Enter 送达 isCta=true。

## §3 错误态重试（合成 guest 500）

mock 将 `lyp_token_exp` 置为 2000-01-01（已过期）使 `hasUsableToken()` 返回 false → 走 guest 通道 → 首次 `POST /auth/guest` 返回 500 → `store/game.ts:384` 置 loginError → 错误卡片渲染。三视口均 bodyLen=82、isError=true、iw=cw=sw=视口宽、hOverflow=false。

- 重试按钮：role=button / tabIndex=0 / cursor=pointer / vis=true，AX `- button "重试" [ref=e1]`。
- 错误文案（AX + 截图）：「无法连接服务器」「请求失败 HTTP 500」「请先启动后端：cd server && go run ./cmd/api」。
- Enter 送达：isCta=true（cls=btn btn-primary, txt=重试）；guestCount 1→2、重发 `POST /auth/guest` —— retry 走 `await store.login()` 而非导航，属 handler 语义差异，非键盘失效。
- 改前基线：重试 role=null/tabIndex=null/cursor=auto、不在键盘序列 —— 已修复。

## §4 截图证据（sha256sum 实物，UNIQUE 6/6）

| 文件 | sha256 | 字节 |
|---|---|---|
| r102-index-390.png | 4a5f89beb08530201949b9531199224afb9149995e8106eca214c4260946bf7d | 见截图 |
| r102-index-768.png | c8f0865db5c2d649f7b0d864906ea4f84f5e4b5a37456ef3db261e88f4a81bca | 99499 |
| r102-index-1280.png | b72320fbd95b3fc0e0daedfd08720cc6a75034b463a9c403fca77d7681bd8d12 | 见截图 |
| r102-err-390.png | 8b05db7d2fd28c7132b091c56ddd1ff4e20b47107d396b7777fc09be12ee129a | 见截图 |
| r102-err-768.png | 257fc37c9d13a0abeedbd540e88ce29c49cb805c8627e1f59be6709477cd5d4f | 见截图 |
| r102-err-1280.png | 9b19436fb0459d826c2bde75aa33a63c2670510fb84e1e11fcb2813215c1a4bb | 55564 |

亲读 2 张非登录页：r102-index-768（首页正常态：QA玩家/游客账号资源栏、构筑评分 D30 雷达图、五维补强 CTA「→ 去背包配元素」「→ 看反应搭配」）；r102-err-1280（错误态：无法连接服务器/请求失败 HTTP 500 + 橙红渐变「重试」大按钮）。另两张 r102-explore-390 / r102-error-390 为改前侦察遗留，不计入本轮 6 张。

## §5 静态回归与门禁

静态（data/static-out.txt）：令牌三锚点 tokens.css:14 `--lyp-token-version: lyp-tokens-v1` / :15 `--lyp-r3-version: lyp-tokens-v3-r3` / :16 `--lyp-r6-version: lyp-tokens-r6` 在；五处 CTA 三件套源码在；不变量 go(d.hint):189 / ann-item aria-expanded:119 在；业务零 `--r102-`（NO_MATCH_PASS）。焦点环 common.css:913 `[role="button"]{cursor:pointer}`、:914-919 `:focus-visible{outline:3rpx solid var(--accent)}` 在。

| 门禁 | 命令/对象 | 结果 | RC |
|---|---|---|---|
| G1 | r102IndexCtaKeyboard.contract | 9 passed | 0 |
| G2 | r60A11yKeyboard.contract | 6 passed | 0 |
| G3 | r99LinkKeyboard.contract | 24 passed | 0 |
| G4 | r100ClickableView.contract | 12 passed | 0 |
| G5 | r101SemanticA11y.contract | 9 passed | 0 |
| G6 | index.test.ts | 27 passed | 0 |
| G7 | miniapp 全量 vitest | 234 passed / 1 skipped (235 files)；2150 passed / 2 skipped (2152 tests) | 0 |
| G8 | vue-tsc（type-check） | RC0 | 0 |
| G9 | build:h5 | RC0 | 0 |
| G10 | git diff --check | RC0 | 0 |

data/gates-out.txt 末行 GATES_DONE。G6/G7 中 `[Vue warn]: Failed to resolve directive: aria-native` 与 `[来一炮] build.attacker 缺失` 为既有告警，非本轮引入。

## §6 诚实边界

- 后端 8080 未起，全部 UI 数据（2 条合成公告、1 条合成战报、构筑评级、占位令牌 r102-qa-placeholder-token）由 init-script 合成，零真值；坐实的是客户端 DOM/AX/键盘因果链。
- 错误态由 r102-init-err.js 将令牌过期时间置 2000 年 + 首次 guest 返回 500 合成，非真实服务故障。
- 焦点环探针 outlineStyle=none：JS `element.focus()` 不触发 `:focus-visible`，故截图/探针未见橙色环；焦点环样式由 common.css:914-919 源码静态保证，改前 recon 已在 768 见 3px 环。
- Enter 导航用 stub 拦截 `uni.navigateTo`（Promise.resolve）记录 URL，避免真实跳转破坏后续探针；坐实 handler 同名与目标正确，未做跳转后页面 DOM 断言。
- annitem Enter 送达（isCta=true）但未做展开态 DOM 断言（AX 仍 expanded=false）；展开态切换由 R101 已坐实，本轮守不回退。
- 截图亲读 2/6，其余 sha 独立重算 UNIQUE 6/6。

## §7 结论与 hygiene

判定 PASS。五处 CTA 三视口真浏览器坐实 role=button/tabIndex=0/AX button/真 Tab 命中/Enter 同 handler，错误态重试恢复路径可达，两处不变量未回退，零横溢，门禁 G1-G10 全 RC0。

hygiene：HEAD=5ec099e5508b853a4cc9b8936d09d76ec1020589 未变；git status 非??项仅 M .gitignore + M laiyipao/miniapp/src/pages/index/index.vue（均预期 t271）；auth 五路径（.env / admin/.env.local.keys / deploy/.local.keys / server/.local.keys / miniapp/.env.local）全 ABSENT；P8080=000；占位令牌仅 localStorage 内存注入零真值；本轮未连 QA 库。自启 H5 dev server job 收尾清理并复验 P5174。
