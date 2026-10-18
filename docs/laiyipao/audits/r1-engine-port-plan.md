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
| D 开火与弹道 | ⬜ 下一步 —— **里程碑：拿到整局 
eplay_hash** |
| D 开火与弹道 | ⬜ |
| E 地形接线 | OK engine_terrain.go + engine_terrain_test.go + engine_terrain_contract_test.go |
| F 卡牌 | OK engine_card.go + engine_card_test.go + engine_card_contract_test.go |
| G 结算与哈希 | OK —— 5 个整局用例 replay_hash **全部逐位相同** |

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

### D 块的完成度（诚实口径）—— 里程碑

实现 `engine_combat.go`：`updateAutoFire` / `fire` / `updateProjectiles` /
`checkProjectileHit` / `hitEnemy` / `currentAttacker` / `triggerSteamBurstAoe` /
`applyAoe`，外加此前缺失的 `sqrtApprox` / `DamageScore` / `KillScore` /
`ReactionIndex` / `LCG.Roll` / `LookupReactionSpec`。

**跨端对齐**：5 个真实构筑 × 逐 tick 比对，全部绿。

| 构筑 | 覆盖 | 实测结果 |
|---|---|---|
| fire_aoe | 溅射 + fire/ice 配对反应 | 12 发 / 5 中 / 2 杀 / 4 反应 |
| kinetic_pierce | 高穿透 + `hits<=shots` 约束 | 21 发 / 7 中 / 2 杀 |
| lightning_chain | 弹射 + 「弹射当场结算」 | 9 发 / 5 中 / 2 杀 / 2 反应 |
| mixed_4slot | 真实玩家构筑 fire/fire/fire/ice | 17 发 / 9 中 / 2 杀 / 2 反应 |
| corrosion_steam | 腐蚀+冰 = steam_burst（唯一有 aoeRadius 的反应） | 17 发 / 8 中 / 5 反应 |

**这是 R1 第一次拿到「完整一局战斗」的两端逐事件一致。**
B/C 只能证明骨架与漏怪；D 证明了开火、命中、伤害结算、反应、
击杀、评分、热量、冷却、元素统计全部一致 ——
即 `replay_hash` 的输入在服务端已可自行计算。

**⚠️ 必须剥离 `terrain` 才敢做这个对齐**：
TS 的 `updateProjectiles` 里有一段「弹丸被掩体挡住」的判定，
而那块属于 E 块。Go 侧 D 阶段不做它 ——
半接的后果是「地形记了触发事件却从不结算效果」，
表现为「玩家看到油桶被点燃但没有任何作用」。
实测这个选择是必需的：第 1 关**带一个 `oil_drum`**，
TS 会真的对它做 `withinTerrain(t.x, t.y, 100, ...)` 判定。
只摘 `terrain`；`element_cap` 与 `max_reaction_tier` 必须保留 ——
它们是 `currentAttacker()` 的直接输入，摘掉就等于在验一个假攻方。

**⚠️ 关卡输入必须显式映射，不能 `{ ...def }`**：
夹具是 snake_case + JSON number，引擎要 camelCase + BigInt。
前两个字段错了会当场抛错（好找），第三个（`slot`）**不会** ——
它表现为「所有技能都被当成主动槽」，而对 4 槽构筑来说结果一样，
于是「槽位映射错了」这件事在 4 槽场景里完全观察不到。
另外 `slot` 必须是**列表下标**而非技能 id：内容表 id 全局递增（1..42），
直接拿 id 当 slot 的话全部技能都 >= activeSlots=4，一发都放不出来。

**两个 map 序列化的顺序陷阱**：
`elementsUsed` / `reactionsUsed` 在 TS 侧按**首次使用顺序**插入
（`JSON.stringify` 保插入序），Go 侧 `map` 按**升序**。
同一份统计在两端会序列化成不同顺序的字符串。
解法是两端都排序 —— 比对只针对「各元素各用了多少次」，
顺序信息不进哈希比对（若将来某个 bug 让顺序影响战果，
它也会体现在 score / hits / replay 上，而那些是逐条比对的）。

