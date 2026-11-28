# R110 清除搜索键盘可达 · 真浏览器 QA 报告

轮次：R110（六页搜索空态「清除搜索」裸 text 补键盘三件套）
任务：t322 / attempt 3de89f9f-9768-4909-898b-4fdd665890ed
验证者：same-route-qa
日期：2026-11-28
HEAD 基线：`f11a1eae323d834433e6feaa749a609e94d95b79`（全程未变）

## 1. 范围与方法

- **目标**：三视口真 Chromium 验证六页（enemies / equipment / gems / reactions / skills / skins）搜索空态的「清除搜索」CTA 补齐 role=button / tabindex=0 / @keydown.enter 三件套，恢复键盘可达与焦点可见。
- **改点**：六处 `<text class="clear-search" role="button" tabindex="0" @click="keyword = ''" @keydown.enter="keyword = ''">清除搜索</text>` 全落地（enemies.vue:162 / equipment.vue:220 / gems.vue:156 / reactions.vue:255 / skills.vue:227 / skins.vue:132）。
- **强对照**：announcements.vue:98 同维度已修 `.link` 三件套（R99_A）保留；权威 AI_VISUAL_SYSTEM §1A.7（:135-138）CTA 键盘可达 + 焦点可见。
- **改前基线**：docs/laiyipao/recon/ROUND110_B_CLEAR_SEARCH_RECON.md（t，104 行）——改前 clear-search 为裸 text，role=null / tabindex=null / AX 为 StaticText / 真 Tab 跳过；本轮引用该 recon，未重做 A/B 对照。
- **方法**：后端 8080 未起，以 init-script（r110-init.js）合成配置与列表数据；patch XHR.open/send 直接合成响应（setTimeout 30ms 触发 onreadystatechange/onload）+ patch fetch + localStorage 注入配置缓存与占位令牌。输入不匹配关键词 `zzzz_no_match` 触发搜索空态。三视口 390×844 / 768×1024 / 1280×800 逐页坐实 DOM 属性、AX 语义、真 Tab 序列、Enter 清空 DOM 闭环、焦点环与零横溢。
- **契约**：laiyipao/miniapp/src/styles/r110ClearSearchKeyboard.contract.test.ts（120 行 / 12 it = 6 缺口 + 6 不变量）。

## 2. A：六页 clear-search 三属性 + AX button（三视口 × 六页全通过）

每视口每页触发空态后，AX 快照均出现 `- button "清除搜索" [ref=eN]`（改前为 StaticText）。DOM 探针 `probe.js` 坐实 `role=button / tabindex=0 / tabIndexProp=0 / vis=true`，标签 `UNI-TEXT`。

| 页 | 390 | 768 | 1280 | AX ref（390/768/1280） |
|---|---|---|---|---|
| enemies | PASS | PASS | PASS | e2 / e2 / e2 |
| equipment | PASS | PASS | PASS | e3 / e3 / e3 |
| gems | PASS | PASS | PASS | e4 / e4 / e4 |
| reactions | PASS | PASS | PASS | e4 / e4 / e4 |
| skills | PASS | PASS | PASS | e3 / e3 / e3 |
| skins | PASS | PASS | PASS | e2 / e2 / e2 |

空态文案逐页坐实：「没有匹配「zzzz_no_match」的敌人 / 装备 / 宝石 / 反应 / 技能 / 皮肤」，与 `.search-empty` v-if 结构一致（R72 空态结构不回退：关键词回显 + 清除 CTA）。

## 3. B：真 Tab 命中 + Enter 清空 DOM 闭环

**真 Tab**（`agent-browser press Tab`，非 JS focus）：enemies 页 TAB1 r3-ai-cta → TAB2 uni-input-input → **TAB3 clear-search**；gems 页 TAB1 r12-priority-cta → TAB2/TAB3 r3-ai-cta → TAB4 input → **TAB5 clear-search**。三视口 390/768/1280 一致命中，位于焦点序列中（改前整段跳过）。

**Enter 清空 DOM 闭环**（compact 探针 ROWS/CLEAR/INP/EMPTY，三视口 × 六页 18 组全 PASS）：

