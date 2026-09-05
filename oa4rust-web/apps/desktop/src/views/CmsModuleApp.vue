<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>CMS模块管理</h1>
        <p class="subtitle">/jaxrs/cms/core/entity/module/list</p>
      </div>
      <button class="btn-primary" @click="showCreate=true">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">📦</div><p>暂无数据</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>标识</th><th>更新时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||item.label||'—' }}</td>
            <td class="mono">{{ item.flag||item.id||'—' }}</td>
            <td>{{ fmtTime(item.updateTime||item.createTime) }}</td>
            <td>
              <button class="btn-sm" @click="editItem(item)">编辑</button>
              <button class="btn-sm btn-del" @click="deleteItem(item)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="showCreate||showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit?'编辑':'新建' }}CMS模块管理</h3>
        <div class="form-group"><label>名称</label><input v-model="form.name" placeholder="名称" class="form-input" /></div>
        <div class="form-group"><label>标识</label><input v-model="form.flag" placeholder="唯一标识" class="form-input" /></div>
        <div class="form-group"><label>描述</label><textarea v-model="form.desc" rows="3" placeholder="描述" class="form-textarea"></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.name" @click="saveItem">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

interface Item { id:string; name?:string; label?:string; flag?:string; desc?:string; updateTime?:string; createTime?:string }

const search = ref(''), showCreate = ref(false), showEdit = ref(false), loading = ref(false)
const items = ref<Item[]>([]), form = ref<Partial<Item>>({}), editingId = ref<string|null>(null)
const qc = useQueryClient()

const ep = '/jaxrs/cms/core/entity/module/list';
const qk = ['cms_Module', 'list'];

const { data } = useQuery({ queryKey: qk, queryFn: async () => { loading.value = true; try { const r = await api.get(ep); return (r as any)?.data ?? [] } finally { loading.value = false } } })
items.value = data.value ?? []

const filtered = computed(() => search.value ? items.value.filter(i => (i.name||'').toLowerCase().includes(search.value.toLowerCase()) || (i.flag||'').toLowerCase().includes(search.value.toLowerCase())) : items.value)

function editItem(item: Item) { form.value = { ...item }; editingId.value = item.id; showEdit.value = true }
function closeModal() { showCreate.value = false; showEdit.value = false; form.value = {} }
const saveM = useMutation({ mutationFn: async (data: any) => { if (editingId.value) return api.put(ep + '/' + editingId.value, data); return api.post(ep, data) }, onSuccess: () => { qc.invalidateQueries({ queryKey: qk }); closeModal() } })
function saveItem() { if (form.value.name) saveM.mutate(form.value) }
const delM = useMutation({ mutationFn: async (id: string) => api.delete(ep + '/' + id), onSuccess: () => { qc.invalidateQueries({ queryKey: qk }) } })
function deleteItem(item: Item) { if (confirmMsg('确定删除？')) delM.mutate(item.id) }
function loadData() { qc.invalidateQueries({ queryKey: qk }) }
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }) } catch { return String(t) } }

