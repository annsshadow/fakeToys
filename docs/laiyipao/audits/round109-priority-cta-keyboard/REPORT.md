# R109 AI 优先级 CTA 键盘可达性 真浏览器复查报告

## 1. 范围与方法

- 对象：R109 在 10 个页面为 AI 优先级 CTA 补键盘三件套（role=button / tabindex=0 / @keydown.enter 同 handler）。其中 `.r12-priority-cta` 4 处（gems/skills/equipment/bag），view 版 `.r3-ai-cta` 6 处（bag/me/battles/enemies/rank/skins）；另有 text 版 `.r3-ai-cta` 9 处三件套为不变量。
- 视口：390×844、768×1024、1280×800 三视口，真 Chromium（agent-browser 0.41.1）。
- 机制：后端 8080 未起，用 init-script（data/r109-init.js）patch XHR.open/send 直接合成响应 + patch fetch + 注入 localStorage 配置缓存与占位令牌；补全 skills/enemies/skins/leaderboard/rank/battles 数据依赖，使 10 处 CTA 全部「自然渲染」（无需 Vue 实例注入）。全程零真值。
- 采集：每页 open(init-script) 后重设 viewport → DOM 探针（role/tabindex/tabIndexProp）→ AX snapshot → 注入 focusin 监听后真 Tab 10 次读焦点环 → stub uni.navigateTo/pageScrollTo 后分别 click 与 Enter 对比同 handler 效果 → 截图。
- 契约：laiyipao/miniapp/src/styles/r109PriorityCtaKeyboard.contract.test.ts（16 it）；SPEC：docs/laiyipao/specs/round109-priority-cta-keyboard/SPEC.md。

## 2. 三视口逐页 A/B 证据（CTA 属性 + 真 Tab + click/Enter 同 handler）

下表 10 处目标 CTA（4×r12-view + 6×r3-view）在三视口全部 role=button / tabindex="0" / tabIndexProp=0，真 Tab 焦点序列命中，click 与 Enter 触发同一 handler（导航 URL 或自指页内定位完全一致）。

| 页面 | 目标 CTA | role/tabindex/tabIndexProp | 真Tab命中 | click vs Enter |
|---|---|---|---|---|
| gems | r12 去配装备 | button / 0 / 0 | 是 | 同 → /pages/equipment/equipment |
| skills | r12 去配装备 | button / 0 / 0 | 是 | 同 → /pages/equipment/equipment |
| equipment | r12 在本页强化装备契合 | button / 0 / 0 | 是 | 同（自指，nav=[]，页内强化） |
| bag | r12 去配装备 + view 去技能槽 | 均 button / 0 / 0 | 是 | 同 → /pages/equipment/equipment |
| me | view 去配装备 | button / 0 / 0 | 是 | 同 → /pages/equipment/equipment |
| battles | view 看战报详情 | button / 0 / 0 | 是 | 同（自指展开战报，nav=[]） |
| enemies | view 切换对应元素 | button / 0 / 0 | 是 | 同（自指，counterElement 赋值） |
| rank | view 去冲榜关卡 | button / 0 / 0 | 是 | 同 → /pages/stage/stage |
| skins | view 查看同流派皮肤 | button / 0 / 0 | 是 | 同（自指，keyword 赋值） |

注：上表结论在三视口 390/768/1280 完全一致（逐视口原始数据在 data/w-390.txt、w-768.txt、w-1280.txt）。AX snapshot（gems 三视口）坐实 `- button "去配装备 →"`、`- button "去商城找宝石 →"`、`- button "去背包镶嵌 →"`，改前基线为 StaticText。

### 2.1 自指分支源码确认（非导航属设计）

- equipment.vue:270 `r12Go()` 内 `if (r12Cta.value.self)` 走页内强化，无 navigateTo。
- enemies.vue:273 `r4FocusEnemy()` 设 `counterElement.value = r4AiEnemy.value.target`（页内切换）。
- skins.vue:262 `r4FocusSkin()` 设 `keyword.value = r4AiSkin.value?.keyword ?? ""`（页内筛选）。
- battles.vue:881 `r4GoBattle()` 展开战报详情（页内）。
- 上述四处 click/Enter 均 nav=[] 且 scroll 一致，符合「自指分支页内定位不导航」口径。

