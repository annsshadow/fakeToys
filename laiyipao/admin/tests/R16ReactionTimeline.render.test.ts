// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R16 实施行为契约：admin 战报详情逐次反应时间线（reaction-timeline-v1 消费）。
 * 与 t141 的 SPEC/源码文本守卫不同，本文件挂载真实 BattlesView 打开详情抽屉，
 * 断言 .r16-* DOM 的真实渲染行为：排序/取名/格式化/空态/截断/诚实红线。
 * test-first：先写本文件跑 RED，再实施 BattlesView 跑 GREEN。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { createMemoryHistory, createRouter } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminBattleView } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const fetchBattles = vi.hoisted(() => vi.fn())
const fetchBattleDetail = vi.hoisted(() => vi.fn())
const fetchReactions = vi.hoisted(() => vi.fn())
const verifyBattle = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchBattles, fetchBattleDetail, fetchReactions, verifyBattle }))

import BattlesView from '@/views/BattlesView.vue'

const battleRow: AdminBattleView = {
  id: 1, user_id: 9, nickname: '测试玩家', level_id: 3,
  result: 'win', stars: 3, score: 88000, kills: 10, leaked: 2,
  wave_reached: 5, duration_ms: 12345, reactions: 5, heat_max: 90,
  replay_hash: 'abcd1234abcd1234abcd1234abcd1234',
  created_at: '2026-01-02T03:04:05Z', total_enemies: 12,
  verify_checked: 0, verify_mismatched: 0,
}

const detailBase = {
  id: 1, user_id: 9, nickname: '铁壁玩家', level_id: 3,
  result: 'win', stars: 3, score: 88000, kills: 10, leaked: 2,
  hp_left: 420, wave_reached: 5, duration_ms: 12345,
  shots: 10, hits: 8, reactions: 5, heat_max: 90,
  elements_used: {}, reactions_used: {}, terrain_used: [],
  replay_hash: 'abcd1234abcd1234abcd1234abcd1234',
  seed: '424242', power_after: 91000,
  created_at: '2026-01-02T03:04:05Z',
}

function reactionSpec(key: string, name: string) {
  return {
    key, name, base_coef: 60, attack_weight_pct: 300,
    status_duration_ms: 0, aoe_radius: 0, dispel_shield: false,
    amplify_pct: 0, descr: '',
  }
}

async function openDetail(detail: Record<string, unknown>) {
  fetchBattleDetail.mockResolvedValueOnce({ battle: { ...detailBase, ...detail } })
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/battles', component: { template: '<div />' } },
      { path: '/users', component: { template: '<div />' } },
    ],
  })
  await router.push('/battles')
  await router.isReady()
  fetchBattles.mockResolvedValue({ items: [battleRow], total: 1 })
  const wrapper = mount(BattlesView, { global: { plugins: [ElementPlus, router] } })
  await flushPromises()
  const btn = wrapper.findAll('button').find((b) => b.text() === '详情')
  expect(btn, '找不到「详情」按钮').toBeTruthy()
  await btn!.trigger('click')
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  fetchReactions.mockResolvedValue([
    reactionSpec('steam_burst', '蒸汽爆发'),
    reactionSpec('overheat', '过热'),
  ])
  verifyBattle.mockResolvedValue({
    battle_id: 1, expected_hash: 'h', actual_hash: 'h', matched: true,
    level_id: 3, seed: '424242', recorded_at: '2026-01-02T03:04:05Z',
  })
})