| 页 | BEFORE | 空态 AFTER | Enter 后 RESTORE |
|---|---|---|---|
| enemies | ROWS=1 CLEAR=0 EMPTY=0 | ROWS=0 CLEAR=1 EMPTY=1 | ROWS=1 CLEAR=0 EMPTY=0 |
| equipment | ROWS=1 CLEAR=0 EMPTY=0 | ROWS=0 CLEAR=1 EMPTY=1 | ROWS=1 CLEAR=0 EMPTY=0 |
| gems | ROWS=1 CLEAR=0 EMPTY=0 | ROWS=0 CLEAR=1 EMPTY=1 | ROWS=1 CLEAR=0 EMPTY=0 |
| reactions | ROWS=1 CLEAR=0 EMPTY=0 | ROWS=0 CLEAR=1 EMPTY=1 | ROWS=1 CLEAR=0 EMPTY=0 |
| skills | ROWS=2 CLEAR=0 EMPTY=0 | ROWS=0 CLEAR=1 EMPTY=1 | ROWS=2 CLEAR=0 EMPTY=0 |
| skins | ROWS=3 CLEAR=0 EMPTY=0 | ROWS=0 CLEAR=1 EMPTY=1 | ROWS=3 CLEAR=0 EMPTY=0 |

结论：Enter 与 @click 同为 `keyword = ''`，清空后条目数恢复（>0）、清空 CTA 从 DOM 消失。三视口逐页一致。

## 4. C：橙焦点环 + 令牌三锚点

