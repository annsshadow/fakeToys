import { useState } from 'react'
import {
  Alert,
  Card,
  Row,
  Col,
  Statistic,
  Button,
  Space,
  Input,
  InputNumber,
  Select,
  Switch,
  Table,
  Tabs,
  message,
  List,
  Tag
} from 'antd'
import DataList from '../components/DataList'
import QualityChart from '../components/QualityChart'
import {
  apiErrorDetail,
  evaluateQuality,
  dedupData,
  qualityReport,
  cleanData,
  annotateData,
  runBenchmark,
  detectOutliers,
  profileDataset,
  validateDataset,
  healthGate
} from '../services/api'
import type {
  AnnotateResponse,
  BenchmarkResponse,
  CleanResponse,
  DatasetValidationResponse,
  DedupResponse,
  HealthGateResponse,
  OutlierEntry,
  OutliersResponse,
  ProfilingResponse,
  QualityEvaluationResponse,
  QualityReportResponse
} from '../types/api'

/** 离群点检测可选方法（与 `OutlierDetector` 支持的三档一致） */
const OUTLIER_METHODS = [
  { value: 'zscore', label: 'Z-Score（双侧）' },
  { value: 'zscore_one_sided', label: 'Z-Score（仅上侧，适合过长文本）' },
  { value: 'iqr', label: 'IQR（四分位距）' }
]

/** 验证预设三档（与 `DatasetValidator.PRESET_RULES` 的键逐项相等，副标题按该字典的实际规则写） */
const VALIDATION_PRESET_OPTIONS = [
  { value: 'basic', label: 'basic（上限 1000 / 5000）' },
  { value: 'strict', label: 'strict（加长度下限与禁用片段）' },
  { value: 'chat', label: 'chat（多 history 字段，上限更紧）' }
]

/**
 * 健康分四个权重的名字，**顺序**就是 `DatasetHealthScore.__init__` docstring 里写的
 * `[完整性, 多样性, 质量分布均衡性, 覆盖率]`（响应里的 `weights` 也按这个序回显，
 * 页面上把它读回来对一遍就知道有没有错位）。默认值 `[0.25] × 4` 刻意**不抄进来**：
 * 四个框留空就是不传，后端用自己那份默认，两处默认只会有一份真相。
 */
const HEALTH_WEIGHT_LABELS = ['完整性', '多样性', '质量均衡', '覆盖率']

/**
 * `GateVerdict` 的三档（小写）。`warned` 那一档**不能**只看 `gate.passed`：实测
 * `passed` 与「verdict 不是 failed」同义 ⇒ warned 时它仍然是 `true`，
 * 所以「放行」和「干净」在这里是两句话，与结构校验那张卡同一个口径。
 *
 * `failed` 那一档也**不能**只看 `failed_rules`：开了「warning 也阻断」之后，实测
 * 只有 completeness 判负的数据回 `verdict: "failed"` / `passed: false`，而
 * `failed_rules` 是**空的**、判负住在 `warned_rules` 里（block_on_warning 改的是
 * verdict 的合成，不是规则自己的严重级）。
 */
const GATE_VERDICT_COPY: Record<string, { type: 'success' | 'warning' | 'error'; text: string }> = {
  passed: { type: 'success', text: '放行：参与判决的规则全部满足' },
  warned: {
    type: 'warning',
    text: '带警告放行：只有 warning 级规则（completeness）判负，此时 gate.passed 仍然是 true'
  },
  failed: {
    type: 'error',
    text: '阻断：有 error 级规则判负；若开了「warning 也阻断」，warning 级判负也会走到这一档——那时 failed_rules 是空的，判负住在 warned_rules 里'
  }
}

/**
 * 质量中心
 *
 * 聚合质量评估、去重、清洗、标注、基准、离群点检测、数据画像、结构校验与健康门禁九项能力，
 * 对应需求 R5 / R6 / R7 / R12 / R13 / R28。
 *
 * 后四项（离群点 / 画像 / 结构校验 / 健康门禁）与前五项的区别是：前五项的结论并进指标卡，
 * 后四项只输出「这批数据长什么样、能不能放行」，所以结果单独用 Tab 呈现。
 */
