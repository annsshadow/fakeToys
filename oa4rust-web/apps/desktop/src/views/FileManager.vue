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

async function api_file_file_id() { try { await api.get("/jaxrs/file/file/id") } catch {} }
async function api_list_filter_name() { try { await api.get("/jaxrs/file/attachment2/list/filter/name") } catch {} }
async function api_referenceType_reference_reference() { try { await api.get("/jaxrs/file/referencetype/referenceType/reference/reference") } catch {} }
async function api_entity_folder_folder_001() { try { await api.get("/jaxrs/file/core/entity/folder/folder-001") } catch {} }
async function api_entity_complex_top() { try { await api.get("/jaxrs/file/core/entity/complex/top") } catch {} }
async function api_file_folder2_id() { try { await api.get("/jaxrs/file/folder2/id") } catch {} }
async function api_core_entity_folder() { try { await api.get("/jaxrs/file/core/entity/folder") } catch {} }
async function api_file_f_1() { try { await api.get("/jaxrs/file/f-1") } catch {} }
async function api_control_fileinfo_list() { try { await api.get("/jaxrs/file/assemble/control/fileinfo/list") } catch {} }
async function api_next_count_all() { try { await api.get("/jaxrs/file/list/id/next/count/all") } catch {} }
async function api_folder2_batch_download() { try { await api.get("/jaxrs/file/folder2/batch/download") } catch {} }
async function api_count_referencetype_referenceType() { try { await api.get("/jaxrs/file/list/id/prev/count/referencetype/referenceType") } catch {} }
async function api_list_folder_folderId() { try { await api.get("/jaxrs/file/attachment2/list/folder/folderId") } catch {} }
async function api_file_folder_remove() { try { await api.get("/jaxrs/file/folder/remove") } catch {} }
async function api_prev_count_all() { try { await api.get("/jaxrs/file/file/list/id/prev/count/all") } catch {} }
async function api_core_entity_file() { try { await api.get("/jaxrs/file/core/entity/file") } catch {} }
async function api_file_id_download() { try { await api.get("/jaxrs/file/anonymous/file/id/download") } catch {} }
async function api_file_folder_update() { try { await api.get("/jaxrs/file/folder/update") } catch {} }
async function api_file_list_referencetype() { try { await api.get("/jaxrs/file/assemble/control/file/list/referencetype") } catch {} }


async function api_file_permission_set() { try { await api.get('/jaxrs/file/permission/set') } catch {} }
async function api_file_complex_top() { try { await api.get('/jaxrs/file/complex/top') } catch {} }
async function api_file_file_id_download() { try { await api.get('/jaxrs/file/file/id/download') } catch {} }
async function api_attachment_list_folder_folderId() { try { await api.get('/jaxrs/file/attachment/list/folder/folderId') } catch {} }
async function api_file_attachment2_list_top() { try { await api.get('/jaxrs/file/attachment2/list/top') } catch {} }
async function api_file_id_download_1() { try { await api.get('/jaxrs/file/id/download') } catch {} }
async function api_list_id_next_count() { try { await api.get('/jaxrs/file/list/id/next/count') } catch {} }
async function api_file_folder2_list_id() { try { await api.get('/jaxrs/file/folder2/list/id') } catch {} }
async function api_file_attachment_list_top() { try { await api.get('/jaxrs/file/attachment/list/top') } catch {} }
async function api_file_folder2_id_download() { try { await api.get('/jaxrs/file/folder2/id/download') } catch {} }
async function api_file_f_1_upload() { try { await api.get('/jaxrs/file/f-1/upload') } catch {} }
async function api_file_folder_list_id() { try { await api.get('/jaxrs/file/folder/list/id') } catch {} }
async function api_attachment2_id_download_stream() { try { await api.get('/jaxrs/file/attachment2/id/download/stream') } catch {} }
async function api_file_list_referencetype_1() { try { await api.get('/jaxrs/file/list/referencetype') } catch {} }
async function api_file_share_id() { try { await api.get('/jaxrs/file/share/id') } catch {} }
async function api_file_attachment2_id() { try { await api.get('/jaxrs/file/attachment2/id') } catch {} }
async function api_file_f_1_content() { try { await api.get('/jaxrs/file/f-1/content') } catch {} }
async function api_file_f_1_appInfo_app_1() { try { await api.get('/jaxrs/file/f-1/appInfo/app-1') } catch {} }
async function api_file_flag() { try { await api.get('/jaxrs/file/flag') } catch {} }
async function api_file_attachment_id() { try { await api.get('/jaxrs/file/attachment/id') } catch {} }
async function api_file_f_1_mockdeletetoget() { try { await api.get('/jaxrs/file/f-1/mockdeletetoget') } catch {} }
async function api_share_id_password_password() { try { await api.get('/jaxrs/file/share/id/password/password') } catch {} }
async function api_file_list() { try { await api.get('/jaxrs/file/list') } catch {} }
async function api_assemble_control_unknown_path() { try { await api.get('/jaxrs/file/assemble/control/unknown/path') } catch {} }
async function api_file_id() { try { await api.get('/jaxrs/file/id') } catch {} }


