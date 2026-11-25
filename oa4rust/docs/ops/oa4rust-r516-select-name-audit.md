# R516 WCAG 2.1 SC 4.1.2 选择控件可访问名称甄别（ConfigDesignerApp 双 select）

## 1. 依据与判据

- 判据：WCAG 2.1 SC 4.1.2 Name, Role, Value——所有用户界面组件必须有可由程序化确定的名称/角色/值。本轮只读甄别，不改码。
- 核心候选：ConfigDesignerApp.vue:21 配置格式 select（JSON/YAML/Properties）与 :26 主题 select（暗色/亮色主题），位于 div.toolbar-left。
- 源码静态：两处 select 均无 id / for / aria-label / aria-labelledby / title；全文件无 label；editorLang/editorTheme 仅 ref 默认值（:190-191）。
- 反预判规则：HTML-AAM 下 select 可访问名不从 option 文本推导，但须以真浏览器 CDP AXTree 实测为准，严禁仅凭源码预判。
- 三分：可达真缺陷 / 不可达 L1 / 正常规范预期，逐项给 DOM+AXTree+截图实证；0 缺陷据实判 0。

## 2. 方法与自洽

- 环境：Playwright Chromium headless 1440x900 DPR1，light/dark 双主题（localStorage oa4rust_theme + data-theme，各等待 1500ms 避过渡），非物理真机。
- 测试服：正式测试站点（域名/主机见第 8 节占位），真浏览器走真实登录流程（凭据运行时从测试服凭据文件读取，仅存在于 gitignored runner，不入仓、不回显）。
- 工具：DOM 属性 + Accessibility.getPartialAXTree（axRole/axName/axDescription/axIgnored/nameSources/props.focusable）+ Tab 焦点顺序 + 注入 aria-label 对照 + 截图与 sha256。
- 0 pageerror / 0 fatal（R516_DONE groups=2 fatals=0 errs=0）。
- 入口浅：路由 /app/admin/config-designer 直达（main.ts:428-430，:491 重定向兼容），无需造数；次选回收站复选框需数据，据实另判。

## 3. 逐项程序化证据（双主题一致）

| 项 | 选择器 | DOM（id/aria-label/aria-labelledby/title/for） | AXTree（role/name/ignored/focusable） | 结论 |
| --- | --- | --- | --- | --- |
| 配置格式 select | select.tb-select[0] | 全空；labelsFor=[]；value=json；可见 87x22 | combobox / axName=空字符串 / ignored=false / focusable=true / hasPopup=menu | Name 缺失 |
| 主题 select | select.tb-select[1] | 全空；labelsFor=[]；value=dark；可见 80x22 | combobox / axName=空字符串 / ignored=false / focusable=true / hasPopup=menu | Name 缺失 |
| 正面对照 时区 | select#cfgTz | id=cfgTz；label for=cfgTz 文本=时区；labelsFor=[时区] | combobox / axName=时区 / ignored=false / focusable=true | 正常 |
| 次选 回收站复选框 | input[type=checkbox] | 当前回收站为空，checkbox 数=0，CDP found=false | 无节点 | 数据不可达，不判 |

- 两个 select 的 axName 实测为空字符串（非 UA 默认、非空名），nameSources 为空来源；ignored=false 且 focusable=true，说明控件进入无障碍树且可聚焦，仅缺 Name。
- Tab 序列：focusTag=select.tb-select，按 Tab 后 activeElement 仍为 class=tb-select 的 select（两个 class 相同无法从 className 区分序号，但坐实两 select 均入键盘焦点序列，与 focusable=true 一致）；该记录仅作可达性佐证，不作名称判定依据。
- 截图：pc-config-selector-light/dark-r516.png 目视配置设计器工具栏显示「JSON」「暗色主题」两个下拉，无可见 label 文字；pc-settings-cfgtz-light/dark-r516.png 目视系统设置基础配置「时区」标签紧邻 Asia/Shanghai 下拉，名实相符。

## 4. 注入对照与全局增强器排查

- 注入对照：对第一个 select 动态设置 aria-label=配置格式 后，CDP 同一节点 axName 立即由空字符串变为「配置格式」，证明该控件名称可由程序化属性驱动，缺陷在于作者未提供，而非控件不可命名。
- labelA11y.ts（main.ts:618 安装）：仅为孤儿 label 与其后紧随 labelable 控件建立 id/htmlFor，ConfigDesigner 两个 select 无相邻 label，增强器不会挂载，实测 labelsFor=[] 印证。
- searchFieldA11y.ts（main.ts:623 安装）：选择器仅含 input.search-input/.sb-input 等输入类，不含 select，实测无 aria-label 印证。
- 相邻 toolbar-right 的 span.tb-info（行数/字符数）与两 select 无 for/id/aria-labelledby 程序化关联，不构成名称；相邻关系不作为可访问名来源（HTML-AAM），实测 axName 空印证。

