/**
 * RecomputesView 测试（R1 重算审计页）。
 *
 * ## 判读顺序上的先后
 *
 * 这个页面唯一不可替代的作用是回答「**现在还在拦吗**」。
 * 熔断触发后系统仍在重算、仍在记录、但**不再拒发**，
 * 而台账数字可能完全正常 —— 不单独指出来，值班的人会以为防护开着。
 *
 * 所以前两条用例守的就是这个，且**必须成对**：
 * 降级时要出现、没降级时不能出现。
 * 只写前一条的话，一个「永远显示告警」的实现也能全绿 ——
 * 而反复误报的告警等于没有告警。
 */
import { describe, expect, it, vi, beforeEach } from 'vitest'
import { mountEl, settle, bodyText } from './helpers'

const fetchRecomputes = vi.fn()
const fetchRecomputeStats = vi.fn()

vi.mock('@/api', () => ({
  fetchRecomputes: (...a: unknown[]) => fetchRecomputes(...a),
  fetchRecomputeStats: (...a: unknown[]) => fetchRecomputeStats(...a),
}))

import RecomputesView from '@/views/RecomputesView.vue'

/** 一条台账行。字段名与后端 `service.RecomputeView` 的 json tag 一致。 */
function row(over: Record<string, unknown> = {}) {
  return {
    id: 1,
    battle_id: 900,
    user_id: 7,
    nickname: 'rc',
    level_id: 3,
    reported: { score: 100, kills: 10, duration_ms: 60000, hash: 'aa' },
    recomputed: { score: 100, kills: 10, duration_ms: 60000, hash: 'aa' },
    mismatched_fields: ['score'],
    mode: 'enforce',
    rejected: true,
    engine_version: 'skills-only/v1',
    created_at: '2026-10-14T00:00:00Z',
    ...over,
  }
}

function summary(over: Record<string, unknown> = {}) {
  return {
    mode: 'enforce',
    // 2026-10-21（二轮审计 P1）：两个 scope 分开报。
    //   mode / skills_mode —— S 段（真正拒发奖励的那一段），永不降级；
    //   engine_mode        —— 引擎侧，可被熔断降级。
    engine_mode: 'enforce',
    skills_mode: 'enforce',
    enforcing: true,
    configured_mode: 'enforce',
    demoted: false,
    demoted_reason: '',
    since: '2026-10-13T00:00:00Z',
    breaker_rate_permille: 20,
    breaker_min_samples: 200,
    totals: { total: 5000, mismatched: 1, rejected: 1, rate_permille: 0 },
    levels: [{ level_id: 3, total: 100, mismatched: 1, rate_permille: 10, max_user_mismatches: 1 }],
    ...over,
  }
}

beforeEach(() => {
  fetchRecomputes.mockReset()
  fetchRecomputeStats.mockReset()
  fetchRecomputes.mockResolvedValue({ items: [row()], total: 1 })
  fetchRecomputeStats.mockResolvedValue(summary())
})

async function mountView() {
  const wrapper = mountEl(RecomputesView)
  await settle(wrapper)
  return wrapper
}

describe('闸门状态：熔断告警（这个页面的核心）', () => {
  it('demoted=true 时顶部出现告警，且写明「不再拒发」与降级后的档位', async () => {
    fetchRecomputeStats.mockResolvedValue(
      summary({
        demoted: true,
        // 引擎侧被降级；S 段**没有**被降级。
        mode: 'enforce',
        engine_mode: 'shadow',
        skills_mode: 'enforce',
        enforcing: true,
        configured_mode: 'enforce',
        demoted_reason: '近 1000 次重算中 300 次不符（30%，上限 20‰）',
      }),
    )
    const wrapper = await mountView()
    const alert = wrapper.find('.el-alert')
    expect(alert.exists(), '熔断了却没有告警 —— 值班的人会以为防护还开着').toBe(true)
    const text = alert.text()
    expect(text).toContain('不再拒发奖励')
    // 必须同时出现「配置值」与「生效值」，否则分不清是配成 shadow 还是被降级的
    expect(text).toContain('enforce')
    expect(text).toContain('shadow')
    expect(text).toContain('30%')
    // 「人工升档」这个动作必须写在告警里，否则修完没人知道要改回去
    expect(text).toContain('人工')
    // ⚠️ 2026-10-21（二轮审计 P1）：熔断**只影响引擎侧**。
    // S 段（replay_skills）才是真正拒发奖励的那一段，且它永不降级。
    // 告警里若不说清这一点，运营会以为整个防护都关了 ——
    // 于是去查错的方向、甚至把不需要改的 S 段也改掉。
    expect(text).toContain('只影响引擎侧')
    expect(text).toContain('S 段')
  })

  it('demoted=false 时不出现告警（反复误报的告警等于没有告警）', async () => {
    fetchRecomputeStats.mockResolvedValue(
      summary({ demoted: false, mode: 'enforce', engine_mode: 'enforce', skills_mode: 'enforce' }),
    )
    const wrapper = await mountView()
    expect(wrapper.find('.el-alert').exists()).toBe(false)
  })

  it('配置值与生效值不同才显示「已被降级」注脚；配置成 shadow 则不算降级', async () => {
    // ⚠️ 两种情形数据上都是 `mode !== configured_mode`，
    //   但只有「本来配 enforce、被熔断降下来」才是要立刻处理的事故。
    //   「本来就配成 shadow」是运营自己的选择，不该被标成事故。
    //
    // 各自**独立挂载**：改 mock 之后必须重新 mount，
    // 因为 onMounted 只在挂载时拉一次，settle 不会重跑它。
    fetchRecomputeStats.mockResolvedValue(
      summary({
        demoted: true,
        mode: 'enforce',
        engine_mode: 'shadow',
        skills_mode: 'enforce',
        configured_mode: 'enforce',
      }),
    )
    const demoted = await mountView()
    expect(demoted.text()).toContain('已被降级')

    fetchRecomputeStats.mockResolvedValue(
      summary({
        demoted: false,
        mode: 'shadow',
        engine_mode: 'shadow',
        skills_mode: 'shadow',
        configured_mode: 'shadow',
      }),
    )
    const shadowByConfig = await mountView()
    expect(shadowByConfig.text()).not.toContain('已被降级')
    // 且确实显示了 shadow 档 —— 证明上面那条不是「什么都没渲染」
    expect(shadowByConfig.text()).toContain('shadow')
  })
})