**变异实测 11 处，4 处首轮存活**：

首轮存活的 4 处与处理：

1. **MUT-D5（`currentAttacker` 漏加 attack 加成）** ——
   `Buffs` 是 F 块写的，F 未移植则恒为 0，于是 `+ Buffs.AttackPermille`
   永远等于「加 0」，写成 `base.Attack` 也全绿。
   补 `TestCurrentAttackerAddsBuffAttack` 等 4 条直接单测后红。
2. **MUT-D6（crit 夹取去掉）** ——
   五个场景的 `critPermille` 是 150‰，离 1000 的边界很远，夹取那段
   **一次都没执行**。而越界是可达的：6 张 crit 属性卡 = 550+480 = 1030 > 1000，
   后果是「每一发都暴击」而暴击率进 `replay_hash` → 永久失配 → 全员被误判作弊。
   补 `TestCurrentAttackerClampsCritPermille` + 边界表后红。
3. **MUT-D7（增伤被状态时长把门）** ——
   这正是 TS 源码注释里预言的洞：「当前 7 条反应里没有任何一条
   落在这个组合」。解法不是等数据出现，而是把那段逻辑抽成
   `applyReactionStatus` **独立可测的函数**，再用一个测试专用的
   反应表替换钩子（`reactionSpecLookup` 函数变量）喂进
   `ReactionSpec{AmplifyPct: 300, StatusDurationMs: 0}`。
   用函数变量而不是改 `reactionSpecs`：后者会让 `/config` 下发
   一个不存在的反应给所有客户端，也会让 `reaction_specs.json`
   的双向契约失败 —— 那是把「数据表」与「代码」的责任边界糊掉。
4. **MUT-D8（索敌忽略飞行/钻地优先）** ——
   对齐场景里没有飞行/钻地敌人当目标。补
   `TestPickTargetPrefersEvASiveForMultiTargetSkills` 后红。

补强后重测：D5/D6/D7/D8 全红。另加 D10（击退改成覆盖而非累加）也红。

**1 处「结构性存活」且不去修**：MUT-D11（去掉 `applyReactionStatus` 的
无反应守卫）在 Go 侧**观察不到差别** —— `Spec()` 对未知 key 返回零值，
所有分支自然不进。那道守卫的价值是**语义分隔**：
若将来给 `Spec()` 加副作用（日志、计数），没有它就会把无反应也记进去。
这是诚实的现状，不是遗漏。

**本轮修掉的自身问题**：

- `TestFilterAlive*` 的断言写错：`in[:0]` 是**原地压缩**，
  `backing[0]` 压缩后已经是存活那只，查它永远非 nil —— 守卫恒绿。
  改为查**最后一个槽位**（被清零的那个），并补 `TestFilterAliveKeepsOrder`。
- 两次 `Substring` 拼接把同文件的其它测试截掉了（结构体定义之后的
  全部内容）。这是本轮反复踩到的工具坑：**拼接必须保留尾部**，
  而「写到文件末尾」只对追加成立。已从 git 恢复并改用带尾部的拼接。
- `heat.cap` 是 TS 的 **getter** 不是方法，生成器里写成 `cap()` 直接抛
  `Cannot mix BigInt and other types`。

**教训（与 B/C 同源，第三次出现）**：
跨端对齐证明的是「在真实数据上两端一致」，**不覆盖「代码想处理的形状」**。
本轮三条存活变异全部属于这一类：
数据落不到分界线上时，关于分界线的断言都是空的。
处理方式有两条：补构造单测（数据可造）或把逻辑抽成可测单元
（数据造不出来时，如 MUT-D7）。
### E 块的完成度（诚实口径）

