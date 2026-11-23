# R425 H5 文档上传 / 收藏删除 / 流程审批 写链误报甄别审计

## 一、审计范围与方法

- 轮次 R425 / 任务 t265；只读甄别、不改码，三分判定（0 缺陷 / P1 真缺陷 / L1 隐患）。
- 三候选写链：文档页上传附件（doc）、我的收藏删除（collect）、审批通过/驳回（process）；均为 H5 端真实 UI 可达入口。
- 真浏览器实测：390x844 headless Chromium，isMobile/hasTouch、deviceScaleFactor=2，**非物理真机**（历史 H5 口径一致，非 1440x900）。
- 判定三全：真 UI 可复现 + 真请求响应体 + 源码定位；决定性对照写请求全程走真后端、不拦截，以 page.on(response) 捕获的新写请求响应体为铁证，排除纯注入伪造。
- 破坏性真写不执行：假 ID 注入列表行 + route 注入替代；后端失败信封由真后端假数据 probe 实测。
- 环境与真值脱敏：测试服域名/主机统一写作 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>，API 只写相对路径；证据原始域名仅留 gitignored 的 test-results/，不入库、不回显。

## 二、源码甄别（三候选均真实可达，无生产门控）

- 门控：四页 grep EndpointFold / devToolsEnabled / import.meta.env.DEV / defineExpose 全空；DOM 实测按钮可见可点。
- doc/doc.vue：doUpload(:118-138) 在 :128 await fileApi.upload(folder, filePath, fileName)，:130 无条件 uni.showToast('上传成功', icon:'success')，不读响应 type；catch(:133) 仅非 2xx。触发链 pickFromAlbumOrCamera(:141-150)/pickFromFilePicker(:152-168) → chooseImage/chooseMessageFile success → doUpload。模板 :174 拍照/相册、:177 上传文件（uni-app 编译为 uni-button.tool，实测 2 个）。
- collect/collect.vue：removeRow(:44-66) showModal 确认(:49-57) → :58 await collectApi.remove(row.id) → :59 过滤行 → :60 uni.showToast('已删除', icon:'none') 不读 type；catch(:61) 红 toast。模板 :107 .item-del @tap.stop=removeRow。
- process/process.vue：act(:219-240) showModal 收集意见 → runAction(:242-252)：:244 completeTask / :245 rejectTask await 后 :246 无条件 uni.showToast('已通过'/'已驳回', icon:'success') 不读 type；catch(:248) 红 toast。模板 :280 驳回 / :283 通过（uni-app 编译为 uni-button）。
- SDK（关键）：mobile/src/services/http.ts :187 discardResponse 或 204 直接 resolve(success:true, data:undefined) 吞信封（collect/process 用）；performUpload(:223-277) :239-249 解析 2xx 的 JSON 直接 resolve，不查 type/success（doc 用）；parseEnvelope(:67-77) 仅 JSON.parse 不查 type。services/index.ts :631 collectApi.remove、:92-108 completeTask/rejectTask 均带 discardResponse；:230-234 fileApi.upload=mapi.upload(/api/attachment/upload/folder/{folderId}, {name:'file',...})。
- 剔除项（正确模式 / 只读）：start.vue uploadAttachment(:229-257) :242-243 const id=String(resp?.data?.id??'')、:243 if(!id) throw 属正确拦截；各页 load*/open* 只读展示 toast 不计缺陷。

## 三、后端失败信封（真后端假数据探针）

- 附件上传：file_assemble_control/src/lib.rs :1119-1192 attachment_upload_folder_folderId；:1125 require_session(401)、:1133 multipart 失败(400)、:1142 读文件失败(400) 均非 2xx；:1156-1158 file_data 为空才返回 Ok(Json(ActionResult::error('no file provided'))) 即 200+type:error，**仅无 file 字段时**；:1185 成功恒 success({id,name,uploaded:true})。路由 :179 POST /api/attachment/upload/folder/{folderId}。
- 收藏删除：program_center/src/lib.rs :393-422 collect_delete；:413-415 !deleted → ownership_error；ownership_error(:9406-9420) 返 AppError::Forbidden(403)/NotFound(404) 非 2xx。路由 :337 DELETE /api/program_center/collect/delete/{id}。
- 审批办理：processplatform_service_processing/src/lib.rs :3795-3870 task_complete（:3811 NotFound / :3823 BadRequest）、:3977-4050 task_reject（:3993 NotFound / :4005 BadRequest）均非 2xx。路由该 crate routes.rs :58 /complete、:59 /reject。注：processplatform_assemble_surface/src/lib.rs:3795 是 legacy_success 无关，非本链 handler。
- 实测探针（真后端，gitignored probe.json）：attachmentUploadNoFile=200 type:error no file provided；attachmentUploadFakeFolder(带 file+假日录)=200 type:success、真写入；collectDeleteFake=404 type:error not found；taskCompleteFake=404 error；taskRejectFake=404 error。

