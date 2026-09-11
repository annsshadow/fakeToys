<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div><h1>公共组件库</h1><p class="subtitle">/jaxrs/general/assemble/control/list</p></div>
      <button class="btn-primary" @click="showCreate=true">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar"><input v-model="search" placeholder="搜索..." class="search-input" /><button class="btn-refresh" @click="loadData">🔄 刷新</button></div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">📦</div><p>暂无数据</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>标识</th><th>更新时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||item.label||'—' }}</td>
            <td class="mono">{{ item.flag||item.id||'—' }}</td>
            <td>{{ fmtTime(item.updateTime||item.createTime) }}</td>
            <td><button class="btn-sm" @click="editItem(item)">编辑</button><button class="btn-sm btn-del" @click="deleteItem(item)">删除</button></td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="showCreate||showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit?'编辑':'新建' }}公共组件库</h3>
        <div class="form-group"><label>名称</label><input v-model="form.name" class="form-input" /></div>
        <div class="form-group"><label>标识</label><input v-model="form.flag" class="form-input" /></div>
        <div class="modal-actions"><button class="btn-cancel" @click="closeModal">取消</button><button class="btn-save" :disabled="!form.name" @click="saveItem">保存</button></div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'

interface Item {
  id: string
  name?: string
  label?: string
  flag?: string
  updateTime?: string
  createTime?: string
}
const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false)
const items = ref<Item[]>([]),
  form = ref<Partial<Item>>({}),
  editingId = ref<string | null>(null)
const qc = useQueryClient()
const ep = '/jaxrs/general/assemble/control/list'
const qk = ['Common', 'list']
const { data } = useQuery({
  queryKey: qk,
  queryFn: async () => {
    loading.value = true
    try {
      const r = await api.get(ep)
      return (r as any)?.data ?? []
    } finally {
      loading.value = false
    }
  },
})
items.value = data.value ?? []
const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.name || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.flag || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)
function editItem(item: Item) {
  form.value = { ...item }
  editingId.value = item.id
  showEdit.value = true
}
function closeModal() {
  showCreate.value = false
  showEdit.value = false
  form.value = {}
}
const saveM = useMutation({
  mutationFn: async (d: any) => (editingId.value ? api.put(ep + '/' + editingId.value, d) : api.post(ep, d)),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
    closeModal()
  },
})
function saveItem() {
  if (form.value.name) saveM.mutate(form.value)
}
const delM = useMutation({
  mutationFn: async (id: string) => api.delete(ep + '/' + id),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
function deleteItem(item: Item) {
  if (confirmMsg('确定删除？')) delM.mutate(item.id)
}
function loadData() {
  qc.invalidateQueries({ queryKey: qk })
}
function fmtTime(t?: string) {
  if (!t) return ''
  try {
    return new Date(t).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
  } catch {
    return String(t)
  }
}

const assemble_control_qrcode_list_ref = ref<any[]>([])
const assemble_control_securityclearance_enable_ref = ref<any[]>([])
const general_assemble_control_invoice_ref = ref<any[]>([])
const general_core_list_ref = ref<any[]>([])
const general_assemble_control_office_ref = ref<any[]>([])
const general_dict_item_create_ref = ref<any[]>([])
const general_area_list_ref = ref<any[]>([])
const general_config_list_ref = ref<any[]>([])
const assemble_control_attendscope_list_ref = ref<any[]>([])
const assemble_control_excel_upload_ref = ref<any[]>([])
const assemble_control_area_create_ref = ref<any[]>([])
const dict_item_list_test_dict_id_ref = ref<any[]>([])
const general_assemble_config_ref = ref<any[]>([])
const assemble_control_permissions_mind_ref = ref<any[]>([])
const general_worktime_isworkday_20240101_ref = ref<any[]>([])
const assemble_control_area_list_ref = ref<any[]>([])
const general_assemble_control_securityclearance_ref = ref<any[]>([])
const assemble_control_securityclearance_list_ref = ref<any[]>([])
const assemble_control_securityclearance_object_ref = ref<any[]>([])
const assemble_control_invoice_list_ref = ref<any[]>([])
const general_assemble_control_status_ref = ref<any[]>([])
const assemble_control_worktime_minutesofworkday_ref = ref<any[]>([])
const general_worktime_isworkday_ref = ref<any[]>([])
const assemble_control_upgrade_2021090902_ref = ref<any[]>([])
const general_file_list_ref = ref<any[]>([])
const api_jaxrs_ap_455_data = ref<any[]>([])
const api_jaxrs_ap_42_data = ref<any[]>([])
const api_jaxrs_co_514_data = ref<any[]>([])
const api_jaxrs_correlatio_588_data = ref<any[]>([])
const api_jaxrs_co_405_data = ref<any[]>([])
const api_jaxrs_correlatio_128_data = ref<any[]>([])
const api_jaxrs_correlatio_39_data = ref<any[]>([])
const api_jaxrs_correlatio_521_data = ref<any[]>([])
const api_jaxrs_correlatio_173_data = ref<any[]>([])
const api_jaxrs_correlatio_939_data = ref<any[]>([])
const api_jaxrs_correlatio_390_data = ref<any[]>([])
const api_jaxrs_co_467_data = ref<any[]>([])
const api_jaxrs_correlatio_484_data = ref<any[]>([])
const jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_1_ref = ref<any[]>([])
const jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_2_ref = ref<any[]>([])
const api_jaxrs_correlatio_347_data = ref<any[]>([])
const api_jaxrs_correlatio_549_data = ref<any[]>([])
const api_jaxrs_co_571_data = ref<any[]>([])
const api_jaxrs_correlatio_116_data = ref<any[]>([])
const jaxrs_correlation_service_processing_unlink_type1_id1_type2_id2_ref = ref<any[]>([])
const api_jaxrs_ge_584_data = ref<any[]>([])
const api_jaxrs_ge_88_data = ref<any[]>([])
const jaxrs_general_assemble_control_excel_upload_with_url_ref = ref<any[]>([])
const api_jaxrs_general_as_790_data = ref<any[]>([])
const api_jaxrs_ge_799_data = ref<any[]>([])
const jaxrs_general_assemble_control_invoice_upload_ref = ref<any[]>([])
const api_jaxrs_ge_807_data = ref<any[]>([])
const jaxrs_general_assemble_control_invoice_upload_with_url_ref = ref<any[]>([])
const jaxrs_general_assemble_control_office_html_to_word_ref = ref<any[]>([])
const api_jaxrs_ge_132_data = ref<any[]>([])
const api_jaxrs_ge_60_data = ref<any[]>([])
const jaxrs_general_assemble_control_securityclearance_subject_ref = ref<any[]>([])
const jaxrs_general_assemble_control_securityclearance_system_ref = ref<any[]>([])
const api_jaxrs_ge_638_data = ref<any[]>([])
const jaxrs_general_assemble_control_upgrade_2021090901_ref = ref<any[]>([])
const api_jaxrs_ge_840_data = ref<any[]>([])
const api_jaxrs_ge_83_data = ref<any[]>([])
const api_jaxrs_general_di_728_data = ref<any[]>([])
const api_jaxrs_general_di_462_data = ref<any[]>([])
const api_jaxrs_general_fi_958_data = ref<any[]>([])
const api_jaxrs_ge_977_data = ref<any[]>([])
const api_jaxrs_general_in_370_data = ref<any[]>([])
const api_jaxrs_ge_489_data = ref<any[]>([])
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
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:480px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{display:block;font-size:12px;color:var(--text-muted);margin-bottom:4px}
.form-input{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>