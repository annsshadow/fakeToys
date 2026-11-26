<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="settings-view" data-testid="settings-business-view">
    <div class="view-header glass-card settings-header" data-testid="settings-header">
      <h1>系统设置</h1>
      <p class="subtitle">系统参数配置与管理</p>
    </div>
    <span class="sr-only" role="status" aria-live="polite" data-a11y-hidden="true" data-testid="settings-section-status">当前设置分组：{{ activeSectionLabel }}</span>
    <div class="settings-layout ai-grid-2">
      <aside class="s-sidebar glass-card" data-testid="settings-sidebar">
        <h2 class="section-title">设置分组</h2>
        <ul class="s-nav" role="tablist" aria-label="设置分组">
          <li v-for="s in sections" :key="s.id" class="s-item" role="tab" :aria-selected="activeSection===s.id" :class="{active:activeSection===s.id}" @click="activeSection=s.id">
            <span class="s-icon">{{s.icon}}</span><span class="s-label">{{s.label}}</span>
          </li>
        </ul>
      </aside>

      <main class="s-main glass-card" data-testid="settings-main">
        <div v-if="activeSection==='basic'" class="s-panel basic-panel">
          <h2>基础配置</h2>
          <form @submit.prevent="saveCfg('basic')">
            <div class="form-grid">
              <div class="fg"><label for="cfgName">系统名称</label><input id="cfgName" v-model="cfg.systemName" class="fi" /></div>
              <div class="fg"><label for="cfgLogo">系统 Logo</label><input id="cfgLogo" v-model="cfg.logoUrl" class="fi" placeholder="图片 URL" /></div>
              <div class="fg"><label for="cfgTz">时区</label><select id="cfgTz" v-model="cfg.timezone" class="fs"><option value="Asia/Shanghai">Asia/Shanghai</option><option value="UTC">UTC</option></select></div>
              <div class="fg"><label for="cfgLocale">语言</label><select id="cfgLocale" v-model="cfg.locale" class="fs"><option value="zh-cn">中文</option><option value="en">English</option><option value="es">Español</option></select></div>
            </div>
            <button type="submit" class="save-btn" :disabled="saving">保存设置</button>
          </form>
        </div>

        <div v-if="activeSection==='security'" class="s-panel security-panel">
          <h2>安全配置</h2>
          <form @submit.prevent="saveCfg('security')">
            <div class="form-grid">
              <div class="fg"><label for="cfgPwdLen">密码最小长度</label><input id="cfgPwdLen" v-model.number="cfg.minPwdLen" type="number" class="fi" /></div>
              <div class="fg"><label for="cfgLock">登录失败锁定次数</label><input id="cfgLock" v-model.number="cfg.lockAttempts" type="number" class="fi" /></div>
              <div class="fg"><label for="cfgTimeout">会话超时（分钟）</label><input id="cfgTimeout" v-model.number="cfg.sessionTimeout" type="number" class="fi" /></div>
              <div class="fg"><label for="cfgRegister">允许注册</label><select id="cfgRegister" v-model="cfg.allowRegister" class="fs"><option :value="true">是</option><option :value="false">否</option></select></div>
            </div>
            <button type="submit" class="save-btn" :disabled="saving">保存设置</button>
          </form>
        </div>

        <div v-if="activeSection==='notify'" class="s-panel notify-panel">
          <h2>通知配置</h2>
          <form @submit.prevent="saveToggles">
            <div class="toggle-row" v-for="t in toggles" :key="t.key">
              <label class="tr-label" :for="'cfg-'+t.key">{{t.label}}</label>
              <label class="switch"><input :id="'cfg-'+t.key" v-model="t.on" type="checkbox" /><span class="slider"></span></label>
            </div>
            <button type="submit" class="save-btn" :disabled="saving">保存</button>
          </form>
        </div>

        <div v-if="activeSection==='about'" class="s-panel about-panel">
          <h2>关于 OA4Rust</h2>
          <div class="about-info">
            <div class="logo-box">◆</div>
            <div class="ai-text">
              <div class="ai-name">OA4Rust</div>
              <div class="ai-version">版本 0.1.0</div>
              <div class="ai-desc">基于 Rust + Axum + SeaORM 构建的企业级 OA 平台前端</div>
              <div class="ai-tech">Vue 3.5 · TypeScript · Vite 5 · Naive UI · TanStack Query</div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, onMounted, ref } from 'vue'
import { toast } from '../utils/toast'

const activeSection = ref('basic')
// R267：系统设置保存为全量 UPSERT，网络慢时并发点击会让旧值覆盖新值；
// 同步占用 saving，按钮禁用，finally 复位（请求真正 settle 后才放开）。
const saving = ref(false)

const sections = [
  { id: 'basic', label: '基础配置', icon: '⚙' },
  { id: 'security', label: '安全设置', icon: '🔒' },
  { id: 'notify', label: '通知配置', icon: '🔔' },
  { id: 'about', label: '关于', icon: 'ℹ' },
]
const activeSectionLabel = computed(() => sections.find((item) => item.id === activeSection.value)?.label ?? '')

