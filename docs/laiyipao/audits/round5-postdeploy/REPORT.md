# Round5 测试环境部署复测报告

- 执行：release-ops（唯一提交者），2026-11-22（Asia/Shanghai）
- **截图说明：以下截图全部来自 agent-browser 真浏览器交互，标注为「非物理真机」**（admin 桌面视口 / H5 390×844 视口）。
- **真值说明：域名、管理口令、DB_DSN、JWT_SECRET 只在运行时从 `laiyipao/deploy/.local.keys` 读取，不回显、不入库；本文统一用 `<YOUR_TEST_DOMAIN>` / `<PASS>` 占位。**

## 部署产物（已上线测试环境）

| 产物 | 文件 | sha256 |
|---|---|---|
| admin | `assets/index-BWOASrsJ.js` | `b1a3066ce956e3f0a13d6d825518376d73d93e8741c0354bc883b8de3a449478` |
| H5 | `assets/index-CLl7NStc.js` | `3f79383dd35b09ba9299ffd6bb3774b4a0f8436d8e0140ed2630ba9568d72eda` |
| 后端 | `lyp-server-amd64` (linux/amd64, CGO_ENABLED=0) | `381ba816401b822e49577f3e5a5844b2a4a29c79cadf8284ceb2af167a857cd0` |

构建基于干净 HEAD `7e53767ba`（t50 verdict=pass 放行后的五批提交），VITE_API_BASE 构建时由环境变量烘焙，源码零真值。部署后 `systemctl is-active=laiyipao` → active，healthz/readyz 200。

## 端到端实测

- HTTP：`/laiyipao-api/healthz=200`、`/readyz=200`；admin/H5 静态入口与新 bundle 均 HTTP 200。
- API：admin login 200、dashboard 200；`stage_funnel=array[3]`、`daily_active=array[14]`、`reaction_usage=array[7]`（F-1 归一保持）；`/config` 200，skills=24（1 号 base_damage=100）、gems=8。
- 玩法闭环：玩家登录 200 → 战斗令牌 200 → 回放结算 200，返回 battle_id/loot(coin+230)/wallet/power/build_rating，防作弊校验与合法结算路径同时生效。
- 真浏览器冒烟（非物理真机）：admin dashboard 三段眉题 + AI 运营洞察卡 + 顶栏 API 标签；Skills/Users/Recomputes/Battles 四视图均见眉题、双级 KPI、FilterBar、DataGrid，AI 卡四判据（徽章/真实结论/依据/CTA）可见：技能页热量效率结论、用户页成长曲线结论、验真页不符率结论、战报页无异常诚实结论。H5 首页诚实文案「伤害上限 30%」可见；signin/stage 的 KPI 与 AI 领取/推进建议可见；login/verify 用轻量 `role=note` AI 提示；双端 pageError/consoleError/apiFailure/failedRequest 探针全 0。
- 顶栏脱敏：admin 各页 `apiTag=1`、`domainInBody=false`，t33 非回环域名脱敏未回退。

## 两个采集口径疑点澄清（非缺陷）

1. 公告页首轮探针 `kpi=0, empty=0`：线上公告列表确为空，页面按 §1A.4 显示诚实空态「暂无公告 / 新服或活动前公告为空是正常的」+`r3-empty` AI 智能空态（复测 `emptyBlock=1, r3Empty=1`）。首轮探针用了错误的选择器 `.empty-state`，实际类名是 `.empty-block` / `.r3-empty`，不是结构性升级缺失。
2. 战斗页首轮 `pct25=false`：25% 文案在「战场图例」遮罩内（按需打开，阅读时暂停战斗）。点击 `aria-label="战场图例"` 后复测 `pct25=true`，蒸汽爆发/过热两条反应均写明「半径…内的敌人受到 25% 范围伤害」，t41 AoE 修复未回退。

## 真值扫描

- 提交前逐批 `git diff --cached` 子串扫描 `STAGED_LEAK_TOTAL=0`；提交后 `lyp_t43_scan.sh` 对 DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN 的 `COUNT_HEAD=0 COUNT_WT=0`。
- HEAD 全仓真域名子串扫描 `HEAD_TRUTH_LEAK=0`（t43 已脱敏历史文档）。
- 15 张 PNG 截图二进制真值子串扫描 `SHOT_LEAK_TOTAL=0`（见上文统计）。

## 截图清单（非物理真机）

`r5-online-admin-dashboard.png`、`r5-online-admin-skills.png`、`r5-online-admin-users.png`、`r5-online-admin-recomputes.png`、`r5-online-admin-battles.png`、`r5-online-h5-index.png`、`r5-online-h5-announcements.png`、`r5-online-h5-announcements-recheck.png`、`r5-online-h5-signin.png`、`r5-online-h5-stage.png`、`r5-online-h5-verify.png`、`r5-online-h5-login.png`、`r5-online-h5-battle.png`、`r5-online-h5-battle-recheck.png`、`r5-online-h5-legend25.png`。
