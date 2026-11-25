# R500 WCAG 2.5.3 Label in Name 标签名称一致性甄别报告

> 唯一写域：docs/ops/oa4rust-r500-label-in-name-audit.md；证据 gitignored test-results/r500-*。
> 本轮只读甄别，不改任何业务代码；修复建议供后续轮次执行。

## 1. 判据与反预判纪律

- WCAG 2.5.3：带可见文本标签的 UI 组件，其可访问名称（accessible name）应包含该可见文本，使语音控制用户可通过可见文本匹配并激活。
- 反预判铁律：严禁仅凭源码 aria-label 与可见文本字面不同即判缺陷；必须真浏览器打开浮层（确认非 R466 式 early-skeleton 名实不符），用 CDP Accessibility.getPartialAXTree 坐实 computed axName、axIgnored 与可见文本的实际包含关系。
- 三分判定：可达真缺陷（可见文本确不在 computed axName 中，且影响语音控制匹配）/不可达 L1（真实数据下浮层打不开，无法实证）/正常规范预期（名称与可见文本一致，或等价命名）。
- 次要候选 C2（BBSForum:145 主题详情）、C3（ProgramCenterApp:384 对象详情）为动态数据，必须真实数据下测，静态片段不预判。

## 2. 基线与测试方法

- 分支 opt1000/oa4rust-pc-ai-refactor；真父 commit 3090ca7f3（台账 §495 R499 收官 + R500 据实立项）；HEAD 领先 UP 1（台账提交），PORCELAIN 空。
- 测试服经 SSH 注入凭据（仅 runner 持有，真值不入库），OA_BASE_URL 文档内以 <YOUR_TEST_DOMAIN> 占位。
- 环境：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题（localStorage oa4rust_theme + data-theme，切换后等待 1400ms 避开 transition）。
- 取证：CDP DOM.getDocument + DOM.querySelectorAll + Accessibility.getPartialAXTree 逐节点取 role/name/ignored；DOM 取可见 h1-h4/strong 文本与 aria-label；0 pageerror。
- 五轮实测：主轮 report.json 6 组；补采 supplement.json 6 组；第三轮 third.json 2 组（真实 UI 发帖造数）；第四轮 fourth.json 2 组（ℹ 按钮超时，失败保留）；第五轮 fifth.json 2 组（分段容错、force click、发帖后 reload），全部证据如实保留，不删失败轮。

## 3. 逐候选程序化证据（与 JSON 一致）

| 候选 | 浮层 | 触发方式 | computed axRole | computed axName | 可见标题文本 | 包含关系 | 双主题 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C1 BBSForum.vue:111 | 发帖区 compose-panel | 点击「✏️ 发起主题」（真实打开，见截图） | dialog（ignored=false） | 发帖区 | 发起新主题（h2, data-testid=bbs-compose-title） | 不包含 | light+dark 一致 |
| C2 BBSForum.vue:145 | 主题详情 topic-detail | 真实 UI 发帖（toast 帖子已发布）后 reload；主题列表 topicCards=0，浮层未打开 | 未取得（浮层未出现） | 未取得 | 动态 viewingTopic.title | 无法实证 | light+dark 均 0 dialog |
| C3 ProgramCenterApp.vue:384 | 对象详情 detail-drawer | 切「脚本」tab（detailLinks=11）点首条 detail-link | dialog（ignored=false） | 对象详情 | 脚本 · script-…（strong，实测动态值，文档脱敏截断） | 不包含 | light+dark 一致 |
| 对照 BBSForum.vue:189 | 版块新建/重命名 | 点击「+」 | dialog（ignored=false） | 新建版块 | 新建版块（h2） | 包含 | light+dark 一致 |
| 对照 BBSForum.vue:213 | 版块详情 | hover 后 force 点 ℹ（aria-label=版块详情） | dialog（ignored=false） | 版块详情 | 版块详情（h2） | 包含 | light+dark 一致 |
| 对照 BBSForum.vue:24 | 搜索按钮（非 dialog） | 页面默认 | — | 搜索（aria-label） | 搜索（button 可见文本） | 包含 | light+dark 一致 |

关键 CDP 原始片段（report.json）：
- light bbs-compose，section[data-testid=bbs-compose-panel]：DLG 1 tag=section role=dialog ariaLabel="发帖区" AXrole="dialog" AXname="发帖区" AXignored=false；heads=["发起新主题"]；containedInAxName=[false]。
- dark bbs-compose：同上，AXname="发帖区"，heads=["发起新主题"]，containedInAxName=[false]。
第五轮原始片段（fifth.json）：
- bbs-section section[aria-label=版块详情]：AXname="版块详情"，heads=["版块详情"]，ignored=false。
- program detail-drawer：AXname="对象详情"，heads=["脚本 · script-…（脱敏截断）"]，ignored=false。

## 4. 三分判定

### 可达真缺陷 2 项（P2）

1. D1 = C1：BBSForum.vue:111 发帖浮层 role=dialog aria-label="发帖区"，可见 h2 为「发起新主题」。computed axName 不含可见文本；语音控制用户按可见标题「发起新主题」无法匹配该 dialog。双主题实证一致。
2. D2 = C3：ProgramCenterApp.vue:384 对象详情抽屉 role=dialog aria-label="对象详情"，可见 strong 为动态「脚本 · script-…」。computed axName 不含可见文本；语音控制用户按所见标题无法匹配。双主题实证一致（真实数据，非静态预判）。

