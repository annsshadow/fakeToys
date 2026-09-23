import { useEffect, useState } from 'react'
import {
  Alert,
  Button,
  Card,
  Checkbox,
  Col,
  InputNumber,
  Row,
  Select,
  Space,
  Statistic,
  Table,
  Tabs,
  Tag,
  message
} from 'antd'
import DataList from '../components/DataList'
import { auditDataset, checkLeakage, getDataFiles, getPiiPatterns, sanitizeData } from '../services/api'
import type {
  AuditResponse,
  LeakageResponse,
  PiiPatternsResponse,
  SanitizeResponse
} from '../types/api'

/**
 * 数据安全中心
 *
 * 聚合隐私脱敏、训练/测试集泄漏检测与数据集就绪审计三项能力。
 *
 * 与「质量中心」的分界：质量中心回答「这批数据好不好」，这里回答
 * 「这批数据能不能用」——脱敏是合规前置，泄漏检测决定评测结果是否可信，
 * 就绪审计把这两类信号合成一个 go / no-go 判定。
 */
export default function Security() {
  const [files, setFiles] = useState<string[]>([])
  const [file, setFile] = useState('')
  const [trainFile, setTrainFile] = useState('')
  const [testFile, setTestFile] = useState('')
  const [referenceFile, setReferenceFile] = useState('')
  const [fuzzyThreshold, setFuzzyThreshold] = useState(0.8)
  const [includeExtra, setIncludeExtra] = useState(false)
  const [patterns, setPatterns] = useState<PiiPatternsResponse | null>(null)
  const [sanitized, setSanitized] = useState<SanitizeResponse | null>(null)
  const [leakage, setLeakage] = useState<LeakageResponse | null>(null)
  const [audit, setAudit] = useState<AuditResponse | null>(null)
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    getPiiPatterns()
      .then(setPatterns)
      .catch(() => message.error('加载 PII 模式清单失败'))
    // 泄漏检测要选两个文件，DataList 只解决「单文件 + 预览」，所以这里另拉一份文件名列表
    getDataFiles()
      .then(result => setFiles(result.files.map(item => item.name)))
      .catch(() => message.error('加载数据文件列表失败'))
  }, [])

  /**
   * 统一的任务包装：置 loading → 分发结果 / 报错
   *
   * 泛型让 `onDone` 与 `task` 的结果类型绑在一起，否则 `onDone` 只能是
   * `(data: any) => void`，而 `src/pages` 层禁止写 `any`。
   */
  const run = async <T,>(
    task: () => Promise<T>,
    onDone: (data: T) => void,
    failText: string
  ) => {
    setLoading(true)
    try {
      onDone(await task())
    } catch {
      message.error(failText)
    } finally {
      setLoading(false)
    }
  }

  const fileOptions = files.map(name => ({ label: name, value: name }))

  const matchRows = Object.entries(sanitized?.report.matches || {}).map(
    ([pattern, count]) => ({ pattern, count })
  )

  const sanitizedPreview = (sanitized?.items || []).slice(0, 5)

  const auditFindings = (audit?.findings || []).map(finding => ({ finding }))

  return (
    <div>
      <DataList value={file} onChange={setFile} previewSize={3} />

      <Card style={{ marginBottom: 16 }}>
        <Space wrap>
          <span>泄漏模糊阈值</span>
          <InputNumber
            min={0}
            max={1}
            step={0.05}
            value={fuzzyThreshold}
            onChange={value => setFuzzyThreshold(Number(value ?? 0.8))}
          />
          <Checkbox
            checked={includeExtra}
            onChange={event => setIncludeExtra(event.target.checked)}
          >
            启用增强脱敏模式（误伤率更高）
          </Checkbox>
        </Space>
        <div style={{ marginTop: 12 }}>
          <Space wrap>
            <Button
              type="primary"
              loading={loading}
              onClick={() =>
                run(
                  () => sanitizeData(file, undefined, includeExtra),
                  setSanitized,
                  'PII 脱敏失败'
                )
              }
            >
              隐私脱敏
            </Button>
            <Button
              loading={loading}
              onClick={() =>
                run(
                  () => auditDataset(file, referenceFile || undefined),
                  setAudit,
                  '就绪审计失败'
                )
              }
            >
              就绪审计
            </Button>
          </Space>
        </div>
      </Card>

      <Tabs
        items={[
          {
            key: 'privacy',
            label: '隐私脱敏',
            children: (
              <>
                <Card
                  title="可用的 PII 模式"
                  style={{ marginBottom: 16 }}
                  extra={
                    <Tag color="blue">
                      默认 {patterns?.default.length ?? 0} 项 · 增强{' '}
                      {patterns?.extra.length ?? 0} 项
                    </Tag>
                  }
                >
                  <Space wrap>
                    {(patterns?.default || []).map(name => (
                      <Tag key={name} color="geekblue">
                        {name}
                      </Tag>
                    ))}
                  </Space>
                  <div style={{ marginTop: 12 }}>
                    <span style={{ marginRight: 8 }}>增强模式：</span>
                    <Space wrap>
                      {(patterns?.extra || []).map(name => (
                        <Tag key={name} color="orange">
                          {name}
                        </Tag>
                      ))}
                    </Space>
                  </div>
                </Card>

                {sanitized ? (
                  <>
                    <Row gutter={16} style={{ marginBottom: 16 }}>
                      <Col span={8}>
                        <Card>
                          <Statistic title="样本总数" value={sanitized.report.total_items} />
                        </Card>
                      </Col>
                      <Col span={8}>
                        <Card>
                          <Statistic title="命中样本" value={sanitized.report.touched_items} />
                        </Card>
                      </Col>
                      <Col span={8}>
                        <Card>
                          <Statistic title="命中次数" value={sanitized.report.total_matches} />
                        </Card>
                      </Col>
                    </Row>
                    <Row gutter={16}>
                      <Col span={12}>
                        <Card title="按模式统计">
                          <Table
                            size="small"
                            rowKey="pattern"
                            pagination={false}
                            columns={[
                              { title: '模式', dataIndex: 'pattern' },
                              { title: '命中次数', dataIndex: 'count', width: 100 }
                            ]}
                            dataSource={matchRows}
                          />
                        </Card>
                      </Col>
                      <Col span={12}>
                        <Card title="脱敏结果预览（前 5 条）">
                          <Table
                            size="small"
                            rowKey={(_, index) => String(index)}
                            pagination={false}
                            columns={[
                              { title: '问题', dataIndex: 'instruction', ellipsis: true },
                              { title: '回答', dataIndex: 'output', ellipsis: true }
                            ]}
                            dataSource={sanitizedPreview}
                          />
                        </Card>
                      </Col>
                    </Row>
                  </>
                ) : (
                  <Card>选择数据集后点击「隐私脱敏」，查看命中明细与脱敏后的数据</Card>
                )}
              </>
            )
          },
          {
            key: 'leakage',
            label: '泄漏检测',
            children: (
              <>
                <Card style={{ marginBottom: 16 }}>
                  <Space wrap>
                    <span>训练集</span>
                    <Select
                      style={{ width: 260 }}
                      value={trainFile || undefined}
                      onChange={setTrainFile}
                      placeholder="请选择训练集"
                      options={fileOptions}
                    />
                    <span>测试集</span>
                    <Select
                      style={{ width: 260 }}
                      value={testFile || undefined}
                      onChange={setTestFile}
                      placeholder="请选择测试集"
                      options={fileOptions}
                    />
                    <Button
                      type="primary"
                      loading={loading}
                      onClick={() =>
                        run(
                          () => checkLeakage(trainFile, testFile, fuzzyThreshold),
                          setLeakage,
                          '泄漏检测失败，请确认已选择两个数据集'
                        )
                      }
                    >
                      检测泄漏
                    </Button>
                  </Space>
                </Card>

                {leakage ? (
                  <>
                    <Alert
                      type={leakage.is_clean ? 'success' : 'warning'}
                      showIcon
                      style={{ marginBottom: 16 }}
                      message={
                        leakage.is_clean
                          ? '未检出泄漏，训练集与测试集互不重叠'
                          : `检出 ${leakage.total_leaks} 处泄漏，评测结果可能被高估`
                      }
                    />
                    <Row gutter={16} style={{ marginBottom: 16 }}>
                      <Col span={4}>
                        <Card>
                          <Statistic title="训练集" value={leakage.train_size} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic title="测试集" value={leakage.test_size} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic title="完全重复" value={leakage.exact_leaks} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic title="模糊重复" value={leakage.fuzzy_leaks} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic title="泄漏总数" value={leakage.total_leaks} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic
                            title="泄漏率"
                            value={(leakage.leak_rate * 100).toFixed(2)}
                            suffix="%"
                          />
                        </Card>
                      </Col>
                    </Row>
                    <Card title={`泄漏样本（${leakage.leaked_examples.length} 条）`}>
                      {/*
                        `leaked_examples` 的键随检测配置（fields / 命中类型）而变，
                        不在类型层建模，因此这里直接渲染 JSON —— 比发明一套
                        可能不存在的列名更诚实。
                      */}
                      <pre style={{ maxHeight: 320, overflow: 'auto', margin: 0 }}>
                        {JSON.stringify(leakage.leaked_examples, null, 2)}
                      </pre>
                    </Card>
                  </>
                ) : (
                  <Card>选择训练集与测试集后点击「检测泄漏」</Card>
                )}
              </>
            )
          },
          {
            key: 'audit',
            label: '就绪审计',
            children: (
              <>
                <Card style={{ marginBottom: 16 }}>
                  <Space wrap>
                    <span>参照集（可选）</span>
                    <Select
                      style={{ width: 260 }}
                      value={referenceFile || undefined}
                      onChange={setReferenceFile}
                      placeholder="不选则不对比参照集"
                      allowClear
                      options={fileOptions}
                    />
                    <span style={{ color: '#888' }}>当前数据集：{file || '未选择'}</span>
                  </Space>
                </Card>

                {audit ? (
                  <>
                    <Alert
                      type={audit.ready ? 'success' : 'warning'}
                      showIcon
                      style={{ marginBottom: 16 }}
                      message={audit.ready ? '数据集已就绪' : '数据集尚未就绪，请先处理下列问题'}
                    />
                    <Row gutter={16} style={{ marginBottom: 16 }}>
                      <Col span={4}>
                        <Card>
                          <Statistic title="样本总数" value={audit.total_items} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic title="PII 命中" value={audit.pii_matches} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic
                            title="重复率"
                            value={(audit.duplicate_rate * 100).toFixed(2)}
                            suffix="%"
                          />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic
                            title="空字段率"
                            value={(audit.empty_field_rate * 100).toFixed(2)}
                            suffix="%"
                          />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic title="泄漏数" value={audit.leak_count} />
                        </Card>
                      </Col>
                      <Col span={4}>
                        <Card>
                          <Statistic
                            title="泄漏率"
                            value={(audit.leak_rate * 100).toFixed(2)}
                            suffix="%"
                          />
                        </Card>
                      </Col>
                    </Row>
                    <Card title={`审计发现（${auditFindings.length} 条）`}>
                      {auditFindings.length > 0 ? (
                        <Table
                          size="small"
                          rowKey="finding"
                          pagination={false}
                          columns={[{ title: '发现', dataIndex: 'finding' }]}
                          dataSource={auditFindings}
                        />
                      ) : (
                        <Alert type="success" showIcon message="未发现问题" />
                      )}
                    </Card>
                  </>
                ) : (
                  <Card>点击「就绪审计」把脱敏、重复、空字段与泄漏信号合成一个判定</Card>
                )}
              </>
            )
          }
        ]}
      />
    </div>
  )
}
