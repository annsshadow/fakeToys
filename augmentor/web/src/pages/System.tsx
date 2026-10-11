import { useEffect, useState } from 'react'
import { Alert, AutoComplete, Button, Card, Col, Input, InputNumber, Popconfirm, Row, Select, Space, Statistic, Table, Tag, message } from 'antd'
import { apiErrorDetail, createBackup, deleteBackup, getDataFiles, getDependencyGraph, getServiceStatus, getVectorStatus, listBackups, listDependencyDatasets, migrateDataset, registerDependencyDataset, restoreBackup, runDatasetAutoTest, runQualityMonitor, streamProcess, systemDependencies } from '../services/api'
import type { BackupInfo, DatasetRegistryEntry, DependencyDiagnostics, DependencyGraphResponse, MigrationResponse, QualitySnapshotResponse, StreamReportResponse, TestSuiteResponse } from '../types/api'
import type { StatusResponse, VectorStatusResponse } from '../types/api'

/**
 * `stream` 的 `operation` 档：与 CLI `--operation` 的 choices、与 `/api/system/stream` 认的那
 * 四档**必须逐项相同**（三向对账由 `tests/unit/test_system_ops_wiring_l234.py` 钉住）。
 * 空串那一档是「页面不送、由服务端取默认」的哨兵，服务层把它丢成 `undefined`——送空串是
 * **400**「不支持的 operation: 」（实测），所以这一格不是形状卫生，是躲判决。
 * L234（A313）之前 CLI 那份写的是 `export`：handler 的 `else` 分支只是原样搬运，名字承诺的
 * 导出从来没发生过，而 HTTP 侧合法的 `none` 反被 argparse 拒；两端现已同一份清单。
 */
const STREAM_OPERATION_OPTIONS = [
  { value: '', label: '服务端默认（none）' },
  { value: 'quality', label: 'quality · 按质量分过滤' },
  { value: 'dedup', label: 'dedup · 去重' },
  { value: 'clean', label: 'clean · 清洗后保留' },
  { value: 'none', label: 'none · 原样搬运' },
]

/**
 * `stream` 的 `output_format` 档：权威是 `augmentor/streaming.py` 的 `WRITER_FORMATS`
 * （`("json", "jsonl")`）。parquet 那一档页面不给，因为服务端是 **400**「format 必须是
 * json / jsonl 之一」（实测）。
 */
const STREAM_FORMAT_OPTIONS = [
  { value: '', label: '服务端默认（jsonl）' },
  { value: 'json', label: 'json · 一整个数组' },
  { value: 'jsonl', label: 'jsonl · 逐行' },
]

/**
 * `migrate` 的规则档：权威是 `augmentor/migration.py` 的 `BUILTIN_MIGRATION_RULE_IDS`。
 * 这里**没有**「服务端默认」那一项——一个都不选就是默认（三条全用），placeholder 写着。
 * 实测 `rules: []` 与省略等价，所以「不选」不是「不迁移」，页面上不能给这一档。
 */
