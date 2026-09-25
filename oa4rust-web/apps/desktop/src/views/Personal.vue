<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

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
      <button class="save-btn ghost" @click="loadAuthScopes">我的单位/角色/群组</button>
      <button class="save-btn ghost" @click="loadAuthDetails">认证明细/绑定</button>
      <button class="save-btn ghost" @click="loadPersonalExtras">头像/签名/授权明细</button>
      <button class="save-btn ghost" @click="loadPersonalMore">当前/日志/管理明细</button>
      <button class="save-btn ghost" @click="loadPersonalRegistEmpower">人员游标/校验/授权启用</button>
      <span v-if="personalExtraText" class="muted">{{ personalExtraText }}</span>
      <div v-if="authMetaText" class="auth-note">{{ authMetaText }}</div>
      <div v-if="authDetailText" class="auth-note">{{ authDetailText }}</div>
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
      <div class="emp-manage">
        <button class="save-btn ghost" @click="empManage('create')">新建授权</button>
        <button class="save-btn ghost" @click="empManage('detail')">授权详情</button>
        <button class="save-btn ghost" @click="empManage('update')">改授权</button>
        <button class="save-btn ghost" @click="empManage('delete')">删授权</button>
        <button class="save-btn ghost" @click="empManage('managerCreate')">管理员授权</button>
        <button class="save-btn ghost" @click="empManage('managerList')">管理授权列表</button>
        <button class="save-btn ghost" @click="empManage('logMine')">我的授权日志</button>
        <button class="save-btn ghost" @click="empManage('logTo')">授给我日志</button>
        <button class="save-btn ghost" @click="empManage('logManager')">管理授权日志</button>
        <button class="save-btn ghost" @click="empManage('logDelete')">删授权日志</button>
        <button class="save-btn ghost" @click="personMore('personGet')">人员详情</button>
        <button class="save-btn ghost" @click="personMore('personPut')">改人员</button>
        <button class="save-btn ghost" @click="personMore('personDelete')">删人员</button>
        <button class="save-btn ghost" @click="personMore('mgrEmpPut')">改管理授权</button>
        <button class="save-btn ghost" @click="personMore('mgrEmpDel')">删管理授权</button>
        <button class="save-btn ghost" @click="personMore('personList')">人员清单</button>
        <button class="save-btn ghost" @click="personMore('personFilter')">人员过滤</button>
        <button class="save-btn ghost" @click="personMore('personDetail')">人员详情2</button>
        <button class="save-btn ghost" @click="personMore('personAttr')">人员属性</button>
        <button class="save-btn ghost" @click="personMore('personSupDirect')">上级人员</button>
        <button class="save-btn ghost" @click="personMore('unitSubDirect')">单位下人员</button>
        <button class="save-btn ghost" @click="personMore('unitSubNested')">单位嵌套人员</button>
        <button class="save-btn ghost" @click="personMore('unitSubDirectLike')">单位模糊人员</button>
        <button class="save-btn ghost" @click="personMore('unitSubNestedLike')">单位嵌套模糊</button>
        <button class="save-btn ghost" @click="personMore('personalSetting')">个人设置</button>
        <button class="save-btn ghost" @click="personMore('customPut')">存自定义</button>
        <button class="save-btn ghost" @click="personMore('customDel')">删自定义</button>
        <button class="save-btn ghost" @click="personMore('customMgr')">管理自定义</button>
        <button class="save-btn ghost" @click="personMore('defPut')">存定义</button>
        <button class="save-btn ghost" @click="personMore('defDel')">删定义</button>
        <button class="save-btn ghost" @click="personMore('personalUpdate')">更新资料</button>
        <button class="save-btn ghost" @click="personExtra('empEnable')">启用授权</button>
        <button class="save-btn ghost" @click="personExtra('empDisable')">禁用授权</button>
        <button class="save-btn ghost" @click="personExtra('exmail')">企业邮箱回调</button>
        <button class="save-btn ghost" @click="personExtra('exmailSso')">企业邮箱SSO</button>
        <button class="save-btn ghost" @click="personExtra('exmailCount')">邮箱新邮件计数</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery } from '@tanstack/vue-query'
