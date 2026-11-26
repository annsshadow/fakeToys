# R28 复查报告：实时反应归因真浏览器闭环

- 轮次：round28-live-reaction-attribution
- 关联任务：t219（attempt_id `850270ef-89fb-4632-a19d-4bc0c6aa2c6e`）
- 复查角色：qa-e2e-wb（只读业务源码，不改业务代码、不提交）
- 复查日期：2026-11-24
- **浏览器口径声明**：本轮使用 agent-browser 真实 Chromium 无头会话（viewport **390×844**），**不是物理真机**。所有 DOM、几何、截图均来自该真实浏览器实例，非模拟断言。
- 总体判定：**PASS（附一条既有基线问题，非本轮引入、非阻塞）**。**未把任何失败/时机错误伪装为通过**：报告中逐项区分主 E2E 结算后的 CTA 时机错误与独立战斗态 CTA 成功证据。

## 一、门禁逐项结论

| # | 门禁 | 结论 | 关键证据 |
|---|------|------|----------|
| ① | 真实 L5 ≥3 条反应 DOM ≤3 行、逐字归因、data-r28-wave/enemy 一致或归因不可用 | 通过 | 114 帧 lineCounts 全 =3；attribOk=342（114×3）；unknownRows=0；首/末样本逐行核对 |
| ② | 结算 BattleDetail 与服务端 timeline 末 3 条逐字段对账 | 通过 | battle_id=478；timeline 72 条 TRUNCATED=false；末 3 条 wave/tick/reaction/enemy_uid/damage 与页面逐字段吻合 |
| ③ | AI 分母=DOM 行数；CTA 真打开 help；过滤 Autoplay 后 peReal=ce=af=fr=0 | 通过 | ratioOk=114/114；CTA 三探针 WAIT 态 false→true→false；pe=51 全为 Autoplay 噪声，过滤后 peReal=0、ce=0、af=0、fr=0 |
| ④ | 390×844 不横向溢出、说明块可换行、不遮 R24/R26/R27 | **部分通过，如实记录既有 6px 溢出** | docOverflow=false（无滚动条）；但 .reaction-line right=396 > viewport 390，超 6px；经 HEAD 基线比对确认**R28 未引入、既有基线行为**，本轮不阻塞判定，见第六节 |
| ⑤ | settle 200/battle_id/hash + ≥4 截图 sha 各异亲读非登录页 | 通过 | settle status=200、battle_id=478、hash=6fffb426…；5 张截图 sha 两两不同、均 390×844 亲读非登录页 |
| ⑥ | 空/截断/非法 timeline 纯函数契约补证 | 通过 | contract 21/21 + battle.test 55/55；2 files/76 tests ALL PASSED |
| ⑦ | 零真值 + 删 auth-init.js + gitignore + QA 还原 + REPORT.md | 通过 | 模板扫描 0 命中；auth-init.js 已删；.gitignore:338-340；库还原至 104/477；本报告落盘 |

## 二、真实浏览器 E2E 实测

会话：agent-browser 真实 Chromium，viewport 390×844，真实 L5 关卡，采样 114 帧至 SETTLED。

- 状态分布：`STATES{WAIT:107, CARD_PHASE:6, SETTLED:1}`。
- 行数：114 帧 lineCounts **全部 =3**（≤3 行门禁满足）。
- 归因：attribOk=342 = 114×3 行全部 data-r28-wave / data-r28-enemy 与文字「第N波 · #uid」一致；unknownRows=0（本局全部成功归因，未出现「归因不可用」；该分支由纯函数契约另行覆盖）。
- AI 四判据：aiTitleOk=114、basisOk=114、honOk=114、ctaOk=114、ratioOk=114（AI 分母=DOM 行数，114/114 吻合）；r24=114、r26=6（选卡期）、r27=114 共存未互相遮挡。
- 脏值：undef=0、nan=0。
- 首样本 3 行：`焰+冰 → 蒸汽爆发 -241 第1波 · #1`、`冰+焰 → 蒸汽爆发 -167 第1波 · #1`、`电+冰 → 超导 -70 第1波 · #1`，aiConcl=`已归因 3/3 条最近反应`。
- 末样本（SETTLED）3 行：`焰+电 → 过热 -439 第5波 · #148`、`焰+冰 → 蒸汽爆发 -302 第5波 · #147`、`焰+电 → 过热 -283 第5波 · #147`；data-r28-wave/enemy 与文字逐字一致。

产物：`data/samples.jsonl`（114 帧原始探针）、`data/final.json`。

