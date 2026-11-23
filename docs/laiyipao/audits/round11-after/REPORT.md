# R11 复查：数值格式化与移动视口审计（改造后真浏览器复查）

> 本轮为真实浏览器实测（Chrome 157，agent-browser 0.41.1 驱动），**非物理真机**。H5 页面强制 390×844 模拟移动视口，admin 为 1258×588 桌面视口。所有截图均为桌面浏览器视口渲染，不代表手机硬件。本地回环 QA（`:8080` API / `:5174` H5 / `:5175` admin / PG 容器回环 `15440→5432`），非线上。

## 1. 结论速览

- **t105/t110 数值格式化修复真浏览器复查 PASS（含真实 DB 数据决定性验证）**：skills 页 `.r6-ev-val` 由修复前 `19.133333333333333次/波` 变为 `19.1次/波`，`longTailFlag` 由 `true` 变 `false`。此前 t105/t110 的 LONGTAIL=false 仅在 network mock 夹具下成立，本轮首次用真实战报数据在真浏览器坐实修复。
- **5 处裸插值已修**：`r6-ev-val`（skills/equipment/reactions 三页）与 `r7BasisText`（skills/equipment 两页）均改走 `fmtEv`（数字→`fmt` 一位小数、字符串型 sha 证据原样保留、`null→—`）。
- **t106 移动视口审计复核 PASS**：§3 全 19 页 PASS、§4 五页假阳性有原子化 CSS 诊断 + 读图决定性证据、真 token 生成文件已删、零真值。
- **真浏览器抽查 H5 五页 + admin 看板全 PASS**：URL/title/结构三重确认非登录页，探针 pe/ce/af/fr 全 0，H5 五页全部 vw=390/vh=844、`longTailFlag=false`、水平溢出 0。
- **R1–R10 不回退**：顶栏 API 脱敏、R9 战斗 HUD 横排、R10 养成差值卡、index 攻击力诚实文案、AoE 25% 文案全在。
- **本轮未发现新的真实业务缺陷**。过程中出现的视口偏差、503、admin 跳登录页均为采集脚本/基础设施问题，非产品缺陷（详见 §8）。

## 2. 环境与方法

- 后端 `go run ./cmd/api`：`:8080`，`DATABASE_URL` 本地回环，`SETTLE_RECOMPUTE_MODE=shadow`；healthz=200 / readyz=200。
- PG 容器回环 `15440→5432`，QA 库 user10 有 3 条真实战报（score 9582/13268/11056）——这是本轮真实数据态的来源。
- 探针 `probe-init.js`（document-start 注入）：捕获 pageError / unhandledrejection / console.error / fetch≥400 / XHR≥400。
- H5 鉴权：从 DB 取 user10 guest_token → `POST /auth/guest` 重签 token → 占位符模板生成 document-start 注入脚本；采集后立即还原为占位符。
- admin 鉴权：从源码公开默认值（非真值）动态提取注入口令，登录后 URL 确认 `/admin/dashboard`；报告统一 `<ADMIN_PASS>` 占位。
- 关键教训：全新会话须先 `open about:blank` 预热再 `set viewport`，且设置后须 eval 验证；否则视口被丢弃为 1258×588。

## 3. 验收① t110 六张补丁截图亲读（三页两视口）

逐张读图确认（sha256 与 t110 README 一致）：

| 截图 | 视口 | 关键读数（肉眼可读） | 长浮点尾数 |
| --- | --- | --- | --- |
| after-skills-mobile.png (02ef6db8…) | 390×844 | 79.6% / 0.3 / cfg-sha-abc123；每波 5.4→9.7；三星率 33.3% | 无 |
| after-skills-desktop.png (160b1e4d…) | 1280×800 | 同上；每波 +4.3 (+79.6%)、战力 +0.3、评分 +4.0 | 无 |
| after-equipment-mobile.png (554eec53…) | 390×844 | 79.6% / 0.3 / cfg-sha-abc123；每波 5.4→9.7 | 无 |
| after-equipment-desktop.png (af7c8a26…) | 1280×800 | 同上 | 无 |
| after-reactions-mobile.png (fd236348…) | 390×844 | 主反应 100.0% / 0.3star / 153.0point | 无 |
| after-reactions-desktop.png (54b83656…) | 1280×800 | 同上 | 无 |

字符串型配置版本 `cfg-sha-abc123` 原样保留，未被误格式化；数字一位小数。t110 README 记 DOM `LONGTAIL=false`，读图与之一致。

## 4. 验收② 5 处裸插值修复 grep 核实

- `laiyipao/miniapp/src/pages/skills/skills.vue:43` → `{{ fmtEv(ev.value) }}{{ ev.unit }}`（原裸插值）
- `laiyipao/miniapp/src/pages/equipment/equipment.vue:48` → 同上
- `laiyipao/miniapp/src/pages/reactions/reactions.vue:48` → 同上（t110 新增）
- `skills.vue:446` / `equipment.vue:439` 新增 `fmtEv(v)`：`typeof v === 'number' ? fmt(v) : (v == null ? '—' : String(v))`
- `r7BasisText`（skills.vue:457 / equipment.vue:450）拼接 evidence 时逐一走 `fmtEv(e.value)`；`r7StarText` 复用 `fmt`。
- 五页 r7Impact/r10-gain/r10-growth 数值插值全走 `fmt`/`fmtSigned`（grep 逐行核对：skills/equipment 41 处、gems 4 处、battles 12 处、battle 12 处，无裸插值残留）。

