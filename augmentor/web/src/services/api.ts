import axios from 'axios'

import type {
  AnalyzeResponse,
  AnnotateResponse,
  AppConfig,
  AuditResponse,
  AugmentProgress,
  BatchExportResponse,
  BenchmarkResponse,
  CheckpointListResponse,
  CleanResponse,
  CreateVersionResponse,
  DataFileListResponse,
  DataItem,
  DataPageResponse,
  DatasetExportResponse,
  DedupResponse,
  ExportFormatsResponse,
  ExportPreviewResponse,
  HealthResponse,
  LeakageResponse,
  ModelsResponse,
  MultimodalFormatsResponse,
  MultimodalRecord,
  MultimodalScanResponse,
  MutationResponse,
  OutliersResponse,
  PiiPatternsResponse,
  ProfilingResponse,
  QualityEvaluationResponse,
  QualityReportResponse,
  SanitizeResponse,
  StartAugmentResponse,
  StatusResponse,
  UpdateConfigResponse,
  UploadResponse,
  VersionDataResponse,
  VersionDetail,
  VersionDiffResponse,
  VersionHistoryResponse,
  VersionListResponse,
  VisualizeResponse,
} from '../types/api'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000,
})

// 数据管理
export const getDataFiles = async (): Promise<DataFileListResponse> => {
  const response = await api.get('/data/list')
  return response.data
}

export const loadData = async (
  filename: string,
  page: number,
  pageSize: number,
  search: string
): Promise<DataPageResponse> => {
  const response = await api.get(`/data/load/${filename}`, {
    params: { page, page_size: pageSize, search },
  })
  return response.data
}

/**
 * 更新单条数据
 *
 * `item` 的字段由数据集本身决定（前端不做建模），所以这里用 `unknown` 而不是 `any`：
 * 语义是"本层不解释这个载荷、原样转发"，调用方的类型由调用方保证。
 * 不写成 `Record<string, unknown>` 是因为 TS 的 interface 没有隐式索引签名，
 * 会让 `DataItem` 这类调用方直接编译失败。
 */
export const updateDataItem = async (
  filename: string,
  index: number,
  item: unknown
): Promise<MutationResponse> => {
  const response = await api.put(`/data/update/${filename}`, item, {
    params: { index },
  })
  return response.data
}

export const deleteDataItem = async (
  filename: string,
  index: number
): Promise<MutationResponse> => {
  const response = await api.delete(`/data/delete/${filename}`, {
    params: { index },
  })
  return response.data
}

export const uploadData = async (file: File): Promise<UploadResponse> => {
  const formData = new FormData()
  formData.append('file', file)
  const response = await api.post('/data/upload', formData)
  return response.data
}

export const exportData = async (
  inputFile: string,
  outputDir: string,
  formats?: string[]
): Promise<DatasetExportResponse> => {
  const response = await api.post('/data/export', {
    input_file: inputFile,
    output_dir: outputDir,
    formats,
  })
  return response.data
}

// 数据增强
export const startAugmentation = async (
  inputFile: string,
  outputFile: string,
  useQuality: boolean,
  useDedup: boolean,
  useCheckpoint: boolean
): Promise<StartAugmentResponse> => {
  const response = await api.post('/augment/start', {
    input_file: inputFile,
    output_file: outputFile,
    use_quality: useQuality,
    use_dedup: useDedup,
    use_checkpoint: useCheckpoint,
  })
  return response.data
}

export const getProgress = async (): Promise<AugmentProgress> => {
  const response = await api.get('/augment/progress')
  return response.data
}

export const getCheckpoints = async (): Promise<CheckpointListResponse> => {
  const response = await api.get('/augment/checkpoints')
  return response.data
}

// 数据分析
export const analyzeData = async (filename: string): Promise<AnalyzeResponse> => {
  const response = await api.get(`/analyze/${filename}`)
  return response.data
}

