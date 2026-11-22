# t44 · Round4 部署后线上真页面回归报告

- 任务：t44「R4部署后线上真页面回归」（QA，只读）
- 时间：2026-11-22（Asia/Shanghai）
- 页面性质：**真实测试环境线上 HTTPS 页面，浏览器自动化 + 截图标注「非物理真机」**（Chrome headless 1258×588 / 390×844 视口，非物理手机/显示器）
- 真值纪律：测试域名 `<YOUR_TEST_DOMAIN>`、后台口令 `<ADMIN_PASS>` 仅从 gitignore 的 `laiyipao/deploy/.local.keys` 注入脚本内存，报告与截图全脱敏；12 张 PNG 二进制扫描域名/口令均 0 命中。

## 0. 结论速览

| 验收锚点 | 结论 |
|---|---|
| ① H5 六页 + admin 五视图 + 命令面板 URL/title/结构三重确认 | **PASS** — 11 页全非登录页，探针 pe/ce/af/fr 全 0 |
| ② AI 四判据线上可见 | **PASS** — admin 看板/经济 + H5 六页共 8 处真实数据结论 + 依据 + CTA |
| ③ 顶栏非回环域名脱敏为纯 `API` | **PASS** — 五视图 apiTag=「API」，正文域名检测 false，截图 0 泄露 |
| ④ t41 AoE 25% 文案线上生效 | **PASS** — config 两处 25% 文案 + reactions 页真浏览器 5×「25%」 |
| ⑤ 截图/报告落盘 + 标注非物理真机 | **PASS** — 12 张 PNG + 本报告 |
| ⑥ 不改源码/不 git/不回显真值 | **PASS** — 仅写 `docs/laiyipao/audits/**` |

**本轮未发现新真实缺陷。** 两项采集口径假阳性已在 §4 澄清。

## 1. 环境与方法

- 入口：admin `https://<YOUR_TEST_DOMAIN>/laiyipao-admin/#/`、H5 `https://<YOUR_TEST_DOMAIN>/laiyipao-h5/#/`、API `https://<YOUR_TEST_DOMAIN>/laiyipao-api/`
- 连通性：admin/H5/healthz/readyz/config 全 HTTP 200（证据 `data/h5-online.log` 前的 `probe-connectivity.sh` 执行结果）。
- 会话：H5 `lyp-r4online-h5`（原生 click `.btn-primary` 登录，`lyp_token`/profile 均建立）、admin `lyp-r4online-admin`（口令动态注入 + 长度校验，跳 `/admin/dashboard`）。
- 探针 `probe-init.js`：在页面最早阶段挂 pageerror/unhandledrejection/console.error/fetch/XHR ≥400 捕获；`pe/ce/af/fr` 全为 0（H5 六页 + admin 五视图）。
- 视口：H5 390×844、admin 1258×588；截图均「非物理真机」。

## 2. H5 六页（逐页数组证据，querySelector 结构计数）

| 页 | title | 登录态 | KPI | AI 卡 | 数据网格 | 探针 |
|---|---|---|---|---|---|---|
| bag | 背包 | token✓ | 1 | 1「AI 构筑诊断」 | grid=1/filter=1 | 0/0/0/0 |
| battles | 我的战报 | token✓ | 1 | 1「AI 防守诊断」 | grid=1/filter=1 | 0/0/0/0 |
| enemies | 敌人图鉴 | token✓ | kicker=1 | 1「AI 克制诊断」 | grid=1/filter=2 | 0/0/0/0 |
| me | 来一炮 | token✓ | 1 | 1「AI 养成诊断」 | grid=1/filter=1 | 0/0/0/0 |
| rank | 排行榜 | token✓ | 1 | 1「AI 冲榜目标」 | grid=1/filter=1 | 0/0/0/0 |
| skins | 皮肤图鉴 | token✓ | 1 | 1「AI 皮肤流派提示」 | grid=1/filter=1 | 0/0/0/0 |

- 六页 `isLogin=false`、`hasToken=true`、`probe{pe:0,ce:0,af:0,fr:0}`；URL/title/结构三重建立。
- AI 卡内容均为**真实数据结论 + 依据 + 可行动 CTA**（如背包「4 种元素已覆盖多类反应链，优先补足 4/4 出战槽 / 依据：图鉴 42 项 / 去技能槽→」），非占位文案。
- 关键截图读图确认：`r4-online-h5-bag.png`（390×844, sha256 be8b6bd3）/ `r4-online-h5-battles.png`（734f2172）/ `r4-online-h5-enemies.png`（4db8888d）/ `r4-online-h5-skins.png`（37695c8f）/ `r4-online-h5-me.png`（c4939000）均为真实 R4 视觉（眉题 + KPI + AI 卡 + 数据），无白屏。

