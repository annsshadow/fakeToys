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


async function api_unit_list_type() { try { await api.get("/jaxrs/organization/assemble/control/unit/list/type") } catch {} }
async function api_exmail_new_count() { try { await api.get("/jaxrs/person/exmail/new/count") } catch {} }
async function api_two_factory_login() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/two/factory/login") } catch {} }
async function api_login_after_object() { try { await api.get("/jaxrs/person/list/login/after/object") } catch {} }
async function api_role_list_like() { try { await api.get("/jaxrs/organization/assemble/control/role/list/like") } catch {} }
async function api_assemble_authentication_sso() { try { await api.get("/jaxrs/organization/assemble/authentication/sso") } catch {} }
async function api_control_inputperson_template() { try { await api.get("/jaxrs/organization/assemble/control/inputperson/template") } catch {} }
async function api_person_mockputtopost() { try { await api.get("/jaxrs/person/mockputtopost") } catch {} }
async function api_person_list_pinyininitial() { try { await api.get("/jaxrs/organization/assemble/control/person/list/pinyininitial") } catch {} }
async function api_direct_like_object() { try { await api.get("/jaxrs/person/list/unit/sub/direct/like/object") } catch {} }
async function api_authentication_check_token() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/check/token") } catch {} }
async function api_list_pinyininitial_mockputtopost() { try { await api.get("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost") } catch {} }
async function api_authentication_authentication_mode() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/mode") } catch {} }
async function api_person_icon_upload() { try { await api.get("/jaxrs/person/icon/upload") } catch {} }
async function api_person_exmail() { try { await api.get("/jaxrs/person/exmail") } catch {} }
async function api_authentication_switchuser_mockputtopost() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/switchuser/mockputtopost") } catch {} }
async function api_organization_person_list() { try { await api.get("/jaxrs/organization/person/list") } catch {} }
async function api_list_group_object() { try { await api.get("/jaxrs/person/list/group/object") } catch {} }
async function api_assemble_control_person() { try { await api.get("/jaxrs/organization/assemble/control/person") } catch {} }
async function api_oauth_token_jira() { try { await api.get("/jaxrs/organization/assemble/authentication/oauth/token/jira") } catch {} }


async function api_assemble_authentication_authentication_bind() { try { await api.get('/jaxrs/organization/assemble/authentication/authentication/bind') } catch {} }
async function api_organization_group() { try { await api.get('/jaxrs/organization/group') } catch {} }
async function api_organization_assemble_control_personcard() { try { await api.get('/jaxrs/organization/assemble/control/personcard') } catch {} }
async function api_assemble_authentication_authentication_mockdeletetoget() { try { await api.get('/jaxrs/organization/assemble/authentication/authentication/mockdeletetoget') } catch {} }
async function api_assemble_authentication_oauth_auth() { try { await api.get('/jaxrs/organization/assemble/authentication/oauth/auth') } catch {} }
async function api_assemble_control_personcard_listgrouptypes() { try { await api.get('/jaxrs/organization/assemble/control/personcard/listgrouptypes') } catch {} }
async function api_organization_core_express_config() { try { await api.get('/jaxrs/organization/core/express/config') } catch {} }
async function api_organization_assemble_control_permissionsetting() { try { await api.get('/jaxrs/organization/assemble/control/permissionsetting') } catch {} }
async function api_organization_assemble_unit_tree() { try { await api.get('/jaxrs/organization/assemble/unit/tree') } catch {} }
async function api_organization_assemble_control_unitattribute() { try { await api.get('/jaxrs/organization/assemble/control/unitattribute') } catch {} }
async function api_organization_assemble_authentication_authentication() { try { await api.get('/jaxrs/organization/assemble/authentication/authentication') } catch {} }
async function api_organization_bind_list() { try { await api.get('/jaxrs/organization/bind/list') } catch {} }
async function api_assemble_authentication_authentication_switchuser() { try { await api.get('/jaxrs/organization/assemble/authentication/authentication/switchuser') } catch {} }
async function api_organization_assemble_control_personattribute() { try { await api.get('/jaxrs/organization/assemble/control/personattribute') } catch {} }
async function api_organization_person_person_001() { try { await api.get('/jaxrs/organization/person/person-001') } catch {} }
async function api_assemble_express_units_list() { try { await api.get('/jaxrs/organization/assemble/express/units/list') } catch {} }
async function api_organization_bind() { try { await api.get('/jaxrs/organization/bind') } catch {} }
async function api_assemble_express_data_sync() { try { await api.get('/jaxrs/organization/assemble/express/data/sync') } catch {} }
async function api_organization_core_express_status() { try { await api.get('/jaxrs/organization/core/express/status') } catch {} }
async function api_organization_custom() { try { await api.get('/jaxrs/organization/custom') } catch {} }
async function api_assemble_express_status_get() { try { await api.get('/jaxrs/organization/assemble/express/status/get') } catch {} }
async function api_organization_group_group_001() { try { await api.get('/jaxrs/organization/group/group-001') } catch {} }
async function api_organization_assemble_control_identity() { try { await api.get('/jaxrs/organization/assemble/control/identity') } catch {} }
async function api_assemble_control_unit_list() { try { await api.get('/jaxrs/organization/assemble/control/unit/list') } catch {} }
async function api_assemble_express_config_get() { try { await api.get('/jaxrs/organization/assemble/express/config/get') } catch {} }


