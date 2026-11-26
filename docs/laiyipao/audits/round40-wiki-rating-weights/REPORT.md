# R40 复查报告：Wiki 评分权重真浏览器三视口对账

- 任务：t27（laiyipao-r36）· 日期：2026-11-25 · 执行：same-route-qa
- 被审对象：laiyipao/admin Wiki 视图的 rating_weights 评分机制区（构筑评分 I-7）
- 权威数据源：公开接口 GET /api/v1/config（快照：data/config.json）
- 总体结论：**FAIL（有阻塞观察）/ 主体功能 PASS**。正常服务端数据的评分表、AI 四要素、CTA 与三视口布局均通过；任务验收中的「搜索命中评分维度并高亮」在本轮真实浏览器里不成立，不得记为通过。
- 重要边界：本任务为 QA，不改任何业务源码；下述失败项是如实上报，不是本轮修复范围。

## 1. 环境与方法

- 被测地址：http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating（Vite dev，base /laiyipao-admin/）；API http://127.0.0.1:8080，QA 库 PostgreSQL 容器 lyp-qa-pg，端口 15440。API 与 Vite 进程由后台任务 bash-67d294875df84485 / bash-a96ff9cc9d5f437c 启动，收尾时已 kill。
- 真实 Chromium（agent-browser 0.41.1 CLI）三视口：390×844、768×1024、1280×800。切 tab、点 CTA、键入搜索词、点搜索结果、截图均为 CLI 真实交互，非 DOM 注入模拟。DOM 细项用 agent-browser eval 读取。
- 鉴权：mint-token.sh 只从环境变量 QA_ADMIN_USER / QA_ADMIN_PASS 读凭据，POST /api/v1/admin/login 后以 data/admin-auth-init.js 注入 localStorage；脚本与报告均不含字面口令。本轮最终环境中两变量未设置，鉴权通过复用此前已生成的会话产物完成；生成脚本的凭据卫生已静态核对。收尾已删除 data/admin-auth-init.js、data/login.json 及根目录副本（finalize.sh 输出 AUTH_DELETED=YES，三路径均复核 ABSENT）。
- 未登录深链探针：未带 token 打开 /wiki?tab=rating 会跳转 #/admin/login?redirect=/wiki?tab=rating，hasRating=false；这是路由守卫行为，不是评分表缺陷。

## 2. 权威配置快照与三视口逐格对账

config.json sha256=4de1ceb05d2bf166e541279dba32e76d9e4dbcb085d744aa39368a6220dcf4d5，version=6237e0b623219b8b。rating_weights 五键逐字为 element_coverage=12、reaction_coverage=15、mastery_done=10、equipment_synergy=12、mechanic_depth=8。配置其它键数：skills 24、enemies 22、levels 100、reactions 7、composite_skills 18、recipes 18、skins 6、drop_rates 6、mastery_families 8（8×12=96 节点）、gems 8、gem_qualities 5。

对账器 docs/laiyipao/audits/round40-wiki-rating-weights/reconcile.py 从仓库根执行（D 为仓库相对路径，若在审计目录执行会 FileNotFoundError），输出：

```
EXPECTED_ROWS=[["元素覆盖","12"],["反应链","15"],["专精投入","10"],["装备契合","12"],["机制深度","8"]]
EXPECTED_AI=权重 5/5 项齐备
VIEWPORT_390 rows=5 mismatches=0 scroll={"sw":324,"cw":324,"before":0,"after":0,"ox":"auto"} ok=true
VIEWPORT_768 rows=5 mismatches=0 scroll={"sw":702,"cw":702,"before":0,"after":0,"ox":"auto"} ok=true
VIEWPORT_1280 rows=5 mismatches=0 scroll={"sw":884,"cw":884,"before":0,"after":0,"ox":"auto"} ok=true
REGRESSION reaction=7 skills=24 composite=18 recipe=18 enemies=22 levels=20 gems=8 mastery=96 skins=6 drop=6
PROBLEMS=["reg:basicsMissingRatingSection","reg:composite=36"]
ALL_RECONCILE_OK=False
```