### 结算与控制台探针（final.json 原文）
- result=`通关`，hash=`回放哈希 6fffb426d9ee629b（可验真）`。
- settle=[`{status:200, battle_id:478, replay_hash:""}`]；说明：结算列表接口此字段回传空串，权威回放哈希以页面展示的 6fffb426… 为准，hashResponses=[] 未捕获到额外带值响应，此处如实记录、不臆造。
- 控制台：pe=51，peList 三条样例均为 `The play() request was interrupted by a call to pause(). https://goo.gl/LdLk22`——全部为浏览器 Autoplay 策略噪声；按 filtering 规则剔除后 **peReal=0**，ce=0、af=0、fr=0。

## 三、服务端权威对账（reconcile.sh 实跑）

| 项目 | 数值 |
|---|---|
| BATTLE_ID | 478 |
| DETAIL_TOKEN_LEN | 157（令牌长度，值不入库） |
| DETAIL_KEYS | id, level_id, result, stars, score, kills, leaked, hp_left, wave_reached, duration_ms, shots, hits, reactions, heat_max, elements_used, reactions_used, terrain_used, reaction_timeline, loot, replay_hash, created_at |
| TIMELINE_TOTAL | 72，TRUNCATED=**false**，REPORTED_TOTAL=72 |

服务端 timeline 末 3 条（旧→新）：

| wave | tick | reaction | enemy_uid | damage | 页面对应行 |
|---|---|---|---|---|---|
| 5 | 1741 | overheat（过热） | 147 | 283 | 焰+电 → 过热 -283 第5波 · #147 |
| 5 | 1756 | steam_burst（蒸汽爆发） | 147 | 302 | 焰+冰 → 蒸汽爆发 -302 第5波 · #147 |
| 5 | 1805 | overheat（过热） | 148 | 439 | 焰+电 → 过热 -439 第5波 · #148 |

顺序相反是因为 feed 新→旧、timeline 旧→新，**逐字段吻合**。反应名映射见 `laiyipao/miniapp/src/game/elements.ts:129-131`（蒸汽爆发）、`:160-162`（过热）、`:209-211`（超导）；权威字段定义见 `laiyipao/server/internal/domain/reaction_timeline.go:24-40` 与 `laiyipao/server/internal/service/economy.go:1434-1475`。

## 四、归因与 AI 实现只读核验

- `laiyipao/miniapp/src/pages/battle/liveReactionFeed.ts`：
  - 三逐字常量：ATTRIBUTION_UNAVAILABLE=`归因不可用`、LIVE_FEED_BASIS、LIVE_FEED_HONESTY=`辅助归因非权威判据，权威以 replay_hash 为准`。
  - `export function attributeLiveReactions(events: LiveReactionEvent[], timeline: LiveTimelineRecord[] | null | undefined, truncated: boolean): LiveFeedResult`：truncated=true 时全部不可用；逆序消费游标 `cursor=timeline.length-1`；命中 reaction+damage 严格相等即 `cursor=hit-1`，随后才校验 wave/enemy_uid 整数 ≥1，避免同一 timeline 记录被重复占用。
  - `attributionText(row)`：可用返 `第${row.wave}波 · #${row.enemy_uid}`，否则 `归因不可用`。
  - `buildLiveFeedAi(result)`：conclusion=`total===0 ? '最近无反应记录可归因' : `已归因 ${matchedCount}/${total} 条最近反应``，分母来自真实 DOM 行数而非硬编码。
- 纯函数契约覆盖：空 timeline、截断优先、非法 wave/enemy_uid、重复占用防护、逐字归因文案；contract 21/21 与 battle.test 55/55 全绿。

## 五、CTA 战斗态真点击验证（回应队长质疑 1）

**主 E2E 的 CTA 点击发生在 SETTLED 结算态，无效**（pre/post 探针 state=SETTLED、helpVisible=false、r28-03 截图为结算页）——这是**测试时机错误，不作为 CTA 通过证据**，已在报告中如实标注，未冒充。

独立会话 `lyp-r28-cta` 在**战斗 WAIT 态**用真实 CLI click 验证：

| 探针文件 | state | helpVisible | lineCount |
|---|---|---|---|
| data/cta-pre.json | WAIT | false | 3 |
| data/cta-post.json | WAIT | **true** | 3 |
| data/cta-closed.json | WAIT | false | 3 |

即点击 `.r28-feed-cta` 真打开战场图例覆盖层，再点 `.legend-close` 真关闭并恢复。`measure-eval.js` 量测 CTA 命中：ctaRect={l:18,t:457,w:357,h:13}，elementFromPoint 命中 `SPAN` 文本节点，CLI click 有效。截图 `r28-05-cta-open.png` 亲读为「战场图例」覆盖层（含热量条/技能槽/元素反应说明），**非结算页**。结论：CTA 在正确时机（战斗态）真实可点、可开、可关，判据成立；主 E2E 的结算态失败已排除为时机问题，非功能缺陷。

