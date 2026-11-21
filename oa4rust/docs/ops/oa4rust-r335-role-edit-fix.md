# R335 PC 角色管理编辑角色 P1 修复

## 1. 范围与环境

- 轮次：R335，任务 t137；性质：真缺陷修复（P1，编辑角色 100% 失效）。
- 对象：PC 端 `RoleManager.vue`（路由 /app/admin/role），修复 R334/t136 审计坐实的 P1 缺陷。
- 环境：测试服 <YOUR_TEST_HOST>，本地 Playwright Chromium headless，视口 1440x900。**截图与 evidence 均为本地无头浏览器实测，非物理真机。**
- 凭据：运行期经 gitignored 脚本与 ssh 通道读取，未入库、未回显；域名/主机统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位。
- 写域：仅 `RoleManager.vue`、`RoleManager.edit.contract.test.ts`、本文；不动后端、SDK、Settings.vue、DocumentApp.vue、AppInfoApp.vue、HotpicApp.vue。

## 2. 根因与方案（fail-loud）

- 四重根因链（R334 审计已源码+DOM+HTTP 确证）：①后端 auth/src/lib.rs:925 role_list 只 SELECT id,name,description、:935-939 map 不含 flag；②前端 RoleManager.vue:148 editRole 因 r.flag 缺失置空 form.flag；③:45 flag 输入框 :disabled=!!editingRole 编辑态禁用无法补填；④:153 onSave 校验 flag 为空 return，:157 api.put 不可达，PUT /api/role/{flag} 从未发出。
- **方案（队长裁决，最小纯前端单文件改动）**：仅改 :148 editRole 的 flag 兜底为 `flag: r.flag || r.id || ''`，使编辑态 form.flag=角色 id(uuid)；onSave(:153) 校验因 flag 非空自然通过，PUT /api/role/{uuid} 发出；后端 role_edit 用 resolve_generic_id(u2_org.rs:937) 支持 id 解析，传 uuid 作 path 可正常工作。
- **fail-loud**：契约测试断言 editRole 含 r.id 兜底 + 编辑态 onSave 真发 PUT；回退到 `r.flag || ''` 即 RED，防止缺陷复发。
- **硬边界（fail-loud 一并声明）**：①P2 候选（新建 flag 输入不生效、后端 uuid 生成）属产品语义差异，本轮不动、不隐藏输入框、不改 role_create；②篡改操作 leaf 保持原样；③后端 auth/src/lib.rs 不改；④Settings.vue /api/config 取证限制不动；⑤不改 SDK/其他页/后端/DDL。实测未与上述边界冲突，未静默改向。

## 3. 修复内容（RoleManager.vue，仅此文件源码改动）

- :148 `editRole` 改为 `form.value = { name: r.name || '', flag: r.flag || r.id || '', desc: r.desc || '' }`（git diff 确认仅此一行）。
- **未动**：onSave(:153) 校验与写链、deleteRole、loadRoles、showRoleMembers、所有 data-testid、模板与样式、后端与 SDK。

## 4. RED→GREEN 台账（静态）

- 契约：`RoleManager.edit.contract.test.ts`，5 it，readFileSync 读源码 + 花括号配对 bodyOf 提取 editRole/onSave 函数体 + new Function 真执行；自定义函数参数全带 `: string` 标注（遵 R330 TS7006 教训）。
- RED（修复前）：`pnpm exec vitest run frontend/apps/desktop/src/views/RoleManager.edit.contract.test.ts` exit=1，2 failed / 3 passed，失败点正是「源码含 r.id 兜底」与「仅 id 角色编辑态 form.flag=uuid」。
- GREEN（修复后）：定点 5/5 passed；全量 `pnpm exec vitest run` 271 files / 2181 tests / 0 failed；`pnpm --filter @oa4rust/desktop typecheck` exit 0。

## 5. 真浏览器复验（1440x900，非物理真机）

- 流程：登录 → /app/admin/role → 新建 marker 角色（填 name+flag）→ 点编辑（断言 editPrefill=uuid 非空、标题「编辑角色」）→ 改名保存并捕获 PUT → 列表名更新 → reload 持久 → 删除 marker 清理（已授权回归路径）。
- 结果：10 断言全 true：createdRoleVisible / flagFallbackToId / editModalIsEdit / putIssued / putPathUsesUuid / nameUpdatedInList / editedNamePersistsAfterReload / deleteRegressionClean / noPageErrors / noApiFailures。
- HTTP：POST /api/role 200、PUT /api/role/{uuid} 200、DELETE /api/role/{uuid} 200；pageErrors=[]、apiFailures=[]。
- 证据：gitignored 目录 test-results/r335-role-edit-fix/，3 张 1440x900 截图 01-after-edit.png（sha256 c6b450256760834bfa614aeeb799243844ad92933febd54925c9c5d4269c0f66）、02-after-reload.png（sha256 e09fdf92929cf99abe5c9bc5b2e5a3d309391808b77465f7e0f5c954e9de7e20）、03-after-delete.png（sha256 2c9b269978b08c95b0a41eb3d9359c3c6581ae4a1c70c91cfa38d32bba611b7a）；evidence.json source 标注 Playwright Chromium headless against deployed test server; NOT physical device。
- 踩坑留痕：首次部署只传 index.html+主 JS/CSS 导致登录后动态 chunk（Dashboard-*.js）404、waitForURL 超时，改整目录 tar 原子替换后恢复；真浏览器脚本两处断言 bug（PUT 路径带 /oa4rust-api 前缀致 indexOf===0 误判、marker 子串匹配含新名致 oldNameGone 误判）修正后 10/10 绿，产品行为本身正常。

## 6. 取证限制与结论

- 破坏性写（新建/删除）仅走已授权回归路径并清理，不额外伪造数据。
- P2 候选与 Settings 取证限制按边界保留，不顺手扩大。
- 结论：R334 坐实的 P1 编辑角色 100% 失效已闭环，PUT 真发出且刷新后名称更新；静态与真浏览器双证据支持，待 t138 终审。
