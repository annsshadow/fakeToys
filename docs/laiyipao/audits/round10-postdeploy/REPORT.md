# Round10 部署后线上回归报告

> 测试环境真机实测记录。域名一律用 `<YOUR_TEST_DOMAIN>` 占位，真值只从 `laiyipao/deploy/.local.keys` 读取，不入库、不回显。
> 截图由 agent-browser 无头真浏览器采集，**非物理真机**（H5/战斗视口 390×844 模拟移动端，admin 1258×588；均非实体设备）。

## 1. 部署结果

| 端 | 产物 | 校验 |
|---|---|---|
| 运营后台 admin | `assets/index-DnuqK3SL.js` | 远端 bundle 与本地构建一致，HTTP 200（R10 未改 admin 前端产物） |
| 小程序 H5 | `assets/index-CWsvkqKR.js` | 远端 bundle 与本地构建一致，HTTP 200（R10 更新产物） |
| 后端 api | `lyp-server-amd64` 17305762 B | sha256 `9be7e1ea3b09d6ebdea924f10498ac5a0945873d2e473fb569f0acec67b6276f` |

- 构建自干净 HEAD（构建前 `git diff HEAD --quiet` 通过），三端 EXIT=0
- systemd `laiyipao.service` = **active**，`/healthz` = **200**，`/readyz` = **200**

## 2. HTTP 端到端抽样（全部 200）

- admin 登录 200、dashboard 200（`stage_funnel` array[4]、`daily_active` array[14]、`reaction_usage` array[7]）
- `/config` 200（skills 24 / gems 8）
- 用户推荐 `me/recommendations?context=skills|equipment|reactions` 全 200，各 1 条 insight / CTA
- **养成前后差值接口**（R10 门禁项）：`/me/battles?limit=20&offset=0` = **200**，返回 16 条真实战报（字段 id/level_id/result/stars/score/kills/leaked/created_at/replay_hash）；`/me/battles/{id}` = **200**，详情含 hp_left/wave_reached/duration_ms/shots/hits/reactions/heat_max/elements_used/reactions_used。养成前后差值由这两次真实结算记录计算（t101 已实测同 session 连打两局 hash bd49af569c6ea194 / 7d18a4ed3fd7b40d 的 13120→10614 差值卡）
- 回放结算：游客建号 200 → battle token 200 → settle **200**，`loot.coin=+230`，返回 wallet/power/build_rating/new_max_stage 完整
- impact 接口：CTA 未携带 `id/replace`（该账号构筑无可比配置），与 R7-R9 一致；R10 已由决定性 HTTP E2E 辟谣（t100：未拥有候选时服务端故意不下发参数，写入端 400 防作弊），非 R10 门禁失败
- `HEAD_TRUTH_LEAK=0`

## 3. 真浏览器实测（agent-browser，**非物理真机**）

### 运营后台 admin
- 登录后进入 dashboard；dashboard/content/equipment/skills 四视图均渲染真实 AI 运营洞察卡：
  - dashboard「第 1 关流失最大（尝试 15 → 通关 7，净流失 8），建议优先复核关卡难度，关卡漏斗柱状图，共 4 关」
  - content「当前配置版本 6237e0b623219b8b，反应条目 7 类，累计战报 18count，累计胜场 10count」
  - equipment「玩家平均战力 76，平均关卡 0.4」
  - skills「「侦察无人机」热量效率最低（每点热量 0.0 DPS），建议优先复核数值平衡，共 42 个技能 / 18 条配方」
- 顶栏脱敏：`domainInBody=false`（正文无真实域名）；探针 pe/ce/af/fr **全 0**
- 亲读截图确认：`r10-online-admin-dashboard.png`（1258×588，sha `6b11b4cc…`）AI 洞察卡 + 核心指标（注册用户 17 / 战斗总数 18 / 平均战力 75.588 / 验算一致率 100%）真实可见
- 观察（如实记录）：`wiki` 视图截图 sha 与 dashboard 相同（`6b11b4cc…`），路由探针回退到 `/admin/dashboard`（R1-R9 已知的 agent-browser 导航行为）；R10 未改 wiki 路由，本项非 R10 门禁判据

