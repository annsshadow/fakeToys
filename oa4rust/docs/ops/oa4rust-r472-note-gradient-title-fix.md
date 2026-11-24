# R472 PC NoteApp 标题渐变遮蔽修复

## 缺陷根因

oa4rust/frontend/apps/desktop/src/views/NoteApp.vue:8 的标题 h1 携带 ai-gradient-text 类；:182 scoped 规则声明渐变背景与 color:transparent。但 :186 `.view-header h1` 的后置实心 color:var(--color-primary) 特异性 (0,2,1) 高于 :182 的 (0,2,0)，实心色胜出覆盖 transparent，background-clip:text 仍在却因实心文字遮蔽，渐变不生效。

真父实测（t333 甄别 / t334 终审双确认）：note-light 标题 color=rgb(37,99,235) 实心、note-dark=rgb(0,212,255) 实心；对照 Dashboard/FileManager/ProcessTaskCenter 标题均 rgba(0,0,0,0)+linear-gradient 渐变正常。同页其它 ai-gradient-text 管理页不受影响，属可局部修复的单页真缺陷。

## 修复方案（最小改动）

仅将 :186 `.view-header h1` 的 color:var(--color-primary) 改为 color:transparent 一行；保留 font-family/font-size:20px/margin 与 text-shadow:0 0 15px var(--color-primary-glow)。transparent 让 :182 渐变与 background-clip:text 生效；同页更高特异性规则仍以 text-shadow 提供柔和发光。

零改纪律：模板、业务结构、接口、data-testid、按钮样式 :188 .btn-primary、:183 渐变品牌样式、其它页面、后端、SDK、构建配置一律不动。git diff 仅 NoteApp.vue 1 行（1 insertion/1 deletion）。

## 契约判据

新建 oa4rust/frontend/apps/desktop/src/views/R472.noteGradientTitle.contract.test.ts（@vitest-environment node，readFileSync 真读源码 + ruleBody 精确取规则体，5 条真实断言，无桩）：
1. 标题 h1 模板仍携带 ai-gradient-text 类（父版 pass）。
2. 核心缺陷断言：.view-header h1 规则块不得再有 color:var(--color-primary) 遮蔽渐变（父版 fail = RED）。
3. 渐变声明块保留 background/background-clip:text/color:transparent（父版 pass）。
4. 标题字体字号、间距、text-shadow 零改（父版 pass）。
5. 按钮样式零改：.btn-primary 原色值与渐变品牌样式保留（父版 pass）。

真父 149aed4d8 定点 RED：1 failed / 4 passed，失败项正是核心缺陷断言，4 条零改守卫全过，无桩错（日志见 gitignored test-results/r472-note-gradient/red2.log）。首次草稿曾因 CSS 无空格与强制高特异规则出现 3 failed，已收敛为宽松正则与最小方案，最终 RED 基线如上。修复版 GREEN：5 passed / 5。

## 三 verify（全 exit 0）

- V1 定点：`cd oa4rust && npx vitest run frontend/apps/desktop/src/views/R472.noteGradientTitle.contract.test.ts` → 1 passed file / 5 passed tests。
- V2 全量：`cd oa4rust && pnpm test` → 320 files passed / 3148 tests passed / 0 failed。
- V3 类型：`cd oa4rust && pnpm typecheck` → 6 项目 Done（含 desktop tsc --noEmit）exit 0。

## 构建与部署

`cd oa4rust && pnpm run build` exit 0，入口 index-DrKOKS-I.js。部署脚本 oa4rust/test-results/deploy-pc-r472.sh：tar 打包 frontend/dist/web → scp 到测试服 → 远端校验入口 hash → 目录原子替换，输出 DEPLOY_R472_PC_FULL_OK；远端 index.html 入口复核与本地一致。踩坑：Windows OpenSSH scp 不识别 MSYS 的 /tmp 与 /d/ 路径，临时 tar 改存仓库内 gitignored 证据目录并用相对路径传给 scp 后成功。

## 真浏览器 E2E 实证（headless 非物理真机，非真机实测）

脚本 oa4rust/test-results/r472-note-gradient-check.cjs + runner（凭据由测试服 .admin-credentials 经 SSH 提取注入环境变量，真值不入库）：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，逐页 computedStyle。终轮 R472_DONE checks=16 passed=16 failed=0 shots=2 errors=0。

- 笔记页 note-light：标题 .view-header h1.ai-gradient-text color=rgba(0,0,0,0)、backgroundImage=linear-gradient(120deg, rgb(67,56,202) 0%, rgb(14,116,144) 100%)、background-clip=text；text-shadow=rgba(37,99,235,0.2) 0px 0px 15px。
- 笔记页 note-dark：color=rgba(0,0,0,0)、backgroundImage=linear-gradient(120deg, rgb(165,180,252) 0%, rgb(103,232,249) 100%)、clip=text；text-shadow=rgba(0,212,255,0.35) 0px 0px 15px。
- 对照零回归：dashboard（.welcome-heading）、process-task-center（.admin-title.ai-gradient-text / h2.ai-gradient-text）、file（header h1.ai-gradient-text）双主题均 rgba(0,0,0,0)+linear-gradient 渐变正常；0 pageerror。

### text-shadow 视觉判据（据实）

color 改 transparent 后 text-shadow 仍按文字形状绘制。真浏览器截图目视确认：浅色与深色标题均为柔和发光，颜色与渐变主色一致，无异常轮廓、重影或色块，属正常增色，故保留 text-shadow。此为视觉实证结论，非推测。

### 截图 sha256（命令行 sha256sum，1440x900）

- note-note-light.png：3073d4e69a41e8a9713c65923d08c8af13fb0a2b0911df0f6abc568be55bd8c0
- note-note-dark.png：5b7c9fb6099896699a23f5222eaac51ce596ca7a26638f3de9a2283ee45ebf6b

证据存 gitignored 目录 oa4rust/test-results/r472-note-gradient/（report.json + 2 张 PNG + 日志 + 临时 tar），不入库。

## 真值纪律

测试域名与主机以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，文档内 0 处真值；口令、密钥、真域名、个人路径均不入库。证据脚本 runner 中的真域名仅位于 gitignored 目录。
