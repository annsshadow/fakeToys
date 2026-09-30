/**
 * WikiView.vue 测试。
 *
 * 文档站直接读线上 /config：字段名与 Go json tag 的对齐（reactions/chapters
 * 曾因 PascalCase 静默空白）靠渲染断言兜底；抗性条的正负与钳制、
 * 实战分布的排序与「主流打法」提示、dashboard 失败时的静默降级都要锁。
 */
import { flushPromises, mount } from '@vue/test-utils'
import { ElSelect } from 'element-plus'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const apiGet = vi.hoisted(() => vi.fn())
const fetchDashboard = vi.hoisted(() => vi.fn())

vi.mock('@/api/client', () => ({ api: { get: apiGet } }))
vi.mock('@/api', () => ({ fetchDashboard }))

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
    { key: 'steam_burst', name: '蒸汽爆发', base_coef: 1500, attack_weight_pct: 300, status_duration_ms: 5000, aoe_radius: 0, dispel_shield: false, amplify_pct: 0 },
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

async function mountWiki() {
  const wrapper = mount(WikiView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
  apiGet.mockResolvedValue(configFixture())
  fetchDashboard.mockResolvedValue({
    reaction_usage: [
      { reaction: 'overheat', count: 50 },
      { reaction: 'steam_burst', count: 10 },
    ],
  })
})

describe('WikiView 配置加载', () => {
  it('以免鉴权方式请求 /config，且 dashboard 失败不影响文档渲染', async () => {
    fetchDashboard.mockRejectedValueOnce(new Error('dashboard 401'))
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
  it('反应表：攻击力权重 ≥300 预警为 warning 色阶，溅射 0 显示「单体」', async () => {
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
    expect(steamRow.text()).toContain('单体') // aoe_radius 0
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
  it('页签切换走 tabs 的 v-model；关卡下拉 v-model 联动章节过滤', async () => {
    const wrapper = await mountWiki()
    const text = wrapper.text()
    expect(text).toContain('火弹')
    expect(text).toContain('超导融合')
    expect(text).toContain('引燃')
    expect(text).toContain('爆燃')
    // 默认第 1 章 → 只显示初见关
    expect(text).toContain('初见')
    expect(text).not.toContain('进阶')

    // 点击「技能图鉴」页签 → el-tabs 发出 update:modelValue
    await wrapper.findAll('.el-tabs__item').find((t) => t.text() === '技能图鉴')!.trigger('click')
    await flushPromises()
    const vm = wrapper.vm as unknown as { activeTab: string }
    expect(vm.activeTab).toBe('skills')

    // 章节下拉切换到第 2 章（v-model 处理器真实执行）
    const select = wrapper.findComponent(ElSelect)
    await select.vm.$emit('update:modelValue', 2)
    await flushPromises()
    expect(wrapper.text()).toContain('进阶')
    expect(vm.activeTab).toBe('skills')
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
    fetchDashboard.mockResolvedValueOnce({ reaction_usage: [] })
    const wrapper = await mountWiki()
    expect(wrapper.text()).not.toContain('当前主流打法是')
    const vm = wrapper.vm as unknown as { topReaction: { reaction: string } | undefined }
    expect(vm.topReaction).toBeUndefined()
    wrapper.unmount()
  })
})
