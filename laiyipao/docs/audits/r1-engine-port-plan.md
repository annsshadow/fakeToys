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
| A 骨架 + tick 序 | ✅ `engine.go` + `engine_block_a_test.go`（13 条守卫） |
| B 波次与刷怪 | ✅ `engine_wave.go` + `engine_wave_contract_test.go`（**首次跨端逐事件对齐**，5 场景 / 685 轨迹步） |
| C 敌人推进与漏怪 | OK engine_enemy.go + engine_enemy_test.go（12 条纯逻辑） |
| D 开火与弹道 | ⬜ 下一步 —— **里程碑：拿到整局 eplay_hash** |
| D 开火与弹道 | ⬜ |
| E 地形接线 | ⬜ |
| F 卡牌 | ⬜ |
| G 结算与哈希 | ⬜ |

### C 块的完成度（诚实口径）

实现 `engine_enemy.go`：`updateEnemies` / `leakDamage` / `leakDamageFor` /
`defenseArmorPermille` / `breachDamage` / `withinEnemy` / `filterAliveEnemies`，
以及 `NewBattleEngine` 里此前缺失的初始化（`BaseHP` / `BaseHPMax` /
`BaseArmorPermille` / `TotalEnemies` / `ActiveSlots` / 热量上限加成）。

**跨端对齐**：9 场景全绿，两个 leak 伤害分支都被覆盖
（第 1 关走 `breachDamage`：1000 底血掉到 700/820/925，对应护甲 0/400/750‰；
第 25 关走 `attack > 0`：1760 底血掉到 1426，167 = 1760×342/3600）。

**B 块的两条已知差异已消除**：`x` 与 `spawn_progress` 现在真正参与比对。
`knownCrossEndDivergences` 清空，`TestCrossEndCoversMotionFields`
额外确认这两个字段的比对**不是死的**（跑一遍场景，若它们全程不变就红）——
只删登记、不加比对的话，「表为空」那条会绿而字段又被静默跳过。

**对齐抓不到、靠构造用例补上的 5 个**：
射程与抵达的互斥、射程边界 `<=`、冷却边界 `<= 0`、钻地/零攻击/零射程
三个前置条件、护甲三项相加（真实场景里永远只有一项非零）、负攻击力走
`breachDamage` 档（写成 `=== 0` 会掉进乘除分支得到负伤害，而 `applyArmor`
对负数原样返回 = 加血）。

**变异实测 10 处，2 处首轮存活**：
- MUT-C1 删掉 `x > BaseX` 互斥守卫 → **首轮存活**。原因是我把血池配成
  「一次就打空」，`if baseHP <= 0 { finish(); return }` 立刻短路，
  第二个分支根本没机会执行。改成 attack=36（单次 1000）、血池 100000 后
  精准命中：leaked=2、血 98000、两条 leak 事件、两条重放事件。
- MUT-C6~C10（射程边界、冷却边界、不剔除死者、忽略 burrow、半径去掉
  flyHeight）首轮全红。
- 另有 C2 除数、C3 护甲符号、C4 兜底 0、C5 事件 a/b 写反，首轮全红。

**教训（与 B 同源，但更隐蔽一层）**：
构造守卫的夹具必须按「要观测的现象**能发生**」来配。
漏怪守卫按「一次就死」配血池，它在结构上就观测不到「第二次」。

### 对齐终止条件：为什么停在 `card_select`

第 2 波必须先解掉卡牌选择，那要过 `rollWaveCards`（消耗 RNG，于是后续
所有波次的刷怪洗牌都会变）与 `skipCards` / `finishCardSelect` ——
三者都属于 F 块。

第一版让长场景硬跑满 22000 tick，结果 224 个采样点里 216 个是
`phase: card_select` 的死空间：向量大 4 倍，什么也没多验。
现在两端都在 `card_select` 停下（判据是「既不是 wave 也不是 prepare」，
这样将来新增终局状态不必改这里），第 2 波的 `atTick` 偏移由
`TestBeginWaveAtLaterIndex` 直接单测覆盖 —— 那个 bug 在第 1 波不可见，
因为那时 `e.Tick` 恒为 0，把 `+ e.Tick` 写成常量 0 也能全绿。

### 向量文件大小的教训

`replay` 是**累积**数组，逐步全量存 = O(n²)。
第一版让长场景逐步全记，向量直接变成 **42 MB** —— 没人会 review 42 MB，
没人看的契约等于没有契约。改成「replay 存本步新增」+「按 `sampleEvery`
抽样并对有事件的步强制采样」后是 **1.0 MB**。

抽样判据必须**两端各自可算**（`tick % sampleEvery == 0 || 本步新增重放事件`），
不能照抄 trace 的下标 —— 照抄下标等于「Go 跑到第 k 个采样点就断言第 k 个
期望值」，两端之间没有任何时序约束，退化成逐点查表，
「Go 少跑了两步」这类错误永远抓不到。
### B 块的完成度（诚实口径）

**已验证（跨端逐事件对齐）**：5 个真实场景 × 逐 tick 比对
`tick` / `phase` / `elapsed_ms` / 刷怪队列（长度+顺序+三字段）/ 每个敌人的
`uid`/`defId`/`y`/`hp`/`shield`/`armor`/`resist`/`dead` / 全部重放事件 / 全部渲染事件。

场景覆盖 4 个种子（含负种子与 0，抓 `INIT_XOR` 与 `asUintN` 的对齐）
与第 1、25 关（单刷怪组 / 多刷怪组）。

**已知差异（`blockBDivergences`，由 C 块消除）**：
`x` 与 `spawn_progress` —— 两者都由 `StepEnemyMotion` 驱动，
而它只被 `updateEnemies`（C 块）调用。向量由**完整** TS 引擎产出，
所以 TS 会推进、Go 侧此刻静止。

`TestBlockBDivergencesAreStillReal` **反向盯着这张表**：一旦 Go 侧开始动
（C 已落地），那条测试立刻红，并提示把比对改成真的、删掉对应条目。
静默跳过会变成没人记得的债。

**B 块的跨端对齐抓不到的两个 bug**（靠构造场景的针对性测试补上）：

1. `updateSpawns` 改成**正序**遍历 —— 五个真实场景**全部照样绿**。
   因为真实关卡把 `atTick` 拉开了，同 tick 到期的通常只有一只，
   而「删元素后跳过紧邻项」只在「一次删多项」时发生。
   由 `TestUpdateSpawnsHandlesMultipleDueInOneTick` 覆盖。
2. 空关卡判成胜利 —— 五个场景的 `waves` 都非空，抓不到。
   由 `TestEmptyLevelIsNotAutoWin` 覆盖。

**这两条是本轮最值得记的教训**：跨端对齐证明的是「在真实数据上两端一致」，
**不覆盖「代码想处理的形状」**。后者要靠构造用例。


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
