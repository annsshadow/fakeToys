# R14 · AI 建议一键执行闭环（三端规范）

> 状态：设计拍板 / test-first 契约已钉；依据 Round2–4 真浏览器实测与服务端代码审计（2026-11-23）。
> 适用范围：服务端权威 CTA、H5 消费层、契约测试。不改动业务视图源码；仅约束字段、路径、选择器和不回退项。

## 0. 依据与问题定义

真浏览器与代码审计确认同一缺口：战报卡 `laiyipao/miniapp/src/pages/battles/battles.vue:187-190` 的 `.reaction-learning` 只渲染 `reactionLearning` 文本（「下一局补 X（已有 Y），可解锁 Z」），图鉴页 `laiyipao/miniapp/src/pages/reactions/reactions.vue` 同样只有说明文字；两处都**没有可点 CTA**，玩家看到 AI 建议后只能自己去技能/装备页碰运气。而 R7 两步确认 UI（`.r7-impact` / `.r7-vs-label` / `.r7-compare-title` / `.r7-btn` / `.apply-done`）已在 `skills.vue:88`、`equipment.vue:93` 落地并可复用，服务端 `recImpactCTA`（`laiyipao/server/internal/service/recommendations.go:166`）已具备「只下发 owned 且数值更优的 `{id,replace}`」权威能力。缺的是把「建议元素」翻译成**服务端算出的可执行参数**并接到现有两步确认上。

事实锚点（不可凭记忆改动）：

- `reactionLearningAdvice(input:ReactionLearningInput):string`（`laiyipao/miniapp/src/game/feedback.ts:133`）只消费权威 `reactionsUsed` / `elementsUsed`，产出纯文本；本规范不改其文本口径。
- `fetchRecommendationImpact(kind,candidateId,replacedId)`（`laiyipao/miniapp/src/pages/_shared/r7ImpactApply.ts:146`）请求 `GET /api/v1/me/recommendations/impact?kind=&id=&replace=`；`applyEquipmentImpact(...)`（:164）走服务端事务整组替换。
- `recOwnedIDs(ctx,userID,kind)`（`recommendations.go:137`）查 `user_equipment.user_equipment` / `user_skills.skill_id`；`recImpactCTA(...)`（:166）只下发 `owned[id]` 且数值更优的 `{id,replace}`，无候选返回 basis 说明。
- `myBattleDetail`（`routes.go:883`，路由 `routes.go:192` 的 `GET /api/v1/me/battles/:id`）返回 detail 含 `elements_used` / `reactions_used` / `terrain_used` / `loot` / `replay_hash`。
- 空态口径（`laiyipao/miniapp/src/pages/_shared/r6Recommendation.ts:68`）：headline「AI 暂时没有足够数据给出建议」，code `NO_HISTORY` / `NOT_ENOUGH_DATA` / `CONFIG_MISSING`，cta「去打一场战斗 →」→ `/pages/stage/stage`。
- 防作弊红线（`laiyipao/miniapp/src/styles/r10GrowthGaps.contract.test.ts:126-127`）：不得伪造 `candidateId` / `replacedId`；服务端 `TestE2ERecommendationCTASkipsUnownedSkill`（`laiyipao/server/internal/httpapi/routes_recommendation_cta_ownership_e2e_test.go`）已验证未拥有候选不下发。

## A. 服务端权威 CTA 契约（回答「落在扩展战报详情还是新增端点」）

**结论：新增专用端点，不扩展 `GET /api/v1/me/battles/:id`。**

理由：① 战报详情是历史只读快照，`elements_used` / `replay_hash` 不可变，而可执行 CTA 依赖**当前** owned 目录与配置版本，塞进历史详情会让同一条战报在补齐装备前后返回不同内容，破坏快照语义；② 图鉴页 `reactions.vue` 没有 battle id，无法复用战报详情；③ 新端点一次定义、两处消费，字段与 `recImpactCTA` 对齐，避免下游重复推导。

### A.1 端点

```
GET /api/v1/me/advice/action
```

- `schema_version`：`advice-action-v1`（成功与空态都带）。
- GET query 白名单仅 `element` / `reaction` / `source`：不含 candidate_id、replaced_id、id、replace 入参。客户端**不得伪造**替换 id；id 只能出现在响应里。
  - `element`：五元素内部 key 之一 `fire` / `ice` / `lightning` / `corrosion` / `kinetic`；服务端白名单校验，非法值回 `INVALID_ELEMENT`。
  - `reaction`：期望解锁的反应 key（可选，仅用于「依据」展示，不参与鉴权与计算）。
  - `source`：`battle` / `reactions`（可选，仅用于消费方埋点与依据展示）。
