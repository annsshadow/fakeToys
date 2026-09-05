<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>热帖管理</h1>
      <p class="subtitle">/jaxrs/hotpic/core/entity/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-toolbar">
        <input v-model="keyword" placeholder="搜索热帖..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">🔥</div><p>暂无热帖数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card glass-card">
            <div class="ic">🔥</div>
            <div class="ib">
              <div class="it">{{ item.title || item.name || '未命名' }}</div>
              <div class="im">{{ item.content || item.desc || item.description || '' }}</div>
              <div class="meta">views: {{ item.views || 0 }} | likes: {{ item.likes || 0 }}</div>
            </div>
            <button class="btn-del" @click.stop="onDelete(item)">删除</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

const keyword = ref('')
const loading = ref(false)
const items = ref<any[]>([])

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '今日热门', value: items.value.filter(i => i.isHot).length, color: 'var(--color-warning)' },
  { label: '精华', value: items.value.filter(i => i.isCream).length, color: 'var(--color-success)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-error)' },
])

async function doSearch() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/hotpic/core/entity/list')
    items.value = r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

async function onDelete(item: any) {
  if (!confirmMsg(`确定删除热帖「${item.title || item.id}」？`)) return
  try {
    await api.delete(`/jaxrs/hotpic/core/entity/delete/${item.id}`)
    items.value = items.value.filter(i => i.id !== item.id)
  } catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '未知错误')) }
}

doSearch()