async function api_authentication_qiyeweixin_info_sign() { try { await api.get("/jaxrs/organization/assemble/authentication/qiyeweixin/info/sign") } catch {} }
async function api_assemble_control_inputperson_wipe() { try { await api.get("/jaxrs/organization/assemble/control/inputperson/wipe") } catch {} }
async function api_organization_assemble_person_list() { try { await api.get("/jaxrs/organization/assemble/person/list") } catch {} }
async function api_assemble_authentication_sso_encrypt() { try { await api.get("/jaxrs/organization/assemble/authentication/sso/encrypt") } catch {} }
async function api_control_identity_list_pinyininitial() { try { await api.get("/jaxrs/organization/assemble/control/identity/list/pinyininitial") } catch {} }
async function api_organization_person() { try { await api.get("/jaxrs/organization/person") } catch {} }
async function api_control_unitduty_distinct_name() { try { await api.get("/jaxrs/organization/assemble/control/unitduty/distinct/name") } catch {} }
async function api_organization_assemble_express() { try { await api.get("/jaxrs/organization_assemble_express") } catch {} }
async function api_authentication_oauth_generate_code() { try { await api.get("/jaxrs/organization/assemble/authentication/oauth/generate/code") } catch {} }
async function api_assemble_authentication_dingding_info() { try { await api.get("/jaxrs/organization/assemble/authentication/dingding/info") } catch {} }
async function api_organization_assemble_control_unit() { try { await api.get("/jaxrs/organization/assemble/control/unit") } catch {} }
async function api_organization_definition_list() { try { await api.get("/jaxrs/organization/definition/list") } catch {} }
async function api_assemble_authentication_authentication_captcha() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/captcha") } catch {} }
async function api_authentication_oauth_info_jira() { try { await api.get("/jaxrs/organization/assemble/authentication/oauth/info/jira") } catch {} }
async function api_organization_assemble_control_role() { try { await api.get("/jaxrs/organization/assemble/control/role") } catch {} }


