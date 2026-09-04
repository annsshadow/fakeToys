<template>
  <div class="settings-view">
    <div class="view-header glass-card">
      <h1>系统设置</h1>
      <p class="subtitle">系统参数配置与管理</p>
    </div>

    <!-- 侧边导航 -->
    <div class="settings-layout">
      <aside class="s-sidebar glass-card">
        <ul class="s-nav">
          <li v-for="s in sections" :key="s.id" class="s-item" :class="{active:activeSection===s.id}" @click="activeSection=s.id">
            <span class="s-icon">{{s.icon}}</span><span class="s-label">{{s.label}}</span>
          </li>
        </ul>
      </aside>

      <main class="s-main glass-card">
        <!-- 基础设置 -->
        <div v-if="activeSection==='basic'" class="s-panel">
          <h3>基础配置</h3>
          <div class="form-grid">
            <div class="fg"><label>系统名称</label><input v-model="cfg.systemName" class="fi" /></div>
            <div class="fg"><label>系统 Logo</label><input v-model="cfg.logoUrl" class="fi" placeholder="图片 URL" /></div>
            <div class="fg"><label>时区</label><select v-model="cfg.timezone" class="fs"><option value="Asia/Shanghai">Asia/Shanghai</option><option value="UTC">UTC</option></select></div>
            <div class="fg"><label>语言</label><select v-model="cfg.locale" class="fs"><option value="zh-cn">中文</option><option value="en">English</option><option value="es">Español</option></select></div>
          </div>
          <button class="save-btn" @click="saveCfg('basic')">保存设置</button>
        </div>

        <!-- 安全设置 -->
        <div v-if="activeSection==='security'" class="s-panel">
          <h3>安全配置</h3>
          <div class="form-grid">
            <div class="fg"><label>密码最小长度</label><input v-model.number="cfg.minPwdLen" type="number" class="fi" /></div>
            <div class="fg"><label>登录失败锁定次数</label><input v-model.number="cfg.lockAttempts" type="number" class="fi" /></div>
            <div class="fg"><label>会话超时（分钟）</label><input v-model.number="cfg.sessionTimeout" type="number" class="fi" /></div>
            <div class="fg"><label>允许注册</label><select v-model="cfg.allowRegister" class="fs"><option :value="true">是</option><option :value="false">否</option></select></div>
          </div>
          <button class="save-btn" @click="saveCfg('security')">保存设置</button>
        </div>

        <!-- 通知设置 -->
        <div v-if="activeSection==='notify'" class="s-panel">
          <h3>通知配置</h3>
          <div class="toggle-row" v-for="t in toggles" :key="t.key">
            <span class="tr-label">{{t.label}}</span>
            <label class="switch"><input v-model="t.on" type="checkbox" /><span class="slider"></span></label>
          </div>
          <button class="save-btn" @click="saveToggles">保存</button>
        </div>

        <!-- 关于 -->
        <div v-if="activeSection==='about'" class="s-panel">
          <h3>关于 OA4Rust</h3>
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
import { ref } from 'vue'
import { useQuery } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

const activeSection = ref('basic')

const sections = [
  { id:'basic', label:'基础配置', icon:'⚙' },
  { id:'security', label:'安全设置', icon:'🔒' },
  { id:'notify', label:'通知配置', icon:'🔔' },
  { id:'about', label:'关于', icon:'ℹ' },
]

const cfg = ref({ systemName:'OA4Rust', logoUrl:'', timezone:'Asia/Shanghai', locale:'zh-cn', minPwdLen:8, lockAttempts:5, sessionTimeout:7200, allowRegister:false })
const toggles = ref([
  { key:'emailNotify', label:'邮件通知', on:true },
  { key:'smsNotify', label:'短信通知', on:false },
  { key:'wsNotify', label:'WebSocket 实时通知', on:true },
  { key:'captchaLogin', label:'登录验证码', on:true },
  { key:'twoFactor', label:'双重认证', on:false },
])

const { data } = useQuery({ queryKey:['settings'], queryFn:()=>api.get('/jaxrs/systemconfig/list').then((r:any)=>(r.data??{})), staleTime:300_000 })
if(data.value) Object.assign(cfg.value, data.value)

function saveCfg(_section:string){ console.log('保存', _section, cfg.value) }
function saveToggles(){ console.log('保存通知配置', toggles.value) }
</script>

<style scoped>
.settings-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.settings-layout{display:flex;flex:1;gap:16px;min-height:0}
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
.save-btn{padding:10px 24px;border-radius:var(--radius-md);border:none;background:linear-gradient(135deg,var(--color-primary),var(--color-primary-deep));color:white;font-weight:600;cursor:pointer;font-size:14px;transition:all var(--transition-fast)}
.save-btn:hover{transform:translateY(-1px);box-shadow:0 4px 12px var(--color-primary-glow)}
.about-info{display:flex;align-items:center;gap:20px;padding:20px}
.logo-box{font-size:48px;color:var(--color-primary);text-shadow:0 0 20px var(--color-primary-glow)}
.ai-text{display:flex;flex-direction:column;gap:4px}
.ai-name{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);font-weight:700}
.ai-version{font-size:12px;color:var(--text-muted)}
.ai-desc{font-size:13px;color:var(--text-secondary)}
.ai-tech{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;margin-top:4px}
@media(max-width:768px){.settings-layout{flex-direction:column}.s-sidebar{width:100%;max-height:120px}.form-grid{grid-template-columns:1fr}}
</style>
