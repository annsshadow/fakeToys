/**
 * 生成「引擎纯函数」的跨端契约向量（R1 第一批移植）。
 *
 * # 范围：只导**不需要 BattleEngine** 的那部分
 *
 * 移植顺序是刻意的。先搬纯函数，把跨端一致性一件件钉死，
 * 再动 `engine.ts` 的 tick 循环 —— 因为循环一旦有偏差，
 * 症状是**整局哈希失配**，而纯函数偏差的症状是「某个函数返回值不对」，
 * 后者能精确定位、也能单独验证。
 *
 * 本文件覆盖：
 *
 *   - `skill.ts`      clampLevel / coefPermilleAt / skillBaseDamageAtLevel /
 *                     upgradeCostFrom / isMaxLevel
 *   - `defense.ts`    applyWorks / validateSnapshot / isShieldActive /
 *                     snapshotDigest / snapshotElements / **种子派生**
 *
 * 明确**不覆盖** `runChallenge` —— 它要整个引擎，Go 侧还没有。
 *
 * # ⚠️ 期望值为什么可以由本文件（TS 侧）产出
 *
 * 与 `gen-replay-vectors.ts` 同一条理由：TS 就是玩家设备上真实运行的代码，
 * 「合法值」在定义上就是 TS 算出来的那个值。
 *
 * 对**纯函数**这条更没有争议 —— 纯函数没有「运行时状态」，
 * 用实现生成期望值不是自证，而是定义。
 *
 * # 用法
 *
 *   cd miniapp && npx tsx scripts/gen-engine-pure-vectors.ts
 *   cd ../server && go test ./internal/domain/ -run TestEnginePureContract
 */
import { readFileSync, writeFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

import {
  clampLevel,
  coefPermilleAt,
  skillBaseDamageAtLevel,
  upgradeCostFrom,
  isMaxLevel,
  DEFAULT_SKILL_RULES,
} from '../src/game/skill'
import {
  validateSnapshot,
  snapshotDigest,
  snapshotElements,
  isShieldActive,
} from '../src/game/defense'
import { hex16, fnv1a64 } from '../src/game/lcg'
import { HeatSystem, CardDeck, ACTIVE_SLOTS, PASSIVE_SLOT, TOTAL_SLOTS, HEAT_MAX, OVERHEAT_DURATION_MS, HEAT_DECAY_PER_SEC, DISCARD_PER_WAVE, DISCARD_HEAT_REFUND, type Card } from '../src/game/heatmap'
import {
  Terrain,
  FIELD_W,
  FIELD_H,
  BASE_X,
  BASE_Y,
  OIL_BURN_MS,
  ROTOR_RADIUS,
  ROTOR_DIR_1000,
  type TerrainContext,
} from '../src/game/terrain'
import type { Enemy, Projectile } from '../src/game/types'
import type { DefenseSnapshot, DefenseView } from '../src/game/defense'
import type { Attacker } from '../src/game/damage'
import { BattleEngine, TICK_MS, SPAWN_PROGRESS_FULL, SPAWN_PROGRESS_STEP } from '../src/game/engine'
import { scoreRulesFromServer } from '../src/game/score'
import type { EnemyDef, SkillDef } from '../src/game/types'

const here = dirname(fileURLToPath(import.meta.url))
const serverRoot = resolve(here, '../../server')

/**
 * 服务端夹具：100 个关卡 + 敌人/技能内容表。
 *
 * ⚠️ 它是**契约向量的输入源**，不是「随便挑的测试数据」——
 *   刷怪的 delay/interval、关卡波数、敌人属性全从这里取，
 *   两端跑的是同一份真实关卡数据。换成合成关卡就测不到
 *   「毫秒→tick 向上取整」这个最容易写错的换算。
 */
const fixture = JSON.parse(
  readFileSync(resolve(serverRoot, 'testdata/smoke_levels.json'), 'utf8'),
)

const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as any[]).map((e) => [e.id, e as EnemyDef]),
)
const skills = new Map<number, SkillDef>(
  [...(fixture.skills as any[]), ...(fixture.composite_skills as any[])].map((s) => [
    s.id,
    s as SkillDef,
  ]),
)

/**
 * 从夹具取分数规则。
 *
 * ⚠️ 必须走 `scoreRulesFromServer`，不能把 JSON 直接塞进去。
 *
 * `ScoreRules` 内部是 **BigInt**，而 JSON 解析出来的是 `number`。
 * 直接传 `fixture.score_rules`，运行时会报
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

/**
 * 契约用的技能规则取**客户端离线兜底常量**。
 *
 * ⚠️ 不是 `DEFAULT_` 之外的另立一份：`skill.ts` 的注释写着
 *   「唯一真源在 Go 侧」，TS 这份只作 config 未到达时的兜底，
 *   由 `skill.test.ts` 的 `TestSkillRulesMatchServerContract` 断言两端一致。
 *
 *   也就是说本文件产出的是「**当前在线配置**下的期望值」。
 *   若运营改了技能规则而没同步两端，先红的是那个一致性测试，
 *   而这里会产出与线上不符的向量 —— 所以那个测试必须一直在跑。
 */
const DEFAULT_RULES = DEFAULT_SKILL_RULES

// ── 攻击者基线 ────────────────────────────────────────────────
//
// 7 项全覆盖（对应 replay_hash 的 A 段），刻意从**非零**起步：
// 全 0 的攻方会让「装置把它改成什么」这件事看不出来。
const COMBAT_ATTACKER: Attacker = {
  attack: 100n,
  critPermille: 1500n,
  critMultiplierPermille: 1500n,
  reactionMultPermille: 1000n,
  elementCap: 8n,
  reactionTier: 1n,
  elementCoefPermille: 1000n,
}

// ── 技能：等级 × 底伤 ─────────────────────────────────────────
/**
 * 底伤取值分两类，因为**两端整数宽度不同**。
 *
 * TS 用 `bigint`（任意精度），Go 用 `int64`（64 位）。这不是 bug，
 * 是语言差异，但它决定了契约的边界在哪 —— 而**把边界写清楚比假装没有更重要**。
 *
 * A 类（全等级可用）：
 *   底伤 ≤ `MaxInt64 / 1450 ≈ 6.36e15`。乘上满级系数 1.45 后是 9.22e18，
 *   仍在 int64 内，所以**任何等级**两端都能逐位一致。
 *
 *   ⚠️ 这里踩过一次坑，值得写下来：我第一版用 `2^53-1 = 9.007e15`，
 *   心里想的是「JS 能精确表示的最大整数」。但那是**最终结果**的约束，
 *   不是**中间乘积**的约束 ——
 *   `9.007e15 × 1050 = 9.46e18` 已经越过 int64 上界 9.22e18 了。
 *
 *   Go 侧 `SkillBaseDamageAtLevel` 是「先乘后除」（TS 的注释专门解释过
 *   为什么不能先除后乘），所以**中间乘积**才是真正的边界。
 *   被 `TestSkillVectorsStayInsideSafeIntRange` 抓到，红得很直接。
 *
 *   改成 `2^52`（4.5e15）：`4.5e15 × 1450 = 6.53e18`，全等级安全。
 *
 * B 类（**仅等级 1**）：
 *   `2^63-1`。等级 1 的系数恰好是 1000‰，TS 走恒等快路径、
 *   **不做乘法**，所以结果就是原值 —— int64 能原样表示。
 *
 *   而等级 ≥2 时 `2^63-1 × 1.45 ≈ 1.34e19` **超出 int64**。
 *   这不是可以「修」的问题：Go 侧若要表示就得换成 `big.Int`，
 *   而伤害在整条计算链上是 `int64`（`HitInput.SkillDamage` 就是 int64）。
 *
 *   所以本文件**不为等级 ≥2 生成 B 类用例**，并在 Go 侧写明这条边界。
 *   生成「两端不可能一致」的向量等于给契约埋一颗永远红的雷；
 *   而悄悄跳过又等于藏了一个限制。正确做法是把它写成一条显式断言。
 */
const SKILL_LEVELS = [1, 2, 3, 5, 9, 10, 11, 0, -1, 99]
const SKILL_BASES_ALL_LEVELS = [0n, 1n, 33n, 100n, 999n, 123456n, 4503599627370496n] // 2^52
const SKILL_BASE_IDENTITY_ONLY = 9223372036854775807n // 2^63-1

const skillRows = SKILL_LEVELS.flatMap((level) => {
  const bases =
    level === 1
      ? [...SKILL_BASES_ALL_LEVELS, SKILL_BASE_IDENTITY_ONLY]
      : SKILL_BASES_ALL_LEVELS
  return bases.map((base) => ({
    level,
    base: base.toString(),
    clamped_level: clampLevel(DEFAULT_RULES, level),
    coef_permille: coefPermilleAt(DEFAULT_RULES, level),
    base_damage_at_level: skillBaseDamageAtLevel(DEFAULT_RULES, base, level).toString(),
    upgrade_cost: upgradeCostFrom(DEFAULT_RULES, level),
    is_max_level: isMaxLevel(DEFAULT_RULES, level),
  }))
})

// ── 防线：装置 ────────────────────────────────────────────────
/**
 * 装置列表刻意包含：
 *   - 三种已知装置的单独形态（逐条对应一条 case）
 *   - 叠加形态（覆盖多次相加）
 *   - **未知代码** `turbo_laser`（覆盖 switch 无 default 的静默忽略分支）
 *   - **空数组** 与 **undefined**（覆盖两个不同的退化路径）
 */
const WORKS_CASES: Array<{ works: string[] | undefined; label: string }> = [
  { works: [], label: 'empty' },
  { works: ['slow_belt'], label: 'slow_belt' },
  { works: ['block_wall'], label: 'block_wall' },
  { works: ['tesla_grid'], label: 'tesla_grid' },
  { works: ['slow_belt', 'block_wall', 'tesla_grid'], label: 'all_three' },
  // 同一种装置出现两次：应当**叠加两次**，不是去重也不是忽略
  { works: ['slow_belt', 'slow_belt'], label: 'duplicate_slow_belt' },
  // 顺序颠倒：加法可交换，结果必须相同
  { works: ['tesla_grid', 'block_welt' as string, 'slow_belt'], label: 'unknown_code_mixed' },
  { works: ['turbo_laser'], label: 'only_unknown' },
  { works: undefined, label: 'undefined' },
]

const workRows = WORKS_CASES.map(({ works, label }) => {
  // ⚠️ 必须走与 `applyWorks` 完全相同的代码路径。
  // applyWorks 在 defense.ts 里不是导出的，所以这里复刻它的 switch ——
  // 这就带来一个真实的弱点：**这个向量证明的是「我复刻得对」，
  // 不是「TS 的 applyWorks 是这样」。
  //
  // 缓解：Go 侧的守卫另外断言了逐条映射关系（见 engine_pure_contract_test.go
  // 的 TestApplyWorksMatchesDocumentedMapping），把「三种装置各加多少」
  // 钉成**独立于本文件**的断言。若两端映射表真的不一致，
  // 至少有一处会红。
  const a: Attacker = { ...COMBAT_ATTACKER }
  for (const code of works ?? []) {
    switch (code) {
      case 'slow_belt':
        a.elementCoefPermille += 100n
        break
      case 'block_wall':
        a.elementCap += 1n
        break
      case 'tesla_grid':
        a.reactionMultPermille += 200n
        break
    }
  }
  return {
    label,
    works: works ?? null,
    element_coef_permille: a.elementCoefPermille.toString(),
    element_cap: a.elementCap.toString(),
    reaction_mult_permille: a.reactionMultPermille.toString(),
  }
})

// ── 防线：种子派生 ────────────────────────────────────────────
/**
 * 种子是本轮最重要的契约。
 *
 * 它是防线挑战里**唯一不需要引擎就能验证**的量 ——
 * 服务端拿自己的 defense_id 与时钟就能算出该用哪个种子，
 * 从而校验客户端上报的 seed。
 *
 * 用例覆盖：普通 id、id 0、极大 id（触发 63 位掩码）、不同时刻
 * （跨分钟桶必须变化）、**同一分钟内必须不变**（否则「反复挑战得到
 * 完全相同战局」的设计意图就破了）。
 */
const SEED_BASE_MS = 1_700_000_000_000 // 固定基准，避免依赖运行时刻
const SEED_IDS = [0, 1, 7, 1000, 4_294_967, 9_007_199_254_740_993]
const SEED_OFFSETS_MS = [0, 1, 59_999, 60_000, 60_001, 3_600_000]

const seedRows = SEED_IDS.flatMap((id) =>
  SEED_OFFSETS_MS.map((off) => {
    const now = SEED_BASE_MS + off
    const minute = Math.floor(now / 60000)
    const v = (BigInt(id) * 1000003n + BigInt(minute)) & ((1n << 63n) - 1n)
    return {
      defense_id: id,
      now_ms: now,
      minute_bucket: minute,
      seed: v.toString(),
    }
  }),
)

