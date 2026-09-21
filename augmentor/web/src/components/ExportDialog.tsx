import { useState } from 'react'
import {
  Modal,
  Form,
  Select,
  Input,
  Button,
  Table,
  Alert,
  Space,
  message,
  Tag
} from 'antd'
import { previewExport, batchExport } from '../services/api'

interface ExportDialogProps {
  /** 待导出的数据集文件名 */
  inputFile?: string
  /** 是否打开 */
  open: boolean
  /** 关闭回调 */
  onClose: () => void
}

const FORMAT_OPTIONS = [
  { label: 'JSONL', value: 'jsonl' },
  { label: 'Llama-Factory', value: 'llama_factory' },
  { label: 'Alpaca', value: 'alpaca' },
  { label: 'ShareGPT', value: 'sharegpt' },
  { label: 'ChatML', value: 'chatml' },
  { label: 'CSV', value: 'csv' }
]

/**
 * 导出配置弹窗
 *
 * 支持「先预览后导出」，避免用户导出后才发现字段缺失。
 */
export default function ExportDialog({ inputFile, open, onClose }: ExportDialogProps) {
  const [form] = Form.useForm()
  const [preview, setPreview] = useState<any>(null)
  const [loading, setLoading] = useState(false)

  const handlePreview = async () => {
    if (!inputFile) {
      message.warning('请先选择数据集')
      return
    }

    const values = await form.validateFields(['format'])
    setLoading(true)
    try {
      const result = await previewExport(inputFile, values.format)
      setPreview(result)
    } catch {
      message.error('预览失败，请检查数据集与格式')
    } finally {
      setLoading(false)
    }
  }

  const handleExport = async () => {
    if (!inputFile) {
      message.warning('请先选择数据集')
      return
    }

    const values = await form.validateFields(['format', 'outputDir'])
    setLoading(true)
    try {
      await batchExport({ dataset: inputFile }, values.outputDir, [values.format])
      message.success('导出完成')
      onClose()
    } catch {
      message.error('导出失败')
    } finally {
      setLoading(false)
    }
  }

  const previewColumns = preview?.converted_data?.length
    ? Object.keys(preview.converted_data[0]).map(key => ({
        title: key,
        dataIndex: key,
        key,
        ellipsis: true,
        render: (value: any) =>
          typeof value === 'object' ? JSON.stringify(value) : String(value)
      }))
    : []

  return (
    <Modal
      open={open}
      title="导出数据集"
      onCancel={onClose}
      width={880}
      footer={
        <Space>
          <Button onClick={onClose}>取消</Button>
          <Button onClick={handlePreview} loading={loading}>
            预览
          </Button>
          <Button type="primary" onClick={handleExport} loading={loading}>
            导出
          </Button>
        </Space>
      }
    >
      <Form form={form} layout="vertical" initialValues={{ format: 'jsonl' }}>
        <Form.Item name="format" label="导出格式" rules={[{ required: true }]}>
          <Select options={FORMAT_OPTIONS} />
        </Form.Item>
        <Form.Item
          name="outputDir"
          label="输出目录"
          rules={[{ required: true, message: '请输入输出目录' }]}
        >
          <Input placeholder="例如 exports" />
        </Form.Item>
      </Form>

      {preview?.warnings?.length > 0 && (
        <Alert
          type="warning"
          showIcon
          style={{ marginBottom: 16 }}
          message="导出前请留意以下问题"
          description={
            <ul style={{ margin: 0, paddingLeft: 18 }}>
              {preview.warnings.map((warning: string, index: number) => (
                <li key={index}>{warning}</li>
              ))}
            </ul>
          }
        />
      )}

      {preview && (
        <>
          <Space style={{ marginBottom: 8 }}>
            <Tag color="blue">共 {preview.format_info?.total_items ?? 0} 条</Tag>
            <Tag color="green">预览 {preview.format_info?.preview_items ?? 0} 条</Tag>
          </Space>
          <Table
            size="small"
            rowKey={(_, index) => String(index)}
            columns={previewColumns}
            dataSource={preview.converted_data || []}
            pagination={false}
            scroll={{ x: 'max-content' }}
          />
        </>
      )}
    </Modal>
  )
}