describe('台账行的两处「不是 0」', () => {
  /**
   * 取**最后一张表**的行单元格。
   *
   * ⚠️ 页面里有两张表（按关卡分布 + 台账列表），
   *   直接 `findAll('.el-table__body td')` 会按文档顺序把两张表连起来数，
   *   于是「第 3 列是战报 id」这种断言会落到关卡表上 ——
   *   而且它**不会报错**，只会安静地比错对象（实测踩过：取到 '100'）。
   */
  function ledgerCells(wrapper: Awaited<ReturnType<typeof mountView>>) {
    const tables = wrapper.findAll('.el-table')
    expect(tables.length, '应当正好两张表（关卡分布 + 台账列表）').toBeGreaterThanOrEqual(2)
    return tables[tables.length - 1].findAll('.el-table__body td')
  }

  it('battle_id=0 显示「—」而不是 0（0 表示没有战报，不是战报 #0）', async () => {
    fetchRecomputes.mockResolvedValue({ items: [row({ battle_id: 0 })], total: 1 })
    const wrapper = await mountView()
    const cells = ledgerCells(wrapper)
    // 战报列是第 3 列（ID / 战报 / 用户…）
    expect(cells[1].text().trim()).toBe('—')
  })

  it('battle_id 有值时照常显示（否则上一条会被「全都不显示」满足）', async () => {
    fetchRecomputes.mockResolvedValue({ items: [row({ battle_id: 900 })], total: 1 })
    const wrapper = await mountView()
    expect(ledgerCells(wrapper)[1].text().trim()).toBe('900')
  })

  it('recomputed=null 显示「—」而不是 0 分（引擎未接入 ≠ 算出来是 0）', async () => {
    // 合法战斗里分数不可能算成 0，所以「没算」与「算出 0」必须能区分。
    fetchRecomputes.mockResolvedValue({ items: [row({ recomputed: null })], total: 1 })
    const wrapper = await mountView()
    const text = wrapper.text()
    expect(text).toContain('100 / 10 / 60000ms') // 上报侧
    expect(text).not.toContain('0 / 0 / 0ms') // 重算侧不能出现全 0
  })
})

describe('mode 与 rejected 分开显示', () => {
  it('rejected=true 显示「已拒发」', async () => {
    fetchRecomputes.mockResolvedValue({ items: [row({ mode: 'enforce', rejected: true })], total: 1 })
    const wrapper = await mountView()
    expect(wrapper.text()).toContain('已拒发')
  })

  it('mode=enforce 但 rejected=false 显示「enforce（本次降级）」，而不是只显示 enforce', async () => {
    // 熔断可以把 enforce 下的单次处置降级 —— 两列不等正是这个信号。
    // 只显示 mode 的话，运营问「为什么这批没被拒」时页面上没有答案。
    fetchRecomputes.mockResolvedValue({ items: [row({ mode: 'enforce', rejected: false })], total: 1 })
    const wrapper = await mountView()
    expect(wrapper.text()).toContain('enforce（本次降级）')
  })

  it('mode=shadow 时如实显示 shadow（与上一条区分开）', async () => {
    fetchRecomputes.mockResolvedValue({ items: [row({ mode: 'shadow', rejected: false })], total: 1 })
    const wrapper = await mountView()
    expect(wrapper.text()).toContain('shadow')
    expect(wrapper.text()).not.toContain('本次降级')
  })
})

