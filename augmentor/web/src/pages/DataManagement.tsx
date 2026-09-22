import { useState, useEffect, useCallback } from 'react'
import {
  Table,
  Button,
  Space,
  message,
  Modal,
  Input,
  Pagination,
  Upload,
  type TableColumnsType,
} from 'antd'
import { DownloadOutlined, DeleteOutlined, EditOutlined, UploadOutlined, PlayCircleOutlined } from '@ant-design/icons'
import { getDataFiles, loadData, updateDataItem, deleteDataItem, exportData, uploadData } from '../services/api'
import type { DataFileInfo, DataItem } from '../types/api'

export default function DataManagement() {
  const [files, setFiles] = useState<DataFileInfo[]>([])
  const [selectedFile, setSelectedFile] = useState<string>('')
  const [data, setData] = useState<DataItem[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [search, setSearch] = useState('')
  const [loading, setLoading] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [editModalVisible, setEditModalVisible] = useState(false)
  const [editingItem, setEditingItem] = useState<DataItem | null>(null)
  const [editingIndex, setEditingIndex] = useState(-1)

  const loadFiles = useCallback(async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files)
      if (result.files.length > 0) {
        setSelectedFile(result.files[0].name)
      }
    } catch {
      message.error('加载文件列表失败')
    }
  }, [])

  const loadDataList = useCallback(async () => {
    setLoading(true)
    try {
      const result = await loadData(selectedFile, page, pageSize, search)
      setData(result.items)
      setTotal(result.total)
    } catch {
      message.error('加载数据失败')
    } finally {
      setLoading(false)
    }
  }, [selectedFile, page, pageSize, search])

  // effect 必须放在被调用函数的声明之后，否则 react-hooks 会报
  // 「Cannot access variable before it is declared」。
  // 依赖数组里放函数本身（而不是它读取的每个值）是安全的：useCallback 的依赖
  // 恰好就是这几个值，函数身份只在它们变化时才变，effect 的触发时机与改动前一致。
  useEffect(() => {
    loadFiles()
  }, [loadFiles])

  useEffect(() => {
    if (selectedFile) {
      loadDataList()
    }
  }, [selectedFile, loadDataList])

  const handleExport = async (format: string) => {
    try {
      await exportData(selectedFile, './exports', [format])
      message.success(`导出为 ${format} 格式成功`)
    } catch {
      message.error('导出失败')
    }
  }

  const handleUpload = async (file: File) => {
    setUploading(true)
    try {
      await uploadData(file)
      message.success('上传成功')
      loadFiles()
    } catch {
      message.error('上传失败')
    } finally {
      setUploading(false)
    }
    return false
  }

  const handleLoadDemo = async () => {
    setLoading(true)
    try {
      const response = await fetch('/api/demo/data')
      const blob = await response.blob()
      const file = new File([blob], 'demo_data.json', { type: 'application/json' })
      await handleUpload(file)
    } catch {
      message.error('加载演示数据失败')
    } finally {
      setLoading(false)
    }
  }

  const handleEdit = (item: DataItem, index: number) => {
    setEditingItem({ ...item })
    setEditingIndex(index)
    setEditModalVisible(true)
  }

  const handleSaveEdit = async () => {
    if (editingItem && editingIndex >= 0) {
      try {
        await updateDataItem(selectedFile, editingIndex, editingItem)
        message.success('保存成功')
        setEditModalVisible(false)
        loadDataList()
      } catch {
        message.error('保存失败')
      }
    }
  }

  const handleDelete = async (index: number) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除这条数据吗？',
      onOk: async () => {
        try {
          await deleteDataItem(selectedFile, index)
          message.success('删除成功')
          loadDataList()
        } catch {
          message.error('删除失败')
        }
      }
    })
  }

  const columns: TableColumnsType<DataItem> = [
    {
      title: '索引',
      width: 60,
      render: (_, __, index) => (page - 1) * pageSize + index + 1
    },
    {
      title: '问题 (instruction)',
      dataIndex: 'instruction',
      key: 'instruction',
      ellipsis: true
    },
    {
      title: '回答 (output)',
      dataIndex: 'output',
      key: 'output',
      ellipsis: true
    },
    {
      title: '操作',
      width: 120,
      render: (_, record, index) => (
        <Space>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record, index)}
          />
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDelete(index)}
          />
        </Space>
      )
    }
  ]

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <Space>
          <select
            value={selectedFile}
            onChange={(e) => setSelectedFile(e.target.value)}
            style={{ padding: '4px 8px', borderRadius: 4 }}
          >
            {files.map(file => (
              <option key={file.name} value={file.name}>{file.name}</option>
            ))}
          </select>
          <Input.Search
            placeholder="搜索问题或回答"
            onSearch={(value) => { setSearch(value); setPage(1) }}
            style={{ width: 300 }}
          />
        </Space>
        <Space>
          <Upload
            accept=".json"
            showUploadList={false}
            beforeUpload={handleUpload}
            disabled={uploading}
          >
            <Button icon={<UploadOutlined />} loading={uploading}>上传数据</Button>
          </Upload>
          <Button icon={<PlayCircleOutlined />} onClick={handleLoadDemo}>加载演示数据</Button>
          <Button icon={<DownloadOutlined />} onClick={() => handleExport('jsonl')}>导出 JSONL</Button>
          <Button icon={<DownloadOutlined />} onClick={() => handleExport('llama_factory')}>导出 Llama-Factory</Button>
          <Button icon={<DownloadOutlined />} onClick={() => handleExport('alpaca')}>导出 Alpaca</Button>
          <Button icon={<DownloadOutlined />} onClick={() => handleExport('sharegpt')}>导出 ShareGPT</Button>
        </Space>
      </div>

      <Table
        columns={columns}
        dataSource={data}
        rowKey={(_, index) => index?.toString() || '0'}
        loading={loading}
        pagination={false}
      />

      <div style={{ marginTop: 16, textAlign: 'right' }}>
        <Pagination
          current={page}
          pageSize={pageSize}
          total={total}
          showSizeChanger
          showQuickJumper
          showTotal={(total) => `共 ${total} 条`}
          onChange={(p, ps) => { setPage(p); setPageSize(ps) }}
        />
      </div>

      <Modal
        title="编辑数据"
        open={editModalVisible}
        onOk={handleSaveEdit}
        onCancel={() => setEditModalVisible(false)}
        width={600}
      >
        {editingItem && (
          <div>
            <div style={{ marginBottom: 12 }}>
              <label>问题 (instruction):</label>
              <Input.TextArea
                rows={3}
                value={editingItem.instruction}
                onChange={(e) => setEditingItem({ ...editingItem, instruction: e.target.value })}
              />
            </div>
            <div style={{ marginBottom: 12 }}>
              <label>回答 (output):</label>
              <Input.TextArea
                rows={5}
                value={editingItem.output}
                onChange={(e) => setEditingItem({ ...editingItem, output: e.target.value })}
              />
            </div>
          </div>
        )}
      </Modal>
    </div>
  )
}
