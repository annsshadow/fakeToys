import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const SOURCE = readFileSync(resolve(import.meta.dirname, 'OrganizationSelector.vue'), 'utf8')

const typeButton = SOURCE.match(/<button[^>]*class="type-button"[\s\S]*?<\/button>/)?.[0] ?? ''
const resultItem = SOURCE.match(/<button[^>]*class="result-item"[\s\S]*?<\/button>/)?.[0] ?? ''

describe('OrganizationSelector 选中状态无障碍契约（WCAG 4.1.2 Name, Role, Value）', () => {
  it('切换类型的 type-button 用 aria-pressed 暴露当前激活类型', () => {
    expect(typeButton).toMatch(/:aria-pressed="activeType === type"/)
  })

  it('搜索结果的 result-item 用 aria-pressed 暴露该项是否已选', () => {
    expect(resultItem).toMatch(/:aria-pressed="isSelected\(item\)"/)
  })
})