- 鉴权：复用现有 me 中间件；游客回诚实空态 `NOT_AUTHENTICATED`，不返回 401 中断页面。

### A.2 成功响应（200）

```json
{
  "schema_version": "advice-action-v1",
  "badge": "AI",
  "element": "corrosion",
  "expected_reaction": "corrosion",
  "based_on": ["battle_records.elements_used", "battle_records.reactions_used", "user_skills", "/config(v=...)"] ,
  "kind": "skill",
  "candidate_id": 12,
  "replaced_id": 4,
  "candidate_name": "毒液弹",
  "replaced_name": "通用弹",
  "params": { "kind": "skill", "id": 12, "replace": 4 },
  "preview_route": "/pages/skills/skills",
  "evidence": [{ "metric": "该反应触发率", "value": 0.31, "source": "battle_records.reactions_used" }],
  "impact": { "...": "RecommendationImpact 原样结构" },
  "empty_reason": null
}
```

六字段契约（三端共同钉死，缺一不可）：`kind`、`candidate_id`、`replaced_id`、`element`、`expected_reaction`、`based_on`。`params` 直接喂给现有 `fetchRecommendationImpact(kind, id, replace)`，`impact` 由服务端 `RecommendationImpact` 同步算好，H5 不重复推导。

### A.3 诚实空态（200，非错误）

```json
{
  "schema_version": "advice-action-v1",
  "badge": "AI",
  "element": "corrosion",
  "expected_reaction": "corrosion",
  "based_on": ["battle_records.elements_used", "user_skills", "/config(v=...)"] ,
  "kind": null,
  "candidate_id": 0,
  "replaced_id": 0,
  "params": null,
  "empty_reason": {
    "code": "NO_OWNED_CANDIDATE",
    "headline": "AI 暂时没有足够数据给出建议",
    "message": "已拥有技能/装备里没有同槽且数值更优的候选，先打一场战斗或去商城补齐。",
    "cta": "去打一场战斗 →",
    "route": "/pages/stage/stage"
  }
}
```

`empty_reason.code` 取值：`NO_OWNED_CANDIDATE`（有历史但无可替换 owned 候选）/ `NO_HISTORY` / `NOT_ENOUGH_DATA` / `CONFIG_MISSING` / `INVALID_ELEMENT` / `NOT_AUTHENTICATED`。前四种与 `r6Recommendation.ts:68` 口径一致；后两种为本端点新增。**任何空态都不得下发非零 `candidate_id` / `replaced_id`**，前端据此只渲染 `.r14-advice-empty`，不渲染按钮。

### A.4 服务端 owned 校验 RED 用例（交给 backend 下游实现）

`laiyipao/server/internal/httpapi/routes_advice_action_ownership_e2e_test.go` 必须先 RED 后 GREEN：

```go
func TestE2EAdviceActionSkipsUnownedCandidate(t *testing.T) {
	// 1) 用未拥有某火系技能的账号请求 element=fire
	// 2) 断言 empty_reason.code == NO_OWNED_CANDIDATE
	// 3) 断言 candidate_id == 0 && replaced_id == 0 && params == nil
	// 4) 授予该技能后重放同一请求
	// 5) 断言 candidate_id > 0 && replaced_id > 0 且 owned[ candidate_id ] == true
	// 6) 断言 params.id/replace 与 candidate_id/replaced_id 一致
}
```

该用例复刻 `TestE2ERecommendationCTASkipsUnownedSkill` 的 RED 形状；`recOwnedIDs` 与 `recImpactCTA` 直接复用，不新写拥有判定。

## B. 元素 → 候选 id 映射（服务端权威，客户端不硬编码）

服务端按如下确定性顺序计算，任一步为空即返回 A.3 空态：

1. 白名单校验 `element`，五元素 key 与 `tokens.css` 的 `--fire/--ice/--lightning/--corrosion/--kinetic` 一一对应。
2. 读 `/config` 当前版本技能/装备/宝石目录，筛 `element == 入参 element` 的条目为候选集（配置缺失 → `CONFIG_MISSING`）。
3. `recOwnedIDs(ctx, userID, kind)` 取当前用户 owned 集合，与候选集取交集（空 → `NO_OWNED_CANDIDATE`）。
4. 按 R10 已钉五条件复核：配置存在 / 已解锁 / 已拥有 / 同槽或同主战 / 数值更优；不满足不下发。
5. 选 `replaced_id` 为当前出战同槽或同主战中数值最低的 owned 项；`candidate_id` 为 owned 且数值更优者中增益最大项。
6. 把 `{kind, candidate_id, replaced_id, params}` 原样给 `RecommendationImpact` 得 `impact`，返回 A.2。

