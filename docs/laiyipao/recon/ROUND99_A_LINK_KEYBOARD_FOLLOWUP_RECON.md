# R99-A 可点 link 键盘可达补漏侦察报告（兑现 R98-A 待办）

- 任务：t247 / R99-A（只读侦察）
- attempt_id：d65acacb-ae7d-4452-80bd-c0fd9ade062b
- 基线 HEAD：05f2acf30950a377b681c09ae05d988f25eac866（clean，tracked diff=0/0）
- 工作目录：worktree dsh-d0bc4721-3248（分支 opt1000/laiyipao-ai-refactor）
- 视角：接续 R98-A 待办——可点 text.link 键盘可达性补漏；复核 R98 后是否已修复、真浏览器坐实、裁定期望落地子集
- 产物性质：本报告 UNTRACKED；未改任何业务源码/契约/规范/令牌，未写 git

## §0 结论

【推荐候选：7 处精确口径为主，12 处同型扩展建议同轮；禁用态 2 处单独排除】
R98 实施（三处 input 补 @confirm）后，R98-A 记录的 21 处可点 link 键盘三属性（role=button / tabindex=0 / @keydown.enter）**全部仍未修复**：静态全量 grep 命中 20 个可点 link 块（7 精确 + 13 复合），role+link 同现 0 处；真 Chromium 三视口 me/bag/mastery 页实测 link 元素 role=null、tabIndex=-1，AX 树无对应 button，干净 Tab 序列从不命中 link。四条件全中，与 R95/R96 完全同维度。改法沿用 R60/R95/R96 范式：在现有标签补 role="button" tabindex="0" @keydown.enter="同名handler"，复用 @click 的同一 handler，零 engine/数值/令牌改动。

## §1 R98 后 21 处清单逐处复核（全部未修复）

静态判据：ops_grep 全仓 pages/**/*.vue，pattern `<text[^>]*class="[^"]*link[^"]*"[^>]*@click`（multiline）命中 20 个可点块；`class="link..."[^>]*role="button"` 命中 0 处。7 处精确口径逐一读源码，三属性全缺，@click 与 R98-A 记录一致：

| # | 文件:行（现基线） | 标签/handler | R98-A 记录行 | 状态 |
|---|---|---|---|---|
| 1 | announcements/announcements.vue:98 | 清除搜索 / `annKeyword = ''` | 98 | 未修复 |
| 2 | bag/bag.vue:134 | 熔炼 / `doSynthesize(r)` | 134 | 未修复 |
| 3 | bag/bag.vue:217 | 装备 / `toggleEquip(s.id)` | 217 | 未修复 |
| 4 | bag/bag.vue:351 | 去商城买一颗 / `goShop` | 351 | 未修复 |
| 5 | bag/bag.vue:417 | 保存当前构筑 / `doSavePreset` | 416 | 未修复（+1 行漂移，上游 R98 插入 @confirm 行） |
| 6 | bag/bag.vue:421 | 点这里填名字 / `focusPreset` | 420 | 未修复（+1 行漂移） |
| 7 | me/me.vue:111 | 复制 / `copyUserId` | 111 | 未修复 |

扩展 14 处（复合 class）同样三属性全缺：me.vue:83 改名(startEditNickname)、448 验真(goVerify(b))、450 详情(toggleBattleDetail(b))；bag.vue:144 查看全部(showAllRecipes)、211 升级(doUpgrade(s.id))、375 佩戴(doEquipGem(g))、380 卸下(doUnequipGem(g))、385 洗练(doWash(g))、447 设为当前(doSetCurrent(p))、452 删除(doDeletePreset(p))、442 应用(doApplyPreset(p))；index/index.vue:188 空维补导(go(d.hint))；mastery.vue:66 重铸(resetMastery，禁用态)；shop.vue:129 购买(doBuy(it)，禁用态)。

纯文本无 @click 6 处（bag:142/213/366/422、mastery:67、shop:131）不是操作入口，不纳入。

## §2 真 Chromium 三视口坐实（agent-browser，H5 dev server 5174）

服务：pnpm run dev:h5 -- --port 5174（uni-app 实际端口 5174，curl=200；Go API 8080/PostgreSQL 未起，本地兜底数据生效）。会话范式：`export AGENT_BROWSER_SESSION=<name>` 同一 shell 链式；先 set viewport 再 open（反之 open 后被重置 about:blank，已两次复现并记录）。