## 5. 三分判定

- 可达真缺陷 1 项（P2）：ConfigDesignerApp.vue:21 与 :26 两个 .tb-select 在 light/dark 双主题真浏览器中 computed name 均为空（axName=空字符串、role=combobox 正确、focusable=true、ignored=false），违反 SC4.1.2 的 Name 要求；选项文本（JSON/YAML/Properties、暗色/亮色主题）在 HTML-AAM 下不提供可访问名。非阻断（不影响鼠标可视操作与角色/值），故判 P2。
- 不可达 L1 1 类：RecycleApp.vue 回收站复选框——当前回收站为空（显示 0/0 项、回收站为空提示），checkbox 节点数=0，无 AX 节点可测，数据不可达，据实判 L1，不预判其名称缺陷。
- 正常规范预期 3 类：（1）Settings.vue:28-29 时区 select 显式 for/id，axName=时区，正面对照通过；（2）两 select 的 Role=combobox、focusable=true、hasPopup=menu 均正确，Value=当前选中项可读，仅 Name 缺失不夸大至 Role/Value；（3）全局增强器边界清晰，不误挂无 label 的 select，也未把相邻 span 当名称。
- 反证边界：本轮仅抽测 ConfigDesignerApp 两个 select、Settings 对照与 RecycleApp 次选，不据此推断全库其他 select。

## 6. 修复建议（供后续修复轮，本轮不改码）

- 最小修复：给两个 select 各加一行 aria-label，例如 :21 加 aria-label=配置格式、:26 加 aria-label=编辑器主题；或各配一个可见 label 并用 for/id 关联。
- 回归验证：真浏览器 CDP AXTree 两节点 axName 分别变为「配置格式」「编辑器主题」且 role/focusable 不变；双主题均通过；不引入 option 文本当名称的误判。

## 7. 证据 sha256（命令行核验一致）

| 文件 | sha256 |
| --- | --- |
| test-results/r516-select-name/report.json | 3e44861acdf6a10b03863cd1436000253d30b63af57b577717e08ae3ea106da4 |
| pc-config-selector-light-r516.png | 6abc4bbd4ace253dc56f7ce4736960abc1b152006a2840f62a0c54722e0198b7 |
| pc-config-selector-dark-r516.png | 7fb00d29f1f666b6e1c6443ac0d8dd0c33b6d2954d5f241230ba35991acec060 |
| pc-settings-cfgtz-light-r516.png | 582a93adfc297717a3203beee129ff4267fef358ba2798028addb27a88d9eb5a |
| pc-settings-cfgtz-dark-r516.png | 60ae08c41a77fec982795d98241478d20acc2eee1b01198976ecbe3b16fd7b48 |
| frontend/apps/desktop/src/views/ConfigDesignerApp.vue | b09c8526b88e5d65e290e885a2aae906acbaa1222d0d297b0395d7cb07a82394 |
| frontend/apps/desktop/src/views/Settings.vue | 3481ee0f0c5ab490b96cb0e2a33d4667dfa010f2ca5c69d5c5be1949f013ecc8 |
| frontend/apps/desktop/src/utils/labelA11y.ts | 0885bb1f2a91b12758431adbfea72387ecf32b9a073d19ba39fa8cdd8302f0ab |
| frontend/apps/desktop/src/utils/searchFieldA11y.ts | d0306c04c8b38e354225ea4c8660952447edb9b07ad7b980eac2850790dced16 |
| frontend/apps/desktop/src/main.ts | e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0 |

## 8. 脱敏边界与结论

- 真域名/主机一律占位：测试服主域 <YOUR_TEST_DOMAIN>、测试服主机 <TEST_HOST>；凭据与 runner 真值仅存在于 gitignored 本地文件，不入库、不回显。
- 截图均标注来源为 Playwright Chromium headless 非物理真机，不冒充真机。
- 结论：可达真缺陷 1 项（P2，两个 .tb-select 可访问名缺失）；不可达 L1 1 类（回收站复选框无数据）；正常 3 类；本轮只读甄别，不改码，交后续修复轮处理。