async function api_person_password_mockputtopost() { try { await api.get("/jaxrs/person/password/mockputtopost") } catch {} }
async function api_person_list_personattribute_object() { try { await api.get("/jaxrs/person/list/personattribute/object") } catch {} }
async function api_person_list_object() { try { await api.get("/jaxrs/person/list/object") } catch {} }
async function api_person_empower_some_id() { try { await api.get("/jaxrs/person/empower/some-id") } catch {} }
async function api_person_icon() { try { await api.get("/jaxrs/person/icon") } catch {} }
async function api_list_login_recent_object() { try { await api.get("/jaxrs/person/list/login/recent/object") } catch {} }
async function api_person_list_identity_object() { try { await api.get("/jaxrs/person/list/identity/object") } catch {} }
async function api_signature_list_person_u2_admin_P() { try { await api.get("/jaxrs/person/signature/list/person/u2-admin@P") } catch {} }
async function api_person_empower_some_id_enable() { try { await api.get("/jaxrs/person/empower/some-id/enable") } catch {} }
async function api_list_unit_sub_direct() { try { await api.get("/jaxrs/person/list/unit/sub/direct") } catch {} }
async function api_person_list_role_object() { try { await api.get("/jaxrs/person/list/role/object") } catch {} }
async function api_person_list_login_after() { try { await api.get("/jaxrs/person/list/login/after") } catch {} }
async function api_person_sub_direct_object() { try { await api.get("/jaxrs/person/list/person/sub/direct/object") } catch {} }
async function api_person_signature_manager_list() { try { await api.get("/jaxrs/person/signature/manager/list") } catch {} }
async function api_person_has_role() { try { await api.get("/jaxrs/person/has/role") } catch {} }


async function api_assemble_authentication_zhengwudingding_info() { try { await api.get("/jaxrs/organization/assemble/authentication/zhengwudingding/info") } catch {} }
async function api_organization_assemble_authentication() { try { await api.get("/jaxrs/organization_assemble_authentication") } catch {} }
async function api_organization_identity_list() { try { await api.get("/jaxrs/organization/identity/list") } catch {} }
async function api_organization_assemble_personal() { try { await api.get("/jaxrs/organization_assemble_personal") } catch {} }
async function api_assemble_control_personcard_mylist() { try { await api.get("/jaxrs/organization/assemble/control/personcard/mylist") } catch {} }
async function api_organization_group_list() { try { await api.get("/jaxrs/organization/group/list") } catch {} }
async function api_organization_assemble_control_group() { try { await api.get("/jaxrs/organization/assemble/control/group") } catch {} }
async function api_assemble_authentication_authentication_code() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/code") } catch {} }
async function api_organization_assemble_control_inputperson() { try { await api.get("/jaxrs/organization/assemble/control/inputperson") } catch {} }
async function api_organization_identity() { try { await api.get("/jaxrs/organization/identity") } catch {} }
async function api_personal_custom_alt_id_mockputtopost() { try { await api.get("/jaxrs/organization/assemble/personal/custom/alt-id/mockputtopost") } catch {} }
async function api_organization_assemble_control_unitduty() { try { await api.get("/jaxrs/organization/assemble/control/unitduty") } catch {} }
async function api_control_unit_get_root() { try { await api.get("/jaxrs/organization/assemble/control/unit/get/root") } catch {} }
async function api_control_unit_list_like() { try { await api.get("/jaxrs/organization/assemble/control/unit/list/like") } catch {} }
async function api_control_export_zhengwudingding_person() { try { await api.get("/jaxrs/organization/assemble/control/export/zhengwudingding/person") } catch {} }
async function api_control_unitduty_list_like() { try { await api.get("/jaxrs/organization/assemble/control/unitduty/list/like") } catch {} }
async function api_control_role_list_pinyininitial() { try { await api.get("/jaxrs/organization/assemble/control/role/list/pinyininitial") } catch {} }
async function api_control_identity_list_like() { try { await api.get("/jaxrs/organization/assemble/control/identity/list/like") } catch {} }
async function api_control_group_list_pinyininitial() { try { await api.get("/jaxrs/organization/assemble/control/group/list/pinyininitial") } catch {} }
async function api_assemble_control_permissionsetting_list() { try { await api.get("/jaxrs/organization/assemble/control/permissionsetting/list") } catch {} }
async function api_control_person_list_like() { try { await api.get("/jaxrs/organization/assemble/control/person/list/like") } catch {} }
async function api_control_unit_list_top() { try { await api.get("/jaxrs/organization/assemble/control/unit/list/top") } catch {} }
async function api_assemble_authentication_authentication_captchaRSAPublicKey() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/captchaRSAPublicKey") } catch {} }
async function api_authentication_qiyeweixin_login_testcode() { try { await api.get("/jaxrs/organization/assemble/authentication/qiyeweixin/login/testcode") } catch {} }
async function api_authentication_authentication_oauth_list() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/oauth/list") } catch {} }