// ── 防线：快照校验 ────────────────────────────────────────────
/**
 * 每条拒绝理由都要被覆盖到。
 *
 * ⚠️ 为什么**话术**也要进契约：玩家与运营看到的提示是排查问题的第一线索，
 *   两端说法不一致时会出现「客户端说能打、服务端说不能打」却没人知道差在哪。
 *   理由的**顺序**同理（见 ValidateSnapshot 的注释）。
 */
const NOW_MS = SEED_BASE_MS
const FULL_SNAP: DefenseSnapshot = {
  skills: [1, 2, 3],
  equipment: [],
  mastery_nodes: [],
  works: ['slow_belt'],
  elements: ['fire', 'ice'],
  mastery: [],
  rating: {},
}

function view(over: Partial<DefenseView>): DefenseView {
  return {
    id: 1,
    owner_id: 2,
    owner_name: 'x',
    name: 'x',
    power: 0,
    element_coverage: 0,
    mastery_done: 0,
    wins: 0,
    losses: 0,
    can_challenge: true,
    snapshot: FULL_SNAP,
    snapshot_hash: '0123456789abcdef',
    ...over,
  } as DefenseView
}

const SNAPSHOT_CASES: Array<{ label: string; view: DefenseView; now_ms: number }> = [
  { label: 'ok', view: view({}), now_ms: NOW_MS },
  // 无快照
  {
    label: 'no_snapshot',
    view: view({ snapshot: undefined }),
    now_ms: NOW_MS,
  },
  // 快照在但技能为空
  {
    label: 'empty_skills',
    view: view({ snapshot: { ...FULL_SNAP, skills: [] } }),
    now_ms: NOW_MS,
  },
  // 技能为空 + 无哈希：必须先报「无技能」，验证**顺序**
  {
    label: 'empty_skills_beats_missing_hash',
    view: view({ snapshot: { ...FULL_SNAP, skills: [] }, snapshot_hash: '' }),
    now_ms: NOW_MS,
  },
  // 有快照有技能但缺哈希
  { label: 'missing_hash', view: view({ snapshot_hash: '' }), now_ms: NOW_MS },
  // 护盾在未来 → 拒绝
  {
    label: 'shield_future',
    view: view({ shielded_until: new Date(NOW_MS + 3_600_000).toISOString() }),
    now_ms: NOW_MS,
  },
  // 护盾在过去 → 放行（这条曾经坏过：按「字段非空」判会误拒）
  {
    label: 'shield_past',
    view: view({ shielded_until: new Date(NOW_MS - 3_600_000).toISOString() }),
    now_ms: NOW_MS,
  },
  // 护盾恰好等于 now：TS 是 `t > nowMs`，所以**相等即过期**
  {
    label: 'shield_exactly_now',
    view: view({ shielded_until: new Date(NOW_MS).toISOString() }),
    now_ms: NOW_MS,
  },
  // 护盾字段存在但无法解析 → 视为无护盾（不永久禁止挑战）
  {
    label: 'shield_unparseable',
    view: view({ shielded_until: 'not-a-date' }),
    now_ms: NOW_MS,
  },
  // 无护盾字段
  { label: 'no_shield_field', view: view({ shielded_until: undefined }), now_ms: NOW_MS },
]

const snapshotRows = SNAPSHOT_CASES.map(({ label, view: v, now_ms }) => {
  const r = validateSnapshot(v, now_ms)
  return {
    label,
    // ⚠️ `now_ms` 必须带进文件：Go 侧要**重建输入**才能调用
    // `ValidateSnapshot`，而重建需要基准时刻。
    // 只存期望不存输入的话，Go 侧就得自己猜一个「当时几点」——
    // 猜对了这条绿，猜错了也是绿（只是测了个别的东西）。
    now_ms,
    ok: r.ok,
    reason: r.ok ? '' : r.reason,
  }
})

// ── 防线：护盾判断与元素过滤（独立于 validateSnapshot 单测）────
const SHIELD_CASES = [
  { label: 'undefined', until: undefined, now_ms: NOW_MS },
  { label: 'empty_string', until: '', now_ms: NOW_MS },
  { label: 'future', until: new Date(NOW_MS + 1).toISOString(), now_ms: NOW_MS },
  { label: 'exactly_now', until: new Date(NOW_MS).toISOString(), now_ms: NOW_MS },
  { label: 'past', until: new Date(NOW_MS - 1).toISOString(), now_ms: NOW_MS },
  { label: 'garbage', until: 'nonsense', now_ms: NOW_MS },
  { label: 'numeric_string', until: String(NOW_MS), now_ms: NOW_MS },
]

const shieldRows = SHIELD_CASES.map(({ label, until, now_ms }) => ({
  label,
  until: until ?? null,
  now_ms,
  active: isShieldActive(until, now_ms),
}))

const ELEMENT_SNAP: DefenseSnapshot = {
  skills: [1],
  equipment: [],
  mastery_nodes: [],
  works: [],
  // 混进非法值：'plasma' 不在合法集合里，必须被过滤掉
  elements: ['fire', 'plasma', 'ice', 'void', 'kinetic', 'FIRE'],
  mastery: [],
  rating: {},
}
const elementRows = [{ label: 'mixed_valid_and_invalid', elements: snapshotElements(view({ snapshot: ELEMENT_SNAP }) as never) }]

// ── 防线：摘要 ────────────────────────────────────────────────
/**
 * 摘要用例要覆盖排序：同一集合不同顺序必须给出**同一摘要**。
 * 不覆盖的话，「玩家改一次装备顺序就算换了一份构筑」这个缺陷会静默存在。
 */
const DIGEST_CASES: Array<{ label: string; works: string[]; skills: number[] }> = [
  { label: 'empty', works: [], skills: [] },
  { label: 'simple', works: ['slow_belt'], skills: [1, 2, 3] },
  { label: 'unsorted_skills', works: ['slow_belt'], skills: [3, 1, 2] },
  { label: 'sorted_skills_same_set', works: ['slow_belt'], skills: [1, 2, 3] },
  { label: 'unsorted_works', works: ['tesla_grid', 'slow_belt', 'block_wall'], skills: [1] },
  { label: 'sorted_works_same_set', works: ['block_wall', 'slow_belt', 'tesla_grid'], skills: [1] },
  { label: 'multi_digit', works: ['slow_belt'], skills: [10, 9, 100, 2] },
  { label: 'negative_skills', works: [], skills: [-1, 5, -20] },
  { label: 'dup_skills', works: [], skills: [1, 1, 2] },
]

const digestRows = DIGEST_CASES.map(({ label, works, skills }) => ({
  label,
  works,
  skills,
  digest: snapshotDigest(works, skills),
}))

// 顺带把 fnv1a64 本身也钉进契约 —— 摘要的地基
const fnvRows: Array<{ input: string; hex: string }> = [
  { input: '', hex: hex16(fnv1a64('')) },
  { input: 'a', hex: hex16(fnv1a64('a')) },
  { input: 'foobar', hex: hex16(fnv1a64('foobar')) },
  { input: '1,2,3|', hex: hex16(fnv1a64('1,2,3|')) },
  { input: '升甲水','hex': hex16(fnv1a64('升甲水')) },
]

// ── 热量系统 ─────────────────────────────────────────────────
/**
 * 热量是纯状态机：一个脚本能完整重放「若干步操作 + 若干帧推进」，
 * 每一步之后记录状态。这样期望值覆盖的是**状态迁移序列**，
 * 而不是单个函数 —— 吸收态那类缺陷正是要连续几步才暴露。
 *
 * ⚠️ 脚本刻意包含两条会互相咬住的路径：
 *   1. `cap_bonus` 很大时 cap 涨到 HEAT_MAX×N，看上限是不是当成绝对值加了
 *   2. 「热量停在 cap-1、放不出也过热不了」的**死区**，
 *      然后连跑很多帧看衰减能不能把它拉出来
 *
 * 第 2 条特别重要：把衰减写成 `(rate*dt)/1000` 时每帧衰减 0，
 * 于是这个状态**永远**出不来，而单测一个 update() 调用看不出来 ——
 * 要连跑够多帧才现形。
 */
interface HeatScript {
  label: string
  capBonus: bigint
  steps: Array<
    | { op: 'cast'; cost: bigint }
    | { op: 'refund'; n: bigint }
    | { op: 'checkOverheat' }
    | { op: 'forceOverheat' }
    | { op: 'absorbOverheat' }
    | { op: 'isStuckFor'; costs: bigint[] }
    | { op: 'update'; dtMs: number }
    | { op: 'setHeat'; v: bigint }
  >
}

const HEAT_SCRIPTS: HeatScript[] = [
  {
    label: 'plain_cast_to_cap',
    capBonus: 0n,
    steps: [
      { op: 'cast', cost: 30n },
      { op: 'cast', cost: 40n },
      { op: 'cast', cost: 30n },
      { op: 'checkOverheat' },
    ],
  },
  {
    label: 'cast_one_over_cap_rejected',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 90n },
      { op: 'cast', cost: 20n }, // 90+20 > 100 → 失败
      { op: 'cast', cost: 10n }, // 90+10 == 100 → 成功（允许正好到上限）
      { op: 'checkOverheat' },
    ],
  },
  {
    // ⚠️ 上限是**千分比**：capBonus=1000‰ 时 cap 应为 200，不是 1100。
    //   写成 `HEAT_MAX + capBonus` 就是 1100，两者差 5 倍。
    label: 'cap_bonus_is_permille',
    capBonus: 1000n,
    steps: [
      { op: 'cast', cost: 200n },
      { op: 'checkOverheat' },
    ],
  },
  {
    label: 'cap_bonus_200_permille',
    capBonus: 200n,
    steps: [{ op: 'setHeat', v: 119n }, { op: 'checkOverheat' }],
  },
  {
    label: 'negative_cap_bonus_falls_back_to_base',
    capBonus: -500n,
    steps: [{ op: 'setHeat', v: 100n }, { op: 'checkOverheat' }],
  },
  {
    label: 'refund_floors_at_zero',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 5n },
      { op: 'refund', n: 10n },
      { op: 'refund', n: 1n },
    ],
  },
  {
    label: 'overheat_blocks_cast_and_cooldown',
    capBonus: 0n,
    steps: [
      { op: 'forceOverheat' },
      { op: 'forceOverheat' }, // 已在过热 → false
      { op: 'cast', cost: 1n }, // 过热中 → 失败
      { op: 'update', dtMs: 1999 },
      { op: 'update', dtMs: 1 }, // 恰好归零 → 退出过热并清空热量
    ],
  },
  {
    // 死区：heat = 99，放不下 20 成本的技能，但 heat < cap 所以不过热。
    //
    // ⚠️ 必须跑**够久**才能看出衰减在工作。
    //   我第一版只跑 10 帧（0.5 秒 → 衰减 5 点 → heat=94），
    //   结尾的 isStuckFor 仍是 true —— 注释却写着「1 秒后应脱困」，
    //   而 94+20=114 仍放不下。注释与实际不符，属于自己骗自己。
    //
    //   真正脱困需要 heat ≤ cap - cost = 80，即至少衰减 19 点 ≈ 1.9 秒。
    //   下面跑 4 秒（80 帧 × 50ms → 衰减 40 点 → heat=59），
    //   59+20=79 ≤ 100，确实脱困。
    //
    //   而若把衰减写成 `(rate*dt)/1000`，每帧衰减 0，
    //   跑 80 帧后 heat 仍是 99 —— 这条向量会立刻分出高下。
    label: 'deadzone_recovered_by_decay',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 99n },
      { op: 'isStuckFor', costs: [20n] },
      { op: 'checkOverheat' }, // false（99 < 100）
      ...Array.from({ length: 80 }, () => ({ op: 'update' as const, dtMs: 50 })),
      { op: 'isStuckFor', costs: [20n] }, // 4 秒后应脱困
      { op: 'cast', cost: 20n }, // 且真的放得下
    ],
  },
  {
    // ⚠️ dtMs 是**小数**的情形。
    //   引擎用 16.666ms 之类的步长，`Math.round(16.666) = 17`。
    //   Go 侧少这一步取整，衰减总量在长局里会与 TS 差一点。
    label: 'fractional_dt_uses_round',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 100n },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
      { op: 'update', dtMs: 16.666 },
    ],
  },
  {
    label: 'absorb_overheat_clears_heat',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 50n },
      { op: 'forceOverheat' },
      { op: 'absorbOverheat' },
      { op: 'cast', cost: 10n },
      { op: 'update', dtMs: 100 },
    ],
  },
  {
    label: 'is_stuck_empty_costs_false',
    capBonus: 0n,
    steps: [{ op: 'setHeat', v: 99n }, { op: 'isStuckFor', costs: [] }],
  },
  {
    label: 'is_stuck_free_skill_not_stuck',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 99n },
      { op: 'isStuckFor', costs: [20n, 0n] },
      { op: 'isStuckFor', costs: [20n, 20n] },
    ],
  },
  {
    label: 'overheat_ends_clears_heat',
    capBonus: 0n,
    steps: [
      { op: 'setHeat', v: 80n },
      { op: 'forceOverheat' },
      { op: 'update', dtMs: 1000 },
      { op: 'update', dtMs: 1000 },
      { op: 'update', dtMs: 10 },
    ],
  },
]

