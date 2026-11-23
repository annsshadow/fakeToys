# R400 PC 个人中心（Personal.vue）写链甄别：失败信封误报成功

- 范围：oa4rust/frontend/apps/desktop/src/views/Personal.vue（808 行），只读甄别，未改源码/契约/配置。
- 环境：测试服 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>，Playwright Chromium headless 1440×900 真浏览器实测；截图来源为自动化无头浏览器，非物理真机。
- 凭据：运行期经 ssh 从测试服读取，未入库、未在本文回显。
- 基线：git HEAD=fec2efc2e，工作树干净；证据目录 test-results/r400-pc-personal-write/（gitignored）。
- 判定方法：逐函数区分真实可达写 / 只读探测 / 死代码 → 真后端假 ID 探针坐实信封形状 → 决定性对照（真 UI 点击 + 写响应不拦截走真后端，排除人造注入伪造）→ 非 2xx → 失败/空态可区分 → 0 pageerror → 三分判定。

## 一、结论：0 个用户可达真缺陷

四个真实可达写函数在后端失败信封下均不产生用户可达的「200+type:error 仍误报成功」：后端失败要么走非 2xx（SDK 抛错→catch→toast.error），要么走 200 success（误报前提不成立）。死代码群无 UI 入口不可达。对齐 R385/R388/R389 先例据实判 0。

## 二、逐函数甄别（真实可达写 4 个）

| 写函数 | 模板入口 | 后端端点/信封（实测） | 前端处理 | 判定 |
|---|---|---|---|---|
| savePassword | 模板:35 @submit | PUT /api/person/password，错密码探针 = HTTP 200 + type:error(old password mismatch) | R328 D1 已在 onSuccess(:138-147) 与 mutate onSuccess(:169-178) 两处判 resp?.type==='error' → toast.error，不清表单 | 已修，非缺陷 |
| toggleEmpower | 模板:94 启用 / :95 禁用 @click | GET /api/person/empower/{id}/enable 与 disable，假 ID 探针 = HTTP 404 + type:error(not found) | await 后 :538 无条件 toast.success，但 404 非 2xx→SDK 抛错→catch(:540)→toast.error | 非缺陷（后端非2xx） |
| saveSignature | 模板:57 @submit | POST /api/person/signature/save，真探针 = HTTP 200 + type:success + saved:true（无 200+error 分支） | .then(()=>toast.success('签名已保存')) 不读 type；真响应 success 故提示一致 | 非缺陷（误报前提不成立） |
| handleAvatarUpload | 模板:21 @change | api.upload('/api/person/icon/{unique}') 默认 POST → 后端该路径仅注册 GET → HTTP 405 | onSuccess(:199-201) 无条件 toast.success，但 405 非 2xx→SDK 抛错→onError→toast.error('头像上传失败') | 非缺陷（后端非2xx） |

### 死代码群（不可达，不判缺陷）

uploadSignatureImage(:208-225)、empManage(:545-583)、personMore(:585-641)、personExtra(:643-658)、acctSec(:667-736)、loadPersonalTwin2(:743+)、loadAuthScopes/loadMailMeta 等 20+ 探针函数——模板无 @click/@submit、无 defineExpose、无 window 调用（CLICKS grep 仅模板 :21/:35/:57/:81/:82/:94/:95）。队长静态侦察称 empManage 有真实 @click 为误判，已据模板纠正。empManage 内 managerList/logMine/logTo/logManager 分页 POST 为只读探测亦剔除。

## 三、真后端假 ID 探针（写响应不注入，真服务端）

- pwdFake：HTTP 200 type:error old password mismatch → 前端判对（R328 已修）。
- empEnableFake / empDisableFake：HTTP 404 type:error not found → 非 2xx。
- sigSave：HTTP 200 type:success data.saved:true → 成功响应，无误报前提。
- iconPostNoFile / iconPostBadMime：POST /api/person/icon/{unique} = HTTP 405（该路径后端只注册 GET）。

## 四、决定性对照（真 UI 点击 + 写响应走真后端不拦截）

- 签名：真点「保存签名」→ 真 POST signature/save = 200 success saved:true → toast「✓签名已保存」（一致，无误报）；截图 personal-decisive-sig.png。
- 授权：注入 1 行假授权行走真后端，真点「启用」→ 真 GET empower/{假ID}/enable = 404 → toast「✗操作失败: 请求的资源不存在或已被删除」（正确错误）；截图 personal-decisive-emp.png。
- 头像：真 UI setInputFiles → 精确捕获 POST /api/person/icon/admin = 405 → toast「✗头像上传失败」（走 catch，正确错误）；截图 personal-decisive-avatar.png / personal-avatar-network.png。
- 人造 200+type:error stub：签名仍弹「✓签名已保存」——仅证明前端不读 type，真后端不产生此形状，不判缺陷；截图 personal-stub-sig.png。

## 五、通过项

- 非 2xx：500 / 403 注入 → toast「✗保存失败」（正确）；头像 405 → toast.error（正确）。
- 失败/空态可区分：授权区注入 500 → 「⚠️授权记录加载失败：HTTP 500: Internal Server Error」+ 重试按钮 1；空 data → 「暂无授权记录」。
- 0 pageerror；无 EndpointFold 门控（源码 grep=0 / DOM fold=0），页面真实渲染。
- DOM 审计：头像 input 1、密码表单 1、签名表单 1、保存签名按钮 1、授权页签 2，均真实可达。

## 六、取证限制 / 产品语义差异 / L1 潜在隐患（均非本轮缺陷）

- 取证限制：破坏性真写（真改密成功、真更换头像成功、真授权状态变更）未获授权不执行，用错密码 / 假 ID / 405 / 可逆签名标记替代。
- 产品语义差异：password handler 用 o2server 兼容式 200+ActionResult::error，empower crate 用非 2xx(NotFound 404)，属既有实现差异。
- L1 潜在隐患一：saveSignature 用 .then 不读 type，若签名后端未来改 200+error 口径会误报（注入 stub 已证实）；仅记录不改码。
- L1 潜在隐患二（功能可用性，非误报缺陷）：头像上传前端 POST /api/person/icon/{unique} 与后端该路径仅注册 GET 不匹配，头像实际上传不成功（405），属独立功能问题，建议另案修复。

## 七、证据与复现

- 脚本（gitignored）：test-results/r400-pc-personal-write.cjs（主走查）、r400-pc-personal-avatar.cjs（头像真 UI）、r400-pc-personal-avatar-net.cjs（精确网络捕获）、r400-pc-personal-write/run.sh。
- 证据（gitignored）：test-results/r400-pc-personal-write/evidence.json、avatar-evidence.json、avatar-network.json 及 1440×900 PNG 截图 10 余张（非物理真机）。
- 复现：登录后进入 /app/personal，错旧密码提交看 toast「密码修改失败」；填签名点保存看响应 200 success + toast「签名已保存」；注入授权列表假行真点启用看 404 + toast.error；设头像文件看 POST 405 + toast.error。
