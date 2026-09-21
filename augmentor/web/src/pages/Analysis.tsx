import { useState, useEffect, useCallback } from 'react'
import { Card, Select, Row, Col, Statistic, Table, Tag, Button, Space, message } from 'antd'
import ReactECharts from 'echarts-for-react'
import { getDataFiles, analyzeData, cleanData, runBenchmark } from '../services/api'

export default function Analysis() {
  const [files, setFiles] = useState<string[]>([])
  const [selectedFile, setSelectedFile] = useState('')
  const [analysis, setAnalysis] = useState<any>(null)
  const [cleaning, setCleaning] = useState<any>(null)
  const [benchmark, setBenchmark] = useState<any>(null)
  const [loading, setLoading] = useState(false)

  const loadFiles = useCallback(async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map((f: any) => f.name))
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
    } catch {
      message.error('数据清洗失败')
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
    } catch {
      message.error('基准测试失败')
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
        </>
      )}
    </div>
  )
}
