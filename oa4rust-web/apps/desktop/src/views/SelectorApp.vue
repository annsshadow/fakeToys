<template>
  <div class="selector-view">
    <div class="view-header glass-card">
      <div>
        <h1>组织选择器</h1>
        <p class="subtitle">搜索并选择真实组织、人员或身份数据</p>
      </div>
      <label class="mode-field">
        选择模式
        <select v-model="mode" class="mode-select">
          <option value="multiple">多选</option>
          <option value="single">单选</option>
        </select>
      </label>
    </div>

    <div class="content-panel glass-card">
      <OrganizationSelector v-model="selection" :mode="mode" :types="['unit', 'person', 'identity']" />
    </div>

    <div class="contract-panel glass-card">
      <h2>当前值</h2>
      <p>v-model 返回带有 <code>type</code>、<code>id</code> 与 <code>name</code> 的选择项。</p>
      <pre>{{ selection }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import { OrganizationSelector, type OrganizationSelectorItem, type OrganizationSelectorMode } from '@oa4rust/ui'
import { ref, watch } from 'vue'

const mode = ref<OrganizationSelectorMode>('multiple')
const selection = ref<OrganizationSelectorItem[]>([])

watch(mode, (value) => {
  if (value === 'single' && selection.value.length > 1) {
    selection.value = selection.value.slice(0, 1)
  }
})
</script>

<style scoped>
.selector-view {
  display: grid;
  gap: 16px;
}
.view-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 16px 24px;
}
.view-header h1 {
  margin: 0 0 4px;
  color: var(--color-primary);
  font-family: 'Orbitron', sans-serif;
  font-size: 20px;
  text-shadow: 0 0 15px var(--color-primary-glow);
}
.subtitle,
.contract-panel p {
  margin: 0;
  color: var(--text-muted);
  font-size: 12px;
}
.mode-field {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-muted);
  font-size: 12px;
}
.mode-select {
  padding: 7px 10px;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  color: var(--text-primary);
}
.content-panel,
.contract-panel {
  padding: 20px 24px;
}
.contract-panel h2 {
  margin: 0 0 6px;
  color: var(--text-primary);
  font-size: 14px;
}
.contract-panel pre {
  min-height: 48px;
  margin: 12px 0 0;
  padding: 12px;
  overflow: auto;
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  color: var(--color-secondary);
  font-family: 'Fira Code', monospace;
  font-size: 12px;
}
@media (max-width: 640px) {
  .view-header {
    gap: 12px;
    flex-direction: column;
  }
}
</style>