- **me 页**（390/768/1280）：`.link`=改名、复制 2 个，三视口均 `{tag:UNI-TEXT, role:null, tabAttr:null, tabIndex:-1, cursor:auto}`；cw=sw=390/768/1280，hOverflow=false。AX（snapshot -i）：仅 button「去专精页 →」+ 3 个 generic clickable（设置账号密码/退出登录/改名区容器），改名/复制无 button 节点。干净 Tab 8 次：仅命中 r3-empty-action「去专精页 →」与 BODY，从不命中 link。
- **bag 页**（390/768/1280）：`.link`=去商城买一颗、保存当前构筑、点这里填名字 3 个，三视口均 role=null/tabIndex=-1/cursor:auto；cw=sw，零横溢。AX：button「去关卡页 →」+ textbox 预设名，link 无 button 节点。干净 Tab 8 次：r3-empty-action→INPUT→BODY 循环，从不命中 link。
- **mastery 页**（390/768/1280）：`.link`=重铸 1 个，三视口均 role=null/tabIndex=-1；cw=sw，零横溢。静态：:66-69 有 `:class="{ 'link-off': resetting || selected.length === 0 || resetTokens <= 0 }"` 禁用态（见 §5）。
- **shop 页**（390/768/1280）：三视口 `.link`=0、focusable=0，页面显示「加载失败：无法连接服务器: requestfail」「在售商品 0」——无后端兜底，shop.vue:129 购买 link 未渲染，**本轮不声称真浏览器坐实该页**，仅静态证据（与 R98-A announcements 页同型诚实边界）。
- 截图 12 张（4 页×3 视口）全部为非登录页（标题：来一炮/背包/专精/商城），亲读 4 张 390 主视图确认视觉上「保存当前构筑」「复制」「重铸」为橙色主色可点文字，与 DOM 不可达形成反差。sha256 见 §6。

## §3 四条件逐条裁定

1. ①真实可复现用户可见缺陷 —— **成立**。三视口 role=null/tabIndex=-1/AX 无 button/干净 Tab 不命中；键盘用户无法到达、无法激活这些视觉上明确可点的文字链接。
2. ②展示层小改 —— **成立**。每处仅在模板标签补三属性，复用 @click 同名 handler，零 engine/数值/逻辑改动。
3. ③权威口径明确 —— **成立**。R60 契约（r60A11yKeyboard.contract.test.ts:29-54）只扫已含 role="button" 的行，补 role 后这些行被纳入且三属性齐备；common.css 已有 .link 样式、[role="button"] cursor:pointer、[tabindex]:focus-visible（outline 3rpx），无需新增 CSS；令牌三锚点不动。
4. ④改动面可控 —— **成立（需界定范围）**。7 处精确口径为最小可控集；12 处同型扩展纯模板属性补齐；禁用态 2 处单独排除（§5）。

四条件全中 → 推荐候选，最小落地集为 7 处精确口径。

## §4 候选行号与改法（R60/R95/R96 同范式）

统一范式：在现有标签内按 `role="button"` → `@click="同名handler"` → `tabindex="0"` → `@keydown.enter="同名handler"` 补齐（与 R60 属性顺序一致）；@keydown.enter 必须复用 @click 的完全同一表达式。

**主候选（7 处精确口径，必改）**：
1. laiyipao/miniapp/src/pages/announcements/announcements.vue:98 —— `role="button" tabindex="0" @keydown.enter="annKeyword = ''"`
2. laiyipao/miniapp/src/pages/bag/bag.vue:134 —— `@keydown.enter="doSynthesize(r)"`
3. laiyipao/miniapp/src/pages/bag/bag.vue:217 —— `@keydown.enter="toggleEquip(s.id)"`
4. laiyipao/miniapp/src/pages/bag/bag.vue:351 —— `@keydown.enter="goShop"`
5. laiyipao/miniapp/src/pages/bag/bag.vue:417 —— `@keydown.enter="doSavePreset"`
6. laiyipao/miniapp/src/pages/bag/bag.vue:421 —— `@keydown.enter="focusPreset"`
7. laiyipao/miniapp/src/pages/me/me.vue:111 —— `@keydown.enter="copyUserId"`

**同型扩展（12 处，建议同轮纳入）**：me.vue:83 startEditNickname / 448 goVerify(b) / 450 toggleBattleDetail(b)；bag.vue:144 showAllRecipes（内联 `showAllRecipes = !showAllRecipes`）/ 211 doUpgrade(s.id) / 375 doEquipGem(g) / 380 doUnequipGem(g) / 385 doWash(g) / 442 doApplyPreset(p) / 447 doSetCurrent(p) / 452 doDeletePreset(p)；index/index.vue:188 go(d.hint)。多行标签（bag:211/375/380/385/442/447/452、index:188）在三属性补齐后须保持 R60 契约可按行扫描到 role+tabindex+@keydown 同行——建议把三属性写在含 role 的同一行，或按 R60 契约逐行校验。

