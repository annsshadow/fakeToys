/**
 * BattlesView.vue 测试。
 *
 * 战报页的守恒检查（kills+leaked）、反应滥用标记（reactions/kills>30）、
 * 以及详情抽屉里 JSON 字符串的容错解析（parseObj 四分支）是数据可信度的关键。
 */
import { flushPromises, mount } from '@vue/test-utils'
import { ElDrawer, ElInputNumber } from 'element-plus'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminBattle } from '@/api'
import { __resetReactionLabelCache } from '@/reactions'

const fetchBattles = vi.hoisted(() => vi.fn())
const fetchBattleDetail = vi.hoisted(() => vi.fn())
// `@/reactions` 内部调 `@/api` 的 fetchReactions，所以 mock 必须提供它
const fetchReactions = vi.hoisted(() => vi.fn())
const verifyBattle = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchBattles, fetchBattleDetail, fetchReactions, verifyBattle }))

import BattlesView from '@/views/BattlesView.vue'

function battleFixture(overrides: Partial<AdminBattle> = {}): AdminBattle {
  return {
    id: 1,
    user_id: 9,
    level_id: 3,
    result: 'win',
    stars: 3,
    score: 88000,
    kills: 10,
    leaked: 2,
    wave_reached: 5,
    duration_ms: 12345,
    reactions: 5,
    heat_max: 90,
    replay_hash: 'abcd1234abcd1234abcd1234abcd1234',
    created_at: '2026-01-02T03:04:05Z',
    total_enemies: 12, // kills(10)+leaked(2) 恰好等于总怪数 → 「守恒」
    verify_checked: 3,
    verify_mismatched: 0,
    ...overrides,
  }
}