## 3. admin 五视图 + 命令面板

| 视图 | title | 登录态 | 眉题 | KPI | 表 | 探针 |
|---|---|---|---|---|---|---|
| dashboard | 数据看板 | ✓非登录 | 3 段（核心指标/经济防线/趋势） | 1 | 3 表 24 行 | 0/0/0/0 |
| content | 公告与兑换码 | ✓非登录 | 内容运营概览 | 3 | 10 表 33 行 | 0/0/0/0 |
| economy | 经济与商城 | ✓非登录 | 经济监控概览 | 5 | 12 表 59 行 | 0/0/0/0 |
| defenses | 防线值守 | ✓非登录 | 防线监控概览 | 3 | 8 表 42 行 | 0/0/0/0 |
| equipment | 装备宝石皮肤 | ✓非登录 | 目录结构概览 | 3 | 12 表 74 行 | 0/0/0/0 |

- 顶栏脱敏：五视图 `apiTag=「API」`、正文域名检测 `hasDomainInBody=false`（§0 ③）。截图 `r4-online-admin-dashboard.png`（1258×588, sha256 62948598）顶栏目视仅「admin / API / 登出」，无真域名。
- 命令面板：Ctrl+K 开启 `.cmd-panel`，`aiGroup=1`（✦ AI 洞察分区）、`aiOpts=3`（查看今日运营洞察/复核流失最大关卡/跳转到元素反应分布）、`aria-modal=true`；截图 `r4-online-admin-palette.png`（sha256 2e86a948）目视确认。
- AI 运营洞察卡（看板）：`第 1 关流失最大（尝试 9 → 通关 7，净流失 2），建议优先复核关卡难度 / 依据：关卡漏斗柱状图，共 3 关… / 复核关卡难度→`。

## 4. 假阳性澄清（诚实记录，不影响 verdict）

1. `dashboard` 首轮 `ai:0`：采集脚本选择器 `.ai-insight` 不匹配看板真实类名 `.ai-insight-card`（`AiInsightCard.vue:18`）→ 补验 `.ai-insight-card`=1 且四判据齐全，截图目视确认；属采集口径，非产品缺陷。
2. `content/defenses/equipment` 无 AI 卡：t36 仅对 EconomyView 设计 AI 洞察卡（§1C.2），三视图无 AI 卡为设计预期，非缺陷。
3. pageerror 噪声（H5 战斗）：与 t9/t32 一致，均为 `play() request was interrupted`（sfx 过滤器 init-script 注册顺序导致探针先捕获）/`didn't interact`（eval 点击自动播放策略），本轮未进入战斗采集故为 0，非业务缺陷。
4. 首屏 `apiTag` 在回环开发环境显示具体地址属 t33 设计（仅非回环脱敏为纯 API），线上已确认为纯「API」。

## 5. AoE 25% 文案线上生效（锚点④）

- 线上 `config` API 返回 2 处 25% 文案：「半径 120 像素内的敌人受到 25% 范围伤害」（steam_burst）、「半径 90 像素…（overheat）」，与 `elements.ts:158/168` 一致。
- 真浏览器 H5 reactions 页：`pct25=5`、`radius120=4`、蒸汽/过热均出现（`data/aoe-legend.log`），t41 AoE 修复与 t42 图例文案线上可见。

## 6. 产物清单（docs/laiyipao/audits/round4-postdeploy/）

- `REPORT.md`（本文件）、`probe-init.js`
- 脚本：`probe-connectivity.sh`、`capture-h5-online.sh`、`capture-admin-online.sh`、`supplement-check.sh`、`verify-aoe-legend.sh`
- 数据：`data/{h5-online.log,admin-online.log,supplement.log,aoe-legend.log}`
- 截图 12 张：`screenshots/r4-online-h5-{bag,battles,enemies,me,rank,skins}.png` + `screenshots/r4-online-admin-{dashboard,content,economy,defenses,equipment,palette}.png`

（标注：所有截图均为真实测试环境线上页面经浏览器自动化截取，非物理真机。）