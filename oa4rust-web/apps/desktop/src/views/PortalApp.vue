<template>
  <div class="portal-view">
    <div class="view-header glass-card">
      <h1>门户管理</h1>
      <p class="subtitle">接入 /jaxrs/portal/* — 页面设计与发布</p>
      <button class="new-page-btn" @click="showEditor = true">+ 新建页面</button>
    </div>
    <div class="page-grid glass-card">
      <div v-if="pages.length === 0" class="empty-state">
        <div class="empty-icon">📄</div>
        <p>暂无门户页面</p>
      </div>
      <div v-for="page in pages" :key="page.id" class="page-card" @click="editPage(page)">
        <div class="page-preview">{{ page.icon || '📄' }}</div>
        <div class="page-info">
          <div class="page-name">{{ page.name || page.title || '未命名页面' }}</div>
          <div class="page-meta">{{ page.createTime || '未知时间' }}</div>
        </div>
        <div class="page-actions">
          <button class="icon-btn" @click.stop="publishPage(page)" title="发布">📤</button>
          <button class="icon-btn danger" @click.stop="deletePage(page.id)" title="删除">🗑</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { confirmMsg } from '../utils/toast';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { api } from '@oa4rust/sdk';

interface PortalPage {
  id: string;
  name?: string;
  title?: string;
  icon?: string;
  createTime?: string;
  appId?: string;
}

const pages = ref<PortalPage[]>([]);
const showEditor = ref(false);
const queryClient = useQueryClient();

const { data } = useQuery({
  queryKey: ['portal', 'pages'],
  queryFn: async () => {
    const resp = await api.get('/jaxrs/portal/assemble/surface/page/list/default');
    return ((resp as any)?.data ?? []) as PortalPage[];
  },
  staleTime: 60_000,
});
pages.value = data.value ?? [];

const deleteMutation = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/portal/assemble/surface/page/${id}`),
  onSuccess: () => {
    pages.value = pages.value.filter(p => p.id !== id);
    queryClient.invalidateQueries({ queryKey: ['portal', 'pages'] });
  },
});

function editPage(_page: PortalPage): void {
  // Navigate to portal designer (future)

}

function publishPage(_page: PortalPage): void {

}

function deletePage(id: string): void {
  if (confirmMsg('确定删除此页面？')) deleteMutation.mutate(id);
}
</script>

<style scoped>
.portal-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px; }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0; }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 4px 0 0; font-family: 'JetBrains Mono', monospace; }
.new-page-btn { padding: 8px 16px; border-radius: var(--radius-md); border: none; background: var(--color-primary); color: white; cursor: pointer; font-weight: 600; }
.page-grid { flex: 1; overflow-y: auto; padding: 16px; display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px; }
.page-card { padding: 16px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); cursor: pointer; transition: all var(--transition-fast); position: relative; }
.page-card:hover { border-color: var(--border-active); transform: translateY(-2px); box-shadow: var(--shadow-glow); }
.page-preview { font-size: 40px; margin-bottom: 8px; }
.page-info { }
.page-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.page-meta { font-size: 11px; color: var(--text-muted); margin-top: 4px; }
.page-actions { position: absolute; top: 8px; right: 8px; display: flex; gap: 4px; opacity: 0; transition: opacity var(--transition-fast); }
.page-card:hover .page-actions { opacity: 1; }
.icon-btn { background: var(--bg-elevated); border: 1px solid var(--border-subtle); width: 28px; height: 28px; border-radius: var(--radius-sm); cursor: pointer; font-size: 14px; display: flex; align-items: center; justify-content: center; }
.icon-btn:hover { border-color: var(--color-primary); }
.icon-btn.danger:hover { border-color: var(--color-error); }
.empty-state { grid-column: 1/-1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px; color: var(--text-muted); gap: 12px; }
.empty-icon { font-size: 48px; opacity: 0.4; }
</style>
