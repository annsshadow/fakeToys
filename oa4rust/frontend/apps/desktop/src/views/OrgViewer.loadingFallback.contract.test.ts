// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'OrgViewer.vue'), 'utf8')
const script = source.slice(source.indexOf('<script'), source.indexOf('</script>'))
const template = source.slice(source.indexOf('<template>'), source.lastIndexOf('</template>'))

/** 提取 withTimeout 函数体（内部不含 TS 注解），转为可执行 JS。 */
function extractWithTimeout(src: string): (p: Promise<unknown>, ms: number) => Promise<unknown> {
  const m = src.match(/function withTimeout\([^)]*\)\s*:\s*Promise<unknown>\s*\{([\s\S]*?)\n\}/)
  if (!m) throw new Error('未找到 OrgViewer withTimeout 函数体')
  return new Function('p', 'ms', m[1]) as (p: Promise<unknown>, ms: number) => Promise<unknown>
}

describe('R301 DR-03 组织架构加载兜底与中文化', () => {
  it('请求挂起时按超时 reject，避免 loading 常驻', async () => {
    const withTimeout = extractWithTimeout(script)
    const hanging = new Promise(() => {})
    await expect(withTimeout(hanging, 20)).rejects.toThrow('请求超时')
  })

  it('请求快速成功时原样返回结果', async () => {
    const withTimeout = extractWithTimeout(script)
    await expect(withTimeout(Promise.resolve({ ok: true }), 1000)).resolves.toEqual({ ok: true })
  })

  it('loadOrgTree 用 withTimeout 包裹 api.put（group/list/like 为 PUT），catch 落错误态且 finally 复位 treeLoading', () => {
    expect(script).toMatch(/await withTimeout\(\s*api\.put\(\s*'\/api\/organization\/assemble\/control\/group\/list\/like'/)
    expect(script).toMatch(/treeError\.value\s*=/)
    expect(script).toMatch(/finally\s*\{[\s\S]{0,160}?treeLoading\.value\s*=\s*false/)
    expect(script).toMatch(/onMounted\(\s*\(\)\s*=>\s*\{\s*void loadOrgTree\(\)\s*\}\s*\)/)
  })

  it('模板提供明确错误提示与重试按钮，空态给出中文引导', () => {
    expect(template).toContain('data-testid="org-tree-error"')
    expect(template).toContain('data-testid="org-tree-retry"')
    expect(template).toContain('treeError')
    expect(template).toMatch(/nodes\.length\s*===\s*0[\s\S]{0,140}?暂无组织架构数据/)
    expect(template).toContain('placeholder="搜索部门或人员"')
  })

  it('详情与成员区英文提示全部改为中文', () => {
    for (const en of ['Select a node to view details', '>Members<', '>No members<', '>Dept<', '>Person<', '>ID<', '>Name<']) {
      expect(template, en).not.toContain(en)
    }
    expect(template).toContain('选择节点查看详情')
    expect(template).toContain('暂无成员')
  })
})
