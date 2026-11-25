# R532 IMChat 会话项键盘可达性（WCAG 2.1 SC 2.1.1）只读甄别

- 任务：R532（台账 §523，真父 `cd586e4112845342c9bc766443a99bd0d58878e1`）
- 性质：只读甄别，全程不改源码、不提交源码
- 环境：本地 Vite dev server + Playwright chromium headless，1440×900、DPR 1、light/dark 双主题；API stub 占位数据，无真实凭据、无真实写库
- 测试地址：`<TEST_HOST>`（占位；运行时使用本地回环，真值不入文档）
- 判定：**可达真缺陷 1 项** — WCAG 2.1 SC 2.1.1 Keyboard（Level A），优先级 P2

## 一、结论

即时通讯会话列表中的会话项（`.conv-item`）是切换会话这一关键功能的唯一入口：它只有 `@click="selectConversation(conv)"`，无 `tabindex`、无 `role=button`、无 `@keydown` 键盘处理器。真浏览器 CDP 实证：会话项 AX `role=listitem`、`tabindex=null`、不可被 Tab 聚焦，Enter/Space 均无法激活 `selectConversation`；同页搜索框仅做过滤、消息输入框 Ctrl+Enter 仅发送消息，均非切换会话的等价键盘入口。键盘用户无法切换到任何会话，违反 SC 2.1.1（Level A）。鼠标点击对照可正常切换会话，证明检测器与功能链路有效，缺陷确凿。

## 二、源码逐行锚点

| 文件 | 行 | 事实 |
|---|---|---|
| `frontend/apps/desktop/src/views/IMChat.vue` | 17 | `<div class="conversation-list" role="list">` 会话列表容器 |
| 同 | 18–20 | `<div v-for="(conv, convIndex) in filteredConversations" class="conv-item" role="listitem" :data-testid="'im-conv-'+(convIndex+1)" @click="selectConversation(conv)">` —— 仅 `@click`，无 tabindex/role=button/@keydown |
| 同 | 674–685 | `function selectConversation(conv)`：结束通话、关闭房间 WS、`selectedChat.value = conv`、`markConversationRead`、`joinRoom`、`loadConversationDetail` —— 切换会话的关键业务逻辑 |
| 同 | 318–326 | `useQuery` 真实 API `GET /api/message/assemble/communicate/im/conversation/list/my`，列表数据真实可达 |
| 同 | 335–341 | `filteredConversations` computed：搜索框 `searchMsg` 仅做客户端过滤 |
| 同 | 14–15 | 搜索框 `input.search-input`：`v-model="searchMsg"`、`aria-label="搜索会话"`，仅过滤 |
| 同 | 96–97 | 消息 `textarea`：`@keydown.enter.ctrl="sendMessage"` / `@keydown.enter.shift.exact`，仅发送消息 |
| 同 | 187/189 | `searchMsg = ref('')`、`selectedChat = ref<Conversation | null>(null)` 状态定义 |
| 同 | 1172–1177 | `.conv-item { cursor:pointer; ... }`、`.conv-item.active` 仅视觉高亮，无焦点样式 |
| `frontend/apps/desktop/src/main.ts` | 68 | `{ path: 'im', name: 'IMChat', component: () => import(.../IMChat.vue) }` → 路由 `/app/im` |
| 全域 grep | — | `selectConversation` 唯一调用点为模板 :20；无其它键盘等价调用路径 |

## 三、判定依据（CDP DOM/AX/键盘实证，light/dark 双主题一致）

1. **列表真实可达**：CDP/DOM 坐实 `.conv-item` 共 2 条（stub API 返回 2 条会话）；右侧为「选择会话」占位态（`noChatPlaceholder=true`、`activeIdx=-1`）。截图确认列表渲染正常，非 R525 式死代码、非空态，排除「不可达 L1」。
2. **AX 语义与可聚焦性**：`Accessibility.getPartialAXTree` 与 `getFullAXTree`（`fullMatched=true`）均显示会话项 `role=listitem`、`name` 空、`ignored=false`、`childIds=3`、`properties={level:1}`；DOM 属性 `class=conv-item`、`role=listitem`、`data-testid=im-conv-1/2`、`tabindex` **缺失（null）**。角色为 listitem 而非 button/可交互控件，无键盘焦点属性。
3. **Tab 不可达**：从搜索框起连续按 Tab 12 次，焦点轨迹依次为 `mark-all-btn → BODY → skip-link → div.logo → nav-item(工作台/通讯录/日历/文件/全局搜索/消息/会议/论坛) …`，`tabReachesConv=false`，从未落到任何 `.conv-item`。
4. **Enter/Space 不激活**：程序化 `el.focus()` 无法使焦点停留（`programmaticFocusStuck=false`，`activeElement` 为 `nav-item`）；对会话项派发 Enter/Space keydown/keyup 后，`keyboardSwitched=false`（`activeIdx` 仍 -1、`hasChatContent` 仍 false），`selectConversation` 未被触发。
5. **去伪：无等价键盘入口**：搜索框填入「乙」后列表由 2 项过滤为 1 项（`searchSwitched=false`），不切换会话；消息 textarea 的 Ctrl+Enter 绑定 `sendMessage`，与会话切换无关；grep 坐实 `selectConversation` 唯一调用点 :20。
6. **鼠标对照证检测器有效**：点击第 2 条会话后 `activeIdx=1`、`hasChatContent=true`、`chatName="R532会话乙"`、`sidebarCollapsed=true`，`mouseSwitched=true`；功能链路与检测器均有效，键盘不可达为真实差距。
7. 两组主题均 `pageErrors=[]`、`fatal=null`。

