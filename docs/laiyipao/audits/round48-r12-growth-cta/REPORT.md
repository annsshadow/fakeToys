# R48 真浏览器 QA：R12 成长 CTA 自指防护与非自指导航

结论：**PASS（带诚实边界）**。在 390×844 真 Chromium 中，分别构造「元素覆盖短板（背包自指）」「装备契合短板（装备页自指）」「反应链短板（非自指导航）」三种真实 UI 态；用底层鼠标 move/down/up 真实点击 CTA，验证了自指 CTA 不跨页、只在本页切换 tab/filter，非自指 CTA 跳转到反应图鉴。未修改任何业务源码，未做 git 写操作。

## 1. 权威口径与浏览器环境

- 规范：docs/laiyipao/specs/round48-r12-growth-cta/SPEC.md（§3 resolveR12Cta，§4 背包/装备自指动作，§5 index.vue DIM_HINT 不改）。
- 页面：静态 H5（http://127.0.0.1:5201，HTTP 200），API（http://127.0.0.1:8080，HTTP 200），浏览器 390×844，innerW=docW=390。
- 认证：临时 auth-init 由 QA 库 guest_token_aliases 在运行时生成，只含 lyp_token/lyp_refresh/lyp_token_exp/lyp_guest/lyp_profile，不落口令/JWT 真值到仓库或报告。
- 短板构造：真实存档 user10 的 build_rating 元素覆盖已满、装备契合为 0，无法形成元素覆盖短板。故用 agent-browser network route 在浏览器侧把 GET /api/v1/me 的 build_rating 替换为 QA 注入值（element=0/reaction=7/mastery=8/equipment=18/mechanic=8 等），仅在浏览器会话中生效，不改数据库、不改服务端。关键配方：先 open 空白页 → 带 init-script 打开深链 → open FIRST 后 set viewport → network route EXACT_URL --body ... --allow-origin http://127.0.0.1:5201 → reload。allow-origin 必须是页面源 5201（不是 API 源 8080），否则 CORS 令 route 失效；5 种失败变体均因此得到真实短板装备契合，已弃用。

## 2. 三种场景的真实点击证据

### 2.1 背包自指：元素覆盖短板

- 注入后 before：hash=#/pages/bag/bag，priorityText=元素覆盖 0/5（0%，权重 21%）：补满可涨约 105 分，优先补它，CTA 逐字「在本页强化元素覆盖 →」，当前 tab=装备6，CTA rect 中心 (196,203)。
- 真实鼠标点击：element-move.txt/element-down.txt/element-up.txt 均 Done，坐标 196,203。
- after：hash 仍 #/pages/bag/bag，CTA 仍「在本页强化元素覆盖 →」，tab 切回技能5，无跨页。截图 sha256=134ed634bd57a6dd69e2635b52f4fb0f59261a3d754fd8608503644523708d56（r48-element-after.png，目视确认背包页与技能 tab 选中）。

### 2.2 装备页自指：装备契合短板

- before：hash=#/pages/equipment/equipment，priorityText=装备契合 0/18（0%，权重 21%）：补满可涨约 378 分，优先补它，CTA 逐字「在本页强化装备契合 →」，filter=全部 18，CTA 中心 (196,238)。
- 真实鼠标点击：equipment move/down/up 均 Done。
- after：hash 仍 #/pages/equipment/equipment，CTA 仍自指文案，filter 回到全部 18（点击前用真实 click 切到「护符 3」作为可观察复位前置态），无跨页。截图 sha256=1851eedbf6306117e96746f1704d92b7584b13d76845fd69ea6edbf28d7e86a5（r48-equipment-after.png，目视确认装备图鉴页与全部筛选选中）。

### 2.3 非自指：反应链短板

- before：hash=#/pages/bag/bag，priorityText=反应链 0/7（0%，权重 26%）：补满可涨约 182 分，优先补它，CTA 逐字「看反应搭配 →」，CTA 中心 (196,203)。
- 真实鼠标点击：reaction move/down/up 均 Done。
- after：hash=#/pages/reactions/reactions，进入反应图鉴页，CTA 不再显示。截图 sha256=1c1694b0890385ad79ce980f12d9c5e87a70b2a77f59c0a9e595fcc70456f046（r48-reaction-after.png，目视确认反应图鉴页）。

## 3. index.vue DIM_HINT 文案/路由未改（源码只读）

- laiyipao/miniapp/src/pages/index/index.vue:668-674：元素覆盖→/pages/bag/bag（去背包配元素）、反应链→/pages/reactions/reactions（看反应搭配）、专精投入→/pages/mastery/mastery、装备契合→/pages/equipment/equipment、机制深度→/pages/bag/bag，全部保持原样。
- laiyipao/miniapp/src/styles/tokens.css:14-16 锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6；无 --r47-/--r48- 新令牌。

## 4. 门禁

- R48 契约（r12GrowthPriority.contract.test.ts）：14/14，exit 0。
- miniapp 全量：212 files passed | 1 skipped，1998 tests passed | 2 skipped，exit 0。
- pnpm run type-check：exit 0；pnpm run build:h5：exit 0（DONE Build complete，uni-app 版本提示除外）。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS；**TestNoRealSecretsInRepo SKIP（非 PASS）**，原文：读不到 WORKTREE_ROOT/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」。

## 5. 截图（6 张互异，均 390×844 非登录页）

| 场景 | sha256 |
| --- | --- |
| r48-element-before.png | b07c22dd88c248af00020d0d33d3814f9d4adcf6194efac66d4e4d2813e0818f |
| r48-element-after.png | 134ed634bd57a6dd69e2635b52f4fb0f59261a3d754fd8608503644523708d56 |
| r48-equipment-before.png | c2c54babe52c372f844296bc1b736223ef5972d90187fd3d3a2cac1245b4507f |
| r48-equipment-after.png | 1851eedbf6306117e96746f1704d92b7584b13d76845fd69ea6edbf28d7e86a5 |
| r48-reaction-before.png | 60f2fbf87a527cfbd598a9bc32ce4fcf832b52a4204536435f34becadd92e5d8 |
| r48-reaction-after.png | 1c1694b0890385ad79ce980f12d9c5e87a70b2a77f59c0a9e595fcc70456f046 |

## 6. 诚实边界

- 三场景 build_rating 短板通过浏览器 network route 注入构造（QA 注入手法），不是真实存档自然产生；已如实披露，不冒充真实存档。
- 早期失败变体（route 注册在首次 open 前 / allow-origin=API 源 8080 / wildcard route）得到的 before 是真实装备契合短板，element-after.json 曾被队长复核发现跨页；这些旧证据已弃用，最终报告仅采用 05:04–05:08 新证据。
- 旧 r48-final-driver.sh 因 shell 变量被 JS 模板提前展开未成功写出，后改用逐场景 r48-one-scene.sh 完成；04:59 旧截图被新证据覆盖，不作最终计数。
- Go 真值扫描守卫 SKIP，不能当作 PASS；契约与全量测试的 skipped 均为仓库既有，未隐藏。
- 未对后端、结算、replay_hash、分数、掉落、令牌做任何改动；无业务源码编辑、无 git 写操作。
