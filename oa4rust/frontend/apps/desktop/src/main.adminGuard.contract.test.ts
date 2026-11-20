import fs from 'node:fs'
import path from 'node:path'
import { describe, expect, test } from 'vitest'

const mainPath = path.resolve(__dirname, './main.ts')
const mainSource = fs.readFileSync(mainPath, 'utf-8')

describe('main.ts 管理区路由守卫契约', () => {
  test('beforeEach 守卫位于 router 创建之后、app.use(router) 之前', () => {
    expect(mainSource).toContain('router.beforeEach(async (to) => {')
    const routerIndex = mainSource.indexOf('router = createRouter(')
    const guardIndex = mainSource.indexOf('router.beforeEach(')
    const useIndex = mainSource.indexOf('app.use(router)')
    expect(routerIndex).toBeGreaterThan(-1)
    expect(guardIndex).toBeGreaterThan(routerIndex)
    expect(guardIndex).toBeLessThan(useIndex)
  })

  test('原鉴权逻辑保留：初始化会话并拒绝未登录访问', () => {
    expect(mainSource).toContain('await session.init()')
    expect(mainSource).toContain("return { name: 'Login', query: { redirect: to.fullPath } }")
  })

  test('守卫识别 /app/admin 管理区路径', () => {
    expect(mainSource).toContain("to.path.startsWith('/app/admin')")
  })

  test('已登录但无 admin 角色时重定向 Dashboard，且角色判断大小写不敏感', () => {
    expect(mainSource).toContain('session.state.user?.roles ?? []')
    expect(mainSource).toContain("roles.some((role) => role.toLowerCase() === 'admin')")
    expect(mainSource).toContain("if (!isAdmin) return { name: 'Dashboard' }")
  })

  test('路由表与 createApp/queryClient/app.use/app.mount 锚点未被破坏', () => {
    expect(mainSource).toContain("history: createWebHistory('/oa4rust/')")
    expect(mainSource).toContain('new QueryClient(')
    expect(mainSource).toContain('const app = createApp(')
    expect(mainSource).toContain('app.use(router)')
    expect(mainSource).toContain("app.mount('#o2-app-root')")
    const namedRoutes = mainSource.match(/name: '/g) ?? []
    expect(namedRoutes.length).toBeGreaterThan(70)
  })

  test('管理区旧路径重定向仍保留', () => {
    expect(mainSource).toMatch(/\/app\/admin\//)
  })
})
