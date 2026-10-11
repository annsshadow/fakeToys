import { useEffect, useState } from 'react'
import { Alert, AutoComplete, Button, Card, Col, Input, Popconfirm, Row, Space, Statistic, Table, Tag, message } from 'antd'
import { apiErrorDetail, createBackup, deleteBackup, getDataFiles, getServiceStatus, getVectorStatus, listBackups, restoreBackup } from '../services/api'
import type { BackupInfo } from '../types/api'
import type { StatusResponse, VectorStatusResponse } from '../types/api'

/**
 * 系统状态
 *
 * 展示 `GET /api/status`：与仪表盘上的 `GET /api/health` 是两件事 ——
 * health 只报存活（给容器探针用，刻意不含可能失败的字段），status 会真的
 * 构造管道并逐个检查可选依赖，因此能回答「当前环境缺什么、哪些功能会降级」。
 *
 * 下面那张卡是 `GET /api/dataset/vector-status`：向量库的闸门生效值与磁盘上到底有没有
 * 一份可读回的索引。它和服务状态是两回事，所以单独取、单独刷新。
 *
 * 第三张卡是备份那一族四只端点（A279 第七批接线，L231）：`GET` / `POST /api/system/backups`、
 * `POST …/{backup_id}/restore`、`DELETE …/{backup_id}`。它同样自取、自报错——备份面挂了不该
 * 把上面两张卡一起拖掉。这一族的实测脾气（同名覆盖留两条索引、校验和不拦恢复、落点键叫
 * `output_path`）写在卡片正文里，别处不复述。
 */
