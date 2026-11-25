# R501 WCAG 2.5.3 Label in Name 两项 P2 修复报告

> 唯一写域：frontend/apps/desktop/src/views/BBSForum.vue、frontend/apps/desktop/src/views/ProgramCenterApp.vue、frontend/apps/desktop/src/views/R501.labelInNameFix.contract.test.ts、docs/ops/oa4rust-r501-label-in-name-fix.md。
> 证据 gitignored：test-results/r501-label-in-name/、test-results/r501-*.log。测试服域名/主机占位 <YOUR_TEST_DOMAIN> / <TEST_HOST>，真值仅存于 gitignored runner 环境变量，不入库。

## 1. 缺陷依据与根因

- R500 甄别（commit 69aba2b8，文档 docs/ops/oa4rust-r500-label-in-name-audit.md）经真浏览器 CDP Accessibility.getPartialAXTree 判定可达真缺陷 2 项 P2：
- D1：BBSForum.vue:111 发帖浮层 section role=dialog aria-label="发帖区"，可见 h2（:113）为「发起新主题」。双主题 computed axName="发帖区" 不含可见文本，语音控制用户按所见标题无法匹配该 dialog。
- D2：ProgramCenterApp.vue:384 对象详情抽屉 role=dialog aria-label="对象详情"，可见 strong（:386）为动态「类型 · 名称」。真实数据下 computed axName="对象详情" 不含可见文本，双主题一致。
- 边界：两处均为 dialog 容器命名与内部标题不一致，浮层内交互控件自身名称与可见文本一致，故记 P2 不升 P1。

## 2. 修复方案（最小改动）

- D1：BBSForum.vue:111 仅把 aria-label="发帖区" 改为 aria-label="发起新主题 · 发帖区"，把可见标题文本纳入可访问名称；保留"发帖区"语义。
- D2：ProgramCenterApp.vue:384 仅把静态 aria-label="对象详情" 改为动态绑定 :aria-label="(selectedDetail.type || '') + ' · ' + (selectedDetail.name || '') + ' 详情'"，使 computed axName 随所见 strong 标题变化。
- 硬纪律：只改两处 dialog 容器 aria-label；可见 h2/strong 文本、@click/@submit/@click.self 处理器、v-if 浮层结构、data-testid、class、样式全部零改；不改 main.ts / utils。
- diff 实证：两源码各 1 行改动（共 2 insertions / 2 deletions），无外溢。

## 3. 契约 RED → GREEN

- 新契约 R501.labelInNameFix.contract.test.ts（5 it）：2 核心断言（D1 名称含发起新主题 / D2 动态名称含 type·name）+ 3 业务零改守卫（可见文本与 data-testid/@click/浮层结构保留、抽屉结构与关闭按钮保留、role=dialog 数量与业务函数声明零改）。
- 真父 69aba2b8 定点 RED：Tests 2 failed | 3 passed（5），EXIT=1；失败正是 D1/D2 两条核心断言，3 条守卫在父版全 pass，无桩错（日志 test-results/r501-red-clean.log）。
- 修复版 GREEN：Tests 5 passed（5），EXIT=0（日志 test-results/r501-v1.log）。

## 4. 三 verify（全 exit 0）

| 命令 | 结果 | 日志 |
| --- | --- | --- |
| npx vitest run frontend/apps/desktop/src/views/R501.labelInNameFix.contract.test.ts | 5/5 passed，exit 0 | r501-v1.log |
| pnpm test | 329 files / 3193 tests passed，0 failed，exit 0 | r501-v2-full.log |
| pnpm typecheck | 6 项目全 Done 含 frontend/apps/desktop，exit 0 | r501-v3-typecheck.log |

## 5. 真浏览器 E2E（1440x900 DPR1 headless，非物理真机）

- 构建 pnpm run build exit 0；入口 index-CncdGAiv.js / index-COmAqEyf.css。经 test-results/deploy-pc-r501.sh 本地 tar.gz → scp → 同一归档 sha256 比对 → 远端解压 web.incoming-r501 → grep 入口 → mv 原子替换；SHA_OK=84dd7b9acd483f512edee2cce82a7ccbb31572dd2b6dc1c584ac6ac4db32e7e9，DEPLOY_R501_PC_FULL_OK。
- E2E：test-results/r501-label-check.cjs + r501-label-run.sh（gitignored；SSH 内联 python 从远端凭据文件提取账号口令注入 OA_TEST_USER/OA_TEST_PASS，OA_BASE_URL 真值仅 runner 持有）。Playwright Chromium headless，light/dark 双主题，复用单 CDP Session（DOM.enable + Accessibility.enable），getPartialAXTree 取 role.value / name.value / ignored。
- 首轮 7 checks / 5 passed，2 fatal：发帖浮层未关闭遮挡背景「新建版块」按钮（脚本交互顺序问题，非代码缺陷；日志 r501-e2e.log 如实保留）。修正为截图后先点「关闭发帖区」再测对照。终轮 R501_DONE checks=15 passed=15 failed=[] shots=4 errs=0（日志 r501-e2e2.log）。
- 核心实证：发帖浮层双主题 axName="发起新主题 · 发帖区"，axIgnored=false，可见 h2=「发起新主题」包含关系成立；对象详情抽屉双主题 axName="脚本 · script-… 详情"（动态值文档截断），axIgnored=false，完整包含可见 strong 标题。
- 对照零回归：版块新建 dialog axName="新建版块"、版块详情 dialog axName="版块详情"、搜索按钮可见文本与 aria-label 均为「搜索」，双主题一致；0 pageerror。

## 6. 截图与 AXTree 证据 SHA-256（命令行 sha256sum，gitignored）

| 文件（1440x900 headless 非物理真机） | SHA-256 |
| --- | --- |
| pc-bbs-compose-light-r501.png | b0767b03647892d760fbd50e9395c81d0f3914337ec21f4b458c95934551bfb5 |
| pc-bbs-compose-dark-r501.png | 3904eb2117dd2b78904d07e23a8682962a18632697bd1a3803313590a7238b78 |
| pc-program-detail-light-r501.png | ed9991369aca40fe35bea79de043cdee6b11a026af029ec825bd630d99f6eb19 |
| pc-program-detail-dark-r501.png | 92f88551ba1f17eabaf5f88d84ac2a310f0d27cbfe5974aa3f3be70e3d06a894 |
| report.json | ed257b9957d79574160050abec8cadc8a095cb323a7d49c2abf6d912b3c2ae34 |

- 截图来源：本地测试服真浏览器 Playwright Chromium headless（非物理真机），非用户设备截图；目视核验发帖浮层与详情抽屉标题完整、业务样式正常。

## 7. 脱敏与边界

- 测试服域名/主机一律占位 <YOUR_TEST_DOMAIN> / <TEST_HOST>；凭据只在 gitignored runner 运行时从远端凭据文件提取，不入库、不回显。
- 动态脚本名在文档截断为「脚本 · script-…」，原始值仅存于 gitignored report.json；未写入任何入库文件。
- 全库 role=dialog 共 5 处已由 R500 甄别全部实测（BBSForum:111/145/189/213 + ProgramCenterApp:384），本轮修其中 2 项 P2；主题详情 C2 真实数据不可达，不预判、不在本轮范围。
- 至少 1000 轮迭代中的一环，非终态。
