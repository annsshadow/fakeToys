import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const repoRoot = join(__dirname, '..', '..', '..', '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const readRepo = (p: string) => readFileSync(join(repoRoot, p), 'utf-8')
const tpl = (src: string) => src.slice(0, src.indexOf('</template>'))

const impactPayload = {
  schema_version: 'rec-v1',
  generated_at: '2026-11-22T05:00:00Z',
  context: 'skill',
  subject: { user_key: 'hash', level: 3, power: 1234 },
  input_snapshot: { battle_count: 3, window_days: 7, config_version: 'cfg-abc' },
  evidence: [
    { metric: '历史命中/波', value: 12, unit: '次/波', source: 'battle_records', window: '7d' },
    { metric: '历史三星率', value: 33.333, unit: '%', source: 'battle_records', window: '7d' },
  ],
  insights: [{
    id: 'impact-skill-2',
    headline: '换上「重炮弹」后，按最近 7 天节奏估算每波伤害 +12.4%',
    basis: ['近 7 天 3 场，命中/波 12', '星级概率无可信模型，未做推测'],
    cta: { label: '去换技能', route: '/pages/skills/skills', params: { kind: 'skill', id: 2, replace: 1 } },
    confidence: 'rule_based',
    model: { name: 'rules', version: 'rec-v1' },
  }],
  empty_reason: null,
  model: { name: 'rules', version: 'rec-v1', kind: 'rule_based' },
  impact: {
    kind: 'skill', candidate_id: 2, replaced_id: 1,
    candidate_name: '重炮弹', replaced_name: '烈焰弹',
    baseline: { attack_permille: 1000, armor_permille: 0, estimated_damage_per_wave: 2200, power: 100, rating_total: 40 },
    candidate: { attack_permille: 1120, armor_permille: 0, estimated_damage_per_wave: 2473, power: 112, rating_total: 44 },
    changes: {
      power_delta: 12, rating_total_delta: 4, attack_permille_delta: 120, armor_permille_delta: 0,
      damage_per_wave_delta: 273, damage_per_wave_pct: 12.4,
      star_rate_observed: 33.333, star_probability_estimate: null,
    },
    method: '命中/波×技能底伤(等级)×(1000+攻击)/1000',
    star_rate_note: '只给历史三星率；星级概率无可信模型，未做推测',
  },
}

describe('R7 H5 推荐落地契约：真实收益预估 + 对比确认写入', () => {
  it('§1F.4 与速查 7D 的对比确认流程、因果解释与诚实口径已落盘', () => {
    const spec = readRepo('docs/laiyipao/design/AI_VISUAL_SYSTEM.md')
    expect(spec).toContain('1F.4')
    for (const s of ['当前 vs 推荐', '确认替换', '取消', 'expected_gain', '禁止无确认', 'evidence[]', 'config_version', '本地规则引擎', '三星']) {
      expect(spec, '规范缺少 ' + s).toContain(s)
    }
    const quick = readRepo('docs/laiyipao/design/TOKENS_QUICK_REF.md')
    expect(quick).toContain('7D')
    expect(quick).toContain('两步 CTA')
  })

  it('令牌锚点未升级为 v4，也没有新增 R7 视觉变量', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    expect(tokens).not.toMatch(/--r7-[a-z-]+:/)
  })

  it('共享 helper 真实调用 impact 端点，三参齐全且拒绝本地兜底冒充', async () => {
    const src = readSrc('pages/_shared/r7ImpactApply.ts')
    expect(src).toContain("/me/recommendations/impact?kind=")
    for (const p of ['kind=', '&id=', '&replace=']) expect(src, '缺少参数 ' + p).toContain(p)
    expect(src).toContain('star_probability_estimate')
    expect(src).toContain('未做推测')
    expect(src).not.toContain('localFallbackCard')
    const mod = await import('../pages/_shared/r7ImpactApply')
    expect(typeof mod.fetchRecommendationImpact).toBe('function')
    expect(typeof mod.normalizeImpact).toBe('function')
    expect(typeof mod.applyEquipmentImpact).toBe('function')
  })

  it('normalizeImpact 从真实响应提取当前 vs 推荐、数据条、依据与三星率口径', async () => {
    const mod = await import('../pages/_shared/r7ImpactApply')
    const view = mod.normalizeImpact(impactPayload, 'skill')
    expect(view).not.toBeNull()
    expect(view!.kind).toBe('skill')
    expect(view!.candidateName).toBe('重炮弹')
    expect(view!.replacedName).toBe('烈焰弹')
    expect(view!.changes.damagePerWavePct).toBe(12.4)
    expect(view!.changes.starRateObserved).toBe(33.333)
    expect(view!.changes.starProbabilityEstimate).toBeNull()
    expect(view!.evidence).toHaveLength(2)
    expect(view!.starRateNote).toContain('未做推测')
    expect(view!.configVersion).toBe('cfg-abc')
    expect(view!.battleCount).toBe(3)
    const card = mod.normalizeImpactCard(impactPayload, 'skill')
    expect(card?.status).toBe('api')
    expect(card?.headline).toContain('重炮弹')
  })

  it('normalizeImpact 对空历史/错上下文/缺失 impact 不编造数值', async () => {
    const mod = await import('../pages/_shared/r7ImpactApply')
    const empty: Record<string, unknown> = structuredClone(impactPayload)
    empty.empty_reason = { code: 'NO_HISTORY', message: '最近 7 天没有战报，无法估算真实收益' }
    empty.impact = null
    const emptyView = mod.normalizeImpact(empty, 'skill')
    expect(emptyView).toBeNull()
    const emptyCard = mod.normalizeImpactCard(empty, 'skill')
    expect(emptyCard?.status).toBe('empty')
    expect(emptyCard?.emptyCode).toBe('NO_HISTORY')
    expect(mod.normalizeImpact(impactPayload, 'equipment')).toBeNull()
    const noImpact: Record<string, unknown> = structuredClone(impactPayload)
    noImpact.impact = null
    expect(mod.normalizeImpact(noImpact, 'skill')).toBeNull()
  })

  const pages = {
    skills: { kind: 'skill', write: 'persistLoadout', img: 'r7-impact' },
    equipment: { kind: 'equipment', write: 'applyEquipmentImpact', img: 'r7-impact' },
  } as const

  for (const page of Object.keys(pages) as (keyof typeof pages)[]) {
    const cfg = pages[page]

    it(page + ' 页面以当前构筑为 replace、推荐项为 id 请求真实 impact 端点', () => {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      expect(src).toContain('fetchRecommendationImpact')
      expect(src).toContain("'" + cfg.kind + "'")
      expect(src).toMatch(/candidateId|r7Candidate/)
      expect(src).toMatch(/replacedId|r7Replaced/)
      expect(src).toMatch(/store.build|store.loadout|store.equipment|store.equipped/)
      expect(src).toContain('收益数据不可用')
      expect(src).not.toContain('预计提升 +12')
    })

    it(page + ' 模板把当前 vs 推荐、三星率、依据与两步 CTA 渲染成可朗读文本', () => {
      const markup = tpl(readSrc('pages/' + page + '/' + page + '.vue'))
      for (const cls of ['r7-impact', 'r7-compare', 'r7-vs-row', 'r7-star', 'r7-why', 'r7-basis', 'r7-btn', 'r7-error']) {
        expect(markup, '模板缺少 ' + cls).toContain(cls)
      }
      expect(markup).toContain('当前 vs 推荐')
      expect(markup).toContain('确认替换')
      expect(markup).toContain('取消')
      expect(markup).toContain('历史三星率')
      expect(markup).toContain('stars=3')
      expect(markup).toContain('不是胜率')
      expect(markup).toContain('收益数据不可用')
      expect(markup).toContain('@click="r7OpenCompare()"')
      expect(markup).toContain('@click="r7ConfirmApply()"')
    })

    it(page + ' 首次点击只预览；只有 r7ConfirmApply 才调用写入接口，失败保留原配置', () => {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      const open = src.slice(src.indexOf('function r7OpenCompare'), src.indexOf('function r7ConfirmApply'))
      const confirm = src.slice(src.indexOf('function r7ConfirmApply'))
      expect(open).toContain('r7CompareOpen.value = true')
      expect(open).not.toContain(cfg.write)
      expect(confirm).toContain(cfg.write)
      expect(confirm).toContain('r7Applying.value = true')
      expect(confirm).toContain('r7ApplyError.value')
      expect(confirm).toContain('重试')
      expect(confirm).not.toMatch(/r7CompareOpen.value = false[sS]{0,120}r7Applying.value = true/)
      expect(src).toContain("r6Rec.status === 'empty'")
    })
  }

  it('common.css 的 R7 对比层复用既有令牌，触控区 ≥88rpx 且可关闭动效', () => {
    const css = readSrc('styles/common.css')
    expect(css).toContain('LYP-R7-IMPACT-APPLY-V1')
    for (const cls of ['.r7-impact', '.r7-bar', '.r7-compare-mask', '.r7-compare', '.r7-btn', '.r7-error']) {
      expect(css, '缺少 ' + cls).toContain(cls)
    }
    expect(css).toMatch(/\.r7-btn[\s\S]{0,300}min-height:\s*88rpx/)
    expect(css).toMatch(/\.r7-compare[\s\S]{0,500}--panel-2/)
    expect(css).toMatch(/prefers-reduced-motion[\s\S]{0,500}\.r7-[\s\S]{0,200}animation:\s*none/)
  })

  it('本地规则兜底也必须显示真实收益不可用的诚实说明', () => {
    for (const page of ['skills', 'equipment']) {
      const src = readSrc('pages/' + page + '/' + page + '.vue')
      expect(src, page + ' 本地兜底未写明收益不可用').toMatch(/r7Error\.value = '本地规则引擎[^']*不展示虚构数值'/)
      expect(src, page + ' 本地兜底隐藏了 R7 收益区').toContain('v-if="r6Rec" class="r7-impact r7-enter"')
    }
  })

  it('两页模板使用的 R7 类名必须在全局 CSS 中有定义', () => {
    const css = readSrc('styles/common.css')
    for (const page of ['skills', 'equipment']) {
      const markup = tpl(readSrc('pages/' + page + '/' + page + '.vue'))
      const classes = [...new Set(markup.match(/\br7-[a-z-]+\b/g) ?? [])]
      expect(classes.length, page + ' 未渲染 R7 结构').toBeGreaterThan(10)
      for (const cls of classes) {
        expect(css, page + ' 使用了未定义的 ' + cls).toContain('.' + cls)
      }
    }
  })

  it('不触碰战斗与游戏逻辑：impact 只出现在技能/装备页与共享 helper', () => {
    const battle = readSrc('pages/battle/battle.vue')
    expect(battle).not.toContain('fetchRecommendationImpact')
    expect(battle).not.toContain('r7ConfirmApply')
    const files = ['pages/skills/skills.vue', 'pages/equipment/equipment.vue']
    for (const f of files) {
      const src = readSrc(f)
      const script = src.slice(src.indexOf('<script setup'), src.indexOf('</script>'))
      expect(script).not.toMatch(/@\/game\/(damage|combat|stage|enemy)/)
      expect(script).toContain('loadPage')
    }
  })
})
