<script setup lang="ts">
import { computed, type Component } from 'vue'
import { useRoute } from 'vue-router'
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
  User,
} from '@element-plus/icons-vue'

const route = useRoute()

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
 * 之所以一直没人发现：`vue-tsc --noEmit` **不报**未解析组件，
 * 而 `tests/AdminLayout.test.ts` 又用一份**硬编码的图标名列表**
 * 注册了空壳替身（注释写「测试环境没有全局注册」）——
 * 那实际上把**产品缺陷**写成了「测试环境的限制」，测试因此永远绿。
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
      // 原本写的是 `Shield`，而 @element-plus/icons-vue 的 293 个导出里
      // **根本没有 Shield**（最接近的只有 Lock / AlarmClock）。
      // 也就是说这一项即使在修好注册之后仍然是坏的 —— 换成了真实存在的 Lock。
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
const currentTitle = computed(() => (route.meta.title as string) || '来一炮')
const apiLabel = import.meta.env.VITE_API_BASE || '127.0.0.1:8080'
</script>

<template>
  <el-container class="admin-shell">
    <el-aside width="216px" class="sidebar">
      <div class="brand">
        <div class="brand-mark">炮</div>
        <div>
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
          >
            <el-icon><component :is="item.icon" /></el-icon>
            <span>{{ item.title }}</span>
          </router-link>
        </div>
      </el-scrollbar>

      <div class="sidebar-foot">
        <router-link to="/wiki" class="wiki-link">
          <el-icon><Reading /></el-icon>
          <span>玩家玩法文档站</span>
        </router-link>
      </div>
    </el-aside>

    <el-container>
      <el-header class="topbar">
        <div class="topbar-title">{{ currentTitle }}</div>
        <div class="topbar-right">
          <el-tag type="info" size="small" effect="dark">API {{ apiLabel }}</el-tag>
        </div>
      </el-header>
      <el-main class="content">
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

.sidebar {
  background: #10151c;
  border-right: 1px solid var(--lyp-border);
  display: flex;
  flex-direction: column;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 16px 16px 14px;
  border-bottom: 1px solid var(--lyp-border);
}

.brand-mark {
  width: 34px;
  height: 34px;
  border-radius: 8px;
  background: linear-gradient(135deg, #ff6b35, #ff3d71);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 17px;
  color: #fff;
}

.brand-name {
  font-weight: 600;
  font-size: 15px;
}

.brand-sub {
  font-size: 11px;
  color: var(--lyp-muted);
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
  color: #6e7681;
  padding: 10px 16px 6px;
  letter-spacing: 1px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 9px 16px;
  color: #adbac7;
  text-decoration: none;
  font-size: 13px;
  border-left: 2px solid transparent;
  transition: all 0.15s;
}

.nav-item:hover {
  background: #1a2029;
  color: var(--lyp-text);
}

.nav-item.active {
  background: #1c2430;
  color: #ff9d6b;
  border-left-color: #ff6b35;
}

.sidebar-foot {
  border-top: 1px solid var(--lyp-border);
  padding: 10px 0;
}

.wiki-link {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 9px 16px;
  color: #adbac7;
  text-decoration: none;
  font-size: 13px;
}

.wiki-link:hover {
  color: var(--lyp-text);
}

.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #10151c;
  border-bottom: 1px solid var(--lyp-border);
  height: 52px;
}

.topbar-title {
  font-size: 15px;
  font-weight: 600;
}

.content {
  padding: 20px;
  overflow-y: auto;
  background: var(--lyp-bg);
}
</style>