async function mountBattles(items: AdminBattle[] = [], total = items.length) {
  fetchBattles.mockResolvedValue({ items, total })
  const wrapper = mount(BattlesView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

/** 构造一份最小反应表，字段与 domain.ReactionSpec 的 json tag 对齐。 */
function reactionSpec(key: string, name: string) {
  return {
    key,
    name,
    base_coef: 60,
    attack_weight_pct: 300,
    status_duration_ms: 0,
    aoe_radius: 0,
    dispel_shield: false,
    amplify_pct: 0,
    descr: '',
  }
}

beforeEach(() => {
  vi.clearAllMocks()
  // ⚠️ 必须清 `@/reactions` 的模块级缓存。
  // `vi.clearAllMocks()` 只清 mock 的调用记录，**清不掉模块里已缓存的 Promise** ——
  // 不清的话第一个用例取到的反应名会泄漏到后面所有用例，
  // 而且症状是「某个用例莫名其妙地看到了中文名」。
  __resetReactionLabelCache()
  fetchReactions.mockResolvedValue([
    reactionSpec('steam_burst', '蒸汽爆发'),
    reactionSpec('overheat', '过热'),
  ])
  verifyBattle.mockResolvedValue({
    battle_id: 1,
    expected_hash: 'stored-hash',
    actual_hash: 'stored-hash',
    matched: true,
    level_id: 3,
    seed: '12345',
    recorded_at: '2026-01-02T03:04:05Z',
  })
})

describe('BattlesView 列表渲染', () => {
  it('胜负/星级/守恒/时长/时间如实渲染；零击杀零漏怪标记「无接触」', async () => {
    const wrapper = await mountBattles([
      battleFixture({ id: 1 }),
      battleFixture({ id: 2, result: 'lose', stars: 0, kills: 0, leaked: 0 }),
    ])
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('胜')
    expect(rows[0].text()).toContain('★★★')
    expect(rows[0].text()).toContain('守恒')
    expect(rows[0].text()).toContain('12.3s')
    // 第 124 轮：时间列渲染成查看者本地时区（服务端下发 UTC 的 2026-01-02T03:04:05Z）
    const d = new Date('2026-01-02T03:04:05Z')
    const p = (n: number) => String(n).padStart(2, '0')
    expect(rows[0].text()).toContain(
      `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`,
    )
    expect(rows[1].text()).toContain('负')
    expect(rows[1].text()).not.toContain('★')
    expect(rows[1].text()).toContain('无接触')
    wrapper.unmount()
  })

  // 第 123 轮：kills+leaked 超出总怪数 → 标「可疑」（伪造信号），不再一律「守恒」
  it('击杀+漏怪超出总怪数标记可疑（第 123 轮：守恒列不再形同虚设）', async () => {
    const wrapper = await mountBattles([
      battleFixture({ id: 1, kills: 50, leaked: 5, total_enemies: 12 }), // 55 > 12
    ])
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('可疑')
    expect(rows[0].text()).toContain('超总怪数')
    expect(rows[0].text()).not.toContain('守恒')
    // 恰好的情况仍是「守恒」
    const ok = await mountBattles([battleFixture({ id: 2 })]) // 10+2=12=total
    expect(ok.findAll('.el-table__row')[0].text()).toContain('守恒')
    ok.unmount()
    wrapper.unmount()
  })

  it('反应滥用标记：reactions/kills > 30 才标红；任一前置为 0 不标记', async () => {
    const wrapper = await mountBattles([
      battleFixture({ id: 1, kills: 1, reactions: 40 }), // 40 > 30 → 滥用
      battleFixture({ id: 2, kills: 1, reactions: 30 }), // 边界：30 不大于 30
      battleFixture({ id: 3, kills: 0, reactions: 40 }), // kills=0 → 不标记
      battleFixture({ id: 4, kills: 5, reactions: 0 }), // reactions=0 → 不标记
    ])
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].find('.danger').exists()).toBe(true)
    expect(rows[1].find('.danger').exists()).toBe(false)
    expect(rows[2].find('.danger').exists()).toBe(false)
    expect(rows[3].find('.danger').exists()).toBe(false)
    wrapper.unmount()
  })

  it('筛选按钮：默认参数与设置过滤后的参数（输入框 v-model 双向生效）', async () => {
    const wrapper = await mountBattles()
    await wrapper.findAll('button').find((b) => b.text() === '筛选')!.trigger('click')
    await flushPromises()
    expect(fetchBattles).toHaveBeenLastCalledWith({ user_id: undefined, level_id: undefined, limit: 50 })

    // 通过组件事件驱动两个 el-input-number 的 v-model（与真实输入等价）
    const nums = wrapper.findAllComponents(ElInputNumber)
    expect(nums).toHaveLength(2)
    await nums[0].vm.$emit('update:modelValue', 5)
    await nums[1].vm.$emit('update:modelValue', 7)
    await flushPromises()
    await wrapper.findAll('button').find((b) => b.text() === '筛选')!.trigger('click')
    await flushPromises()
    expect(fetchBattles).toHaveBeenLastCalledWith({ user_id: 5, level_id: 7, limit: 50 })
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchBattles.mockRejectedValueOnce(new Error('战报库离线'))
    const wrapper = mount(BattlesView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('战报加载失败：战报库离线')
    wrapper.unmount()
  })
})

describe('BattlesView 详情抽屉', () => {
  const detailBase = { ...battleFixture({ id: 88 }), shots: 10, hits: 8 } as AdminBattle & Record<string, unknown>

  it('详情正常打开：回放哈希、发射/命中、元素与反应分布中文标签渲染；关闭抽屉走 v-model', async () => {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: {
        ...detailBase,
        elements_used: '{"fire":3,"kinetic":1,"void":4}', // void 不在映射表 → 回退原始 key
        reactions_used: '{"steam_burst":2,"mega_boom":1}', // mega_boom 同上
        terrain_used: 'oil_drum',
      },
    })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()

    expect(fetchBattleDetail).toHaveBeenCalledWith(1)
    const text = wrapper.text()
    expect(text).toContain('战报详情')
    expect(text).toContain('battle_id')
    expect(text).toContain('10 / 8')
    expect(text).toContain('abcd1234abcd1234abcd1234abcd1234')
    expect(text).toContain('焰 × 3')
    expect(text).toContain('动能 × 1')
    expect(text).toContain('void × 4')
    expect(text).toContain('蒸汽爆发 × 2')
    expect(text).toContain('mega_boom × 1')

    // 用户关闭抽屉：v-model 的 onUpdate 处理器真实执行
    const drawer = wrapper.findComponent(ElDrawer)
    await drawer.vm.$emit('update:modelValue', false)
    await flushPromises()
    const vm = wrapper.vm as unknown as { detailDrawer: boolean }
    expect(vm.detailDrawer).toBe(false)
    wrapper.unmount()
  })

  it('服务端修正过的战报：clamped 标记展示，clamp_note 缺省时显示默认文案', async () => {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: { ...detailBase, clamped: true, clamp_note: '' },
    })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('上报数据越界，已按上限截断')
    wrapper.unmount()
  })

  it('未修正的战报不显示修正标签（v-if 假分支）', async () => {
    fetchBattleDetail.mockResolvedValueOnce({ battle: { ...detailBase } })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).not.toContain('服务端修正')
    wrapper.unmount()
  })

  it('元素/反应 JSON 的容错：非法 JSON 与空值都回退为空分布提示', async () => {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: {
        ...detailBase,
        elements_used: 'not-json{',
        reactions_used: 'also-bad{',
        terrain_used: 0,
      },
    })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()

    const text = wrapper.text()
    // 解析失败 → 空分布 → 引导文案
    expect(text).toContain('无记录 —— 这一局完全没触发反应')
    // terrain_used=0：数值走 !parseObj 的真分支，显示占位
    expect(text).toContain('—')
    wrapper.unmount()
  })

  it('详情接口失败：弹错误且抽屉不打开', async () => {
    fetchBattleDetail.mockRejectedValueOnce(new Error('战报不存在'))
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()
    expect(document.body.textContent).toContain('战报不存在')
    // 接口失败时抽屉保持关闭
    const vm = wrapper.vm as unknown as { detailDrawer: boolean }
    expect(vm.detailDrawer).toBe(false)
    wrapper.unmount()
  })

  it('parseObj 直接验证四个分支：对象透传 / null 回退 / 合法 JSON / 非法 JSON', async () => {
    const wrapper = await mountBattles()
    const vm = wrapper.vm as unknown as { parseObj: (v: unknown) => Record<string, number> }
    expect(vm.parseObj({ fire: 2 })).toEqual({ fire: 2 })
    expect(vm.parseObj(null)).toEqual({})
    expect(vm.parseObj('{"a":1}')).toEqual({ a: 1 })
    expect(vm.parseObj('bad{')).toEqual({})
    wrapper.unmount()
  })
})

