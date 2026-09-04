<template>
  <div class="file-view">
    <div class="view-header glass-card">
      <h1>文件管理</h1>
      <div class="header-actions">
        <button class="action-btn primary" @click="handleUpload">📤 上传</button>
        <button class="action-btn" @click="toggleView">{{ viewType === 'grid' ? '☰ 列表' : '⊞ 网格' }}</button>
      </div>
    </div>

    <!-- 面包屑 -->
    <div class="breadcrumb glass-card">
      <span v-for="(seg, i) in breadcrumbs" :key="i" class="bc-seg">
        <span class="bc-item" :class="{ active: i === breadcrumbs.length - 1 }" @click="navigateTo(i)">{{ seg }}</span>
        <span v-if="i < breadcrumbs.length - 1" class="bc-arrow">›</span>
      </span>
    </div>

    <!-- 文件列表/网格 -->
    <div class="file-panel glass-card" :class="'view-' + viewType">
      <div v-if="loading" class="loading-state">
        <div class="skeleton-row" v-for="i in 6" :key="i"></div>
      </div>
      <div v-else-if="files.length === 0" class="empty-state">
        <div class="empty-icon">📁</div>
        <p>当前文件夹为空</p>
      </div>
      <template v-else>
        <div v-if="viewType === 'list'" class="file-list">
          <div class="file-header">
            <span class="col-name">名称</span>
            <span class="col-size">大小</span>
            <span class="col-time">修改时间</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="f in files" :key="f.id" class="file-row" @dblclick="openItem(f)">
            <span class="col-name">
              <span class="file-icon">{{ iconForFile(f) }}</span>
              {{ f.name }}
            </span>
            <span class="col-size">{{ formatSize(f.size) }}</span>
            <span class="col-time">{{ fmtTime(f.updateTime) }}</span>
            <span class="col-actions">
              <button class="icon-action" @click.stop="downloadFile(f)" title="下载">⬇</button>
              <button class="icon-action" @click.stop="shareFile(f)" title="分享">↗</button>
              <button class="icon-action danger" @click.stop="deleteFile(f)" title="删除">🗑</button>
            </span>
          </div>
        </div>
        <div v-else class="file-grid">
          <div v-for="f in files" :key="f.id" class="file-card" @dblclick="openItem(f)">
            <div class="file-thumb">{{ iconForFile(f) }}</div>
            <div class="file-name">{{ f.name }}</div>
            <div class="file-meta">{{ formatSize(f.size) }}</div>
          </div>
        </div>
      </template>
    </div>

    <!-- 上传弹窗 -->
    <div v-if="showUpload" class="upload-overlay" @click.self="showUpload = false">
      <div class="upload-dialog glass-card">
        <h3>上传文件</h3>
        <div class="upload-area" @dragover.prevent @drop.prevent="handleDrop">
          <div class="upload-icon">📤</div>
          <p>拖放文件到此处，或点击选择</p>
          <input type="file" multiple class="file-input" @change="handleFileSelect" />
        </div>
        <div v-if="uploadProgress > 0" class="progress-bar">
          <div class="progress-fill" :style="{ width: uploadProgress + '%' }"></div>
          <span>{{ uploadProgress }}%</span>
        </div>
        <div class="upload-actions">
          <button class="btn-cancel" @click="showUpload = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { api } from '@oa4rust/sdk';

interface FileItem {
  id: string;
  name: string;
  size?: number;
  type: 'file' | 'folder';
  updateTime?: string;
  parentId?: string;
  [key: string]: unknown;
}

const currentFolder = ref<string>('');
const breadcrumbs = ref<string[]>(['根目录']);
const viewType = ref<'grid' | 'list'>('list');
const loading = ref(false);
const files = ref<FileItem[]>([]);
const showUpload = ref(false);
const uploadProgress = ref(0);
const queryClient = useQueryClient();

