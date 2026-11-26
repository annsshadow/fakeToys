# R10 改造后真浏览器复查报告（round10-after）

- 任务：t102（attempt_id `0c908010-0a35-45b1-a1ae-91c6d77b0eea`）
- 环境：本地回环 QA（H5 `:5174` / admin `:5175` / API `:8080`，PG 容器回环 `:15440`）
- 截图：桌面 Chrome 视口 1258×588、移动视口 390×844，**非物理真机**
- 保密：真值（域名/IP/token/口令）均未入库；报告统一占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_TOKEN>` / `<ADMIN_PASS>`
- 时间：2026-11-22（Asia/Shanghai）

## 0. 结论速览

6 项契约全部满足，未发现新的真实业务缺陷。发现并处置 1 起**采集脚本自身真值泄露**（本轮自产，已即时清除并全目录复扫 0 命中），详见 §6。

## 1. 契约① —— 亲读数据态截图（硬要求）

已逐张亲读 `docs/laiyipao/audits/round10-causality/screenshots/`：

- `after-09-battles-growth.png`（sha256 `b37ed1616a55be64ca0bd51e0e860d1aecc6da7fccb010e75e5e1a29cb7f578f`，390×844）：战报详情「AI·养成前后（同一关可信差值）」卡清晰显示分数 13120→10614（-2506）、星级 1→0、击杀 39→39、漏怪 0→0、反应 85→71（-14）、剩余血量 1000→1000、到达波次 5→5，两条 replay_hash（`7d18a4ed…` / `bd49af56…`）并列，依据诚实标注「同关同难度，构筑变更点未上报，不做估算」。
- `after-08-settle-second.png`（sha256 `4ab2e40a625a90a622c14a342ccaa457308024e0e83899bd20277468ddd7de1d`）：结算页同差值卡 + 服务端钳分串「19734→10614（72s 内上限）」+ AI 因果卡。
- `after-03-gems.png`（sha `1d2fac07…`）与 `after-06-bag.png`（sha `f0b11761…`）：宝石图鉴 AI 因果卡 + 佩戴收益诚实空态 + 双 CTA、背包页签（技能3/装备2/宝石0/皮肤与道具6）。

结论：R10 核心「养成前后」因果视图与宝石三段路径均达到**有数据态**，依据诚实，未编造构筑变更点。

## 2. 契约② —— 宝石双CTA / 页签 / 收益卡 basis / 两步确认

用 DB 重签 user10 的 3 场真实战报（score 9582/13268/11056）注入浏览器（document-start），真浏览器六页采集结果（日志 `data/h5.log`，截图 `screenshots/r10-*.png`）：

| 页面 | title | kicker | AI/收益卡 | basis 依据 | 探针 pe/ce/af/fr |
|---|---|---|---|---|---|
| skills | 技能图鉴 | 1 | r7-impact=1 | 真实每波伤害 +5.4%、历史命中/波、battle_records-7d | 0/0/0/0 |
| equipment | 装备图鉴 | 1 | r7-impact=1 | 真实依据 | 0/0/0/0 |
| gems | 宝石图鉴 | 1 | AI因果卡+属性差分诚实空态 | 依据 battle_records/战报 | 0/0/0/0 |
| bag | 背包 | — | AI防守诊断 | 页签技能4/装备2/宝石0/皮肤与道具6 | 0/0/0/0 |
| battles | 我的战报 | — | AI防守诊断 | 3场 117击杀/0漏怪/均分11302 | 0/0/0/0 |
| battle | 来一炮(战斗) | — | HUD | 进入战斗 | 0/0/0/0 |

gems 页视觉确认双 CTA「去商城找宝石」「去背包镶嵌」；无「接口未提供真实预估」干瘪文案，收益卡消费 insights[].basis。

**两步确认（真点击）**：skills 页「应用推荐 · 先看收益对比 →」原生 click 打开 `当前 vs 推荐 · 确认替换` 弹层（`r10-skills-compare.png`, sha `78da7d6c…`）：干冰弹→重炮弹、每波 1966.33→2073.03、攻击 1390‰→1390‰、评分 153→126、历史三星率 0.0%（注明 stars=3 场数占比非胜率）、依据近7天3场。真点「确认替换」后 UI applied=true，QA DB `user_skill_slots` 从 `1:0,7:2,4:1` 变为 `1:0,10:1,7:2`（skill4→skill10），测后已还原原始态（`UPDATE ... SET skill_id=4`，复核恢复 `1:0,7:2,4:1`）。

## 3. 契约③ —— 真浏览器六页+admin 抽查，探针与三重确认

- H5 六页：URL 正确（uni-app 路由 `#/pages/<name>/<name>`）、title 与各页匹配、结构非空（textLen 830~3476），isLogin=false、hasToken=true，探针 pe/ce/af/fr 全 0。
- admin 看板：URL `#/admin/dashboard`、title「数据看板 · 来一炮 · 运营后台」、kicker=3/ai=1/cmdTrigger=1/textLen=947，探针全 0；dev 登录口令从源码公开默认动态注入（长度校验=10），报告脱敏 `<ADMIN_PASS>`。截图 `r10-admin-dashboard.png`（sha `327e443c…`）读图确认 AI 运营洞察卡、核心指标、经济与防线分区清晰。
- 官方 `errors --json`：H5 基线会话 `errors=[]`。

