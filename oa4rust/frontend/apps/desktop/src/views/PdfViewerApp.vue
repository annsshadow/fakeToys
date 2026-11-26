<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="viewer-view modern-page ai-page">
    <div class="view-header glass-card ai-page ai-panel">
      <div class="header-actions">
        <input v-model="fileId" placeholder="输入文件ID" class="file-input" />
        <button class="btn-primary" @click="loadPdf" :disabled="loading">加载</button>
      </div>
    </div>
    <div class="viewer-panel glass-card ai-panel" v-if="pdfUrl">
      <iframe :src="pdfUrl" class="pdf-frame" title="PDF预览" />
      <div class="page-info">文件ID: {{ fileId }} · 在线预览中</div>
    </div>
    <div class="viewer-panel glass-card" v-else-if="loading">
      <div class="loading-state"><div class="ai-skeleton-line" aria-hidden="true"></div><p>正在加载PDF...</p></div>
    </div>
    <div class="viewer-panel glass-card" v-else>
      <div class="empty-state">
        <div class="vp-icon">📕</div>
        <p>PDF在线预览</p>
        <p class="hint">输入文件ID并点击加载，试试从业务附件复制文件ID即可在线预览</p>
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
    await api.get(`/api/attachment/download/${fid}/stream`)
    pdfUrl.value = `/api/file/download/${fid}`
  } catch {
    pdfUrl.value = `/api/file/download/${fid}`
  } finally {
    loading.value = false
  }
}
</script>
<style scoped>
.viewer-view{display:flex;flex-direction:column;gap:var(--ai-grid-2);height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px;flex-wrap:wrap;gap:12px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.header-actions{display:flex;gap:8px;align-items:center}
.file-input{padding:8px 12px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);font-size:13px;width:200px}
.file-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:var(--ai-grid-1) var(--ai-grid-2);background:var(--ai-gradient-brand);color:#fff;border:none;border-radius:var(--radius-md);cursor:pointer;font-size:13px;font-weight:600;transition:background var(--ai-motion-fast) var(--ai-ease-spring)}
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

/* R306 C 组：AI 令牌深度消费（玻璃/渐变/栅格/骨架/动效），不触碰业务与接口。 */
.ai-page{display:flex;flex-direction:column;gap:var(--ai-grid-2)}
.ai-page .btn-primary{border:none;background:var(--ai-gradient-brand);color:#fff;transition:background var(--ai-motion-fast) var(--ai-ease-spring)}
@keyframes ai-shimmer{0%{background-position:200% 0}100%{background-position:-200% 0}}
.ai-page .view-header{padding:var(--ai-grid-2) var(--ai-grid-3);border:1px solid var(--ai-glass-border);border-radius:var(--ai-cmd-radius);background:var(--ai-gradient-surface)}
.ai-page .viewer-panel{border:1px solid var(--ai-glass-border);border-radius:var(--ai-cmd-radius);background:var(--ai-glass-bg)}
.ai-skeleton-line{width:120px;height:12px;border-radius:var(--radius-sm);background:var(--ai-skeleton-base);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite}
.empty-hint{margin-top:var(--ai-grid-1);color:var(--text-secondary);font-size:12px}
</style>
