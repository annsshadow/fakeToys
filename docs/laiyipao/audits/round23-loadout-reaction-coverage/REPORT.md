# R23 复查报告：备战态构筑反应覆盖诊断（真浏览器实测）

- 任务：t184（attempt 2）｜复查人：qa-e2e-wb｜日期：2026-11-24
- 判定：**PASS**（复审通过）—— 原 R23-BUG【中】深链接假空态已由队长 test-first 修复（doLoadPage() 新增 await store.loadLoadout()），复审实测深链接直入已正常显示真实构筑覆盖标签；核心诊断卡、AI 四判据、探针、契约与渲染测试全部成立；1 项浏览器侧限制（真实空构筑用户不可造）如实标注，单测已覆盖；未改业务源码、未做 git。
- 浏览器：agent-browser（**非物理真机**，本地回环 H5 服务，视口 390×844 与 1258×588 截图）。

## 验收结论一览

| 验收项 | 结论 | 证据 |
| --- | --- | --- |
| 有数据构筑显示覆盖标签 + 补元素建议 | 通过（导航 + 深链接直入均验证） | probe-nav.json：tags=3（蒸汽爆发/过热/超导），gap=补【毒】可新增【燃烧云】等3种反应 |
| 空槽/单元素诚实空态逐字 | 未能在浏览器验证（限制见 §4） | 单测已覆盖；QA 库无法造出全空构筑用户 |
| AI 四判据 + 依据行 + 诚实红线 | 通过 | probe-nav.json：hasAI/hasBasis/hasCTA/hasHonesty 全 true，逐字比对一致 |
| hash 不污染 / 只读诊断 | 通过 | 源码 r23GoLoadout 仅 showToast 不写 store；契约守卫三端不回退 14/14 绿 |
| 探针 pe/ce/af/fr 全 0 | 通过 | 三态探针 pe/ce/af/fr 均为 [] |
| 截图 sha 各异 + 亲读非登录页 | 通过 | 3 张决定性截图 sha 两两不同，均亲读确认为技能图鉴页（非登录页） |
| 零真值 | 通过 | 严格模式扫描无实际 JWT/口令/域名/私钥；模板仅占位符；含真 token 的 auth-init.js 已删 |

## 1. 有数据态（经导航进入）—— 诊断卡真实正确

- 路径：先经背包页触发权威构筑加载，再 hash 切至 #/pages/skills/skills；用户 10 权威构筑 {skill_ids:[1,4,7,0]}（焰/冰/电）。
- DOM 探针（data/probe-nav.json）：.r23-coverage-tag 3 个 = 蒸汽爆发 / 过热 / 超导；.r23-gap-suggest = 补【毒】可新增【燃烧云】等3种反应；覆盖数 3 与 REACTION_PAIRS 派生一致。
- AI 卡逐字：结论「本套构筑覆盖 3 种反应（蒸汽爆发、过热、超导）；补【毒】可新增【燃烧云】等3种反应」；依据「依据：构筑槽元素 × REACTION_PAIRS(源自引擎 lookupReaction)派生」；CTA「去调整出战配置 →」；诚实行「辅助诊断非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准」。四判据齐、逐字与 SPEC §4/§5 一致。
- hasUndefined=false、hasNaN=false；pe/ce/af/fr 均 []。
- 截图：screenshots/r23-03-nav-skills.png（sha256 24c5eed92363bc0af59c516dd1cc6511c7e5ad5e756f9d125b65f11c6fef5cda，1258×588），亲读见技能图鉴页「AI·当前构筑可触发反应」三标签 + 补毒建议 + 依据/CTA/诚实行，非登录页。

## 2. 缺陷 R23-BUG【中】：深链接直入显示假空态（队长已 test-first 修复，复审通过，详见 §8）

