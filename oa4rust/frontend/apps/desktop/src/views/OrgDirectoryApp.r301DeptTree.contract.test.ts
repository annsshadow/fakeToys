// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const dirSource = readFileSync(resolve(import.meta.dirname, 'OrgDirectoryApp.vue'), 'utf8')
const viewerSource = readFileSync(resolve(import.meta.dirname, 'OrgViewer.vue'), 'utf8')
const dirScript = dirSource.slice(dirSource.indexOf('<script'), dirSource.indexOf('</script>'))
const viewerScript = viewerSource.slice(viewerSource.indexOf('<script'), viewerSource.indexOf('</script>'))

function extractWithTimeout(src: string): (p: Promise<unknown>, ms: number) => Promise<unknown> {
  const m = src.match(/function withTimeout\([^)]*\)\s*:\s*Promise<unknown>\s*\{([\s\S]*?)\n\}/)
  if (!m) throw new Error('未找到 withTimeout 函数体')
  return new Function('p', 'ms', m[1]) as (p: Promise<unknown>, ms: number) => Promise<unknown>
}

describe('R301 U-1 组织树/部门树加载兜底', () => {
  it('withTimeout 能在挂起时 reject，避免 loading 常驻', async () => {
    const withTimeout = extractWithTimeout(dirScript)
    await expect(withTimeout(new Promise(() => {}), 20)).rejects.toThrow('请求超时')
  })

  it('部门树请求经超时包裹，catch 落持久错误态，finally 复位 busy', () => {
    expect(dirScript).toMatch(/await withTimeout\(\s*api\.get\(\s*'\/api\/departments\/tree'/)
    expect(dirScript).toMatch(/deptTreeError\.value\s*=/)
    expect(dirScript).toMatch(/finally\s*\{[\s\S]{0,160}?busy\.value\s*=\s*false/)
    expect(dirTemplate()).toContain('data-testid="dept-tree-retry"')
  })

  it('部门树重试按钮重新加载，空数据有中文说明', () => {
    expect(dirScript).toMatch(/function loadDeptTree\(\)/)
    expect(dirScript).toMatch(/onMounted\(\s*\(\)\s*=>\s*\{[\s\S]{0,180}?void loadDeptTree\(\)/)
    expect(dirTemplate()).toContain('data-testid="dept-tree-error"')
    expect(dirTemplate()).toContain('data-testid="dept-tree-empty"')
    expect(dirTemplate()).toContain('重试加载')
  })

  it('OrgViewer 组树首屏加载改用 api.put（后端 group/list/like 为 PUT）', () => {
    expect(viewerScript).toMatch(/await withTimeout\(\s*api\.put\(\s*'\/api\/organization\/assemble\/control\/group\/list\/like'/)
    expect(viewerScript).not.toMatch(/await withTimeout\(\s*api\.get\(\s*'\/api\/organization\/assemble\/control\/group\/list\/like'/)
  })
})

function dirTemplate(): string {
  return dirSource.slice(dirSource.indexOf('<template>'), dirSource.indexOf('</template>'))
}
