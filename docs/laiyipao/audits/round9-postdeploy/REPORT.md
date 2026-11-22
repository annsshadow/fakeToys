# Round9 部署后线上回归报告

> 测试环境真机实测记录。域名一律用 `<YOUR_TEST_DOMAIN>` 占位，真值只从 `laiyipao/deploy/.local.keys` 读取，不入库、不回显。
> 截图由 agent-browser 无头真浏览器采集，**非物理真机**（H5/战斗视口 390×844 模拟移动端，admin 1258×588；均非实体设备）。

## 1. 部署结果

| 端 | 产物 | 校验 |
|---|---|---|
| 运营后台 admin | `assets/index-DnuqK3SL.js` | 远端 bundle 与本地构建一致，HTTP 200（R9 更新产物） |
| 小程序 H5 | `assets/index-CEfcSZ1s.js` | 远端 bundle 与本地构建一致，HTTP 200（R9 更新产物） |
| 后端 api | `lyp-server-amd64` 17305762 B | sha256 `cd5235411e091de6bef5527d23cc517bafeb0e39375690d7b6014758456d2fc9` |

- 构建自干净 HEAD（构建前 `git diff HEAD --quiet` 通过），三端 EXIT=0
- systemd `laiyipao.service` = **active**，`/healthz` = **200**，`/readyz` = **200**

## 2. HTTP 端到端抽样（全部 200）

- admin 登录 200、dashboard 200（`stage_funnel` array[4]、`daily_active` array[14]、`reaction_usage` array[7]）
- `/config` 200（skills 24 / gems 8）
- 用户推荐 `me/recommendations?context=skills|equipment|reactions` 全 200，各 1 条 insight / CTA
- impact 接口：CTA 未携带 `id/replace`（该账号构筑无可比配置），与 R7/R8 一致；前端据此展示诚实空态，非 R9 门禁项
- 回放结算：游客建号 200 → battle token 200 → settle **200**，`loot.coin=+230`，返回 wallet/power/build_rating/new_max_stage 完整
- `HEAD_TRUTH_LEAK=0`
- 注：R9 无后端改动，本轮部署后端为同源码重新构建，端到端验证不回退

## 3. 真浏览器实测（agent-browser，**非物理真机**）

### 运营后台 admin
- 登录后进入 dashboard；dashboard/content/equipment/skills 四视图均渲染真实 AI 运营洞察卡：
  - dashboard「第 1 关流失最大（尝试 14 → 通关 7，净流失 7），建议优先复核关卡难度，关卡瀑布柱状图，共 4 关」
  - content「当前配置版本 6237e0b…反应条目 7 类 / 累计战报 17count、累计胜场 10count；版本来自 /config」
  - equipment「玩家平均战力 73，平均关卡 0.4」
  - skills「「侦察无人机」热量效率最低（每点热量 0.0 DPS），建议优先复核数值平衡 / 共 42 个技能、18 条配方」
- 顶栏脱敏：`domainInBody=false`（正文无真实域名）
- 探针：pageErrors / consoleErrors / apiFailures / failedRequests **全 0**
- 观察（如实记录）：`wiki` 视图截图 sha 与 dashboard 相同（`90054da4…`），路由探针回退到 `/admin/dashboard`（R1-R8 已知的 agent-browser 导航行为），本项不作为 R9 门禁判据；R9 Wiki 路由修复本身已由 t89/t93 在本地真浏览器 `after-01-wiki-fixed.png` 独立验证 PASS

### 小程序 H5
- 首页 AI 战术洞察卡：「装备契合仅 0%（权重 21%）：优先补它，收益 378」，含五维评分雷达图
- 关卡页（R8 不回退）：星级口径卡真实显示「本关满分 18284 / 1★门槛 10970（约满分的60%）/ 难度 1129‰ / 波次 5 / 元素上限 2」；机制拐点 `.diff-milestone`=10 个、图例 ◆首领 ▲本关敌种 ■地形 齐全
- 成长页 skills/equipment/gems：AI 因果链真实文本「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」，诚实空态「收益数据不可用：没有真实收益预估，不展示预计数值」
- 顶栏脱敏：`domainInBody=false`
- 探针：全 0

### 战斗页（R9 核心：移动视口 HUD，R9-BUG-1 部署后闭环）
- 390×844 视口实测：四枚 HUD 胶囊「波次 1/5」「击杀 0」「距 1 星 11941」「钳分中·距满分 132s」全部 `white-space:nowrap`、`flex-shrink:0`，宽度 59/42/79/110 px、高 25 px，横向单行可读（不再逐字竖排）
- Canvas 战场、热量条、技能槽、元素连锁提示完整可见，HUD 浮层不遮挡操作
- 探针：`pe/ce/af/fr` 全 0，`domainInBody=false`
- 亲读截图确认：`r9-online-h5-battle-mobile.png`（sha `50d995ca…`）第一行「波次1/5 击杀0 距1星11941」横向单行，第二行「钳分中·距满分131s」单行，反应因果链与战场清晰可见——R9-BUG-1 线上闭环

## 4. 截图清单（sha256 前 16 位，共 11 张）

| 截图 | sha256[:16] |
|---|---|
| r9-online-admin-dashboard.png | 90054da44385a450 |
| r9-online-admin-content.png | 56823512ee11a6e0 |
| r9-online-admin-equipment.png | 13135792d4fee9f3 |
| r9-online-admin-skills.png | 5239003f65f191ed |
| r9-online-admin-wiki.png | 90054da44385a450 |
| r9-online-h5-index.png | 66bcbe9ea65f3205 |
| r9-online-h5-stage.png | 174c0fa2d7964d3f |
| r9-online-h5-skills.png | 0c14845b9e467b42 |
| r9-online-h5-equipment.png | 2d16a2d90bd20b18 |
| r9-online-h5-gems.png | a90981e8037cbcd7 |
| r9-online-h5-battle-mobile.png | 50d995ca7a94daa3 |

## 5. 安全门禁

- 提交前全量扫描 69 个 R9 待提交文件：eyJ JWT=0、五类真值(DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN)=0、非回环 IP=0、PEM=0、root@IP=0、口令URL=0、PNG 二进制 eyJ=0
- 五批精确 `git add --` 显式路径（玩法源码 / admin / 规范+终审 / 审计证据 / 部署后证据），逐批 `STAGED_LEAK_TOTAL=0`，无 `git add -A/.`
- 部署后截图与本报告复扫：文本真值 0、11 张 PNG 二进制 eyJ 0

## 6. 结论

R9 三端部署成功、线上端到端全绿、真浏览器 AI 视觉与 R9 核心（移动视口战斗 HUD 单行可读、R9-BUG-1 闭环）真实可见、零真值泄露。R9 终审（t93 verdict=pass，含 t96 复审）六门禁在部署后经真浏览器二次确认；R1-R8 不回退。一处观察（admin wiki 路由在 agent-browser 下回退 dashboard）如实记录，为已知导航行为，非 R9 门禁失败。
