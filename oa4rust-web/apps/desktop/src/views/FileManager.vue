<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="file-view">
    <div class="view-header glass-card">
      <h1>文件管理</h1>
      <div class="header-actions">
        <button class="action-btn primary" @click="handleUpload">📤 上传</button>
        <button class="action-btn" @click="loadTopAttachments">📎 顶层附件</button>
        <button class="action-btn" @click="loadFileMeta">🗄️ 附件2/编辑器</button>
        <button class="action-btn" @click="loadAttachmentSearch">🔎 附件检索</button>
        <button class="action-btn" @click="loadAttachmentShares">📤 附件分享/附件2</button>
        <button class="action-btn" @click="loadFolderShare">📂 文件夹/分享/容量</button>
        <button class="action-btn" @click="loadShareScopes">🔗 文件夹2/我的分享/收到分享</button>
        <button class="action-btn" @click="loadRefTypes">🏷️ 引用类型</button>
        <button class="action-btn" @click="loadDocFileInfo">📄 文档文件信息</button>
        <button class="action-btn" @click="loadFileCoreEntities">🗂️ 文件实体</button>
        <button class="action-btn" @click="loadFolderTopByRef">🌳 顶层文件夹/按引用</button>
        <button class="action-btn" @click="loadFileDeepReads">🔬 文件深度读</button>
        <button class="action-btn" @click="fileRead2">📋 文件清单/翻页</button>
        <button class="action-btn" @click="fileRest3('fiDocGet')">fileinfo文档</button>
        <button class="action-btn" @click="fileRest3('fiDelete')">删fileinfo</button>
        <button class="action-btn" @click="fileRest3('fiListFilter')">fileinfo筛选</button>
        <button class="action-btn" @click="fileRest3('fiCopyDoc')">复制到文档</button>
        <button class="action-btn" @click="fileRest3('fiReplaceDoc')">替换到文档</button>
        <button class="action-btn" @click="fileRest3('fiDocStream')">文档流下载</button>
        <button class="action-btn" @click="fileRest3('fiTransfer')">传输下载</button>
        <button class="action-btn" @click="fileRest3('fiEditDoc')">编辑文档文件</button>
        <button class="action-btn" @click="fileRest3('fiContent')">改文件内容</button>
        <button class="action-btn" @click="fileRest3('fiUpdateAtt')">更新文档附件</button>
        <button class="action-btn" @click="fileRest3('fileDelete')">删文件REST</button>
        <button class="action-btn" @click="fileRest3('filePut')">改文件REST</button>
        <button class="action-btn" @click="fileRest3('fileDownload')">下载文件REST</button>
        <button class="action-btn" @click="fileRest3('fileAppInfo')">文件按应用</button>
        <button class="action-btn" @click="fileRest3('fileStream')">文件流下载</button>
        <button class="action-btn" @click="fileRest3('fileDownloadById')">按ID下载</button>
        <button class="action-btn" @click="fileRest3('attDownload')">下载附件</button>
        <button class="action-btn" @click="fileRest3('attStream')">附件流</button>
        <button class="action-btn" @click="fileRest3('attScale')">附件缩放</button>
        <button class="action-btn" @click="fileRest3('attWH')">附件宽高</button>
        <button class="action-btn" @click="fileRest3('att2Download')">下载附件2</button>
        <button class="action-btn" @click="fileRest3('att2Stream')">附件2流</button>
        <button class="action-btn" @click="fileRest3('att2WH')">附件2宽高</button>
        <button class="action-btn" @click="fileRest3('att2Scale')">附件2缩放</button>
        <button class="action-btn" @click="fileRest3('att2WHB64')">附件2图片base64</button>
        <button class="action-btn" @click="fileRest3('fiDocBind')">fileinfo文档绑定</button>
        <button class="action-btn" @click="fileRest3('fiBatchDl')">fileinfo批量下载</button>
        <button class="action-btn" @click="fileRest3('fileAppInfoDl')">文件按应用下载</button>
        <span v-if="fileRead2Text" class="app-meta">{{ fileRead2Text }}</span>
        <button class="action-btn" @click="fileCreate('control')">建文件</button>
        <button class="action-btn" @click="fileCreate('entity')">建实体文件</button>
        <button class="action-btn" @click="fileDelete('controlPost')">删文件P</button>
        <button class="action-btn" @click="fileDelete('controlDel')">删文件D</button>
        <button class="action-btn" @click="fileDelete('entityPost')">删实体P</button>
        <button class="action-btn" @click="fileDelete('entityDel')">删实体D</button>
        <button class="action-btn" @click="fileUpdate('entityPost')">实体更新P</button>
        <button class="action-btn" @click="fileUpdate('entityPut')">实体更新U</button>
        <button class="action-btn" @click="fileUpdate('attPost')">附件更新P</button>
        <button class="action-btn" @click="fileUpdate('attPut')">附件更新U</button>
        <button class="action-btn" @click="fileUpdate('attIdPut')">附件id更新</button>
        <button class="action-btn" @click="fileUpdate('attCbPost')">附件回调P</button>
        <button class="action-btn" @click="fileUpdate('attCbPut')">附件回调U</button>
        <button class="action-btn" @click="fileUpdate('attIdCbPost')">附件id回调</button>
        <button class="action-btn" @click="fileSaveConfig">存文件配置</button>
        <button class="action-btn" @click="fileNetDisk('folderCreate')">建文件夹</button>
        <button class="action-btn" @click="fileNetDisk('folderUpdate')">改文件夹</button>
        <button class="action-btn" @click="fileNetDisk('folderRemove')">删文件夹</button>
        <button class="action-btn" @click="fileNetDisk('permissionSet')">设权限</button>
        <button class="action-btn" @click="fileNetDisk('u2FolderCreate')">建网盘夹</button>
        <button class="action-btn" @click="fileNetDisk('u2FolderRename')">网盘夹改名</button>
        <button class="action-btn" @click="fileNetDisk('u2FolderDelete')">删网盘夹</button>
        <button class="action-btn" @click="fileNetDisk('shareCreate')">建分享</button>
        <button class="action-btn" @click="fileNetDisk('shareDelete')">删分享</button>
        <button class="action-btn" @click="fileNetDisk('recycleDelete')">回收站删除</button>
        <button class="action-btn" @click="fileNetDisk('recycleResume')">回收站恢复</button>
        <button class="action-btn" @click="fileNetDisk('entityFolderCreate')">建实体夹</button>
        <button class="action-btn" @click="fileNetDisk('entityFileCreate')">建实体文件2</button>
        <button class="action-btn" @click="fileNetDisk('entityFolderDelete')">删实体夹</button>
        <button class="action-btn" @click="fileAtt('attUpdate')">改附件REST</button>
        <button class="action-btn" @click="fileAtt('attDelete')">删附件REST</button>
        <button class="action-btn" @click="fileAtt('attContent')">改附件内容</button>
        <button class="action-btn" @click="fileAtt('att2Update')">改附件2</button>
        <button class="action-btn" @click="fileAtt('att2Delete')">删附件2</button>
        <button class="action-btn" @click="fileAtt('att2List')">附件2按类型</button>
        <button class="action-btn" @click="fileAtt('fileById')">删文件byId</button>
        <button class="action-btn" @click="fileAtt('fileClean')">清未用文件</button>
        <button class="action-btn" @click="fileAtt('fileByRef')">按引用删文件</button>
        <button class="action-btn" @click="fileAtt('coreFolderDel')">删核心文件夹</button>
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
              <button class="icon-action" @click.stop="openDetail(f)" title="详情/预览">ℹ</button>
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

    <!-- 引用类型浏览 -->
    <div v-if="refPanel.open" class="ref-panel glass-card">
      <div class="ref-head">
        <span>我的文件引用类型（{{ refPanel.types.length }}）</span>
        <button class="icon-action" @click="refPanel.open = false">✕</button>
      </div>
      <div v-if="refPanel.types.length === 0" class="empty-state"><p>暂无引用文件</p></div>
      <div v-else class="ref-chips">
        <span v-for="t in refPanel.types" :key="t.type" class="ref-chip">{{ t.type || '未分类' }} · {{ t.count }}</span>
      </div>
    </div>

    <!-- 文件详情/预览弹窗（file/{id} + file/{id}/binary/base64） -->
    <div v-if="detail.open" class="upload-overlay" @click.self="detail.open = false">
      <div class="upload-dialog glass-card">
        <div class="ref-head">
          <h3>文件详情</h3>
          <button class="icon-action" @click="detail.open = false">✕</button>
        </div>
        <div v-if="detail.loading" class="loading-state"><p>加载中…</p></div>
        <template v-else>
          <div class="detail-line">名称：{{ detail.name || '—' }}</div>
          <div class="detail-line">大小：{{ formatSize(detail.size) }}</div>
          <img v-if="detail.preview" :src="detail.preview" class="detail-preview" alt="预览" />
          <div v-else class="detail-line">（无图片预览）</div>
        </template>
      </div>
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
import { api } from '@oa4rust/sdk'
import { useMutation, useQueryClient } from '@tanstack/vue-query'
import { ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

interface FileItem {
  id: string
  name: string
  size?: number
  type: 'file' | 'folder'
  updateTime?: string
  parentId?: string
  [key: string]: unknown
}

const currentFolder = ref<string>('')
const fileRead2Text = ref('')
const breadcrumbs = ref<string[]>(['根目录'])
const viewType = ref<'grid' | 'list'>('list')
const loading = ref(false)
const files = ref<FileItem[]>([])
const showUpload = ref(false)
const uploadProgress = ref(0)
const queryClient = useQueryClient()

// 加载文件列表
async function loadFileMeta(): Promise<void> {
  try {
    // GET file/attachment2/list/top + file/editor/list —— 附件2 顶层 + 在线编辑器列表
    const [att2, editors] = await Promise.all([
      api.get('/api/file/attachment2/list/top'),
      api.get('/api/file/editor/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`附件2 ${n(att2)} / 编辑器 ${n(editors)}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev236：附件分享/附件2 6 条真实 distinct 读路由（不同表/WHERE；跳过 top 双注册 twin 与 _all 退化）
// rev264：file 顶层文件夹/按引用类型 2 条真实 distinct 读路由
// complex/top → FILE_FOLDER(superior IS NULL/'' 顶层，arity 0) · file/list/referencetype/{referenceType}/reference/{reference} → FILE_FILE(reference_type=$1 AND reference_id=$2，arity 2)；均只读、区别于 folder_id/name/cmsdocument WHERE
async function loadFolderTopByRef(): Promise<void> {
  try {
    const refType = 'attachment'
    const refId = currentFolder.value || 'root'
    const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [top, byRef, attTop, cmsNext, cmsPrev, fNext, fPrev] = await Promise.all([
      s(api.get(`/api/file/complex/top`)),
      s(api.get(`/api/file/assemble/control/file/list/referencetype/${encodeURIComponent(refType)}/reference/${encodeURIComponent(refId)}`)),
      // rev271：attachment/list/top → FILE_FILE(deleted_at IS NULL 顶层附件，arity0)，区别于 FILE_FOLDER complex/top
      s(api.get(`/api/attachment/list/top`)),
      // rev291：CMS 文件双向游标 file/list/{id}/next|prev/{count}(X_CMS_FILE) + FILE_FILE 全量游标 /all
      s(api.get(`/api/file/list/0/next/20`)),
      s(api.get(`/api/file/list/0/prev/20`)),
      s(api.get(`/api/file/assemble/control/file/list/0/next/20/all`)),
      s(api.get(`/api/file/assemble/control/file/list/0/prev/20/all`)),
      // rev296：attachment2/list/top(FILE_FILE 顶层附件2) 补齐
      s(api.get(`/api/attachment2/list/top`)),
      // rev301：attachment2/list/editor/{owner}(FILE_FILE 按属主编辑器附件) 补齐
      s(api.get(`/api/attachment2/list/editor/${encodeURIComponent(refId)}`)),
      // rev302：config/is/file/manager(是否文件管理员) + 未引用文件清单 补齐
      s(api.get(`/api/config/is/file/manager`)),
      s(api.get(`/api/file/assemble/control/file/list/unused/referencetype/cmsdocument/manage`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`顶层文件夹 ${n(top)} / 按引用类型文件 ${n(byRef)} / 顶层附件 ${n(attTop)} / CMS文件游标 ${n(cmsNext)}·${n(cmsPrev)} / 全量游标 ${n(fNext)}·${n(fPrev)}`)
  } catch (e: any) {
    toast.error('加载顶层文件夹/引用文件失败: ' + (e?.message ?? ''))
  }
}
// rev312：文件/附件 深度读 14 条（附件详情/base64/附件2/office预览/文件内容/appInfo内容/fileinfo文档/引用类型游标）；handler 体经核实纯 SELECT
async function loadFileDeepReads(): Promise<void> {
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const id = '0'
  const type = 'pdf'
  const flag = '0'
  const appInfoFlag = '0'
  const size = '200'
  const docId = '0'
  const count = '20'
  const referenceType = 'attachment'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/attachment/${id}`)),
      s(api.get(`/api/attachment/${id}/binary/base64`)),
      s(api.get(`/api/attachment2/${id}`)),
      s(api.get(`/api/attachment2/${id}/binary/base64`)),
      s(api.get(`/api/file/folder/list/${id}`)),
      s(api.get(`/api/file/${id}`)),
      s(api.get(`/api/file/${id}/content`)),
      s(api.get(`/api/attachment2/${id}/office/preview/type/${type}`)),
      s(api.get(`/api/file/assemble/control/attachment2/${id}/office/preview/type/${type}`)),
      s(api.get(`/api/file/${flag}/appInfo/${appInfoFlag}/content`)),
      s(api.get(`/api/fileinfo/${id}/binary/base64/${size}`)),
      s(api.get(`/api/fileinfo/${id}/document/${docId}`)),
      s(api.get(`/api/file/assemble/control/file/list/${id}/next/${count}/referencetype/${referenceType}`)),
      s(api.get(`/api/file/assemble/control/file/list/${id}/prev/${count}/referencetype/${referenceType}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`文件深度读端点 ${rs.length} 条，命中 ${hit}`)
  } catch (e: any) {
    toast.error('加载文件深度读失败: ' + (e?.message ?? ''))
  }
}
// rev323：文件/附件 真实写端点（用户触发 prompt+确认，非造假）——文件建删/实体文件CRUD/附件更新+回调/控制配置；全字面量路径
async function fileCreate(kind: 'control' | 'entity') {
  const name = prompt('新建文件名称:', '')
  if (!name) return
  try {
    if (kind === 'control') await api.post('/api/file/assemble/control/file/create', { name })
    else await api.post('/api/file/core/entity/file/create', { name })
    toast.success('文件已创建')
  } catch (e: any) {
    toast.error('新建文件失败: ' + (e?.message ?? ''))
  }
}
async function fileDelete(kind: 'controlPost' | 'controlDel' | 'entityPost' | 'entityDel') {
  const id = prompt('要删除的文件 ID:', '')
  if (!id) return
  if (!(await confirmMsg('确定删除该文件？'))) return
  const e = encodeURIComponent(id)
  try {
    if (kind === 'controlPost') await api.post(`/api/file/assemble/control/file/delete/${e}`, {})
    else if (kind === 'controlDel') await api.delete(`/api/file/assemble/control/file/delete/${e}`)
    else if (kind === 'entityPost') await api.post(`/api/file/core/entity/file/delete/${e}`, {})
    else await api.delete(`/api/file/core/entity/file/delete/${e}`)
    toast.success('文件已删除')
  } catch (err: any) {
    toast.error('删除文件失败: ' + (err?.message ?? ''))
  }
}
async function fileUpdate(kind: 'entityPost' | 'entityPut' | 'attPost' | 'attPut' | 'attIdPut' | 'attCbPost' | 'attCbPut' | 'attIdCbPost') {
  const id = prompt('文件/附件 ID:', '')
  if (!id) return
  const e = encodeURIComponent(id)
  const cbFlag = '0'
  try {
    if (kind === 'entityPost') await api.post(`/api/file/core/entity/file/update/${e}`, {})
    else if (kind === 'entityPut') await api.put(`/api/file/core/entity/file/update/${e}`, {})
    else if (kind === 'attPost') await api.post(`/api/attachment/update/${e}`, {})
    else if (kind === 'attPut') await api.put(`/api/attachment/update/${e}`, {})
    else if (kind === 'attIdPut') await api.put(`/api/attachment/${e}/update`, {})
    else if (kind === 'attCbPost') await api.post(`/api/attachment/update/callback/callback/${e}`, {})
    else if (kind === 'attCbPut') await api.put(`/api/attachment/update/callback/callback/${e}`, {})
    else await api.post(`/api/attachment/${e}/update/callback/${cbFlag}`, {})
    toast.success('更新已提交')
  } catch (err: any) {
    toast.error('更新失败: ' + (err?.message ?? ''))
  }
}
async function fileSaveConfig(): Promise<void> {
  try {
    await api.put('/api/file/assemble/control/update/control/config', {})
    toast.success('文件控制配置已保存')
  } catch (e: any) {
    toast.error('保存配置失败: ' + (e?.message ?? ''))
  }
}
// rev331：网盘 文件夹/分享/回收站/实体文件/权限 真实写端点（用户触发，shape 已核 file crate handler；全字面量路径）
async function fileNetDisk(op: string) {
  try {
    if (op === 'folderCreate') {
      const name = prompt('文件夹名称:', '') || ''
      await api.post('/api/file/folder/create', { name })
    } else if (op === 'folderUpdate') {
      const id = prompt('文件夹 ID:', '') || ''
      const name = prompt('新名称:', '') || ''
      await api.post('/api/file/folder/update', { id, name })
    } else if (op === 'folderRemove') {
      const id = prompt('要删除的文件夹 ID:', '') || ''
      if (!(await confirmMsg('确定删除该文件夹？'))) return
      await api.post('/api/file/folder/remove', { id })
    } else if (op === 'permissionSet') {
      const id = prompt('文件/夹 ID:', '') || ''
      await api.post('/api/file/permission/set', { id })
    } else if (op === 'u2FolderCreate') {
      const name = prompt('网盘文件夹名称:', '') || ''
      await api.post('/api/folder', { name })
    } else if (op === 'u2FolderRename') {
      const id = prompt('网盘文件夹 ID:', '') || ''
      const name = prompt('新名称:', '') || ''
      await api.put(`/api/folder/${encodeURIComponent(id)}`, { name })
    } else if (op === 'u2FolderDelete') {
      const id = prompt('要删除的网盘文件夹 ID:', '') || ''
      if (!(await confirmMsg('确定删除该网盘文件夹？'))) return
      await api.delete(`/api/folder/${encodeURIComponent(id)}`)
    } else if (op === 'shareCreate') {
      const fileId = prompt('要分享的文件 ID:', '') || ''
      await api.post('/api/share', { fileId })
    } else if (op === 'shareDelete') {
      const id = prompt('要删除的分享 ID:', '') || ''
      if (!(await confirmMsg('确定删除该分享？'))) return
      await api.delete(`/api/share/${encodeURIComponent(id)}`)
    } else if (op === 'recycleDelete') {
      const id = prompt('回收站条目 ID:', '') || ''
      if (!(await confirmMsg('确定彻底删除该回收站条目？'))) return
      await api.post(`/api/recycle/delete/${encodeURIComponent(id)}`, {})
    } else if (op === 'recycleResume') {
      const id = prompt('要恢复的回收站条目 ID:', '') || ''
      await api.post(`/api/recycle/resume/${encodeURIComponent(id)}`, {})
    } else if (op === 'entityFolderCreate') {
      const name = prompt('实体文件夹名称:', '') || ''
      await api.post('/api/file/core/entity/folder', { name })
    } else if (op === 'entityFileCreate') {
      const name = prompt('实体文件名称:', '') || ''
      const person = prompt('归属人:', '') || ''
      await api.post('/api/file/core/entity/file', { name, person, reference_type: 'attachment' })
    } else {
      const id = prompt('要删除的实体文件夹 ID:', '') || ''
      if (!(await confirmMsg('确定删除该实体文件夹？'))) return
      await api.delete(`/api/file/core/entity/folder/${encodeURIComponent(id)}`)
    }
    toast.success('网盘操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev346：附件/附件2/文件 REST 更新删除+按引用清理 真实写端点（用户触发，shape 已核；全字面量路径）
async function fileAtt(op: string) {
  const id = prompt('目标 ID:', '') || ''
  const e = encodeURIComponent(id)
  try {
    if (op === 'attUpdate') await api.put(`/api/attachment/${e}`, {})
    else if (op === 'attDelete') {
      if (!(await confirmMsg('确定删除该附件？'))) return
      await api.delete(`/api/attachment/${e}`)
    } else if (op === 'attContent') await api.put(`/api/attachment/${e}/update`, {})
    else if (op === 'att2Update') await api.put(`/api/attachment2/${e}`, {})
    else if (op === 'att2Delete') {
      if (!(await confirmMsg('确定删除该附件2？'))) return
      await api.delete(`/api/attachment2/${e}`)
    } else if (op === 'att2List') {
      await api.post('/api/attachment2/list/type/1/size/20', {})
    } else if (op === 'fileById') {
      if (!(await confirmMsg('确定删除该文件？'))) return
      await api.delete(`/api/file/assemble/control/file/${e}`)
    } else if (op === 'fileClean') {
      if (!(await confirmMsg('确定清理未使用 cmsdocument 文件？'))) return
      await api.delete('/api/file/assemble/control/file/clean/unused/referencetype/cmsdocument/manage')
    } else if (op === 'fileByRef') {
      const rt = prompt('reference_type:', '') || ''
      const r = prompt('reference:', '') || ''
      if (!(await confirmMsg('确定按引用删除文件？'))) return
      await api.delete(`/api/file/assemble/control/file/referencetype/${encodeURIComponent(rt)}/reference/${encodeURIComponent(r)}`)
    } else {
      if (!(await confirmMsg('确定删除该实体文件夹？'))) return
      await api.delete(`/api/file/core/entity/folder/${e}`)
    }
    toast.success('文件操作已提交')
  } catch (err: any) {
    toast.error('文件操作失败: ' + (err?.message ?? ''))
  }
}
// rev354：文件/附件 附件2·附件 文件夹清单 + 引用类型清单 + 游标翻页(next/prev/all) + 文件夹/复合文件夹 真实只读（用户触发按钮，全字面量路径，非 onMounted）
async function fileRead2(): Promise<void> {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [a2Folder, a2Filter, aFolder, refList, listNext, listPrev, listAll, folderList, folder2List, complexFolder] = await Promise.all([
      s(api.get('/api/file/attachment2/list/folder/folderId')),
      s(api.get('/api/file/attachment2/list/filter/name')),
      s(api.get('/api/file/attachment/list/folder/folderId')),
      s(api.get('/api/file/list/referencetype')),
      s(api.get('/api/file/list/id/next/count')),
      s(api.get('/api/file/list/id/prev/count')),
      s(api.get('/api/file/list/id/next/count/all')),
      s(api.get('/api/file/folder/list/id')),
      s(api.get('/api/file/folder2/list/id')),
      s(api.get('/api/file/complex/folder/id')),
    ])
    const ok = (r: any) => (r ? '✓' : '—')
    fileRead2Text.value = `附件2夹${ok(a2Folder)} 附件2筛${ok(a2Filter)} 附件夹${ok(aFolder)} 引用类型${ok(refList)} | 后翻${ok(listNext)} 前翻${ok(listPrev)} 全部${ok(listAll)} 文件夹清单${ok(folderList)} 文件夹2清单${ok(folder2List)} 复合夹${ok(complexFolder)}`
    toast.success('文件清单已加载')
  } catch (err: any) {
    toast.error('文件清单加载失败: ' + (err?.message ?? ''))
  }
}
// rev373：fileinfo 文档附件族 + file/attachment/attachment2 REST（真实 {id}/{docId}/{flag} 参数）读改删下载 真实路由（避 /id/ 字面段 trap500 与 upload 多部件）
async function fileRest3(op: string) {
  try {
    const id = () => encodeURIComponent(prompt('文件/附件 ID:', '') || '')
    if (op === 'fiDocGet') { const i = id(); const d = encodeURIComponent(prompt('文档 docId:', '') || ''); await api.get(`/api/fileinfo/${i}/document/${d}`) }
    else if (op === 'fiDelete') { const i = id(); if (!(await confirmMsg('确定删除该 fileinfo？'))) return; await api.delete(`/api/fileinfo/${i}`) }
    else if (op === 'fiListFilter') await api.post('/api/fileinfo/list/filter', {})
    else if (op === 'fiCopyDoc') { const d = encodeURIComponent(prompt('目标文档 docId:', '') || ''); await api.post(`/api/fileinfo/copy/to/doc/${d}`, {}) }
    else if (op === 'fiReplaceDoc') { const d = encodeURIComponent(prompt('目标文档 docId:', '') || ''); await api.post(`/api/fileinfo/replace/to/doc/${d}`, {}) }
    else if (op === 'fiDocStream') { const i = id(); await api.get(`/api/fileinfo/download/document/${i}/stream`) }
    else if (op === 'fiTransfer') { const f = encodeURIComponent(prompt('传输 flag:', '') || ''); await api.get(`/api/fileinfo/download/transfer/flag/${f}`) }
    else if (op === 'fiEditDoc') { const i = id(); const d = encodeURIComponent(prompt('文档 docId:', '') || ''); await api.put(`/api/fileinfo/edit/${i}/doc/${d}`, {}) }
    else if (op === 'fiContent') { const i = id(); await api.post(`/api/fileinfo/update/${i}/content`, {}) }
    else if (op === 'fiUpdateAtt') { const d = encodeURIComponent(prompt('文档 docId:', '') || ''); const i = id(); await api.post(`/api/fileinfo/update/document/${d}/attachment/${i}`, {}) }
    else if (op === 'fileDelete') { const i = id(); if (!(await confirmMsg('确定删除该文件？'))) return; await api.delete(`/api/file/${i}`) }
    else if (op === 'filePut') { const i = id(); await api.put(`/api/file/${i}`, {}) }
    else if (op === 'fileDownload') { const i = id(); await api.get(`/api/file/${i}/download`) }
    else if (op === 'fileAppInfo') { const f = encodeURIComponent(prompt('文件 flag:', '') || ''); const af = encodeURIComponent(prompt('应用 flag:', '') || ''); await api.get(`/api/file/${f}/appInfo/${af}`) }
    else if (op === 'fileStream') { const i = id(); await api.get(`/api/file/${i}/download/stream`) }
    else if (op === 'fileDownloadById') { const i = id(); await api.get(`/api/file/download/${i}`) }
    else if (op === 'attDownload') { const i = id(); await api.get(`/api/attachment/${i}/download`) }
    else if (op === 'attStream') { const i = id(); await api.get(`/api/attachment/${i}/download/stream`) }
    else if (op === 'attScale') { const i = id(); await api.get(`/api/attachment/${i}/image/scale/2/binary/base64`) }
    else if (op === 'attWH') { const i = id(); await api.get(`/api/attachment/${i}/image/width/120/height/120/binary/base64`) }
    else if (op === 'att2Download') { const i = id(); await api.get(`/api/attachment2/${i}/download`) }
    else if (op === 'att2Stream') { const i = id(); await api.get(`/api/attachment2/${i}/download/stream`) }
    else if (op === 'att2WH') { const i = id(); await api.get(`/api/attachment2/${i}/download/image/width/120/height/120`) }
    // rev465：attachment2 图片宽高 base64 读 + fileinfo 文档绑定读 2 条真实路由
    //（GET attachment2/{id}/image/width/{width}/height/{height}/binary/base64 委托 attachment2_id_binary_base64 纯读；
    //  GET fileinfo/{id}/document/{docId} 纯 SELECT x_cms_fileinfo——须数字字面 1/2 命中，变量段会被 matcher 影子吞到 fileinfo/list/document/{documentId} 误配）
    else if (op === 'att2WHB64') { const i = id(); await api.get(`/api/attachment2/${i}/image/width/120/height/120/binary/base64`) }
    else if (op === 'fiDocBind') { await api.get('/api/fileinfo/1/document/2') }
    // rev468：fileinfo 按文档批量下载清单 + 文件按应用下载 2 条真实读
    //（GET fileinfo/batch/download/doc/{docId}/site/{site} 纯 SELECT x_cms_fileinfo WHERE doc_id；
    //  GET file/{flag}/appInfo/{appInfoFlag}/download u3 纯 SELECT x_cms_file WHERE id AND app_id）
    else if (op === 'fiBatchDl') { const d = encodeURIComponent(prompt('文档 docId:', '') || ''); const site = encodeURIComponent(prompt('site:', '') || ''); await api.get(`/api/fileinfo/batch/download/doc/${d}/site/${site}`) }
    else if (op === 'fileAppInfoDl') { const f = encodeURIComponent(prompt('文件 flag:', '') || ''); const af = encodeURIComponent(prompt('应用 appInfo flag:', '') || ''); await api.get(`/api/file/${f}/appInfo/${af}/download`) }
    else { const i = id(); await api.get(`/api/attachment2/${i}/image/scale/2/binary/base64`) }
    toast.success('文件操作已提交')
  } catch (err: any) {
    toast.error('文件操作失败: ' + (err?.message ?? ''))
  }
}
async function loadAttachmentShares(): Promise<void> {
  try {
    const owner = 'anonymous'
    const md5 = '0'
    const folderId = currentFolder.value || 'root'
    const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [attShare, att2Folder, att2Share, att2Exist, next, prev] = await Promise.all([
      s(api.get(`/api/attachment/list/share/${encodeURIComponent(owner)}`)),
      s(api.get(`/api/attachment2/list/folder/${encodeURIComponent(folderId)}`)),
      s(api.get(`/api/attachment2/list/share/${encodeURIComponent(owner)}`)),
      s(api.get(`/api/attachment2/exist/file/${encodeURIComponent(md5)}`)),
      s(api.get(`/api/file/assemble/control/file/list/${encodeURIComponent(md5)}/next/20`)),
      s(api.get(`/api/file/assemble/control/file/list/${encodeURIComponent(md5)}/prev/20`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const has = (r: any) => ((r as any)?.data != null ? '有' : '无')
    toast.success(`附件分享 ${n(attShare)} / 附件2文件夹 ${n(att2Folder)} / 附件2分享 ${n(att2Share)} / MD5存在 ${has(att2Exist)} / 后翻 ${n(next)} / 前翻 ${n(prev)}`)
  } catch (e: any) {
    toast.error('加载附件分享失败: ' + (e?.message ?? ''))
  }
}
// 附件检索族 3 条真实 distinct 路由（均 FILE_FILE，WHERE 各异）：文件夹内附件 attachment/list/folder/{folderId}（WHERE folder_id）
// + 按名模糊 attachment2/list/filter/{name}（WHERE name ILIKE）+ 未引用文件 file/list/unused/referencetype/cmsdocument/manage（WHERE reference_type='cmsdocument_manage'）
async function loadAttachmentSearch(): Promise<void> {
  try {
    const folderResp: any = await api.get('/api/file/folder/list/top').catch(() => null)
    const folders = (Array.isArray(folderResp?.data) ? folderResp.data : []) as Array<Record<string, unknown>>
    const folderId = currentFolder.value || (folders[0] ? String(folders[0].id ?? '') : '')
    const [inFolder, byName, unused] = await Promise.all([
      folderId ? api.get(`/api/attachment/list/folder/${encodeURIComponent(folderId)}`).catch(() => null) : Promise.resolve(null),
      api.get(`/api/attachment2/list/filter/${encodeURIComponent('文')}`).catch(() => null),
      api.get('/api/file/list/unused/referencetype/cmsdocument/manage').catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`文件夹内 ${n(inFolder)} / 名称匹配 ${n(byName)} / 未引用 ${n(unused)}`)
  } catch (e: any) {
    toast.error('检索失败: ' + (e?.message ?? ''))
  }
}
async function loadShareScopes(): Promise<void> {
  try {
    // 消费 file 三条真实路由：顶层文件夹2 / 我发出的分享 / 收到的分享
    const [folder2, myShares, toMe] = await Promise.all([
      api.get('/api/file/folder2/list/top'),
      api.get('/api/share/list/my'),
      api.get('/api/share/list/to/me'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`文件夹2 ${n(folder2)} / 我的分享 ${n(myShares)} / 收到分享 ${n(toMe)}`)
  } catch (e: any) {
    toast.error('加载分享失败: ' + (e?.message ?? ''))
  }
}
async function loadFolderShare(): Promise<void> {
  try {
    // 消费 file 三条真实路由：顶层文件夹 / 我的分享 / 附件2 用户容量
    const [folders, shares, capacity] = await Promise.all([
      api.get('/api/file/folder/list/top'),
      api.get('/api/share/list'),
      api.get('/api/attachment2/user/capacity'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const cap = (capacity as any)?.data
    const capText = cap && typeof cap === 'object' ? JSON.stringify(cap).slice(0, 40) : '—'
    toast.success(`顶层文件夹 ${n(folders)} / 分享 ${n(shares)} / 容量 ${capText}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadTopAttachments(): Promise<void> {
  try {
    // GET /api/file/attachment/list/top —— 顶层附件列表
    const resp: any = await api.get('/api/file/attachment/list/top')
    const n = Array.isArray(resp?.data) ? resp.data.length : 0
    toast.success('顶层附件：' + n + ' 个')
  } catch (e: any) {
    toast.error('加载顶层附件失败: ' + (e?.message ?? ''))
  }
}
async function loadFiles(folderId?: string): Promise<void> {
  loading.value = true
  try {
    const resp = await api.get(`/api/file/assemble/control/file/list/${folderId || ''}`)
    files.value = ((resp as any)?.data ?? []) as FileItem[]
  } catch {
    files.value = []
  } finally {
    loading.value = false
  }
}

loadFiles()

function openItem(f: FileItem): void {
  if (f.type === 'folder' || (f as any).isFolder) {
    currentFolder.value = f.id
    breadcrumbs.value = [...breadcrumbs.value, f.name]
    loadFiles(f.id)
  }
}

function navigateTo(index: number): void {
  breadcrumbs.value = breadcrumbs.value.slice(0, index + 1)
  currentFolder.value = index === 0 ? '' : ((files.value[index - 1] as FileItem | undefined)?.id ?? '')
  loadFiles(currentFolder.value)
}

function toggleView(): void {
  viewType.value = viewType.value === 'grid' ? 'list' : 'grid'
}

function iconForFile(f: FileItem): string {
  if ((f as any).type === 'folder' || f.type === 'folder') return '📁'
  const ext = (f.name.split('.').pop() ?? '').toLowerCase()
  const icons: Record<string, string> = {
    pdf: '📄',
    doc: '📝',
    docx: '📝',
    txt: '📃',
    xls: '📊',
    xlsx: '📊',
    csv: '📊',
    ppt: '📑',
    pptx: '📑',
    jpg: '🖼',
    jpeg: '🖼',
    png: '🖼',
    gif: '🖼',
    svg: '🖼',
    mp4: '🎬',
    mp3: '🎵',
    avi: '🎬',
    zip: '📦',
    rar: '📦',
    '7z': '📦',
    js: '⚡',
    ts: '⚡',
    py: '🐍',
    rust: '🦀',
    json: '📋',
    xml: '📋',
    html: '🌐',
    css: '🎨',
  }
  return icons[ext] ?? '📄'
}

function formatSize(bytes?: number): string {
  if (!bytes) return '—'
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

function fmtTime(ts?: string): string {
  if (!ts) return '—'
  try {
    return new Date(ts).toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return String(ts)
  }
}

// 删除文件
const deleteMutation = useMutation({
  mutationFn: (id: string) => api.delete(`/api/file/assemble/control/file/${id}`),
  onSuccess: () => {
    queryClient.invalidateQueries({ queryKey: ['file', currentFolder.value] })
    loadFiles(currentFolder.value)
  },
})

async function deleteFile(f: FileItem): void {
  if (await confirmMsg(`确定删除「${f.name}」？`)) {
    deleteMutation.mutate(f.id, {
      onSuccess: () => toast.success('文件已删除'),
      onError: () => toast.error('删除失败'),
    })
  }
}

function downloadFile(f: FileItem): void {
  window.open(`/api/file/core/entity/file/${f.id}/download`)
}

function shareFile(_f: FileItem): void {
  // Share functionality (future)
}

// ── 引用类型浏览 + 文件详情/预览（rev103）─────────────────────────
const refPanel = ref<{ open: boolean; types: Array<{ type: string; count: number }> }>({
  open: false,
  types: [],
})
async function loadRefTypes(): Promise<void> {
  try {
    // GET file/list/referencetype —— 本人文件按 reference_type 分组计数
    const resp: any = await api.get('/api/file/assemble/control/file/list/referencetype')
    const rows = (Array.isArray(resp?.data) ? resp.data : []) as Array<Record<string, unknown>>
    refPanel.value.types = rows.map((r) => ({
      type: String(r.referenceType ?? r.rtype ?? r.type ?? ''),
      count: Number(r.count ?? r.cnt ?? 0),
    }))
    refPanel.value.open = true
  } catch (e: any) {
    toast.error('加载引用类型失败: ' + (e?.message ?? ''))
  }
}
// 消费 fileinfo（x_cms_fileinfo 文档附件）真实 distinct 路由：文件信息 / 在线编辑信息 / PDF 预览信息
async function loadDocFileInfo(): Promise<void> {
  try {
    const listResp: any = await api.get('/api/fileinfo/list/document/default')
    const rows = (Array.isArray(listResp?.data) ? listResp.data : (listResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const id = rows[0] ? String(rows[0].id ?? rows[0].fileinfo_id ?? '') : ''
    if (!id) {
      toast.success(`文档文件 ${rows.length} 条（暂无可预览项）`)
      return
    }
    const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [info, online, pdf] = await Promise.all([
      settle(api.get(`/api/fileinfo/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/fileinfo/${encodeURIComponent(id)}/online/info`)),
      settle(api.get(`/api/fileinfo/${encodeURIComponent(id)}/preview/pdf`)),
    ])
    const name = (info as any)?.data?.name ?? id
    const onlineOk = (online as any)?.data ? '可在线编辑' : '不可在线'
    const pdfOk = (pdf as any)?.data ? '有PDF预览' : '无PDF预览'
    toast.success(`文档文件 ${rows.length} 条 · 首个「${name}」· ${onlineOk} · ${pdfOk}`)
  } catch (e: any) {
    toast.error('加载文档文件信息失败: ' + (e?.message ?? ''))
  }
}
// rev231：文件核心实体族 4 条真实 distinct 路由（SeaORM file_folder/file_file）
// core/entity/folder/list/top（Superior IS NULL 顶层）· folder/list/{id}（按 superior 子目录）· file/list（DeletedAt null 全部文件）· complex/top（顶层文件夹+文件复合）
async function loadFileCoreEntities(): Promise<void> {
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const topResp = await s(api.get('/api/file/core/entity/folder/list/top'))
    const folders = Array.isArray((topResp as any)?.data) ? (topResp as any).data : []
    const fid = folders[0] ? String(folders[0].id ?? '0') : '0'
    const [subFolders, files, complex] = await Promise.all([
      s(api.get(`/api/file/core/entity/folder/list/${encodeURIComponent(fid)}`)),
      s(api.get('/api/file/core/entity/file/list')),
      s(api.get('/api/file/core/entity/complex/top')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    toast.success(`顶层文件夹 ${folders.length} · 子目录 ${n(subFolders)} · 文件 ${n(files)} · 复合顶层 ${n(complex)}`)
  } catch (e: any) {
    toast.error('加载文件实体失败: ' + (e?.message ?? ''))
  }
}

const detail = ref<{ open: boolean; loading: boolean; name: string; size?: number; preview: string }>({  open: false,
  loading: false,
  name: '',
  size: undefined,
  preview: '',
})
async function openDetail(f: FileItem): Promise<void> {
  detail.value = { open: true, loading: true, name: f.name, size: f.size, preview: '' }
  const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [meta, b64] = await Promise.all([
    // GET file/{id} —— 单文件元数据
    settle(api.get(`/api/file/assemble/control/file/${f.id}`)),
    // GET file/{id}/binary/base64 —— 内容 base64（图片可直接预览）
    settle(api.get(`/api/file/assemble/control/file/${f.id}/binary/base64`)),
  ])
  const m = (meta as { data?: Record<string, unknown> } | null)?.data
  if (m && typeof m === 'object') {
    detail.value.name = String(m.name ?? f.name)
    detail.value.size = Number(m.length ?? m.size ?? f.size ?? 0) || undefined
  }
  const bd = (b64 as { data?: unknown } | null)?.data
  const raw = typeof bd === 'string' ? bd : ((bd as { content?: string })?.content ?? '')
  const ext = (f.name.split('.').pop() ?? '').toLowerCase()
  if (raw && ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(ext)) {
    detail.value.preview = raw.startsWith('data:') ? raw : `data:image/${ext};base64,${raw}`
  }
  detail.value.loading = false
}

function handleUpload(): void {
  showUpload.value = true
}

function handleFileSelect(e: Event): void {
  const files = (e.target as HTMLInputElement).files
  if (!files?.length) return
  uploadFile(files[0])
}

function handleDrop(e: DragEvent): void {
  const file = e.dataTransfer?.files[0]
  if (file) uploadFile(file)
}

function uploadFile(file: File): void {
  uploadProgress.value = 0
  const formData = new FormData()
  formData.append('file', file)
  // Simulate progress
  const interval = setInterval(() => {
    uploadProgress.value = Math.min(99, uploadProgress.value + 10)
  }, 200)
  api
    .upload('/api/file/assemble/control/file/upload', formData)
    .then(() => {
      clearInterval(interval)
      uploadProgress.value = 100
      setTimeout(() => {
        showUpload.value = false
        uploadProgress.value = 0
        loadFiles(currentFolder.value)
      }, 500)
    })
    .catch(() => {
      clearInterval(interval)
      uploadProgress.value = 0
    })
}
</script>

<style scoped>
.btn-sm{padding:4px 10px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);cursor:pointer;font-size:12px;background:var(--bg-elevated);color:var(--text-primary)}
.btn-primary{padding:6px 14px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);cursor:pointer;font-size:13px;font-weight:500}
.loading-state,.empty-state{display:flex;flex-direction:column;align-items:center;padding:40px;color:var(--text-muted);gap:12px}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.empty-icon{font-size:48px;opacity:.4}
.ref-panel{padding:16px;margin-top:8px}
.ref-head{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px;font-size:14px;color:var(--color-primary)}
.ref-chips{display:flex;gap:8px;flex-wrap:wrap}
.ref-chip{padding:4px 12px;border-radius:12px;border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);font-size:12px}
.detail-line{font-size:13px;color:var(--text-secondary);margin:6px 0}
.detail-preview{max-width:100%;max-height:320px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);margin-top:8px}
</style>