export const visualizeData = async (filename: string): Promise<VisualizeResponse> => {
  const response = await api.get(`/visualize/${filename}`)
  return response.data
}

// 版本管理
export const getVersions = async (): Promise<VersionListResponse> => {
  const response = await api.get('/versions')
  return response.data
}

export const createVersion = async (
  filename: string,
  label: string,
  description: string
): Promise<CreateVersionResponse> => {
  const response = await api.post(`/versions/create?filename=${filename}`, {
    label,
    description,
  })
  return response.data
}

export const getVersion = async (versionId: string): Promise<VersionDetail> => {
  const response = await api.get(`/versions/${versionId}`)
  return response.data
}

export const getVersionData = async (versionId: string): Promise<VersionDataResponse> => {
  const response = await api.get(`/versions/${versionId}/data`)
  return response.data
}

export const diffVersions = async (
  version1: string,
  version2: string
): Promise<VersionDiffResponse> => {
  const response = await api.post('/versions/diff', {
    version1,
    version2,
  })
  return response.data
}

export const rollbackVersion = async (versionId: string): Promise<MutationResponse> => {
  const response = await api.post(`/versions/${versionId}/rollback`)
  return response.data
}

export const deleteVersion = async (versionId: string): Promise<MutationResponse> => {
  const response = await api.delete(`/versions/${versionId}`)
  return response.data
}

export const getVersionHistory = async (
  limit: number = 20
): Promise<VersionHistoryResponse> => {
  const response = await api.get('/versions/history', { params: { limit } })
  return response.data
}

// 质量评估
export const evaluateQuality = async (
  inputFile: string,
  threshold: number = 0.6
): Promise<QualityEvaluationResponse> => {
  const response = await api.post('/quality/evaluate', {
    input_file: inputFile,
    threshold,
  })
  return response.data
}

export const dedupData = async (
  inputFile: string,
  threshold: number = 0.9
): Promise<DedupResponse> => {
  const response = await api.post('/quality/dedup', {
    input_file: inputFile,
    threshold,
  })
  return response.data
}

export const qualityReport = async (
  inputFile: string,
  threshold: number = 0.6
): Promise<QualityReportResponse> => {
  const response = await api.post('/quality/report', {
    input_file: inputFile,
    threshold,
  })
  return response.data
}

export const cleanData = async (inputFile: string): Promise<CleanResponse> => {
  const response = await api.post('/quality/clean', { input_file: inputFile })
  return response.data
}

export const annotateData = async (inputFile: string): Promise<AnnotateResponse> => {
  const response = await api.post('/quality/annotate', { input_file: inputFile })
  return response.data
}

export const runBenchmark = async (
  inputFile: string,
  threshold: number = 0.6
): Promise<BenchmarkResponse> => {
  const response = await api.post('/quality/benchmark', {
    input_file: inputFile,
    threshold,
  })
  return response.data
}

// 导出
export const getExportFormats = async (): Promise<ExportFormatsResponse> => {
  const response = await api.get('/export/formats')
  return response.data
}

export const previewExport = async (
  inputFile: string,
  format: string,
  size: number = 5
): Promise<ExportPreviewResponse> => {
  const response = await api.post('/export/preview', {
    input_file: inputFile,
    format,
    size,
  })
  return response.data
}

export const batchExport = async (
  datasets: Record<string, string>,
  outputDir: string,
  formats: string[]
): Promise<BatchExportResponse> => {
  const response = await api.post('/export/batch', {
    datasets,
    output_dir: outputDir,
    formats,
  })
  return response.data
}

// 多模态
export const getMultimodalFormats = async (): Promise<MultimodalFormatsResponse> => {
  const response = await api.get('/multimodal/formats')
  return response.data
}

/**
 * 单条多模态处理
 *
 * `text` 可省略：后端 `MultimodalRequest.text` 的默认值就是 `""`，
 * 表单里这一项也标了「可选」。
 */
