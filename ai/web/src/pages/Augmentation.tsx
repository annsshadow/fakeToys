import { useState, useEffect } from 'react'
import { Card, Button, Space, Select, Switch, message, Progress, List, Tag } from 'antd'
import { RocketOutlined, PauseOutlined, ReloadOutlined } from '@ant-design/icons'
import { startAugmentation, getProgress, getDataFiles } from '../services/api'

export default function Augmentation() {
  const [files, setFiles] = useState<string[]>([])
  const [inputFile, setInputFile] = useState('')
  const [outputFile, setOutputFile] = useState('augmented_output.json')
  const [useQuality, setUseQuality] = useState(true)
  const [useDedup, setUseDedup] = useState(true)
  const [useCheckpoint, setUseCheckpoint] = useState(true)
  const [running, setRunning] = useState(false)
  const [progress, setProgress] = useState<any>(null)

  useEffect(() => {
    loadFiles()
    const interval = setInterval(checkProgress, 2000)
    return () => clearInterval(interval)
  }, [])

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map((f: any) => f.name))
    } catch (error) {
      message.error('加载文件列表失败')
    }
  }

  const checkProgress = async () => {
    try {
      const result = await getProgress()
      if (result && result.status !== 'no_checkpoint') {
        setProgress(result)
        if (result.progress >= 1) {
          setRunning(false)
          message.success('增强任务完成！')
        }
      }
    } catch (error) {
      // 忽略错误
    }
  }

  const handleStart = async () => {
    if (!inputFile) {
      message.warning('请选择输入文件')
      return
    }

    setRunning(true)
    try {
      await startAugmentation(inputFile, outputFile, useQuality, useDedup, useCheckpoint)
      message.success('增强任务已启动')
    } catch (error) {
      message.error('启动失败')
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
              {files.map(file => (
                <Select.Option key={file} value={file}>{file}</Select.Option>
              ))}
            </Select>
          </div>
          <div>
            <label>输出文件：</label>
            <input
              type="text"
              value={outputFile}
              onChange={(e) => setOutputFile(e.target.value)}
              style={{ width: 300, padding: '4px 8px', borderRadius: 4, border: '1px solid #d9d9d9' }}
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
                <div>任务 ID: <Tag>{progress.task_id}</Tag></div>
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
          <div style={{ textAlign: 'center', color: '#999', padding: 40 }}>
            暂无进行中的任务
          </div>
        )}
      </Card>
    </div>
  )
}