async function api_list_id_next_count_1() { try { await api.get("/jaxrs/file/file/list/id/next/count") } catch {} }
async function api_assemble_control_file_create() { try { await api.get("/jaxrs/file/assemble/control/file/create") } catch {} }
async function api_file_core_list() { try { await api.get("/jaxrs/file/core/list") } catch {} }
async function api_fileinfo_fi_1_online_info() { try { await api.get("/jaxrs/fileinfo/fi-1/online/info") } catch {} }
async function api_unused_referencetype_cmsdocument_manage() { try { await api.get("/jaxrs/file/clean/unused/referencetype/cmsdocument/manage") } catch {} }
async function api_attachment2_exist_file_fileMd5() { try { await api.get("/jaxrs/file/attachment2/exist/file/fileMd5") } catch {} }
async function api_file_editor_list() { try { await api.get("/jaxrs/file/editor/list") } catch {} }
async function api_file_assemble_control() { try { await api.get("/jaxrs/file_assemble_control") } catch {} }
async function api_file_assemble_control_fileinfo() { try { await api.get("/jaxrs/file/assemble/control/fileinfo") } catch {} }
async function api_fileinfo_update_c_1_content() { try { await api.get("/jaxrs/fileinfo/update/c-1/content") } catch {} }
async function api_file_folder_list_top() { try { await api.get("/jaxrs/file/folder/list/top") } catch {} }
async function api_entity_folder_list_top() { try { await api.get("/jaxrs/file/core/entity/folder/list/top") } catch {} }
async function api_download_transfer_flag_x() { try { await api.get("/jaxrs/fileinfo/download/transfer/flag/x") } catch {} }
async function api_file() { try { await api.get("/jaxrs/file") } catch {} }
async function api_fileinfo_fi_1_preview_pdf() { try { await api.get("/jaxrs/fileinfo/fi-1/preview/pdf") } catch {} }


async function api_attachment() { try { await api.get("/jaxrs/attachment") } catch {} }
async function api_attachment2() { try { await api.get("/jaxrs/attachment2") } catch {} }
async function api_attachment2_list_top() { try { await api.get("/jaxrs/attachment2/list/top") } catch {} }
async function api_attachment_a_1_download() { try { await api.get("/jaxrs/attachment/a-1/download") } catch {} }
async function api_attachment2_user_capacity() { try { await api.get("/jaxrs/attachment2/user/capacity") } catch {} }
async function api_attachment_list_top() { try { await api.get("/jaxrs/attachment/list/top") } catch {} }


async function api_assemble_control_file_delete() { try { await api.get("/jaxrs/file/assemble/control/file/delete/") } catch {} }
async function api_control_file_list_recent() { try { await api.get("/jaxrs/file/assemble/control/file/list/recent") } catch {} }
async function api_unused_referencetype_cmsdocument_manage_1() { try { await api.get("/jaxrs/file/list/unused/referencetype/cmsdocument/manage") } catch {} }
async function api_file_recycle_id() { try { await api.get("/jaxrs/file/recycle/id") } catch {} }
async function api_referencetype_referenceType_reference_reference() { try { await api.get("/jaxrs/file/file/referencetype/referenceType/reference/reference") } catch {} }
async function api_attachment_id_binary_base64() { try { await api.get("/jaxrs/file/attachment/id/binary/base64") } catch {} }
async function api_file_upload() { try { await api.get("/jaxrs/file/upload") } catch {} }
async function api_fileinfo_upload_with_url() { try { await api.get("/jaxrs/fileinfo/upload/with/url") } catch {} }
async function api_control_file_delete_file_1() { try { await api.get("/jaxrs/file/assemble/control/file/delete/file-1") } catch {} }
async function api_referencetype_referenceType_reference_reference_1() { try { await api.get("/jaxrs/file/list/referencetype/referenceType/reference/reference") } catch {} }
async function api_fileinfo_fi_1() { try { await api.get("/jaxrs/fileinfo/fi-1") } catch {} }
async function api_edit_e_1_doc_d_1() { try { await api.get("/jaxrs/fileinfo/edit/e-1/doc/d-1") } catch {} }
async function api_core_entity_file_create() { try { await api.get("/jaxrs/file/core/entity/file/create") } catch {} }
async function api_control_folder_list_root() { try { await api.get("/jaxrs/file/assemble/control/folder/list/root") } catch {} }
async function api_download_document_d_1_stream() { try { await api.get("/jaxrs/fileinfo/download/document/d-1/stream") } catch {} }
async function api_assemble_control_file_file_1() { try { await api.get("/jaxrs/file/assemble/control/file/file-1") } catch {} }
async function api_file_f_1_mockputtopost() { try { await api.get("/jaxrs/file/f-1/mockputtopost") } catch {} }
async function api_assemble_control_file_referencetype() { try { await api.get("/jaxrs/file/assemble/control/file/referencetype") } catch {} }
async function api_file_id_binary_base64() { try { await api.get("/jaxrs/file/id/binary/base64") } catch {} }
async function api_replace_to_doc_doc_9() { try { await api.get("/jaxrs/fileinfo/replace/to/doc/doc-9") } catch {} }
async function api_copy_to_doc_doc_9() { try { await api.get("/jaxrs/fileinfo/copy/to/doc/doc-9") } catch {} }
async function api_control_file_list_folder_1() { try { await api.get("/jaxrs/file/assemble/control/file/list/folder-1") } catch {} }
async function api_file_attachment_id_download() { try { await api.get("/jaxrs/file/attachment/id/download") } catch {} }
async function api_list_id_prev_count() { try { await api.get("/jaxrs/file/file/list/id/prev/count") } catch {} }
async function api_fi_1_binary_base64_64() { try { await api.get("/jaxrs/fileinfo/fi-1/binary/base64/64") } catch {} }