export default function Quality() {
  const [file, setFile] = useState('')
  const [threshold, setThreshold] = useState(0.6)
  const [outlierMethod, setOutlierMethod] = useState('zscore')
  const [outlierThreshold, setOutlierThreshold] = useState(3)
  const [loading, setLoading] = useState(false)
  const [evaluation, setEvaluation] = useState<QualityEvaluationResponse | null>(null)
  const [dedup, setDedup] = useState<DedupResponse | null>(null)
  const [report, setReport] = useState<QualityReportResponse | null>(null)
  const [cleaning, setCleaning] = useState<CleanResponse | null>(null)
  const [annotation, setAnnotation] = useState<AnnotateResponse | null>(null)
  const [benchmark, setBenchmark] = useState<BenchmarkResponse | null>(null)
  const [outliers, setOutliers] = useState<OutliersResponse | null>(null)
  const [profiling, setProfiling] = useState<ProfilingResponse | null>(null)
  const [validation, setValidation] = useState<DatasetValidationResponse | null>(null)
  // 初值与契约默认同值（`basic`）⇒ 与「不传 preset」是同一档，页面上没有第二套默认在暗地里生效
  const [validationPreset, setValidationPreset] = useState('basic')
  // 下面五个初值逐项等于 `HealthGateRequest` 的字段默认（'instruction' / 0.6 / 0.3 / 0.8 / false）
  const [gateTextField, setGateTextField] = useState('instruction')
  const [gatePassRateMin, setGatePassRateMin] = useState(0.6)
  const [gateDuplicateMax, setGateDuplicateMax] = useState(0.3)
  const [gateCompletenessMin, setGateCompletenessMin] = useState(0.8)
  const [gateBlockOnWarning, setGateBlockOnWarning] = useState(false)
  // pass_rate 留空 = 不传，后端把该规则摘掉并写进 `skipped_rules`（不是判负）
  const [gatePassRate, setGatePassRate] = useState<number | null>(null)
  const [gateWeights, setGateWeights] = useState<(number | null)[]>([null, null, null, null])
  const [gate, setGate] = useState<HealthGateResponse | null>(null)

  const gateWeightsFilled = gateWeights.filter((w): w is number => typeof w === 'number')
  // 权重「要么四个全填、要么全留空」：只填一两个后端会 400「权重必须包含 4 个元素」。
  // 那是真判决，但在这里属于可预防的输入错误 ⇒ 禁掉按钮并把原因写在旁边，
  // 而不是让用户点一下再收一句 400。
  const gateWeightsPartial =
    gateWeightsFilled.length > 0 && gateWeightsFilled.length < HEALTH_WEIGHT_LABELS.length

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
    } catch (err) {
      message.error(apiErrorDetail(err, failText))
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

  // 离群点条目自带 `index`（它在数据集里的位置），可直接当行 key
  const outlierRows = (outliers?.outliers || []).map((entry: OutlierEntry) => ({
    index: entry.index,
    value: entry.value,
    instruction: entry.item.instruction
  }))

  // `field_completeness` 是「字段名 → 非空率」的扁平字典，展开成两列即可
  const completenessRows = Object.entries(profiling?.field_completeness || {}).map(
    ([field, rate]) => ({ field, rate })
  )

  const keywordRows = profiling?.top_keywords || []

  // `issues` 是 SDK 收窄过的**样本**（`ValidationResult.to_dict()` 只取前 100 条），
  // 而 `error_count` / `warning_count` 按全量算 ⇒ 上面的计数可以大于下面的行数，
  // 那不是 bug。行 key 用样本内序号：同一条记录上同一字段报两次是合法的，
  // 拿 `index` 当 key 会撞。
  const validationRows = (validation?.issues || []).map((issue, i) => ({
    key: i,
    severity: issue.severity,
    field: issue.field,
    index: issue.index,
    message: issue.message
  }))

  // `health.metrics` 是四项固定指标（完整性 / 多样性 / 质量均衡 / 覆盖率）；`gate.metrics`
  // 是门禁**实际看到**的那一份（多出 `duplicate_rate`，传了 pass_rate 时还有它）。
  // 两份并排列出来才复核得了 verdict 是怎么来的——只列健康分那四项，
  // 「重复率 1.0 所以判负」这件事在页面上根本看不见。
  const gateMetricRows = gate
    ? [
        ...Object.entries(gate.health.metrics).map(([metric, value]) => ({
          key: `health.${metric}`,
          source: '健康分',
          metric,
          value
        })),
        ...Object.entries(gate.gate.metrics).map(([metric, value]) => ({
          key: `gate.${metric}`,
          source: '门禁',
          metric,
          value
        }))
      ]
    : []

  // 四个名单是四种不同的坏，不能合并成一个「问题规则」列表：
  // failed = 算出来了不达标（error 级）；warned = 不达标但只警告（默认不阻断，
  // 开了「warning 也阻断」后 verdict 会变成 failed 而这一格仍有名字）；
  // errored = 规则压根算不出来（`evaluate` 抛异常，这一只端点上实测恒空，见下方文案）；
  // skipped = 缺指标所以没参与判决（pass_rate 没传时它住在这里，不是 failed）。
  const gateRuleRows = gate
    ? [
        { key: 'failed', label: '判负（error 级，阻断）', names: gate.gate.failed_rules, color: 'red' },
        { key: 'warned', label: '警告（warning 级）', names: gate.gate.warned_rules, color: 'orange' },
        { key: 'errored', label: '算不出来（errored）', names: gate.gate.errored_rules, color: 'volcano' },
        { key: 'skipped', label: '没参与判决（skipped）', names: gate.skipped_rules, color: 'blue' }
      ]
    : []

  const gateVerdictCopy = gate
    ? (GATE_VERDICT_COPY[gate.gate.verdict] ?? {
        type: 'error' as const,
        text: `未知判定 ${gate.gate.verdict}（后端只有 passed / warned / failed 三档）`
      })
    : null

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
        <Space wrap style={{ marginTop: 12 }}>
          <span>离群点方法</span>
          <Select
            value={outlierMethod}
            onChange={setOutlierMethod}
            options={OUTLIER_METHODS}
            style={{ width: 240 }}
          />
          <span>离群阈值</span>
          <InputNumber
            min={0.1}
            step={0.5}
            value={outlierThreshold}
            onChange={value => setOutlierThreshold(Number(value ?? 3))}
          />
          <Button
            loading={loading}
            onClick={() =>
              run(
                () => detectOutliers(file, outlierMethod, outlierThreshold),
                setOutliers,
                '离群点检测失败'
              )
            }
          >
            离群点检测
          </Button>
          <Button
            loading={loading}
            onClick={() => run(() => profileDataset(file), setProfiling, '数据画像失败')}
          >
            数据画像
          </Button>
          <span>验证预设</span>
          <Select
            value={validationPreset}
            onChange={setValidationPreset}
            options={VALIDATION_PRESET_OPTIONS}
            style={{ width: 260 }}
          />
          <Button
            loading={loading}
            onClick={() =>
              run(() => validateDataset(file, validationPreset), setValidation, '结构校验失败')
            }
          >
            结构校验
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

      <Card title="健康门禁" style={{ marginBottom: 16 }}>
        <Alert
          style={{ marginBottom: 12 }}
          type="info"
          showIcon
          message="这份数据能不能进下游训练：先打健康分，再用三条默认规则判定"
          description="三条规则是 pass_rate ≥ 阈值（error）、duplicate_rate ≤ 阈值（error）、completeness ≥ 阈值（warning）。健康分本身不参与门禁判决——它只回答「这批数据整体好不好」，verdict 只由上面三条规则算出来，所以「等级 healthy」与「门禁 failed」可以同时为真。"
        />
        <Space wrap>
          <span>文本字段</span>
          <Input
            aria-label="门禁文本字段"
            value={gateTextField}
            onChange={e => setGateTextField(e.target.value)}
            style={{ width: 160 }}
          />
          <span>通过率</span>
          <InputNumber
            aria-label="门禁通过率"
            min={0}
            max={1}
            step={0.05}
            value={gatePassRate}
            onChange={value => setGatePassRate(value)}
            placeholder="留空＝不判这一项"
            style={{ width: 180 }}
          />
          <Button
            disabled={!evaluation}
            onClick={() => evaluation && setGatePassRate(evaluation.pass_rate)}
          >
            用上面那次评估的通过率填入
          </Button>
          <Button
            type="primary"
            loading={loading}
            disabled={gateWeightsPartial}
            onClick={() =>
              run(
                () =>
                  healthGate(file, {
                    textField: gateTextField,
                    weights:
                      gateWeightsFilled.length === HEALTH_WEIGHT_LABELS.length
                        ? gateWeightsFilled
                        : undefined,
                    passRate: gatePassRate ?? undefined,
                    passRateMin: gatePassRateMin,
                    duplicateRateMax: gateDuplicateMax,
                    completenessMin: gateCompletenessMin,
                    blockOnWarning: gateBlockOnWarning
                  }),
                setGate,
                '健康门禁失败'
              )
            }
          >
            健康门禁
          </Button>
        </Space>
        {/* 三个阈值框刻意**不设 min/max**：后端这一只端点对它们没有任何范围闸
            （实测 pass_rate_min=-1.0 / 5.0、duplicate_rate_max=99.0、completeness_min=-3.0
            全部 200，判决随阈值单调变化）。前端夹一个范围就等于替后端发明一条它没有的规则，
            还会把「我就想看极端阈值下的判决」这件事挡掉。只有 pass_rate 那一格有 [0,1] 的闸
            （越界是 400「pass_rate 必须在 [0, 1] 区间内」），所以只有它设了 min/max。 */}
        <Space wrap style={{ marginTop: 12 }}>
          <span>pass_rate ≥</span>
          <InputNumber
            aria-label="pass_rate 下限"
            step={0.05}
            value={gatePassRateMin}
            onChange={value => setGatePassRateMin(Number(value ?? 0.6))}
          />
          <span>duplicate_rate ≤</span>
          <InputNumber
            aria-label="duplicate_rate 上限"
            step={0.05}
            value={gateDuplicateMax}
            onChange={value => setGateDuplicateMax(Number(value ?? 0.3))}
          />
          <span>completeness ≥</span>
          <InputNumber
            aria-label="completeness 下限"
            step={0.05}
            value={gateCompletenessMin}
            onChange={value => setGateCompletenessMin(Number(value ?? 0.8))}
          />
          <span>warning 也阻断</span>
          <Switch
            aria-label="warning 也阻断"
            checked={gateBlockOnWarning}
            onChange={setGateBlockOnWarning}
          />
        </Space>
        <Space wrap style={{ marginTop: 12 }}>
          <span>健康分权重</span>
          {HEALTH_WEIGHT_LABELS.map((label, i) => (
            <span key={label}>
              {label}
              <InputNumber
                aria-label={`健康分权重 ${label}`}
                min={0} max={1} step={0.05}
                value={gateWeights[i]}
                onChange={value =>
                  setGateWeights(prev => prev.map((w, j) => (j === i ? value : w)))
                }
                placeholder="留空＝后端默认"
                style={{ width: 140, marginLeft: 4 }}
              />
            </span>
          ))}
        </Space>
        {gateWeightsPartial && (
          <Alert
            style={{ marginTop: 12 }}
            type="warning"
            showIcon
            message={`权重已填 ${gateWeightsFilled.length} / ${HEALTH_WEIGHT_LABELS.length} 个`}
            description="四个权重要么全填、要么全留空（留空就用后端默认）。全填时个数必须是 4、之和必须是 1.0，否则后端回 400。"
          />
        )}
      </Card>

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
          },
          {
            key: 'outliers',
            label: '离群点检测',
            children: outliers ? (
              <>
                <Row gutter={16} style={{ marginBottom: 16 }}>
                  <Col span={8}>
                    <Card>
                      <Statistic title="样本总数" value={outliers.total_items} />
                    </Card>
                  </Col>
                  <Col span={8}>
                    <Card>
                      <Statistic title="离群样本" value={outliers.outlier_count} />
                    </Card>
                  </Col>
                  <Col span={8}>
                    <Card>
                      <Statistic
                        title="离群比例"
                        value={(outliers.outlier_rate * 100).toFixed(2)}
                        suffix="%"
                      />
                    </Card>
                  </Col>
                </Row>
                <Card
                  title={`离群样本（${outliers.method} · 阈值 ${outliers.threshold} · 字段 ${outliers.field}）`}
                >
                  <Table
                    size="small"
                    rowKey="index"
                    columns={[
                      { title: '序号', dataIndex: 'index', width: 80 },
                      { title: '长度', dataIndex: 'value', width: 100 },
                      { title: '内容', dataIndex: 'instruction', ellipsis: true }
                    ]}
                    dataSource={outlierRows}
                  />
                </Card>
              </>
            ) : (
              <Card>点击「离群点检测」找出长度异常的样本</Card>
            )
          },
          {
            key: 'profiling',
            label: '数据画像',
            children: profiling ? (
              <>
                <Row gutter={16} style={{ marginBottom: 16 }}>
                  <Col span={8}>
                    <Card>
                      <Statistic title="样本总数" value={profiling.total_items} />
                    </Card>
                  </Col>
                  <Col span={8}>
                    <Card>
                      <Statistic
                        title="重复率"
                        value={(profiling.duplicate_rate * 100).toFixed(2)}
                        suffix="%"
                      />
                    </Card>
                  </Col>
                  <Col span={8}>
                    <Card>
                      {/* 空数据集时 length_stats 是 {}，所以这里逐项兜底成 "-" */}
                      <Statistic
                        title="平均长度"
                        value={profiling.length_stats.avg?.toFixed(1) ?? '-'}
                      />
                    </Card>
                  </Col>
                </Row>
                <Row gutter={16}>
                  <Col span={8}>
                    <Card title="字段完整度">
                      <Table
                        size="small"
                        rowKey="field"
                        pagination={false}
                        columns={[
                          { title: '字段', dataIndex: 'field' },
                          {
                            title: '非空率',
                            dataIndex: 'rate',
                            render: (value: number) => `${(value * 100).toFixed(1)}%`
                          }
                        ]}
                        dataSource={completenessRows}
                      />
                    </Card>
                  </Col>
                  <Col span={8}>
                    <Card title="长度分布">
                      <Space direction="vertical">
                        <span>最短：{profiling.length_stats.min ?? '-'}</span>
                        <span>最长：{profiling.length_stats.max ?? '-'}</span>
                        <span>平均：{profiling.length_stats.avg?.toFixed(1) ?? '-'}</span>
                        <span>中位：{profiling.length_stats.median ?? '-'}</span>
                      </Space>
                      <div style={{ marginTop: 12 }}>
                        语言分布：
                        {Object.entries(profiling.language_distribution || {}).map(
                          ([lang, count]) => (
                            <Tag key={lang} color="purple">
                              {lang}: {String(count)}
                            </Tag>
                          )
                        )}
                      </div>
                    </Card>
                  </Col>
                  <Col span={8}>
                    <Card title="高频关键词">
                      <Table
                        size="small"
                        rowKey="keyword"
                        pagination={{ pageSize: 8 }}
                        columns={[
                          { title: '关键词', dataIndex: 'keyword' },
                          { title: '频次', dataIndex: 'count', width: 80 }
                        ]}
                        dataSource={keywordRows}
                      />
                    </Card>
                  </Col>
                </Row>
              </>
            ) : (
              <Card>点击「数据画像」查看字段完整度、长度分布与高频关键词</Card>
            )
          },
          {
            key: 'validation',
            label: '结构校验',
            children: validation ? (
              <>
                <Row gutter={16} style={{ marginBottom: 16 }}>
                  <Col span={6}>
                    <Card>
                      <Statistic title="条目总数" value={validation.total_items} />
                    </Card>
                  </Col>
                  <Col span={6}>
                    <Card>
                      <Statistic title="合格条目" value={validation.valid_items} />
                    </Card>
                  </Col>
                  <Col span={6}>
                    <Card>
                      <Statistic title="错误" value={validation.error_count} />
                    </Card>
                  </Col>
                  <Col span={6}>
                    <Card>
                      <Statistic title="警告" value={validation.warning_count} />
                    </Card>
                  </Col>
                </Row>
                {/* `is_valid` 只判「有没有 error」，警告不会把它翻成 false
                    （实测 strict 预设下 warning_count=3 而 is_valid 仍为 true）⇒
                    「通过」和「干净」是两句话，这里分开说。 */}
                <Alert
                  style={{ marginBottom: 16 }}
                  showIcon
                  type={
                    validation.is_valid
                      ? validation.warning_count > 0
                        ? 'warning'
                        : 'success'
                      : 'error'
                  }
                  message={
                    validation.is_valid
                      ? validation.warning_count > 0
                        ? `没有错误，但有 ${validation.warning_count} 条警告`
                        : '全部条目通过校验'
                      : `校验未通过：${validation.error_count} 条错误`
                  }
                  description={
                    validation.is_valid && validation.warning_count > 0
                      ? `合格条目 ${validation.valid_items} / ${validation.total_items}：警告不改变 is_valid，只标出不合建议的条目`
                      : undefined
                  }
                />
                {validationRows.length > 0 ? (
                  <Card
                    title={`问题清单（列出 ${validationRows.length} 条；SDK 单次最多回 100 条样本，计数按全量算）`}
                  >
                    <Table
                      size="small"
                      pagination={{ pageSize: 10 }}
                      rowKey="key"
                      columns={[
                        {
                          title: '级别',
                          dataIndex: 'severity',
                          width: 90,
                          render: (severity: string) => (
                            <Tag color={severity === 'error' ? 'red' : 'orange'}>{severity}</Tag>
                          )
                        },
                        { title: '字段', dataIndex: 'field', width: 140 },
                        {
                          title: '条目下标',
                          dataIndex: 'index',
                          width: 90,
                          render: (index: number | null) => (index === null ? '-' : index)
                        },
                        { title: '说明', dataIndex: 'message' }
                      ]}
                      dataSource={validationRows}
                    />
                  </Card>
                ) : (
                  <Alert type="success" showIcon message="这份文件没有报出任何问题" />
                )}
              </>
            ) : (
              <Card>点击「结构校验」按预设规则检查必填字段、长度上下限与禁用片段</Card>
            )
          },
          {
            key: 'healthgate',
            label: '健康门禁',
            children: gate && gateVerdictCopy ? (
              <div aria-label="健康门禁回执">
                <Alert
                  style={{ marginBottom: 16 }}
                  showIcon
                  type={gateVerdictCopy.type}
                  message={`门禁判定：${gate.gate.verdict}（passed = ${String(gate.gate.passed)}）`}
                  description={`${gateVerdictCopy.text}。gate.passed 与「verdict 不是 failed」同义：verdict 是 warned 时它仍然是 true，所以别把它当「干净」读。`}
                />
                <Row gutter={16} style={{ marginBottom: 16 }}>
                  <Col span={6}>
                    <Card>
                      <Statistic title="健康分" value={gate.health.health_score.toFixed(3)} />
                    </Card>
                  </Col>
                  <Col span={6}>
                    <Card>
                      <Statistic
                        title="等级（≥0.7 healthy / ≥0.4 moderate / 否则 poor）"
                        value={gate.health.level}
                      />
                    </Card>
                  </Col>
                  <Col span={6}>
                    <Card>
                      <Statistic title="样本总数" value={gate.health.total_samples} />
                    </Card>
                  </Col>
                  <Col span={6}>
                    <Card>
                      <Statistic
                        title="生效权重（完整性 / 多样性 / 质量均衡 / 覆盖率）"
                        value={gate.health.weights.join(' / ')}
                      />
                    </Card>
                  </Col>
                </Row>
                <Card
                  title="指标：健康分那四项 + 门禁实际看到的那一份"
                  style={{ marginBottom: 16 }}
                >
                  <Table
                    size="small"
                    pagination={false}
                    rowKey="key"
                    columns={[
                      { title: '来源', dataIndex: 'source', width: 100 },
                      { title: '指标', dataIndex: 'metric', width: 200 },
                      {
                        title: '取值',
                        dataIndex: 'value',
                        render: (value: number) => value.toFixed(4)
                      }
                    ]}
                    dataSource={gateMetricRows}
                  />
                </Card>
                <Card title="规则名单（四个名单是四种不同的坏，不合并）">
                  <List
                    size="small"
                    bordered
                    dataSource={gateRuleRows}
                    renderItem={row => (
                      <List.Item>
                        <Space wrap>
                          <Tag color={row.color}>{row.label}</Tag>
                          {row.names.length > 0 ? (
                            row.names.map(name => <Tag key={name}>{name}</Tag>)
                          ) : (
                            <span>（无）</span>
                          )}
                        </Space>
                      </List.Item>
                    )}
                  />
                </Card>
                <Alert
                  style={{ marginTop: 16 }}
                  type="info"
                  showIcon
                  message="读这一页之前要知道的六件事"
                  description={
                    <ul style={{ margin: 0, paddingLeft: 18 }}>
                      <li>
                        「skipped_rules」里出现 pass_rate 表示这一项没参与判决（通过率留空时），
                        不是判负。端点这么做是因为 GateRule.evaluate() 对缺失的指标键返回
                        False——不摘掉这条规则，「没算这一项」就会被伪装成「算出来不及格」。
                      </li>
                      <li>
                        文本字段填了数据集里不存在的键、空串或纯空白，后端一律 400，detail 里
                        带着首条实际字段（L238 之前这一档**不报错**：所有条目都取到空值，
                        实测 duplicate_rate 变成 1.0，门禁于是在 duplicate_rate 那条上判负）。
                        看到 dup = 1.0 仍然先怀疑字段名，只是它现在不会静默地发生了。
                      </li>
                      <li>
                        权重的判据与评分器共引同一份（L238 起）：个数必须是 4、每项必须是
                        [0, 1] 内的数值（负数与大于 1 一律 400，并点名到 weights 的下标）、
                        四项之和落在 1.0 ± 0.01 **含端点**（0.99 / 1.005 / 1.01 都放行，
                        0.98 与 1.02 才拒，400 的 detail 带着实测和）。health_score 的出口
                        钳进 [0, 1]：改前 [2.0, -0.5, -0.25, -0.25] 能顶到 1.17（超过 1），
                        今天那一档是 400。但门禁三条规则一条都不看权重 ⇒ 「健康分 healthy」
                        与「门禁阻断」仍然可以同时为真，这不是矛盾，是两套判据各说各话。
                      </li>
                      <li>
                        开了「warning 也阻断」之后，只有 completeness 判负的数据会回
                        verdict = failed / passed = false，而 failed_rules 是空的、名字住在
                        warned_rules 里——block_on_warning 改的是 verdict 的合成，不是规则的严重级。
                      </li>
                      <li>
                        三个阈值没有范围闸：负数与大于 1 的值后端都收（实测 pass_rate_min = -1.0
                        让 pass_rate = 0 也算满足、= 5.0 让 pass_rate = 1.0 也判负、
                        duplicate_rate_max = 99.0 直接把 verdict 变成 passed），判决随阈值单调变化。
                        只有 pass_rate 那一格有 [0, 1] 的闸（越界是 400）。
                      </li>
                      <li>
                        「errored_rules」（规则压根算不出来）在这一只端点上实测恒空，列出来是为了
                        它哪天不空时看得见——它与 failed_rules 是两种不同的坏。
                      </li>
                    </ul>
                  }
                />
              </div>
            ) : (
              <Card>
                在上方填好文本字段、阈值与权重（通过率可留空，或点「用上面那次评估的通过率填入」），
                点「健康门禁」查看健康分、verdict 与四个规则名单
              </Card>
            )
          }
        ]}
      />
    </div>
  )
}
