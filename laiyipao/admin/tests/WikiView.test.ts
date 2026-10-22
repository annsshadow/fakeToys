/**
 * WikiView.vue 测试。
 *
 * 文档站直接读线上 /config：字段名与 Go json tag 的对齐（reactions/chapters
 * 曾因 PascalCase 静默空白）靠渲染断言兜底；抗性条的正负与钳制、
 * 实战分布的排序与「主流打法」提示、dashboard 失败时的静默降级都要锁。
 *
 * 2026-10-22 加目录 / 搜索 / 深链后，本文件多了三组用例：
 *  - 目录锚点必须都能在 DOM 里找到（`SECTIONS` 与模板 id 的漂移守卫）
 *  - 深链 `?tab=&anchor=&focus=` 的解析与回落
 *  - 搜索命中的跳转（含「关卡焦点要先切章节」这条最容易漏的联动）
 */
import { flushPromises, mount } from '@vue/test-utils'
import { ElSelect } from 'element-plus'
import ElementPlus from 'element-plus'
import { createMemoryHistory, createRouter, type Router } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const apiGet = vi.hoisted(() => vi.fn())
// 2026-10-21（P2-5）：文档站现在用 **soft** 版看板请求
// （401 是预期答案，不该把访客踢去运营后台登录页）。
const fetchDashboardSoft = vi.hoisted(() => vi.fn())

vi.mock('@/api/client', () => ({ api: { get: apiGet } }))
vi.mock('@/api', () => ({ fetchDashboardSoft }))

import WikiView from '@/views/WikiView.vue'

const configFixture = () => ({
  levels: [
    {
      id: 1, chapter: 1, name: '初见', is_boss: false, terrain: [], difficulty: 1000,
      base_hp: 1000, wave_count: 3, energy_cost: 6, element_cap: 5, max_reaction_tier: 2,
      star_targets: [60, 120, 180],
    },
    {
      id: 6, chapter: 2, name: '进阶', is_boss: true, terrain: [], difficulty: 1800,
      base_hp: 2000, wave_count: 5, energy_cost: 8, element_cap: 8, max_reaction_tier: 3,
      star_targets: [50, 100, 150],
    },
  ],
  enemies: [
    {
      id: 1, code: 'slime', name: '酸液史莱姆', category: 'normal', hp: 100,
      resist: { fire: 500, ice: -300, lightning: 0, corrosion: 0, kinetic: 0 }, descr: '怕冰',
    },
    {
      id: 2, code: 'boss_dragon', name: '熔岩龙', category: 'boss', hp: 99999,
      resist: { fire: 900, ice: -500, lightning: -200, corrosion: 0, kinetic: 100 }, descr: '两弱点',
    },
    {
      id: 3, code: 'ghost', name: '无抗性体', category: 'turret', hp: 50, descr: '未知类型回退原文',
    },
  ],
  skills: [
    { id: 1, name: '火弹', element: 'fire', family: 'flame', kind: 'active', base_damage: 100, heat_cost: 10, cooldown_ms: 500, pierce: 0, aoe_radius: 0, apply_stacks: 2, descr: '火' },
  ],
  composite_skills: [{ id: 9, name: '超导融合', element: 'lightning', descr: '冰+电' }],
  reactions: [
    { key: 'steam_burst', name: '蒸汽爆发', base_coef: 1500, attack_weight_pct: 300, status_duration_ms: 5000, aoe_radius: 120, dispel_shield: false, amplify_pct: 0 },
    { key: 'superconduct', name: '超导', base_coef: 2000, attack_weight_pct: 200, status_duration_ms: 8000, aoe_radius: 2, dispel_shield: true, amplify_pct: 20 },
  ],
  chapters: [
    { id: 1, name: '第一章', start_level: 1, end_level: 10, terrain_kind: 'plain', boss_enemy_id: 2 },
    { id: 2, name: '第二章', start_level: 11, end_level: 20, terrain_kind: 'cave', boss_enemy_id: 2 },
  ],
  rating_weights: { damage: 30, survival: 20, element: 25, economy: 15, tempo: 10 },
  score_rules: {
    per_damage_unit: 50,
    on_kill_normal: 100,
    on_kill_boss: 800,
    star_target_ratio: [600, 850, 980],
    score_full_at_sec: 60,
  },
  skill_rules: { max_level: 5, coef_permille: 120, base_cost: 100 },
  mastery_families: [
    { family: 'flame', name: '焰系', nodes: [{ name: '引燃', layer: 1 }, { name: '爆燃', layer: 2 }] },
  ],
})

