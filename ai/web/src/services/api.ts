import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000
})

// 数据管理
export const getDataFiles = async () => {
  const response = await api.get('/data/list')
  return response.data
}

export const loadData = async (filename: string, page: number, pageSize: number, search: string) => {
  const response = await api.get(`/data/load/${filename}`, {
    params: { page, page_size: pageSize, search }
  })
  return response.data
}

export const updateDataItem = async (filename: string, index: number, item: any) => {
  const response = await api.put(`/data/update/${filename}`, item, {
    params: { index }
  })
  return response.data
}

export const deleteDataItem = async (filename: string, index: number) => {
  const response = await api.delete(`/data/delete/${filename}`, {
    params: { index }
  })
  return response.data
}

export const uploadData = async (file: File) => {
  const formData = new FormData()
  formData.append('file', file)
  const response = await api.post('/data/upload', formData)
  return response.data
}

export const exportData = async (inputFile: string, outputDir: string, formats?: string[]) => {
  const response = await api.post('/data/export', {
    input_file: inputFile,
    output_dir: outputDir,
    formats
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
) => {
  const response = await api.post('/augment/start', {
    input_file: inputFile,
    output_file: outputFile,
    use_quality: useQuality,
    use_dedup: useDedup,
    use_checkpoint: useCheckpoint
  })
  return response.data
}

export const getProgress = async () => {
  const response = await api.get('/augment/progress')
  return response.data
}

export const getCheckpoints = async () => {
  const response = await api.get('/augment/checkpoints')
  return response.data
}

// 数据分析
export const analyzeData = async (filename: string) => {
  const response = await api.get(`/analyze/${filename}`)
  return response.data
}

export const visualizeData = async (filename: string) => {
  const response = await api.get(`/visualize/${filename}`)
  return response.data
}

// 版本管理
export const getVersions = async () => {
  const response = await api.get('/versions')
  return response.data
}
export const createVersion = async (filename: string, label: string, description: string) => {
  const response = await api.post(`/versions/create?filename=${filename}`, {
    label,
    description
  })
  return response.data
}

export const getVersion = async (versionId: string) => {
  const response = await api.get(`/versions/${versionId}`)
  return response.data
}

export const getVersionData = async (versionId: string) => {
  const response = await api.get(`/versions/${versionId}/data`)
  return response.data
}

export const diffVersions = async (version1: string, version2: string) => {
  const response = await api.post('/versions/diff', {
    version1,
    version2
  })
  return response.data
}

export const rollbackVersion = async (versionId: string) => {
  const response = await api.post(`/versions/${versionId}/rollback`)
  return response.data
}

export const deleteVersion = async (versionId: string) => {
  const response = await api.delete(`/versions/${versionId}`)
  return response.data
}

export const getVersionHistory = async (limit: number = 20) => {
  const response = await api.get('/versions/history', { params: { limit } })
  return response.data
}

// 质量评估
export const evaluateQuality = async (inputFile: string, threshold: number = 0.6) => {
  const response = await api.post('/quality/evaluate', {
    input_file: inputFile,
    threshold
  })
  return response.data
}

export const dedupData = async (inputFile: string, threshold: number = 0.9) => {
  const response = await api.post('/quality/dedup', {
    input_file: inputFile,
    threshold
  })
  return response.data
}

export const qualityReport = async (inputFile: string, threshold: number = 0.6) => {
  const response = await api.post('/quality/report', {
    input_file: inputFile,
    threshold
  })
  return response.data
}

export const cleanData = async (inputFile: string) => {
  const response = await api.post('/quality/clean', { input_file: inputFile })
  return response.data
}

export const annotateData = async (inputFile: string) => {
  const response = await api.post('/quality/annotate', { input_file: inputFile })
  return response.data
}

export const runBenchmark = async (inputFile: string, threshold: number = 0.6) => {
  const response = await api.post('/quality/benchmark', {
    input_file: inputFile,
    threshold
  })
  return response.data
}

// 导出
export const getExportFormats = async () => {
  const response = await api.get('/export/formats')
  return response.data
}

export const previewExport = async (inputFile: string, format: string, size: number = 5) => {
  const response = await api.post('/export/preview', {
    input_file: inputFile,
    format,
    size
  })
  return response.data
}

export const batchExport = async (
  datasets: Record<string, string>,
  outputDir: string,
  formats: string[]
) => {
  const response = await api.post('/export/batch', {
    datasets,
    output_dir: outputDir,
    formats
  })
  return response.data
}

// 多模态
export const getMultimodalFormats = async () => {
  const response = await api.get('/multimodal/formats')
  return response.data
}

export const processMultimodal = async (text: string, image?: string, audio?: string) => {
  const response = await api.post('/multimodal/process', { text, image, audio })
  return response.data
}

export const scanMultimodal = async (directory: string) => {
  const response = await api.post('/multimodal/scan', { directory })
  return response.data
}

// 配置管理
export const getConfig = async () => {
  const response = await api.get('/config')
  return response.data
}

export const updateConfig = async (config: any) => {
  const response = await api.post('/config', config)
  return response.data
}

export const getModels = async () => {
  const response = await api.get('/models')
  return response.data
}

// 健康检查
export const healthCheck = async () => {
  const response = await api.get('/health')
  return response.data
}
