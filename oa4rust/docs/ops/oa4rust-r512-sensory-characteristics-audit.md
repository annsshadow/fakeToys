# R512 WCAG 2.1 AA 1.3.3 Sensory Characteristics 感觉特征甄别

## 1. 依据与判据

WCAG 1.3.3 要求：仅依赖形状、颜色、视觉位置、方向等感官特征传达的指令，必须有文字替代。本轮甄别界面提示文案是否**唯一**依赖「上方/下方/左侧」等位置词：若同页浮层标题、按钮自身文本或点名目标已提供文字替代，则不构成缺陷。

反预判铁律：严禁仅凭源码命中位置词就判缺陷，必须真浏览器坐实提示真实出现、同页是否确有文字替代、目标是否可达。区分「检测器/路由错误」与真实缺陷，前者据实披露不作证据。

## 2. 方法与基线

- 真浏览器：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，非物理真机。
- 取证：CDP DOM + Accessibility.getPartialAXTree + getBoundingClientRect，四轮实测全部 0 pageerror / 0 fatal。
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，真父 commit 5918f5904f64f7e2f87f1854db193ae973e2bea7，HEAD 与远端对齐，工作树仅非本人未跟踪文件。
- 部署域名与凭据真值仅存在于 gitignored 本地 runner（环境变量注入），本报告用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，0 真值。
- 源码 sha256：QueryStatementDesigner.vue=c71dba6aa59f54ba576cfc3599f5fa39936b5e6ed133392f96d363c23575371b；ProcessDesigner.vue=623261c1d106e2f295b0ded417a410bfbbbbec7d1d83b549d0bb9bb3136cdb70；OrgDirectoryApp.vue=a8658b5c54c170524cbd3f950e503aa15891392e0ca2a5b6a360ef4a79190964；FormDesigner.vue=4903c8b3772fb2c2978b6f7b7f81f962fd696190ebe6e2126cfe948387549435；ThreeMemberApp.vue=0eead71a27b434d31b37bacd31f528a4afc59c49dd45b0fc93b8b7528b13cf8b；main.ts=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0。

## 3. 逐项程序化证据

### 3.1 核心锚点 QueryStatementDesigner.vue 验证浮层（report.json，主轮 2 组双主题）

- 源码：:1726 valResult 初始 message「点击下方按钮验证」；:559-566 浮层 .val-result 渲染该文案；:579 .val-footer 含 button「▶ 验证」与「关闭」；:32 工具栏 button title=SQL语法验证 文本「✅ 验证」。
- 实测（light/dark 一致）：点工具栏✅验证打开浮层后，hint span text=点击下方按钮验证、visible=true、rect{510.3,429.8,104x19.5,bottom449.3}；modalTitle=✅ SQL 语法验证；resultClass=val-result pending；三按钮 ✕(y379.5 位于 hint 上方)、▶ 验证(y497.5 下方)、关闭(y497.5 下方)，below=[false,true,true]。
- 无滚动：bodyScroll scrollHeight=88=clientHeight；modalRect{460,379.5,520x141,bottom520.5}；footerText=▶ 验证关闭；pageVerifyTexts=[✅ 验证,▶ 验证]；toolbarCount=1。
- AXTree：hint axRole=none / axName 空 / axIgnored=true；三按钮 axRole=button、axName=✕ / ▶ 验证 / 关闭、ignored=false；工具栏 button axRole=none / ignored=true（工具栏另有 title=SQL语法验证，tooltip 为文字线索）。
- 第三轮补采（round3.json）：选中首条左侧语句后 sqlLen=0，验证按钮 disabled=true，点击未生效（after hint 仍为点击下方按钮验证、checks=0）——因首条语句本身为空；此项据实披露，不作位置词判定依据。
- 判定：浮层标题「✅ SQL 语法验证」点名操作对象，按钮自身文本「▶ 验证」与「关闭」明确，用户无需依赖「下方」方位即可识别目标——不构成 1.3.3 缺陷。

### 3.2 同族候选逐项（supplement.json + round3.json，含检测器/路由错误据实披露）