/**
 * 挂载辅助。
 *
 * ⚠️ 必须提供真实 router：文档站的「深链」就是 URL（`?tab=&anchor=&focus=`），
 * 组件通过 `useRoute()/useRouter()` 读写它。用 `createMemoryHistory`
 * 而不是真实 hash history —— 后者会去改 `window.location.hash`，
 * 而 `client_unauthorized.test.ts` 正在别处断言 hash 不被改动。
 */
let lastRouter: Router

async function mountWiki(query: Record<string, string> = {}) {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [{ path: '/wiki', name: 'wiki', component: { template: '<div />' } }],
  })
  await router.push({ path: '/wiki', query })
  await router.isReady()
  lastRouter = router

  const wrapper = mount(WikiView, { global: { plugins: [ElementPlus, router] } })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
  apiGet.mockResolvedValue(configFixture())
  fetchDashboardSoft.mockResolvedValue({
    reaction_usage: [
      { reaction: 'overheat', count: 50 },
      { reaction: 'steam_burst', count: 10 },
    ],
  })
})

describe('WikiView 配置加载', () => {
  it('以免鉴权方式请求 /config，且 dashboard 失败不影响文档渲染', async () => {
    fetchDashboardSoft.mockRejectedValueOnce(new Error('dashboard 401'))
    const wrapper = await mountWiki()

    expect(apiGet).toHaveBeenCalledWith('/config', { auth: false })
    const text = wrapper.text()
    expect(text).toContain('玩家玩法文档站')
    // 实战分布降级为空 → 无「主流打法」提示，但页面正常
    expect(text).not.toContain('当前主流打法是')
    wrapper.unmount()
  })

  it('config 请求失败：弹出错误消息', async () => {
    apiGet.mockRejectedValueOnce(new Error('config 不可达'))
    const wrapper = await mountWiki()
    expect(document.body.textContent).toContain('配置加载失败：config 不可达')
    wrapper.unmount()
  })
})

describe('WikiView 核心规则页签', () => {
  it('反应表：攻击力权重 ≥300 预警为 warning 色阶，溅射半径按值渲染', async () => {
    const wrapper = await mountWiki()
    const rows = wrapper.findAll('.el-table__row')
    const steamRow = rows.find((r) => r.text().includes('蒸汽爆发'))!
    const scRow = rows.find((r) => r.text().includes('超导'))!
    // ⚠️ 只看「攻击力权重」那一列的 tag（文本带 ‰），不能用整行的
    // .el-tag--warning —— 新加的「受击增伤」列在 amplify_pct>0 时也发
    // warning tag，整行匹配会把两列混为一谈（超导 amplify 20 就是这样）。
    const weightTag = (r: (typeof rows)[number]) =>
      r.findAll('.el-tag').find((t) => t.text().includes('‰'))!
    expect(weightTag(steamRow).classes()).toContain('el-tag--warning') // 300 → 预警
    expect(steamRow.text()).toContain('120') // aoe_radius 120（真实值）
    expect(weightTag(scRow).classes()).toContain('el-tag--info') // 200 → info
    expect(scRow.text()).toContain('2') // aoe_radius 2
    wrapper.unmount()
  })

  it('评分权重表把对象展开成维度行', async () => {
    const wrapper = await mountWiki()
    const text = wrapper.text()
    expect(text).toContain('damage')
    expect(text).toContain('30')
    expect(text).toContain('tempo')
    wrapper.unmount()
  })

  it('反应表新列：控制文案按 key 分类、受击增伤/驱散护盾按标志显示', async () => {
    // statusText 有 4 个 key 分支（冻结/减速/眩晕/控制兜底），加上
    // status_duration_ms=0 的「—」分支；受击增伤、驱散护盾各有真/假两态。
    // 这些列此前无断言 —— 改坏 key 分派或标志判断都不会红，这里逐格钉住。
    apiGet.mockResolvedValueOnce({
      ...configFixture(),
      reactions: [
        { key: 'flash_freeze', name: '闪冻', base_coef: 1000, attack_weight_pct: 100, status_duration_ms: 3000, aoe_radius: 0, dispel_shield: false, amplify_pct: 0 },
        { key: 'superconduct', name: '超导', base_coef: 2000, attack_weight_pct: 200, status_duration_ms: 8000, aoe_radius: 2, dispel_shield: true, amplify_pct: 20 },
        { key: 'overheat', name: '过热', base_coef: 1200, attack_weight_pct: 300, status_duration_ms: 2000, aoe_radius: 1, dispel_shield: false, amplify_pct: 0 },
        { key: 'burn_cloud', name: '燃云', base_coef: 900, attack_weight_pct: 150, status_duration_ms: 0, aoe_radius: 3, dispel_shield: false, amplify_pct: 0 },
      ],
    })
    const wrapper = await mountWiki()
    const rows = wrapper.findAll('.el-table__row')
    const row = (n: string) => rows.find((r) => r.text().includes(n))!
    expect(row('闪冻').text()).toContain('冻结 3.0s') // flash_freeze 分支
    expect(row('闪冻').text()).toContain('单体') // aoe_radius 0 → 回退文案
    expect(row('超导').text()).toContain('减速 8.0s') // superconduct 分支
    expect(row('过热').text()).toContain('眩晕 2.0s') // overheat 分支
    expect(row('燃云').text()).not.toContain('s') // status 0 → muted「—」，无秒数
    // 受击增伤：只有超导 amplify>0
    expect(row('超导').text()).toContain('+20%')
    expect(row('闪冻').findAll('.el-tag--warning').length).toBe(0)
    // 驱散护盾：只有超导可驱散
    expect(row('超导').text()).toContain('可驱散')
    expect(row('过热').find('.el-tag--danger').exists()).toBe(false)
    wrapper.unmount()
  })
})

