<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>单元管理</h1>
      <p class="subtitle">/jaxrs/unit/* — 组织单元与身份映射</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="keyword" placeholder="搜索单元..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-primary" @click="loadUnits">刷新全部</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="units.length===0" class="empty"><div class="ei">🏗️</div><p>暂无单元数据</p></div>
        <div v-else class="item-grid">
          <div v-for="u in units" :key="u.id" class="item-card glass-card">
            <div class="ic">🏗️</div>
            <div class="ib">
              <div class="it">{{ u.name || u.title || '未命名单元' }}</div>
              <div class="im">flag: {{ u.flag || u.unitFlag || u.id }}</div>
              <div class="meta">{{ u.desc || u.description || '' }}</div>
            </div>
            <button class="btn-sm" @click="checkUnit(u)">验证</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

type UnitItem = { id: string; name?: string; title?: string; flag?: string; unitFlag?: string; desc?: string; description?: string }

const keyword = ref('')
const loading = ref(false)
const units = ref<UnitItem[]>([])

async function doSearch() {
  loading.value = true
  try {
    // Try search by keyword
    const r = await api.get('/jaxrs/unit/list')
    units.value = r.data ?? []
  } catch { units.value = [] } finally { loading.value = false }
}

async function checkUnit(u: UnitItem) {
  try {
    await api.get(`/jaxrs/unit/check/${u.flag || u.id}`)
  } catch (e: any) { toast.error('验证失败: : ' + (e?.message ?? '')) }
}

loadUnits()

const list_person_sup_nested_ref = ref<any[]>([]);
const list_person_sup_nested_q = useQuery({
  queryKey: ['list_person_sup_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/person/sup/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_level_ref = ref<any[]>([]);
const unit_list_level_q = useQuery({
  queryKey: ['unit_list_level'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/level"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_unit_tree_ref = ref<any[]>([]);
const unit_list_unit_tree_q = useQuery({
  queryKey: ['unit_list_unit_tree'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unit/tree"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_identity_ref = ref<any[]>([]);
const unit_list_identity_q = useQuery({
  queryKey: ['unit_list_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_check_ref = ref<any[]>([]);
const unit_check_q = useQuery({
  queryKey: ['unit_check'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/check"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_unitattribute_object_ref = ref<any[]>([]);
const unit_list_unitattribute_object_q = useQuery({
  queryKey: ['unit_list_unitattribute_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unitattribute/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_identity_type_ref = ref<any[]>([]);
const unit_identity_type_q = useQuery({
  queryKey: ['unit_identity_type'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/identity/type"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_identity_object_ref = ref<any[]>([]);
const unit_list_identity_object_q = useQuery({
  queryKey: ['unit_list_identity_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/identity/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_ref = ref<any[]>([]);
const unit_q = useQuery({
  queryKey: ['unit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_unit_sup_nested_ref = ref<any[]>([]);
const list_unit_sup_nested_q = useQuery({
  queryKey: ['list_unit_sup_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unit/sup/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const check_unit_has_unit_ref = ref<any[]>([]);
const check_unit_has_unit_q = useQuery({
  queryKey: ['check_unit_has_unit'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/check/unit/has/unit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_person_ref = ref<any[]>([]);
const unit_list_person_q = useQuery({
  queryKey: ['unit_list_person'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/person"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_type_dept_object_ref = ref<any[]>([]);
const list_type_dept_object_q = useQuery({
  queryKey: ['list_type_dept_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/type/dept/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_object_ref = ref<any[]>([]);
const unit_list_object_q = useQuery({
  queryKey: ['unit_list_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const check_unit_has_identity_ref = ref<any[]>([]);
const check_unit_has_identity_q = useQuery({
  queryKey: ['check_unit_has_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/check/unit/has/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_unitattribute_ref = ref<any[]>([]);
const unit_list_unitattribute_q = useQuery({
  queryKey: ['unit_list_unitattribute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unitattribute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_unit_sub_nested_ref = ref<any[]>([]);
const list_unit_sub_nested_q = useQuery({
  queryKey: ['list_unit_sub_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unit/sub/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_identity_level_ref = ref<any[]>([]);
const unit_identity_level_q = useQuery({
  queryKey: ['unit_identity_level'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/identity/level"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_identity_type_object_ref = ref<any[]>([]);
const unit_identity_type_object_q = useQuery({
  queryKey: ['unit_identity_type_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/identity/type/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_level_name_object_ref = ref<any[]>([]);
const list_level_name_object_q = useQuery({
  queryKey: ['list_level_name_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/level/name/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_unitduty_object_ref = ref<any[]>([]);
const unit_list_unitduty_object_q = useQuery({
  queryKey: ['unit_list_unitduty_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unitduty/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_identity_ref = ref<any[]>([]);
const unit_identity_q = useQuery({
  queryKey: ['unit_identity'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/identity"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const list_unit_sub_direct_ref = ref<any[]>([]);
const list_unit_sub_direct_q = useQuery({
  queryKey: ['list_unit_sub_direct'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/unit/sub/direct"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_person_object_ref = ref<any[]>([]);
const unit_list_person_object_q = useQuery({
  queryKey: ['unit_list_person_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/person/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const unit_list_all_object_ref = ref<any[]>([]);
const unit_list_all_object_q = useQuery({
  queryKey: ['unit_list_all_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/unit/list/all/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const api_jaxrs_or_150_data = ref<any[]>([]);
const { data: api_jaxrs_or_150_q } = useQuery({queryKey: ['api_jaxrs_or_150', '/jaxrs/organization/assemble/authentication/authentication/oauth/dingding/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/oauth/dingding/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_or_150_q, (v) => { api_jaxrs_or_150_data.value = v ?? []; });
const api_jaxrs_or_60_data = ref<any[]>([]);
const { data: api_jaxrs_or_60_q } = useQuery({queryKey: ['api_jaxrs_or_60', '/jaxrs/organization/assemble/authentication/authentication/oauth/qywx/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/authentication/authentication/oauth/qywx/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_or_60_q, (v) => { api_jaxrs_or_60_data.value = v ?? []; });
const jaxrs_organization_assemble_authentication_sso_encrypt_client_u2c_key_u2key_1234_95d5ba_ref = ref<any[]>([]);
const jaxrs_organization_assemble_authentication_sso_encrypt_client_u2c_key_u2key_1234_95d5ba_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_authentication_sso_encrypt_client_u2c_key_u2key_1234_95d5ba'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/authentication/sso/encrypt/client/u2c/key/u2key-1234567890-abcdef-/credential/user@P"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_group_list_like_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_group_list_like_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_group_list_like_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/group/list/like/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_or_174_data = ref<any[]>([]);
const { data: api_jaxrs_or_174_q } = useQuery({queryKey: ['api_jaxrs_or_174', '/jaxrs/organization/assemble/control/group/list/like/pinyin'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/group/list/like/pinyin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_or_174_q, (v) => { api_jaxrs_or_174_data.value = v ?? []; });
const jaxrs_organization_assemble_control_group_list_like_pinyin_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_group_list_like_pinyin_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_group_list_like_pinyin_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_group_list_pinyininitial_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_group_list_pinyininitial_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_group_list_pinyininitial_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_identity_list_like_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_identity_list_like_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_identity_list_like_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/identity/list/like/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_494_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_494_q } = useQuery({queryKey: ['api_jaxrs_organizati_494', '/jaxrs/organization/assemble/control/identity/list/like/pinyin'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/identity/list/like/pinyin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_494_q, (v) => { api_jaxrs_organizati_494_data.value = v ?? []; });
const jaxrs_organization_assemble_control_identity_list_like_pinyin_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_identity_list_like_pinyin_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_identity_list_like_pinyin_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_identity_list_pinyininitial_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_identity_list_pinyininitial_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_identity_list_pinyininitial_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_person_list_like_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_person_list_like_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_person_list_like_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/person/list/like/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_346_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_346_q } = useQuery({queryKey: ['api_jaxrs_organizati_346', '/jaxrs/organization/assemble/control/person/list/like/pinyin'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/person/list/like/pinyin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_346_q, (v) => { api_jaxrs_organizati_346_data.value = v ?? []; });
const jaxrs_organization_assemble_control_person_list_like_pinyin_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_person_list_like_pinyin_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_person_list_like_pinyin_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/person/list/like/pinyin/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_person_list_pinyininitial_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_person_list_pinyininitial_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_person_list_pinyininitial_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/person/list/pinyininitial/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_256_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_256_q } = useQuery({queryKey: ['api_jaxrs_organizati_256', '/jaxrs/organization/assemble/control/role/list/0/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/0/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_256_q, (v) => { api_jaxrs_organizati_256_data.value = v ?? []; });
const jaxrs_organization_assemble_control_role_list_like_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_role_list_like_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_role_list_like_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/like/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_285_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_285_q } = useQuery({queryKey: ['api_jaxrs_organizati_285', '/jaxrs/organization/assemble/control/role/list/like/pinyin'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/like/pinyin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_285_q, (v) => { api_jaxrs_organizati_285_data.value = v ?? []; });
const jaxrs_organization_assemble_control_role_list_like_pinyin_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_role_list_like_pinyin_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_role_list_like_pinyin_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_824_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_824_q } = useQuery({queryKey: ['api_jaxrs_organizati_824', '/jaxrs/organization/assemble/control/unit/list/0/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/0/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_824_q, (v) => { api_jaxrs_organizati_824_data.value = v ?? []; });
const api_jaxrs_organizati_923_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_923_q } = useQuery({queryKey: ['api_jaxrs_organizati_923', '/jaxrs/organization/assemble/control/unit/list/control/top'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/control/top"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_923_q, (v) => { api_jaxrs_organizati_923_data.value = v ?? []; });
const jaxrs_organization_assemble_control_unit_list_like_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_like_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_like_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/like/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_439_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_439_q } = useQuery({queryKey: ['api_jaxrs_organizati_439', '/jaxrs/organization/assemble/control/unit/list/like/pinyin'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/like/pinyin"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_439_q, (v) => { api_jaxrs_organizati_439_data.value = v ?? []; });
const jaxrs_organization_assemble_control_unit_list_like_pinyin_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_like_pinyin_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_like_pinyin_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/like/pinyin/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_unit_list_pinyininitial_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_pinyininitial_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_pinyininitial_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/pinyininitial/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_unit_list_test_unit_sub_nested_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_test_unit_sub_nested_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_test_unit_sub_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/test-unit/sub/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_unit_list_test_unit_sup_nested_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_test_unit_sup_nested_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_test_unit_sup_nested'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/test-unit/sup/nested"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_organization_assemble_control_unit_list_test_unit_sup_nested_type_company_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_test_unit_sup_nested_type_company_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_test_unit_sup_nested_type_company'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/test-unit/sup/nested/type/company"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_organizati_842_data = ref<any[]>([]);
const { data: api_jaxrs_organizati_842_q } = useQuery({queryKey: ['api_jaxrs_organizati_842', '/jaxrs/organization/assemble/control/unit/list/unit/type'], queryFn: async () => { try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/unit/type"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_organizati_842_q, (v) => { api_jaxrs_organizati_842_data.value = v ?? []; });
const jaxrs_organization_assemble_control_unit_list_unit_type_mockputtopost_ref = ref<any[]>([]);
const jaxrs_organization_assemble_control_unit_list_unit_type_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_organization_assemble_control_unit_list_unit_type_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/organization/assemble/control/unit/list/unit/type/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px}
.meta{font-size:11px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
</style>
