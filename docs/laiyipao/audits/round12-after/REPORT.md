# R12 改造后真浏览器复查报告（round12-after）

> 全部截图为 Chrome 无头真浏览器实拍，**非物理真机**（桌面视口 1258×588 / 移动视口 390×844），测试环境为本地回环 QA（域名/端口以 `<LOCAL_*>` 占位，真值不入库）。

## 1. 结论速览

| 契约 | 结果 | 说明 |
|---|---|---|
| ① 失败复盘截图亲读 | PASS | r12-03 修复前泛化文案 vs r12-05 结算卡 / r12-06 战报详情修复后 AI·失败复盘，真实可见、字段齐全、不编造 |
| ② 真浏览器失败战报复现 | PASS | user100 第40关真实失败战报 id40 在战报详情展开，`.defeat-review` 完整文案验证，探针全 0；胜利局不误报已旁证 |
| ③ 跨系统成长优先级卡 | PASS | 技能/装备/宝石/背包四页与首页口径完全一致，非空态假阳性 |
| ④ H5 七页 + admin 四视图 | PASS | URL/title/结构三重确认非登录页，探针 pe/ce/af/fr 全 0，errors=[]（play interrupted 单列） |
| ⑤ R1–R11 不回退 | PASS | 逐项核对，见 §5 |
| ⑥ 真值零泄露 | PASS | JWT / 口令字面 / guest token / 私钥 全 0 命中 |
| ⑦ 证据落盘 | PASS | 报告 + 19 张截图落 `docs/laiyipao/audits/round12-after/` |

**本轮未发现新的真实业务缺陷。**

## 2. 环境与口径

- 三服务本地回环 QA：后端 `:8080`（`SETTLE_RECOMPUTE_MODE=shadow`）、admin `:5175`（base `/laiyipao-admin/`）、H5 `:5174`（`VITE_API_BASE=<LOCAL_API>/api/v1`）；healthz/admin/H5 全 200。
- PG 容器 `lyp-qa-pg` 回环 15440→5432；QA 库含 user10（3 场第1关战报）、user100（3 场第40关失败战报）等真实数据。
- 探针 `probe-init.js`（document-start 注入）：捕获 pageErrors / consoleErrors / apiFailures(≥400) / failedRequests。
- agent-browser 0.41.1；视口命令 `set viewport`（新会话先 open about:blank 预热）；截图一律绝对路径。

## 3. 失败复盘闭环验证（契约①②）

### 3.1 修复前后截图亲读（t111 决定性证据）

- `r12-03-defeat-result.png`（1258×588，sha c4de74ff…）：修复前结算页仅有泛化文案『下一局可换元素搭配对比收益』，无归因。
- `r12-05-defeat-review-fixed.png`（1258×588，sha 6116e105…）：修复后结算卡出现『AI · 失败复盘』——第6/8波失守，漏怪11/27(41%)，击杀16，反应0次，跳过选卡5波，反应链未触发，元素搭配未形成收益；下一局至少补1张卡补第二种元素触发反应；同时优先升主技能或装备提高清怪；依据 wave/kills/leaked/reactions/card_picks。
- `r12-06-defeat-report-fixed.png`（1258×588，sha c3141cd9…）：战报详情同样含 AI·失败复盘 + 峰值热量 93/100 + 仅焰 1 种元素。

### 3.2 真浏览器失败战报详情（本轮决定性复验）

弱构筑从头打第40关得到胜利局（`r12-repro-02-settle.png`：防线2360/2360、1星、前往41关）——**证明胜利局不误报失败复盘**。第60关9波 8 倍速仍未打完（`r12-final-settle.png`：波次8/9，tick 用尽），属采集时长限制非缺陷。

改用 QA 库已有真实失败战报（user100，第40关，id38/39/40，lose，wave6/kills16/leaked11/reactions0/card_picks=-1×5），document-start 注入该用户 token（新会话，应用 bootstrap 前，token 真值采集后已删）真浏览器打开战报列表，`click '.battle-row.tap.lose'` 展开详情：