describe('WikiView 数值规则页', () => {
  it('score_rules / skill_rules 有数据：星级比例三档 + 满级费用按公式现算', async () => {
    // 费用是线性 base_cost×等级，满级总额 = base_cost × n(n-1)/2。
    // 100 × 5×4/2 = 1000。写死数字会与服务端 CostFrom 漂移，这里验证
    // 页面确实用下发的 base_cost/max_level 现算。
    const wrapper = await mountWiki()
    const text = wrapper.text()
    expect(text).toContain('50') // per_damage_unit
    expect(text).toContain('800') // on_kill_boss
    expect(text).toContain('600‰') // 一星比例
    expect(text).toContain('980‰') // 三星比例
    expect(text).toContain('升到满级共需')
    expect(text).toContain('1000') // fullUpgradeCost = 100×5×4/2
    const vm = wrapper.vm as unknown as {
      fullUpgradeCost: number | string
      starRatioRows: Array<{ label: string; v: string }>
    }
    expect(vm.fullUpgradeCost).toBe(1000)
    expect(vm.starRatioRows.map((r) => r.label)).toEqual(['一星', '二星', '三星'])
    wrapper.unmount()
  })

  it('缺 score_rules / skill_rules：全部回退「—」，满级费用与星级表空', async () => {
    // 老后端或裁剪过的 /config 可能不带这两个规则对象。页面必须
    // 静默回退而不是抛错。这条覆盖 `?? '—'` 与 `if(!r) return '—'` 的兜底。
    const { score_rules, skill_rules, ...rest } = configFixture()
    void score_rules
    void skill_rules
    apiGet.mockResolvedValueOnce(rest)
    const wrapper = await mountWiki()
    const vm = wrapper.vm as unknown as {
      fullUpgradeCost: number | string
      starRatioRows: unknown[]
    }
    expect(vm.fullUpgradeCost).toBe('—')
    expect(vm.starRatioRows).toEqual([])
    wrapper.unmount()
  })

  it('星级比例超过三档时用「N 星」兜底标签（防御性分支）', async () => {
    // 设计上恒三档，但 star_target_ratio 是网络数据，来第 4 个值时
    // labels[3] 落空要回退「4 星」而不是 undefined。这条钉住那个兜底。
    apiGet.mockResolvedValueOnce({
      ...configFixture(),
      score_rules: { ...configFixture().score_rules, star_target_ratio: [500, 700, 900, 990] },
    })
    const wrapper = await mountWiki()
    const vm = wrapper.vm as unknown as { starRatioRows: Array<{ label: string; v: string }> }
    expect(vm.starRatioRows.map((r) => r.label)).toEqual(['一星', '二星', '三星', '4 星'])
    expect(vm.starRatioRows[3].v).toBe('990‰')
    wrapper.unmount()
  })
})