实现 `engine_terrain.go`：`updateTerrain` / `terrainOnHit` / `terrainOnKill` /
`withinTerrain` / `TerrainIndex` / `terrainCtx` 适配器 / `terrainEnemy` 适配器，
并把地形接进 D 块刻意留空的三处：弹丸掩体阻挡、`hitEnemy` 的地形联动、
击杀时的蓄能塔充能。`NewBattleEngine` 现在从 `cfg.Level.Terrain` 构造地形。

**跨端对齐**：5 个真实关卡，全绿。三类地形的状态真的变化了
（探针实测：`burning` / `collapsed`×3 / `closed`×3）。

**⚠️ 两类地形第 1 波内不可达，用构造单测补上**：
- `charge_tower`：param=38~42，需要 38+ 次击杀，而第 1 波只有 2~3 杀
- `rotor_vane`：需要弹丸擦过；探针穷举「含该地形的关卡 × 7 构筑 × 10 种子」
  共 210 个组合，**无一命中**

**⚠️ 场景组合不是拍脑袋选的**：我第一版按「一类地形配一个看起来合理的构筑」
写，结果五类里有三类完全不可达（charge_tower / rotor_vane / tidal_gate ——
闸门的开合周期是 param 毫秒 9276~10855，第一版的种子恰好让它一直开着）。
改成 `scripts/probe-terrain.ts` **穷举跑出**真实可触发的组合。
这个脚本保留在仓库里，将来加地形或调数值时可以直接复用。

**⚠️ 两个适配器是 Go 的硬约束**：
`Enemy` 已有 `HP`/`ArmorPermille`/`Dead` 三个**字段**，
`BattleEngine` 已有 `Enemies`/`Projectiles` 两个**字段**，
而 `TerrainEnemy`/`TerrainCtx` 接口要求同名**方法** ——
Go 不允许同名字段与方法共存，只能包一层。
包一层而不是改字段名：字段名与 TS 一一对应，
而跨端对齐逐字段比对依赖这些名字保持可读。

**变异实测 5 处，1 处首轮存活**：

- MUT-E1 地形编号顺序打乱 → 红（编号进 `replay_hash`）
- MUT-E2 击杀路径不去重 `terrain_used` → 红
- MUT-E3 闸门也挡钻地敌人 → 红
- MUT-E4 高度边界 `<= 60` 改成 `< 60` → 红
- MUT-E5 `ElementCap()` 写死 4 → **首轮存活**。
  原因是我那条 `TestChargeTowerRespectsElementCap` 用了 `probeTerrainCtx`，
  而探针的 `ElementCap()` 走 `p.caps`（固定值），
  **根本没调用生产的 `terrainCtx.ElementCap`**。
  这是「测试替身绕过被测代码」的典型形态 ——
  替身越方便，被测代码越容易被绕开。补 `TestTerrainCtxElementCapComesFromAttacker` 后红。

**⚠️ `terrain_used` 的两条路径语义不同，不能互相照抄**：
- **触发**路径（弹丸挡住 / 命中点燃）：每次都追加，不去重（与 TS 同）
- **击杀**路径（蓄能塔充能）：只首次记入

去重反了会让服务端 `ValidateSettle` 的 `len(terrain_used) <= len(distinct)`
判定失败 —— 症状是「玩家打了 100 次油桶，结算直接 422」。
已写成 `TestTerrainUsedDedupOnKillPath` / `TestTerrainUsedAppendsOnTriggerPath` 两条。

**本轮踩到的工具坑（第四次了）**：
在共享的 `pureVectors` 大结构体上加字段时，我用字符串拼接做替换，
连续三次把同文件里**其它块**的守卫连带截断。
最后的解法是**不碰共享结构体** ——
`terrain_battle` 段独立解码（`engine_terrain_vectors_test.go`）。
共享大结构体的代价是：任何一处改动都要重新审视整个文件。

