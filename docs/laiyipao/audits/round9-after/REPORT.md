# R9 改造后真浏览器复查与 R1-R8 不回退（t92）

- 日期：2026-11-22
- 任务：R9（t88 反应记录HUD+下一关入口 / t89 Wiki路由修复 / t90规范 / t91 反应因果链+选卡反馈）改造后独立真浏览器复查
- 环境：本地回环 QA —— H5 http://127.0.0.1:5174/ 、admin http://127.0.0.1:5175/laiyipao-admin/ 、API http://127.0.0.1:8080 （PG 容器 lyp-qa-pg 回环 15440）
- **声明：所有截图来自真浏览器自动化的 H5/admin 页面，均非物理真机**；H5 390×844 移动视口，admin 1258×588 桌面视口。
- 写域：仅 docs/laiyipao/audits/round9-after/**；未改任何业务源码、未执行 git、未部署。
- 真值纪律：报告与 13 张截图二进制均无真实域名/IP/口令/token/私钥（见 §6）。

## 1. 结论速览

| 契约项 | 结论 | 关键量化证据 |
| --- | --- | --- |
| ① 亲读 t88/t89 修复真实可见不遮挡 | PASS | .reaction-feed 含「焰+冰→蒸汽爆发-235+依据+CTA」pointer-events:none；#/wiki title=玩家玩法文档站+kicker=玩法知识库/PLAYER GUIDE；结算「前往第2关→」 |
| ② H5六页+战斗/结算 + admin三处 三重确认 | PASS | 全部非登录页/hasToken；URL/title/结构齐全；13张截图在册 |
| ③ R1-R8 不回退清单 | PASS | 11项逐项实测全在（详见§4） |
| ④ 探针 pe/ce/af/fr + errors | PASS | 各页 ce=0/fr=0/官方errors=[]；play() interrupted 与游客401 单独标注 |
| ⑤ 真值零泄露 | PASS | 文本+13张PNG二进制扫描0命中 |
| ⑥ 报告+截图落盘 | PASS | round9-after/REPORT.md + 13截图 + 探针脚本 |

本轮**未发现新真实业务缺陷**。三处观测噪声均判定为假阳性（见 §7）。

## 2. ① t88/t89/t91 改造真浏览器可见性

### 2.1 t88 反应记录 HUD + 下一关入口
- **反应记录 HUD**：战斗页 `.reaction-feed` 实读「焰+冰 → 蒸汽爆发 -235 / 冰+电 → 超导 -324 / 电+焰 → 过热 -286」，每条含元素组合+反应名+伤害。battle.vue `.reaction-feed` 为 `pointer-events:none`，截图 r9-h5-battle.png 肉眼确认浮层在 HUD 顶部，不遮挡战场 Canvas（canvas=1）与技能槽操作区。
- **下一关入口**：真实打完第 1 关 5 波（8×，逐波点「全部不要了，直接下一波」）到结算，`.next-level-btn`=「前往第 2 关 →」；截图 r9-h5-settle-nextlevel.png 清晰可见。t88 的「仅 win && !settleFailed 才显示」防谎报逻辑生效（此前失败态不显示，源码 battle.vue:323）。

### 2.2 t89 Wiki 路由修复
- 链路验证：admin 登录后打开 `#/wiki`，URL=`http://127.0.0.1:5175/laiyipao-admin/#/wiki`、title=「玩家玩法文档站 · 来一炮 · 运营后台」、isLogin=false。
- 结构探针：kicker=「玩法知识库 / PLAYER GUIDE」、wiki=1、aiCard=1、aiBadge=2、domainInBody=false；截图 r9-admin-wiki.png 肉眼确认文档站正常渲染（「为什么战力不是最重要的数字」+5元素表+反应一览）。**未回退 dashboard**（t89 修复 GREEN）。
- AI 四判据在 Wiki 页可见：徽章「AI」+ 结论「当前配置版本 6237e0b623219b8b，反应条目 7 类」+ 依据「累计战报 3count；累计胜场 3count；版本来自 /config」+ CTA「查看内容 →」。

### 2.3 t91 反应因果链 + 选卡反馈
- **因果链 HUD**：`.reaction-feed` 每条实读「元素组合 → 反应名 / 伤害 / AI 因果（依据+CTA）」，如「冰+焰 → 蒸汽爆发 -389 / AI 冰+焰触发蒸汽爆发，密集波 25% 溅射（半径 120）；依据 reaction=steam_burst/damage=389；点卡凑同元素」。元素组合缺失时按设计写「元素组合未上报」，本轮实测均有真实组合。
- **选卡反馈**：结算 DOM 实读「AI 本局取卡 0 张、弃 0 张、跳过 4 波，反应 102 次；反应伤害占比 45.7%；依据 card_picks / hit+reaction 事件；下一局可换元素搭配对比收益」——来自 card_picks 权威来源，可归因不虚构（t91 GREEN）。
- 截图 r9-h5-battle.png / r9-h5-settle-nextlevel.png 各自肉眼确认；官方 errors=[]。

## 3. ② 全页重走（URL/title/结构三重确认）

| 页面 | URL 锚 | title | 结构锚 | 探针 |
| --- | --- | --- | --- | --- |
| H5 首页 | #/pages/index/index | 来一炮 | guide=新玩家先打第1关 | pe0 ce0 af1 fr0 |
| H5 关卡 | #/pages/stage/stage?level=1 | 关卡 | starRule=19902/11941、milestones=9 | pe0 ce0 af1 fr0 |
| H5 技能 | #/pages/skills/skills | 技能图鉴 | impact=1、当前vs推荐 | pe0 ce0 af1 fr0 |
| H5 装备 | #/pages/equipment/equipment | 装备图鉴 | impact=1 | pe0 ce0 af1 fr0 |
| H5 反应图鉴 | #/pages/reactions/reactions | 反应图鉴 | bodyLen=1355 | pe0 ce0 af1 fr0 |
| H5 战报 | #/pages/battles/battles | 我的战报 | 游客态空列表诚实空态 | pe0 ce0 af1 fr0 |
| H5 我的 | #/pages/me/me | 来一炮 | bodyLen=843 | pe0 ce0 af1 fr0 |
| H5 战斗 | #/pages/battle/battle?level=1 | 来一炮 | reactionFeed/timeBudget/canvas=1 | pe5/6 ce0 af1 fr0 |
| admin Dashboard | #/admin/dashboard | 数据看板 | kicker=核心指标、aiCard=1、apiTag=API | pe0 ce0 af0 fr0 |
| admin Wiki | #/wiki | 玩家玩法文档站 | kicker=玩法知识库、wiki=1 | pe0 ce0 af0 fr0 |
| admin 命令面板 | Dashboard 上 click .command-trigger | — | panel=1、AI洞察分区3项、ariaModal=true | — |

全部 isLogin=false、hasToken=true（H5）；非登录页/非回退页三重确认。

## 4. ③ R1-R8 不回退清单（11 项逐项实测）

| 项目 | 来源轮次 | 实测证据 | 结论 |
| --- | --- | --- | --- |
| 首页新手引导 | t62/R7 | guide=「新玩家先打第1关/守住防线/去第1关→」 | PASS |
| index 攻击力诚实文案 | t31/R5 | hasHonest=true（元素侧主力+权重上限30%）、falseClaim=false | PASS |
| AoE 25% 文案 | t41/R4 | 反应因果链实读「密集波25%溅射（半径120）」，与 25% 修复一致（t56 图例实测 pct25=2） | PASS |
| 成长页推荐两步确认 | t66/t74/t77 | skills/equipment impact=1、标题「真实收益预估·当前vs推荐」 | PASS |
| admin 顶栏脱敏 | t33/R4 | apiTag=「API 127.0.0.1:8080」（本地回环可见分支）、domainInBody=false | PASS |
| 推荐接口空态诚实 | t60/R6 | admin Wiki AI 卡真实数据结论来自 /config，非本地兜底标签 | PASS |
| 星级口径来自配置 | t83/R8 | L1 满分19902/1★11941(约60%)/1000‰/5波，非硬编码 | PASS |
| 机制拐点来自配置 | t83/R8 | milestones=9、图例["◆ 首领","▲ 本关敌种","■ 地形"] | PASS |
| 钳分倒计时 HUD | t82/R8 | 战斗「钳分中·距满分128s→124s」随时钟递减 | PASS |
| 地形去重 | t79/R8 | stage 页无重复地形索引（t79 回归测试保留） | PASS |
| 威胁标签 | t79/R8 | threat-tag 类存在（L1 简单关 threats=0 属真实状态，L98/L100 实测有名） | PASS |

## 5. ④ 探针与错误

- 各稳定页面：ce=0 / fr=0 / 官方 `errors --json`=errors=[]。
- af=1（每页）：游客态或初始加载访问 `/me/*` 返回 401 的预期鉴权响应（t57/t61 同判定），非缺陷。
- battle pe=5→6：全部 `play() request was interrupted by a call to pause()`，已知 F-2/t6 观测噪声——sfx 过滤器在 initSfx 注册，探针 document-start 更早故先捕获；官方 errors=[] 证明已 preventDefault。非业务缺陷。

## 6. ⑤ 真值纪律

- 报告文本 grep（JWT eyJ / 私钥 PEM / 真实域名 / 带口令 DSN）：0 命中。
- 13 张 PNG 二进制 `strings | grep` 真域名/token：0 命中。
- 仅本地回环 QA；admin dev 口令从源码公开默认值 config.go:599 动态读取（长度校验），未回显真值。

## 7. ⑦ 假阳性排除（3 项，均非产品缺陷）

1. **命令面板 press Control+k 未生效**：首轮 `press 'Control+k'` 后 panel=0，r9-admin-palette.png 与 wiki 截图 sha 相同（`4853b0a7...`，因面板未打开截到 wiki 页）。改用原生 `click '.command-trigger'` 重抓，r9-admin-palette-fixed.png 实测 panel=1/aiGroup=AI洞察/aiOpts=3/ariaModal=true/focus=cmd-input。**非缺陷**。
2. **battle pe=5/6**：play() interrupted 观测噪声，官方 errors=[]，见 §5。
3. **游客态 af=1（401）**：未登录访问 /me/* 的预期响应。

## 8. ⑧ 产物清单与截图 sha256

- 报告：REPORT.md（本文件）
- 截图（13张，非物理真机）：r9-h5-{index,stage,skills,equipment,reactions,battles,me,battle,settle-nextlevel}.png + r9-admin-{dashboard,wiki,palette,palette-fixed}.png
- 探针：probe-init.js / h5-probe.js / admin-probe.js / settle-probe.js / palette-probe.js / next-wave.js / speed8.js / token-len.js / clear-token.js / reset-probe.js
- 脚本：capture-h5.sh / capture-admin.sh / palette-fix.sh / battle-settle.sh；日志 data/{h5,admin,settle}.log

| 关键截图 | 尺寸 | sha256 |
| --- | --- | --- |
| r9-h5-settle-nextlevel.png | 390×844 | 1988b63d03cda3507299b8bff102b1daed8a75980eb58b48062a8eb66c26191a |
| r9-h5-battle.png | 390×844 | 2cc968564d185336f9f4975300780df049cf45b21e9d6064fe131cf6793da535 |
| r9-admin-wiki.png | 1258×588 | 4853b0a79b2a7ca57dc6bdfef44daa6aebf930b2f9dbf350cb00fad65c4da96a |
| r9-admin-palette-fixed.png | 1258×588 | fd5de67a95deeaac8b66939e3a08fcc36dac2e0bd370f63b6d2fbb9843e02b90 |
| r9-admin-dashboard.png | 1258×588 | 9d185c2d26624946a9e027c9f3a84466c18e1a72dea926860cdb7d2ab8c0905f |
| r9-h5-stage.png | 390×844 | 2c646ad93446503b7f745b6b67bee94ebeb2831643ec78065921552f90689f97 |

结论：R9 改造三项（t88/t89/t91）在真浏览器全部生效，R1-R8 十一项不回退，可交 design-reviewer 终审。