const heatRows = HEAT_SCRIPTS.map((s) => {
  const h = new HeatSystem()
  h.capBonus = s.capBonus
  const trace: Array<Record<string, unknown>> = []
  const snap = (op: string, extra: Record<string, unknown> = {}) => {
    trace.push({
      op,
      heat: h.heat.toString(),
      overheated: h.overheated,
      overheat_remaining: h.overheatRemaining,
      cap: h.cap.toString(),
      max_heat: h.maxHeatThisBattle.toString(),
      // ⚠️ `decayRemainder` 虽是 private，但**必须**进向量。
      //
      //   只比 heat 的话，四舍五入与否在短脚本里看不出差别：
      //   dt=16.666 时 round→17、截断→16，每步差 10/1000 点，
      //   要累计 100 步才落到一个整数点上。
      //   我第一版没记它，变异验证把 `math.Round` 去掉后**测试没红** ——
      //   守卫证明不了任何东西。
      //
      //   记下来之后单步就能分辨：第 1 步 remainder 就是 170 vs 160。
      decay_remainder: (h as unknown as { decayRemainder: bigint }).decayRemainder.toString(),
      ...extra,
    })
  }
  for (const step of s.steps) {
    switch (step.op) {
      case 'cast': {
        const ok = h.tryCast(step.cost)
        snap('cast', { cost: step.cost.toString(), result: ok })
        break
      }
      case 'refund':
        h.refundHeat(step.n)
        snap('refund', { n: step.n.toString() })
        break
      case 'checkOverheat':
        snap('checkOverheat', { result: h.checkOverheat() })
        break
      case 'forceOverheat':
        snap('forceOverheat', { result: h.forceOverheat() })
        break
      case 'absorbOverheat':
        h.absorbOverheat()
        snap('absorbOverheat')
        break
      case 'isStuckFor':
        snap('isStuckFor', {
          costs: step.costs.map((c) => c.toString()),
          result: h.isStuckFor(step.costs),
        })
        break
      case 'update':
        h.update(step.dtMs)
        snap('update', { dt_ms: step.dtMs })
        break
      case 'setHeat':
        // 直接写 heat 只为构造起点，不走任何 API —— 刻意如此：
        // 若为了「纯粹」而用 cast 累加出来，某些起点就构造不出来。
        h.heat = step.v
        snap('setHeat', { v: step.v.toString() })
        break
    }
  }
  return { label: s.label, cap_bonus: s.capBonus.toString(), trace }
})

// ── 手牌 ─────────────────────────────────────────────────────
/**
 * 弃牌脚本。要覆盖：
 *   - 正常弃（成功 + 返还）
 *   - 弃不存在的牌（失败，**且不消耗次数**）
 *   - 次数用完（失败）
 *   - drop 不消耗次数（跳过语义）
 *   - 取牌后手牌顺序（.splice 的语义）
 */
interface DeckScript {
  label: string
  hand: string[]
  ops: Array<
    | { op: 'discard'; id: string }
    | { op: 'drop'; id: string }
    | { op: 'newWave' }
  >
}

const DECK_SCRIPTS: DeckScript[] = [
  {
    label: 'discard_then_exhausted',
    hand: ['a', 'b', 'c'],
    ops: [
      { op: 'discard', id: 'b' },
      { op: 'discard', id: 'a' },
    ],
  },
  {
    // ⚠️ 弃不存在的牌**不能**消耗次数，否则玩家会在手牌没变少时耗尽机会。
    label: 'discard_missing_keeps_charge',
    hand: ['a', 'b'],
    ops: [
      { op: 'discard', id: 'zzz' },
      { op: 'discard', id: 'a' },
    ],
  },
  {
    label: 'new_wave_resets',
    hand: ['a', 'b'],
    ops: [
      { op: 'discard', id: 'a' },
      { op: 'newWave' },
      { op: 'discard', id: 'b' },
    ],
  },
  {
    // drop 与 discard 的区别就在「不消耗次数」
    label: 'drop_does_not_consume',
    hand: ['a', 'b', 'c'],
    ops: [
      { op: 'drop', id: 'a' },
      { op: 'drop', id: 'b' },
      { op: 'discard', id: 'c' },
    ],
  },
  {
    label: 'drop_missing',
    hand: ['a'],
    ops: [{ op: 'drop', id: 'nope' }],
  },
  {
    label: 'discard_all_then_hand_empty',
    hand: ['x', 'y'],
    ops: [
      { op: 'discard', id: 'x' },
      { op: 'discard', id: 'y' },
      { op: 'discard', id: 'x' },
    ],
  },
]

const deckRows = DECK_SCRIPTS.map((s) => {
  const d = new CardDeck()
  d.setHand(s.hand.map((id) => ({ id, kind: 'skill', rarity: 'common', name: id, descr: '' }) as Card))
  const trace: Array<Record<string, unknown>> = []
  const snap = (op: string, extra: Record<string, unknown> = {}) => {
    trace.push({
      op,
      hand: d.hand.map((c) => c.id),
      size: d.size,
      discards_left: d.discardsLeft,
      ...extra,
    })
  }
  snap('setHand')
  for (const step of s.ops) {
    if (step.op === 'discard') {
      const r = d.discard(step.id)
      snap('discard', { id: step.id, ok: r.ok, refund: r.refund.toString() })
    } else if (step.op === 'drop') {
      const ok = d.drop(step.id)
      snap('drop', { id: step.id, ok })
    } else {
      d.newWave()
      snap('newWave')
    }
  }
  return { label: s.label, trace }
})

// 顺带把卡牌系统的常量也钉进契约 —— 它们进结算上限校验
const heatConsts = {
  ACTIVE_SLOTS,
  PASSIVE_SLOT,
  TOTAL_SLOTS,
  HEAT_MAX: HEAT_MAX.toString(),
  OVERHEAT_DURATION_MS,
  HEAT_DECAY_PER_SEC: HEAT_DECAY_PER_SEC.toString(),
  DISCARD_PER_WAVE,
  DISCARD_HEAT_REFUND: DISCARD_HEAT_REFUND.toString(),
}

// ── 地形 ─────────────────────────────────────────────────────
/**
 * 地形契约用**「记录型 ctx」**而不是真实引擎。
 *
 * # 为什么可以这样
 *
 * 地形与引擎的耦合面只有 6 个方法（enemies / projectiles / terrainTick /
 * elementCap / within / enemiesInRadius / onKill）。把它们换成
 * 「记录调用并返回预设敌人」的假实现之后，契约断言的仍然是
 * **真实的东西**：
 *
 *   - 地形自己的状态迁移（burning→idle、open↔closed、charge 阈值）
 *   - 地形向引擎**要什么**（半径 90？半径 260？elementCap 传了多少？）
 *   - 地形对敌人**做什么**（扣多少血、叠几层、是否触发 onKill）
 *
 * 这些都不是「我复刻得对」，而是 TS 的真实行为。
 *
 * 真正的引擎实体（Enemy/Projectile）不在契约范围内 ——
 * 那些要等引擎移植后单独验。现在假装能验，就是自己骗自己。
 *
 * ⚠️ 假实现必须**足够真**：敌人要有真实 hp / armorPermille，
 *   否则 applyArmor 的效果测不出来（全都 armor=0 时减伤恒为 1 倍）。
 */
interface MockEnemy {
  id: string
  hp: bigint
  armorPermille: bigint
  x: number
  y: number
  dead: boolean
  hitFlashMs: number
  elements: Array<{ element: string; layers: bigint }>
}

interface MockProjectile {
  id: string
  x: bigint
  y: bigint
  vx: bigint
  vy: bigint
}

class RecordingCtx implements TerrainContext {
  calls: string[] = []
  constructor(
    public enemies: MockEnemy[],
    public projectiles: MockProjectile[],
    public terrainTick: bigint,
    public elementCap: bigint,
  ) {}

  within(terrainX: number, terrainY: number, r: number, x: bigint, y: bigint): boolean {
    const dx = x - BigInt(Math.round(terrainX * 1000))
    const dy = y - BigInt(Math.round(terrainY * 1000))
    const rr = BigInt(Math.round(r * 1000))
    return dx * dx + dy * dy <= rr * rr
  }

  enemiesInRadius(x: number, y: number, r: number): any[] {
    this.calls.push(`enemiesInRadius(${x},${y},${r})`)
    const cx = Math.round(x * 1000)
    const cy = Math.round(y * 1000)
    const rr = r * 1000
    return this.enemies.filter((e) => {
      const dx = e.x * 1000 - cx
      const dy = e.y * 1000 - cy
      return dx * dx + dy * dy <= rr * rr
    }) as any[]
  }

  onKill(e: any): void {
    this.calls.push(`onKill(${e.id})`)
  }

  onTerrainTrigger(kind: string): void {
    // 记录下来是为了**验证它不会被调用** ——
    // TS 里它是死接口，Go 侧根本没有它。
    this.calls.push(`onTerrainTrigger(${kind})`)
  }

  /** 快照当前敌人状态，供契约比对。 */
  enemyState() {
    return this.enemies.map((e) => ({
      id: e.id,
      hp: e.hp.toString(),
      armor_permille: e.armorPermille.toString(),
      dead: e.dead,
      hit_flash_ms: e.hitFlashMs,
      elements: e.elements.map((x) => `${x.element}:${x.layers.toString()}`).join('|'),
    }))
  }

  projectileState() {
    return this.projectiles.map((p) => `${p.id}:${p.vx.toString()},${p.vy.toString()}`)
  }
}

function mkEnemy(id: string, hp: bigint, armor: bigint, x: number, y: number): MockEnemy {
  return {
    id,
    hp,
    armorPermille: armor,
    x,
    y,
    dead: false,
    hitFlashMs: 0,
    elements: [],
    applyElement(element: string, layers: bigint, cap: bigint) {
      const cur = this.elements.find((e) => e.element === element)
      const now = (cur?.layers ?? 0n) + layers
      if (cur) cur.layers = now > cap ? cap : now
      else this.elements.push({ element, layers: now > cap ? cap : now })
    },
  } as unknown as MockEnemy
}

function mkProjectile(id: string, x: number, y: number, vx = 0n, vy = 0n): MockProjectile {
  return {
    id,
    x: BigInt(Math.round(x * 1000)),
    y: BigInt(Math.round(y * 1000)),
    vx,
    vy,
    addVelocity(dx: bigint, dy: bigint) {
      this.vx += dx
      this.vy += dy
    },
  } as unknown as MockProjectile
}

interface TerrainStep {
  op: 'update' | 'onHit' | 'onKillCharging'
  dtMs?: number
  element?: string
  damage?: string
  hitX?: number
  hitY?: number
  cap?: string
}

interface TerrainScript {
  label: string
  placement: { kind: string; x: number; y: number; param: number }
  enemies: Array<{ id: string; hp: string; armor: string; x: number; y: number }>
  projectiles?: Array<{ id: string; x: number; y: number; vx?: string; vy?: string }>
  terrainTick: string
  elementCap: string
  steps: TerrainStep[]
}

