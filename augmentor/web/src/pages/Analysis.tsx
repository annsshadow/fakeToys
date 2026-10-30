import { useState, useEffect, useCallback } from 'react'
import { Alert, Button, Card, Col, Input, InputNumber, Row, Select, Space, Statistic, Table, Tag, message } from 'antd'
import ReactEChartsCore from 'echarts-for-react/lib/core'
import echarts from '../lib/echarts'
import {
  analyzeData,
  apiErrorDetail,
  assessImpact,
  cleanData,
  compareDatasets,
  datasetStats,
  detectFeatures,
  evaluateDataset,
  getDataFiles,
  languageProfile,
  runBenchmark,
  searchDataset,
  semanticSearch,
  visualizeData,
} from '../services/api'
import type {
  AnalyzeResponse,
  BenchmarkResponse,
  CleanResponse,
  CompareResponse,
  EvaluateResponse,
  FeaturesResponse,
  ImpactResponse,
  ImpactSnapshot,
  LanguageProfileResponse,
  QualityScoreFace,
  SearchResponse,
  SemanticSearchResponse,
  StatisticsResponse,
} from '../types/api'

/**
 * 检索方法五档（与 `augmentor/search_enhanced.py` 的 `SEARCH_METHODS` 逐项相等）
 *
 * 这份字面量与后端常数由 `tests/unit/test_wired_dropdown_choices_l226.py` 双向守卫，
 * 口径与 `DataManagement.tsx` 的 `UPLOAD_FORMAT_CHOICES` 一致：后端加一档而 UI 不知道，
 * 用户就永远选不到它，而那条路在 CLI 上是通的。
 */
const SEARCH_METHOD_OPTIONS = [
  { value: 'exact', label: '精确匹配' },
  { value: 'contains', label: '包含子串' },
  { value: 'ngram', label: 'N-gram 相似度' },
  { value: 'fuzzy', label: '模糊匹配' },
  { value: 'regex', label: '正则' }
]

/**
 * 四项增益的中文名与算法口径（逐条取自 `augmentor/impact.py:82` 的 `evaluate()`）
 *
 * 两条不能混：**前两项是比值**（`(after − before) / before`），**后两项是绝对差**
 * （`before.duplicate_rate − after.duplicate_rate`、`after.length_std − before.length_std`），
 * 所以 `dedup_gain` 为正表示重复率**下降**。渲染时统一显示原值、按项标口径，
 * 不折算成「总分」——四种量纲加在一起没有意义。
 */
const GAIN_PRESENTATION: Record<string, { label: string; caliber: string }> = {
  scale_gain: { label: '规模增益', caliber: '比值' },
  diversity_gain: { label: '多样性增益', caliber: '比值（按所选字段的唯一值）' },
  dedup_gain: { label: '去重增益', caliber: '绝对差，正=重复率下降' },
  length_spread_gain: { label: '长度离散增益', caliber: '绝对差（标准差）' },
}

/** 快照里可渲染的键：`extra` 用类型排除掉，而不是在运行时给它编一个标签 */
type SnapshotMetricKey = Exclude<keyof ImpactSnapshot, 'extra'>

/**
 * 两侧快照的渲染清单（`AugmentationMetrics.to_dict()` 六个键里渲染五个）
 *
 * `unique_instructions` 这个名字是**历史误名**：`measure()` 里它数的是
 * `set(texts)`，而 `texts` 取自 `text_field`，所以选了 `output` 时它数的是唯一**回答**数。
 * 这里按真实口径写标签，不跟着键名说谎。第三元是量纲，见 `formatSnapshotValue`。
 */
const SNAPSHOT_PRESENTATION: [SnapshotMetricKey, string, 'count' | 'decimal'][] = [
  ['total_items', '条目数', 'count'],
  ['unique_instructions', '唯一值数（按所选字段）', 'count'],
  ['avg_length', '平均长度', 'decimal'],
  ['length_std', '长度标准差', 'decimal'],
  ['duplicate_rate', '重复率', 'decimal'],
]

/**
 * 快照读数的格式化
 *
 * 长度标准差实测能到 `2.48746859276655` 这种位数，原样渲染会把表格撑破，所以按四位给；
 * 但条目数与唯一值数是**计数**，渲染成 `4.0000` 是假精度，所以分档。
 */
const formatSnapshotValue = (raw: number, kind: 'count' | 'decimal'): string =>
  kind === 'count' ? String(raw) : raw.toFixed(4)

/**
 * 质量三指标的中文名与算法口径（逐条取自 `augmentor/statistics.py:311`–`333`）
 *
 * 最容易读错的一格是**这三条与 `fields` 参数无关**：它们只遍历 `instruction` 与 `output`
 * 两格（实测：`fields=["output"]` 时三条读数一格未动）。用户勾了字段再看质量分，
 * 会以为那是「按所选字段算的质量」，所以口径必须写在行上而不是藏在文档里。
 * `diversity` 的分母是 instruction **非空条数**而不是条目数，`completeness` 的分母
 * 固定是 2×条目数——这两处都不跟着勾选项变。
 */
const STAT_QUALITY_PRESENTATION: Record<string, { label: string; caliber: string }> = {
  completeness: { label: '完整性', caliber: '（instruction 非空数 + output 非空数）÷（2 × 条目数），与 fields 无关' },
  diversity: { label: '多样性', caliber: '唯一 instruction 数 ÷ instruction 非空条数，与 fields 无关' },
  consistency: { label: '一致性', caliber: 'instruction 与 output 都非空且互不相等的条数 ÷ 条目数，与 fields 无关' },
}

/** `_infer_type`（`augmentor/feature_detect.py:105`）的五档字面量，后端没有常数可引 */
const FEATURE_TYPE_LABELS: Record<string, string> = {
  numeric: '数值',
  text: '文本',
  boolean: '布尔',
  mixed: '混合',
  unknown: '未识别',
}

/**
 * 重叠九指标（`augmentor/compare_enhanced.py:186`–`195`）
 *
 * 三档量纲不能混排：计数、带符号差、0–1 比值。`size_ratio` 那一格的「比例」还带一个
 * 实测坑——B 侧 0 条时 SDK 存 `float('inf')`，序列化到线上是 `null`（账本 **A288**），
 * 所以渲染器必须先接住 null 而不是 `toFixed`。
 */
type OverlapMetricKey = keyof CompareResponse['overlap_comparison']['metrics']

