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
import { toast } from '../utils/toast';
import { ref, computed } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
interface Item { id:string; name?:string; label?:string; flag?:string; updateTime?:string; createTime?:string }
const search=ref(''),showCreate=ref(false),showEdit=ref(false),loading=ref(false)
const items=ref<Item[]>([]),form=ref<Partial<Item>>({}),editingId=ref<string|null>(null)
const qc=useQueryClient()
const ep='/jaxrs/general/assemble/control/list';const qk=['Common','list'];
const {data}=useQuery({queryKey:qk,queryFn:async()=>{loading.value=true;try{const r=await api.get(ep);return(r as any)?.data??[]}finally{loading.value=false}}})
items.value=data.value??[]
const filtered=computed(()=>search.value?items.value.filter(i=>(i.name||'').toLowerCase().includes(search.value.toLowerCase())||(i.flag||'').toLowerCase().includes(search.value.toLowerCase())):items.value)
function editItem(item:Item){form.value={...item};editingId.value=item.id;showEdit.value=true}
function closeModal(){showCreate.value=false;showEdit.value=false;form.value={}}
const saveM=useMutation({mutationFn:async(d:any)=>editingId.value?api.put(ep+'/'+editingId.value,d):api.post(ep,d),onSuccess:()=>{qc.invalidateQueries({queryKey:qk});closeModal()}})
function saveItem(){if(form.value.name)saveM.mutate(form.value)}
const delM=useMutation({mutationFn:async(id:string)=>api.delete(ep+'/'+id),onSuccess:()=>{qc.invalidateQueries({queryKey:qk})}})
function deleteItem(item:Item){if(confirmMsg('确定删除？'))delM.mutate(item.id)}
function loadData(){qc.invalidateQueries({queryKey:qk})}
function fmtTime(t?:string){if(!t)return'';try{return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'})}catch{return String(t)}}

const assemble_control_qrcode_list_ref = ref<any[]>([]);
const assemble_control_qrcode_list_q = useQuery({
  queryKey: ['assemble_control_qrcode_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/qrcode/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_securityclearance_enable_ref = ref<any[]>([]);
const assemble_control_securityclearance_enable_q = useQuery({
  queryKey: ['assemble_control_securityclearance_enable'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance/enable"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_assemble_control_invoice_ref = ref<any[]>([]);
const general_assemble_control_invoice_q = useQuery({
  queryKey: ['general_assemble_control_invoice'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/invoice"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_core_list_ref = ref<any[]>([]);
const general_core_list_q = useQuery({
  queryKey: ['general_core_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/core/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_assemble_control_office_ref = ref<any[]>([]);
const general_assemble_control_office_q = useQuery({
  queryKey: ['general_assemble_control_office'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/office"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_dict_item_create_ref = ref<any[]>([]);
const general_dict_item_create_q = useQuery({
  queryKey: ['general_dict_item_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/dict/item/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_area_list_ref = ref<any[]>([]);
const general_area_list_q = useQuery({
  queryKey: ['general_area_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/area/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_config_list_ref = ref<any[]>([]);
const general_config_list_q = useQuery({
  queryKey: ['general_config_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/config/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_attendscope_list_ref = ref<any[]>([]);
const assemble_control_attendscope_list_q = useQuery({
  queryKey: ['assemble_control_attendscope_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/attendscope/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_excel_upload_ref = ref<any[]>([]);
const assemble_control_excel_upload_q = useQuery({
  queryKey: ['assemble_control_excel_upload'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/excel/upload"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_area_create_ref = ref<any[]>([]);
const assemble_control_area_create_q = useQuery({
  queryKey: ['assemble_control_area_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/area/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const dict_item_list_test_dict_id_ref = ref<any[]>([]);
const dict_item_list_test_dict_id_q = useQuery({
  queryKey: ['dict_item_list_test_dict_id'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/dict/item/list/test-dict-id"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_assemble_config_ref = ref<any[]>([]);
const general_assemble_config_q = useQuery({
  queryKey: ['general_assemble_config'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/config"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_permissions_mind_ref = ref<any[]>([]);
const assemble_control_permissions_mind_q = useQuery({
  queryKey: ['assemble_control_permissions_mind'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/permissions/mind"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_worktime_isworkday_20240101_ref = ref<any[]>([]);
const general_worktime_isworkday_20240101_q = useQuery({
  queryKey: ['general_worktime_isworkday_20240101'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/worktime/isworkday/20240101"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_area_list_ref = ref<any[]>([]);
const assemble_control_area_list_q = useQuery({
  queryKey: ['assemble_control_area_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/area/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_assemble_control_securityclearance_ref = ref<any[]>([]);
const general_assemble_control_securityclearance_q = useQuery({
  queryKey: ['general_assemble_control_securityclearance'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_securityclearance_list_ref = ref<any[]>([]);
const assemble_control_securityclearance_list_q = useQuery({
  queryKey: ['assemble_control_securityclearance_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_securityclearance_object_ref = ref<any[]>([]);
const assemble_control_securityclearance_object_q = useQuery({
  queryKey: ['assemble_control_securityclearance_object'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance/object"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_invoice_list_ref = ref<any[]>([]);
const assemble_control_invoice_list_q = useQuery({
  queryKey: ['assemble_control_invoice_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/invoice/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_assemble_control_status_ref = ref<any[]>([]);
const general_assemble_control_status_q = useQuery({
  queryKey: ['general_assemble_control_status'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/status"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_worktime_minutesofworkday_ref = ref<any[]>([]);
const assemble_control_worktime_minutesofworkday_q = useQuery({
  queryKey: ['assemble_control_worktime_minutesofworkday'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/worktime/minutesofworkday"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_worktime_isworkday_ref = ref<any[]>([]);
const general_worktime_isworkday_q = useQuery({
  queryKey: ['general_worktime_isworkday'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/worktime/isworkday/"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_upgrade_2021090902_ref = ref<any[]>([]);
const assemble_control_upgrade_2021090902_q = useQuery({
  queryKey: ['assemble_control_upgrade_2021090902'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/upgrade/2021090902"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const general_file_list_ref = ref<any[]>([]);
const general_file_list_q = useQuery({
  queryKey: ['general_file_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/file/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const api_jaxrs_ap_455_data = ref<any[]>([]);
const { data: api_jaxrs_ap_455_q } = useQuery({queryKey: ['api_jaxrs_ap_455', '/jaxrs/appconfig/a-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/appconfig/a-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ap_455_q, (v) => { api_jaxrs_ap_455_data.value = v ?? []; });
const api_jaxrs_ap_42_data = ref<any[]>([]);
const { data: api_jaxrs_ap_42_q } = useQuery({queryKey: ['api_jaxrs_ap_42', '/jaxrs/appconfig/app-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/appconfig/app-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ap_42_q, (v) => { api_jaxrs_ap_42_data.value = v ?? []; });
const api_jaxrs_co_514_data = ref<any[]>([]);
const { data: api_jaxrs_co_514_q } = useQuery({queryKey: ['api_jaxrs_co_514', '/jaxrs/correlation/core/entity/list/by/user/user-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/core/entity/list/by/user/user-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_co_514_q, (v) => { api_jaxrs_co_514_data.value = v ?? []; });
const api_jaxrs_correlatio_588_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_588_q } = useQuery({queryKey: ['api_jaxrs_correlatio_588', '/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/doc-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/doc-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_588_q, (v) => { api_jaxrs_correlatio_588_data.value = v ?? []; });
const api_jaxrs_co_405_data = ref<any[]>([]);
const { data: api_jaxrs_co_405_q } = useQuery({queryKey: ['api_jaxrs_co_405', '/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/job-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/job-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_co_405_q, (v) => { api_jaxrs_co_405_data.value = v ?? []; });
const api_jaxrs_correlatio_128_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_128_q } = useQuery({queryKey: ['api_jaxrs_correlatio_128', '/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_128_q, (v) => { api_jaxrs_correlatio_128_data.value = v ?? []; });
const api_jaxrs_correlatio_39_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_39_q } = useQuery({queryKey: ['api_jaxrs_correlatio_39', '/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1/site/site-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-1/site/site-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_39_q, (v) => { api_jaxrs_correlatio_39_data.value = v ?? []; });
const api_jaxrs_correlatio_521_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_521_q } = useQuery({queryKey: ['api_jaxrs_correlatio_521', '/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-l/site/s1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/doc-l/site/s1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_521_q, (v) => { api_jaxrs_correlatio_521_data.value = v ?? []; });
const api_jaxrs_correlatio_173_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_173_q } = useQuery({queryKey: ['api_jaxrs_correlatio_173', '/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_173_q, (v) => { api_jaxrs_correlatio_173_data.value = v ?? []; });
const api_jaxrs_correlatio_939_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_939_q } = useQuery({queryKey: ['api_jaxrs_correlatio_939', '/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1/site/site-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/job-1/site/site-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_939_q, (v) => { api_jaxrs_correlatio_939_data.value = v ?? []; });
const api_jaxrs_correlatio_390_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_390_q } = useQuery({queryKey: ['api_jaxrs_correlatio_390', '/jaxrs/correlation/service/processing/correlation/readable/type/cms'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/readable/type/cms"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_390_q, (v) => { api_jaxrs_correlatio_390_data.value = v ?? []; });
const api_jaxrs_co_467_data = ref<any[]>([]);
const { data: api_jaxrs_co_467_q } = useQuery({queryKey: ['api_jaxrs_co_467', '/jaxrs/correlation/service/processing/correlation/readable/type/processplatform'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/readable/type/processplatform"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_co_467_q, (v) => { api_jaxrs_co_467_data.value = v ?? []; });
const api_jaxrs_correlatio_484_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_484_q } = useQuery({queryKey: ['api_jaxrs_correlatio_484', '/jaxrs/correlation/service/processing/correlation/type/cms/document/doc-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/type/cms/document/doc-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_484_q, (v) => { api_jaxrs_correlatio_484_data.value = v ?? []; });
const jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_1_ref = ref<any[]>([]);
const jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_1_q = useQuery({
  queryKey: ['jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/job-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_2_ref = ref<any[]>([]);
const jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_2_q = useQuery({
  queryKey: ['jaxrs_correlation_service_processing_correlation_type_processplatform_job_job_2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/job-2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_correlatio_347_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_347_q } = useQuery({queryKey: ['api_jaxrs_correlatio_347', '/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_347_q, (v) => { api_jaxrs_correlatio_347_data.value = v ?? []; });
const api_jaxrs_correlatio_549_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_549_q } = useQuery({queryKey: ['api_jaxrs_correlatio_549', '/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-u'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/update/type/cms/document/doc-u"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_549_q, (v) => { api_jaxrs_correlatio_549_data.value = v ?? []; });
const api_jaxrs_co_571_data = ref<any[]>([]);
const { data: api_jaxrs_co_571_q } = useQuery({queryKey: ['api_jaxrs_co_571', '/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/job-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/job-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_co_571_q, (v) => { api_jaxrs_co_571_data.value = v ?? []; });
const api_jaxrs_correlatio_116_data = ref<any[]>([]);
const { data: api_jaxrs_correlatio_116_q } = useQuery({queryKey: ['api_jaxrs_correlatio_116', '/jaxrs/correlation/service/processing/unlink/message/msg-1/process/proc-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/correlation/service/processing/unlink/message/msg-1/process/proc-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_correlatio_116_q, (v) => { api_jaxrs_correlatio_116_data.value = v ?? []; });
const jaxrs_correlation_service_processing_unlink_type1_id1_type2_id2_ref = ref<any[]>([]);
const jaxrs_correlation_service_processing_unlink_type1_id1_type2_id2_q = useQuery({
  queryKey: ['jaxrs_correlation_service_processing_unlink_type1_id1_type2_id2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/service/processing/unlink/type1/id1/type2/id2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_ge_584_data = ref<any[]>([]);
const { data: api_jaxrs_ge_584_q } = useQuery({queryKey: ['api_jaxrs_ge_584', '/jaxrs/general/assemble/control/attendscope/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/attendscope/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_584_q, (v) => { api_jaxrs_ge_584_data.value = v ?? []; });
const api_jaxrs_ge_88_data = ref<any[]>([]);
const { data: api_jaxrs_ge_88_q } = useQuery({queryKey: ['api_jaxrs_ge_88', '/jaxrs/general/assemble/control/ecnet/check'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/ecnet/check"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_88_q, (v) => { api_jaxrs_ge_88_data.value = v ?? []; });
const jaxrs_general_assemble_control_excel_upload_with_url_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_excel_upload_with_url_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_excel_upload_with_url'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/excel/upload/with/url"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_general_as_790_data = ref<any[]>([]);
const { data: api_jaxrs_general_as_790_q } = useQuery({queryKey: ['api_jaxrs_general_as_790', '/jaxrs/general/assemble/control/generalfile'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/generalfile"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_general_as_790_q, (v) => { api_jaxrs_general_as_790_data.value = v ?? []; });
const api_jaxrs_ge_799_data = ref<any[]>([]);
const { data: api_jaxrs_ge_799_q } = useQuery({queryKey: ['api_jaxrs_ge_799', '/jaxrs/general/assemble/control/invoice/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/invoice/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_799_q, (v) => { api_jaxrs_ge_799_data.value = v ?? []; });
const jaxrs_general_assemble_control_invoice_upload_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_invoice_upload_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_invoice_upload'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/invoice/upload"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_ge_807_data = ref<any[]>([]);
const { data: api_jaxrs_ge_807_q } = useQuery({queryKey: ['api_jaxrs_ge_807', '/jaxrs/general/assemble/control/invoice/upload/for/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/invoice/upload/for/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_807_q, (v) => { api_jaxrs_ge_807_data.value = v ?? []; });
const jaxrs_general_assemble_control_invoice_upload_with_url_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_invoice_upload_with_url_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_invoice_upload_with_url'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/invoice/upload/with/url"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_general_assemble_control_office_html_to_word_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_office_html_to_word_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_office_html_to_word'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/office/html/to/word"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_ge_132_data = ref<any[]>([]);
const { data: api_jaxrs_ge_132_q } = useQuery({queryKey: ['api_jaxrs_ge_132', '/jaxrs/general/assemble/control/qrcode'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/qrcode"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_132_q, (v) => { api_jaxrs_ge_132_data.value = v ?? []; });
const api_jaxrs_ge_60_data = ref<any[]>([]);
const { data: api_jaxrs_ge_60_q } = useQuery({queryKey: ['api_jaxrs_ge_60', '/jaxrs/general/assemble/control/securityclearance/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_60_q, (v) => { api_jaxrs_ge_60_data.value = v ?? []; });
const jaxrs_general_assemble_control_securityclearance_subject_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_securityclearance_subject_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_securityclearance_subject'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance/subject"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_general_assemble_control_securityclearance_system_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_securityclearance_system_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_securityclearance_system'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/securityclearance/system"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_ge_638_data = ref<any[]>([]);
const { data: api_jaxrs_ge_638_q } = useQuery({queryKey: ['api_jaxrs_ge_638', '/jaxrs/general/assemble/control/status/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/status/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_638_q, (v) => { api_jaxrs_ge_638_data.value = v ?? []; });
const jaxrs_general_assemble_control_upgrade_2021090901_ref = ref<any[]>([]);
const jaxrs_general_assemble_control_upgrade_2021090901_q = useQuery({
  queryKey: ['jaxrs_general_assemble_control_upgrade_2021090901'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/general/assemble/control/upgrade/2021090901"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_ge_840_data = ref<any[]>([]);
const { data: api_jaxrs_ge_840_q } = useQuery({queryKey: ['api_jaxrs_ge_840', '/jaxrs/general/assemble/control/worktime/isholiday/2024-01-01'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/worktime/isholiday/2024-01-01"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_840_q, (v) => { api_jaxrs_ge_840_data.value = v ?? []; });
const api_jaxrs_ge_83_data = ref<any[]>([]);
const { data: api_jaxrs_ge_83_q } = useQuery({queryKey: ['api_jaxrs_ge_83', '/jaxrs/general/assemble/control/worktime/minutes/of/workday'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/assemble/control/worktime/minutes/of/workday"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_83_q, (v) => { api_jaxrs_ge_83_data.value = v ?? []; });
const api_jaxrs_general_di_728_data = ref<any[]>([]);
const { data: api_jaxrs_general_di_728_q } = useQuery({queryKey: ['api_jaxrs_general_di_728', '/jaxrs/general/dict/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/dict/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_general_di_728_q, (v) => { api_jaxrs_general_di_728_data.value = v ?? []; });
const api_jaxrs_general_di_462_data = ref<any[]>([]);
const { data: api_jaxrs_general_di_462_q } = useQuery({queryKey: ['api_jaxrs_general_di_462', '/jaxrs/general/dict/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/dict/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_general_di_462_q, (v) => { api_jaxrs_general_di_462_data.value = v ?? []; });
const api_jaxrs_general_fi_958_data = ref<any[]>([]);
const { data: api_jaxrs_general_fi_958_q } = useQuery({queryKey: ['api_jaxrs_general_fi_958', '/jaxrs/general/file/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/file/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_general_fi_958_q, (v) => { api_jaxrs_general_fi_958_data.value = v ?? []; });
const api_jaxrs_ge_977_data = ref<any[]>([]);
const { data: api_jaxrs_ge_977_q } = useQuery({queryKey: ['api_jaxrs_ge_977', '/jaxrs/general/invoice/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/invoice/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_977_q, (v) => { api_jaxrs_ge_977_data.value = v ?? []; });
const api_jaxrs_general_in_370_data = ref<any[]>([]);
const { data: api_jaxrs_general_in_370_q } = useQuery({queryKey: ['api_jaxrs_general_in_370', '/jaxrs/general/invoice/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/invoice/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_general_in_370_q, (v) => { api_jaxrs_general_in_370_data.value = v ?? []; });
const api_jaxrs_ge_489_data = ref<any[]>([]);
const { data: api_jaxrs_ge_489_q } = useQuery({queryKey: ['api_jaxrs_ge_489', '/jaxrs/general/securityclearance/enable'], queryFn: async () => { try { const r = await api.get("/jaxrs/general/securityclearance/enable"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_ge_489_q, (v) => { api_jaxrs_ge_489_data.value = v ?? []; });
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