const TERRAIN_SCRIPTS: TerrainScript[] = [
  {
    // 油桶：焰元素近处命中达阈值 → 引燃
    label: 'oil_drum_ignites_on_fire',
    placement: { kind: 'oil_drum', x: 500, y: 500, param: 3 },
    enemies: [
      { id: 'e1', hp: '1000', armor: '200', x: 500, y: 500 },
      { id: 'far', hp: '1000', armor: '0', x: 900, y: 900 },
    ],
    terrainTick: '100',
    elementCap: '5',
    steps: [
      { op: 'onHit', element: 'fire', damage: '150', hitX: 500000, hitY: 500000 },
      { op: 'onHit', element: 'fire', damage: '150', hitX: 500000, hitY: 500000 },
      { op: 'onHit', element: 'fire', damage: '150', hitX: 500000, hitY: 500000 },
    ],
  },
  {
    // 非焰元素不得引燃；远处命中也不得
    label: 'oil_drum_rejects_non_fire_and_far',
    placement: { kind: 'oil_drum', x: 500, y: 500, param: 1 },
    enemies: [{ id: 'e1', hp: '1000', armor: '0', x: 500, y: 500 }],
    terrainTick: '100',
    elementCap: '5',
    steps: [
      { op: 'onHit', element: 'ice', damage: '9999', hitX: 500000, hitY: 500000 },
      { op: 'onHit', element: 'fire', damage: '9999', hitX: 800000, hitY: 500000 },
    ],
  },
  {
    // 引燃后火区伤害：护甲必须生效、元素必须叠层、死亡必须回调
    label: 'oil_drum_fire_zone_damage',
    placement: { kind: 'oil_drum', x: 500, y: 500, param: 1 },
    enemies: [
      { id: 'tank', hp: '1000', armor: '500', x: 500, y: 500 },
      { id: 'soft', hp: '1000', armor: '0', x: 505, y: 505 },
      { id: 'doomed', hp: '50', armor: '0', x: 502, y: 502 },
      { id: 'away', hp: '1000', armor: '0', x: 900, y: 900 },
    ],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'onHit', element: 'fire', damage: '200', hitX: 500000, hitY: 500000 },
      { op: 'update', dtMs: 100 },
      { op: 'update', dtMs: 100 },
    ],
  },
  {
    // 火区到时熄灭
    label: 'oil_drum_burn_expires',
    placement: { kind: 'oil_drum', x: 500, y: 500, param: 1 },
    enemies: [{ id: 'e1', hp: '100000', armor: '0', x: 500, y: 500 }],
    terrainTick: '1',
    elementCap: '3',
    steps: [
      { op: 'onHit', element: 'fire', damage: '200', hitX: 500000, hitY: 500000 },
      { op: 'update', dtMs: OIL_BURN_MS },
      { op: 'update', dtMs: 100 },
    ],
  },
  {
    // 潮汐闸：周期开合
    label: 'tidal_gate_cycles',
    placement: { kind: 'tidal_gate', x: 300, y: 700, param: 1000 },
    enemies: [],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'update', dtMs: 500 },
      { op: 'update', dtMs: 500 },
      { op: 'update', dtMs: 1000 },
      { op: 'update', dtMs: 2500 },
    ],
  },
  {
    // period 为 0 或负 → 必须夹到 1，否则 while 循环死转
    label: 'tidal_gate_period_clamped',
    placement: { kind: 'tidal_gate', x: 300, y: 700, param: 0 },
    enemies: [],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'update', dtMs: 1 },
      { op: 'update', dtMs: 2 },
      { op: 'update', dtMs: 3 },
    ],
  },
  {
    // 旋转风障：角度与偏转量
    label: 'rotor_vane_deflects',
    placement: { kind: 'rotor_vane', x: 500, y: 500, param: 100 },
    enemies: [],
    projectiles: [
      { id: 'inside', x: 500, y: 500 },
      { id: 'edge', x: 700, y: 500 },
      { id: 'outside', x: 900, y: 500 },
    ],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'update', dtMs: 50 },
      { op: 'update', dtMs: 50 },
      { op: 'update', dtMs: 3600 },
    ],
  },
  {
    // 崩塌掩体：动能 ×3、其他元素 ×1
    label: 'collapse_wall_kinetic_triple',
    placement: { kind: 'collapse_wall', x: 400, y: 600, param: 10 },
    enemies: [],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'onHit', element: 'ice', damage: '500', hitX: 400000, hitY: 600000 },
      { op: 'onHit', element: 'ice', damage: '500', hitX: 400000, hitY: 600000 },
      { op: 'onHit', element: 'ice', damage: '500', hitX: 400000, hitY: 600000 },
      { op: 'onHit', element: 'kinetic', damage: '500', hitX: 400000, hitY: 600000 },
    ],
  },
  {
    // 崩塌后不再阻挡，且 onHit 直接 false
    label: 'collapse_wall_after_collapse',
    placement: { kind: 'collapse_wall', x: 400, y: 600, param: 3 },
    enemies: [],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'update', dtMs: 50 },
      { op: 'onHit', element: 'kinetic', damage: '900', hitX: 400000, hitY: 600000 },
      { op: 'update', dtMs: 50 },
      { op: 'onHit', element: 'kinetic', damage: '900', hitX: 400000, hitY: 600000 },
    ],
  },
  {
    // 远处命中不得给掩体充能
    label: 'collapse_wall_ignores_far_hits',
    placement: { kind: 'collapse_wall', x: 400, y: 600, param: 1 },
    enemies: [],
    terrainTick: '100',
    elementCap: '3',
    steps: [{ op: 'onHit', element: 'kinetic', damage: '999999', hitX: 900000, hitY: 900000 }],
  },
  {
    // 蓄能塔：击杀累积到 param 才触发
    label: 'charge_tower_accumulates_kills',
    placement: { kind: 'charge_tower', x: 250, y: 750, param: 3 },
    enemies: [{ id: 'e1', hp: '1000', armor: '0', x: 250, y: 750 }],
    terrainTick: '100',
    elementCap: '4',
    steps: [
      { op: 'onKillCharging', element: 'fire', cap: '4' },
      { op: 'onKillCharging', element: 'fire', cap: '4' },
      { op: 'onKillCharging', element: 'fire', cap: '4' },
      { op: 'onKillCharging', element: 'fire', cap: '4' },
    ],
  },
  {
    // 非 charge_tower 收到 onKillCharging 必须无副作用
    label: 'on_kill_charging_ignores_other_kinds',
    placement: { kind: 'oil_drum', x: 500, y: 500, param: 1 },
    enemies: [{ id: 'e1', hp: '1000', armor: '0', x: 500, y: 500 }],
    terrainTick: '100',
    elementCap: '4',
    steps: [{ op: 'onKillCharging', element: 'fire', cap: '4' }],
  },
  {
    // ⚠️ 未知地形类型：必须退化成「什么都不做」而不是崩
    label: 'unknown_kind_is_noop',
    placement: { kind: 'lava_pool', x: 500, y: 500, param: 5 },
    enemies: [{ id: 'e1', hp: '1000', armor: '0', x: 500, y: 500 }],
    projectiles: [{ id: 'p1', x: 500, y: 500 }],
    terrainTick: '100',
    elementCap: '3',
    steps: [
      { op: 'update', dtMs: 50 },
      { op: 'onHit', element: 'fire', damage: '9999', hitX: 500000, hitY: 500000 },
    ],
  },
]

const terrainRows = TERRAIN_SCRIPTS.map((s) => {
  const t = new Terrain({
    kind: s.placement.kind as never,
    x: s.placement.x,
    y: s.placement.y,
    param: s.placement.param,
  } as never)
  const enemies = s.enemies.map((e) =>
    mkEnemy(e.id, BigInt(e.hp), BigInt(e.armor), e.x, e.y),
  )
  const projectiles = (s.projectiles ?? []).map((p) =>
    mkProjectile(p.id, p.x, p.y, BigInt(p.vx ?? '0'), BigInt(p.vy ?? '0')),
  )
  const ctx = new RecordingCtx(enemies, projectiles, BigInt(s.terrainTick), BigInt(s.elementCap))

  const trace: Array<Record<string, unknown>> = []
  const snap = (op: string, extra: Record<string, unknown> = {}) => {
    trace.push({
      op,
      state: t.state,
      timer: t.timer,
      charge: t.charge,
      angle: t.angle,
      triggered: t.triggered,
      serialize: t.serialize(),
      enemies: ctx.enemyState(),
      projectiles: ctx.projectileState(),
      calls: [...ctx.calls],
      ...extra,
    })
  }

  snap('init')
  for (const step of s.steps) {
    if (step.op === 'update') {
      const eff = t.update(step.dtMs ?? 0, ctx)
      snap('update', { dt_ms: step.dtMs ?? 0, blocked: eff.blocked })
    } else if (step.op === 'onHit') {
      const r = t.onHit(step.element as never, BigInt(step.damage ?? '0'), BigInt(step.hitX ?? 0), BigInt(step.hitY ?? 0))
      snap('onHit', {
        element: step.element,
        damage: step.damage,
        hit_x: step.hitX,
        hit_y: step.hitY,
        result: r,
      })
    } else {
      const r = t.onKillCharging(step.element as never, BigInt(step.cap ?? '0'), ctx)
      snap('onKillCharging', { element: step.element, cap: step.cap, result: r })
    }
  }
  return {
    label: s.label,
    placement: s.placement,
    // ⚠️ 输入**必须**带进文件：Go 侧要重建同样的场景才能跑脚本。
    //   只存 trace 的话，Go 只能猜一个场景 —— 而「猜错了但两边都没报错」
    //   正是契约最坏的失效形态（heat 那批已经踩过一次 now_ms 的坑）。
    //
    //   弹丸要**归一化**：脚本里 vx/vy 是可选的，直接回显会让缺省项
    //   在 JSON 里消失，Go 侧拿到空串 —— 而空串解析成 0 恰好是对的，
    //   于是「缺省」与「显式 0」变得不可区分。
    //   显式补 "0" 让两者在文件里长得一样，这是有意的。
    enemies: s.enemies,
    projectiles: (s.projectiles ?? []).map((p) => ({
      id: p.id,
      x: p.x,
      y: p.y,
      vx: p.vx ?? '0',
      vy: p.vy ?? '0',
    })),
    terrain_tick: s.terrainTick,
    element_cap: s.elementCap,
    trace,
  }
})

/**
 * 方向表的抽样下标。
 *
 * 覆盖 0°/90°/180°/270° 四个正交方向及其 +1°（抓角度错位）、
 * 四个对角、以及末项。
 *
 * ⚠️ 全量 720 个数塞进 JSON 没人会 review ——
 *   而没人看的契约等于没有契约。抽样 + Go 侧的「全表模长」性质守卫
 *   组合起来才是完整的：抽样抓「错在哪」，性质守卫抓「有没有错」。
 */
const ROTOR_SAMPLE_IDX = [
  0, 1, 90, 91, 180, 181, 270, 271, 45, 46, 135, 136, 225, 226, 315, 316, 359, 718, 719,
]

/**
 * Block B / C 的**跨端逐事件对齐**场景。
 *
 * # 这是 R1 移植过程中第一次真正的跨端对齐
 *
 * A 块只能验排序不变量（`BeginWave` 还是空实现，事件流必然不一致）。
 * B 块把它换成真实实现之后，「同一关 + 同一种子 + 同一空构筑」下，
 * 两端的**每一个 tick** 的 `tick` / `phase` / 刷怪队列 / 每个敌人的全部字段 /
 * 全部重放事件 / 全部渲染事件 都必须逐条相同。
 *
 * ⚠️ **刻意用真关卡而非合成关卡**：刷怪组的 delay/interval 是 levelgen
 * 真实产出的毫秒值（第 1 关第一波 delay=1012ms）。用拍出来的数字就测不到
 * 「毫秒→tick 向上取整」这个最容易写错的换算。
 *
 * # 为什么逐步比对而不是只比最终状态
 *
 * 只比最终值的话，中间若干 tick 错了又绕回来的实现能全绿。
 * 而刷怪队列的长度是「非增」的，中间的错误很容易被后续追平。
 *
 * # ⚠️⚠️ 为什么长场景要采样而不是逐步记
 *
 * 敌人从 x=960 走到 x=60（防线）要 900 秒 = **18000 tick**，
 * 所以「让漏怪分支真的进一次」必然是万级 tick 的场景。
 *
 * 逐步全量记会让文件爆到 42 MB（我第一版就是这么写的）：
 *   - `replay` 是**累积**数组，每 tick 存全量 = O(n²)
 *   - 每 tick 存全量敌人列表 = 18000 × 4 只 × 200 字节 ≈ 14 MB
 *
 * 两处都改成增量：
 *   - `replay` 只存**本步新增**的（游标式）
 *   - 快照按 `sampleEvery` 抽样，并**对有事件的步强制采样**
 *
 * 抽样的判据必须两端**同样可算**，否则 Go 不知道该在哪几步快照。
 * 这里用 `tick % sampleEvery === 0 || 本步有事件` ——
 * 两端都能各自算出「这一步要不要记」，不依赖任何单侧信息。
 */
interface WaveScenario {
  label: string
  levelID: number
  seed: string
  /** 跑多少个 tick */
  ticks: number
  /**
   * 每隔多少 tick 记一次快照。1 = 逐步全记。
   *
   * ⚠️ 1 只用在短场景上：长场景用 1 会产出 42 MB 的文件，
   * 而没人会去 review 42 MB —— 没人看的契约等于没有契约。
   */
  sampleEvery: number
  /**
   * 攻方的护甲千分比。undefined = 全零攻方。
   *
   * ⚠️ 需要单独的场景来盖「攻方护甲」这条路径：
   *   `defenseArmorPermille()` 是三项相加，而前两项
   *   （关卡 armor_permille 与 buffs）在这批场景里都恒为 0。
   *   不给一个非零值的话，`+ Cfg.Attacker.ArmorPermille` 那一项
   *   就永远是「加 0」—— 把它写成减号也照样全绿。
   */
  armorPermille?: number
}

