# R314 F1 封装守卫盲区修复

## 来源与根因

t80 R314 静态审计（docs/ops/oa4rust-r314-contract-health-audit.md）坐实：tests/contracts/route-methods.test.ts 的 SRC_ROOTS 只含 frontend/apps/desktop/src 与 mobile/src，未包含 frontend/packages/apis/src。R313 三处封装漂移（documentApi.create/update、programCenterApi.dictCreate）正因此长期潜伏，只能靠临时新增的 apiWrapperDrift.R313.contract.test.ts 才发现。

## 方案与论证（不掩盖新增漂移）

- 将 packages/apis/src 纳入 SRC_ROOTS，Call 增加 source:legacy|apis 区分。
- 纳入实测：totalCalls=4948、mismatch=85（全在 index.ts）、unregistered=1027（legacy 860 + apis 167，修正正则后实测 apis 187，187 为最终冻结基线）。
- 用「冻结历史快照」而非 EXEMPT 放行存量：API_LEGACY_MISMATCHES 精确收录 85 条错方法签名，仅表示历史存量另案治理；EXEMPT 仍为空。
- classify() 对 apis 包调用：命中后端方法→ok；签名在快照→legacy（不失败）；否则→mismatch（守卫红）。任何新增错方法签名都不会被快照吸收。
- 未注册计数分开 legacy(<=860) 与 apis(<=187) 两条基线，避免包间互相掩盖；基线只允许下降，修好一批即同步下调。

## F3 五处已知待修（不修、不静默塞入 EXEMPT）

在 F3_KNOWN_PENDING 显式标注，且均与 API_LEGACY_MISMATCHES 交叉核对、EXEMPT 不含它们：

- generalApi.dictItemCreate index.ts:284 POST /api/general/dict/item/{}（后端仅 POST /api/general/dict/item/create）
- jpushApi.templateUpdate index.ts:355 PUT /api/jpush/core/entity/template/{}（后端仅 GET）
- queryViewApi.search index.ts:495 GET /api/queryview/search（后端 POST）
- queryViewApi.importModelList index.ts:491 GET /api/queryview/importmodel/list/{}/{}（后端 POST）
- programCenterApi.moduleInvoke index.ts:538 POST /api/program_center/invoke/{}（后端 GET+PUT+DELETE）

## 守卫有效性验证（test-first）

- RED：纳入 apis 包但不设基线时跑出 mismatch=85 / unregistered=1027，证明盲区真实存在。
- 注入验证：测试内写临时 __r314_guard_probe.ts（api.get(/api/document)），collectCalls 扫到后 classify 判 mismatch，断言签名不在冻结快照；finally 删除探针。
- GREEN：守卫 2/2 passed（含注入验证 it），全量 route-methods+桌面契约+apis 测试 0 failed。

## 证据索引

- 守卫文件：tests/contracts/route-methods.test.ts（SRC_ROOTS/冻结基线/探针验证）
- 存量明细：gitignored test-results/r314-mismatches.json（85 条，已内联冻结）

本任务纯静态本地，未部署测试服（t79 qa-e2e 正在测试服取证，避免产物干扰其基线）。本文件不含任何生产/测试服务器地址、口令或环境真值。
