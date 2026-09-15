<script setup lang="ts">
import { onLoad } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import { fileApi, processApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import {
  initialValues,
  type MobileFormField,
  type MobileFormGroup,
  parseMobileForm,
  validateSimpleForm,
} from '@/utils/form'

const session = useSession()

const processes = ref<Array<{ id: string; name: string }>>([])
const loading = ref(false)
const submitTip = ref('')

const showForm = ref(false)
const formLoading = ref(false)
const selectedProcessId = ref('')
const selectedProcessName = ref('')
const title = ref('')
const fields = ref<MobileFormField[]>([])
const groups = ref<MobileFormGroup[]>([])
const values = ref<Record<string, string>>({})
const errors = ref<Record<string, string>>({})
const submitting = ref(false)

// 附件（FILE_FILE）：fieldKey → 已上传附件 {id,name}[]
const attachments = ref<Record<string, Array<{ id: string; name: string }>>>({})
const uploadingKey = ref('')

onLoad(async () => {
  if (!(await ensureAuthenticated())) {
    uni.navigateBack()
    return
  }
  loadProcesses()
})

async function loadProcesses() {
  loading.value = true
  submitTip.value = ''
  try {
    const resp = await processApi.startableProcesses()
    const payload: unknown = resp.data
    const rows: unknown[] = Array.isArray(payload) ? payload : ((payload as { data?: unknown[] })?.data ?? [])
    processes.value = (rows as Array<Record<string, unknown>>)
      .map((row) => ({ id: String(row.id ?? ''), name: String(row.name ?? row.id ?? '') }))
      .filter((p) => p.id)
  } catch {
    processes.value = []
  } finally {
    loading.value = false
  }
}

function pickProcess(p: { id: string; name: string }): void {
  if (p.id === selectedProcessId.value) return
  selectedProcessId.value = p.id
  selectedProcessName.value = p.name
  values.value = {}
  errors.value = {}
  fields.value = []
  groups.value = []
  attachments.value = {}
  void loadBoundForm(p.id)
}

async function loadBoundForm(processId: string): Promise<void> {
  formLoading.value = true
  try {
    const detail = await processApi.getProcess(processId)
    const definition = (detail.data?.processDefinition ?? detail.data) as Record<string, unknown> | undefined
    const begin = definition?.begin as Record<string, unknown> | undefined
    const manualList = definition?.manualList as Array<Record<string, unknown>> | undefined
    const formFlag =
      (typeof begin?.form === 'string' && begin.form) ||
      (typeof manualList?.[0]?.form === 'string' && manualList[0].form) ||
      ''
    if (!formFlag) {
      fields.value = []
      groups.value = []
      showForm.value = false
      return
    }
    const formResp: { data?: unknown } = await processApi.getForm(formFlag)
    const form = parseMobileForm(formResp.data)
    fields.value = form.fields
    groups.value = form.groups
    values.value = initialValues(form.fields)
    showForm.value = form.fields.length > 0
  } catch {
    fields.value = []
    groups.value = []
    showForm.value = false
  } finally {
    formLoading.value = false
  }
}

/** 按 groupLabel 分组渲染（默认组 groupLabel='' 归到无标题分区）。 */
const groupOrder = computed(() => {
  const order: string[] = []
  for (const g of groups.value) order.push(g.label || g.id)
  // 默认组（无容器）恒在最前
  if (fields.value.some((f) => f.groupId === '')) order.unshift('')
  return order
})
function fieldsInGroup(label: string): MobileFormField[] {
  if (label === '') return fields.value.filter((f) => f.groupId === '')
  return fields.value.filter((f) => (f.groupLabel || f.groupId) === label || (label && f.groupId === label))
}
function groupTitle(label: string): string {
  if (label === '') return ''
  return groups.value.find((g) => (g.label || g.id) === label)?.label || ''
}

async function submit(): Promise<void> {
  if (!selectedProcessId.value || !title.value.trim()) {
    submitTip.value = '请选择流程并填写标题'
    return
  }
  if (fields.value.length) {
    errors.value = validateSimpleForm(fields.value, values.value)
    if (Object.keys(errors.value).length) {
      submitTip.value = '请先修正表单校验错误'
      return
    }
  }
  submitTip.value = ''
  submitting.value = true
  try {
    const created = await processApi.startWork(selectedProcessId.value, title.value.trim())
    const workId = String(created?.data?.id ?? '')
    if (!workId) throw new Error('后端未返回工作 ID')
    if (fields.value.length) {
      // 附件字段：把已上传的 FILE_FILE 附件 id 列表写回 values（后端 saveWorkData 接收）。
      const payload: Record<string, unknown> = { ...values.value }
      for (const f of fields.value) {
        if (f.type !== 'attachment') continue
        const list = attachments.value[f.key] ?? []
        payload[f.key] = list.map((a) => a.id).join(',')
      }
      await processApi.saveWorkData(workId, payload)
    }
    uni.showToast({ title: '流程已发起', icon: 'success' })
    uni.setStorageSync('oa_process_active_tab', 'started')
    uni.switchTab({ url: '/pages/process/process' })
  } catch (e) {
    submitTip.value = `发起失败：${e instanceof Error ? e.message : '未知错误'}`
  } finally {
    submitting.value = false
  }
}

function onFieldInput(field: MobileFormField, e: { detail: { value: string } }): void {
  values.value[field.key] = e.detail.value
  if (errors.value[field.key]) delete errors.value[field.key]
}
function onFieldSelect(field: MobileFormField, e: { detail: { value: number } }): void {
  const idx = Number(e.detail.value)
  values.value[field.key] = field.options[idx]?.value ?? ''
  if (errors.value[field.key]) delete errors.value[field.key]
}
function isCheckOn(field: MobileFormField, value: string): boolean {
  const cur = String(values.value[field.key] ?? '')
  return cur ? cur.split(',').includes(value) : false
}
function toggleCheck(field: MobileFormField, value: string): void {
  const parts = String(values.value[field.key] ?? '')
    .split(',')
    .filter(Boolean)
  const idx = parts.indexOf(value)
  if (idx >= 0) parts.splice(idx, 1)
  else parts.push(value)
  values.value[field.key] = parts.join(',')
  if (errors.value[field.key]) delete errors.value[field.key]
}

// ── 附件控件（P3/P4：上传落点 = 附件存储 FILE_FILE，非 x_file）────────────
function myFolder(): string {
  return session.user?.unique ?? ''
}
function pickAttachment(field: MobileFormField): void {
  if (field.readonly) return
  const picker = (uni as unknown as { chooseMessageFile?: (o: unknown) => void }).chooseMessageFile
  const useFilePicker = typeof picker === 'function'
  const onPicked = (path: string, name: string) => void uploadAttachment(field, path, name)
  if (useFilePicker) {
    picker({
      count: 1,
      type: 'file',
      success: (res: { tempFiles?: Array<{ path?: string; name?: string }> }) => {
        const f = res.tempFiles?.[0]
        if (f?.path) onPicked(f.path, f.name || 'file')
        else uni.showToast({ title: '未选择文件', icon: 'none' })
      },
      fail: () => uni.showToast({ title: '未选择文件', icon: 'none' }),
    })
  } else {
    uni.chooseImage({
      count: 1,
      success: (res) => {
        const f = res.tempFiles?.[0]
        if (f) onPicked(f.path, f.name || 'image')
        else uni.showToast({ title: '未选择文件', icon: 'none' })
      },
      fail: () => uni.showToast({ title: '未选择文件', icon: 'none' }),
    })
  }
}
async function uploadAttachment(field: MobileFormField, filePath: string, fileName: string): Promise<void> {
  const folder = myFolder()
  if (!folder || !filePath) {
    uni.showToast({ title: '未登录或无文件', icon: 'none' })
    return
  }
  uploadingKey.value = field.key
  try {
    const resp: { data?: { id?: string; name?: string } } = (await fileApi.upload(
      folder,
      filePath,
      fileName || 'file',
    )) as unknown as { data?: { id?: string; name?: string } }
    const id = String(resp?.data?.id ?? '')
    if (!id) throw new Error('后端未返回附件 ID')
    const existing = attachments.value[field.key]
    if (existing) {
      existing.push({ id, name: resp?.data?.name || fileName })
    } else {
      attachments.value[field.key] = [{ id, name: resp?.data?.name || fileName }]
    }
    if (errors.value[field.key]) delete errors.value[field.key]
    uni.showToast({ title: '附件已上传', icon: 'success' })
  } catch (e) {
    uni.showToast({ title: `附件上传失败：${e instanceof Error ? e.message : '未知'}`, icon: 'none' })
  } finally {
    uploadingKey.value = ''
  }
}
function removeAttachment(field: MobileFormField, id: string): void {
  if (field.readonly) return
  const list = attachments.value[field.key]
  if (!list) return
  const idx = list.findIndex((a) => a.id === id)
  if (idx >= 0) list.splice(idx, 1)
}
</script>

<template>
  <view class="page">
    <view class="card">
      <view class="card-title">选择流程</view>
      <view v-if="loading" class="tip">加载中…</view>
      <view v-else-if="processes.length === 0" class="tip">暂无可发起的流程</view>
      <view v-else class="proc-list">
        <view
          v-for="p in processes"
          :key="p.id"
          class="proc-item"
          :class="{ on: selectedProcessId === p.id }"
          @tap="pickProcess(p)"
        >
          {{ p.name || p.id }}
        </view>
      </view>
    </view>

    <view class="card">
      <view class="field">
        <text class="label">标题</text>
        <input v-model="title" class="input" type="text" placeholder="工作标题（必填）" />
      </view>
    </view>

    <view v-if="showForm" class="card">
      <view class="card-title">填报表单</view>
      <view v-if="formLoading" class="tip">加载表单…</view>
      <template v-else>
        <view v-for="gLabel in groupOrder" :key="gLabel || 'default'" class="group">
          <view v-if="groupTitle(gLabel)" class="group-title">{{ groupTitle(gLabel) }}</view>
          <view v-for="f in fieldsInGroup(gLabel)" :key="f.id" class="field">
            <text class="label">
              {{ f.label }}<text v-if="f.required" class="req">*</text>
            </text>
            <!-- 文本 / 多行 / 富文本（移动端降级为多行文本） -->
            <input
              v-if="f.type === 'text'"
              class="input"
              type="text"
              :placeholder="f.placeholder || '请输入'"
              :value="values[f.key]"
              :disabled="f.readonly"
              @input="onFieldInput(f, $event)"
            />
            <textarea
              v-else-if="f.type === 'textarea' || f.type === 'richtext'"
              class="textarea"
              :placeholder="f.placeholder || (f.type === 'richtext' ? '请输入内容（支持富文本）' : '请输入')"
              :value="values[f.key]"
              :disabled="f.readonly"
              @input="onFieldInput(f, $event)"
            />
            <!-- 数字 -->
            <input
              v-else-if="f.type === 'number'"
              class="input"
              type="digit"
              :placeholder="f.placeholder || '请输入数字'"
              :value="values[f.key]"
              :disabled="f.readonly"
              @input="onFieldInput(f, $event)"
            />
            <!-- 日期 -->
            <input
              v-else-if="f.type === 'date'"
              class="input"
              type="text"
              placeholder="YYYY-MM-DD"
              :value="values[f.key]"
              :disabled="f.readonly"
              @input="onFieldInput(f, $event)"
            />
            <!-- 下拉 -->
            <picker
              v-else-if="f.type === 'select'"
              :range="f.options.map((o) => o.label)"
              :value="Math.max(0, f.options.findIndex((o) => o.value === values[f.key]))"
              :disabled="f.readonly"
              @change="onFieldSelect(f, $event)"
            >
              <view class="picker">
                {{ f.options.find((o) => o.value === values[f.key])?.label || '请选择' }}
              </view>
            </picker>
            <!-- 勾选（多选项） -->
            <view v-else-if="f.type === 'checkbox'" class="checks">
              <label v-for="o in f.options" :key="o.value" class="check">
                <checkbox :checked="isCheckOn(f, o.value)" @tap="toggleCheck(f, o.value)" />
                <text>{{ o.label }}</text>
              </label>
            </view>
            <!-- 附件（上传落点 = 附件存储 FILE_FILE） -->
            <view v-else-if="f.type === 'attachment'" class="attach">
              <view class="attach-list">
                <view v-for="a in attachments[f.key] ?? []" :key="a.id" class="attach-item">
                  <text class="attach-name">📎 {{ a.name || a.id }}</text>
                  <text v-if="!f.readonly" class="attach-del" @tap="removeAttachment(f, a.id)">✕</text>
                </view>
                <text v-if="!(attachments[f.key] ?? []).length" class="attach-empty">未上传附件</text>
              </view>
              <button v-if="!f.readonly" class="attach-btn" :disabled="uploadingKey === f.key" @tap="pickAttachment(f)">
                {{ uploadingKey === f.key ? '上传中…' : '+ 添加附件' }}
              </button>
            </view>
            <text v-if="errors[f.key]" class="err">{{ errors[f.key] }}</text>
          </view>
        </view>
      </template>
    </view>

    <text v-if="submitTip" class="submit-tip">{{ submitTip }}</text>

    <button class="submit" :disabled="submitting || !selectedProcessId" @tap="submit">
      {{ submitting ? '发起中…' : '发起' }}
    </button>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 24rpx;
  box-sizing: border-box;
}
.card {
  background: #fff;
  border-radius: 16rpx;
  padding: 28rpx;
  margin-bottom: 24rpx;
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
  margin-bottom: 20rpx;
}
.group {
  margin-bottom: 8rpx;
}
.group-title {
  font-size: 26rpx;
  font-weight: 600;
  color: #2d8cf0;
  margin: 8rpx 0 16rpx;
  padding-left: 8rpx;
  border-left: 6rpx solid #2d8cf0;
}
.proc-list {
  display: flex;
  flex-direction: column;
  gap: 12rpx;
}
.proc-item {
  padding: 20rpx;
  border: 1px solid #e3e6eb;
  border-radius: 12rpx;
  color: #4a4a4a;
  font-size: 28rpx;
}
.proc-item.on {
  border-color: #2d8cf0;
  color: #2d8cf0;
  background: #f0f7ff;
  font-weight: 600;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  padding: 20rpx 0;
}
.field {
  margin-bottom: 24rpx;
}
.label {
  font-size: 26rpx;
  color: #666;
  display: block;
  margin-bottom: 10rpx;
}
.req {
  color: #f56c6c;
}
.input,
.textarea {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid #e3e6eb;
  border-radius: 12rpx;
  padding: 18rpx;
  font-size: 28rpx;
  background: #f5f7fa;
}
.textarea {
  min-height: 140rpx;
}
.picker {
  border: 1px solid #e3e6eb;
  border-radius: 12rpx;
  padding: 18rpx;
  font-size: 28rpx;
  background: #f5f7fa;
  color: #263238;
}
.checks {
  display: flex;
  flex-wrap: wrap;
  gap: 16rpx;
}
.check {
  display: flex;
  align-items: center;
  gap: 8rpx;
  font-size: 26rpx;
  color: #4a4a4a;
}
.attach {
  display: flex;
  flex-direction: column;
  gap: 12rpx;
}
.attach-list {
  display: flex;
  flex-direction: column;
  gap: 8rpx;
}
.attach-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border: 1px solid #e3e6eb;
  border-radius: 10rpx;
  padding: 12rpx 16rpx;
  background: #f5f7fa;
}
.attach-name {
  font-size: 26rpx;
  color: #263238;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.attach-del {
  color: #f56c6c;
  font-size: 28rpx;
  padding: 0 8rpx;
}
.attach-empty {
  font-size: 24rpx;
  color: #90979f;
}
.attach-btn {
  font-size: 26rpx;
  color: #2d8cf0;
  background: #f0f7ff;
  border: 1px dashed #2d8cf0;
  border-radius: 10rpx;
}
.err {
  color: #f56c6c;
  font-size: 24rpx;
}
.submit-tip {
  display: block;
  color: #f56c6c;
  font-size: 26rpx;
  margin-bottom: 16rpx;
  padding: 0 8rpx;
}
.submit {
  background: #2d8cf0;
  color: #fff;
  border-radius: 12rpx;
  font-size: 32rpx;
}
.submit[disabled] {
  opacity: 0.5;
}
</style>