- 三视口 DOM 探针（rating-probe.js → data/probe-390.json / probe-768.json / probe-1280.json）：hash 均为 #/wiki?tab=rating，sectionVisible=true，目录含「构筑评分」，表头逐字 [维度,权重]，五行中文标签与整数权重逐格一致；AI 结论含「权重 5/5 项齐备」，basis 含「公开 /api/v1/config」，honesty 含 replay_hash，CTA 文案「查看构筑评分 →」可见，href=#/wiki?tab=rating；secHasPct=false、bad=false（无 NaN/undefined/[object Object]、无 %/‰），docW==innerW（无页面级横向溢出）。
- 横向滚动：两列表格在本数据下 sw=cw，不需要滚动；ox=auto 说明容器只在更窄内容时才可滚，不存在伪滚动或页面溢出。因此「sw>cw 且可滚到末列」这一条在本 R40 两列表上未触发，属实测不适用而非失败。
- 两个 PROBLEMS 是探针脚本假阳性，不是应用缺陷，已逐项复核：
  1. reg-basicsMissingRatingSection：reg-basics.json 打开 #/wiki（默认 basics）时评分区已可见，ratingVisible=true 且五行数据正确；脚本只在 section 的 class 中找 r40-rating-mechanics，但该区块此时用普通 wiki-section 包裹，故误报。
  2. reg-composite=36：reg-composite.json 的三个表分别为技能 24 行、合成 18 行、配方 18 行，页面文本也明确「合成技能」「合成配方」；cp.compositeRows=36 是探针的 #composite 选择器与另一路径重叠重复计数造成的伪影。权威回归行数以此处显式表计数 18/18 为准。
- 因上述两个探针伪影，reconcile.py 的 ALL_RECONCILE_OK=False，本报告不篡改脚本输出、不声称其为真；人工复核后的真实回归结论见第 5 节。

## 3. AI 四要素与 CTA 真实点击

- AI 四要素同屏：数据来自公开 cfg.rating_weights 的确定性聚合（buildRatingMechanicsAi），不依赖焦点行；逐字常量 RATING_AI_BASIS/RATING_AI_HONESTY/RATING_AI_CTA 在 .r40-ai-note 中可检出。实现位置：laiyipao/admin/src/ratingMechanicsText.ts（RATING_KEYS 五键、ratingLabel/ratingValue/buildRatingMechanicsAi 与三条常量）与 laiyipao/admin/src/views/WikiView.vue:972-992（section .r40-rating-mechanics、表 .r40-rating-table-scroll、AI note .r40-ai-note、CTA 使用 urlFor('rating') + goTo({tab:'rating'})，无裸 hash）。
- CTA 点击：cta-click.sh 真实 CLI 点击 .r40-ai-note a，data/cta-click.txt 输出 ✓ Done。点击前 data/cta-pre.json：hash=#/wiki、CTA 可见、href=#/wiki?tab=rating、activeNav=[核心规则]；点击后 data/cta-post.json：hash 与 hashAfter 均为 #/wiki?tab=rating，onWikiPath=true，dashboardJump=false，ratingVisible=true 且 ratingRows=5。CTA 未跳看板。截图：screenshots/r40-04-cta-before.png、r40-05-cta-after.png（两者 sha 不同，见第 7 节）。

## 4. 搜索命中与行高亮（阻塞观察）

- search-click.sh 以真实 CLI 在搜索框输入「反应链」与「mechanic_depth」后，data/search-fanyinglian.json / data/search-mechanic_depth.json：searchPanelVisible=true，hits=[]，emptyText=「没有匹配…」；随后点击 .search-hit 的 data/search-click-*.txt 均为 ✗ Element not found。searchpick 截图与搜索前截图字节一致（无高亮变化）。
- 原因（只读源码定位）：laiyipao/admin/src/views/WikiView.vue:462-589 的 searchIndex 覆盖 levels/enemies/skills/composite/equipment/reactions/gems/qualities/recipes/skins/mastery，但没有把 R40 评分表的五行纳入索引；:594-603 仅按 title/meta/body 匹配。因此这两个评分维度词在 Wiki 搜索里没有命中项，自然也没有点击高亮。
- 任务验收原本要求「输入 反应链 / mechanic_depth 应命中并可点击高亮」，本轮实测失败。因 QA 禁改业务源码，本轮不修复；作为 FAIL/阻塞项上报给队长决定是否转入新任务。此项必须与「主体评分表 PASS」分开陈述，不得合并成全绿。

## 5. 回归（R31–R39）与静态检查

