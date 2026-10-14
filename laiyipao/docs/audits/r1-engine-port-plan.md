# R1 tick 循环移植计划

> 状态：2026-10-14 制定。Block A 已完成（见文末）。

## 为什么需要这份计划

`engine.ts` 的 tick 循环是 1855 行里最危险的一块。它的偏差**不会报错**，
只会让 `replay_hash` 失配 —— 而失配的表现是「验真失败」，即
**把诚实玩家判成作弊**。这是本项目唯一能把「可刷分」漏洞换成 P0 事故的地方。

所以移植它必须满足两条：

1. **任何时刻都有「当前实现对不对」的可执行答案。**
   不是「等全部搬完再跑整局看看」，而是每加一块就多一条独立可验证的增量。
2. **不接任何拒发路径，直到它被证明对。**
   `SETTLE_RECOMPUTE_MODE` 保持 `off`（现有缺省），引擎落地后先在 `shadow` 档
   观察若干天 —— 台账与熔断在第 2026-10-14 第二步就已经造好了。

## 分解判据

按**「每块结束时能独立验证」**切，不按「代码上看起来是一个函数」切。

判据具体是：**在那一块结束时，存在一个 TS 能跑出、Go 也能跑出、且两者
事件流逐条相同的场景。** 达不到这个条件的切法不要采用 ——
那只会得到「大块代码一起对或者一起错」。

## 依赖图

```
A 骨架 + tick 序        ← 无依赖，先做
│
├─ B 波次与刷怪
│    └─ C 敌人移动与元素
│         ├─ D 开火与弹道  ──┐
│         └─ E 地形接线      ├─ F 卡牌（依赖 D）
│                             │
G 结算与哈希（依赖 B~F）  ────┘
```

E 同时依赖 C（敌人）与 D（弹丸），因为地形要操作两者。
F 依赖 D：选牌会改攻方系数，而攻方系数进 D 的伤害计算。

## 各块的验收场景

⚠️ **Block A 的验收与 B~F 不同**：它拿不到 TS 逐事件对齐，
因为 `start()` 会调 `beginWave(0)`，而 `beginWave` 属于 B 块。
所以 A 的验收是**直接验证排序不变量**（见下），第一次真正的跨端逐事件对齐在 B。

| 块 | 内容 | 验收 | 断言 |
|---|---|---|---|
| **A** | 全部实体类型（字段齐）、`BattleEngine` 全部字段、`Start`、`Step`、`emit`/`record`/`DrainEvents`、`CheckStalemate`、`Finish`、`CalcStars`、`ReplaySkillsSegment`/`BuildSnapshotSegment` | **排序不变量**（不依赖 TS 对齐）：停滞恰在 tick 30000 触发、`elapsedMs == tick×50`、终局后 `Step` 不再推进 tick、停滞事件在战斗逻辑之前、`stalemate` 进 replay | 直接断言 |
| **B** | `beginWave`、`spawnEnemy`、`updateSpawns`、`checkWaveEnd` 波次分支、`stepEnemyMotion` | **第一次 TS 逐事件对齐**：第 1 关、空构筑跑到本波清空 | 刷怪 tick 序列 + 事件流逐条 |
| **C** | `updateEnemies`、`leakDamage`、`leakDamageFor`、`withinEnemy` | 敌人推进到漏怪线 | 漏怪事件 + `leaked` |
| **D** | `updateAutoFire`、`fire`、`updateProjectiles`、`checkProjectileHit`、`hitEnemy`、`currentAttacker`、`defenseArmorPermille`、`triggerSteamBurstAoe`、`applyAoe` | 完整战斗（无地形无选牌） | **整局 `replay_hash`** —— 里程碑 |
| **E** | `updateTerrain`、`terrainCtx`、`withinTerrain` | 含地形关 | 地形事件 + `terrain_used` |
| **F** | `finishCardSelect`、`skipCards`、`applyReplayDecision`、`recordPick`、`applyCard`、`applyAttribute`、`applyMechanic` | 含选牌脚本的战斗 | 选牌索引序列 + 哈希 |
| **G** | `replayHash`、`settleInput` 所需字段 | `replay_vectors.json` 那 5 个用例 | hash + expected_stats 逐项 |

**D 就能拿到整局哈希**（无地形无选牌时）。这是第一个真正的里程碑：
它证明「服务端重算」在技术上成立。G 只是把哈希与结算字段接上，
其场景是 B~F 的子集。

### 关于未移植块的处理

B~F 的方法在 A 阶段是**空实现**（no-op stub），`Step` 按最终顺序调用它们。
替换顺序固定：B → C → D → E → F。