describe('WikiView 敌人抗性图鉴', () => {
  it('BOSS 与普通类型分别标色；抗性条覆盖正/负/零/缺失四种取值；未知类型回退原文', async () => {
    const wrapper = await mountWiki()
    const rows = wrapper.findAll('.el-table__row')
    const bossRow = rows.find((r) => r.text().includes('熔岩龙'))!
    const normalRow = rows.find((r) => r.text().includes('酸液史莱姆'))!
    const ghostRow = rows.find((r) => r.text().includes('无抗性体'))!

    expect(bossRow.find('.el-tag--danger').exists()).toBe(true)
    expect(bossRow.text()).toContain('BOSS')
    expect(normalRow.text()).toContain('普通')

    // 未知 category：中文映射表没有 → 回退原始值
    expect(ghostRow.text()).toContain('turret')

    // 五维都渲染：抗性数值换算成百分数（resist/10）
    expect(normalRow.text()).toContain('50%') // fire 500 → 50%
    expect(normalRow.text()).toContain('-30%') // ice -300 → 弱点
    expect(normalRow.text()).toContain('0%') // lightning 0
    // 无 resist 字段的敌人：全部回退 0
    expect(ghostRow.text()).toContain('0%')
    wrapper.unmount()
  })

  it('resistPct 钳制到 ±50（超出范围的原始抗性值不会撑爆进度条）', async () => {
    const wrapper = await mountWiki()
    const vm = wrapper.vm as unknown as { resistPct: (v: number) => number; resistColor: (v: number) => string }
    expect(vm.resistPct(1000)).toBe(50)
    expect(vm.resistPct(-1000)).toBe(-50)
    expect(vm.resistPct(300)).toBe(30)
    expect(vm.resistColor(5)).toBe('#58a6ff')
    expect(vm.resistColor(-5)).toBe('#7ee787')
    wrapper.unmount()
  })
})

describe('WikiView 技能/专精/关卡页签', () => {
  it('目录切节（不再是 el-tabs）；关卡下拉 v-model 联动章节过滤', async () => {
    const wrapper = await mountWiki()
    const text = wrapper.text()
    expect(text).toContain('火弹')
    expect(text).toContain('超导融合')
    expect(text).toContain('引燃')
    expect(text).toContain('爆燃')
    // 默认第 1 章 → 只显示初见关
    expect(text).toContain('初见')
    expect(text).not.toContain('进阶')

    // 点左侧目录的「技能图鉴」
    const navItem = wrapper.findAll('.nav-item').find((t) => t.text() === '技能图鉴')!
    expect(navItem, '目录里没有「技能图鉴」').toBeTruthy()
    await navItem.trigger('click')
    await flushPromises()
    const vm = wrapper.vm as unknown as { activeTab: string }
    expect(vm.activeTab).toBe('skills')
    // 目录点击要写回 URL —— 否则「复制地址栏」分享不到当前这一节
    expect(lastRouter.currentRoute.value.query.tab).toBe('skills')
    expect(navItem.classes()).toContain('on')

    // 章节下拉切换到第 2 章（v-model 处理器真实执行）
    const select = wrapper.findComponent(ElSelect)
    await select.vm.$emit('update:modelValue', 2)
    await flushPromises()
    expect(wrapper.text()).toContain('进阶')
    expect(vm.activeTab).toBe('skills')
    wrapper.unmount()
  })
})

