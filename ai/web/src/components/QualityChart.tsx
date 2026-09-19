import { Card, Row, Col, Empty } from 'antd'
import ReactECharts from 'echarts-for-react'

interface QualityChartProps {
  /** 质量报告数据（ReportGenerator.to_dict() 的输出） */
  report: any
}

/**
 * 质量报告图表
 *
 * 把后端的 score_histogram / metric_radar 结构直接映射为图表，
 * 避免各页面重复拼装 option。
 */
export default function QualityChart({ report }: QualityChartProps) {
  if (!report?.charts) {
    return (
      <Card>
        <Empty description="暂无质量报告数据" />
      </Card>
    )
  }

  const histogram = report.charts.score_histogram || { labels: [], values: [] }
  const radar = report.charts.metric_radar || { labels: [], values: [] }

  const histogramOption = {
    title: { text: '评分分布', left: 'center' },
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'category', data: histogram.labels },
    yAxis: { type: 'value', name: '占比' },
    series: [
      {
        type: 'bar',
        data: histogram.values.map((v: number) => Number((v * 100).toFixed(2))),
        itemStyle: { color: '#1677ff' }
      }
    ]
  }

  const radarOption = {
    title: { text: '指标均值', left: 'center' },
    tooltip: {},
    radar: {
      indicator: radar.labels.map((label: string) => ({ name: label, max: 1 }))
    },
    series: [
      {
        type: 'radar',
        data: [{ value: radar.values, name: '均值' }]
      }
    ]
  }

  return (
    <Row gutter={16}>
      <Col span={12}>
        <Card>
          <ReactECharts option={histogramOption} style={{ height: 320 }} />
        </Card>
      </Col>
      <Col span={12}>
        <Card>
          <ReactECharts option={radarOption} style={{ height: 320 }} />
        </Card>
      </Col>
    </Row>
  )
}
