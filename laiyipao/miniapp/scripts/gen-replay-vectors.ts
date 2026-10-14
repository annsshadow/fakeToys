/**
 * 生成跨端「整局重放」契约向量（R1，服务端结算重算的基础设施）。
 *
 * # 为什么需要它，以及它与既有契约向量有什么不同
 *
 * `server/testdata/formula_vectors.json` 里的向量锁的是**单个公式**：
 * 伤害、反应、护甲、LCG、关卡生成。
 *
 * R1 要锁的是**整局推演**：给定（关卡、种子、构筑、选牌），
 * 两端必须算出**逐位相同**的事件流与回放哈希。
 * 它是「服务端跑一遍引擎」这件事的全部前提 ——
 * 没有它，Go 侧的重算引擎连「对不对」都没法回答。
 *
 * # ⚠️ 期望值为什么可以由本文件（TS 侧）产出
 *
 * 项目既有的规矩是「只导数据，不导期望值，用实现生成期望值就是自证」。
 * 那条规矩对**公式**向量成立，但对**整局重放**不成立 —— 理由：
 *
 *   - 一局战斗的哈希无法人工推导。
 *   - **TS 引擎就是玩家设备上真实运行的那份代码。**
 *     「合法的一局」在定义上就是「TS 引擎算出来的那一局」。
 *     所以 TS 产出的期望值不是「用实现证明实现」，而是**定义**。
 *
 * 若这条不成立（TS 本身错了），那是玩家侧的问题，不是跨端一致性问题 ——
 * 而跨端一致性的目标正是「Go 与 TS 一致」，方向没有搞反。
 *
 * 守卫（`domain/replay_contract_test.go`）只断言 **Go 与本文件的期望值一致**。
 *
 * # 为什么不复用 `cmd/vectors`
 *
 * 那个生成器跑在 Go 侧、读 `domain` 的常量。用它产出重放期望值就是
 * 上面那条「自证」，方向正好反了。所以这个生成器跑在 TS 侧。
 *
 * 用法：
 *
 *   cd miniapp && npx tsx scripts/gen-replay-vectors.ts        （或 node --import tsx）
 *   go run ./cmd/vectors -check     # 确认 JSON 未过期
 */
import { readFileSync, writeFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

import { replay, type BuildSnapshot } from '../src/game/replay'
import type { EnemyDef, SkillDef } from '../src/game/types'
import { DEFAULT_SCORE_RULES, scoreRulesFromServer } from '../src/game/score'

const here = dirname(fileURLToPath(import.meta.url))
const serverRoot = resolve(here, '../../server')
const fixture = JSON.parse(
  readFileSync(resolve(serverRoot, 'testdata/smoke_levels.json'), 'utf8'),
)

/**
 * 要覆盖的关卡。
 *
 * 为什么是这 5 关而不是「跑全部 100 关」：
 *
 *   - 1 / 25 / 50 / 75 / 100 覆盖难度谱两端与中段；
 *   - 其中必有地形关与 BOSS 关（地形与 BOSS 是两套独立分支，
 *     漏掉任一套都会让「契约全绿但覆盖不到那条路径」）；
 *   - **向量文件要小到能被人在 review 时读完**。全 100 关的哈希表
 *     没人会看，而没人看的契约等于没有契约。
 *
 * 覆盖度由 `TestReplayContractCoversEveryBranch` 另行守住 ——
 * 那是 Go 侧的守卫，断言「这批向量确实触到了地形/BOSS/选牌/失败局」。
 */
const LEVEL_IDS = [1, 25, 50, 75, 100]

/**
 * 每个关卡用的种子。
 *
 * ⚠️ 为什么用**写死的字面量**而不是 `Math.random()`：
 * 契约向量必须是可复现的 —— 改一个种子就换一个哈希，
 * 而「为什么换」没有任何信息量。
 *
 * 跨端还要求 **int64 字符串**：JSON number 超过 2^53 后 JS 解析即失精
 * （README 记过：种子曾因此变成 `...8032`，断言悄悄通过）。
 */
const SEEDS: Record<number, string> = {
  1: '7046029255919282421',
  25: '-4611686018427387904',
  50: '1234567890123456789',
  75: '999999999999999999',
  100: '-1',
}

/**
 * 每关的选牌序列。
 *
 * ⚠️ 选牌会改变战斗（技能升格 / 属性加成 / 机制词条），
 *   **缺了它整局就不确定** —— 那是「取牌不改变哈希」这个缺陷的由来。
 *
 * 刻意混合三种形态，让契约覆盖到三条分支：
 *   - `[-1,0,1,...]`  有弃牌、有跳过（覆盖 discardTake / discardSkip / skip）
 *   - `[0,0,0,...]`    连续取牌（覆盖 applyCard 的升格与 buff 叠加）
 *   - `[]`             **空** —— 覆盖「老战报没有该字段」的退化路径
 */
function cardPicksFor(levelId: number): number[] {
  const waves = fixture.levels.find((l: any) => l.id === levelId)?.waves?.length ?? 0
  if (levelId === 1) return [] // 退化路径：无选牌脚本
  if (levelId === 25) return Array.from({ length: waves }, () => 0) // 连续取牌
  if (levelId === 50) {
    // 交替「弃一张取第 0 张 / 弃一张整波跳过 / 整波跳过」
    return Array.from({ length: waves }, (_, i) => (i % 3 === 0 ? 0 : i % 3 === 1 ? -1 : -3))
  }
  // 75 / 100：确定性伪随机但可复现（不用 Math.random）
  return Array.from({ length: waves }, (_, i) => [0, 1, 2, -1, -3, -4][(i * 7 + levelId) % 6])
}

/** 造一份确定的构筑快照。 */
function buildSnapshotFor(levelId: number): BuildSnapshot {
  // 固定 4 槽 + 不同等级：等级必须烘进 baseDamage（replay.ts 的注释），
  // 所以向量要覆盖「不同等级 → 不同哈希」这条。
  const slots = [
    { id: 1, level: 1 },
    { id: 2, level: 5 },
    { id: 3, level: 12 },
    { id: 4, level: 30 },
  ]
  const skills: BuildSnapshot['skills'] = {}
  for (const s of slots) {
    const def: any = fixture.skills.find((k: any) => k.id === s.id)
    skills[String(s.id)] = {
      id: s.id,
      name: def.name,
      family: def.family,
      element: def.element,
      kind: def.kind,
      level: s.level,
      slot: slots.indexOf(s),
    }
  }
  // 攻方系数逐关递增：覆盖「攻方每一项都进 A 段」这条不变式。
  // 缺项一律填 0 —— 与 build_snapshot 的可选字段语义一致（缺失 = 无加成）。
  const bump = levelId * 100
  return {
    skills,
    elements: slots.map((s) => (fixture.skills.find((k: any) => k.id === s.id) as any).element),
    equipment: [],
    mastery_nodes: [],
    active_slots: 4,
    attacker: {
      attack: 100 + bump,
      crit_permille: 1500,
      crit_multiplier_permille: 1500,
      reaction_mult_permille: 1000 + bump,
      element_cap: 8,
      reaction_tier: 1 + (levelId % 3),
      element_coef_permille: 1000 + bump,
      heat_cap_permille: bump % 400,
      armor_permille: bump % 750,
      mechanic_permille: bump % 600,
    },
  } as BuildSnapshot
}

const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as any[]).map((e) => [e.id, e as EnemyDef]),
)
const skills = new Map<number, SkillDef>(
  [...(fixture.skills as any[]), ...(fixture.composite_skills as any[])].map(
    (s) => [s.id, s as SkillDef],
  ),
)

