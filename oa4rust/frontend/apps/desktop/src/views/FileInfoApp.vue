<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view modern-page ai-page-b5 ai-panel">
    <div class="view-header glass-card ai-page ai-panel">
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <button class="btn-primary" @click="loadFiles">刷新</button>
        <button class="btn-primary" @click="cleanUnusedFiles">清理无引用文件</button>
      </div>
      <div class="list-panel">
        <ListLoadError v-if="files.length > 0 && loadError" mode="banner" :message="loadError" @retry="loadFiles" />
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <ListLoadError v-else-if="files.length === 0 && loadError" mode="empty" :message="loadError" @retry="loadFiles" />
        <div v-else-if="files.length===0" class="empty ai-empty"><div class="ei">📎</div><p>暂无文件信息</p><p class="ai-empty-hint">点击「刷新」重新加载文件清单</p></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-name">文件名</span>
            <span class="col-doc">所属文档</span>
            <span class="col-size">大小</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="f in files" :key="f.id" class="table-row glass-card">
            <span class="col-name">
              <span class="file-icon">{{ fileIcon(f) }}</span>
              {{ f.fileName || f.name || '未知文件' }}
            </span>
            <span class="col-doc font-mono">{{ f.documentId || f.docId || '-' }}</span>
            <span class="col-size">{{ formatFileSize(f.size ?? f.fileSize) }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="downloadFile(f)">下载</button>
              <button class="btn-sm" @click="docMeta(f)">元数据</button>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import { formatFileSize } from '../utils/formatSize'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'

type FileInfo = {
  id: string
  fileName?: string
  name?: string
  documentId?: string
  docId?: string
  size?: number
  fileSize?: number
  mimeType?: string
}

const loading = ref(false)
const loadError = ref('')
const files = ref<FileInfo[]>([])

function fileIcon(f: FileInfo) {
  const mime = f.mimeType || ''
  if (mime.includes('pdf')) return '📕'
  if (mime.includes('image')) return '🖼️'
  if (mime.includes('word') || mime.includes('doc')) return '📘'
  if (mime.includes('excel') || mime.includes('sheet')) return '📗'
  return '📄'
}

async function downloadFile(f: FileInfo) {
  try {
    const r = await api.get(`/api/fileinfo/download/document/${f.id}`)
    // 安全审计（2026-10）：noopener,noreferrer 防止新窗口经 window.opener 反向
    // 操作本页（tab-nabbing / 焦点劫持），即便 URL 由后端签发。
    if (r.data?.url) window.open(r.data.url, '_blank', 'noopener,noreferrer')
    else toast.info('下载链接未生成')
  } catch (e: any) {
    toast.error(`下载失败: : ${e?.message ?? ''}`)
  }
}

async function loadFiles() {
  loading.value = true
  loadError.value = ''
  try {
    const r = await api.get('/api/fileinfo/list/all')
    files.value = r.data ?? []
  } catch (e) {
    // 失败保留旧数据并显式报错，不能和「真的没有文件」混成同一种空态。
    loadError.value = describeListError('文件信息加载失败', e)
  } finally {
    loading.value = false
  }
}
// rev449：读某文件在指定文档下的元数据（fileinfo_id_document_documentId Path<(file_id,doc_id)> 查 x_cms_fileinfo）
async function docMeta(f: FileInfo) {
  const docId = f.documentId || f.docId || prompt('文档 ID:', '') || ''
  if (!docId.trim()) return
  try {
    const r: any = await api.get(`/api/fileinfo/${encodeURIComponent(f.id)}/document/${encodeURIComponent(docId)}`)
    toast.success(`元数据：${(r as any)?.data?.originalName ?? (r as any)?.data?.id ?? '无'}`)
  } catch (e: any) {
    toast.error(`读取元数据失败: ${e?.message ?? ''}`)
  }
}
// rev449：清理 cmsdocument_manage 无引用文件（file_clean_unused_referencetype_cmsdocument_manage pool-only，DELETE FILE_FILE by reference_type）
async function cleanUnusedFiles() {
  if (!(await confirmMsg('确定清理 cmsdocument_manage 引用类型的无用文件？'))) return
  try {
    const r: any = await api.get('/api/file/clean/unused/referencetype/cmsdocument/manage')
    toast.success(`已清理无引用文件：${(r as any)?.data?.deleted ?? (r as any)?.data?.count ?? 0}`)
  } catch (e: any) {
    toast.error(`清理失败: ${e?.message ?? ''}`)
  }
}

loadFiles()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:var(--ai-grid-2);height:100%;padding:var(--ai-grid-3);background:var(--ai-gradient-surface);border-radius:var(--ai-grid-2)}
.view-header{padding:var(--ai-grid-2) var(--ai-grid-3);background:var(--ai-gradient-surface);border:1px solid var(--ai-glass-border);backdrop-filter:var(--ai-glass-blur);border-radius:var(--ai-grid-2)}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:var(--ai-grid-2);display:flex;flex-direction:column;gap:var(--ai-grid-2);background:var(--ai-glass-bg);border:1px solid var(--ai-glass-border);backdrop-filter:var(--ai-glass-blur);box-shadow:var(--ai-glass-shadow);border-radius:var(--ai-grid-2)}
.toolbar{display:flex;gap:8px}
.btn-primary{padding:8px 20px;background:var(--ai-gradient-brand);color:#fff;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600;box-shadow:var(--ai-shadow-1);transition:transform var(--ai-motion-fast) var(--ai-ease-spring)}
.btn-primary:hover{transform:translateY(-1px)}
.list-panel{flex:1}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1.5fr 80px 100px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1.5fr 80px 100px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary);display:flex;align-items:center;gap:8px}
.file-icon{font-size:16px}
.col-doc,.col-size{font-size:12px;color:var(--text-muted)}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:linear-gradient(90deg,var(--ai-skeleton-base) 25%,var(--ai-skeleton-shine) 50%,var(--ai-skeleton-base) 75%);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite}
@keyframes ai-shimmer{0%{background-position:200% 0}100%{background-position:-200% 0}}
.font-mono{font-family:'JetBrains Mono',monospace}
.ai-empty{display:flex;flex-direction:column;align-items:center;justify-content:center;gap:var(--ai-grid-1);padding:var(--ai-grid-6);text-align:center;color:var(--text-secondary);background:var(--ai-glass-bg);border:1px dashed var(--ai-glass-border);border-radius:var(--ai-grid-2)}
.ai-empty-hint{font-size:12px;opacity:.75}
</style>
