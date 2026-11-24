import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { buildNextWaveBrief } from './nextWaveBrief'

const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round26-next-wave-brief/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = readFileSync(specPath, 'utf-8')
const battleSrc = readSrc('src/pages/battle/battle.vue')
const engineSrc = readSrc('src/game/engine.ts')
const tokensSrc = readSrc('src/styles/tokens.css')

type Spawn = { enemy_id: number; count: number }
type Wave = { wave_index: number; spawns: Spawn[] }
type EnemyDef = { id: number; name: string; is_boss?: boolean; fly_height?: number; burrow?: boolean; attack_range?: number }

describe('R26 下一波敌情聚合口径（真实模块 buildNextWaveBrief）', () => {
  it('取 waves[waveIndex+1]，按 enemy_id 聚合数量且同组相加', () => {
    const waves: Wave[] = [
      { wave_index: 0, spawns: [{ enemy_id: 1, count: 2 }] },
      { wave_index: 1, spawns: [{ enemy_id: 2, count: 3 }, { enemy_id: 2, count: 2 }, { enemy_id: 3, count: 5 }] },
    ]
    const b = buildNextWaveBrief(waves, 0, { 2: { id: 2, name: '疾行兽' }, 3: { id: 3, name: '浮空囊' } })
    expect(b.waveIndex).toBe(1)
    expect(b.empty).toBe(false)
    expect(b.rows).toHaveLength(2)
    const e2 = b.rows.find(r => r.enemy_id === 2)!
    expect(e2.count).toBe(5)
    expect(e2.name).toBe('疾行兽')
  })

  it('真实 store.enemyMap 形状：按敌种 id 查找而非数组下标', () => {
    const defs = new Map<number, EnemyDef>([
      [42, { id: 42, name: '裂隙兽', burrow: true }],
      [7, { id: 7, name: '首领', is_boss: true }],
    ])
    const b = buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 42, count: 2 }, { enemy_id: 7, count: 1 }] }], -1, defs)
    expect(b.rows.find(r => r.enemy_id === 42)).toMatchObject({ name: '裂隙兽', threat: '[潜地]', unknown: false })
    expect(b.rows.find(r => r.enemy_id === 7)).toMatchObject({ name: '首领', threat: '[首领]', unknown: false })
  })

  it('行数按 count 降序、同数量按 enemy_id 升序稳定排序', () => {
    const b = buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 9, count: 3 }, { enemy_id: 1, count: 3 }, { enemy_id: 5, count: 7 }] }], -1, {})
    expect(b.rows.map(r => r.enemy_id)).toEqual([5, 1, 9])
  })

  it('威胁标签优先级：首领 > 飞行 > 潜地 > 远程', () => {
    const defs: Record<number, EnemyDef> = {
      1: { id: 1, name: '王', is_boss: true, fly_height: 5, burrow: true, attack_range: 9 },
      2: { id: 2, name: '飞', fly_height: 3, burrow: true, attack_range: 9 },
      3: { id: 3, name: '潜', burrow: true, attack_range: 9 },
      4: { id: 4, name: '远', attack_range: 6 },
    }
    const b = buildNextWaveBrief([{ wave_index: 0, spawns: [1, 2, 3, 4].map(id => ({ enemy_id: id, count: 1 })) }], -1, defs)
    expect(b.rows.map(r => r.threat)).toEqual(['[首领]', '[飞行]', '[潜地]', '[远程]'])
  })

  it('未知敌种诚实标注 #id 与 unknown，绝不编造名字或威胁', () => {
    const b = buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 999, count: 2 }] }], -1, {})
    const row = b.rows.find(r => r.enemy_id === 999)!
    expect(row.name).toBe('#999')
    expect(row.unknown).toBe(true)
    expect(row.threat).toBe('')
  })

  it('空态：无 waves / 下一波越界 / spawns 空 / 全无效行 → empty=true 且 rows=[]', () => {
    expect(buildNextWaveBrief(undefined, 0, {}).empty).toBe(true)
    expect(buildNextWaveBrief([], 0, {}).empty).toBe(true)
    expect(buildNextWaveBrief([{ wave_index: 0, spawns: [] }], -1, {}).empty).toBe(true)
    expect(buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 1, count: 0 }] }], -1, {}).empty).toBe(true)
    expect(buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 1.5, count: 1 }] }], -1, {}).rows).toEqual([])
  })

  it('AI 结论真实来自聚合（含飞行数量），无威胁给朴素结论', () => {
    const defs: Record<number, EnemyDef> = { 1: { id: 1, name: '甲' }, 2: { id: 2, name: '浮空囊', fly_height: 3 } }
    const b = buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 1, count: 3 }, { enemy_id: 2, count: 4 }] }], -1, defs)
    expect(b.ai).toContain('2 种敌人共 7 只')
    expect(b.ai).toContain('[飞行]×4')
    const plain = buildNextWaveBrief([{ wave_index: 0, spawns: [{ enemy_id: 1, count: 1 }] }], -1, defs)
    expect(plain.ai).toContain('按伤害与元素协同选卡')
  })
})

