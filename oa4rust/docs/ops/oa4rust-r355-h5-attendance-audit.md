# oa4rust H5 考勤页 attendance.vue 写读端点甄别审计（R355）

- 轮次：R355 / 任务 t165；角色 qa-e2e（缺陷审计与浏览器实测工程师）
- 对象：oa4rust/mobile/src/pages/attendance/attendance.vue（考勤打卡页）
- 性质：只读审计，不改源码 / 契约 / 后端 / SDK / 其它页
- 测试服：域名以占位符 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 代替，真值运行期从 .local.keys / ssh oa4rust-test 读取，未入库、未回显
- 取证环境：Playwright Chromium headless，390x844，isMobile + hasTouch，DPR 2
- 重要声明：本轮截图均为 headless 模拟移动视口，**非物理真机**，不得冒充真机证据

## 1. 任务与验收标准

甄别考勤页写读端点信封解包与字段契约，五项验收：①preCheck 信封与 UI 打卡前置态；②代表性写端点（打卡 check/checkFromOut 用安全分支取证，不污染真实考勤数据）；③selfHolidayApply 表单与提交守卫；④myStatistic/myDetailList/myVersion 三端点信封解包与 UI 渲染；⑤0 pageerror。三分判定，坐实缺陷仅给最小修复方向不改码。

## 2. 覆盖甄别

docs/ops 命中 attendance 相关 4 篇：R324 h5（oa4rust-r324-h5-attendance-audit.md，首次深度功能审计）、R324 pc、R301 skeleton、R305 visual。
- R324 判 0 缺陷，但打卡写因测试账号未加入考勤组属取证限制，且未测各端点信封形状 / 统计字段契约。
- 结论：preCheck / my* 端点解包形状与 statText 字段契约从未专项甄别，本轮为有效增量。

## 3. 源码梳理

### 3.1 请求层（oa4rust/mobile/src/services/http.ts）
parseEnvelope 只做字符串解析不解包顶层 data（前轮 R351/R354 已证）；list()（services/index.ts:24-26）仅类型断言不解包。

### 3.2 服务层（services/index.ts:257-293 attendanceApi）
- preCheck GET v2/mobile/check/pre；check POST v2/mobile/check；checkFromOut POST 'v2/mobile/check/ from/out'（sourceType=外部设备）
- myStatistic POST v2/my/statistic；myDetailList POST v2/my/detail/list（list 包装）；myVersion GET v2/my/version；selfHolidayApply POST selfholidaysimple

### 3.3 页面层（attendance.vue）
- load()（:32-44）：pre.value = resp.data ?? null（:37）
- doCheck（:52-66）/ doCheckFromOut（:73-87）：duplicated = Boolean(resp.data?.duplicated)（:57/:78）
- applyLeave（:96-112）：空日期 toast + return（:97-100）；调 selfHolidayApply（:103）
- loadMine（:127-154）：Promise.allSettled 三端点（:133-137）；myStat = statResp.value.data（:139）；myDetails = toMyDetailRows(detailResp.value.data)（:140）；apiVersion = String((verResp.value.data as {version?})?.version ?? '')（:142）；序号 seq 防竞态（:129/138/152）
- statText()（:156-167）：normal = pick('normalCount') || pick('normal') || '0'（:163）；abnormal = pick('abnormalCount') || pick('abnormal') || '0'（:164）；month = pick('month') || pick('yearMonth') || '本月'（:165）
- detailRows.ts:18-19 toMyDetailRows：只接受 Array.isArray，非数组返回 []（防御性正确）

### 3.4 后端契约（attendance_assemble_control/src/lib.rs）
- v2_my_statistic（:6661-6736）：ActionResult::success{person, month, detailByStatus:[{status,count}], recordCount}（:6722-6735）——**无** normalCount / abnormalCount 平铺字段
- v2_my_detail_list（:8840）返回数组；v2_mobile_pre_check（:8576）返回 {canCheckIn, date, group, records}
- routes.rs:305-312 注册全部端点

## 4. 真浏览器实测（两轮，证据 gitignored）