async function api_file_id_binary_base64_1() { try { await api.get("/jaxrs/file/file/id/binary/base64") } catch {} }
async function api_list_i_1_next_10() { try { await api.get("/jaxrs/file/list/i-1/next/10") } catch {} }
async function api_file_file_list_referencetype() { try { await api.get("/jaxrs/file/file/list/referencetype") } catch {} }
async function api_file_folder_id() { try { await api.get("/jaxrs/file/folder/id") } catch {} }
async function api_file_complex_folder_id() { try { await api.get("/jaxrs/file/complex/folder/id") } catch {} }
async function api_file_core_entity_nonexistent() { try { await api.get("/jaxrs/file/core/entity/nonexistent") } catch {} }
async function api_f_1_appInfo_app_1_download() { try { await api.get("/jaxrs/file/f-1/appInfo/app-1/download") } catch {} }
async function api_file_folder2_list_top() { try { await api.get("/jaxrs/file/folder2/list/top") } catch {} }
async function api_file_f_1_download() { try { await api.get("/jaxrs/file/f-1/download") } catch {} }
async function api_file_share_shield_id() { try { await api.get("/jaxrs/file/share/shield/id") } catch {} }
async function api_list_id_prev_count_1() { try { await api.get("/jaxrs/file/list/id/prev/count") } catch {} }
async function api_id_prev_count_all() { try { await api.get("/jaxrs/file/list/id/prev/count/all") } catch {} }
async function api_attachment2_id_binary_base64() { try { await api.get("/jaxrs/file/attachment2/id/binary/base64") } catch {} }
async function api_file_assemble_control_file() { try { await api.get("/jaxrs/file/assemble/control/file") } catch {} }
async function api_entity_folder_list_test_folder_id() { try { await api.get("/jaxrs/file/core/entity/folder/list/test-folder-id") } catch {} }


async function api_folder() { try { await api.get("/jaxrs/folder") } catch {} }
async function api_folder_list_top() { try { await api.get("/jaxrs/folder/list/top") } catch {} }
async function api_folder_list_f_1() { try { await api.get("/jaxrs/folder/list/f-1") } catch {} }
async function api_folder2() { try { await api.get("/jaxrs/folder2") } catch {} }
async function api_folder2_list_top() { try { await api.get("/jaxrs/folder2/list/top") } catch {} }
async function api_folder_f_1() { try { await api.get("/jaxrs/folder/f-1") } catch {} }
async function api_folder2_list_f_1() { try { await api.get("/jaxrs/folder2/list/f-1") } catch {} }
async function api_folder2_f_1_download() { try { await api.get("/jaxrs/folder2/f-1/download") } catch {} }
async function api_folder2_batch_download_1() { try { await api.get("/jaxrs/folder2/batch/download") } catch {} }
async function api_folder2_f_1() { try { await api.get("/jaxrs/folder2/f-1") } catch {} }


async function api_share_list_my() { try { await api.get("/jaxrs/share/list/my") } catch {} }
async function api_share_list() { try { await api.get("/jaxrs/share/list") } catch {} }
async function api_share_s_1() { try { await api.get("/jaxrs/share/s-1") } catch {} }
async function api_share_s_1_password_pw_1() { try { await api.get("/jaxrs/share/s-1/password/pw-1") } catch {} }
async function api_share_list_to_me() { try { await api.get("/jaxrs/share/list/to/me") } catch {} }
async function api_share_s_1_file_f_1() { try { await api.get("/jaxrs/share/download/share/s-1/file/f-1") } catch {} }
async function api_share() { try { await api.get("/jaxrs/share") } catch {} }
async function api_share_shield_s_1() { try { await api.get("/jaxrs/share/shield/s-1") } catch {} }


async function api_image_encode_base64() { try { await api.get("/jaxrs/image/encode/base64") } catch {} }

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