const user_hotpic_CMS_doc_123_ref = ref<any[]>([]);
const user_hotpic_CMS_doc_123_q = useQuery({
  queryKey: ['user_hotpic_CMS_doc_123'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/user/hotpic/CMS/doc-123"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_save_hotpic_ref = ref<any[]>([]);
const hotpic_save_hotpic_q = useQuery({
  queryKey: ['hotpic_save_hotpic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/save/hotpic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_delete_hotpic_ref = ref<any[]>([]);
const hotpic_delete_hotpic_q = useQuery({
  queryKey: ['hotpic_delete_hotpic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/delete/hotpic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_core_entity_create_ref = ref<any[]>([]);
const hotpic_core_entity_create_q = useQuery({
  queryKey: ['hotpic_core_entity_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/core/entity/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_core_list_ref = ref<any[]>([]);
const hotpic_core_list_q = useQuery({
  queryKey: ['hotpic_core_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/core/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_user_hotpic_hotpic_001_ref = ref<any[]>([]);
const hotpic_user_hotpic_hotpic_001_q = useQuery({
  queryKey: ['hotpic_user_hotpic_hotpic_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/user/hotpic/hotpic-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_assemble_list_ref = ref<any[]>([]);
const hotpic_assemble_list_q = useQuery({
  queryKey: ['hotpic_assemble_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/assemble/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_create_hotpic_ref = ref<any[]>([]);
const hotpic_create_hotpic_q = useQuery({
  queryKey: ['hotpic_create_hotpic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/create/hotpic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_upload_ref = ref<any[]>([]);
const hotpic_upload_q = useQuery({
  queryKey: ['hotpic_upload'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/upload"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_delete_hotpic_test_001_ref = ref<any[]>([]);
const core_entity_delete_hotpic_test_001_q = useQuery({
  queryKey: ['core_entity_delete_hotpic_test_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/core/entity/delete/hotpic-test-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const user_hotpic_exists_check_ref = ref<any[]>([]);
const user_hotpic_exists_check_q = useQuery({
  queryKey: ['user_hotpic_exists_check'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/user/hotpic/exists/check"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_list_hotpics_ref = ref<any[]>([]);
const hotpic_list_hotpics_q = useQuery({
  queryKey: ['hotpic_list_hotpics'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/list/hotpics"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_assemble_control_config_ref = ref<any[]>([]);
const hotpic_assemble_control_config_q = useQuery({
  queryKey: ['hotpic_assemble_control_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/assemble/control/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_get_hotpic_hotpic_001_ref = ref<any[]>([]);
const hotpic_get_hotpic_hotpic_001_q = useQuery({
  queryKey: ['hotpic_get_hotpic_hotpic_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/get/hotpic/hotpic-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_user_hotpic_ref = ref<any[]>([]);
const assemble_control_user_hotpic_q = useQuery({
  queryKey: ['assemble_control_user_hotpic'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/assemble/control/user/hotpic"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const hotpic_list_ref = ref<any[]>([]);
const hotpic_list_q = useQuery({
  queryKey: ['hotpic_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/hotpic/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_hotpic_a_81_data = ref<any[]>([]);
const { data: api_hotpic_a_81_q } = useQuery({queryKey: ['api_hotpic_a_81', '/jaxrs/hotpic_assemble_control'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_81_q, (v) => { api_hotpic_a_81_data.value = v ?? []; });
const api_hotpic_a_902_data = ref<any[]>([]);
const { data: api_hotpic_a_902_q } = useQuery({queryKey: ['api_hotpic_a_902', '/jaxrs/hotpic_assemble_control/save/hotpic'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/save/hotpic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_902_q, (v) => { api_hotpic_a_902_data.value = v ?? []; });
const api_hotpic_a_553_data = ref<any[]>([]);
const { data: api_hotpic_a_553_q } = useQuery({queryKey: ['api_hotpic_a_553', '/jaxrs/hotpic_assemble_control/list/hotpics'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/list/hotpics"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_553_q, (v) => { api_hotpic_a_553_data.value = v ?? []; });
const api_hotpic_a_938_data = ref<any[]>([]);
const { data: api_hotpic_a_938_q } = useQuery({queryKey: ['api_hotpic_a_938', '/jaxrs/hotpic_assemble_control/cipher/hotpic/id'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/cipher/hotpic/id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_938_q, (v) => { api_hotpic_a_938_data.value = v ?? []; });
const api_hotpic_a_451_data = ref<any[]>([]);
const { data: api_hotpic_a_451_q } = useQuery({queryKey: ['api_hotpic_a_451', '/jaxrs/hotpic_assemble_control/create/hotpic'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/create/hotpic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_451_q, (v) => { api_hotpic_a_451_data.value = v ?? []; });
const api_hotpic_a_267_data = ref<any[]>([]);
const { data: api_hotpic_a_267_q } = useQuery({queryKey: ['api_hotpic_a_267', '/jaxrs/hotpic_assemble_control/user/hotpic/changeTitle'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/changeTitle"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_267_q, (v) => { api_hotpic_a_267_data.value = v ?? []; });
const api_hotpic_a_853_data = ref<any[]>([]);
const { data: api_hotpic_a_853_q } = useQuery({queryKey: ['api_hotpic_a_853', '/jaxrs/hotpic_assemble_control/list/control/applications'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/list/control/applications"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_853_q, (v) => { api_hotpic_a_853_data.value = v ?? []; });
const api_hotpic_a_727_data = ref<any[]>([]);
const { data: api_hotpic_a_727_q } = useQuery({queryKey: ['api_hotpic_a_727', '/jaxrs/hotpic_assemble_control/delete/hotpic'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/delete/hotpic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_727_q, (v) => { api_hotpic_a_727_data.value = v ?? []; });
const api_user_hot_589_data = ref<any[]>([]);
const { data: api_user_hot_589_q } = useQuery({queryKey: ['api_user_hot_589', '/jaxrs/hotpic_assemble_control/user/hotpic/exists/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/exists/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_user_hot_589_q, (v) => { api_user_hot_589_data.value = v ?? []; });
const api_hotpic_a_155_data = ref<any[]>([]);
const { data: api_hotpic_a_155_q } = useQuery({queryKey: ['api_hotpic_a_155', '/jaxrs/hotpic_assemble_control/get/hotpic'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/get/hotpic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_155_q, (v) => { api_hotpic_a_155_data.value = v ?? []; });
const api_hotpic_a_48_data = ref<any[]>([]);
const { data: api_hotpic_a_48_q } = useQuery({queryKey: ['api_hotpic_a_48', '/jaxrs/hotpic_assemble_control/update/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/update/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_48_q, (v) => { api_hotpic_a_48_data.value = v ?? []; });
const api_hotpic_a_594_data = ref<any[]>([]);
const { data: api_hotpic_a_594_q } = useQuery({queryKey: ['api_hotpic_a_594', '/jaxrs/hotpic_assemble_control/user/hotpic/id'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_594_q, (v) => { api_hotpic_a_594_data.value = v ?? []; });
const api_hotpic_a_441_data = ref<any[]>([]);
const { data: api_hotpic_a_441_q } = useQuery({queryKey: ['api_hotpic_a_441', '/jaxrs/hotpic_assemble_control/get/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/get/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_441_q, (v) => { api_hotpic_a_441_data.value = v ?? []; });
const api_user_hot_763_data = ref<any[]>([]);
const { data: api_user_hot_763_q } = useQuery({queryKey: ['api_user_hot_763', '/jaxrs/hotpic_assemble_control/user/hotpic/application/infoId'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/user/hotpic/application/infoId"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_user_hot_763_q, (v) => { api_user_hot_763_data.value = v ?? []; });
const api_hotpic_a_799_data = ref<any[]>([]);
const { data: api_hotpic_a_799_q } = useQuery({queryKey: ['api_hotpic_a_799', '/jaxrs/hotpic_assemble_control/list/control/panels'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/list/control/panels"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_hotpic_a_799_q, (v) => { api_hotpic_a_799_data.value = v ?? []; });


const api_cipher_h_274_data = ref<any[]>([]);
const { data: api_cipher_h_274_q } = useQuery({queryKey: ['api_cipher_h_274', '/jaxrs/hotpic_assemble_control/cipher/hotpic/bbs/id'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/cipher/hotpic/bbs/id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cipher_h_274_q, (v) => { api_cipher_h_274_data.value = v ?? []; });
const api_cipher_h_765_data = ref<any[]>([]);
const { data: api_cipher_h_765_q } = useQuery({queryKey: ['api_cipher_h_765', '/jaxrs/hotpic_assemble_control/cipher/hotpic/cms/id'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic_assemble_control/cipher/hotpic/cms/id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cipher_h_765_q, (v) => { api_cipher_h_765_data.value = v ?? []; });


const api_control__12_data = ref<any[]>([]);
const { data: api_control__12_q } = useQuery({queryKey: ['api_control__12', '/jaxrs/hotpic/assemble/control/user/hotpic/changeTitle'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/user/hotpic/changeTitle"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__12_q, (v) => { api_control__12_data.value = v ?? []; });
const api_control__542_data = ref<any[]>([]);
const { data: api_control__542_q } = useQuery({queryKey: ['api_control__542', '/jaxrs/hotpic/assemble/control/update/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/update/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__542_q, (v) => { api_control__542_data.value = v ?? []; });
const api_control_list_con_641_data = ref<any[]>([]);
const { data: api_control_list_con_641_q } = useQuery({queryKey: ['api_control_list_con_641', '/jaxrs/hotpic/assemble/control/list/control/applications'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/list/control/applications"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_list_con_641_q, (v) => { api_control_list_con_641_data.value = v ?? []; });
const api_control_user_hot_220_data = ref<any[]>([]);
const { data: api_control_user_hot_220_q } = useQuery({queryKey: ['api_control_user_hot_220', '/jaxrs/hotpic/assemble/control/user/hotpic/hotpic-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/user/hotpic/hotpic-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_user_hot_220_q, (v) => { api_control_user_hot_220_data.value = v ?? []; });
const api_control_list_con_584_data = ref<any[]>([]);
const { data: api_control_list_con_584_q } = useQuery({queryKey: ['api_control_list_con_584', '/jaxrs/hotpic/assemble/control/list/control/panels'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/list/control/panels"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_list_con_584_q, (v) => { api_control_list_con_584_data.value = v ?? []; });

const api_jaxrs_hotpic_ass_634_data = ref<any[]>([]);
const { data: api_jaxrs_hotpic_ass_634_q } = useQuery({queryKey: ['api_jaxrs_hotpic_ass_634', '/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/1/count/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/1/count/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_hotpic_ass_634_q, (v) => { api_jaxrs_hotpic_ass_634_data.value = v ?? []; });
const api_jaxrs_hotpic_ass_799_data = ref<any[]>([]);
const { data: api_jaxrs_hotpic_ass_799_q } = useQuery({queryKey: ['api_jaxrs_hotpic_ass_799', '/jaxrs/hotpic/assemble/control/user/hotpic/exists/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/user/hotpic/exists/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_hotpic_ass_799_q, (v) => { api_jaxrs_hotpic_ass_799_data.value = v ?? []; });
const api_jaxrs_hotpic_ass_316_data = ref<any[]>([]);
const { data: api_jaxrs_hotpic_ass_316_q } = useQuery({queryKey: ['api_jaxrs_hotpic_ass_316', '/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/1/count/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/1/count/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_hotpic_ass_316_q, (v) => { api_jaxrs_hotpic_ass_316_data.value = v ?? []; });
const api_jaxrs_hotpic_cor_130_data = ref<any[]>([]);
const { data: api_jaxrs_hotpic_cor_130_q } = useQuery({queryKey: ['api_jaxrs_hotpic_cor_130', '/jaxrs/hotpic/core/entity/exists/check/app/app-001/info-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/core/entity/exists/check/app/app-001/info-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_hotpic_cor_130_q, (v) => { api_jaxrs_hotpic_cor_130_data.value = v ?? []; });
const api_jaxrs_hotpic_cor_93_data = ref<any[]>([]);
const { data: api_jaxrs_hotpic_cor_93_q } = useQuery({queryKey: ['api_jaxrs_hotpic_cor_93', '/jaxrs/hotpic/core/entity/list/by/app/app-001/info-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/hotpic/core/entity/list/by/app/app-001/info-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_hotpic_cor_93_q, (v) => { api_jaxrs_hotpic_cor_93_data.value = v ?? []; });
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.list-toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-del{padding:4px 12px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;transition:all var(--transition-fast)}
.btn-del:hover{background:var(--color-error);color:#fff}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(240px,1fr));gap:12px}
.item-card{display:flex;align-items:flex-start;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px;flex-shrink:0}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
</style>
