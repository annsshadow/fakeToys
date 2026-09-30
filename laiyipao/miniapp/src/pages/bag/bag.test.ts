// @vitest-environment jsdom
/**
 * 背包页（bag.vue）组件测试。
 * 覆盖：出战槽同步（服务端权威）、装备/移除/槽位满/被动技能/保存失败全分支、
 * clearSlot 的空槽早退与成功流、名称映射的回退分支（未知元素/家族/槽位）。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Bag from './bag.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig, makeSkill } from '../../test/fixtures'
import type { GameConfig } from '@/api/client'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

/** 技能库：普通 + 带穿透/溅射 + 被动 + 未知元素/家族（驱动名称映射回退分支） */
function bagConfig(): GameConfig {
  return makeConfig({
    skills: [
      makeSkill({ id: 1, name: '燃烧弹', element: 'fire' as any, family: 'flame' }),
      makeSkill({ id: 2, name: '冰霜弹', element: 'ice' as any, family: 'frost' }),
      makeSkill({ id: 3, name: '穿甲弹', element: 'kinetic' as any, family: 'kinetic', pierce: 2, aoe_radius: 0 }),
      makeSkill({ id: 4, name: '被动光环', element: 'light' as any, family: 'light', kind: 'passive' }),
      makeSkill({ id: 5, name: '奇异弹', element: 'bogus' as any, family: 'xyz' }),
    ],
    composite_skills: [makeSkill({ id: 101, name: '蒸汽弹', element: 'fire' as any, family: 'blight' })],
    equipment: [
      { id: 1, name: '焰纹长弓', element: 'fire', descr: '火系武器', slot: 'weapon', tier: 2 },
      { id: 2, name: '神秘戒', element: 'ice', descr: '未知槽位', slot: 'ring', tier: 1 },
    ],
    gems: [{ id: 1, name: '元素石', descr: '提高元素系数' }],
  })
}

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfig.mockResolvedValue(bagConfig())
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  mockApi.saveLoadout.mockImplementation((ids: number[]) =>
    Promise.resolve({ skill_ids: ids }),
  )
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 背包页', () => {
  it('挂载后同步服务端槽位并渲染技能库/装备/宝石（含名称映射与穿透/溅射标记）', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    // 槽位：1/2 已装备，3/4 空槽
    const slots = wrapper.findAll('.slot')
    expect(slots.length).toBe(4)
    expect(slots[0]!.text()).toContain('燃烧弹')
    expect(slots[2]!.text()).toContain('空槽')
    expect(slots[2]!.classes()).toContain('empty')
    // 技能库 = 基础 + 合成
    expect(wrapper.findAll('.skill-row').length).toBe(6)
    expect(wrapper.text()).toContain('蒸汽弹')
    // familyName 映射与回退
    expect(wrapper.text()).toContain('焰')
    expect(wrapper.text()).toContain('xyz') // 未知家族 → 原文
    // 穿透/溅射条件渲染：燃烧弹有溅射、穿甲弹无溅射（v-if 两侧分支）
    expect(wrapper.text()).toContain('穿透 2')
    expect(wrapper.text()).toContain('溅射 60')
    // 装备行：已知槽位映射 + 未知槽位回退
    expect(wrapper.text()).toContain('武器 · T2')
    expect(wrapper.text()).toContain('ring · T1')
    // 宝石
    expect(wrapper.text()).toContain('元素石')
  })

  it('装备一个未装备的技能：放进第一个空槽并保存成功', async () => {
    const { wrapper, store } = mountPage(Bag)
    await flushPromises()
    const rows = wrapper.findAll('.skill-row')
    const steamRow = rows.find((r) => r.text().includes('蒸汽弹'))!
    await steamRow.find('.link').trigger('click') // 装备
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledWith([1, 2, 101, 0])
    expect(store.loadout).toEqual([1, 2, 101, 0])
    // 保存成功后 selected 与服务端一致 → 槽 3 显示蒸汽弹
    expect(wrapper.findAll('.slot')[2]!.text()).toContain('蒸汽弹')
  })

  it('保存失败 → toast 提示且槽位不变', async () => {
    mockApi.saveLoadout.mockRejectedValue(new Error('保存失败'))
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    const steamRow = wrapper.findAll('.skill-row').find((r) => r.text().includes('蒸汽弹'))!
    await steamRow.find('.link').trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '保存失败，出战配置未变更', icon: 'none' })
    expect(wrapper.findAll('.slot')[2]!.text()).toContain('空槽')
  })

  it('被动技能不占主动槽 → toast 且不发请求', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    const passiveRow = wrapper.findAll('.skill-row').find((r) => r.text().includes('被动光环'))!
    await passiveRow.find('.link').trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '被动技能不占主动槽', icon: 'none' })
    expect(mockApi.saveLoadout).not.toHaveBeenCalled()
  })

  it('回归：仅第 1 槽装备（[1,0,0,0]）时槽位正确渲染且无渲染错误', async () => {
    // 此前模板误用 skillOf(i)（1 基）读 0 基数组，任何"已装备槽后面跟空槽"
    // 的存档都会让渲染崩溃、槽位永远卡在"空槽"。
    // 注意：onMounted 的 loadLoadout 会以服务端数据覆盖本地 loadout，
    // 所以必须通过 fetchLoadout mock 控制槽位形态。
    mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    const slots = wrapper.findAll('.slot')
    expect(slots[0]!.text()).toContain('燃烧弹')
    expect(slots[1]!.text()).toContain('空槽')
  })

  it('槽位满（UI 触发版）→ toast 槽位已满且不保存', async () => {
    mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2, 3, 101] })
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    // 4 个槽全满：所有已装备技能只显示"已装备"，其余技能显示"装备"
    const steamRow = wrapper.findAll('.skill-row').find((r) => r.text().includes('奇异弹'))!
    await steamRow.find('.link').trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '槽位已满，先移除一个', icon: 'none' })
    expect(mockApi.saveLoadout).not.toHaveBeenCalled()
  })

  it('toggleEquip 直呼已装备技能 → 走移除分支（existing >= 0）', async () => {
    const { wrapper, store } = mountPage(Bag)
    await flushPromises()
    await (wrapper.vm as any).toggleEquip(2) // 已在槽 2 → 移除
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledWith([1, 0, 0, 0])
    expect(store.loadout).toEqual([1, 0, 0, 0])
  })

  it('toggleEquip 未知技能 id → 直接返回（!s 分支）', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    await (wrapper.vm as any).toggleEquip(999)
    expect(mockApi.saveLoadout).not.toHaveBeenCalled()
  })

  it('槽位上的"移除"：清空对应槽并保存；空槽直呼 clearSlot 早退', async () => {
    const { wrapper, store } = mountPage(Bag)
    await flushPromises()
    await wrapper.findAll('.slot')[0]!.find('.dim').trigger('click') // 移除
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledWith([0, 2, 0, 0])
    expect(store.loadout).toEqual([0, 2, 0, 0])

    // 空槽 idx=2：next[idx] 为 0 → 早退（!next[idx] 分支），不发请求
    const calls = mockApi.saveLoadout.mock.calls.length
    await (wrapper.vm as any).clearSlot(2)
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(calls)
  })

  it('边界：skillOf 对空槽返回 undefined；onMounted 对短 loadout 补 0 到 4 槽', async () => {
    // loadLoadout 失败时保留 store 里那份异常短的 loadout（服务端旧数据/异常态），
    // 页面必须自己补齐到 4 槽
    mockApi.fetchLoadout.mockRejectedValue(new Error('loadout failed'))
    const { wrapper } = mountPage(Bag, (store) => {
      store.loadout = [1]
    })
    await flushPromises()
    const slots = wrapper.findAll('.slot')
    expect(slots.length).toBe(4)
    expect(slots[0]!.text()).toContain('燃烧弹')
    expect(slots[1]!.text()).toContain('空槽')
    // 空槽下标 → skillOf 早退分支（if (!id) return undefined）
    expect((wrapper.vm as any).skillOf(2)).toBeUndefined()
    expect((wrapper.vm as any).skillOf(0)!.name).toBe('燃烧弹')
  })

  it('clearSlot 保存失败 → toast 且槽位保持', async () => {
    mockApi.saveLoadout.mockRejectedValue(new Error('保存失败'))
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    await (wrapper.vm as any).clearSlot(0)
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '保存失败，出战配置未变更', icon: 'none' })
    expect(wrapper.findAll('.slot')[0]!.text()).toContain('燃烧弹')
  })
})
