/**
 * 全局 store（Pinia）单元测试。
 *
 * mock 掉 @/api/client 这个边界（网络不可达也不影响逻辑验证），
 * 覆盖：登录全流程与单飞、配置加载缓存、钱包/档案刷新的容错、
 * 槽位归一化（服务端权威）、attacker 的服务端优先策略、各 computed 分支。
 *
 * ⚠️ attacker 必须来自服务端 build.attacker —— 本地推算会让回放哈希
 * 与服务端不一致（I-6 验真误判），所以这里的断言就是给这个约束上锁。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useGameStore } from './game'
import * as api from '@/api/client'
import { defaultAttacker } from '@/game/damage'
import { makeConfig } from '../test/fixtures'

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

let store: ReturnType<typeof useGameStore>
let warnSpy: ReturnType<typeof vi.spyOn>

beforeEach(() => {
  setActivePinia(createPinia())
  store = useGameStore()
  warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {})

  mockApi.guestLogin.mockResolvedValue({
    access_token: 'acc-1',
    refresh_token: 'ref-1',
    expires_at: '',
    user: { id: 7, nickname: '游客7', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfig.mockResolvedValue(makeConfig())
  mockApi.fetchWallet.mockResolvedValue({ coin: 10, gem: 20, energy: 30, keys: 40 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 123 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  mockApi.saveLoadout.mockResolvedValue({ skill_ids: [1, 2] })
})

afterEach(() => {
  warnSpy.mockRestore()
  vi.clearAllMocks()
})

describe('login（单飞 + 全流程）', () => {
  it('成功：写令牌、账号态置位、拉配置与档案', async () => {
    const ok = await store.login()
    expect(ok).toBe(true)
    expect(mockApi.setTokenInternal).toHaveBeenCalledWith('acc-1', 'ref-1')
    expect(store.userId).toBe(7)
    expect(store.nickname).toBe('游客7')
    expect(store.isGuest).toBe(true)
    expect(store.loggedIn).toBe(true)
    expect(store.loginError).toBe('')
    expect(mockApi.fetchConfig).toHaveBeenCalledTimes(1)
    expect(mockApi.fetchWallet).toHaveBeenCalledTimes(1)
    expect(mockApi.fetchMe).toHaveBeenCalledTimes(1)
    expect(store.config).not.toBeNull()
    expect(store.power).toBe(123)
  })

  it('失败：loginError 记录信息、loggedIn false、不再拉配置', async () => {
    mockApi.guestLogin.mockRejectedValue(new Error('无法连接服务器'))
    const ok = await store.login()
    expect(ok).toBe(false)
    expect(store.loggedIn).toBe(false)
    expect(store.loginError).toBe('无法连接服务器')
    expect(mockApi.fetchConfig).not.toHaveBeenCalled()
  })

  it('并发调用单飞：onLoad 与 onMounted 同时触发也只登录一次', async () => {
    let resolve!: (v: any) => void
    mockApi.guestLogin.mockImplementation(
      () => new Promise((res) => { resolve = res }),
    )
    const p1 = store.login()
    const p2 = store.login()
    resolve({
      access_token: 'a',
      refresh_token: 'r',
      user: { id: 1, nickname: 'n', avatar_url: '', is_guest: true, status: 1 },
    })
    const [r1, r2] = await Promise.all([p1, p2])
    expect(r1).toBe(true)
    expect(r2).toBe(true)
    // 两个游客账号 / 两份 token 互相覆盖是本分支要防的实际事故
    expect(mockApi.guestLogin).toHaveBeenCalledTimes(1)
  })

  it('失败后可重新登录（in-flight 已清理，loginError 先重置）', async () => {
    mockApi.guestLogin.mockRejectedValueOnce(new Error('网络抖动'))
    expect(await store.login()).toBe(false)
    expect(await store.login()).toBe(true)
    expect(store.loginError).toBe('')
    expect(mockApi.guestLogin).toHaveBeenCalledTimes(2)
  })
})

describe('loadConfig（缓存 + loading 态）', () => {
  it('已加载后直接返回，不重复请求（if (config.value) 分支）', async () => {
    await store.login()
    await store.loadConfig()
    expect(mockApi.fetchConfig).toHaveBeenCalledTimes(1)
  })

  it('请求期间 configLoading 为 true，结束后（无论成败）复位', async () => {
    let resolve!: (v: any) => void
    mockApi.fetchConfig.mockImplementation(() => new Promise((res) => { resolve = res }))
    const p = store.loadConfig()
    expect(store.configLoading).toBe(true)
    resolve(makeConfig())
    await p
    expect(store.configLoading).toBe(false)
  })

  it('加载失败：异常向上抛（调用方决定怎么提示），configLoading 仍复位', async () => {
    mockApi.fetchConfig.mockRejectedValue(new Error('config failed'))
    await expect(store.loadConfig()).rejects.toThrow('config failed')
    expect(store.configLoading).toBe(false)
  })
})

describe('服务端时钟基准 estimateServerNowMs（第 140 轮）', () => {
  it('按 config 里的 server_time 抵消本地时钟偏移（本地 2026-09 对服务端 2026-05）', async () => {
    vi.useFakeTimers()
    try {
      vi.setSystemTime(Date.parse('2026-09-01T00:00:00Z')) // 本地时钟
      mockApi.fetchConfig.mockResolvedValue({
        ...makeConfig(),
        server_time: '2026-05-01T00:00:00Z', // 服务端时钟比本地早 4 个月
      })
      await store.loadConfig()
      const est = store.estimateServerNowMs()
      // 估算值跟住服务端时钟（收到响应至今的漂移在秒级）
      expect(Math.abs(est - Date.parse('2026-05-01T00:00:00Z'))).toBeLessThan(5000)
      // 而不是本地时钟（4 个月偏移必须被抵消）
      expect(Math.abs(est - Date.now())).toBeGreaterThan(24 * 3600_000)
    } finally {
      vi.useRealTimers()
    }
  })

  it('server_time 不可解析 → 诚实回退本地时钟', async () => {
    mockApi.fetchConfig.mockResolvedValue({ ...makeConfig(), server_time: 'not-a-date' })
    await store.loadConfig()
    expect(Math.abs(store.estimateServerNowMs() - Date.now())).toBeLessThan(5000)
  })

  it('config 未加载 → 回退本地时钟', () => {
    expect(Math.abs(store.estimateServerNowMs() - Date.now())).toBeLessThan(5000)
  })
})

describe('档案与钱包刷新', () => {
  it('refreshProfile 成功：wallet / power / buildRating / build 全部落位', async () => {
    const build = { attacker: { attack: 100 } }
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build,
      build_rating: { total: 42 },
      power: 555,
    } as any)
    await store.refreshProfile()
    expect(store.wallet).toEqual({ coin: 10, gem: 20, energy: 30, keys: 40 })
    expect(store.power).toBe(555)
    expect(store.buildRating).toEqual({ total: 42 })
    // ref 会把对象包成响应式代理，这里比较的是内容而非同一引用
    expect(store.build).toEqual(build)
  })

  it('refreshProfile 失败：静默（告警但不抛），已有数据保留', async () => {
    mockApi.fetchMe.mockRejectedValue(new Error('boom'))
    await expect(store.refreshProfile()).resolves.toBeUndefined()
    expect(warnSpy).toHaveBeenCalled()
    expect(store.power).toBe(0)
  })

  it('refreshWallet 成功 / 失败都不抛', async () => {
    mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 1, energy: 1, keys: 1 })
    await store.refreshWallet()
    expect(store.wallet).toEqual({ coin: 1, gem: 1, energy: 1, keys: 1 })

    mockApi.fetchWallet.mockRejectedValue(new Error('net'))
    await expect(store.refreshWallet()).resolves.toBeUndefined()
    expect(store.wallet).toEqual({ coin: 1, gem: 1, energy: 1, keys: 1 })
  })
})

describe('出战槽位（服务端权威 + 归一化）', () => {
  it('第 131 轮：equippedSkillIds 初值为空，不再捏造 [1,2,3]', () => {
    // 服务端对一个未保存过槽位的玩家返回 [0,0,0,0]（全空）。
    // 本地若预填 [1,2,3]，消费方（me 页防线）会把这些「玩家没装过的技能」
    // 当成出战技能上报。初值必须是诚实的空。
    expect(store.equippedSkillIds).toEqual([])
    expect(store.loadout).toEqual([0, 0, 0, 0])
  })

  it('setEquippedSkills：超过 4 个截断', () => {
    store.setEquippedSkills([1, 2, 3, 4, 5, 6])
    expect(store.equippedSkillIds).toEqual([1, 2, 3, 4])
  })

  it('loadLoadout：短槽位补 0 到 4、超长截断、空槽不进 equippedSkillIds', async () => {
    mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [7] })
    await store.loadLoadout()
    expect(store.loadout).toEqual([7, 0, 0, 0])
    expect(store.equippedSkillIds).toEqual([7])

    mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2, 3, 4, 5] })
    await store.loadLoadout()
    expect(store.loadout).toEqual([1, 2, 3, 4])

    // 服务端没下发 skill_ids：按全空槽处理（?? [] 分支）
    mockApi.fetchLoadout.mockResolvedValue({} as any)
    await store.loadLoadout()
    expect(store.loadout).toEqual([0, 0, 0, 0])
    expect(store.equippedSkillIds).toEqual([])
  })

  it('loadLoadout 失败：告警并保留本地值（回退分支）', async () => {
    store.setEquippedSkills([1])
    mockApi.fetchLoadout.mockRejectedValue(new Error('loadout failed'))
    await store.loadLoadout()
    expect(warnSpy).toHaveBeenCalled()
    expect(store.loadout).toEqual([0, 0, 0, 0])
    expect(store.equippedSkillIds).toEqual([1])
  })

  it('persistLoadout 成功：以服务端返回为准（有则用响应，无则用归一化结果）', async () => {
    mockApi.saveLoadout.mockResolvedValue({ skill_ids: [9, 0, 0, 0] })
    expect(await store.persistLoadout([9])).toBe(true)
    expect(store.loadout).toEqual([9, 0, 0, 0])
    expect(mockApi.saveLoadout).toHaveBeenCalledWith([9, 0, 0, 0])

    // 响应缺 skill_ids：回退到归一化的请求值（?? normalized 分支）
    mockApi.saveLoadout.mockResolvedValue({} as any)
    expect(await store.persistLoadout([5, 6, 7, 8, 9])).toBe(true)
    expect(mockApi.saveLoadout).toHaveBeenCalledWith([5, 6, 7, 8])
    expect(store.loadout).toEqual([5, 6, 7, 8])
  })

  it('persistLoadout 失败：返回 false 供 UI 提示，槽位不变', async () => {
    mockApi.saveLoadout.mockRejectedValue(new Error('save failed'))
    expect(await store.persistLoadout([1])).toBe(false)
    expect(warnSpy).toHaveBeenCalled()
    expect(store.loadout).toEqual([0, 0, 0, 0])
  })
})

describe('本地状态写入', () => {
  it('setBuild：对象写入，非对象（null）忽略', () => {
    const b = { attacker: {} }
    store.setBuild(b)
    // ref 深层响应式：内容一致即证明写入成功
    expect(store.build).toEqual(b)
    store.setBuild(null)
    expect(store.build).toEqual(b)
  })

  it('setMaxStage：只增不减（保护最高关卡进度）', () => {
    store.setMaxStage(5)
    expect(store.maxStage).toBe(5)
    store.setMaxStage(3)
    expect(store.maxStage).toBe(5)
  })
})

describe('computed', () => {
  it('enemyMap / skillMap（含合成技能）/ levelMap', async () => {
    await store.loadConfig()
    expect(store.enemyMap.get(1)?.name).toBe('游荡者')
    expect(store.skillMap.get(1)?.name).toBe('燃烧弹')
    expect(store.skillMap.get(101)?.name).toBe('蒸汽弹')
    expect(store.levelMap.get(3)?.chapter).toBe(2)
  })

  it('config 为 null 时三个 Map 都是空（?? [] 分支）', () => {
    expect(store.enemyMap.size).toBe(0)
    expect(store.skillMap.size).toBe(0)
    expect(store.levelMap.size).toBe(0)
  })

  it('unlockedLevel：夹在 [1, 100] 区间', () => {
    store.setMaxStage(0)
    expect(store.unlockedLevel).toBe(1)
    store.setMaxStage(42)
    expect(store.unlockedLevel).toBe(43)
    store.setMaxStage(99)
    expect(store.unlockedLevel).toBe(100)
    store.setMaxStage(150)
    expect(store.unlockedLevel).toBe(100)
  })

  it('equippedElements：跳过空槽与未知技能，按 loadout 顺序映射', async () => {
    await store.loadConfig()
    store.loadout = [1, 0, 2, 999]
    expect(store.equippedElements).toEqual(['fire', 'ice'])
  })

  it('attacker：服务端 build.attacker 全字段映射为 BigInt', async () => {
    store.setBuild({
      attacker: {
        attack: 1000,
        crit_permille: 50,
        crit_multiplier_permille: 1500,
        reaction_mult_permille: 1200,
        element_cap: 3,
        reaction_tier: 2,
        element_coef_permille: 400,
        heat_cap_permille: 200,
        armor_permille: 100,
        mechanic_permille: 300,
      },
    } as any)
    const a = store.attacker
    expect(a.attack).toBe(1000n)
    expect(a.critPermille).toBe(50n)
    expect(a.critMultiplierPermille).toBe(1500n)
    expect(a.reactionMultPermille).toBe(1200n)
    expect(a.elementCap).toBe(3n)
    expect(a.reactionTier).toBe(2n)
    expect(a.elementCoefPermille).toBe(400n)
    expect(a.heatCapPermille).toBe(200n)
    expect(a.armorPermille).toBe(100n)
    expect(a.mechanicPermille).toBe(300n)
  })

  it('attacker：老版本快照缺三个新字段 → 缺省为 0（?? 0 分支，BigInt(undefined) 会抛 TypeError）', async () => {
    store.setBuild({
      attacker: {
        attack: 800,
        crit_permille: 0,
        crit_multiplier_permille: 1500,
        reaction_mult_permille: 1000,
        element_cap: 3,
        reaction_tier: 2,
        element_coef_permille: 0,
      },
    } as any)
    const a = store.attacker
    expect(a.heatCapPermille).toBe(0n)
    expect(a.armorPermille).toBe(0n)
    expect(a.mechanicPermille).toBe(0n)
  })

  it('attacker：服务端未下发 → 本地占位 + 控制台告警（I-6 会误判，必须能看见）', () => {
    const fallback = defaultAttacker()
    expect(store.attacker).toEqual(fallback)
    expect(warnSpy).toHaveBeenCalledWith(expect.stringContaining('build.attacker 缺失'))
  })
})
