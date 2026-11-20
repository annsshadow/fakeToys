// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 第 126 轮 laiyipao 真 bug（R276）：bag.vue 阵型预设区四个写入口缺少共享写守卫。
 *
 * doSavePreset / doApplyPreset / doDeletePreset / doSetCurrent 都写同一份 /me/presets
 * 列表（各自成功后 loadPresets() 整表覆盖），doApplyPreset 还顺带写 loadout/equipment/gems：
 *   - presetSaving 只防「保存」自身连点；
 *   - presetBusyId 只防「应用」自身连点；
 *   - doDeletePreset / doSetCurrent 完全无守卫。
 * 四个入口同时可见可点，没有 disabled。弱网下交叉点击（应用预设在途时点删除/设为当前，
 * 或保存预设在途时点应用），后完成的 loadPresets() 会用陈旧快照覆盖先完成的列表，
 * 玩家会看到「刚删的预设又回来了 / 应用没生效」。与 R275 防线保存/护盾共享守卫同构。
 * 本测试让首个写请求挂起，在途期间交叉点击另一入口，断言只发一次写请求。
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
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  upgradeSkill: vi.fn(),
  fetchGems: vi.fn(),
  washGem: vi.fn(),
  equipGem: vi.fn(),
  unequipGem: vi.fn(),
  fetchPresets: vi.fn(),
  saveCurrentPreset: vi.fn(),
  deletePreset: vi.fn(),
  setCurrentPreset: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  fetchMyEquipment: vi.fn(),
  saveEquippedEquipment: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

const preset = (over: Record<string, unknown> = {}) => ({
  id: 1,
  name: '套A',
  slots: { skills: [1], equipment: [], gems: [] },
  is_current: true,
  created_at: '2026-01-01T00:00:00Z',
  ...over,
})

function bagConfig(): GameConfig {
  return makeConfig({
    skills: [
      makeSkill({ id: 1, name: '燃烧弹', element: 'fire' as any, family: 'flame' }),
      makeSkill({ id: 2, name: '冰霜弹', element: 'ice' as any, family: 'frost' }),
    ],
    composite_skills: [],
    equipment: [],
    gems: [],
  })
}

/** 可控 Promise：首个写请求挂起，测试决定何时放行。 */
function deferred<T>() {
  let resolve!: (v: T) => void
  let reject!: (e: Error) => void
  const promise = new Promise<T>((res, rej) => { resolve = res; reject = rej })
  return { promise, resolve, reject }
}

let releaseSave: ((v: unknown) => void) | null = null
let releaseDelete: ((v: unknown) => void) | null = null
let releaseSetCurrent: ((v: unknown) => void) | null = null
let releaseApplyLoadout: ((v: unknown) => void) | null = null

beforeEach(() => {
  installUniMock()
  releaseSave = null
  releaseDelete = null
  releaseSetCurrent = null
  releaseApplyLoadout = null
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  } as any)
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 } as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} } as any)
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: { skills: { '1': { id: 1, level: 1, slot: -1 } } }, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] } as any)
  mockApi.fetchGems.mockResolvedValue({ gems: [] } as any)
  mockApi.fetchMyEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.fetchPresets.mockResolvedValue({
    presets: [preset(), preset({ id: 2, name: '套B', is_current: false })],
  } as any)
  // 首次 saveCurrentPreset 挂起，构造「保存预设在途」窗口。
  mockApi.saveCurrentPreset.mockImplementation((() => {
    const d = deferred<any>()
    releaseSave = d.resolve
    return d.promise
  }) as any)
  mockApi.deletePreset.mockImplementation((() => {
    const d = deferred<any>()
    releaseDelete = d.resolve
    return d.promise
  }) as any)
  mockApi.setCurrentPreset.mockImplementation((() => {
    const d = deferred<any>()
    releaseSetCurrent = d.resolve
    return d.promise
  }) as any)
  mockApi.saveLoadout.mockImplementation((() => {
    const d = deferred<any>()
    releaseApplyLoadout = d.resolve
    return d.promise
  }) as any)
  mockApi.saveEquippedEquipment.mockResolvedValue({ items: [] } as any)
  mockApi.equipGem.mockResolvedValue({} as any)
  mockApi.unequipGem.mockResolvedValue({} as any)
})

afterEach(async () => {
  releaseSave?.({ preset: preset() })
  releaseDelete?.({ ok: true })
  releaseSetCurrent?.({ preset: preset() })
  releaseApplyLoadout?.({ skill_ids: [1] })
  await flushPromises()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 阵型预设共享写守卫（R276）', () => {
  it('保存预设在途时点击「应用」不会交叉写技能槽', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    const input = wrapper.find('.preset-input')
    await input.setValue('我的套')
    await wrapper.find('.preset-save-row .link')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveCurrentPreset).toHaveBeenCalledTimes(1)

    // 在途期间点应用：应用会写 saveLoadout；有共享守卫时不应发请求
    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).not.toHaveBeenCalled()

    releaseSave?.({ preset: preset() })
    await flushPromises()
  })

  it('保存预设在途时点击「设为当前」不会交叉提交', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    const input = wrapper.find('.preset-input')
    await input.setValue('我的套')
    await wrapper.find('.preset-save-row .link')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveCurrentPreset).toHaveBeenCalledTimes(1)

    const rows = wrapper.findAll('.preset-item')
    const setCurrentBtn = rows[1]!.find('.preset-current')
    expect(setCurrentBtn.exists()).toBe(true)
    await setCurrentBtn!.trigger('click')
    await flushPromises()
    expect(mockApi.setCurrentPreset).not.toHaveBeenCalled()

    releaseSave?.({ preset: preset() })
    await flushPromises()
  })

  it('删除预设在途时重复删除/交叉设为当前不会重复提交', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    const rows = wrapper.findAll('.preset-item')
    await rows[1]!.find('.preset-delete')!.trigger('click')
    await flushPromises()
    expect(mockApi.deletePreset).toHaveBeenCalledTimes(1)
    expect(releaseDelete).not.toBeNull()

    // 在途期间再点删除另一预设 + 设为当前（套 B 非当前，有设为当前按钮）：都应被共享守卫挡住
    await rows[0]!.find('.preset-delete')!.trigger('click')
    await rows[1]!.find('.preset-current')!.trigger('click')
    await flushPromises()
    expect(mockApi.deletePreset).toHaveBeenCalledTimes(1)
    expect(mockApi.setCurrentPreset).not.toHaveBeenCalled()

    releaseDelete?.({ ok: true })
    await flushPromises()
  })

  it('应用预设在途时点击「设为当前」不会交叉提交当前标记', async () => {
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    await wrapper.find('.preset-apply')!.trigger('click')
    await flushPromises()
    expect(mockApi.saveLoadout).toHaveBeenCalledTimes(1)
    expect(releaseApplyLoadout).not.toBeNull()

    // 应用在途（saveLoadout 挂起）期间点设为当前：应被共享守卫挡住
    const rows = wrapper.findAll('.preset-item')
    const setCurrentBtn = rows[1]!.find('.preset-current')
    expect(setCurrentBtn.exists()).toBe(true)
    await setCurrentBtn!.trigger('click')
    await flushPromises()
    expect(mockApi.setCurrentPreset).not.toHaveBeenCalled()

    releaseApplyLoadout?.({ skill_ids: [1] })
    await flushPromises()
  })
})