**教训（第四次出现，与 B/C/D 同源）**：
跨端对齐证明的是「在真实数据上两端一致」，**不覆盖「数据到不了的形状」**。
本轮 E5 与 D5/D7/D8 同类。
但 E 块多了一条**元教训**：测试替身本身也会制造盲区 ——
一个绕过被测代码的替身，让被测代码的变异存活。
### F 块的完成度（诚实口径）

实现 `engine_card.go`：卡池表（6 属性 + 5 机制，顺序进 RNG）、
`rollWaveCards` / `rollSkillCards` / `rollAttributeCards` / `rollMechanicCards`、
`beginCardSelect` / `unlockedSkillCards`、`SkipCards` / `TakeCard` / `DiscardCard`、
`finishCardSelect` / `finishCardSelectNextTick` / `applyReplayDecision`、
`applyCard` / `applyAttribute` / `applyMechanic` / `applySkillCard` / `upgradeSkill`、
`cardIndex` / `attributeIndex` / `mechanicIndex`。

**跨端对齐**：6 个真实场景，全绿，且**第一次跑过多波次**
（实测第 25 关走完 7 波、第 42 关 6 次选牌、第 82 关 8 次选牌 60 杀）。
`card_charge_tower` 场景跑出 `terrain_used=[charge_tower]` ——
蓄能塔在 E 块里不可达的那条路径，靠多波次解锁了。

**⚠️ 本轮抓到两个真 bug（都不是「跨端对齐没覆盖到形状」，而是另外两类）**：

1. `triggerSteamBurstAoe` 的溅射击杀**没有**调 `terrainOnKill`。
   `kills` / `score` / 事件流 / 地形状态 / `terrain_used` **全都对得上**，
   唯一差异是蓄能塔 `charge` 少 1 —— 而 `charge` 进 `replay_hash`。
   根因：E 块只在 `hitEnemy` 一条路径接了地形，
   而 steam_burst 的溅射是**另一条独立击杀路径**。
   这是「接线点最容易漏」的第二个实例（E 块当时漏了 1/3）。

2. `markTerrainUsed` 的去重守卫**包住了 emit + record**。
   TS 那边去重守卫只包住 `terrainUsed.push`，
   `emit` 与 `record` 是**并列的三句**。
   第 42 关有两座蓄能塔（param 38 / 42），
   第二座的 `terrain|5|381|607` 事件被静默吃掉。
   E 块的 `TestTerrainUsedDedupOnKillPath` 只断言了**列表**去重，所以全绿。
   又一次「守卫只覆盖了一半语义」。

   修法：去重只作用于列表，事件与 replay **每次都发**。
   已加 `TestMarkTerrainUsedDeduplicatesListButAlwaysEmits`（分三件事断言：
   列表去重 / 事件发两次 / replay 记两条）。

**⚠️ Go 的 `range` + `splice` 陷阱**：
`finishCardSelect` 里 `for _, c := range e.Deck.Hand { e.Deck.Drop(c.ID) }`
在遍历中删除会**跳过元素**，3 张只删 2 张。
TS 那边写的是 `for (const c of [...this.deck.hand])` —— 那个展开运算符正为此存在。
抽成 `dropAllHand()` 并在注释里写明原因。

**⚠️ 生成器里的 `.Replace()` 是全局替换**：
把「card_select 不 break」一次性替换到 4 个循环上，
wave / battle / terrain 三段的轨迹全变了。
教训：改生成器循环时按**位置**改，不要按**文本**改。

**⚠️ 测试夹具本身制造过 4 处假失败**（记录在案）：

- `mustEquip` 一律 `kind="active"` → 「只装被动」用例实际发出的是主动卡
- 只装 3 个技能就断言「槽满」→ `ActiveSlots=4`，槽 3 还空着，走的是另一条分支
- `MechanicPermille=1000`（100% 放大）→ 面值 1 变 2，断言却写 1
- `beginCardSelect` **不改 phase**（那是 `checkWaveEnd` 的职责），夹具要自己置

