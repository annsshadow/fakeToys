/**
 * 共享测试夹具：页面与 store 测试共用的 GameConfig / 实体数据。
 * 只提供「满足类型 + 覆盖消费分支」的最小数据，字段语义见各类型注释。
 */
import type { GameConfig } from '@/api/client'
import type { Element } from '@/game/elements'
import type { EnemyDef, GeneratedLevel, MasteryFamily, SkillDef } from '@/game/types'

export function makeLevel(over: Partial<GeneratedLevel> = {}): GeneratedLevel {
  return {
    id: 1,
    chapter: 1,
    name: '测试关',
    seed: '1',
    base_hp: 1000,
    wave_count: 2,
    difficulty: 1000,
    energy_cost: 6,
    element_cap: 3,
    armor_permille: 0,
    max_reaction_tier: 2,
    is_boss: false,
    star_targets: [100, 200, 300],
    max_score: 500,
    terrain: [],
    waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 2, interval: 50, delay: 0 }] }],
    ...over,
  }
}

export function makeSkill(over: Partial<SkillDef> = {}): SkillDef {
  return {
    id: 1,
    code: 'ember',
    name: '燃烧弹',
    family: 'flame',
    element: 'fire' as Element,
    kind: 'active',
    descr: '投出一枚燃烧弹',
    base_damage: 100,
    heat_cost: 20,
    cooldown_ms: 800,
    pierce: 0,
    aoe_radius: 60,
    apply_element: 'fire',
    apply_stacks: 1,
    projectile_speed: 60000,
    chain: 0,
    unlock_level: 1,
    ...over,
  }
}

export function makeEnemy(over: Partial<EnemyDef> = {}): EnemyDef {
  return {
    id: 1,
    code: 'wanderer',
    name: '游荡者',
    category: 'normal',
    hp: 100,
    speed: 40000,
    armor: 0,
    shield_hp: 0,
    attack: 10,
    attack_range: 0,
    attack_interval: 0,
    fly_height: 0,
    burrow: false,
    is_boss: false,
    resist: { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 },
    descr: '',
    ...over,
  }
}

export function makeMasteryFamily(over: Partial<MasteryFamily> = {}): MasteryFamily {
  return {
    family: 'flame',
    name: '烈焰',
    nodes: [
      {
        id: 11,
        family: 'flame',
        layer: 1,
        slot: 0,
        name: '燃料压缩',
        kind: 'stat',
        value: 10,
        prereq_family: '',
        prereq_layer: 0,
      },
    ],
    ...over,
  }
}

/** 页面/store 共用的最小 GameConfig。over 深度覆盖顶层字段即可。 */
export function makeConfig(over: Partial<GameConfig> = {}): GameConfig {
  return {
    version: 1,
    levels: [makeLevel(), makeLevel({ id: 2, chapter: 1 }), makeLevel({ id: 3, chapter: 2 })],
    enemies: [makeEnemy()],
    skills: [makeSkill(), makeSkill({ id: 2, code: 'frost', name: '冰霜弹', element: 'ice' as Element, family: 'frost' })],
    composite_skills: [makeSkill({ id: 101, code: 'steam', name: '蒸汽弹', element: 'fire' as Element })],
    recipes: [],
    equipment: [
      { id: 1, name: '焰纹长弓', element: 'fire', descr: '火系武器', slot: 'weapon', tier: 2 },
    ],
    gems: [{ id: 1, name: '元素石', descr: '提高元素系数' }],
    gem_qualities: [{ name: '灰', affix_count: 1, mult: 1 }],
    skins: [],
    mastery_families: [makeMasteryFamily()],
    reactions: [
      {
        key: 'steam_burst',
        name: '蒸汽爆发',
        base_coef: 60,
        attack_weight_pct: 300,
        status_duration_ms: 0,
        aoe_radius: 120,
        dispel_shield: true,
        amplify_pct: 0,
      },
    ],
    chapters: [
      { id: 1, name: '第一章', start_level: 1, end_level: 2, terrain_kind: 'none', boss_enemy_id: 0 },
      { id: 2, name: '第二章', start_level: 3, end_level: 3, terrain_kind: 'none', boss_enemy_id: 0 },
    ],
    rating_weights: {
      element_coverage: 1,
      reaction_coverage: 1,
      mastery_done: 1,
      equipment_synergy: 1,
      mechanic_depth: 1,
    },
    // 第 133 轮：score_rules 必须与真实服务端契约同形（千分比整数 + 整数字段）。
    // 旧值是 [0.4,0.7,1] 小数 —— 该字段此前**零消费**，漂移无人发现；
    // 现在引擎改从 score_rules 取数，BigInt(0.4) 直接抛错。
    // 取 DEFAULT_SCORE_RULES 同值，保证「喂给引擎 == 引擎缺省」，测试行为不变。
    score_rules: {
      per_damage_unit: 100,
      on_kill_normal: 500,
      on_kill_boss: 5000,
      star_target_ratio: [600, 850, 980],
      score_full_at_sec: 60,
    },
    skill_rules: { max_level: 10, coef_permille: 100, base_cost: 100 },
    server_time: '2026-01-01T00:00:00Z',
    ...over,
  }
}
