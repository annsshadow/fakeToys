// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 全局主题样式标准契约：尊重系统「减少动态效果」偏好（WCAG 2.3.3）。
 *
 * 桌面有 130+ 处动效，若只在个别视图写 prefers-reduced-motion，其余视图仍会
 * 对前庭敏感用户造成不适；守卫必须在全局主题层统一生效，且要收敛动画时长、
 * 迭代次数、过渡时长、平滑滚动四项（保留 transitionend 而非直接 none）。
 * 本测试钉死这条设计标准，防止后续改动把守卫或关键声明删掉而无人察觉。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const themeCss = readFileSync(path.join(frontendRoot, 'packages/ui/src/theme/dark.css'), 'utf8')
const uiIndex = readFileSync(path.join(frontendRoot, 'packages/ui/src/index.ts'), 'utf8')

describe('全局主题的减少动态效果标准（WCAG 2.3.3）', () => {
  it('全局主题文件包含 prefers-reduced-motion 守卫', () => {
    expect(themeCss).toMatch(/@media\s*\(prefers-reduced-motion:\s*reduce\)/)
  })

  it('守卫覆盖所有元素并收敛动画/过渡/平滑滚动四项', () => {
    const start = themeCss.indexOf('@media (prefers-reduced-motion: reduce)')
    expect(start).toBeGreaterThan(-1)
    const block = themeCss.slice(start)
    expect(block).toMatch(/animation-duration:\s*0\.01ms\s*!important/)
    expect(block).toMatch(/animation-iteration-count:\s*1\s*!important/)
    expect(block).toMatch(/transition-duration:\s*0\.01ms\s*!important/)
    expect(block).toMatch(/scroll-behavior:\s*auto\s*!important/)
  })

  it('全局主题经 ui 包入口引入，守卫对整个桌面生效', () => {
    expect(uiIndex).toMatch(/import\s+['"]\.\/theme\/dark\.css['"]/)
  })
})

describe('全局主题的键盘焦点可见标准（WCAG 2.4.7）', () => {
  it('全局主题提供 :focus-visible 焦点环，兜住 149 处 outline:none', () => {
    expect(themeCss).toMatch(/:focus-visible\s*\{/)
  })

  it('焦点环只作用于可聚焦交互元素并用主题主色而非写死色值', () => {
    const start = themeCss.indexOf(':where(')
    expect(start).toBeGreaterThan(-1)
    const rule = themeCss.slice(start, themeCss.indexOf('}', start))
    expect(rule).toMatch(/a,\s*button,\s*input,\s*textarea,\s*select,\s*\[tabindex\]/)
    expect(rule).toMatch(/outline:\s*2px\s+solid\s+var\(--color-primary\)/)
    expect(rule).toMatch(/outline-offset:\s*2px/)
  })

  it('焦点环用 !important 覆盖各视图 scoped 样式的 outline:none', () => {
    const start = themeCss.indexOf(':where(')
    const rule = themeCss.slice(start, themeCss.indexOf('}', start))
    expect(rule).toMatch(/outline:\s*2px[^;]*!important/)
  })
})