⚠️ 这意味着 A 阶段 `Step` 产出的事件流**必然**与 TS 不同。
这不是缺陷，是「尚未搬完」的如实反映；A 的验收因此不比对事件流内容，
只比对排序不变量。等 B 替换掉 `beginWave` 之后，第一次逐事件对齐才有意义。


## tick 序的三条硬约束

`step()` 里有三处「位置错了就静默失配」，源码注释已经点明，移植时必须逐条对齐：

1. **`cardSelectDone` 收尾必须在 `applyReplayDecision` 之前。**
   否则同一 tick 内 `applyReplayDecision` 被调两次，
   把下一波的决策当成本波的第二张牌（实测 `picks` 记成 `[-3,-3,-1,-1]`）。
2. **`applyReplayDecision` 必须在 `tick++` 之前。**
   tick 会写进每条 replay 事件，多一格会让后续所有时间戳整体偏移。
3. **`checkStalemate` 必须在 `tick++` 之后、任何战斗逻辑之前。**
   这样「到上限」这件事本身是这次 tick 的第一个事实。

这三条要写成守卫，但**不是都在 A 块**：

| 约束 | 验证时机 | 原因 |
|---|---|---|
| (1) `cardSelectDone` 在 `applyReplayDecision` 之前 | **F 块** | A 阶段 `applyReplayDecision` 是空实现，两种顺序行为相同，写不出能失败的守卫 |
| (2) `applyReplayDecision` 在 `tick++` 之前 | **F 块** | 同上 |
| (3) `checkStalemate` 在 `tick++` 之后、战斗逻辑之前 | **A 块** ✅ | 它现在就有行为：差一个 tick 就差 50ms，而停滞事件进哈希 |

F 块里 (1)(2) 的可观测症状是**选牌索引序列错乱**（TS 注释记的实测：
`picks` 记成 `[-3,-3,-1,-1]`）—— 那是具体、可断言的，
而 A 阶段只能断言「两种顺序一样」，那不是守卫。

**「此刻无法验证」要写下来，不要留到忘了。** 否则半年后有人看到
`Step` 里那几行注释，以为三条都已经有守卫兜着。

## 每块必须有的东西

1. **契约向量**：TS 侧生成器扩展，场景与 Go 侧一一对应
2. **逐步比对**：不是只比最终哈希 —— 中间错了又绕回来的实现能骗过哈希
3. **变异验证**：每块至少 1 处，证明守卫抓得住
4. **还原态全量**：变异后必须重跑还原态，不能只看变异态

## 明确不做的事

- **不在任何一块里接线到 `SettleBattle`。** 接线是独立的一步，
  需要「整局重算 == 契约期望值」的全绿断言，而不是「某几块绿了」。
- **不提前提交必然红的测试。** 上一步的规矩：长期红灯的唯一效果是被忽略。
- **不改 TS。** 契约向量的期望值来自 TS，那是玩家设备上真实运行的代码。
  改它会让「合法的一局」的定义跟着改。

## 进度

| 块 | 状态 |
|---|---|
| A 骨架 + tick 序 | ✅ 已完成（`server/internal/domain/engine.go` + `engine_block_a_test.go`，13 条守卫） |
| B 波次与刷怪 | ⬜ 下一步 |
| C 敌人移动与元素 | ⬜ |
| D 开火与弹道 | ⬜ |
| E 地形接线 | ⬜ |
| F 卡牌 | ⬜ |
| G 结算与哈希 | ⬜ |

### A 块的完成度（诚实口径）

**已验证**：tick 节奏（`elapsed == tick×50`）、停滞恰在 tick 30000 触发、
停滞早于战斗逻辑、终局后 `Step` 不推进 tick / 不涨 elapsed / 不动热量、
`DrainEvents` 清空队列、`Finish` 只发一条终局事件、构造纯度、UID 单调、星级阈值。

**尚未验证**：与 TS 的任何逐事件对齐（A 阶段必然不一致，因为 `BeginWave` 是空实现）。

**已知无效的守卫**：`MUT-T` 变异暴露的 —— 排序约束 (1)(2) 在 A 阶段写不出
能失败的守卫，因为 `applyReplayDecision` 是空实现。见上表。

**顺带修掉的既有问题**：`battle.go` 的注释声称 `TestTickMsMatchesClient`
锁住了 `TickMs`，而**那条测试并不存在**。已补
`TestTickConstantsMatchClient`（读 TS 源码字面量比对 `TICK_HZ`/`TICK_MS`/`MAX_BATTLE_TICKS`）。
注释里的假声明比没有注释更糟 —— 它让人以为已经有人管过了。
