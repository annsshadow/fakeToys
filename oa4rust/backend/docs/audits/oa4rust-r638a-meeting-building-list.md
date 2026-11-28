# R638a MeetingApp 楼栋列表恒空甄别审计（t321）

## 结论
判定 1 项真实可达前端缺陷，严重度 P2：MeetingApp 楼栋列表在 setup 同步阶段一次性取 \`bData.value\`（异步查询此时为 undefined），未订阅响应式回填，导致楼栋下拉恒空；会议室预订核心表单不可用。可先手动新建楼栋触发 :548 回填绕过，且非权限问题，故不判 P1/P0。

## 立项性质与范围
- 验证型甄别：零源码改动，唯一交付物为本审计文档；探针与日志置于 gitignored 证据目录，用后即留。
- 真父 HEAD=2b22ce40f2ef2c13e3fdc40117f13fb123319f39。
- 台账去重：\`docs/ops/current-state.md\` 2839 行=并发写入无 latest 门、4030 行=R300 真浏览器楼栋无种子、4749-4755=R326 审计 L1 楼栋无种子；本缺陷（异步响应未回填）从未专项甄别。

## 机制锚点（逐行核实）
- \`oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:150\` \`buildings = ref<Bldg[]>([])\` 初始空数组。
- \`:186-190\` \`const { data: bData } = useQuery({ queryKey: ['meeting','bldgs'], queryFn: () => api.get('/api/meeting/assemble/control/building/list').then(r => r.data ?? []), staleTime: 120000 })\`：标准 TanStack Vue Query 异步查询，\`bData\` 是异步填充的 ref。
- \`:191\` \`buildings.value = bData.value ?? []\`：setup 同步执行一次；首屏 \`bData.value\` 为 undefined → buildings 定为 []。全文件 grep \`watch(\`/\`watchEffect\` 零命中，异步到达后无补写。
- 模板 \`:18\`/\`:29\`/\`:89\` 三处 \`v-for="b in buildings"\`（新建会议表单楼栋下拉/选楼栋看会议室/编辑表单），恒空 → 无法选楼栋 → \`loadRooms\`(:194)/\`loadRoomList\` 拿不到会议室。
- 兄弟范本 \`:540-552\` \`addBuilding\` 在 \`await api.post\` 后 \`:547-548\` \`const r = await api.get(...); buildings.value = (r.data ?? [])\` 正确消费异步响应，证明回填写法在本组件可行。
- 后端 \`crates/meeting_assemble_control/src/routes.rs:32\` \`building/list\` 已注册（u2_building_list），非死路径；前端 \`main.ts:95-96\` \`/app/meeting\` 仅需登录（\`beforeEach\` 守卫仅拦 \`/app/admin\`），普通登录可达。

## 方法（真测试）
- vitest + jsdom（node 环境即可，Vue ref 不依赖 DOM）：\`test-results/t321/r638a_probe.test.ts\` 5 用例。
- 用 \`ts.transpileModule\` + \`new Function\` 真实执行 :150/:186-191 等价逻辑：mock \`bData\` 为初始 undefined 的 ref，微任务后赋值楼栋数组；坐实 setup 同步取值后 buildings 恒空、异步到达后仍空。
- 源码守卫断言同步取值语句存在、watch/watchEffect 零命中、模板三处 v-for；对照 :548 写法能正确回填。

## 决定性实测
| 观测 | 结果 |
| --- | --- |
| setup 同步阶段 bData.value / buildings.value | undefined / [] |
| 异步赋 2 楼栋后 bData.value / buildings.value | 2 条 / []（仍恒空） |
| 源码 watch/watchEffect 命中 | 0 |
| 对照 :548 addBuilding 写法 | 回填 1 楼栋成功 |
| 模板 v-for="b in buildings" 命中 | 3 |
| 测试汇总 | 5 passed，0 failed（EXIT 0） |

## 三分裁决
- 可达 P2：会议室预订核心功能阻断（三处楼栋下拉恒空、连带会议室选项不可用），但存在部分绕过——手动新建楼栋触发 :548 回填后楼栋选项出现；且纯前端响应消费遗漏，非权限/越权/数据泄露问题。故定 P2 而非 P3（影响核心业务）也不预支 P1（非全局阻断且可绕过）。
- 对照同族：R300/R326 为楼栋无种子（后端数据缺失），本缺陷为异步响应未回填（前端消费遗漏），机制不同，独立成立。

## Fail Loud
- 探针为 gitignored 证据目录中的临时文件，不入提交；仅审计文档入仓。
- 未跑真实 GUI 浏览器操作，结论由真实代码执行探针 + 逐行源码锚点支撑，未采信未验证推测。

## 证据 sha256（据实分列，meta 不可复算）
- runnerSha=探针文件 sha256=af3a297132c6e477f14a4602f8dae4ea4214612ed43f7242a26ba167ec3118c6
- sourceSha=MeetingApp.vue sha256=5d0b27660b583b2672397e514a217caa0aa9041306880d91dc2433b033d51d9d
- 测试日志：oa4rust/test-results/t321/r638a-probe.log（5 passed，EXIT 0）
- 占位：环境相关地址统一 \`<TEST_HOST>\`，仓库无真值。

## 修复方向（建议，不实施）
- 将 \`:191\` 同步取值改为 \`watch(bData, v => { buildings.value = v ?? [] }, { immediate: true })\`，或模板直接消费 \`bData ?? []\` 并移除冗余 buildings ref；保留 :548 回填语义。
- 同步核查 :194 loadRooms / :563+ loadRoomList 的楼栋依赖在回填后能重新触发。
