# laiyipao 聚焦缺陷审计台账（2026-11-19 · 第二轮 · lyp-bughunter）

> 本轮主题：**跨端契约漂移**（server DTO ↔ admin/miniapp 类型）与**长尾异常分支**。
> 结论：**未发现可复现的真实 bug**（负结果轮次）。所有怀疑点按「复现证据优先」逐个验证后否决——
> 要么逐字段对齐、要么有契约测试守卫、要么条件不可达、要么是刻意的 JS 精度保护。
> 不为凑数臆造修复（遵循 43.4 策略：不强造 bug，编译错误不算 RED→GREEN）。

---

## 1. 门禁命令与结果（本轮实跑）

| 命令 | 结果 | 证据 |
|---|---|---|
| `cd laiyipao/server && go test ./... -timeout 20m` | **EXIT=0 全绿** | internal/service / httpapi / store / domain / seeder / auth / config 等全部 ok；6 个包无测试文件；日志末行 EXIT=0 |
| `cd laiyipao/admin && pnpm test` | **EXIT=0 全绿** | Test Files 37 passed；Tests 498 passed |
| `cd laiyipao/admin && pnpm build` | **EXIT=0** | vue-tsc 无错 + vite build ✓ |
| `cd laiyipao/miniapp && pnpm test` | **EXIT=0 全绿** | Test Files 157 passed/1 skipped；Tests 1527 passed/2 skipped |
| `cd laiyipao/miniapp && pnpm build:h5` | **EXIT=0** | uni-app DONE Build complete |

> 四个前端门禁**串行**执行，避开第一轮发现的并行 worker 资源竞争 flaky。

## 2. 本轮已验证 bug 清单

**无**。没有候选满足「可复现 + 因正确原因失败的 RED 测试」门槛，故本轮不提交代码修复，无 RED→GREEN 记录。

## 3. 逐个否决的候选（附依据）

| # | 候选（跨端契约漂移 / 长尾分支） | 否决依据 |
|---|---|---|
| 1 | `LevelSummary.seed` 类型漂移：miniapp `types.ts:102` 为 string，server `domain/levelgen.go:167` 与 admin 生成物为 number | miniapp 全仓库**零消费** `LevelSummary.seed`；battle token 用独立 `seed_str` 字符串（server `service/game.go:59-60`，`routes_e2e_test.go:814-815` 钉死为 JS 精度保护）；`AdminLevelView.Seed` 在 server `adminviews.go:78` 本就是 string，admin 生成物正确。非 bug。 |
| 2 | GameConfig 字段跨端漂移 | server `service.go:1499` / miniapp 手写 `client.ts:66` / admin 生成物 `types.gen.ts:430` 逐字段一致（version、require_login、levels、enemies、skills、…、server_time）。 |
| 3 | GameConfig 304 条件请求 + 时钟校准链 | server `routes.go:727-731` 命中即 304；miniapp `client.ts:759 fetchConfigIfVersion` 用 HTTP `Date` 头校准；`cors_expose_date_test.go:31` 守卫 Date 头暴露，`compress_test.go:251` 守卫 304/200。 |
| 4 | 读缓存时用 `cached.config.server_time`（过期快照）校准时钟，疑似护盾判断漂移 | 304 分支会用 Date 头**重新校准**（非只靠缓存）；`store/game.test.ts` 「304 路径仍用 Date 头校准时钟」专项用例 + `calibrateServerClock` 过滤非有限/≤0。长尾已被覆盖。 |
| 5 | request 401 刷新链异常分支 | 单飞刷新（refreshInflight）、401 复用首请求 idempotencyKey、刷新失败清令牌 + notifyAuthFailure、`expires_at` 落盘；`client.test.ts:123-166` 三组鉴权失败信号测试。 |
| 6 | httperr 码表跨端漂移 | `server/internal/httpapi/httperr/codes.go` 码→状态→RetryAfter 权威表；`error_entry_test.go` AST 扫全仓守卫；`login_required=403`（非 401）、所有 503 带 Retry-After。 |
| 7 | SignInStatus / OfflineRewardStatus 长尾 | `economy.go:610-653` 按 periodStart "daily" 的 DATE 串判今日、cycleLen 复用日历缓存；`signin_status_test.go` + `signin_status_cache_test.go` 覆盖。 |
| 8 | Attacker 转换两副本漂移（`store/game.ts:156` vs `i6.e2e.test.ts:50`） | 10 个字段（attack/crit/multiplier/reactionMult/elementCap/tier/coef + heatCap/armor/mechanic）逐字段一致，后三项均 `?? 0` 兼容老战报。 |
| 9 | 技能等级烘焙公式 TS vs Go（生产验真核心路径） | TS `skill.ts skillBaseDamageAtLevel` = `coef=1000+(lv-1)*50`（夹紧 [1,10]），Go `domain/skillrules.go CoefPermilleAt` 同式整除；两端读同一 `formula_vectors.json`，`replay_skills_contract.test.ts` + Go `replay_skills_contract_test.go` 共用字面期望值。等级 1 恒等快路径保存量战报。 |
| 10 | 服务端路由参数解析丢弃错误 | `handlers_admin.go` / `routes.go` 全部 `id, err := ParseInt/Atoi` 并检查 err；`query_param_test.go` AST 守卫禁止 `x, _ := Atoi(...)` 等丢弃错误写法。 |

## 4. 保密合规自检

- 全程**未读取、未回显** `laiyipao/deploy/.local.keys` 真值。
- 台账与对话**未写入**任何真实域名 / IP / 口令 / 私钥 / SSH 端口 / 个人路径；涉及处均为字段形状或占位描述。
- 临时日志（`.golytest8.log` / `.frontend8.log`）由 `.gitignore` 的 `*.log` 拦截，不入库。
- `git status --porcelain` 已自检；本轮仅新增本台账，未碰他人在途改动；**未 git commit / push**。

## 5. 剩余线索（非本轮 bug，供后续聚焦）

1. **I-6 验真副本的技能等级烘焙缺口（低优先，当前不可触发）**：`miniapp/src/game/i6.e2e.test.ts` 的 `equipFromBuild` 直接用 `def.base_damage` **不烘技能等级**，而生产 `equippedFromSnapshot`（`replay.ts:206`）与 Go `domain.EquippedFromSnapshot` 都烘等级。若 I-6 将来改用**已升级账号**（level>1）跑，`hashA` 会与按服务端快照重算的哈希不一致，测试副本会自相矛盾。当前 I-6 用游客新号（level=1，走恒等快路径）故不可触发，无法构造「因正确原因失败的 RED」，本轮**不臆测修复**。建议：若扩展 I-6 覆盖老账号，直接复用 `equippedFromSnapshot` 单一真源，而非保留副本。
2. **admin 测试并行 flaky**：与 miniapp 并行（合计 37 vitest workers）曾 1 例失败，串行单独跑全绿，属资源竞争；CI 稳定性有要求可限制 worker 数。
3. **测试环境噪声**：`skipLink.ts` 的 document is not defined（jsdom 清理后 MutationObserver 回调）、RecomputesView 的 navigation-not-implemented 警告，均为 jsdom 噪声，生产路径有无害守卫。
4. laiyipao 连续多轮负结果（R287–R291 竞态加固 + 本轮聚焦审计均无新 bug），常见 bug 模式与跨端契约已系统覆盖；后续建议转向**运行期真机 E2E**（需服务端 + 真实 PG）以发现静态审计看不见的时序/环境问题。
