<template>
  <div class="ai-view">
    <div class="view-header glass-card">
      <h1>AI 助手</h1>
      <p class="subtitle">/jaxrs/ai/* — 模型对话 + MCP 配置</p>
    </div>
    <div class="ai-layout">
      <aside class="ai-sidebar glass-card">
        <div class="sh"><h3>对话历史</h3><button class="nb" @click="createConv">+ 新建</button></div>
        <div v-if="convs.length===0" class="ec"><p>暂无对话</p></div>
        <ul class="cl">
          <li v-for="c in convs" :key="c.id" class="ci" :class="{active:active?.id===c.id}" @click="selectConv(c)">
            <span class="cn">{{c.title||'新对话'}}</span><span class="ct2">{{c.time}}</span>
          </li>
        </ul>
        <div class="sf"><button class="mbtn" @click="showMcp=!showMcp">MCP 配置</button></div>
      </aside>
      <main class="ai-main glass-card">
        <div v-if="!active" class="ec"><div class="ei">🤖</div><h2>选择或创建对话</h2><p>AI 助手将帮助您处理工作流中的各类任务</p></div>
        <template v-else>
          <div class="ch2"><span class="ct3">{{active.title||'对话'}}</span><span class="mt2">{{active.model||'default'}}</span></div>
          <div ref="msgBox" class="cm"><div v-for="m in msgs" :key="m.id" class="msg" :class="m.role">
            <div class="ma2">{{m.role==='user'?'👤':'🤖'}}</div>
            <div class="mb2" v-html="fmt(m.content)"></div>
          </div>
          <div v-if="streaming" class="msg assistant"><div class="ma2">🤖</div><div class="mb2 sb2"><span class="td2"></span><span class="td2"></span><span class="td2"></span></div></div>
          </div>
          <div class="ci2">
            <textarea v-model="inputTxt" @keydown.ctrl.enter="sendMsg" placeholder="输入消息，Ctrl+Enter 发送..." class="it" rows="2"></textarea>
            <button class="sbtn" :disabled="!inputTxt.trim()||streaming" @click="sendMsg">{{streaming?'生成中...':'发送'}}</button>
          </div>
        </template>
      </main>
    </div>
    <div v-if="showMcp" class="mo" @click.self="showMcp=false">
      <div class="modal glass-card">
        <h3>MCP 配置管理</h3>
        <div class="ml2">
          <div v-for="m in mcps" :key="m.id" class="mi2">
            <span class="mn2">{{m.name}}</span><span class="me2">{{m.endpoint}}</span>
            <div class="ma3"><button class="bsm" @click="toggleMcp(m)">{{m.enabled?'禁用':'启用'}}</button><button class="bsm dg" @click="delMcp(m.id)">删除</button></div>
          </div>
        </div>
        <button class="bp2" @click="addMcp">+ 添加 MCP 服务</button>
        <button class="bc2" @click="showMcp=false">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useQuery } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

interface Conv { id:string; title?:string; model?:string; time:string }
interface Msg { id:string; role:'user'|'assistant'; content:string }

const convs = ref<Conv[]>([])
const active = ref<Conv|null>(null)
const msgs = ref<Msg[]>([])
const inputTxt = ref('')
const streaming = ref(false)
const showMcp = ref(false)
const msgBox = ref<HTMLElement>()
const mcps = ref<Array<{id:string;name:string;endpoint:string;enabled:boolean}>>([])

const { data } = useQuery({ queryKey:['ai','convs'], queryFn:()=>api.get('/jaxrs/ai/conversation/list').then((r:any)=>(r.data??[]) as Conv[]), staleTime:60000 })
convs.value = data.value ?? []

function selectConv(c:Conv){ active.value=c; msgs.value=[] }
function fmt(c:string){ return (c??'').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/\n/g,'<br>') }
function scrollB(){ msgBox.value?.scrollTo({top:msgBox.value!.scrollHeight,behavior:'smooth'}) }

async function createConv(){
  const r = await api.post('/jaxrs/ai_assemble_control/chat/write/completion/extra',{})
  const nc = ((r as any)?.data??{}) as Conv
  convs.value.unshift(nc); active.value=nc; msgs.value=[]
}

async function sendMsg(){
  const txt=inputTxt.value.trim(); if(!txt||!active.value||streaming.value) return
  msgs.value.push({id:`u-${Date.now()}`,role:'user',content:txt}); inputTxt.value=''
  streaming.value=true; scrollB()
  try {
    const r = await api.post('/jaxrs/ai_assemble_control/chat/completion/stream',{conversationId:active.value!.id,message:txt})
    const c = ((r as any)?.data?.content??'收到回复') as string
    msgs.value.push({id:`a-${Date.now()}`,role:'assistant',content:c})
  } catch(e){ msgs.value.push({id:`a-${Date.now()}`,role:'assistant',content:`⚠ ${(e as Error).message}`}) }
  finally { streaming.value=false; scrollB() }
}

async function toggleMcp(m:any){ m.enabled=!m.enabled; await api.post(`/jaxrs/ai_assemble_control/config/${m.enabled?'create':'delete'}/mcp`,{id:m.id}) }
async function delMcp(id:string){ await api.delete(`/jaxrs/ai_assemble_control/config/delete/mcp/${id}`); mcps.value=mcps.value.filter(x=>x.id!==id) }
function addMcp(){ /* future dialog */ }

