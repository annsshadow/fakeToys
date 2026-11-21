# R336 PC Settings 配置保存 + 回收站写路径覆盖甄别（只读审计）

## 1. 范围与环境

- 轮次：R336，任务 t139；性质：只读深度审计，不改源码/契约/配置/后端。
- 对象：PC 系统设置 Settings.vue（路由 /app/settings，main.ts:75-80）与回收站 RecycleApp.vue（路由 /app/recycle，main.ts:107-110）。
- 环境：测试服 <YOUR_TEST_HOST>，本地 Playwright Chromium headless 1440x900。**截图与 evidence 均为本地无头浏览器实测，非物理真机。**
- 凭据：运行期经 gitignored 部署脚本 + ssh 通道读取，未入库、未回显；域名/主机统一用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位，全文 0 真值。
- 方法：真实 UI 走查配置修改→保存→刷新回显、回收站恢复/确认守卫；page.on(response) 记录所有 /api/ 状态；pageerror 全程监听。证据落 gitignored 目录 test-results/r336-settings-recycle-triage/（7 张 1440x900 截图 + evidence.json）。

## 2. 覆盖甄别清单（静态文档核对，先于实测）

逐一核对 docs/ops 下既有审计（约 80 篇 md；未发现独立 R310 文档，任务所列 R315 实为 oa4rust-r316-deep-functional-audit.md，其覆盖日程新建/搜索修复/BAM 周期/通知已读/工作流已办 5 域，不涉回收站写路径）：

| RecycleApp 写端点 | 源码 | 既有覆盖 | 判定 |
|---|---|---|---|
| 列表加载 GET /api/recycle/list | :138 | R301 重写 allItems/items 计算属性与双筛选；R304 视图冒烟仅 recycle.png | 已覆盖（列表/筛选） |
| 单条恢复 POST /api/recycle/resume/{id}（页内按钮） | :150 | R321 仅测 FileManager 侧触发 resume（:45），**非 RecycleApp 页内按钮**；R301 批量恢复覆盖的是 batchResume | 未覆盖（页内单条入口） |
| 批量恢复 Promise.allSettled | :179 | R301 :42 全选+批量恢复回收站 6 项→0 项 | 已覆盖 |
| 单条永久删除 DELETE /api/recycle/{id}/delete | :163 | R304 destructiveGuards.test.ts:16-18 仅源码级守卫断言（4 个破坏性动作含永久删除/批量删除/清空），非真 UI 写 | 未覆盖（真 UI） |
| 批量永久删除 DELETE | :197 | 同上，仅源码级守卫 | 未覆盖（真 UI） |
| 清空 DELETE /api/recycle/empty | :214 | 同上，仅源码级守卫 | 未覆盖（真 UI） |
| 孪生探针 DELETE /api/recycle/0/delete + POST /api/recycle/0/resume | :227 | 无 | 未覆盖（s() 吞错可用性探测） |

结论：R301/R321/R304 覆盖了列表、批量恢复、FileManager 侧恢复与源码级守卫断言；RecycleApp 页内单条恢复、永久删除/批量删除/清空的真 UI 写闭环从未实测。本轮对「未覆盖且非破坏性」的页内单条恢复做真写验证；对三个破坏性写端点只验证确认守卫（弹窗+取消），不伪造写。

## 3. Settings 配置保存三分判定

### 3.1 系统参数 + 通知开关保存→刷新回显 — 【通过】（补 R334 取证限制）

- R334 曾把「刷新回显控件级断言」标为取证限制（脚本 first input 命中全局搜索框，未命中 #cfgName）。本轮改用精确 id 定位与父级 label 点击（.switch input 被 CSS opacity:0;width:0;height:0 隐藏，Settings.vue:188，须点 label.switch），补证成功。
- HTTP：GET /api/config/system=200（loadCfg :118-133，数字 Number/布尔 raw==="true"/字符串原值回填）；修改系统名称后 POST /api/config=200（buildConfigs :138-143 组装 system 类，persistConfigs :145-155）；切换「邮件通知」开关后 POST /api/config=200（notify 类）；刷新后 GET /api/config/system=200。
- DOM 断言：系统名改为测试值后刷新，#cfgName inputValue 回显一致（nameEchoOk=true）；邮件通知由原 true 改 false 后刷新，#cfg-emailNotify isChecked=false 与目标一致（emailEchoOk=true）；截图 S2-reload-echo.png（1440x900）肉眼可见通知配置页邮件通知/短信通知开关为关、WebSocket 实时通知为开，与断言相符。测试末已把系统名与邮件通知复位到原值（cfgRestoreDone=true），未污染测试服配置。
- pageerror=0。控件数：基础配置 input.fi=2（系统名称/系统 Logo），任务描述所提「9 控件」为全部分组之和，实际按 tab 分组（基础/安全/通知/关于）。
- 判定：通过。Settings 保存闭环真实可用，R334 取证限制已消除，非缺陷。

