import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const specPath = join(__dirname, '../../../docs/laiyipao/specs/round19-player-reaction-impact/SPEC.md')
const srcRoot = join(__dirname, '..')
const readSpec = () => readFileSync(specPath, 'utf-8')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

describe('R19 玩家伤害贡献 API 与字段契约', () => {
  it('SPEC 钉死玩家端点、user_id 隔离与窗口字段', () => {
    const spec = readSpec()
    expect(spec).toContain('GET /me/reaction-damage-impact')
    expect(spec).toContain('user_id')
    expect(spec).toContain('绝不返回他人数据')
    for (const k of ['window_days', 'window_from', 'window_to', 'excluded_old_battles', 'items']) {
      expect(spec, `缺窗口字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('window_days=30')
  })
  it('SPEC 钉死 items 五字段与空结果/NaN 口径', () => {
    const spec = readSpec()
    for (const k of ['reaction', 'total_damage', 'damage_share_pct', 'events', 'count']) {
      expect(spec, `缺 items 字段 ${k}`).toContain(k)
    }
    expect(spec).toContain('{reaction,total_damage,damage_share_pct,events,count}')
    expect(spec).toContain('空结果为 `[]`，不是 `null`')
    expect(spec).toContain('`SUM=0` 时全为 `0`，不产生 `NaN`')
  })
})

describe('R19 count 口径裁决', () => {
  it('SPEC 明确用 reactions_used 而非 events / 全平台聚合，并标注口径差异', () => {
    const spec = readSpec()
    expect(spec).toContain('count 口径裁决')
    for (const s of ['不用 `events`', '不用 `dashboard_reaction_counts`', '选 `battle_records.reactions_used`',
      '口径差异的诚实标注', '分母可能不同', '仍计入 count']) {
      expect(spec, `缺口径裁决 ${s}`).toContain(s)
    }
  })
})

describe('R19 选择器与逐字文案', () => {
  it('SPEC 钉死全部 r19 选择器', () => {
    const spec = readSpec()
    for (const sel of ['.r19-my-reaction-impact', '.r19-view-toggle', '.r19-rdi-row',
      '[data-r19-reaction]', '.r19-rdi-empty', '.r19-ai-note']) {
      expect(spec, `缺选择器 ${sel}`).toContain(sel)
    }
  })
  it('SPEC 钉死空态/诚实/依据逐字文案', () => {
    const spec = readSpec()
    expect(spec).toContain('近 30 天无逐次反应伤害记录')
    expect(spec).toContain('辅助分析非权威结算依据，权威请看 replay_hash')
    expect(spec).toContain('依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准')
    expect(spec).toContain('已排除 N 条无逐次记录的旧战报')
  })
})

describe('R19 AI 四判据与诚实红线', () => {
  it('SPEC 钉死 AI 四要素', () => {
    const spec = readSpec()
    for (const s of ['可见汉字「AI」', '真实数据结论', '依据行', '可执行 CTA',
      '禁止硬编码反应名结论']) expect(spec, `缺 AI 判据 ${s}`).toContain(s)
  })
  it('SPEC 钉死诚实红线：非权威、不越权、旧战报如实标注、零真值', () => {
    const spec = readSpec()
    for (const s of ['辅助分析非权威', 'replay_hash', '不泄露他人数据',
      '旧战报如实标注', '不得反推伤害补洞', '零真值']) expect(spec, `缺红线 ${s}`).toContain(s)
  })
})

describe('R19 三端不回退基线', () => {
  it('miniapp tokens: v1/v3-r3/r6 锚点保留且无 r19 新增令牌', () => {
    const s = readSrc('src/styles/tokens.css')
    expect(s).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(s).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(s).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(s).not.toContain('--r19-')
  })
  it('backend: reactions_used / replay_hash / ReplayInfo 基线保留', () => {
    const bc = readSrc('../server/internal/domain/battle_collections.go')
    expect(bc).toContain('reactions_used')
    expect(bc).toContain('replay_hash')
    expect(bc).toContain('maxReplayHashLen')
    const ec = readSrc('../server/internal/service/economy.go')
    expect(ec).toContain('type ReplayInfo struct')
    expect(ec).toContain('parseReplayReactionTimeline')
    expect(ec).toContain('json:"reaction_timeline"')
  })
  it('admin: R18 端点保留（本轮零改动）', () => {
    const s = readSrc('../admin/src/api/index.ts')
    expect(s).toContain("'/admin/reaction-damage-impact'")
  })
  it('h5: reactions.vue 理论构筑视图符号保留', () => {
    const s = readSrc('src/pages/reactions/reactions.vue')
    for (const k of ['myReactions', 'reachableReactions', 'reactionName', '元素反应覆盖矩阵']) {
      expect(s, `理论视图基线丢失 ${k}`).toContain(k)
    }
  })
})