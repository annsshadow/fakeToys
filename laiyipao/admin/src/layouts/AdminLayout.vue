<script setup lang="ts">
import { ref, computed, provide, readonly, onMounted, onUnmounted, type Component } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  Aim,
  Bell,
  Briefcase,
  Coin,
  DataLine,
  Grid,
  Lock,
  MagicStick,
  Reading,
  SwitchButton,
  User,
  Warning,
} from '@element-plus/icons-vue'

import { adminLogout, adminMe } from '@/api'
import { setToken, invalidateTokenVerification } from '@/api/client'
import { ADMIN_ROLE_KEY } from '@/utils/adminRole'

/**
 * 窄屏标记（2026-10-21 P6-11）。
 *
 * 此前侧边栏是**固定 216px**，而项目里 0 个 `@media` ——
 * 于是在 375px 的手机上，侧栏本身就占了 57% 的宽度。
 *
 * 用 `window.matchMedia` 而不是监听 resize：后者每次缩放都触发一次重排。
 */
const NARROW_QUERY = '(max-width: 900px)'
const narrow = ref(false)
let narrowMedia: MediaQueryList | null = null
function onNarrowChange(e: { matches: boolean }): void {
  narrow.value = e.matches
}
onMounted(() => {
  if (typeof window === 'undefined' || !window.matchMedia) return
  narrowMedia = window.matchMedia(NARROW_QUERY)
  narrow.value = narrowMedia.matches
  narrowMedia.addEventListener('change', onNarrowChange)
})
onUnmounted(() => {
  narrowMedia?.removeEventListener('change', onNarrowChange)
  narrowMedia = null
})

const route = useRoute()
const router = useRouter()
const loggingOut = ref(false)

/**
 * 导航项的图标是**组件引用**，不是字符串名。
 *
 * ⚠️ 之前这里是 `icon: string`（`'DataLine'` 这种），模板写
 * `<component :is="item.icon" />`。Vue 对字符串 `:is` 会去解析
 * **全局注册的组件名**，而本项目从来没有注册过任何图标：
 *
 *   - `main.ts` 只 `app.use(ElementPlus)` —— 那是组件，不是图标包
 *   - `vite.config.ts` 里只有 `vue()`，没配 `unplugin-vue-components`
 *     （它在 devDependencies 里，但**从未被使用**）
 *   - 本文件也没有任何 import
 *
 * 实测后果（不注册任何图标、直接挂载）：
 *   `el-icon` 外壳 10 个、`svg` **0** 个、未解析的名字被降级成
 *   `<data-line>` / `<reading>` 这类无意义自定义元素。
 *   **也就是说侧边栏 10 个图标全是空的**，只有文字标签撑着。
 *
 * 改成组件引用后：名字写错就是**编译错误**，而不是运行时一个空元素。
 */
interface NavItem {
  path: string
  title: string
  icon: Component
}

const groups: Array<{ title: string; items: NavItem[] }> = [
  {
    title: '运营概览',
    items: [{ path: '/admin/dashboard', title: '数据看板', icon: DataLine }],
  },
  {
    title: '游戏内容',
    items: [
      { path: '/admin/levels', title: '关卡配置', icon: Grid },
      { path: '/admin/skills', title: '技能与配方', icon: MagicStick },
      { path: '/admin/equipment', title: '装备宝石皮肤', icon: Briefcase },
    ],
  },
  {
    title: '玩家与战斗',
    items: [
      { path: '/admin/users', title: '用户管理', icon: User },
      { path: '/admin/battles', title: '战斗记录与验真', icon: Aim },
      // 重算审计紧跟在战报页后面：两者回答的是同一个问题的两层 ——
      // 战报页看「这一局长什么样」，这一页看「重算认为它对不对、拒没拒」。
      { path: '/admin/recomputes', title: '结算重算审计', icon: Warning },
      { path: '/admin/defenses', title: '防线值守', icon: Lock },
    ],
  },
  {
    title: '商业化与运营',
    items: [
      { path: '/admin/economy', title: '经济与商城', icon: Coin },
      { path: '/admin/content', title: '公告与兑换码', icon: Bell },
    ],
  },
]

const activePath = computed(() => route.path)

