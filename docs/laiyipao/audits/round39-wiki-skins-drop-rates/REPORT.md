# R39 复查报告：Wiki 皮肤图鉴与掉落配置（真浏览器三视口对账）

- 任务：t23（laiyipao-r36）· 日期：2026-11-25 · 执行：worker-nex2
- 被审对象：laiyipao/admin Wiki 视图（ skins 图鉴 + drop_rates 表 + AI 四要素 + 搜索联动）
- 权威数据源：公开接口 `GET /api/v1/config`（快照存于 `data/config.json`）
- 结论：**PASS**（MISMATCHES=0；诚实边界见第 9 节）

## 1. 环境与方法

- 被测地址：http://127.0.0.1:5175/laiyipao-admin/#/wiki（Vite dev，base /laiyipao-admin/）；API http://127.0.0.1:8080（`bash docs/laiyipao/audits/round15-reaction-timeline/start-api.sh`，QA 库 postgres://postgres@127.0.0.1:15440/laiyipao）。
- 真实 Chromium（agent-browser 0.41.1 CLI），三视口 390×844 / 768×1024 / 1280×800。
- 鉴权：`mint-token.sh` 仅从环境变量 `QA_ADMIN_USER`/`QA_ADMIN_PASS` 读凭据 POST /api/v1/admin/login，产出 `data/admin-auth-init.js`（init-script 注入 localStorage），脚本与产物均无字面口令。
- 所有交互（切 tab、点击 CTA、输入搜索词、点击搜索结果）均为 agent-browser CLI 真实点击/键入，非 DOM 注入模拟。

## 2. 皮肤图鉴六字段逐格对账（三视口）

探针：`skin-probe.js` → `data/probe-{390,768,1280}.json`；对账器：`reconcile.py` → `data/reconcile.json`（**ALL_RECONCILE_OK=True**）。

- 三视口均为 6 行 × 5 列（ID / 名称 / 稀有度 / 被动 / 说明），逐格与 `/api/v1/config` 的 skins 比对：**MISMATCHES=0**。
- 行数据（config 原值）：1 游猎者/稀有/kinetic_bonus；2 炽炎使徒/史诗/fire_burn；3 霜行者/史诗/ice_freeze；4 雷鸣者/史诗/volt_chain；5 腐沼行者/传说/blight_spread；6 白垩女王/传说/boss_breaker。说明列均为真实 descr（如 游猎者=「动能系技能额外附带一次破甲击退。」）。
- 稀有度显示中文（rare→稀有、epic→史诗、legendary→传说），被动列保留 code 原样（kinetic_bonus 等），与契约一致。实现：laiyipao/admin/src/skinDropMechanicsText.ts（rarityText/passiveText）；表结构 laiyipao/admin/src/views/WikiView.vue:1241-1264。

## 3. drop_rates 六键对账（rules tab，.r39-drop-table）

探针：`drop-probe.js` → `data/drop-{390,768,1280}.json`。

- 六行逐键与 config 一致，**MISMATCHES=0**，三视口一致：基础金币 200 / 每星金币 150 / 每次击杀金币 20 / 反应加成 30 / 三星钥匙 1 / 通关返还体力 5。
- `hasPct=false`：页面无 %/‰ 单位后缀（laiyipao/admin/src/skinDropMechanicsText.ts 的 dropRatesText 输出纯数值）。
- 表实现：WikiView.vue:1332 起（#drop-rates 标题后两列表格）。

## 4. AI 四要素同屏（skins tab）

`data/probe-*.json` 的 ai 段：

- conclusion：页面文本为「AI 机制概览：共 6 款皮肤：稀有 1、史诗 3、传说 2；掉落 6/6 项齐备」（「AI 机制概览：」为 UI 前缀，契约要求的统计句完整存在）。
- basis：含「公开 /api/v1/config」字样（data-ai-basis）。
- honesty：含 replay_hash 字样（data-ai-honesty）。
- cta：文案「查看皮肤图鉴 →」，href=#/wiki?tab=skins，可见。实现：WikiView.vue:1266-1270；文案源 skinDropMechanicsText.ts 常量。

## 5. CTA 真实点击（不跳看板）

`cta-click.sh`（CLI click .r39-ai-note a）→ `data/cta-pre.json` / `data/cta-post.json`：

