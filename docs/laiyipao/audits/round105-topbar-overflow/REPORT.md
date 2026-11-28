# R105 顶栏 768 横向溢出修复 —— 真浏览器 QA 报告

- 轮次：R105（round105-topbar-overflow）｜契约：`laiyipao/miniapp/src/styles/r105TopbarOverflow.contract.test.ts`（10 it）
- 执行：same-route-qa｜三视口 390×844 / 768×1024 / 1280×800｜agent-browser 0.41.1（真实 Chromium）
- 基线 HEAD：`0151ce97e1216d70efd899e85d72e7ad2f93b750`（全程未变）
- 改前对照：`docs/laiyipao/recon/ROUND105_C_TOPBAR_OVERFLOW_VERIFY.md`（t289 真浏览器坐实 768 溢出 12px）

## §1 范围与方法

验证首页 `.topbar` 三处修复在真实浏览器生效：`.topbar{max-width:100%;box-sizing:border-box}`、`.player-info{min-width:0}`、窄屏（≤900px）`.res-row{flex-wrap:wrap}`（`index.vue:757/758/784/799-801`），并保持宽屏基础 `white-space:nowrap` 不回退。

后端 8080 未启动，采用 `init-script`（`data/r105-init.js`）拦截 XHR/fetch 合成响应：钱包数值用真实量级 coin=52000 / gem=4321 / energy=320 / keys=7（与 t289 real 基线同量级），配置缓存与令牌为合成占位（`r105-qa-placeholder-token`，仅内存注入，非真值），不发起真实请求。三视口均 `isLogin=false`、非登录页。

关键方法教训：首轮走查脚本在 `open --init-script` 之前设置 viewport，导航把视口重置为 1258，三视口数据作废；改为 open 之后重设 viewport 重跑，本报告的探针全部来自修正后的有效样本（`data/walk-out.txt`，含 `WALK_ALL_DONE`）。

## §2 三视口横向溢出与 computed 样式（A / B 项）

| 视口 | clientWidth | scrollWidth | hOverflow | topbar maxW / boxSizing | topbar w / left / right | resRow flexWrap / whiteSpace | playerInfo minW |
|---|---|---|---|---|---|---|---|
| 390 | 390 | 390 | false | 100% / border-box | 390 / 0 / 390 | wrap / nowrap | 0px |
| 768 | 768 | **768** | **false** | 100% / border-box | 560 / 104 / 664 | **wrap** / nowrap | 0px |
| 1280 | 1280 | 1280 | false | 100% / border-box | 560 / 360 / 920 | nowrap / nowrap | 0px |

- **核心修复坐实**：768 视口 `scrollWidth=768=clientWidth`、`hOverflow=false`；对照 t289 改前 real-768（coin=52000）`scrollWidth=780`、溢出 12px。改后 `.res-row.right=639.44`，完全落在 `.topbar.right=664` 限宽容器内（改前 resRow.right=780.09 远超 664）。
- 断点正确：390/768（≤900px）命中媒体查询 `flex-wrap:wrap`；1280（>900px）保持基础 `white-space:nowrap` + `flex-wrap:nowrap`，证明基础 nowrap 未被破坏、断点按预期生效。
- `.topbar` 三视口 `display:flex` / `justify-content:space-between` 保持不变；`max-width:100%` + `border-box` 三视口均在：窄屏 390 撑满 390，宽屏 768/1280 限宽 560 居中。
- `.player-info{min-width:0}` 三视口 computed 均为 `0px`，允许 flex 子项收缩，不再撑破容器。

## §3 四胶囊与玩家信息完整性（C 项）

| 视口 | 52000 | 4321 | 320 | 🔑7 | resRow right / topbar right |
|---|---|---|---|---|---|
| 390 | 82.77px | 75.08px | 67.38px | 51.16px | 377.53 / 390 |
| 768 | 142.20px | 127.05px | 111.91px | 98.75px | 639.44 / 664 |
| 1280 | 80.42px | 73.02px | 65.61px | 49.27px | 908 / 920 |

四胶囊 coin/gem/energy/keys 文案与数值三视口完整渲染、无裁切；768 下胶囊换行落在限宽容器内，🔑7 从第一行折到第二行但仍在 resRow 内；玩家信息「QA玩家 / 游客账号」可断行但被限制在容器内（playerInfo 70.88px，min-width=0），不再向外溢出。
## §4 键盘不变量与令牌（D / E 项）

三视口真 Tab 序列完全一致（各 12 次 Tab，见 `data/walk-out.txt` 的 TAB1–TAB12）：

`ai-cta「去补强 →」→ ann-all「全部 2 条 →」→ ann-item×2 → dim-hint×5 → mg-row×3`

- 每个命中元素 computed `role=button`、`tabIndex=0`，`scrollWidth=clientWidth`（逐元素无横溢）。
- R100/R101 不变量：两条 `ann-item` 均带 `aria-expanded="false"` 且真 Tab 命中，未被本次样式修复误改。
- R102 不变量：`ann-all` 真 Tab 命中且 `role=button`/`tabIndex=0`。
- R99 不变量：`dim-hint link`（即 `go(d.hint)` 补导入口）5 条全部真 Tab 命中且 `role=button`/`tabIndex=0`。
- 令牌锚点：`tokens.css:14 --lyp-token-version: lyp-tokens-v1`、`15 --lyp-r3-version: lyp-tokens-v3-r3`、`16 --lyp-r6-version: lyp-tokens-r6` 三处均在且未改动；业务源码零 `--r105-`（唯一 3 处字符串命中在契约 `r105TopbarOverflow.contract.test.ts` 自身的 it 断言/注释中，非业务 CSS）。

