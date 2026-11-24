# t228 R29 修复后复查：结算结果页真实点击时间线入口

- 任务：t228 / attempt_id=515cbc74-2070-4d16-a2b4-a427e08f5540，执行：qa-e2e-wb
- 环境与口径：真浏览器 agent-browser（Chromium），viewport 390×844，非物理真机；后端与 H5 均为本地测试环境。
- 目的：验证 t227 新增的结算 sheet 时间线入口（.r29-result-timeline-cta）在结算结果页真实可见、真实 CLI 点击，点击后打开完整反应时间线抽屉，逐字段与服务端权威 BattleDetail 对账，不污染结算状态。
- 范围：QA 只读复查，未修改业务源码、未 git commit/push。

## 一、七项验收总表

| 项 | 门禁 | 结论 | 关键证据 |
|---|---|---|---|
| 1 | 真实 CLI click 结算页入口，入口可见在 sheet 内 | 通过 | result-cta-geometry.json：exists=true/文本=查看完整反应时间线 →/inSheet=true/box{l:12,t:560,w:365,h:35}/elementFromPoint命中CTA自身/isCta=true/pointer-events:auto；result-cta-click.txt=✓ Done；截图 r29p-01 |
| 2 | 点击后抽屉 open/z=40/完整行数/help=false | 通过 | result-open-probe.json：state=SETTLED/timelineOpen=true/rowCount=69/helpVisible=false/result=通关；result-open-geometry.json：z=40/box{l:6,t:69,w:378,h:706}/69行右侧=371无溢出；截图 r29p-02 |
| 3 | 关闭回结果页，保持 settled 不恢复战斗 | 通过 | result-close-click.txt=✓ Done；result-closed-probe.json：state=SETTLED/timelineOpen=false/rowCount=0/helpVisible=false/result=通关；截图 r29p-03 |
| 4 | DOM/API reaction_timeline 逐字段对账 + settle200/hash | 通过 | reconcile-full.sh：ROW_COUNT=69/DETAIL_TOTAL=69/TRUNCATED=false/MISMATCHES=0/FULL_RECONCILE_OK=true；final.json：result=通关/hash=回放哈希 9205011be80233e8（可验真）/settle=[{status:200,battle_id:478}] |
| 5 | 390×844 无文档/抽屉横向溢出、R24/R26/R27/R28 共存 | 通过（含范围外backlog） | result-open-geometry.json：docOverflow=false/69行 right=371,w=sw=cw=351/hasUndefined=false/hasNaN=false；result-open-probe.json：hasR24=true/hasR27=true/hasR28=true，hasR26=false 属结算态正常（见四）；既有 R28 feed 6px 基线溢出列 backlog |
| 6 | ≥4 张本轮新截图，sha 两两不同、亲读非登录页 | 通过 | 4张新图 390×844 sha 见第六节，均亲读为结算页/抽屉，非登录页 |
| 7 | 零真值清理、QA 还原、不改业务源码不 git | 通过 | auth-init.js 已删(AUTH_DELETED=YES)、模板仅__LYP_TOKEN__占位符、data/ 与 auth-init.js 被 .gitignore 覆盖；QA 库 pg_restore 后 records=104/max_id=477 基线；git 业务源码改动均为 t222-t227 他人实施，本 QA 未改未提交 |

## 二、结算页真实点击证据（队长关键质疑）

1. **入口存在且可见**：结算 sheet 中模板位于 battle.vue:443-444，逐字『查看完整反应时间线 →』，@click.stop=openReactionTimeline。CSS battle.vue:2831 pointer-events:auto。result-cta-geometry.json 证明元素存在、文本正确、位于 sheet 容器内，computed display=block/visibility=visible/pointerEvents=auto，getBoundingClientRect l=12,t=560,w=365,h=35（390×844 视口内），elementFromPoint 命中 uni-view.r29-result-timeline-cta 自身（isCta=true）——真实用户可点。
2. **真实 CLI 点击**：E2E 脚本 settled-full.sh 使用 agent-browser --session lyp-r29-postfix click '.r29-result-timeline-cta'，result-cta-click.txt=✓ Done（CLI 输出）。点击**之前**截 r29p-01-result-entry.png，亲读可见结算结果页『通关』分数/AI复盘/回放哈希/『查看完整反应时间线 →』入口。点击**之后**探针 timelineOpen=true/rowCount=69，亲读 r29p-02 为 69 行时间线抽屉。
3. **关闭再开**：CLI click '.r29-timeline-close' 后 result-closed-probe.json state=SETTLED/timelineOpen=false/result=通关；之后再次 CLI click 入口（reopen-click.txt=✓ Done）reopen-probe.json timelineOpen=true/rowCount=69/helpVisible=false，行为可重复。
4. **与 t226 的区别**：t226 结算态用 DOM e.click('.r28-feed-cta')（结果 sheet 遮挡，CLI 点击报 covered），本轮改为 t227 结果页新入口且全程真实 CLI click，DOM eval 仅作几何与内容量测，不作最终证据。

## 三、服务端权威对账（69 行逐字段）

