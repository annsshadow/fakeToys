# R98-A 内联文字链接键盘可达性侦察报告

- 任务：t240 / R98-A（只读侦察）
- 基线 HEAD：358901febaaf92ff5543e4f20f379ece52c4ea21
- 工作目录：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
- 视角：小程序 H5 内联文字链接（text.link / 复合 link 类）键盘可达性；R60（带 role 的 48 处）、R95（me 返回按钮）、R96（6 页 r3-empty-action CTA）之后的新盲区维度
- 产物性质：本报告 UNTRACKED；未改任何业务源码/契约/规范/令牌，未写 git

## §0 判定与依据

【结论：推荐候选 1 个】可点 text.link 键盘三属性补漏。
队长给定的 7 处精确 class="link" + @click 元素，role/tabindex/@keydown.enter 三属性全缺，与 R95/R96 完全同维度（R60 契约只扫已带 role=button 的元素，压根无 role 的链接是结构盲区）。真浏览器 me/bag 两页坐实：DOM role=null、tabIndex=-1、AX 树无 button、干净 Tab 序列不命中、注入 tabindex 后 focus 可达但 Enter 不触发 handler。四条件全中，详见 §3。
扩大扫描（含复合类如 class="dim link" / "link preset-apply"）后，实际可点 link 元素共 21 处（去重），全部三属性全缺；其中 7 处为队长精确口径（必改），另 14 处为同型盲区（建议同轮纳入，禁用态 2 处需单独裁定，见 §5）。

## §1 静态全量扫描

精确口径（class="link" 全仓 13 处，其中带 @click 的 7 处，全部零键盘属性）：
1. announcements/announcements.vue:98 `<text class="link" @click="annKeyword = ''">清除搜索</text>`（handler=内联赋值 annKeyword = ''）
2. bag/bag.vue:134 `<text v-else class="link" @click="doSynthesize(r)">熔炼</text>`
3. bag/bag.vue:217 `<text v-else class="link" @click="toggleEquip(s.id)">装备</text>`
4. bag/bag.vue:351 `<text class="link" @click="goShop">去商城买一颗 →</text>`
5. bag/bag.vue:416 `<text v-else class="link" @click="doSavePreset">保存当前构筑</text>`
6. bag/bag.vue:420 `<text class="link" @click="focusPreset">点这里填名字保存一套</text>`
7. me/me.vue:111 `<text class="link" style="font-size: 22rpx" @click="copyUserId">复制</text>`

纯文本无 click 的 6 处（非候选）：bag:142/213/366/421、mastery:67、shop:131。
权威样式口径：common.css:281-297 注释明确 .link 曾三份冲突定义，已统一为「主色文字 + 足够点击盒（>=56rpx）」，当前 .link 含 color:var(--accent)、padding:8rpx 10rpx、min-height:56rpx、display:inline-flex；common.css:468-470 与 863-866 有 .link:active 反馈；R60 后 common.css 已有 [role="button"] cursor:pointer 与 [tabindex]:focus-visible（outline 3rpx）。视觉上明确可点，与 DOM 不可达形成反差。
历史契约盲区铁证：r60A11yKeyboard.contract.test.ts:29-37 roleButtonLines() 只收 text.includes('role="button"') 的行；41-53 两项 it 仅校验这些行含 tabindex/@keydown。六处无 role 的 link 根本不进入扫描集 —— 与 R96 裸 view CTA 的盲区机制逐字同型。

## §2 真浏览器坐实（Chromium / agent-browser 0.41.1，H5 dev server 5174）

服务：pnpm run dev:h5 -- --port 5174（jobId=bash-70e1957a4ddb410e，uni-app 改写 http://127.0.0.1:5174/，curl=200）。Go API 8080 / PostgreSQL 未起，本地兜底数据生效。

### 2.1 me 页（390/768/1280 三视口）
- DOM 探针（r98-me-390，viewport 390）：`.link` 元素 2 个 —— {tag:UNI-TEXT,text:"改名",role:null,tab:null,tabIndex:-1,cursor:"auto"} 与 {tag:UNI-TEXT,text:"复制",role:null,tab:null,tabIndex:-1,cursor:"auto"}；body 含"复制"；cw=sw=390（无横向溢出）。
- AX 树（snapshot -i，三视口一致）：仅 3 个 generic clickable（e1 / "设置账号密码" / "退出登录"），无任何 button 节点 —— "改名""复制"在可访问性树中不是 button。
- 干净 Tab 序列（12 次）：activeElement 全程 BODY，从未命中 link。
- Enter 对照（r98-enter，独立 session）：注入 tabindex="0" 后 eval 返回 {ok:true,active:true,tab:"0"}（证明 focus 可达）；press Enter 后 {nickInput:false,editing:false} 与注入前完全一致 —— 改名 handler 未被 Enter 触发。click 对照组（find text 改名 click）因元素不在视口报 not visible，未成功；故 Enter 无效主要由静态无 @keydown.enter + press Enter 无状态变化共同坐实，click 对照失败如实披露，不伪造。

