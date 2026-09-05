<template>
  <div class="query-view">
    <div class="view-header glass-card">
      <h1>查询管理</h1>
      <p class="subtitle">/jaxrs/query/assemble/designer/*</p>
      <button class="nb" @click="showCreate=true">+ 新建查询</button>
    </div>
    <div class="ql">
      <aside class="qs glass-card">
        <div class="sb2"><span class="si2">⌕</span><input v-model="sq" placeholder="搜索查询..." class="si3" /></div>
        <div v-if="qs2.length===0" class="es2"><p>暂无查询定义</p></div>
        <ul class="ql2">
          <li v-for="q in qs2" :key="q.id" class="qi2" :class="{active:selected?.id===q.id}" @click="selQ(q)">
            <span class="qicon">{{q.icon||'📊'}}</span>
            <div class="qi2i"><div class="qin">{{q.name||q.queryName||'未命名'}}</div><div class="qim">{{q.category||q.entityCategory||'通用'}}</div></div>
            <span class="qit">{{fmtT(q.updateTime)}}</span>
          </li>
        </ul>
      </aside>
      <main class="qm glass-card">
        <template v-if="selected">
          <div class="qh"><h2>{{selected.name||selected.queryName}}</h2><div class="qa2">
            <button class="abtn" @click="runQ">▶ 执行</button>
            <button class="abtn" @click="delQ">🗑 删除</button>
          </div></div>
          <div class="qf"><input v-model="filterText" class="fi2" placeholder="输入筛选条件..." /><button class="rb" @click="runQ">执行</button></div>
          <div class="qr">
            <div v-if="rloading" class="ls2"><div class="sk2" v-for="i in 5" :key="i"></div></div>
            <div v-else-if="rdata.length===0" class="es2"><p>暂无结果，点击执行</p></div>
            <div v-else class="rt2">
              <div class="rth"><span v-for="h in rheaders" :key="h" class="rh2">{{h}}</span></div>
              <div v-for="(row,ri) in rdata" :key="ri" class="tr2"><span v-for="h in rheaders" :key="h" class="rc2">{{row[h]??'—'}}</span></div>
            </div>
          </div>
        </template>
        <div v-else class="em2"><div class="emi2">📊</div><h2>选择查询定义</h2><p>点击左侧查询列表执行查询</p></div>
      </main>
    </div>
    <div v-if="showCreate" class="mo" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>新建查询</h3>
        <div class="fg3"><label>名称</label><input v-model="nform.name" class="fi3" placeholder="查询名称" /></div>
        <div class="fg3"><label>SQL</label><textarea v-model="nform.sql" class="fta3" rows="5" placeholder="SELECT ..."></textarea></div>
        <div class="mf3"><button class="bc3" @click="showCreate=false">取消</button><button class="bs3" :disabled="!nform.name" @click="createQ">创建</button></div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
interface Q{id:string;name?:string;queryName?:string;icon?:string;category?:string;entityCategory?:string;updateTime?:string;sql?:string}
const sq=ref(''),qs2=ref<Q[]>([]),selected=ref<Q|null>(null),filterText=ref(''),rdata=ref<any[]>([]),rheaders=ref<string[]>([]),rloading=ref(false),showCreate=ref(false),nform=ref({name:'',sql:''}),qc=useQueryClient()
const{data}=useQuery({queryKey:['query','defs'],queryFn:()=>api.get('/jaxrs/query/assemble/designer/list').then((r:any)=>(r.data??[])as Q[]),staleTime:60000})
qs2.value=data.value??[]
const qsFiltered=computed(()=>sq.value?qs2.value.filter(q=>(q.name||'').toLowerCase().includes(sq.value.toLowerCase())):qs2.value)
function selQ(q:Q){selected.value=q;rdata.value=[];rheaders.value=[]}
async function runQ(){if(!selected.value)return;rloading.value=true;try{const r=await api.post('/jaxrs/query/assemble/designer/execute',{id:selected.value!.id,filter:filterText.value});const d=(r as any)?.data;rdata.value=d?.list??[];if(rdata.value.length>0)rheaders.value=Object.keys(rdata.value[0])}catch{}finally{rloading.value=false}}
const dm=useMutation({mutationFn:(id:string)=>api.delete(`/jaxrs/query/assemble/designer/delete/${id}`),onSuccess:()=>{qc.invalidateQueries({queryKey:['query','defs']});if(selected.value?.id)selected.value=null}})
function delQ(){if(selected.value&&confirmMsg('确定删除？'))dm.mutate(selected.value.id)}
const cm=useMutation({mutationFn:()=>api.post('/jaxrs/query/assemble/designer/create',nform.value),onSuccess:()=>{showCreate.value=false;qc.invalidateQueries({queryKey:['query','defs']})}})
function createQ(){if(nform.value.name)cm.mutate()}
function fmtT(t?:string){if(!t)return'';try{return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'})}catch{return String(t)}}
</script>
<style scoped>
.query-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.nb{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.ql{display:flex;flex:1;gap:16px;min-height:0}
.qs{width:260px;flex-shrink:0;display:flex;flex-direction:column;padding:12px}
.sb2{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-md);margin-bottom:8px}
.si2{color:var(--text-muted)}
.si3{flex:1;background:none;border:none;outline:none;color:var(--text-primary);font-size:13px}
.ql2{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.qi2{display:flex;align-items:center;gap:8px;padding:10px;border-radius:var(--radius-md);cursor:pointer;transition:all var(--transition-fast);margin-bottom:4px}
.qi2:hover,.qi2.active{background:var(--color-primary-soft)}
.qicon{font-size:18px}
.qi2i{flex:1;min-width:0}
.qin{font-size:13px;color:var(--text-primary);font-weight:500;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.qim{font-size:11px;color:var(--text-muted)}
.qit{font-size:10px;color:var(--text-muted)}
.qm{flex:1;display:flex;flex-direction:column;overflow:hidden;padding:16px}
.qh{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px;padding-bottom:12px;border-bottom:1px solid var(--border-subtle)}
.qh h2{font-size:16px;color:var(--color-primary);margin:0;font-family:'Orbitron',sans-serif}
.qa2{display:flex;gap:6px}
.abtn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px;transition:all var(--transition-fast)}
.abtn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.qf{display:flex;gap:8px;margin-bottom:12px;align-items:center}
.fi2{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:8px 12px;color:var(--text-primary);font-size:13px;outline:none;font-family:'JetBrains Mono',monospace}
.fi2:focus{border-color:var(--color-primary)}
.rb{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600;white-space:nowrap}
.qr{flex:1;overflow:auto}
.rt2{display:flex;flex-direction:column}
.rth,.tr2{display:grid;gap:2px;padding:6px 10px;font-size:13px}
.rth{background:var(--bg-elevated);color:var(--text-muted);font-weight:600;position:sticky;top:0;z-index:1}
.tr2:hover{background:var(--color-primary-soft)}
.rh2,.rc2{padding:2px 4px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.em2,.es2,.ls2{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.emi2{font-size:48px;opacity:0.4}
.sk2{height:32px;border-radius:var(--radius-sm);margin-bottom:6px;background:var(--bg-elevated)}
.mo{position:fixed;inset:0;background:var(--bg-overlay);z-index:200;display:flex;align-items:center;justify-content:center}
.modal{width:560px;padding:24px}
.modal h3{color:var(--color-primary);font-family:'Orbitron',sans-serif;margin:0 0 16px;font-size:16px}
.fg3{display:flex;flex-direction:column;gap:6px;margin-bottom:12px}
.fg3 label{font-size:12px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px}
.fi3{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--text-primary);font-size:14px;outline:none}
.fta3{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:10px 14px;color:var(--color-primary);font-size:13px;outline:none;font-family:'JetBrains Mono',monospace;resize:vertical}
.fta3:focus{border-color:var(--color-primary)}
.mf3{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.bc3{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:none;color:var(--text-secondary);cursor:pointer}
.bs3{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.bs3:disabled{opacity:.5;cursor:not-allowed}
@media(max-width:768px){.ql{flex-direction:column}.qs{width:100%;max-height:200px}}
</style>
