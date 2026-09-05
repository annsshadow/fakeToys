<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>文件信息</h1>
      <p class="subtitle">/jaxrs/fileinfo/* — 文档级文件元数据</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <button class="btn-primary" @click="loadFiles">刷新</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="files.length===0" class="empty"><div class="ei">📎</div><p>暂无文件信息</p></div>
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
            <span class="col-size">{{ formatSize(f.size || f.fileSize) }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="downloadFile(f)">下载</button>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { toast } from '../utils/toast'
import { api } from '@oa4rust/sdk'

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
const files = ref<FileInfo[]>([])

function fileIcon(f: FileInfo) {
  const mime = f.mimeType || ''
  if (mime.includes('pdf')) return '📕'
  if (mime.includes('image')) return '🖼️'
  if (mime.includes('word') || mime.includes('doc')) return '📘'
  if (mime.includes('excel') || mime.includes('sheet')) return '📗'
  return '📄'
}

function formatSize(bytes?: number) {
  if (!bytes) return '-'
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

async function downloadFile(f: FileInfo) {
  try {
    const r = await api.get(`/jaxrs/fileinfo/download/document/${f.id}`)
    if (r.data?.url) window.open(r.data.url, '_blank')
    else toast.info('下载链接未生成')
  } catch (e: any) { toast.error('下载失败: : ' + (e?.message ?? '')) }
}

async function loadFiles() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/fileinfo/list/all')
    files.value = r.data ?? []
  } catch { files.value = [] } finally { loading.value = false }
}

loadFiles()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
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
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
