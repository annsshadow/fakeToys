# Round11 部署后线上回归报告

> 测试环境真机实测记录。域名一律用 `<YOUR_TEST_DOMAIN>` 占位，真值只从 `laiyipao/deploy/.local.keys` 读取，不入库、不回显。
> 截图由 agent-browser 无头真浏览器采集，**非物理真机**（H5 视口 390×844 模拟移动端，admin 1258×588 桌面；均非实体设备）。

## 1. 部署结果

| 端 | 产物 | 校验 |
|---|---|---|
| 运营后台 admin | `assets/index-DnuqK3SL.js` | 远端 bundle 与本地构建一致，HTTP 200（R11 未改 admin 前端产物内容） |
| 小程序 H5 | `assets/index-CMRZW06X.js` | 远端 bundle 与本地构建一致，HTTP 200（R11 数值格式化更新） |
| 后端 api | `lyp-server-amd64` 17305762 B | sha256 `2ee4a2a2728899f60360520f22955cbd96850128921a3c33801fd7ee12b7aaf3` |

- 构建自干净 HEAD（构建前 `git diff HEAD --quiet` 通过），三端 EXIT=0
- systemd `laiyipao.service` = **active**，`/healthz` = **200**，`/readyz` = **200**

## 2. HTTP 端到端抽样（全部 200）

- admin 登录 200、dashboard 200（`stage_funnel` array[4]、`daily_active` array[14]、`reaction_usage` array[7]）
- `/config` 200（skills 24 / gems 8）
- 用户推荐 `me/recommendations?context=skills|equipment|reactions` 全 200，各 1 条 insight / CTA
- impact 接口：CTA 未携带 `id/replace`（构筑无可比配置），与 R7-R10 一致诚实空态，非 R11 门禁项
- 养成差值数据源：`/me/battles` 200 count=17、`/me/battles/19` 200（完整 duration/reactions/elements_used 字段）
- 回放结算：游客建号 200 → battle token 200 → settle **200**，`loot.coin=+230`，返回 wallet/power/build_rating/new_max_stage 完整
- `HEAD_TRUTH_LEAK=0`

## 3. 真浏览器实测（agent-browser，**非物理真机**）

### 运营后台 admin
- 登录后进入 dashboard；真实 AI 运营洞察卡可见：「第 1 关流失最大（尝试 16 → 通关 7，净流失 9），建议优先复核关卡难度」，含真实数值与依据
- 核心指标：注册用户 17、战斗总数 19、平均战力/关卡 78.529、验真一致率 100.0%
- 顶栏脱敏：`domainInBody=false`
- 探针：pageErrors / consoleErrors / apiFailures / failedRequests **全 0**
- 观察（如实记录）：agent-browser 下 content/equipment/skills/wiki 子视图截图 sha 与 dashboard 完全相同（`28506469d8719079`），即路由回退到 dashboard——R1-R10 已知的 agent-browser 路由行为，非 R11 门禁失败；DOM 探针在切换瞬间捕获到 content 视图的 AI 卡文本（含「关卡漏斗柱状图」），说明视图内容本身正常

### 小程序 H5（R11 核心：数值格式化线上验证）
- 首页 AI 战术洞察卡：「装备契合仅 0%（权重 21%）：优先补它，收益 378」，含五维评分雷达图
- 关卡页：星级口径卡「本关满分 18284 / 1★门槛 10970（约满分的60%）」，机制拐点 10 个、图例 ◆▲■ 齐全（R8 不回退）
- 成长页 skills/equipment/gems：AI 因果链真实文本「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」，诚实空态「收益数据不可用：没有真实收益预估，不展示预计数值」（R10 不回退）
- **数值格式化线上决定性证据**：保留登录态的精准探针（`lyp_r11_numfmt_final.sh`）在真实线上数据下实测 skills 页 `.r6-ev-val` = `["43.8%","0.1star","175.0point"]`，全文长浮点正则 `\d.\d{5,}` = **false**，因果链文本 causal=true，探针 pe/ce/af/fr 全 0，domainInBody=false——数值已按一位小数格式化、无长浮点尾数，R11 核心修复在线上坐实（非仅 mock 夹具）
- 顶栏脱敏：`domainInBody=false`
- 探针：全 0
- 观察（如实记录）：`numfmt` 首版独立 session 因 localStorage.clear() 清空登录态 + 该 session 未同步战报，skills 页显示「没有战报」空态（`evVals:[]`）——系采集脚本副作用，非线上缺陷；精准探针保留登录态后取得上述决定性证据

## 4. 截图清单（sha256 前 16 位）

| 截图 | 视口 | sha256[:16] |
|---|---|---|
| r11-online-admin-{dashboard,content,equipment,skills,wiki}.png | 1258×588 | 均 28506469d8719079（agent-browser 路由回退，见 §3） |
| r11-online-h5-index.png | 390×844 | 6db38659cb141cb6 |
| r11-online-h5-stage.png | 390×844 | 174c0fa2d7964d3f |
| r11-online-h5-skills.png | 390×844 | 626eaa9e34cf8302 |
| r11-online-h5-equipment.png | 390×844 | c3b0766eb90997e8 |
| r11-online-h5-gems.png | 390×844 | 967a1e0cd6c5b34a |
| r11-online-h5-skills-numfmt.png | 390×844 | b2f8fcc633dd75d8 |
| r11-online-h5-skills-numfmt-final.png | 390×844 | 见文件（决定性证据截图） |
| r11-online-h5-equipment-numfmt.png | 390×844 | e0533109d4e5a249 |

## 5. 安全门禁

- 提交前全量真值扫描（lyp_t86_scan.sh）：eyJ JWT=0、五类真值=0、非回环 IP=0、PEM=0、root@IP=0、口令URL=0、PNG 二进制 eyJ=0
- 三批源码/规范/审计提交（前序 attempt）：`73c34d4ef` / `d9d485ac3` / `4223094e3`，逐批 STAGED_LEAK_TOTAL=0
- JWT 守卫 `go test ./internal/domain -count=1` = ok（7.205s）
- 部署后截图与本报告复扫：文本真值 0、PNG 二进制 eyJ 0

## 6. 结论

R11 三端部署成功、线上端到端全绿、真浏览器 AI 视觉与数值格式化线上决定性证据（43.8%/0.1star/175.0point，longTail=false）确认、零真值泄露。R11 终审（t108 verdict=pass）五门禁在部署后经真浏览器二次确认；R1-R10 不回退。两处观察（admin 路由回退、numfmt 首版脚本登录态副作用）如实记录，均为采集脚本/已知行为，非 R11 门禁失败。
