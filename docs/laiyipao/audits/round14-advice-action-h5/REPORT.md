# R14 战报/图鉴建议一键替换 CTA 闭环 · H5 实测报告

- 任务：t126（attempt_id=a651b6a8-590f-4f15-899a-30e1916bb3e4）
- 规范：`docs/laiyipao/specs/round14-one-tap-apply/SPEC.md` §A–§D
- 写域：`laiyipao/miniapp/src/pages/_shared/r14AdviceAction.ts`、`battles/battles.vue`、`reactions/reactions.vue`、两页 `advice_action.test.ts`、`styles/common.css`、`styles/r14AdviceActionFlow.contract.test.ts`、本报告。
- 无 git 操作、无真实域名/IP/口令/密钥入文件或对话。

## 实现要点（对照 SPEC）

1. **服务端权威**：`fetchAdviceAction` 只发送白名单 `element`／`reaction`／`source`，不发送 `id`／`replace`；`normalizeAdviceAction` 校验 `schema_version=advice-action-v1`，成功态要求 `candidate_id>0 && replaced_id>0 && params` 与字段一致；空态白名单 `NO_OWNED_CANDIDATE`／`NO_HISTORY`／`NOT_ENOUGH_DATA`／`CONFIG_MISSING`／`INVALID_ELEMENT`／`NOT_AUTHENTICATED`，且 `candidate_id=0`／`params=null`。空态不渲染 CTA，渲染 `.r14-advice-empty` 与可执行空态动作。
2. **impact 复用**：收益差分不重新实现算法，将 `advice.impact` 包成 rec-v1 envelope 交既有 `normalizeImpact` 归一，展示「每波伤害 当前 → 推荐」等服务端真实差分；无差分显示诚实文案，不编数字。
3. **保留原建议**：battles `.reaction-learning`、reactions 图鉴条目原有 `label`／文案原样保留；R14 卡作为新增增强，不替换既有 AI 文本。
4. **两步确认**：点击 `#r14-advice-action-btn.r14-advice-cta[data-r14-source=battle|reactions]` 仅打开复用 R7 的 `.r7-compare-mask` 对比浮层（当前 vs 推荐），第二拍 `r14ConfirmApply()` 才调用 `applyAdviceAction` 写入；技能走 `loadLoadout → indexOf(replacedId) → persistLoadout`，装备走 `loadEquipment → applyEquipmentImpact → loadEquipment`；写入失败保留原配置并显示真实错误与重试。成功后卡内显示 `.apply-done`「已替换为」与配置反馈。
5. **AI 四判据**：卡内可见汉字「AI」徽章、真实结论（基于服务端 `based_on`／`expected_reaction`）、依据行、可执行 CTA；触控 ≥88rpx；`prefers-reduced-motion` 关闭入场动画；令牌复用 `--ai-accent`／`--ai-ink`／`--ai-halo`／`--ai-glow`，零新增令牌，v1／v3-r3／r6 锚点未动。

## 验证证据

- 新契约 `r14AdviceActionFlow.contract.test.ts` 8 项：RED 7 failed | 1 passed → GREEN 8/8。
- DOM 测试 `battles/advice_action.test.ts` + `reactions/advice_action.test.ts`：真实 mountPage，覆盖原建议保留、CTA id/data-source、query 仅白名单、空态无按钮、第一步不写／第二步写一次、成功后「已替换为」。定向 3 files / 14 tests passed。
- 全量 `pnpm test`：188 files passed | 1 skipped，1763 tests passed | 2 skipped（exit 0）。
- `pnpm type-check`：exit 0；`pnpm build:h5`：DONE Build complete（exit 0）。
- R7／R10 红线未回退：既有 `.apply-done`／「已替换为」／`r10GrowthGaps` 契约仍绿；battles `expand` 行为未改（测试中先展开再断言，未改业务）。

## 真浏览器实测评判（非物理真机）

- 采集方式：桌面 Chrome（agent-browser 真实二进制）模拟移动视口 **390×844**，本地回环 `http://127.0.0.1:5174/#/...`，H5 dev server HTTP 200。**非物理真机**、非线上域名。
- 截图（已逐张目视）：

| 文件 | sha256 | 目视结论 |
| --- | --- | --- |
| `screenshots/browser-battles.png` | `3c738687c316eea0215833a2410c5e6794ee2bc120057f00e94afe403cb75c8a` | 战报页「共 0 场」诚实空态；无战报详情，R14 成功态 CTA 无法出现 |
| `screenshots/browser-reactions.png` | `68391007f3c91c9bda19743a2a87b77580f1c219483a0f6927581224d2f7353f` | 反应图鉴表未加载（0/0）空态；无反应条目，R14 逐条 CTA 无法出现 |

- **环境限制如实说明**：本机无 PostgreSQL（5432 ECONNREFUSED），API `go run ./cmd/api` 无法启动，浏览器拿不到真实战报/配置/建议数据。因此浏览器截图只能证明「无数据时页面诚实空态」，**不能**证明成功态 CTA 在真浏览器出现；成功态与两步确认闭环由 jsdom DOM 测试覆盖（见上）。这是产品环境限制，不把空态包装为有数据实证。
- 采集脚本：`capture.sh`（agent-browser，session `lyp-r14`）。

## 结论与后续

R14 H5 侧消费层、空态、两步确认、写入与反馈均已实现并通过契约／DOM 测试／全量测试／类型检查／构建。真实带数据浏览器验收待具备 PostgreSQL 的测试环境后补拍「战报详情展开 → AI 建议 CTA → 对比浮层 → 确认替换 → 已替换为」连续截图。后端 `GET /me/advice/action` 已由 t124 落地契约，需由 release-ops 部署到测试环境联调。

## R14-BUG-1 修复记录（t130）

- 缺陷：qa-e2e 真浏览器复查发现图鉴页 `.r14-advice`／CTA 计数全 0；根因是 `r14AdviceAction.ts` 把可选字段 `expected_reaction` 当必填（空串即 `return null`）。`SPEC.md` A.1 明确 `reaction` 仅用于依据展示、不参与鉴权计算，图鉴页按元素查询时服务端合法回空串。
- test-first RED：`r14AdviceActionFlow.contract.test.ts` 新增「空串 expected_reaction 仍归一 ready/empty」用例、`reactions/advice_action.test.ts` 新增「空串仍渲染 CTA 且显示无目标反应文案」用例，修复前 2 failed | 11 passed。
- 修复：归一器只保留 `element` 必填与全部防作弊校验；图鉴卡在空串时显示「可提升当前构筑（服务端未给出目标反应）」，不伪装目标反应。
- GREEN：定向 3 files / 16 tests passed；全量 `pnpm test` 188 files / 1765 tests passed（+1 file / 2 skipped）；`pnpm type-check`、`pnpm type-check:test` exit 0；`pnpm build:h5` DONE。
- 浏览器缺陷原图：`docs/laiyipao/audits/round14-advice-action-e2e/screenshots/r14-rx-04-bug-no-card.png`（qa-e2e 采集，非物理真机）。修复后真浏览器复验由 qa-e2e 安排。