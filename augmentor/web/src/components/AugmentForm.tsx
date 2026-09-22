import { useEffect, useState } from 'react'
import { Card, Form, Select, Input, Button, Space, message, Alert } from 'antd'
import { getDataFiles, startAugmentation, getProgress } from '../services/api'
import type { AugmentProgress } from '../types/api'

interface AugmentFormProps {
  /** 增强任务提交成功后的回调 */
  onSubmitted?: () => void
}

/** 表单字段，与各 `Form.Item` 的 `name` 一一对应 */
interface AugmentFormValues {
  inputFile: string
  outputFile: string
  useQuality: boolean
  useDedup: boolean
  useCheckpoint: boolean
}

const ENABLE_OPTIONS = [
  { label: '启用', value: true },
  { label: '禁用', value: false }
]

/**
 * 增强任务配置表单
 *
 * 把「选数据 → 设参数 → 启动」这条最高频路径封装为单一组件，
 * 供仪表盘等页面直接复用。
 */
export default function AugmentForm({ onSubmitted }: AugmentFormProps) {
  const [form] = Form.useForm()
  const [files, setFiles] = useState<string[]>([])
  const [submitting, setSubmitting] = useState(false)
  const [progress, setProgress] = useState<AugmentProgress | null>(null)

  useEffect(() => {
    getDataFiles()
      .then(result => setFiles(result.files.map(f => f.name)))
      .catch(() => message.error('加载数据文件列表失败'))
  }, [])

  useEffect(() => {
    const timer = setInterval(() => {
      getProgress()
        .then(setProgress)
        .catch(() => undefined)
    }, 5000)
    return () => clearInterval(timer)
  }, [])

  const handleSubmit = async (values: AugmentFormValues) => {
    setSubmitting(true)
    try {
      await startAugmentation(
        values.inputFile,
        values.outputFile,
        values.useQuality,
        values.useDedup,
        values.useCheckpoint
      )
      message.success('增强任务已提交，可在进度面板查看状态')
      onSubmitted?.()
    } catch {
      message.error('增强任务启动失败')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <Card title="快速增强">
      <Form
        form={form}
        layout="vertical"
        initialValues={{
          useQuality: true,
          useDedup: true,
          useCheckpoint: true
        }}
        onFinish={handleSubmit}
      >
        <Form.Item
          name="inputFile"
          label="输入数据集"
          rules={[{ required: true, message: '请选择输入数据集' }]}
        >
          <Select
            placeholder="请选择种子数据文件"
            options={files.map(name => ({ label: name, value: name }))}
          />
        </Form.Item>

        <Form.Item
          name="outputFile"
          label="输出文件名"
          rules={[{ required: true, message: '请输入输出文件名' }]}
        >
          <Input placeholder="例如 train_data_augmented.json" />
        </Form.Item>

        <Space wrap size="large">
          <Form.Item name="useQuality" label="质量检查" style={{ marginBottom: 0 }}>
            <Select style={{ width: 110 }} options={ENABLE_OPTIONS} />
          </Form.Item>
          <Form.Item name="useDedup" label="智能去重" style={{ marginBottom: 0 }}>
            <Select style={{ width: 110 }} options={ENABLE_OPTIONS} />
          </Form.Item>
          <Form.Item name="useCheckpoint" label="断点续传" style={{ marginBottom: 0 }}>
            <Select style={{ width: 110 }} options={ENABLE_OPTIONS} />
          </Form.Item>
        </Space>

        <Form.Item style={{ marginTop: 16, marginBottom: 0 }}>
          <Button type="primary" htmlType="submit" loading={submitting}>
            启动增强任务
          </Button>
        </Form.Item>
      </Form>

      {/* 无进行中任务时后端只返回 `{ status: 'no_checkpoint' }`，靠 `task_id` 判别 */}
      {progress && 'task_id' in progress && (
        <Alert
          style={{ marginTop: 16 }}
          type="info"
          showIcon
          message={`当前进度 ${progress.progress_percent || '0%'}`}
          description={`已处理 ${progress.processed_items ?? 0} / ${progress.total_items ?? 0} 条，预计剩余 ${progress.remaining_time || '未知'}`}
        />
      )}
    </Card>
  )
}
