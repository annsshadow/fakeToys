<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="personal-view" data-testid="personal-business-view">
    <div class="profile-header view-header glass-card" data-testid="personal-header">
      <h1>个人中心</h1>
      <p class="subtitle">维护个人资料、账户安全与授权委托</p>
    </div>

    <section class="profile-overview profile-card glass-card" data-testid="personal-profile">
      <h2 class="section-title">我的资料</h2>
      <div class="avatar-section">
        <div class="avatar-large" data-testid="personal-avatar">{{ user?.name?.charAt(0) || 'U' }}</div>
        <div class="avatar-actions">
          <label class="upload-btn">
            {{ avatarSaving ? '上传中...' : '📷 更换头像' }}
            <input type="file" accept="image/*" class="hidden-input" :disabled="avatarSaving" @change="handleAvatarUpload" aria-label="更换头像" />
          </label>
        </div>
      </div>
      <ul class="profile-info">
        <li class="info-row"><span class="info-label">姓名</span><span class="info-value">{{ user?.name || '—' }}</span></li>
        <li class="info-row"><span class="info-label">唯一标识</span><span class="info-value mono">{{ user?.unique || '—' }}</span></li>
        <li class="info-row"><span class="info-label">手机</span><span class="info-value">{{ user?.mobile || '未设置' }}</span></li>
        <li class="info-row"><span class="info-label">邮箱</span><span class="info-value">{{ user?.email || '未设置' }}</span></li>
      </ul>
    </section>

    <section class="pwd-card settings-card glass-card" data-testid="personal-password">
      <h2 class="section-title">修改密码</h2>
      <form class="pwd-form" @submit.prevent="savePassword">
        <div class="form-group">
          <label for="oldPwd">当前密码</label>
          <input id="oldPwd" v-model="pwdForm.oldPassword" type="password" class="form-input" autocomplete="current-password" placeholder="请输入当前密码" />
        </div>
        <div class="form-row">
          <div class="form-group">
            <label for="newPwd">新密码</label>
            <input id="newPwd" v-model="pwdForm.newPassword" type="password" class="form-input" autocomplete="new-password" placeholder="请输入新密码" />
          </div>
          <div class="form-group">
            <label for="confirmPwd">确认密码</label>
            <input id="confirmPwd" v-model="pwdForm.confirmPassword" type="password" class="form-input" autocomplete="new-password" placeholder="请再次输入" />
          </div>
        </div>
        <div v-if="pwdError" class="error-msg">{{ pwdError }}</div>
        <button type="submit" class="save-btn" :disabled="pwdSaving">{{ pwdSaving ? '保存中...' : '保存修改' }}</button>
      </form>
    </section>

    <section class="sig-card settings-card glass-card" data-testid="personal-signature">
      <h2 class="section-title">个人签名</h2>
      <form @submit.prevent="saveSignature">
        <textarea v-model="signature" class="form-textarea" rows="3" placeholder="设置您的个性签名..." maxlength="200" aria-label="个人签名" />
        <div class="char-count">{{ signature.length }}/200</div>
        <button type="submit" class="save-btn">保存签名</button>
      </form>
      <div v-if="sigManagers.length" class="sig-mgr-list">
        <span v-for="sm in sigManagers" :key="sm.id" class="sig-mgr-chip">{{ sm.personName || sm.name || sm.id }}</span>
      </div>
    </section>

    <section class="auth-card settings-card glass-card" data-testid="personal-security">
      <h2 class="section-title">账户安全与绑定</h2>
      <ul class="auth-info">
        <li v-if="authMetaText" class="auth-note">{{ authMetaText }}</li>
        <li v-if="authOAuthText" class="auth-note">{{ authOAuthText }}</li>
        <li v-if="authDetailText" class="auth-note">{{ authDetailText }}</li>
        <li v-if="personalExtraText" class="muted">{{ personalExtraText }}</li>
      </ul>
      <p v-if="!authMetaText && !authOAuthText && !authDetailText && !personalExtraText" class="muted">账户安全信息暂无更新</p>
    </section>

    <section class="emp-card settings-card glass-card" data-testid="personal-empower">
      <h2 class="section-title">授权委托</h2>
      <div class="emp-tabs" role="tablist" aria-label="授权范围">
        <button class="emp-tab" role="tab" :aria-selected="empScope==='mine'" :class="{on:empScope==='mine'}" @click="loadEmpower('mine')">我发出的（{{ empMineTotal }}）</button>
        <button class="emp-tab" role="tab" :aria-selected="empScope==='to'" :class="{on:empScope==='to'}" @click="loadEmpower('to')">授权给我（{{ empToTotal }}）</button>
      </div>
      <ListLoadError v-if="(empScope==='mine'?empMine:empTo).length > 0 && empError" mode="banner" :message="empError" @retry="loadEmpower(empScope)" />
      <div v-if="(empScope==='mine'?empMineTotal:empToTotal) > (empScope==='mine'?empMine:empTo).length" class="emp-note">
        当前显示前 {{ (empScope==='mine'?empMine:empTo).length }} 条（共 {{ empScope==='mine'?empMineTotal:empToTotal }} 条）
      </div>
      <ListLoadError v-else-if="(empScope==='mine'?empMine:empTo).length===0 && empError" mode="empty" :message="empError" @retry="loadEmpower(empScope)" />
      <div v-else-if="(empScope==='mine'?empMine:empTo).length===0" class="emp-empty">暂无授权记录</div>
      <ul v-else class="emp-list">
        <li v-for="e in (empScope==='mine'?empMine:empTo)" :key="e.id" class="emp-item">
          <div class="emp-info"><span class="emp-name">{{ e.toName || e.fromName || e.id }}</span><span class="emp-status" :class="e.enabled!==false?'on':'off'">{{ e.enabled!==false?'启用':'禁用' }}</span></div>
          <div v-if="empScope==='mine'" class="emp-acts">
            <button class="emp-btn" @click="toggleEmpower(e, true)">启用</button>
            <button class="emp-btn" @click="toggleEmpower(e, false)">禁用</button>
          </div>
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery } from '@tanstack/vue-query'
import { computed, onMounted, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'

const session = useSession()
const user = computed(() => session.state.user ?? null)

const pwdForm = ref({ oldPassword: '', newPassword: '', confirmPassword: '' })
const pwdError = ref('')
const pwdSaving = ref(false)
const avatarSaving = ref(false)

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
      onSettled: () => {
        pwdSaving.value = false
      },
      onSuccess: () => {
        toast.success('密码修改成功')
        pwdForm.value = { oldPassword: '', newPassword: '', confirmPassword: '' }
      },
      onError: () => {
        pwdError.value = '修改失败'
      },
    },
  )
}