onMounted(()=>{ api.get('/jaxrs/ai_assemble_control/config/list/mcp/paging/1/20').then((r:any)=>{ mcps.value=(r.data??[]) as typeof mcps.value }) })
</script>

<style scoped>
.ai-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.ai-layout{display:flex;flex:1;gap:16px;min-height:0}
.ai-sidebar{width:220px;flex-shrink:0;display:flex;flex-direction:column;padding:12px}
.sh{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}
.sh h3{font-size:11px;color:var(--color-primary);margin:0;text-transform:uppercase;letter-spacing:1px}
.nb{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--color-primary);background:var(--color-primary-soft);color:var(--color-primary);cursor:pointer;font-size:12px}
.cl{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.ci{padding:8px 10px;border-radius:var(--radius-md);cursor:pointer;transition:all var(--transition-fast);margin-bottom:4px}
.ci:hover,.ci.active{background:var(--color-primary-soft)}
.cn{display:block;font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.ct2{font-size:10px;color:var(--text-muted)}
.sf{padding-top:12px;border-top:1px solid var(--border-subtle)}
.mbtn{width:100%;padding:8px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px}
.mbtn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.ai-main{flex:1;display:flex;flex-direction:column;overflow:hidden;padding:16px}
.ec{flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:12px;color:var(--text-muted)}
.ei{font-size:64px;opacity:0.4}
.ec h2{font-family:'Orbitron',sans-serif;color:var(--text-secondary)}
.ch2{display:flex;align-items:center;gap:8px;margin-bottom:12px;padding-bottom:8px;border-bottom:1px solid var(--border-subtle)}
.ct3{font-size:14px;font-weight:600;color:var(--text-primary)}
.mt2{font-size:11px;padding:2px 8px;border-radius:10px;background:var(--color-accent-soft);color:var(--color-accent)}
.cm{flex:1;overflow-y:auto;display:flex;flex-direction:column;gap:12px;padding-bottom:12px}
.msg{display:flex;gap:8px;max-width:85%}
.msg.user{margin-left:auto;flex-direction:row-reverse}
.ma2{width:28px;height:28px;border-radius:50%;flex-shrink:0;display:flex;align-items:center;justify-content:center;font-size:14px}
.msg.user .ma2{background:var(--color-primary-soft)}
.msg.assistant .ma2{background:linear-gradient(135deg,var(--color-accent),var(--color-primary))}
.mb2{padding:10px 14px;border-radius:var(--radius-lg);font-size:14px;line-height:1.6;color:var(--text-primary);white-space:pre-wrap;word-break:break-word}
.msg.user .mb2{background:var(--color-primary-soft);border:1px solid var(--border-active)}
.msg.assistant .mb2{background:var(--bg-elevated);border:1px solid var(--border-subtle)}
.sb2{display:flex;gap:4px;padding:14px}
.td2{width:8px;height:8px;border-radius:50%;background:var(--color-primary);animation:tp 1.2s infinite}
.td2:nth-child(2){animation-delay:.2s}.td2:nth-child(3){animation-delay:.4s}
@keyframes tp{0%,60%,100%{opacity:.2;transform:translateY(0)}30%{opacity:1;transform:translateY(-4px)}}
.ci2{display:flex;gap:8px;margin-top:12px}
.it{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:10px 14px;color:var(--text-primary);font-size:14px;resize:none;outline:none;font-family:inherit;transition:border-color var(--transition-fast)}
.it:focus{border-color:var(--color-primary)}
.sbtn{padding:10px 20px;border-radius:var(--radius-md);border:none;background:linear-gradient(135deg,var(--color-primary),var(--color-primary-deep));color:white;font-weight:600;cursor:pointer;transition:all var(--transition-fast);white-space:nowrap}
.sbtn:hover:not(:disabled){transform:translateY(-1px);box-shadow:0 4px 12px var(--color-primary-glow)}
.sbtn:disabled{opacity:.5;cursor:not-allowed}
.mo{position:fixed;inset:0;background:var(--bg-overlay);z-index:200;display:flex;align-items:center;justify-content:center}
.modal{width:480px;padding:24px}
.modal h3{color:var(--color-primary);font-family:'Orbitron',sans-serif;margin:0 0 16px;font-size:16px}
.ml2{display:flex;flex-direction:column;gap:8px;margin-bottom:16px}
.mi2{display:flex;align-items:center;gap:12px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.mn2{font-size:13px;font-weight:600;color:var(--text-primary);min-width:80px}
.me2{font-size:12px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;flex:1}
.ma3{display:flex;gap:4px}
.bsm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:none;color:var(--text-secondary);cursor:pointer;font-size:11px}
.bsm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.bsm.dg:hover{border-color:var(--color-error);color:var(--color-error)}
.bp2{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-size:13px;margin-right:8px}
.bc2{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:none;color:var(--text-secondary);cursor:pointer;font-size:13px}
</style>
