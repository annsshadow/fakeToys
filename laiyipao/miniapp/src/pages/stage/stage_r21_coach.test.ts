// @vitest-environment jsdom
/**
 * R21 H5：选关页个人卡点教练卡（stage.vue .r21-level-coach）。
 * 消费 GET /me/level-coach?window_days=30 的真实响应，
 * 覆盖：逐行渲染+降序+data-r21-level / AI四判据 / 窗口 / 诚实红线 /
 *       逐字空态 / 非有限数兜底。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Stage from './stage.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchLevel: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  diagnose: vi.fn(),
  fetchMyStars: vi.fn(),
  fetchMyLevelCoach: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

function coachResp(overrides: Record<string, unknown> = {}) {
  return {
    window_days: 30,
    window_from: '2026-10-25',
    window_to: '2026-11-24',
    items: [
      {
        level_id: 2, attempts: 9, wins: 2, best_wave: 5, avg_wave: 3.5,
        last_result: 'lose', stuck: true,
        coach: { title: '先补对空', detail: '连续失利 3 次', suggestions: ['带远程技能'], confidence: 80 },
      },
      {
        level_id: 1, attempts: 4, wins: 4, best_wave: 5, avg_wave: 5,
        last_result: 'win', stuck: false,
        coach: { title: '已稳定通关', detail: '可挑战下一关', suggestions: ['尝试新元素'], confidence: 60 },
      },
    ],
    ...overrides,
  }
}

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({ levels: [makeLevel({ id: 1, chapter: 1 }), makeLevel({ id: 2, chapter: 1 })] }),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.diagnose.mockResolvedValue({
    stage: 'no_element', confidence: 0, title: '', detail: '', suggestions: [],
    failed_times: 0, stuck: false,
  })
  mockApi.fetchMyStars.mockResolvedValue({ stars: {} })
  mockApi.fetchMyLevelCoach.mockResolvedValue(coachResp() as never)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

async function open() {
  const { wrapper } = mountPage(Stage)
  await flushPromises()
  triggerUniHook(wrapper.vm, 'onLoad', {})
  await flushPromises()
  return wrapper
}

describe('stage.vue R21 个人卡点教练卡', () => {
  it('渲染 .r21-level-coach：真实数据按尝试次数降序，每行带 data-r21-level / 次数 / 胜场 / 最好波次 / 平均波次一位小数', async () => {
    const wrapper = await open()
    const card = wrapper.find('.r21-level-coach')
    expect(card.exists(), '缺少 .r21-level-coach').toBe(true)
    expect(card.attributes('role')).toBe('region')
    const rows = card.findAll('.r21-coach-row')
    expect(rows.length).toBe(2)
    expect(rows[0]!.attributes('data-r21-level')).toBe('2')
    expect(rows[1]!.attributes('data-r21-level')).toBe('1')
    const t0 = rows[0]!.text()
    expect(t0).toContain('第 2 关')
    expect(t0).toContain('尝试 9 次')
    expect(t0).toContain('胜 2 次')
    expect(t0).toContain('最好第 5 波')
    expect(t0).toContain('3.5')
    expect(rows[0]!.text()).toContain('卡点') // stuck=true 标注
    expect(rows[1]!.text()).not.toContain('卡点')
    // 窗口标注来自真实响应
    expect(card.text()).toContain('2026-10-25')
    expect(card.text()).toContain('2026-11-24')
    expect(card.text()).toContain('30 天')
    expect(mockApi.fetchMyLevelCoach).toHaveBeenCalledTimes(1)
    wrapper.unmount()
  })

  it('AI四判据：AI标题 + 真实数据结论 + 逐字依据行 + 去补强 CTA', async () => {
    const wrapper = await open()
    const note = wrapper.find('.r21-ai-note')
    expect(note.exists(), '缺少 .r21-ai-note').toBe(true)
    const text = note.text()
    expect(text).toMatch(/AI/)
    expect(text).toContain('近30天')
    expect(text).toContain('尝试 9 次') // 真实 attempts
    expect(text).toContain('胜 2 次')
    expect(text).toContain('最好到第 5 波')
    expect(text).toContain('依据：近 30 天 battle_records 的 level_id/result/wave_reached 聚合；连续失利沿用 StuckThreshold=2')
    const cta = note.find('.r21-coach-cta')
    expect(cta.exists(), '缺少 CTA').toBe(true)
    expect(cta.text()).toContain('去补强')
    wrapper.unmount()
  })

  it('诚实红线：辅助诊断非权威、不参与结算/score/星级/loot，权威看 replay_hash', async () => {
    const wrapper = await open()
    const card = wrapper.find('.r21-level-coach')
    expect(card.text()).toContain('辅助诊断')
    expect(card.text()).toContain('replay_hash')
    expect(card.text()).toContain('不参与结算')
    wrapper.unmount()
  })

  it('空态：items=[] 或请求失败逐字「近 30 天无关卡尝试记录」，不显示 undefined/NaN', async () => {
    mockApi.fetchMyLevelCoach.mockResolvedValue(coachResp({ items: [] }) as never)
    let wrapper = await open()
    expect(wrapper.find('.r21-coach-empty').exists()).toBe(true)
    expect(wrapper.find('.r21-coach-empty').text()).toBe('近 30 天无关卡尝试记录')
    expect(wrapper.text()).not.toContain('undefined')
    expect(wrapper.text()).not.toContain('NaN')
    wrapper.unmount()

    mockApi.fetchMyLevelCoach.mockRejectedValue(new Error('网络异常'))
    wrapper = await open()
    expect(wrapper.find('.r21-coach-empty').text()).toBe('近 30 天无关卡尝试记录')
    expect(um.mock.showToast).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('avg_wave 非有限/分母0 → 0.0，不渲染 NaN 或裸长浮点', async () => {
    mockApi.fetchMyLevelCoach.mockResolvedValue(coachResp({
      items: [{
        level_id: 2, attempts: 1, wins: 0, best_wave: 0, avg_wave: Number.NaN,
        last_result: 'lose', stuck: false,
        coach: { title: '', detail: '', suggestions: [], confidence: 0 },
      }],
    }) as never)
    const wrapper = await open()
    const row = wrapper.find('.r21-coach-row')
    expect(row.text()).toContain('0.0')
    expect(row.text()).not.toContain('NaN')
    wrapper.unmount()
  })
})