describe('WikiView 目录（2026-10-22）', () => {
  it('每个目录锚点都能在 DOM 里找到（SECTIONS 与模板 id 的漂移守卫）', async () => {
    // ⚠️ 这条守的是**两处手写 id 的一致性**：SECTIONS 里写着 `reactions`，
    // 模板里写着 `id="reactions"`。改一处忘另一处时页面不报错，
    // 只是点目录「什么都不发生」—— 那种缺陷没人会主动测。
    const wrapper = await mountWiki()
    const anchors: string[] = []
    // 从 DOM 反查：目录里每个二级项都带 href=#/wiki?tab=x&anchor=y
    for (const a of wrapper.findAll('.nav-sub-item')) {
      const href = a.attributes('href') ?? ''
      const m = /[?&]anchor=([^&]+)/.exec(href)
      if (m) anchors.push(decodeURIComponent(m[1]!))
    }
    expect(anchors.length).toBeGreaterThanOrEqual(6) // basics 2 + skills 2 + rules 2
    for (const id of anchors) {
      // ⚠️ 用 wrapper.find 而不是 document.getElementById：
      // vue-test-utils 默认挂载在**游离**容器里，不在 document 树上。
      expect(wrapper.find(`#${id}`).exists(), `目录里的锚点 #${id} 在页面上不存在`).toBe(true)
    }
    wrapper.unmount()
  })

  it('目录项是真链接（href 指向 #/wiki?tab=…），可复制 / 新标签打开', async () => {
    const wrapper = await mountWiki()
    const items = wrapper.findAll('.nav-item')
    expect(items.length).toBe(8) // 8 个 section
    const enemies = items.find((t) => t.text() === '敌人抗性图鉴')!
    expect(enemies.attributes('href')).toContain('/wiki?tab=enemies')
    // 默认节（basics）不带 tab 参数，链接更短
    expect(items[0]!.attributes('href')).not.toContain('tab=')
    wrapper.unmount()
  })
})

describe('WikiView 深链（2026-10-22）', () => {
  it('?tab= 直接落到对应章节', async () => {
    const wrapper = await mountWiki({ tab: 'equipment' })
    const vm = wrapper.vm as unknown as { activeTab: string }
    expect(vm.activeTab).toBe('equipment')
    // 当前节的目录项高亮
    expect(wrapper.findAll('.nav-item').find((t) => t.text() === '装备图鉴')!.classes()).toContain('on')
    wrapper.unmount()
  })

  it('?tab=&focus=enemy:2 会高亮那一行（不是只切了页）', async () => {
    const wrapper = await mountWiki({ tab: 'enemies', focus: 'enemy:2' })
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')
    const focused = rows.filter((r) => r.classes().includes('row-focus'))
    expect(focused.length, '没有任何行拿到 row-focus —— 搜索/深链跳过去也看不出跳到哪').toBe(1)
    expect(focused[0]!.text()).toContain('熔岩龙')
    wrapper.unmount()
  })

  it('?focus=level:6 先把章节切到第 2 章（否则那一行根本不在表里）', async () => {
    const wrapper = await mountWiki({ tab: 'levels', focus: 'level:6' })
    await flushPromises()
    const vm = wrapper.vm as unknown as { activeChapter: number }
    expect(vm.activeChapter).toBe(2)
    expect(wrapper.text()).toContain('进阶')
    const focused = wrapper.findAll('.el-table__row').filter((r) => r.classes().includes('row-focus'))
    expect(focused.length).toBe(1)
    expect(focused[0]!.text()).toContain('进阶')
    wrapper.unmount()
  })

  it('非法深链（未知 tab / 畸形 focus）→ 回落到核心规则，不抛错', async () => {
    const wrapper = await mountWiki({ tab: 'nope', focus: 'bogus' })
    const vm = wrapper.vm as unknown as { activeTab: string; focus: unknown }
    expect(vm.activeTab).toBe('basics')
    expect(vm.focus).toBeNull()
    // 不认识的 focus 前缀（缺冒号 / 未知 kind）都按「没有焦点」处理
    const w2 = await mountWiki({ tab: 'enemies', focus: 'enemy' })
    expect((w2.vm as unknown as { focus: unknown }).focus).toBeNull()
    const w3 = await mountWiki({ tab: 'enemies', focus: 'unknownkind:1' })
    expect((w3.vm as unknown as { focus: unknown }).focus).toBeNull()
    wrapper.unmount()
    w2.unmount()
    w3.unmount()
  })

  it('?anchor= 写进 URL 后能被回读（目录二级项与深链是同一套 key）', async () => {
    const wrapper = await mountWiki()
    const sub = wrapper.findAll('.nav-sub-item').find((a) => a.text() === '合成技能')!
    await sub.trigger('click')
    await flushPromises()
    expect(lastRouter.currentRoute.value.query).toMatchObject({ tab: 'skills', anchor: 'composite' })
    wrapper.unmount()
  })
})

