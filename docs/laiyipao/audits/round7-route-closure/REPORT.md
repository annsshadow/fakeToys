# R7 路由闭合审计：推荐收益接口临时豁免清除

## 背景

t65 为 `GET /api/v1/me/recommendations/impact` 在 `laiyipao/server/internal/httpapi/route_consumption_test.go` 登记了临时 `KNOWN_UNSERVED` 豁免，理由是当时写域仅限 server，前端接入由 h5-ui 后续任务完成。条目注释明确要求「h5-ui 接上后必须立即删除本条」。

## 真实消费证据

- `laiyipao/miniapp/src/pages/_shared/r7ImpactApply.ts:146-159` 的 `fetchRecommendationImpact()` 真实请求 `'/me/recommendations/impact?kind=' + kind + '&id=' + candidateId + '&replace=' + replacedId`，并用 `normalizeImpactCard` / `normalizeImpact` 消费顶层 `impact`。
- `laiyipao/miniapp/src/styles/r7ImpactApply.contract.test.ts:69` 断言客户端源码包含该路径，契约测试 15/15 绿。
- t66 真浏览器截图已显示接口驱动的诚实空态与收益卡（见 t66 交付记录）。

## 变更

删除 `laiyipao/server/internal/httpapi/route_consumption_test.go` 中 `GET /api/v1/me/recommendations/impact` 的临时豁免条目；其余两条（微信登录外部凭证门禁、外部 Prometheus 抓取器）保持不变。

## 验证

- 基线（豁免仍在）：`go test ./internal/httpapi -run "^TestEveryRouteHasClientCaller$" -count=1 -v` PASS，但端点由豁免跳过，不构成闭合证据。
- 删除豁免后：`go test ./internal/httpapi -run "^TestEveryRouteHasClientCaller$" -count=1 -v` PASS（0.07s），说明守卫是从客户端源码真实找到调用方，而非靠豁免放行。
- `gofmt -l internal/httpapi/route_consumption_test.go`：无输出。
- 全量 httpapi 包：`go test ./internal/httpapi -count=1 -timeout 12m` PASS（432.295s，exit 0，无 FAIL）。

## 结论

推荐收益预估端点已由 H5 成长页真实调用，路由闭合守卫在无豁免状态下绿灯；无客户端“假接”或测试豁免永久化。
