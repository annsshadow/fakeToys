# R26 复查报告：真浏览器选卡敌情闭环

- 任务：t201 / attempt a100f26d-1d50-44e9-89c8-88b26067a953
- 复查对象：docs/laiyipao/specs/round26-next-wave-brief/SPEC.md + H5 选卡页下一波敌情简报（.r26-wave-brief）
- 判定：**PASS（五项验收全部满足）**
- 复查方式：真浏览器 agent-browser（Chromium 390×844；**非物理真机**，属 H5 模拟视口），所有结论均来自实跑 DOM / 网络探针 / 权威端点逐字段比对，不采信实施方自述。
- 约束：QA 只读——未改业务源码、未 git 提交；仅审计目录脚本/证据落盘。

## 1. 权威数据源与口径确认

- /api/v1/config（无认证）实测 levels[5].waves = 0：配置包不下发波次，占包体 56%；store/game.ts:119-123 的 enemyMap 由 config.enemies 派生。
- 权威波次端点 GET /api/v1/levels/{id} **必须带 Bearer token**（无认证返回 unauthorized）。已存回环响应 data/level5.json（900B）。H5 5174 代理对任意 /api 路径均回 200（SPA 回退），**未当作 API 成功判据**。
- 依据行在真浏览器逐字命中：依据：当前关卡 waves[waveIndex+1].spawns 按 enemy_id 聚合；敌种名与威胁标签来自 /config.enemies。

## 2. 下一波聚合数值逐字段比对（L5，权威 vs DOM）

| 选卡帧标题 | waveIndex | 权威 /levels/5 下一波 spawns | DOM 聚合行 | AI 结论 | 判定 |
|---|---|---|---|---|---|
| 下一波 · 第 2 波 | 1 | W1: 1×5 | 游荡者×5 | 1 种敌人共 5 只 | 一致 |
| 下一波 · 第 3 波 | 2 | W2: 1×4 + 1×2 | 游荡者×6 | 1 种敌人共 6 只 | 一致（同 id 相加） |
| 下一波 · 第 4 波 | 3 | W3: 2×5 + 1×3 | 疾行者×5、游荡者×3 | 2 种敌人共 8 只 | 一致（count 降序） |
| 下一波 · 第 5 波 | 4 | W4: 2×7 + 4×4 + 3×3 | 疾行者×7、腐化犬×4、壮硕体×3 | 3 种敌人共 14 只 | 一致 |

- 主 E2E（e2e-full2.sh，level=5）跑满 110 帧到 SETTLED；data/brief-samples.jsonl 双层解析共 110 条、4 个唯一波次标题，逐帧数值与上表一致。
- 全程 unknownTags=0、briefOverflowX=false、docOverflowX=false、hasUndefined=false、hasNaN=false、cardCount=3、hasSkip=true——无横向溢出、选卡可点、无脏值。

## 3. 威胁标签专项（L22，权威 vs DOM）

- 权威 levels[22].waves[3]：3×6, 6×2；config.enemies id=6 电鳍兽 attack_range=150 → [远程]（威胁优先级 首领>飞行>潜地>远程）。
- 真浏览器第 4 波（waveNow=3/waveTotal=6）DOM：壮硕体×6、电鳍兽×2[远程]；AI 结论 下一波 2 种敌人共 8 只，其中 [远程]×2，结合本波威胁与手牌元素协同选卡。逐字段一致。
- 截图 r26-06-brief-threat-tag.png（390×844，sha c519e59db423be63e6687375a16487bbefebd5fd2dbbdea0938226437a47dd7b）亲读可见威胁标记与 AI 结论，非登录页。
- 脚手架说明：首次威胁脚本因状态判断漏 SKIP_ONLY 卡在第 4 波（**脚手架问题，非产品缺陷**），已按队长指导停止无效循环、用真实 CLI click 推进并重写为同帧检测即截图（e2e-threat-v2.sh），重跑抓到目标帧；未用 shim 掩盖。

## 4. AI 四判据与诚实红线

- 判据①可见汉字标题：全部样本 aiTitle = AI · 下一波敌情。
- 判据②真实数据结论：aiConcl 均由当帧聚合派生（见 §2/§3 表）。
- 判据③依据行：逐字 依据：当前关卡 waves[waveIndex+1].spawns 按 enemy_id 聚合；敌种名与威胁标签来自 /config.enemies，全样本命中。
- 判据④ CTA：逐字 结合本波威胁与手牌元素协同选卡，全样本命中；不承诺机制。
- 诚实红线逐字 辅助简报非权威战斗数据，实际敌情以引擎波次为准；权威回放以 replay_hash 为准，全样本命中。

## 5. replay_hash 不污染（静态 + 动态）