const OVERLAP_METRIC_PRESENTATION: [OverlapMetricKey, string, 'count' | 'signed' | 'ratio'][] = [
  ['size_a', 'A 侧条数', 'count'],
  ['size_b', 'B 侧条数', 'count'],
  ['size_diff', '条数差（A − B）', 'signed'],
  ['size_ratio', '条数比（A ÷ B）', 'ratio'],
  ['common_items', '匹配键交集条数', 'count'],
  ['unique_a', '仅 A 侧匹配键数', 'count'],
  ['unique_b', '仅 B 侧匹配键数', 'count'],
  ['similarity_score', '键相似度（交集 ÷ 并集）', 'ratio'],
  ['field_overlap', '字段重叠（交集 ÷ 并集）', 'ratio'],
]

const formatOverlapValue = (raw: number | null, kind: 'count' | 'signed' | 'ratio'): string => {
  if (raw === null) return '—（B 侧 0 条：SDK 存 inf，JSON 面为 null）'
  return kind === 'ratio' ? `${(raw * 100).toFixed(1)}%` : String(raw)
}

/**
 * `winner` 三态（实测：同一份文件自己比自己回 `tie`）
 *
 * 判据住在 `augmentor/comparison.py:260`：**只有平均质量分**参与，两者之差小于 0.01 即
 * 判平手。通过率、去重率、长度面再大也不看，所以标签必须带上这条口径。
 */
const WINNER_PRESENTATION: Record<string, { label: string; color: string }> = {
  a: { label: 'A 侧平均质量分占优', color: 'blue' },
  b: { label: 'B 侧平均质量分占优', color: 'orange' },
  tie: { label: '平手（两侧平均质量分之差 < 0.01）', color: 'default' },
}

/** 质量面对称三组（A/B 同键），组内键集合取自真实回执 */
const SIDE_FACES: { title: string; a: 'dedup_a' | 'length_a' | 'vocabulary_a'; b: 'dedup_b' | 'length_b' | 'vocabulary_b' }[] = [
  { title: '去重面', a: 'dedup_a', b: 'dedup_b' },
  { title: '长度面（固定按 instruction，端点未暴露这一档）', a: 'length_a', b: 'length_b' },
  { title: '词汇面（固定按 instruction，端点未暴露这一档）', a: 'vocabulary_a', b: 'vocabulary_b' },
]

const SIDE_METRIC_LABELS: Record<string, string> = {
  duplication_rate: '重复率',
  unique_count: '唯一条数',
  avg: '平均长度',
  min: '最短长度',
  max: '最长长度',
  std: '长度标准差',
  unique_chars: '唯一字符数',
  total_chars: '字符总数',
}

/**
 * 质量面四指标的口径（`augmentor/comparison.py:113`–`146`）
 *
 * 三条要说清：评分只喂 `instruction` 与 `output`（`original` 与 `generated` 都取
 * `instruction`），所以这不是「增强结果 vs 原文」的质量分；`pass_rate` 是 0–1 的**率**，
 * 与其余三个**分**混在同一格里，渲成 `1` 会被读成「1 条」，所以按 kind 分档；阈值 0.6 由
 * `DatasetComparator` 的出厂默认决定，这只端点没有把它开成参数（实测路由就是无参构造）。
 */
const QUALITY_FACE_PRESENTATION: Record<
  string,
  { label: string; caliber: string; kind: 'score' | 'rate' }
> = {
  avg_score: { label: '平均质量分', caliber: '单条 total_score 的均值', kind: 'score' },
  pass_rate: { label: '通过率', caliber: '达阈值（SDK 固定 0.6）条数 ÷ 条目数', kind: 'rate' },
  min_score: { label: '最低分', caliber: '单条最低', kind: 'score' },
  max_score: { label: '最高分', caliber: '单条最高', kind: 'score' },
}

/**
 * 质量面字典 → 有序键值对
 *
 * 行集合以 **A 侧为准**：`comparison.py:239` 的 `quality_diff` 就是照 `quality_a.keys()`
 * 派生的，所以跟着回执走而不是页面写一份键清单。
 */
const faceEntries = (face: QualityScoreFace): Map<string, number> => new Map(Object.entries(face))

/** 后端回执里的长度类读数既有整数也有小数（实测 `avg_length: 19.25`、`median: 15.5`），按原形状分档 */
const formatMeasure = (raw: number): string =>
  Number.isInteger(raw) ? String(raw) : raw.toFixed(4)

const formatRatio = (raw: number): string => `${(raw * 100).toFixed(2)}%`