边界说明（据实披露，不夸大）：这两处均为 dialog 容器命名与内部标题文本不一致，而非典型按钮/链接控件的可见标签缺失；浮层内交互控件（发布主题、取消、关闭）自身名称与可见文本一致，语音激活这些控件不受影响。故按 P2 记录，不升 P1；修复收益是让浮层容器本身可被所见文本定位。

### 不可达 / 不适用 L1 1 类

- C2：BBSForum.vue:145 主题详情浮层。真实 UI 发帖后右上 toast「帖子已发布」，但主题列表 topicCards=0（reload 后仍 0），浮层始终未出现，computed axName 无法取得。当前真实数据不可达，不据源码 aria-label="主题详情" 与动态标题字面差异判缺陷。

### 正常规范预期若干

- BBSForum.vue:189 版块新建/重命名 dialog：axName「新建版块」含可见 h2「新建版块」。
- BBSForum.vue:213 版块详情 dialog：axName「版块详情」含可见 h2「版块详情」。
- BBSForum.vue:24 搜索按钮：aria-label「搜索」与可见文本「搜索」一致（R477/R478 已审纯图标按钮不在本维度重复）。
- 其余 79 处中文 aria-label 中绝大多数为纯符号按钮（✕/‹/›/↩ 等），符号非文本标签，不属本维度。

## 5. 修复建议（供后续轮次，本轮不改码）

- D1：BBSForum.vue:111 将 aria-label="发帖区" 改为 aria-labelledby 指向 :113 的 h2（给 h2 加 id，例如 bbs-compose-heading），或最直接改为 aria-label="发起新主题"。保持「发帖区」语义可将文案并入，例如 aria-label="发起新主题 · 发帖区"。
- D2：ProgramCenterApp.vue:384 将 aria-label="对象详情" 改为 aria-labelledby 指向 :386 的 strong（给 strong 加 id），或动态绑定 :aria-label="((selectedDetail.type||'') + ' · ' + (selectedDetail.name||'')) + ' 详情'"，使 computed axName 包含可见标题文本。
- C2：待真实数据下主题详情浮层可打开后另轮补测，本轮不预判。

## 6. 截图与 SHA-256 证据（命令行 sha256sum 核验一致）

| 截图（1440x900 headless 非物理真机） | SHA-256 |
| --- | --- |
| pc-bbs-compose-light-r500.png | b0767b03647892d760fbd50e9395c81d0f3914337ec21f4b458c95934551bfb5 |
| pc-bbs-compose-dark-r500.png | 3904eb2117dd2b78904d07e23a8682962a18632697bd1a3803313590a7238b78 |
| pc-bbs-section-create-light-r500.png | dfe6215405fc5c233ce99a521ab0f24a82b54c52c82ce50c737a64b2b251ac02 |
| pc-bbs-section-create-dark-r500.png | a55040bfd9e3b766160d8eb2eb0cff5f925798a051b4b68a13ddb11bc316d776 |
| pc-fifth-section-light-r500.png（版块详情浮层真实打开） | f15010ead16f7ef71815e0b1996484eb2108f15de709e52aea2736cfada399ef |
| pc-fifth-section-dark-r500.png | c754ea0c41b6415bf68a400dd5695f952e443b2f5c804721f7e5603aedff2f5a |
| pc-fifth-program-light-r500.png（对象详情抽屉真实打开） | ed9991369aca40fe35bea79de043cdee6b11a026af029ec825bd630d99f6eb19 |
| pc-fifth-program-dark-r500.png | 92f88551ba1f17eabaf5f88d84ac2a310f0d27cbfe5974aa3f3be70e3d06a894 |
| pc-third-topic-detail-light-r500.png（发帖 toast 已发布、列表仍空） | d4e7b555b6d724b45c70844b6918f8812433b6db1c93e1b3eb64643aa3c967d8 |
| pc-fifth-topic-light-r500.png（reload 后列表仍空） | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| report.json | 1ca14b1b62c1e016fab82db598f2059dcd02e3e6163891db21b488ef11f17a94 |
| fifth.json | 963ccf07e38c8df26f74d26ed6ebf49f20d08030d4f10481b1b141316a8b5cee |

所有截图来源：本地测试服真浏览器 Playwright Chromium headless（非物理真机），非用户设备截图。

## 7. 脱敏与边界

- 测试服域名/主机一律占位 <YOUR_TEST_DOMAIN> / <TEST_HOST>，真值仅存在于 runner 环境变量与 SSH 运行时提取，不入库。
- 动态脚本名（script-…）在文档截断为「脚本 · script-…」，原始值仅存于 gitignored 的 fifth.json，不入库。
- 反证边界：全库 role=dialog 恰 5 处（BBSForum:111/145/189/213 + ProgramCenterApp:384）全部实测；其余页面未穷举，不预判。
- 本维度结论：可达真缺陷 2 项 P2（D1 发帖浮层、D2 对象详情抽屉）；不可达 L1 1 类（主题详情浮层）；正常若干。至少 1000 轮迭代中的一环，非终态。