# R359 H5 会议写操作链甄别（accept/reject/confirm/checkin 信封与 UI 反馈）

> 审计轮次：R359 / 任务 t170。范围：oa4rust/mobile/src/pages/meeting/meeting.vue 五写操作链（accept/reject/confirmAllow/confirmDeny/checkin）。只读甄别，未改任何源码/契约/后端。
> 实测环境：Playwright Chromium headless，390x844、isMobile:true / hasTouch:true / deviceScaleFactor:2，非物理真机。测试服域名仅以占位符 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 表示，证据中的地址来自运行期环境变量，不入库。

## 1. 覆盖甄别

- docs/ops 全库对 meeting 写操作链命中仅 2 处：oa4rust-r309-meeting-create-fix.md:21（后端 create datetime-local 修复）、oa4rust-r326-meeting-search-fix.md:15（status tone 映射），均非 accept/reject/confirm/checkin 信封与 UI 反馈的专项甄别。
- R317 仅记 H5 无新建会议入口；R300/R358 等视觉/双来源轮次未覆盖写操作链。本轮为首次真浏览器专项甄别。

## 2. 源码机理（缺陷链）

- meeting.vue:102-120 的 act(id, fn, okText)：首行 if(busyId.value)return 并发守卫 → busyId.value=id → await fn() → 无条件 uni.showToast({title:okText, icon:'success'}) → await load()；仅 catch 才显示错误 toast。五个包装：accept→'已接受'、reject→'已拒绝'、allow→'已确认出席'、deny→'已确认缺席'、checkin→'签到成功'（:113-117）。
- services/index.ts:403-420 五端点全用 mapi.post(...,{discardResponse:true})。http.ts:187-189 中 if(opt.discardResponse) return emptyResponse()（data:undefined），彻底丢弃响应体；http.test.ts:231-234 单测明确该行为。前端拿不到 bool/type，act() 无法区分业务成功与静默失败。
- 后端 meeting_assemble_control/src/lib.rs：
  - meeting_id_accept:2660、meeting_id_confirm_allow:2790、meeting_id_confirm_deny:2815、meeting_id_reject:2996：UPDATE ... WHERE id=$1 后不检查 rows affected，一律 ActionResult::success + bool = result>0（accepted/confirmed/denied/rejected）。UPDATE 0 行 → HTTP 200 + type:success + bool=false。
  - u2_meeting_owner_gate:2487：行不存在时 None => Ok(())，放行给后续 SQL 的 0 行分支（假 ID 走成功信封 bool=false，而非 404/非 2xx）。
  - meeting_id_checkin:2721：以会话身份 INSERT 新签到行，返回 type:success + checkedIn:bool(true)；本轮未真发签到。
- 缺陷链：0 行 UPDATE 的假 ID → 后端 200 + success + bool:false → 前端 discardResponse:true 丢掉 bool → act() 无条件 toast 成功 → 用户看到成功但业务未发生（R327 改密吞错误 / R349 H5 同型）。

## 3. 真浏览器实测（390x844 headless，非物理真机）

### 3.1 真实后端写端点信封探针（假 ID，0 行 UPDATE，无真实状态翻转）

页面内 fetch 直发四写端点，ID 用 R359-NOSUCH-<ts> 前缀确保不存在（签到 INSERT 未执行）：

| 端点 | HTTP | type | data 关键字段 | bool |
|---|---|---|---|---|
| /meeting/{假ID}/accept | 200 | success | {accepted:false,id} | false |
| /meeting/{假ID}/reject | 200 | success | {rejected:false,id} | false |
| /meeting/{假ID}/confirm/allow | 200 | success | {confirmed:false,id} | false |
| /meeting/{假ID}/confirm/deny | 200 | success | {denied:false,id} | false |

四写端点 200+success+bool:false 全部坐实。

### 3.2 注入对照：UI toast 与信封一致性（route 拦截，零副作用）

注入会议卡片「R359注入会议」带 接受/拒绝/确认出席/确认缺席/签到 按钮，逐操作抓 DOM toast：

| mode | 按钮 | POST 状态 | data bool | UI toast | 期望 |
|---|---|---|---|---|---|
| falseSuccess | 接受 | 200 | accepted:false | 已接受(误报) | 应失败提示 |
| falseSuccess | 拒绝 | 200 | rejected:false | 已拒绝(误报) | 应失败提示 |
| falseSuccess | 确认出席 | 200 | confirmed:false | 已确认出席(误报) | 应失败提示 |
| falseSuccess | 确认缺席 | 200 | denied:false | 已确认缺席(误报) | 应失败提示 |
| falseSuccess | 签到 | 200 | checkedIn:false | 签到成功(误报) | 应失败提示 |
| type:error(200) | 接受 | 200 | 无 | 已接受(误报) | 应失败提示 |
| http403 | 接受 | 403 | 无 | 没有权限执行该操作 | 正确错误 ✓ |
| http500 | 接受 | 500 | 无 | HTTP 500 | 正确错误 ✓ |