export default function Analysis() {
  const [files, setFiles] = useState<string[]>([])
  const [selectedFile, setSelectedFile] = useState('')
  const [referenceFile, setReferenceFile] = useState('')
  const [analysis, setAnalysis] = useState<AnalyzeResponse | null>(null)
  const [cleaning, setCleaning] = useState<CleanResponse | null>(null)
  const [benchmark, setBenchmark] = useState<BenchmarkResponse | null>(null)
  const [profile, setProfile] = useState<LanguageProfileResponse | null>(null)
  const [evaluation, setEvaluation] = useState<EvaluateResponse | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  // 初值与契约默认同值（`contains`）⇒ 下拉显示的那一档就是发出去的那一档，两边不会各说各话
  const [searchMethod, setSearchMethod] = useState('contains')
  const [searchResult, setSearchResult] = useState<SearchResponse | null>(null)
  // 四档旋钮的初值各自等于契约默认（`top_k=5`、`text_key=instruction`、
  // `text_field=instruction`、`min_scale_gain=0.0`）⇒ 显示的就是发出去的那一档，
  // 两边不会各说各话。默认值的权威仍住在请求模型里，见 l227 的对账守卫。
  const [semanticQuery, setSemanticQuery] = useState('')
  const [semanticTopK, setSemanticTopK] = useState(5)
  const [semanticTextKey, setSemanticTextKey] = useState('instruction')
  const [semanticResult, setSemanticResult] = useState<SemanticSearchResponse | null>(null)
  const [afterFile, setAfterFile] = useState('')
  const [impactField, setImpactField] = useState('instruction')
  const [minScaleGain, setMinScaleGain] = useState(0)
  const [impact, setImpact] = useState<ImpactResponse | null>(null)
  // L228 三张卡的初值一律留空：`fields` 与 `key_fields` 在契约里的默认是 `None`（＝全字段 /
  // SDK 内置 ["instruction"]），页面写一个非空初值等于替端点编第二套默认。
  const [stats, setStats] = useState<StatisticsResponse | null>(null)
  const [statsFields, setStatsFields] = useState<string[]>([])
  const [statFieldUniverse, setStatFieldUniverse] = useState<string[]>([])
  const [features, setFeatures] = useState<FeaturesResponse | null>(null)
  const [compareFile, setCompareFile] = useState('')
  const [compareKeyFields, setCompareKeyFields] = useState<string[]>([])
  const [comparison, setComparison] = useState<CompareResponse | null>(null)
  const [charts, setCharts] = useState<Record<string, string> | null>(null)
  const [loading, setLoading] = useState(false)

  const loadFiles = useCallback(async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map(f => f.name))
    } catch {
      console.error('加载文件列表失败')
    }
  }, [])

  const loadAnalysis = useCallback(async () => {
    try {
      const result = await analyzeData(selectedFile)
      setAnalysis(result)
    } catch {
      console.error('分析数据失败')
    }
  }, [selectedFile])

  // effect 放在被调用函数的声明之后（理由同 DataManagement.tsx）
  useEffect(() => {
    loadFiles()
  }, [loadFiles])

  useEffect(() => {
    if (selectedFile) {
      loadAnalysis()
    }
  }, [selectedFile, loadAnalysis])

  const handleClean = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    setLoading(true)
    try {
      setCleaning(await cleanData(selectedFile))
    } catch (err) {
      message.error(apiErrorDetail(err, '数据清洗失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleBenchmark = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    setLoading(true)
    try {
      setBenchmark(await runBenchmark(selectedFile))
    } catch (err) {
      message.error(apiErrorDetail(err, '基准测试失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleVisualize = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    setLoading(true)
    try {
      const result = await visualizeData(selectedFile)
      setCharts(result.charts)
    } catch (err) {
      message.error(apiErrorDetail(err, '图表生成失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleProfile = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    setLoading(true)
    try {
      setProfile(await languageProfile(selectedFile))
    } catch (err) {
      message.error(apiErrorDetail(err, '语言画像失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleEvaluate = async () => {
    // 两侧都要选：`generated_file` 与 `reference_file` 在契约里都是必填。
    // 允许选同一份文件（模型注释里写明这是「用同一份文件的两个字段做对照」的合法用法）。
    if (!selectedFile || !referenceFile) {
      message.warning('评估需要同时选中生成集与参照集')
      return
    }
    setLoading(true)
    try {
      setEvaluation(await evaluateDataset(selectedFile, referenceFile))
    } catch (err) {
      message.error(apiErrorDetail(err, '批量评估失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleSearch = async () => {
    // 空串不发：`query` 在契约里是必填，服务端拿到空串会走「全部命中」那条支路，
    // 报回来的 total_matches 不是用户想问的问题。
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    if (!searchQuery.trim()) {
      message.warning('请输入检索词')
      return
    }
    setLoading(true)
    try {
      setSearchResult(await searchDataset(selectedFile, searchQuery, { method: searchMethod }))
    } catch (err) {
      message.error(apiErrorDetail(err, '内容检索失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleSemantic = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    // 空串必须拦：实测 `query=""` 照样回条目且 `score` 全 0，那是一份退化读数而不是
    // 「全都相关」。服务端不判空，判据就得住在门口。
    if (!semanticQuery.trim()) {
      message.warning('请输入查询文本')
      return
    }
    setLoading(true)
    try {
      setSemanticResult(
        await semanticSearch(selectedFile, semanticQuery, {
          topK: semanticTopK,
          textKey: semanticTextKey,
        })
      )
    } catch (err) {
      message.error(apiErrorDetail(err, '语义检索失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleStats = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    setLoading(true)
    try {
      // 一个都没勾 ⇒ 连 `fields` 这个键都不发（服务层传 undefined，JSON 序列化会丢掉它），
      // 让契约默认值 `None` 去决定「看全部字段」。实测 `fields: []` 与缺省等价，但缺省才是权威。
      const result = await datasetStats(selectedFile, {
        fields: statsFields.length ? statsFields : undefined,
      })
      setStats(result)
      // 可选字段清单只从**未限定字段**的那次回执刷新：勾了 `output` 再跑，回执里就只剩
      // `output`，拿它当清单会把用户锁死在一格里。
      if (!statsFields.length) {
        setStatFieldUniverse(Object.keys(result.field_statistics))
      }
    } catch (err) {
      message.error(apiErrorDetail(err, '字段统计失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleFeatures = async () => {
    if (!selectedFile) {
      message.warning('请先选择文件')
      return
    }
    setLoading(true)
    try {
      setFeatures(await detectFeatures(selectedFile))
    } catch (err) {
      message.error(apiErrorDetail(err, '特征普查失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleCompare = async () => {
    // 两侧都要选：`dataset_a` 与 `dataset_b` 在契约里都是必填。允许选同一份文件，
    // 那是实测过的合法用法（回执 `winner` 为 `tie`、`quality_diff` 四项全 0）。
    if (!selectedFile || !compareFile) {
      message.warning('双集对比需要同时选中 A 侧与 B 侧')
      return
    }
    setLoading(true)
    try {
      setComparison(
        await compareDatasets(selectedFile, compareFile, {
          keyFields: compareKeyFields.length ? compareKeyFields : undefined,
        })
      )
    } catch (err) {
      message.error(apiErrorDetail(err, '双集对比失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleImpact = async () => {
    // 两侧都要选：`before_file` 与 `after_file` 在契约里都是必填。允许选同一份文件
    // （那就是「没有增强」的对照，四项增益应全为 0，这条读数本身有价值）。
    if (!selectedFile || !afterFile) {
      message.warning('影响评估需要同时选中基线集与增强后数据集')
      return
    }
    setLoading(true)
    try {
      setImpact(
        await assessImpact(selectedFile, afterFile, {
          textField: impactField,
          minScaleGain: minScaleGain,
        })
      )
    } catch (err) {
      message.error(apiErrorDetail(err, '影响评估失败'))
    } finally {
      setLoading(false)
    }
  }

  const getQuestionTypeChartOption = () => {
    if (!analysis?.coverage_analysis?.question_type_distribution) return {}
    
    const data = analysis.coverage_analysis.question_type_distribution
    return {
      title: { text: '问题类型分布', left: 'center' },
      tooltip: { trigger: 'item' },
      series: [{
        type: 'pie',
        radius: '50%',
        data: Object.entries(data).map(([name, value]) => ({
          name,
          value: Math.round((value as number) * 100)
        }))
      }]
    }
  }

  const getLengthChartOption = () => {
    if (!analysis?.coverage_analysis?.length_distribution) return {}
    
    const data = analysis.coverage_analysis.length_distribution
    return {
      title: { text: '长度分布', left: 'center' },
      tooltip: { trigger: 'axis' },
      xAxis: {
        type: 'category',
        data: ['短 (<10)', '中 (10-30)', '长 (>30)']
      },
      yAxis: { type: 'value', name: '比例 (%)' },
      series: [{
        type: 'bar',
        data: [
          Math.round((data.short || 0) * 100),
          Math.round((data.medium || 0) * 100),
          Math.round((data.long || 0) * 100)
        ]
      }]
    }
  }

  const topicColumns = [
    { title: '关键词', dataIndex: 'word', key: 'word' },
    { title: '频次', dataIndex: 'count', key: 'count' }
  ]

  const topicData = analysis?.coverage_analysis?.topic_distribution?.top_words
    ? Object.entries(analysis.coverage_analysis.topic_distribution.top_words).map(([word, count]) => ({
        word,
        count
      }))
    : []

  // 高亮展开成行：一个命中条目可能在多个字段命中，每个字段一行。SDK 给的是固定三段
  // `[命中前, 命中文本, 命中后]`；`hit` 单列才看得出「命中的究竟是什么」，`context`
  // 把三段拼回原文给上下文——两列都要，只留 context 等于把高亮退化成再列一遍原文。
  const highlightRows = (searchResult?.highlights || []).flatMap(entry =>
    Object.entries(entry.highlights).map(([field, parts]) => ({
      key: `${entry.index}-${field}`,
      index: entry.index,
      field,
      hit: parts[1] ?? '',
      context: parts.join('')
    }))
  )

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Select
          style={{ width: 300 }}
          value={selectedFile}
          onChange={setSelectedFile}
          placeholder="选择要分析的文件"
        >
          {files.map(file => (
            <Select.Option key={file} value={file}>{file}</Select.Option>
          ))}
        </Select>
      </div>

      {analysis && (
        <>
          <Row gutter={16} style={{ marginBottom: 16 }}>
            <Col span={6}>
              <Card>
                <Statistic title="总数据量" value={analysis.statistics?.total_items || 0} />
              </Card>
            </Col>
            <Col span={6}>
              <Card>
                <Statistic title="平均长度" value={analysis.statistics?.avg_length?.toFixed(1) || 0} />
              </Card>
            </Col>
            <Col span={6}>
              <Card>
                <Statistic title="唯一词数" value={analysis.statistics?.unique_words || 0} />
              </Card>
            </Col>
            <Col span={6}>
              <Card>
                <Statistic title="去重移除数" value={analysis.dedup_report?.removed_count || 0} />
              </Card>
            </Col>
          </Row>

          <Row gutter={16} style={{ marginBottom: 16 }}>
            <Col span={12}>
              <Card>
                <ReactEChartsCore echarts={echarts} option={getQuestionTypeChartOption()} style={{ height: 300 }} />
              </Card>
            </Col>
            <Col span={12}>
              <Card>
                <ReactEChartsCore echarts={echarts} option={getLengthChartOption()} style={{ height: 300 }} />
              </Card>
            </Col>
          </Row>

          <Card title="关键词统计">
            <Table
              columns={topicColumns}
              dataSource={topicData}
              rowKey="word"
              pagination={{ pageSize: 10 }}
            />
          </Card>

          <Card title="质量速览" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }}>
              <Button loading={loading} onClick={handleClean}>
                运行数据清洗
              </Button>
              <Button type="primary" loading={loading} onClick={handleBenchmark}>
                运行质量基准
              </Button>
            </Space>

            {cleaning && (
              <Space wrap style={{ marginBottom: 12 }}>
                <Tag color="blue">原始 {cleaning.original_count}</Tag>
                <Tag color="green">清洗后 {cleaning.cleaned_count}</Tag>
                <Tag color="red">丢弃 {cleaning.dropped_count}</Tag>
                <Tag color="orange">变更 {cleaning.changed_count}</Tag>
              </Space>
            )}

            {benchmark && (
              <Table
                size="small"
                rowKey="metric"
                pagination={false}
                columns={[
                  { title: '基准指标', dataIndex: 'metric' },
                  { title: '数值', dataIndex: 'value' }
                ]}
                dataSource={Object.entries(benchmark.metrics || {}).map(
                  ([metric, value]) => ({
                    metric,
                    value:
                      typeof value === 'number' ? value.toFixed(4) : String(value)
                  })
                )}
              />
            )}
          </Card>

          <Card title="可视化图表文件" style={{ marginTop: 16 }}>
            <Button loading={loading} onClick={handleVisualize}>
              生成图表文件
            </Button>
            {charts && (
              <Table
                size="small"
                style={{ marginTop: 12 }}
                rowKey="name"
                pagination={false}
                columns={[
                  { title: '图表', dataIndex: 'name', key: 'name' },
                  { title: '落盘路径', dataIndex: 'path', key: 'path' }
                ]}
                dataSource={Object.entries(charts).map(([name, path]) => ({ name, path }))}
              />
            )}
          </Card>

          <Card title="语言画像" style={{ marginTop: 16 }}>
            <Button loading={loading} onClick={handleProfile}>
              运行语言画像
            </Button>
            {profile && (
              <>
                <Space wrap style={{ margin: '12px 0' }}>
                  <Tag color="blue">共 {profile.total_items} 条</Tag>
                  <Tag color="purple">支持 {profile.supported_langs.join('、')}</Tag>
                  <Tag color="cyan">默认目标语 {profile.default_target_lang}</Tag>
                </Space>
                {profile.outside_supported.length > 0 && (
                  <Alert
                    type="info"
                    showIcon
                    style={{ marginBottom: 12 }}
                    message={`落在支持清单之外：${profile.outside_supported.join('、')}`}
                    description="其中 mixed 是检测标签而不是语言码，出现它不构成错误。"
                  />
                )}
                <Table
                  size="small"
                  rowKey="label"
                  pagination={false}
                  columns={[
                    { title: '检测标签', dataIndex: 'label' },
                    { title: '条数', dataIndex: 'count' },
                    { title: '占比 (%)', dataIndex: 'ratio' }
                  ]}
                  dataSource={Object.entries(profile.distribution)
                    .map(([label, count]) => ({
                      label,
                      count,
                      ratio: ((profile.ratios[label] || 0) * 100).toFixed(1)
                    }))
                    .sort((a, b) => b.count - a.count)}
                />
              </>
            )}
          </Card>

          <Card title="生成集对比评估" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <Select
                style={{ width: 260 }}
                value={referenceFile}
                onChange={setReferenceFile}
                placeholder="选择参照集"
              >
                {files.map(file => (
                  <Select.Option key={file} value={file}>{file}</Select.Option>
                ))}
              </Select>
              <Button type="primary" loading={loading} onClick={handleEvaluate}>
                运行批量评估
              </Button>
            </Space>
            {evaluation && (
              <>
                <Tag color="blue" style={{ marginBottom: 12 }}>
                  对齐样本数 {evaluation.sample_count}
                </Tag>
                <Table
                  size="small"
                  rowKey="metric"
                  pagination={false}
                  columns={[
                    { title: '指标', dataIndex: 'metric' },
                    { title: '数值', dataIndex: 'value' }
                  ]}
                  dataSource={Object.entries(evaluation.metrics).map(([metric, value]) => ({
                    metric,
                    value: value.toFixed(4)
                  }))}
                />
              </>
            )}
          </Card>

          <Card title="内容检索" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <Input
                style={{ width: 260 }}
                placeholder="检索词"
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
                onPressEnter={handleSearch}
              />
              <Select
                style={{ width: 180 }}
                value={searchMethod}
                onChange={setSearchMethod}
                options={SEARCH_METHOD_OPTIONS}
              />
              <Button type="primary" loading={loading} onClick={handleSearch}>
                运行检索
              </Button>
            </Space>
            {searchResult && (
              <>
                <Space wrap style={{ marginBottom: 12 }}>
                  <Tag color="blue">命中 {searchResult.total_matches} 条</Tag>
                  {/* 命中总数与本次返回条数是两件事：limit 截断时前者大、后者小，
                      只报一个数会把「还有 900 条没回传」说成「只有 100 条命中」。 */}
                  <Tag>本次返回 {searchResult.items.length} 条</Tag>
                  <Tag>方法 {searchResult.method}</Tag>
                  <Tag>耗时 {searchResult.query_time_ms.toFixed(2)} ms</Tag>
                  {searchResult.fuzzy_threshold !== null && (
                    <Tag color="purple">相似度阈值 {searchResult.fuzzy_threshold.toFixed(2)}</Tag>
                  )}
                  {searchResult.ngram_n !== null && (
                    <Tag color="purple">n-gram {searchResult.ngram_n}</Tag>
                  )}
                  {searchResult.matches_before_filters !== null && (
                    <Tag color="orange">过滤前 {searchResult.matches_before_filters} 条</Tag>
                  )}
                </Space>
                {highlightRows.length === 0 && searchResult.total_matches > 0 && (
                  <Alert
                    type="info"
                    showIcon
                    style={{ marginBottom: 12 }}
                    message={`方法 ${searchResult.method} 不产出高亮片段，命中数以「命中 N 条」那个计数为准`}
                  />
                )}
                {highlightRows.length === 0 && searchResult.total_matches === 0 && (
                  <Alert type="warning" showIcon message="没有命中" />
                )}
                {highlightRows.length > 0 && (
                  <Table
                    size="small"
                    rowKey="key"
                    pagination={{ pageSize: 8 }}
                    columns={[
                      { title: '条目下标', dataIndex: 'index', width: 100 },
                      { title: '字段', dataIndex: 'field', width: 120 },
                      { title: '命中片段', dataIndex: 'hit', width: 160 },
                      { title: '上下文', dataIndex: 'context', ellipsis: true }
                    ]}
                    dataSource={highlightRows}
                  />
                )}
              </>
            )}
          </Card>

          <Card title="语义相似检索" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <Input
                style={{ width: 260 }}
                placeholder="查询文本"
                value={semanticQuery}
                onChange={e => setSemanticQuery(e.target.value)}
                onPressEnter={handleSemantic}
              />
              {/* 下界放 0 而不是 1：实测 `top_k=0` 在后端是合法的「只要回显、不要命中」
                  （HTTP 200、`total=0`），替它加一条页面规则就是第二套判据。 */}
              <span>前 K 条</span>
              <InputNumber
                min={0}
                max={50}
                value={semanticTopK}
                onChange={value => setSemanticTopK(value ?? 5)}
              />
              {/* 被编码字段：这一档改的是「拿什么去算相似度」，命中表里那一列文本也跟着换。
                  实测只影响回显的 `text`，排序与 `backend`/`dimension` 的回显口径不变。 */}
              <Select
                style={{ width: 170 }}
                value={semanticTextKey}
                onChange={setSemanticTextKey}
                options={[
                  { value: 'instruction', label: '编码 instruction' },
                  { value: 'output', label: '编码 output' }
                ]}
              />
              <Button type="primary" loading={loading} onClick={handleSemantic}>
                运行语义检索
              </Button>
            </Space>
            {semanticResult && (
              <>
                <Space wrap style={{ marginBottom: 12 }}>
                  <Tag color="blue">命中 {semanticResult.total} 条</Tag>
                  {/* 三个「本次生效值」是契约要求回显的东西：dimension 一改排名就变，
                      不显示就等于让用户猜这一次是用什么算的。 */}
                  <Tag>后端 {semanticResult.backend}</Tag>
                  <Tag>维度 {semanticResult.dimension}</Tag>
                  <Tag>集合 {semanticResult.collection}</Tag>
                </Space>
                {semanticResult.results.length === 0 ? (
                  <Alert type="warning" showIcon message="没有命中（前 K 条取到 0 条或数据面为空）" />
                ) : (
                  <Table
                    size="small"
                    rowKey="index"
                    pagination={{ pageSize: 8 }}
                    columns={[
                      { title: '条目下标', dataIndex: 'index', width: 100 },
                      { title: '相似度', dataIndex: 'score', width: 120 },
                      { title: '命中文本', dataIndex: 'text', ellipsis: true }
                    ]}
                    dataSource={semanticResult.results.map(hit => ({
                      index: hit.index,
                      // 相似度是相对排序量，不是「匹配了百分之几」，保留四位原样给
                      score: hit.score.toFixed(4),
                      text: hit.text
                    }))}
                  />
                )}
              </>
            )}
          </Card>

          <Card title="增强前后影响" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <span>基线：{selectedFile || '未选'}</span>
              <Select
                style={{ width: 220 }}
                placeholder="增强后数据集"
                value={afterFile || undefined}
                onChange={setAfterFile}
                options={files.map(f => ({ value: f, label: f }))}
              />
              <Select
                style={{ width: 180 }}
                value={impactField}
                onChange={setImpactField}
                options={[
                  { value: 'instruction', label: '看 instruction' },
                  { value: 'output', label: '看 output' }
                ]}
              />
              {/* 判决门槛：`beneficial` 的两条判据里只有这一条可调（另一条 −0.05 是
                  SDK 常数，不在这次的面上）。不封顶也不设下界——请求模型没有值域校验，
                  页面替它编一条判据就是第二套规则。 */}
              <span>规模增益门槛</span>
              <InputNumber
                step={0.05}
                value={minScaleGain}
                onChange={value => setMinScaleGain(value ?? 0)}
              />
              <Button type="primary" loading={loading} onClick={handleImpact}>
                运行影响评估
              </Button>
            </Space>
            {impact && (
              <>
                <Space wrap style={{ marginBottom: 12 }}>
                  <Tag color={impact.beneficial ? 'green' : 'red'}>
                    {impact.beneficial ? '判为有增益' : '判为无增益'}
                  </Tag>
                  {/* `is_beneficial` 只看 `scale_gain` 与 `dedup_gain` 两项，另外两项再大
                      也不参与判定——判决口径要说清，否则用户会把红色标签读成「四项全差」。
                      门槛是可调的，所以这里报**本次用的那个值**而不是出厂默认。 */}
                  <Alert
                    type="info"
                    showIcon
                    style={{ width: 'auto' }}
                    message={`判定只看两项：规模增益不低于门槛（本次 ${minScaleGain}），且去重增益不低于 −0.05`}
                  />
                </Space>
                <Table
                  size="small"
                  rowKey="key"
                  pagination={false}
                  style={{ marginBottom: 16 }}
                  columns={[
                    { title: '增益项', dataIndex: 'label' },
                    { title: '键', dataIndex: 'key', width: 180 },
                    { title: '量纲', dataIndex: 'caliber', width: 220 },
                    { title: '数值', dataIndex: 'value', width: 120 }
                  ]}
                  dataSource={Object.entries(impact.gains).map(([key, value]) => ({
                    key,
                    label: GAIN_PRESENTATION[key]?.label ?? key,
                    caliber: GAIN_PRESENTATION[key]?.caliber ?? '未知口径',
                    value: value.toFixed(4)
                  }))}
                />
                <Row gutter={16}>
                  {(['before', 'after'] as const).map(side => (
                    <Col span={12} key={side}>
                      <Table
                        size="small"
                        rowKey="name"
                        pagination={false}
                        title={() => (side === 'before' ? '基线侧' : '增强后侧')}
                        columns={[
                          { title: '指标', dataIndex: 'name' },
                          { title: '数值', dataIndex: 'value', width: 120 }
                        ]}
                        dataSource={SNAPSHOT_PRESENTATION.map(([key, name, kind]) => ({
                          name,
                          value: formatSnapshotValue(impact[side][key], kind),
                        }))}
                      />
                    </Col>
                  ))}
                </Row>
              </>
            )}
          </Card>

          <Card title="字段级统计" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <Select
                mode="multiple"
                style={{ minWidth: 260 }}
                placeholder="限定字段（留空＝全部字段）"
                value={statsFields}
                onChange={setStatsFields}
                options={statFieldUniverse.map(f => ({ value: f, label: f }))}
              />
              <Button type="primary" loading={loading} onClick={handleStats}>
                运行字段统计
              </Button>
              {/* 字段候选只能来自回执：实测 `fields` 给一个数据集中不存在的名字，端点照样回
                  200 并给这一格填整套全零（幽灵字段）。页面自己列一份字段名清单，
                  就是在替数据集编一份可能不存在的清单。 */}
              {!statFieldUniverse.length && (
                <Alert
                  type="info"
                  showIcon
                  style={{ width: 'auto' }}
                  message="字段候选取自回执：先不限定字段跑一次，清单才会出现"
                />
              )}
            </Space>
            {stats && (
              <>
                <Space wrap style={{ marginBottom: 12 }}>
                  <Tag color="blue">{stats.dataset_name}</Tag>
                  <Tag>共 {stats.total_items} 条</Tag>
                  <Tag>{Object.keys(stats.field_statistics).length} 个字段</Tag>
                </Space>
                {stats.total_items === 0 && (
                  <Alert
                    type="warning"
                    showIcon
                    style={{ marginBottom: 12 }}
                    message="空数据集：实测三个统计字典全为 {}，下面按「无内容」渲染而不补 0"
                  />
                )}
                {/* 后端已经排好行的摘要原样给出，重排它就是二次说谎的机会 */}
                <div style={{ whiteSpace: 'pre-line', marginBottom: 12 }}>{stats.summary}</div>
                <Table
                  size="small"
                  rowKey="field"
                  pagination={false}
                  style={{ marginBottom: 16 }}
                  expandable={{
                    expandedRowRender: row =>
                      row.top_values.length === 0 ? (
                        <span>该字段没有可统计的非空值</span>
                      ) : (
                        <Table
                          size="small"
                          rowKey={(_, i) => String(i)}
                          pagination={false}
                          columns={[
                            { title: '取值', dataIndex: 'value' },
                            { title: '次数', dataIndex: 'count', width: 100 }
                          ]}
                          dataSource={row.top_values.map(([value, count]) => ({ value, count }))}
                        />
                      )
                  }}
                  columns={[
                    { title: '字段', dataIndex: 'field' },
                    { title: '条目数', dataIndex: 'total', width: 90 },
                    { title: '非空 / 空', dataIndex: 'filled', width: 110 },
                    { title: '填充率', dataIndex: 'fillRate', width: 100 },
                    { title: '平均长度', dataIndex: 'avg', width: 100 },
                    { title: '中位长度', dataIndex: 'median', width: 100 },
                    { title: '最短 – 最长', dataIndex: 'span', width: 120 },
                    { title: '标准差', dataIndex: 'std', width: 100 },
                    { title: '唯一值数', dataIndex: 'unique', width: 100 }
                  ]}
                  dataSource={Object.values(stats.field_statistics).map(entry => ({
                    field: entry.field_name,
                    total: entry.total_count,
                    filled: `${entry.filled_count} / ${entry.empty_count}`,
                    fillRate: formatRatio(entry.fill_rate),
                    avg: formatMeasure(entry.avg_length),
                    median: formatMeasure(entry.median_length),
                    span: `${entry.min_length} – ${entry.max_length}`,
                    std: formatMeasure(entry.std_deviation),
                    unique: entry.unique_count,
                    top_values: entry.top_values
                  }))}
                />
                {Object.keys(stats.quality_metrics).length === 0 ? (
                  <Alert type="info" showIcon message="回执没有质量指标（空数据集实测如此）" />
                ) : (
                  <Table
                    size="small"
                    rowKey="key"
                    pagination={false}
                    style={{ marginBottom: 16 }}
                    columns={[
                      { title: '质量指标', dataIndex: 'label' },
                      { title: '键', dataIndex: 'key', width: 160 },
                      { title: '口径', dataIndex: 'caliber' },
                      { title: '数值', dataIndex: 'value', width: 110 }
                    ]}
                    dataSource={Object.entries(stats.quality_metrics).map(([key, value]) => ({
                      key,
                      label: STAT_QUALITY_PRESENTATION[key]?.label ?? key,
                      caliber: STAT_QUALITY_PRESENTATION[key]?.caliber ?? '未知口径',
                      value: formatRatio(value)
                    }))}
                  />
                )}
                {stats.content_statistics.text_length && (
                  <Table
                    size="small"
                    rowKey="name"
                    pagination={false}
                    style={{ marginBottom: 16 }}
                    title={() => '文本长度面（按回执的实际字段，非按页面猜的字段）'}
                    columns={[
                      { title: '指标', dataIndex: 'name' },
                      { title: '数值', dataIndex: 'value', width: 120 }
                    ]}
                    dataSource={[
                      { name: '平均长度', value: formatMeasure(stats.content_statistics.text_length.avg) },
                      { name: '最短长度', value: stats.content_statistics.text_length.min },
                      { name: '最长长度', value: stats.content_statistics.text_length.max },
                      { name: '长度合计', value: stats.content_statistics.text_length.total }
                    ]}
                  />
                )}
                {stats.content_statistics.vocabulary && (
                  <>
                    <Space wrap style={{ marginBottom: 8 }}>
                      <Tag>词数合计 {stats.content_statistics.vocabulary.total_words}</Tag>
                      <Tag>唯一词数 {stats.content_statistics.vocabulary.unique_words}</Tag>
                    </Space>
                    <Table
                      size="small"
                      rowKey={(_, i) => String(i)}
                      pagination={{ pageSize: 8 }}
                      title={() => '高频词（[值, 次数] 二元组，与 analyze 那只端点的字典形状不同）'}
                      columns={[
                        { title: '词', dataIndex: 'word' },
                        { title: '次数', dataIndex: 'count', width: 100 }
                      ]}
                      dataSource={stats.content_statistics.vocabulary.top_words.map(
                        ([word, count]) => ({ word, count })
                      )}
                    />
                  </>
                )}
              </>
            )}
          </Card>

          <Card title="特征普查" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <span>当前文件：{selectedFile || '未选'}</span>
              <Button type="primary" loading={loading} onClick={handleFeatures}>
                运行特征普查
              </Button>
              {/* 稀疏判据 `min_coverage`（`augmentor/feature_detect.py:59`，出厂 0.1）是这只端点
                  唯一能调却没调的东西——请求模型只有 `input_file`，实测多送键被静默忽略。
                  所以这里不装旋钮，只把判据写出来。 */}
              <Alert
                type="info"
                showIcon
                style={{ width: 'auto' }}
                message="稀疏判据为覆盖率低于 0.1，该阈值由后端固定、这只端点不接受调用方改它"
              />
            </Space>
            {features && (
              <>
                <Space wrap style={{ marginBottom: 12 }}>
                  <Tag>共 {features.total_items} 条</Tag>
                  <Tag>{features.field_features.length} 个字段</Tag>
                </Space>
                <Table
                  size="small"
                  rowKey="name"
                  pagination={false}
                  style={{ marginBottom: 16 }}
                  title={() => '按覆盖率升序（最稀疏的字段排在最前，这正是这张卡的用途）'}
                  columns={[
                    { title: '字段', dataIndex: 'name' },
                    { title: '推断类型', dataIndex: 'featureType', width: 120 },
                    { title: '覆盖率', dataIndex: 'coverage', width: 110 },
                    { title: '唯一值占比', dataIndex: 'uniqueRatio', width: 120 },
                    { title: '判据', dataIndex: 'description', width: 120 }
                  ]}
                  dataSource={[...features.field_features]
                    .sort((a, b) => a.coverage - b.coverage)
                    .map(entry => ({
                      name: entry.name,
                      featureType: FEATURE_TYPE_LABELS[entry.feature_type] ?? entry.feature_type,
                      coverage: formatRatio(entry.coverage),
                      uniqueRatio: formatRatio(entry.unique_ratio),
                      description: entry.description
                    }))}
                />
                {features.sparse_fields.length === 0 ? (
                  <Alert type="success" showIcon style={{ marginBottom: 16 }} message="无稀疏字段" />
                ) : (
                  <Space wrap style={{ marginBottom: 16 }}>
                    <span>稀疏字段：</span>
                    {features.sparse_fields.map(name => (
                      <Tag key={name} color="orange">{name}</Tag>
                    ))}
                  </Space>
                )}
                {Object.keys(features.intent_distribution).length === 0 ? (
                  <Alert
                    type="warning"
                    showIcon
                    message="意图分布为空：实测空集与「一条问话都没有」的集都会回 {}"
                  />
                ) : (
                  <Table
                    size="small"
                    rowKey="intent"
                    pagination={false}
                    title={() => '意图条数（与上方「覆盖度分析」那张图同一分类器，这边是条数、那边是占比）'}
                    columns={[
                      { title: '意图', dataIndex: 'intent' },
                      { title: '条数', dataIndex: 'count', width: 120 }
                    ]}
                    dataSource={Object.entries(features.intent_distribution).map(([intent, count]) => ({
                      intent,
                      count
                    }))}
                  />
                )}
              </>
            )}
          </Card>

          <Card title="双集对比" style={{ marginTop: 16 }}>
            <Space style={{ marginBottom: 16 }} wrap>
              <span>A 侧：{selectedFile || '未选'}</span>
              <Select
                style={{ width: 220 }}
                placeholder="B 侧数据集"
                value={compareFile || undefined}
                onChange={setCompareFile}
                options={files.map(f => ({ value: f, label: f }))}
              />
              <Select
                mode="multiple"
                style={{ minWidth: 240 }}
                placeholder="匹配键（留空＝后端默认）"
                value={compareKeyFields}
                onChange={setCompareKeyFields}
                options={Object.keys(comparison?.overlap_comparison.field_comparisons ?? {}).map(
                  f => ({ value: f, label: f })
                )}
              />
              <Button type="primary" loading={loading} onClick={handleCompare}>
                运行双集对比
              </Button>
            </Space>
            {comparison && (() => {
              const quality = comparison.quality_comparison
              const overlap = comparison.overlap_comparison
              const aSide = faceEntries(quality.quality_a)
              const bSide = faceEntries(quality.quality_b)
              const diffSide = faceEntries(quality.quality_diff)
              return (
                <>
                  <Space wrap style={{ marginBottom: 12 }}>
                    <Tag color={WINNER_PRESENTATION[quality.winner]?.color ?? 'default'}>
                      {WINNER_PRESENTATION[quality.winner]?.label ?? `未知判定：${quality.winner}`}
                    </Tag>
                    <Alert
                      type="info"
                      showIcon
                      style={{ width: 'auto' }}
                      message="判决只看平均质量分（之差小于 0.01 判平手），通过率与长度面不参与"
                    />
                    {/* 幽灵匹配键比幽灵字段名更危险：实测给一个两侧都没有的键，
                        两份毫不相干的数据会回 `common_items: 1`、`similarity_score: 1.0`
                        ——把「毫无重叠」谎报成「完全重叠」。所以候选只从回执的字段并集里取。 */}
                    <Tag>
                      本次匹配键：
                      {compareKeyFields.length
                        ? compareKeyFields.join('、')
                        : 'instruction（请求未发 key_fields，取 SDK 默认）'}
                    </Tag>
                  </Space>
                  <Table
                    size="small"
                    rowKey="key"
                    pagination={false}
                    style={{ marginBottom: 16 }}
                    title={() => `基本信息：A ${quality.dataset_a_name}（${quality.dataset_a_count} 条） / B ${quality.dataset_b_name}（${quality.dataset_b_count} 条）`}
                    columns={[
                      { title: '质量面指标', dataIndex: 'label' },
                      { title: '键', dataIndex: 'key', width: 140 },
                      { title: '口径', dataIndex: 'caliber' },
                      { title: 'A 侧', dataIndex: 'a', width: 110 },
                      { title: 'B 侧', dataIndex: 'b', width: 110 },
                      { title: '差值（B − A）', dataIndex: 'diff', width: 130 }
                    ]}
                    dataSource={[...aSide].map(([key, value]) => {
                      const shown = (raw: number) =>
                        (QUALITY_FACE_PRESENTATION[key]?.kind ?? 'score') === 'rate'
                          ? formatRatio(raw)
                          : formatMeasure(raw)
                      return {
                        key,
                        label: QUALITY_FACE_PRESENTATION[key]?.label ?? key,
                        caliber: QUALITY_FACE_PRESENTATION[key]?.caliber ?? '未知口径',
                        a: shown(value),
                        b: shown(bSide.get(key) ?? 0),
                        diff: shown(diffSide.get(key) ?? 0)
                      }
                    })}
                  />
                  {SIDE_FACES.map(face => (
                    <Table
                      size="small"
                      key={face.a}
                      rowKey="name"
                      pagination={false}
                      style={{ marginBottom: 16 }}
                      title={() => face.title}
                      columns={[
                        { title: '指标', dataIndex: 'name' },
                        { title: 'A 侧', dataIndex: 'a', width: 120 },
                        { title: 'B 侧', dataIndex: 'b', width: 120 }
                      ]}
                      dataSource={Object.entries(
                        quality[face.a] as unknown as Record<string, number>
                      ).map(([key, value]) => ({
                        name: SIDE_METRIC_LABELS[key] ?? key,
                        a: formatMeasure(value),
                        b: formatMeasure(
                          (quality[face.b] as unknown as Record<string, number>)[key] ?? 0
                        )
                      }))}
                    />
                  ))}
                  <Table
                    size="small"
                    rowKey="key"
                    pagination={false}
                    style={{ marginBottom: 16 }}
                    title={() => '重叠面九项指标'}
                    columns={[
                      { title: '指标', dataIndex: 'name' },
                      { title: '键', dataIndex: 'key', width: 160 },
                      { title: '数值', dataIndex: 'value', width: 200 }
                    ]}
                    dataSource={OVERLAP_METRIC_PRESENTATION.map(([key, name, kind]) => ({
                      key,
                      name,
                      value: formatOverlapValue(overlap.metrics[key], kind)
                    }))}
                  />
                  <Table
                    size="small"
                    rowKey="field"
                    pagination={false}
                    style={{ marginBottom: 16 }}
                    title={() => '字段对照（逐条值差异在行内展开）'}
                    expandable={{
                      expandedRowRender: row =>
                        row.differences.length === 0 ? (
                          <span>该字段没有逐条值差异（两侧配上的条目在这一档同值）</span>
                        ) : (
                          <Table
                            size="small"
                            rowKey={(_, i) => String(i)}
                            pagination={{ pageSize: 8 }}
                            // 明细条目的键集合实测固定是 `{instruction, value_a, value_b}`（匹配键不影响它），
                            // 列头仍从条目键的并集里取：SDK 哪天换形状，页面跟着回执走而不是跟着注释走。
                            columns={Object.keys(row.differences[0]).map(key => ({
                              title: key,
                              dataIndex: key
                            }))}
                            dataSource={row.differences.map(item =>
                              Object.fromEntries(
                                Object.entries(item).map(([k, v]) => [
                                  k,
                                  typeof v === 'string' ? v : JSON.stringify(v)
                                ])
                              )
                            )}
                          />
                        )
                    }}
                    columns={[
                      { title: '字段', dataIndex: 'field' },
                      { title: '仅 A', dataIndex: 'inA', width: 90 },
                      { title: '仅 B', dataIndex: 'inB', width: 90 },
                      { title: '两侧都有', dataIndex: 'inBoth', width: 100 },
                      { title: '类型不一致', dataIndex: 'mismatch', width: 100 },
                      { title: '值差异条数', dataIndex: 'diffCount', width: 110 }
                    ]}
                    dataSource={Object.values(overlap.field_comparisons).map(entry => ({
                      field: entry.field_name,
                      inA: entry.in_a_only,
                      inB: entry.in_b_only,
                      inBoth: entry.in_both,
                      mismatch: entry.type_mismatches,
                      diffCount: entry.value_differences.length,
                      differences: entry.value_differences
                    }))}
                  />
                  {/* 两套摘要各自成段，不是一句话的两半：口径不同，合并渲染会把结论混起来 */}
                  <Row gutter={16}>
                    <Col span={12}>
                      <div style={{ whiteSpace: 'pre-line' }}>
                        <strong>质量面摘要</strong>
                        {quality.summary}
                      </div>
                    </Col>
                    <Col span={12}>
                      <div style={{ whiteSpace: 'pre-line' }}>
                        <strong>重叠面摘要</strong>
                        {overlap.summary}
                      </div>
                    </Col>
                  </Row>
                  {overlap.recommendations.length > 0 && (
                    <ul style={{ marginTop: 12 }}>
                      {overlap.recommendations.map(text => (
                        <li key={text}>{text}</li>
                      ))}
                    </ul>
                  )}
                </>
              )
            })()}
          </Card>
        </>
      )}
    </div>
  )
}
