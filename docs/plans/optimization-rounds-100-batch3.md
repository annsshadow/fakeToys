# 100 轮实质优化 · 轮次计数台账（第三批）

- **目标**：全面优化 fakeToys monorepo 的性能与功能，**每轮必须有性能或功能的
  实质性代码进展**（含测试验证），累计完成 100 轮。
- **口径（用户 2026-10-31 重申）**：验证轮、纯文档轮、单纯编译通过**不计入轮次**。
  每轮 = 一个真实的性能改进（异步化 / 并行化 / 缓存 / 查询优化 / 索引 / 前端体积等）
  或功能改进（新端点 / 新能力 / 修复真实缺陷），并附带测试、基准或 EXPLAIN 证明其效果。
- **验证基线**：全仓门禁 `node scripts/check-warnings.mjs`；laiyipao server `go test ./...` 全绿。
- **说明**：第二批台账（`optimization-rounds-100-real.md`）作废重开，本批从第 1 轮重新计数。

| 轮次 | 日期 | 主题 | 关键改动 | 验证 |
|---|---|---|---|---|
| 1 | 2026-10-31 | **`/me` 三个独立查询并行执行**：`/me` 是最热端点（每次冷启动必拉），`LoadBuildSnapshot` 一趟查 6 张表，其余三个小查询原本串行等它，白等两倍延迟 | `httpapi/routes.go` 的 `me()` 在拿到 build 后并行执行 `LoadMaxStage` / `ComputeRatingFor` / `MyAccount`（`sync.WaitGroup`，各 goroutine 只写自己的变量，主协程 `wg.Wait()` 后读取，无数据竞争；任一失败仍按原优先级 fail-loud，行为不变） | `go build ./...` 通过；`TestE2EMeReturnsAccountInfo` / `TestE2EMeStars` / `TestPlayerLoginEndpoint` 通过；`go test ./...` 全绿。提交 `9dca34880` |
| 2 | 2026-10-31 | **玩家列表端点缺失索引补齐**：`orders` 没有任何 user_id 索引、`redeem_usages` 只有 `(code_id, user_id)` 唯一约束、`user_purchases` 索引 `(user_id, item_id, purchase_date)` 与查询的 `ORDER BY created_at DESC` 不匹配 —— 三个 `/me` 列表端点每次全表扫描+排序 | 迁移 `00021_player_list_indexes.sql` 新增 `idx_orders_user (user_id, created_at DESC)` / `idx_redeem_usages_user (user_id, created_at DESC)` / `idx_user_purchases_user (user_id, created_at DESC)`；新增 `TestPlayerListIndexesExist` 存在性守卫（列序断言，防止被改回无索引状态） | 迁移应用到开发库成功；新测试通过；scratch 库灌 5000 行流水后 EXPLAIN 三查询全部走新索引（orders 为 Index Scan，其余为 Bitmap Index Scan）；`TestE2EMyOrders` / `TestE2EMyPurchases` / `TestE2EMyRedemptions` 等 e2e 通过；`go test ./...` 全绿 |
| ... | | 持续至 100 轮 | | |

> 状态：进行中。当前累计 **2 轮实质优化**，距 100 轮还差 98 轮。