import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../docs/laiyipao/specs/round18-reaction-damage-impact/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R18 reaction-damage-impact 契约守卫：字段契约', () => {
  it('SPEC 钉死聚合 API 路径与响应窗口字段', () => {
    const spec = readSpec()
    expect(spec).toContain('GET /admin/reaction-damage-impact')
    for (const k of ['window_days', 'window_from', 'window_to', 'excluded_old_battles', 'items']) {
      expect(spec, `缺少窗口字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('window_days=30')
  })
  it('SPEC 钉死 items 五字段与权威次数来源', () => {
    const spec = readSpec()
    for (const k of ['reaction', 'total_damage', 'damage_share_pct', 'events', 'count']) {
      expect(spec, `缺少 items 字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('{reaction,total_damage,damage_share_pct,events,count}')
    expect(spec).toContain('dashboard_reaction_counts')
    expect(spec).toContain('不得用 `events` 冒充')
  })
})

describe('R18 选择器与逐字文案', () => {
  it('SPEC 钉死全部 r18 选择器', () => {
    const spec = readSpec()
    for (const sel of [
      '.r18-damage-impact', '.r18-view-toggle', '.r18-view-count', '.r18-view-damage',
      '.r18-damage-row', '[data-r18-reaction]', '.r18-window-note', '.r18-excluded-note',
      '.r18-ai-note', '.r18-honesty-note',
    ]) expect(spec, `缺选择器 ${sel}`).toContain(sel)
  })
  it('SPEC 钉死空态/窗口/排除/诚实逐字文案', () => {
    const spec = readSpec()
    expect(spec).toContain('统计窗口：近 30 天')
    expect(spec).toContain('条无逐次记录的旧战报（不计入伤害）')
    expect(spec).toContain('近 30 天无逐次反应伤害记录')
    expect(spec).toContain('伤害贡献为辅助分析；触发次数以物化视图为准，不参与结算与 replay_hash')
    expect(spec).toContain('为 0 时也要如实显示 0')
  })
})

describe('R18 诚实红线', () => {
  it('SPEC 钉死辅助非权威/次数权威/不参与结算与 replay_hash', () => {
    const spec = readSpec()
    for (const s of ['辅助分析、非权威', '次数权威仍以物化视图 `dashboard_reaction_counts` 为准',
      '不参与结算 / replay_hash', '不得假装全量', '反推伤害补洞', 'H5 本轮零改动', '零真值']) {
      expect(spec, `缺红线 ${s}`).toContain(s)
    }
  })
})

describe('R18 AI 四判据', () => {
  it('SPEC 钉死可见 AI / 真实数据结论 / 依据行 / 可执行 CTA', () => {
    const spec = readSpec()
    for (const s of ['可见汉字「AI」', '真实数据结论', '依据行', '可执行 CTA',
      'overheat 触发占比 X% 但伤害贡献仅 Y%，疑似高频低伤',
      '依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 dashboard_reaction_counts 为准']) {
      expect(spec, `缺 AI 判据 ${s}`).toContain(s)
    }
  })
})

describe('R18 三端不回退基线', () => {
  it('backend: stats.go 的 ReactionCount 与 reaction_usage 保留', () => {
    const s = readSrc('../server/internal/service/stats.go')
    expect(s).toContain('type ReactionCount struct')
    expect(s).toContain('json:"reaction_usage"')
    expect(s).toContain('dashboard_reaction_counts')
  })
  it('admin: 次数图/反应行/空态/取名函数保留', () => {
    const s = readSrc('src/views/DashboardView.vue')
    for (const k of ['reactionRef', 'reactionChart', 'x.count', '尚无反应触发记录', 'reactionRows', 'labelOfReaction']) {
      expect(s, `admin 基线丢失 ${k}`).toContain(k)
    }
  })
  it('tokens: v1/v3-r3/r6 锚点保留且无 r18 新增令牌', () => {
    const s = readSrc('src/styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r18-')
  })
})