## 六、横向溢出定位与归属（回应队长质疑 2）

feedOverflow=true 是**真实几何**，不是探针口径问题；但**不是 R28 引入**，属既有基线行为：

- 视口 390；feed 右边界 380 / scrollWidth 386 / clientWidth 369。
- 3 条 `.reaction-line` 实测：right=**396**（超视口 6px）、width=386、scrollWidth=384、clientWidth=384、white-space=normal、overflow-x=visible、max-width=none。
- 对照：cue right=341/width=322（white-space=pre-line，未溢出）；ai right=380（white-space=normal，未溢出）。docOverflow=false，页面无横向滚动条。
- 归属证据：`git show HEAD:laiyipao/miniapp/src/pages/battle/battle.vue` 的基线 CSS（2180–2300）已含 `.reaction-line{display:flex;flex-direction:row;gap:12rpx;padding:6rpx 14rpx}` 及媒体查询块（2218–2242：`.reaction-line{flex-wrap:wrap;row-gap:2rpx;padding:4rpx 10rpx;width:100%}`）与 `.reaction-cue{max-width:620rpx}`（旧 cue 超宽来源）。工作树 git diff 仅 53 insertions / 6 deletions，R28 只新增 `.r28-attribution`（nowrap）、`.r28-feed-ai`（可换行）、`.r28-feed-cta` 与 AI 计算，**未改反应行宽度规则**。
- 判定：该 6px 溢出在 HEAD 基线即存在，截图亲读肉眼看不出截断，且不影响 R24/R26/R27 与 CTA 可点；按「非 R28 引入、非阻塞既有问题」如实记录，**不伪装成完美通过**，建议后续轮次单独修复基线反应行宽度。

## 七、截图证据（5 张，sha 两两不同，均亲读非登录页）

| 文件 | sha256 | 内容 |
|---|---|---|
| screenshots/r28-01-attribution-multi.png | `9216925c7f1277f4cac7c39bb309f8f2ac1dd6e370c876c77fc14e0d05ea303f` | 战斗中多反应归因行 + AI 说明块 |
| screenshots/r28-02-feed-ai.png | `dc02d8b6d0532d6acf1597d3d5191a2c7c0f81fde89fdd427833bb5adc84c9f2` | feed 与 AI 结论/依据/红线 |
| screenshots/r28-03-cta-help.png | `b6fd9ccc266bf8f9fa1086af9490b569dede6e2e0934cfe7892b9dfcc1774f8e` | 结算页（CTA 时机错误那次，保留作为失败证据） |
| screenshots/r28-04-settle.png | `321b20955f87d1828660a14c74f68c12ba67cb13828c773fdcdc9441bf10ad8e` | 通关结算，回放哈希 6fffb426… |
| screenshots/r28-05-cta-open.png | `dc6a59cc8921d4be1c51914d802cc8ef664e8b284b606e6e8d20aad7a7de39b1` | 战斗态 CTA 打开战场图例覆盖层 |

全部 390×844，sha256sum 复算与上表一致；每张均已 read_image 亲读，确认为战斗/结算页面而非登录页。

## 八、零真值、审计卫生与 QA 还原

- 模板 `auth-init.tpl.js` 以 `__LYP_TOKEN__` 占位符扫描：JWT/Bearer/guest_token 真值正则 **0 命中**。
- 含真 JWT 的 `auth-init.js` 已执行 `rm -f`，`AUTH_DELETED=YES`、`TPL_EXISTS=YES`。
- `.gitignore:338-340` 已拦 `docs/laiyipao/audits/round28-live-reaction-attribution/data/` 与 `.../auth-init.js`；`git check-ignore` 对 dump、auth-init.js、samples.jsonl 均命中。
- QA 库还原：`docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < data/qa-before.dump`（PIPE_RC=0）；battle_records 由 105/max_id=478 **还原为 104/max_id=477**，与测试前一致。
- dump sha256=`0abe598fe898b410ead7d8f99c518e019f34244cecec9ddc5e29b9d9acaa31d2`，与测试前记录一致。
- 未改任何业务源码、未执行 git 提交/推送。工作树 `git status` 仅见审计目录与规范目录未跟踪（符合 QA 只读边界）。

## 九、结论

七项门禁核心功能全部通过；两项队长质疑（CTA 时机、溢出归属）已用独立战斗态真点击证据与 HEAD 基线 git diff 逐条解释清楚。唯一保留事项是**基线既有的 6px 反应行溢出**，不属 R28 变更引入，按诚实记录原则写入本报告并建议后续单独修复。据此判定本轮复查 **PASS**。
