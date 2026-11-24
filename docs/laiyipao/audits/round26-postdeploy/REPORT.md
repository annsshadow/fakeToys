# R26 线上回归报告：选卡阶段下一波敌情简报

- 任务：t203 / attempt 2（bda66455-e8d7-40b8-b562-e17bc28de2de）
- 复查对象：R26 选卡页下一波敌情简报 .r26-wave-brief（纯前端只读派生，无后端/admin 业务改动）
- 判定：**PASS**（三端健康、真浏览器决定性 DOM、结算不污染、零真值四门禁全满足）
- 环境说明：本地回环 127.0.0.1 实测；测试域名以 <YOUR_TEST_DOMAIN> 占位，不入库真值。真浏览器为 agent-browser 无头 Chromium 390×844（**非物理真机**，属 H5 模拟视口）。
- 真值口径：所有脚本/夹具中真实令牌仅写入被 .gitignore 拦截的 auth-init.js，入仓仅 auth-init.tpl.js（__LYP_TOKEN__ 占位）；每批 staged diff 严格扫描真实真值 0 命中。

## 1. 部署产物与三端健康

| 端 | 产物 / 版本 | 哈希（sha256 前段） | 说明 |
|---|---|---|---|
| H5 | 重建 index-BdaCk5iE.js | 277025794347755660d4888cd6beb0e49d1f2ce74bd18a6ecc5f9913e63e8058 | 本次重建；pages-battle-battle.DJTH3a2h.js 命中 r26-wave-brief |
| 后端 | lyp-server-amd64（沿用 R21） | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | R26 无后端改动，沿用经验证产物 |
| admin | index-CGo_zhD3.js（沿用 R20） | d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70 | R26 无 admin 改动，沿用经验证产物 |

- 健康端点（本地回环）：healthz=200 / readyz=200 / H5(5174)=200 / admin(5175/laiyipao-admin/)=200。
- H5 重建 exit0；bundle 逐字节命中 r26-wave-brief 标记，确认改动进入产物而非仅源码。

## 2. 四批精确提交与集成验证

- 批1 feat：059436ef229580ac9f57e59cb165b09dec173d37（nextWaveBrief.ts 纯函数 + battle.vue 接线，2 文件 145 行，STAGED=2 LEAK=0）。
- 批2 docs：342a9108e6df210473d873664e1260e9d9003557（.gitignore + SPEC.md + ROUND26_DESIGN_REVIEW.md，3 文件 132 行，REAL_LEAK=0）。首轮宽松正则命中终审报告描述扫描口径的元文本（良性误报），用要求真实用户名/口令主机/私钥的收窄正则确认 0 命中后提交。
- 批3 test：4d177358fcda4da8859fabe1a2a38fcf1f63c961（契约测试 + 审计证据 + 6 截图，28 文件 585 行，DATA=0 AUTHJS=0 PLACE=3 REAL=0）。
- 批4 线上报告：本报告精确 git add 单文件，逐文件自检零真值后提交。
- 上游终审 t202 verdict=pass（六门禁全过，findings=0），契约 21/21、miniapp 全量 1900 passed/2 skipped、vue-tsc、build:h5、Go 真值守卫均 exit0，为本轮放行前置。
- 每批均显式 git add 路径，禁 -A/.；历史一次性脚本、失败截图、真值 data/ 与 auth-init.js 全部排除。

## 3. 权威数据源与 HTTP 抽样

- 关卡 5 权威端点 GET /api/v1/levels/5（带 Bearer token）逐字段对 DOM：waves[1] 游荡者×5、waves[2] 游荡者×4+×2=6、waves[3] 疾行者×5+游荡者×3、waves[4] 疾行者×7+腐化犬×4+壮硕体×3。
- H5 选卡帧 DOM 聚合与权威逐字段一致：第 2 波游荡者×5、第 3 波游荡者×6、第 4 波 2 种共 8 只（疾行者×5、游荡者×3）；无跳号、无编造。
- 威胁标签专项（关卡 22）：第 4 波壮硕体×6、电鳍兽×2[远程]（attack_range=150）；威胁优先级首领>飞行>潜地>远程，与 stage.vue 口径一致。
- 空态/未知敌种线上不存在，未伪造；由纯函数契约覆盖（无 waves/越界/空 spawns/未知 id → 诚实空态与 #id 标注）。