- reg-tabs.sh 通过真实 CLI 切 tab 产生 data/reg-*.json 与 data/reg-click-*.txt（各点击 ✓ Done）：basics 反应 7 行且腐蚀扩散/破甲语义存在、评分区可见且五行正确；skills 24；composite 18；recipe 18；enemies 22；levels 20；gems 8（品质 5）；mastery 8×12=96；skins 6（首行游猎者）；rules drop 6。所有 tab docW==innerW，bad=false。截屏 screenshots/r40-08-reg-*.png。
- R40 契约测试：cd laiyipao/admin && pnpm exec vitest run tests/WikiRatingMechanics.test.ts --reporter=dot → 1 file / 10 tests passed，exit=0。
- R40 + WikiView 定向：pnpm exec vitest run tests/WikiRatingMechanics.test.ts tests/WikiView.test.ts --reporter=dot → 2 files / 37 tests passed（重定向记录 VITEST_RC=0）。
- admin 全量：pnpm exec vitest run --testTimeout=90000 --reporter=dot → 79 files / 877 tests passed，exit=0；stderr 仅 Vue Router warning 与 jsdom navigation 提示，无失败。
- pnpm run type-check → exit=0；pnpm run build → exit=0（built in 1.45s）。
- Go 守卫：cd laiyipao/server && go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo' -count=1 -v → exit=0；TestNoInfraIdentifiersInRepo PASS(3.47s)；TestNoRealSecretsInRepo **SKIP**，原话：读不到 laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」。本轮无 .local.keys，故该守卫是 SKIP 不是 PASS，如实记录。
- 静态秘密扫描：审计目录 grep 字面 JWT/Bearer/postgres 带密码连接串/C:\Users 模式零命中；mint-token.sh 仅环境变量读取。
- 令牌锚点：laiyipao/admin/src/assets/tokens.css 的 --lyp-token-version: lyp-tokens-v1、--lyp-r3-version: lyp-tokens-v3-r3、--lyp-r6-version: lyp-tokens-r6 保留，未新增 --r40-。

## 6. 截图证据与真实可见性

- 初始三视口截图 data/probe-*.json 对应的 screenshots/r40-390-rating.png / r40-768-rating.png / r40-1280-rating.png 由 CLI 生成；为确认评分区在屏内可见，rating-shot.sh 用真实 CLI scrollintoview .r40-rating-mechanics 后重新截图：screenshots/r40-390-rating-visible.png（390×844）、r40-768-rating-visible.png（768×1024）、r40-1280-rating-visible.png（1280×800）。data/shot-390.json / shot-768.json / shot-1280.json 均 ok=true、rows=5、hash=#/wiki?tab=rating、docW=innerW=对应视口宽。图像预览确认三张 visible 图均能肉眼看到评分表五行和 AI note。
- 共 21 张 PNG，18 个不同 sha256。重复对：r40-1280-rating.png == r40-04-cta-before.png（d25b3f36…）；r40-06-search-fanyinglian.png == r40-07-searchpick-fanyinglian.png（ea888e90…）；r40-06-search-mechanic_depth.png == r40-07-searchpick-mechanic_depth.png（3b24ffea…）。后两对说明点击无高亮变化，与第 4 节失败一致，不隐瞒。

### 截图 sha256 清单（screenshots/）

