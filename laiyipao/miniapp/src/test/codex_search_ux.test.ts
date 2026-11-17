// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 图鉴页搜索结果反馈（100 轮打磨 R23–R28）。
 *
 * 六个图鉴页（敌人/装备/技能/皮肤/宝石/反应）此前搜索无结果时**整块空白** ——
 * 页面只判断「全量表是否为空」，没判断「当前过滤后是否为空」。
 * 本批统一补：结果计数 + 无结果引导（含一键清除）。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Enemies from '@/pages/enemies/enemies.vue'
import Equipment from '@/pages/equipment/equipment.vue'
import Skills from '@/pages/skills/skills.vue'
import Skins from '@/pages/skins/skins.vue'
import Gems from '@/pages/gems/gems.vue'
import Reactions from '@/pages/reactions/reactions.vue'
import { makeConfig } from './fixtures'
import { installUniMock } from './uni-mock'
import { mountPage } from './page'

// gems 页 onShow 直接 import fetchGems；这里给它一个 never-resolve 之外的空结果，
// 让测试只关心图鉴过滤行为（拥有状态是辅助信息）。
vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  fetchGems: vi.fn(() => Promise.resolve({ gems: [] })),
}))

const cfg = () => makeConfig({
  enemies: [
    { id: 1, name: '史莱姆', category: 'normal', descr: '', hp: 10, speed: 1000, armor: 0, shield_hp: 0, attack: 1, attack_interval: 0, fly_height: 0, burrow: false, resist: {} },
    { id: 2, name: '飞龙', category: 'flying', descr: '', hp: 20, speed: 1200, armor: 0, shield_hp: 0, attack: 2, attack_interval: 0, fly_height: 0, burrow: false, resist: {} },
  ],
  equipment: [
    { id: 1, name: '焰纹长弓', code: 'bow', slot: 'weapon', tier: 2, element: 'fire', descr: '', base_armor: 0, base_bonus_pct: 10, unlock_level: 1 },
    { id: 2, name: '寒霜护甲', code: 'armor', slot: 'body', tier: 2, element: 'ice', descr: '', base_armor: 5, base_bonus_pct: 0, unlock_level: 1 },
  ],
  skills: [
    { id: 1, code: 'fire', name: '燃烧弹', element: 'fire', family: 'fire', kind: 'active', descr: '', base_damage: 10, heat_cost: 10, cooldown_ms: 1000 },
    { id: 2, code: 'ice', name: '冰霜弹', element: 'ice', family: 'frost', kind: 'active', descr: '', base_damage: 8, heat_cost: 8, cooldown_ms: 1000 },
  ],
  skins: [
    { id: 1, name: '烈焰战甲', rarity: 'rare', descr: '', passive: '' },
    { id: 2, name: '寒冰战甲', rarity: 'epic', descr: '', passive: '' },
  ],
  gems: [
    { id: 1, name: '元素石', descr: '', attr: 'element_coef' },
    { id: 2, name: '暴击石', descr: '', attr: 'crit' },
  ],
  reactions: [
    { key: 'steam_burst', name: '蒸汽爆发', base_coef: 60, attack_weight_pct: 300, status_duration_ms: 0, aoe_radius: 0, spread_stacks: 0, dispel_shield: false, amplify_pct: 0, armor_shred_permille: 0, knockback: 0, descr: '' },
    { key: 'overload', name: '过载', base_coef: 40, attack_weight_pct: 200, status_duration_ms: 0, aoe_radius: 0, spread_stacks: 0, dispel_shield: false, amplify_pct: 0, armor_shred_permille: 0, knockback: 0, descr: '' },
  ],
} as any)

beforeEach(() => { installUniMock() })
afterEach(() => { vi.unstubAllGlobals() })

/** 输入关键词并等待渲染。 */
async function search(wrapper: any, kw: string) {
  const input = wrapper.find('input')
  ;(input.element as HTMLInputElement).value = kw
  await input.trigger('input')
  await flushPromises()
}

interface Case {
  name: string
  component: unknown
  label: string
  rows: string
}

const cases: Case[] = [
  { name: 'R23 敌人图鉴', component: Enemies, label: '敌人', rows: '.rxn.enemy' },
  { name: 'R24 装备图鉴', component: Equipment, label: '装备', rows: '.rxn.equip' },
  { name: 'R25 技能图鉴', component: Skills, label: '技能', rows: '.rxn.skill' },
  { name: 'R26 皮肤图鉴', component: Skins, label: '皮肤', rows: '.rxn.skin' },
  { name: 'R27 宝石图鉴', component: Gems, label: '宝石', rows: '.rxn.gem' },
  { name: 'R28 反应图鉴', component: Reactions, label: '反应', rows: '.rxn' },
]

describe.each(cases)('$name 搜索反馈', ({ component, label, rows }) => {
  it('结果计数 + 无结果引导 + 一键清除', async () => {
    const { wrapper } = mountPage(component, (store) => {
      store.config = cfg()
      store.loggedIn = true // 跳过 reactions 的自动登录 / 让 gems 的 onShow 走 fetch 分支
    })
    await flushPromises()

    // 初始：两条都显示，计数正确
    expect(wrapper.findAll(rows).length).toBe(2)
    expect(wrapper.find('.result-count').text()).toContain('显示 2 / 共 2 条')
    expect(wrapper.find('.search-empty').exists()).toBe(false)

    // 输入无匹配关键词 → 行消失，但给出明确引导而非空白
    await search(wrapper, '不存在的名字XYZ')
    expect(wrapper.findAll(rows).length).toBe(0)
    const empty = wrapper.find('.search-empty')
    expect(empty.exists()).toBe(true)
    expect(empty.text()).toContain(`没有匹配「不存在的名字XYZ」的${label}`)
    expect(empty.find('.clear-search').exists()).toBe(true)

    // 点清除 → 搜索框清空、列表恢复
    await empty.find('.clear-search').trigger('click')
    await flushPromises()
    expect((wrapper.find('input').element as HTMLInputElement).value).toBe('')
    expect(wrapper.findAll(rows).length).toBe(2)
    expect(wrapper.find('.search-empty').exists()).toBe(false)
  })
})
