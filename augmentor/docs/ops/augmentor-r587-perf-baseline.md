# R587a augmentor/web 首屏关键资源计数器基线实测（只读甄别）

## 概况

- 任务：t133，真父 a64a59efdd39827863f39c846dd3591d7a7c1869（台账 R587a 立项）。只读甄别，不改源码/构建配置/检查脚本，不动台账。
- 目标：对当前构建树做 5 次独立冷加载，记录首屏关键资源计数器原始值、中位数与墙钟观察值，并据实与 `augmentor/web/scripts/check-dist.mjs` 的预算对照，给后续优化以可复现基线。
- 服务地址以 `<TEST_HOST>:<PORT>` 占位；本机回环仅用于取证，不入库。证据统一落 gitignored `oa4rust/test-results/r587-perf/`（.gitignore:28）。

## 方法与执行

1. 构建体检：`cd augmentor/web && VITE_BASE=/augmentor npm run build:check`，exit 0；日志 `build-check.log`。tsc --noEmit + vite build（v8.3.8，3869 modules）后 check-dist 六项全 ✓：入口 js 恰 1 份 43.2 kB ≤ 50 kB；11 页面 chunk 齐全且各 ≤ 100 kB；首屏 modulepreload 3 项无 echarts；大 vendor 未超基线；index.html 引用产物全部在场。
2. 短生命周期预览：`VITE_BASE=/augmentor npm run preview -- --host 127.0.0.1 --port 4173 --strictPort`，curl 验 /augmentor/ 200、入口 js 200(44198B)、antd vendor 200(1143589B)；完成后 job_kill 停止。
3. 真浏览器采集：runner 用 `require('@playwright/test').chromium.launch({headless:true})`，PC 1440x900 / DPR1；每次新建 BrowserContext（冷缓存），`page.goto(BASE + '/augmentor/', {waitUntil:'networkidle'})`；执行 `performance.getEntriesByType('resource')`，逐项记录 name、initiatorType、transferSize、encodedBodySize、decodedBodySize；计数器取中位数；墙钟仅作观察（中位数/P95），依 R566/R570 教训不设硬门槛。

## 5 次冷加载计数器原始值

- 入口脚本数 `entryScriptCount`：`[1,1,1,1,1]` → 中位数 **1**。
- 入口脚本传输字节 `entryScriptTransfer`：`[16178,16178,16178,16178,16178]` → 中位数 **16178 B**；编码字节 `[15878×5]` → **15878 B**；解码字节 `[44198×5]` → **44198 B**。
- 首屏 modulepreload 数 `preloadCount`：`[3,3,3,3,3]` → 中位数 **3**（rolldown-runtime / antd-vendor / react-vendor 各 1）。
- echarts 首屏数：`echartsMainCount=[0,0,0,0,0]`、`echartsVendorCount=[0,0,0,0,0]` → 中位数 **0 / 0**。
- 首屏有请求记录的页面 chunk 数 `pageChunkCount`：`[1,1,1,1,1]` → 中位数 **1**（DataManagement-CBs9IRZG.js，为默认路由，非 11 页全载）。
- 每轮资源条目总数 8；总传输字节 `[437811×5]` → 中位数 **437811 B**；总编码 `[435411×5]` → **435411 B**；总解码 `[1380101×5]` → **1380101 B**。
- 墙钟观察：`[2088,1561,1551,1536,1575] ms`，中位数 **1561 ms**、P95 **2088 ms**（仅观察，不作判定门槛）。5 轮 HTTP 200、rootChildren=1、title 正确、0 失败。

## 预算对照（check-dist.mjs:26-59）

