# R10 推荐收益 CTA 不可达核实（t100）

> 证据环境：本地回环 E2E（真实 PostgreSQL + Fiber `app.Test` 请求链），**非物理真机**。未执行 git；未写入真实域名/IP/口令/私钥。

## 1. 待核实主张

`docs/laiyipao/audits/round10-gameplay/README.md:50` 称：有近 7 场统计后，技能/装备页「真实收益预估」仍是「接口未提供真实预估」、没有应用 CTA；客户端 `loadR6Recommendation()` 只在 CTA `params.id`/`params.replace` 均非零时请求 impact，实测服务端 CTA 不带这两个参数。截图 `r10-08-skills-impact-unavailable.png` / `r10-08-equipment-impact-unavailable.png`（非物理真机）。

源码核实：

- `laiyipao/miniapp/src/pages/skills/skills.vue:448-462`、`equipment.vue:441-455`：门槛确为两个参数非零。
- `laiyipao/miniapp/src/pages/_shared/r7ImpactApply.ts:146-159`：请求 `/me/recommendations/impact?kind=&id=&replace=`。
- `laiyipao/server/internal/service/recommendations.go:163-259`（t77 归属修复）：候选必须**玩家已拥有**、进度解锁、同槽/同主战、数值更优、未占槽；无候选时 `:218`/`:257` 返回诚实 basis，挂在推荐卡 `insights[].basis`。
- `laiyipao/server/internal/service/recommendation_impact.go:268-376`：impact 预览只校验配置存在与当前占槽，不校验归属；写入端 `SaveLoadout` / `SetEquippedEquipment` 才校验归属。

因此 R10 观察到的「有统计但无 id/replace」与 t77 的安全语义一致，不能直接判为新缺陷。

## 2. RED → GREEN 决定性对照

新增 `laiyipao/server/internal/httpapi/routes_recommendation_r10_cta_audit_e2e_test.go`，复用真实注册用户、真实战报与真实 HTTP 请求。

RED（按 t98 字面主张写断言：有历史就必须下发 id/replace）：

```
go test ./internal/httpapi -run ^TestE2ER10RecommendationCTAReasonAudit$ -count=1 -v
routes_recommendation_r10_cta_audit_e2e_test.go:28: R10 声称有历史的推荐 CTA 应下发 id/replace；实际 id=0 replace=0；basis=场均击杀来自 battle_records.kills（1 场） 元素覆盖 3/5 已拥有技能里没有同为主战且基础伤害高于当前 干冰弹(id=4) 的候选，未给出替换 id
exit 1
```

GREEN（同一测试改为对照证据）：

1. 新号 + 1 场真实战报，未拥有 skill 2 / equipment 11：CTA 均无 id/replace；basis 分别含「已拥有技能里没有同为主战且基础伤害高于当前」「已拥有装备里没有同槽且加成高于当前」。卡片依据已向玩家解释收益为何不可用。
2. 强行请求 t98 隐含的配置候选：`?kind=skill&id=2&replace=4` 与 `?kind=equipment&id=11&replace=10` 的 impact 预览均 200，但随后 `PUT /me/loadout`（skill 2 未拥有）与 `PUT /me/equipment`（`[1,11]`，11 未拥有）均 **400**。若服务端按 t98 猜测下发这些 id，玩家点「应用推荐 → 确认」必定失败，正是 t77 修复的 R7-BUG-2。
3. 真实授予 `user_skills(skill_id=2)` / `user_equipment(equipment_id=11)` 后，同一推荐接口自动下发权威 id/replace，两个 `requireReachableImpact` 断言 200 且回显一致。

```
go test ./internal/httpapi -run ^TestE2ER10RecommendationCTAReasonAudit$ -count=1 -v
--- PASS: TestE2ER10RecommendationCTAReasonAudit (0.13s)
ok github.com/laiyipao/server/internal/httpapi 0.164s
exit 0
```

## 3. 结论

**假阳性（不是新的服务端缺陷）**。R10 的收益卡不可用，根因是玩家当时只拥有初始 3 技能/2 装备，已拥有集合里没有更优的替换项；t77 故意不下发不可写 id，防止「预览能看、应用必败」。服务端不得为了点亮 CTA 而编造归属，也不得放宽写入校验。

本任务只补一条审计型 E2E，不修改生产代码；客户端门槛保持不动。若产品希望把「已拥有列表里没有更优候选」直接显示在收益卡的错误位，需要另开前端任务消费推荐卡已有的 `insights[].basis`，服务端不伪造收益字段。

## 4. 验证命令

```bash
cd laiyipao/server
go test ./internal/httpapi -run ^TestE2ER10RecommendationCTAReasonAudit$ -count=1 -v   # --- PASS (0.14s)，ok 0.176s，exit 0
gofmt -l internal/httpapi/routes_recommendation_r10_cta_audit_e2e_test.go              # 无输出，exit 0
go vet ./internal/httpapi                                                              # exit 0
go test ./internal/httpapi -count=1 -parallel 1 -timeout 15m                           # ok 373.619s，exit 0
```