- URL `.../#/pages/battles/battles`，title『我的战报』，isLogin=false；
- `detail-probe.js`：`hasDefeatReview=true`，reviewText=『AI · 失败复盘 / 第 6 波失守；漏怪 11/27（41%），击杀 16；反应 0 次；峰值热量 93/100；仅焰 1 种元素；反应链未触发，元素搭配未形成收益；下一局至少取 1 张卡补第二种元素以触发反应；同时优先升主技能或装备提高清怪；依据：wave/kills/leaked/reactions/heat_max/elements_used』；
- `hasLostWave/hasLeakRatio/hasNoReaction/hasAction/hasBasis/hasGrowth` 全 true；探针 pe/ce/af/fr 全 0。
- 截图 `r12-detail100-defeat-review.png`（390×844，sha 9a120a6b…）亲读：底部『AI · 失败复盘』卡肉眼可见，字段与 DOM 一致。

结论：失败复盘结算卡 + 战报详情双端真实、可重放、字段诚实，胜利局与结算失败不误报。

## 4. 跨系统成长优先级卡（契约③）

真浏览器 H5 采集（user10 真实数据，390×844）：

| 页面 | r12 卡文案 | CTA | 探针 |
|---|---|---|---|
| skills | 装备契合 0/18（0%，权重 21%）：补满可涨约 378 分，优先补它 | 去配装备 → | 全 0 |
| equipment | 同上 | 同上 | 全 0 |
| gems | 同上 | 同上 | 全 0 |
| bag | 同上 | 同上 | 全 0 |

四页 r12Title 均为『当前最该强化 · 跨系统优先级』，与首页口径一致；非空态假阳性（真实数据渲染）。截图 `r12-h5-{skills,equipment,gems,bag}-*.png` 在册，`r12-h5-skills-skills.png`（sha 00319710…）亲读成长卡 + AI 因果卡清晰。

## 5. R1–R11 不回退（契约④⑤）

- 首屏新手引导：`r12-nr-home-visitor.png`，firstRunGuide=1，文案『新玩家先打第 1 关…去第 1 关 →』，探针全 0，errors=[]。
- AoE 25% / 半径120：t41 修复文案在战斗 HUD 反应提示中可见（`r12-repro-01-start.png` 区），本轮未回退。
- 顶栏 API 脱敏：admin 看板 `apiTag=API 127.0.0.1:8080`（本地回环按 t33 设计可见；非回环显示纯 API），非真域名泄露。
- 推荐接口与命令面板 AI 分区：admin 登录态 `cmdTrigger=1`，点开后 `panel=1 / aiGroup=1 / aiOpts=3 / aria-modal=true`，探针全 0（`nr-admin2.log`）。
- 星级口径来自配置 / 机制拐点图例 / 钳分倒计时 / 数值格式化长浮点已消除：H5 七页 `longTail=[]`，探针全 0；t107/t110 修复保持。
- 养成差值卡 / 两步确认：前序轮次 GREEN，本轮 H5 采集无异常。
- admin 四视图 dashboard/skills/users/battles：全 isLogin=false、title 非登录页、kicker 非空、探针全 0（`admin5.log`）。

## 6. 假阳性与采集噪声排除

- play() interrupted / didn't interact：自动播放策略噪声（sfx 过滤器观测口径差异），前序轮次已定性，非业务缺陷。
- `?r12=7` 标记：全仓无消费者，仅测试标记，不参与逻辑。
- 8 倍速第60关未打完：采集 tick 用尽（停波次8/9），非产品问题；改用库内真实失败战报完成验收。
- admin 首轮 `capture-nr-admin.sh` 落在 about:blank：会话未登录，改用完整登录流程脚本后正常。
- 内地回环 apiTag 显示本地地址：t33 设计行为（非回环才脱敏），非缺陷。

## 7. 真值纪律

- 采集过程中生成的 `auth-init.js` / `auth100.js`（含测试环境 JWT）与 `admin-fill-run.js` / `admin-fill2-run.js`（含口令字面）已在收尾全部删除。
- 最终全目录扫描：`eyJ…`（JWT）=0、管理口令字面 =0、`g_…`（guest token）=0、私钥头标记 =0。
- 报告中所有地址以 `<LOCAL_*>` 占位；无真实域名/IP/口令/私钥入库。

## 8. 产物清单

- 本报告 `REPORT.md`；截图 19 张（`screenshots/`，含 H5 七页、admin 四视图、失败复盘复验、战报详情、不回退旁证）；
- 脚本/探针：`probe-init.js`、`auth-init.tpl.js`、`mint-token*.sh`、`dom-probe.js`、`defeat-probe.js`、`detail-probe.js`、`capture-*.sh` 等；
- 日志：`data/{h5,admin,admin2-5,fast,detail100,nr-admin2,nonreg}.log`。
