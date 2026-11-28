# R110 搜索空态「清除搜索」键盘可达契约 SPEC

- 任务：t320 / attempt_id=5abc04e9-a5eb-4366-84d8-953f7ed6b40c
- 基线 HEAD：`f11a1eae323d834433e6feaa749a609e94d95b79`（全程未变）
- 性质：test-first RED→GREEN 规范轮；本轮仅写规范/契约/证据，不改业务源码

## §0 裁定来源

- B 路侦察 docs/laiyipao/recon/ROUND110_B_CLEAR_SEARCH_RECON.md O1：六页搜索空态「清除搜索」裸 text 缺键盘三件套。队长独立裁定推翻侦察员「因禁重复不立轮」的自我设限——按 R109 已发布先例（跨页补齐未覆盖元素的键盘可达）立轮。
- A 路 docs/laiyipao/recon/ROUND110_A_VISUAL_LAYOUT_RECON.md 不立轮（CLS 命题不成立），与本轮不同维度，无冲突。

## §1 缺口（6 处）

均为裸元素 <text class="clear-search" @click="keyword = ''">清除搜索</text>，缺 role="button" / tabindex="0" / @keydown.enter：

- pages/enemies/enemies.vue:162
- pages/equipment/equipment.vue:220
- pages/gems/gems.vue:156
- pages/reactions/reactions.vue:255
- pages/skills/skills.vue:227
- pages/skins/skins.vue:132

视觉上 common.css:366-372 .clear-search 为 fire 色描边 pill + :1076/:1103 :active 反馈，明确可点；但键盘 Tab 不可达（WCAG 2.1.1）。

## §2 唯一改点（GREEN 实施口径）

每处补三属性，@keydown.enter handler 与 @click 逐字一致（keyword = ''）：

<text class="clear-search" role="button" tabindex="0" @click="keyword = ''" @keydown.enter="keyword = ''">清除搜索</text>

- 强对照：pages/announcements/announcements.vue:98 同类 .link 已带完整三件套（R99_A 所修），作不变量抽查点。
- 不改 class="clear-search" 视觉样式，不动 @click handler 语义。

## §3 权威口径

- docs/laiyipao/design/AI_VISUAL_SYSTEM.md §1A.7（:135-138）：CTA 键盘可达且焦点可见（WCAG 2.4.7）。
- WCAG 2.1.1 Keyboard：所有功能可经键盘操作。
- 同仓范式：R60/R96/R99/R100/R102/R104/R109 全部采用 role="button" + tabindex="0" + @keydown.enter 同 handler 三件套。

## §4 不变量契约（6 项，RED 应全通过）

1. announcements.vue:98 已修 .link 清除搜索三件套仍在（R99_A）。
2. r72SearchEmpty 空态结构保留（关键词回显 + 清除搜索 CTA + 「没有匹配」回显）。
3. 令牌三锚点 tokens.css:14-16 未漂移 + 7 页零 --r110-。
4. R99/R100/R102/R104/R109 既有三件套抽查无回退（含 battle.vue:77 timeline CTA、bag r12 CTA、index retry、me goBack 等 8 处）。
5. 6 处 @click handler 与 class="clear-search" 原样保留（双锚点唯一定位）。
6. common.css .clear-search 视觉样式未动（fire 描边 + pill 圆角）。

## §5 契约与验证

- 契约：laiyipao/miniapp/src/styles/r110ClearSearchKeyboard.contract.test.ts（12 it = 6 缺口 + 6 不变量）。
- 定位：真实 fs.readFileSync 读 6+1 vue + common.css + tokens.css；以 class="clear-search" + @click="keyword = ''" 双锚点定位开标签，不依赖行号。
- RED 命令：cd laiyipao/miniapp && npx vitest run src/styles/r110ClearSearchKeyboard.contract.test.ts --reporter=verbose（勿指定 vite.config.ts）。
- RED 预期：exit=1，6 failed（全为 role=button 缺口）| 6 passed（不变量），无 PARSE_ERROR。
- GREEN 后复跑同一命令应 12/12 pass。

## §6 Hygiene 边界

- 写域仅限本 SPEC / RED-evidence / RED-raw.log / 契约文件，禁改业务 vue / .gitignore / 令牌 / 既有契约。
- 零真值，产出全 UNTRACKED，自启 job 全回收。
