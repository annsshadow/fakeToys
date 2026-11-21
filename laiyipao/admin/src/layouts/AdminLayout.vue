<script setup lang="ts">
import { ref, computed, provide, readonly, onMounted, onUnmounted, nextTick, watch, type Component } from 'vue'
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
import { maskApiBase } from '@/utils/apiLabel'

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

/**
 * §1A.5 语义命令入口：AI 洞察分区（独立分区，不混入导航项 .cmd-option）。
 *
 * ⚠️ path 全部从上面的 groups 派生 —— 遵守 §5 单一真源红线：
 *    这里若写 '/admin/levels' 这样的字面量，就是第二份路由表，
 *    侧栏改了路径而这里没改，用户点 AI 命令会跳到 404。
 */
interface AiCommand {
  id: string
  title: string
  keywords: string[]
  path: string
  focusTarget: string
}
const aiCommands: AiCommand[] = [
  {
    id: 'ai-overview',
    title: '查看今日运营洞察',
    keywords: ['洞察', '智能', '分析', 'ai'],
    path: groups[0].items[0].path,
    focusTarget: 'ai-insight-card',
  },
  {
    id: 'ai-funnel',
    title: '复核流失最大关卡',
    keywords: ['流失', '关卡', '洞察', '分析'],
    path: groups[1].items[0].path,
    focusTarget: '',
  },
  {
    id: 'ai-reaction',
    title: '跳转到元素反应分布',
    keywords: ['反应', '元素', '分布', '洞察', '分析'],
    path: groups[0].items[0].path,
    focusTarget: 'reaction-chart',
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
// t33：真域名不得进 DOM；仅回环或显式调试开关显示完整值
const apiLabel = maskApiBase(import.meta.env.VITE_API_BASE, import.meta.env.VITE_SHOW_API_HOST === '1')

/**
 * 命令面板（AI_VISUAL_SYSTEM.md §5，PC 专属）。
 *
 * ⚠️ 导航源**只从上面的 groups 派生**——那份数组已经是侧栏路由表的唯一真源。
 * 这里若再写一份 { title, path } 标签表，就是重蹈 reactionLabel 硬编码的覆辙：
 * 侧栏改了名字而面板没改，用户会在两个地方看到不一致的入口。
 */
const paletteOpen = ref(false)
const cmdQuery = ref('')
/** §1A.5：AI 命令按标题 + 语义关键词（洞察/智能/分析）过滤。 */
const filteredAiCommands = computed(() => {
  const q = cmdQuery.value.trim().toLowerCase()
  if (!q) return aiCommands
  return aiCommands.filter(
    (c) => c.title.toLowerCase().includes(q) || c.keywords.some((k) => k.toLowerCase().includes(q)),
  )
})
const activeOptionPath = ref('')
const cmdInputRef = ref<HTMLInputElement | null>(null)
const panelRef = ref<HTMLElement | null>(null)
let lastFocused: HTMLElement | null = null

const shortcutLabel = computed(() => {
  if (typeof navigator === 'undefined') return 'Ctrl K'
  return /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent) ? '⌘ K' : 'Ctrl K'
})

/** 直接投影 groups；过滤只按「项标题 + 分组标题」匹配，不引入第二份数据。 */
const filteredGroups = computed(() => {
  const q = cmdQuery.value.trim().toLowerCase()
  if (!q) return groups
  return groups
    .map((g) => ({
      title: g.title,
      items: g.items.filter(
        (i) => i.title.toLowerCase().includes(q) || g.title.toLowerCase().includes(q),
      ),
    }))
    .filter((g) => g.items.length > 0)
})

const flatOptions = computed(() => filteredGroups.value.flatMap((g) => g.items))
const hasMatches = computed(() => flatOptions.value.length > 0)
const activeOptionId = computed(() =>
  activeOptionPath.value ? 'cmd-opt-' + activeOptionPath.value : undefined,
)

watch(cmdQuery, () => {
  // 查询变化后高亮回到第一项，避免 activeOption 指向已被过滤掉的路径。
  activeOptionPath.value = flatOptions.value[0]?.path ?? ''
})

function openPalette() {
  lastFocused = document.activeElement as HTMLElement | null
  cmdQuery.value = ''
  activeOptionPath.value = flatOptions.value[0]?.path ?? ''
  paletteOpen.value = true
  nextTick(() => cmdInputRef.value?.focus())
}

function closePalette() {
  paletteOpen.value = false
  nextTick(() => lastFocused?.focus())
}

// 外部路由变化（浏览器前进/后退、侧栏跳转）也要收起面板，避免浮层残留遮挡新页面。
// navigateTo 自身已 closePalette；这里兜住「不经面板选择就换路由」的路径。
watch(() => route.fullPath, () => {
  if (paletteOpen.value) closePalette()
})

function navigateTo(path: string) {
  closePalette()
  void router.push(path)
}

/** §1A.5：执行 AI 命令 —— 关面板、跳真实路由、滚动到目标并聚焦卡内 CTA。 */
function runAiCommand(cmd: AiCommand) {
  closePalette()
  void router.push(cmd.path).then(() => {
    if (!cmd.focusTarget) return
    void nextTick(() => {
      const el = document.getElementById(cmd.focusTarget)
      if (!el) return
      el.scrollIntoView({ block: 'center', behavior: 'smooth' })
      el.querySelector<HTMLElement>('.ai-cta')?.focus()
    })
  })
}

function onGlobalKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    paletteOpen.value ? closePalette() : openPalette()
  } else if (e.key === 'Escape' && paletteOpen.value) {
    e.preventDefault()
    closePalette()
  }
}

