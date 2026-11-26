# Round7 部署后线上回归报告

> 测试环境真机实测记录。域名一律用 `<YOUR_TEST_DOMAIN>` 占位，真值只从 `laiyipao/deploy/.local.keys` 读取，不入库、不回显。
> 截图由 agent-browser 无头真浏览器采集，**非物理真机**（H5 视口 390×844 模拟移动端，非实体手机）。

## 1. 部署结果

| 端 | 产物 | 校验 |
|---|---|---|
| 运营后台 admin | `assets/index-C9gIWE3b.js` | 远端 bundle 与本地构建一致，HTTP 200 |
| 小程序 H5 | `assets/index-CjVeVLG_.js` | 远端 bundle 与本地构建一致，HTTP 200 |
| 后端 api | `lyp-server-amd64` 17305762 B | sha256 `8170ab232926241df6238722fe0d4804dacd41a1a3a4e3b3613e2aa9057e2e6f` |

- systemd `laiyipao.service` = **active**
- `/healthz` = **200**，`/readyz` = **200**
- 构建自干净 HEAD（构建前 `git diff HEAD --quiet` 通过）

## 2. HTTP 端到端抽样（全部 200）

- admin 登录 200、dashboard 200（`stage_funnel` array[4]、`daily_active` array[14]、`reaction_usage` array[7]）
- `/config` 200（skills 24 / gems 8）
- 用户推荐 `me/recommendations?context=skills|equipment|reactions` 全 200，各 1 条 insight / CTA
- `me/recommendations/impact` 抽样 4 组参数（skill 2→1、3→2，equipment 11→10、2→1）全 **200**，`schema_version=rec-v1`，`impact=true`
- 参数校验：缺 kind / 缺 replace / 非整数 id 均 **400**（模糊输入不静默解析）
- 回放结算：游客建号 200 → battle token 200 → settle **200**，`loot.coin=+230`，返回 wallet/power/build_rating/new_max_stage 完整

> 说明：用户侧真实推荐 CTA 未携带 `id/replace`（该账号构筑无满足推导条件的可比配置），前端据此跳过 impact 调用、展示诚实空态「收益数据不可用，不展示预计数值」，与 R7-BUG-1/2 修复约定一致；手动传合法 id/replace 的 impact 接口本身可用（上列 4 组 200）。

## 3. 真浏览器实测（agent-browser，**非物理真机**）

### 运营后台 admin
- 登录后进入 dashboard；四视图 content/equipment/skills/wiki 均渲染真实 AI 运营洞察卡（如「第 1 关流失最大（尝试 12 → 通关 7，净流失 5）」「玩家平均战力 67」「侦察无人机热量效率最低」，含真实数值与依据）
- 顶栏脱敏：`apiTag=1`，`domainInBody=false`（正文无真实域名）
- 探针：pageErrors / consoleErrors / apiFailures / failedRequests **全 0**
- 截图：`screenshots/r7-online-admin-dashboard.png`、`r7-online-admin-{content,equipment,skills,wiki}.png`（1258×588）

### 小程序 H5
- 首页 AI 战术洞察卡：「装备契合仅 0%（权重 21%）：优先补它，收益 378」，含五维评分雷达图
- 成长页 skills/equipment/gems：AI 因果链卡真实文本「本局靠焰 3 次推进，触发 10 次反应（overheat 2 次），拿到 1000 分、0 星」，诚实空态「收益数据不可用，不展示预计数值」
- 顶栏脱敏：`domainInBody=false`
- 探针：全 0
- 截图：`screenshots/r7-online-h5-{index,skills,equipment,gems}.png`（390×844）

## 4. 安全门禁

- 新增 `TestNoJWTLiteralInRepo`（`secrets_leak_test.go`）：test-first RED_EXIT=1（命中历史证据 + 2 个 R7 注入脚本）→ 脱敏后 GREEN_EXIT=0；白名单仅放 OIDC 假 token 夹具 `oidc.rs`
- 3 处真 token 已替换为 `<YOUR_TEST_TOKEN>`，全程未回显 token 原文；全仓 eyJ 三段式 JWT 仅剩白名单 1 处
- 五类真值（DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN）：HEAD 与工作树 COUNT 全 0；`HEAD_TRUTH_LEAK=0`
- 五批源码/证据提交逐批 `STAGED_LEAK_TOTAL=0`；第五批 65 张 PNG 二进制 eyJ 扫描 0 命中

## 5. 结论

R7 三端部署成功、线上端到端全绿、真浏览器 AI 视觉与交互真实可见、零真值泄露。R7-BUG-1/2 双闭环在部署后经 impact 接口抽样与诚实空态双重确认。
