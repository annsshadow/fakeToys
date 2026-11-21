# R327 PC 个人中心（Personal.vue）只读深度审计

- 范围：oa4rust/frontend/apps/desktop/src/views/Personal.vue（794 行），只读审计，未改源码/契约/配置。
- 环境：测试服 <YOUR_TEST_DOMAIN>，Playwright Chromium headless 1440×900，真浏览器实测；截图来源为自动化无头浏览器，非物理真机。
- 凭据：运行期经 test-results/extract-remote-creds.py 从测试服读取，未入库、未在本文回显。
- 基线：定点契约 Personal.business.contract.test.ts 5/5；全量 pnpm exec vitest run 267 files / 2152 tests 0 failed。

## 一、逐项走查

| # | 项 | 真浏览器结果 | 判定 |
|---|---|---|---|
| 1 | 改密表单 | h1=个人中心、副标题正确；两次密码不一致→页面「两次密码不一致」有效；错误旧密码→后端 HTTP 200 type:error / old password mismatch，页面却弹「✓密码修改成功」并清空输入 | D1[P1] 真缺陷 |
| 2 | 头像上传 | input[aria-label=更换头像][type=file] 1 个、按钮「📷 更换头像」存在；源码 :184 api.upload(/api/person/icon/{unique}) | L1 取证限制（未触发真实上传，避免破坏性写；仅验证控件与端点） |
| 3 | 签名上传/保存/列表 | 「签名图片」按钮 0 个（未暴露）；保存→后端 200 {data:{id,saved:true}}、toast「✓签名已保存」；刷新后 textarea 回到空，列表端点 {data:{signatures:[]}} | D2[P2] 真缺陷 |
| 4 | 账户安全区 | auth-note 0、modern-empty 1，恒显示「账户安全信息暂无更新」；loadAuthScopes/loadMailMeta 等只定义未调用、模板无触发按钮 | P1 产品语义差异（探针函数刻意不接 UI，契约 :55 禁止调试 handler 入模板） |
| 5 | 加载失败兜底 | 授权区注入 500 → .list-error-empty（⚠️授权记录加载失败：HTTP 500…重试）；正常空态「暂无授权记录」；解除路由后恢复、无 banner；pageErrors=[] | G1 正常回归（失败/空态可区分） |
| 6 | 既有契约无回归 | 定点 5/5、全量 267/2152 0 failed | G1 正常回归 |

## 二、真缺陷

### D1[P1] 改密业务错误被吞，失败仍提示成功

- 源码：Personal.vue:134-170 savePassword 用 useMutation，onSuccess(:161-164) 直接 toast.success(密码修改成功) 并清空表单；onError(:165-167) 才写「修改失败」。SDK api.ts:275-296 request 仅按 HTTP status 判错（resp.ok），parseJsonEnvelope(:82-88) 原样返回 body，不检查 type:error。
- 真证据：net-evidence.json pwd.req=PUT {oldPassword:WRONG_OLD_PASSWORD_R327,...}；pwd.res=HTTP 200 body {"type":"error","message":"old password mismatch",...}；toast-probe toastEarly=[✓密码修改成功auto-hide]、errorMsg=[]、oldPwdVal=空。截图 08-pwd-wrong-toast.png（sha256 8b124081720d1f04ff7fcdb4df449160b26fc5ed3f44758108aa2ee16b0edfdc）右上绿勾「密码修改成功」与后端 old password mismatch 同屏。
- 影响：用户旧密码错误时误以为改密成功，随后用新密码登录失败；错误分支无任何可见反馈。
- 修复方向（另案）：在 savePassword 对 mutation 返回值检查业务信封 type:error，走 pwdError/toast.error，不进入成功分支；或在 SDK 层统一识别业务错误信封（影响面大，需评估）。

### D2[P2] 签名保存写入与列表读取表错配 + 双层信封未解包

- 源码：保存 personal_extend/src/personal.rs:177-234 save_signature UPSERT x_person_signature（:204/:214/:229），返回 ActionResult::success({id,saved:true})；列表 personal/src/signature.rs:103-142 list 查 x_custom WHERE person=$1（:113-116），返回 {data:{signatures:[...]}}；上传 signature.rs:36-98 写 x_custom，字段为 value（base64）无 content。前端 Personal.vue:123-131 sigData 取 resp.data 后 sigs[0]?.content——把 {signatures:[...]} 当数组、取不存在的 content。
- 真证据：net-evidence.json sig.res=HTTP 200 {data:{id:c478327b-...,saved:true},type:success}；sig.list=HTTP 200 {data:{signatures:[]},type:success}；textareaAfterSave=marker、textareaAfterReload=空。截图 04-signature-save.png（sha256 0010684340ce5232ae8d0a2a5afee3f8e13eb79810a175eec0a518ad59375cf0，绿勾「签名已保存」+ textarea 有 marker）与 05-signature-reload.png（sha256 a49a6f1ad3a9457464196afd15341dbb18af2ff85197d074caa1bcbf23b98b49，与 01 初始态同哈希，刷新后回空）。
- 影响：签名保存看似成功但刷新即丢，个人签名功能实质不可用；上传图片签名亦无法回显。
- 修复方向（另案）：统一保存与列表的存储表/字段，前端适配 data.signatures 与 value 字段；两条写链需保留，避免破坏上传端点。

## 三、取证限制与产品语义差异

- L1：头像上传与真实改密成功分支属破坏性写，未获授权不执行；仅验证控件存在与源码端点，未判定缺陷。
- P1：账户安全区探针函数（OAuth/认证/邮件/身份等 20+ 端点）刻意不接 UI，模板保持静态空态；符合契约「不渲染 /api/* 调试按钮」，非缺陷。若产品期望展示真实绑定状态，应另立需求。

## 四、证据与复现

- 脚本（gitignored）：test-results/r327-personal-audit.cjs（主走查）、r327-net-probe.cjs（网络取证）、r327-toast-probe.cjs（toast 取证）。
- 证据（gitignored）：test-results/r327-pc-personal-audit/evidence.json、net-evidence.json，截图 01–08（均 1440×900 PNG，非物理真机）；关键 sha256 见上文与 evidence 目录。
- 复现：登录后进入 /app/personal，错误旧密码提交看 toast；填签名→保存→刷新看 textarea 回空；授权区切「授权给我」并在路由注入 500 看 .list-error-empty。
