# R596b 验证型甄别：申诉状态值域错配（P2 确凿）

- 任务：t152/R596b；真父 HEAD：`5b6bb7fbd72058f54cab7ffa80d31ecc74ee2bd1`
- 裁决：**确凿，判 P2**。后端提交即写 `appeal_status='appealed'`，前端申诉徽标只认三值域，通过/驳回按钮只认 `pending`；终态徽标与按钮恒不可达。
- 零源码改：唯一产出为本脱敏文档；临时真测夹具与 scratch 库用完即删，工作树源码零改动。

## 一、真测证据（后端真处理器直调，非静态推断）

- 环境：临时 PG 容器（回环 + 临时端口，占位 `<TEST_HOST>:<PORT>`），`DATABASE_URL` 从环境变量注入；scratch 库迁移后清空申诉表并种 admin 角色；测试文件置于 crate/tests 临时路径，运行后删除。
- 夹具：`oa4rust/test-results/r596/r596_audit_probe.rs`（gitignored），文件 sha256 `d7537665a17bbe187949d713fdc0d33ff7746aff57958f1fd3f5d51591169a47`。
- 日志：`oa4rust/test-results/r596/probe.log`，文件 sha256 `c90b701a6457a4428abed052bb80a7ec79342c18358663791850f471c432689a`。
- 命令：`DATABASE_URL=<占位> cargo test -p attendance --test r596_audit_probe -- --nocapture --test-threads=1`，EXIT 0，1 passed / 0 failed。
- 提交真处理器 `attendance::submit_appeal` 响应：`data.status="appealed"`，`type="success"`，落库 ID 唯一。
- 列表真处理器 `attendance::list_appeal_records` 响应：`data[0].status="appealed"`（SELECT `appeal_status` 原值直出，lib.rs:415/437-440）。
- 审计真处理器 `attendance::audit_appeal` 响应 success；DB 双列直查：`appeal_status=appealed`、`audit_status=approved`（lib.rs:539 只写 `audit_status`）。
- 审计后列表复核：`status` 仍 `appealed`，前端徽标读取值未变。
- 对照真写：DB 直写 `appeal_status='approved'` 后列表即 `status=approved`，证明列表本身可读，缺陷在写入端与前端值域的错配，而非读取链路。
- 结论链：提交→`appealed`→审计不回写→列表恒 `appealed`→前端三值域外→破折号/空 class/按钮不可达，全链真测坐实。

## 二、真测证据（前端纯函数，源码抽取真实现）

- 夹具：`oa4rust/test-results/r596/r596_frontend_probe.cjs`（gitignored），文件 sha256 `ec824c905ce16cb62b383964dee920527b44c0d568f2636f0367862e4c94ec9d`。
- 日志：`oa4rust/test-results/r596/frontend.log`，文件 sha256 `daa70e0fb25093146dba982422f1d144fdead198b299c50a62f95eb60e9eced6`。
- 命令：`node oa4rust/test-results/r596/r596_frontend_probe.cjs`，EXIT 0，11 cases / 0 fatals。
- 从 `AttendanceApp.vue` 源码抽取 `appealClass`/`appealStatus` 函数体真执行：`appealStatus('appealed')='—'`、`appealClass('appealed')=''`；`approved/rejected/pending` 三值映射均正确（守卫对照通过）。
- 审批按钮守卫：模板 `v-if="a.status==='pending'"`（:189）为真，`appealed` 不满足 → 通过/驳回按钮初始态即不可达。
- 附加去伪：`updateAppealStatus(a,'processing')` 写入的 `processing` 同样落在三值域外（徽标 `—`、class 空），属同源值域缺口延伸。

## 三、关键去伪：V2 审核替代路径

- `auditAppealV2`（AttendanceApp.vue:423-433）调 `POST /api/attendance/assemble/control/attendanceappealInfo/audit`，后端 `attendanceappealInfo_audit` SQL 为 `UPDATE ... SET audit_status=$1`（assemble_control/src/lib.rs:736），**不写 `appeal_status`**；源码守卫真测通过。
- 故 V2 审核后徽标依旧读 `appeal_status='appealed'`，仍为破折号——**更严重：审核成功但 UI 永远不反映**，据实披露。
- 另存在 `POST .../attendanceappealInfo/appeal/{id}`（同 crate lib.rs:511 `SET appeal_status=$1`）可写 `appeal_status`，但前端按钮写死字面量 `'processing'`（仍在三值域外），因此**现有可达 UI 路径无法让徽标显示已通过/已驳回**。DB 直写 `approved` 对照证明数据链路本身无故障。
- 无触发器/前端兜底同步：迁移目录与前端全局搜索 `appeal_status`、`CREATE TRIGGER ... appeal` 均无命中；`audit_status` 系 migration 088 新增独立列。

## 四、影响与建议（不在本任务写域，仅记录）

- 影响：申诉提交后状态徽标恒为 `—`，通过/驳回按钮永不可见；审核成功（`audit_status`）对用户不可见，属 P2 功能可见性缺陷。
- 修复方向（供后续实施）：统一提交/审核写同一状态列并在列表映射该列；或前端 `appealStatus`/`appealClass` 增补 `appealed`/`processing` 分支并将审核按钮 `v-if` 对齐真实状态值。
- 三分裁决：未发现 `appeal_status` 写 `pending`、无同步触发器、无前端兜底 → 不判 0，维持 P2 确凿。

## 五、证据 sha 表

- runnerSha（紧凑 JSON，`sha256(JSON.stringify(meta))`）：`cfa422d23fde0d6d09e827fb3468181d81d65b8acb56a4837229a0447dab8e05`
- 文件 sha256 见上文逐项（夹具/日志，均 gitignored，非入库交付物）。
- 脱敏：数据库地址仅占位 `<TEST_HOST>:<PORT>`；`DATABASE_URL` 仅环境变量注入，0 真值入库。
- SKIP/未覆盖：未跑浏览器 GUI 真页面（后端真处理器 + 前端纯函数双层真测已覆盖核心判定）；未跨 crate oneshot 调用 V2 audit（attendance crate 无 tower 依赖，零源码改约束下以源码守卫替代，据实标注）。
