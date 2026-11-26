# R17 设计终审报告：验真页逐次反应序列对照

- 任务：t150（attempt_id=b1b42945-caf1-4bcf-8d99-b9950637596d）
- 终审对象：t146 规范 + t147 后端下发 + t148 引擎/H5 实现（t149 复查）
- 终审方式：**独立复核，不采信任何成员自述**——亲读 SPEC/契约/源码、亲读决定性截图、亲算截图 sha、亲自复跑测试。不做 git 操作。
- 判定：**verdict = PASS**（六门禁全部达标，无阻断缺陷）。
- 终审日期：2026-11-24

---

## 门禁① SPEC 契约齐全 —— PASS

**决定性证据（亲读）：**
- `docs/laiyipao/specs/round17-verify-reaction-diff/SPEC.md`（84 行）：§0 真实缺口带行号、§1 字段契约（reaction-timeline-v1 / 五字段 wave-tick-reaction-enemy_uid-damage / 上限 200 / truncated / 诚实空态）、§2 六类固定选择器表 + diff 算法口径（五字段全等、abs(lenA-lenB)、0 基序号 i+1、min）、§3 AI 四判据、§4 不可协商诚实红线、§5 test-first 契约设计、§6 三端分工与不回退——全部钉死。
- `laiyipao/miniapp/src/styles/r17VerifyReactionDiff.contract.test.ts`（81 行，5 用例）：逐字守卫契约主干、选择器与文案、diff 算法、诚实红线、AI 四判据与三端不回退。
- 配套 `verify_r17_diff.test.ts`（154 行渲染）、`r17ReplayTimeline.test.ts`（108 行引擎）。
- **独立复跑**：`vitest run`（miniapp）三文件 → **3 passed / 11 tests passed**，exit 0。

## 门禁② 后端 replay 下发时间线，诚实空态且不改 replay_hash —— PASS

**决定性证据（亲读 `laiyipao/server/internal/service/economy.go`）：**
- :1222-1224 `ReplayInfo.ReactionTimeline domain.ReactionTimeline json:"reaction_timeline"`，注释明确「不参与、不覆盖 replay_hash；权威裁定仍只看 replay_hash matched」。
- :1231-1264 `parseReplayReactionTimeline`——NULL / 空字节 / "null" / 坏 JSON / events 形状错误 / schema 不符一律返回 `schema_version=reaction-timeline-v1 + events=[]`（非 nil 空数组），逐字段校验 :1252-1257（wave<1 / tick<0 / reaction 空 / enemy_uid<1 / damage<0 → 空态），不报错、不反推编造。
- :1594-1642 `getReplay` SELECT 新增 `br.reaction_timeline`（:1601），:1626 调用 parseReplayReactionTimeline；replay_hash / 验真 SQL / compareAndRecord 未碰，玩家侧剥离 settle_input（:1640）保留。
- **决定性实验**：`go test ./internal/service/ -run TestParseReplayReactionTimeline -count=1` → exit 0（纯逻辑单测覆盖 8 类异常输入 + 合法数据读回）。

## 门禁③ 引擎旁路不改 matched / computedHash —— PASS

**决定性证据（亲读 `laiyipao/miniapp/src/game/replay.ts`）：**
- :167-169 `ReplayOutcome.reactionTimeline` / `reactionTimelineTruncated?` 新增。
- :369 `runToEnd` 后，:371-372 `engine.reactionTimeline()` / `engine.reactionTimelineTruncated()` **同帧旁路采集**。
- :374 `computed = engine.replayHash()`、:377 `matched: computed === info.expected_hash`——权威计算**完全不变**；:385-386 仅把 timeline 追加到返回对象。
- `r17ReplayTimeline.test.ts` 断言旁路采集不改哈希、不改 matched，旧记录缺字段仍照常重放。

## 门禁④ H5 对照卡 diff 正确 + AI 四判据 + 诚实红线 —— PASS

**决定性证据（亲读 `laiyipao/miniapp/src/pages/verify/verify.vue`）：**
- 模板 :160 `.r17-reaction-diff` role=region，:161 标题「AI · 逐次反应序列对照」（可见汉字 AI）；:163 summary、:164 截断提示、:165 first-diff、:166-170 逐行 `.r17-diff-row[data-r17-index]`、:172 逐字空态「该战报无逐次反应记录，暂无法对照（旧版本结算）」、:173 依据行、:174 note 逐字「辅助对照线索，非验真判据；权威裁定仍以 replay_hash 为准」、:175 CTA「查看权威裁定 ↑」。
- `r17Diff` computed（:393-421）：`r17Fields`（:368）五字段全等 `r17Same`（:386-388）；min/max 长度差（:401-409）；0 基序号 i、展示 i+1（:165/:167）；仅差异行渲染（:413-417）；firstDiff=diffIndexes[0]（:418）；`r17Valid/r17Sanitize`（:370-385）非法事件丢弃；schema 不符（:396）走空态；反应名 `REACTIONS[e.reaction]?.name`（:390，未知 key 兜底原 key，不硬编码）；damage `toFixed(1)`（:391）。
- **与权威卡隔离**：本卡无 verdict-box、不写「验真通过/失败」、不复用 matched 作结论色；服务端裁定卡（:177-193）独立且保留 outcome/result.matched。AI 四判据（汉字 AI / 真实数据结论 / 依据行 / 可执行 CTA）齐备。

