# R13 设计终审报告（design-reviewer）

- 任务：t121（R13 设计终审），attempt_id 2afdd95e-261c-474d-86c4-a99f03698175
- 审阅对象：t116 冷启动规则候选、t117 反应教学与术语、t118 跨端 hits>shots 修复、t119 音效开关语义反转；依赖 t120 队长独立复核（替代 503 失败的成员复查）
- 判定依据：本人亲自逐张读图 + 亲算 sha256 + grep 源码 + 真值全目录扫描 + base64 解码 JWT 确认真伪；依赖报告仅作源材料，不采信自述
- 环境：本地回环 H5/API 真浏览器，移动 390×844 / 桌面 1258×588，非物理真机；全量测试绿灯数据来自依赖报告，本人未重跑
- 保密：本报告不回显任何真值，凭据一律写占位符

## 结论速览

verdict = needs_revision（status=failed）。六硬门禁中 ① ② ③ ④ ⑤ 全部达标；门禁 ⑥ **真值0泄露 FAIL**：round13-after 采集脚本重新引入真实签发的 JWT 字面量、管理口令字面与本机绝对路径，会触发 `TestNoJWTLiteralInRepo` 守卫，且为 t102 同类问题清除后的回归。另附一条截图证据瑕疵（不阻塞）。清除真值、复扫归零后即可放行。

## 门禁判定

### ① 冷启动规则候选诚实标注（PASS）

- 源码 `growth_cold_start.go:43-51`：`BasedOn=rule_only`、`Label=基于规则而非你的历史战报`、`BattleCount` 由调用方从 battle_records 实读；`:41-42` 注释明确只用构筑快照+配置数值、不查历史战报；三条规则来源逐条落库 `source.rule/fields`（lowest_level_owned_active_skill 等）。
- 真浏览器 fresh2.log：skills/me/index 页 recs=[{status200,schema=rec-v1,code=NO_HISTORY,based=rule_only,bc=0,cands=3}]，hasColdRule=true；亲读 r13-fresh2-skills.png（1258×588，sha 1f96467d）见 AI 冷启动卡真实落地。无历史时不编造个性化结论。

### ② 升级战力前后快照（PASS）

- 源码 `routes.go:678/687`：`beforePower=ComputePowerFor(beforeBuild)`、`afterPower=ComputePowerFor(afterBuild)` 同一函数真实计算；`:698-703` power_before/after/method 与 power_snapshot.applicable=true 齐备。
- 游客转正分支 `routes.go`：power_before/after=nil、power_method=not_applicable、power_snapshot.applicable=false + 说明，诚实空态。

### ③ 反应教学与术语一致性（PASS）

- 源码 `feedback.ts`：`reactionMechanism` 在 spreadStacks>0 时只输出「传播 N 层腐蚀（半径 X，不触发新反应）」，绝不输出 25% 溅射；`reactionLearningAdvice` 只消费权威 reactions_used/elements_used，缺数据返回「本局未上报反应使用记录，不给元素建议。」不推算。
- `reactions.vue` 机制表新增「传播/溅射」两列，腐蚀扩散按 spread_stacks>0 显示「传播半径」而非「溅射半径」。
- 真浏览器 detail.log：hasLearning=true，文案「焰+电触发过热 ×51；下一局补 毒（已有 焰），可解锁燃烧云；依据：reactions_used/elements_used」；fresh2.log reactions 页 hasSpreadRadius=true/hasSplashRadius=true。亲读 r13-final-detail.png（390×844，sha 7b2e6531）战报学习卡可见。

### ④ 跨端 hits>shots 修复（PASS）

- 双端等价：TS `engine.ts:1526-1529` 与 Go `engine_combat.go:434-437` 对称——只在首次主目标命中 `if(!p.hitCounted){hits++;hitCounted=true}`，穿透/溅射/弹射不再累加；旧注释同步改为「首次主目标命中」。
- 服务端校验未放宽：`battle.go` 保留 `if in.Hits > in.Shots → ErrInvalidHitRate`。E2E `routes_e2e_high_pierce_settle_test.go`：旧口径 hits=4/shots=1 仍断言 422，新口径 hits=1/shots=1 断言 200 并查库落库。回放确定性：hitCounted 不进 replay 事件流、replay_hash 口径不变（REPORT :51），vectors 已重生成并校验。

### ⑤ 音效开关语义（PASS）

