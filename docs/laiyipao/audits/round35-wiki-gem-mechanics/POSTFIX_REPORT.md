# R35 修复后复查报告：宝石品质名搜索 gold 真浏览器闭环

> 任务：t274『R35修复后复查：宝石品质名搜索gold真浏览器闭环』（attempt_id=d79ab5ec-dca8-4b91-8a0e-d6d22b271132）
> 审计目录：docs/laiyipao/audits/round35-wiki-gem-mechanics/
> 复查时间：2026-11-25T15:14+08:00（Asia/Shanghai）
> 被审修复：t273（WikiView.vue 新增 quality HitKind、searchIndex 索引 gem_qualities、rowFocusClass 品质行高亮）
> 权威源：公开 /api/v1/config（不入库，仅快照 sha 与字段入 data/，已被 gitignore）

## 〇、首轮缺口与本轮结论

首轮 t272 判 FAILED：真实 CLI 搜索品质名 gold 未命中（hash 停留 #/wiki?tab=gems、focused=[]、highlighted=0），根因为 WikiView.vue:422-507 searchIndex 仅索引 c.gems、未索引 c.gem_qualities。
本轮 t274 在 t273 修复后独立复查：gold 真实 CLI 搜索命中 focus=quality:gold 且 DOM 行高亮，锋锐石 focus=gem:1 不回退；九项硬门禁全部坐实，**整体判 PASS**。

## 一、权威 config 快照

- SAMPLED_AT=2026-11-25T15:14:13+08:00
- CFG_SHA=16f47bedb9aee1474d3213b56988697732e4541c602eba8ecd22a656c3
- version=6237e0b623219b8b；gems=8；gem_qualities=5（与首轮 version 同，字段一致）
- gem1=锋锐石/attack/提高面板攻击力；qualities=[gray/1/1000, blue/1/1200, purple/2/1450, red/2/1700, gold/3/2000]
- 落盘 data/config.json（75367 字节，gitignore 命中，未入库）

## 二、三视口逐字段对账（门禁①②⑥）

e2e-viewports.sh：390×844 / 768×1024 / 1280×800 三视口真实 Chromium，先 about:blank→set viewport→init-script 注入令牌→open→reload→eval gem-probe.js→screenshot。
reconcile.py 390 768 1280 结果（data/reconcile.log）：
- 各视口 GEMS=8 QUALS=5、gem表4列(ID/名称/属性/说明)8行、quality表3列(品质/词条数/系数)5行
- MISMATCHES=0、AI_OK=True、NOTE_OK=True、BAD=False、DOC=viewport/viewport
- 两表 wrap1/wrap2 scrollWidth==clientWidth（324/702/884，列宽自适应无需实际横滚）、CTA='查看宝石图鉴 →' href=#/wiki?tab=gems
- ALL_RECONCILE_OK=True、VERDICT=PASS；千分号 1000‰–2000‰ 逐档一致，禁百分比/禁 MinValue/MaxValue/SeedGemAffixes
- EXPECT_AI=共 8 种宝石，5 档品质，词条 1–3 条

## 三、决定性搜索闭环（门禁②，本轮核心）

search2.sh 真实 CLI：click 'input[type=search]'→keyboard type→press Enter→eval search2-probe.js→screenshot（非 DOM eval/e.click）。
- data/search2-gold.json：hash=#/wiki?tab=gems&focus=quality:gold、focusAttr=quality:gold、focusRows=['gold 3 条 2000‰']、inputValue=gold、activeNav=[宝石图鉴]、goldHasFocus=true、fengHasFocus=false、gemSection=true、badText=false
- data/search2-feng.json：hash=#/wiki?tab=gems&focus=gem:1、focusAttr=gem:1、focusRows=['1 锋锐石 攻击 提高面板攻击力。']、fengHasFocus=true、goldHasFocus=false
- 判定：gold 命中品质行并高亮、锋锐石命中宝石行，首轮缺口已闭环；DOM 行高亮由 search2-probe.js 的 goldHasFocus/focusRows 与截图 r35p-search-gold.png 双重坐实

## 四、CTA 真 CLI 点击（门禁③）

cta-click.sh 会话 lyp-r35-cta，真实 CLI click '[data-r35-cta]'：
- data/cta-click.txt='✓ Done'
- data/cta-pre.json：hashBefore=#/wiki?tab=gems、ctaText=查看宝石图鉴 →、ctaHref=#/wiki?tab=gems、ctaVisible=true、gemRowCount=13
- data/cta-post.json：hashAfter=#/wiki?tab=gems、gemStillThere=true、rowTotal=13、dashboard=false、activeNav=[宝石图鉴]
- 判定：点击后停留 gems tab，未跳 dashboard（hash-router catch-all 未误触发）