const MIGRATION_RULE_OPTIONS = [
  { value: 'rename_instruction', label: 'rename_instruction · instruction → question' },
  { value: 'rename_output', label: 'rename_output · output → answer' },
  { value: 'flatten_conversations', label: 'flatten_conversations · 拍平 conversations' },
]

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
 *
 * 第四张卡是数据集血缘那一族三条（A279 第八批接线，L232）：`GET` / `POST
 * /api/system/dependency/datasets` 与 `GET /api/system/dependency/graph`。它跟上面那张
 * 「依赖明细」不是一件事——那张讲的是**可选依赖包**装没装，这张讲的是**数据集之间**谁由谁
 * 派生，所以标题刻意不叫「依赖」。这一族最要紧的一条实测脾气是：边在产品面上建不出来
 * （没有建边端点，CLI 的 `dependency --action` 也没有），因此这张卡的 `edges` 对纯前端用户
 * 恒为空，页面把这句写在正文里而不是装作画了一张图。
 *
 * 第五张卡是监控与自测两只（A279 第九批接线，L234）：`POST /api/system/monitor` 与
 * `POST /api/system/auto-test`。两支都**不写盘、不挂门禁**，但都是 POST——这一族没有 GET。
 * 它们最要紧的两条实测脾气写在卡片正文里：`alerts` 为空只说明「这一份没触三个阈值」，
 * 不是「质量合格」；`total_tests` 恒 5，`suite` 只认 default，所以页面上不给假下拉。
 *
 * 第六张卡是迁移与流式两只（同批接线）：`POST /api/system/migrate` 与
 * `POST /api/system/stream`，**都写盘、都挂鉴权门禁**。这两支的判决面比上面四张卡都硬：
 * 迁移会改字段名（`instruction` → `question`、`output` → `answer`），输入输出同一个文件时
 * 就地改写而服务端放行；流式的 `processed` 恒等于 `total_input`，取舍只在 `total_output` 上。
 * 两句都写在正文里，按钮上另加二次确认。
 *
 * 「依赖诊断」那两张卡（依赖明细 / 降级功能）在 L236 之前是包在服务状态那道整体门里的：
 * 综合状态拉不动（它要**真构造管道**）就连带看不见「这台机器缺什么」。现在它读
 * `GET /api/system/dependencies`——那条端点不碰管道（把 `get_pipeline` 换成抛异常的替身
 * 仍 200，L236 实测），所以依赖诊断能单独取、单独刷新、在综合状态失败时照样显示。
 * 两面数据**逐字节相同**（同一次实测），因此它涨的是取法不是数据；首载失败时退回综合状态
 * 里那一份，刷新失败时留着上一次读数——不为了「独立」把已经拿到的东西丢掉。
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
  const [datasets, setDatasets] = useState<DatasetRegistryEntry[] | null>(null)
  const [datasetsLoading, setDatasetsLoading] = useState(false)
  const [graph, setGraph] = useState<DependencyGraphResponse | null>(null)
  const [graphLoading, setGraphLoading] = useState(false)
  const [dsFile, setDsFile] = useState('')
  const [dsName, setDsName] = useState('')
  const [dsDesc, setDsDesc] = useState('')
  const [dsTags, setDsTags] = useState('')
  const [dsBusy, setDsBusy] = useState(false)
  // 第五、六张卡（监控/自测 与 迁移/流式）共用一格 busy：'monitor' | 'auto-test' | 'migrate' | 'stream'
  const [opsBusy, setOpsBusy] = useState('')
  const [qaFile, setQaFile] = useState('')
  const [snapshot, setSnapshot] = useState<QualitySnapshotResponse | null>(null)
  const [suiteResult, setSuiteResult] = useState<TestSuiteResponse | null>(null)
  const [wsIn, setWsIn] = useState('')
  const [migOut, setMigOut] = useState('')
  const [migRules, setMigRules] = useState<string[]>([])
  const [migration, setMigration] = useState<MigrationResponse | null>(null)
  const [stOut, setStOut] = useState('')
  const [stOp, setStOp] = useState('')
  const [stChunk, setStChunk] = useState<number | null>(null)
  const [stFmt, setStFmt] = useState('')
  const [streamReport, setStreamReport] = useState<StreamReportResponse | null>(null)
  // 依赖诊断自取一面（L236）：与综合状态里那份逐字节相同，但它走的是不碰管道的端点
  const [deps, setDeps] = useState<DependencyDiagnostics | null>(null)
  const [depsLoading, setDepsLoading] = useState(false)

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

  const refreshDeps = async () => {
    setDepsLoading(true)
    try {
      setDeps(await systemDependencies())
    } catch (err) {
      // 不清 deps：留着上一次那份读数，失败由这句 toast 承担（首载失败时渲染面退回综合状态里那一份）
      message.error(apiErrorDetail(err, '获取依赖诊断失败'))
    } finally {
      setDepsLoading(false)
    }
  }

  useEffect(() => {
    refreshDeps()
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

  // 血缘那一面自取、自报错：注册表目录不存在时这两支只读端点会把它**建出来**（实测），
  // 所以取不到只说明白名单或路径判据挡住了，不该连带把上面几张卡一起判死
  const refreshLineage = async () => {
    setDatasetsLoading(true)
    setGraphLoading(true)
    try {
      setDatasets((await listDependencyDatasets()).datasets)
    } catch (err) {
      setDatasets(null)
      message.error(apiErrorDetail(err, '获取已登记数据集失败'))
    } finally {
      setDatasetsLoading(false)
    }
    try {
      setGraph(await getDependencyGraph())
    } catch (err) {
      setGraph(null)
      message.error(apiErrorDetail(err, '获取数据集依赖图失败'))
    } finally {
      setGraphLoading(false)
    }
  }

  useEffect(() => {
    refreshLineage()
  }, [])

  const handleRegisterDataset = async () => {
    if (!dsName) {
      message.warning('先填数据集名称——登记 id 只由名字算出来')
      return
    }
    if (!dsFile) {
      message.warning('先填要登记的那份服务端数据文件')
      return
    }
    setDsBusy(true)
    try {
      const info = await registerDependencyDataset(dsName, dsFile, {
        description: dsDesc,
        // 中英文逗号都当分隔符：这一格是自由输入，不是受控多选
        tags: dsTags.split(/[,，]/).map((tag) => tag.trim()).filter(Boolean),
      })
      message.success(`已登记 ${info.name}（${info.dataset_id}，${info.item_count} 条）`)
      setDsName('')
      setDsDesc('')
      setDsTags('')
      await refreshLineage()
    } catch (err) {
      message.error(apiErrorDetail(err, '登记数据集失败'))
    } finally {
      setDsBusy(false)
    }
  }

  // 监控与自测都是「读一份数据、回一份判决」，不写盘也不挂门禁，所以共用一格输入、各自报错
  const handleMonitor = async () => {
    if (!qaFile) {
      message.warning('先填要做质量监控的那份服务端数据文件')
      return
    }
    setOpsBusy('monitor')
    try {
      setSnapshot(await runQualityMonitor(qaFile))
    } catch (err) {
      setSnapshot(null)
      message.error(apiErrorDetail(err, '质量监控失败'))
    } finally {
      setOpsBusy('')
    }
  }

  const handleAutoTest = async () => {
    if (!qaFile) {
      message.warning('先填要跑自动化测试的那份服务端数据文件')
      return
    }
    setOpsBusy('auto-test')
    try {
      const result = await runDatasetAutoTest(qaFile)
      setSuiteResult(result)
      message.success(
        `通过率 ${(result.pass_rate * 100).toFixed(0)}%（${result.passed_tests}/${result.total_tests}）`
      )
    } catch (err) {
      setSuiteResult(null)
      message.error(apiErrorDetail(err, '自动化测试失败'))
    } finally {
      setOpsBusy('')
    }
  }

  // 迁移是**破坏性**的：字段被改名，输入输出同一个文件时就地改写（服务端放行）⇒ 二次确认
  const handleMigrate = async () => {
    if (!wsIn) {
      message.warning('先填要迁移的那份服务端数据文件')
      return
    }
    if (!migOut) {
      message.warning('先填迁移产物的文件名')
      return
    }
    setOpsBusy('migrate')
    try {
      const result = await migrateDataset(wsIn, migOut, { rules: migRules })
      setMigration(result)
      message.success(
        `已迁移 ${result.migrated_items}/${result.total_items} 条 ⇒ ${result.target_path}`
      )
    } catch (err) {
      setMigration(null)
      message.error(apiErrorDetail(err, '迁移数据集失败'))
    } finally {
      setOpsBusy('')
    }
  }

  const handleStream = async () => {
    if (!wsIn) {
      message.warning('先填要流式处理的那份服务端数据文件')
      return
    }
    if (!stOut) {
      message.warning('先填流式产物的文件名')
      return
    }
    setOpsBusy('stream')
    try {
      const report = await streamProcess(wsIn, stOut, {
        operation: stOp,
        chunkSize: stChunk ?? undefined,
        outputFormat: stFmt,
      })
      setStreamReport(report)
      message.success(`读入 ${report.total_input} 条，写出 ${report.total_output} 条`)
    } catch (err) {
      setStreamReport(null)
      message.error(apiErrorDetail(err, '流式处理失败'))
    } finally {
      setOpsBusy('')
    }
  }

  // 依赖诊断的两个来源：自己那条端点优先（不碰管道），首载失败时退回综合状态里那一份
  const diag = deps ?? status?.dependencies ?? null

  const installedRows = Object.entries(diag?.installed || {}).map(
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

      {diag && (
        <Card
          title="依赖诊断"
          style={{ marginBottom: 16 }}
          extra={
            <Button onClick={refreshDeps} loading={depsLoading} aria-label="刷新依赖诊断">
              刷新
            </Button>
          }
        >
          {!status && (
            <Alert
              type="info"
              showIcon
              style={{ marginBottom: 16 }}
              message="综合状态没取到，这份依赖诊断来自不碰管道的那条端点"
              description="服务状态要真构造管道，所以它比依赖诊断更容易失败（模型没配、依赖缺失都能让它拉不动）。下面这些读数不依赖它。"
            />
          )}
          <Alert
            type={diag.all_required_present ? 'success' : 'warning'}
            showIcon
            style={{ marginBottom: 16 }}
            message={
              diag.all_required_present
                ? `必需依赖齐备（共 ${diag.available_count} 项可用）`
                : '缺少必需依赖，部分功能将不可用'
            }
            description={
              diag.all_required_present ? undefined : (
                <span>缺失：{diag.missing.join('、') || '未知'}</span>
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
                {(diag.degraded_features || []).length > 0 ? (
                  <Space wrap>
                    {diag.degraded_features.map(feature => (
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
        </Card>
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
          X-API-Key」——到「配置管理 → 写操作鉴权」存一份 key，页面就会随每个请求把它送出去
          （A302 已收）。SDK 里还有 clean_old_backups 与 verify_backup_integrity 两个函数，
          HTTP 面没有入口，所以页面上做不到「清理旧备份」与「单独校验一份备份」（A300）。
        </div>
      </Card>

      <Card
        title="数据集血缘"
        style={{ marginTop: 16 }}
        extra={
          <Button onClick={refreshLineage} loading={datasetsLoading || graphLoading}>
            刷新
          </Button>
        }
      >
        <Space wrap>
          <Input
            aria-label="数据集名称"
            placeholder="数据集名称（登记 id 只由它算）"
            style={{ width: 220 }}
            value={dsName}
            onChange={(e) => setDsName(e.target.value)}
          />
          <AutoComplete
            aria-label="数据集源文件"
            placeholder="服务端数据文件路径"
            style={{ width: 280 }}
            value={dsFile}
            onChange={(value) => setDsFile(value)}
            options={dataFiles.map((path) => ({ value: path }))}
          />
          <Input
            aria-label="数据集描述"
            placeholder="描述"
            style={{ width: 180 }}
            value={dsDesc}
            onChange={(e) => setDsDesc(e.target.value)}
          />
          <Input
            aria-label="数据集标签"
            placeholder="标签，逗号分隔"
            style={{ width: 180 }}
            value={dsTags}
            onChange={(e) => setDsTags(e.target.value)}
          />
          <Button
            type="primary"
            aria-label="登记数据集"
            loading={dsBusy}
            onClick={handleRegisterDataset}
          >
            登记数据集
          </Button>
        </Space>

        {datasets ? (
          <Table
            size="small"
            style={{ marginTop: 12 }}
            rowKey="dataset_id"
            loading={datasetsLoading}
            dataSource={datasets}
            pagination={false}
            columns={[
              { title: '名称', dataIndex: 'name' },
              { title: '登记 id', dataIndex: 'dataset_id', width: 100 },
              { title: '条数', dataIndex: 'item_count', width: 70 },
              {
                title: '描述',
                dataIndex: 'description',
                render: (description: string) =>
                  description || <span style={{ opacity: 0.45 }}>（空）</span>
              },
              {
                title: '标签',
                dataIndex: 'tags',
                width: 160,
                render: (tags: string[]) =>
                  tags.length > 0 ? (
                    tags.map((tag) => <Tag key={tag}>{tag}</Tag>)
                  ) : (
                    <span style={{ opacity: 0.45 }}>（无）</span>
                  )
              },
              { title: '服务端路径', dataIndex: 'path', ellipsis: true },
              { title: '更新时间', dataIndex: 'updated_at', width: 190 }
            ]}
          />
        ) : (
          <Alert
            type="warning"
            showIcon
            style={{ marginTop: 12 }}
            message="已登记数据集未取到"
            description="上面几张卡不受影响：每一面各自独立取。"
          />
        )}

        <Row gutter={16} style={{ marginTop: 12 }}>
          <Col span={6}>
            <Statistic title="图中节点" value={graph ? graph.nodes.length : '-'} />
          </Col>
          <Col span={6}>
            <Statistic title="图中边" value={graph ? graph.edges.length : '-'} />
          </Col>
          <Col span={12}>
            <Statistic
              title="校验问题"
              value={graph ? graph.issues.length : '-'}
              valueStyle={{
                color: graph && graph.issues.length > 0 ? '#cf1322' : undefined
              }}
            />
          </Col>
        </Row>

        {graph && graph.issues.length > 0 && (
          <Alert
            type="error"
            showIcon
            style={{ marginTop: 12 }}
            message={`依赖关系上有 ${graph.issues.length} 处问题`}
            description={
              <ul style={{ margin: 0, paddingLeft: 18 }}>
                {graph.issues.map((issue) => (
                  <li key={`${issue.type}:${issue.message}`}>
                    {issue.type}：{issue.message}
                  </li>
                ))}
              </ul>
            }
          />
        )}

        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          「图中边」这一格对纯前端用户**恒为 0**，不是没数据：三条端点里没有建边入口，CLI 的
          dependency --action 也只有 register/list/graph/validate，边只能由 SDK 的
          add_dependency 建立（A309）。同理，登记过的数据集**在产品面上删不掉**——HTTP 没有
          注销端点，连 SDK 都没有 remove_dataset，只能手改注册表里的 datasets.json。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          「校验问题」只报**悬空端点**（missing_source / missing_target），**不查环**：实测
          A→B 与 B→A 成环时这一格仍是 0，所以 0 不能读成「依赖关系没问题」（A310）。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          登记 id 是 md5(名称) 的前 8 位 ⇒ **同名再登记一次是覆盖同一条**（不留重复），但
          created_at 会被刷新成第二次的时间，且没有 409、没有任何「已存在」提示（A306）。
          源文件受白名单管：不存在是 404「文件不存在」，白名单外是 403，带 .. 是 400，
          顶层不是 JSON 数组是 400「数据文件的顶层必须是 JSON 数组」（四档实测）。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          描述与标签这两格在 L232 之前是**回显即丢**：POST 的响应里有，随后 GET 与磁盘上都是
          空（端点在落盘之后才改内存对象）。本轮已改成随登记一起写下去（A305）。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          registry_path 这一格页面不送，落点由服务端决定：白名单首个根目录下的
          .dependency_registry（出厂 data/.dependency_registry）。CLI 的 --registry-path 默认值
          是 .dependency_registry（**相对工作目录**），两边不是同一份，所以 CLI 登记的数据集在
          这张卡上看不见（A308）。另外这两支只读端点在注册表目录不存在时会**把它建出来**
          （A307），刷一下页面就在服务端留了个空目录。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          登记这一支挂鉴权门禁（两条 GET 不挂）：服务端设了 AUGMENTOR_API_KEY 时它会回 401
          「缺少或无效的 X-API-Key」，那份 key 在「配置管理 → 写操作鉴权」那张卡里存，存了就会
          随每个请求送出去（A302 / A312 已收）。缺必填格时它回的是 FastAPI 的英文 422，其余坏
          参数是中文 400/403/404 —— 两种错误体都要认（A311）。
        </div>
      </Card>

      <Card title="质量监控与自测" style={{ marginTop: 16 }}>
        <Space wrap>
          <AutoComplete
            aria-label="监控数据文件"
            placeholder="服务端数据文件路径"
            style={{ width: 300 }}
            value={qaFile}
            onChange={(value) => setQaFile(value)}
            options={dataFiles.map((path) => ({ value: path }))}
          />
          <Button
            aria-label="生成质量快照"
            loading={opsBusy === 'monitor'}
            onClick={handleMonitor}
          >
            质量监控快照
          </Button>
          <Button
            aria-label="运行自动化测试"
            loading={opsBusy === 'auto-test'}
            onClick={handleAutoTest}
          >
            运行自动化测试
          </Button>
        </Space>

        {snapshot && (
          <>
            <Row gutter={16} style={{ marginTop: 12 }}>
              <Col span={6}>
                <Statistic title="完整度" value={snapshot.metrics.completeness ?? '-'} />
              </Col>
              <Col span={6}>
                <Statistic title="多样性" value={snapshot.metrics.diversity ?? '-'} />
              </Col>
              <Col span={6}>
                <Statistic title="一致性" value={snapshot.metrics.consistency ?? '-'} />
              </Col>
              <Col span={6}>
                <Statistic
                  title="触发告警"
                  value={snapshot.alerts.length}
                  valueStyle={{
                    color: snapshot.alerts.length > 0 ? '#cf1322' : undefined
                  }}
                />
              </Col>
            </Row>
            {snapshot.alerts.length > 0 && (
              <Alert
                type="error"
                showIcon
                style={{ marginTop: 12 }}
                message={`快照 ${snapshot.snapshot_id} 触发 ${snapshot.alerts.length} 条告警`}
                description={
                  <ul style={{ margin: 0, paddingLeft: 18 }}>
                    {snapshot.alerts.map((alert) => (
                      <li key={alert.alert_id}>
                        <Tag color="red">{alert.severity}</Tag>
                        {alert.message}
                      </li>
                    ))}
                  </ul>
                }
              />
            )}
          </>
        )}

        {suiteResult && (
          <>
            <Row gutter={16} style={{ marginTop: 12 }}>
              <Col span={6}>
                <Statistic
                  title="通过率"
                  value={suiteResult.pass_rate * 100}
                  precision={0}
                  suffix="%"
                  valueStyle={{
                    color: suiteResult.failed_tests > 0 ? '#cf1322' : undefined
                  }}
                />
              </Col>
              <Col span={6}>
                <Statistic title="通过" value={suiteResult.passed_tests} />
              </Col>
              <Col span={6}>
                <Statistic title="失败" value={suiteResult.failed_tests} />
              </Col>
              <Col span={6}>
                <Statistic title="套件" value={suiteResult.name} />
              </Col>
            </Row>
            <Table
              size="small"
              style={{ marginTop: 12 }}
              rowKey="test_id"
              pagination={false}
              dataSource={suiteResult.results}
              columns={[
                { title: '测试', dataIndex: 'test_name', width: 120 },
                {
                  title: '结果',
                  dataIndex: 'passed',
                  width: 80,
                  render: (passed: boolean) =>
                    passed ? <Tag color="green">通过</Tag> : <Tag color="red">失败</Tag>
                },
                { title: '说明', dataIndex: 'message' },
                {
                  title: '耗时 ms',
                  dataIndex: 'execution_time_ms',
                  width: 100,
                  render: (ms: number) => ms.toFixed(2)
                }
              ]}
            />
          </>
        )}

        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          「触发告警」为 0 只说明这一份没触三个阈值（实测 completeness 0.7 / diversity 0.4 /
          consistency 0.7），**不是「质量合格」**：这一支只比这三个数，字段缺失、重复、长度
          一概不查——那些要看下面「运行自动化测试」的五条。告警正文是**服务端英文原文**
          （形如 completeness below threshold: 0.50 &lt; 0.70），没有中文那一份。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          三个指标格不是恒在：数据集是空数组时服务端回 metrics 为**空对象**（实测），这三格
          显示「-」而不是 0——0 与「没算」是两件事，页面不把它们混成一个读数。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          自动化测试恒为五条（格式检查 / 字段检查 / 质量检查 / 多样性检查 / 重复检查），
          通过率是 0–1 的小数、这里乘了 100 显示。suite 这一格页面**不送**：服务端只认
          default，拼错与空串都是 400「未知的测试套件」，省略才走默认，所以给一个只有一项的
          下拉只是噪音。说明那一列的明细是服务端**截断过的**（实测最多列 5 条，形如
          「发现 12 个缺失字段: [...]」），列出来的条数不等于总数。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          这两支都不写盘、也**不挂鉴权门禁**（第六张卡那两支挂），但都是 POST 而不是 GET——
          这一族没有只读 GET。源文件受白名单管：不存在 404「文件不存在」、白名单外 403、
          带 .. 是 400「路径包含非法组件」、缺必填是 FastAPI 的英文 422（四档实测）。
        </div>
      </Card>

      <Card title="迁移与流式处理" style={{ marginTop: 16 }}>
        <Space wrap>
          <AutoComplete
            aria-label="源数据文件"
            placeholder="服务端数据文件路径（两支共用）"
            style={{ width: 300 }}
            value={wsIn}
            onChange={(value) => setWsIn(value)}
            options={dataFiles.map((path) => ({ value: path }))}
          />
        </Space>

        <Space wrap style={{ marginTop: 8 }}>
          <AutoComplete
            aria-label="迁移产物文件"
            placeholder="迁移产物文件名"
            style={{ width: 240 }}
            value={migOut}
            onChange={(value) => setMigOut(value)}
            options={dataFiles.map((path) => ({ value: path }))}
          />
          <Select
            aria-label="迁移规则"
            mode="multiple"
            style={{ minWidth: 320 }}
            placeholder="服务端默认（三条全用）"
            value={migRules}
            onChange={(value) => setMigRules(value)}
            options={MIGRATION_RULE_OPTIONS}
          />
          <Popconfirm
            title="迁移会改字段名，产物已存在则整份覆盖"
            description="instruction → question、output → answer；源与产物填同一个名字就是就地改写。"
            okText="照改"
            cancelText="取消"
            onConfirm={handleMigrate}
          >
            <Button type="primary" danger aria-label="迁移数据集结构" loading={opsBusy === 'migrate'}>
              迁移结构
            </Button>
          </Popconfirm>
        </Space>

        {migration && (
          <>
            <Row gutter={16} style={{ marginTop: 12 }}>
              <Col span={6}>
                <Statistic title="读入条数" value={migration.total_items} />
              </Col>
              <Col span={6}>
                <Statistic title="迁移条数" value={migration.migrated_items} />
              </Col>
              <Col span={6}>
                <Statistic
                  title="失败条数"
                  value={migration.failed_items}
                  valueStyle={{
                    color: migration.failed_items > 0 ? '#cf1322' : undefined
                  }}
                />
              </Col>
              <Col span={6}>
                <Statistic title="迁移 id" value={migration.migration_id} />
              </Col>
            </Row>
            <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
              用到的规则：{migration.rules_applied.join(' / ') || '（服务端回了一张空清单）'}
              {' · 产物落在 '}
              {migration.target_path}
            </div>
          </>
        )}

        <Space wrap style={{ marginTop: 8 }}>
          <AutoComplete
            aria-label="流式产物文件"
            placeholder="流式产物文件名"
            style={{ width: 240 }}
            value={stOut}
            onChange={(value) => setStOut(value)}
            options={dataFiles.map((path) => ({ value: path }))}
          />
          <Select
            aria-label="流式操作"
            style={{ width: 220 }}
            value={stOp}
            onChange={(value) => setStOp(value)}
            options={STREAM_OPERATION_OPTIONS}
          />
          <Select
            aria-label="产物格式"
            style={{ width: 200 }}
            value={stFmt}
            onChange={(value) => setStFmt(value)}
            options={STREAM_FORMAT_OPTIONS}
          />
          <InputNumber
            aria-label="分块大小"
            min={1}
            placeholder="分块大小（默认 1000）"
            style={{ width: 180 }}
            value={stChunk}
            onChange={(value) => setStChunk(value)}
          />
          <Button type="primary" aria-label="流式处理" loading={opsBusy === 'stream'} onClick={handleStream}>
            流式处理
          </Button>
        </Space>

        {streamReport && (
          <Row gutter={16} style={{ marginTop: 12 }}>
            <Col span={8}>
              <Statistic title="读入条数" value={streamReport.total_input} />
            </Col>
            <Col span={8}>
              <Statistic title="写出条数" value={streamReport.total_output} />
            </Col>
            <Col span={8}>
              <Statistic
                title="被丢掉"
                value={streamReport.total_input - streamReport.total_output}
                valueStyle={{
                  color:
                    streamReport.total_input - streamReport.total_output > 0
                      ? '#cf1322'
                      : undefined
                }}
              />
            </Col>
          </Row>
        )}

        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          这两支都**写盘**、都挂鉴权门禁：服务端设了 AUGMENTOR_API_KEY 时会回 401「缺少或无效的
          X-API-Key」，那份 key 在「配置管理 → 写操作鉴权」那张卡里存（A302 / A312 已收）。
          产物已存在时**整份覆盖**，没有 409、也没有「已存在」提示（实测同名再写一次字节相同）。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          「被丢掉」是本页自己算的（读入 − 写出）：回执里的 processed **各档实测都等于读入条数**
          （四档全是 5），它数的是「逐条看过」而不是「改动过」⇒ 不能拿它当取舍读数。同一份
          5 条夹具上四档的实测取舍是 none 5 / quality 2 / dedup 3 / clean 5 —— clean 那档**一条不丢，改写内容**：它只动 instruction（全角标点换半角、剥首尾空白），output 里的 HTML 标签与空白原样留着。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          产物格式与文件扩展名**不一致时以格式为准、扩展名不改**（实测 format=json 写进
          .jsonl 文件名里，内容是一整个 JSON 数组）。chunk_size 最小 1：0 与 -1 都是 400
          「chunk_size 必须是不小于 1 的整数」。这一支**不调模型**（与「数据增强」那支的区别），
          所以同步返回报告而不是丢进后台任务。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          迁移是**破坏性结构变化**：字段被改名，且规则清单**留空等于三条全用**（实测 rules 为
          空数组与省略等价），不是「不迁移」——所以那一格没有「不迁移」这一档。
          flatten_conversations 对没有 conversations 字段的数据**静默无操作**，回执里迁移条数
          仍等于读入条数、errors 为空，看不出这条规则其实没干成事。回执里的源与产物路径都是
          服务端 resolve 后的**绝对路径**。
        </div>
      </Card>
    </div>
  )
}
