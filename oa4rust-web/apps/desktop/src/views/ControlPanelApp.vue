<template>
  <div class="nav-view">
    <div class="view-header glass-card">
      <div><h1>控制面板</h1><p class="subtitle">快速导航到各个设计器</p></div>
    </div>
    <div class="content-panel glass-card">
      <div class="nav-grid">
        <a class="nav-card" href="/app/process-designer"><div class="nc-icon">⚡</div><div class="nc-title">流程设计器</div><div class="nc-desc">可视化流程编排</div></a>
        <a class="nav-card" href="/app/form-designer"><div class="nc-icon">📋</div><div class="nc-title">表单设计器</div><div class="nc-desc">JSON表单构建</div></a>
        <a class="nav-card" href="/app/query-designer"><div class="nc-icon">🔍</div><div class="nc-title">查询设计器</div><div class="nc-desc">SQL与可视化查询</div></a>
        <a class="nav-card" href="/app/portal-designer"><div class="nc-icon">🖼</div><div class="nc-title">门户设计器</div><div class="nc-desc">页面与组件设计</div></a>
        <a class="nav-card" href="/app/cms-form-designer"><div class="nc-icon">📝</div><div class="nc-title">CMS表单设计</div><div class="nc-desc">内容模型表单</div></a>
        <a class="nav-card" href="/app/cms-view-designer"><div class="nc-icon">👁</div><div class="nc-title">CMS视图设计</div><div class="nc-desc">内容展示视图</div></a>
        <a class="nav-card" href="/app/cms-script-designer"><div class="nc-icon">💻</div><div class="nc-title">CMS脚本设计</div><div class="nc-desc">内容处理脚本</div></a>
        <a class="nav-card" href="/app/cms-dict-designer"><div class="nc-icon">📚</div><div class="nc-title">CMS字典设计</div><div class="nc-desc">内容字典配置</div></a>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
// DesignCenter - navigation hub for all designers
</script>
<style scoped>
.nav-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.content-panel{padding:24px}
.nav-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:16px}
.nav-card{display:flex;flex-direction:column;align-items:center;padding:24px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);text-decoration:none;color:inherit;transition:all var(--transition-fast);cursor:pointer}
.nav-card:hover{border-color:var(--color-primary);transform:translateY(-2px);box-shadow:0 4px 20px rgba(0,212,255,0.1)}
.nc-icon{font-size:32px;margin-bottom:8px}
.nc-title{font-size:14px;font-weight:600;color:var(--text-primary)}
.nc-desc{font-size:12px;color:var(--text-muted);margin-top:4px}
</style>