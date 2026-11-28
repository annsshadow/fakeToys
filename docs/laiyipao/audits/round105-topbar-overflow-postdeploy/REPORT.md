# R105 index 顶栏 768 横溢修复 —— 部署后真浏览器复查报告

- 轮次：R105（round105-topbar-overflow-postdeploy）｜契约：`laiyipao/miniapp/src/styles/r105TopbarOverflow.contract.test.ts`（10 it）
- 执行：same-route-rel｜三视口 414×896 / 820×1180 / 1440×900｜agent-browser 0.41.1（真实 Chromium）
- 基线：发布起点见 git log（本报告不自引当前提交 hash，避免自引悖论）
- 前置：t289 真浏览器坐实改前 768 溢出 12px → t290 规范 RED → t291 实施 GREEN → t292 QA PASS → t293 终审 verdict=pass → 本轮部署后复查
- 写域：仅 `docs/laiyipao/audits/round105-topbar-overflow-postdeploy/`（data/ 由 .gitignore:671 拦，不入库）

## §1 方法与诚实边界

H5 产物按 B5 重建（bundle 见 §5），后端 8080 未启动，采用 QA 同一 `init-script` 路径（data/r105-init.js）合成 config/公告/战报/钱包，令牌 `r105-qa-placeholder-token`（exp 2099）仅内存注入、占位符零真值；钱包取真实量级 coin=52000/gem=4321/energy=320/keys=7，与 t289 基线同量级。

部署后 DOM/AX/键盘/横溢复查在 **dev server（5174）** 上复现 QA 注入路径完成。原因（与 R103/R104 同根因）：生产静态构建的 `<script setup>` setupState 完全闭包化，外部注入脚本遍历 `__vueParentComponent` 链找不到业务 setupState，无法注入合成配置；静态 bundle 仅做产物名与 sha256 核对，不冒充生产包内 DOM 实测。此边界如实披露。

三视口一律 open 之后重设 viewport（QA 教训：open 之前设 viewport 会被导航重置为 1258 致样本作废），每视口 close --all 干净会话，注入后 wait 1500ms 再取探针。

## §2 三视口横溢与 computed 样式（核心修复）

| 视口 | iw/cw/sw | hOverflow | topbar maxW / boxSizing | resRow flexWrap / whiteSpace | resRow right / esc | playerInfo minW |
|---|---|---|---|---|---|---|
| 414 | 414/414/414 | false | 100% / border-box | wrap / nowrap | 400.77 / false | 0px |
| 820 | 820/820/820 | false | 100% / border-box | wrap / nowrap | 663.77 / false | 0px |
| 1440 | 1440/1440/1440 | false | 100% / border-box | nowrap / nowrap | 988 / false | 0px |

- **核心修复坐实**：窄屏 414/820（≤900px 命中媒体查询）`.res-row` computed `flex-wrap:wrap`，四胶囊换行落在限宽容器内，iw=cw=sw、零横溢；宽屏 1440（>900px）保持基础 `flex-wrap:nowrap` + `white-space:nowrap`，基础 nowrap 未回退。
- `.topbar` 三视口 `max-width:100%` + `box-sizing:border-box` 均在，`display:flex` / `justify-content:space-between` 不变；`.player-info` computed `min-width:0px` 三视口均在，允许 flex 子项收缩。
- 对照 t289 改前 real-768（coin=52000）scrollWidth=780 溢出 12px；改后本轮 820 视口（同样限宽 560 居中）resRow.right=663.77 落在 topbar 限宽容器内、零横溢。改前 A/B 未在本轮重做，引用 t289 真浏览器证据。

## §3 四胶囊完整性与玩家信息

四胶囊 coin=52000 / gem=4321 / energy=320 / 🔑7 三视口完整渲染、无裁切：414 下 🔑7 折到第二行（right=168.38）仍在 res-row 内；820 下四胶囊分两行（52000/4321 第一行、320/🔑7 第二行）；1440 下四胶囊单行 nowrap（right 依次 782.11/861.13/932.73/988）。玩家信息「QA玩家 / 游客账号」可断行但被 `min-width:0` 限制在容器内，不再向外溢出。isLogin=false、bodyLen=775，非登录页。

## §4 键盘不变量与 AX

三视口各 12 次真 Tab 序列完全一致：`ai-cta 去补强 →` → `ann-all 全部 2 条 →` → `ann-item×2` → `dim-hint×5` → `mg-row×3`。每个命中元素 computed `role=button`、`tabIndex=0`、逐元素 `scrollWidth=clientWidth`（无横溢）。AX 完整快照三视口均 33 个 button，`ann-item` 两条带 `expanded=false`（折叠态，与 QA 一致），`ann-all` / `dim-hint link` / `mg-row tap` 均为 button。