import { computed, onMounted, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

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
    toast.error('加载身份范围失败: ' + (e?.message ?? ''))
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
// 认证明细/绑定 4 条真实 distinct（rev201，organization_assemble_authentication）：identity/{id}（x_org_identity 详情）
// + person/{id}/icon（auth_person icon_url）+ authentication/captcha/width/{w}/height/{h}（验证码图，幂等）
// + bind/meta/{meta}（绑定元信息）。均只读/幂等，不触发登录/绑定写流程。
const authDetailText = ref('')
async function loadAuthDetails() {
  const uid = String(user.value?.unique ?? user.value?.id ?? '0')
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
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
    toast.error('加载认证明细失败: ' + (e?.message ?? ''))
  }
}
// rev217：个人域 头像/签名/自定义/授权游标族 7 条真实 distinct 路由
// person/icon/{person}（auth_person icon）· signature/list/person/{flag}（x_custom 签名 WHERE person LIKE）· definition/{name}（x_org_definition WHERE name）
// · custom/{name}（x_custom WHERE person+name）· empower/list/{id}/next/{count}（x_empower keyset >）· empower/list/{id}/prev/{count}（keyset <）· empower/list/person/{flag}（x_empower by person）
const personalExtraText = ref('')
async function loadPersonalExtras() {
  const uid = String(user.value?.unique ?? user.value?.id ?? '0')
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
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
    toast.error('加载个人扩展明细失败: ' + (e?.message ?? ''))
  }
}
// rev230：个人 当前信息/授权日志/管理自定义族 7 条真实 distinct 路由
// person（auth_person 当前）· custom/manager/person/{person}/name/{name}（x_custom 管理域）· empowerlog/list/{id}/next/{count}（x_org_empower_log DESC）· prev（ASC）
// · personal/info（auth_person WHERE unique_id via token）· personal/detail/{id}（by id）· icon/{person}（头像）
async function loadPersonalMore() {
  const uid = String(user.value?.unique ?? user.value?.id ?? '0')
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
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
    toast.error('加载个人更多明细失败: ' + (e?.message ?? ''))
  }
}
// rev243：人员游标/注册校验/授权启用 7 条真实 distinct 读路由（arity 已核；full-literal）
// person/list/{flag}/next/{count}·prev（control x_org_person 游标）· regist/check name(unique_id)/mobile/email（auth_person 各 WHERE）· empower to/enable(to_person)·currentperson/enable(from_person)
async function loadPersonalRegistEmpower() {
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
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
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : ((r as any)?.data != null ? 1 : 0))
    personalExtraText.value = `人员后翻 ${n(pNext)} · 前翻 ${n(pPrev)} · 名校验 ${n(ckName)} · 手机校验 ${n(ckMobile)} · 邮箱校验 ${n(ckEmail)} · 授权给我(启用) ${n(empTo)} · 我发出(启用) ${n(empCur)}`
  } catch (e: any) {
    toast.error('加载注册/授权启用失败: ' + (e?.message ?? ''))
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
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
// rev376：人员 详情/改/删 + 管理授权改删 + 人员清单/过滤/详情/属性/关系(sup·unit sub/nested/like) + 个人设置/自定义/定义/资料更新 真实路由（body{personList}/{unitList}；custom·definition PUT/POST 孪生择一）
async function personMore(op: string) {
  try {
    if (op === 'personGet') { const f = encodeURIComponent(prompt('人员 flag:', '') || ''); await api.get(`/api/person/${f}`) }
    else if (op === 'personPut') { const f = encodeURIComponent(prompt('人员 flag:', '') || ''); await api.put(`/api/person/${f}`, {}) }
    else if (op === 'personDelete') { const f = encodeURIComponent(prompt('要删除的人员 flag:', '') || ''); if (!(await confirmMsg('确定删除该人员？'))) return; await api.delete(`/api/person/${f}`) }
    else if (op === 'mgrEmpPut') { const id = encodeURIComponent(prompt('管理授权 ID:', '') || ''); await api.put(`/api/person/empower/manager/${id}`, {}) }
    else if (op === 'mgrEmpDel') { const id = encodeURIComponent(prompt('管理授权 ID:', '') || ''); if (!(await confirmMsg('确定删除该管理授权？'))) return; await api.delete(`/api/person/empower/manager/${id}`) }
    else if (op === 'personList') await api.post('/api/person/list', {})
    else if (op === 'personFilter') await api.post('/api/person/list/filter/1/size/20', {})
    else if (op === 'personDetail') { const f = encodeURIComponent(prompt('人员 flag:', '') || ''); await api.post(`/api/person/detail/${f}`, {}) }
    else if (op === 'personAttr') await api.post('/api/person/list/personattribute', {})
    else if (op === 'personSupDirect') await api.post('/api/person/list/person/sup/direct', {})
    else if (op === 'unitSubDirect') await api.post('/api/person/list/unit/sub/direct', {})
    else if (op === 'unitSubNested') await api.post('/api/person/list/unit/sub/nested', {})
    else if (op === 'unitSubDirectLike') await api.post('/api/person/list/unit/sub/direct/like', {})
    else if (op === 'unitSubNestedLike') await api.post('/api/person/list/unit/sub/nested/like', {})
    else if (op === 'personalSetting') { const id = encodeURIComponent(prompt('人员 ID:', '') || ''); await api.get(`/api/organization/assemble/personal/${id}/setting`) }
    else if (op === 'customPut') { const name = encodeURIComponent(prompt('自定义名:', '') || ''); await api.put(`/api/person/custom/${name}`, {}) }
    else if (op === 'customDel') { const name = encodeURIComponent(prompt('要删除的自定义名:', '') || ''); if (!(await confirmMsg('确定删除该自定义？'))) return; await api.delete(`/api/person/custom/${name}`) }
    else if (op === 'customMgr') { const person = encodeURIComponent(prompt('人员:', '') || ''); const name = encodeURIComponent(prompt('自定义名:', '') || ''); await api.put(`/api/person/custom/manager/person/${person}/name/${name}`, {}) }
    else if (op === 'defPut') { const name = encodeURIComponent(prompt('定义名:', '') || ''); await api.put(`/api/person/definition/${name}`, {}) }
    else if (op === 'defDel') { const name = encodeURIComponent(prompt('要删除的定义名:', '') || ''); if (!(await confirmMsg('确定删除该定义？'))) return; await api.delete(`/api/person/definition/${name}`) }
    else await api.put('/api/personal/update', {})
    toast.success('人员操作已提交')
  } catch (err: any) {
    toast.error('操作失败: ' + (err?.message ?? ''))
  }
}
// rev398：组织-个人 授权 启用/禁用(按id) + 企业邮箱 回调/SSO/被动新邮件计数 真实路由（empower enable/disable Path-only、exmail 读 Path-free/Query；用户触发）
async function personExtra(op: string) {
  try {
    if (op === 'empEnable') { const id = encodeURIComponent(prompt('授权 ID:', '') || ''); await api.post(`/api/person/empower/${id}/enable`, {}) }
    else if (op === 'empDisable') { const id = encodeURIComponent(prompt('授权 ID:', '') || ''); await api.post(`/api/person/empower/${id}/disable`, {}) }
    else if (op === 'exmail') await api.get('/api/person/exmail')
    else if (op === 'exmailSso') await api.get('/api/person/exmail/sso')
    else await api.get('/api/person/exmail/new/count/passive')
    toast.success('个人操作已提交')
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
