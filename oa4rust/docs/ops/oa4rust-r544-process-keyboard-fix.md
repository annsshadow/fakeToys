# R544 H5 审批页伪按钮键盘可达修复（WCAG 2.1 SC 2.1.1 / 4.1.2）

- 轮次：R544（repair，t53），真父 5ee4a9b924b694f5bbdb8fd1c81f4b6a085edd23（台账 §534，已 push），契约 RED 基准同源 5ee4a9b92。
- 缺陷：R543 判 1 项 P2——process.vue 6 类 <view>/<text> @tap 伪按钮（3 tab / 列表项 / 3 重试 / 加载更多 / 详情关闭 mask+sheet-close / 发起流程 fab）AX role=generic name=null、无 tabindex，Tab22 次无一命中。
- 去伪：R543 已注入对照（role=button+tabindex+aria-label 后同节点 AX=button/focusable=true/name=注入值，Tab11 次）+ calendar:295 正面对照双向坐实通道正常、真实缺名。

## 一、方案取舍（Fail Loud）

与 R540/R542 一致选逐页显式补：只改 process.vue 9 处开标签，业务 handler/API/样式/他页零改；全局增强器方案已（R540）Fail Loud 排除（选择器波及十余他页越界、动态文案派生不准）。
原生 button 明确排除：:281/:284 的 button@tap.stop=act（驳回/通过）属 uni-app 编译层 uni-button 另一族观察项（R543 已列），本轮据契约零改、不混入伪按钮族，Fail Loud 披露。

## 二、写域（3 文件）

| 文件 | 变更 |
|---|---|
| oa4rust/mobile/src/pages/process/process.vue | 9 处 view/text 开标签追加 role=button + tabindex=0 + aria-label 与 @keydown.enter/.space.prevent，复用原 handler |
| oa4rust/mobile/src/pages/R544.processKeyboardA11y.contract.test.ts | 新增，13 用例源码级契约 |
| oa4rust/docs/ops/oa4rust-r544-process-keyboard-fix.md | 本文档 |

9 处：tab（:259，动态 :aria-label=t.label→switchTab）、retry-btn×3（:269 loadFirst / :292 loadMore / :311 retryDetail，aria-label=重试）、列表项 item o2-list-item（:276，动态 :aria-label=titleOf(row,i)→openDetail）、load-more（:295→loadMore）、mask（:299）与 sheet-close（:303）（均 aria-label=关闭详情→closeDetail）、fab（:333，aria-label=发起流程→goStart）。
业务零改：switchTab/openDetail/loadFirst/loadMore/retryDetail/closeDetail/goStart 函数体、processApi.* 8 个真实 API、v-model/条件渲染/样式段、pages.json、main.ts（R537 增强器）、原生 button :281/:284、他页、calendar:295 正面对照逐字未改；原 @tap 鼠标行为保留。

## 三、契约 RED → GREEN

契约 13 用例：6 核心（各伪按钮 role/tabindex/aria-label/keydown + handler 复用）+ 7 守卫（handler 函数体 / API / 原生 button / main.ts / pages.json+calendar:295 / 样式 / 原 @tap 全保留）。
- 真父 5ee4a9b92 覆盖 → pnpm test 定点 exit=1，6 failed（核心全 fail）| 7 passed（守卫全过）。
- 恢复修复态 → exit=0，13 passed。日志 r544-red.log / r544-green.log。

## 四、三 verify（全 exit 0，workdir=oa4rust/mobile）

| # | 命令 | 结果 | 日志 |
|---|---|---|---|
| V1 | pnpm test src/pages/R544.processKeyboardA11y.contract.test.ts | exit 0，13 passed | r544-green.log |
| V2 | pnpm test | exit 0，83 files / 563 tests passed（无历史契约回归） | r544-verify2.log |
| V3 | pnpm typecheck | exit 0，tsc --noEmit | r544-verify3.log |

## 五、决定性 CDP 取证（focused=true 非 root 节点 AX role=button / focusable=true / name 非空）

