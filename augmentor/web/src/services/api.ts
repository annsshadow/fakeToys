import axios from 'axios'

import type {
  AnalyzeResponse,
  AnnotateResponse,
  AppConfig,
  AuditResponse,
  AugmentProgress,
  AutoConfigResponse,
  BatchExportResponse,
  BenchmarkResponse,
  CheckpointListResponse,
  CleanResponse,
  CompareResponse,
  CreateVersionResponse,
  DataFileListResponse,
  DataItem,
  DataPageResponse,
  DatasetExportResponse,
  ConvertResponse,
  DatasetValidationResponse,
  DedupResponse,
  EvaluateResponse,
  ExportFormatsResponse,
  ExportPreviewResponse,
  FeaturesResponse,
  HealthResponse,
  ImpactResponse,
  LanguageProfileResponse,
  LeakageResponse,
  MergeResponse,
  ModelsResponse,
  MultimodalFormatsResponse,
  MultimodalRecord,
  MultimodalScanResponse,
  MutationResponse,
  OutliersResponse,
  PiiPatternsResponse,
  SemanticSearchResponse,
  ProfilingResponse,
  QualityEvaluationResponse,
  QualityReportResponse,
  SplitResponse,
  SanitizeResponse,
  SearchResponse,
  StartAugmentResponse,
  StatisticsResponse,
  StatusResponse,
  UpdateConfigResponse,
  UploadResponse,
  VersionDataResponse,
  VersionDetail,
  VersionDiffResponse,
  VersionHistoryResponse,
  VersionListResponse,
  VectorStatusResponse,
  VisualizeResponse,
} from '../types/api'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000,
})

/**
 * 大块数据请求的**每请求**超时（毫秒），刻意不动实例默认的 30 s
 *
 * 30 s 对这一类请求是错的，依据是实测而非感觉（A167，取证脚本 `Temp/l99q/upload_timing_l99.py`
 * 与 `Temp/l99q/xlsx_scale_l99.py`）。两档要分开说，因为**慢的不是同一种进料**：
 *
 * - JSON/csv：本机 loopback 端到端近线性，128 MiB 档总 3.30 s ⇒ 约 **36.9 MiB/s**，
 *   也就是默认档在快链路上要 ~1 100 MiB 才被掐断——**超出厂字节闸（256 MiB）**，
 *   所以体积本身撞不穿 30 s，风险只剩真实链路的带宽。
 * - xlsx：全部时间都花在**服务端处理段**（交付段实测 0.00 s），单价 3.65～4.95 s/MiB
 *   （0.028 ms/行）⇒ 30 s 预算 ≈ **108 万行 / 约 8 MiB 压缩后的 xlsx**，
 *   而 256 MiB 的闸顶真喂满 xlsx 要 15.6～21 分钟。这才是「前端先放弃」的现实载体。
 *
 * 取 30 分钟 = 闸顶 256 MiB × 实测最慢的 4.95 s/MiB ≈ 21 分钟再留余量。
 * 只挂在具体请求上，是因为 `api.test.ts` 里那条实例契约（`timeout: 30000`）钉的是
 * 「其余短请求不许被顺手放宽」：列表、状态、进度这类请求 30 s 拿不到就是坏了。
 *
 * 导出那一档要如实分开说：**没有**复现出慢（64 MiB 导出实测 1.29 s）。给它同一档
 * 只是「工作量由数据规模决定、没有上界」这一类的便宜保险，不是测出来的缺陷。
 */
export const BULK_REQUEST_TIMEOUT_MS = 30 * 60 * 1000

/**
 * 从请求错误里取后端写的那句 `detail`，取不到才回 `fallback`
 *
 * FastAPI 的失败体是 `{ "detail": ... }`，而 4xx 的 detail 是**可 actions 的判决**
 * （例如「上传内容整份是一份合法 JSON，但按 csv 解析：名字或 input_format 与内容不符」）。
 * 页面原先一律 `catch { message.error('上传失败') }`，等于把后端专门攒起来的那批诊断
 * 吞成一句没有信息量的「失败」⇒ 这一层负责把它取回来。
 *
 * 两种形状都要处理：400/403/413 的 detail 是字符串，422（校验失败）是
 * `[{ loc, msg, type }]` 数组。`error` 收成 `unknown` 而不是 `any`：本层不解释错误对象
 * 的来历（axios 错误、原生 Error、甚至字符串都可能是），逐层可选访问，取不到就走 fallback。
 */
