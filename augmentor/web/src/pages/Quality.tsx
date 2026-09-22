import { useState } from 'react'
import {
  Card,
  Row,
  Col,
  Statistic,
  Button,
  Space,
  InputNumber,
  Table,
  Tabs,
  message,
  List,
  Tag
} from 'antd'
import DataList from '../components/DataList'
import QualityChart from '../components/QualityChart'
import {
  evaluateQuality,
  dedupData,
  qualityReport,
  cleanData,
  annotateData,
  runBenchmark
} from '../services/api'
import type {
  AnnotateResponse,
  BenchmarkResponse,
  CleanResponse,
  DedupResponse,
  QualityEvaluationResponse,
  QualityReportResponse
} from '../types/api'

/**
 * 质量中心
 *
 * 聚合质量评估、去重、清洗、标注与基准五项能力，
 * 对应需求 R5 / R6 / R7 / R12 / R13 / R28。
 */
export default function Quality() {
  const [file, setFile] = useState('')
  const [threshold, setThreshold] = useState(0.6)
  const [loading, setLoading] = useState(false)
  const [evaluation, setEvaluation] = useState<QualityEvaluationResponse | null>(null)
  const [dedup, setDedup] = useState<DedupResponse | null>(null)
  const [report, setReport] = useState<QualityReportResponse | null>(null)
  const [cleaning, setCleaning] = useState<CleanResponse | null>(null)
  const [annotation, setAnnotation] = useState<AnnotateResponse | null>(null)
  const [benchmark, setBenchmark] = useState<BenchmarkResponse | null>(null)

  /**
   * 跑一个质量任务并把结果交给对应的 setState
   *
   * 泛型是为了让 `onDone` 与 `task` 的结果类型绑在一起：否则 `onDone` 只能是
   * `(data: any) => void`，而 `src/pages` 层是禁止写 `any` 的。
   */
  const run = async <T,>(
    task: () => Promise<T>,
    onDone: (data: T) => void,
    failText: string
  ) => {
    if (!file) {
      message.warning('请先选择数据集')
      return
    }
    setLoading(true)
    try {
      onDone(await task())
    } catch {
      message.error(failText)
    } finally {
      setLoading(false)
    }
  }

  const suggestionList = (
    <List
      size="small"
      bordered
      dataSource={report?.improvement_suggestions || []}
      renderItem={(item: string) => <List.Item>{item}</List.Item>}
    />
  )

  const intentData = Object.entries(annotation?.intent_distribution || {}).map(
    ([intent, count]) => ({ intent, count })
  )

  const sentimentData = Object.entries(annotation?.sentiment_distribution || {}).map(
    ([sentiment, count]) => ({ sentiment, count })
  )

  const benchmarkMetrics = Object.entries(benchmark?.metrics || {}).map(
    ([metric, value]) => ({
      metric,
      value: typeof value === 'number' ? value.toFixed(4) : String(value)
    })
  )

  return (
    <div>
      <DataList value={file} onChange={setFile} previewSize={3} />

      <Card style={{ marginBottom: 16 }}>
        <Space wrap>
          <span>质量阈值</span>
          <InputNumber
            min={0}
            max={1}
            step={0.05}
            value={threshold}
            onChange={value => setThreshold(Number(value ?? 0.6))}
          />
          <Button
            type="primary"
            loading={loading}
            onClick={() => run(() => evaluateQuality(file, threshold), setEvaluation, '质量评估失败')}
          >
            质量评估
          </Button>
          <Button
            loading={loading}
            onClick={() => run(() => dedupData(file), setDedup, '去重分析失败')}
          >
            智能去重
          </Button>
          <Button
            loading={loading}
            onClick={() => run(() => qualityReport(file, threshold), setReport, '生成报告失败')}
          >
            生成报告
          </Button>
          <Button
            loading={loading}
            onClick={() => run(() => cleanData(file), setCleaning, '数据清洗失败')}
          >
            数据清洗
          </Button>
          <Button
            loading={loading}
            onClick={() => run(() => annotateData(file), setAnnotation, '自动标注失败')}
          >
            自动标注
          </Button>
          <Button
            loading={loading}
            onClick={() => run(() => runBenchmark(file, threshold), setBenchmark, '基准测试失败')}
          >
            质量基准
          </Button>
        </Space>
      </Card>

      {evaluation && (
        <Row gutter={16} style={{ marginBottom: 16 }}>
          <Col span={6}>
            <Card>
              <Statistic title="样本总数" value={evaluation.total_samples} />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic title="通过样本" value={evaluation.passed_samples} />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic title="过滤样本" value={evaluation.filtered_samples} />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="通过率"
                value={(evaluation.pass_rate * 100).toFixed(2)}
                suffix="%"
              />
            </Card>
          </Col>
        </Row>
      )}

      <Tabs
        items={[
          {
            key: 'report',
            label: '质量报告',
            children: report ? (
              <>
                <QualityChart report={report} />
                <Card title="改进建议" style={{ marginTop: 16 }}>
                  {suggestionList}
                </Card>
              </>
            ) : (
              <Card>点击「生成报告」查看评分分布与改进建议</Card>
            )
          },
          {
            key: 'dedup',
            label: '去重结果',
            children: dedup ? (
              <Card>
                <Space size="large" wrap>
                  <Tag color="blue">原始 {dedup.original_count}</Tag>
                  <Tag color="green">去重后 {dedup.deduplicated_count}</Tag>
                  <Tag color="red">移除 {dedup.removed_count}</Tag>
                  <Tag color="orange">重复组 {dedup.duplicate_groups}</Tag>
                </Space>
              </Card>
            ) : (
              <Card>点击「智能去重」查看重复统计</Card>
            )
          },
          {
            key: 'clean',
            label: '清洗结果',
            children: cleaning ? (
              <Card>
                <Space size="large" wrap>
                  <Tag color="blue">原始 {cleaning.original_count}</Tag>
                  <Tag color="green">清洗后 {cleaning.cleaned_count}</Tag>
                  <Tag color="red">丢弃 {cleaning.dropped_count}</Tag>
                  <Tag color="orange">变更 {cleaning.changed_count}</Tag>
                </Space>
                <div style={{ marginTop: 12 }}>
                  语言分布：
                  {Object.entries(cleaning.language_distribution || {}).map(
                    ([lang, count]) => (
                      <Tag key={lang} color="purple">
                        {lang}: {String(count)}
                      </Tag>
                    )
                  )}
                </div>
              </Card>
            ) : (
              <Card>点击「数据清洗」查看去噪与标准化结果</Card>
            )
          },
          {
            key: 'annotate',
            label: '标注结果',
            children: annotation ? (
              <Row gutter={16}>
                <Col span={12}>
                  <Card title="意图分布">
                    <Table
                      size="small"
                      rowKey="intent"
                      pagination={false}
                      columns={[
                        { title: '意图', dataIndex: 'intent' },
                        { title: '数量', dataIndex: 'count' }
                      ]}
                      dataSource={intentData}
                    />
                  </Card>
                </Col>
                <Col span={12}>
                  <Card title="情感分布">
                    <Table
                      size="small"
                      rowKey="sentiment"
                      pagination={false}
                      columns={[
                        { title: '情感', dataIndex: 'sentiment' },
                        { title: '数量', dataIndex: 'count' }
                      ]}
                      dataSource={sentimentData}
                    />
                  </Card>
                </Col>
              </Row>
            ) : (
              <Card>点击「自动标注」查看实体、意图与情感分布</Card>
            )
          },
          {
            key: 'benchmark',
            label: '质量基准',
            children: benchmark ? (
              <Card>
                <Table
                  size="small"
                  rowKey="metric"
                  pagination={false}
                  columns={[
                    { title: '指标', dataIndex: 'metric' },
                    { title: '数值', dataIndex: 'value' }
                  ]}
                  dataSource={benchmarkMetrics}
                />
              </Card>
            ) : (
              <Card>点击「质量基准」运行标准化评估指标</Card>
            )
          }
        ]}
      />
    </div>
  )
}