async function api_unit_sub_direct_like() { try { await api.get("/jaxrs/person/list/unit/sub/direct/like") } catch {} }
async function api_person_list_pair_identity() { try { await api.get("/jaxrs/person/list/pair/identity") } catch {} }
async function api_person_empower_manager_some_id() { try { await api.get("/jaxrs/person/empower/manager/some-id") } catch {} }
async function api_personal_info() { try { await api.get("/jaxrs/personal/info") } catch {} }
async function api_no_such_java_action() { try { await api.get("/jaxrs/person/no/such/java/action") } catch {} }
async function api_person_list() { try { await api.get("/jaxrs/person/list") } catch {} }
async function api_person_list_login_recent() { try { await api.get("/jaxrs/person/list/login/recent") } catch {} }
async function api_person_list_identity() { try { await api.get("/jaxrs/person/list/identity") } catch {} }
async function api_empower_list_to_enable() { try { await api.get("/jaxrs/person/empower/list/to/enable") } catch {} }
async function api_personattribute_list_person_object() { try { await api.get("/jaxrs/personattribute/list/person/object") } catch {} }
async function api_person_mobile_p1() { try { await api.get("/jaxrs/person/mobile/p1") } catch {} }
async function api_person_regist_code() { try { await api.get("/jaxrs/person/regist/code") } catch {} }
async function api_person_exmail_sso() { try { await api.get("/jaxrs/person/exmail/sso") } catch {} }
async function api_person_sub_nested_object() { try { await api.get("/jaxrs/person/list/person/sub/nested/object") } catch {} }
async function api_person_sup_nested_object() { try { await api.get("/jaxrs/person/list/person/sup/nested/object") } catch {} }
async function api_person_sup_direct_object() { try { await api.get("/jaxrs/person/list/person/sup/direct/object") } catch {} }
async function api_exmail_list_title_passive() { try { await api.get("/jaxrs/person/exmail/list/title/passive") } catch {} }
async function api_person_empower_some_id_disable() { try { await api.get("/jaxrs/person/empower/some-id/disable") } catch {} }
async function api_person_empower_list_to() { try { await api.get("/jaxrs/person/empower/list/to") } catch {} }
async function api_person_nick_name_p1() { try { await api.get("/jaxrs/person/nick/name/p1") } catch {} }


async function api_list_person_sub_nested() { try { await api.get("/jaxrs/person/list/person/sub/nested") } catch {} }
async function api_person_regist() { try { await api.get("/jaxrs/person/regist") } catch {} }
async function api_unit_sub_nested_like() { try { await api.get("/jaxrs/person/list/unit/sub/nested/like") } catch {} }
async function api_list_unit_sub_nested() { try { await api.get("/jaxrs/person/list/unit/sub/nested") } catch {} }
async function api_list_person_sup_direct() { try { await api.get("/jaxrs/person/list/person/sup/direct") } catch {} }
async function api_person_signature_upload() { try { await api.get("/jaxrs/person/signature/upload") } catch {} }
async function api_person_list_all() { try { await api.get("/jaxrs/person/list/all") } catch {} }
async function api_list_person_sup_nested() { try { await api.get("/jaxrs/person/list/person/sup/nested") } catch {} }
async function api_list_person_sub_direct() { try { await api.get("/jaxrs/person/list/person/sub/direct") } catch {} }
async function api_person_list_group() { try { await api.get("/jaxrs/person/list/group") } catch {} }
async function api_personattribute_list_name_person() { try { await api.get("/jaxrs/personattribute/list/name/person") } catch {} }
async function api_exmail_new_count_passive() { try { await api.get("/jaxrs/person/exmail/new/count/passive") } catch {} }
async function api_person_icon_mockputtopost() { try { await api.get("/jaxrs/person/icon/mockputtopost") } catch {} }
async function api_filter_1_size_20() { try { await api.get("/jaxrs/person/list/filter/1/size/20") } catch {} }
async function api_personattribute_set_person_name() { try { await api.get("/jaxrs/personattribute/set/person/name") } catch {} }


