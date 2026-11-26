import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const root = join(__dirname, '..')
const read = (p: string) => readFileSync(join(root, p), 'utf-8')

const PAGES = {
  bag: { ai: 'r4AiBag', cta: 'r4GoBag', source: /selected\.value|skillOf\(/, structs: ['r4-kpi', 'r4-filter', 'r4-grid', 'r4-empty'] },
  battles: { ai: 'r4AiBattle', cta: 'r4GoBattle', source: /battleTotals\.value|streak\.value/, structs: ['r4-kpi', 'r4-filter', 'r4-grid', 'r4-empty'] },
  enemies: { ai: 'r4AiEnemy', cta: 'r4FocusEnemy', source: /counterRanking\.value|ELEMENT_ORDER/, structs: ['r4-section', 'r4-filter', 'r4-grid', 'r4-empty'] },
  me: { ai: 'r4AiMe', cta: 'r4GoMe', source: /store\.buildRating|store\.maxStage/, structs: ['r4-section', 'r4-kpi', 'r4-grid', 'r4-empty'] },
  rank: { ai: 'r4AiRank', cta: 'r4GoRank', source: /chaseGap\.value|scoreBuckets\.value|scoreDistribution\.value/, structs: ['r4-kpi', 'r4-filter', 'r4-grid', 'r4-empty'] },
  skins: { ai: 'r4AiSkin', cta: 'r4FocusSkin', source: /passiveElementMap\.value|rarityDonut\.value/, structs: ['r4-kpi', 'r4-filter', 'r4-grid', 'r4-empty'] },
} as const

type PageName = keyof typeof PAGES

describe('R4 第二批 H5 页结构性契约（六页）', () => {
  it('旧令牌锚点不动，R4 只复用 r3 组件而非新体系', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).not.toMatch(/--r4-/) // R4 零新增令牌
    const css = read('styles/common.css')
    expect(css, 'R4 全局层必须具名').toMatch(/LYP-R4-SECONDARY-V1/)
    for (const cls of ['.r4-section', '.r4-kpi', '.r4-ai', '.r4-filter', '.r4-grid', '.r4-empty']) {
      expect(css, '缺少 R4 语义钩子 ' + cls).toContain(cls)
    }
    expect(css).toContain('.r4-enter')
    expect(css, 'R4 入场动效必须可关闭').toMatch(/prefers-reduced-motion[\s\S]{0,200}\.r4-enter[\s\S]{0,80}animation:\s*none/)
    expect(css).toContain('.r3-ai')
    expect(css).toContain('.r3-data-grid')
    expect(css).toContain('.r3-empty')
  })

  for (const page of Object.keys(PAGES) as PageName[]) {
    const cfg = PAGES[page]

    it(page + ' 具备 R4 指定组件顺序与可见 AI 线索', () => {
      const src = read('pages/' + page + '/' + page + '.vue')
      for (const cls of cfg.structs) expect(src, page + ' 缺少 ' + cls).toContain(cls)
      expect(src, page + ' 未复用 r3 AI 卡').toMatch(/class="[^"]*\br3-ai\b[^"]*\br4-ai\b/)
      expect(src, page + ' AI 徽章必须可见').toMatch(/<text class="r3-ai-badge">AI<\/text>/)
      expect(src, page + ' 必须标注规则引擎').toMatch(/规则引擎/)
      expect(src, page + ' 必须给出依据').toMatch(/依据：/)
      expect(src, page + ' CTA 必须绑定 @click').toMatch(new RegExp('@click="' + cfg.cta + '\\('))
      expect(src, page + ' 智能空态必须有图形槽/标题/原因/动作').toMatch(
        /<view[^>]*class="r3-empty r4-empty[^"]*"[\s\S]{0,700}r3-empty-slot[\s\S]{0,700}r3-empty-title[\s\S]{0,700}r3-empty-reason[\s\S]{0,700}r3-empty-action/,
      )
      const tpl = src.slice(0, src.indexOf('</template>'))
      expect(tpl, page + ' 不得保留裸空态文案').not.toMatch(/(?:^|>)\s*暂无数据\s*(?:<|$)|加载失败(?!：)/)
    })

    it(page + ' 的 AI 结论来自真实数据而非低透明度装饰', () => {
      const src = read('pages/' + page + '/' + page + '.vue')
      const start = src.indexOf('const ' + cfg.ai + ' = computed')
      expect(start, page + ' 缺少专用 AI computed').toBeGreaterThan(-1)
      const block = src.slice(start, start + 1600)
      expect(block, page + ' AI computed 必须引用现有数据源').toMatch(cfg.source)
      expect(block, page + ' 低透明度网格/光斑不算 AI').not.toMatch(/0?\.022|0?\.06\b/)
      expect(src, page + ' CTA 函数缺失').toMatch(new RegExp('function ' + cfg.cta + '\\('))
    })
  }

  it('幂等加载与既有数据入口不被 R4 覆盖', () => {
    const bag = read('pages/bag/bag.vue')
    expect(bag).toContain('store.loadLoadout()')
    expect(bag).toContain('store.refreshProfile()')
    expect(bag).toContain('selected.value = [...store.loadout]')
    for (const page of ['enemies', 'skins'] as PageName[]) {
      const src = read('pages/' + page + '/' + page + '.vue')
      expect(src, page + ' 必须保留幂等 loadPage').toContain('function loadPage(): Promise<void>')
      expect(src, page + ' 必须保留 pageLoaded 守卫').toContain('if (pageLoaded)')
      expect(src, page + ' 必须保留权威配置加载').toContain('store.loadConfig()')
    }
  })
})
