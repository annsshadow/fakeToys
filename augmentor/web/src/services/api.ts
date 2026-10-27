import axios from 'axios'
import { z } from 'zod'

import {
  aggregateResponseSchema,
  annotateResponseSchema,
  auditReportSchema,
  autoConfigResponseSchema,
  backupCleanupResponseSchema,
  backupInfoResponseSchema,
  backupListResponseSchema,
  backupVerifyResponseSchema,
  batchExportResponseSchema,
  benchmarkResponseSchema,
  cleanResponseSchema,
  compareResponseSchema,
  configResponseSchema,
  configUpdateResponseSchema,
  configValidationResponseSchema,
  convertResponseSchema,
  datasetInfoResponseSchema,
  demoItemSchema,
  dependenciesResponseSchema,
  dependencyDatasetListResponseSchema,
  dependencyGraphResponseSchema,
  evaluateResponseSchema,
  exportFormatsResponseSchema,
  exportResponseSchema,
  featuresResponseSchema,
  healthGateResponseSchema,
  healthResponseSchema,
  impactResponseSchema,
  languageProfileResponseSchema,
  leakageResponseSchema,
  mergeResponseSchema,
  migrationResponseSchema,
  modelsResponseSchema,
  multimodalFormatsResponseSchema,
  multimodalRecordResponseSchema,
  multimodalScanResponseSchema,
  outlierResponseSchema,
  piiPatternsResponseSchema,
  previewResponseSchema,
  profilingResponseSchema,
  qualityReportResponseSchema,
  qualitySnapshotResponseSchema,
  ragResponseSchema,
  restoreBackupResponseSchema,
  sampleResponseSchema,
  sanitizeResponseSchema,
  searchResponseSchema,
  semanticSearchResponseSchema,
  splitResponseSchema,
  statisticsResponseSchema,
  statusResponseSchema,
  streamReportResponseSchema,
  successResponseSchema,
  testSuiteResponseSchema,
  translateResponseSchema,
  validationResponseSchema,
  vectorStatusResponseSchema,
  versionDetailResponseSchema,
  versionDiffResponseSchema,
  versionHistoryResponseSchema,
} from '../schemas/full-schemas'
import {
  analyzeResponseSchema,
  augmentProgressSchema,
  checkpointListResponseSchema,
  createVersionResponseSchema,
  dataFileListResponseSchema,
  dataPageResponseSchema,
  dedupResponseSchema,
  qualityEvaluationResponseSchema,
  startAugmentResponseSchema,
  uploadResponseSchema,
  versionDataResponseSchema,
  versionListResponseSchema,
  visualizeResponseSchema,
  zodErrorMessage,
} from '../schemas/main-schemas'
import type {
  AggregateResponse, AnalyzeResponse,
  AnnotateResponse,
  AppConfig,
  AuditResponse,
  AugmentProgress,
  AutoConfigResponse,
  BackupInfo,
  BackupListResponse, BackupVerifyResponse, BackupCleanupResponse,
  BatchExportResponse,
  BenchmarkResponse,
  CheckpointListResponse,
  CleanResponse,
  CompareResponse,
  ConfigValidationResponse,
  CreateVersionResponse,
  DataFileListResponse,
  DataItem,
  DataPageResponse,
  DatasetExportResponse,
  ConvertResponse,
  DatasetValidationResponse,
  DatasetRegistryEntry,
  DatasetRegistryListResponse,
  DedupResponse, DependencyDiagnostics,
  DependencyGraphResponse,
  EvaluateResponse,
  ExportFormatsResponse,
  ExportPreviewResponse,
  FeaturesResponse,
  HealthGateResponse,
  HealthResponse,
  ImpactResponse,
  LanguageProfileResponse,
  LeakageResponse,
  MergeResponse,
  MigrationResponse,
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
  QualityReportResponse, QualitySnapshotResponse, RagResponse, RestoreBackupResponse, SampleResponse,
  SplitResponse,
  SanitizeResponse,
  SearchResponse,
  StartAugmentResponse,
  StatisticsResponse,
  StatusResponse,
  StreamReportResponse,
  TestSuiteResponse, TranslateResponse,
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
 * 写操作鉴权：浏览器存一份 key，由请求拦截器随每个请求送出去
 *
 * 为什么这一层必须有：服务端 `api/deps.py` 的 `verify_api_key` 一旦看见环境变量
 * `AUGMENTOR_API_KEY` 就要求所有写操作带 `X-API-Key`，否则 401「缺少或无效的
 * X-API-Key」。实测（L233 探针）契约面 **75** 条操作里有 **24** 条挂着它，而这 24 条中
 * 有 **21** 条是前端已接线的形状 —— 也就是说开了鉴权以后，页面上「保存配置 / 上传 /
 * 导出 / 启动增强 / 建版本 / 回滚 / 改删单条 / 六只数据集变换 / 备份三只 / 依赖登记 /
 * 质量画像 / 批量导出」这些动作**一律点不动**（A302）。L231 当时只量了备份那一族的
 * 3 只，规模被低估了一个量级；依赖登记那只是同一道门禁的第二次回声（A312）。
 *
 * 存哪儿：`localStorage`，**明文**、只在这一台浏览器上、不进构建产物也不进仓库。
 * 这不是保密措施——同源脚本读得到它——它解决的是「服务端开了鉴权时页面还能用」。
 * 要更强的存放位置就得配后端会话/CSRF 那一整套，超出这一格的范围。
 *
 * 留空即**不送这个头**（`setApiKey('')` 走的是 `removeItem`，不存空串）：服务端没设
 * `AUGMENTOR_API_KEY` 时零配置照旧可用，多送一个空头没有意义。头名的大小写在真链路
 * 上无关（实测送 `X-Api-Key` 同样放行，HTTP 头名不区分大小写），这里仍与后端
 * `Header(alias=...)` 逐字一致，免得读代码时以为是两个不同的头。
 */
export const API_KEY_HEADER = 'X-API-Key'

/** `localStorage` 的键名 */
export const API_KEY_STORAGE_KEY = 'augmentor.api_key'

export const getApiKey = (): string => localStorage.getItem(API_KEY_STORAGE_KEY) ?? ''

export const setApiKey = (key: string): void => {
  const trimmed = key.trim()
  if (trimmed) {
    localStorage.setItem(API_KEY_STORAGE_KEY, trimmed)
  } else {
    localStorage.removeItem(API_KEY_STORAGE_KEY)
  }
}

api.interceptors.request.use(config => {
  const key = getApiKey()
  if (key) {
    config.headers.set(API_KEY_HEADER, key)
  }
  return config
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
 *
 * **401 单独加一句去处**：后端那句 detail 只说「缺少或无效的 X-API-Key」，它没说 key 该
 * 填在哪儿，而 21 个页面动作都会撞这一档（见 `API_KEY_HEADER` 那段）。加在 `apiErrorDetail`
 * 而不是各页面的 `catch` 里，是因为页面全部走这一个漏斗——一处改动，21 个动作同时被指路。
 * 两种成因要分开说：存着 key 却 401 是「那份与服务端不一致」，没存 key 是「还没填」，
 * 让用户去对一台自己没填过的输入框是错的方向。
 */
export const apiErrorDetail = (error: unknown, fallback: string): string => {
  const response = (error as { response?: { status?: number; data?: { detail?: unknown } } })
    ?.response
  const detail = response?.data?.detail
  let text = fallback
  if (typeof detail === 'string') {
    text = detail || fallback
  } else if (Array.isArray(detail)) {
    const msgs = detail
      .map((item) => (item as { msg?: unknown })?.msg)
      .filter((msg): msg is string => typeof msg === 'string' && msg.length > 0)
    if (msgs.length > 0) {
      text = msgs.join('；')
    }
  }
  if (response?.status !== 401) {
    return text
  }
  const cause = getApiKey() ? '浏览器里存着的那一份与服务端不一致' : '这台浏览器还没有存 key'
  return `${text}——${cause}：到「配置管理 → 写操作鉴权」填服务端 AUGMENTOR_API_KEY 的那一份`
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

// ============ 运行时契约校验（Z 批次，10-26-r4 §5）============

/**
 * 运行时契约校验的集中入口
 *
 * 背景：`types/api.ts` 的接口全是编译期声明，`axios` 的 `response.data` 是 `any`，
 * 后端响应形态漂移（少回键 / 类型错 / 键名变更）`tsc` 发现不了，页面只能靠
 * undefined 兜底或崩。本函数把主链路的 14 个函数接上 zod schema，在返回前跑
 * `safeParse`，契约漂移在**运行时**被抓住。
 *
 * 校验失败语义：抛 `Error`，消息是 `zodErrorMessage()` 生成的可定位中文文案
 * （接口名 + 第一个出问题的字段 + issue code），与 `apiErrorDetail` 的收口判据同族——
 * 不裸抛 zod 内部措辞。调用方（页面）的 `catch` 拿到的是 `error.message`，
 * 与既有的 `message.error(apiErrorDetail(err, ...))` 模式兼容（`apiErrorDetail`
 * 对非 HTTP 错误的 `error` 取 `fallback`，所以这里的 `Error` 会落到 `fallback`，
 * 而 `fallback` 由调用方传成「响应格式异常」类文案，见各页面的既有 catch）。
 *
 * 通过后返回**原始** `response.data`，不是 `parsed.data`：zod 默认会剥掉 schema
 * 未声明的键，返回克隆会让调用方看到的对象与后端实发的不一致（也会改变
 * `toBe` 之类的对象身份断言）。校验只负责「形状不对就抛」，不负责裁剪。
 *
 * 覆盖范围：`services/api.ts` 里被接上 schema 的全部 78 个端点函数（Z-FULL，10-27 计划 §4）。
 * `checkedResponse` 给「一只函数 = 一次请求」的 14 只；`checkedData` 给已有响应对象、
 * 只差校验那一步的其余 64 只。
 */
const checkedResponse = async <T>(
  name: string,
  schema: z.ZodType,
  action: () => Promise<AxiosResponseLike>
): Promise<T> => {
  const response = await action()
  const parsed = schema.safeParse(response.data)
  if (!parsed.success) {
    throw new Error(zodErrorMessage(name, parsed.error.issues))
  }
  return response.data as T
}

/** axios 响应的最小形状（本层不 import AxiosResponse 全类型，避免循环依赖） */
interface AxiosResponseLike {
  data: unknown
}

/**
 * 校验**已经取回**的响应体（Z-FULL）
 *
 * 与 `checkedResponse` 同一条口径（失败抛中文可定位 `Error`、通过返回原始对象而不是
 * `parsed.data`），差别只在调用形态：这 64 只函数的请求参数各不相同，硬套
 * 「把请求包成 callback」的写法要给每只再造一个闭包，收益只是少一行。
 *
 * 为什么类型参数不能省：`response.data` 是 `any`，`checkedData<T>` 的返回类型是**函数签名的
 * 唯一来源**。去掉 `<T>` 会让 64 只函数的返回类型退化成 `any`，`tsc` 再也看不见页面侧的误用。
 */
const checkedData = <T>(name: string, schema: z.ZodType, data: unknown): T => {
  const parsed = schema.safeParse(data)
  if (!parsed.success) {
    throw new Error(zodErrorMessage(name, parsed.error.issues))
  }
  return data as T
}

// 数据管理
export const getDataFiles = async (): Promise<DataFileListResponse> => {
  return checkedResponse<DataFileListResponse>(
    'getDataFiles',
    dataFileListResponseSchema,
    () => api.get('/data/list')
  )
}

/**
 * 备份源候选：走 `all_files=true` 档，候选口径与白名单同宽（A304 / L251）。
 *
 * 与 `getDataFiles()` 分成两只函数，是为了让其余 7 处数据文件选择器继续吃默认档
 * （只列 `train_data*.json`）；只有「源文件」这一格要的是「白名单里有什么就列什么」。
 * query 写在路径串里而不是 `params` 对象里，因为消费对账那把尺子（l224）把两种来源
 * 都算，而 `params` 对象要靠字符间隔绑回调用，同一路径两员调用时它分辨不了。
 */
export const getBackupSourceCandidates = async (): Promise<DataFileListResponse> => {
  return checkedResponse<DataFileListResponse>(
    'getBackupSourceCandidates',
    dataFileListResponseSchema,
    () => api.get('/data/list?all_files=true')
  )
}

/**
 * `GET /api/data/load/{filename}`
 *
 * 交回的是**过滤后切片**，所以 `items` 的行号从来不是写端点的坐标；要改或删页里某一行，
 * 用同序返回的 `indices`（盘上 0 基下标）发给 `PUT /api/data/update` 与 `DELETE`（A367 / L277）。
 * `page` 与 `page_size` 在门口判掉：小于 1 一律 **400**（改前 `page_size=-10` 回 200 交出
 * 「除了最后 10 条」的 15 条、`page=-1` 交出中间一段，都是静默错答）；页码超过末尾仍是 200 + 空页。
 */
export const loadData = async (
  filename: string,
  page: number,
  pageSize: number,
  search: string
): Promise<DataPageResponse> => {
  return checkedResponse<DataPageResponse>(
    'loadData',
    dataPageResponseSchema,
    () =>
      api.get(`/data/load/${filename}`, {
        params: { page, page_size: pageSize, search },
      })
  )
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
  return checkedData<MutationResponse>('updateDataItem', successResponseSchema, response.data)
}

export const deleteDataItem = async (
  filename: string,
  index: number
): Promise<MutationResponse> => {
  const response = await api.delete(`/data/delete/${filename}`, {
    params: { index },
  })
  return checkedData<MutationResponse>('deleteDataItem', successResponseSchema, response.data)
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
  return checkedResponse<UploadResponse>('uploadData', uploadResponseSchema, () =>
    api.post('/data/upload', formData, {
      timeout: BULK_REQUEST_TIMEOUT_MS,
    })
  )
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
  return checkedData<DatasetExportResponse>('exportData', exportResponseSchema, response.data)
}

// 数据增强
export const startAugmentation = async (
  inputFile: string,
  outputFile: string,
  useQuality: boolean,
  useDedup: boolean,
  useCheckpoint: boolean
): Promise<StartAugmentResponse> => {
  return checkedResponse<StartAugmentResponse>(
    'startAugmentation',
    startAugmentResponseSchema,
    () =>
      api.post('/augment/start', {
        input_file: inputFile,
        output_file: outputFile,
        use_quality: useQuality,
        use_dedup: useDedup,
        use_checkpoint: useCheckpoint,
      })
  )
}

export const getProgress = async (): Promise<AugmentProgress> => {
  return checkedResponse<AugmentProgress>(
    'getProgress',
    augmentProgressSchema,
    () => api.get('/augment/progress')
  )
}

export const getCheckpoints = async (): Promise<CheckpointListResponse> => {
  return checkedResponse<CheckpointListResponse>(
    'getCheckpoints',
    checkpointListResponseSchema,
    () => api.get('/augment/checkpoints')
  )
}

// 数据分析
export const analyzeData = async (filename: string): Promise<AnalyzeResponse> => {
  return checkedResponse<AnalyzeResponse>('analyzeData', analyzeResponseSchema, () =>
    api.get(`/analyze/${filename}`)
  )
}

export const visualizeData = async (filename: string): Promise<VisualizeResponse> => {
  return checkedResponse<VisualizeResponse>(
    'visualizeData',
    visualizeResponseSchema,
    () => api.get(`/visualize/${filename}`)
  )
}

// 版本管理
export const getVersions = async (): Promise<VersionListResponse> => {
  return checkedResponse<VersionListResponse>(
    'getVersions',
    versionListResponseSchema,
    () => api.get('/versions')
  )
}

export const createVersion = async (
  filename: string,
  label: string,
  description: string
): Promise<CreateVersionResponse> => {
  // L277：`filename` 改走 axios 的 `params`，不再拼进路径串。拼串时一个含 `&` / `=` / `#`
  // 或空格的文件名会把 query 拆开、截断甚至整个丢掉 `#` 之后的部分（`a&b.json` 实测变成
  // `filename=a` 外加一个野键 `b.json`），编码交给 axios，键名仍是契约里的 `filename`。
  // `params` 块紧贴调用（同 L231 为 `createBackup` 定下的口径），别让绑定窗口去够它。
  const body = { label, description }
  return checkedResponse<CreateVersionResponse>(
    'createVersion',
    createVersionResponseSchema,
    () => api.post('/versions/create', body, { params: { filename } })
  )
}

export const getVersion = async (versionId: string): Promise<VersionDetail> => {
  const response = await api.get(`/versions/${versionId}`)
  return checkedData<VersionDetail>('getVersion', versionDetailResponseSchema, response.data)
}

export const getVersionData = async (versionId: string): Promise<VersionDataResponse> => {
  return checkedResponse<VersionDataResponse>(
    'getVersionData',
    versionDataResponseSchema,
    () => api.get(`/versions/${versionId}/data`)
  )
}

export const diffVersions = async (
  version1: string,
  version2: string
): Promise<VersionDiffResponse> => {
  const response = await api.post('/versions/diff', {
    version1,
    version2,
  })
  return checkedData<VersionDiffResponse>('diffVersions', versionDiffResponseSchema, response.data)
}

export const rollbackVersion = async (versionId: string): Promise<MutationResponse> => {
  const response = await api.post(`/versions/${versionId}/rollback`)
  return checkedData<MutationResponse>('rollbackVersion', successResponseSchema, response.data)
}

export const deleteVersion = async (versionId: string): Promise<MutationResponse> => {
  const response = await api.delete(`/versions/${versionId}`)
  return checkedData<MutationResponse>('deleteVersion', successResponseSchema, response.data)
}

export const getVersionHistory = async (
  limit: number = 20
): Promise<VersionHistoryResponse> => {
  const response = await api.get('/versions/history', { params: { limit } })
  return checkedData<VersionHistoryResponse>('getVersionHistory', versionHistoryResponseSchema, response.data)
}

// 质量评估
export const evaluateQuality = async (
  inputFile: string,
  threshold: number = 0.6
): Promise<QualityEvaluationResponse> => {
  return checkedResponse<QualityEvaluationResponse>(
    'evaluateQuality',
    qualityEvaluationResponseSchema,
    () =>
      api.post('/quality/evaluate', {
        input_file: inputFile,
        threshold,
      })
  )
}

export const dedupData = async (
  inputFile: string,
  threshold: number = 0.9
): Promise<DedupResponse> => {
  return checkedResponse<DedupResponse>('dedupData', dedupResponseSchema, () =>
    api.post('/quality/dedup', {
      input_file: inputFile,
      threshold,
    })
  )
}

export const qualityReport = async (
  inputFile: string,
  threshold: number = 0.6
): Promise<QualityReportResponse> => {
  const response = await api.post('/quality/report', {
    input_file: inputFile,
    threshold,
  })
  return checkedData<QualityReportResponse>('qualityReport', qualityReportResponseSchema, response.data)
}

export const cleanData = async (inputFile: string): Promise<CleanResponse> => {
  const response = await api.post('/quality/clean', { input_file: inputFile })
  return checkedData<CleanResponse>('cleanData', cleanResponseSchema, response.data)
}

export const annotateData = async (inputFile: string): Promise<AnnotateResponse> => {
  const response = await api.post('/quality/annotate', { input_file: inputFile })
  return checkedData<AnnotateResponse>('annotateData', annotateResponseSchema, response.data)
}

export const runBenchmark = async (
  inputFile: string,
  threshold: number = 0.6
): Promise<BenchmarkResponse> => {
  const response = await api.post('/quality/benchmark', {
    input_file: inputFile,
    threshold,
  })
  return checkedData<BenchmarkResponse>('runBenchmark', benchmarkResponseSchema, response.data)
}

// 导出
export const getExportFormats = async (): Promise<ExportFormatsResponse> => {
  const response = await api.get('/export/formats')
  return checkedData<ExportFormatsResponse>('getExportFormats', exportFormatsResponseSchema, response.data)
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
  return checkedData<ExportPreviewResponse>('previewExport', previewResponseSchema, response.data)
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
  return checkedData<BatchExportResponse>('batchExport', batchExportResponseSchema, response.data)
}

// 多模态
export const getMultimodalFormats = async (): Promise<MultimodalFormatsResponse> => {
  const response = await api.get('/multimodal/formats')
  return checkedData<MultimodalFormatsResponse>('getMultimodalFormats', multimodalFormatsResponseSchema, response.data)
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
  return checkedData<MultimodalRecord>('processMultimodal', multimodalRecordResponseSchema, response.data)
}

export const scanMultimodal = async (
  directory: string
): Promise<MultimodalScanResponse> => {
  const response = await api.post('/multimodal/scan', { directory })
  return checkedData<MultimodalScanResponse>('scanMultimodal', multimodalScanResponseSchema, response.data)
}

// 配置管理
export const getConfig = async (): Promise<AppConfig> => {
  const response = await api.get('/config')
  return checkedData<AppConfig>('getConfig', configResponseSchema, response.data)
}

/**
 * 整份覆盖配置
 *
 * 同 `updateDataItem`：配置的完整结构由后端 `config.yaml` 定义，
 * 前端只做转发，因此用 `unknown` 而非 `any`。
 */
export const updateConfig = async (config: unknown): Promise<UpdateConfigResponse> => {
  const response = await api.post('/config', config)
  return checkedData<UpdateConfigResponse>('updateConfig', configUpdateResponseSchema, response.data)
}

export const getModels = async (): Promise<ModelsResponse> => {
  const response = await api.get('/models')
  return checkedData<ModelsResponse>('getModels', modelsResponseSchema, response.data)
}

// 健康检查
export const healthCheck = async (): Promise<HealthResponse> => {
  const response = await api.get('/health')
  return checkedData<HealthResponse>('healthCheck', healthResponseSchema, response.data)
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
  return checkedData<StatusResponse>('getServiceStatus', statusResponseSchema, response.data)
}

// ============ 内置演示数据 ============

/**
 * 拉取内置演示数据集（**裸数组**，不是 `{data: [...]}` 包装）
 *
 * 返回的条目结构与 `DataItem` 一致，可直接喂给 `uploadData` 落盘成数据集。
 */
export const getDemoData = async (): Promise<DataItem[]> => {
  const response = await api.get('/demo/data')
  return checkedData<DataItem[]>('getDemoData', z.array(demoItemSchema), response.data)
}

// ============ 隐私脱敏 ============

export const getPiiPatterns = async (): Promise<PiiPatternsResponse> => {
  const response = await api.get('/privacy/patterns')
  return checkedData<PiiPatternsResponse>('getPiiPatterns', piiPatternsResponseSchema, response.data)
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
  return checkedData<SanitizeResponse>('sanitizeData', sanitizeResponseSchema, response.data)
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
  return checkedData<LeakageResponse>('checkLeakage', leakageResponseSchema, response.data)
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
  return checkedData<AuditResponse>('auditDataset', auditReportSchema, response.data)
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
  return checkedData<OutliersResponse>('detectOutliers', outlierResponseSchema, response.data)
}

export const profileDataset = async (inputFile: string): Promise<ProfilingResponse> => {
  const response = await api.post('/quality/profiling', { input_file: inputFile })
  return checkedData<ProfilingResponse>('profileDataset', profilingResponseSchema, response.data)
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
  return checkedData<LanguageProfileResponse>('languageProfile', languageProfileResponseSchema, response.data)
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
  return checkedData<EvaluateResponse>('evaluateDataset', evaluateResponseSchema, response.data)
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
  return checkedData<VectorStatusResponse>('getVectorStatus', vectorStatusResponseSchema, response.data)
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
  return checkedData<SearchResponse>('searchDataset', searchResponseSchema, response.data)
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
  return checkedData<DatasetValidationResponse>('validateDataset', validationResponseSchema, response.data)
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
  return checkedData<AutoConfigResponse>('autoConfigRecommend', autoConfigResponseSchema, response.data)
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
  return checkedData<ImpactResponse>('assessImpact', impactResponseSchema, response.data)
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
  return checkedData<SemanticSearchResponse>('semanticSearch', semanticSearchResponseSchema, response.data)
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
  return checkedData<StatisticsResponse>('datasetStats', statisticsResponseSchema, response.data)
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
  return checkedData<FeaturesResponse>('detectFeatures', featuresResponseSchema, response.data)
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
  return checkedData<CompareResponse>('compareDatasets', compareResponseSchema, response.data)
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
  return checkedData<ConvertResponse>('convertDataset', convertResponseSchema, response.data)
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
  return checkedData<MergeResponse>('mergeDatasets', mergeResponseSchema, response.data)
}

/**
 * `POST /api/dataset/split`
 *
 * 六个可省键一律「不传即不写」，默认全部住在请求模型（`0.8 / 0.1 / 0.1`、`stratify false`、
 * `stratify_key "instruction"`、`shuffle true`、`seed None`）。比例判据只住一处（**A324 / L256**
 * 收掉 A289 那格分歧）：两条支路共引 `validation.require_ratio_list`——±0.01 含端点、逐项 0..1、
 * 长度 3，同一个 `ratios` 两侧同码同答。`stratify_key` 给一个不存在的字段名**不报错**，它会塌成
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
  return checkedData<SplitResponse>('splitDataset', splitResponseSchema, response.data)
}

// ============ 采样 / 聚合 / RAG 转换（A279 第六批接线，L230） ============

/**
 * `POST /api/dataset/sample`
 *
 * 四个可省键一律「不传即不写」，默认全部住在请求模型（`method "random"`、`size None`、
 * `ratio None`、`seed None`、`stratify_key "instruction"`）。`size` 与 `ratio` 同给时
 * **size 赢**（实测 `3` 与 `0.9` 同给 ⇒ 出 3 条），所以本层不做二选一，两格都照发。
 *
 * 越界的归宿**三种 method 一种结局**（实测，20 条语料）：`size=-1` 三支一致 400「size 必须是不小于 0
 * 的整数，当前是 -1」；`ratio=2` 与 `ratio=-0.5` 也一致 400「ratio 必须是 0.0 到 1.0 之间的比例，
 * 当前是 …」，且 400 之前不落任何产物。**A292 的范围维在 L277 收口**——改前只有 `random` 支回 400，
 * 且那句是 CPython 的英文原话「Sample larger than population or is negative」，`systematic` /
 * `stratified` 两支照收：`ratio=2` 交全量 20 条，`ratio=-0.5` 走负数切片交出**前 10 条**（分层支交 0 条）。
 */
export const sampleDataset = async (
  inputFile: string,
  outputFile: string,
  opts: {
    method?: string
    size?: number
    ratio?: number
    seed?: number
    stratifyKey?: string
  } = {}
): Promise<SampleResponse> => {
  const response = await api.post('/dataset/sample', {
    input_file: inputFile,
    output_file: outputFile,
    method: opts.method,
    size: opts.size,
    ratio: opts.ratio,
    seed: opts.seed,
    stratify_key: opts.stratifyKey,
  })
  return checkedData<SampleResponse>('sampleDataset', sampleResponseSchema, response.data)
}

/**
 * `POST /api/dataset/aggregate`
 *
 * `datasets` 是「来源名 → 文件」的映射，**名字由调用方起**（契约里没有默认，也不会替你取
 * 文件名的 stem）；空映射 400「datasets 不能为空」。`target_size` 只对 `weighted` 生效，
 * 配 union / intersection / consistent 时**完全空转**（实测 `target_size=1` 配 union 仍交 20 条，
 * 见 **A294**）。SDK 的 `key_fields` / `value_fields` / `random_state` 在这只端点上一个都不可设
 * （路由是 `DataAggregator()` 无参构造），所以判重恒认 `instruction`、一致性恒认 `output`，
 * 且 `weighted` 同参两次落盘**不相等**（**A293**）。
 */
export const aggregateDatasets = async (
  datasets: Record<string, string>,
  outputFile: string,
  opts: { strategy?: string; targetSize?: number } = {}
): Promise<AggregateResponse> => {
  const response = await api.post('/dataset/aggregate', {
    datasets,
    output_file: outputFile,
    strategy: opts.strategy,
    target_size: opts.targetSize,
  })
  return checkedData<AggregateResponse>('aggregateDatasets', aggregateResponseSchema, response.data)
}

/**
 * `POST /api/dataset/rag`
 *
 * `format` 省略时那一格读 `config.rag.default_format`（实测 `llamaindex`）；送空串与省略**等价**
 * （实测两者都回 `llamaindex`），所以本层把页面「服务端默认」那档的空串丢成 `undefined`，线上
 * 只留一种形状。合法值三项来自 `augmentor/rag.py` 的 `SUPPORTED_FORMATS`，写错 ⇒ 400 且 detail
 * 里把三项列全。切块的两个旋钮 `chunk_size` / `chunk_overlap` **只从服务端配置读**，请求里多送
 * 这两格会被静默忽略（实测 200，产物与不送时相同）。
 */
export const convertToRag = async (
  inputFile: string,
  outputFile: string,
  opts: { format?: string } = {}
): Promise<RagResponse> => {
  const response = await api.post('/dataset/rag', {
    input_file: inputFile,
    output_file: outputFile,
    format: opts.format || undefined,
  })
  return checkedData<RagResponse>('convertToRag', ragResponseSchema, response.data)
}

// ============ 系统 · 数据备份（A279 第七批接线，L231） ============

/**
 * `GET /api/system/backups`
 *
 * `backup_dir` 省略时落点是**白名单首个根目录下的 `.backups`**（出厂即 `data/.backups`），
 * 显式送空串是 **400**「目录路径 不能为空」、`..` 是 **400**「路径包含非法组件」、白名单外的
 * 绝对路径是 **403**——所以本层把页面上的空串丢成 `undefined`，这一处哨兵**是躲判决的**
 * （与 `convertToRag` 那处只做形状卫生的不同）。CLI 的 `--backup-dir` 默认值是 `.backups`
 * （相对工作目录），与这里的缺省**不是同一个目录**（账本 A301）。
 */
export const listBackups = async (backupDir?: string): Promise<BackupListResponse> => {
  const response = await api.get('/system/backups', {
    params: { backup_dir: backupDir || undefined },
  })
  return checkedData<BackupListResponse>('listBackups', backupListResponseSchema, response.data)
}

/**
 * `POST /api/system/backups`
 *
 * `name` 就是回执里的 `backup_id`，省略时服务端按秒级时间戳取 `backup_<YYYYmmdd_HHMMSS>`；
 * 送空串与省略**等价**（`name or …` 那一步把空串读成没给，实测 200 且回时间戳 id）。
 * 名字必须是**一个 URL 路径段**：`..`、`/`、反斜杠、`#`、`?`、控制字符、尾随点或空白、超 255 字符一律
 * **400**「备份名称不能含路径分隔符，否则寻不了址…」。**A366 / L277** 起的口径——改前带 `/` 的名字
 * 放行（`.backups/sub/deep.json` 建得出、清单也列得出），但 `{backup_id}` 只吃一段 ⇒ 恢复/删除恒 404。
 */
export const createBackup = async (
  inputFile: string,
  opts: { name?: string; backupDir?: string } = {}
): Promise<BackupInfo> => {
  // 请求体提到上面一行：`params` 块与所属调用的间隔必须在消费对账那把尺子的窗口内
  // （`test_api_endpoint_consumption_l224.py` 的 `PARAMS_MAX_GAP = 60`），否则这一条调用的
  // `backup_dir` 就绑不上、query 那一维对它失明（改前实测：这一条是四条里唯一没绑上的）
  const body = { input_file: inputFile, name: opts.name || undefined }
  const response = await api.post('/system/backups', body, {
    params: { backup_dir: opts.backupDir || undefined },
  })
  return checkedData<BackupInfo>('createBackup', backupInfoResponseSchema, response.data)
}

/**
 * `POST /api/system/backups/{backup_id}/restore`
 *
 * 目标文件已存在时**整份覆盖**（实测），备份 id 不存在是 **404**「备份不存在」。
 * 回执的落点键叫 `output_path` 而不是同族其余五只的 `output_file`。
 * id 走 `encodeURIComponent`：**存量**名字里可能有 `#`（A366 之前建得出来），裸插值会被浏览器
 * 当 fragment 丢掉 ⇒ 服务端只收到前半截，restore 拿 405、delete 拿 404。
 */
export const restoreBackup = async (
  backupId: string,
  outputFile: string,
  backupDir?: string
): Promise<RestoreBackupResponse> => {
  const response = await api.post(
    `/system/backups/${encodeURIComponent(backupId)}/restore`,
    { output_file: outputFile },
    { params: { backup_dir: backupDir || undefined } }
  )
  return checkedData<RestoreBackupResponse>('restoreBackup', restoreBackupResponseSchema, response.data)
}

/**
 * `DELETE /api/system/backups/{backup_id}`
 *
 * 删掉的是**索引里所有同 id 的记录**外加那一份文件（同名多行只剩 L250 之前写出的存量索引，
 * 新建的同名只留一行）。id 不存在是 **404**「备份不存在」，与恢复那一支同码同文案。
 */
export const deleteBackup = async (
  backupId: string,
  backupDir?: string
): Promise<MutationResponse> => {
  const response = await api.delete(`/system/backups/${encodeURIComponent(backupId)}`, {
    params: { backup_dir: backupDir || undefined },
  })
  return checkedData<MutationResponse>('deleteBackup', successResponseSchema, response.data)
}

/**
 * `GET /api/system/backups/verify`（A299 / A300，L243 接线）
 *
 * 逐只重算备份文件的 sha256 并与索引里那一格比对，是恢复那道 **409** 闸的主动查版本。
 * 三个实测口径：目录不存在 ⇒ **200** `{"results": []}`（且**不会**把目录建出来，因为这是
 * 只读端点）；`backup_id` 给一个索引里没有的名字 ⇒ 同样是空列表，**不是 404**；这一只端点
 * 自 L244 起有 CLI 动作（`backup --action verify`，查到损坏时退出码 1，A329 已收）。
 */
export const verifyBackups = async (
  opts: { backupId?: string; backupDir?: string } = {}
): Promise<BackupVerifyResponse> => {
  const response = await api.get('/system/backups/verify', {
    params: { backup_id: opts.backupId || undefined, backup_dir: opts.backupDir || undefined },
  })
  return checkedData<BackupVerifyResponse>('verifyBackups', backupVerifyResponseSchema, response.data)
}

/**
 * `POST /api/system/backups/cleanup`（A300，L243 接线）
 *
 * 破坏性写操作，与创建 / 恢复 / 删除同档挂鉴权。`max_backups` 是**保留份数**：库里
 * `require_count` 把 0 与负数一律判 **400**（「保留 0 份」与手滑想打 10 无从分辨，而负数在
 * 按时间戳排序的删除算术里等于把整目录删光）。缺省 10 与 SDK 形参默认同值，页面把它做成可
 * 输入框，因为「清理」的后果不可逆。
 */
export const cleanupBackups = async (
  opts: { maxBackups?: number; backupDir?: string } = {}
): Promise<BackupCleanupResponse> => {
  const body = { max_backups: opts.maxBackups ?? 10 }
  const response = await api.post('/system/backups/cleanup', body, {
    params: { backup_dir: opts.backupDir || undefined },
  })
  return checkedData<BackupCleanupResponse>('cleanupBackups', backupCleanupResponseSchema, response.data)
}

// ============ 系统 · 数据集血缘（A279 第八批接线，L232） ============

/**
 * `GET /api/system/dependency/datasets`
 *
 * `registry_path` 省略时落点是**白名单首个根目录下的 `.dependency_registry`**（出厂即
 * `data/.dependency_registry`），显式送空串是 **400**「目录路径 不能为空」、`..` 是 **400**
 * 「路径包含非法组件」、白名单外的绝对路径是 **403**——所以本层把页面上的空串丢成
 * `undefined`，这一处哨兵**是躲判决的**（与 `listBackups` 同款）。CLI 的 `--registry-path`
 * 默认值是 `.dependency_registry`（相对工作目录），与这里的缺省**不是同一个目录**（A308）。
 * 另有一条口径：注册表目录不存在时这两支只读端点**不会**把它建出来（A307，L249 收口）。
 */
export const listDependencyDatasets = async (
  registryPath?: string
): Promise<DatasetRegistryListResponse> => {
  const response = await api.get('/system/dependency/datasets', {
    params: { registry_path: registryPath || undefined },
  })
  return checkedData<DatasetRegistryListResponse>('listDependencyDatasets', dependencyDatasetListResponseSchema, response.data)
}

/**
 * `POST /api/system/dependency/datasets`
 *
 * 必填两格 `name` 与 `input_file`；缺 `input_file` 是 **422**（FastAPI 英文 `Field required`），
 * 而其余坏参数走中文 400/403/404 —— 页面那句错误提示因此不能只认一种错误体（A311）。
 * `input_file` 不存在是 **404**「文件不存在」，白名单外是 **403**，带 `..` 是 **400**，
 * 空串是 **400**「文件路径 不能为空」，顶层不是 JSON 数组是 **400**「数据文件的顶层必须是
 * JSON 数组，当前是对象」。`dataset_id = md5(name)[:8]`，所以**同名再登记是覆盖**（不留
 * 重复，但 `created_at` 被刷新，A306）。这一支挂鉴权门禁（A312）：服务端设了
 * `AUGMENTOR_API_KEY` 而无那个头就是 **401**。L233 起本层的请求拦截器会把「配置管理」页
 * 存下的那份 key 随每个请求送出去（A302 / A312 已收），头名的带引号字面量全前端只有
 * `API_KEY_HEADER` 那一处 —— 多一处就等于多一条各自为政的通道，L233 那条判据会点名。
 */
export const registerDependencyDataset = async (
  name: string,
  inputFile: string,
  opts: { description?: string; tags?: string[]; registryPath?: string } = {}
): Promise<DatasetRegistryEntry> => {
  // 请求体提到调用之前：`params` 块与所属调用的间隔必须落在消费对账那把尺子的窗口内
  // （`test_api_endpoint_consumption_l224.py` 的 `PARAMS_MAX_GAP = 60`），否则 `registry_path`
  // 绑不上、query 那一维对这一条失明——`createBackup` 已经踩过一次（L231）
  const body = {
    name,
    input_file: inputFile,
    description: opts.description || undefined,
    tags: opts.tags && opts.tags.length > 0 ? opts.tags : undefined,
  }
  const response = await api.post('/system/dependency/datasets', body, {
    params: { registry_path: opts.registryPath || undefined },
  })
  return checkedData<DatasetRegistryEntry>('registerDependencyDataset', datasetInfoResponseSchema, response.data)
}

/**
 * `GET /api/system/dependency/graph`
 *
 * 三键恒在：`nodes` / `edges` / `issues`。`registry_path` 的档位判决与列表那一支逐字相同。
 * `issues` 只报**悬空端点**（`missing_source` / `missing_target`），**不查环**：实测 A→B 与
 * B→A 成环时 `issues` 仍为空，所以这一格不能渲染成「校验通过」（A310）。`edges` 对纯前端
 * 用户恒为空——三条端点里没有建边入口，CLI 的 `dependency --action` 也只有
 * register/list/graph/validate，边只能由 SDK 的 `add_dependency` 建立（A309）。
 */
export const getDependencyGraph = async (
  registryPath?: string
): Promise<DependencyGraphResponse> => {
  const response = await api.get('/system/dependency/graph', {
    params: { registry_path: registryPath || undefined },
  })
  return checkedData<DependencyGraphResponse>('getDependencyGraph', dependencyGraphResponseSchema, response.data)
}

// ============ 系统 · 监控 / 自测 / 迁移 / 流式 / 配置校验（A279 第九批接线，L234） ============

/**
 * `POST /api/system/monitor`
 *
 * 只读不写盘，但**不是 GET**（同族四只都是 POST 带 body）。`input_file` 必填：缺是 **422**
 * 英文 `Field required`，不存在是 **404**「文件不存在」，白名单外 **403**，带 `..` **400**
 * 「路径包含非法组件」——与同族其余写盘端点逐字同一套判决。多送未知键被静默忽略（实测 200）。
 */
export const runQualityMonitor = async (inputFile: string): Promise<QualitySnapshotResponse> => {
  const response = await api.post('/system/monitor', { input_file: inputFile })
  return checkedData<QualitySnapshotResponse>('runQualityMonitor', qualitySnapshotResponseSchema, response.data)
}

/**
 * `POST /api/system/auto-test`
 *
 * `suite` 只认 `default`（来自 `AUTO_TEST_SUITES`）：拼错是 **400**「未知的测试套件: …
 * （当前只支持 default；自定义套件需要先由调用方注册，本端点暂无注册入口）」，**空串同样是
 * 400**（服务端判的是 `suite is not None`，空串算给了一个不认识的名字），而 `null` 与省略
 * 才走默认 ⇒ 本层把页面「服务端默认」那档的空串丢成 `undefined`，这一处哨兵**是躲判决的**
 * （与 `listBackups` / `listDependencyDatasets` 同款，不是形状卫生）。
 */
export const runDatasetAutoTest = async (
  inputFile: string,
  opts: { suite?: string } = {}
): Promise<TestSuiteResponse> => {
  const response = await api.post('/system/auto-test', {
    input_file: inputFile,
    suite: opts.suite || undefined,
  })
  return checkedData<TestSuiteResponse>('runDatasetAutoTest', testSuiteResponseSchema, response.data)
}

/**
 * `POST /api/system/migrate`
 *
 * 挂鉴权门禁（L233 起请求拦截器会把「配置管理 → 写操作鉴权」存下的那份 key 送出去）。
 * `rules` 省略或**空数组**都等于「三条内置规则全用」（`rename_instruction` /
 * `rename_output` / `flatten_conversations`），不是「不迁移」⇒ 本层把空数组丢成 `undefined`，
 * 免得页面上那一档被读成 no-op。未知规则名是 **400**「未知的迁移规则: …（可选 …）」，
 * 且**先于文件判决**（规则名写错时连 `input_file` 存不存在都不看）。
 * 这是**破坏性**操作：字段被改名（`instruction` → `question`、`output` → `answer`），
 * `input_file` 与 `output_file` 相同时就地改写，服务端放行（实测 200）。
 */
export const migrateDataset = async (
  inputFile: string,
  outputFile: string,
  opts: { rules?: string[] } = {}
): Promise<MigrationResponse> => {
  const response = await api.post('/system/migrate', {
    input_file: inputFile,
    output_file: outputFile,
    rules: opts.rules && opts.rules.length > 0 ? opts.rules : undefined,
  })
  return checkedData<MigrationResponse>('migrateDataset', migrationResponseSchema, response.data)
}

/**
 * `POST /api/system/stream`
 *
 * 挂鉴权门禁。四档 `operation` 在同一份 5 条夹具上的实测写出条数：`none` 5、`quality` 2、
 * `dedup` 3、`clean` 5（clean **一条不丢**，改写落在内容里）；未知值与**空串**都是 **400**
 * 「不支持的 operation: …（可选 quality / dedup / clean / none）」⇒ 空串同样丢成 `undefined`。
 * `output_format` 只认 json / jsonl（parquet 是 **400**「format 必须是 json / jsonl 之一，
 * 当前是 'parquet'」），`chunk_size` 0 / -1 是 **400**「chunk_size 必须是不小于 1 的整数，当前是 0」。
 * 这一支**不调模型**（与 `/api/augment/start` 的区别，路由 docstring 里写着），所以同步返回。
 */
export const streamProcess = async (
  inputFile: string,
  outputFile: string,
  opts: { operation?: string; chunkSize?: number; outputFormat?: string } = {}
): Promise<StreamReportResponse> => {
  const response = await api.post('/system/stream', {
    input_file: inputFile,
    output_file: outputFile,
    operation: opts.operation || undefined,
    chunk_size: opts.chunkSize && opts.chunkSize > 0 ? opts.chunkSize : undefined,
    output_format: opts.outputFormat || undefined,
  })
  return checkedData<StreamReportResponse>('streamProcess', streamReportResponseSchema, response.data)
}

/**
 * `POST /api/system/validate-config`
 *
 * **校验不通过也是 200**，判决在 body 的 `is_valid` 上 ⇒ 页面不能靠 `catch` 判失败。
 * `path` 省略时校验的是**服务自己**正在用的那份（`AUGMENTOR_CONFIG_PATH`，缺省 `config.yaml`）；
 * 那份不存在时同样 200 + `is_valid: false` + 「文件不存在: …」。显式 path 走数据白名单：
 * 白名单外 **403**、不存在 **404**，而**空串是 400**「文件路径 不能为空」⇒ 这一处哨兵
 * 也是躲判决的，不是形状卫生。
 */
export const validateConfigFile = async (
  opts: { path?: string } = {}
): Promise<ConfigValidationResponse> => {
  const response = await api.post('/system/validate-config', {
    path: opts.path || undefined,
  })
  return checkedData<ConfigValidationResponse>('validateConfigFile', configValidationResponseSchema, response.data)
}

// ============ 质量 · 健康门禁（A279 第十批接线，L235） ============

/**
 * `POST /api/quality/health-gate`
 *
 * 不挂鉴权门禁、只读不写盘：对数据集打健康分（`DatasetHealthScore`），再用默认三条门禁
 * 规则判定能否放行（pass_rate=error / duplicate_rate=error / completeness=warning）。
 *
 * 下面这些哨兵三种动机各不相同，都是量出来的，不是形状卫生：
 *
 * * `text_field` 的 `|| undefined` 改前是**躲判决的**（A321：空串不报错、`duplicate_rate`
 *   顶成 1.0）；L238 起后端判空串 / 纯空白 / 幽灵键一律 400 ⇒ 这一格只剩「不填走默认」。
 * * 三档阈值与 `block_on_warning` 的 `|| undefined` 是**必须的**：它们不是 `Optional`，
 *   送 `null` 实测 **422**（英文 `float_type` / `bool_type`）；而 `weights` 与 `pass_rate`
 *   是 `Optional`，送 `null` 是 200。axios 的 `JSON.stringify` 丢 `undefined` 但留 `null`。
 * * **`pass_rate` 刻意不用 `|| undefined`**：`0` 是这一格的合法值（区间 [0,1] 闭），而
 *   `/api/quality/evaluate` 在简化评分下**真的会回 0.0**。用 `||` 会把「算出来是 0」静默
 *   变成「没算这一项」（进 `skipped_rules`），正是端点 docstring 里写着要避免的那件事：
 *   `GateRule.evaluate()` 对缺失的指标键返回 False，于是 FAILED 冒充成「不及格」。
 *
 * `weights` 不做长度/和值过滤：那两句 400（「权重必须包含 4 个元素」「权重之和必须为 1.0」）
 * 是该让用户看见的真判决。三档阈值**没有范围闸**（-1.0 / 5.0 / 99.0 实测全 200，判决随阈值
 * 单调变化），所以页面要自己把口径写明白。这里不写第二套默认值——默认住在后端契约上，
 * 抄一份进来就有两份真相。
 */
export const healthGate = async (
  inputFile: string,
  opts: {
    textField?: string
    weights?: number[]
    passRate?: number
    passRateMin?: number
    duplicateRateMax?: number
    completenessMin?: number
    blockOnWarning?: boolean
  } = {}
): Promise<HealthGateResponse> => {
  const response = await api.post('/quality/health-gate', {
    input_file: inputFile,
    text_field: opts.textField || undefined,
    weights: opts.weights,
    pass_rate: opts.passRate,
    pass_rate_min: opts.passRateMin,
    duplicate_rate_max: opts.duplicateRateMax,
    completeness_min: opts.completenessMin,
    block_on_warning: opts.blockOnWarning,
  })
  return checkedData<HealthGateResponse>('healthGate', healthGateResponseSchema, response.data)
}

/**
 * `GET /api/system/dependencies`
 *
 * 与 `/api/status` 里那份 `dependencies` 是**同一份数据的两面**（实测两条端点回的诊断
 * 逐字节相等），所以这一只的价值不在数据，在**取法**：它不碰管道——把 `get_pipeline()`
 * 整个换成抛异常的替身，它照样 200（实测 `available_count` 仍是 3），而 status 那条虽然
 * 也 catch 住了异常、却必须先跑完一次综合状态（`model_default` 退成 `unknown`）。
 *
 * 页面用它把「依赖诊断」从 `status &&` 那道整体门里摘出来单独刷新：从前 status 这一个
 * 请求失败，依赖明细与降级功能两块会**一起**消失，而这两块恰好是「为什么某项功能不可用」
 * 的第一手读数。挂鉴权与否实测为**否**（只读、不打模型、不落盘，与 `language-profile`
 * 同档），所以这一只不需要 key 通道。
 */
export const systemDependencies = async (): Promise<DependencyDiagnostics> => {
  const response = await api.get('/system/dependencies')
  return checkedData<DependencyDiagnostics>('systemDependencies', dependenciesResponseSchema, response.data)
}

/**
 * `POST /api/dataset/translate`
 *
 * **这是按 token 计费的那条路径**（逐条调 `model_backend.generate`，挂 `X-API-Key`，
 * 且受 `multilingual.enabled` 闸控制——出厂 `false` ⇒ 400，闸在读盘之前）。所以本层
 * 的每一个哨兵都要回答同一句话：它有没有替用户省下一笔不该花的钱，或者有没有把一次
 * 真判决藏起来。四格逐个是量出来的：
 *
 * * `target_lang` 的 `|| undefined` **不藏判决**：空串与省略实测**等价**（两者都回
 *   `target_lang='en'`，即落 `multilingual.default_target_lang`），而清单外的值照样
 *   400 且 detail 把 `supported_langs` 列全、模型调用 **0** 次——语言闸在花钱之前。
 * * `output_file` 的 `|| undefined` 同理：空串与省略实测都回 `output_file=null`
 *   （后端那格是 falsy 判断），而越界落点是 **403 且模型调用 0 次**。
 * * `text_key` **刻意不做归一**：这一格自 L238 起**有判决了**——空串与数据集里没有的键一律
 *   **400**，且判在模型与写盘之前（改前实测 200、模型调用 **0** 次、`failed=0`，产物里凭空
 *   多出 `''` / `no_such_field` 两键，那是 **A322** 的案面）⇒ 页面那道必填闸只剩省一个来回。
 * * `keep_original` **直传**：它不是 `Optional`，送 `null` 实测 **422**（`bool_type`），
 *   而 `false` 是合法值 ⇒ 不能用 `||`（会把「不要原文」静默变成「要原文」）。
 *
 * 失败回执四键（`failed` / `failed_indices` / `errors` / `errors_truncated`）必须与
 * `items` 一起读：没翻成的条目**回退原文**，形状与成功时逐格相同。落盘那份也一样——
 * `output_file` 只是让这次花钱留下东西，它不会把失败条目挑出来。
 */
export const translateDataset = async (
  inputFile: string,
  opts: {
    outputFile?: string
    targetLang?: string
    textKey?: string
    keepOriginal?: boolean
  } = {}
): Promise<TranslateResponse> => {
  const response = await api.post('/dataset/translate', {
    input_file: inputFile,
    output_file: opts.outputFile || undefined,
    target_lang: opts.targetLang || undefined,
    text_key: opts.textKey,
    keep_original: opts.keepOriginal,
  })
  return checkedData<TranslateResponse>('translateDataset', translateResponseSchema, response.data)
}
