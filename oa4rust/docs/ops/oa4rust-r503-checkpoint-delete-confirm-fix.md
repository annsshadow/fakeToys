# R503 WCAG 3.3.4 删除断点二次确认修复

## 1. 缺陷与根因
R502 甄别（commit fb973d8a，文档 `oa4rust/docs/ops/oa4rust-r502-error-prevention-audit.md`）判定 1 项可达真缺陷 P2：
`augmentor/web/src/pages/Augmentation.tsx` 的 `doDeleteCheckpoint` 单击断点 Tag 关闭图标即直接调用 `deleteCheckpoint(taskId)`，无二次确认。真浏览器四时点观测 dialog/popconfirm/native 均为 0，仅出现成功 toast，服务端 checkpoints 立即清空——不可逆破坏性操作缺少确认，违反 WCAG 3.3.4（Error Prevention）。

## 2. 修复方案
仅在 `doDeleteCheckpoint` 外包一层 Ant Design `Modal.confirm`：
- title：`确认删除断点`；content：`删除后该任务须从头重跑，确定继续？`
- 原 `try { await deleteCheckpoint(taskId); message.success('断点已删除'); await loadCheckpoints() } catch ...` 整体移入 `onOk`；取消路径不调用删除。
- antd import 追加 `Modal`；Tag `onClose` 仍调用 `doDeleteCheckpoint(id)`。
硬纪律：业务处理器语义、API 端点、toast 文案、Tag 渲染、`data-testid`、样式、其他页面全部零改。

## 3. 写域扩展（据实披露）
写域由 3 文件扩展为 4 文件（队长批准）：新增 `augmentor/web/src/pages/Augmentation.test.tsx`。原因：该文件第 56 轮旧用例编码了「点关闭即直接删」的废弃行为，新增确认框后必然失败；取消断言属运行时绕过确认的假通过，禁止。故更新该用例为「取消不删 + 确定才删」双段语义（引入 `Modal.destroyAll()` 清理 confirm DOM），非削弱断言。

## 4. 契约 RED → GREEN
契约 `augmentor/web/src/pages/R503.checkpointDeleteConfirm.contract.test.ts`（源码字符串断言，5 it）：核心 3 项（Modal.confirm 存在、deleteCheckpoint 在 onOk 内且之前无调用、content 含「从头重跑」）+ 2 组守卫（Tag onClose / data-testid / toast 文案 / loadCheckpoints 内容零改）。
- 真父 fb973d8a 定点 RED：Tests 3 failed | 2 passed（5），失败正是 3 核心断言，守卫 pass，无桩错。
- 修复版 GREEN：5/5 passed。

## 5. 三验证（全 exit 0）
- `cd augmentor/web && pnpm exec vitest run src/pages/R503.checkpointDeleteConfirm.contract.test.ts`：5/5。
- `cd augmentor/web && pnpm test`：26 files / 449 tests，0 failed（含更新后 Augmentation.test.tsx 16/16）。
- `cd augmentor/web && pnpm typecheck`：`tsc --noEmit` exit 0。

## 6. 真浏览器实测
Playwright Chromium headless，1440x900 DPR1（非物理真机），测试服 `/augmentor/` 子路径部署。断点数据经真实后端 API 流程生成（`POST /api/data/upload` + `POST /api/augment/start`），非桩数据。augmentor App 写死暗色主题，无主题切换，故单暗态实测，不冒充双主题。
构建与部署注意：Vite 需同时设 `VITE_BASE=/augmentor/`（静态资源子路径）与 `VITE_API_BASE=/augmentor-api/api`（API 反代前缀； nginx `location /augmentor-api/` 剥离前缀后转发到后端 `/`）。首版缺省构建导致静态资源与 API 均 404 白屏，属部署配置问题，非代码缺陷，已据实修正重建。
CDP（`DOM.enable` + `Accessibility.enable` + `getPartialAXTree`）坐实：`role=dialog`、`axName=确认删除断点`、`axIgnored=false`；Cancel/OK 按钮均可达。
双路径：点关闭 → 弹确认 → 点 Cancel：断点 Tag 保留、0 DELETE 请求；再点关闭 → 点 OK：出现 DELETE `/api/augment/checkpoints/{task_id}`、断点列表变「暂无断点」、`断点已删除` toast 由 MutationObserver 捕获。对照页数据管理（DataManagement）零回归，0 pageerror。终轮 13/13 checks 全绿。

## 7. 证据（sha256，命令行原值；目录 gitignored）
证据目录 `oa4rust/test-results/r503-checkpoint-confirm/`（不入版本库）：
- `pc-datamanagement-control.png` = `60a9608940475a04a8bce846572463cf334438cd4cd2fe2f48c4258d1420ad31`
- `pc-checkpoint-before.png` = `0503dc4239dd469644ba6e9c544f35af3242df2034730ec242e95311049a19cd`
- `pc-checkpoint-cancel.png` = `802791893ffee33b07a81e98d04144c6facd900cc88d32a979faba8db0da2bb6`
- `pc-checkpoint-confirmed.png` = `735d3cb15d2f4af8e4ea054db766b88f46c07e4f358a7b5f8d01fc897f8a9cc5`
- `r503-report.json` = `9153eceda1600fc5049d928918b1d78f0584fdb8f031e1cacd2faeb3f20904f1`
截图均为 headless 非物理真机 1440x900，如实标注，不冒充真机。

## 8. 真值纪律
测试域名/主机以 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；API key 仅从部署环境进程环境读取并注入运行态 localStorage，不入库、不回显。证据目录与部署/造数脚本均在 gitignored `oa4rust/test-results/`，不在提交范围。