## 四、真浏览器决定性对照（v4，走真后端不拦截）

- COLLECT：注入 list 1 行假 ID（personId/person_id='admin' 以过 :41 过滤）→ 真点删除 → uni-modal 确定 → 真 DELETE /api/program_center/collect/delete/{fake} 未拦截 = HTTP 404（writesDelta=1，响应体铁证），红 toast「删除失败，请重试」，行仍在（rowsAfter=1）= 正确模式，无误报。
- PROCESS：注入 task list 1 行假 ID → 切「待办」页签 → 点通过 → uni-modal 确定 → 真 POST /api/task/{fake}/complete 未拦截 = HTTP 404（body {"opinion":""}，writesDelta=1），红 toast「请求的资源不存在或已被删除」= 正确模式。
- DOC：evaluate 触发 uni-button.tool → filechooser setFiles → 真 POST /api/attachment/upload/folder/admin 未拦截 = HTTP 200 success（writesDelta=1，uploaded:true），绿 toast「上传成功」= 成功语义与响应一致，无误报。真 UI 必带 file 字段，后端 200+error 仅在无 file 时产生，误报前提不成立。
- 注入对照（仅佐证 L1，不判缺陷）：route 注入 200+type:error → collect 绿「已删除」、process 绿「已通过」、doc 绿「上传成功」；证明三写链若遇 200+error 会误报，但真后端不产生此形状。
- 非 2xx / 取消 / 状态：collect 500·403 均红「删除失败，请重试」，取消零写 writesDelta=0，失败态「收藏加载失败」+重试 1 / 空态「暂无收藏」重试 0；process 500→红「HTTP 500」、403→红「没有权限执行该操作」，取消零写，失败态 fail+重试 1 / 空态 empty；doc 500→红「HTTP 500」、403→红「没有权限执行该操作」。全程 0 pageerror，totalWrites=12。
- 截图 sha256（命令行 sha256sum 原始值；390x844 headless 非物理真机）：r425-collect-real.png=73c709e065d8b5a631c4e1d1f07ef99c9cb84f9634d0ab3bea7e916436970ce6；r425-process-real.png=a52aa620c6585552d18052ce962d1d7a9e32769bf6bcda6ccfa12c65e243ccce；r425-doc-real.png=9acbd407161a71a5af49c6e6f2837fd57b17173d8517a642be5f94705996aa42。三张决定性对照已目视确认（真写响应与 toast 同屏）。

## 五、三分判定

- **0 个用户可达真缺陷（P1）**：三候选误报前提均不成立——collect/process 失败走非 2xx（SDK 抛错 → catch 红 toast，行为正确）；doc 真上传恒 success、200+error 仅无 file 字段时（真 UI 不会触发）。
- L1 隐患（不改码，登记）：三写链不读响应 type——collect/process 由 discardResponse 吞信封、doc performUpload 不查 type；若后端改为 200+type:error 口径会误报（注入对照已佐证）。
- 正确模式：collect/process 非 2xx → http.ts 抛错 → catch 红 toast 且保留列表行；doc 上传成功后 :132 load() 刷新列表；showModal 取消均零写；失败态与空态可区分。

## 六、取证限制与产品语义差异

- 取证限制：破坏性真删除/真审批未对真实数据执行，使用假 ID + route 注入替代；doc 失败态 errorState fail=false/retry=0——attachment/list 注入 500 时 fileList 仍成功、模板失败态文案未命中，据实记录不判缺陷；注入 200+error 变体仅作 L1 佐证。
- 产品语义差异：ActionResult error 的 200 信封为 o2server 兼容口径；本轮 collect/process 真实失败为 404/403 非 2xx，与 200+error 口径不同；附件上传假目录无外键门禁仍 200 success 真写入（history 同型）。

## 七、最小修复建议（本轮不实施）

- 若未来统一 200+error 口径：为 collect/process 去除 discardResponse 或读取解析后信封的 type/success，失败 toast.error 透传 message 并保留列表行；doc performUpload 后校验 parsed.type/success，失败不弹「上传成功」。
- 建议补单测覆盖 200+type:error 变体，固化「失败不弹成功 toast」契约。

## 附：证据索引（均 gitignored，不入库）

- test-results/r425-h5-doc-collect-process-write/：probe.json、dom-probe.json、evidence.json、截图集。
- 脚本：test-results/r425-probe.cjs、r425-dom.cjs、r425-h5-doc-collect-process-write.cjs 及对应 *-run.sh。
