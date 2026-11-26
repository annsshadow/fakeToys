// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t37 RED→GREEN：工作台待办主标题必须业务名优先，原始流程键折叠为副文本。
import { readFileSync } from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { fileURLToPath } from 'node:url'
import { transformWithEsbuild } from 'vite'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const shared = readFileSync(path.join(here, '..', 'utils', 'processTitle.ts'), 'utf8')
const dash = readFileSync(path.join(here, 'Dashboard.vue'), 'utf8')
const uuid = '71d52ed3-6d4d-4c0e-9b1e-0a9b1e163700e'
const rawKey = 'work-1795102580411-' + uuid.replace(/-/g, '').slice(0, 16)

async function api(): Promise<Record<string, any>> {
  const snippet = (await transformWithEsbuild(shared, 'processTitle.ts', { loader: 'ts', format: 'cjs', target: 'es2022' })).code
  const sandbox: Record<string, any> = { exports: {}, module: { exports: {} } }
  vm.createContext(sandbox)
  new vm.Script(snippet).runInContext(sandbox)
  return sandbox.module.exports
}
const helpers = await api()

describe('Dashboard 待办标题可读性（R302-DR06-R）', () => {
  it('RED：断言共享工具已抽出且旧模板直出原始键的行被替换', () => {
    const oldExpr = "item.title || item.processName || '新审批'"
    expect(dash).not.toContain(oldExpr)
    expect(dash).toContain('data-testid="dashboard-todo-list"')
  })
  it('GREEN：共享 helper 业务名优先、原始键回退可读标题并折叠副文本', () => {
    expect(helpers.displayTodoTitle('报销审批', rawKey)).toBe('报销审批')
    expect(helpers.displayTodoTitle(rawKey, undefined)).toBe('工作实例')
    expect(helpers.displayTodoTitle(rawKey)).not.toContain(rawKey)
    expect(helpers.foldRawKeySubtext(rawKey)).toMatch(/^流程 · work-/)
    expect(helpers.foldRawKeySubtext('报销审批')).toBe('')
    const script = dash.slice(dash.indexOf('<script setup'), dash.indexOf('</script>'))
    expect(script).toContain('displayTodoTitle')
    expect(script).toContain('foldRawKeySubtext')
    expect(dash).toContain('class="pending-title"')
    expect(dash).toContain('data-testid="dashboard-todo-handle"')
  })
})
