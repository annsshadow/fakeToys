# R334 PC RoleManager / Settings 写路径深度审计（只读甄别）

## 1. 范围与环境

- 轮次：R334，任务 t136；性质：只读深度审计，不改源码/契约/配置/后端。
- 对象：PC 角色管理 RoleManager.vue（路由 /app/admin/role，main.ts:205-210）与系统设置 Settings.vue（路由 /app/settings，main.ts:75-80）。两页均不使用 AdminToolbar/AdminTable，不属 R332 默认插槽风险类。
- 环境：测试服 <YOUR_TEST_HOST>，本地 Playwright Chromium headless 1440x900。**截图与 evidence 均为本地无头浏览器实测，非物理真机。**
- 凭据：运行期经 gitignored 部署脚本 + ssh 通道读取，未入库、未回显；域名/主机统一用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位。
- 方法：真实 UI 走查新建→回显→编辑→成员→删除→500 兜底；page.on(response) 记录所有 /api/ 状态；pageerror 全程监听。

## 2. RoleManager（角色管理）三分判定

### 2.1 新建角色 → 列表回显 — 【产品语义差异 + 可用性候选】

- HTTP：POST /oa4rust-api/api/role = 200；列表由 9 行增至 10 行（新增行 flag=75c6e2bc-…，uuid）；pageerror=0。
- 源码：onSave（RoleManager.vue:152-163）payload={name, description}，**用户输入到 flag 输入框的值未发给后端**；后端 role_create（organization_assemble_control/u2_org.rs:894-926）`let id = uuid::Uuid::new_v4()` 生成 id 作为标识返回。
- DOM：用户输入 flag=R334V293498136，回显列表 flag 却是 uuid，userFlagEchoed=false。
- 判定：新建功能本身可用（写成功、列表回显），但界面标注 Flag（唯一标识）的输入框（:44-45）用户输入完全不生效、实际标识由后端 uuid 生成，属产品语义差异 + 误导性 UI 的可用性候选（P2），非阻断性缺陷。最小建议：新建弹窗去掉 flag 输入或标注由系统生成，避免用户误以为自定义标识生效。

### 2.2 编辑角色 PUT → 回显更新 — 【真缺陷 P1】

- **编辑功能完全失效，PUT 请求根本未发出。**
- HTTP：编辑流程中 api 仅见 POST /api/role 与 DELETE /api/role/{id}，**无任何 PUT /api/role/{flag}**；editStatus=null。业务后果：编辑后再刷新，名称仍为原名 R334V2-审计角色（未变成尝试填写的 R334V2-已编辑），editedNameEcho 未更新。
- 根因链（三重叠加，源码+DOM+HTTP 全部坐实）：
  1. 后端 role_list（auth/src/lib.rs:921-950）`SELECT id, name, description FROM auth_role`，返回 map 只含 id/name/description，**不含 flag 字段**。
  2. 前端 editRole（RoleManager.vue:146-150）把 form.flag 设为 r.flag 或空串；因列表项无 flag，form.flag 被置空。DOM 证据：editModalFlagPrefill 为空串。
  3. 编辑弹窗 flag 输入框 :disabled=!!editingRole（:45），用户无法手动补填。
  4. onSave（:153）`if (!form.value.flag.trim()) { toast.info(Flag不能为空); return }` 在编辑态因 flag 为空直接 return，**PUT 被拦截在发请求之前**（:157 的 api.put 不可达）。
- 判定：真缺陷 P1。编辑是角色管理核心操作，100% 不可用且无任何网络错误（前端静默拦截），用户仅见 Flag不能为空 提示却无法修正（输入框禁用），属功能性阻断。
- 最小修复方向（仅建议，未改源码）：编辑态用 r.id 兜底填 flag（与列表显示 r.flag 或 r.id 一致，:25），并将 onSave 校验改为仅新建态校验 flag；因后端 role_edit 用 resolve_generic_id（u2_org.rs:937）支持 id 解析，传 uuid 作为 path 可正常工作。

### 2.3 删除角色 DELETE → 列表刷新消失 — 【通过】

- HTTP：点击删除弹出 #oa4-confirm-overlay（confirmText=确认操作/确定删除角色/取消/确认），点确认后 DELETE /oa4rust-api/api/role/{id} = 200；刷新后目标行消失（targetAfterDelete=false）。
- 源码：deleteRole（RoleManager.vue:170-178）confirmMsg 确认后 api.delete(`/api/role/${r.flag||r.id}`)。pageerror=0。判定：删除闭环正常。

### 2.4 角色成员授权只读可视化复核 — 【通过】