## 4. R26 核心：选卡敌情简报决定性 DOM

- 容器 .r26-wave-brief[role=region]、标题 .r26-brief-head、敌种行 .r26-enemy-row[data-r26-enemy]、AI 注释 .r26-ai-note[role=note] 均存在。
- AI 四判据逐字命中：① 标题 AI · 下一波敌情；② 结论由当帧真实聚合派生（如下一波 2 种敌人共 8 只，按伤害与元素协同选卡）；③ 依据行逐字「依据：当前关卡 waves[waveIndex+1].spawns 按 enemy_id 聚合；敌种名与威胁标签来自 /config.enemies」；④ CTA 逐字「结合本波威胁与手牌元素协同选卡」。
- 诚实红线逐字命中：「辅助简报非权威战斗数据，实际敌情以引擎波次为准；权威回放以 replay_hash 为准」。
- 脏值与布局：全部采样帧 hasUndefined=false、hasNaN=false、unknownTags=0、briefOverflowX=false、docOverflowX=false；手卡 3 张与「全部不要了」按钮始终可见可点（cardCount=3 hasSkip=true）。

## 5. 真浏览器线上回归（本地回环）

- 修正后的主 E2E（e2e-full2.sh，关卡 5）跑满 94 帧至 SETTLED，CARD_CLICKS=4；3 个唯一波次标题（第 2/3/4 波）逐帧数值与权威端点一致。
- 结算决定性：result=通关；网络探针 settle=[{status:200,battle_id:477}]；hashResponses=[]（简报自身未产生任何 replay_hash 写响应）；回放哈希为本次 QA 库还原后重新结算的真实产出，回放链路未被污染（静态纯函数 + 动态结算均验证）。
- 错误探针：ce=0 / af=0 / fr=0；pe=59 全部为自动播放噪声（The play() request was interrupted by a call to pause()），按 R24/R25 同口径过滤后真实页面错误 peReal=0。
- 首次主脚本因脚手架对转义输出的 state 判断失配卡在第 2 波（CARD_CLICKS=0，非产品缺陷），已改用 state-eval.js 判态的 e2e-full2.sh 重跑成功；重拍截图已 git checkout HEAD 恢复入库版，保持相邻提交稳定。

## 6. 截图证据（入库版，各异、亲读、非登录页）

| 文件 | sha256 | 内容 |
|---|---|---|
| r26-01-brief-wave2.png | 3d4b23ffae9d970339c91f0eba313b6261592ac7ae1d9f7c4a80eac6d1072a69 | 第 2 波 游荡者×5 + AI 四判据 |
| r26-02-brief-wave4.png | 5450d5950c29538b0f9d1681d0da490e333b6d376bdd2dda2cd6c59650cfb963 | 第 4 波 疾行者×5/游荡者×3 |
| r26-03-settle.png | 856c83009afd70ffdb34435fd9b855a3cc32efa7eafbdc0815d7e6d6537e9472 | 通关结算 + 回放哈希 |
| r26-04-brief-scroll.png | ec75b3e822817594dbbfe9fecd41a5a3c058f1c650070799336559145e6183d7 | 第 4 波滚动后简报/手牌/AI 区 |
| r26-05-brief-threat.png | f158b31e90da6464ebe9bf4391d2dad615b2a35e672d9b898829790116b04d75 | 关卡 22 第 2 波（浅跑） |
| r26-06-brief-threat-tag.png | c519e59db423be63e6687375a16487bbefebd5fd2dbbdea0938226437a47dd7b | 关卡 22 第 4 波 电鳍兽×2[远程] + AI [远程]×2（本轮亲读确认） |

## 7. 结论

- 三端健康与版本、四批精确提交零真值、真浏览器决定性 DOM、权威逐字段一致、结算不污染、截图各异亲读——六项验收全部满足。
- R26 简报为纯只读派生，未改引擎/结算/replay_hash，未新增令牌，令牌锚点 v1/v3-r3/r6 保留，无 --r26- 新令牌。
- 诚实边界：agent-browser 为模拟视口非物理真机；空态/未知敌种线上不可达由纯函数契约补证，未伪造。
- 最终判定：PASS，可推送。
