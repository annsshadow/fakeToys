# R304 契约同步 D — UI 玻璃令牌与 H5 空态断言（用例 11/20）

## 依据与范围
- t41 契约漂移审计与 §219 裁决：两条断言与已迁移实现不一致，需语义等价同步，实现零改动。
- 仅改两个契约测试文件；未修改 AppShell.vue、任何 H5 页面实现或运行时代码。
- 测试服访问走 SSH 隧道/代理，真值从本机凭据与运行时读取；文档不写入真实域名/IP/口令，均以 <YOUR_TEST_DOMAIN> 占位。

## 用例 11：UI 玻璃表面令牌
- 旧断言：popupSurfaces.contract.test.ts 断言 .user-dropdown 含 `var(--bg-glass)`/`var(--glass-blur)`/`var(--glass-border)`。
- 真实现：AppShell.vue .user-dropdown 已使用 `--ai-glass-bg`/`--ai-glass-blur`/`--ai-glass-border`，并有 `--ai-shadow-2` 阴影，语义完整（背景+模糊+描边+阴影）。
- 新断言：仍精确断言三条 `var(--ai-glass-*)` 令牌存在，非空断言。

## 用例 20：H5 空态 class 修饰兼容
- 旧断言：design-system.contract.test.ts 用 `class="o2-empty"` 整串匹配五页。
- 真实现：message.vue/meeting.vue 等使用 `class="o2-empty empty-guide"` 修饰类组合。
- 新断言：正则 `/class="[^"]*\bo2-empty\b[^"]*"/` 以 class 分词匹配，仍保留 `o2-empty-icon` 图标断言，未弱化为空断言。

## 验证
- RED（改前）：两条定点 vitest 各 1 failed —— popupSurfaces 1 failed/8 passed；design-system 1 failed/6 passed。
- GREEN（改后）：`pnpm exec vitest run frontend/apps/desktop/src/views/popupSurfaces.contract.test.ts` → 9 passed；`pnpm exec vitest run src/design-system.contract.test.ts` → 7 passed。
- 实现零改动：git diff 仅两测试文件；AppShell.vue 与 oa4rust/mobile/src/pages/ 下文件均未改。
- 本轮仅同步断言/清单，无运行时代码变更 → 按裁决对已部署构建做真浏览器冒烟，不重建不原子部署。
- 真浏览器冒烟：headless Chromium 桌面 1440x900 与 H5 390x844 isMobile/hasTouch DPR2，运行时读凭据经真实 UI 登录；桌面展开用户下拉检查 `--ai-glass-*` 根令牌与实际背景/模糊/描边；H5 逐页（收藏/浏览/会议/回收站）检查空态图标+主文案与引导入口；截图与 evidence.json 落 gitignored 目录 test-results/r304-contract-sync-smoke/。
- 截图来源声明：真浏览器渲染真实测试服页面，**本地渲染截图，非物理真机**，非静态/假数据拼图。
- 文档脱敏：真实 host/域名仅存在于 gitignored 脚本与证据，不入库；提交信息仅引用脱敏证据路径。

## 迭代声明
本轮仅为 ≥1000 轮持续迭代中的一环（R304 契约同步 D），非项目完成态；后续按独立设计评审结论续作。
