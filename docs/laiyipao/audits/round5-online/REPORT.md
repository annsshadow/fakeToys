# t56 · Round5 部署后线上独立回归报告

> 角色：qa-e2e ｜ 尝试 attempt_id：`c13b5105-fffd-4b63-abd8-025a6ae07bd3`
> 性质：**线上真实测试环境**真浏览器回归；截图均在桌面 Chrome 移动/桌面视口采集，**非物理真机**。
> 真值纪律：域名/口令仅从 `.local.keys` 读取进内存，报告与脚本统一占位 `<YOUR_TEST_DOMAIN>` / `<ADMIN_PASS>`；落盘文件扫描 0 泄露。

## 1. 结论速览

| 验收锚点 | 结果 |
| --- | --- |
| ① H5 七页 + admin 五视图 + 命令面板逐页复查，探针全 0、非登录页 | PASS |
| ② AI 四判据可见 + 诚实空态/诚实文案 | PASS |
| ③ 顶栏 API 标签非回环域名脱敏为纯 API | PASS |
| ④ 回放结算闭环线上 200（含 replay_hash 入账） | PASS |
| ⑤ 截图 + 报告落 `round5-online/`，标注非物理真机 | PASS |
| ⑥ 真值注入不回显、截图二进制零泄露 | PASS |
| ⑦ 不改源码 / 不 git / 不部署 | PASS |

本轮**未发现新真实缺陷**；诚实记录 1 处工具输出脱敏失误与 1 处脚本采集瑕疵（均非产品缺陷），见 §6。

## 2. 环境

- 线上入口：admin `https://<YOUR_TEST_DOMAIN>/laiyipao-admin/`、H5 `https://<YOUR_TEST_DOMAIN>/laiyipao-h5/`、API `https://<YOUR_TEST_DOMAIN>/laiyipao-api/`。
- 连通性：admin / H5 / healthz / readyz / config 全 200；`PLAYER_LOGIN_REQUIRED=true`。
- 浏览器：agent-browser 0.41.1；会话 H5=`lyp-r5online-h5`、admin=`lyp-r5online-admin`；`--init-script` 注入探针捕获 pageErrors / consoleErrors / API 失败 / 失败请求。
- 视口：H5 390x844（移动）、admin 1258x588（桌面）。所有截图**非物理真机**。
- H5 玩家登录走原生 `click .btn-primary`（uni-app 无语义 button，eval click 会造成假会话），登录后 `lyp_token` len=157。

## 3. 锚点①：逐页复查（URL/title/结构三重确认 + 探针全 0）

### H5 七页（全部 `isLogin=false`、`hasToken=true`，探针 pe/ce/af/fr 全 0）

| 页面 | 关键结构证据 |
| --- | --- |
| index | AI 战术洞察卡四判据：装备契合 0%（权重 21%）→ 优先补它（收益 378）+ 依据 + 去补强 CTA |
| announcements | 诚实空态：「暂无公告 / 新服正常」+ AI 智能空态「近 30 天没公告，去看看战报」+ CTA |
| login | 登录页正常渲染（`uni-view.btn-primary`），探针全 0 |
| signin / stage / reactions | `kicker=1`、`kpi=1`、`aiCard=1` 三件套齐 |
| verify | `kicker=1`，结构完整 |

诚实文案核验：index `falseClaim=false` / `honest=true`，实读「反应伤害中元素侧是主力，面板攻击力也有贡献但占比受限（权重上限 30%）」——R5 文案修复线上生效，无回退。

### admin 五视图（全部 `isLogin=false`、`apiTag=API`、`domainInBody=false`，探针全 0）

| 视图 | 结构证据 |
| --- | --- |
| dashboard | kicker 段=3、KPI 卡=8、表格=3（合计 24 行）、AI 运营洞察卡四判据（「第 1 关流失最大 尝试 10→通关 7 净流失 3」+ 依据 + 复核关卡难度 CTA），无 F-1 红横幅回退 |
| skills | rows=42、kicker/kpiStrip/kpiCards(4)/filterBar/grid/aiCard 齐 |
| users | rows=45（本页 17 用户）、四判据齐 |
| recomputes | rows=33、四判据齐 |
| battles | rows=54（本页 12 战报）、四判据齐 |