describe('漂移与作弊的形态提示', () => {
  it('不符集中在单一关卡且分属不同玩家 → 提示像引擎漂移', async () => {
    // max_user_mismatches 远小于 mismatched：不是某一个号在刷
    fetchRecomputeStats.mockResolvedValue(
      summary({
        levels: [{ level_id: 3, total: 100, mismatched: 60, rate_permille: 600, max_user_mismatches: 2 }],
      }),
    )
    const wrapper = await mountView()
    expect(wrapper.text()).toContain('像引擎漂移')
  })

  it('不符几乎全来自同一个号 → 提示像作弊', async () => {
    fetchRecomputeStats.mockResolvedValue(
      summary({
        levels: [{ level_id: 3, total: 100, mismatched: 60, rate_permille: 600, max_user_mismatches: 60 }],
      }),
    )
    const wrapper = await mountView()
    expect(wrapper.text()).toContain('像作弊')
  })

  it('干净的关卡不给任何提示（不制造噪音，否则真出问题时没人看）', async () => {
    fetchRecomputeStats.mockResolvedValue(
      summary({
        levels: [{ level_id: 3, total: 100, mismatched: 0, rate_permille: 0, max_user_mismatches: 0 }],
      }),
    )
    const wrapper = await mountView()
    expect(wrapper.text()).not.toContain('像引擎漂移')
    expect(wrapper.text()).not.toContain('像作弊')
  })
})

describe('阈值来自服务端，不在前端另设一个', () => {
  /**
   * 按**文本**定位那个比率标签。
   *
   * ⚠️ 第一版写的是 `findAll('.el-tag--danger').length > 0`，
   *   变异验证时它**存活了** —— 因为页面上「不符字段」那一列也是 danger 色，
   *   那个断言每次都被无关元素满足。
   *   换句话说它是个装饰品：把 `rateType` 里的下发值换成硬编码 20，它照样绿。
   *
   *   所以必须按文本锁定到比率标签本身。
   */
  function rateTag(wrapper: Awaited<ReturnType<typeof mountView>>, text: string) {
    return wrapper.findAll('.el-tag').find((t) => t.text().trim() === text)
  }

  it('低于阈值 → 不是危险色', async () => {
    // 19‰ = 1.9%，服务端阈值 20‰ → 不到一半阈值 → success
    fetchRecomputeStats.mockResolvedValue(
      summary({ breaker_rate_permille: 20, totals: { total: 1000, mismatched: 19, rejected: 19, rate_permille: 19 } }),
    )
    const wrapper = await mountView()
    const tag = rateTag(wrapper, '1.9%')
    expect(tag, '页面上找不到不符率标签').toBeTruthy()
    expect(tag!.classes()).not.toContain('el-tag--danger')
  })

  it('阈值改了以后严重度跟着变（证明它读的是下发值而不是硬编码）', async () => {
    // 同一个 19‰：服务端把阈值设成 10‰ 时就该变红。
    // 变异验证把 `rateType` 里的下发值换成硬编码 20 后，
    // 第一版断言**没红**（被「不符字段」列的 danger 标签满足）。
    fetchRecomputeStats.mockResolvedValue(
      summary({ breaker_rate_permille: 10, totals: { total: 1000, mismatched: 19, rejected: 19, rate_permille: 19 } }),
    )
    const wrapper = await mountView()
    const tag = rateTag(wrapper, '1.9%')
    expect(tag, '页面上找不到不符率标签').toBeTruthy()
    expect(tag!.classes(), '阈值 10‰ 时 19‰ 应该已经算危险 —— 若这里不红，说明前端硬编码了阈值').toContain(
      'el-tag--danger',
    )
  })
})

describe('部分失败时的降级行为', () => {
  it('总账接口挂了仍要显示台账列表（概览失败不该连证据一起丢）', async () => {
    fetchRecomputeStats.mockRejectedValue(new Error('boom'))
    const wrapper = await mountView()
    // 列表仍在
    expect(wrapper.text()).toContain('已拒发')
    // 且给了明确提示，而不是静默
    expect(bodyText()).toContain('总账加载失败')
  })

  it('列表接口挂了要提示，且不假装有数据', async () => {
    fetchRecomputes.mockRejectedValue(new Error('boom'))
    await mountView()
    expect(bodyText()).toContain('重算台账加载失败')
  })
})

describe('筛选参数', () => {
  it('默认只看不符，且带上窗口/分页以外的筛选键', async () => {
    await mountView()
    const params = fetchRecomputes.mock.calls[0][0]
    expect(params.only_mismatch).toBe(true)
    expect(params.limit).toBe(50)
  })

  it('总账按所选时间窗请求（熔断看 10 分钟、漂移看 24 小时，两个尺度）', async () => {
    await mountView()
    expect(fetchRecomputeStats.mock.calls[0][0]).toBe(24)
  })
})
