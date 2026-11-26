# R573 laiyipao/miniapp 专精页 3 类裸 @click 伪按钮键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。mastery.vue 重铸／家族 tab／专精节点 3 类控件键盘完全不可达、辅助技术语义 generic/name=null。属与已修 battles/rank/stage/enemies/index/signin 六页零重叠的新实例，不因同 SC 维度已覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：laiyipao/miniapp/src/pages/mastery/mastery.vue（`164f9a1049909f040dd3cf494e61ae221d1f052300a68409603bd7b9a8d24428`，545 行）。
- 3 类锚点（全无 role/tabindex/aria/@keydown，全文件仅此 3 处 @click）：:26-33 text.link @click=resetMastery（重铸，:class link-off 仅禁用外观无 disabled 语义）；:42-52 view.tab-chip v-for families @click="curFamily=f.family"（家族 tab 内联赋值纯前端）；:106-124 view.node v-for nodesInLayer @click=toggle(n)（专精节点）。
- 全页 grep 无原生 button、无 @confirm/@longpress、无 role/tabindex/@keydown/aria-label 等价入口；scroll-view 的 scroll-x 仅可键盘滚动、不等于触发 tab。
- 入口：index/index.vue:403 专精 cell（playMenus 第 4 项）一跳 navigateTo，pages.json:37 注册；onMounted(:354-357) 未登录先 store.login() 自动登录，探针预置令牌跳过 guestLogin。

## 可达性分层（非死代码确认，据实不夸大）
- :47 家族 tab 为决定性样本：零业务造数，仅依赖全局 store.config.mastery_families（/config 启动下发非页面级造数）、纯前端零写请求；三重可程序化反馈 = .tab-chip.active class 移动 + curName 专精树标题变化 + curNodes 节点集切换。
- :29 重铸需 resetTokens>0 且 selected.length>0，有 POST /mastery/reset 写请求；:115 节点需 points>0 且未锁定，有 POST /mastery/allocate 写请求。二者均造数分层据实披露，严禁夸大为全部零造数。
- 空态下 reset 呈 class=link link-off，AX 仍 generic、focus 后仍 BODY，证 link-off 无 disabled 语义仍需 Tab 聚焦，非合法禁用态。