const WAVE_SCENARIOS: WaveScenario[] = [
  // ── 短场景：逐步全记（B 块验刷怪，C 块验位移与出场进度）──
  { label: 'lvl1_first_wave', levelID: 1, seed: '7046029255919282421', ticks: 120, sampleEvery: 1 },
  // 第二个种子：RNG 路径不同 → 不同的洗牌结果与 y 坐标。
  // 只用一个种子的话，「洗牌算法恰好对」与「洗牌算法错但那个种子下恰好相同」
  // 区分不开。
  { label: 'lvl1_alt_seed', levelID: 1, seed: '1234567890123456789', ticks: 120, sampleEvery: 1 },
  // 负种子：INIT_XOR 与 asUintN 的对齐在这里最容易被写错。
  { label: 'lvl1_negative_seed', levelID: 1, seed: '-1', ticks: 90, sampleEvery: 1 },
  { label: 'lvl1_zero_seed', levelID: 1, seed: '0', ticks: 90, sampleEvery: 1 },
  // 第 25 关：波数更多、刷怪组更多 → 覆盖「一 tick 内刷多只」的路径。
  { label: 'lvl25_multi_group', levelID: 25, seed: '-4611686018427387904', ticks: 260, sampleEvery: 1 },

  // ── 长场景：抽样记（跑到漏怪分支真的进为止）──
  // 全零攻方：盖「关卡护甲 + 卡牌护甲」两项为 0 时的裸漏怪伤害。
  { label: 'lvl1_reach_base', levelID: 1, seed: '7046029255919282421', ticks: 22000, sampleEvery: 100 },
  // 攻方护甲 400‰：盖 `defenseArmorPermille` 的第三项。
  { label: 'lvl1_armor400', levelID: 1, seed: '7046029255919282421', ticks: 22000, sampleEvery: 100, armorPermille: 400 },
  // 攻方护甲 750‰（封顶）：盖 `applyArmor` 的夹取边界。
  { label: 'lvl1_armor750', levelID: 1, seed: '7046029255919282421', ticks: 22000, sampleEvery: 100, armorPermille: 750 },
  // 第 25 关：波次内敌人**有攻击力** -> 盖 `attack > 0` 那条漏怪路径。
  //   （第 1 关第一波全是零攻击力杂兵，只盖 `breachDamage` 那档。）
  { label: 'lvl25_full', levelID: 25, seed: '-4611686018427387904', ticks: 22000, sampleEvery: 100 },
]

/**
 * 场景的「攻方」配置。
 *
 * ⚠️ **刻意取全零**而不是一个像样的玩家。
 *
 *   有构筑时开火（D 块）与卡牌（F 块）会介入，
 *   于是对齐结果里混进「这发打中了吗」的噪声。
 *   空构筑保证「没有任何东西伤害敌人」——
 *   于是敌人只会被「抵达防线」这一条路径处理掉，
 *   C 块要验的行为就是唯一行为。
 *
 * `heatCapPermille` 必须显式给 0：它是 BigInt，
 * 不给就是 `undefined`，而构造器里 `undefined` 赋给数值字段会变 NaN。
 */
const ZERO_ATTACKER = {
  attack: 0n,
  critPermille: 0n,
  critMultiplierPermille: 0n,
  reactionMultPermille: 1000n,
  elementCap: 3n,
  reactionTier: 1n,
  elementCoefPermille: 1000n,
  heatCapPermille: 0n,
  armorPermille: 0n,
  mechanicPermille: 0n,
} as never

const waveRows = WAVE_SCENARIOS.map((s) => {
  const level = fixture.levels.find((l: any) => l.id === s.levelID)
  if (!level) throw new Error(`夹具里没有第 ${s.levelID} 关`)
  const enemyDefs = new Map<number, any>((fixture.enemies as any[]).map((e) => [e.id, e]))
  const skillDefs = new Map<number, any>(
    [...(fixture.skills as any[]), ...(fixture.composite_skills as any[])].map((x) => [
      x.id,
      x,
    ]),
  )

  const engine = new BattleEngine({
    level: level as never,
    enemies: enemyDefs,
    skills: skillDefs,
    equipped: [],
    attacker: {
      ...ZERO_ATTACKER,
      armorPermille: BigInt(s.armorPermille ?? 0),
    } as never,
    seed: BigInt(s.seed),
    scoreRules: scoreRulesFromFixture(),
  } as never)

  engine.start()

  const trace: Array<Record<string, unknown>> = []
  /** replay 游标：只把「新增的」写进 trace（见上面的 O(n²) 说明）。 */
  let replayCursor = 0

  const snap = () => {
    const replayAll = (engine as any).replay as any[]
    const fresh = replayAll.slice(replayCursor).map((r: any) => `${r.t}|${r.type}|${r.a}|${r.b}|${r.c}`)
    replayCursor = replayAll.length
    trace.push({
      tick: engine.tick,
      phase: engine.phase,
      elapsed_ms: engine.elapsedMs,
      spawn_queue: (engine as any).spawnQueue.map((q: any) => ({
        enemy_id: q.enemyId,
        at_tick: q.atTick,
        uid: q.uid,
      })),
      enemies: engine.enemies.map((e) => ({
        uid: e.uid,
        def_id: e.defId,
        x: e.x.toString(),
        y: e.y.toString(),
        hp: e.hp.toString(),
        shield: e.shield.toString(),
        armor_permille: e.armorPermille.toString(),
        spawn_progress: e.spawnProgress,
        dead: e.dead,
        resist: (['fire', 'ice', 'lightning', 'corrosion', 'kinetic'] as const)
          .map((el) => `${el}:${(e.resist.get(el) ?? 0n).toString()}`)
          .join('|'),
        // C 块新增：远程怪的攻击冷却。
        // ⚠️ 必须记：漏怪伤害取决于「冷却何时归零」，
        //   而冷却只在这一步递减 —— 不记就抓不到「冷却递减差一 tick」这类错误。
        attack_cooldown: e.attackCooldown.toString(),
        attack_range: e.attackRange.toString(),
      })),
      replay: fresh,
      events: engine.drainEvents().map((ev: any) => `${ev.type}:${ev.damage ?? ''}`),
      // C 块：防线状态。⚠️ 逐步记录 base_hp 是必需的 ——
      // `leaked` 只在最后比对的话，「同一 tick 漏两只」与
      // 「两个 tick 各漏一只」会得到同样的终值，而那是两种完全不同的战斗。
      base_hp: (engine as any).baseHp.toString(),
      leaked: (engine as any).leaked,
    })
  }

  snap()
  for (let i = 0; i < s.ticks; i++) {
    // ⚠️ `card_select` 是**对齐的终点**，不是可以硬跑过去的空闲状态。
    //
    //   要进入第 2 波必须解掉卡牌选择，而那要过 `rollWaveCards`
    //   （它会消耗 RNG，于是后续所有波次的刷怪洗牌都会变）
    //   与 `skipCards` / `finishCardSelect` —— 三者都属于 F 块。
    //
    //   所以这里停在 `card_select`，而不是硬跑满 22000 tick。
    //   我第一版让它继续跑，结果 224 个采样点里 216 个是
    //   `phase: card_select` 的死空间 —— 向量大 4 倍，却什么也没多验。
    //
    //   第 2 波及以后的覆盖由 Go 侧的直接单测承担
    //   （`TestBeginWaveAtLaterIndex`）。
    //
    // ⚠️ 条件写成「**既不是 wave 也不是 prepare**」而不是
    //   「是 won 或 lost」：前者能自动涵盖将来新增的任何终局状态，
    //   后者需要每加一个终局就改一次 —— 而漏改的后果是
    //   「在终局状态下继续 step，跑到一个不存在的状态上去」。
    // ⚠️ 这里**不能**在 card_select 停下：
    //   TS 的 pplyReplayDecision 是在 step() 内部被调的
    //   （phase === 'card_select' && replayScript !== null），
    //   所以解卡牌**需要一次 step**，而 break 会正好跳过它。
    //   结果就是 card_picks 永远是空 —— 选牌脚本根本没被执行。
    //
    //   B~E 的场景之所以可以「非 wave 即停」，是因为它们没有选牌脚本。
    if (engine.phase !== 'wave' && engine.phase !== 'prepare') break
    const before = (engine as any).replay.length
    engine.step()
    const replayGrew = (engine as any).replay.length > before
    // ⚠️ 「本步是否记快照」的判据必须**两端各自可算**。
    //   这里用 `tick % sampleEvery === 0 || 本步新增了 replay 事件`：
    //   `tick` 两端都有，replay 长度变化两端也都能数出来。
    if (engine.tick % s.sampleEvery === 0 || replayGrew) snap()
  }
  // ⚠️ 收尾必须再 snap 一次：最后一步可能既不是 sampleEvery 的倍数、
  //   也没产生 replay 事件（只产生了渲染事件，比如 float 过期）。
  //   而 Go 侧靠 `phase` 判定终局后**不再 step**，
  //   若这里不补，Go 侧那一步就没有对应的期望值可比。
  if (trace[trace.length - 1].tick !== engine.tick) snap()

  return {
    label: s.label,
    level_id: s.levelID,
    seed: s.seed,
    ticks: s.ticks,
    sample_every: s.sampleEvery,
    // 攻方护甲（输入回显）：用 0 表示全零攻方。
    // ⚠️ 必须写成数字而不是透传 undefined ——
    //   `JSON.stringify` 会**丢掉**值为 undefined 的键，
    //   Go 侧就分不清「没给」与「给了 undefined」，而两者应当等价。
    attacker_armor_permille: s.armorPermille ?? 0,
    // 关卡刷怪组 + 血量 + 护甲（输入回显）：Go 侧要重建同样的场景。
    // ⚠️ base_hp 与 armor_permille **必须**带过来：
    //   它们是漏怪伤害的分母与护甲基线，而 `buildWaveEngine` 只手工拼 Waves ——
    //   不带的话 Go 侧拿到 base_hp=0，于是 `breachDamage` 走 100 的兜底分支，
    //   整条漏怪伤害都算错而**看不出**为什么。
    level_waves: level.waves,
    level_base_hp: level.base_hp,
    level_armor_permille: level.armor_permille ?? 0,
    trace,
  }
})
/**
 * Block D（开火与弹道）的场景：在 B/C 的基础上**带上构筑**。
 *
 * # 为什么这一块是里程碑
 *
 * B/C 的场景是**空构筑** —— 敌人只会被「抵达防线」处理掉。
 * D 第一次让引擎真的开火、命中、结算伤害、触发反应、击杀，
 * 于是**整局 `replay_hash`** 第一次可以被服务端自行计算。
 *
 * # ⚠️ 必须剥离 `terrain`
 *
 * TS 的 `updateProjectiles` 里有一段「弹丸被掩体挡住」的判定，
 * 而那块地形逻辑属于 E 块。Go 侧 D 阶段**不做**它 ——
 * 半接的后果是「地形记了触发事件却从不结算效果」，
 * 表现为「玩家看到油桶被点燃但没有任何作用」。
 *
 * 所以场景里把 `level.terrain` 清空：这不是「造了个假关卡」，
 * 而是把 E 的输入从这个场景里摘出去。
 * ⚠️ 只摘 terrain；`element_cap` 与 `max_reaction_tier` 必须保留 ——
 *   它们是 `currentAttacker()` 的直接输入，摘掉就等于在验一个假攻方。
 *
 * 实测这个选择是必需的：第 1 关**带一个 `oil_drum`**，
 * 而 TS 会真的对它做 `withinTerrain(t.x, t.y, 100, ...)` 判定。
 *
 * # 构筑怎么选
 *
 * 每个构筑覆盖一条**独立的代码路径**：
 *
 * | 构筑 | 覆盖 |
 * |---|---|
 * | fire_aoe      | 溅射（aoe>0） + fire/ice 配对反应 |
 * | kinetic_pierce | 高穿透（pierce=8） + `hits<=shots` 约束 |
 * | lightning_chain | 弹射（chain=4） + 「弹射当场结算」 |
 * | mixed_4slot   | 真实玩家构筑：热量上限 + 冷却 + 多元素 |
 *
 * 全部用**真实技能 id**（取自 `fixture.skills`，与线上同一张表）。
 */

interface BuildScenario {
  label: string
  levelID: number
  seed: string
  ticks: number
  sampleEvery: number
  /** 技能 id 列表，槽位 = 下标 */
  skillIDs: number[]
  /** 攻方（全部非零，以便 currentAttacker 的每一项都被读到） */
  attacker: {
    attack: bigint
    critPermille: bigint
    critMultiplierPermille: bigint
    reactionMultPermille: bigint
    elementCap: bigint
    reactionTier: bigint
    elementCoefPermille: bigint
    heatCapPermille: bigint
    armorPermille: bigint
    mechanicPermille: bigint
  }
}

/**
 * 攻方基线。
 *
 * ⚠️ 每项都**非零**：`currentAttacker()` 是「相加 + 夹取」的结构，
 *   全零会让「加 0」与「漏加」观察不到差别。
 *   而每一项都参与重放哈希 —— 漏加一项就是哈希不符。
 *
 * `critPermille: 150`（15%）而不是 1000：
 * 夹取那条路径（`min(crit, 1000)`）需要一个**不会触发夹取**的值，
 *   否则「加法」与「夹取」两段里只有一段被验到。
 *   夹取那条另有一个专测（`TestCurrentAttackerClampsCritPermille`）。
 */
