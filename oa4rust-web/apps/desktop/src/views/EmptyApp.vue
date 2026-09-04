<template>
  <div class="placeholder-view">
    <div class="view-header glass-card">
      <div><h1>占位页面</h1><p class="subtitle">Empty</p></div>
    </div>
    <div class="content-panel glass-card" style="padding:60px;text-align:center">
      <div class="ph-icon">⬜</div>
      <h2 style="color:var(--text-muted);font-weight:400;margin:16px 0">开发中</h2>
      <p style="color:var(--text-muted);font-size:13px">Empty 正在实现中，敬请期待</p>
    </div>
  </div>
</template>
<script setup lang="ts">
// 占位页面 - Placeholder view
</script>
<style scoped>
.placeholder-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.content-panel{flex:1}
.ph-icon{font-size:48px}
</style>