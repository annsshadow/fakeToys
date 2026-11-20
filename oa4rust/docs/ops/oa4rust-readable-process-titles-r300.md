# R300 流程待办标题可读性修复（t33）

## 背景
用户评审指出 PC 待办与 H5 审批列表把原始流程实例键/UUID 当作主标题，业务可读性差。问题来源：DR-06、A3 P2-2、C1 流程批判。

## 修改范围
- PC：`frontend/apps/desktop/src/views/ProcessTaskCenterApp.vue`
- H5：`mobile/src/pages/process/process.vue`、`mobile/src/pages/process/start.vue`
- 契约：对应 `*.readableTitle*.test.ts`
- 本文：`docs/ops/oa4rust-readable-process-titles-r300.md`

## 业务规则
1. 展示层优先使用可读业务标题；原始 `start:`、`work-`、`s3-process-`、`probe-w-`、`task-`、`process-`、`node-`、`job-`、`snap-`、`activity-`、`reply-`、`data-` 与 UUID 形态键只折叠为副文本/详情标识。
2. `start:S3 运行闭环 work-…` 标题剥除实例键后若名称为空，必须回退为 `S3 运行闭环`，严禁空白主标题（本轮真浏览器复现并修复的回归）。
3. H5 流程列表 `titleOf` 与 PC `displayTitle` 均执行上述规则；`textOf` 继续供记录/日志原文使用，不改业务接口字段、字段来源与路由。
4. H5 发起页 `start.vue` 只保留真实 `name`，无名称时用 `流程 N` 占位，不把原始 ID 作主标签。

## 验证
- RED→GREEN：先新增可执行运行时契约（从真实 SFC 抽取脚本、esbuild 转换后 vm 执行 `displayTitle`/`titleOf`），在旧源码红灯；修复后 PC 7 tests passed、H5 29 tests passed。
- typecheck：desktop/ui/mobile 均通过。
- build：PC/H5 均通过；H5 修复后重新构建得到新 `index-DO3VAtrS.js` 与路由 chunk `pages-process-process.B5t-KFNz.js`。
- 部署：按既有静态资源原子替换流程部署到测试服；SSH 使用别名与 BatchMode，真值从环境或本机配置读取，不入库。
- 真浏览器：Playwright Chromium headless 对测试服真实页面实跑 PC 登录→流程待办、H5 登录→审批列表；最终 `evidence.json`：PC 45 个标题格，首标题 `S3 运行闭环`；H5 9 条，首标题 `S3 运行闭环`，`h5BlankTitleCount=0`、`h5RawTitleCount=0`。

## 证据来源与边界
证据位于 gitignored `test-results/r33-readable-titles-final/`，包含 `01-pc-process-titles.png`、`02-h5-entry.png`、`03-h5-process.png` 与 `evidence.json`。截图来源标注为 Playwright Chromium headless 对测试服真实页面，不是物理真机。测试域名、SSH 真值、账号口令均不写入仓库；以 `<YOUR_TEST_DOMAIN>` 与环境变量占位。

## 续作
项目处于 ≥1000 轮持续迭代，后续按独立设计评审与队长台账结论继续；本轮不代表项目完成。