| 指标 | 实测中位数 | 预算/基线 | 判定 |
| --- | --- | --- | --- |
| 入口 js 份数 | 1 | 恰 1（:27） | 通过 |
| 入口 js 体积 | 44198 B ≈ 43.2 kB | ≤ 50 kB（:28） | 通过 |
| 首屏 modulepreload 数 | 3 | 3 项、无 echarts（:49-51） | 通过 |
| echarts 首屏请求数 | 0 | 0（:50） | 通过 |
| antd vendor 体积 | 1143589 B ≈ 1116.8 kB | ≤ 1200 kB（:54） | 通过 |
| react vendor 体积 | 435 B ≈ 0.4 kB | ≤ 300 kB（:54） | 通过 |
| echarts vendor 体积（磁盘） | 561758 B ≈ 548.6 kB | ≤ 1500 kB（:54），首屏未请求 | 通过 |
| 页面 chunk | 首屏 1/11 | 11 页齐全、各 ≤100 kB（:32-45） | 通过（check-dist 六项全 ✓；首屏仅默认路由） |

## 首屏资源构成（run 1 明细，5 轮一致）

index-DbsW3xCT.js（script, 16178/15878/44198 B）、react-vendor-LKYXHABd.js（other, 735/435/435）、rolldown-runtime-XKyaHtl9.js（other, 1026/726/726）、antd-vendor-CoxS7Ohj.js（other, 358133/357833/1143589）、index-Bv0nMeyd.css（link, 617/317/317）、DataManagement-CBs9IRZG.js（script, 8733/8433/24194）、api-Dpz00v8j.js（script, 52089/51789/166642）、/api/data/list（xhr, 300/0/0）；字段顺序为 transfer/encoded/decoded B。antd vendor 占首屏传输约 81.8%（358133/437811），是后续优化首要观察点；本次仅记基线，不提改动建议。

## 结论与 Fail Loud

- 当前构建树首屏计数器稳定，所有 check-dist 预算均未越线；本轮为性能基线记录，无缺陷判定，不提出代码改动。
- 已据实记录：报告 `runner` 字段为 `r587-perf-runner.cjs`，runner 路径 `oa4rust/test-results/r587-perf/r587-perf-runner.cjs`；证据在 gitignored 的嵌套 `r587-perf/r587-perf/` 子目录（由 runner 的 `__dirname + 'r587-perf'` 计算得出，未改脚本重跑，sha 与内容一致）。
- `pageChunkCount=1` 仅统计 `performance.getEntriesByType('resource')` 的真实请求，非磁盘产物数；11 页齐全由 build:check 独立验证，二者不可混用。
- `/api/data/list` 在预览回环下由 Vite preview 回落到非业务响应（curl 抽测 502 / 浏览器 Perf 记 300 B transfer）；本轮仅记资源计数，不据其业务含义推断。
- 0 真值入库：文档仅占位域名与回环路径，未含密钥/令牌/真实域名或 IP/个人目录；本次无源码/配置写入，写域仅本文档。

## 证据 sha256（文件字节哈希）

- runner：`oa4rust/test-results/r587-perf/r587-perf-runner.cjs` = `ebd07695f15f90a52b9808db8b78294d571b073e8fbbddb2b1cd905a98e267bf`
- 报告：`oa4rust/test-results/r587-perf/r587-perf/report.json` = `285adcba705f914f10cd40e77066e90d976d8013405c50176466c0dac1e83288`
- runner 日志：`oa4rust/test-results/r587-perf/runner.log` = `6b78759e986735663320652b8716dbce236e7cca5aa0edcb3c58ee2a8d78ac34`
- 构建日志：`oa4rust/test-results/r587-perf/build-check.log` = `84997971488dd76bb31b60c21ba77913124d0fb275e8c27a1d9652fafdc94385`
- 构建产物 `augmentor/web/dist/index.html` = `ebc16172bd7531a53fb867d012ccd12ec7b570668873fdcb7bcd7a64d1d4d347`；预算脚本 `augmentor/web/scripts/check-dist.mjs` = `891d0103c04d8d2f768e98af47f93a3ed4b8cc6d9bf26455b723652b03308a5b`（只读锚点，未改）
