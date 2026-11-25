# R548 H5 浏览页伪按钮键盘可达修复（WCAG 2.1 SC 2.1.1 / 4.1.2）

- 轮次：R548（repair，t61），真父 0b5297d1dfa8b3db89aac4d747312aa106367fac（台账 §538），契约 RED 基准同源。
- 缺陷：R547 判 1 项 P2——browse.vue 7 类 <view @tap> 伪按钮（3 页签 + 2 retry + 空态刷新 + 去搜索 + 列表卡片 + 详情关闭）role/tabindex/aria 全 null、AX role=generic name=null、Tab 无命中；键盘用户无法激活、读屏无法识别为按钮。
- 去伪：R547 注入 role=button+tabindex=0+aria-label 后同节点 AX=button/focusable=true/name=注入值且稳定进 Tab；calendar:295 正面对照 role=button/name=新建日历；缺陷真实。

## 一、方案取舍（Fail Loud）

- 采用逐页显式补（与 R540/R542/R544/R546 同口径）：browse.vue 7 处开标签追加 role=button + tabindex=0 + 动态/固定 aria-label + @keydown.enter/.space.prevent 复用原 handler。
- 全局增强器越界（影响十余他页的 .retry-btn/.card/.tab）、动态文案需行数据上下文，已 Fail Loud 排除。
- main.ts/R537、calendar:295 正面对照、业务函数/API/样式/他页零改。

## 二、写域（实际改动 3 文件）

| 文件 | 变更 |
|---|---|
| oa4rust/mobile/src/pages/browse/browse.vue | 7 处 a11y 属性：页签动态 label→switchTab、retry/空态刷新固定文案→load()、去搜索→goSearch、卡片动态名→open(r)、详情关闭→detail=null |
| oa4rust/mobile/src/pages/R548.browseKeyboardA11y.contract.test.ts | 新增，9 用例源码级契约 |
| oa4rust/docs/ops/oa4rust-r548-browse-keyboard-fix.md | 本文档 |

业务零改：load/switchTab/open/goSearch 函数体、portalApi/queryviewApi/cmsApi 调用、listGate/detailGate、样式段、他页、calendar:295 逐字未改，原 @tap 保留。git diff 仅 7 行 a11y 属性追加。

## 三、契约 RED → GREEN

契约 9 用例：5 核心（7 类控件 role/tabindex/aria-label/keydown）+ 4 守卫（handler+API / 原 @tap / main.ts / calendar:295+样式段）。回调参数显式类型避免 TS7006。锚点用 aria-label 文案而非 @tap+文案邻接（修复后 @tap 与文案间插入 keydown 致旧锚点失效）。
- 真父 0b5297d1d 覆盖 → exit=1，5 failed（核心）| 4 passed（守卫），r548-red.log。
- 恢复修复态 → exit=0，9 passed，r548-green.log。

## 四、三 verify（全 exit 0，workdir=oa4rust/mobile）

| V1 | pnpm test src/pages/R548.browseKeyboardA11y.contract.test.ts | 9 passed |
| V2 | pnpm test | 85 files / 583 tests passed（无历史守卫回归） |
| V3 | pnpm typecheck | tsc --noEmit 无错 |

## 五、决定性 CDP 取证（focused=true 非 root 节点 AX role=button/focusable/name 非空）

- runner oa4rust/test-results/r548-e2e.cjs，Playwright chromium headless，390×844 DPR2 isMobile/hasTouch，自起 uni H5（<TEST_HOST>:<PORT>），stub portal/queryview/cms API（0 真值）。
- 判据：getFullAXTree 中 prop(n,focused)===true 非 RootWebArea 节点，AX role=button/focusable=true/name===预期；document.activeElement=UNI-VIEW[role=button]；未探宿主 wrapper 假阳性（R535 教训）。
- 结果 R548_E2E_DONE：cases=10（light/dark × withData/error/empty/queryError/cmsError）namesBad=0 fatals=0 pageErrors=0 enterPass=2 spacePass=2。
- 全态页签 name=门户/查询/栏目；withData 卡片 name=查看 R548portal0 详情；error/queryError/cmsError retry name=重新加载浏览数据；empty 刷新 name=刷新浏览数据、去搜索 name=去搜索。
- Tab 序列命中 24-26 个 role=button[aria-label] 节点。
- Enter 聚焦卡片 → 打开详情弹层；Space 聚焦查询页签 → 切到查询态。
- 注入对照：移除页签 aria-label → name 失配；注入 R548-injected-tab → role=button/focusable=true/name=R548-injected-tab；正面对照 calendar:295 新建日历，零回归。

### Fail Loud

- 默认回环端口被陈旧服务占用（No match found for location），实际取证端口顺延，runner 指向实际服务端口；域名/端口占位不入库。
- H5 无主题切换：light/dark 同 sha，据实记录不伪造暗色差异。

## 六、证据 sha256

| 产物 | sha256 |
|---|---|
| browse.vue（修复后） | 94122c510b7899c9a81da1a147e2d0e9dca9874bf05a014ce01fc0d5fc14bed8 |
| R548 契约 | 9405b8003fc658f87b8bd0bc82a32130be9f5f01d95feb258d657d1eab5802f4 |
| r548-e2e.cjs | 325dae90af4d2a5738b75fa3406fe437b9beafd9a7de0e2904e7e228d3f49dc6 |
| r548-ax-report.json | e4d8a884270ea24a720b219e804f116d25472bb6a234cccb16b0a8bb595a788e |
| r548-red.log | 018caa64f0da023f4cdb4fd630bc14dae57ead759966d9516a544b2bfc8b394a |
| r548-green.log | 109be4623c9e9d4d10e1a5c6d86ce92e0bc3b655e47548338b4491ae07e81f39 |
| r548-verify2.log | a7787c8d954487d9194ff6d6acc1e0c4d5eaeb227a33e233bfab8cf84ff2dc40 |
| r548-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r548-withData-light=dark.png | 007e24f60c610980fbd195d229c8ed5f2e848c256b3304331dbca1ea98f41a3a |
| r548-error-light=dark.png | 53c47d24230d1ee946337264405fa002b69982d4a57fafe3cc3e1c6511bffb95 |
| r548-empty-light=dark.png | 2e0007841343eb7f29609b7704b6805c6073d1af1868679a2d6b86e3d3c81906 |
| r548-queryError-light.png | 5292f27eab0ba92fc496f00a34e28800e713b1250d08cd47f4fffa8e9f13437e |
| r548-cmsError-light.png | 71e08071ecde6b8ce8a888099d3434bd904b1f09c6191d011bfbae8cd2897934 |

截图 780×1687（390×844 @DPR2），withData/error/empty 三态 light 图目视正常（页签/列表/错误/空态入口完整渲染，a11y 属性不影响视觉）。证据均在 .gitignore 排除的 oa4rust/test-results/，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 <TEST_HOST>:<PORT> 占位，0 真值入库。