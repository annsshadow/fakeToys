# Round12 部署后线上回归报告

> 测试环境真机实测记录。域名一律用 `<YOUR_TEST_DOMAIN>` 占位，真值只从 `laiyipao/deploy/.local.keys` 读取，不入库、不回显。
> 截图由 agent-browser 无头真浏览器采集，**非物理真机**（H5 视口 390×844 模拟移动端，admin 1258×588 桌面；均非实体设备）。

## 1. 部署结果

| 端 | 产物 | 校验 |
|---|---|---|
| 运营后台 admin | `assets/index-DnuqK3SL.js` | 远端 bundle 与本地构建一致，HTTP 200（R12 未改 admin 前端产物内容） |
| 小程序 H5 | `assets/index-CIODp3Gc.js` | 远端 bundle 与本地构建一致，HTTP 200（R12 失败复盘 / 成长优先级卡更新） |
| 后端 api | `lyp-server-amd64` 17305762 B | sha256 `8b285b11ea0d099f4d20d085821fe45bb467b2b29d41783b87f5caf8da516252` |

- 三端构建自干净 HEAD（`ADMIN_EXIT=0` / `H5_EXIT=0` / `SRV_EXIT=0`）
- systemd `laiyipao.service` = **active**，`/healthz` = **200**，`/readyz` = **200**，`-REMOTE_EXIT=0`

## 2. HTTP 端到端抽样（全部 200）

- admin 登录 200、dashboard 200（`stage_funnel` array[4]、`daily_active` array[14]、`reaction_usage` array[7]）
- `/config` 200（skills 24 / gems 8）
- 用户推荐 `me/recommendations?context=skills|equipment|reactions` 全 200，各 1 条 insight / CTA
- impact 接口：CTA 未携带 `id/replace`（构筑无可比配置），与 R7–R11 一致诚实空态，非 R12 门禁项
- 回放结算：游客建号 200 → battle token 200 → settle 200，`loot.coin=+230`
- `/me/battles` 200 含真实失败局（rows=18、lose=10），为失败复盘线上复现提供数据基础
- `HEAD_TRUTH_LEAK=0`

## 3. 真浏览器实测（agent-browser，**非物理真机**）

### 运营后台 admin
- 登录后进入 dashboard；真实 AI 运营洞察卡可见（content 视图探针捕获：「当前配置版本 6237e0b623219b8b，反应条目 7 类；累计战报 20count；累计胜场 10count」；equipment 视图：「玩家平均战力 81，平均关卡 0.4」；skills 视图：「侦察无人机 热量效率最低，建议优先复核数值平衡」）
- 五视图 dashboard/content/equipment/skills/wiki：`domainInBody=false`，探针 pe/ce/af/fr **全 0**
- 观察（如实记录）：agent-browser 下 admin wiki 截图 sha 与 dashboard 相同（`bd524a37042ea62f`），即路由回退到 dashboard——R1–R11 已知的 agent-browser 路由行为，非 R12 门禁失败；DOM 探针在切换瞬间捕获到各子视图真实 AI 卡文本，说明视图内容本身正常

### 小程序 H5
- 首页 AI 战术洞察卡：「装备契合仅 0%（权重 21%）：优先补它，收益 378」，五维评分雷达图齐全
- 关卡页：星级口径卡「本关满分 18284 / 1★门槛 10970（约满分的60%）」，机制拐点 10 个、图例 ◆▲■ 齐全（R8 不回退）
- 成长页 skills/equipment/gems：AI 因果链真实文本「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」，诚实空态「收益数据不可用：没有真实收益预估，不展示预计数值」（R10 不回退）
- 顶栏脱敏：`domainInBody=false`，探针全 0

### R12 核心修复线上决定性证据

**① 跨系统成长优先级卡（t112）** — H5 精准线上探针（`lyp_r12_postdeploy_probe.sh`，登录后真实数据）实测四页：

| 页面 | `.r12-priority-title` | `.r12-priority-text` | CTA | 探针 |
|---|---|---|---|---|
| skills | 当前最该强化 · 跨系统优先级 | 装备契合 0/18（0%，权重 21%）：补满可涨约 378 分，优先补它 | 去配装备 → | 全 0 |
| equipment | 同上 | 同上 | 同上 | 全 0 |
| gems | 同上 | 同上 | 同上 | 全 0 |
| bag | 同上 | 同上 | 同上 | 全 0 |

四页口径与首页完全一致，真实数据渲染（非空态假阳性），`domainInBody=false`。

**② 失败复盘（t111）** — H5 线上真实失败战报复现：战报列表 rows=18/lose=10，点击失败行后 `.defeat-review` 真实渲染：「AI · 失败复盘 / 第 1 波失守；漏怪 0/0（0%），击杀 0；反应 10 次；峰值热量 50/100；仅焰 1 种元素；防线压力高于本局构筑承受力；下一局优先升主技能或装备，并调整元素搭配提高反应覆盖；依据：wave/kills/leaked/reactions/heat_max/elements_used」，探针 pe/ce/af/fr **全 0**，`domainInBody=false`。线上失败结算卡与战报详情双端可见（round12-after 报告已用修复前后截图亲读坐实），胜利局不误报。

## 4. R1–R11 不回退

- 首屏新手引导（R11）：round12-after 探针 firstRunGuide=1 已验证
- AoE 25% / 半径120（R4）：战斗 HUD 反应提示文案保留
- 顶栏 API 脱敏（R6）：双端 `domainInBody=false`
- 数值格式化（R11）：`.r6-ev-val` 一位小数、无长浮点尾数（round12-after 数据）
- R7–R10 推荐 CTA / 养成因果链 / stage 星级口径均在线可见

## 5. 真值零泄露

- `lyp_t43_scan`：DOMAIN / ADMIN_PASS / DB_DSN / JWT_SECRET / BASEDOMAIN 的 `COUNT_HEAD=0 COUNT_WT=0`
- JWT 三段式守卫 `TestNoJWTLiteralInRepo`：go test ./internal/domain 全绿
- 部署后截图 12 张 PNG 二进制扫描 `eyJ`：0 命中
- 报告与脚本内真域名 / 口令 / token 均为占位符，无真值回显

## 6. 截图清单（全部非物理真机）

| 截图 | sha256(前16) |
|---|---|
| r12-online-admin-dashboard.png | bd524a37042ea62f |
| r12-online-admin-content.png | 81c6741ebccc5e00 |
| r12-online-admin-equipment.png | 603692b445324aa4 |
| r12-online-admin-skills.png | 5239003f65f191ed |
| r12-online-admin-wiki.png | bd524a37042ea62f（agent-browser 路由回退，同上观察） |
| r12-online-h5-index.png | 3702c17a245b654e |
| r12-online-h5-stage.png | 813668311205207a |
| r12-online-h5-skills.png | 4262a656f0af86f1 |
| r12-online-h5-equipment.png | b045abe708c7058b |
| r12-online-h5-gems.png | 98fd9ec0455d3280 |
| r12-online-growth-priority.png | d736aa7e6b99427e |
| r12-online-defeat-review.png | 60bd72cabe164ce9 |

## 7. 结论

R12 部署成功，三端健康；失败复盘与跨系统成长优先级卡两项核心修复在线上真实数据下实测通过，探针全 0、真值零泄露；R1–R11 不回退。admin wiki 截图路由回退为已知 agent-browser 行为，如实记录，不构成门禁失败。