- runner oa4rust/test-results/r544-e2e.cjs，Playwright chromium headless，390x844 DPR2 isMobile/hasTouch，自起 uni H5（<TEST_HOST>），stub 对齐 R543 实证的真实 API 路径（/api/processplatform/assemble/surface/task/list/my/paging 等），三态（列表/错误500/空态）。
- 判据口径：getFullAXTree 中 prop(n,focused)===true 且非 RootWebArea 节点，AX role=button / focusable=true / name===预期；document.activeElement=UNI-VIEW[role=button]。未探宿主 wrapper 假阳性（R535 教训），CDP focused 在 n.properties[]。
- 结果 R544_E2E_DONE：groups=6（light/dark × 三态），namesBad=0 fatals=0 pageErrors=0。列表态探针：3 tab（待办/已办/我发起）+ 列表项（R544流程标题）+ fab（发起流程）全部 pass；错误态探针：重试 pass；Tab 序列 40 次命中 32–33 个 button。
- Enter/Space 等价动作：切 tab（第2项）→ .on 类移动（tabSwitched=true）；列表项 Enter → 详情浮层打开（专项探针 opened=true）；sheet-close focused AX name=关闭详情（sheetPass=true），Space → 浮层关闭（spaceClosed=true）；fab Enter → #/pages/process/start。
- 注入对照：移除 fab aria-label → focused name 失配（removed=true）；注入 R544-injected-control → focused AX role=button/focusable=true/name=R544-injected-control，通道有效。
- 正面对照：calendar:295 的 role=button[aria-label] → 新建日历，零回归；原生 button 驳回/通过源码零改（源码守卫钉死）。

### Fail Loud

- 主 runner 三态截图同 sha（85573ce7…）：截图在交互终态（已切到已办空态）采集，属截图时机问题；决定性判据在交互前已正确采到列表项/重试探针，非产品缺陷。补充详情态专项探针 r544-detail.cjs 独立坐实 mask/sheet-close 关闭类（独立截图 r544-detail-light.png sha=56315efc…）。
- H5 无主题切换：light/dark 截图 sha 相同，据实记录不伪造暗色差异。

## 六、证据 sha256

| 产物 | sha256 |
|---|---|
| process.vue（修复后） | 0f9b3451c65eda20a67c66d707b31607ffacb2023ebc9cb2bd7e344d1eba6e7f |
| main.ts（零改） | b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b |
| calendar.vue（正面对照零改） | 75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838 |
| pages.json（零改） | fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466 |
| R544 契约 | d93f45b1e105df403b07b18c16c17da7a3a6617bd823a59d4614153b976076db |
| r544-e2e.cjs | 941536cd5ff080fb32e940ef86281e69b7ae6f9c12aad3bc359f10541ef24d5c |
| r544-detail.cjs | aa51d811fa2c8c10e5b6f4409384be30e36a69e789dd1b942a4a7d14190b1440 |
| r544-ax-report.json | e25c19cc4e8725c7ffddd5ba7e86669012d3bf6643178c48ea5cc92d3ed42757 |
| r544-detail-report.json | 35819d784e282a37504615f170e92ce33f2c370ef7feba8ab09f78fd9976f55c |
| r544-red.log | 52b7b473ada0a057438ef191907cf794d79f3a59bc31734379830c44198cbd6f |
| r544-green.log | 4f68982c84288243ef12bfef502a75e955dcf78f9a8df7d2bdf7d128ca7989a2 |
| r544-verify2.log | e99354fc310dc521369b614f13f95e6844b2f8bb4b6f8a3b1a72696c41f17385 |
| r544-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r544-data/error/empty light=dark | 85573ce7bbd84ac56619fd151481e0888201af4b6e0ea5d477ed7b1889f9a9e8 |
| r544-detail-light.png | 56315efc7d4ca845a287de609ceb0ada24bf6c03632bc82b610cae812a6c0bb5 |

截图均 780x1687（390x844 @DPR2），目视审批页 tab/列表/空态与详情浮层渲染正常，a11y 属性不影响视觉布局。runner/日志/报告/截图位于 .gitignore 排除的 oa4rust/test-results/，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 <TEST_HOST>、端口以 <PORT> 占位，认证与列表数据为 stub 占位，0 真值入库。