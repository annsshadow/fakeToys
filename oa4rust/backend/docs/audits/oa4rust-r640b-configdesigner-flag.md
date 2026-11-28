# R640b ConfigDesignerApp flag 字段值域错配甄别审计（t332）

## 结论
判定 1 项真实可达前端契约错配缺陷，严重度 P3：ConfigDesignerApp 的 ConfigItem 与后端 /api/config/system 返回行均不含 flag 字段，但模板与导出代码在 :90/:337/:340/:347/:350 读取 flag。列表映射后 flag 恒 undefined：配置路径码退化为原始 UUID、下载文件名退化为 config.json、导出 JSON 的 flag 字段恒 undefined。每处消费点均有 name/id/config 回退，主显示与搜索不阻断，故判 P3。

## 立项性质与范围
- 验证型甄别：零源码改动，唯一交付物为本审计文档；探针与日志置于本地证据目录，临时探针用后即删。
- 基线 HEAD=f496de2e2b75fb220768d99041ed6d94902d9178（工作开始时所处 HEAD；工作期间并发推进属正常）。
- 台账去重：docs/ops/current-state.md:2621 记 R217 搜索框 a11y、:2920-2922 记 R516/R517 copyText/a11y 修复；R334/R347/R420/R439/R441/R446 等均无 flag 字段值域专项甄别，本缺陷独立成立。

## 机制锚点（逐行核实）
- 前端 oa4rust/frontend/apps/desktop/src/views/ConfigDesignerApp.vue:176-184 interface ConfigItem 声明 flag?: string（:179），但 :225-240 queryFn 的行映射 :230-238 仅产出 {id,name,category,config,updateTime}，不含 flag。
- :242-248 watch(data, rows => { if (rows) items.value = rows }, { immediate: true }) 异步回填正确，非 R639b 类同步取值遗漏——问题在映射字段缺失，不在回填时机。
- 消费点：:90 配置路径码取 selected.flag || selected.id；:337 下载名 selected.value?.flag || config；:350 下载名 selected.value.flag || config；:340 批量导出 flag:i.flag；:347 单项导出 flag:selected.value.flag。
- 回退点：:62 item.name||item.flag||item.id、:85 selected.name||selected.flag||未命名、:205 statusText 有 name 回退，故列表名与搜索不阻断。
- 后端 oa4rust/backend/crates/file_assemble_control/src/lib.rs:2729-2783 config_system_config：:2738 SQL SELECT id::text,name,value,category,description,update_time::text ...；:2743-2779 响应映射键集仅 {id,name,value,category,description,updateTime}，无 flag。
- 路由 main.ts:428 path config-designer/、:491 redirect 可达，普通登录即可进入，无需 admin。

## 方法（真测试）
- vitest + ts.transpileModule + new Function 真实执行：从源码切片 queryFn 的行映射回调（:230-238）并 new Function 执行，喂入后端真实形状行（id/name/value/category/description/updateTime，无 flag）；再用 new Function 逐条执行 :90/:337/:340/:347/:350/:62/:85 的原样表达式。
- 6 用例：①映射行 flag 恒 undefined 且不含键；②:90 退化为 uuid-1234-5678；③:337/:350 文件名恒 config.json；④:340/:347 导出 flag 恒 undefined；⑤对照行自带 flag 时 :90/:337 正确显示 sys.core.path（证明模板逻辑无误、是数据缺字段）；⑥:62/:85 name 回退正常。

## 决定性实测
| 观测 | 结果 |
| --- | --- |
| 映射对象是否含 flag 键 | 否 |
| mapped.flag | undefined |
| :90 配置路径码 | uuid-1234-5678（原始 UUID） |
| :337/:350 下载文件名 | config.json / config.json |
| :340/:347 导出 flag | undefined / undefined |
| 对照行自带 flag 时 :90 | sys.core.path（模板逻辑正确） |
| :62/:85 名称回退 | 系统配置项（不阻断） |
| 测试汇总 | 6 passed, 0 failed（EXIT 0） |

## 三分判级
- 可达性：路由普通登录可达（非死代码），映射与消费点逐行核实，6 用例真执行坐实——非误报。
- 为何不判 P1：无越权、无写库、无凭据泄露、无认证绕过；主列表显示与搜索经 name 回退仍正常，仅次要展示/导出面受损。
- 为何不判 P2：不阻断任何核心业务流程（配置仍可查看/编辑/复制；下载与导出仍产出有效内容，仅命名/字段降级），危害局限于标识可读性与导出元数据。
- 综合判定 P3：确定的契约错配 + 真实可达 + 台账未甄别，但仅次要面受损，符合 P3 口径。

## 行为现状 / 可复现
任意登录用户进入配置设计器，列表项均显示后端 name（正常），但配置路径码显示原始 UUID 而非语义 flag；点击下载得到 config.json；导出 JSON 中 flag 字段为 undefined（序列化时被丢弃）。复现见「方法」段 6 用例。

## Fail Loud
- 临时探针位于 views 目录，跑通后已删除（rm -f EXIT 0），不入提交；日志留在本地证据目录 oa4rust/test-results/r640b-probe.log。
- 未跑真实 GUI 浏览器与 tsc --noEmit（后者不解析 .vue，属盲区）；结论由源码逐行锚点 + 真实执行探针支撑，未采信未验证推测。
- runnerSha 为已删除临时探针的 sha256，日志 sha256 可复算；docSha 口径为除自身行外正文 sha256。

## 证据 sha256（据实分列）
- docSha（除本行外正文按 LF 连接、不含末尾换行的 sha256，可复算）= cfebea8764d80c2ab1a464a0bf7879159f2fc79b6ad15e78ddba093e52d402f8
- runnerSha（临时探针文件 sha256sum）= 83a56cbc12c6dc62994ea45a26ba45324844d87b9af36973d6060feeffce6312
- 日志 oa4rust/test-results/r640b-probe.log sha256sum = 1083f840dee08d12497abfaaa13c213ffb36737109077155af97ebd58af2ec5e
- 源码 ConfigDesignerApp.vue sha256 = bb5814c462a527c12376550da0992f0b7a203d18eb8b554b5a2c4a3e51e554b5
- 源码 file_assemble_control/src/lib.rs sha256 = 39824e39ef2b77ef3a0d3dc63a8f31975f45d5e7558342e6f333bff9cb4d3a44
- 源码 main.ts sha256 = e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0
- 占位：环境相关地址统一 <TEST_HOST>，仓库无真值。

## 修复方向（建议，不实施）
后端 config_system_config 响应映射补 flag 字段（取自 x_system_config 的 name 或新增 flag 列并按语义填值），前端 ConfigItem 映射 :230-238 同步补 flag: String(row.flag ?? '')；或统一改为消费 name 语义、移除 flag 读取以消除二义。两处需对齐同一值域。