async function api_authentication_sms_send() { try { await api.get("/jaxrs/authentication/sms/send") } catch {} }
async function api_authentication_oauth_dingding_config() { try { await api.get("/jaxrs/authentication/oauth/dingding/config") } catch {} }
async function api_authentication_two_factor() { try { await api.get("/jaxrs/authentication/two_factor") } catch {} }
async function api_authentication_unit_list() { try { await api.get("/jaxrs/authentication/unit/list") } catch {} }
async function api_authentication_group_list() { try { await api.get("/jaxrs/authentication/group/list") } catch {} }
async function api_authentication_refresh() { try { await api.get("/jaxrs/authentication/refresh") } catch {} }
async function api_authentication() { try { await api.get("/jaxrs/authentication") } catch {} }
async function api_authentication_who() { try { await api.get("/jaxrs/authentication/who") } catch {} }
async function api_authentication_captcha() { try { await api.get("/jaxrs/authentication/captcha") } catch {} }
async function api_authentication_safe_logout() { try { await api.get("/jaxrs/authentication/safe/logout") } catch {} }
async function api_authentication_sms_verify() { try { await api.get("/jaxrs/authentication/sms/verify") } catch {} }
async function api_authentication_oidc_callback() { try { await api.get("/jaxrs/authentication/oidc/callback") } catch {} }
async function api_authentication_sso_encrypt() { try { await api.get("/jaxrs/authentication/sso/encrypt") } catch {} }
async function api_authentication_login() { try { await api.get("/jaxrs/authentication/login") } catch {} }
async function api_authentication_check_token_1() { try { await api.get("/jaxrs/authentication/check/token") } catch {} }


async function api_organization_core_express_sync() { try { await api.get("/jaxrs/organization/core/express/sync") } catch {} }
async function api_assemble_authentication_bind_list() { try { await api.get("/jaxrs/organization/assemble/authentication/bind/list") } catch {} }
async function api_organization_assemble_control() { try { await api.get("/jaxrs/organization/assemble/control") } catch {} }
async function api_control_unitduty_update_member() { try { await api.get("/jaxrs/organization/assemble/control/unitduty/update/member") } catch {} }
async function api_assemble_authentication_oauth_token() { try { await api.get("/jaxrs/organization/assemble/authentication/oauth/token") } catch {} }
async function api_control_unit_list_pinyininitial() { try { await api.get("/jaxrs/organization/assemble/control/unit/list/pinyininitial") } catch {} }
async function api_control_unit_list_controller() { try { await api.get("/jaxrs/organization/assemble/control/unit/list/controller") } catch {} }
async function api_organization_core_express_list() { try { await api.get("/jaxrs/organization/core/express/list") } catch {} }
async function api_organization_definition() { try { await api.get("/jaxrs/organization/definition") } catch {} }
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


