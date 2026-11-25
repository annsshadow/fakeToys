# R545 H5 通讯录页伪按钮键盘可达性甄别（WCAG 2.1 SC 2.1.1 A 关联 SC 4.1.2 A）
- 轮次：R545（audit，t55），真父 `d77158c17046f91f185eecb30928b39097a34da6`（台账 §535，已 push），attempt `6f1b16ad-0566-4404-a6a0-9c07ee86ba1b`。
- 性质：只读甄别，不改源码、不提交业务代码。属 R539/R540（index）、R541/R542（message）、R543（process）已修同缺陷类的不同页面实例（先例 R509/R515/R532），维度覆盖 ≠ 实例覆盖；静态线索严禁预判，全部据真浏览器 CDP 取证。
- 环境：自起 uni H5（`http://<TEST_HOST>:<PORT>/oa4rust-h5/`，占位），Playwright chromium headless 390×844 DPR2，独立 stub 造数（认证/人员/单位/群组/详情 API）；H5 App.vue 无主题机制，light/dark 截图 sha 相同。0 真值入库。
- 判据口径：决定性问题为 `document.activeElement` 与 `Accessibility.getFullAXTree` 中属性 `focused=true` 的非 RootWebArea 节点（R535 教训：严禁探宿主 wrapper 假阳性）；并对每个候选读 DOM role/tabindex/aria-label 与 `getPartialAXTree`/`getFullAXTree` 按 backendDOMNodeId 映射比对；注入对照 + calendar:295 正面对照双向去伪。

## 一、结论（三分判定）
**可达真缺陷 1 项 P2**：contacts 通讯录页 `oa4rust/mobile/src/pages/contacts/contacts.vue` 共 10+ 处 <view>/<text> @tap 伪按钮在键盘通道完全不可达且 AX 语义不可读——
① 3 个 mode tab（:210 人员 / :211 单位 / :212 群组）；② 人员搜索错误态 retry（:221）；③ 人员列表项 body（:230）；④ 单位/群组错误态 retry（:247）；⑤ 单位列表项 body（:256）；⑥ 单位人员详情关闭（:268）；⑦ 单位人员详情 retry（:275）；⑧ 人员详情关闭（:293）；⑨ 人员详情 retry（:300）。
违反 SC 2.1.1 Keyboard（A）：功能只能鼠标/触控操作，键盘用户无路径；关联 SC 4.1.2 Name, Role, Value（A）：role/tabindex/aria 全空，AX role=generic name=null。
三分明细：上述 9 类候选均为「可达真缺陷」（造数下点击可打开详情/切换模式/重试，非死代码）；空态（:223-226/:249-252/:277-279）无 @tap 控件、不计入；原生 <button class=go/chat-btn>（:204/:234）编译为 uni-button 自定义元素属另一族观察项，按契约排除，不混入本族立案。

## 二、源码逐行锚点
- :194-207 搜索区：input:195-203 已带 aria-label=通讯录搜索关键词 + confirm-type=search + @confirm=doSearch（:200），button:204 @tap=doSearch 为原生搜索按钮。
- :209-213 modes：三个 `<view class=mode @tap=switchMode(...)`（:210/:211/:212），无 role/tabindex/aria；switchMode(:97-102) 切 mode 并对非 person 载 loadOrg。
- :215-239 人员态：error retry `<view class=retry-btn @tap=search()>`（:221）；列表项 `<view class=body @tap=openPerson(row)>`（:230）；:234 <button class=chat-btn @tap=startChat> 属 uni-button 另一族。
- :241-263 单位/群组态：error retry `<view class=retry-btn @tap=loadOrg()>`（:247）；单位项 `<view class=body @tap=mode==='unit'?openUnit(o):undefined>`（:256）。
- :265-288 单位人员详情：关闭 `<text class=detail-close @tap=closeUnitPeople>`（:268）；error retry `<view class=retry-btn @tap=openUnit(target)>`（:275）；列表项 body @tap=openPerson(p)（:283，同类复用 :230 族）。
- :290-313 人员详情：关闭 `<text class=detail-close @tap=detail=null>`（:293）；error retry `<view class=retry-btn @tap=openPerson(detail)>`（:300）。
- 业务函数：search(:40-51)/doSearch(:58-60)/loadOrg(:83-95)/switchMode(:97-102)/openPerson(:105-127)/openUnit(:140-161)/closeUnitPeople(:163-168) 均真实存在，onShow(:53-56) 经 ensureAuthenticated 后 search()，非死代码。
- main.ts installH5NativeInputNameA11y(:16-67) 只处理 input/textarea/picker 名称透传，不覆盖 @tap 伪按钮键盘语义；全局无 installA11y 键盘增强器（与 desktop main.ts 差异）。

