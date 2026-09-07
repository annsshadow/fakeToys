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
import { confirmMsg } from '../utils/toast';
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
  if (confirmMsg(`确定删除「${f.name}」？`)) {
    deleteMutation.mutate(f.id, {
      onSuccess: () => toast.success('文件已删除'),
      onError: () => toast.error('删除失败'),
    });
  }
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
.btn-sm{padding:4px 10px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);cursor:pointer;font-size:12px;background:var(--bg-elevated);color:var(--text-primary)}
.btn-primary{padding:6px 14px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);cursor:pointer;font-size:13px;font-weight:500}
.loading-state,.empty-state{display:flex;flex-direction:column;align-items:center;padding:40px;color:var(--text-muted);gap:12px}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.empty-icon{font-size:48px;opacity:.4}
</style>