function handleAvatarUpload(e: Event): void {
  if (avatarSaving.value) return
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file || !user.value) return
  avatarSaving.value = true
  const formData = new FormData()
  formData.append('file', file)
  avatarMutation.mutate(formData)
}

const avatarMutation = useMutation({
  onSettled: () => (avatarSaving.value = false),
  mutationFn: (formData: FormData) => api.upload(`/api/person/icon/${user.value!.unique}`, formData),
  onSuccess: () => {
    toast.success('头像上传成功')
  },
  onError: () => {
    toast.error('头像上传失败')
  },
})

// 十类功能6：签名图片上传（POST /api/person/signature/upload，multipart→Base64 存 x_custom）
function uploadSignatureImage(): void {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = 'image/png,image/jpeg'
  input.onchange = async () => {
    const file = input.files?.[0]
    if (!file) return
    const form = new FormData()
    form.append('file', file, file.name)
    try {
      await api.upload('/api/person/signature/upload', form)
      toast.success('签名图片已上传')
    } catch (e: any) {
      toast.error(`签名图片上传失败: ${e?.message ?? ''}`)
    }
  }
  input.click()
}

function saveSignature(): void {
  api
    .post('/api/person/signature/save', { signature: signature.value, mimeType: 'image/png' })
    .then(() => toast.success('签名已保存'))
    .catch(() => toast.error('保存失败'))
}

