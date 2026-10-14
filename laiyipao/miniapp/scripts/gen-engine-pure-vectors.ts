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
import { writeFileSync } from 'node:fs'
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
import type { DefenseSnapshot, DefenseView } from '../src/game/defense'
import type { Attacker } from '../src/game/damage'

const here = dirname(fileURLToPath(import.meta.url))
const serverRoot = resolve(here, '../../server')

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
const BASE_ATTACKER: Attacker = {
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
  // 的 TestApplyWorksMatchesDocumentedMapping），把「三���装置各加多少」
  // 钉成**独立于本文件**的断言。若两端映射表真的不一致，
  // 至少有一处会红。
  const a: Attacker = { ...BASE_ATTACKER }
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

const doc = {
  _comment:
    '引擎**纯函数**的跨端契约向量。由 miniapp/scripts/gen-engine-pure-vectors.ts 从真实 TS 实现产出。' +
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
      attack: BASE_ATTACKER.attack.toString(),
      crit_permille: BASE_ATTACKER.critPermille.toString(),
      crit_multiplier_permille: BASE_ATTACKER.critMultiplierPermille.toString(),
      reaction_mult_permille: BASE_ATTACKER.reactionMultPermille.toString(),
      element_cap: BASE_ATTACKER.elementCap.toString(),
      reaction_tier: BASE_ATTACKER.reactionTier.toString(),
      element_coef_permille: BASE_ATTACKER.elementCoefPermille.toString(),
    },
    rows: workRows,
  },
  challenge_seed: { rows: seedRows },
  snapshot_validation: { rows: snapshotRows },
  shield: { rows: shieldRows },
  elements: { rows: elementRows },
  digest: { rows: digestRows },
  fnv: { rows: fnvRows },
}

const outPath = resolve(serverRoot, 'testdata/engine_pure_vectors.json')
writeFileSync(outPath, JSON.stringify(doc, null, 2) + '\n', 'utf8')
console.log(`写入 ${outPath}`)
console.log(`  技能行 ${skillRows.length} / 装置行 ${workRows.length} / 种子行 ${seedRows.length}`)
console.log(`  校验行 ${snapshotRows.length} / 护盾行 ${shieldRows.length} / 摘要行 ${digestRows.length}`)
