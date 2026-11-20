# R301 t12 框架层实质升级：命令面板焦点陷阱

## 变更

- t11 首轮设计评审指出外壳/导航应做实质升级而非换色；r303 评审要求后续轮次按结论续作。
- 复核现有 B1 命令面板：role=dialog + aria-modal=true 已声明，但没有 focus trap，键盘用户按 Tab 可进入被遮挡背景页，且关闭后焦点不归还，属于 aria-modal 语义与实现不一致的真实无障碍缺陷。
- 本轮以 test-first 落地：新增 AppShell.focusTrap.contract.test.ts，在 jsdom 真实 DOM 上先验证 RED（嵌套/空面板失败、AppShell 集成缺失），再补齐 GREEN。
- 新增 packages/ui/src/components/focusTrap.ts：实时查询可聚焦元素，Tab/Shift+Tab 在面板内循环，关闭后归还触发者；空面板自动补 tabindex=-1 接住焦点。
- AppShell.vue 集成：面板根节点绑定 ref，打开时安装陷阱，Esc/命令导航/关闭时 release()，组件卸载时释放；保留全部原导航/搜索/键盘契约锚点。

## 验证

- 契约：11 个文件 60 条测试全通过（含 keyboardA11y / layoutTokens / navIntegrity / offsetGeometry / aiShell / focusTrap / globalSearchShortcut / shortcutMarker / searchFieldA11y / aiDesignTokens）。
- 类型检查：desktop 与 ui 包 tsc --noEmit 均通过。
- 构建：desktop 与 ui 包构建成功。
- 部署：按既有原子替换流程部署到测试服 <YOUR_TEST_DOMAIN> 对应静态目录，远端 index.html 哈希与本地构建一致。
- 真浏览器：Playwright Chromium headless 对部署测试服真实 UI 登录并进入命令面板，断言：
  - 打开后焦点在面板内输入框；
  - 首项 Shift+Tab 循环到末项，末项 Tab 循环回首项，均未逃逸到背景按钮；
  - Esc 关闭后焦点归还命令面板触发按钮；
  - 重新打开并输入“通讯录”回车，真实跳转到 /app/directory，命令导航业务未破坏。
- 证据目录（gitignored）：oa4rust/test-results/r301-t12-focus-trap/，含 3 张 PNG 与 evidence.json。截图来源：Playwright Chromium headless 对测试服真实页面，非物理真机。
- 口令/测试域名/SSH 真值均运行时从环境或测试服文件读取，未写入本仓库。
