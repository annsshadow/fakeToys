import { useState, useEffect } from 'react'
import { Card, Select, Row, Col, Statistic, Table, Tag } from 'antd'
import ReactECharts from 'echarts-for-react'
import { getDataFiles, analyzeData } from '../services/api'

export default function Analysis() {
  const [files, setFiles] = useState<string[]>([])
  const [selectedFile, setSelectedFile] = useState('')
  const [analysis, setAnalysis] = useState<any>(null)

  useEffect(() => {
    loadFiles()
  }, [])

  useEffect(() => {
    if (selectedFile) {
      loadAnalysis()
    }
  }, [selectedFile])

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map((f: any) => f.name))
    } catch (error) {
      console.error('加载文件列表失败')
    }
  }

  const loadAnalysis = async () => {
    try {
      const result = await analyzeData(selectedFile)
      setAnalysis(result)
    } catch (error) {
      console.error('分析数据失败')
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

  const getWordCloudOption = () => {
    if (!analysis?.coverage_analysis?.topic_distribution?.top_words) return {}
    
    const words = analysis.coverage_analysis.topic_distribution.top_words
    return {
      title: { text: '关键词云', left: 'center' },
      tooltip: {},
      series: [{
        type: 'wordCloud',
        shape: 'circle',
        sizeRange: [12, 60],
        rotationRange: [-90, 90],
        data: Object.entries(words).map(([name, value]) => ({
          name,
          value
        }))
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
        </>
      )}
    </div>
  )
}
