# R411 PC 文件管理删除写链甄别报告（FileManager.vue）

> 审计轮次：R411 / 任务 t246；身份：qa-e2e（只读甄别，不改码）。
> 环境：1440x900 headless Chromium，本地测试服（非物理真机，未使用真机）；域名与主机均以占位符表示。
> 判定：0 个用户可达真缺陷（队长静态备料点经源码甄别为死代码，真实 UI 写链已有正确守卫）。

## 一、审计范围与口径
- 目标：PC 端文件管理页删除写链，静态备料指向 fileDelete 四端点裸 toast.success。
- 三全判据：真 UI 可复现 + 真请求响应 + 源码定位；三者齐备才记真缺陷。
- 决定性对照：写响应全程不拦截、走真后端，排除注入伪造。
- 服务地址脱敏：页面相对路径 `/app/file`，接口相对路径 `/oa4rust-api`，测试域 `<YOUR_TEST_DOMAIN>`、测试主机 `<YOUR_TEST_HOST>` 均为占位。

## 二、源码甄别（纠正队长静态备料）
- 文件：`frontend/apps/desktop/src/views/FileManager.vue`（1518 行）。无 EndpointFold / devToolsEnabled / import.meta.env.DEV 门控，生产构建真渲染。
- 模板真实 @click 删除按钮两处：第 64 行 `@click.stop=deleteFile(f)`（title 删除，文件列表行）、第 85 行 `@click=deleteFile(f)`（title 删除（进入回收站），可清理文件区）。两处都调 `deleteFile(f)`。
- 真实 `deleteFile(f: FileItem)`（第 911-928 行）：第 913 行 `confirmMsg` 确认守卫；第 915 行 `deleteMutation.mutate(f.id)`；`deleteMutation`（第 903-909 行）mutationFn 走 `api.delete`（路径 `/api/file/assemble/control/file/{id}`）。
- onSuccess（第 916-923 行）第 917 行 `if(!writeSucceeded(r)){toast.error(...);return}` —— 已用 R369 helper 守卫；仅成功后过滤 recyclableFiles、刷新回收站、toast.success（文件已删除）。onError（第 925 行）toast.error（删除失败）；onSettled（第 926 行）清 deletingId。
- writeSucceeded（第 185-189 行）：type 为 error 或 success 为 false 判失败；type 为 success 或（type 缺省且 success 不为 false）判成功 —— type 为主、不强求 bool，可识别 HTTP200+type:error。
- 队长备料的 `fileDelete(kind)`（第 351-365 行，prompt 假 ID + confirmMsg + 四端点 + 第 361 行裸 toast.success）经全量 grep：调用点仅第 351 行定义、模板第 64/85 行（实为 deleteFile）、第 614 行 `else if(op==='fileDelete')`（op 字符串名比较，属另一批量分发函数）。该函数无 UI 调用，为死代码，不判缺陷。
- createRecyclableFile（第 217-241 行）第 226 行已用 writeSucceeded 守卫 + created?.id 校验，属正确模式，剔除。

## 三、后端信封铁证
- 真实 UI 删除端点：`routes.rs` 第 286 行 `DELETE /api/file/assemble/control/file/{id}` → `u2_file_delete_by_id`（`lib.rs` 第 5016-5043 行）；假 ID 第 5030 行 ActionResult::error（file not found），HTTP200。
- 备料四端点（死代码路径）：routes.rs 第 122 行 POST、第 190 行 DELETE `/api/file/assemble/control/file/delete/{id}` → delete_file（lib.rs 第 507-548 行）；entity 端点 delete_file_entity（第 733-774 行）失败第 749/763 行 error（file not found or already deleted）。
- SDK：`frontend/packages/sdk/src/api.ts` 第 82-84 行 parseJsonEnvelope 仅 resp.json 不查 type；第 296/365/508 行仅非 2xx 抛错 —— 故 HTTP200+type:error 会正常 resolve，靠前端 writeSucceeded 兜底。

## 四、真浏览器实测（决定性对照）
- 环境：1440x900 headless Chromium，本地测试服（非物理真机）；pageErrors=[]，totalWrites=5。
- DOM 审计：标题『文件管理』命中；删除按钮 2 个；fold 控件 0。
- 假 ID 探针（真后端、不注入响应）：3 端点（control / POST-delete / DELETE-delete）均 HTTP200 + type:error（file not found）。
- 决定性对照：注入列表 1 行『R411测试文件』（id=R411-FAKE-FILE-<ts>）→ 真 UI 点删除图标 → confirm 确认 → 真 `DELETE /api/file/assemble/control/file/{fake}` 不拦截走真后端，返回 HTTP200+type:error（newWrites 捕获真响应体）；结果：writesDelta=1，界面弹红 toast（✗file not found），不弹成功，注入行仍在（rowStill=1）—— writeSucceeded 正确拦截。
- confirm 取消：writesDelta=0，零写。
- 非 2xx：500 / 403 均弹红 toast（✗删除失败），为 toast.error，正确。
- 失败态：『文件列表加载失败: HTTP 500: Internal Server Error』+ 重试按钮 1；空态：『当前文件夹为空』可区分。
- 截图（1440x900 非物理真机，sha256）：
  - r411-decisive-delete.png = 9912a0a0d87875e5341d2a22d5a696e008a8c1ae993a400d319c68e594ef504f（红 toast file not found + 注入行仍在）
  - r411-500.png = c0b73d7f9963e260fd93d9664b4eb89740d5a8c465ad15ff0ef6c1d6e66b7f1a（红 toast 删除失败）
  - r411-error-state.png = bc05bfa313db1752e96fa8878553505ab862820063a45719abe719158ccfde90（失败态 + 重试按钮）

## 五、三分判定
- 0 个用户可达真缺陷。真实 UI 删除写链 deleteFile 已有 writeSucceeded 守卫，决定性对照坐实 200+type:error 被拦截并弹 error、不弹成功、不删本地行。
- 通过项：守卫生效 / 决定性对照走真后端 / 假 ID 探针 200error / 500·403 正确 error / confirm 取消零写 / 失败态与空态可区分 / 无门控真实可达 / 0 pageerror。
- 不适用：prompt 取消零写（本写链仅 confirm 无 prompt，据实说明）。

## 六、取证限制
- 破坏性真删未执行，用假 ID + 列表注入替代（假 ID UPDATE/DELETE 0 行，无副作用）。
- 备料 fileDelete(kind) 四端点为死代码，未 UI 触发；全量按钮按代表性覆盖。
- 测试服路由：文件页为 `/app/file`；旧 `/app/admin/file` 会踢回登录，已在 E2E 用 SPA 侧栏点击核实。

## 七、结论与产品语义差异
- 结论：本写链无需修复。队长备料指向的裸 toast.success 属死代码，真实可达路径已在早前轮次（R369 helper）修复并生效。
- 产品语义差异：ActionResult::error 以 HTTP200 返回是 o2server 兼容既有设计；真实删除走 u2_file_delete_by_id，非备料所列四端点。
- 建议：如后续为 fileDelete(kind) 增加真实 UI 入口，须同步接入 writeSucceeded 守卫，避免重演裸成功提示。