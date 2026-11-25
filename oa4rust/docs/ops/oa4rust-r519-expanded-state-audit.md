# R519 只读甄别：WCAG 4.1.2 disclosure 状态属性 aria-expanded 缺失

- 类型：只读甄别（不改码、不提交源码）
- 真父：447ce21d2dbc425ecc5efbb5879ef891e321603b（台账 §512）
- 目标：PC 组织通讯录 / 组织查看页新建面板 disclosure 按钮，SC 4.1.2 Name, Role, Value 的 State Properties
- 运行环境：Playwright Chromium headless，1440×900 DPR1，light/dark 双主题，真浏览器 + CDP DOM.getAttributes + Accessibility.getPartialAXTree
- 脱敏：报告与日志中的测试域名统一替换为占位符 <TEST_HOST>，不入库任何真值域名/IP/凭据；截图与 runner 脚本位于 gitignored 目录 oa4rust/test-results/r519-*

## 一、结论

**三分判定：可达真缺陷（2 处）。** 组织通讯录 `org-create-unit-btn` 与组织查看页 `org-create-group-btn` 均为典型 disclosure：按钮切换 `v-if` 面板显隐，但折叠态和展开态 `aria-expanded` 属性均为 null，CDP AXTree 中亦无 `expanded` 状态属性，辅助技术无法读出面板开合状态。light/dark 双主题表现一致，4 项 × 2 主题 × 2 态全部 fatal=0。正面对照 AttendanceApp / CalendarApp 同名场景具备 `aria-expanded="false"→"true"` 且 AXTree 同步呈现 `expanded:false→true`，排除"CDP 读不到/工具假阴性"与"文案变化足以传达状态"两类去伪辩解。

## 二、源码甄别（逐行锚点）

| 文件 | 行 | 关键代码 | 判定 |
| --- | --- | --- | --- |
| `oa4rust/frontend/apps/desktop/src/views/OrgDirectoryApp.vue` | 72 | `<button type="button" class="org-create-toggle" data-testid="org-create-unit-btn" @click="showUnitCreate = !showUnitCreate">{{ showUnitCreate ? '收起' : '新建部门' }}</button>` | disclosure 切换按钮，无 `:aria-expanded` |
| 同上 | 73 | `<form v-if="showUnitCreate" ...>`（容器：71 `data-testid=org-create-unit-form`） | 受控面板随按钮显隐 |
| `oa4rust/frontend/apps/desktop/src/views/OrgViewer.vue` | 128 | `<button type="button" class="org-create-toggle" data-testid="org-create-group-btn" @click="showGroupCreate = !showGroupCreate">{{ showGroupCreate ? '收起' : '新建组织组' }}</button>` | disclosure 切换按钮，无 `:aria-expanded` |
| 同上 | 129 | `<form v-if="showGroupCreate">`（容器：127 `data-testid=org-create-group-form`，位于 EndpointFold 之外） | 受控面板随按钮显隐 |

同仓 utils 无全局增强器为按钮补 `aria-expanded`，静态缺失即运行时缺失。

## 三、契约与判据

- WCAG 2.1 SC 4.1.2 Name, Role, Value：用于展开/折叠内容的控件必须向辅助技术暴露当前状态（State Properties）。按钮可访问名从"新建部门/新建组织组"变为"收起"只提供动作提示，**不等价于状态属性**；同仓正面对照在文案变化之外仍显式绑定 `aria-expanded`，证明本地约定二者兼备。
- 三分判定口径：①可达真缺陷＝真浏览器实证状态属性缺失且控件可达；②不可达 L1＝控件不可达/被遮挡/需特殊权限；③正常＝状态属性齐备、AXTree 可读或无 disclosure 语义。
- 反预判：所有结论均来自 headless Chromium 运行时 CDP 探针，不以源码静态推断代替浏览器实证。

## 四、决定性对照（同仓既有约定）

| 对照 | 文件与行 | 关键代码 | 折叠态 | 展开态 |
| --- | --- | --- | --- | --- |
| AttendanceApp | `oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue:13`，面板 :15 | `<button data-testid="advanced-operations-toggle" :aria-expanded="showAdvanced" @click="showAdvanced = !showAdvanced">{{ showAdvanced ? '收起高级操作' : '⚙ 高级操作' }}</button>` | attr/prop `"false"`，AXTree `expanded:false` | attr/prop `"true"`，AXTree `expanded:true`，面板可见 |
| CalendarApp | `oa4rust/frontend/apps/desktop/src/views/CalendarApp.vue:47-54`，面板 :55-59 | `<button data-testid="calendar-create-entry" :aria-expanded="calCreateOpen" aria-label="新建日历" @click="calCreateOpen = !calCreateOpen">+ 新建日历</button>` | attr/prop `"false"`，AXTree `expanded:false` | attr/prop `"true"`，AXTree `expanded:true`，面板可见 |

两个正面对照与待测按钮同为"点击切换 + v-if 面板 + 文案变化"，唯一实质差异是 `aria-expanded` 绑定，因此去伪风险被证伪。

## 五、真浏览器取证结果（CDP 四维探针）

Runner：`oa4rust/test-results/r519-e2e.cjs` + `oa4rust/test-results/r519-run.sh`；报告：`oa4rust/test-results/r519-expanded-state/report.json`；日志 `oa4rust/test-results/r519-run.log`。摘要 `R519_DONE reportSha=9513a0d0707edf4932a1506fc9e81eff6e3c55683fc8e7475b790fa5e309fa2d groups=2 fatals=0`。DOM 探针读 `aria-expanded` 属性/property/全部属性/面板可见性；CDP 探针读 DOM.getAttributes 与 Accessibility.getPartialAXTree 的 `expanded` 状态属性。

