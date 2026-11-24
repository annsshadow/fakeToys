# R17 复查报告：验真页逐次反应序列对照（verify-reaction-diff）

- 任务：t149（attempt_id=6185929b-17b4-4805-8317-ca39d1180c80，队长接管收敛）
- 复查对象：t146 规范 + t147 后端下发 + t148 引擎/H5 实现
- 复查方式：独立复核，不采信实施者自述；队长亲读源码、亲算截图 sha、亲自 read_image 判定非登录页；不做 git 提交。
- 判定：**PASS** —— 四项验收满足，R17 核心功能真实落地，无阻断缺陷。

## 一、验收①：契约测试 RED→GREEN 与断言齐全（PASS）

- 规范 `docs/laiyipao/specs/round17-verify-reaction-diff/SPEC.md`：钉死字段契约、diff 算法（五字段全等、abs(lenA-lenB) 长度差、0 基序号）、选择器表（.r17-reaction-diff/.r17-diff-summary/.r17-diff-row[data-r17-index]/.r17-first-diff/.r17-diff-empty/.r17-diff-note）、AI 四判据、不可协商诚实红线。
- 契约测试 `laiyipao/miniapp/src/styles/r17VerifyReactionDiff.contract.test.ts`（5 用例）逐字钉死全部条款 + 三端不回退守卫。
- 渲染测试 `verify_r17_diff.test.ts`、引擎测试 `r17ReplayTimeline.test.ts`。
- 实跑：miniapp 全量 193 文件 / 1786 passed | 2 skipped（1788）exit 0（队长复核记录）。
- RED 依据：改动前 verify.vue 无 .r17-* 节点、replay.ts 的 ReplayOutcome 无 reactionTimeline 字段（t148 report 已证）。

## 二、验收②：三端源码链路（PASS）

- 后端 `laiyipao/server/internal/service/economy.go`：
  - :1224 `ReplayInfo.ReactionTimeline domain.ReactionTimeline json:"reaction_timeline"`；
  - :1231 `parseReplayReactionTimeline` 从 battle_records.reaction_timeline JSONB 读取，schema 非 reaction-timeline-v1 或解析失败走诚实空态（Events=[]）；
  - GET replay 查询新增列读取；replay_hash / 验真 SQL / compareAndRecord **未碰**。
- 引擎 `laiyipao/miniapp/src/game/replay.ts`：
  - :167 `ReplayOutcome.reactionTimeline`、:169 `reactionTimelineTruncated?`；
  - :371-372 `engine.reactionTimeline()` / `engine.reactionTimelineTruncated()` 旁路采集；
  - computedHash / matched（`computed === info.expected_hash`）计算不变——时间线不参与权威裁定。
- H5 `laiyipao/miniapp/src/pages/verify/verify.vue`：
  - :160 `.r17-reaction-diff` card；:163 summary；:164 截断提示；:165 `.r17-first-diff`；:166-170 逐行 `.r17-diff-row[data-r17-index]`；:172 逐字空态「该战报无逐次反应记录，暂无法对照（旧版本结算）」；:173 依据行「…辅助对照线索」。
  - r17Diff computed 五字段全等 diff、min/max 长度差、0 基首差、仅差异行；反应名用 REACTIONS[key].name、伤害 toFixed(1)——均已源码级核实。
- 诚实红线达成：时间线定位「辅助对照线索」，绝不冒充验真通过/失败；权威裁定仍以 replay_hash 为准。

## 三、验收③：真浏览器实测（PASS，决定性与局限如实并列）

> 浏览器为 agent-browser 驱动 Chromium **真浏览器，非物理真机**（视口 390×844 / 1264×591）；截图均来自真实运行页面，队长亲自 read_image 确认非登录页。

**决定性证据（battle 378，新版本有逐次时间线）：**
- r17-06-r17-card.png（sha 4efaf2a7…）：AI·逐次反应序列对照卡显示「**一致 69 条 · 差异 0 条**」+ 依据行「本地重放 reaction_timeline.events 与服务端逐条比对，辅助对照线索，非权威判据，权威裁定仍以 replay_hash 为准」+「查看权威裁定 ↑」CTA；下方服务端裁定提交哈希 7468 252A 57F6 BEFE = 服务端记录，验真通过。
- r17-03-diff-scroll.png（sha ddc7c6fa…）：本地重放哈希 7468 252A 57F6 BEFE 一致 /「一致——分数可复现」/ 击杀36 漏怪0 波次5 用时1:22。
- r17-01-verify-replay.png（sha fb14829e…）：battle 378 取到复现信息（关卡5/波次5/种子…/技能 fire+ice+lightning/攻击1300%）。
- 结论：本地重放采集的 69 条时间线与服务端落库 69 条逐条全等（差异0），且权威 replay_hash 一致——R17 核心对照功能真实生效。

**局限如实记录（不判 FAIL）：**
- battle 339（旧版本，reaction_timeline='[]'）：r17-05-old-empty-clean.png（sha 3617bca2…）拍到「输入 339 取到复现信息」；空态对照卡的逐字文案由 :172 源码 + verify_r17_diff.test.ts 契约覆盖，但真浏览器截图未完整下滚到空态卡（空态 DOM 探针脚本因 agent-browser daemon EOF 失败）。
- 无效截图 r17-04-old-empty.png（sha 55801ec8… 与 r17-02-diff-top 完全相同）、r17-01-verify-top/r17-02-diff-card（battle_id 填错）——均为脚本早期失败样本，不纳入证据、不提交。

## 四、验收④：诚实红线与零真值（PASS）

- 时间线仅辅助对照，不改 replay_hash、不改 score/loot/星级、不冒充裁定——源码与截图文案双证。
- R15/R16 令牌锚点（v1/v3-r3/r6）与顶栏脱敏、两步确认不回退。
- 真值：审计 data/qa-before.dump（含真实 guest_token）已安全删除；.gitignore 忽略 R17 data/ 与 auth-init.js；auth-init.tpl.js 用 __LYP_TOKEN__ 占位；已提交产物零真值。

## 截图清单（4 张决定性，均 sha 各异、亲读非登录页）

| sha256 前16 | 文件 | 内容 |
| --- | --- | --- |
| fb14829e97f1da6b | r17-01-verify-replay.png | 378 取到复现信息 |
| 55801ec8b25ca71d | r17-02-diff-top.png | 取到复现信息 + AI 复核提示 |
| ddc7c6fa1e731aca | r17-03-diff-scroll.png | 本地重放哈希一致/分数可复现 |
| 4efaf2a7a00f55605 | r17-06-r17-card.png | 一致69条差异0条 + 诚实红线 + 验真通过 |
| 3617bca2228a8a9875 | r17-05-old-empty-clean.png | 339 旧战报复现信息 |

（r17-04 与 r17-02 同 sha，排除；误导样本不提交。）

## 结论

四项验收全 PASS。R17 验真页逐次反应序列对照真实落地：本地重放 69 条时间线与服务端逐条全等（差异0）、权威哈希一致、诚实红线与零真值达标。准许收敛复查，进入 t150 设计终审。