### 小程序 H5（R10 核心：养成因果链与收益可解释）
- 首页 AI 战术洞察卡：「装备契合仅 0%（权重 21%）：优先补它，收益 378」，含五维评分雷达图
- 关卡页（R8 不回退）：星级口径卡真实显示「本关满分 18284 / 1★门槛 10970（约满分的60%）/ 难度 1129‰ / 波次 5 / 元素上限 2」；机制拐点 `.diff-milestone`=10 个、图例 ◆首领 ▲本关敌种 ■地形 齐全
- 成长页 skills/equipment/gems（R10 核心数据态）：AI 因果链卡真实文本「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」；诚实空态「换上推荐：收益数据不可用：没有真实收益预估，不展示预计数值」；gems 页「宝石页只提供收集线索：「锋锐石」还没拥有（攻击）：当前 0/8」
- 亲读截图确认：`r10-online-h5-skills.png`（390×844，sha `2e896cf0…`）「真实收益预估 · 当前 vs 推荐」卡显示「当前 — → 推荐 —」双空态，依据「收益数据不可用：服务端确认：已拥有技能里没有同为主动且非当前上场（id=4）的候补；本次没有可执行的战斗/装备替换。不展示虚构数值」——R10 核心承诺「养成收益可解释且不虚构」真实数据态落地
- 顶栏脱敏：`domainInBody=false`；探针 pe/ce/af/fr **全 0**

### 战斗页（R9 不回退）
- 390×844 视口实测：四枚 HUD 胶囊「波次 1/5」「击杀 0」「距 1 星 11941」「钳分中·距满分 132s」全部 `nowrap`/`flex-shrink:0`，宽 59/42/79/110×25 px 单行可读
- Canvas=1、domainInBody=false、探针全 0；`r10-online-h5-battle-mobile.png`（sha `bc9f549e…`）

### 观察（如实记录，非 R10 门禁）
- skills 页 AI 洞察卡数值出现浮点未格式化（`46.666666666666664%` 近 7 场胜率、`0.06666666666666667` 场均星），属显示精度小瑕疵；本任务为提交+部署 scope，未改业务代码，留待后续轮次优化
- admin wiki 路由在 agent-browser 下回退 dashboard（R1-R9 已知）

## 4. 截图清单（sha256 前 16 位，共 11 张）

| 截图 | sha256[:16] |
|---|---|
| r10-online-admin-dashboard.png | 6b11b4cc5ec8da5e |
| r10-online-admin-content.png | d2649bc9b2d65669 |
| r10-online-admin-equipment.png | acb704dead3c0942 |
| r10-online-admin-skills.png | 5239003f65f191ed |
| r10-online-admin-wiki.png | 6b11b4cc5ec8da5e |
| r10-online-h5-index.png | f3f01238abc4c0e6 |
| r10-online-h5-stage.png | 57d70d8555f7f970 |
| r10-online-h5-skills.png | 2e896cf046156875 |
| r10-online-h5-equipment.png | 51cb647db87e6168 |
| r10-online-h5-gems.png | c2a35d929b4d7fef |
| r10-online-h5-battle-mobile.png | bc9f549e02c3876e |

## 5. 安全门禁

- 提交前全量扫描 99 个 R10 待提交文件：eyJ JWT=0、五类真值(DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN)=0、非回环 IP=0、PEM=0、root@IP=0、口令URL=0、PNG 二进制 eyJ=0
- 审计脚本中 `auth-init.js`/`auth-init.tpl.js`/`mint-token.sh` 均为占位符模板（`<YOUR_TEST_TOKEN>` / `__LYP_TOKEN__` 运行时替换），无真 token 入库
- 五批精确 `git add --` 显式路径，逐批 `STAGED_LEAK_TOTAL=0`，无 `git add -A/.`
- 另按队长指示补提 R9 遗留 `docs/laiyipao/audits/round9-wiki-online/`（15 文件）：单独扫描 EYJ=0、五类真值=0、非回环IP=0、PNG 二进制 eyJ=0

## 6. 结论

R10 三端部署成功、线上端到端全绿（含养成前后数据源 /me/battles 200）、真浏览器养成因果链与「收益不虚构」诚实空态真实可见、零真值泄露。R10 终审（t103 verdict=pass）六门禁在部署后经真浏览器二次确认；R1-R9 不回退。两处观察（admin wiki 回退、skills 浮点精度）如实记录，均非 R10 门禁失败。