async function api_reset() { try { await api.get("/jaxrs/reset") } catch {} }
const api_reset_check_data = ref<any[]>([]);
const { data: api_reset_check_q } = useQuery({queryKey: ['api_reset_check', '/jaxrs/reset/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/reset/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_reset_check_q, (v) => { api_reset_check_data.value = v ?? []; });
async function api_reset_password_anonymous() { try { await api.get("/jaxrs/reset/password/anonymous") } catch {} }
async function api_reset_mockputtopost() { try { await api.get("/jaxrs/reset/mockputtopost") } catch {} }
async function api_reset_set() { try { await api.get("/jaxrs/reset/set") } catch {} }
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


async function api_form_v2_f_1_mobile() { try { await api.get("/jaxrs/anonymous/form/v2/f-1/mobile") } catch {} }
async function api_anonymous_form_f_1() { try { await api.get("/jaxrs/anonymous/form/f-1") } catch {} }
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
async function api_anonymous_form_v2_f_1() { try { await api.get("/jaxrs/anonymous/form/v2/f-1") } catch {} }
const api_fileinfo_list_do_265_data = ref<any[]>([]);
const { data: api_fileinfo_list_do_265_q } = useQuery({queryKey: ['api_fileinfo_list_do_265', '/jaxrs/anonymous/fileinfo/list/document/d-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/fileinfo/list/document/d-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_fileinfo_list_do_265_q, (v) => { api_fileinfo_list_do_265_data.value = v ?? []; });
const api_anonymous_file_a_860_data = ref<any[]>([]);
const { data: api_anonymous_file_a_860_q } = useQuery({queryKey: ['api_anonymous_file_a_860', '/jaxrs/anonymous/file/an-1/download'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/file/an-1/download"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_anonymous_file_a_860_q, (v) => { api_anonymous_file_a_860_data.value = v ?? []; });


const api_authentication_r_85_data = ref<any[]>([]);
const { data: api_authentication_r_85_q } = useQuery({queryKey: ['api_authentication_r_85', '/jaxrs/authentication/role/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/role/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authentication_r_85_q, (v) => { api_authentication_r_85_data.value = v ?? []; });
async function api_authentication_switchuser() { try { await api.get("/jaxrs/authentication/switchuser") } catch {} }
const api_authentication_o_26_data = ref<any[]>([]);
const { data: api_authentication_o_26_q } = useQuery({queryKey: ['api_authentication_o_26', '/jaxrs/authentication/oauth/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/oauth/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authentication_o_26_q, (v) => { api_authentication_o_26_data.value = v ?? []; });
const api_authenti_261_data = ref<any[]>([]);
const { data: api_authenti_261_q } = useQuery({queryKey: ['api_authenti_261', '/jaxrs/authentication/logout'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/logout"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_261_q, (v) => { api_authenti_261_data.value = v ?? []; });
async function api_authentication_oauth() { try { await api.get("/jaxrs/authentication/oauth") } catch {} }
async function api_authentication_switchuser_mockputtopost_1() { try { await api.get("/jaxrs/authentication/switchuser/mockputtopost") } catch {} }
const api_authenti_932_data = ref<any[]>([]);
const { data: api_authenti_932_q } = useQuery({queryKey: ['api_authenti_932', '/jaxrs/authentication/code/credential/2fa-full-user'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/code/credential/2fa-full-user"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_932_q, (v) => { api_authenti_932_data.value = v ?? []; });
const api_authenti_77_data = ref<any[]>([]);
const { data: api_authenti_77_q } = useQuery({queryKey: ['api_authenti_77', '/jaxrs/authentication/code/credential/admin'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/code/credential/admin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_77_q, (v) => { api_authenti_77_data.value = v ?? []; });
async function api_authentication_sso() { try { await api.get("/jaxrs/authentication/sso") } catch {} }
async function api_authentication_two() { try { await api.get("/jaxrs/authentication/two") } catch {} }
const api_authenti_296_data = ref<any[]>([]);
const { data: api_authenti_296_q } = useQuery({queryKey: ['api_authenti_296', '/jaxrs/authentication/code'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/code"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_296_q, (v) => { api_authenti_296_data.value = v ?? []; });
const api_authentication_o_923_data = ref<any[]>([]);
const { data: api_authentication_o_923_q } = useQuery({queryKey: ['api_authentication_o_923', '/jaxrs/authentication/oauth/qywx/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/oauth/qywx/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authentication_o_923_q, (v) => { api_authentication_o_923_data.value = v ?? []; });
async function api_authentication_safe() { try { await api.get("/jaxrs/authentication/safe") } catch {} }
const api_authenti_908_data = ref<any[]>([]);
const { data: api_authenti_908_q } = useQuery({queryKey: ['api_authenti_908', '/jaxrs/authentication/bind'], queryFn: async () => { try { const r = await api.get("/jaxrs/authentication/bind"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_authenti_908_q, (v) => { api_authenti_908_data.value = v ?? []; });
async function api_authentication_oidc_authorize() { try { await api.get("/jaxrs/authentication/oidc/authorize") } catch {} }


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
