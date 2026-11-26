<script setup lang="ts">
/**
 * 共享骨架屏（AI_VISUAL_SYSTEM.md §6 / P6）。
 * 加载态先给结构再给数据，避免整页 v-loading 灰遮罩造成的布局跳动（CLS）。
 * 宽度按主 70% / 次 45% / 末 25% 分层，shimmer 复用 main.css 的 .skeleton-bar。
 */
</script>

<template>
  <div class="page-skeleton" role="status" aria-busy="true" aria-label="数据加载中">
    <span class="sr-only">数据加载中</span>
    <div class="sk-head">
      <span class="skeleton-bar" style="width: 28%"></span>
      <span class="skeleton-bar" style="width: 45%"></span>
    </div>
    <div class="sk-kpis">
      <div v-for="i in 4" :key="'k' + i" class="sk-card">
        <span class="skeleton-bar" style="width: 45%"></span>
        <span class="skeleton-bar sk-num" style="width: 70%"></span>
        <span class="skeleton-bar" style="width: 25%"></span>
      </div>
    </div>
    <div class="sk-table">
      <div class="skeleton-row sk-thead">
        <span class="skeleton-bar" style="width: 25%"></span>
        <span class="skeleton-bar" style="width: 45%"></span>
        <span class="skeleton-bar" style="width: 25%"></span>
      </div>
      <div v-for="i in 6" :key="'r' + i" class="skeleton-row">
        <span class="skeleton-bar" style="width: 70%"></span>
        <span class="skeleton-bar" style="width: 45%"></span>
        <span class="skeleton-bar" style="width: 25%"></span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-skeleton { padding: 4px 0 12px; }
.sr-only {
  position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px;
  overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;
}
.sk-head { display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px; }
.sk-head .skeleton-bar:first-child { height: 18px; }
.sk-kpis { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 12px; margin-bottom: 16px; }
.sk-card {
  display: flex; flex-direction: column; gap: 10px;
  padding: 18px 16px; border-radius: var(--lyp-radius);
  background: var(--lyp-panel); border: 1px solid var(--lyp-border-soft);
}
.sk-num { height: 22px; }
.sk-table {
  padding: 16px; border-radius: var(--lyp-radius);
  background: var(--lyp-panel); border: 1px solid var(--lyp-border-soft);
}
.sk-thead { border-bottom: 1px solid var(--lyp-border-soft); padding-bottom: 12px; margin-bottom: 4px; }
@media (max-width: 900px) {
  .sk-kpis { grid-template-columns: repeat(2, minmax(0, 1fr)); }
}
</style>