| 主题 | 对象 | 折叠态 attr/prop | 折叠态 AXTree | 折叠态面板 | 展开态 attr/prop | 展开态 AXTree | 展开态面板 | 判定 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| light | pc-unit (`/oa4rust/app/directory/`) | null / null | role=button name="新建部门"，属性仅 `invalid:false, focusable:true`，`hasExpandedProperty=false` | panelFound=false / visible=false | null / null | role=button name="收起"，属性仅 `invalid:false, focusable:true, focused:true`，`hasExpandedProperty=false` | panelFound=true / visible=true | 缺陷 |
| light | pc-group (`/oa4rust/app/org/`) | null / null | role=button name="新建组织组"，`hasExpandedProperty=false` | panelFound=false / visible=false | null / null | role=button name="收起"，`hasExpandedProperty=false` | panelFound=true / visible=true | 缺陷 |
| light | pos-attendance（对照） | `"false"` / `"false"` | `expanded:false` | panelFound=false / visible=false | `"true"` / `"true"` | `expanded:true` | panelFound=true / visible=true | 正常 |
| light | pos-calendar（对照） | `"false"` / `"false"` | `expanded:false` | panelFound=false / visible=false | `"true"` / `"true"` | `expanded:true` | panelFound=true / visible=true | 正常 |
| dark | pc-unit | null / null | 同 light，`hasExpandedProperty=false` | panelFound=false / visible=false | null / null | 同 light，`hasExpandedProperty=false` | panelFound=true / visible=true | 缺陷 |
| dark | pc-group | null / null | 同 light，`hasExpandedProperty=false` | panelFound=false / visible=false | null / null | 同 light，`hasExpandedProperty=false` | panelFound=true / visible=true | 缺陷 |
| dark | pos-attendance（对照） | `"false"` / `"false"` | `expanded:false` | panelFound=false / visible=false | `"true"` / `"true"` | `expanded:true` | panelFound=true / visible=true | 正常 |
| dark | pos-calendar（对照） | `"false"` / `"false"` | `expanded:false` | panelFound=false / visible=false | `"true"` / `"true"` | `expanded:true` | panelFound=true / visible=true | 正常 |

折叠态 `panelFound=false` 是 `v-if` 面板未渲染的正确行为；展开态 `panelFound=true/panelVisible=true` 证明按钮点击确实控制了面板显隐，disclosure 语义成立而状态属性缺失。

### AXTree 属性级证据（示例，light pc-unit）

- 折叠：`axRole="button"`, `axName="新建部门"`, `axIgnored=false`, `axProperties={"invalid":"false","focusable":true}`, `hasExpandedProperty=false`
- 展开：`axRole="button"`, `axName="收起"`, `axIgnored=false`, `axProperties={"invalid":"false","focusable":true,"focused":true}`, `hasExpandedProperty=false`
- 对照（light pos-attendance 折叠/展开）：`axProperties={"invalid":"false","focusable":true,"expanded":false}` → `{...,"expanded":true}`，`hasExpandedProperty=true`

## 六、截图与哈希（脱敏，均在 gitignored 目录）

16 张 1440×900 PNG 存于 `oa4rust/test-results/r519-expanded-state/`，命名 `<label>-<collapsed|expanded>-<light|dark>-r519.png`。代表性截图（展开态，视觉可见"收起"按钮与展开的创建表单）：

| 截图 | SHA-256 |
| --- | --- |
| `oa4rust/test-results/r519-expanded-state/pc-unit-expanded-light-r519.png` | 547bff40596d80832e742ba9d9e09bb446da6b3ca2180c7ce2313edf4f7673a2 |
| `oa4rust/test-results/r519-expanded-state/pc-unit-collapsed-light-r519.png` | 48753f352f50fff3a3eb1e98e2ec7266303bccd2321d598b2f76cdcffed37b6b |
| `oa4rust/test-results/r519-expanded-state/pc-group-expanded-light-r519.png` | 8366b972a7b300c43fab5e4d88fb68143e4c04e80542ec39e26edabf408c3ef9 |
| `oa4rust/test-results/r519-expanded-state/pc-group-collapsed-light-r519.png` | e6660e9702b990795f1c5a852f7505011642073e3c429ec7ce6b63727dfb1e7b |
| `oa4rust/test-results/r519-expanded-state/pos-attendance-expanded-light-r519.png` | a3a7a2d3b8ffa8fd92cef6a6af12dbb30d5387f6db610bcdc8b401677d9e4bfe |
| `oa4rust/test-results/r519-expanded-state/pos-calendar-expanded-light-r519.png` | afd1a95202bbc398ae6ae5c54829042feaa42e1ff492a8fa1b7f4f81cc14867c |

dark 主题对应截图 `*-dark-r519.png` 完整列于 `report.json`（全部文件与 sha 一致可读，报告整体 SHA-256 见运行日志）。日志界面按钮文案变化与面板显隐均肉眼可核对，CDP 属性缺失与对照齐备形成闭环。

## 七、覆盖甄别与转介

- 次强候选 `SearchApp.vue:22-59` summary[role=tab] 缺 `aria-selected`：去伪风险较高（自定义 tab 的 role/semantics 需另行甄别），本轮仅备料不主判，不纳入结论。
- 只读边界：本轮未修改任何源码，未改动 `OrgDirectoryApp.vue` / `OrgViewer.vue`；缺陷修复建议（在两按钮补 `:aria-expanded` 绑定）交队长另行排期，不属本甄别交付。
- 交付边界：唯一入库文档为本文件；runner 脚本、日志、报告、截图全部位于 gitignored 的 `oa4rust/test-results/r519-*`，精确 `git add` 本文档，禁 `-A` / `git add .`；push 若失败 FAIL-LOUD 报告队长补推。