### 2.2 bag 页（390/768/1280 三视口）
- DOM 探针：三视口均 {tag:UNI-TEXT,role:null,tab:null,tabIndex:-1,cursor:"auto"}；"保存当前构筑" 390 w=85/h=29、768 w=168/h=57、1280 w=82/h=28；"点这里填名字保存一套" w=135/266/130；"去商城买一颗" w=0（当前无宝石分支未渲染可见盒，静态存在）。三视口 cw=sw，hOverflow=false。
- AX 树（snapshot -i，三视口一致）：generic clickable + button "去关卡页 →"（该 button 是已有三属性的 r3-empty-action，非本次 link），link 元素无 button 节点。
- 干净 Tab 序列（r98-bagClean，未注入）：T1/T4/T7 = r3-empty-action "去关卡页 →"，T2/T5/T8 = INPUT uni-input-input，T3/T6 = BODY —— 8 次均未命中任何 link。
- 注入对照（r98-bagEnter session 曾给"保存当前构筑"注入 tabindex=0）：Tab T4 命中 UNI-TEXT.link "保存当前构筑"（isLink=true）；证明缺口纯粹是三属性缺失，补上即可进入 Tab 序列。该 session 因注入被污染，仅作机制对照，不作为"当前状态可达"证据。

### 2.3 announcements 页（未坐实，如实披露）
- 无后端时 api.fetchAnnouncements 失败→items=[]→模板走"暂无公告"空态卡（announcements.vue:61-73），搜索框 input.search-inp 不渲染（在 v-else 分支 :75 内），故"清除搜索" link 未在真浏览器自然复现；fill input.search-inp 报 Element not found。静态源码铁证（:96-98）成立，但本轮不声称真浏览器坐实该页。

### 2.4 截图与亲读
- 截图 6 张已采集并算 sha256（见 §6）。本轮亲读像素数 0 张 —— 证据以 DOM 属性/AX 树/Tab/Enter 实测为主，截图仅作视觉旁证，未逐张肉眼复核，不虚报亲读。

## §3 四条件逐条裁定

1. ①真实可复现用户可见缺陷 —— 成立。me/bag 两页三视口实测 role=null/tabIndex=-1/AX 无 button/干净 Tab 不命中；me 页注入 tabindex 后 Enter 仍不触发 handler。键盘用户无法到达、无法激活这些视觉上明确可点的链接。
2. ②展示层小改 —— 成立。每处仅在模板标签补 role="button" tabindex="0" @keydown.enter="同名handler" 三属性，与 R95 me.vue:11、R96 六处 CTA 逐字同构；零 engine/数值/令牌/逻辑改动。
3. ③不碰语义/令牌/测试不变量 —— 成立。R60 契约只扫已有 role=button 的行，补齐后这些行被纳入且三属性齐备（不会破坏契约）；common.css 已有 .link 样式与 [role="button"] cursor:pointer、[tabindex]:focus-visible，无需新增 CSS；令牌三锚点（tokens.css:14 --lyp-token-version: lyp-tokens-v1 / :15 --lyp-r3-version: lyp-tokens-v3-r3 / :16 --lyp-r6-version: lyp-tokens-r6）不动。
4. ④改动面可控 —— 成立（需限定范围）。7 处精确口径为最小可控集；扩大后 21 处同型盲区仍属纯模板属性补齐，但 mastery:66 的 link-off 与 shop:129 的 is-disabled 有禁用态，键盘可达后需防止禁用态误触发，建议单独裁定（见 §5）。

四条件全中 → 推荐候选 1 个。

## §4 精确改点清单（7 处主候选，逐文件逐行）