- **焦点环**（focusin 监听读 computed style，三视口 × 六页全 `active=true`）：390 / 1280 = `solid 1px rgb(255, 107, 53)` offset 1px；768 = `solid 3px rgb(255, 107, 53)` offset 2px。焦点环 3rpx 随视口换算缩放（390/1280 为 1px，768 为 3px），颜色恒为橙色 `--accent`。
- **令牌三锚点**（tokens.css:14/15/16）在：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`。
- **零新令牌**：业务源码全局检索 `--r110-` 无命中（NO_MATCH=PASS；仅契约自身断言中出现）。

## 5. D：不变量保持

- announcements.vue:98 `.link` 三件套保留（role=button + tabindex=0 + @keydown.enter）。
- R72 搜索空态结构（关键词回显 + 清除 CTA）未改。
- 六页既有键盘三件套未回退：走查焦点序列同时命中 r3-ai-cta / r12-priority-cta / uni-input-input 等既有可达控件（见 §3 真 Tab 序列）。
- **零横溢**：三视口 × 六页全部 `cw=sw`、`hOverflow=false`（见 §3 compact 探针 CW/SW/HOVF 字段）。
- 非登录页：全部 `isLogin=false`。

## 6. E：门禁（全绿，独立复跑 job bash-2f7a6e0320eb4ede exit0）

| 项 | 命令 | 结果 |
|---|---|---|
| G1 | r110ClearSearchKeyboard.contract.test.ts | 12 passed / RC0 |
| G2 | 六回归契约（r72/r99/r60/r100/r104/r109） | 76 passed / RC0 |
| G3 | miniapp 全量 vitest | 241 passed | 1 skipped files / 2237 passed | 2 skipped tests / RC0 |
| G4 | pnpm run type-check（vue-tsc） | RC0 |
| G5 | pnpm run build:h5 | RC0 |
| G6 | git diff --check | RC0 |
| — | GATES_DONE | 全部 RC0 |

G3 中 `[来一炮] build.attacker 缺失` 为标准输出告警，测试既有，非本轮引入。

## 7. F：截图与逐张 sha256sum（落盘实物实测）

18 张（6 页 × 3 视口，空态 + 清除 CTA 焦点环），命名 `r110-<page>-<W>.png`。sha256sum 在 screenshots/ 目录实测，UNIQUE 18/18，与下表逐字一致。

| 页面 | 390 | 768 | 1280 |
|---|---|---|---|
| enemies | 7aa72223a14cedb6b6fa06eda2dd982b8eca3aa71912f36b86530c0a10733261 | 8f554435af5d5880c3200b414c7ed63de1155c8deaee15b2bc4a3877a31d3b88 | 06ebcfaac1a24805fdca1fb0a0246efad5cdcbeea88cab765d484708708da763 |
| equipment | 1c83a5bdfd852bd51f64dfc3ab62b35bd67119900d066a83a478b63173d5fb5e | 7125ef331f7aa1e88f9bb648de56ec2672081662cadc79f19b27fc4bc850ccba | 98623b0a0551b8a9a43d8ddecd3c188a5cf40da7c6c8402183cc8e4422bf24ad |
| gems | f01b10caaeb74bb4b0ff657d461b11a4eab09aa0db96e42c3aa7dc3b36a94359 | 5647c47ed8d68ddc13d234c9c7fe197b32158833b812818110a5a96d2fa7b48a | 6e84764ced0578177b71845cef74785672d8809d2f44931b783d514fa3f4aa47 |
| reactions | 32f47b7fa8b77e5093966ce44d5a25a93af5f0a4a7ef9d110086b45836f52e35 | 497efd53b1e2c10c8a4c81f0ab6c66c7f58216d371880682610d6d622fc5c7fa | c60dc0b603a44a5157726d758a56c0d9117e6cba3e5e842e030b9496611c0c25 |
| skills | 41e56a2118af3e6284551dea02ab736f248d3a4607e86ea941c46072d8780c3e | 75ba65c9cb614a7809469a1e1eea52b30bd1e5a7adc5e5eab948ecfc7b7b849d | 1ed85370fb20b734a07d7efb9ccb4ecd4892accd791f2e4cad91407da4270d2d |
| skins | e87ac80a200769d9ebd0e243e5a470ff9c31e53fae29f22d5eda1c224df6fdc2 | 50c8f4d27bd5a7a16c4d5931f6f1281f8db4b8631b0e7b6a984e873ccd7bd275 | eec9e7db139aa52a27c36babf48c436f2e0b2d3beaa7eb44cc43f8c460d07615 |

**亲读 2 张非登录页**（视觉与探针一致）：
- `r110-enemies-768.png`（768×1024）：敌人图鉴，克制排行与血量分布图，「显示 0 / 共 1 条」，下方「没有匹配「zzzz_no_match」的敌人」+ 橙框「清除搜索」（焦点环）。
- `r110-gems-1280.png`（1280×800）：宝石图鉴，搜索框内 `zzzz_no_match`、AI 收益面板，「显示 0 / 共 1 条」，下方「没有匹配「zzzz_no_match」的宝石」+ 橙框「清除搜索」。

## 8. 诚实边界 /  Hygiene

**诚实边界**
- 后端 8080 未起：全部数据经 init-script 合成（配置缓存 + 列表 + 占位令牌 `r110-qa-placeholder-token`，仅 localStorage 内存注入，零真值）。
- 搜索词 `zzzz_no_match` 由 input 事件注入；gems 页首轮因 mock 的 gem 缺 `attr` 字段致 filtered computed 抛错、整页渲染崩溃，补 `attr/code/descr` 后恢复正常——此为 mock 数据缺陷，非实现缺陷，已修正后坐实。
- Enter 用合成 KeyboardEvent（与 @keydown.enter 同语义）；真 Tab 用 `agent-browser press Tab` 坐实；焦点环用 focusin 监听读 computed style。未做跳转后 DOM 断言（清空为页内状态变更，已用 DOM 闭环断言覆盖）。
- 改前基线引用 t B-recon，未重做旧代码 A/B 对照。
- 截图亲读 2/18。

**Hygiene**
- HEAD = `f11a1eae323d834433e6feaa749a609e94d95b79` 全程未变。
- git status 非 ?? 项仅 `.gitignore` + 六页 vue（enemies/equipment/gems/reactions/skills/skins，全为预期实施未提交）；QA 产出 data/（.gitignore:688 拦）与 screenshots/ 均 UNTRACKED。
- auth 五路径全 ABSENT（`.env` / `laiyipao/admin/.env.local.keys` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.local.keys` / `laiyipao/miniapp/.env.local`）。
- 零真值；占位令牌仅内存注入。
- QA 库本轮未连。
- 自启 H5 dev server job 于收尾 kill 并复验 P5174=000 / P8080=000。

## 9. 结论

三视口 × 六页真 Chromium 全部通过：六处 clear-search 补齐 role=button / tabindex=0 / @keydown.enter，AX 由 StaticText 变为 button；真 Tab 命中焦点序列、Enter 清空并恢复列表（DOM 闭环）；橙焦点环可见（3rpx 随视口缩放）；令牌三锚点、零新令牌、既有不变量与零横溢保持。门禁 G1–G6 全 RC0。18 图 sha 与实物逐字一致，亲读 2 张非登录页视觉吻合。**判定 PASS**（边界如上诚实披露）。
