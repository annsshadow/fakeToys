// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R303 DR07-R 回归契约：部署页标题层级唯一且语义正确；CMS/流程实例页纳入 B5 统一视觉。 */
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

const here = dirname(fileURLToPath(import.meta.url))
const read = (p: string) => readFileSync(resolve(here, p), 'utf8')

function template(src: string): string {
  const start = src.indexOf('<template>')
  if (start < 0) return ''
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

const deploymentSrc = read('DeploymentApp.vue')
const deployment = template(deploymentSrc)
const cms = template(read('CmsIndexApp.vue'))
const processManager = template(read('ProcessManagerApp.vue'))

describe('R303 DR07-R 部署页标题层级', () => {
  it('部署页标题为唯一 h1，描述为 p 段落，保留 testid 与操作', () => {
    const h1s = deployment.match(/<h1\b/g) ?? []
    expect(h1s.length, 'DeploymentApp 模板应只有一个 h1').toBe(1)
    expect(deployment).toContain('<h1 class="ai-gradient-text" data-testid="deployment-page-title">部署管理</h1>')
    expect(deployment).toContain('<p class="subtitle">维护应用部署记录与运行状态</p>')
    expect(deployment).not.toContain('<h2 class="subtitle">')
    expect(deployment).toContain('data-testid="deployment-create"')
    expect(deploymentSrc).toContain('inject(ADMIN_LAYOUT_HIDE_TITLE')
    expect(deploymentSrc).toMatch(/hideTitle\.value = true/)
  })
})

describe('R303 DR07-R 管理页 B5 统一视觉', () => {
  it('CMS 索引与流程实例管理根容器含 modern-page/ai-page-b5/ai-panel', () => {
    for (const [name, tpl] of [['CmsIndexApp', cms], ['ProcessManagerApp', processManager]] as const) {
      const root = tpl.match(/<div class="([^"]+)"/)?.[1] ?? ''
      expect(root, name + ' 根 class').toContain('modern-page')
      expect(root, name + ' 根 class').toContain('ai-page-b5')
      expect(root, name + ' 根 class').toContain('ai-panel')
    }
  })

  it('保留关键 testid、业务入口与端点调试中文化，不嵌入 API 路径', () => {
    expect(cms).toContain('data-testid="cms-index-page-title"')
    expect(processManager).toContain('data-testid="process-manager-page-title"')
    for (const tpl of [cms, processManager]) {
      expect(tpl).toContain('EndpointFold label="高级操作"')
      expect(tpl).not.toContain('label="端点调试"')
      expect(tpl).not.toContain('/api/')
    }
  })
})
