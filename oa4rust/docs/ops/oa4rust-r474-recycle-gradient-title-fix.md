# R474 PC 回收站标题渐变遮蔽修复（RED→GREEN + 双主题真浏览器实证）

- 轮次：R474，任务 t339，attempt 29dbbb61-d116-4f9a-ade0-535f19387e4a
- 日期：2026-11-24
- 性质：真缺陷修复（P2，可达），业务零改，仅视觉样式一行
- 真父：716bb9d09215eab161b609fecbc985cf688bdcb0（HEAD 3748dda8f 仅其后追加台账 commit，RecycleApp.vue blob 286ace33a 与真父一致，已 git rev-parse 核实）
- 分支：opt1000/oa4rust-pc-ai-refactor

## 1. 缺陷根因（t337 甄别 + t338 终审双确认）

- RecycleApp.vue:7 模板 `<h1 class="ai-gradient-text">回收站</h1>`；
- :272 scoped `.ai-gradient-text{background:var(--ai-gradient-text);-webkit-background-clip:text;background-clip:text;color:transparent}` 声明渐变（特异性 0,1,0）；
- :278 `.view-header h1{font-family:Orbitron;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}` 后置规则特异性 (0,1,1) 高于 :272，实心 color 胜出覆盖 transparent。
- 结果：background-clip:text 仍在但渐变被实心遮蔽。实测父版 recycle-light color=rgb(37,99,235)、recycle-dark=rgb(0,212,255) 实心，gradVisible=false。与 NoteApp(R472) 修复前同构。

## 2. 修复方案（最小改动，仅 1 行）

- RecycleApp.vue:278 将 `color:var(--color-primary)` 改为 `color:transparent`；
- 同规则块内 font-family(Orbitron)/font-size(20px)/margin(0 0 4px)/text-shadow(0 0 15px var(--color-primary-glow)) 全部零改；
- :273/.290 按钮样式、:274/.275 危险按钮、模板、data-testid、业务方法、接口、后端、SDK、其它 view 一律零改；
- git diff --stat：仅 RecycleApp.vue 1 文件 1 insertion / 1 deletion。

## 3. 契约 RED→GREEN

- 新契约 oa4rust/frontend/apps/desktop/src/views/R474.recycleGradientTitle.contract.test.ts（5 it，readFileSync + ruleBody 精确取规则体，真断言无桩）：
  1) 标题 h1 模板仍带 ai-gradient-text 类与 data-testid 零改；
  2) .view-header h1 不得有实心 color:var(--color-primary)（核心缺陷断言）；
  3) 渐变声明块保留 background:var(--ai-gradient-text) + background-clip:text + color:transparent；
  4) 标题字体字号/间距/text-shadow 零改（守卫）；
  5) 按钮品牌渐变与原色/危险色声明零改（守卫）。
- 真父定点 RED：1 failed / 4 passed（5），EXIT=1；失败项正是核心断言，4 项守卫全过，无 ReferenceError/TypeError 桩错。日志 test-results/r474-red.log（gitignored）。
- 修复版 GREEN：5 passed / 5，EXIT=0。日志 test-results/r474-green.log（gitignored）。

## 4. 三 verify（全 exit 0，2026-11-24）

- V1 `cd oa4rust && npx vitest run frontend/apps/desktop/src/views/R474.recycleGradientTitle.contract.test.ts`：Test Files 1 passed / Tests 5 passed，EXIT=0（日志 r474-v1.log）。
- V2 `cd oa4rust && pnpm test`：321 files / 3153 tests passed，0 failed，EXIT=0（日志 r474-v2-full.log）。
- V3 `cd oa4rust && pnpm typecheck`：6 项目全 Done（含 frontend/apps/desktop），EXIT=0（日志 r474-v3-typecheck.log）。

## 5. 真浏览器实证（PC 1440x900 DPR1 headless 非物理真机，双主题 9 页）