## 五、回归四 tab 不回退（门禁⑤）

reg.sh 五 tab（basics/skills/enemies/levels/gems）探针 data/reg-*.json 全部一致：
- reactionCount=7、corrosionSpread=true（传播 1 层）、armorShred=true（150‰）、armorKnock=true（4px）
- skillCount=24、enemyCount=22、levelCount=20、bad=false、docW=innerW=1258
- 判定：R31 反应 / R32 技能 / R33 敌人 / R34 关卡四 tab 均不回退

## 六、截图证据（门禁⑦）

本轮新增 4 张唯一截图（sha256，两两不同且≠首轮 t272 截图）：
- r35p-search-gold.png=9997a21484e1639cb33a2f59032fc4f192986be08422f90f0ca5c718d30cdc2f（390×844，亲读 gold 品质行 2000‰ 蓝底高亮 + AI 四要素 + 诚实文案，非登录页）
- r35p-gold-768.png=6bf950c769477981db9f04cd81d77e2b51d4d9aa4894de19f2e45f82ce340841（768×1024）
- r35p-gold-1280.png=bae5abca11fa3b49d801b862f285aeb3a664afa1f4c094085170eeb516ee3b10（1280×800）
- r35p-feng-768.png=f57309bba8c00cd87d7a84a9c3d05558d3047bcda395a9a64fdd79d869fc7e14（768×1024）
- 重复图不计独立证据：r35p-search-feng.png 与首轮 r35-search-锋锐石.png 同 sha=fde56332…；r35-04-cta-before.png 与 r35-390-gems.png 同 sha=3dd91d6d…
- 另亲读首轮 r35-390-gems.png（3dd91d6d…，非登录页宝石表）、r35-1280-gems.png（25241659…，宝石表+品质表可见）

## 七、测试与零真值（门禁④⑧）

测试 job bash-448d1b138e874459（RC=0）：
- R35 定向：pnpm exec vitest run tests/WikiGemMechanics.test.ts → 1 file / 9 tests passed（931ms）
- 前序 R31-R34+R35：5 files / 79 tests passed（18.25s）
- 全量：pnpm test -- --testTimeout=90000 → 74 files / 827 tests passed（46.54s）
- pnpm type-check：RC=0；pnpm build：RC=0（✓ built in 1.62s）
- **TestNoRealSecretsInRepo：因 laiyipao/deploy/.local.keys 缺失，按 secrets_leak_test.go:145 显式 SKIP，未执行，不计 PASS**

零真值与清理：
- zero-secret-scan.sh：BAD_COUNT=0、SYNTAX_RC=0（通用正则不 exclude 自身）
- .gitignore:402 data/、.gitignore:403 admin-auth-init.js，git check-ignore 均命中
- data/admin-auth-init.js（含临时令牌）已删除：AUTH_AFTER=NO
- QA 库 docker exec lyp-qa-pg psql -tAc 'select count(*),max(id) from battle_records;' → 104|477（基线一致，本轮无写入）
- 未改业务源码、未 git；git status 中 .gitignore/WikiView.vue/WikiView.test.ts 为他人改动，本 QA 未触及

## 八、九项门禁逐项判定

1. 三视口逐字段对账（8宝石+5品质 MISMATCHES=0、千分号）：PASS
2. gold 搜索命中 focus=quality:gold + DOM 高亮 + 锋锐石不回退：PASS
3. CTA 真 CLI 点击停留 gems：PASS
4. AI 四要素同屏 + 诚实文案 + 禁数值范围：PASS
5. R31/R32/R33/R34 四 tab 不回退：PASS
6. 表内横滚 + 无页面溢出：PASS（sw==cw 自适应，docW==innerW）
7. ≥4 张新截图 sha 两两不同且≠首轮：PASS
8. 删 auth-init / gitignore / QA库104|477 / 零真值：PASS
9. 不改业务源码不 git：PASS

## 九、结论

t273 修复有效：首轮 t272 判 FAILED 的 gold 品质名搜索缺口已用真浏览器真实 CLI 闭环（focus=quality:gold + 行高亮 + 截图），其余八项门禁同样通过。本轮复查判 **PASS**。唯一未执行项 TestNoRealSecretsInRepo 因缺 .local.keys 显式 SKIP，如实标注不计 PASS。
