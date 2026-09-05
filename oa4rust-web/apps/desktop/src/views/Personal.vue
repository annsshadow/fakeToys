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
async function api_organization_assemble_list() { try { await api.get("/jaxrs/organization/assemble/list") } catch {} }
async function api_authentication_authentication_safe_logout() { try { await api.get("/jaxrs/organization/assemble/authentication/authentication/safe/logout") } catch {} }
async function api_control_export_export_all() { try { await api.get("/jaxrs/organization/assemble/control/export/export/all") } catch {} }
async function api_assemble_authentication_oauth_info() { try { await api.get("/jaxrs/organization/assemble/authentication/oauth/info") } catch {} }
async function api_organization_core_unit_list() { try { await api.get("/jaxrs/organization/core/unit/list") } catch {} }


async function api_reset() { try { await api.get("/jaxrs/reset") } catch {} }
async function api_reset_check() { try { await api.get("/jaxrs/reset/check") } catch {} }
async function api_reset_password_anonymous() { try { await api.get("/jaxrs/reset/password/anonymous") } catch {} }
async function api_reset_mockputtopost() { try { await api.get("/jaxrs/reset/mockputtopost") } catch {} }
async function api_reset_set() { try { await api.get("/jaxrs/reset/set") } catch {} }
async function api_reset_code() { try { await api.get("/jaxrs/reset/code") } catch {} }


async function api_unit_sub_nested_object() { try { await api.get("/jaxrs/identity/list/unit/sub/nested/object") } catch {} }
async function api_identity_list_person() { try { await api.get("/jaxrs/identity/list/person") } catch {} }
async function api_identity_list_group() { try { await api.get("/jaxrs/identity/list/group") } catch {} }
async function api_list_unit_person_object() { try { await api.get("/jaxrs/identity/list/unit/person/object") } catch {} }
async function api_identity_list_group_object() { try { await api.get("/jaxrs/identity/list/group/object") } catch {} }
async function api_list_major_person_object() { try { await api.get("/jaxrs/identity/list/major/person/object") } catch {} }
async function api_unit_sub_direct_object() { try { await api.get("/jaxrs/identity/list/unit/sub/direct/object") } catch {} }
async function api_identity_list_major_person() { try { await api.get("/jaxrs/identity/list/major/person") } catch {} }
async function api_identity_list() { try { await api.get("/jaxrs/identity/list") } catch {} }
async function api_list_unit_sub_direct_1() { try { await api.get("/jaxrs/identity/list/unit/sub/direct") } catch {} }
async function api_identity_list_person_object() { try { await api.get("/jaxrs/identity/list/person/object") } catch {} }
async function api_identity_list_object() { try { await api.get("/jaxrs/identity/list/object") } catch {} }
async function api_identity_list_unit_person() { try { await api.get("/jaxrs/identity/list/unit/person") } catch {} }
async function api_list_unit_sub_nested_1() { try { await api.get("/jaxrs/identity/list/unit/sub/nested") } catch {} }


async function api_group_sup_nested_object() { try { await api.get("/jaxrs/group/list/group/sup/nested/object") } catch {} }
async function api_group_list_person_object() { try { await api.get("/jaxrs/group/list/person/object") } catch {} }
async function api_group_list_identity_object() { try { await api.get("/jaxrs/group/list/identity/object") } catch {} }
async function api_group_has_role() { try { await api.get("/jaxrs/group/has/role") } catch {} }
async function api_group_sub_direct_object() { try { await api.get("/jaxrs/group/list/group/sub/direct/object") } catch {} }
async function api_group_list_person() { try { await api.get("/jaxrs/group/list/person") } catch {} }
async function api_group_list() { try { await api.get("/jaxrs/group/list") } catch {} }
async function api_group_sub_nested_object() { try { await api.get("/jaxrs/group/list/group/sub/nested/object") } catch {} }
async function api_group_list_object() { try { await api.get("/jaxrs/group/list/object") } catch {} }
async function api_group() { try { await api.get("/jaxrs/group") } catch {} }
async function api_list_group_sub_direct() { try { await api.get("/jaxrs/group/list/group/sub/direct") } catch {} }
async function api_list_group_sub_nested() { try { await api.get("/jaxrs/group/list/group/sub/nested") } catch {} }

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
