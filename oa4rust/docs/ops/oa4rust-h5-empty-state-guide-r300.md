# H5 关键业务空态引导与首页空白修复（R300 / t32）

- 模块：oa4rust/mobile（H5 / uni-app Vue3）
- 类型：实现（test-first RED→GREEN）
- 来源：t11 设计评审 DR-05（业务空态无引导 + H5 首页空白）与 C1 H5 逐页批判（文档/会议/收藏/浏览/回收站空态缺引导）

## 目标与范围
为 H5 关键业务空态页提供可读引导文案与可操作入口，消除纯空白/裸空态；**不伪造数据**、**不改业务逻辑/接口/路由**。

路径偏差说明：任务 inScope 写的 `favorite/**` 与 `home/**` 在实际仓库中不存在；实际收藏页为 `oa4rust/mobile/src/pages/collect/collect.vue`，首页为 `oa4rust/mobile/src/pages/index/index.vue`。按实际路径落地。

## 变更
1. 契约（test-first）：新增 `oa4rust/mobile/src/pages/emptyGuide.emptyState.test.ts`，源码级断言 6 页含 `empty-guide` 容器、引导文案、真实函数绑定、无裸空态。首轮 RED：6 用例全失败（缺失引导）。
2. 文档页 `doc.vue`：附件空态「还没有附件」+ 拍照/相册、上传文件、刷新；文件空态「文档库暂无文件」+ 上传文件、刷新。
3. 会议页 `meeting.vue`：空态「换个页签看看或刷新」+ 刷新、切换页签。
4. 收藏页 `collect.vue`：空态「暂无收藏」+ 去浏览内容（navigateTo 浏览页）、刷新。
5. 浏览页 `browse.vue`：空态「暂无数据」+ 刷新、去搜索（navigateTo 搜索页）。
6. 回收站 `recycle.vue`：空态「回收站为空」+ 刷新、去文档（navigateTo 文档页）。
7. 首页 `index.vue`：公告空态「暂无公告」+ 刷新公告，消除纯空白。
8. 各页 scoped 样式注入 `.empty-guide*` 局部类，入口 `min-height:44px` 符合触控规范，配色沿用 `--color-*` 令牌；首页 stat-card 保留 R300 AI 渐变。

兼容性处理：既有的 browseFeed/collectFeed 契约要求「暂无数据/暂无收藏」字面量且位于错误分支之后。引导文案保留这些字面量于主文案，新增说明置于 hint，使新旧契约同时满足（GREEN）。

## 验证（全部通过）
- `cd oa4rust/frontend && pnpm --filter @oa4rust/mobile exec vitest run src/pages --reporter=dot`：40 files / 175 tests passed（含新增契约 7 用例）。
- `cd oa4rust/frontend && pnpm --filter @oa4rust/mobile run typecheck`：tsc --noEmit 通过。
- `cd oa4rust/frontend && pnpm --filter @oa4rust/mobile build`：H5 构建成功。
- 测试服部署：构建产物经本地 tar → SSH 备份 → scp → 原子切换，远端 index.html 引用新构建哈希，grep 确认 `empty-guide` 已上线。
- 真浏览器实测：Chromium 390x844 真实 UI 登录 + 6 页逐页断言与截图，loggedIn=true / guidePages=6 / bad=0 / tapBad=0 / pageErrors=0 / passed=true。

## 证据
- 证据目录：`oa4rust/test-results/r300-h5-empty-guide/`（gitignored），含 manifest.md、evidence.json、7 张逐页截图。
- 截图为**真浏览器渲染真实测试服页面**，**非物理真机**、**非静态/假数据拼图**；凭据运行时读取不回显不入库；测试服域名/端口真值不入库（文档中统一以 <YOUR_TEST_DOMAIN> 占位）。

## 迭代声明
本项目为 ≥1000 轮持续迭代，本轮仅为其中一环，不代表项目完成；后续按评审结论继续。
