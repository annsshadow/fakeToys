# R607b ProcessDesigner 重置默认按钮死绑定真修复

- 结论：真 bug 已修复，RED→GREEN 坐实。ProcessDesigner.vue 的「重置默认」按钮 @click 指向 resetRules()，全文件仅模板一处出现、script 中无任何定义；点击必抛 ReferenceError。
- 真父 HEAD：4097aa6a08e2f12e3bd750026f86ddc0fd52c64e（git 核实 HEAD 等于真父；真父..工作树 diff 仅本次一行）。
- 写域精确 3 文件：M ProcessDesigner.vue（仅 :976 一行）、A 契约测试、A 本文档。其余零改。

## 立项与机制锚点
- 缺陷锚点 ProcessDesigner.vue:976：<button class="bc" @click="resetRules()">重置默认</button>。ops_grep 全文件 resetRules 仅命中 1 处（该行），script 段无 function/const/let/var 定义。
- 触发面：:947 showRulesModal 弹窗内 :975 rr-actions 操作区，:24 与 :802 的「🔗 规则」按钮打开弹窗后即可点击，缺陷真实可达。
- 设计意图铁证（兄弟正确实现）：同文件 :5931-5958 function resetConnectionRules() 完整生成默认连线规则并赋值 connectionRules.value = rules；:5959 初始化即调用。:5960 isAllowed / :5963 toggleRule / :5967 saveRules 配套齐全。
- 修复选择：复用已有正确实现 resetConnectionRules()，不新增函数、不改兄弟函数体，最小且语义一致。

## 修复（diff 1 file changed, 1 insertion(+), 1 deletion(-)）
- 仅 :976 将 @click="resetRules()" 改为 @click="resetConnectionRules()"；按钮文案、class、相邻「💾 保存规则」按钮及全部模板/script/style 零改。

## RED → GREEN（队长可独立 replay）
| 版本 | 契约结果 | 关键行 |
|---|---|---|
| 真父源码 | 2 failed / 2 passed | handlerName=resetRules 且 script 无定义；未重绑 resetConnectionRules |
| 修复态 | 4 passed / 0 failed | 抽到 resetConnectionRules 且 script 有定义；兄弟守卫全命中 |
- 契约从 Vue 源码 readFileSync 定位「重置默认」按钮的 @click 表达式，仅取 script 段判定目标是否有定义；真父版必然失败，队长 stash 该文件回退即可复现 RED，恢复后 sha 与修复态一致。
- replay：git stash push -- ProcessDesigner.vue → RED → git stash pop 恢复 → 修复态文件 sha=add7505f1b754aca8762dd240231a7b8b5cb279c857946e99e919cf6e60cc586。

## 三 verify（全绿）
- V1：新契约 src/views/ProcessDesigner.resetRules.contract.test.ts 4 passed；既有 ProcessDesigner.writeEnvelope（17）+ ProcessDesigner.copy（1）共 18 passed。
- V2：pnpm --filter @oa4rust/desktop typecheck → tsc --noEmit EXIT0，无 TS 报错。
- V3：biome check 两改动文件 EXIT0（4 条 useTemplate info 级可选提示，非 error、不阻塞）；全量 vitest 2241 passed（279 files），比上轮基线 2237 多 4 即新契约本身。
- 全量 vitest 另有 2 个 suite 加载失败，均为既有基线、非本次引入、两文件 git status 干净未改：tabStateA11y.contract.test.mjs 无 test suite；TaskCompleted.R314.contract.test.ts 需 backend cwd 否则源码路径 ENOENT。

## 六维守卫（零外溢）
- ProcessDesigner.vue diff 仅 1 行；resetConnectionRules 函数体（:5931-5958）、isAllowed、toggleRule、saveRules、connectionRules 默认规则赋值、showRulesModal 弹窗容器、rr-actions 其余按钮全部零改。
- main.ts（sha=21986760a56c6f3696dc71f42671e5664ae9edf3）、desktop package.json（sha=6fc62d3b435359c0e6636a228d3e9e56d4393d56）零改；components/、其他 views/、backend crates、迁移、docs/ops/current-state.md 零改（git status 核实）。

## Fail Loud
- desktop 无 test 脚本，pnpm test 为 EXIT0 空操作；以 vitest run 全量作为真实证据，据实披露。
- biome 首检因命令行 --config-path 触发 ignore-file 错误，改从 oa4rust 根目录自动发现配置后 EXIT0；源码与契约逻辑未变。
- 工作树含他组既有 untracked（%SystemDrive%/、augmentor/web/oa4rust/、docs/ops/current-state.md.dry、oa4rust/oa4rust/、test-results/），本次完全未触碰；精确 git add 仅 3 写域文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=6c206cf8fe84d87a28733d8391791dfa0f9673e83dcc5ae073ba318297a7e78c
- apps/desktop/src/views/ProcessDesigner.vue=add7505f1b754aca8762dd240231a7b8b5cb279c857946e99e919cf6e60cc586
- apps/desktop/src/views/ProcessDesigner.resetRules.contract.test.ts=a647bf1d1c38f3371b0b840dbd62faa3153394a3132347e92c195c7ce09e0698
- apps/desktop/src/main.ts（守卫零改）=21986760a56c6f3696dc71f42671e5664ae9edf3
- apps/desktop/package.json（守卫零改）=6fc62d3b435359c0e6636a228d3e9e56d4393d56

## 修复方向
- 已完工：重置默认按钮改绑已有 resetConnectionRules()，点击恢复默认连线规则、不再抛 ReferenceError；后续可补 e2e 覆盖真实点击后的规则矩阵复位。
