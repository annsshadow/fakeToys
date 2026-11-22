# Round8 部署后线上回归报告

> 测试环境真机实测记录。域名一律用 `<YOUR_TEST_DOMAIN>` 占位，真值只从 `laiyipao/deploy/.local.keys` 读取，不入库、不回显。
> 截图由 agent-browser 无头真浏览器采集，**非物理真机**（H5 视口 390×844 模拟移动端，admin 1258×588；均非实体设备）。

## 1. 部署结果

| 端 | 产物 | 校验 |
|---|---|---|
| 运营后台 admin | `assets/index-C9gIWE3b.js` | 远端 bundle 与本地构建一致，HTTP 200（R8 未改 admin 前端产物内容） |
| 小程序 H5 | `assets/index-jYs6k2nf.js` | 远端 bundle 与本地构建一致，HTTP 200（R8 更新产物） |
| 后端 api | `lyp-server-amd64` 17305762 B | sha256 `c118f543cb6c6f95adbcf1a47f6fa97642929568a8c360cd6536d1928a9ff1c4` |

- 构建自干净 HEAD（构建前 `git diff HEAD --quiet` 通过），三端 EXIT=0
- systemd `laiyipao.service` = **active**，`/healthz` = **200**，`/readyz` = **200**

## 2. HTTP 端到端抽样（全部 200）

- admin 登录 200、dashboard 200（`stage_funnel` array[4]、`daily_active` array[14]、`reaction_usage` array[7]）
- `/config` 200（skills 24 / gems 8）
- 用户推荐 `me/recommendations?context=skills|equipment|reactions` 全 200，各 1 条 insight / CTA
- 回放结算：游客建号 200 → battle token 200 → settle **200**，`loot.coin=+230`，返回 wallet/power/build_rating/new_max_stage 完整
- impact 接口：CTA 未携带 `id/replace`（该账号构筑无可比配置），与 R7 一致；前端据此展示诚实空态「收益数据不可用，不展示预计数值」，符合 R7-BUG-1/2 修复约定，非 R8 门禁项
- `HEAD_TRUTH_LEAK=0`

## 3. 真浏览器实测（agent-browser，**非物理真机**）

### 运营后台 admin
- 登录后进入 dashboard；dashboard/content/equipment/skills 四视图均渲染真实 AI 运营洞察卡，例如「第 1 关流失最大（尝试 13 → 通关 7，净流失 6），建议优先复核关卡难度」「玩家平均战力 70，平均关卡 0.4」「「侦察无人机」热量效率最低（每点热量 0.0 DPS）」，含真实数值与依据
- 顶栏脱敏：`domainInBody=false`（正文无真实域名）
- 探针：pageErrors / consoleErrors / apiFailures / failedRequests **全 0**
- 观察（如实记录）：`wiki` 视图探针显示路由回退到 `/admin/dashboard`（R1-R7 已知的路由行为），该截图与 dashboard 画面一致，不作为 R8 门禁判据；AI 四判据已在其余四视图确认可见

### 小程序 H5
- 首页 AI 战术洞察卡：「装备契合仅 0%（权重 21%）：优先补它，收益 378」，含五维评分雷达图
- 关卡页（R8 核心）：星级口径卡真实显示「本关满分 18284 / 1★门槛 10970（约满分的60%）/ 难度 1129‰ / 波次 5 / 元素上限 2」，文案「按本关满分百分比给星……不代表高关更容易」；机制拐点 `.diff-milestone`=10 个、图例 ◆首领 ▲本关敌种 ■地形 齐全；AI 建议卡为真实机制结论「第 5 关是机制拐点：本关敌种 游荡者（新机制 · 调整对空/远减/清场手段）……不能只看分数斜率」，已非 +33‰ 假洞察
- 成长页 skills/equipment/gems：AI 因果链真实文本「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」，诚实空态「收益数据不可用：没有真实收益预估，不展示预计数值」
- 顶栏脱敏：`domainInBody=false`
- 探针：全 0
- 观察（如实记录）：脚本 URL 带 `?level_id=98`，页面仍渲染存档当前关（第 5 关）——stage 页按存档当前关展示，星级数值随配置变化而非硬编码；t84/t85 已通过其它路径实测 L98/L100（33646/20187、52311/31386），本项不构成 R8 门禁失败

## 4. 截图清单（sha256 前 16 位）

| 截图 | sha256[:16] |
|---|---|
| r8-online-admin-dashboard.png | e39b442b19ceed7f |
| r8-online-admin-content.png | 325b743714bbee87 |
| r8-online-admin-equipment.png | 4c23773751b9f5de |
| r8-online-admin-skills.png | 5239003f65f191ed |
| r8-online-admin-wiki.png | e39b442b19ceed7f |
| r8-online-h5-index.png | 586c71b0ec2da31c |
| r8-online-h5-stage.png | 813668311205207a |
| r8-online-h5-skills.png | dc681b080eb4521f |
| r8-online-h5-equipment.png | 72e911bc3ae4765b |
| r8-online-h5-gems.png | f3ec4e24c9fbceef |

## 5. 安全门禁

- 提交前全量扫描 58 个 R8 待提交文件：eyJ JWT=0、五类真值(DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN)=0、非回环 IP=0、PEM=0、root@IP=0、口令URL=0、PNG 二进制 eyJ=0
- 五批精确 `git add --` 显式路径，逐批 `STAGED_LEAK_TOTAL=0`，无 `git add -A/. `
- 部署后截图与本报告复扫：文本真值 0、10 张 PNG 二进制 eyJ 0

## 6. 结论

R8 三端部署成功、线上端到端全绿、真浏览器 AI 视觉与 R8 星级口径/机制拐点诚实文本真实可见、零真值泄露。R8 终审（t85 verdict=pass）六门禁在部署后经真浏览器二次确认；R1-R7 不回退。两处观察（admin wiki 路由回退、stage 按存档当前关展示）如实记录，均为已知行为，非 R8 门禁失败。