## 3. 焦点环（C）

真 Tab 聚焦目标 CTA 时 focusin 监听读到橙色焦点环，三视口一致（active=true）：

| 视口 | outline | offset | 说明 |
|---|---|---|---|
| 390 | solid 1px rgb(255, 107, 53) | 1px | 3rpx 在小视口缩放为 1px |
| 768 | solid 3px rgb(255, 107, 53) | 2px | 3rpx 基准 |
| 1280 | solid 1px rgb(255, 107, 53) | 1px | 3rpx 在大视口按 DPR 缩放 |

颜色均 `rgb(255, 107, 53)`（--accent），线宽随 rpx 视口换算变化属正常。public common.css:914-921 两层 `:focus-visible` 未回退。

## 4. 不变量与静态回归（D）

- 零横溢：全部 9 页 × 3 视口 cw=sw=视口宽、hOverflow=false、isLogin=false。
- 令牌三锚点在：tokens.css:14 `--lyp-token-version: lyp-tokens-v1`、:15 `--lyp-r3-version: lyp-tokens-v3-r3`、:16 `--lyp-r6-version: lyp-tokens-r6`。
- 业务零 `--r109-`：src 全局仅 2 处命中，均在契约自身断言（r109PriorityCtaKeyboard.contract.test.ts），业务 CSS 零命中。
- text 版 r3-ai-cta 9 处三件套保留（announcements/equipment/gems/mastery/reactions/shop/signin/skills/stage），由契约静态守。
- R60/R96/R99/R100/R102/R104 既有键盘三件套由门禁 G2 五契约 48/48 守住。

## 5. 门禁（E，全部真实 RC=0，独立复跑）

| 项 | 命令/文件 | 结果 | RC |
|---|---|---|---|
| G1 | r109PriorityCtaKeyboard.contract.test.ts | 16 passed (16) | 0 |
| G2 | 五 a11y 契约 r100/r102/r104/r96/r60 | 48 passed (48) | 0 |
| G3 | miniapp 全量 vitest | 240 passed / 1 skipped (241) files；2225 passed / 2 skipped (2227) tests | 0 |
| G4 | vue-tsc type-check | 通过 | 0 |
| G5 | build:h5 (uni build) | 通过 | 0 |
| G6 | git diff --check | 通过 | 0 |

gates-out.txt 尾部 G1..G6_RC=0 + GATES_DONE（ops_bash grep 复核 7741 行完整）。

## 6. 截图 sha256sum（F，落盘实物实测，UNIQUE 27/27）

| 页面 | 390 | 768 | 1280 |
|---|---|---|---|
| gems | 7c4af98c7d73c8169621781ab0bc1d638da827edbcc9eda610bc6d133cb45da7 | 10ab3c2359e0506b6f1b6e8d7f0c50f2821f52d0d42796b1dcf846938f52660d | 6992cf349ab2b83c01ef6537cbb55c8fdfcba1040d07620d09b62ce26e4be21c |
| skills | 4ab0f370dac1bbf45341127eadf031e0a8e848b96d8d9ca4188512d176e0c5df | 411839aad428c3a77091e40e8c72bd006458aa748fe24ee75470bd93e84c4ce9 | 4d843915058313abc13a54eb28fc213e1fb83b8999434cab33a503b4cabde480 |
| equipment | 2802c6c039ca9012eb6c55a175b18b17f2cf50db2dfe6ea022221a520175c2f6 | d217cc194a3c3d8e0890cfa8055abe689afe2718bc6aeab5e23cd6457c767494 | b718941a2716e08daa6295c008d3c6e08752577377de7057b6b228f17eea7e83 |
| bag | bd41b964353137f39005c8623812dfd71b9f7577f33d76e13ddd51873ee5ab95 | 0f410cf0ed9bfe9057772cb6688beaaca80529df3a79cbf3e6e6502a683c2b79 | d446cbab2dfb830f3245b9490ade9c4100655c0049cc57d9785019a894cb54e0 |
| me | 50dd0a17cb7565db681680a1ff39e210347f727c295c6846d1557e8f1f5482ea | e2b6b22481043431acc9c6a8edc8d29df246484de95ceca50e5bf17727696f23 | 32432ea644db8ab02bdcca1dd3a3c3630bf2b0d83fac9fdbfa3c6e58109ea11b |
| battles | 4c7644bb2608aa9d1d7b13dea6df151ec348390cfe397acfa0c3424889360871 | 6a8eebc22f18ccd0d2c34e5ef4c9d587a6ca04a647d096aefca1d95146adff1c | 4f92fe42b689fbdf0b9cc9e065e4edd4e10dc17275196f13af9d80cc55a37dde |
| enemies | 7ce191c41686e0a77a2ff89a1194373cb96f1244dfc713e3d24802be0a0d3fdf | 6edd94051ae3c077053353fa8ff5636284a659ea486795a74cbc7f5cf13ceb97 | 8d5052f5651c54eee8e14c72384a76f0ef4613805dd9ae40192603387e435796 |
| rank | 3a75403e51d68ea2a184b5afb093e0d747b189f25a3774489bacb7b2dda588dd | 8bbcac72c5f295e0bcc23e94bdec81dbee8fe8e8fcf38932c9f9f2a699a22352 | 4dbfa28049d30493e9441a427d2a77e48d4111af3ec150f73c2bebca63806506 |
| skins | fc100060621b464975e0c2c38668728e847003517b51d0b554fb53ac2302eed9 | c2bae8a98ac5c8e2e26e2330a50ef131d33a8f9a6d511754492f493be9ed6d49 | 857d551b4c6f405fa0f0463f6665a325ad3a4610017886629ee84688c4f17243 |

