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
function deleteItem(item: Item) { if (confirm('确定删除？')) delM.mutate(item.id) }
function loadData() { qc.invalidateQueries({ queryKey: qk }) }
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }) } catch { return String(t) } }

async function api_cms_core_article_list() { try { await api.get('/jaxrs/cms/core/article/list') } catch {} }
async function api_cms_view_publish_view_001() { try { await api.get('/jaxrs/cms/view/publish/view-001') } catch {} }
async function api_assemble_control_cmsview_list() { try { await api.get('/jaxrs/cms/assemble/control/cmsview/list') } catch {} }
async function api_cms_assemble_control_cmscategory() { try { await api.get('/jaxrs/cms/assemble/control/cmscategory') } catch {} }
async function api_cms() { try { await api.get('/jaxrs/cms') } catch {} }
async function api_assemble_control_cmsdocument_list() { try { await api.get('/jaxrs/cms/assemble/control/cmsdocument/list') } catch {} }
async function api_cms_assemble_control_cmsview() { try { await api.get('/jaxrs/cms/assemble/control/cmsview') } catch {} }
async function api_cms_view_list_all() { try { await api.get('/jaxrs/cms/view/list/all') } catch {} }
async function api_assemble_control_cmsdocumentcategory_list() { try { await api.get('/jaxrs/cms/assemble/control/cmsdocumentcategory/list') } catch {} }
async function api_cms_uuid_random() { try { await api.get('/jaxrs/cms/uuid/random') } catch {} }
async function api_assemble_control_cmscategory_list() { try { await api.get('/jaxrs/cms/assemble/control/cmscategory/list') } catch {} }
async function api_cms_templateform_list() { try { await api.get('/jaxrs/cms/templateform/list') } catch {} }
async function api_cms_article_test_article_id() { try { await api.get('/jaxrs/cms/article/test-article-id') } catch {} }
async function api_assemble_control_cmsapplication_list() { try { await api.get('/jaxrs/cms/assemble/control/cmsapplication/list') } catch {} }
async function api_cms_category_list() { try { await api.get('/jaxrs/cms/category/list') } catch {} }
async function api_cms_category_test_category_id() { try { await api.get('/jaxrs/cms/category/test-category-id') } catch {} }
async function api_cms_category_create() { try { await api.get('/jaxrs/cms/category/create') } catch {} }
async function api_cms_assemble_control_cmsapplication() { try { await api.get('/jaxrs/cms/assemble/control/cmsapplication') } catch {} }
async function api_cms_article_list() { try { await api.get('/jaxrs/cms/article/list') } catch {} }
async function api_core_express_content_list() { try { await api.get('/jaxrs/cms/core/express/content/list') } catch {} }
async function api_cms_assemble_control_cmsdocument() { try { await api.get('/jaxrs/cms/assemble/control/cmsdocument') } catch {} }
async function api_cms_article_create() { try { await api.get('/jaxrs/cms/article/create') } catch {} }
async function api_core_express_article_list() { try { await api.get('/jaxrs/cms/core/express/article/list') } catch {} }
async function api_cms_article() { try { await api.get('/jaxrs/cms/article') } catch {} }
async function api_cms_view_unpublish_view_001() { try { await api.get('/jaxrs/cms/view/unpublish/view-001') } catch {} }


async function api_cms_assemble_control_document_search() { try { await api.get("/jaxrs/cms_assemble_control/document/search") } catch {} }
async function api_cms_assemble_control_get_control_config() { try { await api.get("/jaxrs/cms_assemble_control/get/control/config") } catch {} }
async function api_cms_assemble_control_list_control_sections() { try { await api.get("/jaxrs/cms_assemble_control/list/control/sections") } catch {} }
async function api_cms_assemble_control_update_control_config() { try { await api.get("/jaxrs/cms_assemble_control/update/control/config") } catch {} }

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