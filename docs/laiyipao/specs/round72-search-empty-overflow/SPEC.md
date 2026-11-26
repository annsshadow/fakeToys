# R72 搜索空态极端值溢出修复 — 规范（SPEC）

- 轮次：R72 搜索空态极端值溢出修复（规范 + test-first RED）
- 执行者：same-route-lead（R41 规范设计）
- 候选来源：`docs/laiyipao/recon/ROUND72_B_EXTREME_CONTENT_RECON.md` 候选 R72-B-C1（四条件全中，队长已独立核验坐实）
- 性质：本轮只写规范与契约；不改业务源码、不写真值、不写 git。

## §0 背景与候选来源

侦察报告在真 Chromium（390/768/1280 三视口）中，对 skills / skins 两页搜索框注入 len=356 极端串（「没有匹配-A」×20 + emoji + HTML 片段 + 引号 + Z×200），6 组合全部 `hOverflow=true`（sw 最高 1920 vs cw=768），基线对照 `hOverflow=false`，`badTokens` 全空（HTML 被 Vue 插值转义，无 XSS）。根因：搜索框无 maxlength；空态关键词回显文本无宽度约束/省略范式，`.search-empty` 仅 `flex column + align-items:center`，子 `<text>` 由内容撑开。其余 5 页（announcements/enemies/equipment/gems/reactions）同构，静态一并纳入。

受影响 7 处空态关键词回显（逐行铁证，均为「没有匹配「{{ keyword }}」的XX」形态）：

- `laiyipao/miniapp/src/pages/skills/skills.vue:221` `<text class="muted">没有匹配「{{ keyword }}」的技能</text>`
- `laiyipao/miniapp/src/pages/skins/skins.vue:131` `…的皮肤`
- `laiyipao/miniapp/src/pages/announcements/announcements.vue:96-99` `…的公告`（结构异：外层 `<text class="muted">` 内含 `<text class="link">清除搜索</text>`，无 `.search-empty` 容器，关键词变量为 `annKeyword`）
- `laiyipao/miniapp/src/pages/enemies/enemies.vue:161` `…的敌人`
- `laiyipao/miniapp/src/pages/equipment/equipment.vue:219` `…的装备`
- `laiyipao/miniapp/src/pages/gems/gems.vue:155` `…的宝石`
- `laiyipao/miniapp/src/pages/reactions/reactions.vue:254` `…的反应`

## §1 权威范式与口径决策

- 权威省略范式：`laiyipao/miniapp/src/styles/common.css:184-189` `.ellipsis{overflow:hidden;text-overflow:ellipsis;white-space:nowrap;min-width:0}`；`laiyipao/miniapp/src/pages/announcements/announcements.vue:274-282` `.ann-title{font-size:28rpx;font-weight:600;flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}`。
- 权威换行范式：`laiyipao/miniapp/src/pages/verify/verify.vue:696` `word-break:break-all`（保留完整内容可见性）。
- 通用 `<text>` 块级范式：`common.css:191` 注释「uni 的 `<text>` 默认 inline，每处都要手写 display:block —— 给个名字」，故新类须含 `display:block`。
- 决策：采用 **`word-break:break-all`** 而非 ellipsis。理由：① 极端串含无空格 Z×200，ellipsis 只截尾会丢失关键词信息，玩家需看到完整关键词以修正输入；② `.search-empty` 为 flex column，子项宽度由内容撑开，直接 ellipsis 在 flex 子项上需父级 `min-width:0` 才生效，而 `word-break:break-all` + `max-width:100%` 可直接在子项收敛。实施轮须真浏览器三视口复验 `hOverflow=false`。
- 新专用类（拟名 `.search-empty-text`）：`display:block; max-width:100%; word-break:break-all;`。不新增视觉令牌，不引入 `--r72-` 业务令牌（全仓零命中）。

## §2 精确改点（实施轮，本轮不改）

1. `common.css`：在 `.search-empty`（:352-358）与 `.clear-search`（:360-366）之间新增 `.search-empty-text{display:block;max-width:100%;word-break:break-all;}`。
2. 7 页空态关键词回显 `<text>` 追加 `search-empty-text` class（保留 `muted`）：
   - skills.vue:221 / skins.vue:131 / enemies.vue:161 / equipment.vue:219 / gems.vue:155 / reactions.vue:254：`<text class="muted search-empty-text">没有匹配「{{ keyword }}」的…</text>`
   - announcements.vue:96：外层 `<text class="muted search-empty-text">`，保留其内 `<text class="link" @click="annKeyword = 空串">清除搜索</text>` 原样。