第 2 条尤其值得记：**前置条件不成立时，断言会「意外地通过」并给出错误的安心感**。

**跨端向量必须带 `picks`**：选牌脚本就是这一局的「用户输入」，
漏带的话 Go 侧只能猜，而不同脚本给出不同战斗。

**变异实测 12 处，10 处被捕获，2 处判定为「结构等价」**：

| 变异 | 结果 |
|---|---|
| MUT-F1 手牌顺序改成 技能/机制/属性 | 红 |
| MUT-F2 `markTerrainUsed` 去重守卫包住 emit | 红 |
| MUT-F3 `dropAllHand` 去掉「先复制」 | 红 |
| MUT-F4 弃+取后用 `continue` | 红 |
| MUT-F5 属性卡编码基数 500→501 | 红 |
| MUT-F6 `recordPick` 写 `cardIndex` 而非 `handIdx` | 红 |
| MUT-F8 `free_discard` 不同步加到 `DiscardsLeft` | 红 |
| MUT-F9 找槽用 `slot < activeSlots`（历史顶槽 bug） | 红 |
| MUT-F10 `rollSkillCards` 去掉重抽 guard | 红（5s 超时） |
| MUT-F12 `recordPick` 去掉「每波一次」守卫 | 红 |
| MUT-F7 抽牌池改用 `e.Skills` 而非 `Cfg.Equipped` | **存活 —— 结构等价** |
| MUT-F11 池子提前滤掉 passive | **存活 —— 结构等价** |

MUT-F7 / F11 的存活是**可证明的等价**，不是漏洞：

- 池子来源 `Cfg.Equipped` 只可能包含**已装备**的技能，
  所以技能卡永远命中「已装备 → 升格」，
  `e.Skills` 不会新增也不会重排 —— 两者在当前设计下永远相同。
- `rollSkillCards` 内部还会再滤一次 `kind === 'active'`，
  所以池子是否预先滤掉 passive 都不改变 `actives`。

它们要变得可观测，需要先出现「池子里有未装备技能」的形状，
而那要求放开「技能卡只能给已装备技能」这条限制 ——
那是产品决策，不是移植任务。**不在此处补假守卫。**

**教训（第五次出现，但本轮多了一条新形态）**：

跨端对齐证明的是「真实数据上两端一致」，**不覆盖「代码想处理的形状」**。
本轮两个 bug 都不是「形状到不了」，而是

- **「守卫只覆盖了一半语义」** —— 列表去重 ✓ / 事件必发 ✗
- **「接线点漏了一条路径」** —— 主命中 ✓ / 溅射 ✗

所以补守卫的方向不是「加更多场景」，而是
**把一处语义拆成两个各自可观测的断言**。
场景数量已经足够（B/C/D/E/F 共 25 个），继续加的边际收益在下降。

### G 块的完成度：**全绿**

实现 `replay.go`：`BuildSnapshot` / `BuildSnapshotAttacker` / `BuildSnapshotSkill`、
`EquippedFromSnapshot` / `AttackerFromSnapshot`、`ReplayHash` / `ReplaySkillsSegment`、
`ReplayEvent.String`、`RunToEnd`、`ReplayOutcome` / `Replay` / `ReplayStats`、
`MaxTicksPerReplay`；`BattleConfig.ExtraBaseHp`。

**5 个整局用例的 `replayHash` 全部与 TS 逐位相同**（第 1/25/50/75/100 关）。

这一块全绿意味着：**服务端能拿存档里的 `build_snapshot` + `card_picks` + `seed`
自行重跑整局，并与存档里的 `replay_hash` 逐位比对。**
README 记的「A 段含局内 buff，服务端算不出」那条限制至此解除 ——
局内 buff 由 `card_picks` 决定，两者都在 `battle_records` 同一行里。

