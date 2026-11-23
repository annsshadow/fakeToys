# R14 设计终审报告（design-reviewer）

- 任务：t128（R14 六门禁设计终审），attempt_id 4650f25d-bdce-4300-b7ba-f6e8cbcd01ad
- 审阅对象：t124 规范（一键执行闭环三端契约）/ t125 后端（GET /me/advice/action 权威 CTA）/ t126 H5（战报·图鉴一键替换 CTA）/ t127 真浏览器端到端复查
- 判定依据：本人独立读源码 + 读 SPEC + grep 关键符号 + 全目录真值扫描 + 读 t127 复查报告与 5 张截图清单；依赖报告仅作源材料，不采信自述
- 环境：本地回环 H5 真浏览器 390×844，非物理真机；截图 sha 与报告一致
- 保密：本报告不回显任何令牌/口令/路径真值

## 结论速览

verdict = needs_revision（status=failed）。六门禁中 ① 真值零泄露、② test-first 证据、③ 真浏览器证据有效、④ AI 四判据、⑤ 先例不回退 全部达标；门禁 ⑥ 闭环真实可达 FAIL——t127 真浏览器决定性实验发现 R14-BUG-1【高】：图鉴页 R14 建议卡完全不可达（核心功能在一个页面 100% 不可见），源码未修复、专项修复任务 t130 仍 in_progress。

## 六门禁判定

### ① 真值零泄露（PASS）

- 本人独立 grep round14-advice-action-e2e 目录：JWT 三段式 0、口令字面 0、可疑真域名 0（仅本地回环/占位符）。t127 报告 §8 记录真值扫描 JWT=0/Bearer=0/私钥=0/口令字面=0/非回环域名=0，auth-init.js 采集后已删、模板仅留占位符，5 张 PNG 二进制零命中。

### ② test-first 证据（PASS）

- t124 契约 r14AdviceAction.contract.test.ts RED 3 failed/1 passed（SPEC ENOENT）→ GREEN 4/4；t125 三条 E2E RED→GREEN；t126 契约 RED 7 failed→GREEN 8/8，定向 DOM 3 files/14 tests passed。各任务均先 RED 后 GREEN，证据链完整。
- 保留意见（不单独构成本门禁失败）：t127 发现契约测试夹具固定 expected_reaction=overheat，从未覆盖图鉴页空串路径，导致 1763 tests 全绿仍漏检 R14-BUG-1——属测试设计盲区，已在 finding 修复要求中要求补空串用例。

### ③ 真浏览器证据有效性（PASS）

- t127 截图 5 张 sha 各异：r14-battle-01-cta-ready(4d51b8d7) / r14-battle-02-compare(a0dd4589) / r14-battle-03-applied(69415263) + r14-rx-04-bug-no-card / r14-battle-05-empty-no-cta，均 390×844 非物理真机、非登录页有数据态。探针 pe/ce/af/fr 全 0。

### ④ AI 四判据（PASS）

- 契约钉死：badge=AI、expected_reaction/based_on 依据行（evidence[].source 指向 battle_records.* 服务端权威数据，禁写死百分比）、可执行 CTA / 诚实空态、服务端 impact 差分；战报页「AI·反应搭配建议」与图鉴卡片同源渲染。四判据齐全且不可伪造。

### ⑤ 先例不回退（PASS）

- 令牌锚点 v1/v3-r3/r6 无 v4、无 --r14-（契约 :118-122 钉死）；R7 impact helper /me/recommendations/impact?kind= 与 R10 不伪造替换 id 红线保持（:123-126）；t127 §6 源码锚点逐项核对 R1-R13（新手引导/AoE25%/r10-growth/fmtEv/失败复盘等）全在。

### ⑥ 闭环真实可达（FAIL）

- 战报页闭环 PASS：看建议→#r14-advice-action-btn 一键→.r7-compare-mask 两步确认→.apply-done→DB 落库 slots 0:1,1:4,2:7 变 0:1,1:8,2:7（服务端事务生效非本地乐观值）。防作弊空态/非法元素/游客全 PASS。
- 图鉴页闭环 FAIL：详见 finding R14-BUG-1。核心功能在一页完全不可达，门禁不成立。

## Findings

### R14-BUG-1【blocker】图鉴页 R14 建议卡完全不可达（前端把可选字段当必填）

- 问题：t127 真浏览器实测图鉴页 reactions.vue 打开的 7 条 advice 请求全部 200 / cand 非零（3/6/8）/ empty=null，但 DOM 中 .r14-advice / .r14-advice-empty / #r14-advice-action-btn 计数全为 0——玩家在图鉴页完全看不到任何 R14 建议卡与一键 CTA。浏览器内决定性实验：normalizeAdviceAction({...expected_reaction:空串, cand:3}) 返回 null；同一对象改 expected_reaction=steam_burst 即返回 ready/cand=3，根因 100% 落在该行校验。
- 根因：服务端 advice_action.go 的 expected_reaction 直取 query 的 reaction 原值；图鉴页 reactions.vue:487 调 fetchAdviceAction({element, source:reactions}) 不传 reaction，故响应 expected_reaction 为空串；前端归一器 r14AdviceAction.ts:73 `if (!element || !expectedReaction) return null` 把空串判为无效。本人核验当前源码 :72-73 仍为该写法，未修复。SPEC.md A.1 明确 reaction「可选，仅用于依据展示，不参与鉴权与计算」，六字段契约只要求 expected_reaction 存在、未要求非空；C.2 要求图鉴条目同源渲染 CTA 或空态——实现违背规范。
- 未修复状态：专项修复任务 t130（图鉴建议卡不可达）当前仍 in_progress，helper 唯一副本 laiyipao/miniapp/src/pages/_shared/r14AdviceAction.ts:73 未改动，契约夹具仍全为 expected_reaction=overheat 无空串用例。
- 严重度：blocker（核心功能在图鉴页 100% 不可达，且 SPEC 明文要求可达；参照 R9-BUG-1 先例需专项修复+复审闭环后方可放行）。
- 修复要求：test-first RED→GREEN——按 SPEC 方案 A（最小）：normalizeAdviceAction 去掉 !expectedReaction 的强制非空，允许空串（保留 element 必填与其余防作弊校验）；补契约用例覆盖「expected_reaction 空串 + cand 非零 → ready」与「图鉴页不传 reaction 路径」，RED 复现 DOM 计数为 0 后 GREEN；真浏览器图鉴页复测 7 条建议卡可见、CTA 可点、两步确认与落库闭环可达；清空串回归证据后回复审。不得放宽防作弊红线（空态仍不得带非零 id/params）。

## 诚实口径

- 本轮终审基于源码/SPEC/契约/复查报告与截图清单独立核验；R14-BUG-1 的浏览器决定性实验由 t127 完成（本人采信其可重放的三层根因证据并复核了对应源码行与 SPEC 规范依据）。
- 全量测试绿灯（188 files/1763 tests）为 t126/t127 依赖报告数据，本人未重跑。
- 门禁⑥仅因图鉴页单点不可达 FAIL；战报页闭环、后端权威裁决、防作弊、真值、AI 判据、先例不回退均真实达标，修复 BUG-1 后其余门禁无需返工。

## 最终结论

verdict=needs_revision，status=failed。R14-BUG-1（blocker）修复并复审 pass 后放行 t129 收官提交。报告落盘 docs/laiyipao/reviews/ROUND14_DESIGN_REVIEW.md。

> 所有截图均为真浏览器自动化采集，非物理真机。