## 三、决定性 CDP 证据矩阵（12 组 light/dark × person/unit/error/unitError/detailError/empty + 2 组详情态，全 fatals=0 pageErrors=0）
| 候选 | DOM role/tabindex/aria | partial AX | full AX | focus 后 document.activeElement | getFullAXTree focused 非 root | Tab 命中 |
|---|---|---|---|---|---|---|
| mode tab ×3（:210-212） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 30 次 0 命中 |
| 人员项 body（:230） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 人员 retry（:221，error 态命中 count=1） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 单位项 body（:256，unit 态命中） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 单位 retry（:247，error 态命中） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 详情关闭（:268/:293，详情态命中 count=1） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[]；详情态 Tab24 次 detail-close 命中=0 | 0 命中 |
| 详情 retry（:275/:300，detailError 态命中 count=1） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中 |
| 搜索按钮/聊天按钮（:204/:234，uni-button 另一族） | 全 null | role=generic name=null | role=generic name=null | BODY，isTarget=false | axFocused=[] | 0 命中（按契约排除不计缺陷） |

## 四、反预判与去伪
1. **AX 通道正常（注入对照）**：对 mode tab 运行时注入 role=button + tabindex=0 + aria-label=R545-injected-mode 后，同节点 partial AX role=button / name=R545-injected-mode，Tab 30 次命中该注入节点 10 次（双主题一致）——证明 generic/null 是源码缺属性所致，非 CDP 通道失灵。
2. **正面对照**：calendar:295 按钮 partial AX role=button / name=新建日历 可读、可聚焦，证伪「本页 AX 全不可读」的环境性辩解。
3. **非死代码**：造数点击人员项 body → personDetailOpened=true、详情关闭 → detailClosed=true；点击单位 mode → 单位列表 items=3、单位项 body → unitDetailOpened=true；error 态 retry-btn count=1 且 errorText=联系人加载失败：HTTP 500 真实呈现。故 L1 不可达抗辩不成立。
4. **等价键盘入口逐项核查**：input:200 已有 aria-label+@confirm=doSearch，仅可替代搜索按钮（:204）；mode 切换（switchMode）、详情关闭（closeUnitPeople / detail=null）、各 retry（search/loadOrg/openUnit/openPerson）、结果列表项（openPerson/openUnit）均无其他键盘可达控件或快捷入口，不构成豁免。
5. **另一族排除**：原生 <button> 在 uni-app H5 编译为 <uni-button> 自定义元素（内部无原生 button，AX role=generic，focus=BODY），属 uni-app 编译层观察项，按契约明确排除，不混入 @tap 伪按钮族；已修 index/message/process/calendar:295 实例、PC 侧已覆盖实例、3.3.8 验证码一律不重复立案。
6. **空态据实**：empty 态 mode/inputs 存在但无 @tap 候选（modes=3, items=0, retry=0, detailClose=0），不据此判缺陷；缺陷判定全部建立在 person/unit/error/unitError/detailError 态的真实命中之上。

## 五、最小修复方向（供后续 repair，业务零改）
按 R544 逐页显式补口径（全局增强器已 Fail Loud 排除，不新增全局逻辑）：为上述 9 类 <view>/<text> @tap 控件追加 role=button + tabindex=0 + 动态 aria-label（按行数据/状态生成，如「查看 {姓名} 详情」「重试加载人员」），并追加 `@keydown.enter.prevent` 与 `@keydown.space.prevent` 复用原 @tap handler；保留 @tap 不动，鼠标/触控行为零改。uni-button 编译层问题另案，不在此修复范围。

## 六、证据 SHA-256（全在 gitignored oa4rust/test-results/r545/，未入库）
- 源码：contacts.vue=`b23e6e9ff0adfab2544ec05ed5d6ab371e5329d6c24ce50c7b4f618b35a911c3`；main.ts=`b2b627d70fcf47b8c3e5273a589d16e9ecbb21a502b0e15d5dc097bacf01b96b`；pages.json=`fc21d8e8a8a2f9f80d2cf192db965ec506e894a1b358685ebf2731169e41a466`。
- 报告：r545-report.json=`041aee959f725041412d9ae474332ae001c0d79aff63cea8c82adc1ddd385180`（12 组）；r545-detail-report.json=`66d994702e176ba3e34ad5af3e00939fea392317a54e89207f9b8c325ca7947b`（2 组详情态）。
- 截图（780×1688，light/dark 同 sha 因 H5 无主题机制）：列表 person=`5085f8dc36a22f0c97c0e87e0f035aaa9099c65bc432df3441fa048a38c90210`；error=`81c2387c0e4c11519992253c22d19a908f5422234a67cd529f69f896bbb451d9`；empty=`a25561cc046451ad1167bbb967869d3d34bc5c33ac4d474c3d8086fd6274d646`；详情 person=`ad433f0f6d8811eda90774b1591782afd48103aec4129d06c7c0108f7557e619`；详情 unit=`e61b5be1548b475bc323cb2fb930de93682e5f494ecf9597b4b7b960ce2fddca`。
- 探针脚本：r545-e2e.cjs / r545-parse.cjs / r545-detail.cjs 与 runner 均在 gitignored 目录；域名/端口用 <TEST_HOST>/<PORT> 占位，0 真值。