- HTTP：GET /api/organization/assemble/control/person/list/role/{id} = 200；DOM：[data-testid=role-member-detail] 弹窗可见，文案含 角色权限明细 / 当前共有 0 名成员 / 该角色暂无分配成员，可在组织架构中完成分配。与 R309 结论一致，纯只读，未执行任何授权写。

### 2.5 加载失败 500 兜底与空态可区分 — 【通过】

- 用 page.route 将 role/list fulfill 500：页面显示 [data-testid=role-list-error]，文案 角色列表加载失败：HTTP 500: Internal Server Error 重试，mode=empty（empty=1/banner=0）；与真正 暂无角色 空态（:16 图标+新建第一个角色按钮）明显可区分。源码 loadRoles（:131-144）加载失败保留旧数据并显式报错。

## 3. Settings（系统设置）三分判定

### 3.1 表单加载 + 控件 — 【通过（取证口径见下）】

- HTTP：GET /api/config/system = 200。DOM：4 个设置分组（基础配置/安全设置/通知配置/关于）齐全；基础配置可见 4 控件（系统名称=OA4Rust、系统Logo=图片URL占位、时区=Asia/Shanghai、语言=中文）+ 保存设置 按钮；label 清单=[系统名称, 系统 Logo, 时区, 语言]。
- 任务描述的 9 控件 为本轮实测口径修正：当前 Settings 仅基础配置分组渲染 4 个输入控件，其余分组为独立 tab 未同时挂载；此为产品结构差异，非缺陷。

### 3.2 保存 POST /api/config → 刷新回显 — 【写端点通过，回显取值取证限制】

- HTTP：POST /oa4rust-api/api/config = 200（源码 onSave :149 api.post(/api/config, {configs})），保存动作真实成功、pageerror=0。
- 取证限制：脚本用 page.locator(input).first() 读取值，命中的是全局搜索框（返回空串）而非系统名称输入框，导致 cfgFirstInputBefore/After 均为空、cfgValueKept 比较不具判别力；但截图 S1-settings-load.png 肉眼可见系统名称=OA4Rust、时区=Asia/Shanghai、语言=中文，加载值真实存在。刷新回显的 值一致性 未做精确控件级断言，标注为取证限制而非缺陷。

### 3.3 加载失败兜底 — 【通过（弱提示，符合源码设计）】

- 用 page.route 将 config/system fulfill 500：页面 body 含 系统参数读取失败: HTTP 500: Internal Server Error 提示 toast。源码（Settings.vue:121-123）加载失败仅 toast、无 ListLoadError 空态组件——与 RoleManager 的显式兜底不同，属产品实现差异，非缺陷（错误信息已明确呈现）。

## 4. 关键契约回归与 pageerror

- 两页全程 pageerror=0（首轮 errors 为空，v2 补测 errors 为空）。
- 端点注册核对：role 全套 POST /api/role→role::create、PUT/DELETE /api/role/{flag}、GET /api/authentication/role/list 均已注册；config GET /api/config/system 与 POST /api/config 双路由同 handler（lib.rs:2733-2734 注释确认）。契约形状一致。
- 脚本过滤说明：首轮 roleCreateStatus/cfgSaveStatus 报 null 系脚本用精确路径 p===/api/role 匹配，而实测 p 带 /oa4rust-api 前缀；v2 改 endsWith 后 POST /api/role=200 坐实，非业务问题。

## 5. 结论汇总

- 真缺陷：1 个 P1（编辑角色完全失效，2.2）。0 个 P0。
- 产品语义差异/可用性候选：1 个（新建 flag 输入不生效，2.1，P2 候选）。
- 取证限制：1 个（Settings 刷新回显控件级断言，3.2）。
- 通过项：新建写、删除闭环、成员只读复核、角色 500 兜底与空态区分、Settings 加载/保存端点/500 弱提示。
- 本轮只写本文档，未改任何源码/契约/后端。P1 与 P2 候选待队长另派修复。≥1000 轮持续迭代，非终态。

## 6. 证据

- 截图（gitignored）：test-results/r334-role-settings-audit/ 下 R1-role-list、R2-role-created、S1-settings-load、S2-settings-saved、R6-role-load-fail、S3-settings-load-fail、V2-R3-edited、V2-R5-member、V2-R4-deleted 等 png，均为 1440x900 headless 渲染，非物理真机。
- 结构化证据（gitignored）：test-results/r334-role-settings-audit/evidence.json 与 evidence-v2.json（含各页 DOM、API 状态、verdicts、pageerror）。
- 脚本（gitignored）：test-results/r334-role-settings.cjs / -v2.cjs 及对应 run.sh；BASE 经规范化补 /oa4rust，凭据运行期 ssh 读取不回显。