## §5 排除 / 待裁定

- **禁用态 2 处单独排除，不纳入本轮最小候选**：mastery.vue:66 `link-off`（resetting/selected.length===0/resetTokens<=0 时视觉禁用）与 shop.vue:129 `is-disabled`（soldOut(it) 时视觉禁用）。两者 handler 内部已有守卫：mastery:370 `if (resetting.value || allocBusy.value) return`、shop:405 `if (soldOut(it)) return`，故即使键盘可达也不会误触发提交；但禁用态是否应保留在 Tab 序列（aria-disabled / tabindex 动态切换）属产品裁定，且 shop 页本轮无后端未真浏览器坐实——按 R98-A 口径单列，不在本轮候选。
- 纯文本 6 处（bag:142/213/366/422、mastery:67、shop:131）无 @click，不是操作入口，排除。
- 裸 view @click（非 link 类）属 R96-A 观察项，本视角不重复立案。

## §6 截图与 sha256（12 张，均非登录页）

审计目录：docs/laiyipao/audits/round99-a-link-keyboard/{data,screenshots}/；data 含 probe-link.js / ax-tab.js / active-el.js / driver3.sh / ax-tab.sh 与 driver3.log / ax-tab.log。

- r99a-me-390.png      dc150b2c358d27f4db4f980497677d66077d3686a7406a6cc10e45cf9033f02f
- r99a-me-768.png      5adc912769d2251fa3265dffe373bcdadc2f9529fb1740cafd372128eef88d3e
- r99a-me-1280.png     121d516b0182c21a62f435998623be97d7530724a095891d35a8ba1919273cc6
- r99a-bag-390.png     62b0f15773b75c30d5359b597c9d0e68ec66689df0ae93bff5fa397ac1a81104
- r99a-bag-768.png     9a3797392ee03627b23488442d769f3c7d761cd7d15b781f0aacbe08be827a3d
- r99a-bag-1280.png    b271cef3251d1cdfa774e0101983c4916b4ddb2374cd34b97389c9c4d6a57fd5
- r99a-mastery-390.png c905b4ccf7636d3fca4a3113fd789159fb29034d6d733de0863d98f478bf074f
- r99a-mastery-768.png 15f38621b3525e6d6fa1c64178b88ffd5adb3e80105aae8463503ef59759215a
- r99a-mastery-1280.png 22d3bf5e2c93cca9ee7854d224410fa4a5fd93bffeba1fd0f0687d758dd7af48
- r99a-shop-390.png    97d4f259c5dd80b904219df4563b5e13b4affb8ebe51d16d26649b66498d4f5e
- r99a-shop-768.png    d23b9d766fbbb42df88a4481ec2a17717541e39e75d8cedb64e3125eb9743295
- r99a-shop-1280.png   0e614279286f9a4becf3ff7e429c1a6b37e763b635cef174c5754c793bae5f44

注：r99a-bag-768.png 的 sha256 与 R98-A 的 bag-link-768.png 相同（9a3797…），系两轮均在同一 768 视口渲染同一空态背包页面、页面内容确定，属正常重现而非复用旧图；其余 11 张与 R98-A 图 distinct。

## §7 收尾核验

- HEAD：05f2acf30950a377b681c09ae05d988f25eac866（未变）
- git diff --check：RC=0；tracked diff=0（业务源码零改动）
- auth 四路径（laiyipao/deploy/.local.keys、laiyipao/deploy/remote-config.env、laiyipao/deploy/.env、根 .env）：ABSENT
- 未改业务源码/契约/规范/.gitignore/令牌；未写真值；未 git add/commit/push
- 自启 job：dev server bash-8314ba499b374f0a 收尾时 job_kill；浏览器会话进程清理完毕

## §8 诚实边界

- shop 页因无后端（requestfail）未渲染 link，三视口坐实仅限 me/bag/mastery；announcements「清除搜索」同 R98-A 空态未自然复现，仅静态证据。
- ax-tab.log 末尾 1 次 `Invalid response: EOF`（daemon 偶发），此前 7 次 Tab 序列已完整记录，不足部分以上述 DOM+AX+静态三重证据覆盖，不伪造。
- driver.sh / driver2.sh 两轮用 --session 参数导致 about:blank（已废止，保留为教训日志），有效证据仅来自 driver3.sh（env 会话）。
