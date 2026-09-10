<template>
  <div class="viewer-view">
    <div class="view-header glass-card">
      <div><h1>PDF查看器</h1><p class="subtitle">在线预览文件</p></div>
      <div class="header-actions">
        <input v-model="fileId" placeholder="输入文件ID" class="file-input" />
        <button class="btn-primary" @click="loadPdf" :disabled="loading">加载</button>
      </div>
    </div>
    <div class="viewer-panel glass-card" v-if="pdfUrl">
      <iframe :src="pdfUrl" class="pdf-frame" title="PDF预览" />
      <div class="page-info">文件ID: {{ fileId }} · 在线预览中</div>
    </div>
    <div class="viewer-panel glass-card" v-else-if="loading">
      <div class="loading-state"><div class="spinner"></div><p>正在加载PDF...</p></div>
    </div>
    <div class="viewer-panel glass-card" v-else>
      <div class="empty-state">
        <div class="vp-icon">📕</div>
        <p>PDF在线预览</p>
        <p class="hint">输入文件ID并点击加载，或在 /app/pdf-viewer?id=xxx 中传入参数</p>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const fileId = ref<string>((route.query.id as string) ?? '')
const pdfUrl = ref<string>('')
const loading = ref(false)

onMounted(() => {
  if (fileId.value) loadPdf()
})

async function loadPdf() {
  const fid = fileId.value.trim()
  if (!fid) return
  loading.value = true
  pdfUrl.value = ''
  try {
    await api.get(`/jaxrs/attachment/download/${fid}/stream`)
    pdfUrl.value = `/jaxrs/file/download/${fid}`
  } catch {
    pdfUrl.value = `/jaxrs/file/download/${fid}`
  } finally {
    loading.value = false
  }
}
</script>
<style scoped>
.viewer-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px;flex-wrap:wrap;gap:12px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.header-actions{display:flex;gap:8px;align-items:center}
.file-input{padding:8px 12px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);font-size:13px;width:200px}
.file-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 16px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);cursor:pointer;font-size:13px;font-weight:600}
.btn-primary:disabled{opacity:.5;cursor:not-allowed}
.viewer-panel{flex:1;padding:24px;display:flex;flex-direction:column;align-items:center;justify-content:center;overflow:auto}
.pdf-frame{width:100%;height:70vh;border:none;border-radius:var(--radius-md)}
.page-info{margin-top:12px;font-size:12px;color:var(--text-muted)}
.empty-state{text-align:center;color:var(--text-muted)}
.vp-icon{font-size:64px;margin-bottom:16px}
.hint{font-size:13px;color:var(--text-muted);margin-top:8px}
.loading-state{display:flex;flex-direction:column;align-items:center;gap:16px;color:var(--text-muted)}
.spinner{width:40px;height:40px;border:3px solid var(--border-subtle);border-top-color:var(--color-primary);border-radius:50%;animation:spin 1s linear infinite}
@keyframes spin{to{transform:rotate(360deg)}}
@media(max-width:768px){.file-input{width:140px}}
</style>