| 项 | 值 |
|---|---|
| battle_id | 478 |
| settle 状态 | 200 |
| 回放哈希 | 回放哈希 9205011be80233e8（可验真） |
| BattleDetail 字段 | id,level_id,result,stars,score,kills,leaked,hp_left,wave_reached,duration_ms,shots,hits,reactions,heat_max,elements_used,reactions_used,terrain_used,reaction_timeline,loot,replay_hash,created_at |
| reaction_timeline | total=69 / events=69 / truncated=false |
| DOM 行数 | 69 |
| 逐字段比对 | wave / tick / reaction / enemy_uid / damage 全匹配，MISMATCHES=0 |
| 末 3 行 DOM | 第5波 · 1332 · 蒸汽爆发 · #117 · 228.0 伤害；第5波 · 1361 · 过热 · #117 · 283.0 伤害；第5波 · 1376 · 蒸汽爆发 · #117 · 444.0 伤害 |
| 末 3 行 API | {wave:5,tick:1332,reaction:steam_burst,enemy_uid:117,damage:228}；{5,1361,overheat,117,283}；{5,1376,steam_burst,117,444} |

反应名映射与 t226 一致：steam_burst=蒸汽爆发、overheat=过热、superconduct=超导。DOM 文本逐行由 data-r29-index/wave/enemy 与权威 events 下标一一对应；截断标记为 false，无空态编造。

## 四、抽屉 UI / 暂停与互斥 / 滚动

- 抽屉标题『AI · 完整反应时间线』、结论『本局显示 69 条逐次反应记录』、依据『依据：引擎 reactionTimeline() 只读旁路（前 200 条，不进 replay_hash）；反应名以服务端 config.reactions 为准』、诚实红线『辅助展示非权威判据，权威以 replay_hash 为准』，逐字正确。
- 互斥：result-open-probe.json helpVisible=false，result-open-geometry.json legend=false，抽屉 z-index=40 高于图例/暂停层 z=30（battle.vue:1861/2061），结算态图例不会同时出现。
- 暂停所有权：结算态 pauseLabels=[]，关闭后 state=SETTLED，未误恢复战斗；战斗态手动暂停/自动暂停的互斥归属沿用 t226 的 own-* 十快照验证（own-1..own-9），本轮未改该逻辑。
- 滚动：抽屉正文使用 uni-scroll-view（sh=1562/ch=591），scroll3.json after=971/lastVisible=true，r29p-05 亲读末行『第5波 · 1376 · 蒸汽爆发 · #117 · 444.0 伤害』完整可见，顶部与底部均可滚动；.r29-timeline-scroll 为 uni 包装层(sh=ch=591)不是真正滚动容器，已按真实可滚元素量测。
- 溢出：抽屉 69 行 right=371 < 390，w=sw=cw=351，document.scrollWidth<=clientWidth+1；hasUndefined=false、hasNaN=false。R24/R27/R28 共存：hasR24=true/hasR27=true/hasR28=true；hasR26=false 属**结算态正常**（R26 敌情简报只在选卡期显示，t226 main E2E CARD_PHASE 帧 hasR26=5 已证共存）。
- 范围外 backlog（非本轮引入）：R28 既有 .reaction-line 6px 基线溢出（HEAD 基线既有，git show 已核），本 QA 不修、不混入本轮结论。

## 五、测试与探针

- 契约复跑：pnpm vitest run --config vitest.config.ts src/pages/battle/battle_r29_live_reaction_timeline.contract.test.ts src/pages/battle/battle.test.ts → 2 files / 77 tests ALL PASSED（23:19:39，5.44s）。
- 注入探针 auth-init.tpl.js 仅 __LYP_TOKEN__ 占位符；window.uni.request 劫持收集 settleResponses/hashResponses/pageErrors，过滤 Autoplay 噪声（The play() request was interrupted by a call to pause()）后 peReal=0，ce=0、af=0、fr=0。
- 零真值：清理后 tools.grep 正则扫描本轮目录 *.js 返回 0 命中；含真 JWT 的 auth-init.js 已删除；.gitignore 新增 data/ 与 auth-init.js 两条且 git check-ignore 命中；QA 库 pg_restore 后 battle_records=104/max_id=477（与 t226 基线一致）。

## 六、截图清单（本轮新图，sha256，均 390×844 亲读非登录页）

| 图 | sha256 | 内容 |
|---|---|---|
| r29p-01-result-entry.png | 96ef722d4e37c7b308af7421a2d14f539a0427b25355317d3025e7a627cf11db | 结算结果页，『查看完整反应时间线 →』入口可见 |
| r29p-02-result-drawer.png | ee4fc6039162ad347122f0ac383080016f2481dbcfb2c9182a1fecf8bfc91115 | 点击后抽屉打开，69 行时间线顶部 |
| r29p-03-result-closed.png | 456f1209fc5b01355fe9f70ecabedd3b63483874b7f5fd43b4c805fa43a720d0 | 关闭抽屉后回到结算 sheet |
| r29p-05-scroll-bottom.png | 0b0a93dbda9fe4ebe949fe5b035a6b81d5aacce8729acd50391e058cf1b1c7bf | 抽屉滚动到底，末行『第5波 · 1376 · 蒸汽爆发 · #117 · 444.0 伤害』可见 |

四张 sha 两两不同；曾生成 r29p-04-scroll-last.png 与 02 完全相同（重复图）已删除，不列入证据。未复用 t226 旧图。

## 七、结论

t228 七项门禁全部通过：结算结果页时间线入口真实可见、用户可用真实 CLI 点击、打开完整 69 行抽屉、与服务端 reaction_timeline 逐字段零差异、关闭不误恢复战斗、无横向溢出/undefined/NaN、截图与清理合规。**PASS**。范围外 backlog：R28 既有 .reaction-line 6px 基线溢出（非本轮引入，已在前轮登记，不在 scope）。
