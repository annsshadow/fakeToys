# R29 复查报告：完整反应时间线抽屉真浏览器闭环

日期：2026-11-24。角色：qa-e2e-wb（只读复查，未改业务源码、未 git 提交）。浏览器：agent-browser 会话，viewport 390×844（非物理真机）。口径：AGENTS.md 中文、真值零入库、发现缺陷如实登记。

## 结论

PASS（附两条诚实备注）：R28 CTA 在战斗态真实 CLI 点击可打开 R29 时间线抽屉，结算态程序化点击可覆盖结果页；72 行抽屉与服务端 BattleDetail.reaction_timeline 逐字段 72/72 吻合；暂停所有权、互斥、滚动、共存、契约均通过。备注：①结算态结果 sheet 遮挡 CTA 原生点击点，真实用户在该页无可见入口，本轮用 DOM click 证明实现可用，登记 backlog；②R28 既有 reaction-feed 6px 横向溢出属 HEAD 基线行为，不在 R29 scope，登记 backlog。

## 七门禁证据表

| 门禁 | 结论 | 证据 |
|---|---|---|
| ① R28 CTA 真打开抽屉且图例不同时存在 | PASS | 独立会话 lyp-r29-cta2 在 WAIT 态真实 CLI click '.r28-feed-cta' → cta2-probe.json timelineOpen=true / rowCount=3 / helpVisible=false；cta2-geometry.json z=40、box l=6/t=330/w=378/h=183、3 行 right=371=w=351=sw=cw 无溢出；标题『AI · 完整反应时间线』，结论『本局显示 3 条逐次反应记录』，依据/诚实红线逐字正确。lyp-r29-cta 首次会话 cta-wait-pre/post/closed：WAIT 态 false→true→false，rowCount 6。早期主 E2E 结算态 CLI 点击失败（drawer-open.json open=false）系结果 sheet 遮挡的测试时机错误，不伪装成功。 |
| ② 抽屉行逐字段来自引擎 timeline、结算后对账 | PASS | 结算 battle_id=479，settle status=200；reconcile.sh 得 detail.json：reaction_timeline events=72 / truncated=false / total=72；settled-open-probe.json 抽屉 72 行。Node 逐字段聚合：rows=72、detailTotal=72、mismatches=0，末 3 条 DOM『第5波 · 1583 · 过热 · #134 · 412.0 伤害』『第5波 · 1599 · 蒸汽爆发 · #133 · 302.0 伤害』『第5波 · 1601 · 过热 · #133 · 412.0 伤害』与权威 detail 末 3 条 wave/tick/reaction(中文映射)/enemy_uid/damage 全吻合。稳定排序由纯函数 wave→tick→原序保证。 |
| ③ 打开暂停、关闭恢复、手动暂停不夺回、与图例互斥 | PASS | ownership-test.sh：own-1 手动暂停→own-2 打开抽屉 pauseLabels 仍含『继续战斗』→own-3 关闭仍暂停→own-4 手动继续后恢复；own-5 自动暂停→own-7 关闭自动恢复；own-8 抽屉开→own-9 经 R27 CTA 后 timeline=false/legend=true，抽屉与图例不并存。结算态 drawer z=40 高于 legend/pause 的 z=30。 |
| ④ 结算后可打开覆盖结果页，可滚动；空态/截断契约补证 | PASS（含 backlog 备注） | settled-full.sh 到 SETTLED 后 DOM click CTA：settled-open-probe.json state=SETTLED/timelineOpen=true/rowCount=72/result 仍在；settled-open-geometry.json open=true/z=40/result=true/legend=false；r29-09-settled-drawer.png 亲读为覆盖结果页的抽屉。滚动：uni 内部真实 scroller chain[2] scrollHeight=1630/clientHeight=591，scrollTop=1039 后第 72 行进入视口，r29-12-scroll-last.png 亲读显示末行。空/截断/非法由 15 条 R29 契约测试补证（空态『本局暂无逐次反应记录』、截断『已截断，仅显示前 200 条』、未知 key『（未知反应）』、非法过滤）。备注：结算态 CTA 原生点击点被结果 sheet 遮挡，靠 DOM click 证明打开能力，真实用户入口缺位登记 backlog。 |
| ⑤ R24/R26/R27/R28 共存，抽屉无横向溢出/undefined/NaN | PASS | samples-final.jsonl 82 帧（WAIT76/CARD_PHASE5/SETTLED1）：lineCount 全=3、hasR24=82、hasR26=5(选卡期)、hasR27=82、hasR28=82、hasUndefined=0、hasNaN=0、docOverflow=0。feedOverflow=82 为 R28 既有基线（HEAD CSS 已含超宽规则，非 R29 引入），登记 backlog 不混入本轮。抽屉行 right=371/w=351/sw=351/cw=351（viewport 390），无抽屉横向溢出。 |
| ⑥ 真实 L5 settle200、hash 正常、探针零错 | PASS | final-final.json：result=通关、hash『回放哈希 c26bff2362f616bd（可验真）』、settle=[{status:200,battle_id:480,replay_hash:""}]、hashResponses=[]、ce=0/af=0/fr=0、pe=59 全为 Autoplay 噪声『The play() request was interrupted by a call to pause()』，过滤后 peReal=0。R15 timeline 契约 2/2 与 R29 契约 15/15 全绿。 |
| ⑦ ≥4 截图 sha 各异亲读非登录页、清理与 QA 还原 | PASS | 见下方截图清单（全部 390×844、sha 两两不同、亲读为战斗/抽屉/结算非登录页）；finalize.sh：AUTH_DELETED=YES、TPL_EXISTS=YES、覆盖 rounds29 data 与 auth-init.js 的 .gitignore 条目存在且 git check-ignore 命中；QA 库 pg_restore 后 battle_records=104/max_id=477（基线一致）；零真值正则扫描 0 命中。 |

