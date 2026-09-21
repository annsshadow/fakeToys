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
import { PlusOutlined, RollbackOutlined, DeleteOutlined, DiffOutlined } from '@ant-design/icons'
import { getVersions, createVersion, deleteVersion, rollbackVersion, diffVersions, getDataFiles } from '../services/api'
import type { VersionDiffResponse, VersionInfo } from '../types/api'

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

  useEffect(() => {
    loadVersions()
    loadFiles()
  }, [])

  const loadVersions = async () => {
    try {
      const result = await getVersions()
      setVersions(result.versions)
    } catch {
      message.error('加载版本列表失败')
    }
  }

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map(f => f.name))
    } catch {
      message.error('加载文件列表失败')
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
    } catch {
      message.error('创建版本失败')
    }
  }

  const handleRollback = async (versionId: string) => {
    try {
      await rollbackVersion(versionId)
      message.success('回滚成功')
      loadVersions()
    } catch {
      message.error('回滚失败')
    }
  }

  const handleDelete = async (versionId: string) => {
    try {
      await deleteVersion(versionId)
      message.success('删除成功')
      loadVersions()
    } catch {
      message.error('删除失败')
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
    } catch {
      message.error('对比失败')
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
    </div>
  )
}
