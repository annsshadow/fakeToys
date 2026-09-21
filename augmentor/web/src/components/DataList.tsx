import { useEffect, useState } from 'react'
import { Card, Select, Table, Input, Space, Tag, message } from 'antd'
import { getDataFiles, loadData } from '../services/api'

interface DataListProps {
  /** 当前选中的数据集文件名 */
  value?: string
  /** 选择变化回调 */
  onChange?: (filename: string) => void
  /** 预览条数 */
  previewSize?: number
  /** 是否显示预览表格 */
  showPreview?: boolean
}

/**
 * 数据集选择与预览组件
 *
 * 统一各页面选择数据集的交互，避免每个页面重复实现分页与搜索逻辑。
 */
export default function DataList({
  value,
  onChange,
  previewSize = 5,
  showPreview = true
}: DataListProps) {
  const [files, setFiles] = useState<string[]>([])
  const [items, setItems] = useState<any[]>([])
  const [total, setTotal] = useState(0)
  const [search, setSearch] = useState('')
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    getDataFiles()
      .then(result => setFiles(result.files.map((f: any) => f.name)))
      .catch(() => message.error('加载数据文件列表失败'))
  }, [])

  useEffect(() => {
    if (!value || !showPreview) {
      setItems([])
      setTotal(0)
      return
    }

    setLoading(true)
    loadData(value, 1, previewSize, search)
      .then(result => {
        setItems(result.items || [])
        setTotal(result.total || 0)
      })
      .catch(() => message.error('加载数据预览失败'))
      .finally(() => setLoading(false))
  }, [value, search, previewSize, showPreview])

  const columns = [
    {
      title: '问题',
      dataIndex: 'instruction',
      key: 'instruction',
      ellipsis: true
    },
    {
      title: '回答',
      dataIndex: 'output',
      key: 'output',
      ellipsis: true
    }
  ]

  return (
    <Card
      title="数据集"
      extra={<Tag color="blue">共 {total} 条</Tag>}
      style={{ marginBottom: 16 }}
    >
      <Space style={{ marginBottom: 16 }} wrap>
        <Select
          style={{ width: 320 }}
          value={value || undefined}
          onChange={onChange}
          placeholder="请选择数据集"
          options={files.map(name => ({ label: name, value: name }))}
        />
        {showPreview && (
          <Input.Search
            style={{ width: 240 }}
            placeholder="搜索问题或回答"
            allowClear
            onSearch={setSearch}
          />
        )}
      </Space>

      {showPreview && (
        <Table
          size="small"
          rowKey={(_, index) => String(index)}
          loading={loading}
          columns={columns}
          dataSource={items}
          pagination={false}
        />
      )}
    </Card>
  )
}