// 加载文件列表
async function loadFiles(folderId?: string): Promise<void> {
  loading.value = true;
  try {
    const resp = await api.get(`/jaxrs/file/assemble/control/file/list/${folderId || ''}`);
    files.value = ((resp as any)?.data ?? []) as FileItem[];
  } catch {
    files.value = [];
  } finally {
    loading.value = false;
  }
}

loadFiles();

function openItem(f: FileItem): void {
  if (f.type === 'folder' || (f as any).isFolder) {
    currentFolder.value = f.id;
    breadcrumbs.value = [...breadcrumbs.value, f.name];
    loadFiles(f.id);
  }
}

function navigateTo(index: number): void {
  breadcrumbs.value = breadcrumbs.value.slice(0, index + 1);
  currentFolder.value = index === 0 ? '' : (files.value[index - 1] as FileItem | undefined)?.id ?? '';
  loadFiles(currentFolder.value);
}

function toggleView(): void {
  viewType.value = viewType.value === 'grid' ? 'list' : 'grid';
}

function iconForFile(f: FileItem): string {
  if ((f as any).type === 'folder' || f.type === 'folder') return '📁';
  const ext = (f.name.split('.').pop() ?? '').toLowerCase();
  const icons: Record<string, string> = {
    pdf: '📄', doc: '📝', docx: '📝', txt: '📃',
    xls: '📊', xlsx: '📊', csv: '📊',
    ppt: '📑', pptx: '📑',
    jpg: '🖼', jpeg: '🖼', png: '🖼', gif: '🖼', svg: '🖼',
    mp4: '🎬', mp3: '🎵', avi: '🎬',
    zip: '📦', rar: '📦', '7z': '📦',
    js: '⚡', ts: '⚡', py: '🐍', rust: '🦀',
    json: '📋', xml: '📋', html: '🌐', css: '🎨',
  };
  return icons[ext] ?? '📄';
}

