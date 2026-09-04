<template>
  <div class="viewer-view">
    <div class="view-header glass-card">
      <div><h1>PDF查看器</h1><p class="subtitle">PDF在线查看</p></div>
    </div>
    <div class="viewer-panel glass-card">
      <div class="viewer-placeholder">
        <div class="vp-icon">📕</div>
        <p>PDF查看器</p>
        <p class="hint">支持在线预览PDF文件</p>
        <div class="vp-controls">
          <button class="btn-secondary" disabled>⬅ 上一页</button>
          <span class="page-info">第 1 页 / 共 -- 页</span>
          <button class="btn-secondary" disabled>下一页 ➡</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
// PdfViewer - PDF online viewer (placeholder for pdfjs integration)
</script>
<style scoped>
.viewer-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.viewer-panel{flex:1;padding:24px;display:flex;align-items:center;justify-content:center}
.viewer-placeholder{text-align:center;color:var(--text-muted)}
.vp-icon{font-size:64px;margin-bottom:16px}
.vp-controls{display:flex;align-items:center;gap:16px;margin-top:16px}
.page-info{font-size:13px;color:var(--text-primary)}
.btn-secondary{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.hint{font-size:12px;margin-top:8px}
</style>