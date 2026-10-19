// @vitest-environment jsdom
/**
 * 背包页（bag.vue）组件测试。
 * 覆盖：出战槽同步（服务端权威）、装备/移除/槽位满/未解锁/被动技能/保存失败全分支、
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
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  upgradeSkill: vi.fn(),
  fetchGems: vi.fn(),
  washGem: vi.fn(),
  setTokenInternal: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

/**
 * 服务端 `build.skills` 的已解锁技能行（键 = 技能 id 字符串）。
 *
 * 判「能不能装备/升级」的唯一依据就是这张表：服务端对不在其中的技能
 * 一律拒（SaveLoadout 400「技能 N 未解锁」/ UpgradeSkill「未拥有」）。
 */
function owned(...ids: number[]): Record<string, { id: number; level: number; slot: number }> {
  return Object.fromEntries(ids.map((id) => [String(id), { id, level: 1, slot: -1 }]))
}


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
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: bagConfig(), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7,
    // 缺省把测试用到的技能都设为已解锁；「未解锁」形态由用例各自覆盖
    // （服务端只给新号基础 4 个技能，合成技能没有获取途径 —— 见未解锁用例）。
    build: { skills: owned(1, 2, 3, 4, 5, 101) },
    build_rating: null,
    power: 9,
  } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [1, 2] })
  // 宝石默认空：用例需要时各自覆盖。不设默认的话 loadGems 会靠 catch 兜住
  // undefined，那条路径能跑但是在测「拉到 undefined」，不是测「拉到空列表」。
  mockApi.fetchGems.mockResolvedValue({ gems: [] })
  mockApi.saveLoadout.mockImplementation((ids: number[]) =>
    Promise.resolve({ skill_ids: ids }),
  )
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('bag.vue 背包页', () => {
  it('道具卡片逐条标明能不能用，下架的写「已下架」', async () => {
    // 第 150 轮。`tokens` 是 `/wallet` 首次有了消费方的那批字段。
    //
    // 断言的重点不是「有文字」，而是**下架那两件不能写成「无法使用」**：
    // 商城侧已经把它们 enabled=false 了（seedShop 的 delisted），
    // 界面如果还写「目前无法使用」，就是在描述一个已经不存在的购买行为 ——
    // 而玩家点进商城找不到商品时，第一反应就是「这是个 bug」。
    mockApi.fetchWallet.mockResolvedValue({
      coin: 1, gem: 2, energy: 3, keys: 4,
      tokens: { revive_token: 2, gem_wash_token: 1, mastery_reset: 3 },
    })
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    const text = wrapper.text()
    expect(text).toContain('专精重铸券')
    expect(text).toContain('复活币')
    expect(text).toContain('宝石洗练券')

    // 可用的那件：写明用法
    expect(text).toContain('清空全部已点亮的专精节点')

    // 下架的那件：必须写「已下架」，并说明已购余额怎么办
    expect(text).toContain('已从商城下架')
    expect(text).toContain('已购余额暂时无法使用')
  })

  it('复活币接通消费端点后必须写成「可用」，不能还留着「已下架」', async () => {
    // 第 150 轮续：服务端已在 StartBattle 接通复活币（开战扣 1 枚 + 血量加成），
    // 商品也恢复了上架。界面如果还写「已下架」，就是在陈述一个已经不成立的
    // 事实 —— 玩家会以为自己买了却用不了，而实际上开战就自动生效了。
    mockApi.fetchWallet.mockResolvedValue({
      coin: 1, gem: 2, energy: 3, keys: 4,
      tokens: { revive_token: 2 },
    })
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    const text = wrapper.text()
    expect(text).toContain('复活币')
    expect(text).toContain('开战时自动消耗 1 枚')
    expect(text).toContain('+50%')
  })

  it('背包页不再承诺「词条可洗练」—— 玩家拿不到宝石实例', async () => {
    // 「承诺一个玩不到的东西」比不承诺更糟：
    // 它把一个内部未完成项变成了对外的产品说明。
    const { wrapper } = mountPage(Bag)
    await flushPromises()
    expect(wrapper.text()).not.toContain('词条可洗练')
  })

  describe('我的宝石（第 150 轮）', () => {
    it('列出持有的宝石：品质中文化、词条可读、空态有引导', async () => {
      mockApi.fetchGems.mockResolvedValue({
        gems: [
          {
            id: 7,
            gem_id: 3,
            code: 'gem_crit',
            name: '暴击石',
            descr: '暴击率提升',
            quality: 'gold',
            affixes: [
              { affix: 'crit', value: 120 },
              { affix: 'attack', value: 70 },
            ],
            equipped_slot: '',
          },
        ],
      })
      const { wrapper } = mountPage(Bag)
      await flushPromises()
      const text = wrapper.text()
      expect(text).toContain('我的宝石')
      expect(text).toContain('暴击石')
      // 品质名走中文映射，且未知档位不能被吞
      expect(text).toContain('金')
      // 词条可读：名字映射 + 数值
      expect(text).toContain('暴击 +120')
      expect(text).toContain('攻击 +70')
    })

    it('没有宝石时给出引导文案，而不是空白', async () => {
      mockApi.fetchGems.mockResolvedValue({ gems: [] })
      const { wrapper } = mountPage(Bag)
      await flushPromises()
      expect(wrapper.text()).toContain('还没有宝石')
    })

    it('洗练：成功后就地替换该颗宝石并刷新余额（以服务端为准）', async () => {
      mockApi.fetchGems.mockResolvedValue({
        gems: [
          {
            id: 7, gem_id: 3, code: 'gem_crit', name: '暴击石', descr: '暴击率提升',
            quality: 'gold', affixes: [{ affix: 'crit', value: 100 }], equipped_slot: '',
          },
        ],
      })
      mockApi.fetchWallet.mockResolvedValue({
        coin: 1, gem: 2, energy: 3, keys: 4, tokens: { gem_wash_token: 1 },
      })
      mockApi.washGem.mockResolvedValue({
        gem: {
          id: 7, gem_id: 3, code: 'gem_crit', name: '暴击石', descr: '暴击率提升',
          quality: 'gold', affixes: [{ affix: 'crit', value: 155 }], equipped_slot: '',
        },
      })
      const { wrapper } = mountPage(Bag)
      await flushPromises()
      expect(wrapper.text()).toContain('暴击 +100')

      await wrapper.find('.owned-gem')!.find('.link')!.trigger('click')
      await flushPromises()
      await flushPromises()

      expect(mockApi.washGem).toHaveBeenCalledWith(7)
      // 词条被就地替换 —— 不是追加一行
      expect(wrapper.text()).toContain('暴击 +155')
      expect(wrapper.text()).not.toContain('暴击 +100')
      // 余额以服务端为准：扣券后必须重拉，否则下次进来还显示 1
      expect(mockApi.fetchWallet.mock.calls.length).toBeGreaterThanOrEqual(2)
      expect(um.mock.showToast).toHaveBeenCalledWith({ title: '洗练完成', icon: 'none' })
    })

    it('洗练失败 → toast 原因，不静默（券是花钱买的）', async () => {
      mockApi.fetchGems.mockResolvedValue({
        gems: [
          {
            id: 7, gem_id: 3, code: 'gem_crit', name: '暴击石', descr: '',
            quality: 'gray', affixes: [{ affix: 'crit', value: 10 }], equipped_slot: '',
          },
        ],
      })
      mockApi.fetchWallet.mockResolvedValue({
        coin: 1, gem: 2, energy: 3, keys: 4, tokens: { gem_wash_token: 1 },
      })
      mockApi.washGem.mockRejectedValue(new Error('洗练券不足'))

      const { wrapper } = mountPage(Bag)
      await flushPromises()
      await wrapper.find('.owned-gem')!.find('.link')!.trigger('click')
      await flushPromises()
      await flushPromises()

      expect(um.mock.showToast).toHaveBeenCalledWith({ title: '洗练券不足', icon: 'none' })
    })

    it('没有洗练券时点洗练：先本地拦下，不发请求', async () => {
      mockApi.fetchGems.mockResolvedValue({
        gems: [
          {
            id: 7, gem_id: 3, code: 'gem_crit', name: '暴击石', descr: '',
            quality: 'gray', affixes: [{ affix: 'crit', value: 10 }], equipped_slot: '',
          },
        ],
      })
      mockApi.fetchWallet.mockResolvedValue({
        coin: 1, gem: 2, energy: 3, keys: 4, tokens: {},
      })
      const { wrapper } = mountPage(Bag)
      await flushPromises()
      await wrapper.find('.owned-gem')!.find('.link')!.trigger('click')
      await flushPromises()

      expect(mockApi.washGem).not.toHaveBeenCalled()
      expect(um.mock.showToast).toHaveBeenCalledWith({ title: '没有洗练券', icon: 'none' })
    })

    it('拉不到宝石时不当成「没有宝石」以外的任何结论：页面照常渲染', async () => {
      mockApi.fetchGems.mockRejectedValue(new Error('网络炸了'))
      const { wrapper } = mountPage(Bag)
      await flushPromises()
      // 背包页其余部分不受影响
      expect(wrapper.findAll('.slot').length).toBe(4)
    })
  })

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

  it('未解锁的技能：行内标「未解锁」，不给装备/升级入口', async () => {
    // 服务端只解锁了基础技能（新号 = 1..4）；合成技能 101 当前没有获取途径。
    // 修前：技能库对全表一律渲染「装备」，点未解锁的那些只会拿到
    // 400「技能 5 未解锁」，界面弹的却是「保存失败，出战配置未变更」。
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: { skills: owned(1, 2, 3, 4) },
      build_rating: null,
      power: 9,
    } as any)
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    const steamRow = wrapper.findAll('.skill-row').find((r) => r.text().includes('蒸汽弹'))!
    expect(steamRow.text()).toContain('未解锁')
    expect(steamRow.find('.link').exists()).toBe(false) // 没有任何可点的操作
    // 已解锁的技能照旧给入口：穿甲弹未装备（槽位 [1,2]），
    // 升级因 coin=1 显示「金币不足」（非链接），链接是「装备」那个。
    const apRow = wrapper.findAll('.skill-row').find((r) => r.text().includes('穿甲弹'))!
    expect(apRow.find('.link').exists()).toBe(true)
  })

  it('未解锁的技能：直呼 toggleEquip / doUpgrade 也被本地拦下（不发请求）', async () => {
    // 模板已经不给入口了，这里守的是「入口被别的路径调到」：
    // 放行的话服务端必回 400 /「未拥有」，玩家看到的是「保存失败」「升级失败」——
    // 一个本地就该拦下的操作被报成了系统故障。
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: { skills: owned(1) },
      build_rating: null,
      power: 9,
    } as any)
    const { wrapper } = mountPage(Bag)
    await flushPromises()

    await (wrapper.vm as any).toggleEquip(101)
    await (wrapper.vm as any).doUpgrade(101)

    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '技能未解锁', icon: 'none' })
    expect(mockApi.saveLoadout).not.toHaveBeenCalled()
    expect(mockApi.upgradeSkill).not.toHaveBeenCalled()
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

  it('第 145 轮：已登录态进背包也会拉服务端 build（技能等级/升级花费以真实养成为准，不恒 0）', async () => {
    // 服务端：玩家拥有技能 1（Lv.3）
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: { skills: { '1': { id: 1, level: 3, slot: 0 } } },
      build_rating: null,
      power: 9,
    } as any)
    // 模拟「已登录、build 尚未拉取」——不触发 onMounted 的 login()，
    // 只有挂载逻辑本身跑，才能证明 build 是页面主动刷新的。
    // 已登录意味着 login()（含 loadConfig）不会跑，config 需手动就位，
    // 否则 allSkills/skill_rules 全空、技能行一行都渲染不出来。
    const { wrapper, store } = mountPage(Bag, (s) => {
      s.loggedIn = true
      s.config = bagConfig()
      s.build = null
    })
    await flushPromises()
    // 修前：onMounted 只 loadLoadout，build 仍是 null → levelOf 恒 0
    expect(mockApi.fetchMe).toHaveBeenCalled() // refreshProfile 在挂载时被调用
    expect(store.build?.skills?.['1']?.level).toBe(3)
    const burnRow = wrapper.findAll('.skill-row').find((r) => r.text().includes('燃烧弹'))!
    expect(burnRow.text()).toContain('Lv.3') // 等级来自服务端 build，不是 0
    // 升级花费 = base_cost(100) × 当前等级(3) = 300；钱包 coin=1 → 不足
    expect(burnRow.text()).toContain('金币不足（300）')
  })
})