- 构建：oa4rust 根 pnpm run build EXIT=0，入口 index-D_SIERDR.js / CSS index-COmAqEyf.css（日志 r474-build.log）。
- 部署：整目录 tar → scp → sha256 比对 → 远端 /usr/local/oa4rust 原子替换（web.incoming-r474 → web），DEPLOY_R474_PC_FULL_OK。
- E2E：test-results/r474-recycle-gradient-check.cjs（gitignored），R474_DONE checks=32 passed=32 failed=0 shots=11 errors=0。
- 回收站双主题 computedStyle：
  - light：color=rgba(0, 0, 0, 0)，bgImage=linear-gradient(120deg, rgb(67,56,202) 0%, rgb(14,116,144) 100%)，clip=text，fill=rgba(0,0,0,0)；
  - dark：color=rgba(0, 0, 0, 0)，bgImage=linear-gradient(120deg, rgb(165,180,252) 0%, rgb(103,232,249) 100%)，clip=text，fill=rgba(0,0,0,0)。
- 8 对照页（deployment/file/meeting/process/bbs/bam/dashboard/note）双主题均 color 透明 + linear-gradient + clip=text，零回归。
- text-shadow 据实处理与判据：声明层守「零改」——契约 it4 断言 text-shadow:0 0 15px var(--color-primary-glow) 父版与修复版均存；E2E computedStyle 层保留生效——light=rgba(37,99,235,0.2) 0px 0px 15px、dark=rgba(0,212,255,0.35) 0px 0px 15px（checks recycle-shadow-kept-* 全 true）；视觉层因 color:transparent + background-clip:text，阴影绘于透明文字轮廓外、不产生突兀实心光晕，截图目视标题为干净蓝紫/青渐变，与 R472 NoteApp 修复后一致，属预期。结论：text-shadow 声明零改、样式生效、视觉无副作用，满足验收。
- 0 pageerror（errors=[]）。

## 6. 截图 sha256（命令行原始值，gitignored 证据目录 test-results/r474-recycle-gradient/）

    8456a5267ac759247176081ae925fc0a4248fe6fc32637152a1f43d0e2475472  pc-recycle-light.png
    f73f7f32bfc4f021e5924b70d28a0d80e19edae1f84e613f2aa888e664d40c87  pc-recycle-dark.png
    612f84e58105254d62bde5be7e0db5d94ef7dcd4e856fd2239650ce934baba34  pc-deployment-light.png
    066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f  pc-file-light.png
    94c4840fbd266eeaae6c3287abe09f1d0e3b7e874f6e35ef8a358d73fd891b26  pc-meeting-light.png
    1cdfa352dc73baedcc374b29e31413d3ca4c4c876b068c3af0dd2ebb53762a72  pc-process-light.png
    888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105  pc-bbs-light.png
    db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7  pc-bam-light.png
    3f43ef73ae638ad6542f495108b826d8b7432493c11020cd0dbb029349d22392  pc-dashboard-light.png
    3073d4e69a41e8a9713c65923d08c8af13fb0a2b0911df0f6abc568be55bd8c0  pc-note-light.png
    803f39ce719360f5e95c925e22c1895bb4fb8669066a947f8885f56c0caed8e4  pc-bam-dark.png

截图来源如实标注：Playwright Chromium headless 非物理真机，PC 1440x900 DPR1，双主题。回收站双主题两图已 read_image 目视确认渐变生效、布局完整。

## 7. 真值纪律

- 本文所有服务器地址/账号/口令/端口均以占位符表示（<YOUR_TEST_DOMAIN>/<TEST_HOST>），0 真值入库。
- 真域名、账号、口令仅存在于 gitignored 的 test-results runner 中，运行时从受控凭据源提取注入进程环境，不回显、不入库。
- 证据目录 oa4rust/test-results/r474-recycle-gradient/ 已被 .gitignore 拦截，不随提交入库。