脚本与证据：test-results/r355-h5-attendance/ 下 evidence.json + evidence-v2.json，截图 att-initial / att-probe / att-guard / att-fail500 / att-field-contract.png（均 390x844 headless，非物理真机）。

### 4.1 首轮（R355_DONE，0 pageerror）
- 页面：未加入考勤组，四个打卡按钮 + 假期申请 + 今日暂无记录 + 月度考勤「2026-11 · 正常 0 · 异常 0」+ 明细 1 条 2026-11-20 normal
- 信封探针：preCheck 200 data={canCheckIn:false,date,group:null,records:[]}（单层对象）；myVersion 200 data={version:'2'}（单层）；myStatistic 200 data={detailByStatus:[{count:1,status:'normal'}],month,person,recordCount:0}（单层对象）；myDetailList 200 data=数组 len1（单层）；checkGET / checkFromOutGET 均 405（路由存在方法不符）
- UI：statLine='2026-11 · 正常 0 · 异常 0'，ver='v2'，明细 1 行
- 守卫：空日期点「提交假期申请」→ toast「请选择假期日期」，selfHolidayWrites=[]，无真实写（PASS）
- 500 兜底：preCheck 强制 HTTP 500 → 「考勤信息加载失败：HTTP 500 重试」（PASS，区分空态）

### 4.2 v2 字段契约决定性对照（R355V2_DONE，0 pageerror）
- Case A 注入后端真实形状 detailByStatus normal=5 / abnormal=3（无平铺字段）→ UI 显示「2026-11 · 正常 0 · 异常 0」（错误）
- Case B 注入平铺 normalCount=7 / abnormalCount=2 → UI 显示「2026-11 · 正常 7 · 异常 2」（正确）
- 截图 att-field-contract.png 肉眼坐实 Case B 显示「正常 7 · 异常 2」，与注入值一致

## 5. 三分判定

### 5.1 真缺陷：1 个 P2

**月度考勤统计恒显「正常 0 · 异常 0」（字段契约错配）**
- 源码：attendance.vue statText()（:163-164）读 s['normalCount']/s['normal'] 与 s['abnormalCount']/s['abnormal']
- 后端：v2_my_statistic（lib.rs:6722-6735）实际返回 detailByStatus:[{status,count}] + recordCount，无上述平铺字段
- 现象：真浏览器首轮后端真实数据 1 条 normal，UI 却显示「正常 0 · 异常 0」；v2 注入对照 Case A（真实形状）显示 0、Case B（平铺）显示注入值，铁证读错字段
- 影响：月度统计数值永远为 0，用户无法看到真实正常/异常出勤数；与同页明细 1 条 normal 自相矛盾
- 最小修复方向（不改码，仅建议）：statText 改为从 detailByStatus 数组按 status 聚合 normal/abnormal 的 count，平铺字段仅作 fallback；保持 month 取值不变。参考 R352 解包范式做兼容归一化。

### 5.2 取证限制
- 打卡写端点 check / checkFromOut：测试账号 canCheckIn=false（未加入考勤组），按钮禁用且未获真实打卡授权，未执行真实写；仅以 GET 405 只读确证路由存在与方法约束，未伪造 duplicated=true 分支写。
- selfHolidayApply：仅验空日期守卫，未真提交假期申请（避免污染真实数据）。

### 5.3 产品语义差异
- 无。preCheck / myVersion / myStatistic / myDetailList 信封均为单层 data，与 R340/R344/R351 分组信封不同型；toMyDetailRows 防御性只收数组符合仓内约定。

## 6. 结论与后续

本轮坐实 1 个 P2 真缺陷（statText 字段契约错配致月度统计恒 0），给出最小修复方向，不改源码。其余 preCheck / myVersion / myDetailList 单层解包正确，守卫与 500 兜底 PASS，0 pageerror。建议队长另派独立 implementation 任务修 D1（statText 兼容 detailByStatus），修后可用本轮 v2 脚本回归。

## 7. 敏感自检

文档仅含 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位符，无真域名 / IP / 口令 / 私钥 / 个人路径；真值运行期读取未入库。截图来源均如实标注为 headless 非物理真机。项目持续迭代中，非终态。