- **静态**：nextWaveBrief.ts 导出纯函数 buildNextWaveBrief(waves, waveIndex, enemyDefs)，仅输入数组/下标/敌种表并返回只读快照；battle.vue:803-806 的 computed 只读 eng.cfg.level.waves 与 eng.waveIndex，不调用 step()/settle()。契约测试源码守卫（battle_r26_next_wave_brief.contract.test.ts:85-108,144-156）确认无写路径、无新令牌 --r26-，三端锚点 lyp-tokens-v1/v3-r3/r6 保留。
- **动态**：主 E2E 全程选卡/跳过后正常结算 SETTLED；结算帧 result=通关、DOM 回放哈希 ec1451ab96e66b20（可验真），网络探针 settle=[{status:200,battle_id:477}]；hashResponses=[]——R26 简报自身未产生任何 replay_hash 相关写响应，结算哈希正常产出，回放链路未被污染。
- 契约测试复跑：npx vitest run ... battle_r26_next_wave_brief.contract.test.ts → **1 file / 21 tests ALL PASSED**（exit 0，339ms）。

## 6. 空态与未知敌种（不伪造线上）

- 浏览器侧：L5/L22 权威波次全部为已知敌种，线上不存在空 spawns，故**未伪造**空态/未知数据。
- 纯函数契约覆盖：battle_r26_next_wave_brief.contract.test.ts:59-73 断言未知敌种 name=#999、unknown=true、threat=（空），不编造名字/威胁；以及无 waves / 越界 / 空 spawns / 无效行 → empty=true、rows=[]；[未知敌种] 逐字文案由源码守卫确认。

## 7. 探针与错误

- 主 E2E 最终探针：ce=0、af=0、fr=0；pe=58 全部为 unhandled:The play() request was interrupted by a call to pause(). https://goo.gl/...（Autoplay 限制噪声，与 R24/R25 同口径），**过滤后真实页面错误 peReal=0**。威胁专项同样 ce/af/fr=0，pe 均为同类噪声。

## 8. 截图证据（均亲读、非登录页、sha 两两不同）

| 文件 | sha256 | 内容 |
|---|---|---|
| r26-01-brief-wave2.png | 3d4b23ffae9d970339c91f0eba313b6261592ac7ae1d9f7c4a80eac6d1072a69 | 第 2 波 游荡者×5 + AI 四判据 |
| r26-02-brief-wave4.png | 5450d5950c29538b0f9d1681d0da490e333b6d376bdd2dda2cd6c59650cfb963 | 第 4 波 疾行者×5/游荡者×3（160rpx 滚动上限内） |
| r26-03-settle.png | 856c83009afd70ffdb34435fd9b855a3cc32efa7eafbdc0815d7e6d6537e9472 | 通关结算 + 回放哈希文本 |
| r26-04-brief-scroll.png | ec75b3e822817594dbbfe9fecd41a5a3c058f1c650070799336559145e6183d7 | 第 4 波滚动后简报/手牌/AI 区 |
| r26-05-brief-threat.png | f158b31e90da6464ebe9bf4391d2dad615b2a35e672d9b898829790116b04d75 | L22 第 2 波 游荡者×4（浅跑） |
| r26-06-brief-threat-tag.png | c519e59db423be63e6687375a16487bbefebd5fd2dbbdea0938226437a47dd7b | L22 第 4 波 电鳍兽×2[远程] + AI [远程]×2 |

## 9. 零真值、环境还原与工作树

- 含真 JWT 的 auth-init.js 已删除（AUTH_DELETED=YES），仅留 auth-init.tpl.js（__LYP_TOKEN__ 占位符）；.gitignore:327-328 覆盖 R26 data/ 与 auth-init.js，git check-ignore 命中。
- 零真值正则扫描审计目录（JWT / 真实 IP / 域名 / 口令 / 个人绝对路径）：0 命中。
- QA 库已还原：data/qa-before.dump sha256=45882bd58cfc3cc6ec215cae1f092dd683b881d6e739ebbb9494517fc6b18b24，pg_restore --clean 后 battle_records 由 104 回落至 103、本轮战报 477 已消失；dump 归位到被忽略的 data/。
- git status：R26 相关仅 .gitignore（本 QA 为补 ignore 规则的必要改动）、specs/、审计目录/、实施方 nextWaveBrief.ts / battle.vue（t199/t200 实施产物，非本 QA 修改）。未改业务源码逻辑、未 git 提交。

## 10. 限制与结论

- 限制：agent-browser 为 Chromium 模拟视口，**非物理真机**；空态/未知敌种线上不存在，浏览器侧未覆盖，由纯函数契约补证，未伪造。
- 五项验收逐条：①真浏览器权威逐字段比对通过 ②空态/未知不伪造且契约覆盖 ③AI 四判据+CTA+无横向溢出+选卡可点通过 ④hash 不污染（静态纯函数+动态结算哈希正常）通过 ⑤6 张截图 sha 各异亲读非登录页通过。
- 最终判定：**PASS**。