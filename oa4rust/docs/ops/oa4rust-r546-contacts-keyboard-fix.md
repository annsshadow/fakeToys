# R546 H5 通讯录伪按钮键盘可达修复（WCAG 2.1 SC 2.1.1 / 4.1.2）

- 轮次：R546（repair，t57），真父 033f05bb21fced929342e4d61ec4dc9c2f4641b8（台账 §536），契约 RED 基准同源。
- 缺陷：R545 判 1 项 P2——contacts.vue 9 类 <view>/<text> @tap 伪按钮（3 mode tab + 人员 retry + 人员项 body + 单位 retry + 单位项 body + 单位人员详情关闭/retry + 人员详情关闭/retry）role/tabindex/aria 全 null、AX role=generic name=null、Tab 全 BODY 无命中；键盘用户无法激活、读屏无法识别为按钮。
- 去伪：R545 注入 role=button+tabindex=0+aria-label 后同节点 AX=button/focusable=true/name=注入值且稳定进 Tab 序列；calendar:295 正面对照 role=button/name=新建日历；AX 通道正常、缺陷真实。

## 一、方案取舍（Fail Loud）

- 采用逐页显式补（与 R540/R542/R544 同口径）：contacts.vue 12 处（9 类）开标签追加 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
- 全局增强器方案已排除：会越界影响十余个他页的 .retry-btn/.mode/.body，超出写域；且动态文案需行数据上下文，全局扫描无法准确派生。
- main.ts 零改，R537 installH5NativeInputNameA11y 与 calendar:295 正面对照完整保留。
- 原生 <button class=go/chat-btn>（:204/:234，uni-app 编译为 uni-button）属另一族观察项，按 R545 契约排除、本轮零改，Fail Loud 披露。

## 二、写域（实际改动 3 文件）

| 文件 | 变更 |
|---|---|
| oa4rust/mobile/src/pages/contacts/contacts.vue | 12 处 role/tabindex/aria-label/keydown；mode tab 固定名、人员/单位项动态名（nameOf/orgNameOf）、各 retry/关闭固定文案 |
| oa4rust/mobile/src/pages/R546.contactsKeyboardA11y.contract.test.ts | 新增，11 用例源码级契约 |
| oa4rust/docs/ops/oa4rust-r546-contacts-keyboard-fix.md | 本文档 |

业务零改：search/loadOrg/switchMode/openPerson/openUnit/closeUnitPeople/startChat 函数体、8 个 orgApi/messageApi 调用、personDetailGate/unitPeopleGate 竞态逻辑、原生 button :204/:234、v-model/placeholder/@confirm、模板结构、样式段、他页、calendar:295 逐字未改。git diff 仅 12 行 a11y 属性追加。

## 三、契约 RED → GREEN

契约 11 用例：5 核心（mode tab×3 / 人员 retry+body / 单位 retry+body / 单位人员详情关闭+retry+body / 人员详情关闭+retry 的 role+tabindex+aria-label+keydown）+ 6 守卫（handler 函数体 / API+gate / 原生 uni-button / 搜索 input+@tap / main.ts / calendar:295+样式段）。
- 真父 033f05bb2 覆盖 → pnpm test 定点 exit=1，5 failed（核心全 fail）| 6 passed（守卫全过）。
- 恢复修复态 → exit=0，11 passed。日志 r546-red.log / r546-green.log。

## 四、三 verify（全 exit 0，workdir=oa4rust/mobile）

| # | 命令 | 结果 |
|---|---|---|
| V1 | pnpm test src/pages/R546.contactsKeyboardA11y.contract.test.ts | exit 0，11 passed |
| V2 | pnpm test | exit 0，84 files / 574 tests passed |
| V3 | pnpm typecheck | exit 0，tsc --noEmit |

写域外历史契约核查：design-system/ai-visual/design-tokens/r305Branded/contacts.detailRace 等均未逐字钉死本次修改的旧字面量（r305Branded:15 用正则 class="go[^"]*ai-brand-fill" 仍匹配原生 button，R542 已放宽的会话字面量与本页无关），V2 全量无历史守卫回归。

## 五、决定性 CDP 取证（focused=true 非 root 节点 AX role=button/focusable=true/name 非空）

