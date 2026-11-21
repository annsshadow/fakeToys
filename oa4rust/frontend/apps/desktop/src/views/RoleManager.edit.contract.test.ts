// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R335 契约：RoleManager.vue 编辑角色时 PUT /api/role/{flag} 必须真正发出。
 * R334 审计坐实：role_list 不返 flag → editRole 置空 form.flag → 编辑态 input disabled →
 * onSave 校验拦截 → PUT 从未发出，编辑 100% 失效。
 * 断言为源码级静态结构 + new Function 真执行 editRole/onSave 函数体（不引入新依赖）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'RoleManager.vue'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('花括号未配对: ' + signature)
}

const editBody = bodyOf(src, 'function editRole(')
const saveBody = bodyOf(src, 'async function onSave(')
  .replace(/catch \((\w+):\s*any\)/g, 'catch ($1)')

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor

describe('R335 RoleManager.vue 编辑角色 PUT 可达契约', () => {
  it('editRole 源码对 flag 使用 r.id 兜底（role_list 不返 flag）', () => {
    expect(editBody).toContain('r.flag || r.id')
  })

  it('只有 id、无 flag 的角色进入编辑态后 form.flag 非空且等于 id', () => {
    const form = { value: { name: '', flag: '', desc: '' } }
    const editingRole = { value: null as unknown }
    const showCreate = { value: false }
    const fn = new Function('r', 'editingRole', 'form', 'showCreate', editBody)
    fn({ id: 'uuid-123', name: '测试角色', desc: 'd' }, editingRole, form, showCreate)
    expect(form.value.flag).toBe('uuid-123')
    expect(editingRole.value).not.toBeNull()
    expect(showCreate.value).toBe(true)
  })

  it('编辑态 onSave 真发出 PUT 且不被 Flag 校验拦截', async () => {
    const calls = { put: [] as Array<[string, unknown]>, post: [] as Array<[string, unknown]>, info: [] as string[], error: [] as string[], load: 0 }
    const api = {
      put: async function (u: string, p: unknown): Promise<void> { calls.put.push([u, p]) },
      post: async function (u: string, p: unknown): Promise<void> { calls.post.push([u, p]) },
    }
    const toast = { info: (m: string): void => { calls.info.push(m) }, error: (m: string): void => { calls.error.push(m) } }
    const form = { value: { name: '新名称', flag: 'uuid-123', desc: '' } }
    const editingRole = { value: { id: 'uuid-123', name: '原角色' } as unknown }
    const creating = { value: false }
    const showCreate = { value: true }
    const loadRoles = (): void => { calls.load++ }
    const fn = new AsyncFn('api', 'toast', 'form', 'editingRole', 'creating', 'showCreate', 'loadRoles', saveBody)
    await fn(api, toast, form, editingRole, creating, showCreate, loadRoles)
    expect(calls.info, 'Flag 校验不应拦截编辑态').not.toContain('Flag不能为空')
    expect(calls.put.length, '编辑态必须发出 PUT').toBe(1)
    expect(calls.put[0][0]).toBe('/api/role/uuid-123')
    expect((calls.put[0][1] as { name: string }).name).toBe('新名称')
    expect(calls.post.length, '编辑态不应发 POST').toBe(0)
  })

  it('新建态 flag 为空时仍被校验拦截（保护写链不误伤）', async () => {
    const calls = { put: [] as string[], post: [] as string[], info: [] as string[], error: [] as string[], load: 0 }
    const api = {
      put: async function (u: string): Promise<void> { calls.put.push(u) },
      post: async function (u: string): Promise<void> { calls.post.push(u) },
    }
    const toast = { info: (m: string): void => { calls.info.push(m) }, error: (m: string): void => { calls.error.push(m) } }
    const form = { value: { name: '新角色', flag: '', desc: '' } }
    const editingRole = { value: null as unknown }
    const creating = { value: false }
    const showCreate = { value: true }
    const loadRoles = (): void => { calls.load++ }
    const fn = new AsyncFn('api', 'toast', 'form', 'editingRole', 'creating', 'showCreate', 'loadRoles', saveBody)
    await fn(api, toast, form, editingRole, creating, showCreate, loadRoles)
    expect(calls.info).toContain('Flag不能为空')
    expect(calls.put.length).toBe(0)
    expect(calls.post.length).toBe(0)
  })

  it('写读链未回退：list 端点/删除兜底/flag 编辑态 disabled/P2 输入框保持原样', () => {
    expect(src).toContain('/api/authentication/role/list')
    expect(src).toContain('api.delete(')
    expect(src).toContain('r.flag || r.id')
    expect(src).toContain(':disabled="!!editingRole"')
    expect(src).toContain('Flag不能为空')
  })
})