- 现象：直接打开 #/pages/skills/skills（用户 10，服务端权威构筑明明是焰/冰/电）时，诊断卡显示空态「当前构筑没有元素技能，无法诊断反应覆盖」，.r23-coverage-tag 数 = 0（data/probe-data.json，status=empty、me:[{status:200,loadout:null}]）。而同一用户经导航进入却正确显示 3 标签（§1）。这是真实的假阳性空态：玩家直接落到技能页会看不到本该显示的覆盖诊断。
- 证据截图：screenshots/r23-01-skills-top.png（sha256 b0ac2e0bbcb7cb536b018995e80fc1bf13034876c803cee345aac901df2152ee，390×844），亲读确认为技能图鉴页、诊断卡显示空态文案，非登录页。
- 根因链（只读分析，未改源码）：
  1. laiyipao/miniapp/src/store/game.ts:591-599 的 loadLoadout() 才从服务端加载权威 4 槽；
  2. laiyipao/miniapp/src/store/game.ts:334-380 的 doLogin() 只 Promise.all([loadConfig(), refreshProfile()])，不含 loadLoadout()；
  3. laiyipao/miniapp/src/pages/index/index.vue:702 的 onMounted 只调 login；
  4. laiyipao/miniapp/src/pages/skills/skills.vue:662-691 的 doLoadPage() 只 loadConfig()、不 loadLoadout()；
  5. 于是深链接直达时 store.loadout 仍为默认 [0,0,0,0]，r23Coverage 派生出空集 → 走 SPEC 分支 1 空态。
- 测试覆盖缺口：t183 的 skills_r23_coverage.test.ts 直接向 store 注入 loadout，未覆盖「深链接进入、loadout 尚未加载」这一真实导航场景，故测试全绿未抓到。
- 修复建议（交实施者，QA 不改源码）：在 skills.vue 的 doLoadPage() 中显式 await store.loadLoadout()（或复用统一守卫），保证任意入口进入都能拿到权威构筑后再派生；并补一条深链接/未加载 store 的渲染用例。

## 3. 旧战报 / hash 不污染 / 三端不回退

- 诊断卡完全只读：r23Coverage/r23BestGap 均为 computed；CTA r23GoLoadout() 仅 uni.showToast，不写 store.loadout、不触 persistLoadout/saveLoadout。
- 契约测试 skills_r23_coverage.contract.test.ts 14/14 与渲染 skills_r23_coverage.test.ts 3 用例（共 2 文件 17 测试）实跑全绿；RED 基线：git show HEAD 的 skills.vue grep -c r23-coverage = 0、测试文件 NOT_IN_HEAD，证明 test-first。
- 三端不回退：tokens.css 锚点 v1/v3-r3/r6 保留、无 --r23- 新令牌；admin 零改动；server replay_hash 口径未动（契约守卫全绿佐证）。

## 4. 诚实标注：真实空态分支无法在浏览器验证（限制，非缺陷）

- 试图用空构筑用户验证 SPEC 分支 1 的逐字空态。实测 user 127/11/1 经 GET /api/v1/me/loadout 权威返回均为 {skill_ids:[1,4,7,0]}（默认焰/冰/电），QA 库 user_skill_slots 中不存在全零槽位用户（bool_and(skill_id=0) 查无结果）。
- 因此当前 QA 数据下无法在真浏览器造出真实空构筑，空态分支只能在单元测试（skills_r23_coverage.test.ts 已覆盖逐字文案）验证。screenshots/r23-04-empty-user127.png（sha256 2014f38b38b4c03abd7bb7c7d280f8c43500c3479ad35bfb2d7826d538da120e）亲读实为有数据态（三标签 + 补毒建议），不作为空态证据，仅留作此限制的说明。
- §2 中观察到的深链接假空态是缺陷（数据本该有值却显示空态），与此处真空态不可造是两件事，不混淆。

## 5. 截图清单（sha256，亲读均非登录页）

| 截图 | sha256 | 说明 |
| --- | --- | --- |
| r23-01-skills-top.png | 0a99fd94bed26a1499cee647e109e3ef560ff10a3b134cd03b304241e40f9f1b | 修复后深链接直入有数据态，三标签 + 补毒建议（390×844） |
| r23-03-nav-skills.png | 24c5eed92363bc0af59c516dd1cc6511c7e5ad5e756f9d125b65f11c6fef5cda | 导航进入有数据态，三标签 + 补毒建议（1258×588） |
| r23-04-empty-user127.png | 2014f38b38b4c03abd7bb7c7d280f8c43500c3479ad35bfb2d7826d538da120e | 空态尝试失败（实为有数据态），非证据（1258×588） |
| r23-05-deeplink-fixed.png | 8215da87df6b2247123622ac2f2e6885bc62729dc4ded325fefbeb2a27a075e5 | 修复后深链接直入（滚动态），三标签 + 补毒建议（1258×588） |

