import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'

import { confirmTokenValid, getToken, setToken } from '@/api/client'

/**
 * 路由表与 AdminLayout 的侧边栏一一对应。
 * 新增页面时必须同时改这两处，否则会出现「有路由没菜单」或「有菜单点 404」。
 */
const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/admin/dashboard' },
  { path: '/admin/login', name: 'login', component: () => import('@/views/LoginView.vue'), meta: { title: '登录' } },
  {
    path: '/admin',
    component: () => import('@/layouts/AdminLayout.vue'),
    children: [
      { path: '', redirect: '/admin/dashboard' },
      { path: 'dashboard', name: 'dashboard', component: () => import('@/views/DashboardView.vue'), meta: { title: '数据看板' } },
      { path: 'levels', name: 'levels', component: () => import('@/views/LevelsView.vue'), meta: { title: '关卡配置' } },
      { path: 'skills', name: 'skills', component: () => import('@/views/SkillsView.vue'), meta: { title: '技能与配方' } },
      { path: 'equipment', name: 'equipment', component: () => import('@/views/EquipmentView.vue'), meta: { title: '装备宝石皮肤' } },
      { path: 'users', name: 'users', component: () => import('@/views/UsersView.vue'), meta: { title: '用户管理' } },
      { path: 'battles', name: 'battles', component: () => import('@/views/BattlesView.vue'), meta: { title: '战斗记录与验真' } },
      { path: 'recomputes', name: 'recomputes', component: () => import('@/views/RecomputesView.vue'), meta: { title: '结算重算审计' } },
      { path: 'defenses', name: 'defenses', component: () => import('@/views/DefensesView.vue'), meta: { title: '防线值守' } },
      { path: 'economy', name: 'economy', component: () => import('@/views/EconomyView.vue'), meta: { title: '经济与商城' } },
      { path: 'content', name: 'content', component: () => import('@/views/ContentView.vue'), meta: { title: '公告与兑换码' } },
    ],
  },
  // 玩法文档站在后台壳外，全宽展示。/config 是公开端点，
  // 但 /admin/dashboard 拿不到令牌，所以仍要求登录，避免混入 401 报错。
  { path: '/wiki', name: 'wiki', component: () => import('@/views/WikiView.vue'), meta: { title: '玩家玩法文档站' } },
  { path: '/:pathMatch(.*)*', redirect: '/admin/dashboard' },
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

/**
 * 无令牌时统一跳登录。
 *
 * 不做这个守卫的话，未登录用户会进入后台壳并看到一整页 401 报错 ——
 * 那种体验既不专业也会让人误以为后端挂了。
 *
 * ⚠️ 这里**只管「有没有令牌」，不管「令牌还有效吗」**。
 * 有效性检测在 `api/client.ts` 的 401 分支（handleUnauthorized）——
 * 本轮之前那个分支**根本不存在**，而这段注释却一直声称它在，
 * 于是「令牌失效兜底」实际是个虚构的安全网。
 * 守卫拦得住「从没登录」，拦不住「登录过但令牌过期」；
 * 后者只能由 api 层清掉令牌后重新触发本守卫。
 *
 * ⚠️ 第 154 轮补上第二半：**「令牌存在」不等于「令牌有效」**。
 *
 * 原守卫只看 `getToken()` 真值，于是浏览器控制台执行
 * `localStorage.setItem('lyp_admin_token','x')` 即可进入全部 11 个后台
 * 视图（组件照样加载、路由照样导航），只是每个数据请求返回 401。
 * 这不是服务端越权（`requireAdmin` 每次都查库），但它的后果是真实的：
 * 运营看到的是「一整页 401 报错」，而不是被送回登录页 ——
 * 恰恰是上面那句注释说要避免的体验。而且后台**不按角色隐藏任何按钮**
 * （readonly 也看得见全部写操作），于是「壳进去了」这个事实会让
 * 运营以为自己什么都能点，直到挨个 403。
 *
 * 修法：进入后台壳时**验一次**令牌（`confirmTokenValid`），
 * 无效就当没令牌处理 —— 跳登录页并带上 redirect。
 * 验真结果在同一次会话内缓存，不会每次导航都打一次 /admin/me。
 */
const AUTH_REQUIRED = /^\/admin(?:\/(?!login)(.*))?$/

router.beforeEach(async (to) => {
  if (to.path === '/admin/login') return true
  if (!getToken()) return toLogin(to)

  // /admin 下的页面（含壳外重定向进来的）才需要验真；
  // /wiki 用的是公开端点，令牌无效也不该被拦。
  if (AUTH_REQUIRED.test(to.path)) {
    if (!(await confirmTokenValid())) return toLogin(to)
  }
  return true
})

function toLogin(to: { fullPath: string }) {
  // 保留目标路径，登录后跳回用户本来想去的地方
  return { path: '/admin/login', query: { redirect: to.fullPath } }
}

/**
 * 兜底清理：一旦发现没有令牌就清掉 localStorage 里那枚空/旧的。
 *
 * 真正的「令牌已失效」检测在 api 层（见 client.ts 的 handleUnauthorized）——
 * 那里清掉令牌并改 hash 后，本守卫会在下一次导航时生效。
 * 这里只负责「已经是无令牌状态，别留垃圾」这一半。
 */
router.afterEach(() => {
  if (router.currentRoute.value.path !== '/admin/login' && !getToken()) {
    setToken('')
  }
})