文件名格式 `r109-<page>-<W>.png`，存放 screenshots/。亲读 2 张非登录页：r109-gems-768（768×1024，宝石图鉴 AI 建议卡片「去商城找宝石」「去背包镶嵌」两枚 CTA）、r109-battles-1280（1280×800，我的战报战绩总账 1 场/30 击杀/3 漏怪/1200 均分），视觉与探针一致。

## 7. 诚实边界

1. 后端 8080 未起，全部响应由 init-script 合成；占位令牌 r109-qa-placeholder-token 仅 localStorage 内存注入，零真值。为使 10 处 CTA 自然渲染，mock 补全了 skills/enemies/skins/leaderboard/rank/battles 数据依赖，非真实后端返回。
2. 焦点环线宽 390/1280 为 1px、768 为 3px，系 3rpx 随视口 DPR 换算，颜色均 --accent 橙色；非线宽不一致缺陷。
3. click/Enter 同 handler 通过 stub 拦截 uni.navigateTo / pageScrollTo 记录 URL 与滚动参数验证，未做跳转后的 DOM 断言；自指分支 nav=[] 为页内定位。
4. AX snapshot 仅 gems 页三视口全量抓取；其余页以 DOM role/tabindex/tabIndexProp + focusin 焦点日志坐实。
5. 改前基线引用 docs/laiyipao/recon/ROUND109_A_PRIORITY_CTA_KEYBOARD_RECON.md（改前 role=null/tabIndex=null/tabIndexProp=-1/AX StaticText/真 Tab 跳过），本轮未重做改前 A/B。
6. 截图 27 张亲读 2 张；G3/G4 中 `[来一炮] build.attacker 缺失` 为测试既有 stderr 告警，非本轮引入。

## 8. 结论与 hygiene

- R109 10 处 AI 优先级 CTA 键盘可达性在三视口真 Chromium 全部坐实：role=button / tabindex=0 / 真 Tab 命中 / Enter 与 click 同 handler / 橙色焦点环；改前 StaticText 与跳过缺口全部修复。
- 不变量、令牌三锚点、零 --r109-、零横溢保持；门禁 G1-G6 全 RC=0；27 图 sha 实测 UNIQUE。
- hygiene：HEAD=652e237fbdf5ce38460f1f159e5b48a6ec9c7646 全程未变；git status 非 ?? 项仅 .gitignore + 10 个预期 vue（t314 实施）；产出全 UNTRACKED（data/ 由 .gitignore:683 拦，screenshots/ UNTRACKED）；auth 五路径 ABSENT；零真值；自启 H5 dev server 待结算前回收复验 P5174=000。

判定：PASS（含上述诚实边界）。