- 5 个 falseSuccess（后端明确 bool:false）UI 全弹成功 toast；type:error 的 200 响应也弹成功——误报确凿。
- 仅非 2xx（403 权限门 / 500）正确走 catch 显示错误 toast（owner_gate 权限失败确实非 2xx，该分支无误）。
- busyId 并发守卫：act() 首行 if(busyId.value)return，在途期间后续点击直接返回，源码确证；注入批次每操作间留 1.3s 间隔避免守卫干扰。
- 失败/空态可区分：空列表 → 暂无待接受的会议邀请；500 注入的列表加载 → 会议加载失败 HTTP 500 重试（loadList label），文案不同可区分。
- 三轮 E2E（首轮真实探针 + v2 UI 点击 + v3 DOM toast）全程 0 pageerror。

### 3.3 截图证据

- test-results/r359-h5-meeting-write/meeting-initial.png：真实初始态（空，暂无待接受的会议邀请）。
- meeting-v2-injected.png：注入会议卡片「R359注入会议」带接受/拒绝按钮，末次 500 操作后 toast 显示「HTTP 500」；截图展示缩略 1378x3043（源 1960x4328），390x844 视口 fullPage 拼接。
- meeting-injected.png / meeting-empty.png：注入中间态与空态对照。

## 4. 三分判定

### 真缺陷（1 个，P1）

五写操作链在后端 200 + type:success + bool:false（以及 200 + type:error）下无条件 toast 成功，业务实际未发生。触发条件真实可达——假 ID（不存在的会议）经 u2_meeting_owner_gate 的 None=>Ok(()) 放行后，UPDATE 0 行仍返回 success/accepted:false；前端 discardResponse:true 丢掉响应体，act() 无从判断。影响 accept/reject/confirmAllow/confirmDeny 四写操作（checkin 同型但本轮未真发）。与 R327 改密吞错误、R349 H5 同型。

最小修复方向（不改源码，建议后续独立任务）：① 五端点去掉 discardResponse:true，使 act() 能读取信封；② act() 内校验 type==='success' 且对应 bool 字段（accepted/rejected/confirmed/denied/checkedIn）为 true 才弹 okText，否则弹业务失败 toast（如 操作未生效，请刷新后重试）。后端可保持现状（成功信封是既有契约），前端判定为准。

### 取证限制

- 真实会议状态翻转（accept/reject/confirm）与签到 INSERT 属破坏性写，未授权不执行；本轮用假 ID 0 行 UPDATE 真实探针 + route 注入对照验证信封与 UI 映射，未在真实会议数据上做闭环。
- 注入对照修改 route 响应、非真实后端状态变化；真实后端仅做假 ID 探针（0 行 UPDATE，无副作用）。

### 产品语义差异

- u2_meeting_owner_gate 对不存在行放行（保留既有 not-found 语义由 0 行分支走成功信封 bool:false）属后端既有设计；缺陷在前端 UI 反馈层，非权限门设计。
- checkin 每次 INSERT 新签到行（非幂等覆盖）属设计，本轮未真发。

## 5. 证据与命令

- 证据目录（gitignored）：oa4rust/test-results/r359-h5-meeting-write/（evidence.json 真实探针 + evidence-v2.json UI 点击 + evidence-v3.json DOM toast；截图 4 张；脚本 r359-h5-meeting-write.cjs / -v2.cjs / -v3.cjs 及 run.sh）。
- 实测命令：bash test-results/r359-h5-meeting-write-run.sh、-v2-run.sh、-v3-run.sh（workdir oa4rust/），均输出 R359_*_DONE，0 pageerror。
- 敏感扫描：全文仅 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位符，0 真值（域名/IP/口令/私钥均未写入）。

## 6. 结论

R359 甄别出 1 个 P1 真缺陷：H5 会议五写操作链对 200 成功信封下的静默失败（bool:false / type:error）误报成功，假 ID 可真实触发。给出最小前端修复方向，不改码。其余项（权限门非 2xx 错误 toast、busyId 守卫、空态/失败文案可区分、0 pageerror）通过。