- 点击前 hashBefore=#/wiki?tab=skins（rowCount 6，CTA 可见）。
- 点击后 hashAfter=#/wiki?tab=skins，**onDashboard=false**，activeNav=[皮肤图鉴]，皮肤表仍 6 行可见。
- 即 CTA 停留在皮肤 tab，未跳看板。截图：screenshots/r39-04-cta-before.png、r39-05-cta-after.png。

## 6. 搜索命中与行高亮（真实 CLI）

两步验证（对应 `data/search-*.json` 与 `data/searchpick-*.json`）：

1. 键入「游猎者」→ 面板命中 {kind:皮肤, title:游猎者, meta:皮肤 · 稀有}；键入「skin_chalk」→ 命中 {皮肤, 白垩女王, 皮肤 · 传说}。仅键入不产生行高亮（skinRowFocus=false）——这是输入态的预期行为。
2. 真实 CLI 点击 .search-hit（search-pick.sh）后：
   - 游猎者：hash=#/wiki?tab=skins&focus=skin:1，tr.row-focus 命中，focusText 含「1 游猎者 稀有 kinetic_bonus 动能系技能额外附带一次破甲击退。」
   - skin_chalk：hash=#/wiki?tab=skins&focus=skin:6，tr.row-focus 命中，focusText 含「6 白垩女王 传说 boss_breaker 对 BOSS 的反应伤害提高 30%。」
- 机制链：pick(h)（WikiView.vue:816-826）→ goTo 带 focus → rowFocusClass（WikiView.vue:394-401）按 focus.kind+id 匹配行；:839 :data-focus 为高亮依赖。截图：r39-08-searchpick-youliezhe.png、r39-08-searchpick-skin_chalk.png（可见蓝框高亮行）。

## 7. 表内横滚（移动端不撑破页面）

`data/probe-390.json` / `probe-768.json` / `probe-1280.json`：

- 390 视口：.r39-skin-table-scroll sw=640 > cw=324，scrollLeft 0→316 可滚到末列（说明列完整可达）。
- 768 视口：sw=cw=702；1280 视口：sw=cw=884——整表直接可见，末列直接可见。
- 三视口 documentElement.scrollWidth==innerWidth（390/768/1280）：页面级无横向溢出。横滚容器实现：WikiView.vue:1250。

## 8. 回归（R31–R38，真实 CLI 切 tab）

`reg-tabs.sh` → `data/reg-*.json`（每 tab 均为点击导航后对可见 section 计数）：

| tab | 期望 | 实测 | 备注 |
|---|---|---|---|
| basics 反应表 | 7 | 7 | 腐蚀扩散行含「传播 1 层」、破甲行含 150‰ 与 4px（R31/R38 语义回归） |
| basics 星级表 | 5 | 5 | |
| skills 技能表 | 24 | 24 | |
| skills 合成 | 18 | 18 | |
| skills 配方 | 18 | 18 | |
| enemies | 22 | 22 | |
| levels | 20 | 20 | |
| gems | 8 | 8 | 另有品质 5 行 |
| mastery | 8×12=96 | 8×12=96 | 深链 #/wiki?tab=mastery 复核：on=[专精树]、可见 section=r37-mastery-mechanics（data/mcheck.json、r39-09-mastery-deeplink.png） |
| skins | 6 | 6 | |
| rules | 星级 3 + drop 6 | 一致 | |

全部 tab docW==innerW=1258，无 NaN/undefined/[object Object]（bad=false）。

## 9. 测试门禁与诚实边界

- R39 契约：vitest run tests/WikiSkinDropMechanics.test.ts tests/WikiReactionDescr.test.ts → **14/14 通过**（含 11 条 R39 契约）。
- admin 全量：**867/867 通过**；79 个测试文件中 78 过，唯一失败文件 tests/WikiRatingMechanics.test.ts「× R40 评分表、选择器、真实调用与 AI 四要素齐全」——这是 **R40 前瞻 RED**（src/ratingMechanicsText.ts 已存在、UI 未实施），属计划内 TDD，非 R39 缺陷。
- type-check（vue-tsc --noEmit）通过；vite build 通过（WikiView-8EISUX1b.js 51.84 kB）。
- **TestNoRealSecretsInRepo：SKIP（因本机无 .local.keys），非 PASS**，如实记录。
- 凭据卫生：审计目录 grep admin12345 零命中；mint-token.sh 仅用环境变量；收尾时删除 data/admin-auth-init.js 与 data/login.json（脚本 finalize.sh），QA 库还原 battle_records count=104 / max(id)=477（基线 dump：data/qa-before.dump）。
- 截图 19 张、17 个不同 sha256；其中 r39-04-cta-before.png == r39-05-cta-after.png == r39-1280-skins.png（265806d1…）：CTA 点击后页面视觉不变（同 tab 重进），属诚实结果，非截图缺失。
- mcheck 深链探针（data/mcheck.json）中 263 行/表为探针选择器伪影（把整页 tbody 计入）；权威行数以 reg-mastery.json 的 8×12=96 为准。

