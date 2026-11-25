# R48 部署后真浏览器复核：R12 成长 CTA 自指防护与非自指导航

结论：**PASS（带诚实边界）**。在重建 H5 静态产物上，用 390×844 真 Chromium 对三场景逐一做底层鼠标 move/down/up 真点击：背包自指元素覆盖、装备页自指装备契合两场景页内动作、hash 不变；反应链非自指场景真实跳转到反应图鉴。逐字段对账 before/after 与 t65 QA 基线一致，六张有效截图与 QA 基线确定性同 sha（已亲读三张 after 图）。

## 1. 部署产物与服务

- H5 重建：cd laiyipao/miniapp && pnpm build:h5 后台 job completed，DONE Build complete（Compiler 5.24 vue3，仅 Appid 提示）。
- 重建后确定性产物 sha 不变：laiyipao/miniapp/dist/build/h5/assets/index-BM_sn-oz.js = 5b3914547c65a0d0d9e83ba331c51c839cfcb09951ca3ef938b15f77d86e155a。bundle grep 命中「在本页强化」「看反应搭配」「r12-priority-cta」（含 pages-bag/pages-equipment/r12GrowthPriority 分块）。
- 服务：QA API 用 docs/laiyipao/audits/round15-reaction-timeline/start-api.sh 后台启动，8080 healthz/readyz/config 均 200；H5 静态用 npx http-server 127.0.0.1:5201，/ 与 /index.html 均 200。curl 一律加 --noproxy 星号。
- 不回退产物：后端 lyp-server-amd64 = 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6；admin WikiView-ST1PbAt2.js = f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b。
- QA 库 battle_records = 104|477。原 user10 别名过期，验证前刷新为 3 小时有效别名，会话结束清理。

## 2. 认证与短板构造

- QA 库 guest_token_aliases 中 user10 未过期别名，gen-alias-auth-init.sh 经 POST /api/v1/auth/guest 换取 user10 的 user token，auth-init.js 只含 lyp_token/lyp_refresh/lyp_token_exp/lyp_guest/lyp_profile，无口令/JWT 真值入库。
- 真实存档 user10 的元素覆盖已满，无法自然构造元素覆盖短板；短板由 agent-browser network route 在浏览器侧替换 GET /api/v1/me 的 build_rating 注入（element=0/reaction=7/mastery=8/equipment=18/mechanic=8 等）。不改数据库、不改服务端，仅当前浏览器会话生效。
- 关键配方：fresh session open 空白页 → 带 init-script 打开深链 → set viewport 390 844 → network route EXACT_URL --body ... --allow-origin 页面源 5201 → reload → wait .r12-priority-cta → 真鼠标点击。allow-origin 必须是页面源 5201（API 源 8080 会 CORS 失效）。

## 3. 三场景逐字段对账（before → after）

### 3.1 背包自指：元素覆盖

- before：hash=#/pages/bag/bag，priorityText=元素覆盖 0/5（0%，权重 21%）：补满可涨约 105 分，优先补它；CTA=在本页强化元素覆盖 →；tab=装备6；ctaRect 中心 (196,203)；bodyLen=1507。
- 点击：move (196,203) ✓ Done、down left ✓ Done、up left ✓ Done。
- after：hash 仍 #/pages/bag/bag；CTA 仍自指文案；tab=技能5；skillsCardVisible=true / skillLibraryVisible=true / moreCardVisible=true；bodyLen=3601。自指切 tab 成功、无跨页。

### 3.2 装备页自指：装备契合

- before：hash=#/pages/equipment/equipment，priorityText=装备契合 0/18（0%，权重 21%）：补满可涨约 378 分，优先补它；CTA=在本页强化装备契合 →；filter=护符 3（点击前用真点击切入作为复位前置态）；ctaRect 中心 (196,238)；bodyLen=1335。
- 点击：move (196,238) ✓ Done、down left ✓ Done、up left ✓ Done。
- after：hash 仍 #/pages/equipment/equipment；CTA 仍自指文案；filter=全部 18；bodyLen=2320。自指复位 filter 成功、无跨页。

### 3.3 非自指：反应链

- before：hash=#/pages/bag/bag，priorityText=反应链 0/7（0%，权重 26%）：补满可涨约 182 分，优先补它；CTA=看反应搭配 →；ctaRect 中心 (196,203)；bodyLen=3596。
- 点击：move (196,203) ✓ Done、down left ✓ Done、up left ✓ Done。
- after：hash=#/pages/reactions/reactions，CTA 不再显示，filter=全部 7，bodyLen=2497。非自指真实导航成功。

## 4. 截图证据（6 张，390×844 非登录页）

| 场景 | 文件 | sha256 |
| --- | --- | --- |
| 元素覆盖 before | pd-element-before.png | b07c22dd88c248af00020d0d33d3814f9d4adcf6194efac66d4e4d2813e0818f |
| 元素覆盖 after | pd-element-after.png | 134ed634bd57a6dd69e2635b52f4fb0f59261a3d754fd8608503644523708d56 |
| 装备契合 before | pd-equipment-before.png | c2c54babe52c372f844296bc1b736223ef5972d90187fd3d3a2cac1245b4507f |
| 装备契合 after | pd-equipment-after.png | 1851eedbf6306117e96746f1704d92b7584b13d76845fd69ea6edbf28d7e86a5 |
| 反应链 before | pd-reaction-before.png | 60f2fbf87a527cfbd598a9bc32ce4fcf832b52a4204536435f34becadd92e5d8 |
| 反应链 after | pd-reaction-after.png | 1c1694b0890385ad79ce980f12d9c5e87a70b2a77f59c0a9e595fcc70456f046 |

- 三张 after 图已亲读：pd-element-after 显示背包页「技能 5」选中；pd-equipment-after 显示装备图鉴「全部 18」选中；pd-reaction-after 显示反应图鉴页。与逐字段 JSON 对账一致。

## 5. 门禁复核

- Go 守卫：go test ./internal/domain -run TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo -count=1 -v：TestNoInfraIdentifiersInRepo PASS（4.81s）；TestNoRealSecretsInRepo SKIP（非 PASS），原文：读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」。
- t65 QA 门禁：R48 契约 14/14；miniapp 全量 1998 passed|2 skipped；type-check/build exit 0。skipped 均为仓库既有。

## 6. 诚实边界

- 六张部署后截图与 t65 QA 基线六图一一同 sha：同一确定性 H5 产物、同一 390×844 视口、同一注入数据、同一操作路径产生同一像素。本次未产出互异的新图 sha；部署后点击生效以 before/after 探针逐字段位移为准，不冒充新图。
- 三场景短板是浏览器侧 network route 注入构造，不是真实存档自然短板；未改 DB/服务端。
- 认证使用临时刷新别名，验证结束 finalize 清理 auth-init.js 并核对 QA 库 104|477。
- R47 的 admin/review 与后端产物均未改动、未回退。

## 7. 结论

重建 H5 部署后三场景真浏览器复核全部 PASS：自指 CTA 页内生效不跳页（背包切 tab、装备复位 filter），非自指 CTA 真导航反应图鉴；产物不回退、服务 200、守卫如实标注。R48 可发布。