const BUILD_ATTACKER: BuildScenario['attacker'] = {
  attack: 100n,
  critPermille: 150n,
  critMultiplierPermille: 1500n,
  reactionMultPermille: 1200n,
  elementCap: 3n,
  reactionTier: 1n,
  elementCoefPermille: 1000n,
  heatCapPermille: 0n,
  armorPermille: 200n,
  mechanicPermille: 1000n,
}

/**
 * 把夹具里的技能定义映射成引擎要的 `EquippedSkill`。
 *
 * # ⚠️⚠️ 这不是「顺手转一下字段名」，是一个**必须显式做**的映射
 *
 * 夹具（`smoke_levels.json`，与线上同一份内容表）是 **snake_case + JSON number**：
 *
 *   base_damage, heat_cost, cooldown_ms, aoe_radius, apply_element, ...
 *
 * 而 `EquippedSkill` 是 **camelCase + BigInt**：
 *
 *   baseDamage, heatCost, cooldownMs, aoeRadius, applyElement, ...
 *
 * 直接 `{ ...def }` 的话：
 *   - `heatCost` 是 undefined -> `isStuckFor` 里 `heat + undefined` **抛错**
 *   - `baseDamage` 是 undefined -> 伤害恒 0
 *   - `slot` 是 undefined -> `s.slot >= activeSlots` 恒 false（全部当主动）
 *
 * 前两个会当场抛错（好找），第三个**不会** ——
 * 它表现为「所有技能都被当成主动槽」，而对 4 槽构筑来说结果一样，
 * 于是「槽位映射错了」这件事在 4 槽场景里完全观察不到。
 * 到 5 槽或 2 槽构筑时才会暴露，而那时已经没人记得这个映射了。
 *
 * 所以：逐字段显式列出，不做 `...def` 展开。
 * 展开的写法在「多一个字段」时是便利，在「少一个字段」时是静默 undefined。
 */
function toEquippedSkill(def: any, slot: number) {
  return {
    skillId: def.id as number,
    name: def.name as string,
    element: def.element as never,
    kind: def.kind as never,
    heatCost: BigInt(def.heat_cost),
    cooldownMs: def.cooldown_ms as number,
    pierce: def.pierce as number,
    aoeRadius: def.aoe_radius as number,
    baseDamage: BigInt(def.base_damage),
    applyElement: (def.apply_element ?? '') as never,
    applyStacks: BigInt(def.apply_stacks ?? 0),
    projectileSpeed: def.projectile_speed as number,
    chain: def.chain as number,
    // ⚠️ slot = **列表下标**，不是技能 id。
    //   内容表 id 是全局递增的（1..42），直接拿 id 当 slot 的话
    //   全部技能都 >= activeSlots = 4，一发都放不出来。
    slot,
    cooldownRemaining: 0,
  }
}

/**
 * 按键排序后序列化计数 map。
 *
 * # ⚠️ 必须两端都排序，不能靠「序列化结果碰巧一致」
 *
 * `JSON.stringify` 按**插入序**输出，而 `engine.elementsUsed` 是
 * 「首次使用时插入」—— 也就是「玩家实际开火的元素顺序」。
 * Go 侧的 `map[string]int64` 按**升序**输出（Go 1.12+ 的 map 迭代随机，
 * `encoding/json` 会显式排序）。
 *
 * 于是同一份统计在两端会序列化成不同顺序的字符串：
 *
 *   TS: {"fire":2,"lightning":8}     （先打 fire 后打 lightning）
 *   Go: {"lightning":8,"fire":2}     （升序）
 *
 * 值完全相同，只是顺序不同 —— 而顺序恰好是「这个玩家先用了哪个元素」
 * 的信息，**它本身不该进哈希比对**。
 * 所以两边都排序，让比对只针对「各元素各用了多少次」。
 *
 * ⚠️ 代价：顺序信息在这个守卫里被丢掉了。
 *   若将来某个 bug 让「元素使用顺序」影响战果，它抓不到。
 *   但那种 bug 也会体现在 score / hits / replay 上 ——
 *   而那些是**逐条**比对的。
 */
function sortedJSON(m: Record<string, number>): string {
  const keys = Object.keys(m).sort()
  return '{' + keys.map((k) => JSON.stringify(k) + ':' + String(m[k])).join(',') + '}'
}
const BUILD_SCENARIOS: BuildScenario[] = [
  {
    label: 'fire_aoe',
    levelID: 25,
    seed: '-4611686018427387904',
    ticks: 4000,
    sampleEvery: 25,
    // 2=napalm(aoe140) + 5=hail(aoe160, ice) + 6=glacier(pierce3) + 1=ember(aoe60)
    skillIDs: [2, 5, 6, 1],
    attacker: BUILD_ATTACKER,
  },
  {
    label: 'kinetic_pierce',
    levelID: 25,
    seed: '7046029255919282421',
    ticks: 4000,
    sampleEvery: 25,
    // 13=laser(pierce6) + 14=ray(pierce8) + 12=siege(pierce4) + 11=shrapnel(aoe200)
    skillIDs: [13, 14, 12, 11],
    attacker: BUILD_ATTACKER,
  },
  {
    label: 'lightning_chain',
    levelID: 25,
    seed: '1234567890123456789',
    ticks: 4000,
    sampleEvery: 25,
    // 9=stormgrid(chain4) + 7=arc(chain2) + 8=thunderlance(pierce2,chain1)
    skillIDs: [9, 7, 8, 3],
    attacker: BUILD_ATTACKER,
  },
  {
    label: 'mixed_4slot',
    levelID: 25,
    seed: '-1',
    ticks: 4000,
    sampleEvery: 25,
    // 真实玩家构筑：fire/fire/fire/ice —— 与引擎注释里那个例子一致
    skillIDs: [1, 2, 3, 5],
    attacker: BUILD_ATTACKER,
  },
  {
    label: 'corrosion_steam',
    levelID: 25,
    seed: '0',
    ticks: 4000,
    sampleEvery: 25,
    // 腐蚀 + 冰 = steam_burst（唯一有 aoeRadius 的反应）
    skillIDs: [16, 17, 4, 18],
    attacker: BUILD_ATTACKER,
  },
]

/** 战斗场景与 B/C 的波次场景共用一份快照函数。 */
function makeTracer(engine: any, sampleEvery: number) {
  const trace: Array<Record<string, unknown>> = []
  let replayCursor = 0
  const snap = () => {
    const replayAll = engine.replay as any[]
    const fresh = replayAll.slice(replayCursor).map((r: any) => `${r.t}|${r.type}|${r.a}|${r.b}|${r.c}`)
    replayCursor = replayAll.length
    trace.push({
      tick: engine.tick,
      phase: engine.phase,
      elapsed_ms: engine.elapsedMs,
      projectiles: (engine.projectiles as any[]).map((p: any) => ({
        uid: p.uid,
        skill_id: p.skillId,
        x: p.x.toString(),
        y: p.y.toString(),
        vx: p.vx.toString(),
        vy: p.vy.toString(),
        pierce_left: p.pierceLeft,
        chain_left: p.chainLeft,
        hit_set: [...p.hitSet].sort((a: any, b: any) => Number(a) - Number(b)).join(','),
        aoe_radius: p.aoeRadius,
        dead: p.dead,
      })),
      enemies: (engine.enemies as any[]).map((e: any) => ({
        uid: e.uid,
        def_id: e.defId,
        x: e.x.toString(),
        y: e.y.toString(),
        hp: e.hp.toString(),
        shield: e.shield.toString(),
        spawn_progress: e.spawnProgress,
        dead: e.dead,
        attack_cooldown: e.attackCooldown.toString(),
        // 元素层数：跨元素反应的全部状态都挂在这里
        stacks: (['fire', 'ice', 'lightning', 'corrosion', 'kinetic'] as const)
          .map((el) => `${el}:${(e.stacks.get(el) ?? 0n).toString()}`)
          .join('|'),
        frozen_ms: e.frozenMs,
        stunned_ms: e.stunnedMs,
        slowed_ms: e.slowedMs,
        armor_shred_ms: e.armorShredMs,
        armor_shred_permille: e.armorShredPermille.toString(),
        knockback: e.knockback.toString(),
        amplify_permille: e.amplifyPermille.toString(),
        hit_flash_ms: e.hitFlashMs,
      })),
      replay: fresh,
      events: engine.drainEvents().map((ev: any) => `${ev.type}:${ev.damage ?? ''}`),
      base_hp: engine.baseHp.toString(),
      leaked: engine.leaked,
      kills: engine.kills,
      shots: engine.shots,
      hits: engine.hits,
      reactions: engine.reactionsCount,
      score: engine.score.toString(),
      heat: engine.heat.heat.toString(),
      heat_cap: engine.heat.cap.toString(),
      overheated: engine.heat.overheated,
      cooldowns: (engine.skills as any[]).map((s: any) => s.cooldownRemaining.toString()),
      elements_used: sortedJSON(engine.elementsUsed),
      reactions_used: sortedJSON(engine.reactionsUsed),
      // --- E 块：地形 ---
      // 必须逐字段记地形的**全部状态**，不只是「触发没触发」。
      // 油桶的火区计时、蓄能塔的充能、旋转风障的角度都跨 tick 演进 ---
      // 只记布尔量的话，「状态演进不一致」完全观察不到，
      // 而它的症状是「火区早结束一帧」这类极细的偏差。
      terrains: (engine.terrains as any[]).map((t: any) => ({
        kind: t.kind,
        x: t.x,
        y: t.y,
        state: t.state,
        timer: t.timer.toString(),
        charge: t.charge,
        angle: t.angle,
        triggered: t.triggered,
      })),
      terrain_used: (engine as any).terrainUsed.slice().join(String.fromCharCode(44)),
terrain_used: (engine as any).terrainUsed.slice().join(String.fromCharCode(44)),
      // ── F 块：卡牌 ──
      // ⚠️ 必须逐步记 `cardPicks`：它是「每波选了什么」的**完整序列**，
      // 而服务端重算需要它来复现局内 buff。
      // 只记最终值的话，「第 2 波与第 3 波选反了」观察不到。
      card_picks: (engine as any).cardPicks.join(String.fromCharCode(44)),
      // 手牌内容：验证 rollWaveCards 的产出（顺序、种类、id）
      hand: (engine.deck.hand as any[]).map((c: any) => `${c.kind}:${c.id}`),
      discards_left: (engine.deck as any).discardsLeft,
      // ⚠️ 技能状态：升格会改 baseDamage / applyStacks / pierce，
      // 而这些直接进 `currentAttacker` 与弹丸参数。
      skills: (engine.skills as any[]).map((s: any) =>
        `${s.skillId}@${s.slot}:${s.baseDamage}:${s.applyStacks}:${s.pierce}:${s.cooldownRemaining}`,
      ),
      // 局内加成：applyAttribute / applyMechanic 的直接产物。
      // ⚠️ `heat.capBonus` 必须单列 —— 它不是 Buffs 的一部分，
      //   被 `Heat.cap` 单独读走。
      buffs: JSON.stringify({
        attack: (engine as any).buffs.attackPermille.toString(),
        element_coef: (engine as any).buffs.elementCoefPermille.toString(),
        crit: (engine as any).buffs.critPermille.toString(),
        element_cap: (engine as any).buffs.elementCapBonus.toString(),
        armor: (engine as any).buffs.armorPermille.toString(),
        pierce: (engine as any).buffs.pierceBonus,
        chain: (engine as any).buffs.chainBonus,
        aoe: (engine as any).buffs.aoeBonus,
        free_discard: (engine as any).buffs.freeDiscard,
        overheat_guard: (engine as any).buffs.overheatGuard,
        heat_cap_bonus: (engine as any).heat.capBonus.toString(),
      }),
    })
  }
  return { trace, snap }
}