命令面板（Ctrl+K）：`panel=1`、`aiGroup=1`(.cmd-ai-group-label)、`aiOptions=3`、`ariaModal=true`、焦点落 `cmd-input`，AI 分区可见。

## 4. 锚点②③：AI 可见性、顶栏脱敏、专项

- **AI 四判据可见**：H5 index（AI 战术洞察）+ announcements（AI 智能空态）+ admin dashboard（AI 运营洞察）+ 四视图各 1 张 AI 卡 + 命令面板 AI 分区，均含可见「AI」字样 + 真实数据结论 + 依据 + 可行动 CTA。
- **顶栏 API 标签**：非回环域名场景下 `apiTag=API`、正文 `domainInBody=false`，t33 脱敏线上生效。
- **AoE 25% 修复（t41）**：点击战场图例（`aria-label=战场图例`）后 `pct25=2`、`radius120=2`、`steam=true`、`overheat=true`；截图 `r5-online-h5-legend25.png` 肉眼可见「半径 120 像素内的敌人受到 25% 范围伤害」。
- **config 线上**：`config-env.json` 75368 字节，技能 1「燃烧弹」`base_damage=100`，24 技能 / 100 关卡 / 8 宝石 / 18 配方 / 22 敌人，JSON 内无 URL。

## 5. 锚点④：回放结算闭环（线上 GREEN）

- 从第 1 关「开始战斗（消耗 6 体力）」真实进入，逐波推进：wave 1/5→5/5，kills 0→42，HP 全程 1075/1075（未失守）。
- 结算文本：「通关 | 金币 +1370 | 体力 +5 | 返回关卡」；截图 `r5-online-e2e-settle.png` 显示大字「通关」+ C 评级 + 击杀 42 / 漏怪 0 / 反应 119 / 分数 11140 + 金币 +1370 / 体力 +5 + 回放哈希 `3f3f985bd38f615b`。
- 结算超时重算线上生效：结算面板显示「服务已修正本次结算：得分超时重算 21299→11140（75s 内上限，满分带 60s）」。
- 战报入账：`/laiyipao-api/api/v1/me/battles` status=200，count=11，最新 id=13、score=11140、`replay_hash=3f3f985bd38f615b`，与结算页哈希一致。

## 6. 诚实记录：失误与假阳性（非产品缺陷）

1. **工具输出脱敏失误**：一次检查玩家 token 时 sed 模式写了字面占位符而非动态 `$DOMAIN` 替换，导致真域名进入一次工具输出。已立即纠正为统一 `mask()` 动态替换；本报告中已全部占位，落盘文件复扫 0 泄露。属 QA 操作失误，非产品缺陷。
2. **闭环脚本 break 条件瑕疵**：`e2e-battle-loop.sh` 的 `grep` 模式未匹配 JSON 转义串 `"hasResult":true`，循环跑满 40 轮而非在 t=25 首达时退出。不影响结算已达成的事实；属采集脚本瑕疵，非产品缺陷。
3. **统计 SDK 上报**：探针偶见 `tongji-collector` 外部 fetch 属统计上报，非业务 API 失败，不计入 af/fr。

## 7. 产物清单

- 报告：本文件 `docs/laiyipao/audits/round5-online/REPORT.md`
- 截图 16 张（`screenshots/`）：H5 9 张（index/index-honest/login/signin/stage/verify/announcements/reactions/legend25）+ e2e 结算 1 张 + admin 6 张（dashboard/skills/users/recomputes/battles/palette）。均标注非物理真机。
- 日志（`data/`）：`h5.log`、`admin.log`、`special.log`、`e2e.log`、`config-env.json`。
- 脚本/探针：`probe-init.js`、`probe-connectivity.sh`、`capture-h5.sh`、`capture-admin.sh`、`verify-special.sh`、`e2e-battle-check.js`、`e2e-battle-loop.sh`。

## 8. 结语

Round5 线上真页面回归七项验收全 PASS，未发现新真实缺陷。本任务不修改业务源码、不执行 git、不部署。团队总目标 ≥1000 轮，本轮不宣称整体完成。