/**
 * 从夹具取分数规则。
 *
 * ⚠️ 必须走 `scoreRulesFromServer`，不能把 JSON 直接塞进去。
 *
 * `ScoreRules` 内部是 **BigInt**，而 JSON 解析出来的是 `number`。
 * 第一版我直接传 `fixture.score_rules`，运行时报
 * `TypeError: Cannot mix BigInt and other types` ——
 * 而且**只在跑到第一次伤害时才炸**（`damageScore` 里除法那行），
 * 前面几十行配置全部正常通过。
 *
 * 那是个很坏的报错位置：如果向量里选的关卡一波都没开火，
 * 这个类型错误会**完全不出现**，生成器照常产出「合法」的向量文件，
 * 而文件里的分数规则是错的。
 *
 * `JSON.parse` 的结果是 `any`，所以 TypeScript **不会**替我们抓到这里。
 */
function scoreRulesFromFixture() {
  const raw = fixture.score_rules
  if (!raw) {
    // 夹具缺这一段时**必须炸**，不能静默回落到 DEFAULT ——
    // 回落后生成的是「用默认规则算的哈希」，而线上用的是服务端下发的规则，
    // 两者一旦漂移就是全体玩家被误判。夹具坏了要立刻知道。
    throw new Error(
      '夹具缺 score_rules —— 绝不能回落到 DEFAULT_SCORE_RULES：' +
        '那会产出「用默认规则算的哈希」，与线上服务端下发的规则可能不同，' +
        '一旦不同就是全体玩家被误判成作弊。',
    )
  }
  return scoreRulesFromServer(raw)
}

const cases = LEVEL_IDS.map((levelId) => {
  const level = fixture.levels.find((l: any) => l.id === levelId)
  if (!level) throw new Error(`夹具里没有第 ${levelId} 关`)
  const cardPicks = cardPicksFor(levelId)
  const out = replay(
    {
      battle_id: 0,
      level_id: levelId,
      seed: SEEDS[levelId],
      level,
      expected_hash: '',
      created_at: '',
      build: buildSnapshotFor(levelId),
      card_picks: cardPicks,
    },
    { enemies, skills, scoreRules: scoreRulesFromFixture() },
  )
  if (out.error) throw new Error(`第 ${levelId} 关重放出错：${out.error}`)
  return {
    level_id: levelId,
    seed: SEEDS[levelId],
    card_picks: cardPicks,
    build: buildSnapshotFor(levelId),
    expected_hash: out.computedHash,
    expected_stats: out.stats,
  }
})

const doc = {
  _comment:
    '跨端「整局重放」契约向量。由 miniapp/scripts/gen-replay-vectors.ts 从**真实 TS 引擎**产出；' +
    '期望值不由 Go 侧生成（那是自证）。TS 引擎即玩家设备上运行的那份代码，' +
    '所以它的输出就是「合法的一局」的定义。Go 侧 domain/replay_contract_test.go 只断言与本文件一致。',
  version: 1,
  hash_note:
    'replayHash = hex16(fnv1a64("L{id}|A{7 项攻方系数}|S{槽位技能段}|X{额外血量};事件流"))。' +
    '事件逐条为 "{tick}|{类型}|{a}|{b}|{c}"，以 ";" 连接。',
  cases,
}

const outPath = resolve(serverRoot, 'testdata/replay_vectors.json')
writeFileSync(outPath, JSON.stringify(doc, null, 2) + '\n', 'utf8')
console.log(`写入 ${outPath}（${cases.length} 个用例）`)
for (const c of cases) {
  console.log(
    `  关 ${String(c.level_id).padStart(3)}  hash=${c.expected_hash}  ` +
      `kills=${c.expected_stats.kills} leaked=${c.expected_stats.leaked} ` +
      `score=${c.expected_stats.score} dur=${c.expected_stats.durationMs}ms ` +
      `picks=${c.card_picks.length}`,
  )
}