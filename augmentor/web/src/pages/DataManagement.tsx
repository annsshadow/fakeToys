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
import { apiErrorDetail, bulkErrorDetail, isTimeoutError, getDataFiles, loadData, updateDataItem, deleteDataItem, exportData, uploadData, getDemoData } from '../services/api'
import type { DataFileInfo, DataItem } from '../types/api'

/**
 * 上传时可显式声明的源格式
 *
 * 这份字面量与 `augmentor/converter.py` 的 `INPUT_FORMAT_CHOICES` 必须**逐项相等**，
 * 由 `tests/unit/test_upload_declared_format_l92.py` 双向守卫（每轮 pytest 全量都跑得到）。
 * 放在页面而不是 api.ts：它是「用户在 UI 上能选什么」，不是服务层契约。
 * 空串那档（「自动」）不在这里，由 `<option value="">` 单独给。
 */
const UPLOAD_FORMAT_CHOICES = [
  'json', 'jsonl', 'csv', 'tsv', 'alpaca', 'sharegpt', 'chatml',
  'llama_factory', 'vicuna', 'belle', 'excel', 'xlsx', 'xls',
]

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
  const [uploadFormat, setUploadFormat] = useState('')
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
    } catch (err) {
      message.error(apiErrorDetail(err, '加载文件列表失败'))
    }
  }, [])

  const loadDataList = useCallback(async () => {
    setLoading(true)
    try {
      const result = await loadData(selectedFile, page, pageSize, search)
      setData(result.items)
      setTotal(result.total)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载数据失败'))
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
    } catch (err) {
      message.error(bulkErrorDetail(err, '导出失败'))
    }
  }

  /**
   * `format` 默认取页面上的声明值，演示数据那一条会显式传 `json`
   *
   * 两条口径在这里交汇，都得留着：
   * - 声明值来自 `uploadFormat`，因为后端「显式声明 > 扩展名」，而 `<input type=file>`
   *   给不了内容类型之外的信息（无扩展名、扩展名与内容不符都要靠这一格救）；
   * - 调用它的地方必须是 `beforeUpload={(file) => handleUpload(file)}`，不能直接
   *   `beforeUpload={handleUpload}`：antd 把 `fileList` 作为**第二个**实参传给
   *   `beforeUpload`，直传会把这个数组当成格式声明发出去。
   */
  const handleUpload = async (file: File, format = uploadFormat) => {
    setUploading(true)
    try {
      await uploadData(file, format)
      message.success('上传成功')
      loadFiles()
    } catch (err) {
      message.error(bulkErrorDetail(err, '上传失败'))
      // 超时这一支要主动刷一次列表：客户端停止等待不等于服务端失败 —— 字节已被服务端取走
      // 时（axios 的 timeout 到期正是这种优雅关闭），实测服务端照样把整份文件落完
      // （`Temp/l99q/fin_after_drain_l99.py` 的 D 档；同一支的 E 档「交完立刻关闭」就**不**落）。
      // 不刷的话，用户看到的是一句「失败」加一份已经在盘上、列表里却没显示的数据。
      if (isTimeoutError(err)) loadFiles()
    } finally {
      setUploading(false)
    }
    return false
  }

  /**
   * 载入内置演示数据
   *
   * 走服务层而不是裸 `fetch('/api/demo/data')`：端点返回的是**裸数组**，
   * `getDemoData()` 已经把「解包 + 类型」都声明好了，这里只需再包成 `File`
   * 喂给 `uploadData`（上传端点只吃 multipart，不能直接收 JSON body）。
   */
  const handleLoadDemo = async () => {
    setLoading(true)
    try {
      const items = await getDemoData()
      const file = new File([JSON.stringify(items, null, 2)], 'demo_data.json', {
        type: 'application/json'
      })
      // 显式传 `json` 而不是用页面上的声明值：这份内容是我们自己序列化的，格式没有
      // 未知数；若跟着用户的下拉框走，选了 `csv` 的人一点「加载演示数据」就会收到
      // 一句「内容整份是 JSON，但按 csv 解析」——那是判据在替用户撒谎。
      await handleUpload(file, 'json')
    } catch (err) {
      message.error(apiErrorDetail(err, '加载演示数据失败'))
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
      } catch (err) {
        message.error(apiErrorDetail(err, '保存失败'))
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
        } catch (err) {
          message.error(apiErrorDetail(err, '删除失败'))
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
          <select
            value={uploadFormat}
            onChange={(e) => setUploadFormat(e.target.value)}
            style={{ padding: '4px 8px', borderRadius: 4 }}
            aria-label="上传源格式"
            title="显式声明源格式；选「自动」则按文件扩展名推断"
          >
            <option value="">自动（按扩展名）</option>
            {UPLOAD_FORMAT_CHOICES.map((fmt) => (
              <option key={fmt} value={fmt}>{fmt}</option>
            ))}
          </select>
          <Upload
            accept=".json,.jsonl,.csv,.tsv,.xlsx,.xls"
            showUploadList={false}
            beforeUpload={(file) => handleUpload(file)}
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
