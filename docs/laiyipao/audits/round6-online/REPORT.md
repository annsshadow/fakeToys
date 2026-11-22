# R6 部署后线上独立回归报告（t61）

> 测试环境：真实部署的测试环境（域名见 `laiyipao/deploy/.local.keys`，本报告以 `<YOUR_TEST_DOMAIN>` 占位）。
> 截图均为「非物理真机」——Chrome 无头浏览器真实渲染（H5 390x844 移动视口 / admin 1258x588 桌面视口），非实体手机/物理设备。
> 只读审计任务：未修改任何业务源码、未执行 git、未部署。口令以 `<ADMIN_PASS>` 占位，运行时从 local keys 注入，未入库/未回显。

## 1. 结论速览

- **7 项验收全部 PASS**；本轮**未发现新的真实业务缺陷**。
- **R6-BUG-1（t57 发现的 admin 合法空态误标本地规则引擎）线上已验证修复生效**：6 个 admin scope 推荐接口全部 `200 / rec-v1 / insights=1`（真实洞察），五视图 AI 卡 `localTag=false`，无「本地规则引擎」误标。
- H5 三页（equipment/reactions/skills）真实消费 `GET /me/recommendations?context=...`，接口 `200 / rec-v1 / insights=1`，AI 卡 `rec=2`、`localTag=false`；空态/失败时才走诚实兜底（源码 `r6Recommendation.ts:77-84` 明确标注「本地规则引擎」），属设计而非缺陷。
- R1–R5 不回退：顶栏 `apiTag="API"`（纯 API 脱敏，`domainInBody=false`）、25% AoE 图例文案生效、index 攻击力诚实文案在（`honest=true/falseClaim=false`）、config 技能1 燃烧弹 `base_damage=100`。
- 登录后 H5 三页差分探针 `pe/ce/af/fr` 全 0；admin 五视图探针全 0；无白屏、无 pageerror。

## 2. 环境与方法

- 线上入口：admin `https://<YOUR_TEST_DOMAIN>/laiyipao-admin/#/admin/...`，H5 `https://<YOUR_TEST_DOMAIN>/laiyipao-h5/#/pages/...`，API `https://<YOUR_TEST_DOMAIN>/laiyipao-api/`。连通性探测 `/laiyipao-admin/`、`/laiyipao-h5/`、`/laiyipao-api/healthz`、`/laiyipao-api/readyz` 全 200。
- 真值从 `laiyipao/deploy/.local.keys`（DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET）读入 shell 变量，脚本输出经 `mask()` 动态替换为占位符；`bash -x` 全程未启用。
- 浏览器：agent-browser 0.41.1，H5 会话 `lyp-r6online-h5`、admin 会话 `lyp-r6online-admin`；`--init-script probe-init.js` 注入探针（捕获 pageErrors/unhandledrejection/console.error/fetch≥400/XHR≥400/recResponses.status）。
- 登录：H5 需先清 `lyp_token/lyp_token_exp/lyp_profile` 残留，再 `input[type=text]`/`input[type=password]` 填账号口令、原生 `click .btn-primary`（uni-app 按钮非语义 button，eval click 是假登录）；admin `localStorage.clear` + `input[placeholder=admin]`/`input[placeholder=至少 10 位]` + eval 精确匹配 innerText 点「登录」。

## 3. H5 三页逐页结果（登录后，探针差分口径）

| 页面 | URL/title 三重确认 | kicker | KPI | rec 卡 | AI 结论 | localTag | 接口 recResponses | 探针(登录后差分) |
|---|---|---|---|---|---|---|---|---|
| equipment 装备图鉴 | ✓ 非登录页 / 装备图鉴 | 1 装备锻造台 | 11 | 2 | 构筑短板「没有任何装备与技能同系 → 契合加成为 0」优先调整装备与专精 | false | 200 | pe0/ce0/af0/fr0 |
| reactions 反应图鉴 | ✓ 非登录页 / 反应图鉴 | 1 反应作战台 | 11 | 2 | 主反应 overheat 近 7 天触发 217 次 | false | 200 | pe0/ce0/af0/fr0 |
| skills 技能图鉴 | ✓ 非登录页 / 技能图鉴 | 1 技能构筑台 | 11 | 2 | 近 7 场场均击杀 24.4、平均推进到第 3.5 波 | false | 200 | pe0/ce0/af0/fr0 |

- 三页 `hasToken=true`，AI 卡标注「规则引擎 · rules rec-v1」=**服务端真实 rec-v1 响应的引擎标签**（源码 `r6Recommendation.ts:47-53`：未知/非 stats 模型一律诚实降级为 rules，不冒充 GPT/LLM），**不是**接口失败时的本地兜底（兜底标签是「本地规则引擎 · 当前页面数据」，`r6Recommendation.ts:84`）。截图 `r6-online-h5-equipment.png` 可见完整 AI 卡（结论+3 条依据+「为什么推荐」+「查看搭配 →」CTA）。
- 探测说明：首轮采集脚本因误用 `input[placeholder=...]`（uni-app input 元素无 placeholder 属性，placeholder 在 wrapper 上，fill rc=1）导致登录未成功，三页停在登录页、接口 401——**这是采集假阳性，非产品缺陷**；已用 `input[type=...]` + 清 token + 原生 click 重抓（诊断链见 `data/diag-login.log`、`data/diag-login2.log`）。最终登录后差分探针（`data/probe-diff.log`）三页全 0。

## 4. admin 五视图 + 命令面板逐页结果

