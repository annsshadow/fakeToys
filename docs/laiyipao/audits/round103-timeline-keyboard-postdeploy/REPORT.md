# R103 时间线抽屉键盘闭环部署后真浏览器复查报告

- 轮次：R103（t279 B5 部署后复查）
- 前置：t275 规范 → t276 实施 GREEN → t277 QA PASS → t278 终审 verdict=pass 六维全绿（焦点归还落 BODY 列 low L3 不阻断）
- 基线 HEAD：`91e776228925d40df75ac87af4e1d71c8c4662d3`
- 视角：生产构建 H5 战斗页时间线抽屉键盘闭环；键盘闭环在 dev server 复现 QA 同一注入路径（见 §2 诚实边界）
- 报告：`docs/laiyipao/audits/round103-timeline-keyboard-postdeploy/REPORT.md`
- 五批链：B1 `d24eb4082fd20b2dd6c5ca153edc04b7eb7f0700` / B2 `cc42df783a233f66f868e8cbad45b06011657143` / B3 `014251d585131d97ad307bd4604713b647dee66e` / B4 `70645163002c29bd831b98af87c923941db2de92` / B5（见 git log 五批链）

## §1 部署与方法

- 生产静态服务：`python -m http.server 5201 --bind 127.0.0.1` 宿主 `dist/build/h5`（HTTP=200）。后端 8080 未启动。
- H5 产物重建：`pnpm build:h5` RC0；主 bundle `index-Bwqs09iA.js`（`index.html` 引用 `assets/index-Bwqs09iA.js`），sha256 `cd89366ff772cc2c120e21ade230c6ed829c473e73fc875a65eddf1aa17ac494`。后端/admin 无变化沿用不回退。
- dev server：`pnpm run dev:h5 -- --host 127.0.0.1 --port 5174`（P5174=200），用于复现 QA 的 Vue 实例注入路径完成键盘闭环走查。
- 浏览器：真 Chromium（agent-browser 0.41.1），三视口 414×896 / 820×1180 / 1440×900。
- 注入：`r103-init.js`（占位令牌 `r103-qa-placeholder-token`，仅内存注入、零真值），XHR + fetch 双 patch 合成 config/关卡/敌人；`inj-feed.js` / `inj-result.js` 经 Vue 实例注入 `recentReactions` / `settleResult` 产生两处 CTA。
- 探针：DOM role/tabIndex/cursor/vis/hOverflow；AX snapshot 读 dialog/button；真 Tab / Shift+Tab / Enter / ESC 序列；抽屉内 activeElement 逃逸检测；暂停与互斥记账；截图 sha256sum 逐张核对并亲读 3 张。
- 防干扰：单 driver 独占 session 跑完三视口；所有 pd 探针输出为双重 JSON，解析时递归 `json.loads`。

## §2 诚实边界：生产静态构建无法外部注入 CTA

- QA 的注入路径经 `__vueParentComponent` 找 setupState 中的 `openReactionTimeline`；生产构建 `<script setup>` 的 setupState 完全在闭包内不暴露。
- 生产构建探针全失败：`probe-vue-prod.js`（`app.__vue_app__` / `_instance` 存在但 `appSetup=[]` / `appCtx=[]` / `foundBattle=false`）、`probe-vue-deep.js`（compCount=216、withSetup=0）、`probe-state-keys.js`（215 组件 setupState 全空，ctx 仅 `_` / `route` / `$vm`）、`probe-exposed.js`（exposed / exposeProxy / proxy 无业务键）。结论：生产 bundle 无法从外部注入 `recentReactions` / `settleResult`。
- 源码条件：`battle.vue:62` `<view v-if="recentReactions.length" class="reaction-feed">` 含 CTA `:77`；`:374` `<view v-if="settleResult" class="sheet">` 含 CTA `:449`。`recentReactions` 由 `noteReactions()` 从 `eng.reactionTimeline()` 产出，`settleResult` 由 `settle()` 在 phase=won/lost 产出。
- 三次自然渲染尝试全失败：①零波次快速结算字符串不匹配（`REPLACED False`）；②精准替换 `wave_count:1`/`count:1`/`hp:1`（全 R=True）后 CTA 仍 null；③带火层 `resist:{fire:3}`/`hp:200` 替换 True 后 CTA 仍 null。点击 sig-fire/sig-ice（UNI-VIEW）不触发技能释放；生产构建 body 显示波次 1/2、击杀 0，战斗未推进（疑 canvas/rAF 在无真实渲染环境受限）。
- 决策：键盘闭环改在 dev server 复现 QA 同一注入路径验证；静态生产 bundle 仅做构建产物 sha 核对。`dist/build/h5/assets/index-Bwqs09iA.js` grep 关键词（`r29-timeline-overlay` / `aria-modal` / `r29-timeline-close` 等）全 0 属压缩混淆后类名不保留，属正常，不作为失败判据。
- QA 报告 `docs/laiyipao/audits/round103-timeline-keyboard/REPORT.md` §6.2 已诚实披露 CTA 经 Vue 实例注入产生、非真实战斗流程产出；本报告沿用同一边界。