统一范式：在现有标签内按 role="button" → @click → tabindex="0" → @keydown.enter="同名handler" 顺序补齐（与 R60/R95/R96 属性顺序一致）。
1. laiyipao/miniapp/src/pages/announcements/announcements.vue:98 —— role="button" tabindex="0" @keydown.enter="annKeyword = ''"（内联赋值沿用原表达式）
2. laiyipao/miniapp/src/pages/bag/bag.vue:134 —— role="button" tabindex="0" @keydown.enter="doSynthesize(r)"
3. laiyipao/miniapp/src/pages/bag/bag.vue:217 —— role="button" tabindex="0" @keydown.enter="toggleEquip(s.id)"
4. laiyipao/miniapp/src/pages/bag/bag.vue:351 —— role="button" tabindex="0" @keydown.enter="goShop"
5. laiyipao/miniapp/src/pages/bag/bag.vue:416 —— role="button" tabindex="0" @keydown.enter="doSavePreset"
6. laiyipao/miniapp/src/pages/bag/bag.vue:420 —— role="button" tabindex="0" @keydown.enter="focusPreset"
7. laiyipao/miniapp/src/pages/me/me.vue:111 —— role="button" tabindex="0" @keydown.enter="copyUserId"

## §5 扩展盲区与排除/待裁定

扩大扫描（含复合类 link）后另发现 14 处同型盲区，均 role/tabindex/@keydown 三缺（脚本 docs/laiyipao/audits/round98-a-link-keyboard/data/analyze-links.py 输出 22 行含 mastery 重复行，去重后 21 处）：
- me.vue:83 startEditNickname（改名，真浏览器已坐实）、:448 goVerify(b)（验真）、:450 toggleBattleDetail(b)（详情）
- bag.vue:140 showAllRecipes 切换、:211 doUpgrade(s.id)、:375 doEquipGem(g)、:380 doUnequipGem(g)、:385 doWash(g)、:442 doSetCurrent(p)、:448 doDeletePreset(p)
- index/index.vue:185 go(d.hint)（dim-hint link）
- mastery/mastery.vue:66 resetMastery（有 :class link-off 禁用态）
- shop/shop.vue:129 doBuy(it)（有 is-disabled 禁用态）
建议：前 12 处纯静态属性补齐，与 7 处主候选同轮纳入；mastery:66 / shop:129 禁用态需先确认 @click handler 内部是否自带守卫（如 resetting/soldOut），确认后再补，列为待裁定项，不在本轮最小候选内。
排除项：class 含 link 但无 @click 的纯文本 6 处（bag:142/213/366/421、mastery:67、shop:131）不是操作入口，不纳入。idx bagnet 等裸 view @click（非 link 类）属 R96 观察项口径，本视角不重复。

## §6 证据索引与截图 sha256

审计目录：docs/laiyipao/audits/round98-a-link-keyboard/{data,screenshots}/
- data/analyze-links.py（21 处可点 link 全量扫描）、probe-links.js / probe-me.js / probe-active.js / check-nick.js / inject-nick.js / enter-probe.js、drive-ann-390.sh / drive-me-390.sh / drive-me-tabs.sh / drive-me-ax.sh / drive-bag.sh / drive-bag-enter.sh / drive-bag-clean-tab.sh / drive-enter.sh
- 截图 sha256（6 张，390×844 / 768×1024 / 1280×900）：
  - bag-link-390.png  52742f594f3b9ef8b4cab8fbaca9621db3e8120143bf71620bceeda55f111da0（70398 B）
  - bag-link-768.png  9a3797392ee03627b23488442d769f3c7d761cd7d15b781f0aacbe08be827a3d（128062 B）
  - bag-link-1280.png 649ddc1557317030f081fb3bc017c890c5bd93bf2ae77666ab5777d21509b390（84173 B）
  - me-link-390.png   3ab6294ca35b49db853dda343ead65c8e5851a46d3be7c2ee9a54d67aade9966（114954 B）
  - me-link-768.png   3cbab45525f4e7842f8d7f7f6a1cd27229cea9efca65e6568b5ed02c78578ae1（258722 B）
  - me-link-1280.png  e1b71f5f503d8d8caafc7eb982b662f8801f5359dbdf6a46e8b8fc3694a23298（154305 B）

## §7 收尾核验

- HEAD：358901febaaf92ff5543e4f20f379ece52c4ea21（未变）
- git status --porcelain 非 ?? 项：NONE
- git diff --check：RC=0
- auth 四路径（laiyipao/deploy/.local.keys、laiyipao/deploy/remote-config.env、laiyipao/deploy/.env、根 .env）：ABSENT
- 自启 job：H5 服务 bash-70e1957a4ddb410e 已 job_kill；其余 promoted/background job 均已终止或自然完成
- 未改业务源码/契约/规范/.gitignore/令牌；未写真值；未 git add/commit/push