function onInputKeydown(e: KeyboardEvent) {
  const opts = flatOptions.value
  if (!opts.length) return
  if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
    e.preventDefault()
    const idx = opts.findIndex((o) => o.path === activeOptionPath.value)
    const next = e.key === 'ArrowDown'
      ? (idx + 1) % opts.length
      : (idx - 1 + opts.length) % opts.length
    activeOptionPath.value = opts[next].path
  } else if (e.key === 'Enter') {
    e.preventDefault()
    if (activeOptionPath.value) navigateTo(activeOptionPath.value)
  }
}

/** 焦点陷阱：面板内可聚焦元素只有输入框 / 清空 / ESC 三个按钮。 */
function onPanelKeydown(e: KeyboardEvent) {
  if (e.key !== 'Tab') return
  const focusables = panelRef.value?.querySelectorAll<HTMLElement>('button, input, [tabindex="0"]')
  if (!focusables?.length) return
  const first = focusables[0]
  const last = focusables[focusables.length - 1]
  if (e.shiftKey && document.activeElement === first) {
    e.preventDefault()
    last.focus()
  } else if (!e.shiftKey && document.activeElement === last) {
    e.preventDefault()
    first.focus()
  }
}

onMounted(() => window.addEventListener('keydown', onGlobalKeydown))
onUnmounted(() => window.removeEventListener('keydown', onGlobalKeydown))

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
        <button type="button" class="command-trigger" aria-haspopup="dialog" aria-label="打开命令面板" @click="openPalette">
          <span class="command-trigger-glyph" aria-hidden="true">⌕</span>
          <span class="command-trigger-label">搜索功能</span>
          <kbd class="command-trigger-kbd">{{ shortcutLabel }}</kbd>
        </button>
        <div class="topbar-right">
          <el-tag v-if="adminName" type="info" size="small" effect="plain">{{ adminName }}</el-tag>
          <el-tag type="info" size="small" effect="dark" class="api-tag">{{ apiLabel }}</el-tag>
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

  <!-- 命令面板（§5）：导航源复用 groups，不另抄一份路由表 -->
  <div v-if="paletteOpen" class="cmd-overlay" @click.self="closePalette">
    <div
      ref="panelRef"
      class="cmd-panel glass-surface"
      role="dialog"
      aria-modal="true"
      aria-label="命令面板"
      @keydown="onPanelKeydown"
    >
      <div class="cmd-input-row">
        <span class="cmd-search-glyph" aria-hidden="true">⌕</span>
        <input
          ref="cmdInputRef"
          v-model="cmdQuery"
          class="cmd-input"
          type="text"
          role="combobox"
          aria-label="搜索页面"
          aria-autocomplete="list"
          aria-controls="cmd-list"
          :aria-expanded="true"
          :aria-activedescendant="activeOptionId"
          placeholder="输入页面名或功能名…"
          @keydown="onInputKeydown"
        />
        <button v-if="cmdQuery" type="button" class="cmd-clear" aria-label="清空搜索" @click="cmdQuery = ''">清空</button>
        <button type="button" class="cmd-esc" @click="closePalette">ESC</button>
      </div>
      <div class="cmd-hint"><span class="muted">↑↓ 选择 · Enter 跳转 · Esc 关闭</span></div>
      <ul id="cmd-list" class="cmd-list" role="listbox" aria-label="页面列表">
        <!-- §1A.5：AI 语义命令分区（独立 class，不混导航项，单一路由源从 groups 派生） -->
        <template v-if="filteredAiCommands.length">
          <li class="cmd-ai-group-label" role="presentation">
            <span class="cmd-ai-spark" aria-hidden="true">✦</span> AI 洞察
          </li>
          <li
            v-for="cmd in filteredAiCommands"
            :key="cmd.id"
            class="cmd-ai-option"
            role="option"
            aria-selected="false"
            @click="runAiCommand(cmd)"
            @mousemove="activeOptionPath = ''"
          >
            <span class="cmd-ai-spark" aria-hidden="true">✦</span>
            <span class="cmd-ai-title">{{ cmd.title }}</span>
            <span class="cmd-ai-hint muted">洞察</span>
          </li>
        </template>
        <template v-for="g in filteredGroups" :key="g.title">
          <li class="cmd-group-label" role="presentation">{{ g.title }}</li>
          <li
            v-for="item in g.items"
            :id="'cmd-opt-' + item.path"
            :key="item.path"
            class="cmd-option"
            :class="{ active: item.path === activeOptionPath }"
            role="option"
            :aria-selected="item.path === activeOptionPath"
            @click="navigateTo(item.path)"
            @mousemove="activeOptionPath = item.path"
          >
            <el-icon><component :is="item.icon" /></el-icon>
            <span class="cmd-option-title">{{ item.title }}</span>
            <span class="cmd-option-group muted">{{ g.title }}</span>
          </li>
        </template>
        <li v-if="!hasMatches" class="cmd-empty">
          <span class="muted">没有匹配的页面，试试其他关键词</span>
        </li>
      </ul>
    </div>
  </div>
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

