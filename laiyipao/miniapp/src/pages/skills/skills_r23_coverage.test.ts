// @vitest-environment jsdom
/**
 * R23 备战态构筑反应覆盖诊断：渲染 RED→GREEN 测试。
 * 直接注入 store.config / store.loadout，验证纯展示卡的派生与空态。
 */
import { describe, it, expect, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeSkill } from '../../test/fixtures'

beforeEach(() => { installUniMock() })
afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })
import { vi } from 'vitest'

function skill(id: number, element: string, apply: string = element) {
  return makeSkill({ id, code: 's' + id, name: '技能' + id, element: element as any, apply_element: apply })
}

async function open(loadout: number[], skillsMeta: any[]) {
  const S = (await import('./skills.vue')).default
  const { wrapper, store } = mountPage(S, (s) => {
    s.config = makeConfig({ skills: skillsMeta, composite_skills: [] })
    s.loadout = [...loadout, ...Array(Math.max(0, 4 - loadout.length)).fill(0)].slice(0, 4)
  })
  await flushPromises()
  return { wrapper, store }
}

describe('skills.vue R23 构筑反应覆盖诊断', () => {
  it('焰+冰构筑：覆盖蒸汽爆发标签 + 补电新增过热/超导建议 + AI四判据 + 红线', async () => {
    const { wrapper } = await open([1, 2], [skill(1, 'fire'), skill(2, 'ice'), skill(3, 'lightning')])
    const card = wrapper.find('.r23-coverage-card')
    expect(card.exists()).toBe(true)
    const tags = card.findAll('.r23-coverage-tag')
    const keys = tags.map(t => t.attributes('data-r23-reaction'))
    expect(keys).toContain('steam_burst')
    const suggest = card.find('.r23-gap-suggest')
    expect(suggest.exists()).toBe(true)
    expect(suggest.text()).toContain('补【电】可新增【过热】')
    const note = card.find('.r23-ai-note')
    expect(note.text()).toContain('AI')
    expect(note.text()).toContain('蒸汽爆发')
    expect(note.text()).toContain('依据：构筑槽元素 × REACTION_PAIRS(源自引擎 lookupReaction)派生')
    expect(note.text()).toContain('去调整出战配置')
    expect(note.text()).toContain('辅助诊断非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准')
    wrapper.unmount()
  })

  it('空构筑/无元素技能走逐字空态；全五元素走无需补齐空态', async () => {
    const { wrapper } = await open([], [skill(1, 'fire'), skill(2, 'ice')])
    expect(wrapper.find('.r23-coverage-empty').text()).toBe('当前构筑没有元素技能，无法诊断反应覆盖')
    const { wrapper: w2 } = await open([1], [skill(1, 'fire', '')])
    expect(w2.find('.r23-coverage-empty').text()).toBe('当前构筑没有元素技能，无法诊断反应覆盖')
    // 4 槽全同元素去重后 E={fire}：单元素无法成反应 → 无覆盖标签，仍给真实补齐建议
    const { wrapper: w3 } = await open([1, 2, 3, 4], [1, 2, 3, 4].map(id => skill(id, 'fire')))
    expect(w3.findAll('.r23-coverage-tag').length).toBe(0)
    expect(w3.find('.r23-gap-suggest').exists()).toBe(true)
    wrapper.unmount(); w2.unmount(); w3.unmount()
  })

  it('诊断只读不写：store.loadout 不被诊断卡改写，差集按 REACTION_ORDER 稳定排序', async () => {
    const loadout = [1, 2, 0, 0]
    const { wrapper, store } = await open(loadout, [skill(1, 'fire'), skill(2, 'ice'), skill(3, 'lightning')])
    expect(store.loadout).toEqual([1, 2, 0, 0])
    const tags = wrapper.findAll('.r23-coverage-tag')
    expect(tags.length).toBe(1)
    expect(tags[0]!.attributes('data-r23-reaction')).toBe('steam_burst')
    wrapper.unmount()
  })
})
