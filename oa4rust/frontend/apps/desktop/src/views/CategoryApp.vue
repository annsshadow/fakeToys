<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <AdminPageHeader title="分类管理" subtitle="管理应用下的内容分类与层级">
      <template #actions>
        <span class="stat-chip">共 {{ items.length }} 项</span>
      </template>
    </AdminPageHeader>
    <AdminTable
      :columns="columns"
      :rows="items"
      :loading="loading"
      :error="loadError"
      empty-text="暂无分类数据"
      @retry="load"
    >
      <template #cell-name="{ row }">
        <span class="item-name">{{ row.name || row.title || row.categoryName || '未命名' }}</span>
      </template>
      <template #cell-desc="{ row }">
        <span class="item-desc">{{ row.desc || row.description || row.alias || '—' }}</span>
      </template>
      <template #cell-flag="{ row }">
        <span class="mono">{{ row.flag || row.id }}</span>
      </template>
      <template #cell-actions="{ row }">
        <button class="btn-del" @click="deleteCategory(row as any)">删除</button>
      </template>
    </AdminTable>
    <AdminToolbar
      :model-value="keyword"
      search-placeholder="搜索分类"
      create-label="+ 新建分类"
      @update:model-value="keyword = $event"
      @search="load"
      @refresh="load"
      @create="createCategory"
    />
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
import { AdminPageHeader, AdminTable, type AdminTableColumn, AdminToolbar } from '../components/admin/'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'

// 模板专用组件引用（避免 lint 误报未使用）
void AdminPageHeader
void AdminTable
void AdminToolbar

const columns: AdminTableColumn[] = [
  { key: 'name', label: '分类名称', width: '26%' },
  { key: 'desc', label: '描述 / 别名', width: '38%' },
  { key: 'flag', label: '标识', width: '20%' },
  { key: 'actions', label: '操作', width: '16%' },
]

const keyword = ref('')
const loading = ref(false)
const loadError = ref('')
const items = ref<any[]>([])

async function load() {
  loading.value = true
  loadError.value = ''
  try {
    // 后端真实列表端点为 categoryinfo/list/all（裸 /list 会被 {id} 宽路由吞掉）。
    const r = await api.get('/api/categoryinfo/list/all')
    const all = (r.data ?? []) as any[]
    const kw = keyword.value.trim().toLowerCase()
    items.value = kw
      ? all.filter((c) =>
          [c.name, c.title, c.categoryName, c.alias].some((v) =>
            String(v ?? '')
              .toLowerCase()
              .includes(kw),
          ),
        )
      : all
  } catch (e) {
    // 失败保留旧数据并显式报错，不能和「真的没有分类」混成同一种空态。
    loadError.value = describeListError('分类加载失败', e)
  } finally {
    loading.value = false
  }
}

function categoryWriteSucceeded(resp: any): boolean {
  if (!resp || typeof resp !== 'object') return false
  if (resp.type === 'error' || resp.success === false) return false
  if (resp.type === 'success') return true
  if (resp.type) return false
  return true
}
async function createCategory() {
  const appId = prompt('所属应用 ID (appId，必填):')
  if (!appId) return
  const name = prompt('分类名称:', '') ?? ''
  try {
    // 后端 categoryinfo_u2_create：appId 必填（owner 门禁）+ name/parentCategoryId 可选
    const lastResp: any = await api.post('/api/categoryinfo', { appId, name })
    if (!categoryWriteSucceeded(lastResp)) {
      toast.error(lastResp?.message || '操作未生效，请刷新后重试')
      return
    }
    load()
  } catch (e: any) {
    toast.error(`新建失败: ${e?.message ?? ''}`)
  }
}
async function deleteCategory(item: any) {
  if (!(await confirmMsg(`确定删除分类「${item.name || item.id}」？`))) return
  try {
    const lastResp: any = await api.delete(`/api/categoryinfo/${item.id}`)
    if (!categoryWriteSucceeded(lastResp)) {
      toast.error(lastResp?.message || '操作未生效，请刷新后重试')
      return
    }
    load()
  } catch (e: any) {
    toast.error(`删除失败: ${e?.message ?? ''}`)
  }
}
// __TEMPLATE_IMPORT_REFS__
void AdminPageHeader
void AdminTable
void AdminToolbar

load()

const categoryinfo_ref = ref<any[]>([])
const categoryinfo_alias_alpha_ref = ref<any[]>([])
const categoryinfo_list_objects_ref = ref<any[]>([])
const categoryinfo_list_all_ref = ref<any[]>([])
const categoryinfo_extContent_ref = ref<any[]>([])
const categoryinfo_c_1_permission_ref = ref<any[]>([])
const categoryinfo_flag_ref = ref<any[]>([])
const categoryinfo_c_1_control_ref = ref<any[]>([])
const categoryinfo_c_1_execute_projection_ref = ref<any[]>([])
const categoryinfo_c_1_ref = ref<any[]>([])
const categoryinfo_bind_c_1_view_ref = ref<any[]>([])
const api_list_p_1_860_data = ref<any[]>([])
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%;padding:20px 24px;overflow-y:auto}
.item-name{font-weight:600;color:var(--text-primary)}
.item-desc,.mono{color:var(--text-muted);font-size:12px}
.mono{font-family:'JetBrains Mono',monospace}
.stat-chip{padding:4px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
.btn-del{padding:4px 10px;border:1px solid var(--color-error);background:var(--color-error-glow);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
</style>