- R99/R102 不变量：`dim-hint`（go(d.hint)）5 条、`ann-all` 真 Tab 命中且 role=button/tabIndex=0；retry 三件套本轮正常态 `retryN=0` 未坐实真浏览器，由 r99/r102 契约静态守住，不冒充 PASS。
- R100/R101 不变量：两条 `ann-item` 带 `aria-expanded="false"` 且真 Tab 命中；展开态切换由 R101 报告覆盖，本轮守不回退。
- 令牌三锚点 `tokens.css:14/15/16`（--lyp-token-version / --lyp-r3-version / --lyp-r6-version）未动，业务源码零 `--r105-`。

## §5 构建产物与门禁

- H5 `build:h5`（pnpm run build:h5，RC=0）：主 bundle `index-Lm_ZCtus.js`（index.html 引用 `assets/index-Lm_ZCtus.js`），sha256 = `27c418460a281843f7301ae6a24dbd6a0fdbe838731584be7240bb0e68b9a439`。后端/admin 本轮无变化，沿用不回退。
- QA 门禁 G1–G7 独立复跑全 RC=0：G1 r105 契约 10/10、G2 a11y 五契约 64/64、G3 index.test 27/27、G4 全量 237 passed|1 skipped files / 2185 passed|2 skipped tests、G5 type-check、G6 build:h5、G7 git diff --check。
- Go 守卫：`TestNoInfraIdentifiersInRepo` PASS 8.22s；`TestNoRealSecretsInRepo` **SKIP 非 PASS**（`secrets_leak_test.go:182` 读不到 `.local.keys` —— 真值扫描这条守卫**没有执行**，不是「通过了」），如实披露。

## §6 部署后截图 sha256sum（UNIQUE 6/6，与 QA6 图交集 0）

| 截图 | sha256 |
|---|---|
| pd105-index-414.png | `4740eb5753b531e75506cacc82da3329a51c6d139892fed569f2ede905be43ab` |
| pd105-index-820.png | `c6d2625cc5e46f3c946c0fd6e74a2fe2054fbea8784171a78619608877242fce` |
| pd105-index-1440.png | `a064edf2f3f77de68ae5269681e8beaf0133201764c8fb3218e819d5ddc2edfd` |
| pd105-topbar-414.png | `f8a96fd7f693c65546e718b1f2beac3cd90b6bde323a1da1f707830dbb347a1e` |
| pd105-topbar-820.png | `46d2009d4a393103f65af091e41f8ef8ca7a6a99fa7dee43431a4038861a17a4` |
| pd105-topbar-1440.png | `449dfdee04393d34c4e46defed9ee0d83c05ad5a50e0d0b81da57c445115bbaf` |

亲读 3 张非登录页：`pd105-index-414.png`（414×896 首页，顶栏四胶囊，🔑7 折到第二行仍在容器内，AI 战术洞察 / 公告 / 构筑评分）；`pd105-topbar-820.png`（560×197 顶栏特写，四胶囊分两行完整渲染，玩家信息「游客账号」断行在容器内、无裁切）；`pd105-index-1440.png`（1440×900 桌面首页，顶栏四胶囊单行 nowrap、限宽 560 居中）。视觉与探针数据一致。其余 3 张以实测 sha 与探针为准。

## §7 结论与 hygiene

**结论：PASS。** R105 顶栏 768 横溢修复在三视口真浏览器坐实——414/820 窄屏 flex-wrap 生效、1440 宽屏 nowrap 保留，三视口 iw=cw=sw 零横溢；`.topbar{max-width:100%;box-sizing:border-box}`、`.player-info{min-width:0}`、≤900px `.res-row{flex-wrap:wrap}` 三改点 computed 全生效；四胶囊与玩家信息完整、键盘不变量与 AX 保持、门禁全绿。

- hygiene：auth 五路径全 ABSENT（.env / laiyipao/admin/.env.local.keys / laiyipao/deploy/.local.keys / laiyipao/server/.local.keys / laiyipao/miniapp/.env.local）；`.gitignore` 671 行，R105 四规则 668-671；tracked 本轮 data=0（postdeploy data/ 由 .gitignore:671 拦）。
- 占位令牌 `r105-qa-placeholder-token` 仅内存注入，未写入任何入库文件；报告零真值。
- 诚实边界汇总：dev server 复现 QA 注入路径（生产 bundle setupState 闭包化无法注入，静态包仅 sha 核对）；后端 8080 未起全合成；retry 三件套未真浏览器坐实；改前 12px 对照引用 t289 未重做 A/B；截图亲读 3/6。
