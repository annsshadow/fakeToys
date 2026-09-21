import { useEffect, useState } from 'react'
import {
  Card,
  Button,
  Table,
  Tag,
  Space,
  Alert,
  message,
  Form,
  Select,
  Input,
  List,
  type TableColumnsType
} from 'antd'
import DataList from '../components/DataList'
import ExportDialog from '../components/ExportDialog'
import { getExportFormats, previewExport, batchExport } from '../services/api'
import type { ExportPreviewResponse } from '../types/api'

/** 批量导出表单的字段，与各 `Form.Item` 的 `name` 一一对应 */
interface BatchExportFormValues {
  files: string[]
  formats: string[]
  outputDir: string
}

/**
 * 导出中心
 *
 * 支持格式预览、单数据集导出与批量导出，
 * 对应需求 R8 / R9 / R10。
 */
export default function Export() {
  const [file, setFile] = useState('')
  const [formats, setFormats] = useState<string[]>([])
  const [previewFormat, setPreviewFormat] = useState('jsonl')
  const [preview, setPreview] = useState<ExportPreviewResponse | null>(null)
  const [dialogOpen, setDialogOpen] = useState(false)
  const [loading, setLoading] = useState(false)
  const [batchForm] = Form.useForm()
  const [batchResult, setBatchResult] = useState<Record<
    string,
    Record<string, string>
  > | null>(null)

  useEffect(() => {
    getExportFormats()
      .then(result => setFormats(result.formats || []))
      .catch(() => message.error('加载导出格式失败'))
  }, [])

  const handlePreview = async () => {
    if (!file) {
      message.warning('请先选择数据集')
      return
    }
    setLoading(true)
    try {
      setPreview(await previewExport(file, previewFormat))
    } catch {
      message.error('预览失败')
    } finally {
      setLoading(false)
    }
  }

  const handleBatch = async (values: BatchExportFormValues) => {
    setLoading(true)
    try {
      const datasets = values.files.reduce(
        (acc: Record<string, string>, name: string) => {
          acc[name.replace(/\.[^.]+$/, '')] = name
          return acc
        },
        {}
      )
      const result = await batchExport(datasets, values.outputDir, values.formats)
      setBatchResult(result.results)
      message.success('批量导出完成')
    } catch {
      message.error('批量导出失败')
    } finally {
      setLoading(false)
    }
  }

  const previewColumns: TableColumnsType<Record<string, unknown>> = preview?.converted_data
    ?.length
    ? Object.keys(preview.converted_data[0]).map(key => ({
        title: key,
        dataIndex: key,
        key,
        ellipsis: true,
        render: (value: unknown) =>
          typeof value === 'object' ? JSON.stringify(value) : String(value)
      }))
    : []

  return (
    <div>
      <DataList value={file} onChange={setFile} previewSize={3} />

      <Card title="格式预览" style={{ marginBottom: 16 }}>
        <Space wrap style={{ marginBottom: 16 }}>
          <Select
            style={{ width: 220 }}
            value={previewFormat}
            onChange={setPreviewFormat}
            options={formats.map(fmt => ({ label: fmt, value: fmt }))}
          />
          <Button type="primary" loading={loading} onClick={handlePreview}>
            预览转换结果
          </Button>
          <Button onClick={() => setDialogOpen(true)} disabled={!file}>
            导出数据集
          </Button>
        </Space>

        {/* 写成 `preview && …` 而不是 `preview?.warnings?.length > 0`：
            后者收窄不了 `preview`，里面的 `preview.warnings.map` 会报「可能为 null」。 */}
        {preview && preview.warnings.length > 0 && (
          <Alert
            type="warning"
            showIcon
            style={{ marginBottom: 16 }}
            message="导出前请留意以下问题"
            description={
              <ul style={{ margin: 0, paddingLeft: 18 }}>
                {preview.warnings.map((warning, index) => (
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
              <Tag color="green">
                预览 {preview.format_info?.preview_items ?? 0} 条
              </Tag>
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
      </Card>

      <Card title="批量导出">
        <Form form={batchForm} layout="inline" onFinish={handleBatch}>
          <Form.Item
            name="files"
            rules={[{ required: true, message: '请选择数据集' }]}
          >
            <Select
              mode="multiple"
              style={{ minWidth: 320 }}
              placeholder="选择多个数据集"
              options={file ? [{ label: file, value: file }] : []}
            />
          </Form.Item>
          <Form.Item name="formats" rules={[{ required: true, message: '请选择格式' }]}>
            <Select
              mode="multiple"
              style={{ minWidth: 260 }}
              placeholder="选择导出格式"
              options={formats.map(fmt => ({ label: fmt, value: fmt }))}
            />
          </Form.Item>
          <Form.Item name="outputDir" rules={[{ required: true, message: '请输入输出目录' }]}>
            <Input placeholder="输出目录" />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading}>
              批量导出
            </Button>
          </Form.Item>
        </Form>

        {batchResult && (
          <List
            style={{ marginTop: 16 }}
            size="small"
            bordered
            header={<strong>导出结果</strong>}
            dataSource={Object.entries(batchResult)}
            renderItem={([name, files]: [string, Record<string, string>]) => (
              <List.Item>
                <Space wrap>
                  <Tag color="blue">{name}</Tag>
                  {Object.entries(files).map(([fmt, path]) => (
                    <Tag key={fmt} color="green">
                      {fmt}: {String(path)}
                    </Tag>
                  ))}
                </Space>
              </List.Item>
            )}
          />
        )}
      </Card>

      <ExportDialog
        inputFile={file}
        open={dialogOpen}
        onClose={() => setDialogOpen(false)}
      />
    </div>
  )
}
