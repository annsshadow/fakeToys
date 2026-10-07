// @vitest-environment jsdom
/**
 * 我的页（me.vue）组件测试。
 * 覆盖：防线加载（mine/candidates/今日余量/护盾态）、保存防线与护盾开关、
 * 挑战流程的全部守卫（不可挑战 / 无关卡 / 无出战技能）与成功 / 上报失败分支、
 * 快照校验网关（snapshotOk 阻断挑战）、窃取展示文案。
 *
 * runChallenge / validateSnapshot（@/game/defense）是重型本地模拟，
 * 这里 mock 掉只测页面自身逻辑；error 形态的 outcome 与真实实现一致
 * （fail() 恒带 report 与 stats）。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Me from './me.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel, makeSkill } from '../../test/fixtures'
import type { ChallengeOutcome, DefenseView } from '@/game/defense'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  fetchDefenses: vi.fn(),
  saveDefense: vi.fn(),
  challengeDefense: vi.fn(),
}))

vi.mock('@/game/defense', () => ({
  runChallenge: vi.fn(),
  validateSnapshot: vi.fn((c: DefenseView) => ({ ok: !String(c.snapshot_hash).startsWith('bad') })),
}))

const mockApi = vi.mocked(api, true)
const defenseModule = await import('@/game/defense')
const runChallengeMock = vi.mocked(defenseModule.runChallenge)

let um: ReturnType<typeof installUniMock>

function defenseView(over: Partial<DefenseView> = {}): DefenseView {
  return {
    id: 5,
    name: '我的防线',
    owner_name: '对手',
    power: 100,
    element_coverage: 3,
    wins: 1,
    losses: 2,
    snapshot_hash: 'abcdef1234567890',
    snapshot: { skills: [1] },
    can_challenge: true,
    challenge_blocked: '',
    ...over,
  } as DefenseView
}

/** 与真实 runChallenge 的 fail() 同构：error 结果恒带 report 与 stats */
function outcome(over: Partial<ChallengeOutcome> = {}): ChallengeOutcome {
  return {
    won: true,
    report: { seed: '42', won: true, duration_ms: 1000, hp_left_pct: 0, replay_hash: 'aaaabbbb' },
    stats: { kills: 3, leaked: 0, waves: 2, score: 100, durationMs: 1000, hpLeftPct: 0 },
    ...over,
  } as ChallengeOutcome
}

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfig.mockResolvedValue(
    makeConfig({ levels: [makeLevel()], skills: [makeSkill({ id: 1, apply_element: '' })] }),
  )
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] })
  mockApi.fetchDefenses.mockResolvedValue({
    mine: defenseView({ shielded_until: '2026-01-02', my_attempts_today: 2 } as any),
    candidates: [defenseView()],
    attempt_limit: 5,
  } as any)
  mockApi.saveDefense.mockResolvedValue({ defense: defenseView({ snapshot_hash: 'feedface1234' }) } as any)
  mockApi.challengeDefense.mockResolvedValue({ stolen: { coin: 10, gem: 2, keys: 1, energy: 3, vip: 9 } })
  runChallengeMock.mockReturnValue(outcome())
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('me.vue 我的页', () => {
  it('挂载后加载防线：渲染我的防线、候选列表与今日剩余次数', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()
    expect(wrapper.text()).toContain('我的防线')
    expect(wrapper.text()).toContain('3/5') // 元素覆盖
    expect(wrapper.text()).toContain('1 胜 / 2 负')
    expect(wrapper.text()).toContain('可挑战的防线')
    expect(wrapper.text()).toContain('今日剩余 3 次') // attempt_limit 5 - my_attempts_today 2
    expect(wrapper.text()).toContain('关闭 24h 护盾') // shielded_until 有值
  })

  it('mine 为空 → "尚未设置防线"；快照不完整的候选不给挑战入口', async () => {
    mockApi.fetchDefenses.mockResolvedValue({
      mine: null,
      candidates: [defenseView({ snapshot_hash: 'badbadbadbad' })],
      attempt_limit: 3,
    } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    expect(wrapper.text()).toContain('尚未设置防线')
    expect(wrapper.text()).not.toContain('开启 24h 护盾')
    expect(wrapper.text()).toContain('快照不完整，无法模拟') // snapshotOk=false 的提示
  })

  it('今日余量下限夹 0（attempt_limit < 已用次数）', async () => {
    mockApi.fetchDefenses.mockResolvedValue({
      mine: defenseView({ my_attempts_today: 5 } as any),
      candidates: [defenseView()],
      attempt_limit: 2,
    } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    expect(wrapper.text()).toContain('今日剩余 0 次') // max(0, 2-5)
  })

  it('防线接口整体失败 → 静默清空（不白屏）；候选里不可挑战者显示回退文案', async () => {
    mockApi.fetchDefenses.mockRejectedValueOnce(new Error('防线接口挂了'))
    const { wrapper } = mountPage(Me)
    await flushPromises()
    expect(wrapper.text()).toContain('尚未设置防线')
    expect(wrapper.text()).not.toContain('可挑战的防线')

    // 恢复后返回带不可挑战候选的数据：blocked 缺省 → "暂时无法挑战"
    mockApi.fetchDefenses.mockResolvedValue({
      mine: null,
      candidates: [defenseView({ can_challenge: false, challenge_blocked: '' })],
      attempt_limit: 3,
    } as any)
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(wrapper.text()).toContain('暂时无法挑战')
  })

  it('护盾开关失败 → toast 错误信息', async () => {
    // 未开盾状态才有"开启 24h 护盾"按钮
    mockApi.fetchDefenses.mockResolvedValue({
      mine: defenseView({ shielded_until: undefined }),
      candidates: [],
      attempt_limit: 3,
    } as any)
    mockApi.saveDefense.mockRejectedValue(new Error('护盾保存失败'))
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const shieldBtn = wrapper.findAll('.btn').find((b) => b.text().includes('开启 24h 护盾'))!
    await shieldBtn.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '护盾保存失败', icon: 'none' })
    // 状态未翻转
    expect(wrapper.text()).toContain('开启 24h 护盾')
  })

  it('保存防线成功：上报固定 works、toast 快照哈希并刷新列表', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const saveBtn = wrapper.findAll('.btn').find((b) => b.text().includes('保存当前构筑为防线'))!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledWith(
      expect.objectContaining({ name: '我的防线', works: ['slow_belt', 'block_wall', 'tesla_grid'], shield_hours: 0 }),
    )
    expect(um.mock.showToast).toHaveBeenCalledWith(
      expect.objectContaining({ title: expect.stringContaining('feedface') }),
    )
  })

  it('第 131 轮：点「保存防线」上报的 skills == 服务端槽位（非本地缺省 [1,2,3]）', async () => {
    mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [4, 0, 0, 0] } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const saveBtn = wrapper.findAll('.btn').find((b) => b.text().includes('保存当前构筑为防线'))!
    await saveBtn.trigger('click')
    await flushPromises()
    // 修前这里会是本地缺省 [1,2,3]（技能 2/3 玩家根本没装）；修后必须是服务端 [4]
    expect(mockApi.saveDefense).toHaveBeenCalledWith(
      expect.objectContaining({ skills: [4] }),
    )
  })

  it('保存防线失败 → toast 错误信息', async () => {
    mockApi.saveDefense.mockRejectedValue(new Error('保存失败'))
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const saveBtn = wrapper.findAll('.btn').find((b) => b.text().includes('保存当前构筑为防线'))!
    await saveBtn.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '保存失败', icon: 'none' })
  })

  it('护盾开关：未开 → shield_hours 24，成功后按钮翻转为关闭', async () => {
    mockApi.fetchDefenses.mockResolvedValue({
      mine: defenseView({ shielded_until: undefined }),
      candidates: [],
      attempt_limit: 3,
      mine_attempts_today: 0,
    } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const shieldBtn = wrapper.findAll('.btn').find((b) => b.text().includes('开启 24h 护盾'))!
    await shieldBtn.trigger('click')
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenCalledWith(
      expect.objectContaining({ shield_hours: 24 }),
    )
    expect(wrapper.text()).toContain('关闭 24h 护盾')
  })

  it('挑战成功：本地模拟后上报，展示战报与窃取资源、刷新钱包与防线', async () => {
    const { wrapper, store } = mountPage(Me)
    await flushPromises()
    await wrapper.findAll('.cand .btn')[0]!.trigger('click')
    await new Promise((r) => setTimeout(r, 60)) // setTimeout(30) 让按钮先进"模拟中"
    await flushPromises()
    expect(runChallengeMock).toHaveBeenCalledWith(
      expect.objectContaining({ id: 5 }),
      expect.objectContaining({ level: expect.objectContaining({ id: 1 }), myEquipped: expect.anything() }),
    )
    expect(mockApi.challengeDefense).toHaveBeenCalledWith(5, {
      seed: '42', // 第 132 轮：字符串上报（修前是 Number('42')=42）
      won: true,
      duration_ms: 1000,
      hp_left_pct: 0,
      replay_hash: 'aaaabbbb',
    })
    expect(wrapper.text()).toContain('攻破防线')
    expect(wrapper.text()).toContain('金币 10 · 钻石 2 · 钥匙 1 · 体力 3 · vip 9')
    expect(store.wallet.coin).toBe(1) // refreshWallet 生效
  })

  it('第 132 轮：大种子按字符串上报，不被 Number() 截断', async () => {
    // 引擎产出的 63-bit 种子（> 2^53）必须以字符串原样上报。
    const bigSeed = '9007199254740993' // 2^53+1，Number() 会舍成 ...992
    runChallengeMock.mockReturnValue(
      outcome({ report: { seed: bigSeed, won: true, duration_ms: 1000, hp_left_pct: 0, replay_hash: 'fff' } as any }),
    )
    mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1] } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    await wrapper.findAll('.cand .btn')[0]!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(mockApi.challengeDefense).toHaveBeenCalledWith(
      5,
      expect.objectContaining({ seed: bigSeed }), // 字符串，且是精确值
    )
  })

  it('模拟报错（outcome.error）→ 只展示战报不上报', async () => {
    runChallengeMock.mockReturnValue(
      outcome({ won: false, error: '对方护盾尚未过期' }),
    )
    const { wrapper } = mountPage(Me)
    await flushPromises()
    await wrapper.findAll('.cand .btn')[0]!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(mockApi.challengeDefense).not.toHaveBeenCalled()
    expect(wrapper.text()).toContain('对方护盾尚未过期')
    expect(wrapper.text()).toContain('未能攻破')
  })

  it('上报失败 → 页面兜底构造失败战报并展示错误', async () => {
    mockApi.challengeDefense.mockRejectedValue(new Error('上报失败'))
    const { wrapper } = mountPage(Me)
    await flushPromises()
    await wrapper.findAll('.cand .btn')[0]!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).toContain('未能攻破')
    expect(wrapper.text()).toContain('上报失败')
  })

  it('挑战守卫：无出战技能 / 无关卡数据 / 不可挑战的候选', async () => {
    // 第 131 轮：「无出战技能」现在由**服务端**返回空槽位来模拟 ——
    // me.vue 挂载时会 loadLoadout()，把 equippedSkillIds 同步成服务端值。
    // 若服务端返回 [4]，本地就不会是空，守卫不会触发。
    mockApi.fetchLoadout.mockResolvedValue({} as any) // 空槽位 → equippedSkillIds=[]
    const { wrapper, store } = mountPage(Me, (s) => {
      s.equippedSkillIds = []
    })
    await flushPromises()
    const btn = () => wrapper.findAll('.cand .btn')[0]!

    // 无出战技能（关卡已加载）
    await btn().trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '请先到「背包」页装备出战技能', icon: 'none' })

    // 无关卡数据：config 置空后技能也不为空 → 先撞"关卡未加载"守卫
    store.config = null
    store.equippedSkillIds = [1]
    await btn().trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '关卡数据未加载完成', icon: 'none' })

    // can_challenge=false → challenge_blocked 文案（按钮态由 disabled 样式承担，逻辑仍守卫）
    ;(wrapper.vm as any).challenge(defenseView({ can_challenge: false, challenge_blocked: '今天已挑战过' }))
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '今天已挑战过', icon: 'none' })

    // challenge_blocked 缺省 → 回退"暂时无法挑战"
    ;(wrapper.vm as any).challenge(defenseView({ can_challenge: false, challenge_blocked: '' }))
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '暂时无法挑战', icon: 'none' })
  })

  it('展示回退分支：快照缺失 / 哈希缺失 / 微信账号 / 非零 maxStage', async () => {
    mockApi.fetchDefenses.mockResolvedValue({
      mine: null,
      candidates: [defenseView({ owner_name: '', snapshot: undefined, snapshot_hash: '' })],
      attempt_limit: 3,
    } as any)
    // 预置已登录，避免 onMounted 的 login 用游客态覆盖 isGuest
    const { wrapper } = mountPage(Me, (st) => {
      st.loggedIn = true
      st.isGuest = false
      st.maxStage = 7
      st.buildRating = { total: 88 }
    })
    await flushPromises()
    expect(wrapper.text()).toContain('微信账号')
    expect(wrapper.text()).toContain('第 7 关')
    expect(wrapper.text()).toContain('88') // buildRating?.total 有值 → 走 ?? 左侧
    expect(wrapper.text()).toContain('0 技能') // snapshot?.skills?.length ?? 0
    expect(wrapper.text()).toContain('—') // 空 snapshot_hash 的占位
    expect(wrapper.text()).toContain('玩家 的防线') // owner_name 空 → 回退
  })

  it('防线响应缺 attempt_limit / mine → 均按缺省值计算', async () => {
    mockApi.fetchDefenses.mockResolvedValue({} as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    expect(wrapper.text()).toContain('尚未设置防线') // mine 缺省 → null
  })

  it('toggleShield 在无防线时：名称回退 + 未开盾 24h / 已开盾 0h 两侧', async () => {
    mockApi.fetchDefenses.mockResolvedValue({ mine: null, candidates: [], attempt_limit: 3 } as any)
    const { wrapper } = mountPage(Me)
    await flushPromises()
    // 第一次：myDefense null → 名称回退；shielded=false → shield_hours 24
    ;(wrapper.vm as any).toggleShield()
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenLastCalledWith(
      expect.objectContaining({ shield_hours: 24, name: '我的防线' }),
    )
    // 第二次：上一轮成功后 shielded 已翻转为 true → shield_hours 0
    ;(wrapper.vm as any).toggleShield()
    await flushPromises()
    expect(mockApi.saveDefense).toHaveBeenLastCalledWith(
      expect.objectContaining({ shield_hours: 0 }),
    )
  })

  it('challengeLevel 三级回退：levelMap 未命中 → levels[0] → null', async () => {
    const { wrapper, store } = mountPage(Me)
    await flushPromises()
    const btn = () => wrapper.findAll('.cand .btn')[0]!

    // levelMap 无 1 号关 → 回退 config.levels[0]（id=2）。
    // 直接替换 store.config（onShow 不重新 loadConfig）
    store.config = makeConfig({ levels: [makeLevel({ id: 2 })] })
    await flushPromises()
    await btn().trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(runChallengeMock).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ level: expect.objectContaining({ id: 2 }) }),
    )

    // config 无 levels → null → toast 守卫
    store.config = {} as any
    await btn().trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '关卡数据未加载完成', icon: 'none' })
  })

  it('上报响应缺 stolen → 窃取文案为空', async () => {
    mockApi.challengeDefense.mockResolvedValue({})
    const { wrapper } = mountPage(Me)
    await flushPromises()
    await wrapper.findAll('.cand .btn')[0]!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).not.toContain('窃取成功')
  })

  it('onShow（已登录）→ 刷新档案与防线', async () => {
    const { wrapper } = mountPage(Me)
    await flushPromises()
    const calls = mockApi.fetchDefenses.mock.calls.length
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchDefenses).toHaveBeenCalledTimes(calls + 1)
    expect(mockApi.fetchWallet).toHaveBeenCalled()
  })
})