## 5. 验收③④ 真浏览器抽查（真实 DB 数据，非 mock）

| 页面 | 视口 | longTailFlag | `.r6-ev-val` 实测 | pe/ce/af/fr | 三重确认 |
| --- | --- | --- | --- | --- | --- |
| skills 技能图鉴 | 390×844 | false | 19.1次/波 / 17.7次/波 / 0.0% | 0/0/0/0 | 技能图鉴/非登录 |
| equipment 装备图鉴 | 390×844 | false | 100.0% / 0.3star / 153.0point | 0/0/0/0 | 装备图鉴/非登录 |
| gems 宝石图鉴 | 390×844 | false | —（无 evidence 区，空态诚实） | 0/0/0/0 | 宝石图鉴/非登录 |
| battles 我的战报 | 390×844 | false | — | 0/0/0/0 | 我的战报/非登录 |
| reactions 反应图鉴 | 390×844 | false | 100.0% / 0.3star / 153.0point | 0/0/0/0 | 反应图鉴/非登录 |
| admin 数据看板 | 1258×588 | — | kicker 三段 / AI 洞察卡 / apiTag=API回环 | 0/0/0/0 | 数据看板/非登录 |

**决定性证据**：skills 页 `.r6-ev-val` 首轮采集（t107 早期、t110 修复前）真值为 `19.133333333333333次/波`，本轮重采集为 `19.1次/波`；`longTail` 数组由 4 条长浮点变空。这是在真实 3 场战报数据下、经完整 fetch→render 链验证，排除了 t105/t110 仅 mock 夹具覆盖的疑虑。

## 6. 验收⑤ R1–R10 不回退

- 顶栏 API 脱敏（t33）：`admin/src/utils/apiLabel.ts` 的 `isLoopbackApiBase()` 仅对回环（127.0.0.1/localhost/::1/0.0.0.0）或显式调试开关显示完整值，其余显示「API」；本次本地回环显示 `API 127.0.0.1:8080` 属设计允许的回环分支（非真值）。
- R9 战斗 HUD 横排（`battle.vue:1784`）：`.hud-row{display:flex;flex-direction:row;flex-wrap:wrap}`，`.hud-pill{flex-shrink:0;white-space:nowrap}`；t106 亦确认 390×844 下 HUD 横排未回退为竖排。
- R10 养成前后差值卡：`battles.vue:188` / `battle.vue:291` `.r10-growth`「AI · 养成前后（同一关可信差值）」+ replay_hash 哈希并列均在。
- index 攻击力诚实文案（`index.vue:310`）：「反应伤害中元素侧是主力，面板攻击力也有贡献但占比受限（权重上限 30%…）」未回退。
- AoE 反应邻居 25%：t44 线上已验 pct25=2；本轮源码与图例文案未发现回退。

## 7. 验收⑥ 零真值

- H5 注入脚本 `auth-init.js` 已用模板还原（含 `__LYP_TOKEN__` 占位符，无 JWT 字面量）。
- 全目录扫描：`eyJ…` JWT 0、`<ADMIN_PASS>` 口令 0（admin 诊断日志已掩码为 `<ADMIN_PASS>`）、非回环域名 0、私钥 PEM 0。
- 6 张本轮截图二进制携带本地回环地址（127.0.0.1:8080 / 127.0.0.1:5174/5175），无真域名/IP/口令。

## 8. 假阳性排除与过程记录

- **视口 1258×588（首轮 H5/eactions）**：`set viewport` 在浏览器 `about:blank` 预热前设置被丢弃；加 `sleep` + eval 验证 + 必要时 reload 后均为 390×844。非产品缺陷。
- **API 失败 af=4 / 503**：过程中 PG 容器被停（Docker daemon 退出），后端 `acquire db connection` 报 503；重启容器并设 `--restart unless-stopped` 后 af=0。基础设施问题，非产品缺陷。
- **admin 跳登录页**：二次 `open --init-script` 重建了浏览器上下文、丢失 token；改为单次会话内登录 + 注入探针后 `isLogin=false`、URL=/admin/dashboard。采集脚本问题，非产品缺陷。
- 本轮无真实业务缺陷上报。

## 9. 产物清单

- 报告：`docs/laiyipao/audits/round11-after/REPORT.md`（本文件）
- 截图（非物理真机）：`screenshots/r11-{skills,equipment,gems,battles,reactions}.png`（390×844）+ `r11-admin-dashboard.png`（1258×588），共 6 张。
- 探针/脚本：`probe-init.js`、`auth-init.tpl.js`（占位符模板）、`auth-init.js`（已还原占位符）、`dom-probe.js`、`probe-detail.js`、`full-basis.js`、`locate-longtail.js`、`admin-probe.js`、`mint-token.sh`、`capture-h5.sh`、`capture-admin*.sh`、`capture-reactions.sh`、`admin-login-diag.sh`。
- 日志：`data/{h5-rerun2,admin-final,reactions2,admin-diag2}.log` 等。

全程未执行任何 git 操作，未改业务源码，未部署。团队总目标 ≥1000 轮，本轮不宣称整体完成。