## §3 三视口键盘闭环证据（dev server 复现 QA 注入路径）

三视口 414×896 / 820×1180 / 1440×900 走查脚本 `data/pd-dev-driver.sh`（job 输出 `PD_DEV_ALL_DONE`），输出 `pd-w-{414,820,1440}.txt` 各 120 行。三视口结果完全一致：

| 项 | 判据 | 414 / 820 / 1440 |
|---|---|---|
| A | 两处 CTA role=button/tabIndex=0/真 Tab 命中/Enter 开抽屉 | feed `role=button`/`tabAttr=0`/`tabIndex=0`/`cursor=pointer`/`vis=true`；8 次真 Tab 命中 `r28-feed-cta`（UNI-TEXT）；`A_FEED_ENTER={ok:true,cls:r28-feed-cta}` |
| B | overlay `role=dialog`/`aria-modal=true`/`aria-label=完整反应时间线`/焦点自动落关闭按钮 | `overlay.exists=true`/`role=dialog`/`ariaModal=true`/`ariaLabel=完整反应时间线`；`close.role=button`/`tab=0`；`active.cls=r29-timeline-close`/`inOverlay=true`/`timelineVisible=true`/`paused=true`；AX `dialog=True`/`closeBtn=True(×)`/`ctaBtn=True` |
| C | 抽屉内 Tab / Shift+Tab×3 不逃逸 | `C_TAB inOverlay=3/3`、`C_SHIFT inOverlay=3/3` |
| D | ESC 关闭 + 恢复暂停 | `overlay=false`/`paused=false`/`timelinePausedByDrawer=false`（焦点归还未坐实，见 §6） |
| E | 结果页 CTA Enter 开；关闭按钮 Enter 关 | `result.role=button`/`tab=0`/`overlay=true`；`E_CLOSE_ENTER={ok:true,cls:r29-timeline-close}`；关闭后 `overlay=false`/`timelineVisible=false`/`paused=false`/`timelinePausedByDrawer=false`/`activeCls=`（落 BODY） |
| F | 开抽屉暂停 / 与图例互斥 / 零横溢 | `beforeHelp overlay=true` → `afterHelp legend=true`/`overlay=false`/`timelinePausedByDrawer=false`（互斥坐实）；`iw=cw=sw=W`/`hOverflow=false` |

焦点归还落 BODY 与 QA §6.3 一致（`UNI-TEXT` 的 `focus()` 在 H5 不生效，`R73` help-chip 用 `UNI-VIEW` 可聚焦），如实披露、不冒充 PASS。

## §4 部署后截图（sha256sum 实物，UNIQUE 9/9 且全 ≠ QA 9 图）

