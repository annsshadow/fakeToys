// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// P4 契约测试：把 /api/authentication/who 返回的 roleList 贯通到管理区守卫的
// 最小链路固定为源码级契约：后端 whoami 必须查出真实角色名（不能恒空），SDK
// restoreCurrentUser 必须把 roleList 映射为 user.roles，管理区守卫必须按
// roles 中的 admin 角色判定。任一端回退，管理员都会被误挡在工作台。
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { describe, expect, it } from 'vitest'

const desktopSrc = (name: string) => path.resolve(__dirname, name)
const whoamiSrc = () => path.resolve(__dirname, '../../../../backend/crates/auth/src/lib.rs')
const sdkSessionSrc = () => path.resolve(__dirname, '../../../packages/sdk/src/session.ts')
const read = (file: string) => readFileSync(file, 'utf-8')

// 抽出 whoami 函数体，避免 login/anonymous 分支的同名占位符干扰断言。
function whoamiBody(): string {
  const source = read(whoamiSrc())
  const start = source.indexOf('pub async fn whoami(')
  expect(start).toBeGreaterThanOrEqual(0)
  const end = source.indexOf('\npub ', start + 1)
  return source.slice(start, end === -1 ? source.length : end)
}

describe('P4 管理区角色可见性：源码级契约', () => {
  it('管理区守卫按 roles 中的 admin 角色放行', () => {
    const main = read(desktopSrc('main.ts'))
    expect(main).toContain('session.state.user?.roles ?? []')
    expect(main).toContain("roles.some((role) => role.toLowerCase() === 'admin')")
    expect(main).toContain("if (!isAdmin) return { name: 'Dashboard' }")
  })

  it('后端 whoami 查询真实角色名填充 roleList，而不是恒空数组', () => {
    const body = whoamiBody()
    expect(body).toContain('auth_role r')
    expect(body).toContain('JOIN auth_person_role pr ON r.id = pr.role_id')
    expect(body).toContain('pr.person_id = $1')
    expect(body).toContain('r.deleted_at IS NULL')
    expect(body).toContain('roleList')
    expect(body).not.toMatch(/map\.insert\(\s*"roleList"\.to_string\(\),\s*Value::Array\(vec!\[\]\)\)/)
  })

  it('SDK restoreCurrentUser 把 whoami 的 roleList 映射为 user.roles', () => {
    const session = read(sdkSessionSrc())
    expect(session).toContain("'/api/authentication/who'")
    expect(session).toContain('roleList')
    expect(session).toContain('roles')
    const start = session.indexOf('async function restoreCurrentUser(')
    const end = session.indexOf('\n  async function ', start + 1)
    const fn = session.slice(start, end === -1 ? session.length : end)
    expect(fn).toContain('roleList')
    expect(fn).toContain('roles')
  })

  it('路由初始化锚点保持原位（守卫仍挂在 router.beforeEach）', () => {
    const main = read(desktopSrc('main.ts'))
    expect(main).toContain("history: createWebHistory('/oa4rust/')")
    expect(main).toContain('new QueryClient(')
    expect(main).toContain('const app = createApp(')
    expect(main).toContain('app.use(router)')
    expect(main).toContain("app.mount('#o2-app-root')")
  })
})
