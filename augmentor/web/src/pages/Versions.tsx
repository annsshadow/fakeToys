import { useState, useEffect } from 'react'
import {
  Card,
  Button,
  Space,
  Table,
  Modal,
  Input,
  message,
  Tag,
  Popconfirm,
  type TableColumnsType,
} from 'antd'
import { PlusOutlined, RollbackOutlined, DeleteOutlined, DiffOutlined, HistoryOutlined, EyeOutlined, DatabaseOutlined } from '@ant-design/icons'
import { apiErrorDetail, getVersions, createVersion, deleteVersion, rollbackVersion, diffVersions, getDataFiles, getVersion, getVersionData, getVersionHistory } from '../services/api'
import type { DataItem, VersionDetail, VersionDiffResponse, VersionHistoryEntry, VersionInfo } from '../types/api'

export default function Versions() {
  const [versions, setVersions] = useState<VersionInfo[]>([])
  const [files, setFiles] = useState<string[]>([])
  const [selectedFile, setSelectedFile] = useState('')
  const [createModalVisible, setCreateModalVisible] = useState(false)
  const [diffModalVisible, setDiffModalVisible] = useState(false)
  const [newVersionLabel, setNewVersionLabel] = useState('')
  const [newVersionDesc, setNewVersionDesc] = useState('')
  const [diffVersion1, setDiffVersion1] = useState('')
  const [diffVersion2, setDiffVersion2] = useState('')
  const [diffResult, setDiffResult] = useState<VersionDiffResponse | null>(null)
  const [detailModalVisible, setDetailModalVisible] = useState(false)
  const [detail, setDetail] = useState<VersionDetail | null>(null)
  const [detailItems, setDetailItems] = useState<DataItem[]>([])
  const [historyModalVisible, setHistoryModalVisible] = useState(false)
  const [history, setHistory] = useState<VersionHistoryEntry[]>([])

  useEffect(() => {
    loadVersions()
    loadFiles()
  }, [])

  const loadVersions = async () => {
    try {
      const result = await getVersions()
      setVersions(result.versions)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载版本列表失败'))
    }
  }

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map(f => f.name))
    } catch (err) {
      message.error(apiErrorDetail(err, '加载文件列表失败'))
    }
  }

  const handleCreate = async () => {
    if (!selectedFile) {
      message.warning('请选择文件')
      return
    }

    try {
      await createVersion(selectedFile, newVersionLabel, newVersionDesc)
      message.success('创建版本成功')
      setCreateModalVisible(false)
      setNewVersionLabel('')
      setNewVersionDesc('')
      loadVersions()
    } catch (err) {
      message.error(apiErrorDetail(err, '创建版本失败'))
    }
  }

  const handleRollback = async (versionId: string) => {
    try {
      await rollbackVersion(versionId)
      message.success('回滚成功')
      loadVersions()
    } catch (err) {
      message.error(apiErrorDetail(err, '回滚失败'))
    }
  }

  const handleDelete = async (versionId: string) => {
    try {
      await deleteVersion(versionId)
      message.success('删除成功')
      loadVersions()
    } catch (err) {
      message.error(apiErrorDetail(err, '删除失败'))
    }
  }

  const handleDiff = async () => {
    if (!diffVersion1 || !diffVersion2) {
      message.warning('请选择两个版本')
      return
    }

    try {
      const result = await diffVersions(diffVersion1, diffVersion2)
      setDiffResult(result)
    } catch (err) {
      message.error(apiErrorDetail(err, '对比失败'))
    }
  }

  // 详情与数据分两个按钮、两个端点：`GET /api/versions/{id}` 只回元数据，
  // `GET /api/versions/{id}/data` 要整份加载版本文件——点开一行不该默认付后者的代价。
  const handleShowDetail = async (versionId: string) => {
    try {
      const result = await getVersion(versionId)
      setDetail(result)
      setDetailItems([])
      setDetailModalVisible(true)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载版本详情失败'))
    }
  }

  const handleShowData = async (versionId: string) => {
    try {
      const result = await getVersionData(versionId)
      setDetailItems(result.items)
      if (!detail || detail.version_id !== versionId) {
        setDetail(await getVersion(versionId))
      }
      setDetailModalVisible(true)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载版本数据失败'))
    }
  }

  const handleShowHistory = async () => {
    try {
      const result = await getVersionHistory(50)
      setHistory(result.history)
      setHistoryModalVisible(true)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载操作历史失败'))
    }
  }

  const columns: TableColumnsType<VersionInfo> = [
    {
      title: '版本 ID',
      dataIndex: 'version_id',
      key: 'version_id',
      render: (text: string) => <Tag>{text}</Tag>
    },
    {
      title: '标签',
      dataIndex: 'label',
      key: 'label'
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true
    },
    {
      title: '数据条数',
      dataIndex: 'item_count',
      key: 'item_count'
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at'
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space>
          <Button type="link" size="small" icon={<EyeOutlined />} onClick={() => handleShowDetail(record.version_id)}>详情</Button>
          <Button type="link" size="small" icon={<DatabaseOutlined />} onClick={() => handleShowData(record.version_id)}>数据</Button>
          <Popconfirm
            title="确定回滚到此版本？"
            onConfirm={() => handleRollback(record.version_id)}
          >
            <Button type="link" size="small" icon={<RollbackOutlined />}>回滚</Button>
          </Popconfirm>
          <Popconfirm
            title="确定删除此版本？"
            onConfirm={() => handleDelete(record.version_id)}
          >
            <Button type="link" size="small" danger icon={<DeleteOutlined />}>删除</Button>
          </Popconfirm>
        </Space>
      )
    }
  ]

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={() => setCreateModalVisible(true)}>
            创建版本
          </Button>
          <Button icon={<DiffOutlined />} onClick={() => setDiffModalVisible(true)}>
            版本对比
          </Button>
          <Button icon={<HistoryOutlined />} onClick={handleShowHistory}>
            操作历史
          </Button>
        </Space>
      </div>

      <Table columns={columns} dataSource={versions} rowKey="version_id" />

      <Modal
        title="创建版本"
        open={createModalVisible}
        onOk={handleCreate}
        onCancel={() => setCreateModalVisible(false)}
      >
        <Space direction="vertical" style={{ width: '100%' }}>
          <div>
            <label>选择文件：</label>
            <select
              value={selectedFile}
              onChange={(e) => setSelectedFile(e.target.value)}
              style={{ width: '100%', padding: '4px 8px', borderRadius: 4 }}
            >
              <option value="">请选择文件</option>
              {files.map(file => (
                <option key={file} value={file}>{file}</option>
              ))}
            </select>
          </div>
          <div>
            <label>版本标签：</label>
            <Input
              value={newVersionLabel}
              onChange={(e) => setNewVersionLabel(e.target.value)}
              placeholder="例如: v1.0"
            />
          </div>
          <div>
            <label>版本描述：</label>
            <Input.TextArea
              rows={3}
              value={newVersionDesc}
              onChange={(e) => setNewVersionDesc(e.target.value)}
              placeholder="描述这个版本的内容"
            />
          </div>
        </Space>
      </Modal>

      <Modal
        title="版本对比"
        open={diffModalVisible}
        onOk={handleDiff}
        onCancel={() => { setDiffModalVisible(false); setDiffResult(null) }}
        width={600}
      >
        <Space direction="vertical" style={{ width: '100%' }}>
          <div>
            <label>版本 1：</label>
            <select
              value={diffVersion1}
              onChange={(e) => setDiffVersion1(e.target.value)}
              style={{ width: '100%', padding: '4px 8px', borderRadius: 4 }}
            >
              <option value="">请选择版本</option>
              {versions.map(v => (
                <option key={v.version_id} value={v.version_id}>{v.label} ({v.version_id})</option>
              ))}
            </select>
          </div>
          <div>
            <label>版本 2：</label>
            <select
              value={diffVersion2}
              onChange={(e) => setDiffVersion2(e.target.value)}
              style={{ width: '100%', padding: '4px 8px', borderRadius: 4 }}
            >
              <option value="">请选择版本</option>
              {versions.map(v => (
                <option key={v.version_id} value={v.version_id}>{v.label} ({v.version_id})</option>
              ))}
            </select>
          </div>
          {diffResult && (
            <Card size="small" title="对比结果">
              <Space direction="vertical">
                <div>新增: <Tag color="green">{diffResult.added_count}</Tag></div>
                <div>移除: <Tag color="red">{diffResult.removed_count}</Tag></div>
                <div>修改: <Tag color="orange">{diffResult.modified_count}</Tag></div>
              </Space>
            </Card>
          )}
        </Space>
      </Modal>

      <Modal
        title={`版本详情：${detail?.version_id ?? ''}`}
        open={detailModalVisible}
        onOk={() => setDetailModalVisible(false)}
        onCancel={() => setDetailModalVisible(false)}
        width={720}
        okText="关闭"
        cancelButtonProps={{ style: { display: 'none' } }}
      >
        {detail && (
          <Space direction="vertical" style={{ width: '100%' }}>
            <div>标签：{detail.label}</div>
            <div>描述：{detail.description}</div>
            <div>创建时间：{detail.created_at}</div>
            <div>数据条数：<Tag>{detail.item_count}</Tag></div>
            <div>
              元数据：
              <pre data-testid="version-metadata" style={{ maxHeight: 160, overflow: 'auto' }}>
                {JSON.stringify(detail.metadata, null, 2)}
              </pre>
            </div>
            {detailItems.length > 0 && (
              <Table
                size="small"
                rowKey={(_, index) => String(index)}
                pagination={{ pageSize: 5 }}
                columns={[
                  { title: 'instruction', dataIndex: 'instruction', key: 'instruction', ellipsis: true },
                  { title: 'output', dataIndex: 'output', key: 'output', ellipsis: true }
                ]}
                dataSource={detailItems}
              />
            )}
          </Space>
        )}
      </Modal>

      <Modal
        title="操作历史"
        open={historyModalVisible}
        onOk={() => setHistoryModalVisible(false)}
        onCancel={() => setHistoryModalVisible(false)}
        width={720}
        okText="关闭"
        cancelButtonProps={{ style: { display: 'none' } }}
      >
        <Table
          size="small"
          rowKey={(_, index) => String(index)}
          pagination={{ pageSize: 10 }}
          columns={[
            { title: '操作', dataIndex: 'action', key: 'action' },
            { title: '版本 ID', dataIndex: 'version_id', key: 'version_id' },
            { title: '时间', dataIndex: 'timestamp', key: 'timestamp' },
            {
              title: '详情',
              key: 'detail',
              render: (_, record: VersionHistoryEntry) => JSON.stringify(record.detail)
            }
          ]}
          dataSource={history}
        />
      </Modal>
    </div>
  )
}
