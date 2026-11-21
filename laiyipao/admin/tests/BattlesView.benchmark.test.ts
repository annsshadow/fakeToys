// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * F-10-3 标杆重构契约（战报）：钉住「筛选动线分层 + 本页速览指标带 + 关键列强调」结构。
 * 退化回单一拥挤 toolbar、无速览指标、列全部等权时，这些断言失败——不是自述。
 */
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
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

const SRC = readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'views', 'BattlesView.vue'), 'utf8')

function battleFixture(overrides: Partial<AdminBattleView> = {}): AdminBattleView {
  return {
    id: 1, user_id: 9, nickname: '测试玩家', level_id: 3, result: 'win', stars: 3,
    score: 88000, kills: 10, leaked: 2, wave_reached: 5, duration_ms: 12345,
    reactions: 5, heat_max: 90, replay_hash: 'abcd1234abcd1234abcd1234abcd1234',
    created_at: '2026-01-02T03:04:05Z', total_enemies: 12,
    verify_checked: 3, verify_mismatched: 0, ...overrides,
  }
}

async function mountBattles(items: AdminBattleView[] = []) {
  fetchBattles.mockResolvedValue({ items, total: items.length })
  const router = createRouter({ history: createMemoryHistory(), routes: [
    { path: '/battles', component: { template: '<div />' } },
    { path: '/users', component: { template: '<div />' } },
  ] })
  await router.push('/battles')
  await router.isReady()
  const wrapper = mount(BattlesView, { global: { plugins: [ElementPlus, router] } })
  await flushPromises()
  return wrapper
}

function reactionSpec(key: string, name: string) {
  return { key, name, base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0,
    aoe_radius: 0, dispel_shield: false, amplify_pct: 0, descr: '' }
}

beforeEach(() => {
  vi.clearAllMocks()
  __resetReactionLabelCache()
  fetchReactions.mockResolvedValue([reactionSpec('steam_burst', '蒸汽爆发')])
})

describe('F-10-3 战报标杆重构契约', () => {
  it('筛选动线分层：主筛选行 + 次级筛选行，不再是单一拥挤 toolbar', async () => {
    const w = await mountBattles([battleFixture()])
    expect(w.find('.filter-bar').exists(), '应有 filter-bar 容器').toBe(true)
    expect(w.findAll('.filter-bar .filter-row').length, '应拆成 >=2 行筛选动线').toBeGreaterThanOrEqual(2)
    w.unmount()
  })

  it('本页速览指标带从当前页数据派生胜率/验真异常，不新增接口', async () => {
    const w = await mountBattles([
      battleFixture({ id: 1, result: 'win' }),
      battleFixture({ id: 2, result: 'lose' }),
      battleFixture({ id: 3, verify_mismatched: 2 }),
    ])
    const s = w.find('[data-test="battle-summary"]')
    expect(s.exists(), '应有本页速览指标带').toBe(true)
    expect(s.text()).toContain('本页胜率')
    expect(s.text()).toContain('验真异常')
    w.unmount()
  })

  it('关键列强调且不丢列：结果/验真/一致性带 emphasis，表格列与按钮不回退', async () => {
    const w = await mountBattles([battleFixture()])
    expect(SRC).toContain('col-emphasis')
    expect(w.findAll('.el-table__header th').length, '表格列不得减少').toBeGreaterThanOrEqual(15)
    expect(w.findAll('button').some((b) => b.text() === '筛选')).toBe(true)
    expect(w.findAll('button').some((b) => b.text().includes('导出 CSV'))).toBe(true)
    w.unmount()
  })
})
