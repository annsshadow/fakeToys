<template>
  <div class="workbench">
    <aside class="side glass-card">
      <div class="side-head">
        <h2>{{ title }}</h2>
        <button class="link" @click="createNew">+ 新建</button>
      </div>
      <input v-model="search" placeholder="搜索..." class="search" />
      <div class="list">
        <p v-if="loading" class="muted">加载中…</p>
        <p v-else-if="!filtered.length" class="muted">暂无脚本</p>
        <button
          v-for="item in filtered"
          :key="item.id"
          class="item"
          :class="{ active: item.id === activeId }"
          @click="open(item)"
        >
          <strong>{{ item.name }}</strong>
          <span>{{ item.category || item.id }}</span>
        </button>
      </div>
    </aside>
    <main class="editor-pane">
      <section v-if="!activeId && !creating" class="empty glass-card"><p>选择或新建一个脚本</p></section>
      <template v-else>
        <header class="meta glass-card">
          <label>名称<input v-model="form.name" placeholder="脚本名称" /></label>
          <label v-if="withCategory">分类<input v-model="form.category" placeholder="分类（可选）" /></label>
          <div class="actions">
            <button class="btn primary" :disabled="!form.name.trim() || saving" @click="save">
              {{ saving ? '保存中…' : '保存' }}
            </button>
            <button v-if="activeId" class="btn danger" :disabled="deleting" @click="remove">删除</button>
          </div>
        </header>
        <div class="code glass-card">
          <ScriptCodeEditor v-model="form.code" />
        </div>
        <section v-if="versions.length" class="versions glass-card">
          <h3>版本历史（点击版本恢复内容）</h3>
          <button v-for="(v, index) in versions" :key="index" class="version" @click="restore(v)">
            <strong>v{{ v.version ?? index + 1 }}</strong>
            <span>{{ v.creator || '—' }} · {{ v.createTime || '—' }}</span>
          </button>
        </section>
      </template>
    </main>
  </div>
</template>

