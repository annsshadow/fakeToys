import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const repoRoot = join(__dirname, '..', '..', '..', '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const readRepo = (p: string) => readFileSync(join(repoRoot, p), 'utf-8')
const tpl = (src: string) => src.slice(0, src.indexOf('</template>'))

const FOCUS = {
  equipment: {
    context: 'equipment',
    legacy: 'r3AiEquip',
    topic: /宝石|护甲|属性|搭配/,
    notTopic: /触发率/,
  },
  reactions: {
    context: 'reactions',
    legacy: 'r3AiReaction',
    topic: /反应|触发|元素/,
    notTopic: /护甲/,
  },
  skills: {
    context: 'skills',
    legacy: 'r3AiSkill',
    topic: /技能|热量|热度/,
    notTopic: /护甲/,
  },
} as const

type FocusPage = keyof typeof FOCUS

describe('R6 H5 真实数据驱动推荐契约', () => {
  it('§1E 七个子条款、31 界面审计与接口字段齐全，且写明本轮不实现后端', () => {
    const spec = readRepo('docs/laiyipao/design/AI_VISUAL_SYSTEM.md')
    for (const clause of ['1E.1', '1E.2', '1E.3', '1E.4', '1E.5', '1E.6', '1E.7']) {
      expect(spec, '缺少 ' + clause).toContain(clause)
    }
    for (const page of ['equipment.vue', 'reactions.vue', 'skills.vue', 'login.vue', 'verify.vue']) {
      expect(spec, '审计缺少 ' + page).toContain(page)
    }
    expect(spec).toContain('ContentView')
    expect(spec).toContain('WikiView')
    expect(spec).toContain('浅升级')
    expect(spec).toContain('无 AI')
    expect(spec).toContain('/api/v1/me/recommendations?context=')
    for (const field of ['schema_version', 'generated_at', 'context', 'evidence', 'insights', 'model', 'confidence', 'cta', 'empty_reason', 'NO_HISTORY']) {
      expect(spec, '接口字段缺少 ' + field).toContain(field)
    }
    expect(spec).toContain('本轮**不实现**')
    expect(spec).toContain('元素反应链')
    expect(spec).toContain('装备/宝石搭配')
    expect(spec).toContain('关卡难度与 BOSS')
    expect(spec).toContain('可见汉字「AI」')
    expect(spec).toContain('依据')
    expect(spec).toContain('CTA')
    expect(spec).toContain('local')
    expect(spec).toMatch(/本地规则引擎/)
    expect(spec).toContain('skeleton-row')
    expect(spec).toContain('prefers-reduced-motion')
    expect(spec).toMatch(/\.022/)
  })

  it('令牌双锚点与 R6 标记保留，禁止误升 v4 或新增视觉令牌', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--r6-[a-z-]+:/)
  })

  it('common.css 有具名 R6 推荐卡组件层且加载/空态等高、可减动效', () => {
    const css = readSrc('styles/common.css')
    expect(css).toContain('LYP-R6-PERSONALIZED-V1')
    for (const cls of ['.r6-rec', '.r6-badge', '.r6-headline', '.r6-evidence', '.r6-why', '.r6-cta', '.r6-empty', '.r6-skeleton']) {
      expect(css, '缺少 ' + cls).toContain(cls)
    }
    expect(css).toMatch(/\.r6-rec[\s\S]{0,500}--panel-2/)
    expect(css).toMatch(/\.r6-rec[\s\S]{0,500}--ai-accent/)
    expect(css).toMatch(/\.r6-badge[\s\S]{0,300}--ai-ink/)
    expect(css).toMatch(/\.r6-cta[\s\S]{0,300}min-height:\s*88rpx/)
    expect(css).toMatch(/prefers-reduced-motion[\s\S]{0,400}\.r6-[\s\S]{0,200}animation:\s*none/)
  })

  it('共享 helper 存在且纯函数拒绝假置信度、识别诚实空态与本地兜底', async () => {
    const mod = await import('../pages/_shared/r6Recommendation')
    expect(typeof mod.fetchRecommendations).toBe('function')
    expect(typeof mod.normalizeRecommendation).toBe('function')
    expect(typeof mod.localFallbackCard).toBe('function')
    expect(typeof mod.emptyRecommendation).toBe('function')
    const good = mod.normalizeRecommendation({
      schema_version: 'rec-v1',
      context: 'equipment',
      generated_at: '2026-01-01T00:00:00Z',
      evidence: [{ metric: '护甲缺口', value: 12, unit: '点', source: 'config', window: 'now' }],
      insights: [{
        id: 'e1',
        headline: '优先替换护甲缺口最大的部位',
        basis: ['当前护甲低于同阶', '配置版本已更新'],
        cta: { label: '看搭配', route: '/pages/gems/gems', params: { slot: 2 } },
        confidence: 'rule_based',
        model: { name: 'rules', version: 'rec-v1' },
      }],
      empty_reason: null,
      model: { name: 'rules', version: 'rec-v1', kind: 'rule_based' },
    }, 'equipment')
    expect(good).not.toBeNull()
    expect(good!.status).toBe('api')
    expect(good!.headline).toContain('护甲')
    expect(good!.evidence).toHaveLength(1)
    expect(good!.basis).toHaveLength(2)
    expect(good!.cta.route).toBe('/pages/gems/gems')
    expect(good!.engineLabel).not.toMatch(/GPT|LLM/i)
    const fake = mod.normalizeRecommendation({ schema_version: 'rec-v1', context: 'equipment', insights: [{ headline: 'x', cta: { route: '/x' }, confidence: 0.93 }] }, 'equipment')
    expect(fake, '禁止 0.93 一类假置信度').toBeNull()
    const empty = mod.emptyRecommendation('NO_HISTORY')
    expect(empty.status).toBe('empty')
    expect(empty.emptyCode).toBe('NO_HISTORY')
    expect(empty.basis.join('')).toMatch(/缺|下一步|战斗/)
    const local = mod.localFallbackCard({ context: 'skills', headline: '本地结论', basis: ['本地数据'], ctaLabel: '去背包', ctaRoute: '/pages/bag/bag' })
    expect(local.status).toBe('local')
    expect(local.engineLabel).toContain('本地规则引擎')
  })

  for (const page of Object.keys(FOCUS) as FocusPage[]) {
    const cfg = FOCUS[page]

    it(page + ' 接推荐接口、保留本地兜底且四区推荐卡可朗读可行动', () => {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      const markup = tpl(src)
      expect(src, '缺少 context 参数').toContain("fetchRecommendations('" + cfg.context + "')")
      expect(src, '旧规则引擎 computed 必须保留').toContain('const ' + cfg.legacy + ' = computed')
      expect(src).toContain('const r6Loading = ref(true)')
      expect(src).toContain('const r6Rec = ref<R6Card | null>(null)')
      expect(src).toContain('const r6Open = ref(false)')
      expect(src).toContain('function r6Go()')
      for (const cls of ['r6-rec', 'r6-badge', 'r6-headline', 'r6-evidence', 'r6-ev', 'r6-why', 'r6-cta', 'r6-empty', 'r6-skeleton']) {
        expect(markup, '模板缺少 ' + cls).toContain(cls)
      }
      expect(markup).toContain('skeleton-row')
      expect(markup).toContain('skeleton-bar')
      expect(markup).toContain('{{ r6Rec.headline }}')
      expect(markup).toContain('{{ r6Rec.engineLabel }}')
      expect(markup).toContain('r6Rec.cta.route')
      expect(markup).toMatch(/@click="r6Go\(\)"/)
      expect(markup).toMatch(/@click="r6Open = !r6Open"/)
      expect(markup).toContain("r6Rec.status === 'empty'")
      expect(markup).toContain('r3-empty')
      expect(markup).toContain('本地规则引擎')
      expect(markup).toMatch(/AI/)
      expect(markup, page + ' 结论不得复述其它页').toMatch(cfg.topic)
    })

    it(page + ' 的本地兜底结论来自本页真实派生数据且路由可用', () => {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      const block = src.slice(src.indexOf('function loadR6Recommendation'))
      expect(block, '缺少本地兜底函数').toBeTruthy()
      expect(block).toContain('localFallbackCard')
      expect(block).toContain('context: ' + JSON.stringify(cfg.context))
      expect(block).toMatch(/store\.|items\.value|skills\.value|allReactions\.value|bestPerSlot\.value|ownedCount\.value|efficiencyRank\.value/)
      expect(block).toMatch(cfg.topic)
      expect(block).not.toMatch(cfg.notTopic)
      expect(block).toMatch(/ctaRoute: '\/pages\/[a-z]+\/[a-z]+'/)
    })
  }

  it('login/verify 只保留轻量 AI 提示，不伪装个性化推荐', () => {
    for (const page of ['login', 'verify']) {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      expect(src).toContain('r5-ai-note')
      expect(src).toMatch(/AI/)
      expect(src).not.toContain('fetchRecommendations')
      expect(src).not.toContain('r6Rec')
      expect(src).not.toMatch(/置信度\s*\d|0\.93/)
    }
  })

  it('三页不复用 .022 网格/.06 辉光充当 AI，也不触碰战斗脚本', () => {
    for (const page of Object.keys(FOCUS) as FocusPage[]) {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      expect(src).not.toContain('rgba(255,255,255,.022)')
      expect(src).not.toContain('rgba(255,107,53,.06)')
    }
    const battle = readSrc('pages/battle/battle.vue')
    expect(battle).toContain('battle-sigil-rail')
  })
})
