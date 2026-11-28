# R639b MeetingApp 楼栋列表异步恒空补回填（真修复 P2）

## 结论
修复 MeetingApp.vue 楼栋列表因 useQuery 异步响应未在回填点消费而恒空的问题：以 \`watch(bData, ...)\` 替代 setup 同步一次性赋值，使异步到达的楼栋清单正确回填 \`buildings\`。判级沿用 R638a 甄别结论 P2，非新判级。

## 立项性质与范围
- 基线 HEAD=236ef513c05a56efa434e46a1fba3b97b16e507f（与派发一致）。
- 甄别文档：oa4rust-r638a-meeting-building-list.md（P2，零源码改）。本次为真修复。
- 写域仅 3 文件：MeetingApp.vue（M）、MeetingApp.buildings.test.ts（A）、本文档（A）。未碰台账与他人文件。
- 路由可达：main.ts:95-96 \`/app/meeting\` 普通登录可达，故非死代码。

## 缺陷机制（修复前，基线逐行核实）
- :118 \`import { computed, onMounted, ref } from 'vue'\` —— 无 watch。
- :150 \`buildings = ref<Bldg[]>([])\`。
- :186-190 useQuery：queryKey \`['meeting','bldgs']\`，queryFn \`api.get('/api/meeting/assemble/control/building/list')\`，staleTime 120000。
- :191 \`buildings.value = bData.value ?? []\` —— 在 setup 同步阶段执行一次，首屏 bData undefined → buildings=[]；全文件零 watch/watchEffect，异步到达后无补写。
- 模板 :18/:29/:89 三处 \`v-for="b in buildings"\` 恒空，会议室预订核心表单楼栋/会议室下拉不可用。
- 兄弟路径 :540-552 addBuilding 中 \`buildings.value = (r.data ?? []) as Bldg[]\` 正确回填可绕过，故 P2 而非 P1。

## 修复内容（基线 → 现状）
- :118 \`import { computed, onMounted, ref, watch } from 'vue'\`（增 watch）。
- :191 \`buildings.value = bData.value ?? []\` → \`watch(bData, (v) => { buildings.value = v ?? [] }, { immediate: true })\`。
- \`immediate: true\` 保留首屏 undefined→[] 语义；异步到达后由 watch 回填真实楼栋清单。
- addBuilding/removeBuilding 本地更新语义与 :548 回填写法保留不变。

## 真测试（RED→GREEN）
契约测试 oa4rust/frontend/apps/desktop/src/views/MeetingApp.buildings.test.ts：
- 用 ts.transpileModule + new Function 真实执行从源码提取的 useQuery 段到"代际令牌"注释前的回填逻辑，传入真实 vue \`watch/ref/effectScope\`。
- 喂 bData 先 undefined 后异步三栋楼，断言 buildings 从 [] 回填为三栋（含逐名）。
- 首屏 undefined 不报错且 buildings 为空数组；源码守卫断言 watch(bData 注册且同步赋值已移除；import 含 watch；对照 :548 回填写法仍在。
- 真父（无 watch）RED：3 failed | 2 passed，EXIT=1；修复后 GREEN：5 passed，EXIT=0。
- 在 desktop 包 cwd 跑 vitest 入口，规避 R622c cwd/ENOENT 坑。

## 验证矩阵（Fail Loud 据实）
| 项 | 命令 | 结果 |
|---|---|---|
| V1 定点 | \`pnpm exec vitest run src/views/MeetingApp.buildings.test.ts\`（desktop cwd） | 5 passed，EXIT=0 |
| V2 类型 | \`pnpm exec tsc --noEmit\`（desktop cwd） | EXIT=0（.vue 为 tsc 盲区，据实披露） |
| V3 格式 | \`pnpm exec biome check MeetingApp.buildings.test.ts\` | EXIT=0 |
| V3 Vue | \`pnpm exec biome check MeetingApp.vue\` | EXIT=1，唯一错误为 :214 useTemplate（基线既有，修复行 :118/:191 零命中） |

## 行为变化
- 首屏：bData undefined → buildings=[]（与修复前一致）。
- 异步到达：修复前恒空；修复后回填为后端返回楼栋清单，楼栋/会议室下拉可用。
- 无服务端/权限/落库变更；纯前端响应回填修复。

## SHA256 分列
- baseline HEAD=236ef513c05a56efa434e46a1fba3b97b16e507f
- sourceSha（MeetingApp.vue）=796a30194c064ad5a6377c20e390fd0d93cce45c1588105b44016178e201e336
- runnerSha（契约测试）=fe6970e272d119364c414c7f484209300b526895c72ef2f12f57bfc8980ce7be

## 脱敏
无凭据、无真实域名/IP/端口；测试夹具为本地构造对象；占位符 <TEST_HOST>（未使用真值）。
