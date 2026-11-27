# R99 规范：可点 text.link 键盘可达补漏（19 处三属性）

- 轮次：R99（test-first RED）
- 任务：t249 规范契约（写域仅本 SPEC + 契约 + RED 证据；不改任何业务 vue）
- 基线 HEAD：05f2acf30950a377b681c09ae05d988f25eac866（clean，tracked diff=0/0）
- 上游：t247 R99-A 侦察 docs/laiyipao/recon/ROUND99_A_LINK_KEYBOARD_FOLLOWUP_RECON.md（四条件全中；队长已读源码独立坐实并裁定立轮）

## §0 四条件裁定

1. **①真实可复现用户可见缺陷 —— 成立**。R99-A 真 Chromium 三视口（390/768/1280）坐实 me/bag/mastery 页可点 `text.link` 元素 `role=null`、`tabIndex=-1`、`cursor:auto`；AX（snapshot -i）无对应 button 节点；干净 Tab 8 次从不命中 link；视觉上 `common.css:.link` 是主色文字 + ≥56rpx 点击盒，明确可点却键盘不可达（WCAG 2.1.1）。
2. **②展示层小改 —— 成立**。每处仅在现有 `<text>` 开标签内补 `role="button" tabindex="0" @keydown.enter="同名handler"` 三属性，`@keydown.enter` 复用 `@click` 完全同一表达式；零 engine / 数值 / 逻辑改动。
3. **③权威口径明确 —— 成立**。R60 契约 `laiyipao/miniapp/src/styles/r60A11yKeyboard.contract.test.ts:29-54` 以 `role="button"` 行集为扫描集，补齐后 19 行自动纳入且三属性齐备；`common.css:289-297`（.link 主色 + 56rpx 盒）与全局 `[role="button"] cursor:pointer`、`[tabindex]):focus-visible`（outline 3rpx）已存在，无需新增 CSS；令牌三锚点不动。R60（role=button 四件套）、R95（me.vue:11 返回按钮）、R96（裸 view CTA）为本轮同范式前例。
4. **④改动面可控 —— 成立**。精确限定 19 处（7 精确口径 + 12 同型扩展），仅 4 个 `.vue` 文件；禁用态 2 处明确排除；21 处原始清单中纯文本 6 处无 `@click` 非入口，R96 裸 view 观察项不重复立案。

四条件全中 → 立轮。契约须在补属性前真 RED（19 个缺口断言失败），补后 GREEN（24/24）。

## §1 权威口径

