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
  Card,
  Checkbox,
  InputNumber,
  type TableColumnsType,
} from 'antd'
import { DownloadOutlined, DeleteOutlined, EditOutlined, UploadOutlined, PlayCircleOutlined } from '@ant-design/icons'
import { apiErrorDetail, bulkErrorDetail, isTimeoutError, getDataFiles, loadData, updateDataItem, deleteDataItem, exportData, uploadData, getDemoData, convertDataset, mergeDatasets, splitDataset } from '../services/api'
import type { ConvertResponse, DataFileInfo, DataItem, MergeResponse, SplitResponse } from '../types/api'

/**
 * 上传时可显式声明的源格式
 *
 * 这份字面量与 `augmentor/converter.py` 的 `INPUT_FORMAT_CHOICES` 必须**逐项相等**，
 * 由 `tests/unit/test_upload_declared_format_l92.py` 双向守卫（每轮 pytest 全量都跑得到）。
 * 放在页面而不是 api.ts：它是「用户在 UI 上能选什么」，不是服务层契约。
 * 空串那档（「自动」）不在这里，由 `<option value="">` 单独给。
 *
 * L229 起它同时是**转换卡「源格式」下拉**的清单：读边只有这一份权威。
 */
const UPLOAD_FORMAT_CHOICES = [
  'json', 'jsonl', 'csv', 'tsv', 'alpaca', 'sharegpt', 'chatml',
  'llama_factory', 'vicuna', 'belle',
  'excel', 'xlsx', 'xls',
]

/**
 * 转换卡「目标格式」下拉——与源格式那张表**不是同一份清单**
 *
 * 权威是 `augmentor/converter.py` 的 `OUTPUT_FORMAT_CHOICES`，由
 * `tests/unit/test_transform_wiring_l229.py` 逐项对账。差的那一个是 `xls`：读边认它
 * （openpyxl 读得了老 BIFF），写边**刻意不认**（写 `.xls` 会得到「扩展名是 xls、字节是
 * zip」的假容器），所以它只能出现在源下拉里。给 `target_format` 发 `xls` 的回执是 400，
 * 且 detail 里逐条列出这十二个合法目标——页面不必再自造一份解释。
 */