**客户端约束**：H5 只回传 `element` key（来自服务端建议文本的权威字段或图鉴条目 key），**不硬编码**「元素→技能/装备 id」映射表，也不得伪造 id。服务端按白名单 + owned + 五条件二次裁决，即使客户端回传非法元素也只能拿到空态。

## C. H5 消费层（两处卡片、R7 复用、AI 四判据）

### C.1 战报卡（`battles.vue`）

- 保留 `.reaction-learning` 的 label「AI · 反应搭配建议」与 `{{reactionLearning}}` 文本，不回退文案。
- 文本下方按服务端响应二选一渲染：
  - 成功 → 按钮 `#r14-advice-action-btn.r14-advice-cta[data-r14-source="battle"]`，文案「一键替换为 {{candidate_name}} →」。
  - 空态 → `.r14-advice-empty[data-r14-source="battle"]`，显示 `empty_reason.headline` 与 `message`。
### C.2 图鉴卡（`reactions.vue`）

- 每个反应条目下同源渲染 `#r14-advice-action-btn.r14-advice-cta[data-r14-source="reactions"]` 或 `.r14-advice-empty[data-r14-source="reactions"]`。
- `element` 取该图鉴条目的权威元素 key，不在前端拼 id。
### C.3 一键 → 两步确认（复用 R7，不新建流程）

五步闭环（E2E 必须逐步可观测）：

1. **看建议**：卡片可见「AI · 反应搭配建议」与真实数据结论。
2. **一键替换**：点 `.r14-advice-cta` 打开对比浮层 `.r7-compare-mask`。
3. **两步确认**：浮层内 `.r7-compare-title` + `.r7-vs-label` 展示 `current vs 推荐`，CTA 按钮 `.r7-btn`；未确认不得写入。
4. **写入**：确认后调现有 `applyEquipmentImpact(...)` / 技能等价事务；成功显示 `.apply-done`。
5. **战力变化**：写入后回到卡片展示服务端真实属性差分（来自 `impact`），不虚构数值。

### C.4 AI 可见四判据（P8 下限，缺一不可）

- 首屏肉眼可辨：字体/颜色走 `--ai-accent #c9a7ff` / `--ai-ink` / `--ai-halo`，不使用极低透明度。
- 可见汉字「AI」（label 已含）。
- 真实数据结论：`evidence[].source` 指向 `battle_records.*` / 服务端属性差分，禁止写死百分比。
- 可行动 CTA：成功态必有 `.r14-advice-cta`，空态必有 `.r14-advice-empty` 的「去打一场战斗 →」。

## D. 三端可测试契约（test-first）

契约文件：`laiyipao/miniapp/src/styles/r14AdviceAction.contract.test.ts`（已落盘，SPEC 缺失时 3 failed / 1 passed 的 RED），四组断言钉住：

1. 服务端：端点路径 `/api/v1/me/advice/action`、`advice-action-v1`、六字段、`NO_OWNED_CANDIDATE`、`recOwnedIDs` / `recImpactCTA`、query 不含 id/replace、`TestE2EAdviceActionSkipsUnownedCandidate`。
2. 客户端不伪造：SPEC 明写「不硬编码」「白名单」「只回传 element」「不得伪造」，并复验 `r10GrowthGaps.contract.test.ts` 的「不伪造替换 id」与 `r7ImpactApply.ts` 的 impact 查询串未被弱化。
3. 端到端闭环可达：五步关键词 + 选择器 `#r14-advice-action-btn` / `.r14-advice-cta` / `.r14-advice-empty` / `[data-r14-source` + R7 复用 `.r7-compare-title` / `.apply-done` + AI 四判据「可见汉字「AI」」「真实数据结论」「依据」「CTA」+ 空态文案。
4. 不回退：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 不变，无 `--r14-` 新视觉变量。

下游并行消费约定：backend 只认 A/B 的端点、字段、code 与 Go 用例；h5-ui 只认 A.2/A.3 schema、C 选择器与五步闭环。两端按同一 `advice-action-v1` 对接，无需猜测。

## 不回退清单

- R1–R6 诚实文案与空态口径、25% AoE 真实数值、顶栏脱敏策略不变。
- R7 两步确认流程与服务端事务写入不变；本规范只复用，不替换。
- R10 五条件与「不虚构」红线不变；防伪造 id 断言不弱化。
- 令牌版本锚点不变；本轮零新增视觉令牌。
