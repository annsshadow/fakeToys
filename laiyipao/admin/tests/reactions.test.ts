/**
 * `@/reactions` 模块的守卫。
 *
 * ## 为什么要单独测它
 *
 * 后台有**两个**页面要显示反应的中文名（看板的分布图、战报详情的 chip）。
 * 两处一开始各自硬编码了一张 `REACTION_LABEL` ——
 * R38 只消除了 dashboard 那张，因为当时只在**注意到气味的那个页面**上找。
 * BattlesView 那张留了下来，仍是活的。
 *
 * 所以「取名 + 查名」收敛到了本模块。收敛之后，模块本身就成了
 * 单点：它坏了，两个页面一起坏。因此它需要自己的守卫。
 *
 * ## 三条性质
 *
 * 1. **成功才缓存** —— 失败必须清掉缓存，否则一次网络抖动会把
 *    「取不到名字」固化到整个会话，页面从此一直显示原始 key 且毫无迹象。
 * 2. **兜底返回原始 key** —— 不是空串、不是 `undefined`。
 *    覆盖「后台连的是旧版服务端」。
 * 3. **两个页面共用一次请求** —— 记忆化是这个模块存在的一半理由。
 */
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { __resetReactionLabelCache, fetchReactionLabels, labelOfReaction } from '@/reactions'

const fetchReactions = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchReactions }))

function spec(key: string, name: string) {
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
  __resetReactionLabelCache()
  fetchReactions.mockResolvedValue([
    spec('steam_burst', '蒸汽爆发'),
    spec('overheat', '过热'),
  ])
})

describe('@/reactions 记忆化', () => {
  it('同一会话只请求一次', async () => {
    const a = await fetchReactionLabels()
    const b = await fetchReactionLabels()
    expect(fetchReactions).toHaveBeenCalledTimes(1)
    expect(a).toEqual(b)
  })

  it('重置缓存后会重新请求', async () => {
    await fetchReactionLabels()
    __resetReactionLabelCache()
    await fetchReactionLabels()
    expect(fetchReactions).toHaveBeenCalledTimes(2)
  })
})

describe('@/reactions 失败不缓存', () => {
  it('失败会抛错给调用方', async () => {
    fetchReactions.mockRejectedValueOnce(new Error('ECONNREFUSED'))
    await expect(fetchReactionLabels()).rejects.toThrow('ECONNREFUSED')
  })

  it('⚠️ 失败后下一次会重新尝试（不把故障固化到整个会话）', async () => {
    // 这一条是本模块最容易写错的地方：如果 catch 里不清缓存，
    // 第二次调用会**直接返回同一个 rejected Promise**，
    // 于是「重启页面」这个最自然的补救手段会完全失效 ——
    // 运营会看到一个永远修不好的页面。
    fetchReactions.mockRejectedValueOnce(new Error('boom'))
    await expect(fetchReactionLabels()).rejects.toThrow('boom')

    // 服务端恢复
    await fetchReactionLabels()
    expect(fetchReactions).toHaveBeenCalledTimes(2)
    const labels = await fetchReactionLabels()
    expect(labels.steam_burst).toBe('蒸汽爆发')
  })
})

describe('@/reactions 查名兜底', () => {
  it('已知 key 返回中文名', async () => {
    const labels = await fetchReactionLabels()
    expect(labelOfReaction(labels, 'steam_burst')).toBe('蒸汽爆发')
  })

  it('未知 key 退回原始 key，不返回 undefined/空串', async () => {
    const labels = await fetchReactionLabels()
    // 覆盖「后台连的是旧版服务端、响应里没有新反应」
    expect(labelOfReaction(labels, 'brand_new_rx')).toBe('brand_new_rx')
  })

  it('空表时也退回原始 key', async () => {
    // 「不下发任何反应名」不该让页面崩，也不该显示空白
    expect(labelOfReaction({}, 'steam_burst')).toBe('steam_burst')
  })

  it('表里有空字符串时也退回原始 key（`||` 而非 `??`）', async () => {
    // 空串是 falsy，`||` 会退回原始 key；`??` 不会。
    // 这里钉住 `||`：空串在页面上等于「看不见标签」，没有意义。
    expect(labelOfReaction({ steam_burst: '' }, 'steam_burst')).toBe('steam_burst')
  })
})