const cms_core_article_list_ref = ref<any[]>([]);
const cms_core_article_list_q = useQuery({
  queryKey: ['cms_core_article_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/core/article/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_view_publish_view_001_ref = ref<any[]>([]);
const cms_view_publish_view_001_q = useQuery({
  queryKey: ['cms_view_publish_view_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/view/publish/view-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_cmsview_list_ref = ref<any[]>([]);
const assemble_control_cmsview_list_q = useQuery({
  queryKey: ['assemble_control_cmsview_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsview/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_assemble_control_cmscategory_ref = ref<any[]>([]);
const cms_assemble_control_cmscategory_q = useQuery({
  queryKey: ['cms_assemble_control_cmscategory'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmscategory"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_ref = ref<any[]>([]);
const cms_q = useQuery({
  queryKey: ['cms'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_cmsdocument_list_ref = ref<any[]>([]);
const assemble_control_cmsdocument_list_q = useQuery({
  queryKey: ['assemble_control_cmsdocument_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsdocument/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_assemble_control_cmsview_ref = ref<any[]>([]);
const cms_assemble_control_cmsview_q = useQuery({
  queryKey: ['cms_assemble_control_cmsview'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsview"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_view_list_all_ref = ref<any[]>([]);
const cms_view_list_all_q = useQuery({
  queryKey: ['cms_view_list_all'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/view/list/all"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_cmsdocumentcategory_list_ref = ref<any[]>([]);
const assemble_control_cmsdocumentcategory_list_q = useQuery({
  queryKey: ['assemble_control_cmsdocumentcategory_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsdocumentcategory/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_uuid_random_ref = ref<any[]>([]);
const cms_uuid_random_q = useQuery({
  queryKey: ['cms_uuid_random'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/uuid/random"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_cmscategory_list_ref = ref<any[]>([]);
const assemble_control_cmscategory_list_q = useQuery({
  queryKey: ['assemble_control_cmscategory_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmscategory/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_templateform_list_ref = ref<any[]>([]);
const cms_templateform_list_q = useQuery({
  queryKey: ['cms_templateform_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/templateform/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_article_test_article_id_ref = ref<any[]>([]);
const cms_article_test_article_id_q = useQuery({
  queryKey: ['cms_article_test_article_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/article/test-article-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_cmsapplication_list_ref = ref<any[]>([]);
const assemble_control_cmsapplication_list_q = useQuery({
  queryKey: ['assemble_control_cmsapplication_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsapplication/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_category_list_ref = ref<any[]>([]);
const cms_category_list_q = useQuery({
  queryKey: ['cms_category_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/category/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_category_test_category_id_ref = ref<any[]>([]);
const cms_category_test_category_id_q = useQuery({
  queryKey: ['cms_category_test_category_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/category/test-category-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_category_create_ref = ref<any[]>([]);
const cms_category_create_q = useQuery({
  queryKey: ['cms_category_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/category/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_assemble_control_cmsapplication_ref = ref<any[]>([]);
const cms_assemble_control_cmsapplication_q = useQuery({
  queryKey: ['cms_assemble_control_cmsapplication'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsapplication"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_article_list_ref = ref<any[]>([]);
const cms_article_list_q = useQuery({
  queryKey: ['cms_article_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/article/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_express_content_list_ref = ref<any[]>([]);
const core_express_content_list_q = useQuery({
  queryKey: ['core_express_content_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/core/express/content/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_assemble_control_cmsdocument_ref = ref<any[]>([]);
const cms_assemble_control_cmsdocument_q = useQuery({
  queryKey: ['cms_assemble_control_cmsdocument'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/cmsdocument"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_article_create_ref = ref<any[]>([]);
const cms_article_create_q = useQuery({
  queryKey: ['cms_article_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/article/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_express_article_list_ref = ref<any[]>([]);
const core_express_article_list_q = useQuery({
  queryKey: ['core_express_article_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/core/express/article/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_article_ref = ref<any[]>([]);
const cms_article_q = useQuery({
  queryKey: ['cms_article'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/article"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const cms_view_unpublish_view_001_ref = ref<any[]>([]);
const cms_view_unpublish_view_001_q = useQuery({
  queryKey: ['cms_view_unpublish_view_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/view/unpublish/view-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_cms_asse_559_data = ref<any[]>([]);
const { data: api_cms_asse_559_q } = useQuery({queryKey: ['api_cms_asse_559', '/jaxrs/cms_assemble_control/document/search'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/document/search"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cms_asse_559_q, (v) => { api_cms_asse_559_data.value = v ?? []; });
const api_cms_asse_804_data = ref<any[]>([]);
const { data: api_cms_asse_804_q } = useQuery({queryKey: ['api_cms_asse_804', '/jaxrs/cms_assemble_control/get/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/get/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cms_asse_804_q, (v) => { api_cms_asse_804_data.value = v ?? []; });
const api_cms_asse_862_data = ref<any[]>([]);
const { data: api_cms_asse_862_q } = useQuery({queryKey: ['api_cms_asse_862', '/jaxrs/cms_assemble_control/list/control/sections'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/list/control/sections"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cms_asse_862_q, (v) => { api_cms_asse_862_data.value = v ?? []; });
const api_cms_asse_501_data = ref<any[]>([]);
const { data: api_cms_asse_501_q } = useQuery({queryKey: ['api_cms_asse_501', '/jaxrs/cms_assemble_control/update/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/update/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cms_asse_501_q, (v) => { api_cms_asse_501_data.value = v ?? []; });

const api_jaxrs_an_996_data = ref<any[]>([]);
const { data: api_jaxrs_an_996_q } = useQuery({queryKey: ['api_jaxrs_an_996', '/jaxrs/anonymous/document/filter/list/i-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/document/filter/list/i-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_an_996_q, (v) => { api_jaxrs_an_996_data.value = v ?? []; });
const api_jaxrs_an_237_data = ref<any[]>([]);
const { data: api_jaxrs_an_237_q } = useQuery({queryKey: ['api_jaxrs_an_237', '/jaxrs/anonymous/document/filter/list/p-1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/document/filter/list/p-1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_an_237_q, (v) => { api_jaxrs_an_237_data.value = v ?? []; });
const api_jaxrs_an_787_data = ref<any[]>([]);
const { data: api_jaxrs_an_787_q } = useQuery({queryKey: ['api_jaxrs_an_787', '/jaxrs/anonymous/form/v2/lookup/document/d-1/mobile'], queryFn: async () => { try { const r = await api.get("/jaxrs/anonymous/form/v2/lookup/document/d-1/mobile"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_an_787_q, (v) => { api_jaxrs_an_787_data.value = v ?? []; });
const api_jaxrs_cm_42_data = ref<any[]>([]);
const { data: api_jaxrs_cm_42_q } = useQuery({queryKey: ['api_jaxrs_cm_42', '/jaxrs/cms/assemble/control/appinfo/alias/alias'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms/assemble/control/appinfo/alias/alias"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cm_42_q, (v) => { api_jaxrs_cm_42_data.value = v ?? []; });
const api_jaxrs_cms_assemb_820_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_820_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_820', '/jaxrs/cms/assemble/control/categoryinfo/alias/alias'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms/assemble/control/categoryinfo/alias/alias"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_820_q, (v) => { api_jaxrs_cms_assemb_820_data.value = v ?? []; });
const api_jaxrs_cms_assemb_441_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_441_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_441', '/jaxrs/cms/assemble/control/document/cipher/publish/content'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms/assemble/control/document/cipher/publish/content"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_441_q, (v) => { api_jaxrs_cms_assemb_441_data.value = v ?? []; });
const jaxrs_cms_assemble_control_document_cipher_publish_content_mockputtopost_ref = ref<any[]>([]);
const jaxrs_cms_assemble_control_document_cipher_publish_content_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_cms_assemble_control_document_cipher_publish_content_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms/assemble/control/document/cipher/publish/content/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_cms_assemb_446_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_446_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_446', '/jaxrs/cms/assemble/control/fileinfo/upload/with/url'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms/assemble/control/fileinfo/upload/with/url"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_446_q, (v) => { api_jaxrs_cms_assemb_446_data.value = v ?? []; });
const api_jaxrs_cms_contro_561_data = ref<any[]>([]);
const { data: api_jaxrs_cms_contro_561_q } = useQuery({queryKey: ['api_jaxrs_cms_contro_561', '/jaxrs/cms/control/any/route'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms/control/any/route"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_contro_561_q, (v) => { api_jaxrs_cms_contro_561_data.value = v ?? []; });
const api_jaxrs_cms_assemb_881_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_881_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_881', '/jaxrs/cms_assemble_control/anonymous/document/filter/list/id/next/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/anonymous/document/filter/list/id/next/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_881_q, (v) => { api_jaxrs_cms_assemb_881_data.value = v ?? []; });
const jaxrs_cms_assemble_control_anonymous_document_filter_list_id_next_count_mockputtopost_ref = ref<any[]>([]);
const jaxrs_cms_assemble_control_anonymous_document_filter_list_id_next_count_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_cms_assemble_control_anonymous_document_filter_list_id_next_count_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms_assemble_control/anonymous/document/filter/list/id/next/count/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_cms_assemb_479_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_479_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_479', '/jaxrs/cms_assemble_control/anonymous/document/filter/list/page/size/size'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/anonymous/document/filter/list/page/size/size"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_479_q, (v) => { api_jaxrs_cms_assemb_479_data.value = v ?? []; });
const jaxrs_cms_assemble_control_anonymous_document_filter_list_page_size_size_mockputtopost_ref = ref<any[]>([]);
const jaxrs_cms_assemble_control_anonymous_document_filter_list_page_size_size_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_cms_assemble_control_anonymous_document_filter_list_page_size_size_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms_assemble_control/anonymous/document/filter/list/page/size/size/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_cms_assemb_716_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_716_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_716', '/jaxrs/cms_assemble_control/appinfo/filter/list/id/next/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/appinfo/filter/list/id/next/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_716_q, (v) => { api_jaxrs_cms_assemb_716_data.value = v ?? []; });
const jaxrs_cms_assemble_control_appinfo_filter_list_id_next_count_mockputtopost_ref = ref<any[]>([]);
const jaxrs_cms_assemble_control_appinfo_filter_list_id_next_count_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_cms_assemble_control_appinfo_filter_list_id_next_count_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms_assemble_control/appinfo/filter/list/id/next/count/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_cms_assemb_879_data = ref<any[]>([]);
const { data: api_jaxrs_cms_assemb_879_q } = useQuery({queryKey: ['api_jaxrs_cms_assemb_879', '/jaxrs/cms_assemble_control/appinfo/filter/list/id/prev/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/cms_assemble_control/appinfo/filter/list/id/prev/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_cms_assemb_879_q, (v) => { api_jaxrs_cms_assemb_879_data.value = v ?? []; });
const jaxrs_cms_assemble_control_appinfo_filter_list_id_prev_count_mockputtopost_ref = ref<any[]>([]);
const jaxrs_cms_assemble_control_appinfo_filter_list_id_prev_count_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_cms_assemble_control_appinfo_filter_list_id_prev_count_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/cms_assemble_control/appinfo/filter/list/id/prev/count/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
</script>
<style scoped>
.crud-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:flex-start;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.btn-primary{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.content-panel{padding:16px}
.toolbar{display:flex;gap:8px;margin-bottom:16px}
.search-input{flex:1;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.btn-refresh{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-weight:600;font-size:12px;text-transform:uppercase}
.data-table tr:hover{background:var(--bg-hover)}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.btn-del:hover{background:var(--color-danger-soft)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:480px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{display:block;font-size:12px;color:var(--text-muted);margin-bottom:4px}
.form-input,.form-textarea{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>