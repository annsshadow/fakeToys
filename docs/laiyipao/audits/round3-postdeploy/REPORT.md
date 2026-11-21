# Round3 测试环境部署后真页面回归报告（post-deploy）

> 环境标注：**真实测试环境**（经部署文档配置读取，真值仅从被 gitignore 的 `laiyipao/deploy/.local.keys` 注入内存）；浏览器为 **Windows Chrome headless（非物理真机）**，通过 agent-browser CDP 驱动。
> 脱敏声明：本报告、日志、截图中的测试域名统一写作 `<YOUR_TEST_DOMAIN>`，管理员口令写作 `<ADMIN_PASS>`；真实值不入库、不落日志、不进对话记录。
> 任务：t32（Round3 第三批部署后回归），依赖 t25 release-ops 提交部署。写域仅 `docs/laiyipao/audits/**`，未执行 git、未改源码。

## 1. 结论速览

| 验收锚点 | 结论 | 关键证据 |
|---|---|---|
| ① 真值从 gitignore 文件注入，报告/截图/日志占位 | PASS | 真值仅从主 checkout `.local.keys` 读入 shell 变量，所有输出经 sed mask；全目录 grep 真域名/口令无命中 |
| ② H5 六页 + skins 深链真浏览器逐页 | PASS | 7 页全部 `isLogin=false/hasToken=true`，探针 pe/ce/af/fr 全 0，六页 kicker/KPI/AI 卡/数据行齐全，skins 6 条皮肤 |
| ② admin 四视图 + Dashboard AI 卡 + 命令面板 AI 分区 | PASS | 四视图 URL/title/结构三重确认非登录页；Dashboard aiCard=1、三段 kicker、3 图表、F-1 红横幅未回归；Ctrl+K 面板 aiGroup=1/aiOptions=3 |
| ③ 端到端玩法闭环 | PASS | 真浏览器玩完第 1 关 5 波→结算「通关 / 金币 +1280 / 体力 +5 / 回放哈希 e7dd4d510ca4b」；`` GET /me/battles` 200，战报 id=10 score=11056 带合法 hash；config 技能 1 `base_damage=100` |
| ④ 截图/探针落库 + 非登录页假阳性核对 | PASS | 14 张截图全部 URL/title/结构/肉眼四重确认；含真域名的 admin `.api-tag` 已隐藏后重截脱敏 |
| ⑤ 诚实记录回退/部署差异 | PASS | 无功能回退；2 类自动播放策略噪声按观测口径如实定性（见 §5） |
| ⑥ 写域约束 | PASS | 仅写 `docs/laiyipao/audits/round3-postdeploy/**`；无 git/源码操作 |

## 2. 环境与方法

- **测试环境地址**：从 `laiyipao/deploy/README.md` 得路径规则（admin `/laiyipao-admin/`、H5 `/laiyipao-h5/`、API `/laiyipao-api/`），域名真值从主 checkout `laiyipao/deploy/.local.keys` 的 `DOMAIN` 行注入（当前 worktree 无该文件，已跨 checkout 定位）。
- **连通性**：HTTP 强制 301 → HTTPS；HTTPS 下 admin_root / api_healthz / api_readyz / h5_root / api_config **全部 200**。
- **登录**：测试环境 `PLAYER_LOGIN_REQUIRED=true`。H5 用共享玩家账号（与超管同名，口令 `<ADMIN_PASS>`）；admin 用 `admin/<ADMIN_PASS>`。
- **探针**：`probe-init.js` 注入 init-script，捕获 pageErrors / console.error / fetch≥400 / XHR≥400 / failedRequests。
- **浏览器**：agent-browser 0.41.1，H5 移动视口 390×844，admin 1258×588。

## 3. H5 逐页数据（会话 lyp-posth5b，真实登录态）

| 页面 | URL 路由 | kicker | KPI | AI 卡 | 数据行 | 探针 pe/ce/af/fr |
|---|---|---|---|---|---|---|
| 宝石图鉴 | /pages/gems/gems | 宝石收集雷达 | 0/8 | 「锋锐石」还没拥有 | 8/8 | 0/0/0/0 |
| 技能构筑 | /pages/skills/skills | 技能构筑台 | 攻城锤 | 取舍与热量 | 24/24 | 0/0/0/0 |
| 装备锻造 | /pages/equipment/equipment | 装备锻造台 | 母核碎片 | 搭配缺口 | 18/18 | 0/0/0/0 |
| 专精作战室 | /pages/mastery/mastery | 专精作战室 | 冰·3点 | 先补哪一系 | 9 | 0/0/0/0 |
| 商城决策 | /pages/shop/shop | 商城决策台 | 金币箱 | 买什么最划算 | 9 | 0/0/0/0 |
| 反应图鉴 | /pages/reactions/reactions | 反应作战台 | 4/7 | 下一条反应 | 7/7 | 0/0/0/0 |
| 皮肤（深链直入） | /pages/skins/skins | — | — | — | 6 条 | 0/0/0/0 |

所有页 `isLogin=false / hasToken=true / localStorage 含 lyp_token`——真实登录态，非游客假象。

**登录方式教训（重要）**：uni-app 的登录按钮是 `<uni-view class="btn btn-primary btn-dim">`，无 `<button>`/role。用 `eval .click()` 是 untrusted 点击，**不会真正建立会话**（localStorage 无 token，受保护页被 401 重定向 login，af 递增且假阳性）；必须用 agent-browser **原生 `click .btn-primary`** 才触发真实登录链。首次采集踩此坑，已用原生点击重采并全部 GREEN。

## 4. admin 逐页数据（会话 lyp-postadmin）

| 视图 | URL | title | kicker | 主/次 KPI | 数据行 | 探针 |
|---|---|---|---|---|---|---|
| Dashboard | /admin/dashboard | 数据看板 | 3 段（核心指标/经济与防线/趋势与分布） | kpiPrimary+kpiSecondary 各1，secondaryCards=4 | 3 图表 | 0/0/0/0 |
| 关卡配置 | /admin/levels | 关卡配置 | 关卡配置·关卡总览 | 2 主 + 2 次 | 100 | 0/0/0/0 |
| 技能与配方 | /admin/skills | 技能与配方 | 技能配置·元素与配方 | 2 主 + 2 次 | 42 | 0/0/0/0 |
| 用户管理 | /admin/users | 用户管理 | 用户管理·玩家档案 | 2 主 + 2 次 | 17 | 0/0/0/0 |
| 结算重算 | /admin/recomputes | 结算重算审计 | 结算审计·重算与不符 | 2 主 + 2 次 | 5 | 0/0/0/0 |

- **Dashboard AI 运营洞察卡**：aiCard=1；文案基于真实数据「第 1 关流失最大（尝试 7 → 通关 6，净流失 1）…关卡漏斗柱状图，共 3 关」——非模板空话。
- **F-1 回归检查**：`hasZeroFake=false`，零战报红横幅「后端不可达」未回归。
- **命令面板 AI 分区**：Ctrl+K 打开，`panel=1 / aiGroup=1(.cmd-ai-group-label) / aiOptions=3(.cmd-ai-option) / ariaModal=true / focusInInput=true`，AI 命令「✦ AI 洞察」分组可见。
- **假阳性核对**：每个视图均确认 URL 落在 `/admin/<view>`、title 为对应中文标题、结构探针非 0，排除「截图停在登录页」假阳性。

## 5. 端到端玩法闭环（会话 lyp-posth5b）

1. **申请凭证**：POST `/battle/token {level_id:1}` → token_id=24, seed=3897979951225094497, level 1 / 5 波, build_attacker=true（200）。
2. **真实对局**：真浏览器进入 battle，自动塔防 + 选牌自动点「全部不要了，直接下一波」推进；波次 1/5→5/5，击杀 0→39，防线满血 1000/1000。
3. **结算**：页面显示 **通关 / C 级 / 击杀 39 / 分数 11056 / 金币 +1280 / 体力 +5 / 回放哈希 e7dd4d510ca4b（可验真）**，并有「查看本局战报」入口。证明 settle 200 且金币/体力入账。
4. **战报生成**：GET `/me/battles?limit=3` → 200，`id=10, level=1, score=11056, replay_hash=e7dd4d510ca4`，与结算页一致。
5. **配置基线**：GET `/api/v1/config` → 技能 id=1「燃烧弹」`base_damage=100`（24 技能 / 100 关卡 / 18 配方 / 22 敌人，require_login=true）。

截图：`screenshots/post-e2e-battle-settle.png`（390×844，肉眼确认通关结算界面，非登录页）。

## 6. pageError 观测口径判定（诚实记录，非业务缺陷）

端到端会话探针最终读数 `pe=91`，逐条明细**全部**是 `unhandledrejection: The play() request was interrupted by a call to pause()`。判定为**浏览器自动播放策略噪声**，非本轮回归缺陷，依据：

- 该类型是 t6 已修复的 F-2；`sfx.ts` 的 `installMediaRejectionFilter()`（`initSfx()` 内注册）对 message 含 `play() request was interrupted` 的 rejection 调 `event.preventDefault()`。
- 探针在 init-script 最早期注册，**先于** sfx 过滤器捕获事件——探针记录到 ≠ 未被浏览器消费。此口径差异 t9 已用决定性实验（后置监听器读 `defaultPrevented`：interrupted=5 / preventedTrue=5 / preventedFalse=0）确认过。
- 本轮 `f2-decisive` 实验在 lyp-posth5b 重玩 18s：后置监听器读到 pageErrors=40，但类型是**另一类** `play() failed because the user didn't interact with the document first`（hash 直入无用户手势触发，sfx 过滤器按设计**不拦截**它以免静默真实异常）；interrupted 类为 0。干净会话 `f2-final` 则 pe=0。
- 两类 rejection 均不改变战斗结果（正常通关/入账/战报生成），不影响真实游玩体验。

## 7. 假阳性排除清单

1. **eval .click 假登录**：untrusted 点击不建会话→受保护页 401→login 假象；原生 `click .btn-primary` 后全部正常。
2. **apiFailures 403/401**：未登录阶段 `/auth/guest` 403 与受保护端点 401 是守卫正常响应，非业务 API 失败；真实登录后 af=0。
3. **pageError 两类自动播放噪声**：见 §6。
4. **admin 截图 `.api-tag` 真域名**：`AdminLayout.vue:390` 把 `VITE_API_BASE` 渲染在 UI 顶栏，截图含真域名；已在截图前用 eval `display:none` 隐藏并重截覆盖（脱敏），属既有设计非本轮回归，作为 low backlog 提示 captain（考虑生产环境是否隐藏 API 标签）。

## 8. 产物清单

- 报告：`docs/laiyipao/audits/round3-postdeploy/REPORT.md`（本文件）
- 截图（`screenshots/`，均标注真实测试环境 + 非物理真机）：post-admin-{dashboard,levels,skills,users,recomputes,palette}.png、post-h5-{gems,skills,equipment,mastery,shop,reactions,skins}.png、post-e2e-battle-settle.png 共 14 张。
- 日志（`data/`）：admin.log、h5.log、e2e-battle.log、config-env.json（游戏配置，非真值）、f2-final.log、f2-decisive.log、recapture-admin.log。
- 脚本/探针（目录根）：probe-init.js、capture-admin.sh、capture-h5.sh、verify-config.sh、probe-token.js、e2e-battle-check.js、e2e-battle-loop.sh、probe-*.js/sh、f2-*.js/sh、check-sfx-*.js/sh、recapture-admin-masked.sh、hide-apitag.js。
- 真值泄漏扫描：对 round3-postdeploy 全目录 grep 真域名/口令，输出 SCAN_DONE 无 LEAK 命中。

## 9. 脱敏复核（补充）

- admin 截图重截后 `.api-tag`（真域名标签）已用 eval `display:none` 隐藏并重截 6 张覆盖；肉眼复核 dashboard 截图顶栏仅余 admin/登出，无真域名。
- `strings` 对全部 14 张 PNG 二进制扫描真域名 → `PNG_SCAN_DONE` 无 LEAK；对全目录文本 grep 真域名/口令 → SCAN_DONE 无 LEAK。
- 运行时复核：`apiTagHidden=true, count=1`（标签存在但已隐藏，DOM 结构未破坏）。