- runner oa4rust/test-results/r546-e2e.cjs，Playwright chromium headless，390×844 DPR2 isMobile/hasTouch，自起 uni H5（<TEST_HOST>:<PORT>），stub 认证/人员/单位/群组/详情 API（0 真值）。
- 判据：getFullAXTree 中 prop(n,focused)===true 且非 RootWebArea 的节点，AX role=button/focusable=true/name===预期；document.activeElement=UNI-VIEW[role=button]。未探宿主 wrapper 假阳性（R535 教训）。
- 结果 R546_E2E_DONE：cases=12（light/dark × person/unit/error/unitError/detailError/empty）namesBad=0 fatals=0 pageErrors=0 enterPass=2 spacePass=2。
- 列表态 4 项 probe（人员/单位/群组/人员项 body）全 pass，AX name=预期（人员/单位/群组/查看 R546person0 详情）；单位态先切 tab 后 body name=查看 R546unit0 人员；error 态 retry name=重试加载联系人；unitError 态 retry name=重试加载单位。
- Tab 可达：Tab 序列 30 次命中 21 个 role=button[aria-label] 节点。
- Enter 等价导航：聚焦人员项 body 按 Enter → 打开人员详情弹层（detailOpen=true, title=R546person0）。
- Space 等价导航：聚焦单位 tab 按 Space → 切到单位态（unitOn=true, items=2）。
- 注入对照：移除 mode[0] aria-label → focused name 失配；注入 R546-injected-control → role=button/focusable=true/name=R546-injected-control，证明通道有效。
- 正面对照：calendar:295 [role=button][aria-label] → role=button/aria-label=新建日历，零回归。

### Fail Loud

- 原生 uni-button（:204/:234）编译层 AX role=generic，属 uni-app 框架观察项，按 R545 契约排除、不在本族修复范围；本轮业务零改。
- E2E 用自起 H5 服务端口 <PORT>（默认回环端口被陈旧服务占用时重起顺延），runner 默认指向实际服务端口；域名/端口均占位不入库。
- H5 无主题切换：light/dark 各组截图同 sha，据实记录、不伪造暗色差异。

## 六、证据 sha256

| 产物 | sha256 |
|---|---|
| contacts.vue（修复后） | 9230722b0002482569bded5e552abd1f8e67bb742969e9d61ade92f71dbfe665 |
| R546 契约 | 0a2601d8a097dd4de94d2d4fca5ee1977002c9fc5664494a474ab989642fb370 |
| r546-e2e.cjs | a405bd0ae66dd35a03d4a2d390afc405e339fbaa17f4b1f103537cd2ab13f75b |
| r546-ax-report.json | c956e836b1ba2bb06177a7b040e997e37fd22eb3d9281b30770ccd9edc8e5d45 |
| r546-red.log | af0b76b73a117498047a2e58636920ca09506fabe67252e1f173518edcc3e1a8 |
| r546-green.log | 5ea9264c72988360c78c8eb25934324830e9d9a15efd40edc3e28844c9811f5b |
| r546-verify2.log | 559f863fd36ebee3e3f05e0e77ad0cee8c733da98f79b7711b69de0976f0f8ca |
| r546-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r546-person-light=dark.png | 9ed39ccac1a5fc6e31c9221aab378656967395fc3fee3c495d4b6c524e1ae94e |
| r546-unit-light=dark.png | 9c7497eed0a0b20f9b19789d66d0fa624dd70f9b46cfdc22a360395af16a1016 |
| r546-error-light=dark.png | 100476c5c200f8fc5bb0a05afcb953d791997a91e8aaaa7ca4c98eeb6375a677 |
| r546-unitError-light.png | 0d7cda9267aa09f78332cb7bf0e07830a3954e4ad95afa76d76d4e90f5725b99 |
| r546-detailError-light.png | 987291b2a219848a9f3b19d4ccc86dff4f1451d441bb67335a42c7ec9aa31a68 |
| r546-empty-light.png | 2faa819a58075d720ad7e5edabe91172fc18da89f40196a825af0f4d23f1e2be |

截图 780×1687（390×844 @DPR2），person/unit/error 三态 light 图目视正常（搜索框/mode tab/列表/详情/重试按钮完整渲染，a11y 属性不影响视觉）。runner/日志/报告/截图位于 .gitignore 排除的 oa4rust/test-results/，不入库。

## 七、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 <TEST_HOST>:<PORT> 占位，认证与列表数据为 stub 占位，0 真值入库。