## 四、三分判定

| 分类 | 结论 | 依据 |
|---|---|---|
| 可达真缺陷（P2） | **成立：SC 2.1.1 Keyboard（Level A）** | 会话项列表可达（2 条）、为切换会话唯一入口、键盘 Tab/Enter/Space 全部无法激活、无等价键盘入口、鼠标对照有效 |
| 不可达 L1（列表无数据） | 排除 | stub API 返回 2 条会话、DOM 渲染 2 项、截图可见，非死代码非空态 |
| 正常规范预期（有等价键盘入口） | 排除 | 搜索框仅过滤、textarea Ctrl+Enter 仅发消息、selectConversation 唯一调用点 :20，无同功能键盘控件 |

说明：`r532-scout` 曾把此项归入「已覆盖 2.1.1 维度内禁报」，但台账 R492（FileManager 面包屑/文件行）、R493–R494（MeetingApp 会议卡片/日历条目）、R495–R496（QueryStatementDesigner `.sb-item` 语句项）均为不同组件实例；SC 维度覆盖 ≠ 具体实例覆盖（先例：R509 FileManager file input 与 R515 QueryManagerDeep file input 同构但分别立案）。IMChat 会话项实例此前从未专项甄别，本次据实判定，不因 scout 字样预判 0，也不因 `@click` 直接预判 P2。

## 五、证据清单（sha256）

| 证据 | sha256 | 备注 |
|---|---|---|
| `oa4rust/test-results/r532/r532-ax-report.json` | `6c79ae40b4ec648088ccb555afd7a86ec8ddaf075f156a4394c7fbaaaff6c416` | 双主题 CDP/DOM/AX/键盘报告（fatals=0） |
| `oa4rust/test-results/r532/r532-im-list-light.png` | `a348bef29730afe5bdc9c9c3bc5ee2f3289a0602ad78e2d1129835ed7e7c8d7b` | 1440×900，177630 B，会话列表初始态 |
| `oa4rust/test-results/r532/r532-im-list-dark.png` | `f0a53fb8979a06715b4c337ac45fe595eca56abea959f638e92130d3d5e62c59` | 1440×900，231367 B，会话列表初始态 |
| `oa4rust/test-results/r532/r532-im-light.png` | `a9ab33cf7b86e3b508837dd5345a1fc32d8651177db4f261df0009fd42104c15` | 1440×900，183689 B，鼠标切换后态 |
| `oa4rust/test-results/r532/r532-im-dark.png` | `f739b3df9546cab2b5af8447d643fb9eed9285621dd4af92c8a5f5c37d5de76a` | 1440×900，243076 B，鼠标切换后态 |
| `oa4rust/frontend/apps/desktop/src/views/IMChat.vue`（源码锚点） | `3fa954a5ead62873d5da672a59102ef2fd9dc6c9d7b53020faa6bb50b67eec74` | 只读，未改动 |

截图目视：light/dark 均见会话列表两条会话项与右侧「选择会话」占位，与 DOM 初始态一致。

## 六、最小修复方向（不在本次改动范围）

类比 R494/R496 的做法：为 `.conv-item` 增加 `tabindex="0"` 与 `@keydown.enter` / `@keydown.space.prevent` 复用 `selectConversation(conv)`，使其可由键盘聚焦并激活；同时保留 `role="list"` 容器的列表语义（注意 `role=listitem` 与可交互角色的兼容，必要时改用内层 `<button>` 承载点击目标）。业务函数零改，不改变鼠标行为。

## 七、脱敏自检

文档仅含占位域名 `<TEST_HOST>` 与占位 stub 数据；无真实域名、IP、SSH 端口、口令、个人目录或环境路径。探针用本地回环加载仓库源码、API stub 占位数据，真值均不入仓。