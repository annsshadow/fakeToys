import { useState, useEffect, useCallback } from 'react'
import { Alert, Button, Card, Col, Input, InputNumber, Row, Select, Space, Statistic, Table, Tag, message } from 'antd'
import ReactECharts from 'echarts-for-react'
import {
  analyzeData,
  apiErrorDetail,
  assessImpact,
  cleanData,
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
  EvaluateResponse,
  ImpactResponse,
  ImpactSnapshot,
  LanguageProfileResponse,
  SearchResponse,
  SemanticSearchResponse,
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
                <ReactECharts option={getQuestionTypeChartOption()} style={{ height: 300 }} />
              </Card>
            </Col>
            <Col span={12}>
              <Card>
                <ReactECharts option={getLengthChartOption()} style={{ height: 300 }} />
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
        </>
      )}
    </div>
  )
}
