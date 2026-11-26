# R300 B6 H5 视觉语言首轮对齐

## 目标与范围
移动端 19 个路由页面 + App.vue 全局样式（共 20 个视觉单元）接入桌面 R300 AI 令牌，统一配色、8px 栅格、组件与微交互；不改动业务接口、状态流转与既有 --color-* 语义色合同。

## 变更摘要
- App.vue page{} 引入完整 --ai-* light 令牌（与桌面 ai-tokens.css 同值），新增 .ai-brand-fill（靛蓝到青渐变）、.ai-panel（克制玻璃）、.ai-gradient-text、ai-shimmer 骨架屏等全局类。
- 19 个页面根节点统一挂载 .ai-surface-gradient；首页统计卡、登录主按钮挂 .ai-brand-fill。
- 针对 uni-app H5 的 scoped 规则特异性问题，渐变锚点使用 uni-page-body .ai-brand-fill.ai-brand-fill / body 变体（特异性 (0,2,1)），在真浏览器 DOM computed style 中确认渐变生效。
- 新增 ai-visual.contract.test.ts 契约，覆盖 20 页面锚点与渐变优先级；test-first RED→GREEN。

## 验证
- 契约与全量测试：61 files / 393 tests passed。
- H5 构建成功，产物 CSS 含 AI 令牌与渐变规则。
- 测试服部署后真实 Chromium 登录 + 19 路由逐页截图与审计：BAD=0，API_FAILURES=0；统计卡 DOM computed backgroundImage 为 AI 渐变。
- 证据目录：oa4rust/test-results/r300-h5-visual/（gitignored，含 audit.json/manifest.md 与逐页截图；截图来源为真浏览器渲染真实测试服页面，非物理真机）。

## 约束
本轮为 1000 轮持续迭代中的一环，非终态；后续按设计评审结论继续优化。