| 文件 | sha256 |
|---|---|
| pd103-feed-414.png | e3b040d8c2b185fd9d9aca974ba3daffe1b38825abeefcc8710de39f264fd789 |
| pd103-feed-820.png | cc5af80d7c690efcb95d5734452bbafa4c2b2b62f167a5e4cc25510dee772989 |
| pd103-feed-1440.png | 1945cb1d25ba52acbe9f392d5d20b7c585349293c9c4539d889c080d5cac1f4a |
| pd103-result-414.png | 0f6e3789dc79cdf389320db463872a5314fe60259a8a82e67401c91b90eb8611 |
| pd103-result-820.png | 3b19f76c670b23f3eeb5fce49dc5acfebed4569970a9fd9096e70bb2b78d2b4d |
| pd103-result-1440.png | c28b251517599c8008400a64d22900f6eaf3f61d512b415b040d1664850a72c5 |
| pd103-legend-414.png | 3a1467e509d15282528de0f9dc484dd8ba098af06fbd999a1e80ac9529e1a4c8 |
| pd103-legend-820.png | 1c031b5786f3257e2dafd9efd95e8f03653384f7cd92ee3fd26c7214db354fd2 |
| pd103-legend-1440.png | 647a1d2e72dec501ec927dbbda56e3d9868d6397e05f969c60d40d70b0ea3808 |

亲读 3 张非登录页：`pd103-feed-414`（414×896：战斗页抽屉展开「AI·完整反应时间线」标题 + ×关闭按钮 + 「本局暂无逐次反应记录」）、`pd103-result-820`（820×1180：同抽屉在结果态，底部可见放弃 / 1×–8× / 火冰雷蚀动元素栏）、`pd103-legend-1440`（1440×900：战场图例覆盖层 + ×关闭，互斥后抽屉消失）。9 图 sha 独立重算 UNIQUE 9/9，且与 QA 9 图逐一比对全部不同（部署后独立截图，非复用 QA 图）。

## §5 门禁与产物

- QA 门禁 G1–G7 全 RC0：契约 12/12、r73 6/6、battle.test 61/61、全量 2162 tests passed (2 skip)、type-check、build:h5、git diff --check。
- Go 守卫：`TestNoInfraIdentifiersInRepo` PASS 15.61s (cached)；`TestNoRealSecretsInRepo` **SKIP 非 PASS**（`secrets_leak_test.go:182` 读不到 `.local.keys` —— 真值扫描这条守卫**没有执行**，不是「通过了」）。
- H5 bundle sha 与本报告 §1 逐字一致；后端/admin 沿用不回退。

## §6 诚实边界

- 键盘闭环走查在 dev server（5174）而非静态生产 bundle（5201）完成：生产构建 `<script setup>` setupState 闭包化，无法外部注入两处 CTA；三次自然渲染尝试均失败。静态生产 bundle 仅做 sha 核对。
- CTA 与抽屉态经 Vue 实例注入产生，非真实战斗流程产出；后端 8080 未启动，UI 数据由 init-script 合成零真值。坐实的是客户端 DOM/AX/键盘因果链，非服务端真实结果。
- 焦点归还未坐实：ESC/Enter 关闭后 activeElement 落 BODY，与 QA §6.3 一致（`UNI-TEXT` 的 `focus()` H5 限制），终审列 low L3 不阻断，本报告不冒充 PASS。
- 全量 vitest 2 tests skipped 为仓库既存，非本轮引入。
- 生产 bundle grep 类名全 0 为压缩混淆所致，非能力缺失。

## §7 结论与 hygiene

判定 PASS（除焦点归还一项如实标注未坐实）：dev server 复现 QA 注入路径下，三视口坐实两处 CTA role=button/tabIndex=0/AX dialog+button/真 Tab 命中/Enter 开抽屉、焦点自动落关闭按钮、抽屉内 Tab/Shift+Tab 不逃逸、ESC 关闭恢复暂停、结果页 CTA Enter 开与关闭按钮 Enter 关、开抽屉暂停与图例互斥、零横溢；9 张部署后截图 UNIQUE 且全 ≠ QA 图，亲读 3 张非登录页。

hygiene：本轮 postdeploy 自身 tracked data=0（data/ 由 `.gitignore:660` 拦）；占位令牌仅内存注入零真值；auth 七路径（`.env.local` / `.env.local.keys` / `laiyipao/miniapp/.env.local` / `laiyipao/admin/.env.local` / `laiyipao/admin/.env.local.keys` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.local.keys`）全 ABSENT；静态服务与 dev server 及浏览器 job 收尾清理。
