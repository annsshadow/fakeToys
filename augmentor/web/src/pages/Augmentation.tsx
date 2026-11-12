// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { useState, useEffect, useCallback, useRef } from 'react'
import { Card, Button, Space, Select, Switch, message, Progress, Tag, Input } from 'antd'
import { RocketOutlined, ReloadOutlined } from '@ant-design/icons'
import {
  apiErrorDetail,
  startAugmentation,
  getProgress,
  getCheckpoints,
  deleteCheckpoint,
  getAugmentReport,
  getDataFiles,
} from '../services/api'
import type { AugmentProgressDetail, AugmentReport } from '../types/api'

export default function Augmentation() {
  const [files, setFiles] = useState<string[]>([])
  const [inputFile, setInputFile] = useState('')
  const [outputFile, setOutputFile] = useState('augmented_output.json')
  // 第 34 轮：终态 toast 要带上输出文件名，而 checkProgress 是空依赖的稳定回调 ——
  // 用 ref 同步最新值（与 runningRef 同款）。
  const outputFileRef = useRef(outputFile)
  outputFileRef.current = outputFile
  const [useQuality, setUseQuality] = useState(true)
  const [useDedup, setUseDedup] = useState(true)
  const [useCheckpoint, setUseCheckpoint] = useState(true)
  // 异步引擎默认关：只有后端覆写了原生异步（目前是 OpenAI）才真的省线程，
  // 其余后端会由服务端自动回退线程池并打 warning。
  const [useAsync, setUseAsync] = useState(false)
  const [running, setRunning] = useState(false)
  // 第 33 轮：checkProgress 是空依赖的稳定回调（见下），它读 `running`
  // 会拿到闭包里的陈旧值 —— 用 ref 同步最新值，回调身份保持稳定。
  const runningRef = useRef(running)
  runningRef.current = running
  const [progress, setProgress] = useState<AugmentProgressDetail | null>(null)
  // A408：**这次运行**的任务身份（服务端按解析后的输入路径算出来，客户端自己
  // 算不出来）。用 ref 同步最新值（与 runningRef 同款）：`checkProgress` 是空依赖
  // 的稳定回调，读 state 会拿到闭包里的陈旧值。
  const taskIdRef = useRef<string | undefined>(undefined)
  const [checkpoints, setCheckpoints] = useState<string[]>([])
  // L277：轮询开关是一个**状态**，定时器由 effect 拥有——终态（`progress >= 1`）把开关关掉，
  // 表随之被 cleanup 清掉；再点一次「开始增强」把开关打开，表重新起。
  // 改前这里没有开关：`setInterval` 住在挂载 effect 的局部变量里，`checkProgress` 够不着它，
  // 于是任务完成后定时器照跑——「增强任务完成！」每 2 秒重发一次，页面永远在打
  // `GET /api/augment/progress`。
  const [polling, setPolling] = useState(false)
  // L322：增强报告（`GET /api/augment/report/{task_id}`）第一次进这个页面。
  // 任务 ID 由服务端按解析后的输入路径算出来 ⇒ 客户端自己算不出来，
  // 所以这一格是**手填**的（启动回执递回的那一枚会自动填进来）。
  const [reportTaskId, setReportTaskId] = useState('')
  const [report, setReport] = useState<AugmentReport | null>(null)
  const [reportLoading, setReportLoading] = useState(false)

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map((f) => f.name))
    } catch (err) {
      message.error(apiErrorDetail(err, '加载文件列表失败'))
    }
  }

  // 第 56 轮：删除断点（从头重跑的前置能力）。删除后刷新列表。
  const doDeleteCheckpoint = async (taskId: string) => {
    try {
      await deleteCheckpoint(taskId)
      message.success('断点已删除')
      await loadCheckpoints()
    } catch (err) {
      message.error(apiErrorDetail(err, '删除断点失败'))
    }
  }

  const loadCheckpoints = async () => {
    try {
      const result = await getCheckpoints()
      setCheckpoints(result.checkpoints)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载断点列表失败'))
    }
  }

  /**
   * L322：读一次增强报告。
   *
   * 服务端那只端点**挂着 `verify_api_key` 门禁** ⇒ 开了鉴权以后它要带 key；
   * L233 那条通道对**所有**请求生效（含 GET）⇒ 配了 key 之后这里照样读得到。
   *
   * **404 是正常结局而不是故障**：报告只缓存在处理那个任务的 worker 内存里，
   * 任务没跑完、或跑在另一个 worker 上，都会回 404 ⇒ 那句「报告不存在」直接
   * 透给用户（走 `apiErrorDetail` 这一个漏斗），页面上把上一次那份清掉。
   */
  const loadReport = async () => {
    const id = reportTaskId.trim()
    if (!id) {
      message.warning('请填写任务 ID')
      return
    }
    setReportLoading(true)
    try {
      const result = await getAugmentReport(id)
      setReport(result)
    } catch (err) {
      setReport(null)
      message.error(apiErrorDetail(err, '读取增强报告失败'))
    } finally {
      setReportLoading(false)
    }
  }

  // `useCallback` 的依赖是空集：函数体只碰 setter（React 保证身份稳定）与模块级服务函数。
  // 这不是「为了过 lint 而包一层」——它进下面那只 effect 的依赖数组，身份不稳定就会每次渲染
  // 清表再起表，轮询节奏变成随渲染抖动。
  const checkProgress = useCallback(async () => {
    try {
      // A408：点名任务 ⇒ 读的是**我这次**的进度；没点名时仍是后端的环境态读法
      // （「当下谁的断点在跑」）。回执里的 `task_id` 因此不再可能是别人的那次。
      const result = await getProgress(taskIdRef.current)
      // 后端有两种响应：`{ status: 'no_checkpoint' }`，或完整进度（**不带** `status`）。
      // 所以判别只能用 `task_id` 在不在——读 `status` 既收窄不了类型，
      // 也不比这个判断更贴合后端实现。
      if ('task_id' in result) {
        setProgress(result)
        if (result.progress >= 1) {
          setRunning(false)
          message.success(`增强任务完成！输出文件：${outputFileRef.current}`)
          // L277：到终态就收表。改前这一句之后定时器照跑，于是「增强任务完成！」
          // 每 2 秒重发一次、并且永远在打 `GET /api/augment/progress`。
          setPolling(false)
          // 第 34 轮：产物已落盘，立刻刷新文件列表 —— 改前左侧输入选择器
          // 看不到新文件，用户要手动刷新页面才能加载增强结果。
          try {
            const fr = await getDataFiles()
            setFiles(fr.files.map((f) => f.name))
          } catch {
            // 文件列表刷新失败不影响进度展示；下次手动刷新仍可恢复。
          }
        }
      } else if (!runningRef.current) {
        // 第 33 轮：没有任务时也收表。改前挂载即开表，`no_checkpoint` 响应
        // 什么都不做 —— 页面挂着就每 2 秒白打一次进度接口，永不停。
        // 任务启动（handleStart）会重新 setPolling(true)，且那时
        // runningRef.current 为 true（任务可能还没创建断点），不会被误关。
        setPolling(false)
      }
    } catch {
      // A408：点名读法在「那个任务还没建断点」时回 404 —— 运行中就继续等
      // （断点是增强开始后很快建起来的）；不在运行就收表，别让表一直挂着。
      if (!runningRef.current) setPolling(false)
    }
  }, [])

  useEffect(() => {
    loadFiles()
    loadCheckpoints()
    setPolling(true)
  }, [])

  // 开关开 ⇒ 起表；开关关或组件卸载 ⇒ cleanup 清表。定时器只住在这里，
  // 页面上任何一处（`checkProgress`、`handleStart`）都只翻转开关，不碰句柄。
  useEffect(() => {
    if (!polling) return
    const timer = setInterval(checkProgress, 2000)
    return () => clearInterval(timer)
  }, [polling, checkProgress])

  const handleStart = async () => {
    if (!inputFile) {
      message.warning('请选择输入文件')
      return
    }

    setRunning(true)
    try {
      const started = await startAugmentation(
        inputFile,
        outputFile,
        useQuality,
        useDedup,
        useCheckpoint,
        useAsync,
      )
      // A408：任务身份由服务端递回（客户端算不出来）⇒ 之后轮询点名读**这次**的进度
      taskIdRef.current = started.task_id
      // L322：同一个 id 顺手填进「增强报告」那一格 —— 报告按任务 id 查，
      // 而客户端能拿到的唯一一枚 id 就是这一枚（服务端递回的那一枚）。
      setReportTaskId(started.task_id)
      message.success('增强任务已启动')
      // 上一轮到终态时把表关了；这里重新打开，否则第二次点「开始增强」页面不会再刷进度。
      setPolling(true)
    } catch (err) {
      message.error(apiErrorDetail(err, '启动失败'))
      setRunning(false)
    }
  }

  return (
    <div>
      <Card title="数据增强配置" style={{ marginBottom: 16 }}>
        <Space direction="vertical" style={{ width: '100%' }}>
          <div>
            <label>输入文件：</label>
            <Select
              style={{ width: 300 }}
              value={inputFile}
              onChange={setInputFile}
              placeholder="选择输入文件"
            >
              {files.map((file) => (
                <Select.Option key={file} value={file}>
                  {file}
                </Select.Option>
              ))}
            </Select>
          </div>
          <div>
            <label>输出文件：</label>
            <input
              type="text"
              value={outputFile}
              onChange={(e) => setOutputFile(e.target.value)}
              style={{
                width: 300,
                padding: '4px 8px',
                borderRadius: 4,
                border: '1px solid #d9d9d9',
              }}
            />
          </div>
          <div style={{ display: 'flex', gap: 24 }}>
            <div>
              <label>质量检查：</label>
              <Switch checked={useQuality} onChange={setUseQuality} />
            </div>
            <div>
              <label>智能去重：</label>
              <Switch checked={useDedup} onChange={setUseDedup} />
            </div>
            <div>
              <label>断点续传：</label>
              <Switch checked={useCheckpoint} onChange={setUseCheckpoint} />
            </div>
            <div>
              <label>异步引擎：</label>
              <Switch checked={useAsync} onChange={setUseAsync} />
            </div>
          </div>
          <Button
            type="primary"
            icon={<RocketOutlined />}
            onClick={handleStart}
            loading={running}
            disabled={running}
          >
            开始增强
          </Button>
        </Space>
      </Card>

      <Card title="增强进度">
        {progress ? (
          <div>
            <Progress
              percent={Math.round((progress.progress || 0) * 100)}
              status={running ? 'active' : 'normal'}
            />
            <div style={{ marginTop: 16 }}>
              <Space direction="vertical">
                <div>
                  任务 ID: <Tag>{progress.task_id}</Tag>
                </div>
                <div>总条数: {progress.total_items}</div>
                <div>已处理: {progress.processed_items}</div>
                <div>失败: {progress.failed_items}</div>
                <div>已用时间: {progress.elapsed_time}</div>
                <div>预计剩余: {progress.remaining_time}</div>
                {progress.avg_quality_score > 0 && (
                  <div>平均质量评分: {progress.avg_quality_score.toFixed(3)}</div>
                )}
              </Space>
            </div>
          </div>
        ) : (
          <div style={{ textAlign: 'center', color: '#999', padding: 40 }}>暂无进行中的任务</div>
        )}
      </Card>

      <Card
        title="断点列表"
        style={{ marginTop: 16 }}
        extra={
          <Button size="small" icon={<ReloadOutlined />} onClick={loadCheckpoints}>
            刷新
          </Button>
        }
      >
        {checkpoints.length > 0 ? (
          <Space wrap data-testid="checkpoint-list">
            {checkpoints.map((id) => (
              <Tag
                key={id}
                closable
                onClose={(e) => {
                  e.preventDefault()
                  doDeleteCheckpoint(id)
                }}
              >
                {id}
              </Tag>
            ))}
          </Space>
        ) : (
          <span style={{ color: '#999' }}>暂无断点</span>
        )}
      </Card>

      <Card
        title="增强报告"
        style={{ marginTop: 16 }}
        extra={
          <Button
            size="small"
            icon={<ReloadOutlined />}
            onClick={loadReport}
            loading={reportLoading}
          >
            查看报告
          </Button>
        }
      >
        <Space direction="vertical" style={{ width: '100%' }}>
          <div>
            <label>任务 ID：</label>
            <Input
              style={{ width: 320 }}
              value={reportTaskId}
              onChange={(e) => setReportTaskId(e.target.value)}
              placeholder="增强任务的任务 ID"
            />
          </div>
          {report ? (
            <div data-testid="augment-report">
              <Space direction="vertical">
                <div>
                  任务 ID: <Tag>{report.task_id}</Tag>
                </div>
                <div>输入文件: {report.input_file}</div>
                <div>输出文件: {report.output_file}</div>
                <div>
                  条数: {report.input_count} → {report.output_count}
                </div>
                <div>
                  开关: 质量检查 {report.quality_check ? '开' : '关'} / 去重{' '}
                  {report.dedup ? '开' : '关'} / 并行 {report.parallel ? '开' : '关'}
                </div>
                <div>失败明细: {report.errors.length} 条</div>
                {report.errors.map((item, index) => (
                  <div key={index} style={{ color: '#a8071a' }}>
                    {Object.entries(item)
                      .map(([k, v]) => `${k}: ${typeof v === 'string' ? v : JSON.stringify(v)}`)
                      .join(' / ')}
                  </div>
                ))}
              </Space>
            </div>
          ) : (
            <span style={{ color: '#999' }}>暂无报告</span>
          )}
        </Space>
      </Card>
    </div>
  )
}
