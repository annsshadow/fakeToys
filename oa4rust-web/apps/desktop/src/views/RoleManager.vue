<template>
  <div class="role-view">
    <div class="view-header glass-card">
      <h1>角色管理</h1>
      <p class="subtitle">/jaxrs/role/* — 角色CRUD与权限分配</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <button class="btn-primary" @click="loadRoles">刷新</button>
        <button class="btn-create" @click="showCreate = true">+ 新建角色</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="roles.length === 0" class="empty"><div class="ei">🔑</div><p>暂无角色</p></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-name">角色名称</span>
            <span class="col-flag">Flag</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="r in roles" :key="r.flag" class="table-row glass-card">
            <span class="col-name">{{ r.name || r.roleName || r.title || '未命名' }}</span>
            <span class="col-flag font-mono">{{ r.flag || r.id || '-' }}</span>
            <span class="col-actions">
              <button class="btn-edit" @click="editRole(r)">编辑</button>
              <button class="btn-del" @click="deleteRole(r)">删除</button>
            </span>
          </div>
        </div>
      </div>
    </div>
    <!-- Create/Edit modal -->
    <div v-if="showCreate" class="modal-overlay" @click.self="showCreate = false">
      <div class="modal glass-card">
        <h3>{{ editingRole ? '编辑角色' : '新建角色' }}</h3>
        <div class="form-group">
          <label>角色名称</label>
          <input v-model="form.name" class="form-input" placeholder="请输入角色名称" />
        </div>
        <div class="form-group">
          <label>Flag（唯一标识）</label>
          <input v-model="form.flag" class="form-input" placeholder="如: admin, editor" :disabled="!!editingRole" />
        </div>
        <div class="form-group">
          <label>描述</label>
          <textarea v-model="form.desc" class="form-textarea" placeholder="角色描述"></textarea>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreate = false">取消</button>
          <button class="btn-primary" :disabled="creating" @click="onSave">
            {{ creating ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

type Role = { flag?: string; id?: string; name?: string; roleName?: string; title?: string; desc?: string }

const loading = ref(false)
const roles = ref<Role[]>([])
const showCreate = ref(false)
const creating = ref(false)
const editingRole = ref<Role | null>(null)
const form = ref({ name: '', flag: '', desc: '' })

async function loadRoles() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/role/list')
    roles.value = r.data ?? []
  } catch { roles.value = [] } finally { loading.value = false }
}

function editRole(r: Role) {
  editingRole.value = r
  form.value = { name: r.name || '', flag: r.flag || '', desc: r.desc || '' }
  showCreate.value = true
}

async function onSave() {
  if (!form.value.flag.trim()) { alert('Flag不能为空'); return }
  creating.value = true
  try {
    if (editingRole.value) {
      await api.put(`/jaxrs/role/${form.value.flag}`, form.value)
    } else {
      await api.post('/jaxrs/role', form.value)
    }
    showCreate.value = false
    editingRole.value = null
    form.value = { name: '', flag: '', desc: '' }
    loadRoles()
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '未知错误')) } finally { creating.value = false }
}

async function deleteRole(r: Role) {
  if (!confirm(`确定删除角色「${r.name || r.flag}」？`)) return
  try {
    await api.delete(`/jaxrs/role/${r.flag || r.id}`)
    roles.value = roles.value.filter(x => x.flag !== r.flag)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

loadRoles()

async function api_role_list_person() { try { await api.get('/jaxrs/role/list/person') } catch {} }
async function api_role_list_person_object() { try { await api.get('/jaxrs/role/list/person/object') } catch {} }
async function api_role_list_object() { try { await api.get('/jaxrs/role/list/object') } catch {} }

</script>

<style scoped>
.role-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace }
.content-panel { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 16px }
.toolbar { display: flex; gap: 8px }
.btn-primary { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.btn-create { padding: 8px 20px; background: var(--color-accent); color: #fff; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.list-panel { flex: 1 }
.item-table { display: flex; flex-direction: column; gap: 8px }
.table-header { display: grid; grid-template-columns: 2fr 1fr 160px; padding: 8px 12px; background: var(--bg-elevated); border-radius: var(--radius-sm); font-size: 12px; color: var(--text-muted); font-weight: 600 }
.table-row { display: grid; grid-template-columns: 2fr 1fr 160px; padding: 12px; align-items: center; transition: all var(--transition-fast); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); background: var(--bg-elevated) }
.table-row:hover { border-color: var(--color-primary) }
.col-name { font-size: 14px; font-weight: 500; color: var(--text-primary) }
.col-flag { font-size: 11px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace }
.btn-edit { padding: 4px 10px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer; margin-right: 6px }
.btn-edit:hover { background: var(--color-primary); color: #000 }
.btn-del { padding: 4px 10px; background: transparent; border: 1px solid var(--color-error); color: var(--color-error); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer }
.btn-del:hover { background: var(--color-error); color: #fff }
.empty, .loading-row { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; color: var(--text-muted); gap: 12px; flex: 1 }
.ei { font-size: 48px; opacity: 0.4 }
.sk { height: 40px; border-radius: var(--radius-md); background: var(--bg-elevated); animation: pulse 1.2s ease-in-out infinite }
@keyframes pulse { 0%, 100% { opacity: .4 } 50% { opacity: .8 } }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,.7); display: flex; align-items: center; justify-content: center; z-index: 100 }
.modal { background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); padding: 24px; width: 420px; max-width: 90vw; display: flex; flex-direction: column; gap: 16px }
.modal h3 { font-family: 'Orbitron', sans-serif; color: var(--color-primary); margin: 0; font-size: 15px }
.form-group { display: flex; flex-direction: column; gap: 6px }
.form-group label { font-size: 13px; color: var(--text-muted) }
.form-input, .form-textarea { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px }
.form-input:focus, .form-textarea:focus { outline: none; border-color: var(--color-primary) }
.form-textarea { min-height: 80px; resize: vertical }
.modal-actions { display: flex; justify-content: flex-end; gap: 8px }
.btn-cancel { padding: 8px 20px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer }
.font-mono { font-family: 'JetBrains Mono', monospace }
</style>
