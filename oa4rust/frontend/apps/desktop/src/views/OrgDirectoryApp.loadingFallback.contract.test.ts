// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'OrgDirectoryApp.vue'), 'utf8')
const script = source.slice(source.indexOf('<script'), source.indexOf('</script>'))
const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

/** 从源码提取 withTimeout 的函数体（不含类型注解），转为可执行 JS。 */
function extractWithTimeout(src: string): (p: Promise<unknown>, ms: number) => Promise<unknown> {
  const m = src.match(/function withTimeout\([^)]*\)\s*:\s*Promise<unknown>\s*\{([\s\S]*?)\n\}/)
  if (!m) throw new Error('未找到 withTimeout 函数体')
  // 函数体内部无 TS 类型注解；setTimeout/clearTimeout/Promise/Error 取全局。
  return new Function('p', 'ms', m[1]) as (p: Promise<unknown>, ms: number) => Promise<unknown>
}

describe('DR-03b 通讯录加载超时兜底与空态/重试', () => {
  it('请求挂起时按超时 reject，避免 loading 常驻', async () => {
    const withTimeout = extractWithTimeout(script)
    const hanging = new Promise(() => {})
    await expect(withTimeout(hanging, 20)).rejects.toThrow('请求超时')
  })

  it('请求快速成功时原样返回结果', async () => {
    const withTimeout = extractWithTimeout(script)
    await expect(withTimeout(Promise.resolve({ ok: true }), 1000)).resolves.toEqual({ ok: true })
  })

  it('loadPersonFilter 用 withTimeout 包裹请求，catch 落错误态且 finally 复位 busy', () => {
    expect(script).toMatch(/await withTimeout\(\s*api\.post\(/)
    expect(script).toMatch(/catch[\s\S]{0,200}?loadError\.value\s*=/)
    expect(script).toMatch(/finally\s*\{[\s\S]{0,120}?busy\.value\s*=\s*false/)
  })

  it('模板提供明确错误提示与重试按钮，空态给出加载引导', () => {
    expect(template).toContain('data-testid="org-load-error"')
    expect(template).toContain('data-testid="org-load-retry"')
    expect(template).toContain('loadError')
    expect(template).toContain('data-testid="org-person-empty"')
  })
})