/**
 * 当前管理员身份（2026-10-21 P5-6）。
 *
 * 此前 `adminMe()` **从未被调用** —— 于是 `role` 从未被读取，
 * readonly 账号看得见全部写按钮，只有点下去吃一个 403 才知道自己不能写。
 *
 * 现在布局在挂载时取一次身份，并把它挂到 `provide` 上，
 * 视图用 `useAdminRole()` 读。
 */
const adminRole = ref<string>('')
const adminName = ref<string>('')

/** 是不是只读账号。未知时按「可写」处理（宁可多显示，也不要误伤）。 */
const isReadonly = computed(() => adminRole.value === 'readonly')

async function loadIdentity() {
  try {
    const res = await adminMe()
    adminRole.value = res.admin?.role ?? ''
    adminName.value = res.admin?.username ?? ''
  } catch {
    // 取不到身份**不阻断**后台使用：服务端仍会对写操作做鉴权，
    // 这里只是让界面少显示几个按钮而已。
  }
}
const currentTitle = computed(() => (route.meta.title as string) || '来一炮')
const apiLabel = import.meta.env.VITE_API_BASE || '127.0.0.1:8080'

/**
 * 第 155 轮：后台登出入口。
 *
 * 服务端真切断（`POST /admin/logout` 吊销该管理员全部会话），
 * 不是只清 localStorage —— 后者那枚 2h 有效的签名合法令牌还在
 * 别人手上 / 别的标签页里继续可用。
 *
 * 登出动作本身对 readonly 也放行（会话生命周期不是数据写，
 * 见 readonly_guard_test.go 的 /logout 豁免理由）；这里不区分角色，
 * 所有人都能立刻切断自己的凭证。
 */
async function logout() {
  loggingOut.value = true
  try {
    await adminLogout()
  } catch {
    // 服务端吊销失败（如网络错误）不影响前端清令牌：
    // 凭证还在就留在，清掉本地的这一份仍是净收益，不阻塞跳转。
  } finally {
    setToken('')
    invalidateTokenVerification()
    router.replace('/admin/login')
  }
}

// 把身份 provide 给子视图：它们用 `useAdminRole()` 读。
provide(ADMIN_ROLE_KEY, { role: readonly(adminRole), name: readonly(adminName), isReadonly })

onMounted(loadIdentity)
</script>

<template>
  <el-container class="admin-shell">
    <el-aside width="216px" class="sidebar" :class="{ 'aside-narrow': narrow }">
      <div class="brand">
        <div class="brand-mark">炮</div>
        <div class="brand-text">
          <div class="brand-name">来一炮</div>
          <div class="brand-sub">运营后台</div>
        </div>
      </div>

      <el-scrollbar class="nav-scroll">
        <div v-for="g in groups" :key="g.title" class="nav-group">
          <div class="nav-group-title">{{ g.title }}</div>
          <router-link
            v-for="item in g.items"
            :key="item.path"
            :to="item.path"
            class="nav-item"
            :class="{ active: activePath === item.path }"
            :title="item.title"
          >
            <el-icon><component :is="item.icon" /></el-icon>
            <span class="nav-label">{{ item.title }}</span>
          </router-link>
        </div>
      </el-scrollbar>

      <div class="sidebar-foot">
        <router-link to="/wiki" class="wiki-link" title="玩家玩法文档站">
          <el-icon><Reading /></el-icon>
          <span class="nav-label">玩家玩法文档站</span>
        </router-link>
      </div>
    </el-aside>

    <el-container class="shell-body">
      <el-header class="topbar">
        <div class="topbar-title">{{ currentTitle }}</div>
        <div class="topbar-right">
          <el-tag v-if="adminName" type="info" size="small" effect="plain">{{ adminName }}</el-tag>
          <el-tag type="info" size="small" effect="dark" class="api-tag">API {{ apiLabel }}</el-tag>
          <el-button
            size="small"
            :loading="loggingOut"
            class="logout-btn"
            @click="logout"
          >
            <el-icon style="margin-right: 4px"><SwitchButton /></el-icon>
            登出
          </el-button>
        </div>
      </el-header>
      <el-main id="main-content" class="content">
        <router-view v-slot="{ Component }">
          <keep-alive :max="4">
            <component :is="Component" />
          </keep-alive>
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.admin-shell {
  height: 100vh;
}