export const apiErrorDetail = (error: unknown, fallback: string): string => {
  const detail = (error as { response?: { data?: { detail?: unknown } } })?.response?.data
    ?.detail
  if (typeof detail === 'string') {
    return detail || fallback
  }
  if (Array.isArray(detail)) {
    const msgs = detail
      .map((item) => (item as { msg?: unknown })?.msg)
      .filter((msg): msg is string => typeof msg === 'string' && msg.length > 0)
    if (msgs.length > 0) {
      return msgs.join('；')
    }
  }
  return fallback
}

/**
 * 是不是「客户端等不及了」—— 而不是「服务端答错了」
 *
 * 三种适配器给的是两种 code：`fetch` 档报 `ETIMEDOUT`（`composeSignals.js`），
 * `xhr` / `http` 档在 `transitional.clarifyTimeoutError`（默认关）下报 `ECONNABORTED`
 * （`lib/adapters/xhr.js`）。只看 code 不够：`ECONNABORTED` 同时也被用来报
 * 「请求被主动中止」，把用户取消说成超时会指错方向。三条路径的 message 都是
 * `timeout of <n>ms exceeded`，所以用「精确 ETIMEDOUT ∥ ECONNABORTED + 这句 message」
 * 两条并列判别，宁可漏判也不误判。
 */
export const isTimeoutError = (error: unknown): boolean => {
  const e = error as { code?: unknown; message?: unknown } | undefined | null
  if (!e || typeof e !== 'object') return false
  if (e.code === 'ETIMEDOUT') return true
  // `timeout exceeded`（不带秒数）是同一处只在 `config.timeout` 为假值时才走的分支，
  // 本仓的三种请求都带秒数；留在判别式里是因为它同样只可能出自那句 ontimeout。
  return (
    e.code === 'ECONNABORTED' &&
    /^(timeout of \d+ms exceeded|timeout exceeded)$/.test(String(e.message ?? ''))
  )
}

/**
 * 大块请求失败时给用户看的那句
 *
 * 超时单独走一条文案，因为「超时」在这条代码路径上是**已实测的假警报**：客户端交完字节
 * 后 0.6 s 收场（优雅关闭 —— axios 的 timeout 到期就是这个形状，不是 RST），服务端照样把
 * 那份 45 840 001 B 的文件写完（`Temp/l99q/fin_after_drain_l99.py`）。
 *
 * 后半句的口径按实测收窄过一轮：同名二次上传实测是**覆盖**（后写赢，
 * `Temp/l99q/upload_timing_l99.py` 的 R4 档），不是「多落一份」⇒ 这句劝的是「先确认结果」，
 * 不是「怕落重」。
 */
export const bulkErrorDetail = (error: unknown, fallback: string): string => {
  if (isTimeoutError(error)) {
    return (
      `${fallback}：客户端已停止等待，` +
      '但服务端可能仍在处理并已把结果落盘，请先确认结果再重试（重试会覆盖那一份）'
    )
  }
  return apiErrorDetail(error, fallback)
}

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

/**
 * 上传数据
 *
 * `inputFormat` 是**源格式**的显式声明，对应后端的 `input_format` 表单字段；空串等价于
 * 「不传」，由后端按扩展名推断（权威表 `converter.INPUT_FORMAT_CHOICES` /
 * `INPUT_EXTENSION_FORMATS`，漂移守卫在 `tests/unit/test_upload_declared_format_l92.py`）。
 * 无条件 append 而不是「有值才加」：后端默认值就是 `""`，两种写法同一条代码路径，
 * 前端少一条要测的分支。
 */