## 4. RecycleApp 未覆盖写端点三分判定

### 4.1 页内单条恢复 POST — 【通过（API 层坐实，DOM 行数部分取证限制）】

- HTTP：点页内首行「恢复」按钮，POST /api/recycle/resume/{id}=200（resumeStatus=200，窗口内该 POST 计数=1）；源码 resume（RecycleApp.vue:150）成功即本地 filter 移除该项。
- DOM：恢复前页内「恢复」按钮 2 个（2 条待处理项）；恢复并刷新后截图 R1-resumed.png / R2-pdel-confirm.png 可见待恢复列表由 2 条减为 1 条（剩余 R309末次文件），恢复生效。备注：脚本用于统计行数的选择器（.recycle-row/tr/.list-row）未命中实际 DOM 类名，recycleRows 字段为 0 属选择器失配，不作强断言；写端点 200 与截图对比已足以坐实闭环。
- 判定：通过。补全 R321 未覆盖的 RecycleApp 页内单条恢复入口，非缺陷。

### 4.2 单条永久删除（确认守卫） — 【取证限制：确认弹窗取消，未执行破坏性写】

- 源码 permanentDelete（:163）前置 confirmMsg(:171)。真浏览器点首行「永久删除」弹出 #oa4-confirm-overlay（role=dialog），文案「确定永久删除「R309末次文件_…」？此操作不可恢复。」+ 取消/确认（截图 R2-pdel-confirm.png 清晰可见）。
- 点「取消」后窗口内 DELETE /api/recycle/{id}/delete 计数=0（pdelGuardOk=true）：确认守卫真实生效，未授权破坏性写不伪造。
- 判定：取证限制（按铁律不伪造破坏性写）。守卫逻辑本身通过；真实 DELETE 落库效果不在本轮授权范围，不记为缺陷。

### 4.3 清空回收站（确认守卫） — 【取证限制：确认弹窗取消，未执行破坏性写】

- 源码 emptyRecycle（:213）confirmMsg「确定清空回收站？所有项目将被永久删除。」(截图 R3-empty-confirm.png)。点「取消」后 DELETE /api/recycle/empty 计数=0（emptyGuardOk=true）。判定同 4.2：取证限制，守卫通过，非缺陷。

### 4.4 批量永久删除 DELETE — 【取证限制】

- 源码 batchDelete（:197）与单条永久删除共用 confirmMsg 守卫，同属未经授权不得伪造的破坏性写；R304 仅有源码级守卫断言。本轮不真写，标取证限制；批量恢复已由 R301 覆盖，不重复。

### 4.5 孪生探针 s() 吞错 — 【产品语义差异】

- loadRecycleTwin（:227）以 s()=p.catch(()=>null) 提交 DELETE /api/recycle/0/delete 与 POST /api/recycle/0/resume 两个占位探针并吞掉失败，与 R329/R332 已判同构（可用性探测，非用户触发）。判定：产品语义差异，非缺陷。

## 5. 汇总

- 新增真缺陷：0（本轮为甄别与补证轮，未坐实任何新缺陷）。
- 通过项 3：Settings 系统参数+通知开关保存刷新回显（补 R334 取证限制）、RecycleApp 页内单条恢复、加载与确认守卫生效。
- 取证限制 3：单条永久删除、批量永久删除、清空回收站——破坏性写未授权，仅验证确认弹窗与取消后 0 DELETE。
- 产品语义差异 1：孪生探针 s() 吞错。
- 全程 pageerror=0；0 真值入文档，截图均为 1440x900 本地无头浏览器实测、非物理真机。
- 未给修复方向：本轮无新增真缺陷，故不适用；若后续授权真跑破坏性写，建议另派独立审计并先做数据备份。