# R13-BUG-1：穿透弹丸 hits > shots 导致结算被拒

## 1. 现象与证据

- 来源：`docs/laiyipao/audits/round13-reaction-learning/README.md:53`。
- 真实电 + 腐蚀 127 反应局结算返回 422：`命中 165 > 发射 160`（`ErrInvalidHitRate`）。
- 该局是合法高穿透构筑；战绩不入账是真体验损失，不是前端提示问题。

## 2. 根因（跨端同源缺陷）

| 端 | 位置 | 旧行为 |
| --- | --- | --- |
| TS | `laiyipao/miniapp/src/game/engine.ts:1405` | `fire()` 每次开火 `this.shots++`（正确） |
| TS | `laiyipao/miniapp/src/game/engine.ts:1515-1522` | 穿透弹丸每命中一只主目标就 `this.hits++`；后续穿透只是 `continue` 回主目标循环，于是同一次开火被重复计数 |
| Go | `laiyipao/server/internal/domain/engine_combat.go:417-441` | 对称实现：主目标分支无条件 `e.Hits++`；穿透分支 `PierceLeft--` 后回到同一循环 |
| 服务端校验 | `laiyipao/server/internal/domain/battle.go:504-514` | `in.Hits > in.Shots` → `ErrInvalidHitRate`，并另有发射数物理上界 |

结论：**前端/内核命中口径错误，服务端校验正确**。不能用放宽 `ValidateSettle` 的方式“修”；那样会把作弊上报一并放行。

旧注释已经写下正确口径（“hits 是有多少次开火命中了东西”），但代码没有兑现，属于注释与实现漂移。

## 3. RED：先把真实缺陷钉住

新增两条定向测试（均未改生产代码时执行）：

1. TS `laiyipao/miniapp/src/game/multi_target.test.ts` 新用例「穿透：一发弹丸命中 4 只敌人只能记 1 次命中（hits≤shots）」：

   ```
   AssertionError: 穿透一发命中 4 只却记 hits=4 > shots=1 —— 服务端会以 ErrInvalidHitRate 拒掉这局: expected 4 to be less than or equal to 1
   ```

2. Go `laiyipao/server/internal/domain/engine_pierce_hit_count_test.go`：

   ```
   engine_pierce_hit_count_test.go:57: 穿透弹丸一发多命中却记成 hits=4 > shots=1 —— 服务端 ValidateSettle 会拒掉这局
   ```

两条测试都先断言 4 只敌人确实掉血，再断言 `hits <= shots`，排除“夹具没打中所以假绿”。

## 4. GREEN：修的是计数口径，不是校验

- `laiyipao/miniapp/src/game/types.ts`：`Projectile` 增加 `hitCounted: boolean`。
- `laiyipao/miniapp/src/game/engine.ts`：
  - 弹丸创建时 `hitCounted: false`；
  - 主目标命中分支只在 `!p.hitCounted` 时 `this.hits++` 并置位；后续穿透 / 溅射 / 弹射不再累加。
- `laiyipao/server/internal/domain/engine.go`：`Projectile` 增加 `HitCounted bool`（注释写明口径）。
- `laiyipao/server/internal/domain/engine_combat.go`：
  - 弹丸创建时 `HitCounted: false`；
  - 主目标命中分支只在 `!p.HitCounted` 时 `e.Hits++` 并置位。
- 两个引擎的旧注释同步改成「首次主目标命中」，避免再次出现注释正确、实现错误的漂移。
- 该字段不进 replay 事件流，`replay_hash` 计算口径不变；但 `engine_pure_vectors.json` 的快照含 `hits/shots`，已用 `scripts/gen-engine-pure-vectors.ts` 重新生成。

### 测试夹具配套

- 三个 TS 测试里手工构造的 `Projectile` 补 `hitCounted: false`：`corrosion_spread.test.ts`、`multi_target.test.ts`、`overheat_aoe.test.ts`。
- `laiyipao/miniapp/src/game/no_deadlock.test.ts` 的命中率下限从 0.30 调整为 0.25：原阈值按旧口径标定；新口径全 100 关实测最低 0.2975（第 34 关），0.25 仍比挂死时的 1% 高一个数量级，保留“命中率崩塌即报警”的原意图，仅消除口径变化造成的假红。

## 5. HTTP 边界实测（不是只跑单元测试）

新增 `laiyipao/server/internal/httpapi/routes_e2e_high_pierce_settle_test.go`：

1. 真实开局拿 token；
2. 先上报旧口径 `hits=4, shots=1` → 断言仍被 422 拒绝（证明服务端校验未放宽）；
3. 再上报修正口径 `hits=1, shots=1, kills=4` → 断言 200；
4. 直接查 `battle_records` 断言落库 `hits=1, shots=1`（不是只信响应码）。

测试库从环境变量注入 DSN（迁移 + 种子在独立库执行），真值与连接信息不入库、不写报告、不写日志。

## 6. 验证结果

- TS 定向：`npx vitest run --config vitest.config.ts src/game/multi_target.test.ts` —— 7 passed。
- Go 定向：`go test ./internal/domain -run TestPierceProjectileCountsOneHitPerShot` —— PASS。
- miniapp 全量：`npm test` —— 184 files passed | 1 skipped；1745 tests passed | 2 skipped。
- miniapp 类型检查：`npm run type-check` —— exit 0。
- Go domain 全量：`go test ./internal/domain -count=1` —— ok。
- `gofmt -l internal/ cmd/` —— 无输出；`go vet` —— exit 0。
- HTTP E2E（独立迁移+种子库）：`go test ./internal/httpapi -run TestE2EHighPierceSettleAccepted` —— PASS。
- service 重算/回放定向：`go test ./internal/service -run "TestRecompute|TestReplay" -count=1` —— ok 3.525s。
- vectors 校验：`go run ./cmd/vectors -check` —— OK（reaction_specs 7 条、smoke_levels 6 关、level_seeds 100 关）。
- 全量 httpapi：`go test ./internal/httpapi -count=1 -parallel 1 -timeout 15m` —— ok 486.105s（exit 0）。

### 顺带修掉的全量测试隔离缺陷

首次全量 httpapi 出现一条与本次引擎改动无关的失败：
`TestE2EAdminRecommendationsFromDashboard` 用
`INSERT ... VALUES ((SELECT id FROM users ORDER BY id LIMIT 1), ...)` 造战报，
在独立库中该子查询为空，触发 PostgreSQL `23502`（user_id 非空约束）。
这是测试自身依赖“库里恰好有用户”的顺序耦合：已在
`laiyipao/server/internal/httpapi/routes_recommendations_e2e_test.go` 改为
用 `e.newPlayer` 自建玩家并传 `$1`，修复后定向 PASS，全量转绿。
服务端生产代码与防作弊语义未因此改动。

## 7. 零真值声明

本轮所有新增与修改文件均不包含真实域名、IP、口令、私钥或个人环境路径；数据库连接仅通过 `TEST_DATABASE_URL` / `DATABASE_URL` 环境变量注入。未执行任何 git 操作。