describe('R26 真实源码守卫（防止只测 SPEC 文本）', () => {
  it('battle.vue 模板落地 R26 六类选择器与逐字文案', () => {
    for (const s of ['r26-wave-brief', 'r26-brief-head', 'r26-enemy-row', 'data-r26-enemy', 'r26-brief-empty', 'r26-ai-note']) expect(battleSrc).toContain(s)
    expect(battleSrc).toContain('暂无下一波敌情信息')
    expect(battleSrc).toContain('[未知敌种]')
    expect(battleSrc).toContain('辅助简报非权威战斗数据，实际敌情以引擎波次为准；权威回放以 replay_hash 为准')
  })

  it('battle.vue 使用 store.enemyMap，且下一波标题按返回的下一波下标 +1 显示', () => {
    expect(battleSrc).toMatch(/buildNextWaveBrief\(eng\.cfg\.level\.waves,\s*eng\.waveIndex,\s*store\.enemyMap\s*\)/)
    expect(battleSrc).not.toContain('store.config?.enemies as any')
    expect(battleSrc).toContain('r26Brief.waveIndex + 1')
    expect(battleSrc).not.toContain('r26Brief.waveIndex + 2')
  })

  it('battle.vue CSS 落地 max-height: 160rpx 与 overflow-y: auto 滚动约束', () => {
    expect(battleSrc).toMatch(/max-height\s*:\s*160rpx/)
    expect(battleSrc).toMatch(/overflow-y\s*:\s*auto/)
  })

  it('battle.vue 从 nextWaveBrief 导入并调用真实纯函数', () => {
    expect(battleSrc).toContain('buildNextWaveBrief')
    expect(battleSrc).toContain('./nextWaveBrief')
  })
})

describe('R26 SPEC 选择器与逐字文案契约', () => {
  it('SPEC 钉死六类选择器', () => {
    for (const s of ['.r26-wave-brief', '.r26-brief-head', '.r26-enemy-row', '[data-r26-enemy]', '.r26-brief-empty', '.r26-ai-note']) {
      expect(spec).toContain(s)
    }
  })

  it('SPEC 钉死空态与未知敌种逐字文案', () => {
    expect(spec).toContain('暂无下一波敌情信息')
    expect(spec).toContain('[未知敌种]')
  })

  it('SPEC 钉死布局约束与逐字依据行', () => {
    expect(spec).toContain('max-height: 160rpx')
    expect(spec).toContain('overflow-y: auto')
    expect(spec).toContain('依据：当前关卡 waves[waveIndex+1].spawns 按 enemy_id 聚合；敌种名与威胁标签来自 /config.enemies')
  })
})

describe('R26 诚实红线与 AI 四判据', () => {
  it('SPEC 含逐字诚实红线', () => {
    expect(spec).toContain('辅助简报非权威战斗数据，实际敌情以引擎波次为准；权威回放以 replay_hash 为准')
  })

  it('SPEC 钉死只读派生边界', () => {
    for (const k of ['不得调用', 'step()', 'replay_hash', 'score', 'loot']) expect(spec).toContain(k)
  })

  it('SPEC 含 AI 四判据关键词', () => {
    for (const k of ['可见汉字', '真实数据结论', '依据行', 'CTA']) expect(spec).toContain(k)
  })
})

describe('R26 既有数据源依据与三端不回退', () => {
  it('engine.ts 含 checkWaveEnd 与 waveIndex+1 依据', () => {
    for (const k of ['checkWaveEnd', 'const next = this.waveIndex + 1', 'card_select', 'cfg.level.waves']) expect(engineSrc).toContain(k)
  })

  it('battle.vue 保留选卡遮罩与 R22 协同卡锚点', () => {
    for (const k of ['showCards', 'class="sheet"', 'hand-card', 'r22-synergy-card']) expect(battleSrc).toContain(k)
  })

  it('令牌三锚点保留且无 --r26- 新令牌', () => {
    for (const k of ['lyp-tokens-v1', 'lyp-tokens-v3-r3', 'lyp-tokens-r6']) expect(tokensSrc).toContain(k)
    expect(tokensSrc).not.toContain('--r26-')
  })
})

describe('R26 零真值', () => {
  it('SPEC 与源码不含域名/IP/JWT/口令/个人绝对路径', () => {
    for (const text of [spec, battleSrc, tokensSrc]) {
      expect(text).not.toMatch(/eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}/)
      expect(text).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/)
      expect(text).not.toMatch(/postgres:\/\/[^\s]+:[^\s]+@/)
      expect(text).not.toMatch(/C:\\Users\\[^\s]+/)
    }
  })
})