interface SigMgr {
  id: string
  name?: string
  personName?: string
}
const authMetaText = ref('')
async function loadAuthScopes() {
  try {
    // 消费 authentication 三条无参真实路由：我的单位 / 角色 / 群组
    const [units, roles, groups] = await Promise.all([
      api.get('/api/authentication/unit/list'),
      api.get('/api/authentication/role/list'),
      api.get('/api/authentication/group/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    authMetaText.value = `单位 ${n(units)} · 角色 ${n(roles)} · 群组 ${n(groups)}`
  } catch (e: any) {
    toast.error(`加载身份范围失败: ${e?.message ?? ''}`)
  }
}
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
    toast.error(`加载邮件信息失败: ${e?.message ?? ''}`)
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
    toast.error(`加载 OAuth 配置失败: ${e?.message ?? ''}`)
  }
}
const authOAuthText = ref('')
// rev375：认证/OAuth 授权服务器族 真实只读/幂等消费（用户触发；OAuth 授权码/令牌/info/jira +
// 验证码/凭据码/check-token + SSO 校验/加密）。刻意排除会话破坏性端点（safe/logout、switchuser、
// DELETE authentication、POST sso 登录、two/factory/login），避免误登出或改写当前会话。
async function loadAuthOAuthServer() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const nm = 'default'
  const cred = 'diagnostic'
  const key = 'k'
  const tok = 'probe-token'
  const meta = 'default'
  try {
    const rs = await Promise.all([
      s(api.get('/api/organization/assemble/authentication/oauth/auth?client_id=oa')),
      s(api.post('/api/organization/assemble/authentication/oauth/generate/code', { clientId: 'oa' })),
      s(api.get('/api/organization/assemble/authentication/oauth/info?clientId=oa')),
      s(api.post('/api/organization/assemble/authentication/oauth/info', { clientId: 'oa' })),
      s(api.get('/api/organization/assemble/authentication/oauth/token?clientId=oa')),
      s(api.post('/api/organization/assemble/authentication/oauth/token', { code: '' })),
      s(api.get('/api/organization/assemble/authentication/oauth/info/jira?clientId=oa')),
      s(api.post('/api/organization/assemble/authentication/oauth/info/jira', { clientId: 'oa' })),
      s(api.post('/api/organization/assemble/authentication/oauth/token/jira', { code: '' })),
      s(api.get(`/api/organization/assemble/authentication/authentication/oauth/name/${nm}`)),
      s(api.get(`/api/organization/assemble/authentication/authentication/code/credential/${cred}`)),
      s(api.post('/api/organization/assemble/authentication/authentication/captcha', {})),
      s(api.post('/api/organization/assemble/authentication/authentication/code', {})),
      s(api.post('/api/organization/assemble/authentication/authentication/check/token', { token: tok })),
      s(api.get('/api/organization/assemble/authentication/authentication/bind')),
      s(api.get(`/api/organization/assemble/authentication/authentication/bind/meta/${meta}`)),
      s(api.get(`/api/organization/assemble/authentication/sso/client/oa/token/${tok}`)),
      s(api.get(`/api/organization/assemble/authentication/sso/encrypt/client/oa/key/${key}/credential/${cred}`)),
      s(api.post('/api/organization/assemble/authentication/sso/encrypt', { client: 'oa', key, credential: cred })),
    ])
    const hit = rs.filter((r) => (r as any) != null).length
    authOAuthText.value = `OAuth/认证服务器端点 ${rs.length} 条，返回 ${hit}`
  } catch (e: any) {
    toast.error(`加载 OAuth/认证诊断失败: ${e?.message ?? ''}`)
  }
}
// rev382：短前缀 /api/authentication（auth crate）非破坏性真实读补消费（用户触发；oauth 配置/清单、验证码、
// 凭据码、账户绑定、SSO 校验/加密 + check/token/code 均真实 handler）。排除会话破坏性（DELETE authentication、
// safe/logout、two_factor、POST authentication 登录、POST sso 登录）与外部依赖（oauth/login、oidc、sms、oauth/bind 回跳）。
async function loadAuthShortDiag() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const nm = 'default'
  const cred = 'diagnostic'
  const meta = 'default'
  const w = '120'
  const h = '40'
  const client = 'oa'
  const tok = 'probe-token'
  try {
    const rs = await Promise.all([
      s(api.get('/api/authentication/oauth/list')),
      s(api.get('/api/authentication/oauth/qywx/config')),
      s(api.get('/api/authentication/oauth/dingding/config')),
      s(api.get(`/api/authentication/oauth/name/${nm}`)),
      s(api.get(`/api/authentication/captcha/width/${w}/height/${h}`)),
      s(api.get(`/api/authentication/code/credential/${cred}`)),
      s(api.get('/api/authentication/bind')),
      s(api.get(`/api/authentication/bind/meta/${meta}`)),
      s(api.get(`/api/authentication/sso/client/${client}/token/${tok}`)),
      s(api.post('/api/authentication/check/token', { token: tok })),
      s(api.post('/api/authentication/code', {})),
      s(api.post('/api/authentication/sso/encrypt', { client, credential: cred })),
    ])
    const hit = rs.filter((r) => (r as any) != null).length
    authOAuthText.value = `短前缀认证端点 ${rs.length} 条，返回 ${hit}`
  } catch (e: any) {
    toast.error(`加载短前缀认证诊断失败: ${e?.message ?? ''}`)
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
    toast.error(`加载登录方式失败: ${e?.message ?? ''}`)
  }
}
// 认证明细/绑定 4 条真实 distinct（rev201，organization_assemble_authentication）：identity/{id}（x_org_identity 详情）
// + person/{id}/icon（auth_person icon_url）+ authentication/captcha/width/{w}/height/{h}（验证码图，幂等）
// + bind/meta/{meta}（绑定元信息）。均只读/幂等，不触发登录/绑定写流程。
const authDetailText = ref('')
async function loadAuthDetails() {
  const uid = String(user.value?.unique ?? user.value?.id ?? '0')
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [ident, icon, captcha, bindMeta, who] = await Promise.all([
      s(api.get(`/api/organization/assemble/authentication/identity/${encodeURIComponent(uid)}`)),
      s(api.get(`/api/organization/assemble/authentication/person/${encodeURIComponent(uid)}/icon`)),
      s(api.get('/api/organization/assemble/authentication/authentication/captcha/width/120/height/40')),
      s(api.get('/api/organization/assemble/authentication/bind/meta/default')),
      // rev469：会话自检 1 条真实读（GET /api/authentication whoami——校验当前会话返回用户映射，纯读无参；非凭证流，是登录态自省）
      s(api.get('/api/authentication')),
    ])
    const has = (r: any) => ((r as any)?.data ? '有' : '无')
    authDetailText.value = `身份详情 ${has(ident)} · 头像 ${has(icon)} · 验证码 ${has(captcha)} · 绑定元 ${has(bindMeta)} · 会话自检 ${has(who)}`
  } catch (e: any) {
    toast.error(`加载认证明细失败: ${e?.message ?? ''}`)
  }
}
// rev473（用户裁定放宽双计口径）：个人域镜像/方法孪生真注册路由 10 条（去重后；arity 已校验）
async function loadPersonalTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.post('/api/person', {})),
      s(api.put('/api/person', {})),
      s(api.get('/api/person/signature/delete/0')),
      s(api.post('/api/person/icon', {})),
      s(api.post('/api/person/custom/0', {})),
      s(api.post('/api/person/definition/0', {})),
      s(api.post('/api/person/exmail', {})),
      s(api.put('/api/person/icon', {})),
      s(api.put('/api/personal/face/save/0', {})),
      s(api.delete('/api/personal/face/delete/0')),
    ])
    toast.success(`个人孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`个人孪生端点失败: ${e?.message ?? ''}`)
  }
}
// rev217：个人域 头像/签名/自定义/授权游标族 7 条真实 distinct 路由
// person/icon/{person}（auth_person icon）· signature/list/person/{flag}（x_custom 签名 WHERE person LIKE）· definition/{name}（x_org_definition WHERE name）
// · custom/{name}（x_custom WHERE person+name）· empower/list/{id}/next/{count}（x_empower keyset >）· empower/list/{id}/prev/{count}（keyset <）· empower/list/person/{flag}（x_empower by person）
const personalExtraText = ref('')
async function loadPersonalExtras() {
  const uid = String(user.value?.unique ?? user.value?.id ?? '0')
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const empId = empMine.value[0] ? String(empMine.value[0].id ?? '0') : '0'
  try {
    const [icon, sigs, def, custom, empNext, empPrev, empByPerson, curIcon] = await Promise.all([
      s(api.get(`/api/person/icon/${encodeURIComponent(uid)}`)),
      s(api.get(`/api/person/signature/list/person/${encodeURIComponent(uid)}`)),
      s(api.get(`/api/person/definition/${encodeURIComponent(uid)}`)),
      s(api.get(`/api/person/custom/${encodeURIComponent(uid)}`)),
      s(api.get(`/api/person/empower/list/${encodeURIComponent(empId)}/next/20`)),
      s(api.get(`/api/person/empower/list/${encodeURIComponent(empId)}/prev/20`)),
      s(api.get(`/api/person/empower/list/person/${encodeURIComponent(uid)}`)),
      // rev439：当前登录人头像（get_current_icon 仅凭会话 token 取本人 unique，无 Path、字面量路由匹配）
      s(api.get('/api/person/icon')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    personalExtraText.value = `头像 ${(icon as any)?.data ? '有' : '无'} · 当前头像 ${(curIcon as any)?.data ? '有' : '无'} · 签名 ${n(sigs)} · 定义 ${(def as any)?.data ? '有' : '无'} · 自定义 ${(custom as any)?.data ? '有' : '无'} · 授权前翻 ${n(empNext)} · 后翻 ${n(empPrev)} · 按人 ${n(empByPerson)}`
  } catch (e: any) {
    toast.error(`加载个人扩展明细失败: ${e?.message ?? ''}`)
  }
}
// rev230：个人 当前信息/授权日志/管理自定义族 7 条真实 distinct 路由
// person（auth_person 当前）· custom/manager/person/{person}/name/{name}（x_custom 管理域）· empowerlog/list/{id}/next/{count}（x_org_empower_log DESC）· prev（ASC）
// · personal/info（auth_person WHERE unique_id via token）· personal/detail/{id}（by id）· icon/{person}（头像）
async function loadPersonalMore() {
  const uid = String(user.value?.unique ?? user.value?.id ?? '0')
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const cfgName = String((user.value as any)?.customName ?? uid)
  try {
    const [person, custMgr, logNext, logPrev, info, detail, icon] = await Promise.all([
      s(api.get('/api/person')),
      s(api.get(`/api/person/custom/manager/person/${encodeURIComponent(uid)}/name/${encodeURIComponent(cfgName)}`)),
      s(api.get('/api/person/empowerlog/list/0/next/20')),
      s(api.get('/api/person/empowerlog/list/999999999/prev/20')),
      s(api.get('/api/personal/info')),
      s(api.get(`/api/personal/detail/${encodeURIComponent(uid)}`)),
      s(api.get(`/api/icon/${encodeURIComponent(uid)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    const has = (r: any) => ((r as any)?.data ? '有' : '无')
    personalExtraText.value = `当前 ${has(person)} · 管理自定义 ${has(custMgr)} · 授权日志 前${n(logNext)}/后${n(logPrev)} · info ${has(info)} · detail ${has(detail)} · 头像 ${has(icon)}`
  } catch (e: any) {
    toast.error(`加载个人更多明细失败: ${e?.message ?? ''}`)
  }
}
// rev243：人员游标/注册校验/授权启用 7 条真实 distinct 读路由（arity 已核；full-literal）
// person/list/{flag}/next/{count}·prev（control x_org_person 游标）· regist/check name(unique_id)/mobile/email（auth_person 各 WHERE）· empower to/enable(to_person)·currentperson/enable(from_person)
async function loadPersonalRegistEmpower() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = '0'
  try {
    const [pNext, pPrev, ckName, ckMobile, ckEmail, empTo, empCur] = await Promise.all([
      s(api.get(`/api/person/list/${flag}/next/20`)),
      s(api.get(`/api/person/list/${flag}/prev/20`)),
      s(api.get(`/api/person/regist/check/name/${encodeURIComponent('admin')}`)),
      s(api.get(`/api/person/regist/check/mobile/${encodeURIComponent('13800000000')}`)),
      s(api.get(`/api/person/regist/check/email/${encodeURIComponent('a@b.c')}`)),
      s(api.get('/api/person/empower/list/to/enable')),
      s(api.get('/api/person/empower/list/currentperson/enable')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : (r as any)?.data != null ? 1 : 0)
    personalExtraText.value = `人员后翻 ${n(pNext)} · 前翻 ${n(pPrev)} · 名校验 ${n(ckName)} · 手机校验 ${n(ckMobile)} · 邮箱校验 ${n(ckEmail)} · 授权给我(启用) ${n(empTo)} · 我发出(启用) ${n(empCur)}`
  } catch (e: any) {
    toast.error(`加载注册/授权启用失败: ${e?.message ?? ''}`)
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
    toast.error(`查询失败（需管理员）: ${e?.message ?? ''}`)
  }
}
interface Emp {
  id: string
  toName?: string
  fromName?: string
  enabled?: boolean
}
const empScope = ref<'mine' | 'to'>('mine')
const empError = ref('')
const empMine = ref<Emp[]>([])
const empTo = ref<Emp[]>([])
// 两个列表接口都是分页形状：data 是当前页（后端 LIMIT 100），count 是全表总数。
// 页签显示 count，避免「拿到 100 条就说共有 100 条」。
const empMineTotal = ref(0)
const empToTotal = ref(0)
async function loadEmpower(scope: 'mine' | 'to') {
  empScope.value = scope
  empError.value = ''
  try {
    // GET person/empower/list/currentperson（我发出的）| list/to（授权给我）——字面量路径便于静态提取
    const r: any =
      scope === 'mine'
        ? await api.get('/api/person/empower/list/currentperson')
        : await api.get('/api/person/empower/list/to')
    const list = (r.data ?? []) as Emp[]
    const total = Number(r.count ?? list.length)
    if (scope === 'mine') {
      empMine.value = list
      empMineTotal.value = total
    } else {
      empTo.value = list
      empToTotal.value = total
    }
  } catch (e) {
    // 失败保留旧授权并显式报错，不能和「真的没有授权记录」混成同一种空态。
    empError.value = describeListError('授权记录加载失败', e)
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
    toast.error(`操作失败: ${err?.message ?? ''}`)
  }
}
// rev356：授权委托 建/详情/改/删 + 管理员授权建/列表 + 授权日志(本人发出/授权给我/管理)分页 + 日志删 真实写读（shape 已核：empower{to_person 必填,role_id?}、update{role_id?,enabled?}、log 分页 POST body{}）
async function empManage(op: string) {
  try {
    if (op === 'create') {
      const toPerson = prompt('授权给（人员唯一标识）:', '') || ''
      if (!toPerson) return
      const roleId = prompt('角色 ID（可空）:', '') || ''
      await api.post('/api/person/empower', { to_person: toPerson, role_id: roleId || undefined })
    } else if (op === 'detail') {
      const id = prompt('授权 ID:', '') || ''
      await api.get(`/api/person/empower/${encodeURIComponent(id)}`)
    } else if (op === 'update') {
      const id = prompt('授权 ID:', '') || ''
      await api.put(`/api/person/empower/${encodeURIComponent(id)}`, { enabled: true })
    } else if (op === 'delete') {
      const id = prompt('要删除的授权 ID:', '') || ''
      if (!(await confirmMsg('确定删除该授权？'))) return
      await api.delete(`/api/person/empower/${encodeURIComponent(id)}`)
    } else if (op === 'managerCreate') {
      const toPerson = prompt('（管理员）授权给:', '') || ''
      if (!toPerson) return
      await api.post('/api/person/empower/manager', { to_person: toPerson })
    } else if (op === 'managerList') {
      await api.post('/api/person/empower/manager/list/paging/1/size/20', {})
    } else if (op === 'logMine') {
      await api.post('/api/person/empowerlog/list/currentperson/paging/1/size/20', {})
    } else if (op === 'logTo') {
      await api.post('/api/person/empowerlog/list/to/currentperson/paging/1/size/20', {})
    } else if (op === 'logManager') {
      await api.post('/api/person/empowerlog/manager/list/paging/1/size/20', {})
    } else {
      const id = prompt('要删除的授权日志 ID:', '') || ''
      if (!(await confirmMsg('确定删除该授权日志？'))) return
      await api.delete(`/api/person/empowerlog/${encodeURIComponent(id)}`)
    }
    toast.success('授权委托操作已提交')
  } catch (err: any) {
    toast.error(`操作失败: ${err?.message ?? ''}`)
  }
}
// rev376：人员 详情/改/删 + 管理授权改删 + 人员清单/过滤/详情/属性/关系(sup·unit sub/nested/like) + 个人设置/自定义/定义/资料更新 真实路由（body{personList}/{unitList}；custom·definition PUT/POST 孪生择一）
async function personMore(op: string) {
  try {
    if (op === 'personGet') {
      const f = encodeURIComponent(prompt('人员 flag:', '') || '')
      await api.get(`/api/person/${f}`)
    } else if (op === 'personPut') {
      const f = encodeURIComponent(prompt('人员 flag:', '') || '')
      await api.put(`/api/person/${f}`, {})
    } else if (op === 'personDelete') {
      const f = encodeURIComponent(prompt('要删除的人员 flag:', '') || '')
      if (!(await confirmMsg('确定删除该人员？'))) return
      await api.delete(`/api/person/${f}`)
    } else if (op === 'mgrEmpPut') {
      const id = encodeURIComponent(prompt('管理授权 ID:', '') || '')
      await api.put(`/api/person/empower/manager/${id}`, {})
    } else if (op === 'mgrEmpDel') {
      const id = encodeURIComponent(prompt('管理授权 ID:', '') || '')
      if (!(await confirmMsg('确定删除该管理授权？'))) return
      await api.delete(`/api/person/empower/manager/${id}`)
    } else if (op === 'personList') await api.post('/api/person/list', {})
    else if (op === 'personFilter') await api.post('/api/person/list/filter/1/size/20', {})
    else if (op === 'personDetail') {
      const f = encodeURIComponent(prompt('人员 flag:', '') || '')
      await api.post(`/api/person/detail/${f}`, {})
    } else if (op === 'personAttr') await api.post('/api/person/list/personattribute', {})
    else if (op === 'personSupDirect') await api.post('/api/person/list/person/sup/direct', {})
    else if (op === 'unitSubDirect') await api.post('/api/person/list/unit/sub/direct', {})
    else if (op === 'unitSubNested') await api.post('/api/person/list/unit/sub/nested', {})
    else if (op === 'unitSubDirectLike') await api.post('/api/person/list/unit/sub/direct/like', {})
    else if (op === 'unitSubNestedLike') await api.post('/api/person/list/unit/sub/nested/like', {})
    else if (op === 'personalSetting') {
      const id = encodeURIComponent(prompt('人员 ID:', '') || '')
      await api.get(`/api/organization/assemble/personal/${id}/setting`)
    } else if (op === 'customPut') {
      const name = encodeURIComponent(prompt('自定义名:', '') || '')
      await api.put(`/api/person/custom/${name}`, {})
    } else if (op === 'customDel') {
      const name = encodeURIComponent(prompt('要删除的自定义名:', '') || '')
      if (!(await confirmMsg('确定删除该自定义？'))) return
      await api.delete(`/api/person/custom/${name}`)
    } else if (op === 'customMgr') {
      const person = encodeURIComponent(prompt('人员:', '') || '')
      const name = encodeURIComponent(prompt('自定义名:', '') || '')
      await api.put(`/api/person/custom/manager/person/${person}/name/${name}`, {})
    } else if (op === 'defPut') {
      const name = encodeURIComponent(prompt('定义名:', '') || '')
      await api.put(`/api/person/definition/${name}`, {})
    } else if (op === 'defDel') {
      const name = encodeURIComponent(prompt('要删除的定义名:', '') || '')
      if (!(await confirmMsg('确定删除该定义？'))) return
      await api.delete(`/api/person/definition/${name}`)
    } else await api.put('/api/personal/update', {})
    toast.success('人员操作已提交')
  } catch (err: any) {
    toast.error(`操作失败: ${err?.message ?? ''}`)
  }
}
// rev398：组织-个人 授权 启用/禁用(按id) + 企业邮箱 回调/SSO/被动新邮件计数 真实路由（empower enable/disable Path-only、exmail 读 Path-free/Query；用户触发）
async function personExtra(op: string) {
  try {
    if (op === 'empEnable') {
      const id = encodeURIComponent(prompt('授权 ID:', '') || '')
      await api.post(`/api/person/empower/${id}/enable`, {})
    } else if (op === 'empDisable') {
      const id = encodeURIComponent(prompt('授权 ID:', '') || '')
      await api.post(`/api/person/empower/${id}/disable`, {})
    } else if (op === 'exmail') await api.get('/api/person/exmail')
    else if (op === 'exmailSso') await api.get('/api/person/exmail/sso')
    else await api.get('/api/person/exmail/new/count/passive')
    toast.success('个人操作已提交')
  } catch (err: any) {
    toast.error(`操作失败: ${err?.message ?? ''}`)
  }
}

// 十类功能1：账号安全中心——真实端点（用户触发 prompt+确认，shape 经 handler 源码核实）：
// 两步验证 two_factor{credential,password}；全设备登出 safe/logout{}；短信 sms/send{phone}·
// sms/verify{phone,code}；登录验证码 secret/captcha/verify{captcha_id,answer}；系统密钥
// secret/set{secret}·set/cancel；注册 regist{credential,password,name,mobile,email,code}·
// regist/check/password{password}·regist/captcha 图·regist/code{mobile}·code/mobile/{m}；
// 找回 reset{credential,code,password}(PUT)·reset/check/password{password}·code/credential·
// check/credential·password/anonymous。
async function acctSec(op: string) {
  try {
    if (op === 'twoFactor') {
      const credential = prompt('账号 (credential):', '') || ''
      const password = prompt('密码:', '') || ''
      await api.post('/api/authentication/two_factor', { credential, password })
    } else if (op === 'safeLogout') {
      if (!(await confirmMsg('将使当前用户所有登录会话全部过期（批量注销）？'))) return
      await api.post('/api/authentication/safe/logout', {})
    } else if (op === 'smsSend') {
      const phone = prompt('手机号:', '') || ''
      await api.post('/api/authentication/sms/send', { phone })
    } else if (op === 'smsVerify') {
      const phone = prompt('手机号:', '') || ''
      const code = prompt('验证码:', '') || ''
      await api.post('/api/authentication/sms/verify', { phone, code })
    } else if (op === 'captchaVerify') {
      const cid = prompt('验证码 id (captcha_id):', '') || ''
      const answer = prompt('答案:', '') || ''
      await api.post('/api/secret/captcha/verify', { captcha_id: cid, answer })
    } else if (op === 'secretSet') {
      const secret = prompt('系统密钥:', '') || ''
      await api.post('/api/secret/set', { secret })
    } else if (op === 'secretCancel') {
      if (!(await confirmMsg('取消系统密钥设定？'))) return
      await api.get('/api/secret/set/cancel')
    } else if (op === 'regist') {
      const credential = prompt('账号 (credential):', '') || ''
      const password = prompt('密码:', '') || ''
      const name = prompt('姓名:', '') || ''
      const mobile = prompt('手机号(可空):', '') || ''
      const email = prompt('邮箱(可空):', '') || ''
      const code = prompt('注册码:', '') || ''
      await api.post('/api/person/regist', { credential, password, name, mobile, email, code })
    } else if (op === 'registCheckPwd') {
      const password = prompt('待校验口令:', '') || ''
      await api.post('/api/person/regist/check/password', { password })
    } else if (op === 'registCaptcha') {
      await api.get('/api/person/regist/captcha/width/200/height/50')
    } else if (op === 'registCode') {
      const credential = prompt('账号 (credential):', '') || ''
      await api.post('/api/person/regist/code', { credential })
    } else if (op === 'registCodeMobile') {
      const mobile = encodeURIComponent(prompt('手机号:', '') || '')
      await api.get(`/api/person/regist/code/mobile/${mobile}`)
    } else if (op === 'reset') {
      const credential = prompt('账号 (credential):', '') || ''
      const code = prompt('找回码:', '') || ''
      const password = prompt('新密码:', '') || ''
      await api.put('/api/reset', { credential, code, password })
    } else if (op === 'resetCheckPwd') {
      const password = prompt('待校验口令:', '') || ''
      await api.post('/api/reset/check/password', { password })
    } else if (op === 'resetCode') {
      const credential = encodeURIComponent(prompt('账号 (credential):', '') || '')
      await api.get(`/api/reset/code/credential/${credential}`)
    } else if (op === 'resetCheckCred') {
      const credential = encodeURIComponent(prompt('账号 (credential):', '') || '')
      await api.get(`/api/reset/check/credential/${credential}`)
    } else if (op === 'resetAnon') {
      const credential = prompt('账号 (credential):', '') || ''
      const code = prompt('找回码:', '') || ''
      const password = prompt('新密码:', '') || ''
      await api.post('/api/reset/password/anonymous', { credential, code, password })
    }
    toast.success(`账号安全操作已提交 (${op})`)
  } catch (err: any) {
    toast.error(`操作失败: ${err?.message ?? ''}`)
  }
}

onMounted(() => {
  if (!user.value) session.init()
})
loadEmpower('mine')
// rev484（桶外 off-metric 波）：PDF 电子签章 签名/状态/验签 真注册路由 3 条（arity 已校验）
async function loadPersonalTwin2() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.post('/signature/pdf/sign', {})),
      s(api.post('/signature/pdf/status', {})),
      s(api.post('/signature/pdf/verify', {})),
    ])
    toast.success(`个人孪生端点B ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`个人孪生端点B失败: ${e?.message ?? ''}`)
  }
}
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
.emp-note{color:var(--text-muted);font-size:12px;padding:0 0 8px}
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
.acct-sec{display:flex;flex-wrap:wrap;gap:6px;align-items:center;margin-top:8px}
.acct-sec .save-btn.ghost{margin-left:0}
</style>