首轮用 `#/skills`（错误路由）导致 6 页 textLen=15 空白，经 Vue Router warn「No match found for location with path /skills」定位为**采集脚本 URL 格式错误（探针假阳性）**，改用 `#/pages/skills/skills` 后全部正常，非产品缺陷。

## 4. 契约④ —— R1-R9 不回退

| 项 | 结果 | 证据 |
|---|---|---|
| index 攻击力诚实文案 | PASS | `honest=true/falseClaim=false`（正则匹配「元素侧主力/权重上限30%」，无「攻击力无关」） |
| AoE 25% | PASS | 战斗页点图例后 `pct25=2/radius120=2` |
| admin 顶栏脱敏 | PASS（本地回环） | apiTag=「API 127.0.0.1:8080」——回环可见为 t33 设计分支；非回环应显示纯「API」 |
| 首屏引导卡 | PASS | t101 源码态；本会话老号 maxStage>0 故 firstRun=0（符合设计，空态非缺陷） |
| 0星差额 / 钳分HUD | PASS | t101 结算截图含「19734→10614（72s 内上限）」；t102 battle 页进入正常 |
| 推荐接口空态与 basis | PASS | skills/equipment insights[].basis 真实；gems 佩戴收益诚实空态 |
| 命令面板 AI 分区 | PASS | admin cmdTrigger=1 |

## 5. 契约⑤⑥ —— 真值零泄露与证据落盘

- 全目录文本扫描：`eyJ…` JWT / `Bearer …` / 带口令连接串 / 域名模式 **0 命中**。
- 9 张 PNG 二进制 `strings` 域名扫描：**0 泄露**。
- 证据目录：`docs/laiyipao/audits/round10-after/`（`REPORT.md`、`probe-init.js`、`auth-init.tpl.js`、`dom.js`、采集/核对脚本、`data/*.log`、`screenshots/` 9 张）。

## 6. 假阳性与收尾处置

1. **错误路由空白页**（首轮）：`#/skills` → `#/pages/skills/skills`，采集脚本问题。
2. **battle pe=1**：`ur: play() failed because the user didn't interact with the document first.` 是自动播放策略噪声；`sfx.ts:65` 设计上只过滤 `play() request was interrupted`，不过滤此条，与 t9/t32/t84 判定一致，非业务缺陷。
3. **采集脚本真值泄露（本轮自产，已处置）**：`auth-init.js` 由 mint-token.sh 用 sed 替换模板时，真正的 JWT 曾落入第 1 行注释（`<YOUR_TEST_TOKEN>` 未覆盖到注释里的历史串）。已用 OPS `ops_edit_file`（CAS 修订号）将该注释改为占位符，全目录复扫 JWT 0 命中；`auth-init.js` 现仅含 `<YOUR_TEST_TOKEN>` 占位符。未改业务源码、未 git、未部署。
4. **QA DB 还原**：apply 验证改写的 user10 技能槽已还原至原始态并复核。

## 7. 最终结论

R10 改造后真浏览器复查 6 项契约全部满足；数据态养成前后卡、宝石双CTA与收益卡 basis、两步确认弹层与写入、R1-R9 不回退均在真浏览器生效；本轮未发现新的真实业务缺陷。采集脚本泄露已即时清除并复扫归零。

> 所有截图均为桌面 Chrome / 移动视口自动化采集，**非物理真机**。