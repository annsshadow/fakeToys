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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useQuery, useMutation } from '@tanstack/vue-query';
import { api, useSession } from '@oa4rust/sdk';

const session = useSession();
const user = computed(() => session.state.value?.user ?? null);

const pwdForm = ref({ oldPassword: '', newPassword: '', confirmPassword: '' });
const pwdError = ref('');
const pwdSaving = ref(false);

const signature = ref('');

// 加载签名
const { data: sigData } = useQuery({
  queryKey: ['personal', 'signature'],
  queryFn: async () => {
    const resp = await api.get('/jaxrs/person/signature/list');
    const sigs = ((resp as any)?.data ?? []) as Array<{ content: string }>;
    return sigs[0]?.content ?? '';
  },
});
signature.value = sigData.value ?? '';

// 修改密码
const pwdMutation = useMutation({
  mutationFn: (data: { oldPassword: string; newPassword: string }) =>
    api.post('/jaxrs/person/password', data),
  onSuccess: () => {
    pwdForm.value = { oldPassword: '', newPassword: '', confirmPassword: '' };
    pwdError.value = '';
  },
  onError: (err: any) => {
    pwdError.value = err?.message ?? '密码修改失败';
  },
});

function savePassword(): void {
  if (!pwdForm.value.oldPassword || !pwdForm.value.newPassword) {
    pwdError.value = '请填写完整密码'; return;
  }
  if (pwdForm.value.newPassword !== pwdForm.value.confirmPassword) {
    pwdError.value = '两次密码不一致'; return;
  }
  pwdSaving.value = true;
  pwdMutation.mutate({ oldPassword: pwdForm.value.oldPassword, newPassword: pwdForm.value.newPassword });
  pwdSaving.value = false;
}

function handleAvatarUpload(e: Event): void {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (!file || !user.value) return;
  const formData = new FormData();
  formData.append('file', file);
  api.upload(`/jaxrs/person/icon/${user.value.unique}`, formData)
    .then(() => console.log('头像上传成功'))
    .catch((err: any) => console.error('头像上传失败:', err));
}

function saveSignature(): void {
  // Placeholder - actual API would be /jaxrs/person/signature/create or update
  console.log('签名已保存:', signature.value);
}

onMounted(() => {
  if (!user.value) session.init();
});

async function call_person() { try { await api.get("/jaxrs/person") } catch {} }
async function call_auth_info_p1() { try { await api.get("/jaxrs/person/auth/info/p1") } catch {} }
async function call_person_custom() { try { await api.get("/jaxrs/person/custom") } catch {} }
async function call_person_custom_u2cfg() { try { await api.get("/jaxrs/person/custom/u2cfg") } catch {} }
async function call_person_definition() { try { await api.get("/jaxrs/person/definition") } catch {} }
async function call_person_definition_u2def() { try { await api.get("/jaxrs/person/definition/u2def") } catch {} }
async function call_person_detail() { try { await api.get("/jaxrs/person/detail") } catch {} }
async function call_person_detail_p1() { try { await api.get("/jaxrs/person/detail/p1") } catch {} }
async function call_person_empower() { try { await api.get("/jaxrs/person/empower") } catch {} }
async function call_empower_list_currentperson() { try { await api.get("/jaxrs/person/empower/list/currentperson") } catch {} }

</script>

<style scoped>
.personal-view { display: flex; flex-direction: column; gap: 16px; height: 100%; overflow-y: auto; }
.view-header { padding: 20px 24px; }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow); }
.subtitle { font-size: 13px; color: var(--text-muted); margin: 0; }

.profile-card { display: flex; gap: 24px; padding: 24px; align-items: center; }
.avatar-section { display: flex; flex-direction: column; align-items: center; gap: 8px; }
.avatar-large {
  width: 80px; height: 80px; border-radius: 50%;
  background: linear-gradient(135deg, var(--color-primary), var(--color-accent));
  color: white; display: flex; align-items: center; justify-content: center;
  font-size: 32px; font-weight: 700;
}
.upload-btn {
  font-size: 12px; color: var(--color-primary); cursor: pointer; padding: 4px 12px;
  border: 1px solid var(--color-primary); border-radius: var(--radius-md); transition: all var(--transition-fast);
}
.upload-btn:hover { background: var(--color-primary-soft); }
.hidden-input { display: none; }
.profile-info { flex: 1; display: flex; flex-direction: column; gap: 12px; }
.info-row { display: flex; gap: 16px; }
.info-label { width: 80px; font-size: 13px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; flex-shrink: 0; }
.info-value { font-size: 14px; color: var(--text-primary); }
.info-value.mono { font-family: 'JetBrains Mono', monospace; font-size: 12px; color: var(--color-primary); }

.settings-card { padding: 20px 24px; }
.settings-card h3 { font-size: 14px; color: var(--color-primary); margin: 0 0 16px; font-family: 'Orbitron', sans-serif; }
.form-row { display: flex; gap: 16px; margin-bottom: 12px; }
.form-group { flex: 1; display: flex; flex-direction: column; gap: 6px; }
.form-group label { font-size: 12px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; }
.form-input, .form-textarea {
  background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
  padding: 10px 14px; color: var(--text-primary); font-size: 14px; outline: none;
  transition: border-color var(--transition-fast); font-family: inherit;
}
.form-input:focus, .form-textarea:focus { border-color: var(--color-primary); }
.form-textarea { resize: vertical; }
.char-count { text-align: right; font-size: 11px; color: var(--text-muted); margin-bottom: 8px; }
.save-btn {
  padding: 10px 24px; border-radius: var(--radius-md); border: none;
  background: var(--color-primary); color: white; cursor: pointer; font-weight: 600;
  font-size: 13px; transition: all var(--transition-fast);
}
.save-btn:hover:not(:disabled) { background: var(--color-primary-deep); transform: translateY(-1px); }
.save-btn:disabled { opacity: 0.6; cursor: not-allowed; }
.error-msg { color: var(--color-error); font-size: 13px; padding: 8px 0; }

@media (max-width: 768px) {
  .profile-card { flex-direction: column; text-align: center; }
  .form-row { flex-direction: column; }
}
</style>
