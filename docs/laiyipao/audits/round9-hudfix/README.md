# R9-BUG-1：移动视口战斗 HUD 胶囊竖排修复（t95）

> 环境：本地回环 H5 真浏览器（agent-browser / Chromium，会话 `lyp-gp-t95`，视口 390×844）。
> **非物理真机**。所有截图均为本地回环页面，不含真实域名、IP、口令或私钥。

## 1. 缺陷与证据

design-reviewer t93 终审发现：390×844 移动视口战斗页顶部 HUD 胶囊逐字竖排，无法阅读。旧证据 `docs/laiyipao/audits/round9-after/screenshots/r9-h5-battle.png`（390×844，98812 bytes，sha256 `2cc968564d185336f9f4975300780df049cf45b21e9d6064fe131cf6793da535`）：波次、击杀、距 1 星、钳分文案均被压成单字宽竖列。

根因：`laiyipao/miniapp/src/pages/battle/battle.vue` 的 `.hud-row` 只有横向 flex，没有换行策略；`.hud-pill` 为 `inline-flex` 且可收缩；文字没有 `white-space: nowrap`。R7 星级差额胶囊与 R8 钳分胶囊叠加后，五项在 390px 宽度内被 flex 压缩，文字逐字换行。

## 2. test-first RED→GREEN

在 `laiyipao/miniapp/src/pages/battle/battle.test.ts` 新增「R9-BUG-1 移动视口 HUD 胶囊不竖排：nowrap/flex-shrink:0 且窄屏换行」：先挂载页面并注入星级门槛与波次，使四枚胶囊全部出现；再读取 `battle.vue` 的样式契约，断言 `.hud-row` 有 `flex-wrap: wrap`、`.hud-pill` 有 `flex-shrink: 0` 与 `white-space: nowrap`、`.time-budget-text` 有 `white-space: nowrap`、存在 `@media (max-width: 430px)` 且窄屏隐藏 spacer。

RED：`npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts -t "R9-BUG-1" --reporter=verbose` → exit 1，逐字失败 `expected ... to match /\.hud-row\s*\{[^}]*flex-wrap:\s*wrap/`。

GREEN 修改 `laiyipao/miniapp/src/pages/battle/battle.vue`：

- `.hud-row` 增加 `flex-wrap: wrap; align-content: flex-start;` 并缩小 gap。
- `.hud-pill` 增加 `flex: 0 0 auto; flex-shrink: 0; white-space: nowrap;`，高度与内边距略缩。
- `.hp-box` 增加 `flex-shrink: 0`，文字类胶囊统一 `white-space: nowrap`。
- 新增 `@media (max-width: 430px)`：隐藏 spacer、进一步缩窄胶囊与字体；同时给 `.reaction-line` 留出换行空间，保留 AI 因果链可读。
- `pointer-events` 仍由 `.overlay-top` 统一关闭，HUD 不拦截战场点击。

GREEN 定向：1 passed | 51 skipped。

## 3. 真浏览器复测（非物理真机 / 本地回环）

打开 `http://127.0.0.1:5174/?r9hud=1#/pages/battle/battle?level=1`，视口 390×844。实测 DOM 几何：

- `.hud-row` 宽 369px、高 85px（两行，不再把字挤竖）。
- `波次 1 / 5`：x=150,y=26,w=59,h=25；`击杀 3`：x=213,y=26,w=42,h=25；`距 1 星 10423`：x=259,y=26,w=79,h=25。
- `钳分中 · 距满分 125s`：x=10,y=70,w=110,h=25。四枚胶囊 `white-space=nowrap`、`flex-shrink=0`。
- 反应因果链同屏可读：`焰+冰 → 蒸汽爆发 -222`、`焰+电 → 过热 -73` 与 AI 依据 `reaction=steam_burst/damage=222`。

真实点击「全部不要了，直接下一波」后继续战斗：战场、热量条、技能槽完整可见，HUD 与反应浮层只占顶部，未遮挡操作按钮。浏览器 `errors --json` 全程 `errors=[]`。

截图（均 390×844 PNG，非物理真机）：

- `screenshots/r9hud-01-mobile-hud.png`：97376 bytes，sha256 `19cdae6286378d066344df192fa0cd833223b69cdb874751f6129395b4953dfa`；开局 HUD 四胶囊横向可读 + 因果链 + 选卡反馈。
- `screenshots/r9hud-02-mobile-combat.png`：94689 bytes，sha256 `843322e69b04ace5d91a4bb0f351b7805f47e3bb7dd6860a16eb346b13fa916d`；真实跳过后战斗画面，HUD 不遮挡战场与技能槽。

两张新图 sha256 均与旧图 `2cc96856…` 不同。

## 4. 验证

```bash
cd laiyipao/miniapp
npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts -t "R9-BUG-1" --reporter=verbose   # 1 passed | 51 skipped
npm test                                                                                                   # 178 files passed | 1 skipped; 1699 passed | 2 skipped
npm run type-check                                                                                         # exit 0
npm run type-check:test                                                                                    # exit 0
```

## 5. 边界

仅修改 `laiyipao/miniapp/src/pages/battle/battle.vue` 与其测试，以及本证据目录；未碰 admin、后端、stage.vue；未执行 git；未新增 CSS 令牌；未写入真实凭据/域名/私钥。