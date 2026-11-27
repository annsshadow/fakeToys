# R600 文档筛选失效真修复（后端两变体过滤 + 前端 tab 联动）

- 标识: oa4rust-r600-document-filter-fix / 轮次 R600 / 任务 t163
- 类型: implementation（真修复，非甄别）
- 真父提交: f529326d8f629eb1143e99fbdc6b2b3be41e2987
- 前置: R599/t161 验证型甄别判 P2（审计文档 oa4rust-r599-document-filter-audit.md）
- 结论: 两变体现按 body 的 type 映射 status、keyword 映射 title 参数化过滤；无 body 兼容全量；前端页签切换触发重新查询。

## 一、根因（R599 已坐实，本轮不改判定）
- 后端 document_filter_paging_u3 与 document_filter_paging_manager_u3 均无 Json 提取器，请求体被 Axum 丢弃；硬编码 where 只有 deleted_at IS NULL + ORDER BY/LIMIT 200，无 type/keyword 语义。
- 前端 DocumentApp.vue 页签 @click 只改 tab，无 watch，切页签不重新查询；doSearch 已发 params.type/keyword，stats 恒取 items.length。
- 改用另一端点无效：两变体同一缺陷，故必须在后端两处都修。

## 二、修复（最小改动）
- lib.rs 新增纯函数 document_filter_where(body: Option<Json<Value>>) -> (String, Vec<String>)：keyword 转 title ILIKE $n（%kw%），type 转 status = $n；值全部参数化绑定，绝不拼进 SQL；无 body/空值退回原全量口径；保留 deleted_at IS NULL / ORDER BY create_time DESC / LIMIT 200。
- 两变体加 body: Option<Json<Value>> 参数并调用 helper；manager 的 u2_require_admin 先门禁原样保留，过滤在全量结果内做，不涉越权。
- tests.rs:813 直调 manager_u3 补 None 参数（Fail Loud 授权写域）。
- DocumentApp.vue：两枚页签 @click 追加 doSearch()；import 加 watch；doSearch 后加 watch(tab, () => { void doSearch() }) 兜底；stats 已取 items.length，零改。
- 零改其他端点/SQL 表/迁移/routes/Cargo 依赖。parity generated_tests.rs:13790/13812 与 tests_generated.rs:6721 生成冒烟测试零改：Option<Json> 兼容无 body 调用，契约已验证冒烟仍通过。

## 三、据实披露（Fail Loud）
- tests.rs 自身两处原本不过 rustfmt（:749 client.execute、:825 assert）；为使包级 fmt --check 全绿执行 cargo fmt -p cms_assemble_control，同一写域文件内引入两行格式副作用（另有一行任务改动 None）。未改其他文件。
- workspace 级 cargo fmt --check 仍报既有 bbs/src/tests.rs、bbs_assemble_control/src/lib.rs 等格式差异（真父既有，与本任务无关），不属本次写域，据实披露未改。
- 真父覆盖 RED 由 cp gitignored 备份实现，非手工反向编辑。
- manager 管理员全量口径保留：过滤只是在全量内按条件筛选，不构成越权或数据泄露。
- 上下文压缩导致一次恢复编辑失败，已据实重做并复核 GREEN；最终工作树 sha 与下方证据逐一吻合。

## 四、验证
- 真父覆盖定点 RED：r600-red.log EXIT101，E0061 两变体仍 1/2 参数（真父无 body 提取器），坐实契约在真父失败。
- 恢复后 GREEN：r600-green-final3.log EXIT0 2passed（两变体 × 四场景 + 源码守卫）。
- V1 cd oa4rust/backend && cargo test -p cms_assemble_control → EXIT0（348+1+2+0 passed，基础库先迁移）。
- V2 cd oa4rust && pnpm test:contracts → EXIT0（7 files/22 tests）；前端定点 vitest 3passed；pnpm typecheck EXIT0。
- V3 cd oa4rust/backend && cargo clippy -p cms_assemble_control -D warnings → EXIT0；cargo fmt --check -p cms_assemble_control → EXIT0 空。
- 临时 PG 连接串由环境变量注入，模板占位，无真值入库；scratch 库用完 drop，容器清理。访问形状统一占位 <TEST_HOST>:<PORT>。

## 五、法医证据（runnerSha 与文件 sha256sum 分列，勿混用）
- runnerSha = sha256(JSON.stringify(meta)) = a322d8ecc5b2954aeb479fc99e7236a9d4f2635cc6df2858f2bf47640b0ed9be
- 文件字节 sha256：
  - lib.rs = 8db2613450d995a4bad823e12214570f90e70d6aa0799648fa1cc76e3c599ae5
  - tests.rs = 62d2a15af109e8417139ff0b7e81eb77932c279ffd0a3b8b1c5aaf54fe0e28c1
  - DocumentApp.vue = 3b6a9970c9f9cd7c3dc0c45273f1b7f5d93b766152405e85f091d8936c85a8aa
  - 后端契约 r600_document_filter_contract.rs = 43939c1d07af90d1f3b5541824fb4c8d2cc76e155d5f8f77d20058941b7f33c5
  - 前端契约 DocumentApp.filterReload.contract.test.ts = 757ef2794a675b57d235cc0830b6964b4d05412e416b33a9acd54a6ae8a79a42
  - r600-red.log = 44df22b2ead8faaec142313444a5c2fe7ea6ad72fe46093c9864716dc08ec544
  - r600-green-final3.log = 829f38ade8031313d5862842e61cef0692afa652e48c46828f3a5d2b1592e783
  - r600-v1-final.log = c91cfc90dfceabffe72523f6a91710ec703c309aa1fe6c0d26de933956d94b7c
  - r600-testcontracts.log = d3cb911947248aa8b94938e1d98a691f3ee8c46f427212ca11336ce09d044428
  - r600-typecheck.log = 15a1cb81056b74929321923db42727ad98ce716deb7114421c20ddec7619c898
  - r600-fmt-final2.log = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855（空）
  - r600-clippy-final2.log = f542b42dbef9b971985f0aea56ac5796472263c0cf7771af9f78e9419f8b17f0
  - r600-fmt-workspace.log = cb3b52ad771dc1552b5821f8832c2d6bc5ce1927924e87297f29ad189065af0a（既有差异）
