<template>
  <div class="editor-view">
    <div class="view-header glass-card">
      <div><h1>查找设计器</h1><p class="subtitle">/jaxrs/query/assemble/designer/find/list</p></div>
      <div class="header-actions">
        <button class="btn-secondary" @click="preview">预览</button>
        <button class="btn-primary" @click="save">💾 保存</button>
      </div>
    </div>
    <div class="editor-layout glass-card">
      <div class="editor-sidebar">
        <div class="sb-title">配置列表</div>
        <div v-if="loading" class="loading-sm">加载中...</div>
        <ul v-else class="sb-list">
          <li v-for="item in items" :key="item.id" class="sb-item" :class="{active:selected?.id===item.id}" @click="selectItem(item)">{{ item.name||item.flag||item.id }}</li>
          <li v-if="items.length===0" class="empty">暂无配置</li>
        </ul>
        <button class="btn-sm sb-add" @click="createNew">+ 新建</button>
      </div>
      <div class="editor-main">
        <div v-if="!selected" class="empty-main"><div class="emi">⚙</div><p>选择或创建配置</p></div>
        <div v-else class="editor-content">
          <div class="ec-header"><span class="ec-title">{{ selected.name||selected.flag||'未命名' }}</span></div>
          <textarea v-model="config" class="code-editor" placeholder="在此输入JSON配置..." rows="20"></textarea>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { toast } from '../utils/toast'
import { useQuery } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
interface Item { id:string; name?:string; flag?:string; config?:string }
const loading=ref(false),items=ref<Item[]>([]),selected=ref<Item|null>(null),config=ref('')
const ep='/jaxrs/query/assemble/designer/find/list';
const {data}=useQuery({queryKey:['FindDesigner','list'],queryFn:async()=>{loading.value=true;try{const r=await api.get(ep);return(r as any)?.data??[]}finally{loading.value=false}}})
items.value=data.value??[]
function selectItem(item:Item){selected.value=item;config.value=item.config?'\n'+item.config:'{}'}
function createNew(){const n:Item={id:Date.now().toString(),name:'未命名',flag:'',config:'{}'};items.value=[n,...items.value];selectItem(n)}
function preview(){toast.info('配置预览: '+config.value)}
function save(){if(selected.value&&config.value){api.put(ep+'/'+selected.value.id,{...selected.value,config:config.value}).then(()=>toast.info('保存成功'))}}
</script>
<style scoped>
.editor-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.header-actions{display:flex;gap:8px}
.btn-secondary{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.btn-primary{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.editor-layout{display:flex;flex:1;min-height:0;gap:16px}
.editor-sidebar{width:240px;flex-shrink:0;display:flex;flex-direction:column;padding:12px}
.sb-title{font-size:12px;color:var(--text-muted);text-transform:uppercase;margin-bottom:8px}
.sb-list{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.sb-item{padding:8px 12px;border-radius:var(--radius-md);cursor:pointer;font-size:13px;color:var(--text-primary);margin-bottom:2px}
.sb-item:hover,.sb-item.active{background:var(--color-primary-soft)}
.sb-add{margin-top:8px;padding:6px 12px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:100%}
.empty{padding:12px;color:var(--text-muted);text-align:center;font-size:12px}
.editor-main{flex:1;display:flex;flex-direction:column;min-height:0}
.empty-main{flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;color:var(--text-muted)}
.emi{font-size:32px;margin-bottom:8px}
.editor-content{flex:1;display:flex;flex-direction:column;gap:8px}
.ec-header{display:flex;align-items:center;justify-content:space-between}
.ec-title{font-size:14px;font-weight:600;color:var(--text-primary)}
.code-editor{flex:1;min-height:300px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code',monospace;font-size:13px;outline:none;resize:none;tab-size:2}
.loading-sm{padding:12px;color:var(--text-muted);font-size:12px}
</style>