const CONVERT_TARGET_FORMATS = [
  'json', 'jsonl', 'csv', 'tsv', 'alpaca', 'sharegpt', 'chatml',
  'llama_factory', 'vicuna', 'belle', 'excel', 'xlsx',
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

  // ---- L229 写盘型变换三张卡（A279 第五批接线）----
  // 初值逐个等于请求模型的默认（`target_format="jsonl"`、`source_format=None`、
  // `deduplicate=True`、`0.8/0.1/0.1`、`seed=None`、`stratify=False`、
  // `stratify_key="instruction"`、`shuffle=True`），由 l229 的守卫逐项对账；
  // `output_file` / `output_dir` 在契约里是必填，页面**不给默认**，空着就发不出去。
  const [convertTarget, setConvertTarget] = useState('jsonl')
  const [convertSource, setConvertSource] = useState('')
  const [convertOut, setConvertOut] = useState('')
  const [convertResult, setConvertResult] = useState<ConvertResponse | null>(null)
  const [mergeInputs, setMergeInputs] = useState<string[]>([])
  const [mergeOut, setMergeOut] = useState('')
  const [mergeDedup, setMergeDedup] = useState(true)
  const [mergeResult, setMergeResult] = useState<MergeResponse | null>(null)
  const [splitOutDir, setSplitOutDir] = useState('')
  const [trainRatio, setTrainRatio] = useState<number | null>(0.8)
  const [valRatio, setValRatio] = useState<number | null>(0.1)
  const [testRatio, setTestRatio] = useState<number | null>(0.1)
  const [splitSeed, setSplitSeed] = useState<number | null>(null)
  const [splitStratify, setSplitStratify] = useState(false)
  const [stratifyKey, setStratifyKey] = useState('instruction')
  const [splitShuffle, setSplitShuffle] = useState(true)
  const [splitResult, setSplitResult] = useState<SplitResponse | null>(null)

  const handleConvert = async () => {
    // 两个必填都要有：`input_file` 与 `output_file` 在契约里都没有默认，写哪儿由调用方点名。
    if (!selectedFile || !convertOut) {
      message.warning('格式转换需要选中源文件并点名输出文件名')
      return
    }
    setLoading(true)
    try {
      // `convertSource` 是空串（「自动」）时服务层会把这一格整个丢掉，
      // 让「省略」与「显式声明」在 API 面上保持两种形状。
      setConvertResult(await convertDataset(selectedFile, convertOut, {
        targetFormat: convertTarget,
        sourceFormat: convertSource,
      }))
    } catch (err) {
      message.error(apiErrorDetail(err, '格式转换失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleMerge = async () => {
    if (!mergeInputs.length || !mergeOut) {
      message.warning('合并至少要勾一个输入文件并点名输出文件名')
      return
    }
    setLoading(true)
    try {
      setMergeResult(await mergeDatasets(mergeInputs, mergeOut, { deduplicate: mergeDedup }))
    } catch (err) {
      message.error(apiErrorDetail(err, '合并失败'))
    } finally {
      setLoading(false)
    }
  }

  const handleSplit = async () => {
    if (!selectedFile || !splitOutDir) {
      message.warning('分割需要选中源文件并点名输出目录')
      return
    }
    setLoading(true)
    try {
      // 比例用 `?? undefined` 而不是 `|| undefined`：`0` 是有意义的档位
      // （实测 `1.0/0/0` 回 `6/0/0` 且三段键都在），清空输入才是「交回服务端默认」。
      setSplitResult(await splitDataset(selectedFile, splitOutDir, {
        trainRatio: trainRatio ?? undefined,
        valRatio: valRatio ?? undefined,
        testRatio: testRatio ?? undefined,
        seed: splitSeed ?? undefined,
        stratify: splitStratify,
        stratifyKey,
        shuffle: splitShuffle,
      }))
    } catch (err) {
      message.error(apiErrorDetail(err, '分割失败'))
    } finally {
      setLoading(false)
    }
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

      <Card title="格式转换" size="small" style={{ marginTop: 16 }}>
        <Space wrap>
          <label>
            目标格式
            <select
              value={convertTarget}
              onChange={(e) => setConvertTarget(e.target.value)}
              aria-label="转换目标格式"
              style={{ marginLeft: 6, padding: '4px 8px', borderRadius: 4 }}
            >
              {CONVERT_TARGET_FORMATS.map((fmt) => (
                <option key={fmt} value={fmt}>{fmt}</option>
              ))}
            </select>
          </label>
          <label>
            源格式
            <select
              value={convertSource}
              onChange={(e) => setConvertSource(e.target.value)}
              aria-label="转换源格式"
              style={{ marginLeft: 6, padding: '4px 8px', borderRadius: 4 }}
            >
              <option value="">自动（按扩展名）</option>
              {UPLOAD_FORMAT_CHOICES.map((fmt) => (
                <option key={fmt} value={fmt}>{fmt}</option>
              ))}
            </select>
          </label>
          <Input
            placeholder="输出文件名，如 out.jsonl"
            aria-label="转换输出文件名"
            value={convertOut}
            onChange={(e) => setConvertOut(e.target.value)}
            style={{ width: 220 }}
          />
          <Button onClick={handleConvert} loading={loading}>转换选中的文件</Button>
        </Space>
        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          源格式那张表比目标格式多一个 <code>xls</code>：读边认它，写边刻意不认（老 BIFF 容器写出来是假的）。
        </div>
        {convertResult && (
          <div aria-label="转换回执" style={{ marginTop: 8 }}>
            按 <code>{convertResult.source_format}</code> 读入，落成{' '}
            <code>{convertResult.target_format}</code>：{convertResult.input_count} 条进 /{' '}
            {convertResult.output_count} 条出 ⇒ {convertResult.output_file}（源 {convertResult.input_file}）
          </div>
        )}
      </Card>

      <Card title="合并数据集" size="small" style={{ marginTop: 16 }}>
        <Space wrap direction="vertical" align="start">
          <Space wrap>
            {files.map((file) => (
              <Checkbox
                key={file.name}
                checked={mergeInputs.includes(file.name)}
                onChange={(e) => setMergeInputs(
                  e.target.checked
                    ? [...mergeInputs, file.name]
                    : mergeInputs.filter((name) => name !== file.name)
                )}
              >
                {file.name}
              </Checkbox>
            ))}
          </Space>
          <Space wrap>
            <Input
              placeholder="输出文件名，如 merged.json"
              aria-label="合并输出文件名"
              value={mergeOut}
              onChange={(e) => setMergeOut(e.target.value)}
              style={{ width: 220 }}
            />
            <Checkbox checked={mergeDedup} onChange={(e) => setMergeDedup(e.target.checked)}>
              去重
            </Checkbox>
            <Button onClick={handleMerge} loading={loading}>合并勾选的文件</Button>
          </Space>
        </Space>
        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          这一档只控「开 / 关」：SDK 里那个宽松 / 保守两档的去重阈值在这只端点上不可调（见账本 A290）。
        </div>
        {mergeResult && (
          <div aria-label="合并回执" style={{ marginTop: 8 }}>
            {mergeResult.input_files} 份输入共 {mergeResult.total_input} 条 ⇒{' '}
            {mergeResult.total_output} 条出，其中去重删掉 {mergeResult.removed_duplicates} 条 ⇒{' '}
            {mergeResult.output_file}
          </div>
        )}
      </Card>

      <Card title="训练 / 验证 / 测试分割" size="small" style={{ marginTop: 16 }}>
        <Space wrap direction="vertical" align="start">
          <Space wrap>
            <Input
              placeholder="输出目录，如 splits"
              aria-label="分割输出目录"
              value={splitOutDir}
              onChange={(e) => setSplitOutDir(e.target.value)}
              style={{ width: 180 }}
            />
            <label>
              train
              <InputNumber
                aria-label="train 比例"
                value={trainRatio}
                onChange={setTrainRatio}
                step={0.05}
                style={{ marginLeft: 6, width: 90 }}
              />
            </label>
            <label>
              val
              <InputNumber
                aria-label="val 比例"
                value={valRatio}
                onChange={setValRatio}
                step={0.05}
                style={{ marginLeft: 6, width: 90 }}
              />
            </label>
            <label>
              test
              <InputNumber
                aria-label="test 比例"
                value={testRatio}
                onChange={setTestRatio}
                step={0.05}
                style={{ marginLeft: 6, width: 90 }}
              />
            </label>
            <label>
              seed
              <InputNumber
                aria-label="分割随机种子"
                value={splitSeed}
                onChange={setSplitSeed}
                style={{ marginLeft: 6, width: 90 }}
              />
            </label>
          </Space>
          <Space wrap>
            <Checkbox checked={splitStratify} onChange={(e) => setSplitStratify(e.target.checked)}>
              分层
            </Checkbox>
            <Input
              aria-label="分层字段名"
              value={stratifyKey}
              onChange={(e) => setStratifyKey(e.target.value)}
              style={{ width: 160 }}
            />
            <Checkbox checked={splitShuffle} onChange={(e) => setSplitShuffle(e.target.checked)}>
              先打乱
            </Checkbox>
            <Button onClick={handleSplit} loading={loading}>分割选中的文件</Button>
          </Space>
        </Space>
        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          两条支路的判据不同：比例之和的容差默认支是 0.01、分层支是 1e-6，负数只有分层支会拦（A289）。
          分层字段给一个不存在的名字**不报错**——它把全部条目塌成一个分组，分段数看起来与用默认键时一样。
        </div>
        {splitResult && (
          <Table
            aria-label="分割回执"
            size="small"
            style={{ marginTop: 8 }}
            pagination={false}
            rowKey="name"
            dataSource={Object.entries(splitResult.splits).map(([name, part]) => ({
              name,
              count: part.count,
              file: part.file.split(/[\\/]/).pop(),
            }))}
            columns={[
              { title: '段名', dataIndex: 'name', key: 'name' },
              { title: '条数', dataIndex: 'count', key: 'count' },
              { title: '落盘文件', dataIndex: 'file', key: 'file' },
            ]}
          />
        )}
        {splitResult && (
          <div style={{ marginTop: 4, fontSize: 12 }}>
            目录：{splitResult.output_dir}（输入 {splitResult.input_file}）
          </div>
        )}
      </Card>
    </div>
  )
}