重复误导截图 r23-02-coverage-card.png（滚动脚本未生效、与 01 同 sha）已删除；四张截图 sha 两两不同，均亲读确认为技能图鉴页非登录页。

## 6. 零真值与环境还原

- 严格模式扫描审计目录：无实际 JWT（eyJ 串）、无口令字面量、无私钥头、无真实域名/个人路径；auth-init.tpl.js 仅含占位符 __LYP_TOKEN__，模板与报告中无真值。
- 含真 token 的 auth-init.js 已删除并确认不存在；.gitignore 已追加本审计目录 data/ 与 auth-init.js 两条。
- QA 库已 pg_restore 还原 data/qa-before.dump（还原退出码 0，还原后 battle_records=103），测试数据未污染。
- 服务：本地回环 API 与 H5 服务运行正常（根路径 404 为无根路由的正常表现，H5=200）。

## 7. 最终判定

**PASS（复审通过）**：诊断卡核心算法、AI 四判据、诚实红线、探针与测试均在真浏览器得到验证且正确；原 R23-BUG【中】深链接假空态已由队长 test-first 修复并经真浏览器复审确认消除（详见 §8）。空态浏览器验证受 QA 数据限制未能完成，已如实标注，单测已覆盖。

## 8. 复审记录（队长修复 R23-BUG 后，2026-11-24）

- 修复内容（队长 test-first）：laiyipao/miniapp/src/pages/skills/skills.vue:679-682 的 doLoadPage() 在 loadConfig 后新增 await store.loadLoadout()（loadLoadout 幂等 + 序号守卫）；契约测试新增「深链接直入必须 loadLoadout」守卫（RED 1 → GREEN）。QA 亲读源码确认该行存在。
- 深链接直入复审（不经任何导航，直接打开 #/pages/skills/skills，用户 10）：DOM 探针 data/probe-data.json 从修复前的 status=empty、tags=[] 变为 status=ok、tagsCount=3（蒸汽爆发/过热/超导），gap=补【毒】可新增【燃烧云】等3种反应，AI 四判据 hasAI/hasBasis/hasCTA/hasHonesty 全 true，逐字与 SPEC 一致，hasUndefined/hasNaN=false，pe/ce/af/fr 全 []，loginPage=false。
- 截图 screenshots/r23-01-skills-top.png（sha256 0a99fd94bed26a1499cee647e109e3ef560ff10a3b134cd03b304241e40f9f1b，390×844）与 screenshots/r23-05-deeplink-fixed.png（sha256 8215da87df6b2247123622ac2f2e6885bc62729dc4ded325fefbeb2a27a075e5，1258×588）均已亲读：诊断卡真实显示三标签 + 补毒建议 + 依据/CTA/诚实行，非登录页。
- 原有导航态证据（r23-03-nav-skills.png、probe-nav.json）与 AI 四判据、探针零错结论仍成立，未回退。
- 测试：npx vitest run skills_r23_coverage.contract.test.ts + skills_r23_coverage.test.ts => 2 files / 18 tests ALL PASSED（含新增深链接守卫）；npx vue-tsc --noEmit exit 0。
- QA 构筑默认限制不变：QA 库全用户 /me/loadout 均返回 [1,4,7,0]，无全零槽位用户，真实空态分支浏览器侧仍不可验证，单测已覆盖，如实标注。
- 复审后 QA 库已 pg_restore 还原（退出码 0，战报 103）；含真 token 的 auth-init.js 已删；.gitignore 已含本审计目录 data/ 与 auth-init.js；零真值扫描无实际 JWT/口令/私钥/域名（仅占位符 __LYP_TOKEN__）。
- 复审判定：PASS（原 needs_revision 的缺陷已修复并实测通过）。
