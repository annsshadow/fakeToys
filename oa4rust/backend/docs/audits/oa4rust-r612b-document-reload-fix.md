# R612b DocumentApp 刷新/重试按钮死绑定真修复审计

## 结论
- 判定：P2（前端可用性缺陷，已修复）。真父 HEAD `fe330ade932f1235edecd496d3f39abb5995dad5`。
- 缺陷：`DocumentApp.vue` 模板两处事件绑定指向未定义的 `reload`，刷新/重试按钮点击无任何效果（no-op）。
- 写域：3 个源码/测试文件 + 本文档，不触碰 out-of-scope 文件。
- 路径说明：任务书原定 `frontend/docs/audits/` 目录在本仓库不存在，据实改用既有审计目录 `backend/docs/audits/`。

## 立项与范围
- 任务 t208，attempt 6f2b7703。目标：将死绑定改绑到已存在的 `doSearch`，以契约锁定并 RED→GREEN 坐实。
- 修改：`DocumentApp.vue`（2 行）、`DocumentApp.slots.contract.test.ts`（1 行断言）、新增 `documentReload.test.ts`。

## 机制锚点（file:line）
- `oa4rust/frontend/apps/desktop/src/views/DocumentApp.vue:12`：`<AdminToolbar ... @search="doSearch" @refresh="reload">`。
- 同文件 `:27`：`<AdminTable ... @retry="reload">`。
- 全文件 `reload` 仅上述 2 处命中，`<script setup>` 无 `reload` 定义；`listQuery.ts:26-30` 虽有同名导出但未被 import，属真死绑定。
- 正确目标 `doSearch`（同文件 `:207-232`）执行 `api.post("/api/document/filter/list/1/size/50/manager", ...)`；兄弟范本 `AppInfoApp.vue:14/:27` 均绑 `doSearch`。

## 修复
- `:12` `@refresh="reload"` → `@refresh="doSearch"`。
- `:27` `@retry="reload"` → `@retry="doSearch"`。
- 同步修正测试锁定断言 `DocumentApp.slots.contract.test.ts:48`（原锁定 `reload`，据实披露并改为 `doSearch`）。

## 决定性实测（RED→GREEN，本人亲跑）
| 阶段 | 命令 | 结果 |
|---|---|---|
| RED（真父版覆盖） | `pnpm vitest run src/views/documentReload.test.ts src/views/DocumentApp.slots.contract.test.ts` | EXIT=1，2 files failed，4 failed / 5 passed |
| GREEN（修复态） | 同上 | EXIT=0，2 files passed，9 passed / 9 |
- 真父版 SHA256 `3b6a9970c9f9cd7c3dc0c45273f1b7f5d93b766152405e85f091d8936c85a8aa`；修复态见 SHA 清单；RED replay 全程 HEAD 未变。

## 三 verify
- V1 定点：9 passed，EXIT=0。
- V2 `pnpm typecheck`（`tsc --noEmit`）EXIT=0；`pnpm vitest run` 全量 2249 tests 全 passed，EXIT=1 仅因 2 个既有基线 suite 加载失败（`tabStateA11y.contract.test.mjs` 无 test suite；`TaskCompleted.R314.contract.test.ts` 需 backend cwd 致 ENOENT），两文件 git status 干净未改。
- V3 新契约 biome check EXIT=0。

## 三分判定
- 可达 P2 已修 1 项：刷新/重试按钮 no-op，点击无网络请求。
- 不可达 0 项 / 正常 0 项。非 P1：仅影响手动刷新入口，tab 切换仍触发 `doSearch`，无数据损坏或越权。

## Fail Loud 披露
- `tsc --noEmit` 非 `vue-tsc`，不解析 `.vue` 模板，模板绑定正确性由 jsdom 真挂载契约覆盖。
- 全量 vitest 2 个既有基线 suite 失败非本次引入，未冒充 pass。
- `DocumentApp.vue` 整文件 biome 存在基线 format 错误，远离改动行 `:12/:27`，未整文件 `--write` 以免扩散 diff。
- slots 契约原测试锁定错误断言，已据实修正。

## SHA 清单
- runnerSha（=sha256(JSON.stringify(meta))）：`29d73473f298b2d5b08e1f70b8373af16113ba0be5ca285afd01e6e446faf83b`
- `DocumentApp.vue`：`f3e9f45214fc306b4382d0a84323def94f712eed61c2a0a48bfa7d1becfcc60b`
- `DocumentApp.slots.contract.test.ts`：`b380e707002331cdd90bc07a2808a428f529e47e748145040538cc5d3bbc3d5a`
- `documentReload.test.ts`：`aaa76529421dbd9574a945a55ce327fead697ed4049ec287cfdb16de1792fb53`