- 源码 `me.vue:161` :checked=sfxEnabled；`:663-668` `sfxEnabled=ref(!isMuted())`，onSfxToggle 取 `next=e.detail.value === true`，`setMuted(!next)`、`sfxEnabled.value=next`——开关 checked=true 即音效开启，取反写静音偏好。sfx.ts 手势解锁与持久化未动。
- 真浏览器 sfx.log：INITIAL checked=true/muted=null → AFTER_CLICK checked=false/muted=1 → AFTER_RELOAD 仍 false/1（持久化）→ RESTORED checked=true/muted=0；swdom.log 开关 rgb(255,107,53) 橙色开启态。亲读 r13-sfx-after-toggle.png（390×844，sha 856297d8）开关状态可见。

### ⑥ 综合门禁（FAIL — 见 findings）

- AI 四判据 / R1-R12 不回退 / 令牌锚点：PASS。令牌 v1/v3-r3/r6 无 v4、无 r13 新增（tokens.css 确认）；t120 旁证 admin 四视图、longTail=[] 不回退。
- 截图 sha：5 张决定性截图各异（8562…/7b2e…/b7d9…/1f96…/2ce5…），非登录页有数据态；但反应图鉴 before/after 一对 sha 相同（见 F-13-2）。
- **真值0泄露：FAIL（F-13-1，阻塞）**。

## Findings

### F-13-1【blocker】真值泄露回归：采集脚本含真实 JWT / 管理口令 / 本机绝对路径

- 问题：`docs/laiyipao/audits/round13-after/auth-init.js` 与 `auth-fresh-init.js` 虽在注释写「占位符…采集后删除生成文件」，但实际 `localStorage.setItem` 注入的是完整三段式 JWT 字面量。本人 base64 解码 payload 确认为真实签发 token（sub=10 与 sub=731、knd=user、iat/exp 为真实时间戳），非占位符。`admin-fill-run.js` 含管理口令字面；`mint-token.sh`/`power-check.sh` 含本机绝对路径与 worktree id。
- 依据：`secrets_leak_test.go::TestNoJWTLiteralInRepo` 用 jwtLiteralRe=eyJ[8+].[8+].[8+] 扫整个工作树（walkTracked 用 filepath.Walk，docs 不在 skipDirs），这些 JWT 必然命中导致守卫红。AGENTS.md 明令令牌/口令/本机绝对路径不入库。`*.gitignore` 未忽略 docs/laiyipao/audits/，一旦提交即公开。t103 记录显示 t102 同类 auth-init.js JWT 泄露曾「CAS 清除复扫归零」，本轮为同一问题回归。
- 修复要求：清除或占位符化这些采集脚本中的真值——JWT 改 <YOUR_TEST_TOKEN> 占位符（或运行时从环境变量/DB 注入后不落盘）、口令改 <TEST_PASSWORD>/环境变量、本机绝对路径改相对路径；删除声称「采集后删除」却残留的生成脚本；复跑 `go test ./internal/domain -run TestNoJWTLiteralInRepo` 与全目录真值扫描归零后，方可回设计终审。
- 定性：非业务源码缺陷，属审计产物留存；但保密纪律与守卫为硬门禁，未清零不能放行。

### F-13-2【low】反应图鉴 before/after 截图 sha 相同，对比证据不成立

- 问题：`round13-reaction-learning/screenshots/r13-01-reactions-codex.png`（报告称修复前，标签写「溅射半径 120」）与 `r13-02-reactions-fixed.png`（修复后，显示「传播半径 120」）sha256 完全相同（均 92f6497f…，110327B），两张字节一致，不构成 before/after 对比。
- 判断：D1 修复源码正确（reactions.vue 传播/溅射两列 + feedback.ts reactionMechanism）且 DOM 探针 hasSpreadRadius/hasSplashRadius=true 已证明修复后状态，故不阻塞功能判定。
- 修复要求：补一张真实的修复前截图，或修正 README 对该图的表述，避免造成「伪造对比」观感。

## 诚实口径说明（不阻塞）

1. 门禁②战力快照以源码逻辑（ComputePowerFor 前后调用 + not_applicable 分支）判定，未亲见 upgrade 实际响应日志（power-check.sh 仅为脚本），如实标注。
2. 全量测试绿灯数据为依赖报告自述，本人未重跑。
3. 四项改造的功能实现（①②③④⑤）真实可达，本轮 fail 的唯一阻塞是门禁⑥真值泄露这一可快速修复的清理问题。

## 最终结论

verdict=needs_revision，status=failed。F-13-1 清零（真值占位符化/删脚本 + 守卫复跑归零）并修正 F-13-2 后即可复审放行。报告落盘 docs/laiyipao/reviews/ROUND13_DESIGN_REVIEW.md。

> 所有截图均为桌面 Chrome / 移动视口自动化采集，非物理真机。