const cfg = ref({
  systemName: 'OA4Rust',
  logoUrl: '',
  timezone: 'Asia/Shanghai',
  locale: 'zh-cn',
  minPwdLen: 8,
  lockAttempts: 5,
  sessionTimeout: 7200,
  allowRegister: false,
})
const toggles = ref([
  { key: 'emailNotify', label: '邮件通知', on: true },
  { key: 'smsNotify', label: '短信通知', on: false },
  { key: 'wsNotify', label: 'WebSocket 实时通知', on: true },
  { key: 'captchaLogin', label: '登录验证码', on: true },
  { key: 'twoFactor', label: '双重认证', on: false },
])

// 系统参数读/写为真实现（file_assemble_control / x_system_config 表）：
// GET /api/config/system（configApi.systemConfig 同形状）未删行全量；
// POST /api/config {"configs":[…]} 按 name UPSERT。值统一字符串落库，
// 装载时按本页表单类型（数字/布尔）还原。
async function loadCfg() {
  try {
    const r: any = await api.get('/api/config/system')
    const items: Array<{ name: string; value?: string }> = r?.data ?? []
    const map = Object.fromEntries(items.map((it) => [it.name, it.value ?? '']))
    const store = cfg.value as Record<string, unknown>
    for (const [k, cur] of Object.entries(cfg.value)) {
      const raw = map[k]
      if (raw === undefined) continue
      if (typeof cur === 'number') store[k] = Number(raw)
      else if (typeof cur === 'boolean') store[k] = raw === 'true'
      else store[k] = raw
    }
    for (const t of toggles.value) {
      if (map[t.key] !== undefined) t.on = map[t.key] === 'true'
    }
  } catch (e: any) {
    toast.error(`系统参数读取失败: ${e?.message ?? '未知错误'}`)
  }
}
onMounted(loadCfg)

function buildConfigs() {
  return [
    ...Object.entries(cfg.value).map(([name, value]) => ({ name, value: String(value), category: 'system' })),
    ...toggles.value.map((t) => ({ name: t.key, value: String(t.on), category: 'notify' })),
  ]
}

async function persistConfigs() {
  if (saving.value) return
  saving.value = true
  try {
    await api.post('/api/config', { configs: buildConfigs() })
    toast.success('系统参数已保存')
  } catch (e: any) {
    toast.error(`系统参数保存失败: ${e?.message ?? '未知错误'}`)
  } finally {
    saving.value = false
  }
}
function saveCfg(_section: string) {
  persistConfigs()
}
function saveToggles() {
  persistConfigs()
}
</script>

<style scoped>
.settings-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.settings-layout.ai-grid-2{display:flex;flex:1;gap:var(--ai-grid-2);min-height:0;padding:var(--ai-grid-1) 0}
.sr-only{position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border:0}
.s-sidebar{width:200px;flex-shrink:0;padding:12px}
.s-nav{list-style:none;padding:0;margin:0}
.s-item{display:flex;align-items:center;gap:10px;padding:10px 12px;border-radius:var(--radius-md);cursor:pointer;color:var(--text-secondary);transition:all var(--transition-fast);margin-bottom:4px}
.s-item:hover,.s-item.active{background:var(--color-primary-soft);color:var(--color-primary)}
.s-icon{font-size:18px}
.s-label{font-size:13px;font-weight:500}
.s-main{flex:1;overflow-y:auto;padding:20px}
.s-panel h3{font-size:15px;color:var(--color-primary);margin:0 0 16px;font-family:'Orbitron',sans-serif}
.form-grid{display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-bottom:16px}
.fg{display:flex;flex-direction:column;gap:6px}
.fg label{font-size:12px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px}
.fi,.fs{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--text-primary);font-size:14px;outline:none;font-family:inherit}
.fi:focus,.fs:focus{border-color:var(--color-primary)}
.toggle-row{display:flex;align-items:center;justify-content:space-between;padding:12px 0;border-bottom:1px solid var(--border-subtle)}
.tr-label{font-size:14px;color:var(--text-primary)}
.switch{position:relative;display:inline-block;width:44px;height:24px}
.switch input{opacity:0;width:0;height:0}
.slider{position:absolute;cursor:pointer;inset:0;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:12px;transition:all var(--transition-fast)}
.slider:before{content:'';position:absolute;height:18px;width:18px;left:2px;bottom:2px;background:white;border-radius:50%;transition:all var(--transition-fast)}
input:checked+.slider{background:var(--color-primary);border-color:var(--color-primary)}
input:checked+.slider:before{transform:translateX(20px)}
.save-btn{padding:10px 24px;border-radius:var(--radius-md);border:none;background:var(--ai-gradient-brand);color:white;font-weight:600;cursor:pointer;font-size:14px;transition:all var(--ai-motion-fast) var(--ai-ease-spring)}
.save-btn:hover{transform:translateY(-1px);box-shadow:0 4px 12px var(--color-primary-glow)}
.save-btn:disabled{opacity:.6;cursor:not-allowed;transform:none;box-shadow:none}
.about-info{display:flex;align-items:center;gap:20px;padding:20px}
.logo-box{font-size:48px;color:var(--color-primary);text-shadow:0 0 20px var(--color-primary-glow)}
.ai-text{display:flex;flex-direction:column;gap:4px}
.ai-name{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);font-weight:700}
.ai-version{font-size:12px;color:var(--text-muted)}
.ai-desc{font-size:13px;color:var(--text-secondary)}
.ai-tech{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;margin-top:4px}
@media(max-width:768px){.settings-layout{flex-direction:column}.s-sidebar{width:100%;max-height:120px}.form-grid{grid-template-columns:1fr}}
</style>
