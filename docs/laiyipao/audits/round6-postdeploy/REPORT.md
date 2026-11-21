# Round6 测试环境部署复测报告

- 执行：release-ops（唯一提交者），2026-11-22（Asia/Shanghai）
- **截图说明：全部来自 agent-browser 真浏览器交互，标注为「非物理真机」**（admin 桌面视口 / H5 390×844 视口）。
- **真值说明：域名与口令只在运行时从 `laiyipao/deploy/.local.keys` 读取，不回显、不入库；本文统一用 `<YOUR_TEST_DOMAIN>` / `<PASS>` 占位。**

## 部署产物（已上线测试环境）

| 产物 | 文件 | sha256 |
|---|---|---|
| admin | `assets/index-C9gIWE3b.js` | `27c1b07bc366a8b2f1b9dafae32fe5bceb2c075b59b417c2854de3debb854702` |
| H5 | `assets/index-DVFZ7HO3.js` | `da98e8caae82d86e2783bfea0f4de5342d624ec1d37f739815752fec5de9ca2c` |
| 后端 | `lyp-server-amd64` (linux/amd64, CGO_ENABLED=0) | `32cf2dbba97fa27fb7cd4630ea9281d9713da085f5a17315ebeca83fe34cb3ca` |

构建基于干净 HEAD `75f5e8a29`（t58 verdict=pass 放行后的六批提交），VITE_API_BASE 构建时由环境变量烘焙。部署后 `systemctl is-active=laiyipao` → active，healthz/readyz 200，线上 bundle hash 与新产物一致。

## 端到端实测

- HTTP：healthz/readyz 200；admin/H5 静态入口与新 bundle 均 200。
- API：admin login 200、dashboard 200，`stage_funnel=array[4]`（本轮新增一关）、`daily_active=array[14]`、`reaction_usage=array[7]`；`/config` 200，skills=24（1 号 base_damage=100）、gems=8。
- 玩法闭环：玩家登录 200 → 战斗令牌 200 → 回放结算 200，返回 battle_id / loot(coin+230) / wallet / power / build_rating。
- 推荐接口联通：admin 侧 9 个 scope 中 8 个 200（`login` 400 为登录页不提供推荐的预期行为），玩家侧 3 个 context（equipment/reactions/skills）全部 200；响应 `schema_version=rec-v1`，含 `evidence`（累计战报 13、胜场 10、配置版本等）与 `insights`，非兜底伪造。
- 真浏览器（非物理真机）：admin content/defenses/equipment 与 `#/wiki` 四页 AI 洞察卡可见（wiki 用 `.ai-insight-card`，含「AI 运营洞察 · 规则引擎」+真实配置版本/战报结论+依据+CTA）；`#/admin/login` 未登录态 `role=note` AI 提示可见；H5 equipment/reactions/skills 三页 `.r6-rec` 推荐卡可见（标注「规则引擎 · rules rec-v1」，含近 7 场胜率 63.63%、构筑评分 175 等真实证据）。双端 pageError/consoleError/apiFailure/failedRequest 探针全 0。
- 顶栏脱敏：admin 各页 `apiTag=1`、`domainInBody=false`，t33 非回环域名脱敏未回退。

## 采集口径澄清（非缺陷）

1. 首轮 H5 三页 `aiTexts=[]`、wiki `rec=0` 是选择器误用：H5 推荐卡类名是 `.r6-rec`（不是 `[class*=ai-]`），WikiView 复用 `.ai-insight-card`。改用正确选择器后均得 1 且内容真实。
2. 首轮 admin route `#/admin/wiki` / 已登录态 `#/admin/login` 回退 dashboard：真实路由分别是 `#/wiki` 与未登录态 `#/admin/login`（router/index.ts 的 `/:pathMatch(.*)*` 兜底）。按正确路由复测后 wiki AI 卡与 login role=note 均可见。
3. H5 复测曾报 `InvalidCharacterError: atob` 因脚本 heredoc 用单引号未展开口令 base64 变量，改双引号 heredoc 后登录成功，非产品问题。

## 真值扫描

- 六批提交逐批 `git diff --cached` 子串扫描 `STAGED_LEAK_TOTAL=0`；`lyp_t43_scan.sh` 五类真值 `COUNT_HEAD=0 COUNT_WT=0`，`HEAD_DOMAIN_HITS=0`。
- 线上 HEAD 全仓真域名扫描 `HEAD_TRUTH_LEAK=0`；HEAD 外部 IP 命中仅 augmentor 子项目测试固定 IP（10.255.255.1 / 192.168.1.10 / 2.2.2.2），非真实服务器地址。
- 15 张 PNG 二进制真值扫描 `SHOTS=15 SHOT_LEAK_TOTAL=0`。

## 截图清单（非物理真机）

`r6-online-admin-dashboard.png`、`r6-online-admin-content.png`、`r6-online-admin-defenses.png`、`r6-online-admin-equipment.png`、`r6-online-admin-login.png`、`r6-online-admin-login-note.png`、`r6-online-admin-wiki.png`、`r6-online-admin-wiki-correct.png`、`r6-online-admin-wiki-rec.png`、`r6-online-h5-equipment.png`、`r6-online-h5-equipment-rec.png`、`r6-online-h5-reactions.png`、`r6-online-h5-reactions-rec.png`、`r6-online-h5-skills.png`、`r6-online-h5-skills-rec.png`。