| 视图 | URL/title 三重确认 | kicker | KPI | 表格行 | AI 卡四判据 | localTag | 接口 recResponses | 探针 |
|---|---|---|---|---|---|---|---|---|
| content 公告与兑换码 | ✓ / 公告与兑换码 | 1 内容运营概览 | 4 | 33 | 结论+依据+CTA 齐 | false | 200 | 全 0 |
| defenses 防线值守 | ✓ / 防线值守 | 1 防线监控概览 | 4 | 28 | 结论+依据+CTA 齐 | false | 200 | 全 0 |
| equipment 装备宝石皮肤 | ✓ / 装备宝石皮肤 | 1 目录结构概览 | 4 | 60 | 结论+依据+CTA 齐 | false | 200 | 全 0 |
| wiki 玩家玩法文档站 | ✓ #/wiki / 玩家玩法文档站 | –（文档站） | – | 127 | AI 卡在（内容型） | false | 200 | 全 0 |
| login（守卫重定向 dashboard） | ✓ / 数据看板 | 3 | 8 | 25 | 结论+依据+CTA 齐 | false | 200 | 全 0 |

- **顶栏脱敏**：五视图 `apiTag="API"`（纯 API 字样）、`domainInBody=false`，非回环域名不出现在页面正文或截图；截图 `r6-online-admin-equipment.png` 顶栏仅显示「admin / API / 登出」。
- **命令面板**：Ctrl+K 打开，`panel=1 / aiGroup=1(.cmd-ai-group-label) / aiOpts=3(.cmd-ai-option) / ariaModal="true" / focus=cmd-input`；截图 `r6-online-admin-palette.png` 在册。
- **R6-BUG-1 修复验证（重点）**：接口抽样 `data/api-admin.log` 显示 content/defenses/equipment/wiki/dashboard/skills 六个 scope 全 `200 / rec-v1 / insights=1 / empty=null`，五视图 AI 卡消费真实洞察（`localTag=false`），未出现「本地规则引擎」误标；与 t57 本地复查时「合法空态 insights=[] 仍误标」形成 GREEN 对照。根因修复见 `laiyipao/admin/src/utils/recommendation.ts:57-73`（合法空态 empty_reason.code 优先于本地兜底）。

## 5. 推荐接口抽样（线上真值响应，URL 不入库）

- H5（`data/api-h5.log`）：equipment/reactions/skills 三个 context 全 `200 / rec-v1 / insights=1`。
- admin（`data/api-admin.log`）：六个 scope 全 `200 / rec-v1 / insights=1`。
- 抽样在浏览器内用探针捕获的真实请求 URL + 本地 token（`lyp_token` / `lyp_admin_token`，Bearer）发起，输出仅含 status/schema/empty/insights 条数，不含域名与令牌。

## 6. R1–R5 不回退核对

- 顶栏脱敏：见 §4（apiTag=API / domainInBody=false）✓
- 25% AoE 图例：`data/regression.log` LEGEND_BTN=1、AOE `pct25=2 / radius120=2 / steam=true / overheat=true`；截图 `r6-online-h5-legend25.png` 肉眼可见「半径 120 像素内的敌人受到 25% 范围伤害」+ 蒸汽/过热元素链，t41 修复线上生效 ✓
- 0 星差额：`battle.vue:395-411` 的 0 星动态差额文案源码在（按 `star_targets` 算下一星门槛，0 星不空喊冲 3 星，battle.test.ts:460 断言），本轮 battle 页正常渲染无异常 ✓
- index 攻击力诚实文案：`data/h5.log` INDEX `honest=true / falseClaim=false`，未出现「与面板攻击力无关/全靠攻击力」等虚假表述 ✓
- 配置基线：`data/regression.log` config 75368 字节、24 技能/100 关卡、技能1 燃烧弹 `base_damage=100` ✓

## 7. 假阳性排除（3 项）

1. **H5 首轮登录失败**（placeholder 选择器 + 会话残留）→ 采集假阳性，已用 type 选择器 + 清 token 重抓，最终三页全 PASS。
2. **H5 主采集 af=15 累积**：探针从首次打开登录页起记录，登录前未鉴权请求 401 累积；登录后差分探针清零重测三页 pe/ce/af/fr 全 0，非登录后缺陷。
3. **AI 卡「规则引擎 · rules rec-v1」标签**：是服务端 rec-v1 真实响应的诚实引擎标签，非本地兜底误标（兜底标签为「本地规则引擎 · 当前页面数据」）；admin `localTag=false` 与源码双证。

## 8. 产物清单

- 报告：本文件 `docs/laiyipao/audits/round6-online/REPORT.md`
- 截图（非物理真机）11 张：`screenshots/r6-online-h5-{equipment,reactions,skills,index-honest,legend25}.png`、`screenshots/r6-online-admin-{content,defenses,equipment,wiki,login,palette}.png`
- 数据：`data/{h5,admin,api-admin,api-h5,probe-diff,regression,diag-login,diag-login2}.log`、`data/config-env.json`
- 脚本/探针：`probe-init.js`、`capture-h5.sh`、`capture-admin.sh`、`api-sample-{admin,h5}.{js,sh}`、`probe-diff.sh`、`verify-regression.sh`、`parse-config.js`、`diag-login{,.js,.2}.sh` 等
- 真值纪律：报告/脚本/日志统一占位符，落盘扫描 `grep -rl "$DOMAIN|$ADMIN_PASS" docs/laiyipao/audits/round6-online/` 无命中（SCAN_DONE 无输出）。