export const processMultimodal = async (
  text?: string,
  image?: string,
  audio?: string
): Promise<MultimodalRecord> => {
  const response = await api.post('/multimodal/process', { text, image, audio })
  return response.data
}

export const scanMultimodal = async (
  directory: string
): Promise<MultimodalScanResponse> => {
  const response = await api.post('/multimodal/scan', { directory })
  return response.data
}

// 配置管理
export const getConfig = async (): Promise<AppConfig> => {
  const response = await api.get('/config')
  return response.data
}

/**
 * 整份覆盖配置
 *
 * 同 `updateDataItem`：配置的完整结构由后端 `config.yaml` 定义，
 * 前端只做转发，因此用 `unknown` 而非 `any`。
 */
export const updateConfig = async (config: unknown): Promise<UpdateConfigResponse> => {
  const response = await api.post('/config', config)
  return response.data
}

export const getModels = async (): Promise<ModelsResponse> => {
  const response = await api.get('/models')
  return response.data
}

// 健康检查
export const healthCheck = async (): Promise<HealthResponse> => {
  const response = await api.get('/health')
  return response.data
}

// ============ 服务综合状态 ============

/**
 * 服务综合状态（含依赖诊断）
 *
 * 与 `healthCheck` 的区别：health 是给容器探针用的轻量存活检查，
 * status 会构造管道并检查可选依赖，因此能回答「当前环境缺什么」。
 */
export const getServiceStatus = async (): Promise<StatusResponse> => {
  const response = await api.get('/status')
  return response.data
}

// ============ 内置演示数据 ============

/**
 * 拉取内置演示数据集（**裸数组**，不是 `{data: [...]}` 包装）
 *
 * 返回的条目结构与 `DataItem` 一致，可直接喂给 `uploadData` 落盘成数据集。
 */
export const getDemoData = async (): Promise<DataItem[]> => {
  const response = await api.get('/demo/data')
  return response.data
}

// ============ 隐私脱敏 ============

export const getPiiPatterns = async (): Promise<PiiPatternsResponse> => {
  const response = await api.get('/privacy/patterns')
  return response.data
}

/**
 * PII 脱敏
 *
 * `fields` 留空表示按后端默认字段集脱敏；`includeExtra` 打开后会额外套用
 * 一组更激进的模式（误伤率也更高，默认关闭）。
 */
export const sanitizeData = async (
  inputFile: string,
  fields?: string[],
  includeExtra: boolean = false
): Promise<SanitizeResponse> => {
  const response = await api.post('/privacy/sanitize', {
    input_file: inputFile,
    fields,
    include_extra: includeExtra
  })
  return response.data
}

// ============ 训练/测试集泄漏检测 ============

export const checkLeakage = async (
  trainFile: string,
  testFile: string,
  fuzzyThreshold: number = 0.8
): Promise<LeakageResponse> => {
  const response = await api.post('/leakage/check', {
    train_file: trainFile,
    test_file: testFile,
    fuzzy_threshold: fuzzyThreshold
  })
  return response.data
}

// ============ 数据集就绪审计 ============

/**
 * 数据集就绪审计
 *
 * `referenceFile` 留空表示不做「与参照集对比」的那部分检查。
 */
export const auditDataset = async (
  inputFile: string,
  referenceFile?: string
): Promise<AuditResponse> => {
  const response = await api.post('/audit', {
    input_file: inputFile,
    reference_file: referenceFile
  })
  return response.data
}

// ============ 离群点与画像 ============

export const detectOutliers = async (
  inputFile: string,
  method: string = 'zscore',
  threshold: number = 3,
  field: string = 'length'
): Promise<OutliersResponse> => {
  const response = await api.post('/quality/outliers', {
    input_file: inputFile,
    method,
    threshold,
    field
  })
  return response.data
}

export const profileDataset = async (inputFile: string): Promise<ProfilingResponse> => {
  const response = await api.post('/quality/profiling', { input_file: inputFile })
  return response.data
}