`knownReplayDivergences` 台账**已清空**，并由 `TestNoKnownReplayDivergences`
守着：它绿 = 「整局重放没有已知分歧」= 可以接线 `SettleBattle`。

#### G 块抓到的 bug：技能卡全都是哑弹

**现象**：第 25 关 tick 444 那发，TS 直接伤害 754 / Go 698、反应伤害 339 / 325。
中间量探针（两端各落 174/180 条 `resolveHit` 的完整中间量后逐行 diff）
把范围收到一条：

```
hit #13  TS: skillDamage=120  applyStacks=2
        Go: skillDamage=100  applyStacks=1
```

**根因**：`rollSkillCards` 忘了置 `hasSkillID`。
`applyCard` 的技能分支判据是 `c.Kind == CardKindSkill && c.hasSkillID`
（对应 TS 的 `card.skillId !== undefined`），而 `hasSkillID` 是**非导出**字段，
除了 JSON 反序列化只有 `rollSkillCards` 会置它。
于是**每一张技能卡都是哑弹**：卡被 `Deck.Take` 取走、`emit(card_taken)`、
`record(card)` 全都照做（所以 replay 事件流看起来完全正常），
但 `applyCard` 一个分支都没进，技能一次都没升格。

**它为什么能一路绿灯穿过 B~F 的 25 个跨端场景**，靠三个巧合：

1. `card` 的 replay 记录正常 → 事件流比对看不出问题
2. `kills` / `waves` / `terrain_used` 正常 → 只有伤害低一档
3. **B~F 的 6 个卡牌场景用的全是 `-3` 编码 =「弃一张 + 取一张」**，
   取到的是**弃牌之后**的第 0 张（属性卡）——
   **没有任何一个场景取过纯技能卡**

第 3 条是关键：`-3` 不是「取第 0 张」，是「弃一张，然后取第 0 张」。
我写场景时把它当成前者了，于是整整两个块都没碰到这条路径。

**教训（三条，都可复用）**：

- **「动作发生了」不等于「效果发生了」**。
  `TakeCard` 无论 `applyCard` 有没有效果都会 emit + record ——
  所以「replay 事件流正常」这个证据**完全不能**用来推断效果正确。
  已写成 `TestTakeCardStillRecordsWhenEffectIsSilent`（反向守卫，说明为什么查不出）。
- **非导出字段 + 布尔标记 = 哑弹温床**。
  `hasSkillID` 这类「字段是否出现」的标记在 Go 里没有 `undefined` 语义支撑，
  靠人工置位就一定会漏。要么每次构造都置（并有守卫钉住），要么干脆别用。
- **测试替身绕过被测代码**（本项目第五次，MUT-E5 / A 段 elementCap / 本次同族）。
  那批 `TestApplySkillCard*` 全都手工构造了 `Card{..., hasSkillID: true}`，
  于是验的是「`applySkillCard` 对不对」，而不是「deck 里的卡能不能走到那里」。

**新增守卫** `engine_card_dispatch_test.go` 四条，分别盯三个环节：

| 守卫 | 盯什么 |
|---|---|
| `TestRolledSkillCardsCarryTheSkillIDFlag` | 卡牌**产出**端带没带 `skill_id` |
| `TestApplyCardDispatchesOnRolledCards` | 三类卡都能被 `applyCard` 分发 |
| `TestTakingSkillCardFromDeckUpgradesSkill` | **玩家动作全路径**（主守卫） |
| `TestTakeCardStillRecordsWhenEffectIsSilent` | 反向：为什么事件流查不出 |

第三条**全程只用玩家能做的动作**（`TakeCard`），
不手工构造 `Card`、不直接调 `applyCard`、不设任何字段 ——
前两条只要有一个跟着实现一起改就会失效，而这条只依赖「发牌 → 取牌 → 效果」这条链本身。

**变异实测 4 处，全部被捕获**：

