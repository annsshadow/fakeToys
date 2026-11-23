# R14 后端：权威一键落地端点 /api/v1/me/advice/action

## 1. 结论

R14 规范要求的「服务端裁剪 id/replace、客户端不能自行猜候选」已落地为独立只读端点
`GET /api/v1/me/advice/action`；候选 id、替换 id、收益预估、证据字段均来自服务端真实战报、
当前构筑、用户拥有表与配置版本。游客得到 **200 + 诚实空态**（NOT_AUTHENTICATED），不返回 401；
未知 id/replace 查询参数被忽略而非信任。未放宽任何写入校验或防作弊规则。

## 2. 改动

| 文件 | 改动 |
|---|---|
| `laiyipao/server/internal/service/advice_action.go` | 新增 `AdviceAction` / `AdviceActionEmptyReason`、schema_version=`advice-action-v1`；`AdviceAction(ctx,userID,element,reaction,source)` 按 6 类空态返回；真实 JSONB 统计 `elements_used`/`reactions_used`，调用现有 `recImpactCTA` 与 `RecommendationImpact` 产出权威 CTA。`Params any` 保证空态 wire 为 `null`（NormalizeNilSlices 对 nil interface 不改写）。 |
| `laiyipao/server/internal/service/recommendations.go` | `recImpactCTA` 增加 element 过滤参数；element="" 时既有推荐行为不变，skills 按技能元素、equipment 按装备元素过滤候选。 |
| `laiyipao/server/internal/httpapi/handlers_advice_action.go` | 新 handler；query 白名单 element/reaction/source，重复参数 400，未知参数忽略。 |
| `laiyipao/server/internal/httpapi/middleware.go` | 新增 `optionalUser`；仅合法 Bearer 注入 user_id，缺失/无效凭证继续以游客身份执行（限定该只读端点）。 |
| `laiyipao/server/internal/httpapi/routes.go` | 在 me 路由组前注册 `v1.Get("/me/advice/action", optionalUser(s.Svc), s.meAdviceAction)`。 |
| `laiyipao/server/internal/httpapi/routes_advice_action_ownership_e2e_test.go` | 3 条 E2E：拥有权过滤 / 元素定向 / 六类空态。 |

## 3. test-first 证据

RED（新增 E2E，尚未注册路由）：

```
go test ./internal/httpapi -run TestE2EAdviceAction -count=1 -v
routes_advice_action_ownership_e2e_test.go:53: 未拥有候选的 advice/action = 404
  map[error:map[code:http_error message:Cannot GET /api/v1/me/advice/action]]
exit 1
```

实现后第一次 GREEN 失败（契约 params 必须 null）：

```
routes_advice_action_ownership_e2e_test.go:65:
  空态不得下发非零 id/replace/params：id=0 replace=0 params=map[]
routes_advice_action_ownership_e2e_test.go:145: 同族失败
exit 1
```

根因：`NormalizeNilSlices` 将 nil map 归一为 `{}`；把 `Params` 从 `map[string]any` 改为 `any`
后，nil interface 保持 `null`，成功态仍编码对象，未修改通用归一化契约。

GREEN：

```
go test ./internal/httpapi -run 'TestE2EAdviceAction|TestE2ERecommendation|TestE2EMeRecommendation|
  TestE2EAdminRecommendations|TestE2ER10Recommendation|TestEveryRouteHasClientCaller' -count=1 -v
--- PASS: TestE2EAdviceActionSkipsUnownedCandidate (0.12s)
--- PASS: TestE2EAdviceActionEquipmentOwnedCandidate (0.10s)
--- PASS: TestE2EAdviceActionHonestEmptyStates (0.14s)
--- PASS: TestE2ERecommendationImpactCTAIsReachable (0.09s)
--- PASS: TestE2ERecommendationImpactCTAOmitsInvalidIds (0.07s)
--- PASS: TestE2ERecommendationCTASkipsUnownedSkill (0.10s)
--- PASS: TestE2ERecommendationCTASkipsUnownedEquipment (0.11s)
--- PASS: TestE2EMeRecommendationImpactFromRealBattles (0.09s)
--- PASS: TestE2ER10RecommendationCTAReasonAudit (0.14s)
--- PASS: TestE2EMeRecommendationsFromRealBattles (0.09s)
--- PASS: TestE2EAdminRecommendationsFromDashboard (0.17s)
--- PASS: TestEveryRouteHasClientCaller (0.05s)
PASS  ok  github.com/laiyipao/server/internal/httpapi  1.309s
```

`go vet ./internal/service ./internal/httpapi` exit 0；`gofmt -l` 改动文件无输出。
路由闭合守卫无需新增豁免：`laiyipao/miniapp/src/pages/_shared/r14AdviceAction.ts:120`
已真实请求 `/me/advice/action?element=...&source=...`。

## 4. 全量结果

第一次全量 `go test ./internal/httpapi -count=1 -parallel 1 -timeout 20m` 失败（388.506s），
失败点与本轮改动无关：共享测试库 `admin_audit_logs` 表缺失、`wallet.keys` 列缺失
（SQLSTATE 42P01 / 42703），属于 QA 容器重建后历史半迁移库的环境状态。

据此新建独立库并执行 `go run ./cmd/seed`（内含 migrate），种子写入完整
（敌人 22 / 技能 42 / 装备 18 / 宝石 8 / 皮肤 6 / 关卡 100），再以该库重跑：

```
go test ./internal/httpapi -count=1 -parallel 1 -timeout 20m
ok  github.com/laiyipao/server/internal/httpapi  359.457s
```

H5 消费层契约测试（只读交叉验证）`r14AdviceAction.contract.test.ts` 与
`r14AdviceActionFlow.contract.test.ts`：2 files / 12 tests passed。

## 5. 零真值声明

本轮未新增真实域名/IP/口令/私钥；测试 DSN 由 `docker port` 在本地 shell 派生并立即注入进程环境，
输出经掩码处理；仓库文件仅含 `127.0.0.1` 回环地址。未执行任何 git 操作。
