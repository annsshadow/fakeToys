# R590 UsersView 最近战报结果列中英混排一致性修复

- 任务：t141 / R590（implementation）
- 子项目：laiyipao/admin（Vue3 + Element Plus + Vite）
- 真父提交：f8438565078d446652e3a968d0662f0012bdb6c7
- 写域：精确 4 文件（含队长裁决 A 授权的既有测试断言更新一处）
- 脱敏级别：0 真值（端口统一占位 <TEST_HOST>:<PORT>）

## 1. 缺陷背景

R589 真浏览器甄别坐实：laiyipao/admin/src/views/UsersView.vue 玩家详情抽屉「最近战报」结果列（:682）为原生直出列：

    <el-table-column prop="result" label="结果" width="70" />

status/result 英文枚举 win/lose 直接呈现在运营后台界面，与同页其余中文化字段（关卡/分数/击杀/漏怪/时间）及同字段其他页面口径不一致，属中英混排（P2 一致性缺陷）。

## 2. 口径去伪（三方并存，据实披露）

同一 result 字段在代码库存在三套中文口径，R589 真浏览器均已坐实：

- 玩家端 miniapp/src/pages/me/me.vue:381：win→胜利 / lose→失败
- 管理端 BattlesView.vue 列表 :392-398：win→胜 / lose→负（el-tag）
- 管理端 BattlesView.vue 详情 :472-476：win→通关 / lose→失败
- 管理端 me.vue:728（事件流）：win→胜 / lose→负

队长裁决指定对齐 admin 列表口径：win→胜 / lose→负；value/result 英文枚举（win/lose）原样保留在请求与数据中，仅渲染层映射为中文。本修复严格按此口径实现，其余两处口径零改并在此据实披露。

## 3. 修复内容（写域精确 4 文件）

- M laiyipao/admin/src/views/UsersView.vue（:682-684）：单行直出列改为带 template 的中文映射列：

      <el-table-column label="结果" width="70">
        <template #default="{ row }">{{ row.result === 'win' ? '胜' : '负' }}</template>
      </el-table-column>

  零改 handler / state / import / 样式 / 其他列。

- M laiyipao/admin/tests/UsersView.test.ts（:715）：既有断言 expect(text).toContain('win') → toContain('胜')。此为队长裁决 A 授权纳入写域：该断言编码的正是 R589 要消除的英文直出缺陷行为（缺陷被测试锁定），属行为变更的必然配套断言更新；仅改这一处字符串，测试文件其余断言/用例零改、不增删用例。理由：维持 3 文件写域将使代码库处于测试失败损坏态，违反 Fail Loud 与「测试验证意图」（AGENTS.md）。

- A laiyipao/admin/tests/R590.consistency.contract.test.ts：8 用例（3 核心 + 5 守卫），纯源码字符串断言（node:path resolve(process.cwd(), 相对路径) 读源码）。
  - 核心 1：结果列 template 中文映射 win→胜 / lose→负存在
  - 核心 2：英文直出 prop="result" 已移除
  - 核心 3：中文口径与 BattlesView 列表 win→胜 / lose→负一致
  - 守卫 4：fetchUsers 调用块内无 status 参数（筛选请求/后端枚举零改）
  - 守卫 5：handler / state / 序号守卫 / asRow 零改
  - 守卫 6：其他列（id/level_id/score/kills/leaked/created_at 经 fmtTime）零改
  - 守卫 7：他页（BattlesView 详情通关/失败、me.vue:381 胜利/失败、R588 EconomyView orderStatusText）零改、无 MutationObserver、recent_battles 表恰 1
  - 守卫 8：测试文件含 expect(text).toContain('胜') 且不含 win（钉死裁决 A 断言）

- A 本文档 laiyipao/docs/ops/laiyipao-r590-usersview-result-fix.md（脱敏、不含自身 sha）

方案取舍：采用逐页显式 template 最小改（方案 B）。全局增强器 / MutationObserver 方案越界 Fail Loud 排除。

## 4. 契约 RED → GREEN（本人独立 replay）

- GREEN（修复版）：EXIT 0，8 passed。
- RED（cp 真父备份覆盖 UsersView.vue）：EXIT 1，3 failed（核心 1/2/3）| 5 passed（守卫 4-8），符合「核心 fail、守卫 pass」纪律。
- 恢复修复版：sha 复核一致，GREEN EXIT 0，8 passed。
- 契约 RED 由反向覆盖真父备份实现（未另建脚本），据实披露。HEAD 测试阶段未变 f8438565，AB=0。

## 5. 三 verify（workdir laiyipao/admin，字面命令，全 exit 0）

- V1：npx vitest run tests/R590.consistency.contract.test.ts — EXIT 0，8 passed。
- V2：npm run test — EXIT 0，39 files passed / 513 tests passed，无 skip。
- V3：npm run type-check（vue-tsc --noEmit）— EXIT 0，无错误。