| 文件 | sha256 |
|---|---|
| r40-390-rating.png | f5a5d0030964c8eb7e5ae6aaf1994614341d39caf9d9adf94ce04f7a3a34dcee |
| r40-768-rating.png | 83562922f59adb47fcd92fdab4f743b313a1354b79c58eacb972d994e8c2be10 |
| r40-1280-rating.png | d25b3f3642d81f3013d72c60b905d0fc3f544ec8f419dc73d414d40c546dc100 |
| r40-04-cta-before.png | d25b3f3642d81f3013d72c60b905d0fc3f544ec8f419dc73d414d40c546dc100 |
| r40-05-cta-after.png | 8b9bfd97c58faf50dc16fe5c15e221a8dec2558753209d3a508c5a1fa25d842c |
| r40-06-search-fanyinglian.png | ea888e90214ceaf3c607b2c5c4c99fe5cfc4a6af40df329bb4d423a284e3adab |
| r40-07-searchpick-fanyinglian.png | ea888e90214ceaf3c607b2c5c4c99fe5cfc4a6af40df329bb4d423a284e3adab |
| r40-06-search-mechanic_depth.png | 3b24ffea3895cffcfe6a8cb5ce914f008736e2892787799900e95edf6ea90107 |
| r40-07-searchpick-mechanic_depth.png | 3b24ffea3895cffcfe6a8cb5ce914f008736e2892787799900e95edf6ea90107 |
| r40-08-reg-basics.png | 0db7d33d45814700f87eb84020a3d6b4ea1155262aa6fc3a3e2416583f2c2aa2 |
| r40-08-reg-skills.png | 889f28fc9041f987d49e95445883d2b99a1174a364438bab79d923fbd821e7dd |
| r40-08-reg-composite.png | 65b7db3b4ec2fee01bcc6d266c8dc79f8071dc7dc0e70a635ef1c611ebd90198 |
| r40-08-reg-enemies.png | e901c522564d3168d920ac3d935add7d4bde8be363db9fa1479f84182a5b11c7 |
| r40-08-reg-levels.png | bca88f6016e74e6eda5495845741e52d32903df6ee2c6a55ce5eb47663c17858 |
| r40-08-reg-gems.png | 2500550050f930e3f8254a5cc7548d779cf766e98493cd5c2d2b3242a24bef60 |
| r40-08-reg-mastery.png | 654c7fbc6358107f4e2865fc2cb068b4de7bfc39f440994b61631bc08689e646 |
| r40-08-reg-skins.png | a0ca9af3b181815a1c195bb0c3f2b75bf87684070d27cf017c329726247a690a |
| r40-08-reg-rules.png | 61e08e0cec873528bacb123a64157190a9173ec1d97bf91284a02b5ddbcea2ad |
| r40-390-rating-visible.png | b6ce1d7a88e4c76580a7f9b28ef38e6fa40a837c3798bfcbb3eef41609ec6be8 |
| r40-768-rating-visible.png | 561e03e83e8b3fd24bc65e2b0bb9adfd2437378f14f03d7383cde076c19925e1 |
| r40-1280-rating-visible.png | 9f4b2c6f7d3f24a83c69345633106c07403f86d72a1c5a41d41f5e04fe6892ac |

## 7. 收尾与环境状态

- finalize.sh 首次执行发现缺陷：data/qa-before.dump 是 pg_dump 纯 SQL 文本（首行 -- PostgreSQL database dump，pg_restore 报 input file appears to be a text format dump），脚本却用 pg_restore 且隐藏 stderr，导致 TRUNCATE 后未恢复，DB_CHECK=0_0。这不是业务缺陷，是本轮审计脚本问题，必须如实记录。
- 已修复数据状态（不改 finalize.sh 文件本身，避免伪造已通过脚本）：从 qa-before.dump 提取 public.battle_records 与 public.replay_verifications 的 COPY 段，经 docker exec -i lyp-qa-pg psql ... 重新导入；dashboard_battle_stats 在库中已有 55 行聚合数据、dump 中同键会重复，故未覆盖该聚合表。最终复核：battle_records count=104、max(id)=477；replay_verifications 16 行；dashboard_battle_stats 55 行。任务要求的 QA 库基线 104/477 已恢复。
- 临时鉴权产物已删除（见第 1 节），API/Vite 后台任务已 kill（job_list 显示两者 status=killed）。
- 未执行任何 git 写操作，未修改业务源码。新增/改动仅在本审计目录（报告、脚本、探针数据、截图）。

## 8. 最终判定与交接

- PASS 部分：权威五键与默认权重三视口逐格一致（mismatches=0）；AI 四要素同屏且确定性；CTA 真实点击保持 #/wiki?tab=rating 不跳看板；R31–R39 回归行数一致；R40 契约 10/10、定向 37/37、admin 全量 877/877、type-check/build 通过；截图 21 张（18 个不同 sha）满足 ≥4 张新增非登录页的要求；凭据与 QA 库收尾完成。
- FAIL / 阻塞部分：Wiki 搜索不索引评分五行，输入「反应链」或「mechanic_depth」无命中、无可点击高亮，违反本任务验收项；reconcile.py ALL_RECONCILE_OK=False（两个探针伪影，已人工复核说明）；TestNoRealSecretsInRepo=SKIP 不是 PASS。
- 建议：若队长要求评分维度可被搜索命中并高亮，应另开实施任务把 rating 五行纳入 WikiView.vue searchIndex 与 pick/highlight 链路；本 QA 任务不改业务源码，故以 FAIL 上报该项。
- 诚实边界：本报告只陈述本轮真实浏览器与命令实测；未对未执行的检查给出通过结论。
