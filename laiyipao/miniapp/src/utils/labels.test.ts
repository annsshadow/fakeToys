/**
 * 界面用词表的契约守卫（2026-10-21 P6-5）。
 *
 * 这个文件存在的理由就是「同一个东西在两个页面叫两个名字」——所以测试要钉的
 * 不是「查得到」，而是两条更硬的约束：
 *
 * 1. **已知 key 必须翻成中文且与原 key 不同**。若某条写成 `coin: 'coin'`
 *    （复制粘贴漏改），查得到、也不报错，但玩家看到的仍是英文 key，
 *    与「回落原样返回」的分支在界面上无法区分。
 * 2. **未知 key 必须原样返回**。返回空串/undefined 会让新道具在界面上显示空白，
 *    而空白看起来像「数量是 0」——比显示英文 key 更误导。
 */
import { describe, expect, it } from 'vitest'
import { affixName, currencyName, qualityName } from '@/utils/labels'

describe('currencyName', () => {
  it('translates every known currency to a distinct Chinese label', () => {
    const known = {
      coin: '金币',
      gem: '钻石',
      energy: '体力',
      keys: '钥匙',
      revive_token: '复活币',
      mastery_reset: '专精重铸券',
      gem_wash_token: '宝石洗练券',
    }
    for (const [key, label] of Object.entries(known)) {
      expect(currencyName(key)).toBe(label)
      expect(currencyName(key)).not.toBe(key)
    }
  })

  it('returns unknown keys verbatim so new server currencies stay visible', () => {
    expect(currencyName('season_token')).toBe('season_token')
    expect(currencyName('')).toBe('')
  })
})

describe('qualityName', () => {
  it('translates every known quality to a distinct Chinese label', () => {
    for (const [key, label] of Object.entries({
      gray: '灰',
      blue: '蓝',
      purple: '紫',
      red: '红',
      gold: '金',
    })) {
      expect(qualityName(key)).toBe(label)
      expect(qualityName(key)).not.toBe(key)
    }
  })

  it('returns unknown qualities verbatim', () => {
    // 后端新增一档（比如 'rainbow'）时不能显示空白
    expect(qualityName('rainbow')).toBe('rainbow')
  })
})

describe('affixName', () => {
  it('translates every known affix to a distinct Chinese label', () => {
    for (const [key, label] of Object.entries({
      attack: '攻击',
      crit: '暴击',
      crit_multiplier: '暴击倍率',
      element_coef: '元素系数',
      reaction_mult: '反应倍率',
      element_cap: '元素层数',
      heat_cap: '热量上限',
      armor: '护甲',
    })) {
      expect(affixName(key)).toBe(label)
      expect(affixName(key)).not.toBe(key)
    }
  })

  it('returns unknown affixes verbatim', () => {
    expect(affixName('lifesteal')).toBe('lifesteal')
  })
})

describe('跨表一致性', () => {
  it('never returns an empty label for any input', () => {
    // 空白标签在界面上会被读成「数量为 0」或「没有该属性」
    for (const fn of [currencyName, qualityName, affixName]) {
      for (const input of ['coin', 'gray', 'attack', 'unknown-key', '']) {
        const out = fn(input)
        expect(out).toBeTypeOf('string')
        if (input !== '') expect(out.length).toBeGreaterThan(0)
      }
    }
  })

  it('keeps currency / quality / affix vocabularies disjoint', () => {
    // 同一个 key 在两张表里给出不同中文，正是 P6-5 要消灭的现象；
    // 这里只断言「没有 key 同时出现在两张表里且译名不同」。
    const pairs: Array<[string, (k: string) => string]> = [
      ['currency', currencyName],
      ['quality', qualityName],
      ['affix', affixName],
    ]
    const seen = new Map<string, string>()
    for (const [name, fn] of pairs) {
      for (const key of ['coin', 'gem', 'gray', 'blue', 'attack', 'crit', 'armor']) {
        const label = fn(key)
        if (label === key) continue // 不在该表里，跳过
        const previous = seen.get(key)
        if (previous) expect(previous, `key ${key} 译名冲突`).toBe(`${name}:${label}`)
        else seen.set(key, `${name}:${label}`)
      }
    }
  })
})