## 6. 真浏览器 E2E 决定性判据（Playwright + CDP）

自起 admin Vite（端口 <TEST_HOST>:<PORT>，实测 HTTP 200），Playwright chromium headless，视口 1440x900 DPR1。stub ctx.route('**/api/v1/**')：
- /admin/me → 管理员身份（过路由守卫 confirmTokenValid）
- /admin/users → 列表一项
- /admin/users/7 → 详情，recent_battles 造 win 与 lose 两条，坐实胜/负双向映射
- /admin/battles → 正面对照两项

token 经 localStorage 注入；hash 路由 /laiyipao-admin/#/admin/users → 点「详情」打开抽屉 → 读最近战报表结果列。

判据（cases=1，fatals=[]，pe=[]）：
- resultCells = ['胜','负']，winCN=true，loseCN=true，noEN=true，cellsAreCNOnly=true
- battleRowsCount=2，drawerHasResultHeader=true（抽屉内确有「结果」列表头）
- battlesResultCells = ['胜','负']，battlesCN=true（BattlesView 正面对照同字段同口径）
- 0 pageerror / 0 fatal；其他列与交互零回归；截图目视名实相符
- URL 在报告与 runner 中统一占位 <TEST_HOST>:<PORT>；runnerSha = sha256(JSON.stringify(meta)) 紧凑 JSON，与报告文件字节 sha256sum 分列

## 7. sha256 证据表（A 组 runner 紧凑 JSON；B 组文件字节 sha256sum）

A 组：
- runnerSha（R590_DONE meta 紧凑 JSON）= dfe350fda9ad4c46c8763ca43a5248da3ed7f6c8bc6eb8c74e742e304d7ab652

B 组（文件字节 sha256sum）：
- laiyipao/admin/src/views/UsersView.vue = 6aa01bb9ddcbec731eaa73d3b9fc0465db67dfd65835e421ea9e9061a998e997
- laiyipao/admin/tests/UsersView.test.ts = 641f2acef67607b71d013538fee77bcc0048f33798ee1526802e8ab5fe8cdb9e
- laiyipao/admin/tests/R590.consistency.contract.test.ts = 450cf9a99a4a08ca0f416c2e392599ec885682ed83c7fae109be8d95eb5110dc
- oa4rust/test-results/r590/r590-e2e.cjs = a9fc4c24d577cb255147968b2e1d6e1a5e9c86b712fc73dcc3f2ac0eedd0d092
- oa4rust/test-results/r590/r590-report.json = b987202b3bc38490c8137794fc9e7795c2d80f6a03f3d4cbb34833766797c710
- oa4rust/test-results/r590/r590-usersview-detail.png = 4f7f60612c7ca8cfee0df267deddda48ecb42ecda018f123f597c6219e176f93
- oa4rust/test-results/r590/r590-battles-positive.png = b3f537a0fc13a1a88464ab19748e93263699455b01fa8988bb9ae9016d9cdd1e
- /tmp/r590-v1b.log = bb49da0b27da99d6655957229a29151c1830ed943dd6881e858f928e75d19736
- /tmp/r590-v2b.log = 1a312e586fce4ac55c0f32269c5059e634c585a501b90d2e2ae5a2c706a5d81b
- /tmp/r590-v3b.log = 84155fa5bcb6a3e9b085eb417db166c6b194e4d6b6e5b81b78485727b5138859
- /tmp/r590-red2.log = 1fa38ab70fee85103cdd16bb6d8107739c889ee28d0c66eb4755def09e9b994e
- /tmp/r590-green3.log = 47e16f94bda028d08d0739b5704f3b1571f63c80ddb380f73874fdebee5491e4
- /tmp/r590-e2e.log = 5f41e9ec5de0681c982840b3c4f05d61f18b95c0bdf3f2e08931dc39990bb184

（本文档不含自身 sha，避免写入后 sha 变动的自引用悖论。）

## 8. 脱敏自检

文档与证据中：无密钥/token/口令/私钥、无 Authorization/Cookie 头、无真实域名/IP/主机名/SSH 端口、无个人环境信息。所有端口/主机统一占位为 <TEST_HOST>:<PORT>。E2E token 为测试桩内临时值，不入证据文件（runner 中为 R590-test-token 桩标识，仅 localStorage 注入，非真实凭据）。LEAKSCAN 0 真值命中。

## 9. 结论

R590 按队长裁决 A 完成：UsersView 最近战报结果列中文化（win→胜/lose→负），既有测试断言同步更新一处，契约 RED→GREEN，三 verify 全绿，真浏览器 E2E 决定性判据全绿且 0 pageerror，同字段三页口径据实披露并钉死守卫，脱敏 0 真值。
