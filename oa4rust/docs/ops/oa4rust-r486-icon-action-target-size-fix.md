# R486 FileManager 图标按钮目标尺寸不足修复（WCAG 2.5.8 AA）

## 1. 概况

- 轮次：R486（台账 §481 据实立项，真父提交 `321ae330e7edca22e25226fbcce6e92430f69b03`）。
- 缺陷来源：R485 甄别（commit `a118244f202a4547513eab6f15188d57994be5dd`）PC 1440×900 DPR1 双主题真浏览器几何实测判定 1 项 P2。
- 缺陷：FileManager.vue:61-64/85/124/137 共 7 处 `.icon-action` 按钮（ℹ详情/⬇下载/↗分享/🗑删除/可清理文件🗑/两处关闭✕）实测 11.30–17.34px 宽 × 21px 高，相邻中心距 13–16px，尺寸与间距双判据均不满足 WCAG 2.5.8 AA 24px。
- 修复：在 FileManager.vue scoped style 末尾新增 `.icon-action{min-width:24px;min-height:24px}` 一行，业务属性与视觉语义零改。
- 测试服：受控测试环境（域名以 `<YOUR_TEST_DOMAIN>`、主机以 `<TEST_HOST>` 占位）；凭据运行时从受控凭据源提取注入，真值不入库。

## 2. 根因与修复方案

- CDP CSS.getMatchedStylesForNode 坐实：`button` 等 UA 默认规则（padding-block 1px、padding-inline 6px、border 2px）+ `*{padding:0}` 命中，无任何自定义 min-width/min-height；尺寸完全由单个符号字符 + UA 内边距决定，窄至 11.30px。
- 修复选择：新增最小点击区域规则 `.icon-action{min-width:24px;min-height:24px}`，同时覆盖 7 处图标按钮；24px 见方后尺寸判据达标，相邻中心距随之 ≥24px，不再依赖间距豁免。
- 硬纪律：仅新增 1 行 CSS；`@click`/`@click.stop`/`title`/`aria-label`/`:disabled`/可见符号（ℹ⬇↗🗑✕）/class/data-testid/业务处理器零改；scoped 布局与主题零改。

## 3. 契约 RED→GREEN

- 契约：`oa4rust/frontend/apps/desktop/src/views/R486.iconActionTargetSize.contract.test.ts`（源码字符串断言，readFileSync + `ruleBody` 花括号配对提取规则体）。
- 覆盖 5 项：① `.icon-action` 规则块含 min-width:24px + min-height:24px（核心缺陷断言）②文件行 4 按钮 @click.stop/title/符号零改 ③可清理文件删除按钮零改 ④两处关闭按钮 aria-label/处理器零改 ⑤ 7 处按钮数量 + col-actions + data-testid 守卫。
- 真父 321ae330e 定点 RED：Tests 1 failed | 4 passed（5），失败正是核心尺寸断言（`必须新增 .icon-action 尺寸规则块: expected 空串 not to be 空串`），4 项业务零改守卫通过证明非桩错。
- 修复版 GREEN：Tests 5 passed（5）。

## 4. 三 verify（全 exit 0）

- V1 定点契约：`cd oa4rust && npx vitest run frontend/apps/desktop/src/views/R486.iconActionTargetSize.contract.test.ts` → 1 passed (5 tests)，exit 0。
- V2 全量测试：`cd oa4rust && pnpm test` → 324 files / 3168 tests passed，0 failed，exit 0。
- V3 类型检查：`cd oa4rust && pnpm typecheck` → 6 项目全 Done（含 frontend/apps/desktop），exit 0。
- 构建：`cd oa4rust && pnpm run build` exit 0，入口 `index-BJ1IjvFz.js`。

## 5. 部署与真浏览器 E2E

- 部署脚本 `oa4rust/test-results/deploy-pc-r486.sh`：本地 tar -czf 打包 → scp → 同一归档 sha256 比对（`SHA_OK=048b274153a6e4f86892a4a056bce4e0cbb0cfb2621ec8f60e6746662f1f9ce1`）→ 远端解压到 web.incoming-r486 → grep 入口 `index-BJ1IjvFz.js` → 原子替换 → `DEPLOY_R486_PC_FULL_OK`。
- E2E：`oa4rust/test-results/r486-icon-action-check.cjs` + runner `r486-icon-action-run.sh`（gitignored），Playwright Chromium headless 非物理真机 PC 1440×900 DPR1，light/dark 双主题，CDP getBoundingClientRect + getComputedStyle CSS 像素。
- 终轮：`R486_DONE checks=12 passed=12 failed=[] shots=2 errors=0`，0 pageerror。
- 几何实测：每个 `.icon-action` 宽高均 24.00×24.00px（minWidth/minHeight 均 24px），相邻中心距 24.00px（≥24px），尺寸判据达标，不再依赖间距豁免；双主题一致。
- 运行态说明：测试服文件列表有两行历史文件（每行 4 个按钮），实测 `.icon-action` 数量为 8 ≥ 模板 7 处；E2E 断言据实写为 `>=7`，并注明额外为运行态数据驱动实例，非模板新增。
- 对照零回归：attendance/bbs 页按钮可见，0 pageerror。
- 来源如实标注：截图来自 Playwright Chromium headless 非物理真机，未冒充物理真机。

## 6. 证据与 sha256（命令行 sha256sum 原始值）

- pc-file-icon-light-r486.png 6d2230edbcdb7b4b62d40935be08903f9ae858640ce956c982d366636e21942e
- pc-file-icon-dark-r486.png 54bb7cfec34f1e0e3e7eb683babd934fc965b0b6cd486dfb48a6e3fd41c12ff1
- report.json dd7cd1f1afb7a291bbd8e016d0b533b7e5111db6388a5b6f6f7a3fe1e00b9d2e
- 证据目录（gitignored）：`oa4rust/test-results/r486-icon-action-target-size/`（2 PNG + report.json + 脚本）。

## 7. 真值纪律

- 测试域名/IP/口令/SSH 端口/个人路径等真值一律不入库，域名用 `<YOUR_TEST_DOMAIN>`、主机用 `<TEST_HOST>` 占位。
- 凭据在 runner 运行时从受控凭据源提取后注入进程环境，真值仅存在于 gitignored 脚本，不回显、不入库。
- 源码 diff 仅 1 行 CSS；业务属性/处理器/主题/布局/其它 view 零改。