3. 不改全局 `.muted`（common.css:158-161 / 826 / 1018）、不改 `.search-empty`、不改 `.ellipsis`、不改 `.clear-search`、不改任何搜索框（不加 maxlength，避免改变交互契约）。

## §3 不变量（实施轮不得回退）

- 7 页 `v-if` 空态条件原样：`keyword && <list>.length && <filtered>.length === 0`（announcements 为 `annKeyword && filteredItems.length === 0`）。
- 清除搜索 CTA 原样：6 页 `<text class="clear-search">清除搜索</text>`（带 @click 清空 keyword）；announcements `<text class="link">清除搜索</text>`（带 @click 清空 annKeyword）。
- `{{ keyword }}` / `{{ annKeyword }}` 回显原样，不得删除或替换关键词。
- 权威范式原样：`.muted{color:var(--muted);font-size:var(--fs-sm)}`、`.search-empty{display:flex;flex-direction:column;align-items:center;gap:16rpx;padding:48rpx 0 32rpx}`、`.ellipsis{overflow:hidden;text-overflow:ellipsis;white-space:nowrap;min-width:0}`。
- 令牌三锚点原样：`laiyipao/miniapp/src/styles/tokens.css:14` `--lyp-token-version: lyp-tokens-v1`、:15 `--lyp-r3-version: lyp-tokens-v3-r3`、:16 `--lyp-r6-version: lyp-tokens-r6`。
- 不回退历史轮次：R60 键盘 role/tabindex、R61 对比浮层、R55 图鉴、R66 缓存守卫、R69 昵称 busy、R71 记录时间本地化（各契约测试仍全绿）。
- 不碰 protected 域：engine / replay / hash / score / settle / loot / tokens 与后端；不改 `.gitignore`；不写真值；不写 git。

## §4 契约清单（`laiyipao/miniapp/src/styles/r72SearchEmpty.contract.test.ts`，5 it）

- it1：common.css 存在 `.search-empty-text` 规则块，含 `display:block`、`max-width:100%`、`word-break:break-all`（或等价 overflow 处理）。
- it2：7 页空态关键词回显 `<text>` 均含 `search-empty-text` class（逐路径真实读源码断言）。
- it3：7 处空态结构保留（`v-if` 条件、清除搜索 CTA、`{{ keyword }}`/`{{ annKeyword }}` 回显）。
- it4：不变量原样（`.muted` / `.search-empty` / `.ellipsis` 权威范式、令牌三锚点 tokens.css:14/15/16）。
- it5：业务源码零 `--r72-` 令牌（tokens.css 与 7 页 + common.css 扫描），写域边界（本轮仅契约+SPEC 为未跟踪新增，不触碰业务源码）。

契约范式：真实 `node:fs.readFileSync(join(__dirname,".."),"utf-8")` 读源码，禁内嵌实现副本；通用 `blockAfter(text,keyword)` 大括号配对提取 CSS 规则块。

## §5 RED 预期

`pnpm exec vitest run src/styles/r72SearchEmpty.contract.test.ts --reporter=verbose`：exit=1，预期 2 failed | 3 passed。失败均为缺口断言：it1（`.search-empty-text` 类尚不存在）、it2（7 处 `<text>` 尚未加 class）；it3/it4/it5 通过（结构/不变量/令牌现状正确）。无解析/模块/超时错误。

## §6 验证命令（实施轮/终审）

- `cd laiyipao/miniapp && pnpm exec vitest run src/styles/r72SearchEmpty.contract.test.ts --reporter=verbose`
- `cd laiyipao/miniapp && pnpm exec vitest run --reporter=verbose`
- `cd laiyipao/miniapp && pnpm exec vue-tsc --noEmit`
- `cd laiyipao/miniapp && pnpm run build:h5`
- `git diff --check`
- 真浏览器三视口复验：注入 len=356 极端串后 `hOverflow=false`（实施轮必做）。

## §7 诚实边界

- 本轮只写规范+契约，未实施修复、未跑全门禁、未真浏览器复验修复效果；`word-break:break-all` 在三视口的实际消除效果须实施轮坐实，若不满足则回退到 `max-width:100% + ellipsis 四件套` 并真浏览器复验。
- 7 页中仅 skills/skins 已在侦察轮真浏览器坐实，其余 5 页为同构静态纳入；announcements 结构异（无 `.search-empty` 容器），实施轮须单独真浏览器复验。
- 搜索框无 maxlength 属交互契约，本规范刻意不加（仅修回显展示层）；极端值下 CTA 仍居中可点（侦察轮截图坐实）。
- rank 昵称、index/me 昵称、超长数字为侦察轮未坐实疑点，不属本候选，不在本规范范围。
