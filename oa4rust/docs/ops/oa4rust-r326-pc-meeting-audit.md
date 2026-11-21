# R326 PC 会议管理深度功能审计（首次全模块覆盖）

> 审计轮次：R326（t120）｜范围：PC \`/oa4rust/app/meeting\` 会议管理页（\`MeetingApp.vue\`，\`h1=会议管理\`，副标题「会议室预订 · 我的会议 · 日程安排」）
> 性质：只读深度功能审计，不修改任何源码/契约/配置，不部署、不修复。
> 浏览器口径：Playwright Chromium headless，1440×900，对部署测试服实测；截图与 evidence 的 \`source\` 均标注 \`Playwright Chromium headless against deployed test server; not a physical device\`，**非物理真机**。
> 环境口径：测试服域名/host/凭据仅在运行期从 gitignored 脚本读取，本文档一律用 \`<YOUR_TEST_DOMAIN>\` / \`<YOUR_TEST_HOST>\` 占位，0 真值。
> 判定纪律：①真缺陷=真 UI 可复现 + 真请求/响应 + 页面回显 + 源码定位；②取证限制=无种子数据/无入口无法驱动；③产品语义差异=设计取舍。不把②③当 bug，不臆造，不重复已闭环项。
> 与既有审计的关系：R309 已修会议新建两处缺陷（datetime-local 直绑 timestamp 致 HTTP500；\`applied=false\` 致新建会议不在列表可见），本轮核验两修复生效，不重复报；R317 覆盖 H5 侧，本轮聚焦 PC 从未深度审计的模块链。

## 一、结论摘要

| 编号 | 判定 | 严重度 | 一句话 |
| --- | --- | --- | --- |
| D1 | 真缺陷 | P2 | 「我的会议」搜索框/状态筛选有 UI 但不生效：前端构造了 query 参数却从未传给请求，后端端点也不支持过滤，输入无匹配词后列表纹丝不动、无空态 |
| L1 | 取证限制 | — | 楼栋/会议室联动无法完成有效选择：后端 \`building/list\`、\`room/list\` 返回空数组（无种子数据），非前端填充缺陷 |
| P1 | 产品语义差异 | — | 会议卡片无「删除/取消」入口：模板有意只暴露 7 个业务按钮（加入/参会人/邀请/邀请管理/改时间/标记完成/签到码），脚本层有端点但前端未渲染 |
| G1 | 回归观察（正常） | — | R309 两修复生效：真新建 200、列表/刷新可见；详情 5 端点全 200 且弹窗正确；加载失败兜底明确；全程 0 pageerror |

## 二、D1【真缺陷·P2】搜索/筛选 UI 承诺不生效

**可复现路径**：登录 → \`/oa4rust/app/meeting\` → 在「我的会议」搜索框输入 \`____NO_SUCH_MEETING____\` → 点「搜索」→ 列表仍显示 3 条会议（含无关会议），无任何空态提示。切换状态筛选「未开始/进行中/已结束」同样不生效。

**真请求/真响应/页面回显**（Playwright 页面内实测）：
- 搜索前 \`listBeforeSearch=3\`；输入无匹配词点搜索后 \`listAfterNoMatchSearch=3\`；\`emptyAfterSearch=[]\`（无空态）。
- 全程监听到的列表请求始终是固定 URL \`GET /oa4rust-api/api/meeting/assemble/control/meeting/list/apply/1/size/50\`（HTTP 200），**没有携带 key 或 status 查询参数**。

**源码定位（协议与源码双证）**：
- 前端 \`oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:208-212\`：
  \`const p: Record<string, string> = {}\`、\`if (searchKey.value) p.key = searchKey.value\`、\`if (statusFilter.value !== '') p.status = statusFilter.value\`，但 \`:212\` 调用 \`api.get('/api/meeting/assemble/control/meeting/list/apply/1/size/50')\` —— 构造的 \`p\` 从未拼接进 URL，成为死变量。
- 后端 \`oa4rust/backend/crates/meeting_assemble_control/src/lib.rs:1054-1079\` \`meeting_list_apply_page_size_size(pool, session, Path((page, size)))\` —— 处理器签名只接 \`Path((page,size))\`，无 \`Query\` 参数；SQL \`FROM x_meeting WHERE applied=true ... ORDER BY create_time DESC LIMIT $1 OFFSET $2\` 无 key/status 过滤条件。路由 \`routes.rs:42\` / \`:131\` 注册该 GET/POST 路径。

**修复方向**（本轮只读未改）：前端按 \`p\` 构造 URL query（\`?key=...&status=...\`）且后端加对应 \`Query\` 过滤并下推 SQL，两侧需联动；任一侧单独改都无法闭环。建议另立 implementation 任务，先写契约锁定「搜索无匹配返回 0 条 + 空态」再修。

## 三、L1【取证限制】楼栋/会议室联动无法完成有效选择

**现象**：楼栋下拉只有占位 \`选择楼栋\`，选楼栋后会议室下拉仍为空；\`roomOptsAfterCascade=[]\`。

**决定性探针**（\`test-results/r326-meeting-probe.cjs\` → \`probe.json\`，页面内 fetch）：
- \`GET /api/meeting/assemble/control/building/list\` → 200 \`type:success\`，\`dataLen=0\`（空数组）。
- \`GET .../building/list/start/1/completed/0\` → 400 \`type:error\`（该变体端点非本页调用路径，仅探针探测）。
- \`GET .../room/list\`（无参）→ 200，\`dataLen=0\`。
- 对照 \`GET .../meeting/list/apply/1/size/50\` → 200，\`dataLen=3\`（含新建会议），证明会话与接口连通正常。

**判定**：楼栋/会议室为空是后端无种子数据，不是前端填充 bug（前端 \`loadRooms\` 带代际令牌请求 \`room/list?buildingId=\`，链路存在）。不重复计为真缺陷。需补种子数据后方可验证联动闭环。

## 四、P1【产品语义差异】会议卡片无删除/取消入口

**现象**：\`cardButtons=[加入, 参会人, 邀请, 邀请管理, 改时间, 标记完成, 签到码]\`，\`hasDeleteOrCancel=false\`。R321/R326 需求描述提到「取消/删除」，但模板 \`MeetingApp.vue\` 会议卡未渲染这两个按钮；脚本层虽有 \`meeting/delete\`、\`meeting/cancel\` 类端点，前端有意不暴露。

**判定**：属产品语义/设计取舍（前端只暴露 7 个会议运营按钮），非缺陷。若业务确需取消/删除入口，应作为需求变更单独立项，不作为本轮 bug。

## 五、G1【回归观察·正常】R309 修复生效与核心链路

1. **真新建（R309 缺陷 A/B 回归）**：填表（主题 \`R326审计会议_1795276849927\`、时间 2026-11-24 14:00~15:00、说明「R326 深度审计真写验证」，楼栋/会议室为空因 L1）→ \`POST .../meeting/create\` → 200 \`{"data":{"id":"dbb96076-...","creator":"admin","title":"R326审计会议_1795276849927",...},"type":"success"}\`（不再 HTTP500，证明 datetime-local→timestamp 修复仍在）。\`errText=[]\`。
2. **列表可见与持久（R309 缺陷 B 回归）**：\`listCountInitial=2 → listCountAfterCreate=3\`，\`titleInList=true\`；reload 后 \`listCountAfterReload=3\`、\`titleAfterReload=true\`（证明 \`applied=TRUE\` 写入修复仍在）。
3. **详情弹窗**：点列表首卡 \`detailOpen=1\`，\`detailModalText\` 含标题/时间 2026-11-24 14:00:00~15:00:00/创建者 admin/说明/控制项 0 项/关联 附件0·后续会议4·前序会议0；并发 5 端点全 200（\`meeting/{id}\`、\`list/{id}\`、\`list/{id}/next/5\`、\`list/{id}/prev/5\`、\`attachment/list/meeting/{id}\`）。
4. **加载失败兜底**：主动注入列表端点路由返回 500 → 页面显示 \`会议列表加载失败：HTTP 500: Internal Server Error\` + 「重试」，保留旧列表态（该项 \`apiFailures\` 中的 500 为主动取证注入，**不算真缺陷**）。
5. **稳定性**：\`pageErrors=[]\`，除注入 500 外 net 13 条全 200，无其他 API 失败。

## 六、逐条验收项覆盖（对应任务 7 项审计要求）

| 审计项 | 结论 |
| --- | --- |
| 楼栋联动 | L1 取证限制（后端无种子），链路存在但无法完成有效选择 |
| 新建会议真写 | G1-1 通过：create 200，R309 修复生效 |
| 我的会议列表 | G1-2 通过：列表即时回显 + reload 持久 |
| 详情 | G1-3 通过：5 端点 200、弹窗字段齐全 |
| 取消/删除 | P1 产品语义差异：无 UI 入口，非缺陷 |
| 兜底 | G1-4 通过：注入 500 显示明确错误并保留旧态 |
| R309 回归 | G1-1/G1-2 通过：两修复仍生效 |

## 七、证据路径与口径

- 审计脚本（gitignored）：\`oa4rust/test-results/r326-meeting-audit.cjs\`、\`oa4rust/test-results/r326-meeting-probe.cjs\`。
- 证据数据（gitignored）：\`oa4rust/test-results/r326-meeting-audit/evidence.json\`（\`source=Playwright Chromium headless against deployed test server; not a physical device\`）、\`probe.json\`。
- 截图（gitignored，均 1440×900 非物理真机）：\`01-meeting-initial.png\`、\`02-building-room-cascade.png\`、\`03-form-filled.png\`、\`04-after-create.png\`（sha256 \`727250112f58f92dffb1432434d6d5982b4503d69bad29ee60319b7f55dd3876\`）、\`05-detail.png\`、\`06-search-nomatch.png\`、\`07-load-error-fallback.png\`。
- 本轮只读：未改任何源码/契约/配置，未部署，未修复。D1 修复方向见第二节，建议另立 implementation 任务按 test-first 推进。