describe('BattlesView 验真列与过滤器', () => {
  it('未验真（0/0）显示「未验真」而不是「一致」', async () => {
    // 与 UsersView 同一个语义：**0/0 是未知，不是干净**。
    // 两个页面对同一概念说不同的话，运营会以为是数据矛盾。
    const wrapper = await mountBattles([
      battleFixture({ id: 1, verify_checked: 0, verify_mismatched: 0 }),
    ])
    expect(wrapper.text()).toContain('未验真')
    expect(wrapper.text()).not.toContain('一致 ')
  })

  it('有不匹配时显示「不匹配 m/n」', async () => {
    const wrapper = await mountBattles([
      battleFixture({ id: 1, verify_checked: 6, verify_mismatched: 2 }),
    ])
    expect(wrapper.text()).toContain('不匹配 2/6')
  })

  it('勾选「只看验真不匹配」会把 only_mismatched 传给接口', async () => {
    const wrapper = await mountBattles([battleFixture()])
    // 先确认默认不带这个参数
    expect(fetchBattles).toHaveBeenLastCalledWith(
      expect.objectContaining({ only_mismatched: undefined }),
    )

    // 再勾选并触发筛选
    const cb = wrapper.findAll('.el-checkbox').find((c) => c.text().includes('只看验真不匹配'))
    expect(cb, '找不到「只看验真不匹配」复选框').toBeTruthy()
    await cb!.find("input").setValue(true)
    await flushPromises()
    ;(wrapper.vm as unknown as { load: () => Promise<void> }).load()
    await flushPromises()

    expect(fetchBattles).toHaveBeenLastCalledWith(
      expect.objectContaining({ only_mismatched: true }),
    )
  })

  it('反应名取不到时战报列表仍然出得来（锦上添花 ≠ 必需）', async () => {
    // 反应名只影响详情抽屉里的 chip。取不到时列表必须照常显示，
    // 否则一次网络抖动会让运营**看不到任何战报**。
    fetchReactions.mockRejectedValue(new Error('reactions down'))
    const wrapper = await mountBattles([battleFixture({ id: 7 })])
    expect(wrapper.findAll(".el-table__row").length).toBeGreaterThan(0)
    expect(wrapper.text()).not.toContain('reactions down')
  })
})

