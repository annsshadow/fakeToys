<template>
  <div class="personal-view">
    <div class="view-header glass-card">
      <h1>个人中心</h1>
      <p class="subtitle">管理您的账户信息</p>
    </div>

    <div class="profile-card glass-card">
      <div class="avatar-section">
        <div class="avatar-large">{{ user?.name?.charAt(0) || 'U' }}</div>
        <div class="avatar-actions">
          <label class="upload-btn">
            📷 更换头像
            <input type="file" accept="image/*" class="hidden-input" @change="handleAvatarUpload" />
          </label>
        </div>
      </div>
      <div class="profile-info">
        <div class="info-row">
          <span class="info-label">姓名</span>
          <span class="info-value">{{ user?.name || '—' }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">唯一标识</span>
          <span class="info-value mono">{{ user?.unique || '—' }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">手机</span>
          <span class="info-value">{{ user?.mobile || '未设置' }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">邮箱</span>
          <span class="info-value">{{ user?.email || '未设置' }}</span>
        </div>
      </div>
    </div>

    <!-- 密码修改 -->
    <div class="settings-card glass-card">
      <h3>修改密码</h3>
      <div class="form-row">
        <div class="form-group">
          <label>当前密码</label>
          <input v-model="pwdForm.oldPassword" type="password" class="form-input" placeholder="请输入当前密码" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>新密码</label>
          <input v-model="pwdForm.newPassword" type="password" class="form-input" placeholder="请输入新密码" />
        </div>
        <div class="form-group">
          <label>确认密码</label>
          <input v-model="pwdForm.confirmPassword" type="password" class="form-input" placeholder="请再次输入" />
        </div>
      </div>
      <div v-if="pwdError" class="error-msg">{{ pwdError }}</div>
      <button class="save-btn" :disabled="pwdSaving" @click="savePassword">{{ pwdSaving ? '保存中...' : '保存修改' }}</button>
    </div>

    <!-- 签名管理 -->
    <div class="settings-card glass-card">
      <h3>个人签名</h3>
      <textarea v-model="signature" class="form-textarea" rows="3" placeholder="设置您的个性签名..." maxlength="200" />
      <div class="char-count">{{ signature.length }}/200</div>
      <button class="save-btn" @click="saveSignature">保存签名</button>
      <button class="save-btn ghost" @click="loadSignatureManagers">查看全员签名（管理员）</button>
      <div v-if="sigManagers.length" class="sig-mgr-list">
        <span v-for="sm in sigManagers" :key="sm.id" class="sig-mgr-chip">{{ sm.personName || sm.name || sm.id }}</span>
      </div>
    </div>

    <!-- 登录方式/账户绑定 -->
    <div class="settings-card glass-card">
      <h3>登录方式 / 账户绑定</h3>
      <button class="save-btn ghost" @click="loadAuthMeta">查看</button>
      <button class="save-btn ghost" @click="loadOauthConfig">OAuth 配置</button>
      <button class="save-btn ghost" @click="loadMailMeta">内部邮件/注册方式</button>
      <div v-if="authMetaText" class="auth-note">{{ authMetaText }}</div>
    </div>

    <!-- 授权委托 -->
    <div class="settings-card glass-card">
      <h3>授权委托</h3>
      <div class="emp-tabs">
        <button class="emp-tab" :class="{on:empScope==='mine'}" @click="loadEmpower('mine')">我发出的（{{ empMine.length }}）</button>
        <button class="emp-tab" :class="{on:empScope==='to'}" @click="loadEmpower('to')">授权给我（{{ empTo.length }}）</button>
      </div>
      <div v-if="(empScope==='mine'?empMine:empTo).length===0" class="emp-empty">暂无授权记录</div>
      <div v-else class="emp-list">
        <div v-for="e in (empScope==='mine'?empMine:empTo)" :key="e.id" class="emp-item">
          <div class="emp-info"><span class="emp-name">{{ e.toName || e.fromName || e.id }}</span><span class="emp-status" :class="e.enabled!==false?'on':'off'">{{ e.enabled!==false?'启用':'禁用' }}</span></div>
          <div v-if="empScope==='mine'" class="emp-acts">
            <button class="emp-btn" @click="toggleEmpower(e, true)">启用</button>
            <button class="emp-btn" @click="toggleEmpower(e, false)">禁用</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery } from '@tanstack/vue-query'
import { computed, onMounted, ref } from 'vue'
import { toast } from '../utils/toast'

const session = useSession()
const user = computed(() => session.state.user ?? null)

const pwdForm = ref({ oldPassword: '', newPassword: '', confirmPassword: '' })
const pwdError = ref('')
const pwdSaving = ref(false)

const signature = ref('')

// 加载签名
const { data: sigData } = useQuery({
  queryKey: ['personal', 'signature'],
  queryFn: async () => {
    const resp = await api.get('/api/person/signature/list')
    const sigs = ((resp as any)?.data ?? []) as Array<{ content: string }>
    return sigs[0]?.content ?? ''
  },
})
signature.value = sigData.value ?? ''

// 修改密码
const pwdMutation = useMutation({
  mutationFn: (data: { oldPassword: string; newPassword: string }) => api.put('/api/person/password', data),
  onSuccess: () => {
    pwdForm.value = { oldPassword: '', newPassword: '', confirmPassword: '' }
    pwdError.value = ''
  },
  onError: (err: any) => {
    pwdError.value = err?.message ?? '密码修改失败'
  },
})

function savePassword(): void {
  if (!pwdForm.value.oldPassword || !pwdForm.value.newPassword) {
    pwdError.value = '请填写完整密码'
    return
  }
  if (pwdForm.value.newPassword !== pwdForm.value.confirmPassword) {
    pwdError.value = '两次密码不一致'
    return
  }
  pwdSaving.value = true
  pwdMutation.mutate(
    { oldPassword: pwdForm.value.oldPassword, newPassword: pwdForm.value.newPassword },
    {
      onSuccess: () => {
        toast.success('密码修改成功')
        pwdForm.value = { oldPassword: '', newPassword: '', confirmPassword: '' }
      },
      onError: () => {
        pwdError.value = '修改失败'
        pwdSaving.value = false
      },
    },
  )
  pwdSaving.value = true
}

function handleAvatarUpload(e: Event): void {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file || !user.value) return
  const formData = new FormData()
  formData.append('file', file)
  avatarMutation.mutate(formData)
}

const avatarMutation = useMutation({
  mutationFn: (formData: FormData) => api.upload(`/api/person/icon/${user.value!.unique}`, formData),
  onSuccess: () => {
    toast.success('头像上传成功')
  },
  onError: () => {
    toast.error('头像上传失败')
  },
})

function saveSignature(): void {
  api
    .post('/api/person/signature/save', { signature: signature.value, mimeType: 'image/png' })
    .then(() => toast.success('签名已保存'))
    .catch(() => toast.error('保存失败'))
}

interface SigMgr { id: string; name?: string; personName?: string }
const authMetaText = ref('')
async function loadMailMeta() {
  try {
    // 消费 personal 三条真实路由：内部邮件新邮件数 / 标题列表(被动) / 注册方式
    const [count, titles, regist] = await Promise.all([
      api.get('/api/person/exmail/new/count'),
      api.get('/api/person/exmail/list/title/passive'),
      api.get('/api/person/regist/mode'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const cnt = (count as any)?.data
    const cntText = typeof cnt === 'number' ? cnt : (cnt ?? '—')
    authMetaText.value = `新邮件 ${cntText} · 邮件标题 ${n(titles)} · 注册方式 ${(regist as any)?.data ? '已配置' : '—'}`
  } catch (e: any) {
    toast.error('加载邮件信息失败: ' + (e?.message ?? ''))
  }
}
async function loadOauthConfig() {
  try {
    // GET org/auth oauth/qywx/config + oauth/dingding/config + captchaRSAPublicKey —— 企微/钉钉 OAuth 配置/验证码公钥
    const [qywx, dingding, rsa] = await Promise.all([
      api.get('/api/organization/assemble/authentication/authentication/oauth/qywx/config'),
      api.get('/api/organization/assemble/authentication/authentication/oauth/dingding/config'),
      api.get('/api/organization/assemble/authentication/authentication/captchaRSAPublicKey'),
    ])
    const has = (r: any) => ((r as any)?.data ? '有' : '无')
    authMetaText.value = `企微配置 ${has(qywx)} · 钉钉配置 ${has(dingding)} · 验证码公钥 ${has(rsa)}`
  } catch (e: any) {
    toast.error('加载 OAuth 配置失败: ' + (e?.message ?? ''))
  }
}
async function loadAuthMeta() {
  try {
    // GET org/assemble/authentication mode + bind/list + oauth/list —— 登录方式/账户绑定/OAuth
    const [mode, binds, oauth] = await Promise.all([
      api.get('/api/organization/assemble/authentication/authentication/mode'),
      api.get('/api/organization/assemble/authentication/bind/list'),
      api.get('/api/organization/assemble/authentication/authentication/oauth/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const m = (mode as any)?.data
    authMetaText.value = `登录方式 ${typeof m === 'string' ? m : JSON.stringify(m ?? {}).slice(0, 30)} · 绑定 ${n(binds)} · OAuth ${n(oauth)}`
  } catch (e: any) {
    toast.error('加载登录方式失败: ' + (e?.message ?? ''))
  }
}
const sigManagers = ref<SigMgr[]>([])
async function loadSignatureManagers() {
  try {
    // GET /api/person/signature/manager/list —— 管理员查看全员签名（admin 门禁）
    const r: any = await api.get('/api/person/signature/manager/list')
    sigManagers.value = (r.data ?? []) as SigMgr[]
    if (sigManagers.value.length === 0) toast.success('暂无签名记录')
  } catch (e: any) {
    toast.error('查询失败（需管理员）: ' + (e?.message ?? ''))
  }
}
interface Emp { id: string; toName?: string; fromName?: string; enabled?: boolean }
const empScope = ref<'mine' | 'to'>('mine')
const empMine = ref<Emp[]>([])
const empTo = ref<Emp[]>([])
async function loadEmpower(scope: 'mine' | 'to') {
  empScope.value = scope
  try {
    // GET person/empower/list/currentperson（我发出的）| list/to（授权给我）——字面量路径便于静态提取
    const r: any =
      scope === 'mine'
        ? await api.get('/api/person/empower/list/currentperson')
        : await api.get('/api/person/empower/list/to')
    const list = (r.data ?? []) as Emp[]
    if (scope === 'mine') empMine.value = list
    else empTo.value = list
  } catch {
    if (scope === 'mine') empMine.value = []
    else empTo.value = []
  }
}
async function toggleEmpower(e: Emp, on: boolean) {
  try {
    // GET person/empower/{id}/enable | disable —— 字面量后缀便于静态提取
    if (on) await api.get(`/api/person/empower/${encodeURIComponent(e.id)}/enable`)
    else await api.get(`/api/person/empower/${encodeURIComponent(e.id)}/disable`)
    toast.success(on ? '已启用' : '已禁用')
    loadEmpower('mine')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}

onMounted(() => {
  if (!user.value) session.init()
})
loadEmpower('mine')
</script>

<style scoped>
.profile-card{display:flex;gap:16px;align-items:flex-start;padding:16px}
.form-group{display:flex;flex-direction:column;gap:6px;margin-bottom:12px}
.form-input{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 12px;color:var(--text-primary);font-size:14px}
.form-input:focus{outline:none;border-color:var(--color-primary)}
.save-btn{padding:10px 24px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600;font-size:13px}
.save-btn:disabled{opacity:.6;cursor:not-allowed}
.error-msg{color:var(--color-error);font-size:13px;padding:8px 0}
@media(max-width:768px){.profile-card{flex-direction:column;text-align:center}}
.emp-tabs{display:flex;gap:8px;margin-bottom:12px}
.emp-tab{padding:4px 12px;border-radius:12px;border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px}
.emp-tab.on{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.emp-empty{color:var(--text-muted);font-size:13px;padding:12px;text-align:center}
.emp-list{display:flex;flex-direction:column;gap:8px}
.emp-item{display:flex;align-items:center;justify-content:space-between;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.emp-info{display:flex;align-items:center;gap:10px}
.emp-name{font-size:13px;color:var(--text-primary);font-weight:500}
.emp-status{font-size:11px;padding:1px 8px;border-radius:8px}
.emp-status.on{background:var(--color-success-glow);color:var(--color-success)}
.emp-status.off{background:var(--color-warning-glow);color:var(--color-warning)}
.emp-acts{display:flex;gap:6px}
.emp-btn{padding:3px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:transparent;color:var(--text-secondary);cursor:pointer;font-size:12px}
.emp-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.save-btn.ghost{background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);margin-left:8px}
.sig-mgr-list{display:flex;flex-wrap:wrap;gap:6px;margin-top:10px}
.sig-mgr-chip{padding:2px 10px;border-radius:10px;background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-primary)}
.auth-note{margin-top:8px;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary);word-break:break-all}
</style>
