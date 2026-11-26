# Round3 皮肤页「配置未加载」假空态修复审计

- 任务：t30（backend），attempt_id 41f4fb61-9d6e-4f52-abc4-d658e87dd669
- 写域：`laiyipao/miniapp/src/pages/skins/**`、对应测试、`docs/laiyipao/audits/round3-skins/**`
- 截图性质：**真浏览器实测截图，非物理真机**（Playwright 驱动 Chromium，390x844 视口）

## 现象与根因

- `laiyipao/miniapp/src/pages/skins/skins.vue:5-10` 与 `:86` 存在「皮肤表还没加载出来」「皮肤表为空（配置未加载？）」文案。
- 页面数据只来自 `store.config?.skins`，原实现没有任何 `store.login() / store.loadConfig()` 调用。
- 基线同类证据：`laiyipao/audits/baseline-round1/data/h5-collection.log` 中 PAGE[8] enemies / PAGE[9] skills / PAGE[10] equipment / PAGE[11] gems 均出现空表与「配置未加载？」文案，但各页 `pageErrors=[], failedRequests=[], apiFailures=[]`。刚修复的 skills/equipment/enemies/gems 均属同一类：深链接或刷新直入时 pinia store 的 config 为空，页面却把「未加载」说成「配置为空」。
- 服务端 `/api/v1/config` 实际非空：本次本地实测 `skins=6, version=6237e0b623219b8b`；`laiyipao/server/internal/httpapi/routes_config_gems_e2e_test.go` 曾验证 gems=8。故不是种子缺失，而是前端加载链缺失。

## 修复（test-first RED→GREEN）

1. 新增 `laiyipao/miniapp/src/pages/skins/skins_fresh_load.test.ts`：`mountPage` 注入 `loggedIn=true, config=null`，mock `fetchConfigIfVersion` 返回两条 skin，**不在测试里手工 loadConfig**；断言 `fetchConfigIfVersion` 被调用 1 次、渲染 2 条 `.rxn.skin`、含皮肤名、不含两种假空态文案。
2. RED 命令（workdir `laiyipao/miniapp`）：`npx vitest run --config vitest.config.ts src/pages/skins/skins_fresh_load.test.ts --reporter=verbose`，exit 1，`skins_fresh_load.test.ts:64: expected "vi.fn()" to be called 1 times, but got 0 times`。
3. GREEN 修改 `laiyipao/miniapp/src/pages/skins/skins.vue`：
   - `:98-99` import 改为 `import { computed, onMounted, ref } from 'vue'` 与 `import { onLoad, onShow } from '@dcloudio/uni-app'`；
   - `:237-267` 新增幂等 `pageLoaded / loadInflight / doLoadPage / loadPage`，config 为空时 `login() → loadConfig()`，并让 `onLoad / onMounted / onShow` 统一调用。
   - 未改任何模板、样式或数据展示逻辑；onShow 仍负责刷新持有量。
4. GREEN 同一命令：1 passed，exit 0。

## 真浏览器深链接实测

- 脚本：`docs/laiyipao/audits/round3-skins/capture.mjs`（Tabbit Playwright CLI，实例变量 `TABBIT_PLAYWRIGHT_INSTANCE` 由本地环境注入；未写入仓库）。
- 步骤：setViewportSize 390x844 → 打开本地 H5 origin → `localStorage.clear()` → 深链接 `http://127.0.0.1:5174/#/pages/skins/skins` networkidle → 等待 `.rxn.skin` → 采集 DOM 文本 → 截图。
- 结果：`rows=6`，`countText="显示 6 / 共 6 条"`，`falseEmpty=false`，`names=[游猎者, 炽炎使徒, 霜行者, 雷鸣者, 腐沼行者, 白垩女王]`。
- 截图：`docs/laiyipao/audits/round3-skins/screenshots/skins-deep-link-after.png`（390x844，79951 bytes）。画面可见「显示 6 / 共 6 条」与皮肤卡片，假空态消失。**非物理真机**。

## 回归验证

- `cd laiyipao/miniapp && npm test`：Test Files 166 passed | 1 skipped (167)；Tests 1574 passed | 2 skipped (1576)；Duration 14.91s。
- `cd laiyipao/miniapp && npm run type-check`：`vue-tsc --noEmit -p tsconfig.json` exit 0。
- 上述 stderr 中出现的既有 `build.attacker 缺失` 警告来自其他测试，非本任务引入，且未导致失败。
- 全程未执行 git 操作；未将真实域名 / IP / 口令 / 私钥写入仓库、日志或对话（本地 keys 仅用于启动进程，未回显值）。

## 结论

皮肤页假空态已按既有 t21/t27 模式修复并经真实浏览器深链接实测确认非空表；测试与类型检查全绿。
