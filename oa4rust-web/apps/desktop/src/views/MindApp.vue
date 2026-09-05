<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>思维导图</h1>
      <p class="subtitle">/jaxrs/mind/*</p>
    </div>
    <div class="split-panel">
      <!-- 左侧目录树 -->
      <div class="tree-panel glass-card">
        <div class="tree-toolbar">
          <button class="btn-sm" @click="loadFolders">刷新</button>
        </div>
        <div v-if="loadingFolder" class="tree-loading">加载中...</div>
        <ul v-else class="tree">
          <li v-for="f in folders" :key="f.id" class="tree-node">
            <span class="node-label" @click="selectFolder(f)">{{ f.name || f.title || '未命名目录' }}</span>
            <span class="node-children" v-if="hasChildren(f)">{{ expandedFolders.has(f.id) ? '▼' : '▶' }}</span>
            <ul v-if="expandedFolders.has(f.id)" class="tree-children">
              <li v-for="c in (f.children || [])" :key="c.id" class="tree-node child">
                <span class="node-label" @click="selectFolder(c)">{{ c.name || c.title || '未命名' }}</span>
              </li>
            </ul>
          </li>
        </ul>
        <div v-if="!loadingFolder && folders.length === 0" class="tree-empty">暂无目录</div>
      </div>
      <!-- 右侧内容 -->
      <div class="content-panel glass-card">
        <div class="content-header">
          <h3>{{ currentFolder?.name || '全部思维导图' }}</h3>
          <span class="count">{{ minds.length }} 个导图</span>
        </div>
        <div v-if="loadingMinds" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="minds.length === 0" class="empty"><div class="ei">🧠</div><p>该目录下暂无思维导图</p></div>
        <div v-else class="mind-grid">
          <div v-for="m in minds" :key="m.id" class="mind-card glass-card" @click="viewMind(m)">
            <div class="mc-icon">🧠</div>
            <div class="mc-info">
              <div class="mc-title">{{ m.title || m.name || '未命名导图' }}</div>
              <div class="mc-meta">更新: {{ formatDate(m.updatedAt || m.updateTime) }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Mind detail modal -->
    <div v-if="selectedMind" class="modal-overlay" @click.self="selectedMind=null">
      <div class="modal glass-card">
        <div class="modal-header">
          <h3>{{ selectedMind.title || selectedMind.name }}</h3>
          <button class="btn-close" @click="selectedMind=null">✕</button>
        </div>
        <pre class="mind-json">{{ formatMindJson(selectedMind) }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { toast } from '../utils/toast';
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

type Folder = { id: string; name?: string; title?: string; children?: Folder[] }
type MindItem = { id: string; title?: string; name?: string; updatedAt?: string; updateTime?: string; [k: string]: unknown }

const loadingFolder = ref(false)
const loadingMinds = ref(false)
const folders = ref<Folder[]>([])
const currentFolder = ref<Folder | null>(null)
const minds = ref<MindItem[]>([])
const expandedFolders = ref(new Set<string>())
const selectedMind = ref<MindItem | null>(null)

function hasChildren(f: Folder) { return (f.children?.length ?? 0) > 0 }

function toggleFolder(f: Folder) {
  const s = expandedFolders.value
  if (s.has(f.id)) s.delete(f.id); else s.add(f.id)
  expandedFolders.value = new Set(s)
}

function selectFolder(f: Folder) {
  if (hasChildren(f)) toggleFolder(f)
  currentFolder.value = f
  loadMinds(f.id)
}

async function loadFolders() {
  loadingFolder.value = true
  try {
    const r = await api.get('/jaxrs/mind/folder/tree/my')
    folders.value = r.data ?? []
  } catch { folders.value = [] } finally { loadingFolder.value = false }
}

async function loadMinds(folderId: string) {
  loadingMinds.value = true
  try {
    const r = await api.get(`/jaxrs/mind/mind/list/${folderId}/1`)
    minds.value = r.data?.list ?? r.data ?? []
  } catch { minds.value = [] } finally { loadingMinds.value = false }
}

function viewMind(m: MindItem) { selectedMind.value = m }
function formatDate(d?: string) { return d ? new Date(d).toLocaleString('zh-CN') : '-' }
function formatMindJson(m: MindItem) {
  const { title, name, updatedAt, updateTime, ...rest } = m
  return JSON.stringify(rest, null, 2)
}

loadFolders()

const api_entity_f_578_data = ref<any[]>([]);
const { data: api_entity_f_578_q } = useQuery({queryKey: ['api_entity_f_578', '/jaxrs/mind/core/entity/folder/folder-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/core/entity/folder/folder-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_f_578_q, (v) => { api_entity_f_578_data.value = v ?? []; });
const api_mind_version_data = ref<any[]>([]);
const { data: api_mind_version_q } = useQuery({queryKey: ['api_mind_version', '/jaxrs/mind/version'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/version"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_version_q, (v) => { api_mind_version_data.value = v ?? []; });
const api_mind_mind_data = ref<any[]>([]);
const { data: api_mind_mind_q } = useQuery({queryKey: ['api_mind_mind', '/jaxrs/mind/mind'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/mind"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_mind_q, (v) => { api_mind_mind_data.value = v ?? []; });
const api_folder_x_force_data = ref<any[]>([]);
const { data: api_folder_x_force_q } = useQuery({queryKey: ['api_folder_x_force', '/jaxrs/mind/assemble/control/folder/x/force'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/folder/x/force"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_folder_x_force_q, (v) => { api_folder_x_force_data.value = v ?? []; });
const api_mind_recycle_x_data = ref<any[]>([]);
const { data: api_mind_recycle_x_q } = useQuery({queryKey: ['api_mind_recycle_x', '/jaxrs/mind/assemble/control/mind/recycle/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/recycle/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_recycle_x_q, (v) => { api_mind_recycle_x_data.value = v ?? []; });
const api_mind_view_x_data = ref<any[]>([]);
const { data: api_mind_view_x_q } = useQuery({queryKey: ['api_mind_view_x', '/jaxrs/mind/assemble/control/mind/view/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/view/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_view_x_q, (v) => { api_mind_view_x_data.value = v ?? []; });
const api_mind_core_list_data = ref<any[]>([]);
const { data: api_mind_core_list_q } = useQuery({queryKey: ['api_mind_core_list', '/jaxrs/mind/core/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/core/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_core_list_q, (v) => { api_mind_core_list_data.value = v ?? []; });
const api_entity_m_376_data = ref<any[]>([]);
const { data: api_entity_m_376_q } = useQuery({queryKey: ['api_entity_m_376', '/jaxrs/mind/core/entity/mind/mind-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/core/entity/mind/mind-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_m_376_q, (v) => { api_entity_m_376_data.value = v ?? []; });
const api_mind_map_data = ref<any[]>([]);
const { data: api_mind_map_q } = useQuery({queryKey: ['api_mind_map', '/jaxrs/mind/map'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/map"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_map_q, (v) => { api_mind_map_data.value = v ?? []; });
const api_control__997_data = ref<any[]>([]);
const { data: api_control__997_q } = useQuery({queryKey: ['api_control__997', '/jaxrs/mind/assemble/control/folder/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/folder/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__997_q, (v) => { api_control__997_data.value = v ?? []; });
const api_mind_folder_data = ref<any[]>([]);
const { data: api_mind_folder_q } = useQuery({queryKey: ['api_mind_folder', '/jaxrs/mind/folder'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/folder"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_folder_q, (v) => { api_mind_folder_data.value = v ?? []; });
const api_control__448_data = ref<any[]>([]);
const { data: api_control__448_q } = useQuery({queryKey: ['api_control__448', '/jaxrs/mind/assemble/control/mind/save'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/save"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__448_q, (v) => { api_control__448_data.value = v ?? []; });
const api_list_x_s_620_data = ref<any[]>([]);
const { data: api_list_x_s_620_q } = useQuery({queryKey: ['api_list_x_s_620', '/jaxrs/mind/assemble/control/mind/list/x/shareRecords'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/list/x/shareRecords"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_x_s_620_q, (v) => { api_list_x_s_620_data.value = v ?? []; });
const api_assemble_184_data = ref<any[]>([]);
const { data: api_assemble_184_q } = useQuery({queryKey: ['api_assemble_184', '/jaxrs/mind/assemble/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_184_q, (v) => { api_assemble_184_data.value = v ?? []; });
const api_folder_tree_my_data = ref<any[]>([]);
const { data: api_folder_tree_my_q } = useQuery({queryKey: ['api_folder_tree_my', '/jaxrs/mind/assemble/control/folder/tree/my'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/folder/tree/my"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_folder_tree_my_q, (v) => { api_folder_tree_my_data.value = v ?? []; });
const api_mind_x_icon_data = ref<any[]>([]);
const { data: api_mind_x_icon_q } = useQuery({queryKey: ['api_mind_x_icon', '/jaxrs/mind/assemble/control/mind/x/icon'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/x/icon"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_x_icon_q, (v) => { api_mind_x_icon_data.value = v ?? []; });
const api_entity_f_454_data = ref<any[]>([]);
const { data: api_entity_f_454_q } = useQuery({queryKey: ['api_entity_f_454', '/jaxrs/mind/core/entity/folder/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/core/entity/folder/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_f_454_q, (v) => { api_entity_f_454_data.value = v ?? []; });
const api_list_x_version_data = ref<any[]>([]);
const { data: api_list_x_version_q } = useQuery({queryKey: ['api_list_x_version', '/jaxrs/mind/assemble/control/mind/list/x/version'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/list/x/version"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_x_version_q, (v) => { api_list_x_version_data.value = v ?? []; });
const api_mind_ass_232_data = ref<any[]>([]);
const { data: api_mind_ass_232_q } = useQuery({queryKey: ['api_mind_ass_232', '/jaxrs/mind/assemble/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_ass_232_q, (v) => { api_mind_ass_232_data.value = v ?? []; });
const api_share_x_cancel_data = ref<any[]>([]);
const { data: api_share_x_cancel_q } = useQuery({queryKey: ['api_share_x_cancel', '/jaxrs/mind/assemble/control/mind/share/x/cancel'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/share/x/cancel"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_share_x_cancel_q, (v) => { api_share_x_cancel_data.value = v ?? []; });


const mind_core_entity_version_ref = ref<any[]>([]);
const mind_core_entity_version_q = useQuery({
  queryKey: ['mind_core_entity_version'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/core/entity/version"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_config_update_ref = ref<any[]>([]);
const assemble_control_config_update_q = useQuery({
  queryKey: ['assemble_control_config_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/assemble/control/config/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mind_map_list_ref = ref<any[]>([]);
const mind_map_list_q = useQuery({
  queryKey: ['mind_map_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/map/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_folder_save_ref = ref<any[]>([]);
const assemble_control_folder_save_q = useQuery({
  queryKey: ['assemble_control_folder_save'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/assemble/control/folder/save"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mind_core_entity_list_ref = ref<any[]>([]);
const mind_core_entity_list_q = useQuery({
  queryKey: ['mind_core_entity_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/core/entity/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_control_mind_x_ref = ref<any[]>([]);
const assemble_control_mind_x_q = useQuery({
  queryKey: ['assemble_control_mind_x'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/assemble/control/mind/x"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mind_core_entity_mind_ref = ref<any[]>([]);
const mind_core_entity_mind_q = useQuery({
  queryKey: ['mind_core_entity_mind'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/core/entity/mind"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const mind_core_entity_folder_ref = ref<any[]>([]);
const mind_core_entity_folder_q = useQuery({
  queryKey: ['mind_core_entity_folder'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/mind/core/entity/folder"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_control__696_data = ref<any[]>([]);
const { data: api_control__696_q } = useQuery({queryKey: ['api_control__696', '/jaxrs/mind/assemble/control/mind/x/destoryrecycle'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/x/destoryrecycle"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__696_q, (v) => { api_control__696_data.value = v ?? []; });
const api_entity_v_388_data = ref<any[]>([]);
const { data: api_entity_v_388_q } = useQuery({queryKey: ['api_entity_v_388', '/jaxrs/mind/core/entity/version/list/mind-001'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/core/entity/version/list/mind-001"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_v_388_q, (v) => { api_entity_v_388_data.value = v ?? []; });
const api_control__12_data = ref<any[]>([]);
const { data: api_control__12_q } = useQuery({queryKey: ['api_control__12', '/jaxrs/mind/assemble/control/mind/restore/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/restore/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__12_q, (v) => { api_control__12_data.value = v ?? []; });
const api_mind_ass_989_data = ref<any[]>([]);
const { data: api_mind_ass_989_q } = useQuery({queryKey: ['api_mind_ass_989', '/jaxrs/mind_assemble_control'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind_assemble_control"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_mind_ass_989_q, (v) => { api_mind_ass_989_data.value = v ?? []; });
const api_control_mind_ver_254_data = ref<any[]>([]);
const { data: api_control_mind_ver_254_q } = useQuery({queryKey: ['api_control_mind_ver_254', '/jaxrs/mind/assemble/control/mind/version/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/version/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_mind_ver_254_q, (v) => { api_control_mind_ver_254_data.value = v ?? []; });
const api_control_mind_x_d_527_data = ref<any[]>([]);
const { data: api_control_mind_x_d_527_q } = useQuery({queryKey: ['api_control_mind_x_d_527', '/jaxrs/mind/assemble/control/mind/x/destorymind'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/x/destorymind"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_mind_x_d_527_q, (v) => { api_control_mind_x_d_527_data.value = v ?? []; });
const api_control_folder_move_x_data = ref<any[]>([]);
const { data: api_control_folder_move_x_q } = useQuery({queryKey: ['api_control_folder_move_x', '/jaxrs/mind/assemble/control/folder/move/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/folder/move/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_folder_move_x_q, (v) => { api_control_folder_move_x_data.value = v ?? []; });
const api_control_mind_share_x_data = ref<any[]>([]);
const { data: api_control_mind_share_x_q } = useQuery({queryKey: ['api_control_mind_share_x', '/jaxrs/mind/assemble/control/mind/share/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/share/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control_mind_share_x_q, (v) => { api_control_mind_share_x_data.value = v ?? []; });

const api_jaxrs_mind_assem_234_data = ref<any[]>([]);
const { data: api_jaxrs_mind_assem_234_q } = useQuery({queryKey: ['api_jaxrs_mind_assem_234', '/jaxrs/mind/assemble/control/mind/filter/list/x/next/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/filter/list/x/next/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_mind_assem_234_q, (v) => { api_jaxrs_mind_assem_234_data.value = v ?? []; });
const api_jaxrs_mind_assem_540_data = ref<any[]>([]);
const { data: api_jaxrs_mind_assem_540_q } = useQuery({queryKey: ['api_jaxrs_mind_assem_540', '/jaxrs/mind/assemble/control/mind/filter/recycle/x/next/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/filter/recycle/x/next/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_mind_assem_540_q, (v) => { api_jaxrs_mind_assem_540_data.value = v ?? []; });
const api_jaxrs_mind_assem_93_data = ref<any[]>([]);
const { data: api_jaxrs_mind_assem_93_q } = useQuery({queryKey: ['api_jaxrs_mind_assem_93', '/jaxrs/mind/assemble/control/mind/filter/shared/x/next/1'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/filter/shared/x/next/1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_mind_assem_93_q, (v) => { api_jaxrs_mind_assem_93_data.value = v ?? []; });
const api_jaxrs_mind_assem_929_data = ref<any[]>([]);
const { data: api_jaxrs_mind_assem_929_q } = useQuery({queryKey: ['api_jaxrs_mind_assem_929', '/jaxrs/mind/assemble/control/mind/x/icon/size/64'], queryFn: async () => { try { const r = await api.get("/jaxrs/mind/assemble/control/mind/x/icon/size/64"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_mind_assem_929_q, (v) => { api_jaxrs_mind_assem_929_data.value = v ?? []; });
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.split-panel{flex:1;display:grid;grid-template-columns:260px 1fr;gap:16px;overflow:hidden}
.tree-panel,.content-panel{padding:16px;display:flex;flex-direction:column;gap:12px;overflow:hidden}
.tree-panel{overflow-y:auto}
.tree-toolbar{display:flex;gap:8px}
.btn-sm{padding:4px 12px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-sm);color:var(--text-secondary);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tree{list-style:none;padding:0;margin:0}
.tree-node{padding:6px 8px;border-radius:var(--radius-sm);cursor:pointer;display:flex;align-items:center;gap:6px;font-size:13px;color:var(--text-secondary)}
.tree-node:hover{background:var(--color-primary-soft);color:var(--color-primary)}
.node-label{flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.node-children{font-size:10px;color:var(--text-muted)}
.tree-children{list-style:none;padding-left:16px;margin:0}
.tree-children .tree-node.child{font-size:12px;color:var(--text-muted)}
.tree-empty,.tree-loading{color:var(--text-muted);font-size:13px;text-align:center;padding:20px}
.content-header{display:flex;align-items:center;justify-content:space-between}
.content-header h3{font-size:15px;color:var(--text-primary);margin:0}
.content-header .count{font-size:12px;color:var(--text-muted)}
.mind-grid{flex:1;overflow-y:auto;display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:10px}
.mind-card{display:flex;align-items:center;gap:10px;padding:12px;cursor:pointer;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.mind-card:hover{border-color:var(--color-primary);transform:translateX(3px);box-shadow:var(--shadow-glow)}
.mc-icon{font-size:24px}
.mc-info{flex:1;min-width:0}
.mc-title{font-size:13px;font-weight:600;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.mc-meta{font-size:11px;color:var(--text-muted);margin-top:2px}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:36px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;max-width:600px;width:90%;max-height:80vh;display:flex;flex-direction:column;overflow:hidden}
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}
.modal-header h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0;font-size:16px}
.btn-close{background:none;border:none;color:var(--text-muted);font-size:18px;cursor:pointer}
.btn-close:hover{color:var(--color-error)}
.mind-json{flex:1;overflow:auto;background:var(--bg-base);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:12px;font-size:12px;color:var(--text-secondary);font-family:'JetBrains Mono',monospace;white-space:pre-wrap;word-break:break-all}
@media(max-width:768px){.split-panel{grid-template-columns:1fr}}
</style>