<script setup lang="ts">
// W7：三域脚本设计器共享工作台——列表 + CodeMirror 编辑器 + 保存/删除 + 版本历史。
// 后端差异（端点/字段名）由各视图的 adapter 承接，本组件只管交互。
import { computed, ref } from 'vue'
import { toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ScriptCodeEditor from './ScriptCodeEditor.vue'

export interface ScriptListItem {
  id: string
  name: string
  category?: string
  updateTime?: string
}

export interface ScriptVersion {
  version?: string
  content?: string
  creator?: string
  createTime?: string
}

export interface ScriptWorkbenchAdapter {
  list(): Promise<ScriptListItem[]>
  load(id: string): Promise<{ name: string; category?: string; code: string }>
  create(data: { name: string; category?: string; code: string }): Promise<unknown>
  save(id: string, data: { name: string; category?: string; code: string }): Promise<unknown>
  remove(id: string): Promise<unknown>
  versions?(id: string): Promise<ScriptVersion[]>
}

const props = defineProps<{
  title: string
  adapter: ScriptWorkbenchAdapter
  withCategory?: boolean
}>()

const search = ref('')
const loading = ref(false)
const saving = ref(false)
const deleting = ref(false)
const items = ref<ScriptListItem[]>([])
const activeId = ref<string | null>(null)
const creating = ref(false)
const versions = ref<ScriptVersion[]>([])
const form = ref({ name: '', category: '', code: '' })

const filtered = computed(() =>
  search.value
    ? items.value.filter((item) => item.name.toLowerCase().includes(search.value.toLowerCase()))
    : items.value,
)

async function refresh(): Promise<void> {
  loading.value = true
  try {
    items.value = await props.adapter.list()
  } catch (error: unknown) {
    toast.error(`加载脚本失败: ${errorMessage(error)}`)
  } finally {
    loading.value = false
  }
}

function errorMessage(error: unknown): string {
  return (error as { message?: string })?.message ?? '未知错误'
}

async function open(item: ScriptListItem): Promise<void> {
  activeId.value = item.id
  creating.value = false
  versions.value = []
  try {
    const detail = await props.adapter.load(item.id)
    form.value = { name: detail.name, category: detail.category ?? '', code: detail.code }
    if (props.adapter.versions) versions.value = await props.adapter.versions(item.id)
  } catch (error: unknown) {
    toast.error(`加载脚本失败: ${errorMessage(error)}`)
  }
}

function createNew(): void {
  activeId.value = null
  creating.value = true
  versions.value = []
  form.value = { name: '', category: '', code: '' }
}

async function save(): Promise<void> {
  saving.value = true
  const payload = { name: form.value.name.trim(), category: form.value.category, code: form.value.code }
  try {
    if (activeId.value) {
      await props.adapter.save(activeId.value, payload)
      toast.success('脚本已保存并生成版本快照')
    } else {
      await props.adapter.create(payload)
      toast.success('脚本已创建')
    }
    await refresh()
  } catch (error: unknown) {
    toast.error(`保存失败: ${errorMessage(error)}`)
  } finally {
    saving.value = false
  }
}

async function remove(): Promise<void> {
  if (!activeId.value || !confirmMsg('确定删除该脚本？')) return
  deleting.value = true
  try {
    await props.adapter.remove(activeId.value)
    activeId.value = null
    creating.value = false
    form.value = { name: '', category: '', code: '' }
    await refresh()
    toast.success('已删除')
  } catch (error: unknown) {
    toast.error(`删除失败: ${errorMessage(error)}`)
  } finally {
    deleting.value = false
  }
}

function restore(version: ScriptVersion): void {
  if (typeof version.content === 'string') form.value.code = version.content
}

function confirmMsg(message: string): boolean {
  return typeof window !== 'undefined' ? window.confirm(message) : false
}

refresh()
</script>

<style scoped>
.workbench{display:grid;grid-template-columns:260px 1fr;gap:14px;height:100%;min-height:0}
.side,.editor-pane{display:flex;flex-direction:column;gap:12px;min-height:0;overflow:auto}
.side-head{display:flex;align-items:center;justify-content:space-between}
.side-head h2{margin:0;font-size:15px;color:var(--color-primary)}
.link{border:0;background:transparent;color:var(--color-primary);cursor:pointer}
.search{padding:8px 10px;border-radius:8px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary)}
.item{display:flex;flex-direction:column;gap:2px;width:100%;padding:10px;margin-top:8px;text-align:left;border:1px solid var(--border-color);border-radius:8px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.item.active{border-color:var(--color-primary)}
.item span{font-size:11px;color:var(--text-muted)}
.meta{display:flex;gap:12px;align-items:end;padding:12px 14px}
.meta label{display:flex;flex-direction:column;gap:4px;font-size:12px;color:var(--text-muted);flex:1}
.meta input{padding:8px 10px;border-radius:6px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary)}
.actions{display:flex;gap:8px}
.btn{padding:8px 14px;border-radius:7px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.btn.primary{border-color:var(--color-primary);background:var(--color-primary);color:#fff}
.btn.danger{border-color:var(--color-danger);color:var(--color-danger)}
.btn:disabled{opacity:.5;cursor:not-allowed}
.code{flex:1;min-height:280px;padding:8px;display:flex}
.code :deep(.script-code-editor){flex:1;height:100%}
.versions{padding:12px 14px}
.versions h3{margin:0 0 8px;font-size:13px;color:var(--color-secondary)}
.version{display:flex;gap:10px;align-items:baseline;width:100%;padding:6px 8px;margin-top:4px;border:1px solid var(--border-color);border-radius:6px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;text-align:left}
.version span{font-size:11px;color:var(--text-muted)}
.empty{display:grid;place-items:center;flex:1;color:var(--text-muted)}
.muted{color:var(--text-muted);font-size:12px;padding:8px 0}
@media(max-width:900px){.workbench{grid-template-columns:1fr}}
</style>