## 门禁⑤ 真浏览器决定性证据（sha 各异 + 亲读非登录页 + DOM 探针）—— PASS

**决定性证据（本人亲读截图 + 亲算 sha256）：**
- `r17-06-r17-card.png`（sha **4efaf2a7a00f5560**，1264×591，亲读）：「AI · 逐次反应序列对照」+「**一致 69 条 · 差异 0 条**」+ 依据行 + note「辅助对照线索，非验真判据，权威裁定仍以 replay_hash 为准」+「查看权威裁定 ↑」CTA；下方服务端裁定卡哈希 7468 252A 57F6 BEFE 一致、「验真通过」——非登录页，R17 核心功能真实生效。
- `r17-03-diff-scroll.png`（sha **ddc7c6fa1e731aca**，390×844，亲读）：本地重放哈希 7468 252A 57F6 BEFE 与服务端记录一致、「一致 — 分数可复现」、击杀 36 / 漏怪 0 / 波次 5 / 用时 1:22——权威 replay_hash 一致。
- `r17-01-verify-replay.png`（sha fb14829e…）：battle 378 取到复现信息。
- **sha 亲算**（sha256sum 全目录）：决定性证据 5 图哈希两两不同（fb14829e / 55801ec8 / ddc7c6fa / 3617bca2 / 4efaf2a7）。r17-04 与 r17-02 同为 55801ec8（早期失败样本，已排除、不纳入证据）——与复查报告口径一致。
- **局限如实记录（不判 FAIL）**：battle 339 旧战报空态卡真浏览器下滚探针因 agent-browser daemon EOF 失败（`data/empty-probe.log` 亲读确认），空态逐字文案由 `verify.vue:172` 源码 + `verify_r17_diff.test.ts` 契约（4 类空态）覆盖；`.r17-reaction-diff` DOM 探针脚本（probe-final.sh）对 battle 378 有数据态做 box/summary/rows 查询，功能判定由有数据态决定性截图 + 契约测试共同承担。

## 门禁⑥ 零真值 + R1-R16 不回退 —— PASS

**决定性证据（亲读 + grep + 扫描）：**
- 令牌锚点：`tokens.css:14-16` `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 均在，全仓无 `--r17-` 新视觉令牌。
- 验真权威口径不回退：`verify.vue` 仍含 outcome.matched / result.matched / 「验真通过」/「验真失败」；契约测试 :70-73 守卫三锚点。
- 权威裁定文案：SPEC §4 逐字「不参与也不覆盖 replay_hash / 辅助对照线索 / 非权威验真判据 / 权威裁定仍是 replay_hash matched / 绝不冒充验真通过/失败」；H5 note 逐字一致。
- 顶栏脱敏：`admin/src/utils/apiLabel.ts:23` `maskApiBase` + `AdminLayout.vue:178` 使用；两步确认：admin 9 视图 `ElMessageBox.confirm` 均在。
- R15/H5 选择器：`battles.vue:192/198/205` `.r15-reaction-timeline/.r15-timeline-row/.r15-timeline-empty` 在；R16/admin 选择器：`BattlesView.vue:671/680/685` `.r16-*` 在。
- 25% AoE：`engine.ts:1864/1897/1910` 蒸汽爆发 25% 溅射逻辑在。
- **零真值**：审计目录扫描命中仅为 SQL 查询字段名（`mint-token.sh:3` guest_token 列名）、回环地址（:4 127.0.0.1）与描述性文字（REPORT.md:49 空态说明），**无真实 token / 域名 / 口令入库**；含真实 guest_token 的 qa-before.dump 已删；`.gitignore:282-283` 忽略 R17 data/ 与 auth-init.js；`auth-init.tpl.js` 用 `__LYP_TOKEN__` 占位；data 目录仅 `empty-probe.log`（无真值）。

## 是否发现新的真实缺陷 —— 无阻断缺陷

- 未发现影响 R17 功能、权威裁定正确性或诚实红线的新真实缺陷。
- 低优先级观察（不判 FAIL，记录供后续改进）：battle 339 旧战报空态卡的真浏览器下滚截图因探针 daemon EOF 未完整拍到空态文案，已由源码与契约测试双重覆盖；不影响功能判定。

---

## 终审结论

六门禁全部 PASS，verdict = **pass**。R17 验真页逐次反应序列对照真实落地：本地重放 69 条时间线与服务端落库逐条全等（差异 0）、权威 replay_hash 一致、诚实红线与非权威定位准确、三端不回退与零真值达标。准许本轮设计终审收敛。
