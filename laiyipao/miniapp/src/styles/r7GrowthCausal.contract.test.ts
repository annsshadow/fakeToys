import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const repoRoot = join(__dirname, '..', '..', '..', '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const readRepo = (p: string) => readFileSync(join(repoRoot, p), 'utf-8')
const tpl = (src: string) => src.slice(0, src.indexOf('</template>'))

const battleDetail = {
  id: 321,
  level_id: 42,
  result: 'win',
  stars: 0,
  score: 10909,
  kills: 39,
  leaked: 0,
  hp_left: 80,
  wave_reached: 12,
  reactions: 91,
  heat_max: 100,
  shots: 120,
  hits: 110,
  duration_ms: 74000,
  elements_used: { fire: 40, ice: 30, lightning: 21 },
  reactions_used: { 'fire+ice': 40, 'ice+lightning': 30, 'fire+lightning': 21 },
  terrain_used: ['desert'],
  loot: {},
  replay_hash: 'abcdef0123456789',
  created_at: '2026-11-21T10:00:00Z',
}

describe('R7 成长页推荐因果解释：本局靠什么拿分 → 换上后预计提升', () => {
  it('§1F.4 规范要求解释因果、收益字段与两步确认，且不编造数值', () => {
    const spec = readRepo('docs/laiyipao/design/AI_VISUAL_SYSTEM.md')
    for (const s of ['为什么是它', 'evidence[]', 'input_snapshot', 'expected_gain', '当前 vs 推荐', '确认替换', '禁止无确认']) {
      expect(spec, '规范缺少 ' + s).toContain(s)
    }
    const quick = readRepo('docs/laiyipao/design/TOKENS_QUICK_REF.md')
    expect(quick).toContain('两步 CTA')
  })

  it('令牌锚点保持 v1 / v3-r3 / r6，无 v4 或新增 r7 变量', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--r7-[a-z-]+:/)
  })

  it('共享因果 helper 只从真实战报字段提取，不写死分数和反应数', async () => {
    const src = readSrc('pages/_shared/r7GameplayCausal.ts')
    for (const s of ['myBattleHistory', 'myBattleDetail', 'reactions_used', 'elements_used', 'score', 'stars', 'leaked', '没有战报']) {
      expect(src, 'helper 缺少 ' + s).toContain(s)
    }
    expect(src).not.toMatch(/19726|10909|11941/)
    const mod = await import('../pages/_shared/r7GameplayCausal')
    expect(typeof mod.fetchLatestBattleCausal).toBe('function')
    expect(typeof mod.normalizeBattleCausal).toBe('function')
  })

  it('normalizeBattleCausal 从真实战报生成可读因果链，缺字段时诚实空态', async () => {
    const mod = await import('../pages/_shared/r7GameplayCausal')
    const view = mod.normalizeBattleCausal(battleDetail)
    expect(view.hasHistory).toBe(true)
    expect(view.score).toBe(10909)
    expect(view.reactions).toBe(91)
    expect(view.leaked).toBe(0)
    expect(view.text).toContain('本局靠')
    expect(view.text).toContain('反应')
    expect(view.text).toContain('10909')
    expect(view.topElements.length).toBeGreaterThan(0)
    expect(view.topReactions.length).toBeGreaterThan(0)
    expect(view.source).toContain('321')
    const empty = mod.normalizeBattleCausal(null)
    expect(empty.hasHistory).toBe(false)
    expect(empty.text).toContain('没有战报')
    expect(empty.text).not.toMatch(/\d{4,}/)
  })

  const pages = ['skills', 'equipment', 'gems'] as const
  for (const page of pages) {
    it(page + ' 模板展示「本局靠什么拿分 → 换上后预计提升」与诚实空态', () => {
      const markup = tpl(readSrc('pages/' + page + '/' + page + '.vue'))
      for (const cls of ['r7-causal', 'r7-causal-empty', 'r7-causal-next']) {
        expect(markup, page + ' 缺少 ' + cls).toContain(cls)
      }
      expect(markup).toContain('本局靠什么拿分')
      expect(markup).toContain('换上后预计提升')
      expect(markup).toContain('AI')
      expect(markup).toContain('规则引擎')
      expect(markup).not.toContain('预计提升 +12')
    })

    it(page + ' script 读取真实战报与推荐，写配置只在确认后', () => {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      expect(src).toContain('fetchLatestBattleCausal')
      expect(src).toContain('r7Causal')
      expect(src).toMatch(/收益数据不可用|不展示虚构收益/)
      if (page !== 'gems') {
        const open = src.slice(src.indexOf('function r7OpenCompare'), src.indexOf('function r7ConfirmApply'))
        expect(open).not.toMatch(/persistLoadout|saveEquippedEquipment|applyEquipmentImpact|equipGem/)
      }
    })
  }

  it('宝石页推荐只做导航与诚实标注，不写出战配置', () => {
    const src = readSrc('pages/gems/gems.vue')
    expect(src).toContain('本地规则引擎')
    expect(src).not.toContain('persistLoadout')
    expect(src).not.toContain('saveEquippedEquipment')
    expect(src).toContain('loadPage()')
  })

  it('因果卡复用既有令牌并提供 reduced-motion 关闭入口', () => {
    const css = readSrc('styles/common.css')
    expect(css).toContain('LYP-R7-CAUSAL-V1')
    for (const cls of ['.r7-causal', '.r7-causal-empty', '.r7-causal-next', '.r7-causal-enter']) {
      expect(css, '缺少 ' + cls).toContain(cls)
    }
    expect(css).toContain('var(--panel-2)')
    expect(css).toContain('var(--ai-accent)')
    expect(css).toMatch(/prefers-reduced-motion[\s\S]{0,200}\.r7-causal-enter/)
  })

  it('不回退 R6 诚实空态、R5 文案与 R4 顶栏脱敏标记', () => {
    const skills = readSrc('pages/skills/skills.vue')
    const equipment = readSrc('pages/equipment/equipment.vue')
    for (const src of [skills, equipment]) {
      expect(src).toContain("r6Rec.status === 'empty'")
      expect(src).toContain('收益数据不可用')
      expect(src).toContain('r6-rec')
    }
    const rank = readSrc('pages/rank/rank.vue')
    expect(rank).toContain('{{ it.nickname || \'玩家\' }}')
  })
})