const buildRows = BUILD_SCENARIOS.map((s) => {
  const raw = fixture.levels.find((l: any) => l.id === s.levelID)
  if (!raw) throw new Error(`夹具里没有第 ${s.levelID} 关`)
  // ⚠️ 摘掉 terrain（E 块的范围），其余原样保留。
  const level = { ...raw, terrain: [] }

  const enemyDefs = new Map<number, any>((fixture.enemies as any[]).map((e) => [e.id, e]))
  const skillDefs = new Map<number, any>(
    [...(fixture.skills as any[]), ...(fixture.composite_skills as any[])].map((x) => [x.id, x]),
  )

  const equippedFixed = s.skillIDs.map((id, i) => {
    const def = skillDefs.get(id)
    if (!def) throw new Error(`技能 id ${id} 不在夹具里`)
    return toEquippedSkill(def, i)
  })

  const engine = new BattleEngine({
    level: level as never,
    enemies: enemyDefs,
    skills: skillDefs,
    equipped: equippedFixed as never,
    attacker: s.attacker as never,
    seed: BigInt(s.seed),
    scoreRules: scoreRulesFromFixture(),
    activeSlots: 4,
  } as never)

  engine.start()
  const { trace, snap } = makeTracer(engine, s.sampleEvery)
  snap()
  for (let i = 0; i < s.ticks; i++) {
    // ⚠️ 这里**不能**在 card_select 停下：
    //   TS 的 pplyReplayDecision 是在 step() 内部被调的
    //   （phase === 'card_select' && replayScript !== null），
    //   所以解卡牌**需要一次 step**，而 break 会正好跳过它。
    //   结果就是 card_picks 永远是空 —— 选牌脚本根本没被执行。
    //
    //   B~E 的场景之所以可以「非 wave 即停」，是因为它们没有选牌脚本。
    if (engine.phase !== 'wave' && engine.phase !== 'prepare') break
    const before = (engine as any).replay.length
    engine.step()
    if (engine.tick % s.sampleEvery === 0 || (engine as any).replay.length > before) snap()
  }
  if (trace[trace.length - 1].tick !== engine.tick) snap()

  return {
    label: s.label,
    level_id: s.levelID,
    seed: s.seed,
    ticks: s.ticks,
    sample_every: s.sampleEvery,
    level_waves: level.waves,
    level_base_hp: level.base_hp,
    level_armor_permille: level.armor_permille ?? 0,
    level_element_cap: level.element_cap ?? 0,
    level_max_reaction_tier: level.max_reaction_tier ?? 1,
    // ⚠️ 构筑**必须带进文件**：Go 侧要重建同样的技能与槽位。
    //   只存 skillIDs 不够 —— Go 侧还要从内容表取属性，
    //   而「取哪张表」本身就是个可能出错的地方。
    equipped: s.skillIDs,
    attacker: {
      attack: s.attacker.attack.toString(),
      crit_permille: s.attacker.critPermille.toString(),
      crit_multiplier_permille: s.attacker.critMultiplierPermille.toString(),
      reaction_mult_permille: s.attacker.reactionMultPermille.toString(),
      element_cap: s.attacker.elementCap.toString(),
      reaction_tier: s.attacker.reactionTier.toString(),
      element_coef_permille: s.attacker.elementCoefPermille.toString(),
      heat_cap_permille: s.attacker.heatCapPermille.toString(),
      armor_permille: s.attacker.armorPermille.toString(),
      mechanic_permille: s.attacker.mechanicPermille.toString(),
    },
    trace,
  }
})
/**
 * Block E（地形接线）的场景：**带地形**的完整战斗。
 *
 * # 为什么现在才做
 *
 * D 块的五个场景刻意剥离了 `terrain` —— 那时 Go 侧还没接地形，
 * 半接的后果是「地形记了触发事件却从不结算效果」，
 * 症状落在数值上（分数、击杀、漏怪）而根因在接线，极难定位。
 *
 * 现在 E 块接上了，这里把债还掉：**每一类地形都要有一个场景**。
 *
 * # 地形类型与选关
 *
 * | 类型 | 关卡 | 覆盖 |
 * |---|---|---|
 * | `oil_drum` | 1 | 被点燃 -> 火区持续伤害 |
 * | `collapse_wall` | 22 | **阻挡弹丸**（掩体，必须用动能砸开） |
 * | `charge_tower` | 42 | 击杀充能 -> 全场施加主元素 |
 * | `tidal_gate` | 61 | 关闭时阻挡低层敌人（`blocked` 分支） |
 * | `rotor_vane` | 82 | 周期性改变弹丸方向 |
 *
 * 五类各一个，是因为它们走**完全不同的代码路径**：
 *
 *   - `onHit`（油桶/掩体/风障）：弹丸或命中触发
 *   - `blocksProjectile`（掩体）：弹丸被吃掉
 *   - `blocked`（潮汐闸）：敌人被推回
 *   - `onKillCharging`（蓄能塔）：击杀驱动
 *   - `update`（全部）：每 tick 推进
 *
 * 只验其中一类的话，其余四类的接线错误全部观察不到 ——
 * 而「接线」正是最容易漏的地方（少一个调用点，不报任何错）。
 *
 * ⚠️ 选关用**真实关卡 id**，地形参数（位置/半径/血量）全取自
 *   `smoke_levels.json` —— 与线上同一份数据。
 *   用拍出来的地形参数就测不到「坐标是否对得上」这类错误。
 */

interface TerrainScenario {
  label: string
  levelID: number
  seed: string
  ticks: number
  sampleEvery: number
  skillIDs: number[]
  attacker: BuildScenario['attacker']
}

/**
 * 五类地形各一个场景。
 *
 * ⚠️ 组合不是拍脑袋选的，而是 `scripts/probe-terrain.ts` **跑出来**的：
 * 穷举「含该地形的关卡 × 7 种构筑 × 10 个种子」，
 * 只保留「地形状态真的发生过变化」的组合。
 *
 * 为什么必须这么做：我第一版按「一类地形配一个看起来合理的构筑」写，
 * 结果五类里有**三类**在第 1 波内完全不可达：
 *
 *   - `charge_tower`：param=38~42，即需要 38+ 次击杀才触发，
 *     而第 1 波只有 2~3 杀
 *   - `rotor_vane`：要弹丸经过它才转，第一版的弹道根本擦不到
 *   - `tidal_gate`：闸门的开合周期是 param 毫秒（9276~10855），
 *     第一版的种子恰好让闸门一直开着
 *
 * 这三类要真正覆盖，需要**多波次战斗**（进第 2 波要累计击杀），
 * 而那要 F 块解锁。当前它们只验到「`update()` 每 tick 被调用、
 * 且不产生错误状态」—— 这是如实的边界，不是覆盖。
 * 它们各自的转移逻辑由 Go 侧的构造单测覆盖。
 */
const TERRAIN_SCENARIOS: TerrainScenario[] = [
  // 第 1 关 + fire 构筑 + seed 1：油桶被点燃，进入 burning（探针实测）
  { label: 'terr_oil_drum', levelID: 1, seed: '1', ticks: 4000, sampleEvery: 25, skillIDs: [1, 2, 3], attacker: BUILD_ATTACKER },
  // 第 22 关 + weak1（单技能 ember）+ seed 7：三面掩体全部崩塌（探针实测）
  { label: 'terr_collapse_wall', levelID: 22, seed: '7', ticks: 4000, sampleEvery: 25, skillIDs: [1], attacker: BUILD_ATTACKER },
  // 第 61 关 + weak1 + seed 7：三道闸门全部关闭（探针实测）
  { label: 'terr_tidal_gate', levelID: 61, seed: '7', ticks: 4000, sampleEvery: 25, skillIDs: [1], attacker: BUILD_ATTACKER },
  // 蓄能塔 / 旋转风障：第 1 波内不可达，见上方注释
  { label: 'terr_charge_tower', levelID: 41, seed: '1', ticks: 4000, sampleEvery: 25, skillIDs: [1], attacker: BUILD_ATTACKER },
  { label: 'terr_rotor_vane', levelID: 81, seed: '1', ticks: 4000, sampleEvery: 25, skillIDs: [1], attacker: BUILD_ATTACKER },
]

const wireTerrainRows = TERRAIN_SCENARIOS.map((s) => {
  const level = fixture.levels.find((l: any) => l.id === s.levelID)
  if (!level) throw new Error(`夹具里没有第 ${s.levelID} 关`)
  if (!(level.terrain || []).length) throw new Error(`第 ${s.levelID} 关没有地形 —— 场景选错了关`)

  const enemyDefs = new Map<number, any>((fixture.enemies as any[]).map((e) => [e.id, e]))
  const skillDefs = new Map<number, any>(
    [...(fixture.skills as any[]), ...(fixture.composite_skills as any[])].map((x) => [x.id, x]),
  )
  const equipped = s.skillIDs.map((id, i) => {
    const def = skillDefs.get(id)
    if (!def) throw new Error(`技能 id ${id} 不在夹具里`)
    return toEquippedSkill(def, i)
  })

  const engine = new BattleEngine({
    level: level as never, // ⚠️ **不剥 terrain**
    enemies: enemyDefs,
    skills: skillDefs,
    equipped: equipped as never,
    attacker: s.attacker as never,
    seed: BigInt(s.seed),
    scoreRules: scoreRulesFromFixture(),
    activeSlots: 4,
  } as never)

  engine.start()
  const { trace, snap } = makeTracer(engine, s.sampleEvery)
  snap()
  for (let i = 0; i < s.ticks; i++) {
    // ⚠️ 这里**不能**在 card_select 停下：
    //   TS 的 pplyReplayDecision 是在 step() 内部被调的
    //   （phase === 'card_select' && replayScript !== null），
    //   所以解卡牌**需要一次 step**，而 break 会正好跳过它。
    //   结果就是 card_picks 永远是空 —— 选牌脚本根本没被执行。
    //
    //   B~E 的场景之所以可以「非 wave 即停」，是因为它们没有选牌脚本。
    if (engine.phase !== 'wave' && engine.phase !== 'prepare') break
    const before = (engine as any).replay.length
    engine.step()
    if (engine.tick % s.sampleEvery === 0 || (engine as any).replay.length > before) snap()
  }
  if (trace[trace.length - 1].tick !== engine.tick) snap()

  return {
    label: s.label,
    level_id: s.levelID,
    seed: s.seed,
    ticks: s.ticks,
    sample_every: s.sampleEvery,
    level_waves: level.waves,
    level_base_hp: level.base_hp,
    level_armor_permille: level.armor_permille ?? 0,
    level_element_cap: level.element_cap ?? 0,
    level_max_reaction_tier: level.max_reaction_tier ?? 1,
    // ⚠️ 地形**必须**带进文件：Go 侧要重建同样的地形实例。
    //   只存地形类型就够 —— 位置/参数由关卡数据决定，
    //   而关卡数据两侧读的是同一份 `smoke_levels.json`。
    terrain_kinds: (level.terrain as any[]).map((t: any) => t.kind),
    equipped: s.skillIDs,
    attacker: {
      attack: s.attacker.attack.toString(),
      crit_permille: s.attacker.critPermille.toString(),
      crit_multiplier_permille: s.attacker.critMultiplierPermille.toString(),
      reaction_mult_permille: s.attacker.reactionMultPermille.toString(),
      element_cap: s.attacker.elementCap.toString(),
      reaction_tier: s.attacker.reactionTier.toString(),
      element_coef_permille: s.attacker.elementCoefPermille.toString(),
      heat_cap_permille: s.attacker.heatCapPermille.toString(),
      armor_permille: s.attacker.armorPermille.toString(),
      mechanic_permille: s.attacker.mechanicPermille.toString(),
    },
    trace,
  }
})
/**
 * Block F（卡牌）的场景：**带选牌脚本的多波次战斗**。
 *
 * # 这是 R1 第一次能跑过多波次
 *
 * B~E 的所有场景都停在第 1 波末尾的 `card_select` ——
 * 硬跑满 tick 只会得到一片死空间。F 落地后，
 * 场景能真正走完第 2、3、… 波。
 *
 * 而多波次是 `charge_tower`（需要 38+ 次击杀）与
 * `rotor_vane`（需要更长的弹道里程）这两类地形**唯一的**可达路径 ——
 * 它们在 E 块里只能靠构造单测覆盖，正是因为第 1 波打不满。
 *
 * # 选牌脚本必须**显式**给，不能让引擎自己决定
 *
 * `setReplayScript` 注入的序列是重放闭环的最后一环：
 * 选牌会改变后续战斗（技能升格 / 属性加成 / 机制词条），
 * 第三方拿不到这个序列就复现不出原局。
 *
 * ⚠️ 编码约定（两端必须一致，第 159 轮重写）：
 *   0/1/2             → 取当次手牌第 idx 张（不弃牌）
 *   -1                → 整波跳过
 *   -(3 + code)       → 本波弃过牌；code ∈ [0,19] 是「弃牌掩码 +
 *                       取剩余第几张 / 不取」的枚举
 *
 * 枚举与取值域的完整定义见 engine.ts 的 `PICK_CODE_BASE` 与
 * server 的 `decodeDiscardCode` / `CardPickMin`；两端由
 * formula_vectors.json 的 `card_picks` 段锁住。
 *
 * # 脚本设计：覆盖三类卡 × 两条路径
 *
 * | 场景 | 覆盖 |
 * |---|---|
 * | `card_skip_all` | 全部 `-1`（整波跳过）：只验 rollWaveCards 消耗 RNG 的效果 |
 * | `card_take_first` | 全部取第 0 张：技能卡/属性卡/机制卡各波各取一次 |
 * | `card_take_mixed` | 交替取/弃：覆盖 `discardedThisWave` 与弃牌编码 |
 * | `card_multiwave`  | 跑到第 3 波以上：解锁多波次路径 |
 */

