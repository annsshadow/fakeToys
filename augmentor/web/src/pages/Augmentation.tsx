import { useState, useEffect, useCallback } from 'react'
import { Card, Button, Space, Select, Switch, message, Progress, Tag } from 'antd'
import { RocketOutlined, ReloadOutlined } from '@ant-design/icons'
import {
  apiErrorDetail,
  startAugmentation,
  getProgress,
  getCheckpoints,
  getDataFiles,
} from '../services/api'
import type { AugmentProgressDetail } from '../types/api'

export default function Augmentation() {
  const [files, setFiles] = useState<string[]>([])
  const [inputFile, setInputFile] = useState('')
  const [outputFile, setOutputFile] = useState('augmented_output.json')
  const [useQuality, setUseQuality] = useState(true)
  const [useDedup, setUseDedup] = useState(true)
  const [useCheckpoint, setUseCheckpoint] = useState(true)
  // 异步引擎默认关：只有后端覆写了原生异步（目前是 OpenAI）才真的省线程，
  // 其余后端会由服务端自动回退线程池并打 warning。
  const [useAsync, setUseAsync] = useState(false)
  const [running, setRunning] = useState(false)
  const [progress, setProgress] = useState<AugmentProgressDetail | null>(null)
  const [checkpoints, setCheckpoints] = useState<string[]>([])
  // L277：轮询开关是一个**状态**，定时器由 effect 拥有——终态（`progress >= 1`）把开关关掉，
  // 表随之被 cleanup 清掉；再点一次「开始增强」把开关打开，表重新起。
  // 改前这里没有开关：`setInterval` 住在挂载 effect 的局部变量里，`checkProgress` 够不着它，
  // 于是任务完成后定时器照跑——「增强任务完成！」每 2 秒重发一次，页面永远在打
  // `GET /api/augment/progress`。
  const [polling, setPolling] = useState(false)

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map((f) => f.name))
    } catch (err) {
      message.error(apiErrorDetail(err, '加载文件列表失败'))
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

  // `useCallback` 的依赖是空集：函数体只碰 setter（React 保证身份稳定）与模块级服务函数。
  // 这不是「为了过 lint 而包一层」——它进下面那只 effect 的依赖数组，身份不稳定就会每次渲染
  // 清表再起表，轮询节奏变成随渲染抖动。
  const checkProgress = useCallback(async () => {
    try {
      const result = await getProgress()
      // 后端有两种响应：`{ status: 'no_checkpoint' }`，或完整进度（**不带** `status`）。
      // 所以判别只能用 `task_id` 在不在——读 `status` 既收窄不了类型，
      // 也不比这个判断更贴合后端实现。
      if ('task_id' in result) {
        setProgress(result)
        if (result.progress >= 1) {
          setRunning(false)
          message.success('增强任务完成！')
          // L277：到终态就收表。改前这一句之后定时器照跑，于是「增强任务完成！」
          // 每 2 秒重发一次、并且永远在打 `GET /api/augment/progress`。
          setPolling(false)
        }
      }
    } catch {
      // 忽略错误
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
      await startAugmentation(inputFile, outputFile, useQuality, useDedup, useCheckpoint, useAsync)
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
              <Tag key={id}>{id}</Tag>
            ))}
          </Space>
        ) : (
          <span style={{ color: '#999' }}>暂无断点</span>
        )}
      </Card>
    </div>
  )
}