describe('WikiView 搜索（2026-10-22）', () => {
  async function search(wrapper: Awaited<ReturnType<typeof mountWiki>>, q: string) {
    await wrapper.find('.search-input').setValue(q)
    await flushPromises()
  }

  it('跨类型命中：技能名 / 敌人名 / 关卡名都能搜到', async () => {
    const wrapper = await mountWiki()
    await search(wrapper, '熔岩龙')
    let hits = wrapper.findAll('.search-hit')
    expect(hits.length).toBe(1)
    expect(hits[0]!.text()).toContain('敌人')

    await search(wrapper, '火弹')
    hits = wrapper.findAll('.search-hit')
    expect(hits[0]!.text()).toContain('技能')

    await search(wrapper, '初见')
    hits = wrapper.findAll('.search-hit')
    expect(hits[0]!.text()).toContain('关卡')
    wrapper.unmount()
  })

  it('点命中 → 切到对应章节并高亮那一行', async () => {
    const wrapper = await mountWiki()
    await search(wrapper, '超导融合')
    await wrapper.findAll('.search-hit')[0]!.trigger('click')
    await flushPromises()

    const vm = wrapper.vm as unknown as { activeTab: string; searchOpen: boolean }
    expect(vm.activeTab).toBe('skills')
    expect(vm.searchOpen).toBe(false) // 选中后收起面板
    const focused = wrapper.findAll('.el-table__row').filter((r) => r.classes().includes('row-focus'))
    expect(focused.length).toBe(1)
    expect(focused[0]!.text()).toContain('超导融合')
    expect(lastRouter.currentRoute.value.query.focus).toBe('composite:9')
    wrapper.unmount()
  })

  it('搜关卡名 → 章节联动（第 2 章的关要能跳过去）', async () => {
    const wrapper = await mountWiki()
    await search(wrapper, '进阶')
    await wrapper.findAll('.search-hit')[0]!.trigger('click')
    await flushPromises()
    const vm = wrapper.vm as unknown as { activeTab: string; activeChapter: number }
    expect(vm.activeTab).toBe('levels')
    expect(vm.activeChapter).toBe(2)
    wrapper.unmount()
  })

  it('无命中 → 空态文案；回车不崩（没有第一条可跳）', async () => {
    const wrapper = await mountWiki()
    await search(wrapper, '不存在的东西')
    expect(wrapper.text()).toContain('没有匹配「不存在的东西」的内容')
    await wrapper.find('.search-input').trigger('keydown.enter')
    await flushPromises()
    const vm = wrapper.vm as unknown as { activeTab: string }
    expect(vm.activeTab).toBe('basics') // 原地不动
    wrapper.unmount()
  })

  it('回车跳第一条；命中超过 20 条时给出「还有 N 条」的提示', async () => {
    // 造 25 个同名技能，验证截断 + 提示
    apiGet.mockResolvedValueOnce({
      ...configFixture(),
      skills: Array.from({ length: 25 }, (_, i) => ({
        id: 100 + i, name: `测试技能${i}`, element: 'fire', family: 'flame', kind: 'active',
        base_damage: 1, heat_cost: 1, cooldown_ms: 1, pierce: 0, aoe_radius: 0, apply_stacks: 1, descr: '测试',
      })),
    })
    const wrapper = await mountWiki()
    await search(wrapper, '测试技能')
    expect(wrapper.findAll('.search-hit').length).toBe(20)
    expect(wrapper.text()).toContain('共 25 条')

    await wrapper.find('.search-input').trigger('keydown.enter')
    await flushPromises()
    expect((wrapper.vm as unknown as { activeTab: string }).activeTab).toBe('skills')
    wrapper.unmount()
  })
})

describe('WikiView 实战分布', () => {
  it('按触发次数降序展示占比，主流打法提示取榜首', async () => {
    const wrapper = await mountWiki()
    const text = wrapper.text()
    expect(text).toContain('当前主流打法是「overheat」')
    const rows = wrapper.findAll('.el-table__row')
    const usageRows = rows.filter((r) => r.text().includes('overheat') || r.text().includes('steam_burst'))
    expect(usageRows[0].text()).toContain('overheat') // 50 次在前
    expect(usageRows[1].text()).toContain('steam_burst') // 10 次在后
    wrapper.unmount()
  })

  it('无实战数据时 reactUsage 为空、topReaction 为 undefined（不渲染提示）', async () => {
    fetchDashboardSoft.mockResolvedValueOnce({ reaction_usage: [] })
    const wrapper = await mountWiki()
    expect(wrapper.text()).not.toContain('当前主流打法是')
    const vm = wrapper.vm as unknown as { topReaction: { reaction: string } | undefined }
    expect(vm.topReaction).toBeUndefined()
    wrapper.unmount()
  })
})