describe('R16 战报详情逐次反应时间线渲染', () => {
  it('合法事件按 wave/tick 升序渲染，服务端取名 + 伤害格式化 + AI 四要素', async () => {
    const wrapper = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1',
        truncated: false,
        total: 3,
        events: [
          { wave: 2, tick: 1, reaction: 'overheat', enemy_uid: 7, damage: 100 },
          { wave: 1, tick: 5, reaction: 'steam_burst', enemy_uid: 3, damage: 12.345678 },
          { wave: 1, tick: 2, reaction: 'steam_burst', enemy_uid: 2, damage: 55 },
        ],
      },
    })
    const box = wrapper.find('.r16-reaction-timeline')
    expect(box.exists(), '找不到 .r16-reaction-timeline').toBe(true)
    expect(box.attributes('role')).toBe('status')
    expect(box.text()).toContain('AI')

    const rows = wrapper.findAll('.r16-timeline-row')
    expect(rows.length).toBe(3)
    expect(rows.map((r) => r.attributes('data-r16-wave'))).toEqual(['1', '1', '2'])
    expect(rows[0].text()).toContain('#2')
    expect(rows[1].text()).toContain('#3')
    expect(rows[2].text()).toContain('#7')
    expect(rows[0].text()).toContain('蒸汽爆发')
    expect(rows[2].text()).toContain('过热')
    expect(rows[0].text()).toContain('55伤害')
    expect(rows[1].text()).toContain('12.3伤害')
    expect(rows[1].text()).not.toContain('12.345678')
    expect(rows[2].text()).toContain('100伤害')

    const text = box.text()
    expect(text).toContain('本局记录 3 次逐次反应')
    expect(text).toContain('依据：reaction_timeline.events')
    expect(text).toContain('展示性归因')
    expect(text).toContain('发起验真')
    wrapper.unmount()
  })

  it('非法事件被丢弃，合法事件数如实计数', async () => {
    const wrapper = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1',
        truncated: false, total: 6,
        events: [
          { wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 1, damage: 10 },
          { wave: 0, tick: 1, reaction: 'overheat', enemy_uid: 2, damage: 10 },
          { wave: 1, tick: -1, reaction: 'overheat', enemy_uid: 3, damage: 10 },
          { wave: 1, tick: 2, reaction: '', enemy_uid: 4, damage: 10 },
          { wave: 1, tick: 3, reaction: 'overheat', enemy_uid: 1.5, damage: 10 },
          { wave: 1, tick: 4, reaction: 'mega_boom', enemy_uid: 5, damage: Number.NaN },
        ],
      },
    })
    const rows = wrapper.findAll('.r16-timeline-row')
    expect(rows.length).toBe(1)
    expect(rows[0].text()).toContain('蒸汽爆发')
    expect(wrapper.find('.r16-reaction-timeline').text()).toContain('本局记录 1 次逐次反应')
    wrapper.unmount()
  })

  it('未知反应 key 用服务端原始 key 兜底（不硬编码中文名）', async () => {
    const wrapper = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1', truncated: false, total: 1,
        events: [{ wave: 1, tick: 1, reaction: 'mega_boom', enemy_uid: 9, damage: 5 }],
      },
    })
    expect(wrapper.find('.r16-timeline-row').text()).toContain('mega_boom')
    wrapper.unmount()
  })

  it('截断态：truncated=true 显示逐字提示，false 不显示', async () => {
    const on = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1', truncated: true, total: 999,
        events: [{ wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 1, damage: 10 }],
      },
    })
    expect(on.find('.r16-truncated-note').exists()).toBe(true)
    expect(on.find('.r16-truncated-note').text()).toBe('已截断，仅显示前 200 条')
    on.unmount()

    const off = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1', truncated: false, total: 1,
        events: [{ wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 1, damage: 10 }],
      },
    })
    expect(off.find('.r16-truncated-note').exists()).toBe(false)
    off.unmount()
  })

  it.each([
    ['字段缺失', {} as Record<string, unknown>],
    ['空数组', { reaction_timeline: { schema_version: 'reaction-timeline-v1', truncated: false, total: 0, events: [] } }],
    ['schema 不符', { reaction_timeline: { schema_version: 'legacy', truncated: false, total: 1, events: [{ wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 1, damage: 10 }] } }],
    ['结构异常', { reaction_timeline: { schema_version: 'reaction-timeline-v1', truncated: false, total: 1, events: undefined } }],
  ])('空态诚实：%s 显示逐字空态且不渲染任何行', async (_name, detail) => {
    const wrapper = await openDetail(detail)
    const box = wrapper.find('.r16-reaction-timeline')
    expect(box.exists()).toBe(true)
    const empty = wrapper.find('.r16-timeline-empty')
    expect(empty.exists(), '找不到 .r16-timeline-empty').toBe(true)
    expect(empty.text()).toBe('该战报无逐次反应记录（旧版本结算）')
    expect(wrapper.findAll('.r16-timeline-row').length).toBe(0)
    expect(box.text()).not.toContain('undefined')
    expect(box.text()).not.toContain('NaN')
    wrapper.unmount()
  })

  it('诚实红线：admin 不复用 H5 r15 选择器，明确展示性归因非权威结算', async () => {
    const wrapper = await openDetail({
      reaction_timeline: {
        schema_version: 'reaction-timeline-v1', truncated: false, total: 1,
        events: [{ wave: 1, tick: 1, reaction: 'steam_burst', enemy_uid: 1, damage: 10 }],
      },
    })
    expect(wrapper.find('.r15-reaction-timeline').exists()).toBe(false)
    expect(wrapper.find('.r16-reaction-timeline').text()).toContain('非权威结算')
    wrapper.unmount()
  })
})
