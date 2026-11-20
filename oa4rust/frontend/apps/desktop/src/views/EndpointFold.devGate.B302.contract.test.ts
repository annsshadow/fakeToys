// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R302 DR-03a/DR-07: 端点调试仅在开发模式可见，管理区标题层级与部署页 AI 容器可验证。 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (p: string) => readFileSync(resolve(import.meta.dirname, p), 'utf8')
const fold = read('../components/admin/EndpointFold.vue')
const deploy = read('DeploymentApp.vue')

function template(src: string): string {
  const start = src.indexOf('<template>')
  let depth = 0
  for (let i = start; i < src.length; i++) {
    if (src.startsWith('<template', i) && !src.startsWith('</template', i)) depth++
    if (src.startsWith('</template>', i)) {
      depth--
      if (depth === 0) return src.slice(start, i + '</template>'.length)
    }
  }
  return src.slice(start)
}

describe('R302 DR-03a 端点调试开发模式门控', () => {
  it('EndpointFold 只读取开发模式开关，生产/测试构建整块不渲染', () => {
    expect(fold).toContain('import.meta.env.DEV')
    expect(fold).toMatch(/v-if="[^"]*devToolsEnabled|v-if="[^"]*isDev/)
    expect(fold).toContain("label: '高级操作'")
    expect(fold).not.toContain("label: '端点调试'")
    expect(fold).not.toContain("'展开查看全部端点'")
  })

  it('业务/管理页调试入口统一中文化，不再出现“端点调试”标签', () => {
    const callers = [
      'BamApp.vue', 'CmsIndexApp.vue', 'NoteApp.vue', 'OrgViewer.vue',
      'ProcessManagerApp.vue', 'ProcessTaskCenterApp.vue',
    ]
    for (const p of callers) {
      const src = read(p)
      expect(src, p).not.toContain('label="端点调试"')
      expect(src, p).not.toContain("label: '端点调试'")
    }
  })
})

describe('R302 DR-07 管理区标题语义与部署页 AI 容器', () => {
  it('部署管理页有 aiPageB5 标记，标题层级符合两种合法模式之一，无重复 shell h1', () => {
    const tpl = template(deploy)
    expect(tpl).toContain('ai-page-b5')
    const h1Count = (tpl.match(/<h1[\s>]/g) ?? []).length
    const h2Count = (tpl.match(/<h2[\s>]/g) ?? []).length
    const hidesShellTitle =
      /inject\(ADMIN_LAYOUT_HIDE_TITLE/.test(deploy) && /\.value = true/.test(deploy)
    // 合法模式 (a)：shell h1 模式——页内 h2 分区标题且无本地 h1。
    // 合法模式 (b)：页面自持唯一 h1——页内恰好 1 个 h1，且同文件注入 ADMIN_LAYOUT_HIDE_TITLE 隐藏 shell h1。
    const shellH1Mode = h1Count === 0 && h2Count >= 1
    const selfHeldH1Mode = h1Count === 1 && hidesShellTitle
    expect(
      shellH1Mode || selfHeldH1Mode,
      '标题层级须为 shell-h1 模式(页内 h2 无 h1)或页面自持唯一 h1 模式(1 个 h1 + 隐藏 shell h1)',
    ).toBe(true)
    expect(tpl).not.toContain('端点调试')
    expect(tpl).not.toContain('class="mono"')
  })

  it('管理页只保留 AdminLayout shell h1，页内用 h2 分区标题，保留既有关键 testid', () => {
    for (const p of ['CmsIndexApp.vue', 'ProcessManagerApp.vue', 'ProcessTaskCenterApp.vue']) {
      const tpl = template(read(p))
      expect(tpl, p).not.toMatch(/<h1[\s>]/)
      expect(tpl, p).toMatch(/<h2[\s>]/)
      expect(tpl, p).not.toContain('label="端点调试"')
    }
  })
})