function formatSize(bytes?: number): string {
  if (!bytes) return '—';
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

function fmtTime(ts?: string): string {
  if (!ts) return '—';
  try {
    return new Date(ts).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' });
  } catch { return String(ts); }
}

// 删除文件
const deleteMutation = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/file/assemble/control/file/${id}`),
  onSuccess: () => {
    queryClient.invalidateQueries({ queryKey: ['file', currentFolder.value] });
    loadFiles(currentFolder.value);
  },
});

function deleteFile(f: FileItem): void {
  if (confirm(`确定删除「${f.name}」？`)) deleteMutation.mutate(f.id);
}

function downloadFile(f: FileItem): void {
  window.open(`/jaxrs/file/core/entity/file/${f.id}/download`);
}

function shareFile(_f: FileItem): void {
  // Share functionality (future)
}

function handleUpload(): void { showUpload.value = true; }

function handleFileSelect(e: Event): void {
  const files = (e.target as HTMLInputElement).files;
  if (!files?.length) return;
  uploadFile(files[0]);
}

function handleDrop(e: DragEvent): void {
  const file = e.dataTransfer?.files[0];
  if (file) uploadFile(file);
}

function uploadFile(file: File): void {
  uploadProgress.value = 0;
  const formData = new FormData();
  formData.append('file', file);
  // Simulate progress
  const interval = setInterval(() => {
    uploadProgress.value = Math.min(99, uploadProgress.value + 10);
  }, 200);
  api.upload('/jaxrs/file/assemble/control/file/upload', formData)
    .then(() => {
      clearInterval(interval);
      uploadProgress.value = 100;
      setTimeout(() => { showUpload.value = false; uploadProgress.value = 0; loadFiles(currentFolder.value); }, 500);
    })
    .catch(() => {
      clearInterval(interval);
      uploadProgress.value = 0;
    });
}
</script>

<style scoped>
.file-view { display: flex; flex-direction: column; gap: 12px; height: 100%; }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px; }
.view-header h1 {
  font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0;
}
.header-actions { display: flex; gap: 8px; }
.action-btn {
  padding: 6px 16px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle);
  background: var(--bg-elevated); color: var(--text-secondary); cursor: pointer; font-size: 13px;
  transition: all var(--transition-fast);
}
.action-btn:hover { border-color: var(--color-primary); color: var(--color-primary); }
.action-btn.primary { background: var(--color-primary); color: white; border-color: var(--color-primary); }

.breadcrumb { padding: 8px 20px; display: flex; align-items: center; gap: 4px; font-size: 13px; }
.bc-seg { display: flex; align-items: center; gap: 4px; }
.bc-item {
  padding: 2px 8px; border-radius: var(--radius-sm); cursor: pointer; color: var(--text-muted);
  transition: all var(--transition-fast);
}
.bc-item:hover, .bc-item.active { color: var(--color-primary); background: var(--color-primary-soft); }
.bc-arrow { color: var(--text-muted); }

.file-panel { flex: 1; overflow: auto; padding: 12px; }
.file-header {
  display: grid; grid-template-columns: 2fr 100px 140px 100px; gap: 8px;
  padding: 8px 12px; font-size: 12px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px;
  border-bottom: 1px solid var(--border-subtle);
}
.file-row {
  display: grid; grid-template-columns: 2fr 100px 140px 100px; gap: 8px;
  padding: 10px 12px; border-radius: var(--radius-md); cursor: pointer;
  transition: all var(--transition-fast); align-items: center;
}
.file-row:hover { background: var(--color-primary-soft); }
.col-name { display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--text-primary); }
.col-size, .col-time { font-size: 12px; color: var(--text-muted); }
.col-actions { display: flex; gap: 4px; }
.icon-action {
  background: none; border: none; cursor: pointer; font-size: 14px; padding: 4px 6px;
  border-radius: var(--radius-sm); transition: background var(--transition-fast);
}
.icon-action:hover { background: var(--bg-elevated); }
.icon-action.danger:hover { background: var(--color-error-glow); }

.file-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); gap: 12px; }
.file-card {
  display: flex; flex-direction: column; align-items: center; padding: 16px 8px;
  border-radius: var(--radius-md); cursor: pointer; transition: all var(--transition-fast);
  border: 1px solid transparent;
}
.file-card:hover { background: var(--color-primary-soft); border-color: var(--border-active); }
.file-thumb { font-size: 36px; margin-bottom: 8px; }
.file-name { font-size: 12px; color: var(--text-primary); text-align: center; word-break: break-all; }
.file-meta { font-size: 10px; color: var(--text-muted); margin-top: 4px; }

.loading-state { padding: 40px; }
.skeleton-row { height: 40px; border-radius: var(--radius-md); margin-bottom: 8px; }
.empty-state { display: flex; flex-direction: column; align-items: center; padding: 60px; color: var(--text-muted); gap: 12px; }
.empty-icon { font-size: 48px; opacity: 0.4; }

/* 上传弹窗 */
.upload-overlay {
  position: fixed; inset: 0; background: var(--bg-overlay); z-index: 200;
  display: flex; align-items: center; justify-content: center;
}
.upload-dialog {
  width: 400px; padding: 24px; text-align: center;
}
.upload-dialog h3 { color: var(--color-primary); font-family: 'Orbitron', sans-serif; margin: 0 0 16px; }
.upload-area {
  border: 2px dashed var(--border-subtle); border-radius: var(--radius-lg);
  padding: 40px; cursor: pointer; transition: all var(--transition-fast);
}
.upload-area:hover { border-color: var(--color-primary); background: var(--color-primary-soft); }
.upload-icon { font-size: 36px; margin-bottom: 8px; }
.upload-area p { font-size: 13px; color: var(--text-muted); margin: 0 0 12px; }
.file-input { display: none; }
.progress-bar { display: flex; align-items: center; gap: 8px; margin: 12px 0; }
.progress-fill { height: 4px; background: var(--color-primary); border-radius: 2px; transition: width 0.3s; flex: 1; }
.progress-bar span { font-size: 12px; color: var(--color-primary); }
.upload-actions { display: flex; justify-content: flex-end; margin-top: 16px; }
.btn-cancel { padding: 6px 16px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); background: none; color: var(--text-secondary); cursor: pointer; }
</style>
