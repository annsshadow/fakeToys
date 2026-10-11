import { useState, useEffect, useCallback } from 'react'
import { Alert, Button, Card, Col, Row, Select, Space, Statistic, Table, Tag, message } from 'antd'
import ReactECharts from 'echarts-for-react'
import {
  analyzeData,
  apiErrorDetail,
  cleanData,
  evaluateDataset,
  getDataFiles,
  languageProfile,
  runBenchmark,
  visualizeData,
} from '../services/api'
import type {
  AnalyzeResponse,
  BenchmarkResponse,
  CleanResponse,
  EvaluateResponse,
  LanguageProfileResponse,
} from '../types/api'

export default function Analysis() {
  const [files, setFiles] = useState<string[]>([])
  const [selectedFile, setSelectedFile] = useState('')
  const [referenceFile, setReferenceFile] = useState('')
  const [analysis, setAnalysis] = useState<AnalyzeResponse | null>(null)
  const [cleaning, setCleaning] = useState<CleanResponse | null>(null)
  const [benchmark, setBenchmark] = useState<BenchmarkResponse | null>(null)
  const [profile, setProfile] = useState<LanguageProfileResponse | null>(null)
  const [evaluation, setEvaluation] = useState<EvaluateResponse | null>(null)
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
        </>
      )}
    </div>
  )
}
