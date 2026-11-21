// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * F-10-2 / §1A.3：看板可见 AI 元素契约。
 * 真浏览器基线：看板无 AI 元素。本测试钉住「AI 徽章 + 规则引擎标识 + 结论 + 依据 + CTA」
 * 与加载骨架、空态、令牌消费等可观测行为。
 */
import { mount } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'
import AiInsightCard from '@/components/AiInsightCard.vue'
import type { AiInsight } from '@/utils/aiInsight'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const cardSource = readFileSync(
  join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'components', 'AiInsightCard.vue'),
  'utf8',
)

const funnelInsight: AiInsight = {
  kind: 'funnel',
  headline: '第 2 关流失最大（尝试 400 → 通关 120），建议优先复核关卡难度',
  evidence: '关卡漏斗柱状图，共 3 关',
  ctaLabel: '复核关卡难度',
  ctaPath: '/admin/levels',
  asOf: '2026-01-03',
}

describe('AiInsightCard 可见 AI 元素（§1A.3）', () => {
  it('渲染 AI 徽章 + 标题 + 规则引擎标识 + 数据截至日期', () => {
    const w = mount(AiInsightCard, { props: { insight: funnelInsight, loading: false } })
    expect(w.find('.ai-badge').text()).toContain('AI')
    expect(w.text()).toContain('AI 运营洞察')
    expect(w.text()).toContain('规则引擎')
    expect(w.text()).toContain('2026-01-03')
    expect(w.attributes('id')).toBe('ai-insight-card')
  })

  it('结论与依据分别渲染，依据用 muted 弱化', () => {
    const w = mount(AiInsightCard, { props: { insight: funnelInsight, loading: false } })
    expect(w.find('.ai-headline').text()).toContain('第 2 关流失最大')
    expect(w.find('.ai-evidence').text()).toContain('关卡漏斗')
    expect(w.find('.ai-evidence').classes()).toContain('muted')
  })

  it('CTA 是真实路由 router-link 且指向洞察建议页', () => {
    const w = mount(AiInsightCard, {
      props: { insight: funnelInsight, loading: false },
      global: { stubs: { RouterLink: { props: ['to'], template: '<a :data-to="to"><slot /></a>' } } },
    })
    const cta = w.find('.ai-cta')
    expect(cta.exists()).toBe(true)
    expect(cta.text()).toContain('复核关卡难度')
    expect(cta.attributes('data-to')).toBe('/admin/levels')
  })

  it('加载态显示骨架与 aria-busy，不渲染结论', () => {
    const w = mount(AiInsightCard, { props: { insight: null, loading: true } })
    expect(w.attributes('aria-busy')).toBe('true')
    expect(w.findAll('.skeleton-bar').length).toBeGreaterThan(0)
    expect(w.find('.ai-headline').exists()).toBe(false)
  })

  it('空态诚实展示数据不足，不渲染 CTA', () => {
    const w = mount(AiInsightCard, {
      props: {
        insight: { kind: 'empty', headline: '数据不足：AI 洞察需要至少 3 天活跃记录，当前 1 天', evidence: '', ctaLabel: '', ctaPath: '', asOf: '' },
        loading: false,
      },
    })
    expect(w.find('.ai-empty').text()).toContain('数据不足')
    expect(w.find('.ai-cta').exists()).toBe(false)
  })

  it('样式消费 AI 令牌 --lyp-ai-*，不硬编码紫色', () => {
    expect(cardSource).toMatch(/var\(--lyp-ai-accent\)/)
    expect(cardSource).toMatch(/var\(--lyp-ai-halo\)/)
    expect(cardSource).not.toMatch(/#c9a7ff/i)
  })
})