| 变异 | 结果 |
|---|---|
| MUT-G1 `rollSkillCards` 不置 `hasSkillID`（还原本次 bug） | 红 |
| MUT-G2 `applyCard` 去掉 `hasSkillID` 判据 | **存活 —— 见下** |
| MUT-G3 `upgradeSkill` 只加层数不加伤害 | 红 |
| MUT-G4 `applySkillCard` 命中已装备技能时直接 return | 红 |

**MUT-G2 存活是「暂时安全」而不是「正确」**：
去掉判据后，Go 的行为与 TS 在**当前数据**上完全一致
（内容表 id 从 1 开始，不存在 `skill_id` 缺失的技能卡）。
但判据本身对应 TS 的 `card.skillId !== undefined`，
去掉它等于放弃「畸形输入不许悄悄生效」这条防线。
所以**保留判据**，并用 MUT-G1 + 那条主守卫钉住「必须置位」。

#### 中间量探针（已留在仓库里）

定位手段：`miniapp/scripts/probe-replay-dump.ts` 导出 TS 侧的 replay 事件流，
`TestProbeReplayDump` 导出 Go 侧的同一份，逐行 diff 即可定位第一处分歧。

更细的「每发命中的完整中间量」需要临时在 `resolveHit` 里插桩
（本次用的就是这招）：导出 `d` / `directRaw` / `elementDmg` / `reactionDmg` /
`reactionElemPortion` / `reactionAttackPortion` / `stacks` / `resist*` /
`roll` / `skillDamage` / `applyStacks` / 守方 `hp`/`shield`/`armor`/
`stacks`/`resist` / 攻方 7 项系数，两端键名逐字一致，改完 `git checkout` 还原。

**插桩时的两个坑**：

- TS 侧必须在 `engine.start()` **之前**注册钩子，
  否则整局跑完钩子还没装上，导出 0 条。
- TS 的 `Map` 懒建、Go 的 map 预填 0，
  所以 `stacksDump` / `resistDump` 必须归一化成「5 个元素全列出、缺失记 0」，
  否则第 0 条就报差异，掩盖真正的分歧点。

#### 一个潜伏的 JSON tag bug

`replay_contract_test.go` 里 `replayStats.DurationMs` 的 tag 写成了 `duration_ms`，
而向量文件里的键是 **`durationMs`**（camelCase）。
解码永远是 0 —— 而**此前没有任何断言读它**，所以一直没被发现。
接线 G 块后 `TestWholeBattleReplayMatchesCrossEnd` 断言了它，当场变红。

**教训：解码失败是静默的（零值），只有断言会暴露它。**

#### 一个我自己写错又改回来的默认值

`BuildSnapshotAttacker.MechanicPermille` 我第一版写了「缺省 1000」，
推理是「缺 0 会让『机制改造』专精树整层失效」。
那个推理是错的：缺 0 只意味着「没有专精加成」，
而没有加成的玩家本来就不该有加成；填 1000 反而会让所有**老战报**
（该字段还不存在时记录的）重放出「卡面被放大 2 倍」的机制卡。
TS 是 `BigInt(a.mechanic_permille ?? 0)`，逐字对应。

**教训：「为了让某个特性不至于完全失效而填非 0 缺省」是个危险的直觉** ——
缺省值参与哈希，填错就是全量历史战报失配。

#### 一个测试断言自己踩空

`TestReplayHashAnchorUsesOnlySevenAttackerFields` 第一版写「改
`Cfg.Attacker.ElementCap` 应当改变哈希」，结果红。
原因：`currentAttacker()` 里 `elementCap` 取的是**关卡**的值
（`lv.element_cap ? lv.element_cap : base.elementCap`），
`reactionTier` 更是只读关卡 —— 攻方快照里的同名字段**是死的**。

修正：两种来源都测（关卡给了值时测关卡、没给时测攻方）。
与 E 块 MUT-E5 同源：**断言看起来在验锚点，实际验的是「这个字段有没有接线」**。
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
