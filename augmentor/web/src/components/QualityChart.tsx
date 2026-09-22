import { Card, Row, Col, Empty } from 'antd'
import ReactECharts from 'echarts-for-react'
import type { QualityReportResponse } from '../types/api'

interface QualityChartProps {
  /** 质量报告数据（`ReportGenerator.to_dict()` 的输出） */
  report: QualityReportResponse
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

  // `score_histogram` 是「桶名 → 条数」的字典（见 `augmentor/report.py: _build_charts`），
  // **不是** `{ labels, values }`。原来写成 `histogram.values.map(...)`，
  // `histogram.values` 恒为 undefined，一点「生成报告」就抛
  // `TypeError: Cannot read properties of undefined (reading 'map')`，
  // 把整个质量中心页打成白屏。同理 y 轴是条数，不是占比。
  const histogram = report.charts.score_histogram || {}
  const histogramLabels = Object.keys(histogram)
  const histogramValues = Object.values(histogram)

  const radar = report.charts.metric_radar || { labels: [], values: [] }

  const histogramOption = {
    title: { text: '评分分布', left: 'center' },
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'category', data: histogramLabels },
    yAxis: { type: 'value', name: '样本数' },
    series: [
      {
        type: 'bar',
        data: histogramValues,
        itemStyle: { color: '#1677ff' }
      }
    ]
  }

  const radarOption = {
    title: { text: '指标均值', left: 'center' },
    tooltip: {},
    radar: {
      indicator: radar.labels.map(label => ({ name: label, max: 1 }))
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
