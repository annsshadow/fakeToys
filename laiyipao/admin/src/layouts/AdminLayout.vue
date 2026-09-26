<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

interface NavItem {
  path: string
  title: string
  icon: string
}

const groups: Array<{ title: string; items: NavItem[] }> = [
  {
    title: '运营概览',
    items: [{ path: '/admin/dashboard', title: '数据看板', icon: 'DataLine' }],
  },
  {
    title: '游戏内容',
    items: [
      { path: '/admin/levels', title: '关卡配置', icon: 'Grid' },
      { path: '/admin/skills', title: '技能与配方', icon: 'MagicStick' },
      { path: '/admin/equipment', title: '装备宝石皮肤', icon: 'Briefcase' },
    ],
  },
  {
    title: '玩家与战斗',
    items: [
      { path: '/admin/users', title: '用户管理', icon: 'User' },
      { path: '/admin/battles', title: '战斗记录与验真', icon: 'Aim' },
      { path: '/admin/defenses', title: '防线值守', icon: 'Shield' },
    ],
  },
  {
    title: '商业化与运营',
    items: [
      { path: '/admin/economy', title: '经济与商城', icon: 'Coin' },
      { path: '/admin/content', title: '公告与兑换码', icon: 'Bell' },
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
