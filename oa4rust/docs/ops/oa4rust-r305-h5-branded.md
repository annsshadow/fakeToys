# R305 H5 品牌渐变实心与骨架屏对齐（t51）

## 背景
R305 视觉基线（t50）显示 19 页中仅 index/login 有 `.ai-brand-fill`，骨架屏检测为 0/19。经取证，骨架屏 class 口径 `[class*=skeleton]` 已同时覆盖旧 `.o2-skeleton` 与新 `.ai-skeleton-list/.ai-skeleton-line`，0/19 实为采样太晚（真实 API 已返回、`v-if` 关闭），因此不改骨架 class 名（避免破坏 t17 结构化骨架契约）。

## 变更
- 17 个 previously branded=0 页面：每页至少挂载一个 `.ai-brand-fill` 品牌渐变锚点（页签、提交按钮、空态引导、会话横幅、发帖入口等），复用全局 `.ai-brand-fill{background:var(--ai-gradient-brand)}`（t9 高特异性规则，scoped `.tab.on` 选中底色仍生效）。
- 条件渲染锚点（message 离线横幅、doc 附件空态、bbs-topic 无权限发布入口）：真实数据下可能不显示，由源码契约断言挂载。
- 新增 `src/r305Branded.contract.test.ts`（22 用例）：17 页锚点 + on 态页签容器 + t25/t29/t32 不回退 + 骨架双 class 覆盖。
- 未改 App.vue / index.vue / login.vue / desktop / 后端。

## 验证
- RED：新增契约时 17/21 失败（功能缺失真实红）。
- GREEN：22/22 passed；`cd oa4rust/frontend && pnpm --filter @oa4rust/mobile exec vitest run src/pages --reporter=dot` 46 files / 202 tests passed；`cd oa4rust && pnpm run typecheck` exit 0；H5 build 成功（index-4TKb15fX.js / index-CdSwBuzB.css）。
- 测试服：原子替换 H5 产物，grep 新哈希与 ai-brand-fill 命中。
- 真浏览器：19/19 页 branded>=1、computed 渐变生效、骨架捕获 11 页、pageErrors=0、API 失败=0，结果 H5_BRANDED_OK。证据 `oa4rust/test-results/r305-h5-branded/`（manifest.md + evidence.json + 逐页 PNG），来源为真实 Chromium 渲染真实测试服页面，**非物理真机、非静态/假数据拼图**；敏感域名与凭据不入库，文档以 `<YOUR_TEST_DOMAIN>` 占位。

## 状态
1000 轮持续迭代中的一环，非终态。