.shell-body {
  min-width: 0;
}

.sidebar {
  background: var(--lyp-panel);
  border-right: 1px solid var(--lyp-border-soft);
  display: flex;
  flex-direction: column;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 16px 16px 14px;
  border-bottom: 1px solid var(--lyp-border-soft);
  overflow: hidden;
}

.brand-mark {
  flex: 0 0 auto;
  width: 34px;
  height: 34px;
  border-radius: 9px;
  background: var(--lyp-accent-grad);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 17px;
  color: #fff;
}

.brand-text {
  min-width: 0;
}

.brand-name {
  font-weight: 600;
  font-size: 15px;
  white-space: nowrap;
}

.brand-sub {
  font-size: 11px;
  color: var(--lyp-muted);
  white-space: nowrap;
}

.nav-scroll {
  flex: 1;
  padding: 8px 0;
}

.nav-group {
  margin-bottom: 10px;
}

.nav-group-title {
  font-size: 11px;
  color: var(--lyp-dim);
  padding: 10px 16px 6px;
  letter-spacing: 1px;
  white-space: nowrap;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 9px 16px;
  color: var(--lyp-text-2);
  text-decoration: none;
  font-size: 13px;
  border-left: 2px solid transparent;
  transition: background 0.15s, color 0.15s;
  white-space: nowrap;
  overflow: hidden;
}

.nav-item .el-icon {
  flex: 0 0 auto;
}

.nav-label {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-item:hover {
  background: var(--lyp-panel-2);
  color: var(--lyp-text);
}

.nav-item.active {
  background: var(--lyp-panel-3);
  color: #ff9d6b;
  border-left-color: var(--lyp-accent);
}

.sidebar-foot {
  border-top: 1px solid var(--lyp-border-soft);
  padding: 10px 0;
}

.wiki-link {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 9px 16px;
  color: var(--lyp-text-2);
  text-decoration: none;
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
}

.wiki-link:hover {
  color: var(--lyp-text);
  background: var(--lyp-panel-2);
}

.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  background: var(--lyp-panel);
  border-bottom: 1px solid var(--lyp-border-soft);
  height: 52px;
}

.topbar-title {
  font-size: 15px;
  font-weight: 600;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 0 0 auto;
}

.logout-btn {
  margin-left: 2px;
}

.content {
  padding: 20px 24px 40px;
  overflow-y: auto;
  background: var(--lyp-bg);
  min-width: 0;
}

/*
 * 窄屏（2026-10-21 P6-11，2026-11-16 修「只改宽度」）。
 *
 * 之前这里只把 aside 宽度改成 64px，**没隐藏文字** ——
 * 于是每个导航项的文字在 64px 里逐字折行，一项高 87~122px，
 * 品牌名高 60px，整条侧栏看起来像被压扁的竖排文本。
 *
 * 现在：文字全部隐藏、图标居中、行高固定 44px，并用 title 属性
 * 保留鼠标悬停提示。
 */
@media (max-width: 900px) {
  .aside-narrow {
    width: 64px !important;
  }
  .aside-narrow .brand {
    justify-content: center;
    padding: 14px 0 12px;
  }
  .aside-narrow .brand-text,
  .aside-narrow .nav-group-title,
  .aside-narrow .nav-label {
    display: none;
  }
  .aside-narrow .nav-item,
  .aside-narrow .wiki-link {
    justify-content: center;
    height: 44px;
    padding: 0;
    gap: 0;
  }
  .aside-narrow .nav-item .el-icon,
  .aside-narrow .wiki-link .el-icon {
    font-size: 18px;
  }
  .content {
    padding: 14px 12px 32px;
  }
  .topbar {
    padding-left: 12px;
    padding-right: 12px;
  }
  /* API 标签在窄屏最不重要，让位给标题与登出 */
  .api-tag {
    display: none;
  }
}

@media (max-width: 520px) {
  .aside-narrow {
    width: 52px !important;
  }
  .content {
    padding: 10px 8px 28px;
  }
}
</style>
