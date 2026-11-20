// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R301：CMS 索引页调试端点默认折叠契约。
 * 源码级断言——91 个 API 调试按钮一个不丢，仅按语义分入 EndpointFold 的
 * 读/写/配置三组具名插槽；页头不再外露 /api 端点副标题。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'CmsIndexApp.vue'), 'utf8')
const template = src.slice(src.indexOf('<template>'), src.lastIndexOf('</template>'))

const EXPECTED_BUTTONS = 91

describe('R301 CmsIndexApp 端点折叠契约', () => {
  it('保留 AdminPageHeader 主标题，但不再外露 /api 端点副标题', () => {
    expect(template).toContain('<AdminPageHeader title="CMS 索引设计"')
    expect(template).not.toMatch(/subtitle="[^"]*\/api\//)
    expect(template).not.toMatch(/<p class="subtitle">/)
  })

  it('使用全局 EndpointFold 容器并配置三组', () => {
    expect(template).toContain('<EndpointFold')
    expect(template).toContain('</EndpointFold>')
    expect(template).toContain(':groups="foldGroups"')
    expect(template).toContain(':total-count="endpointCount"')
    expect(src).toMatch(/import \{[^}]*EndpointFold[^}]*\} from '\.\.\/components\/admin\//)
    expect(src).toMatch(/const foldGroups = \[/)
    for (const key of ['read', 'write', 'config']) {
      expect(template).toContain(`#group-${key}`)
    }
    expect((src.match(/key: '(?:read|write|config)'/g) ?? []).length).toBe(3)
  })

  it('分组总数不变：91 个 btn-refresh 全部保留且都在折叠插槽内', () => {
    const allButtons = (template.match(/<button class="btn-refresh"/g) ?? []).length
    expect(allButtons).toBe(EXPECTED_BUTTONS)
    const slotButtons = [...template.matchAll(/<template #group-(?:read|write|config)>([\s\S]*?)<\/template>/g)].reduce(
      (sum, m) => sum + (m[1].match(/<button class="btn-refresh"/g) ?? []).length,
      0,
    )
    expect(slotButtons).toBe(EXPECTED_BUTTONS)
    expect(src).toContain('const endpointCount = 91')
  })

  it('各类处理器逐字保留，未被折叠改写', () => {
    const handlers = [
      '@click="loadData"',
      '@click="loadCmsDeepReads"',
      "@click=\"cmsQuery('docFilterPaging')",
      "@click=\"cmsQuery3('viewUnread')",
      "@click=\"cmsRoot('formUpdate')",
      "@click=\"cmsWrite2('noteDelete')",
      "@click=\"cmsWrite3('scriptRunApp')",
      "@click=\"cmsDelete('templateform')",
      '@click="createCmsArticle"',
      '@click="cmsSaveConfig"',
      '@click="cmsUpdateDocument"',
      '@click="toggleViewPublish(true)"',
    ]
    for (const handler of handlers) {
      expect(template, handler).toContain(handler)
    }
  })

  it('默认折叠由 EndpointFold 承担：未传 defaultOpen', () => {
    expect(template).not.toContain('default-open')
    expect(template).not.toContain(':defaultOpen')
  })
})
