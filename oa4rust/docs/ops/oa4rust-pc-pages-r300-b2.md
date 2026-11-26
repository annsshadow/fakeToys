# R300 B2 工作台与个人域 AI 风格实测记录

## 范围
- 修改 PC 四个高频页面，基于 t1 的 `--ai-*` 双主题令牌落地，保持业务接口、流程与既有 testid 不变：
  - 工作台 `frontend/apps/desktop/src/views/Dashboard.vue`：欢迎栏改为玻璃 AI 面板与渐变表面，统计栅格使用 `--ai-grid-4`，加载中/失败不再被呈现为零值，待办空态给出“去流程中心发起”的动作入口，待办元信息不再出现原始占位“未知应用”，骨架屏用 shimmer。
  - 个人中心 `frontend/apps/desktop/src/views/Personal.vue`：标题区改为品牌渐变 Hero，头像与主操作使用品牌渐变、spring 动效，副标题改为“完善资料，让协作更顺畅”。
  - 设置 `frontend/apps/desktop/src/views/Settings.vue`：分组布局使用 AI 栅格间距，保存按钮使用品牌渐变；可见区域移除调试式播报，改为 `sr-only` + `role="status"` + `aria-live="polite"` 的无障碍播报座。
  - 消息中心 `frontend/apps/desktop/src/views/IMChat.vue`：双栏使用 AI 栅格与玻璃背景，会话头像/输入器采用品牌渐变/面板令牌，空会话给出引导，输入中样式已定义。
- 新增契约 `frontend/apps/desktop/src/views/Pages.aiStyle.B2.contract.test.ts`（6 项，仅读源码）。首次运行 6 项失败（RED）；其中两项引用了规范中不存在的变量名，按 t1 实际令牌表改为 `--ai-cmd-radius` / `--ai-gradient-surface` 后，6 项通过（GREEN），未臆造新令牌。

## 验证与证据
- 契约测试：`pnpm --filter @oa4rust/desktop exec vitest run src/views/Pages.aiStyle.B2.contract.test.ts --reporter=dot`，6/6 通过。
- 构建：`pnpm --filter @oa4rust/desktop build` 通过，产物入口 `index-CfiLC8vO.js` / `index-CcsYol9C.css`。
- 部署：构建静态产物部署到测试服 PC 静态目录；真域名、SSH 别名与凭据只从本地环境读取，仓库与提交信息不写真值。部署后远端 `index.html` 资源哈希核对与本次构建一致。
- 真浏览器实测：Playwright 无头 Chromium 对已部署测试服执行真实登录后逐页访问工作台、个人中心、设置、消息中心，采集 DOM 指标并 fullPage 截图；非物理真机、非静态截图。截图与机器证据位于本地 `test-results/r300-b2-pages/`（gitignored）：`01-dashboard.png`、`02-personal.png`、`03-settings.png`、`04-im.png`，均为 1440×900；`evidence.json` 的 source 标注为 “Playwright Chromium headless against deployed test server; not a physical device”。
- 关键 DOM 断言（浏览器实测）：
  - 工作台：欢迎栏 class 含 `ai-panel`，统计卡 4 张，统计空值占位 0 个，背景渐变令牌生效。
  - 个人中心：标题区 class 含 `ai-panel`，头像与保存按钮为品牌渐变，副标题文案更新可见。
  - 设置：布局 class 为 `settings-layout ai-grid-2`，可见布局不再带调试隐藏属性，保存按钮品牌渐变；可见区域只出现设置文本，不出现旧调试式播报。
  - 消息中心：真实存在 11 个会话，根布局 class 为 `im-view ai-grid-2`，侧栏玻璃背景生效，输入框 class 含 `ai-panel`，首会话空态引导可见。
- 实测缺口：消息中心的输入中动画样式在源码中已定义，但本轮首会话不处于“对方正在输入”状态，浏览器 CSSOM 未检出该规则命中；如实记录为“样式已定义、本轮未触发”，不伪报为缺陷。

## 注记
- 测试服地址统一用占位符 `<YOUR_TEST_DOMAIN>`；真域名/IP/SSH 端口不入库，凭据只从 `.local.keys`/环境变量读取。
- 截图来源如实标注：Playwright Chromium headless + 测试服真实页面，不是物理真机。
- 未改动其他视图及 B3/B4 队友并行文件；项目持续 ≥1000 轮迭代，本轮为其中一环，非完成态。