describe('BattlesView 运营侧验真入口', () => {
  /**
   * 打开某条战报的详情抽屉。
   *
   * ⚠️ 三处都抄本文件里已经跑通的写法：
   *  - 按钮用 `text() === '详情'`（不是「行内第一个按钮」，那个会匹配到别的东西）；
   *  - 关抽屉用 `ElDrawer` 的 `update:modelValue`（不是点关闭按钮，jsdom 下点不到）；
   *  - 每次自己 mock `fetchBattleDetail`（本文件**没有**它的默认实现）。
   *
   * 早先一版三条都踩了，症状是 6 条用例全红而功能其实写好了 ——
   * 「测试全红」和「功能坏了」看起来一模一样，必须逐条核对前提。
   */
  async function openDetailOf(row: AdminBattle) {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: { ...row, shots: 10, hits: 8, seed: '424242' } as AdminBattle &
        Record<string, unknown>,
    })
    const wrapper = await mountBattles([row])
    const btn = wrapper.findAll('button').find((b) => b.text() === '详情')
    expect(btn, '找不到「详情」按钮').toBeTruthy()
    await btn!.trigger('click')
    await flushPromises()
    return wrapper
  }

  async function closeDrawer(wrapper: Awaited<ReturnType<typeof openDetailOf>>) {
    await wrapper.findComponent(ElDrawer).vm.$emit('update:modelValue', false)
    await flushPromises()
  }

  function hashInput(wrapper: Awaited<ReturnType<typeof openDetailOf>>) {
    const el = wrapper.find('input[placeholder*="重放算出的 replay_hash"]')
    expect(el.exists(), '找不到粘贴哈希的输入框').toBe(true)
    return el
  }

  function submitBtn(wrapper: Awaited<ReturnType<typeof openDetailOf>>) {
    const b = wrapper.findAll('button').find((x) => x.text().includes('提交比对'))
    expect(b, '找不到「提交比对」按钮').toBeTruthy()
    return b!
  }

  it('抽屉里给出验真入口与种子', async () => {
    // 本后台**不重放**（服务端没有引擎），运营必须在别处用种子重跑才能算哈希。
    // 种子不给出来，这个入口就只是个摆设。
    const wrapper = await openDetailOf(battleFixture({ id: 1 }))
    expect(wrapper.text()).toContain('发起验真')
    expect(wrapper.text()).toContain('424242')
  })

  it('提交时带上 battle_id 与粘贴的哈希（去掉首尾空白）', async () => {
    const wrapper = await openDetailOf(battleFixture({ id: 42 }))
    await hashInput(wrapper).setValue('  recomputed-hash  ')
    await submitBtn(wrapper).trigger('click')
    await flushPromises()
    expect(verifyBattle).toHaveBeenCalledWith(42, 'recomputed-hash')
  })

  it('空输入不提交（免得凭空记一条「不匹配」）', async () => {
    const wrapper = await openDetailOf(battleFixture({ id: 7 }))
    await submitBtn(wrapper).trigger('click')
    await flushPromises()
    expect(verifyBattle).not.toHaveBeenCalled()
  })

  it('一致时显示结论与两个哈希', async () => {
    verifyBattle.mockResolvedValueOnce({
      battle_id: 1,
      expected_hash: 'stored-AAA',
      actual_hash: 'actual-BBB',
      matched: true,
      level_id: 3,
      seed: '424242',
      recorded_at: '2026-01-02T03:04:05Z',
    })
    const wrapper = await openDetailOf(battleFixture({ id: 1 }))
    await hashInput(wrapper).setValue('whatever')
    await submitBtn(wrapper).trigger('click')
    await flushPromises()
    const text = wrapper.text()
    // ⚠️ 判据用**哈希字符串**，不用「一致」二字 ——
    // 表格的验真列本来就写着「一致 3」，用它当判据会恒绿。
    expect(text).toContain('actual-BBB')
    expect(text).toContain('stored-AAA')
  })

  it('不匹配时给出「不匹配」结论', async () => {
    verifyBattle.mockResolvedValueOnce({
      battle_id: 5,
      expected_hash: 'stored-CCC',
      actual_hash: 'actual-DDD',
      matched: false,
      level_id: 3,
      seed: '424242',
      recorded_at: '2026-01-02T03:04:05Z',
    })
    const wrapper = await openDetailOf(battleFixture({ id: 5 }))
    await hashInput(wrapper).setValue('different')
    await submitBtn(wrapper).trigger('click')
    await flushPromises()
    // 表格里不会出现「不匹配」（那三行的验真列都是「一致 3」），所以这里可以安全用它
    expect(wrapper.text()).toContain('不匹配')
    expect(wrapper.text()).toContain('actual-DDD')
  })

  it('换一条战报会清掉上一次的输入与结果', async () => {
    // 不清的话会把 A 的比对结论看成 B 的 —— 而两者都是合法的界面状态。
    verifyBattle.mockResolvedValueOnce({
      battle_id: 1,
      expected_hash: 'stored-EEE',
      actual_hash: 'actual-FFF',
      matched: true,
      level_id: 3,
      seed: '424242',
      recorded_at: '2026-01-02T03:04:05Z',
    })
    fetchBattleDetail
      .mockResolvedValueOnce({ battle: { ...battleFixture({ id: 1 }) } as never })
      .mockResolvedValueOnce({ battle: { ...battleFixture({ id: 2 }) } as never })

    const wrapper = await mountBattles([battleFixture({ id: 1 }), battleFixture({ id: 2 })])
    const detailBtns = () => wrapper.findAll('button').filter((b) => b.text() === '详情')
    await detailBtns()[0].trigger('click')
    await flushPromises()

    await hashInput(wrapper).setValue('hash-of-battle-1')
    await submitBtn(wrapper).trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('actual-FFF')

    await closeDrawer(wrapper)
    // 下标 1 = 第二行。写 0 的话又打开了第一场，断言就成了假阳性：
    // 同一条战报的状态**本来就该保留**。
    await detailBtns()[1].trigger('click')
    await flushPromises()

    const el = wrapper.find('input[placeholder*="重放算出的 replay_hash"]')
    expect((el.element as HTMLInputElement).value).toBe('')
    expect(wrapper.text()).not.toContain('actual-FFF')
  })
})