- ProcessDesigner.vue:903「点击上方「🔍 流程验证」按钮开始检查」——文案直接点名目标按钮文字「流程验证」，属正面对照，判正常（未作为缺陷候选）。
- FormDesigner.vue:69「拖拽字段到此处，或点击上方组件添加」——除位置词外含文字替代「组件」，判正常。
- ThreeMemberApp.vue:18 空态「暂无三方成员，点击下方「新建」添加第一位成员」——紧邻 button 文本「新建」点名目标，属文字替代；第三轮修正路由 /app/admin/three-member/ 后实测 h1=三方成员管理、表格已有多条记录（R488测试等），空态当前数据不可达（emptyPresent=false、emptyText 空）。判不可达 L1（当前数据）+ 正常（点名目标）。
- OrgDirectoryApp.vue:503/504 toast「成员管理为写动作，请使用下方按钮逐项执行」/「认证/密码为会话动作，请使用下方按钮逐项执行」——源码 RUNNERS.manage/auth 分支，但 tabs 数组（:131-140）在 script 定义、模板 1-115 行无循环渲染（grep v-for tabs / data-testid org-tab 均 NO_MATCH），tab 切换入口不可达，toast 为不可达代码路径。第三轮误用 /app/org/directory/ 落 login 系检测器路由错误（真实路由 main.ts:55-59 为 /app/directory/），据实披露不作缺陷证据。判不可达 L1（代码路径）。
- IMChat.vue:104-108「从左侧选择一个对话开始沟通」/ QueryDesigner.vue:97-101「从左侧列表选择查询定义，或创建新查询」——「左侧」描述导航区域而非操作目标定位，且含文字替代（对话/查询定义/创建新查询），不列为缺陷候选。QueryManagerDeep.vue 位置词 grep NO_MATCH。

## 4. 三分判定

- **可达真缺陷：0 项。** 所有位置词候选均有文字替代（浮层标题点名目标、按钮自身文本、点名目标按钮文案），未发现唯一依赖方位传达指令的界面文案。
- **不可达 / 不适用 L1：2 类。** ① OrgDirectoryApp RUNNERS.manage/auth toast：tabs 模板未渲染，代码路径不可达；② ThreeMemberApp 空态提示：当前已有数据，空态不可触发。均据实测，非缺陷。
- **正常规范预期：4 类。** ① QueryStatementDesigner 验证浮层（标题+按钮文本替代）；② ProcessDesigner 流程验证（点名目标）；③ FormDesigner 组件添加（文字替代）；④ ThreeMember 新建按钮 / IMChat / QueryDesigner（点名目标或文字替代）。
- 反证边界：静态命中位置词候选已逐项实证，未穷举全库其他页面，不作超出实测范围的外推。

## 5. 修复建议

本维度无可达真缺陷，无需修复。非阻断观察（移交后续轮次，不并入本轮判定）：QueryStatementDesigner 验证浮层提示「点击下方按钮验证」可优化为「点击「▶ 验证」按钮验证」以彻底消除方位依赖，属增强而非缺陷。

## 6. 证据与 sha256

全部 PNG 与 JSON 经命令行 sha256sum 核验一致（下面为前 8 位）：

- report.json=bb9985e75ec7288a10c3ab22c33eed52e3dc436cbbe88a58de0551937a34c04b
- supplement.json=f2f37150f46d1fff2c1ca1fe59e05238978e1aa8606f5adf893a1fe5642112ef
- round3.json=6f7c351bab0f8c60fc3d28d4796f81a20e5e54cc5670d6b63d834b569836cebb
- pc-qsd-validator-light-r512.png=b4e9747185539afb092c669b76ff9c7fbb9a8a306fdb926dd9a3e312c5d4fa81 / dark=8159dcb8f95d2758ee42c6e98cdb1ccdf02028c485baa83f31818375d6060f7e
- pc-qsd-verifyclick-light-r3-r512.png=cefdcdb230c4b37df523daa3cd8732720c3a459d29cde1d6ad8f8db0dfec3edc / dark=1fbaa4f040b0b2eeb996560c57dbf3642fb2e5171c2d85154a853046cce723fc
- pc-threemember-empty-light-r3-r512.png=d1031f3df3c9240204dc6e1c563de135aa451530d5daa79e642332d9cf801df0 / dark=8e0f811504be1847be7a07e50e98f19034641f5a824cff2e899b5c83ddccccd6
- pc-org-directory-light-r3-r512.png=91608d1d09218269d129e575220af974b38257670c9769002e0921d21955dc70 / dark=27b34e278a4aa8cd48137e3a36c695526854544695631a003e349b17ccdb2213
- pc-org-toast-light-sup-r512.png=fcc0cbe2e1078b0aae6b9a966d68d8fa30b568af5900f8559995abf414d4e09b / dark=f8089ceef5ff0db48a0138d7737e66ed96cf693d16623c4ff06cae8f3b7be925
- pc-threemember-empty-light-sup-r512.png=aa897cd337b9369e173854440efe336ef370f7075fffaf259ac4e132ce4f0d5d / dark=7f2d0ef6756fc7f05c725a1f6678a053b77011df002ef200cfee89ab421f4c27
- pc-qsd-verify-light-sup-r512.png=0296a77ff98ad5d1d345fc569f41d7bb094b78e5952efc91f0198450005dd9b4 / dark=e8a0dce7230bb3493ae4a619360f7da384e124a3692d836c94c3b7108da5c12a

## 7. 脱敏边界

本报告与证据分离：真域名、账号、口令、SSH 信息仅存在于 gitignored 本地 runner 与证据 JSON，文档仅用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，无任何真值入库。截图均为 headless 非物理真机，据实标注。