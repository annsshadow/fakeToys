# R447 PC R300 B5 契约 6 页 scoped 根容器渐变令牌词序修复

- 轮次：R447（任务 t299）｜角色：page-core 实现修复
- 分支：opt1000/oa4rust-pc-ai-refactor｜真父提交：2b5a12cf3164cff8dd1d3ac437533305916a7575
- 来源：R446 甄别文档 oa4rust/docs/ops/oa4rust-r446-pc-r300-b5-consistency-audit.md（94 行）三向坐实的真实样式缺陷
- 测试入口域名：`<YOUR_TEST_DOMAIN>`（占位，真值不入库）｜测试主机：`<TEST_HOST>`（占位）
- 截图来源标注：Playwright Chromium headless 1440x900 → 测试服真实页面（NOT a physical device，非物理真机）

## 1. 缺陷与修复

R300 B5 契约 6 页的 scoped 样式块根容器统一误写 `background: var(--ai-surface-gradient);`；令牌文件 oa4rust/frontend/packages/ui/src/theme/ai-tokens.css 全文只有 `--ai-gradient-surface`（词序相反），浏览器丢弃无效声明 → 根容器 `backgroundImage=none`。R446 真浏览器 deep2.json 坐实错误令牌解析为空、正确令牌为 `linear-gradient(160deg, #4f46e50f 0%, #0891b205 100%)`。

本轮仅把 6 页 scoped 块该 1 行改为 `background: var(--ai-gradient-surface);`：

| 文件 | 行号 |
| --- | --- |
| oa4rust/frontend/apps/desktop/src/views/RoleManager.vue | 236 |
| oa4rust/frontend/apps/desktop/src/views/ThreeMemberApp.vue | 251 |
| oa4rust/frontend/apps/desktop/src/views/CmsDictDesignerApp.vue | 221 |
| oa4rust/frontend/apps/desktop/src/views/ControlPanelApp.vue | 123 |
| oa4rust/frontend/apps/desktop/src/views/ConfigDesignerApp.vue | 508 |
| oa4rust/frontend/apps/desktop/src/views/LogViewerApp.vue | 118 |

严格不追加 modern-page/ai-panel 三标记（R446 判定这 6 页是 R300 独立自洽 B5 体系，追加会与 scoped 高特异选择器+外部 ui 语义冲突、体系重复）。业务结构/接口/data-testid/端点/payload/method/后端/SDK/main.ts/adminNav.ts/R300 契约/其它 view/构建配置零改；git diff 仅 6 文件各 1+/1-。

## 2. 契约 test-first

新增 oa4rust/frontend/apps/desktop/src/views/R447.b5Token.contract.test.ts（18 it：6 页 × 3 条真实静态断言：正确令牌存在、错误令牌全文件消失、R300 体系保持含原基类+ai-page-b5+data-testid+scoped 边框阴影且无 ai-panel）。

- 真父 2b5a12cf3（源码未修）跑新契约：RED 12 failed / 6 passed（6 页正确令牌断言全失败 + 6 页错误令牌消失断言全失败；R300 保持断言 6/6 通过立即排除桩错误）。
- 修复版：GREEN 18/18。
- 调试踩坑：契约中 scoped 块断言须检查 `source` 全文而非 `block` 截取体（选择器在高特异规则头部，不在花括号体内）；基类断言须正则容忍 RoleManager 的 `role-view modern-page ai-page-b5` 中间类。

## 3. 三 verify（全 exit 0）

```
cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/R447.b5Token.contract.test.ts  → 1 file 18/18 passed, exit 0
cd oa4rust && pnpm test                                                                    → 314 files / 3104 tests 0 failed, exit 0
cd oa4rust && pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit                → exit 0
```

构建入口 index-CrR-C6vC.js；部署脚本 oa4rust/test-results/deploy-pc-r447.sh 整目录 tar → scp → sha256 本地远端一致 e1945a7678ed18aec31ac1a34dc00d51b64fdb406e900656f895a06a825701cf → 远端 /usr/local/oa4rust 原子替换，DEPLOY_R447_PC_FULL_OK。

## 4. 真浏览器 6 路由实测（headless 1440x900，非物理真机）

脚本 oa4rust/test-results/r447-pc-b5-token-check.cjs：登录一次逐页 goto role / three-member / cms-dict-designer / control-panel / config-designer / log-viewer，读根容器 computedStyle 令牌与背景、标题/按钮保留、0 pageerror。终轮 R447_DONE：49 断言全 true / failed=[] / errors=[]。

修复前（R446 甄别 deep2.json）根容器 `backgroundImage=none`，令牌 `--ai-gradient-surface=linear-gradient(160deg, #4f46e50f 0%, #0891b205 100%)`；修复后 6 页实测 `backgroundImage=linear-gradient(160deg, rgba(79, 70, 229, 0.06) 0%, rgba(8, 145, 178, 0.02) 100%)`（同一令牌的浏览器规范化计算值），`--ai-gradient-surface` 非空、`--ai-surface-gradient` 空、边框/阴影/圆角/渐变标题保留、无 ai-panel 追加、0 pageerror。

## 5. 证据截图 sha256（命令行 sha256sum *.png 原始值）

```
77134fa8a0ea090109c4ceb5c90177a10ebf793a86ee1ed5773032cad319a373  cms-dict-designer.png
bef1b41df1a111486ec9a0da3f83498ddcfbad5948ba10f9ced3aeabdaa34384  config-designer.png
40671cdcba534b87b36799fae13f1fe7fa200dbb6e9cb01c991c5939b7609ea2  control-panel.png
7f0e365b922e25835c14160719873919099fb4ec122faa9c6378a251b088ab91  log-viewer.png
97033ee231fa8e7e00be4c49c40b1285506d7516fe7729513c5291327e5be5f5  role.png
e42aceffe8e5dcae5012ff8ac00008bebbb8c7dd8c122d2f8e82b746f57d8685  three-member.png
```

证据目录 oa4rust/test-results/r447-pc-b5-token-fix/（gitignored，6 png + report.json）。role/control-panel/log-viewer 三张 1440x900 截图已 read_image 目视确认：真 OA4RUST admin 页、渐变根容器背景生效、标题/按钮完整。

## 6. 真值纪律

- 文档仅含 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；口令/私钥/会话数据/真实域名与 IP 不入库，凭据经 ssh 运行时注入环境变量。
- 截图、部署脚本、E2E 脚本与日志均留 gitignored test-results/ 证据目录，来源标注非物理真机。

≥1000 轮持续迭代中一环，非终态。