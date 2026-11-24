# R478 PC 纯图标按钮无障碍名称修复（WCAG 4.1.2）

## 1. 缺陷与根因
- 依据：t345/t346 R477 甄别（commit 6dce8aa32），真浏览器 PC 1440x900 DPR1 非物理真机，10 页 221 按钮 CDP Accessibility.getPartialAXTree 实测：174 合法名 / 46 符号名 / 1 空名。
- 根因：W3C 可访问名称计算中 name-from-content 优先于 title。31 个按钮已写中文 title（如 编辑/删除/撤销），但符号内容（✏/🗑/↩）遮蔽了 title，AX name 仍是符号；另 16 个无 title/aria 或数据驱动空名。
- 影响：屏幕阅读器用户无法可靠获知按钮语义（WCAG 4.1.2 Name, Role, Value，A 级），定 P2；不阻断主流视觉用户，低成本可修复。

## 2. 修复方案（业务零改，仅新增 aria-label）
- ProcessDesigner.vue 第 12/13/20 行：撤销↩/重做↪/清空🗑 三个按钮补 aria-label="撤销/重做/清空"，title、符号、@click 原样保留。
- FormApp.vue 第 33 行：删除表单按钮补 aria-label="删除表单"，class="btn-sm btn-del"、@click.stop、🗑 原样保留。
- ConfigDesignerApp.vue 第 66/67/130 行：编辑✏/删除🗑/历史删除🗑 分别补 aria-label="编辑/删除/删除历史记录"。
- FileManager.vue 第 64/85 行：列表删除补 aria-label="删除文件"，可清理文件删除补 aria-label="删除可清理文件"，title、:disabled、@click 原样保留。
- CalendarApp.vue 第 35-43 行 cal-chip 数据驱动空名：新增 :aria-label="c.name || '选择日历'" fallback，不影响正常日历项文本显示；测试数据中 name 为空的项现在有中文可访问名。
- 硬纪律：只新增 aria-label 属性，title / 符号 / @click / data-testid / class / 样式零改；不靠提升 title 或改符号绕过。

## 3. 契约 RED→GREEN
- 新契约 R478.iconButtonA11y.contract.test.ts（5 it）：readFileSync 读 5 个 vue，按唯一 token 定位按钮开标签，断言 aria-label 到位 + title/@click/class/符号保留 + Calendar fallback。
- 真父 137721a12 定点 RED：5 failed / 0 passed，失败全为真实断言（无 ReferenceError/TypeError 桩错）。
- 修复版 GREEN：5 passed / 5。

## 4. 三 verify（全 exit 0）
- V1 定点：cd oa4rust && npx vitest run frontend/apps/desktop/src/views/R478.iconButtonA11y.contract.test.ts → 5 passed / 5。
- V2 全量：cd oa4rust && pnpm test → 322 files / 3158 tests / 0 failed。
- V3 类型：cd oa4rust && pnpm typecheck → 6 项目全 Done（含 frontend/apps/desktop）。

## 5. 真浏览器 E2E（1440x900 DPR1 headless 非物理真机）
- 方法：Playwright Chromium + CDP DOM.getDocument/depth:-1 + DOM.querySelectorAll(button) + Accessibility.getPartialAXTree，逐按钮取 name/role/ignored；复用单 CDP session 避免页面导航后失效。
- 路由实测校准：admin 前缀页（process-designer/form-designer/form/config-designer）与 app 普通页（im/file/bbs/calendar/process/recycle）混用，均经 r478-route-probe 实测确认；错误路由会 302 到登录页导致按钮为 0，已修正。
- 铁律①补数据：file 页可清理文件列表初始为空、删除按钮不渲染；E2E 走真实 UI 流程点 新建可清理文件 + prompt 输入名称经真实 API 创建，使该按钮出现后再验证（real-ui-created-recyclable-file 断言通过）。
- 终轮 R478_DONE：pages=10，checks=44，passed=44，failed=0，named=222，symbol=0，empty=0，shots=10，errors=0（0 pageerror）。
- named=222 构成据实：R477 基线 174 + 本次修复 47 = 221，另 +1 为 E2E 真实 UI 新建的可清理文件所增加的删除按钮；符号名/空名均归零，与 174 正常按钮零回归一致。

## 6. 截图 sha256 清单（命令行 sha256sum 原始值，与 report.json 一致；10 张均 1440x900）
- pc-process-designer.png：fe56c115230a76401cf394441e7ed47b523ebd60e0689a88f27f5031fb018f7b
- pc-form-designer.png：c1586ab9feff86ffddfbf5c4c52502d897b00fc4c7ab0a64e76122c0e6b0011c
- pc-form.png：577b73c3de9a05e9ab28d20d1551e3de8af93c0f607d2b23c904997db8c24f80
- pc-config-designer.png：f25ba78690c613ed116c080c69628daa4173da44f4e07b6400e5a32fdf58851e
- pc-im.png：4305f68b1b168138bead0dc7d973c0af31aed7bcb79ebe6e8b338c1a6cbdc24a
- pc-file.png：8b35c52c3adc1212f40d62f2ae9a9f1e366cb37a843a9ea6132c971c0581d2a2
- pc-bbs.png：888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105
- pc-calendar.png：8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad
- pc-process.png：1cdfa352dc73baedcc374b29e31413d3ca4c4c876b068c3af0dd2ebb53762a72
- pc-recycle.png：8456a5267ac759247176081ae925fc0a4248fe6fc32637152a1f43d0e2475472
- 证据目录（gitignored）：oa4rust/test-results/r478-icon-button-a11y/（report.json + 10 PNG）。
- 目视核验：pc-file.png / pc-process-designer.png / pc-calendar.png 三张经 read_image 确认为真 OA4RUST admin 页，1440x900，业务功能完整，非物理真机（模拟浏览器）。

## 7. 真值纪律与提交
- 测试服域名与主机用占位符 <YOUR_TEST_DOMAIN> / <TEST_HOST>；真值仅存在 gitignored runner 的环境变量与远端凭据文件，未入库。
- 截图来源：Playwright Chromium headless 模拟浏览器（非物理真机），如实标注。
- 精确写域 7 文件：5 个 vue + 1 契约 + 本文档；业务/接口/data-testid/后端/SDK/H5/其它 view 零改。
- 单提交：精确 git add（带 oa4rust/ 前缀，禁 -A/点）；push 失败仅单次即止、fail-loud 报 SHA，请队长补推。