## 10. 截图 sha256 清单（screenshots/）

| 文件 | sha256 |
|---|---|
| r39-390-skins.png | 4c5763f8262ee2397082a37cbbd37c0aaeaa87466ec556b9fa5a815594b98ebb |
| r39-768-skins.png | 5663471e852afc7962f636b67fec3ca8223a881b31bb720ffbf2cc051a05e91f |
| r39-1280-skins.png | 265806d1724a725cb68ee4de5c8b96b46d0f54fab4937bec22a073398273eafb |
| r39-04-cta-before.png | 265806d1724a725cb68ee4de5c8b96b46d0f54fab4937bec22a073398273eafb |
| r39-05-cta-after.png | 265806d1724a725cb68ee4de5c8b96b46d0f54fab4937bec22a073398273eafb |
| r39-06-search-youliezhe.png | 4591363a9bca4d30978c706ebe6b87beaf401dd0a54422932c53c72dd7a7b163 |
| r39-06-search-skin_chalk.png | 6dabf654354e477f7593b1da39116d765cdb3db08ecddf7b730b90773cbd095e |
| r39-08-searchpick-youliezhe.png | ed2f40ce9b8647530a0a3bff3d842e48a1399d5f573c6fc42b1b860f86ddbe88 |
| r39-08-searchpick-skin_chalk.png | ff1f373803e3a6e12bc9575b3ba59d2f145e7370b918989c53c9b52134d1159e |
| r39-07-reg-basics.png | 0db7d33d45814700f87eb84020a3d6b4ea1155262aa6fc3a3e2416583f2c2aa2 |
| r39-07-reg-skills.png | 889f28fc9041f987d49e95445883d2b99a1174a364438bab79d923fbd821e7dd |
| r39-07-reg-composite.png | 65b7db3b4ec2fee01bcc6d266c8dc79f8071dc7dc0e70a635ef1c611ebd90198 |
| r39-07-reg-enemies.png | e901c522564d3168d920ac3d935add7d4bde8be363db9fa1479f84182a5b11c7 |
| r39-07-reg-levels.png | bca88f6016e74e6eda5495845741e52d32903df6ee2c6a55ce5eb47663c17858 |
| r39-07-reg-gems.png | 2500550050f930e3f8254a5cc7548d779cf766e98493cd5c2d2b3242a24bef60 |
| r39-07-reg-mastery.png | 654c7fbc6358107f4e2865fc2cb068b4de7bfc39f440994b61631bc08689e646 |
| r39-07-reg-rules.png | 61e08e0cec873528bacb123a64157190a9173ec1d97bf91284a02b5ddbcea2ad |
| r39-07-reg-skins.png | a0ca9af3b181815a1c195bb0c3f2b75bf87684070d27cf017c329726247a690a |
| r39-09-mastery-deeplink.png | ec248d70eaaf560af47131a5e88494be296761d14e41e5885401f81b6ed7d386 |

## 11. 证据文件索引（本目录）

- 权威快照：data/config.json（/api/v1/config）
- 三视口皮肤：data/probe-{390,768,1280}.json；drop：data/drop-{390,768,1280}.json；对账：data/reconcile.json
- CTA：data/cta-pre.json、data/cta-post.json
- 搜索：data/search-{youliezhe,skin_chalk}.json（键入）、data/searchpick-{youliezhe,skin_chalk}.json（点击后高亮）
- 回归：data/reg-{basics,skills,composite,enemies,levels,gems,mastery,skins,rules}.json、data/mcheck.json
- 脚本：mint-token.sh、e2e-viewports.sh、cta-click.sh、search-click.sh、search-pick.sh、reg-tabs.sh、reconcile.py、finalize.sh、各 probe.js
- 基线：data/qa-before.dump（289,790 字节）
- 源码锚点：WikiView.vue:96-97、:140-172、:394-401、:551-558、:816-826、:839、:1241-1270、:1332；skinDropMechanicsText.ts:1-85
