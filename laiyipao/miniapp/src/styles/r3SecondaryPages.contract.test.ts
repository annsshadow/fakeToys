import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const root = join(__dirname, '..')
const read = (p: string) => readFileSync(join(root, p), 'utf-8')

const PAGES = {
  gems: 'r3AiGem',
  shop: 'r3AiShop',
  mastery: 'r3AiMastery',
  reactions: 'r3AiReaction',
  skills: 'r3AiSkill',
  equipment: 'r3AiEquip',
} as const

type PageName = keyof typeof PAGES

describe('R3 次级页结构性升级契约（六页）', () => {
  it('令牌包与全局组件层已落地', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--accent-soft:\s*rgba\(255,\s*107,\s*53,\s*0\.18\)/)
    expect(tokens).toMatch(/--progress-track:\s*#39465a/)

    const css = read('styles/common.css')
    expect(css, '必须具名标记 R3 组件层').toMatch(/LYP-R3-SECONDARY-V1/)
    for (const cls of ['.r3-kpi', '.r3-ai', '.r3-data-grid', '.r3-empty', '.r3-filter', '.r3-progress']) {
      expect(css, '缺少全局组件 ' + cls).toContain(cls)
    }
    expect(css).toContain('var(--progress-track)')
    expect(css).toContain('var(--accent-soft)')
    expect(css).toContain('var(--accent-ink)')
    expect(css).toContain('var(--ai-accent)')
    // FilterBar 活动态禁止 accent 色字叠 soft（4.37:1 未过线）
    const active = css.match(/\.r3-filter[^{]*\.active\s*\{[^}]*\}/)
    expect(active, 'FilterBar 活动态规则必须存在').not.toBeNull()
    expect(active![0]).toContain('var(--accent-soft)')
    expect(active![0]).toContain('var(--accent-ink)')
    expect(active![0]).not.toMatch(/color:\s*var\(--accent\)/)
  })

  for (const page of Object.keys(PAGES) as PageName[]) {
    const aiComputed = PAGES[page]

    it(page + ' 有眉题/指标带、数据容器与可朗读 AI 线索', () => {
      const src = read('pages/' + page + '/' + page + '.vue')
      expect(src, page + ' 必须有分区眉题或指标带').toMatch(/section-kicker|kpi-strip|r3-kpi/)
      expect(src, page + ' 必须有数据容器').toMatch(/data-grid|progress-tree|list-row|r3-data-grid|r3-progress/)
      expect(src, page + ' 必须有可见 AI + 依据线索').toMatch(/AI[\s\S]{0,40}(依据|建议|提示|诊断)/)
      expect(src, page + ' 不得保留裸空态文案').not.toMatch(/暂无数据|加载失败(?!：)/)
      expect(src, page + ' 缺少专用 AI computed').toContain('const ' + aiComputed + ' = computed')
      expect(src, page + ' 模板未消费 AI computed').toContain(aiComputed)
      expect(src, page + ' AI 线索必须标注为规则引擎').toMatch(/规则引擎/)
    })

    it(page + ' 的 AI 结论绑定现有数据且 CTA 可点击', () => {
      const src = read('pages/' + page + '/' + page + '.vue')
      const start = src.indexOf('const ' + aiComputed + ' = computed')
      expect(start, page + ' 缺少专用 AI computed 声明').toBeGreaterThan(-1)
      const block = src.slice(start, start + 1200)
      expect(block, page + ' 的 AI computed 必须引用现有数据源').toMatch(
        /store\.|gems\.value|items\.value|families\.value|allReactions\.value|skills\.value|efficiencyRank\.value|familyOverview\.value|pointPlanner\.value|ownedCount\.value|valueTable\.value|bestPerSlot\.value/,
      )
      expect(src, page + ' 的 AI CTA 必须是可点击元素').toMatch(/@click="[^"]*"/)
    })
  }

  it('专精/宝石的 ProgressTree 同时给出轨道与数字文本', () => {
    for (const page of ['mastery', 'gems'] as PageName[]) {
      const src = read('pages/' + page + '/' + page + '.vue')
      expect(src, page + ' 必须有进度树').toMatch(/progress-tree|r3-progress/)
      expect(src, page + ' 必须有数字进度文案').toMatch(/(已点亮|已拥有|共)[^<]{0,24}\{\{/)
    }
  })

  it('专精/商城/宝石的智能空态含动作与可访问状态', () => {
    for (const page of ['mastery', 'shop', 'gems'] as PageName[]) {
      const src = read('pages/' + page + '/' + page + '.vue')
      expect(src, page + ' 缺少智能空态').toMatch(/r3-empty|empty-state--ai/)
      expect(src, page + ' 空态缺少可访问状态').toMatch(/aria-live|role="status"/)
      expect(src, page + ' 空态缺少动作按钮').toMatch(/@click="[^"]*"/)
    }
  })

  it('不回退 gems 幂等加载逻辑与其他图鉴既有加载入口', () => {
    const gems = read('pages/gems/gems.vue')
    for (const marker of ['doLoadPage', 'loadPage', 'onLoad', 'onMounted', 'onShow', 'refreshOwned']) {
      expect(gems, 'gems 加载逻辑不得丢失 ' + marker).toContain(marker)
    }
    // login/loadConfig 的缺口由 backend 后续任务单独修；skills/equipment 在基线中
    // 本就没有 onMounted/onLoad，因此这里只守「配置数据源 + 现有加载态模板」不被视觉改造覆盖。
    for (const page of ['skills', 'equipment'] as PageName[]) {
      const src = read('pages/' + page + '/' + page + '.vue')
      expect(src, page + ' 的配置数据源不得丢失').toContain('store.config')
      expect(src, page + ' 的加载态模板不得丢失').toContain('store.configLoading')
    }
  })
})
