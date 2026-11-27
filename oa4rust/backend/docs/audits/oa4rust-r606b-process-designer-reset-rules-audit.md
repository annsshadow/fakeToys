# R606b 流程设计器连接规则弹窗 resetRules 死绑定审计（验证型甄别）

- 结论：P2。ProcessDesigner.vue:976 的「重置默认」按钮绑定 resetRules()，但该标识符在整份 <script setup> 中无任何定义；点击必抛 ReferenceError，属用户可感知的局部功能失效；可手动逐条 toggle 规则绕过，不破坏数据、不越权、不阻断主流程。
- 派发真父 HEAD：2a6b9b5f7a1fae0c7a0af2f6280471ef4de9fcd1（git 核实存在）。
- 实际工作树 HEAD：fb28706c893991af800de4e51d0ae0207ea8dbca（git rev-parse 核实）；git merge-base --is-ancestor 2a6b9b5f7 HEAD 返回 0，即派发父为实际 HEAD 的祖先，期间前端目标源码字节未变（见 SHA 清单）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 临时路径，用后即删。

## 立项与范围
- 主对象 apps/desktop/src/views/ProcessDesigner.vue:946-981 连接规则弹窗模板；:976 <button class="bc" @click="resetRules()">重置默认</button> 为缺陷锚点。
- 路由真实可达：apps/desktop/src/main.ts:212 path:'process-designer'、:214 component 动态导入 ProcessDesigner.vue、:511 redirect:'/app/admin/process-designer'，requiresAuth。
- 次要候选（非主对象）：apps/desktop/src/views/OrgDirectoryApp.vue 部门树过滤，锚点属实，详见「次要观察」。
- 不扩范围：不修改任何源码/路由/测试配置；仅甄别与取证。

## 机制锚点（真父源码逐行）
- resetRules 全文件唯一出现即模板 :976 一次；tools.grep 与全 src 目录检索均只此一处，<script setup> 内 function/const/let/var/defineExpose 六种定义形式全部无命中。
- 兄弟真函数对照：saveRules() 定义于 :5967；resetConnectionRules() 定义于 :5931 并在 :5959 被调用；resetConnectionRulesEnhanced() 定义于 :7424（内部调 renderConnectionRulesGridEnhanced() + saveConnectionRulesEnhanced()）。证明「重置规则」类操作本有实现，唯 resetRules 遗漏。
- 弹窗容器 :947 <div v-if="showRulesModal" class="modal-overlay">，由工具栏 :24 <button ... @click="showRulesModal=true" title="连接规则">🔗 规则</button> 打开。
- 规则网格 :964 @click="toggleRule(fromType, toType)" 提供手动逐条切换，是缺陷的绕过路径。
- typecheck 门盲区：desktop 脚本为 tsc --noEmit，不解析 .vue 的 script setup，故该未定义标识符不会被 TS2304 捕获，缺陷存活于 HEAD。

## 决定性实测（vitest 真执行）
- 探针 tests/r606b_probe.test.ts（临时，已删）：4 个 itest；命令 cd oa4rust && npx vitest run tests/r606b_probe.test.ts → EXIT 0，4 passed / 0 failed。
- 用例 1：模板绑定 resetRules() 但 script 无定义（六种定义正则全 null、script 出现 0 次、全文件出现 1 次）。
- 用例 2：从源码抽出 @click 目标，经 new Function('event', '"use strict"; resetRules()') 真执行，断言抛 ReferenceError 且消息含 'resetRules is not defined'——死绑定坐实，非静态猜测。
- 用例 3：兄弟函数 saveRules/resetConnectionRules/resetConnectionRulesEnhanced 均真定义，弹窗与路由锚点存在。
- 用例 4：次要候选 OrgDirectory 锚点存在。
- 决定性日志（gitignored 临时路径，用后即删）：<PROBE_LOG_PATH>（885 字节），字节 sha256=bcd24aca6576f8e410ffab4514c8493671f39538a14114230141e16968201c7e。

## 三分裁决（据实，不夸大不降级）
1. 判 P2：点击「重置默认」100% 必现 ReferenceError，按钮功能完全失效且用户可感知。
2. 不判 P1：不越权、不写库、不外泄数据，且不阻断流程设计器主流程；用户可经 :964 toggleRule 手动逐条恢复。
3. 不判 P3：按钮为弹窗内明确功能入口，失效直接可见，非边缘文案问题。
4. 非安全族：与 R419 DEV 门控、R606a 身份盲 IDOR 无关；纯粹前端死绑定。

## 次要观察（非主对象，仅锚点核实）
- OrgDirectoryApp.vue:50 部门树节点点击仅写 $refs.personSearch.value 与 lastAction；:96 对已加载 rows 客户端 JSON 全字段 filter().slice(0,20)，:87 deptTree.slice(0,30) 客户端截断。
- 影响：部门树点击不走服务端分页/筛选，仅过滤本地已加载数据，属局部交互局限；未做浏览器实测定量，故仅作次要观察，不单独立项、不判级。

## Fail Loud
- 未跑浏览器 GUI 真页面；以 vitest 真执行 + new Function 真调用 + 源码逐行锚点替代，未静默跳过。
- 派发父 2a6b9b5f7 与实际 HEAD fb28706c 不一致，已用 git 证明祖先关系且目标源码字节未变，据实披露。
- 测试夹具与日志均在 gitignored 临时路径并已删除，未入库；未读取或回显任何密钥/口令/真实地址。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=1437a0d066ac7628d120b66aeb7029c966d9880c9ecc7fb4ea75cb6f3271b2cf
- meta={"task":"t183","round":"R606b","kind":"verification-work","target":"apps/desktop/src/views/ProcessDesigner.vue:976 resetRules()","verdict":"P2","evidence":"vitest 4passed + new Function real exec ReferenceError","parent":"2a6b9b5f7a1fae0c7a0af2f6280471ef4de9fcd1","head":"fb28706c893991af800de4e51d0ae0207ea8dbca","logSha":"bcd24aca6576f8e410ffab4514c8493671f39538a14114230141e16968201c7e"}
- 证据日志字节 sha256=bcd24aca6576f8e410ffab4514c8493671f39538a14114230141e16968201c7e
- apps/desktop/src/views/ProcessDesigner.vue=623261c1d106e2f295b0ded417a410bfbbbbec7d1d83b549d0bb9bb3136cdb70
- apps/desktop/src/main.ts=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0
- apps/desktop/src/views/OrgDirectoryApp.vue=e4260eb944e04faa3ae30051e076888d23197179e9d4cbd9ea5e1ef8c443765f

## 修复方向
- 于 ProcessDesigner.vue <script setup> 补 resetRules()：复用 resetConnectionRules()/默认规则表重置规则状态，或直接把 :976 绑定改为已有的 resetConnectionRules()（需语义核对）；补一条「点击重置默认不抛错且规则恢复默认」的前端回归测试。
