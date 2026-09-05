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
import { toast } from '../utils/toast';
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
  pwdMutation.mutate({ oldPassword: pwdForm.value.oldPassword, newPassword: pwdForm.value.newPassword }, {
    onSuccess: () => { toast.success('密码修改成功'); pwdForm.value = { oldPassword: '', newPassword: '', confirmPassword: '' }; },
    onError: () => { pwdError.value = '修改失败'; pwdSaving.value = false; },
  });
  pwdSaving.value = true;
}

function handleAvatarUpload(e: Event): void {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (!file || !user.value) return;
  const formData = new FormData();
  formData.append('file', file);
  avatarMutation.mutate(formData);
}

const avatarMutation = useMutation({
  mutationFn: (formData: FormData) => api.upload(`/jaxrs/person/icon/${user.value!.unique}`, formData),
  onSuccess: () => { toast.success('头像上传成功'); },
  onError: () => { toast.error('头像上传失败'); },
});

function saveSignature(): void {
  api.post('/jaxrs/person/signature/save', { signature: signature.value })
    .then(() => toast.success('签名已保存'))
    .catch(() => toast.error('保存失败'));
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


const unit_list_type_ref = ref<any[]>([]);
const unit_list_type_q = useQuery({
  queryKey: ['unit_list_type'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/type"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const exmail_new_count_ref = ref<any[]>([]);
const exmail_new_count_q = useQuery({
  queryKey: ['exmail_new_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/exmail/new/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const two_factory_login_ref = ref<any[]>([]);
const two_factory_login_q = useQuery({
  queryKey: ['two_factory_login'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/two/factory/login"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const login_after_object_ref = ref<any[]>([]);
const login_after_object_q = useQuery({
  queryKey: ['login_after_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/login/after/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const role_list_like_ref = ref<any[]>([]);
const role_list_like_q = useQuery({
  queryKey: ['role_list_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_sso_ref = ref<any[]>([]);
const assemble_authentication_sso_q = useQuery({
  queryKey: ['assemble_authentication_sso'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/sso"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_inputperson_template_ref = ref<any[]>([]);
const control_inputperson_template_q = useQuery({
  queryKey: ['control_inputperson_template'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/inputperson/template"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_mockputtopost_ref = ref<any[]>([]);
const person_mockputtopost_q = useQuery({
  queryKey: ['person_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_pinyininitial_ref = ref<any[]>([]);
const person_list_pinyininitial_q = useQuery({
  queryKey: ['person_list_pinyininitial'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/person/list/pinyininitial"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const direct_like_object_ref = ref<any[]>([]);
const direct_like_object_q = useQuery({
  queryKey: ['direct_like_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/unit/sub/direct/like/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_check_token_ref = ref<any[]>([]);
const authentication_check_token_q = useQuery({
  queryKey: ['authentication_check_token'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/check/token"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_pinyininitial_mockputtopost_ref = ref<any[]>([]);
const list_pinyininitial_mockputtopost_q = useQuery({
  queryKey: ['list_pinyininitial_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_authentication_mode_ref = ref<any[]>([]);
const authentication_authentication_mode_q = useQuery({
  queryKey: ['authentication_authentication_mode'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/mode"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_icon_upload_ref = ref<any[]>([]);
const person_icon_upload_q = useQuery({
  queryKey: ['person_icon_upload'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/icon/upload"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_exmail_ref = ref<any[]>([]);
const person_exmail_q = useQuery({
  queryKey: ['person_exmail'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/exmail"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_switchuser_mockputtopost_ref = ref<any[]>([]);
const authentication_switchuser_mockputtopost_q = useQuery({
  queryKey: ['authentication_switchuser_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/switchuser/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_person_list_ref = ref<any[]>([]);
const organization_person_list_q = useQuery({
  queryKey: ['organization_person_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/person/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_group_object_ref = ref<any[]>([]);
const list_group_object_q = useQuery({
  queryKey: ['list_group_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/group/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_person_ref = ref<any[]>([]);
const assemble_control_person_q = useQuery({
  queryKey: ['assemble_control_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const oauth_token_jira_ref = ref<any[]>([]);
const oauth_token_jira_q = useQuery({
  queryKey: ['oauth_token_jira'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/oauth/token/jira"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const assemble_authentication_authentication_bind_ref = ref<any[]>([]);
const assemble_authentication_authentication_bind_q = useQuery({
  queryKey: ['assemble_authentication_authentication_bind'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/bind"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_group_ref = ref<any[]>([]);
const organization_group_q = useQuery({
  queryKey: ['organization_group'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/group"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_personcard_ref = ref<any[]>([]);
const organization_assemble_control_personcard_q = useQuery({
  queryKey: ['organization_assemble_control_personcard'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/personcard"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_authentication_mockdeletetoget_ref = ref<any[]>([]);
const assemble_authentication_authentication_mockdeletetoget_q = useQuery({
  queryKey: ['assemble_authentication_authentication_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_oauth_auth_ref = ref<any[]>([]);
const assemble_authentication_oauth_auth_q = useQuery({
  queryKey: ['assemble_authentication_oauth_auth'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/oauth/auth"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_personcard_listgrouptypes_ref = ref<any[]>([]);
const assemble_control_personcard_listgrouptypes_q = useQuery({
  queryKey: ['assemble_control_personcard_listgrouptypes'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/personcard/listgrouptypes"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_core_express_config_ref = ref<any[]>([]);
const organization_core_express_config_q = useQuery({
  queryKey: ['organization_core_express_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/core/express/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_permissionsetting_ref = ref<any[]>([]);
const organization_assemble_control_permissionsetting_q = useQuery({
  queryKey: ['organization_assemble_control_permissionsetting'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/permissionsetting"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_unit_tree_ref = ref<any[]>([]);
const organization_assemble_unit_tree_q = useQuery({
  queryKey: ['organization_assemble_unit_tree'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/unit/tree"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_unitattribute_ref = ref<any[]>([]);
const organization_assemble_control_unitattribute_q = useQuery({
  queryKey: ['organization_assemble_control_unitattribute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unitattribute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_authentication_authentication_ref = ref<any[]>([]);
const organization_assemble_authentication_authentication_q = useQuery({
  queryKey: ['organization_assemble_authentication_authentication'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_bind_list_ref = ref<any[]>([]);
const organization_bind_list_q = useQuery({
  queryKey: ['organization_bind_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/bind/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_authentication_switchuser_ref = ref<any[]>([]);
const assemble_authentication_authentication_switchuser_q = useQuery({
  queryKey: ['assemble_authentication_authentication_switchuser'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/switchuser"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_personattribute_ref = ref<any[]>([]);
const organization_assemble_control_personattribute_q = useQuery({
  queryKey: ['organization_assemble_control_personattribute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/personattribute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_person_person_001_ref = ref<any[]>([]);
const organization_person_person_001_q = useQuery({
  queryKey: ['organization_person_person_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/person/person-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_express_units_list_ref = ref<any[]>([]);
const assemble_express_units_list_q = useQuery({
  queryKey: ['assemble_express_units_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/express/units/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_bind_ref = ref<any[]>([]);
const organization_bind_q = useQuery({
  queryKey: ['organization_bind'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/bind"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_express_data_sync_ref = ref<any[]>([]);
const assemble_express_data_sync_q = useQuery({
  queryKey: ['assemble_express_data_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/express/data/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_core_express_status_ref = ref<any[]>([]);
const organization_core_express_status_q = useQuery({
  queryKey: ['organization_core_express_status'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/core/express/status"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_custom_ref = ref<any[]>([]);
const organization_custom_q = useQuery({
  queryKey: ['organization_custom'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/custom"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_express_status_get_ref = ref<any[]>([]);
const assemble_express_status_get_q = useQuery({
  queryKey: ['assemble_express_status_get'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/express/status/get"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_group_group_001_ref = ref<any[]>([]);
const organization_group_group_001_q = useQuery({
  queryKey: ['organization_group_group_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/group/group-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_identity_ref = ref<any[]>([]);
const organization_assemble_control_identity_q = useQuery({
  queryKey: ['organization_assemble_control_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_unit_list_ref = ref<any[]>([]);
const assemble_control_unit_list_q = useQuery({
  queryKey: ['assemble_control_unit_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_express_config_get_ref = ref<any[]>([]);
const assemble_express_config_get_q = useQuery({
  queryKey: ['assemble_express_config_get'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/express/config/get"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const authentication_qiyeweixin_info_sign_ref = ref<any[]>([]);
const authentication_qiyeweixin_info_sign_q = useQuery({
  queryKey: ['authentication_qiyeweixin_info_sign'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/qiyeweixin/info/sign"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_inputperson_wipe_ref = ref<any[]>([]);
const assemble_control_inputperson_wipe_q = useQuery({
  queryKey: ['assemble_control_inputperson_wipe'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/inputperson/wipe"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_person_list_ref = ref<any[]>([]);
const organization_assemble_person_list_q = useQuery({
  queryKey: ['organization_assemble_person_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/person/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_sso_encrypt_ref = ref<any[]>([]);
const assemble_authentication_sso_encrypt_q = useQuery({
  queryKey: ['assemble_authentication_sso_encrypt'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/sso/encrypt"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_identity_list_pinyininitial_ref = ref<any[]>([]);
const control_identity_list_pinyininitial_q = useQuery({
  queryKey: ['control_identity_list_pinyininitial'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/identity/list/pinyininitial"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_person_ref = ref<any[]>([]);
const organization_person_q = useQuery({
  queryKey: ['organization_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unitduty_distinct_name_ref = ref<any[]>([]);
const control_unitduty_distinct_name_q = useQuery({
  queryKey: ['control_unitduty_distinct_name'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unitduty/distinct/name"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_express_ref = ref<any[]>([]);
const organization_assemble_express_q = useQuery({
  queryKey: ['organization_assemble_express'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization_assemble_express"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_oauth_generate_code_ref = ref<any[]>([]);
const authentication_oauth_generate_code_q = useQuery({
  queryKey: ['authentication_oauth_generate_code'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/oauth/generate/code"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_dingding_info_ref = ref<any[]>([]);
const assemble_authentication_dingding_info_q = useQuery({
  queryKey: ['assemble_authentication_dingding_info'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/dingding/info"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_unit_ref = ref<any[]>([]);
const organization_assemble_control_unit_q = useQuery({
  queryKey: ['organization_assemble_control_unit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_definition_list_ref = ref<any[]>([]);
const organization_definition_list_q = useQuery({
  queryKey: ['organization_definition_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/definition/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_authentication_captcha_ref = ref<any[]>([]);
const assemble_authentication_authentication_captcha_q = useQuery({
  queryKey: ['assemble_authentication_authentication_captcha'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/captcha"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_oauth_info_jira_ref = ref<any[]>([]);
const authentication_oauth_info_jira_q = useQuery({
  queryKey: ['authentication_oauth_info_jira'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/oauth/info/jira"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_role_ref = ref<any[]>([]);
const organization_assemble_control_role_q = useQuery({
  queryKey: ['organization_assemble_control_role'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/role"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const person_password_mockputtopost_ref = ref<any[]>([]);
const person_password_mockputtopost_q = useQuery({
  queryKey: ['person_password_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/password/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_personattribute_object_ref = ref<any[]>([]);
const person_list_personattribute_object_q = useQuery({
  queryKey: ['person_list_personattribute_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/personattribute/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_object_ref = ref<any[]>([]);
const person_list_object_q = useQuery({
  queryKey: ['person_list_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_empower_some_id_ref = ref<any[]>([]);
const person_empower_some_id_q = useQuery({
  queryKey: ['person_empower_some_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/empower/some-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_icon_ref = ref<any[]>([]);
const person_icon_q = useQuery({
  queryKey: ['person_icon'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/icon"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_login_recent_object_ref = ref<any[]>([]);
const list_login_recent_object_q = useQuery({
  queryKey: ['list_login_recent_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/login/recent/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_identity_object_ref = ref<any[]>([]);
const person_list_identity_object_q = useQuery({
  queryKey: ['person_list_identity_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/identity/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const signature_list_person_u2_admin_P_ref = ref<any[]>([]);
const signature_list_person_u2_admin_P_q = useQuery({
  queryKey: ['signature_list_person_u2_admin_P'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/signature/list/person/u2-admin@P"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_empower_some_id_enable_ref = ref<any[]>([]);
const person_empower_some_id_enable_q = useQuery({
  queryKey: ['person_empower_some_id_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/empower/some-id/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_unit_sub_direct_ref = ref<any[]>([]);
const list_unit_sub_direct_q = useQuery({
  queryKey: ['list_unit_sub_direct'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/unit/sub/direct"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_role_object_ref = ref<any[]>([]);
const person_list_role_object_q = useQuery({
  queryKey: ['person_list_role_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/role/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_login_after_ref = ref<any[]>([]);
const person_list_login_after_q = useQuery({
  queryKey: ['person_list_login_after'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/login/after"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_sub_direct_object_ref = ref<any[]>([]);
const person_sub_direct_object_q = useQuery({
  queryKey: ['person_sub_direct_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sub/direct/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_signature_manager_list_ref = ref<any[]>([]);
const person_signature_manager_list_q = useQuery({
  queryKey: ['person_signature_manager_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/signature/manager/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_has_role_ref = ref<any[]>([]);
const person_has_role_q = useQuery({
  queryKey: ['person_has_role'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/has/role"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const assemble_authentication_zhengwudingding_info_ref = ref<any[]>([]);
const assemble_authentication_zhengwudingding_info_q = useQuery({
  queryKey: ['assemble_authentication_zhengwudingding_info'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/zhengwudingding/info"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_authentication_ref = ref<any[]>([]);
const organization_assemble_authentication_q = useQuery({
  queryKey: ['organization_assemble_authentication'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization_assemble_authentication"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_identity_list_ref = ref<any[]>([]);
const organization_identity_list_q = useQuery({
  queryKey: ['organization_identity_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/identity/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_personal_ref = ref<any[]>([]);
const organization_assemble_personal_q = useQuery({
  queryKey: ['organization_assemble_personal'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization_assemble_personal"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_personcard_mylist_ref = ref<any[]>([]);
const assemble_control_personcard_mylist_q = useQuery({
  queryKey: ['assemble_control_personcard_mylist'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/personcard/mylist"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_group_list_ref = ref<any[]>([]);
const organization_group_list_q = useQuery({
  queryKey: ['organization_group_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/group/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_group_ref = ref<any[]>([]);
const organization_assemble_control_group_q = useQuery({
  queryKey: ['organization_assemble_control_group'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/group"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_authentication_code_ref = ref<any[]>([]);
const assemble_authentication_authentication_code_q = useQuery({
  queryKey: ['assemble_authentication_authentication_code'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/code"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_inputperson_ref = ref<any[]>([]);
const organization_assemble_control_inputperson_q = useQuery({
  queryKey: ['organization_assemble_control_inputperson'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/inputperson"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_identity_ref = ref<any[]>([]);
const organization_identity_q = useQuery({
  queryKey: ['organization_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const personal_custom_alt_id_mockputtopost_ref = ref<any[]>([]);
const personal_custom_alt_id_mockputtopost_q = useQuery({
  queryKey: ['personal_custom_alt_id_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/personal/custom/alt-id/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_unitduty_ref = ref<any[]>([]);
const organization_assemble_control_unitduty_q = useQuery({
  queryKey: ['organization_assemble_control_unitduty'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unitduty"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unit_get_root_ref = ref<any[]>([]);
const control_unit_get_root_q = useQuery({
  queryKey: ['control_unit_get_root'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/get/root"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unit_list_like_ref = ref<any[]>([]);
const control_unit_list_like_q = useQuery({
  queryKey: ['control_unit_list_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_export_zhengwudingding_person_ref = ref<any[]>([]);
const control_export_zhengwudingding_person_q = useQuery({
  queryKey: ['control_export_zhengwudingding_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/export/zhengwudingding/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unitduty_list_like_ref = ref<any[]>([]);
const control_unitduty_list_like_q = useQuery({
  queryKey: ['control_unitduty_list_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unitduty/list/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_role_list_pinyininitial_ref = ref<any[]>([]);
const control_role_list_pinyininitial_q = useQuery({
  queryKey: ['control_role_list_pinyininitial'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/pinyininitial"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_identity_list_like_ref = ref<any[]>([]);
const control_identity_list_like_q = useQuery({
  queryKey: ['control_identity_list_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/identity/list/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_group_list_pinyininitial_ref = ref<any[]>([]);
const control_group_list_pinyininitial_q = useQuery({
  queryKey: ['control_group_list_pinyininitial'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/group/list/pinyininitial"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_permissionsetting_list_ref = ref<any[]>([]);
const assemble_control_permissionsetting_list_q = useQuery({
  queryKey: ['assemble_control_permissionsetting_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/permissionsetting/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_person_list_like_ref = ref<any[]>([]);
const control_person_list_like_q = useQuery({
  queryKey: ['control_person_list_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/person/list/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unit_list_top_ref = ref<any[]>([]);
const control_unit_list_top_q = useQuery({
  queryKey: ['control_unit_list_top'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/top"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_authentication_captchaRSAPublicKey_ref = ref<any[]>([]);
const assemble_authentication_authentication_captchaRSAPublicKey_q = useQuery({
  queryKey: ['assemble_authentication_authentication_captchaRSAPublicKey'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/captchaRSAPublicKey"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_qiyeweixin_login_testcode_ref = ref<any[]>([]);
const authentication_qiyeweixin_login_testcode_q = useQuery({
  queryKey: ['authentication_qiyeweixin_login_testcode'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/qiyeweixin/login/testcode"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_authentication_oauth_list_ref = ref<any[]>([]);
const authentication_authentication_oauth_list_q = useQuery({
  queryKey: ['authentication_authentication_oauth_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/oauth/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const unit_sub_direct_like_ref = ref<any[]>([]);
const unit_sub_direct_like_q = useQuery({
  queryKey: ['unit_sub_direct_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/unit/sub/direct/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_pair_identity_ref = ref<any[]>([]);
const person_list_pair_identity_q = useQuery({
  queryKey: ['person_list_pair_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/pair/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_empower_manager_some_id_ref = ref<any[]>([]);
const person_empower_manager_some_id_q = useQuery({
  queryKey: ['person_empower_manager_some_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/empower/manager/some-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const personal_info_ref = ref<any[]>([]);
const personal_info_q = useQuery({
  queryKey: ['personal_info'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/personal/info"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const no_such_java_action_ref = ref<any[]>([]);
const no_such_java_action_q = useQuery({
  queryKey: ['no_such_java_action'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/no/such/java/action"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_ref = ref<any[]>([]);
const person_list_q = useQuery({
  queryKey: ['person_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_login_recent_ref = ref<any[]>([]);
const person_list_login_recent_q = useQuery({
  queryKey: ['person_list_login_recent'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/login/recent"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_identity_ref = ref<any[]>([]);
const person_list_identity_q = useQuery({
  queryKey: ['person_list_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const empower_list_to_enable_ref = ref<any[]>([]);
const empower_list_to_enable_q = useQuery({
  queryKey: ['empower_list_to_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/empower/list/to/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const personattribute_list_person_object_ref = ref<any[]>([]);
const personattribute_list_person_object_q = useQuery({
  queryKey: ['personattribute_list_person_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/personattribute/list/person/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_mobile_p1_ref = ref<any[]>([]);
const person_mobile_p1_q = useQuery({
  queryKey: ['person_mobile_p1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/mobile/p1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_regist_code_ref = ref<any[]>([]);
const person_regist_code_q = useQuery({
  queryKey: ['person_regist_code'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/regist/code"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_exmail_sso_ref = ref<any[]>([]);
const person_exmail_sso_q = useQuery({
  queryKey: ['person_exmail_sso'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/exmail/sso"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_sub_nested_object_ref = ref<any[]>([]);
const person_sub_nested_object_q = useQuery({
  queryKey: ['person_sub_nested_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sub/nested/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_sup_nested_object_ref = ref<any[]>([]);
const person_sup_nested_object_q = useQuery({
  queryKey: ['person_sup_nested_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sup/nested/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_sup_direct_object_ref = ref<any[]>([]);
const person_sup_direct_object_q = useQuery({
  queryKey: ['person_sup_direct_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sup/direct/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const exmail_list_title_passive_ref = ref<any[]>([]);
const exmail_list_title_passive_q = useQuery({
  queryKey: ['exmail_list_title_passive'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/exmail/list/title/passive"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_empower_some_id_disable_ref = ref<any[]>([]);
const person_empower_some_id_disable_q = useQuery({
  queryKey: ['person_empower_some_id_disable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/empower/some-id/disable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_empower_list_to_ref = ref<any[]>([]);
const person_empower_list_to_q = useQuery({
  queryKey: ['person_empower_list_to'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/empower/list/to"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_nick_name_p1_ref = ref<any[]>([]);
const person_nick_name_p1_q = useQuery({
  queryKey: ['person_nick_name_p1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/nick/name/p1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const list_person_sub_nested_ref = ref<any[]>([]);
const list_person_sub_nested_q = useQuery({
  queryKey: ['list_person_sub_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sub/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_regist_ref = ref<any[]>([]);
const person_regist_q = useQuery({
  queryKey: ['person_regist'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/regist"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_sub_nested_like_ref = ref<any[]>([]);
const unit_sub_nested_like_q = useQuery({
  queryKey: ['unit_sub_nested_like'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/unit/sub/nested/like"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_unit_sub_nested_ref = ref<any[]>([]);
const list_unit_sub_nested_q = useQuery({
  queryKey: ['list_unit_sub_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/unit/sub/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_person_sup_direct_ref = ref<any[]>([]);
const list_person_sup_direct_q = useQuery({
  queryKey: ['list_person_sup_direct'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sup/direct"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_signature_upload_ref = ref<any[]>([]);
const person_signature_upload_q = useQuery({
  queryKey: ['person_signature_upload'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/signature/upload"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_all_ref = ref<any[]>([]);
const person_list_all_q = useQuery({
  queryKey: ['person_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_person_sup_nested_ref = ref<any[]>([]);
const list_person_sup_nested_q = useQuery({
  queryKey: ['list_person_sup_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sup/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_person_sub_direct_ref = ref<any[]>([]);
const list_person_sub_direct_q = useQuery({
  queryKey: ['list_person_sub_direct'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/person/sub/direct"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_list_group_ref = ref<any[]>([]);
const person_list_group_q = useQuery({
  queryKey: ['person_list_group'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/group"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const personattribute_list_name_person_ref = ref<any[]>([]);
const personattribute_list_name_person_q = useQuery({
  queryKey: ['personattribute_list_name_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/personattribute/list/name/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const exmail_new_count_passive_ref = ref<any[]>([]);
const exmail_new_count_passive_q = useQuery({
  queryKey: ['exmail_new_count_passive'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/exmail/new/count/passive"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const person_icon_mockputtopost_ref = ref<any[]>([]);
const person_icon_mockputtopost_q = useQuery({
  queryKey: ['person_icon_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/icon/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const filter_1_size_20_ref = ref<any[]>([]);
const filter_1_size_20_q = useQuery({
  queryKey: ['filter_1_size_20'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/person/list/filter/1/size/20"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const personattribute_set_person_name_ref = ref<any[]>([]);
const personattribute_set_person_name_q = useQuery({
  queryKey: ['personattribute_set_person_name'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/personattribute/set/person/name"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const authentication_sms_send_ref = ref<any[]>([]);
const authentication_sms_send_q = useQuery({
  queryKey: ['authentication_sms_send'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/sms/send"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_oauth_dingding_config_ref = ref<any[]>([]);
const authentication_oauth_dingding_config_q = useQuery({
  queryKey: ['authentication_oauth_dingding_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/oauth/dingding/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_two_factor_ref = ref<any[]>([]);
const authentication_two_factor_q = useQuery({
  queryKey: ['authentication_two_factor'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/two_factor"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_unit_list_ref = ref<any[]>([]);
const authentication_unit_list_q = useQuery({
  queryKey: ['authentication_unit_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/unit/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_group_list_ref = ref<any[]>([]);
const authentication_group_list_q = useQuery({
  queryKey: ['authentication_group_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/group/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_refresh_ref = ref<any[]>([]);
const authentication_refresh_q = useQuery({
  queryKey: ['authentication_refresh'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/refresh"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_ref = ref<any[]>([]);
const authentication_q = useQuery({
  queryKey: ['authentication'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_who_ref = ref<any[]>([]);
const authentication_who_q = useQuery({
  queryKey: ['authentication_who'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/who"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_captcha_ref = ref<any[]>([]);
const authentication_captcha_q = useQuery({
  queryKey: ['authentication_captcha'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/captcha"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_safe_logout_ref = ref<any[]>([]);
const authentication_safe_logout_q = useQuery({
  queryKey: ['authentication_safe_logout'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/safe/logout"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_sms_verify_ref = ref<any[]>([]);
const authentication_sms_verify_q = useQuery({
  queryKey: ['authentication_sms_verify'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/sms/verify"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_oidc_callback_ref = ref<any[]>([]);
const authentication_oidc_callback_q = useQuery({
  queryKey: ['authentication_oidc_callback'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/oidc/callback"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_sso_encrypt_ref = ref<any[]>([]);
const authentication_sso_encrypt_q = useQuery({
  queryKey: ['authentication_sso_encrypt'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/sso/encrypt"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_login_ref = ref<any[]>([]);
const authentication_login_q = useQuery({
  queryKey: ['authentication_login'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/login"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_check_token_1_ref = ref<any[]>([]);
const authentication_check_token_1_q = useQuery({
  queryKey: ['authentication_check_token_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/check/token"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const organization_core_express_sync_ref = ref<any[]>([]);
const organization_core_express_sync_q = useQuery({
  queryKey: ['organization_core_express_sync'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/core/express/sync"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_bind_list_ref = ref<any[]>([]);
const assemble_authentication_bind_list_q = useQuery({
  queryKey: ['assemble_authentication_bind_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/bind/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_assemble_control_ref = ref<any[]>([]);
const organization_assemble_control_q = useQuery({
  queryKey: ['organization_assemble_control'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unitduty_update_member_ref = ref<any[]>([]);
const control_unitduty_update_member_q = useQuery({
  queryKey: ['control_unitduty_update_member'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unitduty/update/member"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_authentication_oauth_token_ref = ref<any[]>([]);
const assemble_authentication_oauth_token_q = useQuery({
  queryKey: ['assemble_authentication_oauth_token'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/oauth/token"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unit_list_pinyininitial_ref = ref<any[]>([]);
const control_unit_list_pinyininitial_q = useQuery({
  queryKey: ['control_unit_list_pinyininitial'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/pinyininitial"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const control_unit_list_controller_ref = ref<any[]>([]);
const control_unit_list_controller_q = useQuery({
  queryKey: ['control_unit_list_controller'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/controller"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_core_express_list_ref = ref<any[]>([]);
const organization_core_express_list_q = useQuery({
  queryKey: ['organization_core_express_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/core/express/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const organization_definition_ref = ref<any[]>([]);
const organization_definition_q = useQuery({
  queryKey: ['organization_definition'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/definition"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_organiza_643_data = ref<any[]>([]);
const { data: api_organiza_643_q } = useQuery({queryKey: ['api_organiza_643', '/jaxrs/organization/assemble/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_organiza_643_q, (v) => { api_organiza_643_data.value = v ?? []; });
const api_authenti_87_data = ref<any[]>([]);
const { data: api_authenti_87_q } = useQuery({queryKey: ['api_authenti_87', '/jaxrs/organization/assemble/authentication/authentication/safe/logout'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/safe/logout"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_87_q, (v) => { api_authenti_87_data.value = v ?? []; });
const api_control__117_data = ref<any[]>([]);
const { data: api_control__117_q } = useQuery({queryKey: ['api_control__117', '/jaxrs/organization/assemble/control/export/export/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/export/export/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__117_q, (v) => { api_control__117_data.value = v ?? []; });
const api_assemble_151_data = ref<any[]>([]);
const { data: api_assemble_151_q } = useQuery({queryKey: ['api_assemble_151', '/jaxrs/organization/assemble/authentication/oauth/info'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/authentication/oauth/info"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_151_q, (v) => { api_assemble_151_data.value = v ?? []; });
const api_organiza_368_data = ref<any[]>([]);
const { data: api_organiza_368_q } = useQuery({queryKey: ['api_organiza_368', '/jaxrs/organization/core/unit/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/core/unit/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_organiza_368_q, (v) => { api_organiza_368_data.value = v ?? []; });


const reset_ref = ref<any[]>([]);
const reset_q = useQuery({
  queryKey: ['reset'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/reset"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_reset_check_data = ref<any[]>([]);
const { data: api_reset_check_q } = useQuery({queryKey: ['api_reset_check', '/jaxrs/reset/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/reset/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_reset_check_q, (v) => { api_reset_check_data.value = v ?? []; });
const reset_password_anonymous_ref = ref<any[]>([]);
const reset_password_anonymous_q = useQuery({
  queryKey: ['reset_password_anonymous'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/reset/password/anonymous"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const reset_mockputtopost_ref = ref<any[]>([]);
const reset_mockputtopost_q = useQuery({
  queryKey: ['reset_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/reset/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const reset_set_ref = ref<any[]>([]);
const reset_set_q = useQuery({
  queryKey: ['reset_set'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/reset/set"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_reset_code_data = ref<any[]>([]);
const { data: api_reset_code_q } = useQuery({queryKey: ['api_reset_code', '/jaxrs/reset/code'], queryFn: async () => { try { const r = await api.get("/jaxrs/reset/code"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_reset_code_q, (v) => { api_reset_code_data.value = v ?? []; });


const api_unit_sub_910_data = ref<any[]>([]);
const { data: api_unit_sub_910_q } = useQuery({queryKey: ['api_unit_sub_910', '/jaxrs/identity/list/unit/sub/nested/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/unit/sub/nested/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_unit_sub_910_q, (v) => { api_unit_sub_910_data.value = v ?? []; });
const api_identity_960_data = ref<any[]>([]);
const { data: api_identity_960_q } = useQuery({queryKey: ['api_identity_960', '/jaxrs/identity/list/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_960_q, (v) => { api_identity_960_data.value = v ?? []; });
const api_identity_440_data = ref<any[]>([]);
const { data: api_identity_440_q } = useQuery({queryKey: ['api_identity_440', '/jaxrs/identity/list/group'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/group"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_440_q, (v) => { api_identity_440_data.value = v ?? []; });
const api_list_uni_202_data = ref<any[]>([]);
const { data: api_list_uni_202_q } = useQuery({queryKey: ['api_list_uni_202', '/jaxrs/identity/list/unit/person/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/unit/person/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_uni_202_q, (v) => { api_list_uni_202_data.value = v ?? []; });
const api_identity_615_data = ref<any[]>([]);
const { data: api_identity_615_q } = useQuery({queryKey: ['api_identity_615', '/jaxrs/identity/list/group/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/group/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_615_q, (v) => { api_identity_615_data.value = v ?? []; });
const api_list_maj_891_data = ref<any[]>([]);
const { data: api_list_maj_891_q } = useQuery({queryKey: ['api_list_maj_891', '/jaxrs/identity/list/major/person/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/major/person/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_maj_891_q, (v) => { api_list_maj_891_data.value = v ?? []; });
const api_unit_sub_655_data = ref<any[]>([]);
const { data: api_unit_sub_655_q } = useQuery({queryKey: ['api_unit_sub_655', '/jaxrs/identity/list/unit/sub/direct/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/unit/sub/direct/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_unit_sub_655_q, (v) => { api_unit_sub_655_data.value = v ?? []; });
const api_identity_870_data = ref<any[]>([]);
const { data: api_identity_870_q } = useQuery({queryKey: ['api_identity_870', '/jaxrs/identity/list/major/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/major/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_870_q, (v) => { api_identity_870_data.value = v ?? []; });
const api_identity_list_data = ref<any[]>([]);
const { data: api_identity_list_q } = useQuery({queryKey: ['api_identity_list', '/jaxrs/identity/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_list_q, (v) => { api_identity_list_data.value = v ?? []; });
const api_list_uni_755_data = ref<any[]>([]);
const { data: api_list_uni_755_q } = useQuery({queryKey: ['api_list_uni_755', '/jaxrs/identity/list/unit/sub/direct'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/unit/sub/direct"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_uni_755_q, (v) => { api_list_uni_755_data.value = v ?? []; });
const api_identity_929_data = ref<any[]>([]);
const { data: api_identity_929_q } = useQuery({queryKey: ['api_identity_929', '/jaxrs/identity/list/person/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/person/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_929_q, (v) => { api_identity_929_data.value = v ?? []; });
const api_identity_303_data = ref<any[]>([]);
const { data: api_identity_303_q } = useQuery({queryKey: ['api_identity_303', '/jaxrs/identity/list/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_303_q, (v) => { api_identity_303_data.value = v ?? []; });
const api_identity_920_data = ref<any[]>([]);
const { data: api_identity_920_q } = useQuery({queryKey: ['api_identity_920', '/jaxrs/identity/list/unit/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/unit/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_identity_920_q, (v) => { api_identity_920_data.value = v ?? []; });
const api_list_uni_780_data = ref<any[]>([]);
const { data: api_list_uni_780_q } = useQuery({queryKey: ['api_list_uni_780', '/jaxrs/identity/list/unit/sub/nested'], queryFn: async () => { try { const r = await api.get("/jaxrs/identity/list/unit/sub/nested"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_uni_780_q, (v) => { api_list_uni_780_data.value = v ?? []; });


const api_group_su_361_data = ref<any[]>([]);
const { data: api_group_su_361_q } = useQuery({queryKey: ['api_group_su_361', '/jaxrs/group/list/group/sup/nested/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sup/nested/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_su_361_q, (v) => { api_group_su_361_data.value = v ?? []; });
const api_group_li_139_data = ref<any[]>([]);
const { data: api_group_li_139_q } = useQuery({queryKey: ['api_group_li_139', '/jaxrs/group/list/person/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/person/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_li_139_q, (v) => { api_group_li_139_data.value = v ?? []; });
const api_group_li_561_data = ref<any[]>([]);
const { data: api_group_li_561_q } = useQuery({queryKey: ['api_group_li_561', '/jaxrs/group/list/identity/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/identity/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_li_561_q, (v) => { api_group_li_561_data.value = v ?? []; });
const api_group_has_role_data = ref<any[]>([]);
const { data: api_group_has_role_q } = useQuery({queryKey: ['api_group_has_role', '/jaxrs/group/has/role'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/has/role"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_has_role_q, (v) => { api_group_has_role_data.value = v ?? []; });
const api_group_su_857_data = ref<any[]>([]);
const { data: api_group_su_857_q } = useQuery({queryKey: ['api_group_su_857', '/jaxrs/group/list/group/sub/direct/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sub/direct/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_su_857_q, (v) => { api_group_su_857_data.value = v ?? []; });
const api_group_li_721_data = ref<any[]>([]);
const { data: api_group_li_721_q } = useQuery({queryKey: ['api_group_li_721', '/jaxrs/group/list/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_li_721_q, (v) => { api_group_li_721_data.value = v ?? []; });
const api_group_list_data = ref<any[]>([]);
const { data: api_group_list_q } = useQuery({queryKey: ['api_group_list', '/jaxrs/group/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_list_q, (v) => { api_group_list_data.value = v ?? []; });
const api_group_su_511_data = ref<any[]>([]);
const { data: api_group_su_511_q } = useQuery({queryKey: ['api_group_su_511', '/jaxrs/group/list/group/sub/nested/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sub/nested/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_su_511_q, (v) => { api_group_su_511_data.value = v ?? []; });
const api_group_li_964_data = ref<any[]>([]);
const { data: api_group_li_964_q } = useQuery({queryKey: ['api_group_li_964', '/jaxrs/group/list/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_li_964_q, (v) => { api_group_li_964_data.value = v ?? []; });
const api_group_data = ref<any[]>([]);
const { data: api_group_q } = useQuery({queryKey: ['api_group', '/jaxrs/group'], queryFn: async () => { try { const r = await api.get("/jaxrs/group"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_q, (v) => { api_group_data.value = v ?? []; });
const api_list_gro_71_data = ref<any[]>([]);
const { data: api_list_gro_71_q } = useQuery({queryKey: ['api_list_gro_71', '/jaxrs/group/list/group/sub/direct'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sub/direct"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_gro_71_q, (v) => { api_list_gro_71_data.value = v ?? []; });
const api_list_gro_677_data = ref<any[]>([]);
const { data: api_list_gro_677_q } = useQuery({queryKey: ['api_list_gro_677', '/jaxrs/group/list/group/sub/nested'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sub/nested"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_gro_677_q, (v) => { api_list_gro_677_data.value = v ?? []; });


const api_person_e_24_data = ref<any[]>([]);
const { data: api_person_e_24_q } = useQuery({queryKey: ['api_person_e_24', '/jaxrs/person/empower/manager'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/empower/manager"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_person_e_24_q, (v) => { api_person_e_24_data.value = v ?? []; });
const api_person_l_119_data = ref<any[]>([]);
const { data: api_person_l_119_q } = useQuery({queryKey: ['api_person_l_119', '/jaxrs/person/list/all/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/list/all/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_person_l_119_q, (v) => { api_person_l_119_data.value = v ?? []; });
const api_personal_559_data = ref<any[]>([]);
const { data: api_personal_559_q } = useQuery({queryKey: ['api_personal_559', '/jaxrs/personal/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/personal/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_personal_559_q, (v) => { api_personal_559_data.value = v ?? []; });
const api_person_l_163_data = ref<any[]>([]);
const { data: api_person_l_163_q } = useQuery({queryKey: ['api_person_l_163', '/jaxrs/person/list/role'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/list/role"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_person_l_163_q, (v) => { api_person_l_163_data.value = v ?? []; });
const api_empower__555_data = ref<any[]>([]);
const { data: api_empower__555_q } = useQuery({queryKey: ['api_empower__555', '/jaxrs/person/empower/list/currentperson/enable'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/empower/list/currentperson/enable"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_empower__555_q, (v) => { api_empower__555_data.value = v ?? []; });
const api_person_r_628_data = ref<any[]>([]);
const { data: api_person_r_628_q } = useQuery({queryKey: ['api_person_r_628', '/jaxrs/person/regist/mode'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/regist/mode"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_person_r_628_q, (v) => { api_person_r_628_data.value = v ?? []; });
const api_person_l_496_data = ref<any[]>([]);
const { data: api_person_l_496_q } = useQuery({queryKey: ['api_person_l_496', '/jaxrs/person/list/personattribute'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/list/personattribute"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_person_l_496_q, (v) => { api_person_l_496_data.value = v ?? []; });
const api_unit_sub_642_data = ref<any[]>([]);
const { data: api_unit_sub_642_q } = useQuery({queryKey: ['api_unit_sub_642', '/jaxrs/person/list/unit/sub/nested/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/list/unit/sub/nested/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_unit_sub_642_q, (v) => { api_unit_sub_642_data.value = v ?? []; });
const api_unit_sub_447_data = ref<any[]>([]);
const { data: api_unit_sub_447_q } = useQuery({queryKey: ['api_unit_sub_447', '/jaxrs/person/list/unit/sub/direct/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/list/unit/sub/direct/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_unit_sub_447_q, (v) => { api_unit_sub_447_data.value = v ?? []; });
const api_person_n_416_data = ref<any[]>([]);
const { data: api_person_n_416_q } = useQuery({queryKey: ['api_person_n_416', '/jaxrs/person/nick/name'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/nick/name"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_person_n_416_q, (v) => { api_person_n_416_data.value = v ?? []; });
const api_list_att_896_data = ref<any[]>([]);
const { data: api_list_att_896_q } = useQuery({queryKey: ['api_list_att_896', '/jaxrs/personattribute/list/attribute/person/name'], queryFn: async () => { try { const r = await api.get("/jaxrs/personattribute/list/attribute/person/name"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_att_896_q, (v) => { api_list_att_896_data.value = v ?? []; });
const api_personat_650_data = ref<any[]>([]);
const { data: api_personat_650_q } = useQuery({queryKey: ['api_personat_650', '/jaxrs/personattribute/append/person/name'], queryFn: async () => { try { const r = await api.get("/jaxrs/personattribute/append/person/name"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_personat_650_q, (v) => { api_personat_650_data.value = v ?? []; });


const form_v2_f_1_mobile_ref = ref<any[]>([]);
const form_v2_f_1_mobile_q = useQuery({
  queryKey: ['form_v2_f_1_mobile'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/anonymous/form/v2/f-1/mobile"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const anonymous_form_f_1_ref = ref<any[]>([]);
const anonymous_form_f_1_q = useQuery({
  queryKey: ['anonymous_form_f_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/anonymous/form/f-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_v2_lookup_docume_21_data = ref<any[]>([]);
const { data: api_v2_lookup_docume_21_q } = useQuery({queryKey: ['api_v2_lookup_docume_21', '/jaxrs/anonymous/form/v2/lookup/document/d-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/form/v2/lookup/document/d-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_v2_lookup_docume_21_q, (v) => { api_v2_lookup_docume_21_data.value = v ?? []; });
const api_download_documen_318_data = ref<any[]>([]);
const { data: api_download_documen_318_q } = useQuery({queryKey: ['api_download_documen_318', '/jaxrs/anonymous/fileinfo/download/document/d-1/stream'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/fileinfo/download/document/d-1/stream"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_download_documen_318_q, (v) => { api_download_documen_318_data.value = v ?? []; });
const api_fileinfo_fi_1_do_680_data = ref<any[]>([]);
const { data: api_fileinfo_fi_1_do_680_q } = useQuery({queryKey: ['api_fileinfo_fi_1_do_680', '/jaxrs/anonymous/fileinfo/fi-1/document/d-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/fileinfo/fi-1/document/d-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_fileinfo_fi_1_do_680_q, (v) => { api_fileinfo_fi_1_do_680_data.value = v ?? []; });
const api_file_an_1_downlo_103_data = ref<any[]>([]);
const { data: api_file_an_1_downlo_103_q } = useQuery({queryKey: ['api_file_an_1_downlo_103', '/jaxrs/anonymous/file/an-1/download/stream'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/file/an-1/download/stream"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_file_an_1_downlo_103_q, (v) => { api_file_an_1_downlo_103_data.value = v ?? []; });
const anonymous_form_v2_f_1_ref = ref<any[]>([]);
const anonymous_form_v2_f_1_q = useQuery({
  queryKey: ['anonymous_form_v2_f_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/anonymous/form/v2/f-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_fileinfo_list_do_265_data = ref<any[]>([]);
const { data: api_fileinfo_list_do_265_q } = useQuery({queryKey: ['api_fileinfo_list_do_265', '/jaxrs/anonymous/fileinfo/list/document/d-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/fileinfo/list/document/d-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_fileinfo_list_do_265_q, (v) => { api_fileinfo_list_do_265_data.value = v ?? []; });
const api_anonymous_file_a_860_data = ref<any[]>([]);
const { data: api_anonymous_file_a_860_q } = useQuery({queryKey: ['api_anonymous_file_a_860', '/jaxrs/anonymous/file/an-1/download'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/file/an-1/download"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_anonymous_file_a_860_q, (v) => { api_anonymous_file_a_860_data.value = v ?? []; });


const api_authentication_r_85_data = ref<any[]>([]);
const { data: api_authentication_r_85_q } = useQuery({queryKey: ['api_authentication_r_85', '/jaxrs/authentication/role/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/role/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authentication_r_85_q, (v) => { api_authentication_r_85_data.value = v ?? []; });
const authentication_switchuser_ref = ref<any[]>([]);
const authentication_switchuser_q = useQuery({
  queryKey: ['authentication_switchuser'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/switchuser"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_authentication_o_26_data = ref<any[]>([]);
const { data: api_authentication_o_26_q } = useQuery({queryKey: ['api_authentication_o_26', '/jaxrs/authentication/oauth/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/oauth/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authentication_o_26_q, (v) => { api_authentication_o_26_data.value = v ?? []; });
const api_authenti_261_data = ref<any[]>([]);
const { data: api_authenti_261_q } = useQuery({queryKey: ['api_authenti_261', '/jaxrs/authentication/logout'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/logout"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_261_q, (v) => { api_authenti_261_data.value = v ?? []; });
const authentication_oauth_ref = ref<any[]>([]);
const authentication_oauth_q = useQuery({
  queryKey: ['authentication_oauth'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/oauth"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_switchuser_mockputtopost_1_ref = ref<any[]>([]);
const authentication_switchuser_mockputtopost_1_q = useQuery({
  queryKey: ['authentication_switchuser_mockputtopost_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/switchuser/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_authenti_932_data = ref<any[]>([]);
const { data: api_authenti_932_q } = useQuery({queryKey: ['api_authenti_932', '/jaxrs/authentication/code/credential/2fa-full-user'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/code/credential/2fa-full-user"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_932_q, (v) => { api_authenti_932_data.value = v ?? []; });
const api_authenti_77_data = ref<any[]>([]);
const { data: api_authenti_77_q } = useQuery({queryKey: ['api_authenti_77', '/jaxrs/authentication/code/credential/admin'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/code/credential/admin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_77_q, (v) => { api_authenti_77_data.value = v ?? []; });
const authentication_sso_ref = ref<any[]>([]);
const authentication_sso_q = useQuery({
  queryKey: ['authentication_sso'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/sso"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const authentication_two_ref = ref<any[]>([]);
const authentication_two_q = useQuery({
  queryKey: ['authentication_two'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/two"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_authenti_296_data = ref<any[]>([]);
const { data: api_authenti_296_q } = useQuery({queryKey: ['api_authenti_296', '/jaxrs/authentication/code'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/code"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_296_q, (v) => { api_authenti_296_data.value = v ?? []; });
const api_authentication_o_923_data = ref<any[]>([]);
const { data: api_authentication_o_923_q } = useQuery({queryKey: ['api_authentication_o_923', '/jaxrs/authentication/oauth/qywx/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/oauth/qywx/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authentication_o_923_q, (v) => { api_authentication_o_923_data.value = v ?? []; });
const authentication_safe_ref = ref<any[]>([]);
const authentication_safe_q = useQuery({
  queryKey: ['authentication_safe'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/safe"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_authenti_908_data = ref<any[]>([]);
const { data: api_authenti_908_q } = useQuery({queryKey: ['api_authenti_908', '/jaxrs/authentication/bind'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/bind"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_908_q, (v) => { api_authenti_908_data.value = v ?? []; });
const authentication_oidc_authorize_ref = ref<any[]>([]);
const authentication_oidc_authorize_q = useQuery({
  queryKey: ['authentication_oidc_authorize'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/authentication/oidc/authorize"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_group_list_identity_data = ref<any[]>([]);
const { data: api_group_list_identity_q } = useQuery({queryKey: ['api_group_list_identity', '/jaxrs/group/list/identity'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/identity"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_list_identity_q, (v) => { api_group_list_identity_data.value = v ?? []; });
const api_group_list_group_tree_data = ref<any[]>([]);
const { data: api_group_list_group_tree_q } = useQuery({queryKey: ['api_group_list_group_tree', '/jaxrs/group/list/group/tree'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/tree"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_list_group_tree_q, (v) => { api_group_list_group_tree_data.value = v ?? []; });
const api_group_sup_direct_726_data = ref<any[]>([]);
const { data: api_group_sup_direct_726_q } = useQuery({queryKey: ['api_group_sup_direct_726', '/jaxrs/group/list/group/sup/direct/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sup/direct/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_group_sup_direct_726_q, (v) => { api_group_sup_direct_726_data.value = v ?? []; });
const api_list_group_sup_direct_data = ref<any[]>([]);
const { data: api_list_group_sup_direct_q } = useQuery({queryKey: ['api_list_group_sup_direct', '/jaxrs/group/list/group/sup/direct'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sup/direct"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_group_sup_direct_q, (v) => { api_list_group_sup_direct_data.value = v ?? []; });
const api_list_group_sup_nested_data = ref<any[]>([]);
const { data: api_list_group_sup_nested_q } = useQuery({queryKey: ['api_list_group_sup_nested', '/jaxrs/group/list/group/sup/nested'], queryFn: async () => { try { const r = await api.get("/jaxrs/group/list/group/sup/nested"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_group_sup_nested_q, (v) => { api_list_group_sup_nested_data.value = v ?? []; });

const api_jaxrs_person_emp_379_data = ref<any[]>([]);
const { data: api_jaxrs_person_emp_379_q } = useQuery({queryKey: ['api_jaxrs_person_emp_379', '/jaxrs/person/empower/manager/list/paging/1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/empower/manager/list/paging/1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_person_emp_379_q, (v) => { api_jaxrs_person_emp_379_data.value = v ?? []; });
const api_jaxrs_person_emp_727_data = ref<any[]>([]);
const { data: api_jaxrs_person_emp_727_q } = useQuery({queryKey: ['api_jaxrs_person_emp_727', '/jaxrs/person/empowerlog/list/currentperson/paging/1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/empowerlog/list/currentperson/paging/1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_person_emp_727_q, (v) => { api_jaxrs_person_emp_727_data.value = v ?? []; });
const api_jaxrs_person_lis_778_data = ref<any[]>([]);
const { data: api_jaxrs_person_lis_778_q } = useQuery({queryKey: ['api_jaxrs_person_lis_778', '/jaxrs/person/list/unit/sub/nested/like/object'], queryFn: async () => { try { const r = await api.get("/jaxrs/person/list/unit/sub/nested/like/object"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_person_lis_778_q, (v) => { api_jaxrs_person_lis_778_data.value = v ?? []; });
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