/* ── 命令面板（§5）：视口 15% 居中 / max 640 / 60vh / 玻璃浮层 ─────────── */
.command-trigger {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  height: 30px;
  padding: 0 6px 0 10px;
  margin-right: 4px;
  border: 1px solid var(--lyp-border);
  border-radius: var(--lyp-radius-sm);
  background: var(--lyp-panel-2);
  color: var(--lyp-muted);
  font-size: var(--lyp-fs-sm);
  cursor: pointer;
  transition: background var(--lyp-motion-fast) ease, border-color var(--lyp-motion-fast) ease, color var(--lyp-motion-fast) ease;
}
.command-trigger:hover {
  background: var(--lyp-panel-3);
  border-color: var(--lyp-accent);
  color: var(--lyp-text);
}
.command-trigger-glyph { font-size: 15px; line-height: 1; color: var(--lyp-accent); }
.command-trigger-kbd,
.cmd-kbd {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 10px;
  padding: 2px 5px;
  border: 1px solid var(--lyp-border);
  border-radius: 4px;
  background: var(--lyp-panel-3);
  color: var(--lyp-text-2);
}
.cmd-overlay {
  position: fixed;
  inset: 0;
  z-index: var(--lyp-z-command);
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding-top: 15vh;
  background: rgba(5, 8, 12, 0.62);
  animation: cmd-fade var(--lyp-motion-fast) ease-out;
}
.cmd-panel {
  width: min(640px, calc(100vw - 32px));
  max-height: 60vh;
  display: flex;
  flex-direction: column;
  border-radius: var(--lyp-radius);
  overflow: hidden;
  animation: cmd-rise var(--lyp-motion-base) var(--lyp-motion-ease);
}
.cmd-input-row {
  display: flex;
  align-items: center;
  gap: var(--lyp-gap-2);
  padding: 10px 12px;
  border-bottom: 1px solid var(--lyp-glass-border);
}
.cmd-search-glyph { color: var(--lyp-accent); font-size: 16px; }
.cmd-input {
  flex: 1;
  min-width: 0;
  height: 30px;
  border: 0;
  outline: 0;
  background: var(--lyp-panel-2);
  border-radius: var(--lyp-radius-sm);
  padding: 0 10px;
  color: var(--lyp-text);
  font-size: var(--lyp-fs-md);
}
.cmd-input::placeholder { color: var(--lyp-glass-text-dim); }
.cmd-clear,
.cmd-esc {
  flex: 0 0 auto;
  border: 1px solid var(--lyp-glass-border);
  background: var(--lyp-panel-2);
  color: var(--lyp-glass-text-muted);
  border-radius: var(--lyp-radius-sm);
  height: 26px;
  padding: 0 8px;
  font-size: var(--lyp-fs-xs);
  cursor: pointer;
}
.cmd-esc { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
.cmd-hint { padding: 8px 14px 4px; font-size: var(--lyp-fs-xs); }
.cmd-list {
  list-style: none;
  margin: 0;
  padding: 0 8px 10px;
  overflow-y: auto;
  min-height: 0;
}
.cmd-group-label {
  padding: 10px 8px 4px;
  font-size: var(--lyp-fs-xs);
  letter-spacing: 1px;
  color: var(--lyp-glass-text-dim);
}
.cmd-option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  border-radius: var(--lyp-radius-sm);
  color: var(--lyp-text-2);
  font-size: var(--lyp-fs-base);
  cursor: pointer;
}
.cmd-option.active,
.cmd-option:hover { background: var(--lyp-panel-3); color: var(--lyp-text); }
.cmd-option-title { flex: 1; min-width: 0; }
.cmd-option-group { font-size: var(--lyp-fs-xs); }
.cmd-empty { padding: 26px 14px; text-align: center; font-size: var(--lyp-fs-sm); }

/* §1A.5：AI 语义命令分区 —— 用 AI 令牌色，与看板 AI 卡同色阶 */
.cmd-ai-group-label {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px 4px;
  font-size: var(--lyp-fs-xs);
  font-weight: 600;
  color: var(--lyp-ai-accent);
}
.cmd-ai-spark { color: var(--lyp-ai-accent); }
.cmd-ai-option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 14px;
  cursor: pointer;
  color: var(--lyp-text);
  font-size: var(--lyp-fs-sm);
}
.cmd-ai-option:hover { background: var(--lyp-ai-halo); }
.cmd-ai-option .cmd-ai-spark {
  flex: 0 0 auto;
  width: 20px;
  text-align: center;
}
.cmd-ai-title { flex: 1; min-width: 0; }
.cmd-ai-hint { font-size: var(--lyp-fs-xs); }
@keyframes cmd-fade { from { opacity: 0; } to { opacity: 1; } }
@keyframes cmd-rise { from { opacity: 0; transform: translateY(-8px); } to { opacity: 1; transform: none; } }
@media (max-width: 900px) {
  .command-trigger-label { display: none; }
  .cmd-overlay { padding-top: 10vh; }
}
</style>