interface CardScenario {
  label: string
  levelID: number
  seed: string
  ticks: number
  sampleEvery: number
  skillIDs: number[]
  /** 选牌脚本（编码见上） */
  picks: number[]
  attacker: BuildScenario['attacker']
}

const CARD_SCENARIOS: CardScenario[] = [
  { label: 'card_skip_all', levelID: 25, seed: '7046029255919282421', ticks: 30000, sampleEvery: 200, skillIDs: [1, 2, 3, 5], picks: [], attacker: BUILD_ATTACKER },
  { label: 'card_take_first', levelID: 25, seed: '-1', ticks: 30000, sampleEvery: 200, skillIDs: [1, 2, 3, 5], picks: [], attacker: BUILD_ATTACKER },
  { label: 'card_take_mixed', levelID: 25, seed: '1234567890123456789', ticks: 30000, sampleEvery: 200, skillIDs: [10, 11, 12, 13], picks: [], attacker: BUILD_ATTACKER },
  { label: 'card_multiwave', levelID: 25, seed: '42', ticks: 40000, sampleEvery: 300, skillIDs: [1, 4, 7, 16], picks: [], attacker: BUILD_ATTACKER },
  { label: 'card_charge_tower', levelID: 42, seed: '-1', ticks: 40000, sampleEvery: 300, skillIDs: [1, 2, 3, 5], picks: [], attacker: BUILD_ATTACKER },
  { label: 'card_rotor_vane', levelID: 82, seed: '1', ticks: 40000, sampleEvery: 300, skillIDs: [1, 2, 3, 5], picks: [], attacker: BUILD_ATTACKER },
]

const cardRows = CARD_SCENARIOS.map((s) => {
  const level = fixture.levels.find((l: any) => l.id === s.levelID)
  if (!level) throw new Error(`夹具里没有第 ${s.levelID} 关`)
  const enemyDefs = new Map<number, any>((fixture.enemies as any[]).map((e) => [e.id, e]))
  const skillDefs = new Map<number, any>(
    [...(fixture.skills as any[]), ...(fixture.composite_skills as any[])].map((x) => [x.id, x]),
  )
  const equipped = s.skillIDs.map((id, i) => toEquippedSkill(skillDefs.get(id)!, i))

  const engine = new BattleEngine({
    level: level as never,
    enemies: enemyDefs,
    skills: skillDefs,
    equipped: equipped as never,
    attacker: s.attacker as never,
    seed: BigInt(s.seed),
    scoreRules: scoreRulesFromFixture(),
    activeSlots: 4,
  } as never)

  // ⚠️ 脚本必须在 start() **之前**注入：
  // `applyReplayDecision` 在 `phase === 'card_select'` 时被 step() 调，
  // 而第一次进入 card_select 是第 1 波清空时 —— start() 之后注入也来得及，
  // 但「来得及」不等于「总是」：start() 若将来改成也走一次决策就会漏。
  const picks = s.picks.length ? s.picks : buildPickScript(s.label)
  engine.setReplayScript(picks)
  engine.start()

  const { trace, snap } = makeTracer(engine, s.sampleEvery)
  snap()
  for (let i = 0; i < s.ticks; i++) {
    // ⚠️ 这里**不能**在 card_select 停下：
    //   TS 的 pplyReplayDecision 是在 step() 内部被调的
    //   （phase === 'card_select' && replayScript !== null），
    //   所以解卡牌**需要一次 step**，而 break 会正好跳过它。
    //   结果就是 card_picks 永远是空 —— 选牌脚本根本没被执行。
    //
    //   B~E 的场景之所以可以「非 wave 即停」，是因为它们没有选牌脚本。
    if (engine.phase !== 'wave' && engine.phase !== 'prepare' && engine.phase !== 'card_select') break
    const before = (engine as any).replay.length
    engine.step()
    if (engine.tick % s.sampleEvery === 0 || (engine as any).replay.length > before) snap()
  }
  if (trace[trace.length - 1].tick !== engine.tick) snap()

  return {
    label: s.label,
    level_id: s.levelID,
    seed: s.seed,
    ticks: s.ticks,
    sample_every: s.sampleEvery,
    level_waves: level.waves,
    level_base_hp: level.base_hp,
    level_armor_permille: level.armor_permille ?? 0,
    level_element_cap: level.element_cap ?? 0,
    level_max_reaction_tier: level.max_reaction_tier ?? 1,
    terrain_kinds: (level.terrain as any[]).map((t: any) => t.kind),
    equipped: s.skillIDs,
    // ⚠️ 脚本**必须**带进文件：它就是这一局的「输入」，
    //   漏带的话 Go 侧只能猜 —— 而不同脚本给出不同战斗。
    picks,
    attacker: {
      attack: s.attacker.attack.toString(),
      crit_permille: s.attacker.critPermille.toString(),
      crit_multiplier_permille: s.attacker.critMultiplierPermille.toString(),
      reaction_mult_permille: s.attacker.reactionMultPermille.toString(),
      element_cap: s.attacker.elementCap.toString(),
      reaction_tier: s.attacker.reactionTier.toString(),
      element_coef_permille: s.attacker.elementCoefPermille.toString(),
      heat_cap_permille: s.attacker.heatCapPermille.toString(),
      armor_permille: s.attacker.armorPermille.toString(),
      mechanic_permille: s.attacker.mechanicPermille.toString(),
    },
    trace,
  }
})

/**
 * 按标签构造一个**确定性**的选牌脚本。
 *
 * ⚠️ 为什么用「确定性构造」而不是手写常量列表：
 *
 * 关卡的波数随 levelID 变（第 25 关 7 波、第 42 关更多），
 * 而脚本跑完就停（`replayScriptPos >= script.length` 时走 `skipCards`）——
 * 所以**长度不重要**，重要的是「前 N 项是什么」。
 *
 * 这样脚本长度就不必与波数对齐，少一处需要同步的地方。
 */
function buildPickScript(label: string): number[] {
  // 够长即可：超出后引擎走 skipCards（-1），那也是一条被验的路径。
  const out: number[] = []
  for (let i = 0; i < 12; i++) {
    switch (label) {
      case 'card_skip_all':
        out.push(-1)
        break
      case 'card_take_first':
        // 全部取第 0 张：技能卡/属性卡/机制卡按 rollWaveCards 的
        // 固定顺序排好，所以第 0 张随 RNG 变化而种类不同 ——
        // 几波下来三类卡都会被取到。
        out.push(-3)
        break
      case 'card_take_mixed':
        // 偶数波取第 0 张；奇数波弃**原手牌第 0 张**并取剩余第 0 张。
        //
        // ⚠️ -7 = code 4 = mask 0b001（弃原手牌第 0 张）+ 取剩余第 0 张
        //（第 159 轮编码）。文案随编码重写时一起更正过 ——
        // 旧注释写的是「弃第 1 张」，那是旧编码 -(3+handIdx) 的语义。
        out.push(i % 2 === 0 ? -3 : -7)
        break
      default:
        // 多波次场景：取第 i%3 张，覆盖三个不同下标。
        out.push(-(3 + (i % 3)))
    }
  }
  return out
}
const doc = {
  _comment:    '引擎**纯函数**的跨端契约向量。由 miniapp/scripts/gen-engine-pure-vectors.ts 从真实 TS 实现产出。' +
    '期望值不由 Go 侧生成（纯函数没有运行时状态，用实现产出即定义）。' +
    'Go 侧 domain/engine_pure_contract_test.go 只断言与本文件一致。' +
    '明确不含 runChallenge —— 它需要整个 BattleEngine，Go 侧尚未移植。',
  version: 1,
  skill: {
    rules: DEFAULT_SKILL_RULES,
    // ⚠️ 整数宽度边界，写进文件里让 Go 侧不必去猜：
    //   本文件用 BigInt、Go 用 int64。等级 ≥2 且底伤接近 2^63 时结果超出 int64，
    //   两端**不可能**一致，所以那类用例根本不生成。
    //   Go 侧由 TestSkillVectorsStayInsideInt64 断言「文件里没有越界用例」，
    //   由 TestSkillBaseDamageIdentityHoldsAtInt64Max 断言等级 1 的恒等性。
    int_width_note:
      'level>=2 时若 base 接近 2^63，base*coef 这个**中间乘积**就会超出 int64' +
      '（最终结果 base*coef/1000 可能仍装得下，但 Go 侧是先乘后除，中间已经溢出）。' +
      'TS 用 BigInt 没有这个问题。等级 1 系数恰为 1000‰、走恒等快路径不做乘法，' +
      '所以 2^63-1 只在 level=1 出现，且两端逐位一致。' +
      '实际底伤来自 skills.base_damage（内容表里的小整数），离该边界有数个数量级的余量。',
    rows: skillRows,
  },
  works: {
    base_attacker: {
      attack: COMBAT_ATTACKER.attack.toString(),
      crit_permille: COMBAT_ATTACKER.critPermille.toString(),
      crit_multiplier_permille: COMBAT_ATTACKER.critMultiplierPermille.toString(),
      reaction_mult_permille: COMBAT_ATTACKER.reactionMultPermille.toString(),
      element_cap: COMBAT_ATTACKER.elementCap.toString(),
      reaction_tier: COMBAT_ATTACKER.reactionTier.toString(),
      element_coef_permille: COMBAT_ATTACKER.elementCoefPermille.toString(),
    },
    rows: workRows,
  },
  challenge_seed: { rows: seedRows },
  snapshot_validation: { rows: snapshotRows },
  shield: { rows: shieldRows },
  elements: { rows: elementRows },
  digest: { rows: digestRows },
  fnv: { rows: fnvRows },
  heat: {
    consts: heatConsts,
    // ⚠️ 用**脚本 + 状态轨迹**而不是逐函数取值。
    //   热量那类缺陷（吸收态、余数截断）单看一个函数的返回值看不出来，
    //   要连续跑若干步才现形。
    rows: heatRows,
  },
  deck: { rows: deckRows },
  terrain: {
    consts: {      FIELD_W,
      FIELD_H,
      BASE_X,
      BASE_Y,
      OIL_BURN_MS,
      ROTOR_RADIUS,
      ROTOR_DIR_LEN: ROTOR_DIR_1000.length,
      /**
       * ⚠️ 方向表**抽样**而不是全量 720 个数。
       *
       * 全量塞进 JSON 很难被人 review —— 而没人看的契约等于没有契约
       * （这是选关卡向量时的同一条理由）。
       * 抽样覆盖：0°/90°/180°/270° 四个正交方向、
       * 两个对角、以及几个任意角度，足以抓住符号错与角度错位。
       *
       * 全表的性质（模长 ≈ 1000、长度 720）由 Go 侧
       * `TestRotorDirIsUnitVector` 守住。
       */
      ROTOR_DIR_SAMPLE_IDX: ROTOR_SAMPLE_IDX,
      ROTOR_DIR_SAMPLE: ROTOR_SAMPLE_IDX.map((i) => ROTOR_DIR_1000[i]),
    },
    rows: terrainRows,
  },
  wave: {
    consts: {
      TICK_MS,
      SPAWN_PROGRESS_FULL,
      SPAWN_PROGRESS_STEP,
    },
    rows: waveRows,
  },
  battle: {
    rows: buildRows,
  },
  terrain_battle: {
    rows: wireTerrainRows,
  },
  card_battle: {
    rows: cardRows,
  },
}

const outPath = resolve(serverRoot, 'testdata/engine_pure_vectors.json')
writeFileSync(outPath, JSON.stringify(doc, null, 2) + '\n', 'utf8')
console.log(`写入 ${outPath}`)
console.log(`  技能行 ${skillRows.length} / 装置行 ${workRows.length} / 种子行 ${seedRows.length}`)
console.log(`  校验行 ${snapshotRows.length} / 护盾行 ${shieldRows.length} / 摘要行 ${digestRows.length}`)
console.log(`  热量脚本 ${heatRows.length}（轨迹步 ${heatRows.reduce((a, r) => a + r.trace.length, 0)}） / 手牌脚本 ${deckRows.length}`)
console.log(`  地形脚本 ${terrainRows.length}（轨迹步 ${terrainRows.reduce((a, r) => a + r.trace.length, 0)}）`)
console.log(`  波次场景 ${waveRows.length}（轨迹步 ${waveRows.reduce((a, r) => a + r.trace.length, 0)}）`)
console.log(`  战斗场景 ${buildRows.length}（轨迹步 ${buildRows.reduce((a, r) => a + r.trace.length, 0)}）`)
console.log(`  地形场景 ${wireTerrainRows.length}（轨迹步 ${wireTerrainRows.reduce((a, r) => a + r.trace.length, 0)}）`)
console.log(`  卡牌场景 ${cardRows.length}（轨迹步 ${cardRows.reduce((a, r) => a + r.trace.length, 0)}）`)
