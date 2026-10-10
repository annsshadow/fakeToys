/**
 * 反应链数值表的跨端一致性检查。
 *
 * 背景：`server/internal/domain/elements.go` 的 reactionSpecs 与本文件的
 * REACTIONS 常量是**两份手工同步的副本**，而它们直接决定伤害数值。
 * 单方面改动会让两端算出不同结果 —— 线上表现为
 * 「所有人的回放都验不出真伪」（I-6 的哈希比对失去意义），且不会有任何报错。
 *
 * 唯一的仲裁者是 server/testdata/reaction_specs.json：
 * 它由 `go run ./cmd/vectors` 从 Go 侧真相源导出（**不含任何期望值** ——
 * 用实现生成期望值就是自证），本测试断言本文件的副本与它一致。
 *
 * ⚠️ 改动反应数值时必须两侧同步，并重新执行：
 *   cd server && go run ./cmd/vectors
 */
import { describe, it, expect } from 'vitest'
import reactionSpecs from '@vectors/reaction_specs.json'
import { REACTIONS, REACTION_ORDER, type ReactionSpec } from './elements'

interface Row {
  key: string
  name: string
  base_coef: number
  attack_weight_pct: number
  status_duration_ms: number
  aoe_radius: number
  dispel_shield: boolean
  amplify_pct: number
  armor_shred_permille: number
  knockback: number
  descr: string
}

const rows = reactionSpecs as Row[]

describe('跨端契约：反应链数值表', () => {
  it('契约文件本身不能为空', () => {
    expect(rows.length).toBeGreaterThan(0)
  })

  it('条数与顺序必须与 REACTION_ORDER 一致', () => {
    // 顺序是契约的一部分：Go 的 AllReactionSpecs 按 AllReactions() 的固定序导出，
    // 客户端按导出顺序断言。顺序漂移会让逐条对比全部错位，
    // 表现为一堆看不懂的字段不匹配。
    expect(rows.map((r) => r.key)).toEqual([...REACTION_ORDER])
  })

  it('REACTIONS 必须恰好覆盖契约里的每一条', () => {
    expect(Object.keys(REACTIONS).sort()).toEqual(rows.map((r) => r.key).sort())
  })

  for (const row of rows) {
    describe(`反应 ${row.key}`, () => {
      const got = REACTIONS[row.key as keyof typeof REACTIONS] as ReactionSpec

      it('名称一致', () => {
        expect(got.name).toBe(row.name)
      })
      it('基础系数一致（千分比）', () => {
        expect(got.baseCoef).toBe(row.base_coef)
      })
      it('攻击力权重一致（千分比）', () => {
        // 单独拎出来断言，是因为这是 I-1 的反通胀红线：
        // 两端"一致地"超过 300‰ 时，逐字段对比照样会绿。
        // 红线由本文件与 Go 侧 TestReactionAttackWeightRedLine 各自独立守住。
        expect(got.attackWeightPct).toBe(row.attack_weight_pct)
        expect(got.attackWeightPct).toBeLessThanOrEqual(300)
      })
      it('状态时长一致（毫秒）', () => {
        expect(got.statusDurationMs).toBe(row.status_duration_ms)
      })
      it('溅射半径一致', () => {
        expect(got.aoeRadius).toBe(row.aoe_radius)
      })
      it('驱散护盾标记一致', () => {
        expect(got.dispelShield).toBe(row.dispel_shield)
      })
      it('受击放大比例一致（千分比）', () => {
        expect(got.amplifyPct).toBe(row.amplify_pct)
      })
      it('削甲量一致（千分比）', () => {
        // armor_break 专用字段：引擎 hitEnemy 在削甲期内折进 Defender 的有效护甲。
        // 两端漂移会让「破甲击退」在客户端与验真侧重放出不同伤害。
        expect(got.armorShredPermille).toBe(row.armor_shred_permille)
        expect(got.armorShredPermille).toBeGreaterThanOrEqual(0)
      })
      it('击退位移一致（定点 ×1000）', () => {
        // armor_break 专用字段：一次性右推，stepEnemyMotion 的 knockback 分支消费。
        expect(got.knockback).toBe(row.knockback)
        expect(got.knockback).toBeGreaterThanOrEqual(0)
      })
      it('效果说明一致', () => {
        // descr 是文档站「效果」列的数据源。曾经服务端根本没这个字段，
        // 于是文档站上是一列空白 —— 而文档站的定位就是「与线上版本一致」。
        expect(got.descr).toBe(row.descr)
      })
    })
  }
})