export const uploadData = async (
  file: File,
  inputFormat = ''
): Promise<UploadResponse> => {
  const formData = new FormData()
  formData.append('file', file)
  formData.append('input_format', inputFormat)
  const response = await api.post('/data/upload', formData, {
    timeout: BULK_REQUEST_TIMEOUT_MS,
  })
  return response.data
}

export const exportData = async (
  inputFile: string,
  outputDir: string,
  formats?: string[]
): Promise<DatasetExportResponse> => {
  const response = await api.post(
    '/data/export',
    {
      input_file: inputFile,
      output_dir: outputDir,
      formats,
    },
    { timeout: BULK_REQUEST_TIMEOUT_MS }
  )
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
  const response = await api.post(
    '/export/batch',
    {
      datasets,
      output_dir: outputDir,
      formats,
    },
    { timeout: BULK_REQUEST_TIMEOUT_MS }
  )
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

// ============ 语言画像 / 批量评估 / 向量库状态（A279 第一批接线，L225） ============

/**
 * `POST /api/dataset/language-profile`
 *
 * `textKey` 省略 ⇒ 服务端取模型默认 `instruction`，本层不替它写第二个默认值。
 */
export const languageProfile = async (
  inputFile: string,
  textKey?: string
): Promise<LanguageProfileResponse> => {
  const response = await api.post('/dataset/language-profile', {
    input_file: inputFile,
    text_key: textKey
  })
  return response.data
}

/**
 * `POST /api/dataset/evaluate`
 *
 * 两侧字段名（`generated_field` / `reference_field`）与 `metrics` 一律不传，
 * 让服务端按「请求参数 > 配置键 > 内置默认」那序回落 —— 页面只负责选文件。
 * `includeDetails` 默认 false：契约默认是 true，但逐条明细在这个页面没有渲染色，
 * 大数据集上把每条都搬回来只是把响应撑大，所以本层显式关掉。
 */
export const evaluateDataset = async (
  generatedFile: string,
  referenceFile: string,
  includeDetails: boolean = false
): Promise<EvaluateResponse> => {
  const response = await api.post('/dataset/evaluate', {
    generated_file: generatedFile,
    reference_file: referenceFile,
    include_details: includeDetails
  })
  return response.data
}

/**
 * `GET /api/dataset/vector-status`
 *
 * 只读，无入参。`enabled`（闸门生效值）与 `exists`（磁盘上有没有可读回的索引）
 * 必须分开渲染 —— 两真两假是四种不同事实，合成一个「向量库正常」会把「配了没跑过」
 * 报成正常。
 */
export const getVectorStatus = async (): Promise<VectorStatusResponse> => {
  const response = await api.get('/dataset/vector-status')
  return response.data
}

// ============ 内容检索 / 结构校验 / 配置推荐（A279 第二批接线，L226） ============

/**
 * `POST /api/dataset/search`
 *
 * 可选三键全部「不传即不写」：`method` / `limit` / `offset` 的权威默认在契约里
 * （`contains` / 100 / 0），本层不替它写第二套默认值。`undefined` 会被 JSON 序列化
 * 丢掉，所以传 `undefined` 与不传是同一件事——这正是我们要的回落。
 * `limit=0` 是**合法请求**（实测：`total_matches` 照给、`items` 为空），别在这里
 * 兜「0 就改成 100」，那会把用户明确要的「只要计数」偷换成一次全量取回。
 */
export const searchDataset = async (
  inputFile: string,
  query: string,
  opts: { method?: string; limit?: number; offset?: number } = {}
): Promise<SearchResponse> => {
  const response = await api.post('/dataset/search', {
    input_file: inputFile,
    query,
    method: opts.method,
    limit: opts.limit,
    offset: opts.offset
  })
  return response.data
}

/**
 * `POST /api/dataset/validate`
 *
 * `preset` 只有 `basic` / `strict` / `chat` 三档（实测自 `DatasetValidator.PRESET_RULES`），
 * 未知值由路由判 400；下拉的选项从页面给，本层不校验，也不在省略时写第二默认值。
 */
export const validateDataset = async (
  inputFile: string,
  preset?: string
): Promise<DatasetValidationResponse> => {
  const response = await api.post('/dataset/validate', {
    input_file: inputFile,
    preset
  })
  return response.data
}

/**
 * `POST /api/dataset/auto-config`
 *
 * 只读推荐，不写配置：它回的是「按这份数据的观测该用哪几档阈值」，
 * 真要落地仍走 `updateConfig`，两者分开才留得住「我看了建议、我决定不改」这一步。
 */
export const autoConfigRecommend = async (
  inputFile: string
): Promise<AutoConfigResponse> => {
  const response = await api.post('/dataset/auto-config', { input_file: inputFile })
  return response.data
}

// ============ 影响评估 / 语义检索（A279 第三批接线，L227） ============

/**
 * `POST /api/dataset/impact`
 *
 * 两个文件都是必填（契约里 `before_file` / `after_file` 都没有默认值）。
 * `text_field` 与 `min_scale_gain` 走「不传即不写」：权威默认 `instruction` / 0.0 住在
 * `ImpactRequest`，本层不写第二套。空基线由服务端判 **400**（四项增益的分母是基线，
 * 空基线算出的 0.0 会被读成「毫无效果」而不是「没有可比的东西」）——这条判决要在页面上
 * 出声，不在这里替它兜。
 */
export const assessImpact = async (
  beforeFile: string,
  afterFile: string,
  opts: { textField?: string; minScaleGain?: number } = {}
): Promise<ImpactResponse> => {
  const response = await api.post('/dataset/impact', {
    before_file: beforeFile,
    after_file: afterFile,
    text_field: opts.textField,
    min_scale_gain: opts.minScaleGain,
  })
  return response.data
}

/**
 * `POST /api/dataset/semantic-search`
 *
 * `top_k` 默认 5 与 `text_key` 默认 `instruction` 都住在 `SemanticSearchRequest`
 * （`api/routes/vector.py:34`），本层不写第二套。实测 `top_k=0` 是合法的「只要回显、
 * 不要命中」⇒ 空 `results` 与「库里没有东西」要分开读，所以 `total` 照原样渲染。
 */
export const semanticSearch = async (
  inputFile: string,
  query: string,
  opts: { topK?: number; textKey?: string } = {}
): Promise<SemanticSearchResponse> => {
  const response = await api.post('/dataset/semantic-search', {
    input_file: inputFile,
    query,
    top_k: opts.topK,
    text_key: opts.textKey,
  })
  return response.data
}

// ============ 字段级统计 / 特征普查 / 双集对比（A279 第四批接线，L228） ============

/**
 * `POST /api/dataset/stats`
 *
 * `fields` 走「不选即不写」：省略时服务端统计**数据集里全部字段**（实测 `a.json` 回
 * `instruction` 与 `output` 两格）。实测 `fields: []` 与省略等价，但页面的「没选」应当
 * 真的是没选，而不是替服务端写一个空清单当默认。
 * 文件不存在回 **404**（`detail` 是字符串「文件不存在」），字段名给错回 **200 + 全零幽灵字段**
 * ——后者不会出声，所以页面上的候选只能来自回执。
 */
export const datasetStats = async (
  inputFile: string,
  opts: { fields?: string[] } = {}
): Promise<StatisticsResponse> => {
  const response = await api.post('/dataset/stats', {
    input_file: inputFile,
    fields: opts.fields,
  })
  return response.data
}

/**
 * `POST /api/dataset/features`
 *
 * 请求模型 `DatasetFileRequest` 只有 `input_file` 一格（实测多送别的键被静默忽略），
 * 所以本层没有可省的默认可写；稀疏阈值 0.1 住在 `augmentor/feature_detect.py:59`，
 * 这只端点不开放调整。
 */
export const detectFeatures = async (
  inputFile: string
): Promise<FeaturesResponse> => {
  const response = await api.post('/dataset/features', { input_file: inputFile })
  return response.data
}

/**
 * `POST /api/dataset/compare`
 *
 * 两侧文件都必填。`name_a` / `name_b` 省略时由**路由**取文件名的 stem（实测 `a.json` 回 `a`），
 * 这一格默认不住在本层也不住在请求模型里，所以三个可省键一律「不传即不写」。
 * `key_fields` 的默认是 SDK 里的 `["instruction"]`（`augmentor/compare_enhanced.py:126`），
 * 不是请求模型的默认；页面不抄它，见 l228 的对账守卫。
 */
export const compareDatasets = async (
  datasetA: string,
  datasetB: string,
  opts: { nameA?: string; nameB?: string; keyFields?: string[] } = {}
): Promise<CompareResponse> => {
  const response = await api.post('/dataset/compare', {
    dataset_a: datasetA,
    dataset_b: datasetB,
    name_a: opts.nameA,
    name_b: opts.nameB,
    key_fields: opts.keyFields,
  })
  return response.data
}

// ============ 写盘型变换：convert / merge / split（A279 第五批接线，L229） ============

/**
 * `POST /api/dataset/convert`
 *
 * `output_file` 在契约里是**必填**（写哪儿由调用方点名，服务端不猜落名），所以本层不给默认。
 * `source_format` 省略时由 SDK 按扩展名推断并在回执里回**推断结果**（实测回 `json`）。
 * 坏值归宿（实测）：目标名写 `xls` ⇒ 400 且 detail 里逐条列出十二个合法目标；写一个不在清单上的
 * 名字 ⇒ 400「不支持的转换: json -> no_such」；`output_file` 含 `../` ⇒ 400「路径包含非法组件」，
 * 传绝对路径 ⇒ 403；输入不存在 ⇒ 404。这三档都是**字符串** detail。
 */
export const convertDataset = async (
  inputFile: string,
  outputFile: string,
  opts: { targetFormat?: string; sourceFormat?: string } = {}
): Promise<ConvertResponse> => {
  const response = await api.post('/dataset/convert', {
    input_file: inputFile,
    output_file: outputFile,
    target_format: opts.targetFormat,
    source_format: opts.sourceFormat || undefined,
  })
  return response.data
}

/**
 * `POST /api/dataset/merge`
 *
 * `inputs` 一个文件也是合法请求（实测 200，`input_files: 1`），空清单才 400「inputs 不能为空」。
 * `deduplicate` 的默认 `true` 住在请求模型里，本层不写第二套默认：省略即让服务端说话。
 * 注意这一档端点只把 `deduplicate` 传给 SDK，`MergeConfig` 的去重阈值等旋钮不可达（**A290**）。
 */
export const mergeDatasets = async (
  inputs: string[],
  outputFile: string,
  opts: { deduplicate?: boolean } = {}
): Promise<MergeResponse> => {
  const response = await api.post('/dataset/merge', {
    inputs,
    output_file: outputFile,
    deduplicate: opts.deduplicate,
  })
  return response.data
}

/**
 * `POST /api/dataset/split`
 *
 * 六个可省键一律「不传即不写」，默认全部住在请求模型（`0.8 / 0.1 / 0.1`、`stratify false`、
 * `stratify_key "instruction"`、`shuffle true`、`seed None`）。两条支路的判据**不同**（实测）：
 * 比例和的容差是默认支 `0.01`、分层支 `1e-6`；负数只有分层支拦（400「比例不能为负数」），
 * 默认支照收并回 200（**A289**）。`stratify_key` 给一个不存在的字段名**不报错**，它会塌成
 * 一个分组，分段数与用默认键时看起来一样。
 */
export const splitDataset = async (
  inputFile: string,
  outputDir: string,
  opts: {
    trainRatio?: number
    valRatio?: number
    testRatio?: number
    seed?: number
    stratify?: boolean
    stratifyKey?: string
    shuffle?: boolean
  } = {}
): Promise<SplitResponse> => {
  const response = await api.post('/dataset/split', {
    input_file: inputFile,
    output_dir: outputDir,
    train_ratio: opts.trainRatio,
    val_ratio: opts.valRatio,
    test_ratio: opts.testRatio,
    seed: opts.seed,
    stratify: opts.stratify,
    stratify_key: opts.stratifyKey,
    shuffle: opts.shuffle,
  })
  return response.data
}