## §5 门禁（F 项）

| 编号 | 项目 | 结果 | RC |
|---|---|---|---|
| G1 | r105TopbarOverflow 契约 | 10 passed | 0 |
| G2 | a11y 五契约（r104/r101/r99/r60/r100） | 64 passed | 0 |
| G3 | index.test.ts | 27 passed | 0 |
| G4 | miniapp 全量 vitest | 237 passed \| 1 skipped files；2185 passed \| 2 skipped tests | 0 |
| G5 | vue-tsc / type-check | 通过 | 0 |
| G6 | build:h5 | 通过 | 0 |
| G7 | git diff --check | 通过 | 0 |

证据：`data/gates-out.txt`（`GATES_DONE`，各段 `RC=0`）。G3/G4 中 `[来一炮] build.attacker 缺失，正在使用本地占位属性` 为测试既有 stderr 告警，非本轮引入，不影响 passed。

## §6 截图 sha256sum 与亲读（G 项）

6 张截图（每视口 1 张整页 + 1 张顶栏特写），`sha256sum` 落盘实物实测，UNIQUE 6/6：

| 截图 | sha256 |
|---|---|
| r105-index-390.png | `deb4df460671d380474dff8953c93b611a9c4870f62b7ee1aac8dad9c2727775` |
| r105-index-768.png | `1f742f71addf85b04d15dfba0daa7e44194d57797154417f2ebf1f82e7ea0e26` |
| r105-index-1280.png | `7626e313663a478f398850d0dc74c0c6ab13a8dbce7e41d522a22955aa2c7eb8` |
| r105-topbar-390.png | `9a0ef333df6b1690fd7942e24fc4c8c5bf4b53d0a5c5eecf9d0de3ebde5e63c7` |
| r105-topbar-768.png | `c758b0693ed196ae43b41dd6bd5c99db081e8b8b028a4917cb6a856a53eb424b` |
| r105-topbar-1280.png | `40cd0bd5c9ab520c7f44b5d3d817d45b11fc467e329a2ec5acc91d5c6063b2b8` |

亲读 2 张非登录页：`r105-index-768.png`（768×1024，整页首页，构筑评分 D30 / 五维补强卡、机制深度条目高亮可见）与 `r105-topbar-768.png`（560×186，顶栏特写，四胶囊 52000/4321/320/🔑7 分两行完整渲染、玩家信息「QA玩家/游客账号」断行在容器内、无裁切无溢出）。视觉与探针数据一致。

## §7 诚实边界

1. 后端 8080 未启动，全部数据经 `init-script` 合成，非真实服务端响应；钱包数值取真实量级 coin=52000 与 t289 基线一致，令牌 `r105-qa-placeholder-token` 为占位符仅内存注入，零真值。
2. 改前 12px 溢出结论引用 t289 真浏览器对照报告（`docs/laiyipao/recon/ROUND105_C_TOPBAR_OVERFLOW_VERIFY.md`），本轮未注入旧 CSS 重做 A/B，改后数据为本轮真浏览器实测。
3. 错误态 `retry` 三件套本轮未坐实：探针正常态无错误卡片（`retryN=0`），`retry` 的 role/tabIndex/keydown 由 r99/r102 契约静态守住，不冒充真浏览器 PASS。
4. 截图亲读 2/6（768 整页 + 768 顶栏特写）；其余 4 张以实测 sha 与探针数据为准。
5. `aria-expanded` 仅验证折叠态 `false` 与真 Tab 命中；展开态切换由 R101 报告覆盖，本轮守不回退。

## §8 结论与 hygiene

**结论：PASS。** R105 顶栏 768 横向溢出修复在三视口真浏览器坐实——768 `scrollWidth` 由改前 780 回落到 768（零横溢），computed 样式三项（`max-width:100%`+`border-box`、`min-width:0`、窄屏 `flex-wrap:wrap`）全部生效，宽屏基础 nowrap 不回退，四胶囊与玩家信息完整；键盘不变量与令牌三锚点保持，门禁 G1–G7 全 RC0。

- HEAD `0151ce97…` 全程未变；`git status` 非 `??` 项仅 `M .gitignore` + `M laiyipao/miniapp/src/pages/index/index.vue`（预期 t291 实施）。
- auth 五路径全部 ABSENT（`.env`、`laiyipao/admin/.env.local.keys`、`laiyipao/deploy/.local.keys`、`laiyipao/server/.local.keys`、`laiyipao/miniapp/.env.local`）。
- 自启 H5 dev server（5174）结束前 kill 并复验 P5174=000；后端 8080 未启动（P8080=000）。QA 库本轮未连接。
- 报告与 data/、screenshots/ 仅落在唯一写域 `docs/laiyipao/audits/round105-topbar-overflow/`；data/ 与 screenshots/ 为 UNTRACKED 证据，不入库。