## 对账表（节选，完整 72 行逐字段 0 不符）

| # | wave | tick | reaction | enemy_uid | damage | 页面文本 |
|---|---|---|---|---|---|---|
| 0 | 1 | 86 | superconduct | 1 | 70 | 第1波 · 86 · 超导 · #1 · 70.0 伤害 |
| 1 | 1 | 91 | steam_burst | 1 | 167 | 第1波 · 91 · 蒸汽爆发 · #1 · 167.0 伤害 |
| 2 | 1 | 91 | overheat | 3 | 79 | 第1波 · 91 · 过热 · #3 · 79.0 伤害 |
| 69 | 5 | 1583 | overheat | 134 | 412 | 第5波 · 1583 · 过热 · #134 · 412.0 伤害 |
| 70 | 5 | 1599 | steam_burst | 133 | 302 | 第5波 · 1599 · 蒸汽爆发 · #133 · 302.0 伤害 |
| 71 | 5 | 1601 | overheat | 133 | 412 | 第5波 · 1601 · 过热 · #133 · 412.0 伤害 |

## 队长质疑回应

1. CTA 真实触发：早期 r29-03/r29-05 是结算态 CLI 点击失败留下的错误截图，已废弃不作为通过证据。新证据为独立 WAIT 会话真实 CLI click '.r28-feed-cta'：点击前 timelineOpen=false、点击后=true 且 rowCount 3/6、图例 false，cta-wait-diag.json elementFromPoint 命中 CTA 自身、开口 z=40；结算态通过 DOM click 得到 72 行抽屉与 z=40 覆盖结果页截图。两套 DOM+截图均在报告中如实区分点击方式，未用 shim。
2. 暂停/互斥/结算：own-* 十份 JSON 完整记录手动暂停不被夺回（own-2/3）、自动暂停关闭才恢复（own-5/7）、抽屉与图例不并存（own-9）、结算态 z=40（settled-open-geometry.json）。结算态原生点击被结果 sheet 遮挡已如实登记 backlog，不冒充真实手指点击。

## 截图清单（sha256，内容实质差异，均已亲读）

| 文件 | sha256 | 内容 |
|---|---|---|
| r29-06-drawer-live.png | 25bb3f5a6184dc29dd2888301dd218be3f52d3ddbf77009be29294290ec8f9a7 | 战斗 WAIT 态 6 行抽屉 |
| r29-09-settled-drawer.png | 211fb2650c7914430ffc5636e213b1fd225b4e8af57ace2edeadacaef6b0c4ca | 结算态 72 行抽屉覆盖结果页 |
| r29-10-drawer-conclusion.png | 2518641052c7e24b3caba3c6616542eb1d07bdba3e6bce9bb6ef59275442b682 | 战斗态 3 行抽屉含结论文本 |
| r29-12-scroll-last.png | 64f5feaae38783ac57f0b2e17707b862aa327124668e812b292dae3dc3f6f8fd | 滚动到底显示第 72 行 |
| r29-13-feed.png | ac21c3c723ec32117a27f082e41b1606347571fc4c6409a1532231750edee77a | 战斗 HUD + 三行归因 + CTA |
| r29-14-feed-ai.png | 0f1932c33f18d93e22c1abefa568112be6f094de74fe62c97c59b062fec6e750 | 同屏后帧，状态数值变化 |
| r29-15-settle.png | 02134c6144ac37080ba406c5ed011c0c1ab558acd4a3f6bcfc7e04518dc8a874 | 通关结算页 hash |

## 命令与测试

- pnpm exec vitest run 两文件（R29 契约 15 + R15 timeline 2）：2 files / 17 tests ALL PASSED。
- e2e-final.sh（agent-browser 会话 lyp-r29-final，390×844）：82 帧采到 SETTLED，final-final.json 如上。
- settled-full.sh（lyp-r29-sfull）：结算 battle_id=479，72 行抽屉与 detail 逐字段 0 不符。
- ownership-test.sh（lyp-r29-own）：own-0..own-9 十份快照。
- reconcile.sh + Node 聚合核对：见对账表。
- finalize.sh：删 auth-init.js、pg_restore 还原 QA 库、git check-ignore 命中、截图 sha256。

## 范围外 backlog

1. 结算结果 sheet 遮挡 R28 CTA，真实用户无可点击入口（功能实现可用，需产品决定是否加结果页按钮）。
2. R28 reaction-feed 在 390×844 下超出视口 6px，git show HEAD 证明基线既有，非 R29 引入。