export default function System() {
  const [status, setStatus] = useState<StatusResponse | null>(null)
  const [loading, setLoading] = useState(false)
  const [vector, setVector] = useState<VectorStatusResponse | null>(null)
  const [vectorLoading, setVectorLoading] = useState(false)
  const [backups, setBackups] = useState<BackupInfo[] | null>(null)
  const [backupsLoading, setBackupsLoading] = useState(false)
  const [dataFiles, setDataFiles] = useState<string[]>([])
  const [sourceFile, setSourceFile] = useState('')
  const [backupName, setBackupName] = useState('')
  const [restoreTarget, setRestoreTarget] = useState('')
  // 一格 busy 同时管三只写操作：'create' 或某个 backup_id，避免同一行被点两次
  const [busy, setBusy] = useState('')

  const refresh = async () => {
    setLoading(true)
    try {
      setStatus(await getServiceStatus())
    } catch (err) {
      message.error(apiErrorDetail(err, '获取服务状态失败，请确认后端已启动'))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    refresh()
  }, [])

  const refreshVector = async () => {
    setVectorLoading(true)
    try {
      setVector(await getVectorStatus())
    } catch (err) {
      message.error(apiErrorDetail(err, '获取向量库状态失败'))
    } finally {
      setVectorLoading(false)
    }
  }

  // 向量库那一面单独取、单独报错：它失败不该把整页服务状态一起拖掉
  useEffect(() => {
    refreshVector()
  }, [])

  const refreshBackups = async () => {
    setBackupsLoading(true)
    try {
      setBackups((await listBackups()).backups)
    } catch (err) {
      message.error(apiErrorDetail(err, '获取备份列表失败'))
    } finally {
      setBackupsLoading(false)
    }
  }

  useEffect(() => {
    refreshBackups()
  }, [])

  // 备份源是**服务端上的路径**，不是浏览器里的文件；候选来自 /api/data/list。
  // 这一面失败只影响候选，不该连带把备份列表也判死，所以只清候选并报一句。
  useEffect(() => {
    getDataFiles()
      .then((result) => setDataFiles(result.files.map((file) => file.path)))
      .catch((err) => {
        setDataFiles([])
        message.error(apiErrorDetail(err, '获取数据文件列表失败，备份源需手填路径'))
      })
  }, [])

  const handleCreateBackup = async () => {
    if (!sourceFile) {
      message.warning('先填要备份的那份服务端数据文件')
      return
    }
    setBusy('create')
    try {
      const info = await createBackup(sourceFile, { name: backupName })
      message.success(`已备份 ${info.item_count} 条 ⇒ ${info.backup_id}`)
      setBackupName('')
      await refreshBackups()
    } catch (err) {
      message.error(apiErrorDetail(err, '创建备份失败'))
    } finally {
      setBusy('')
    }
  }

  const handleRestore = async (row: BackupInfo) => {
    if (!restoreTarget) {
      message.warning('先填恢复目标文件名')
      return
    }
    setBusy(row.backup_id)
    try {
      const result = await restoreBackup(row.backup_id, restoreTarget)
      message.success(`已恢复 ${result.item_count} 条 ⇒ ${result.output_path}`)
    } catch (err) {
      message.error(apiErrorDetail(err, '恢复备份失败'))
    } finally {
      setBusy('')
    }
  }

  const handleDelete = async (row: BackupInfo) => {
    setBusy(row.backup_id)
    try {
      await deleteBackup(row.backup_id)
      message.success(`已删除备份 ${row.backup_id}`)
      await refreshBackups()
    } catch (err) {
      message.error(apiErrorDetail(err, '删除备份失败'))
    } finally {
      setBusy('')
    }
  }

  const installedRows = Object.entries(status?.dependencies.installed || {}).map(
    ([name, available]) => ({ name, available })
  )

  return (
    <div>
      <Card
        title="服务状态"
        style={{ marginBottom: 16 }}
        extra={
          <Button onClick={refresh} loading={loading}>
            刷新
          </Button>
        }
      >
        <Row gutter={16}>
          <Col span={6}>
            <Statistic
              title="运行状态"
              value={status?.status === 'ok' ? '正常' : '未知'}
              valueStyle={{ color: status?.status === 'ok' ? '#3f8600' : '#cf1322' }}
            />
          </Col>
          <Col span={6}>
            <Statistic title="版本" value={status?.version || '-'} />
          </Col>
          <Col span={6}>
            <Statistic title="默认模型" value={status?.model_default || '-'} />
          </Col>
          <Col span={6}>
            <Statistic
              title="模型可用"
              value={status ? (status.model_available ? '是' : '否') : '-'}
              valueStyle={{
                color: status?.model_available ? '#3f8600' : '#cf1322'
              }}
            />
          </Col>
        </Row>
      </Card>

      {status && (
        <>
          <Alert
            type={status.dependencies.all_required_present ? 'success' : 'warning'}
            showIcon
            style={{ marginBottom: 16 }}
            message={
              status.dependencies.all_required_present
                ? `必需依赖齐备（共 ${status.dependencies.available_count} 项可用）`
                : '缺少必需依赖，部分功能将不可用'
            }
            description={
              status.dependencies.all_required_present ? undefined : (
                <span>缺失：{status.dependencies.missing.join('、') || '未知'}</span>
              )
            }
          />

          <Row gutter={16}>
            <Col span={12}>
              <Card title="依赖明细">
                <Table
                  size="small"
                  rowKey="name"
                  pagination={false}
                  columns={[
                    { title: '依赖', dataIndex: 'name' },
                    {
                      title: '状态',
                      dataIndex: 'available',
                      width: 100,
                      render: (available: boolean) => (
                        <Tag color={available ? 'green' : 'red'}>
                          {available ? '已安装' : '未安装'}
                        </Tag>
                      )
                    }
                  ]}
                  dataSource={installedRows}
                />
              </Card>
            </Col>
            <Col span={12}>
              <Card title="降级功能">
                {(status.dependencies.degraded_features || []).length > 0 ? (
                  <Space wrap>
                    {status.dependencies.degraded_features.map(feature => (
                      <Tag key={feature} color="orange">
                        {feature}
                      </Tag>
                    ))}
                  </Space>
                ) : (
                  <Alert type="success" showIcon message="没有功能因依赖缺失而降级" />
                )}
              </Card>
            </Col>
          </Row>
        </>
      )}

      <Card
        title="向量库状态"
        style={{ marginTop: 16 }}
        extra={
          <Button onClick={refreshVector} loading={vectorLoading}>
            刷新
          </Button>
        }
      >
        {vector ? (
          <>
            <Row gutter={16}>
              <Col span={6}>
                <Statistic
                  title="自动建库"
                  value={vector.enabled ? '开' : '关'}
                  valueStyle={{ color: vector.enabled ? '#3f8600' : '#cf1322' }}
                />
              </Col>
              <Col span={6}>
                <Statistic
                  title="磁盘索引"
                  value={vector.exists ? '已存在' : '不存在'}
                  valueStyle={{ color: vector.exists ? '#3f8600' : '#cf1322' }}
                />
              </Col>
              <Col span={6}>
                <Statistic title="向量条数" value={vector.count} />
              </Col>
              <Col span={6}>
                <Statistic title="维度" value={vector.dimension} />
              </Col>
            </Row>
            <Space wrap style={{ marginTop: 12 }}>
              <Tag color="blue">backend: {vector.backend}</Tag>
              <Tag color="purple">collection: {vector.collection}</Tag>
              <Tag>storage_dir: {vector.storage_dir || '未配置'}</Tag>
            </Space>
          </>
        ) : (
          <Alert
            type="warning"
            showIcon
            message="向量库状态未取到"
            description="上面那张服务状态不受影响：两面各自独立取。"
          />
        )}
      </Card>

      <Card
        title="数据备份"
        style={{ marginTop: 16 }}
        extra={
          <Button onClick={refreshBackups} loading={backupsLoading}>
            刷新
          </Button>
        }
      >
        <Space wrap>
          <AutoComplete
            aria-label="备份源文件"
            placeholder="服务端数据文件路径"
            style={{ width: 280 }}
            value={sourceFile}
            onChange={(value) => setSourceFile(value)}
            options={dataFiles.map((path) => ({ value: path }))}
          />
          <Input
            aria-label="备份名称"
            placeholder="备份名称（留空则用时间戳 id）"
            style={{ width: 240 }}
            value={backupName}
            onChange={(e) => setBackupName(e.target.value)}
          />
          <Button
            type="primary"
            aria-label="创建备份"
            loading={busy === 'create'}
            onClick={handleCreateBackup}
          >
            创建备份
          </Button>
        </Space>

        <Space wrap style={{ marginTop: 12 }}>
          <Input
            aria-label="恢复目标文件名"
            placeholder="恢复目标文件名，如 restored.json"
            style={{ width: 280 }}
            value={restoreTarget}
            onChange={(e) => setRestoreTarget(e.target.value)}
          />
          <span style={{ fontSize: 12, opacity: 0.75 }}>
            填好目标文件名后点某一行的「恢复」；目标已存在会被整份覆盖
          </span>
        </Space>

        {backups ? (
          <Table
            size="small"
            style={{ marginTop: 12 }}
            // 索引里可以有两条同 id 的记录（同名创建两次，实测时间戳不同），所以行键
            // 拼上时间戳；不用 antd 那个 `index` 形参——它已被标记 deprecated
            rowKey={(row) => `${row.backup_id}#${row.timestamp}`}
            dataSource={backups}
            pagination={false}
            columns={[
              { title: '备份 id', dataIndex: 'backup_id' },
              { title: '时间', dataIndex: 'timestamp', width: 190 },
              { title: '条数', dataIndex: 'item_count', width: 70 },
              { title: '字节', dataIndex: 'file_size', width: 80 },
              {
                title: '校验和（sha256）',
                dataIndex: 'checksum',
                width: 130,
                render: (checksum: string) => (
                  <span title={checksum}>{checksum.slice(0, 12)}</span>
                )
              },
              {
                title: '操作',
                width: 150,
                render: (_: unknown, row: BackupInfo) => (
                  <Space>
                    <Button
                      size="small"
                      aria-label={`恢复备份 ${row.backup_id}`}
                      loading={busy === row.backup_id}
                      onClick={() => handleRestore(row)}
                    >
                      恢复
                    </Button>
                    <Popconfirm
                      title={`删除备份 ${row.backup_id}？`}
                      description="同 id 的索引记录会一起消失，且那份文件从磁盘删掉"
                      onConfirm={() => handleDelete(row)}
                    >
                      <Button size="small" danger aria-label={`删除备份 ${row.backup_id}`}>
                        删除
                      </Button>
                    </Popconfirm>
                  </Space>
                )
              }
            ]}
          />
        ) : (
          <Alert
            type="warning"
            showIcon
            style={{ marginTop: 12 }}
            message="备份列表未取到"
            description="上面两张卡不受影响：三面各自独立取。"
          />
        )}

        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          源文件与恢复目标都受**同一份白名单**管：白名单外的绝对路径是 403「路径超出允许的数据
          目录范围」，带 .. 的是 400「路径包含非法组件」，恢复目标为空串是 400「文件路径 不能为
          空」（三档实测）。下拉里的候选来自 /api/data/list，而那一只端点**只列白名单根目录下
          名为 train_data*.json 的文件**——别的文件要手填路径，这也是这一格用可输入下拉而不是
          纯下拉的原因。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          备份 id 就是创建时填的名称，留空才由服务端取 backup_&lt;时间戳&gt;。同一个名称创建两次
          会**覆盖同一份文件**、并在索引里留下两条同 id 记录（实测：列表回三条、磁盘只有一份），
          而删除是按 id 过滤的 ⇒ 删一次两条一起消失（A298）。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          checksum 是备份文件的 sha256，但它**只在恢复时写日志、不拦截恢复**：把备份文件改坏再
          恢复照样 200，交回来的就是改坏的内容（A299）。别把这一格读成完整性保证。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          恢复的回执里落点键叫 output_path，而同族其余五只写盘端点叫 output_file（split 叫
          output_dir）；这一格是 SDK 的返回键原样上线，不是笔误。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          backup_dir 这一格页面不送，落点由服务端决定：白名单首个根目录下的 .backups（出厂
          data/.backups）。CLI 的 --backup-dir 默认值是 .backups（相对工作目录），**与这里看到的
          不是同一个目录**（A301）。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          服务端设了 AUGMENTOR_API_KEY 时，这一族的三只写操作会回 401「缺少或无效的
          X-API-Key」——前端目前没有送这个头的通道（A302）。SDK 里还有 clean_old_backups 与
          verify_backup_integrity 两个函数，HTTP 面没有入口，所以页面上做不到「清理旧备份」
          与「单独校验一份备份」（A300）。
        </div>
      </Card>
    </div>
  )
}