## 方法
- 自起 pnpm dev:h5（默认端口被他组残留连接占用，Vite 自动切 <PORT>，真值不入库；报告 base 写 `<TEST_HOST>:<PORT>`）；Playwright chromium headless 390x844 DPR2 isMobile/hasTouch；stub 仅拦 **/api/v1/**，造 config(mastery_families 两系各 3 层×4 节点)/mastery/wallet/auth。
- 决定性判据：DOM.describeNode 取 backendNodeId → getPartialAXTree({backendNodeId}) + getFullAXTree 按 backendDOMNodeId 映射读 role/name；DOM.focus 后读 document.activeElement；过滤 getFullAXTree focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训），UNI-VIEW/UNI-TEXT 身份独立确认。
- 6 组：light/dark × withData/empty + func + mouse；4 组静态 fatals=0、pageErrors=0，func2/pos 补探针 fatals=0 pe=0。

## 决定性证据
- 原始 DOM：3 类 role/tabindex/aria 全 null；partial+full AX role=generic name=null ignored=false；DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false、axFocused=[]。
- 干净 Tab 30 次：4 组 hits{reset,tab0,tab1,node0,node1} 全 0，distinct 仅 [BODY:]，候选 0 命中。
- ★注入对照（决定性去伪）：注入 role=button + tabindex=0 + aria-label + keydown 后 5 类 AX=button name 可读（node4/reset 另由 func2 坐实 AX=button）；Tab 命中各 2 次，distinct 含 UNI-TEXT:reset、UNI-VIEW:tab0/tab1/node0..node11（身份独立确认非 wrapper）。
- ★功能真命中（非仅焦点移动）：家族 tab Enter 致专精树标题 焰系→霜系、tabClasses active 转移（节点集随之切换）；Space 致 霜系→焰系 恢复；func2 中 node4 Enter 致 writes[allocate:104] + 第 3 格 class 由 node selectable→node on locked；reset Enter 致 writes[allocate:104,reset] + 节点全清为 node selectable、resetClass 转 link link-off。
- 鼠标对照非死代码：点 tab1→霜系专精树、tabClasses active 转移；点 reset→writes[reset]、节点全清。
- 正面对照：battles.vue 已修 role=button/tabindex=0/aria-label 控件 count=6、Tab 命中 26、AX role=button name 可读，证 AX 通道正常、检测器有效。

## Fail Loud（据实披露探针缺陷，均已修复）
主探针 func case 曾 1 fatal：家族 tab Enter 切系后 v-for 重建致后续 node 的 data-r573 标记丢失（focus null）；非页面缺陷，func2 探针在每步操作前重跑 mark+inject 覆盖，最终 func2 fatals=0 pe=0，node/reset Enter 功能真命中全部坐实。4 组静态与主 mouse case 均 fatals=0 pe=0。

## 最小修复方向
3 类逐一显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent，复用原 handler（resetMastery／curFamily=f.family／toggle(n)），业务零改；家族 tab 可选 role=tab + aria-selected（容器 role=tablist）语义更贴切；link-off 禁用态应补 aria-disabled=true 而非仅 class。沿用 R561–R572 范式。

## 证据（gitignored：oa4rust/test-results/r573/，借用已忽略目录存放 laiyipao 证据，据实披露）
runner sha = crypto.sha256(JSON.stringify(report)) 紧凑串；文件 sha256sum = 美化写盘 JSON 的字节哈希，两者分列不可混用。
| 项 | runner 紧凑 JSON sha | 文件 sha256sum |
| --- | --- | --- |
| 主报告 r573-report.json（6 组） | cf8d6154cdcc82944c3c75e0d26dfa0267fb30eda8bc6840ba0cbd89cc597f37 | 681af0c4fd78b6503e39392980ec800ccfca832b164b4ad7465ed3d9596829fd |
| 功能补探针 r573-func2-report.json | 4d5c9dd17fea2e14e7490f10328dfad9c038a279330261a08e80340886b1638b | 6bdb6bfb98bf272a899f41f14843d0f57977d713c60cc55b616639a1866a872e |
| 正面对照 r573-pos-report.json | 53a5cd87c430fa4e1a44e81c2c95a625e4d7a2336a42a72713d6c31e7ea6cc83 | 2b4524de822205a2f233a43b1a904b188e8755406f367deceaf53365e07d02df |
| 截图 r573-withData-light.png | — | 6a8317483241460240f6dc43381b092618425e1de1387612f8362c52fac70620 |
| 截图 r573-withData-dark.png | — | a86f45c4b9631d91a4a84c519d52699641e0e0a0f9b613de8c474166f1d7eb12 |
| 截图 r573-empty-light/dark.png（同 sha） | — | 132f482600e4f3d4eab8921d34e1e640614f581f88b13df824d2cb675e0789bf |
| 源码 mastery.vue | — | 164f9a1049909f040dd3cf494e61ae221d1f052300a68409603bd7b9a8d24428 |
| 源码 index.vue | — | 91d0052edd24f86bd6e6e88387215ddeb2824828367f5e6b316d7de72b8f9a36 |
| 源码 api/client.ts | — | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 基线 HEAD | — | 05e527b541ed155dfbc014580d4f82a3ced82da0 |

截图 read_image 目视有效：withData-light（专精点 5 + 重铸道具 2/重铸 + 焰系/霜系 tab + 专精投入总览 + 全系结构 + 焰系专精树 3 层×4 节点，含已点亮勾选与可选 + 脉冲）；empty-dark（专精点 3 + 重铸道具 0/重铸置灰 + 全 0/12 + 节点全可选）。miniapp H5 无主题切换，empty light/dark 文件同 sha 据实披露。