- **R60 契约**：`r60A11yKeyboard.contract.test.ts:29-38` `roleButtonLines()` 只收 `text.includes('role="button"') && /<(text|view)\b/` 的行；:41-53 校验每行含 `tabindex="0"` 与 `@keydown`。无 role 的 link 是结构盲区（R98-A §1 铁证），本轮补齐即被纳入。
- **common.css .link 样式**：`common.css:281-297` 注释明确 .link 统一为「主色文字 + 足够点击盒（≥56rpx）」：`color: var(--accent)` / `padding: 8rpx 10rpx` / `min-height: 56rpx` / `display: inline-flex`。`.link-off`（:299-301）与 `.is-disabled` 为禁用态视觉。
- **令牌三锚点**（`styles/tokens.css:14-16`）：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`，一律不动，且业务源码与令牌文件零 `--r99-`。
- **属性顺序范式**（R60/R95/R96）：`role="button"` → `@click="handler"` → `tabindex="0"` → `@keydown.enter="handler"`。多行标签须保证三属性在同一行源码，便于 R60 契约逐行扫描。

## §2 精确改点表（19 处；行号取自当前基线，实施以 handler 定位为准）

| # | 口径 | 文件:行（标签起 / @click 行） | handler（@keydown.enter 与 @click 同一表达式） |
|---|---|---|---|
| 1 | 精确 | announcements/announcements.vue:98 | `annKeyword = ''` |
| 2 | 精确 | bag/bag.vue:134 | `doSynthesize(r)` |
| 3 | 精确 | bag/bag.vue:217 | `toggleEquip(s.id)` |
| 4 | 精确 | bag/bag.vue:351 | `goShop` |
| 5 | 精确 | bag/bag.vue:417 | `doSavePreset` |
| 6 | 精确 | bag/bag.vue:421 | `focusPreset` |
| 7 | 精确 | me/me.vue:111 | `copyUserId` |
| 8 | 扩展 | me/me.vue:83 | `startEditNickname` |
| 9 | 扩展 | me/me.vue:448 | `goVerify(b)` |
| 10 | 扩展 | me/me.vue:450 | `toggleBattleDetail(b)` |
| 11 | 扩展 | bag/bag.vue:140/144 | `showAllRecipes = !showAllRecipes` |
| 12 | 扩展 | bag/bag.vue:211/214 | `doUpgrade(s.id)` |
| 13 | 扩展 | bag/bag.vue:375/378 | `doEquipGem(g)` |
| 14 | 扩展 | bag/bag.vue:380/383 | `doUnequipGem(g)` |
| 15 | 扩展 | bag/bag.vue:385/388 | `doWash(g)` |
| 16 | 扩展 | bag/bag.vue:442 | `doApplyPreset(p)` |
| 17 | 扩展 | bag/bag.vue:443/447 | `doSetCurrent(p)` |
| 18 | 扩展 | bag/bag.vue:449/452 | `doDeletePreset(p)` |
| 19 | 扩展 | index/index.vue:185/188 | `go(d.hint)` |

> 注：bag:416/420（R98-A 记录）→ 现 417/421 为上游 R98 插入 `@confirm` 行导致的 +1 漂移，非键盘属性修复；契约以 `@click` 表达式稳健定位，不依赖行号。

**明确排除**：mastery.vue:66 `resetMastery`（`:class link-off`，handler 守卫 mastery:370 `if (resetting.value || allocBusy.value) return`）与 shop.vue:129 `doBuy(it)`（`:class is-disabled`，handler 守卫 shop:405 `if (soldOut(it)) return`）禁用态 2 处不在范围，须保持无 role（契约设防误改断言）。纯文本无 `@click` 6 处（bag:142/213/366/422、mastery:67、shop:131）与 R96 裸 view 观察项均不纳入。

## §3 不变量（契约须逐条断言）

1. 19 处原 `@click="handler"` 入口全部保留（不得用 @keydown 替换 @click）。
2. `@keydown.enter` 的表达式与 `@click` **完全同名同参**（含内联表达式 `annKeyword = ''` / `showAllRecipes = !showAllRecipes`）。
3. 令牌三锚点不动，令牌文件与 4 个业务 vue 零 `--r99-`。
4. 禁用态 2 处（mastery 重铸 / shop 购买）保持无 `role="button"`（防误改）。
5. `common.css` 的 `.link` 主色点击盒与全局 `[role="button"] cursor:pointer`、`[tabindex]):focus-visible` 锚点保留。

## §4 验证口径

- **契约**：`laiyipao/miniapp/src/styles/r99LinkKeyboard.contract.test.ts`，共 24 it = 19 缺口断言 + 5 不变量。
- **RED（本轮）**：`npx vitest run src/styles/r99LinkKeyboard.contract.test.ts --reporter=verbose`（在 `laiyipao/miniapp` 下）→ exit=1，Tests **19 failed | 5 passed (24)**；失败全部来自「须 role=button / tabindex=0 / @keydown.enter 同名 handler」，无解析/模块错误；5 不变量通过。
- **GREEN（实施轮）**：同一命令 exit=0，Tests **24 passed (24)**；并复跑 R60 `r60A11yKeyboard.contract.test.ts` 全绿（补齐后新 role 行须同样满足 R60 契约）。
- RED 原始逐字日志落 `docs/laiyipao/specs/round99-link-keyboard/RED-evidence.md`。

## §5 诚实边界

- shop 页因本地无 Go 后端（`requestfail`、在售商品 0）未渲染购买 link，R99-A 三视口未真浏览器坐实该页；禁用态排除基于静态源码与 handler 守卫，非真浏览器坐实。
- announcements 页「清除搜索」在空态（items=[]）下搜索框不渲染，未真浏览器自然复现，仅静态铁证（announcements.vue:96-98）。
- 本规范任务**不改业务 vue**；RED 是预期失败，非缺陷未收口；实施轮负责把 19 处补齐至 GREEN。
- 行号仅供定位参考；上游插行会造成漂移，契约以 `@click` 表达式稳健定位，不依赖绝对行号。
