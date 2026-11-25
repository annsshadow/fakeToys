# R553 H5 回收站页 @tap 伪按钮键盘可达性甄别

## 结论

判定：**可达真缺陷 1 项 P2**（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。
oa4rust/mobile/src/pages/recycle/recycle.vue 六类 @tap 伪按钮无 role/tabindex/@keydown，不可聚焦、辅助技术读不到语义；鼠标均可用、非死代码。

## 范围与锚点（recycle.vue，sha256=73cb67bafa686c750807554fed7856ed4edd84cbf046ce4b23b510e8d9ba75b0）

| 锚点 | 代码 | 状态 |
| --- | --- | --- |
| :153 | 错误态 retry-btn @tap=load() | 错误态可达 |
| :160 | 空态刷新 @tap=load() | 空态可达 |
| :161 | 空态去文档 @tap=goDoc | 空态可达 |
| :167 | 有行错误 banner retry @tap=load() | 删行造错可达 |
| :170 | 列表项 card-title @tap=showDetail(r) | 造 3 行可达 |
| :178 | 详情关闭 @tap=detail=null | 详情态可达 |

入口 index.vue:171 回收站 cell 已武装 role=button/tabindex=0/@keydown.enter/.space.prevent，1 跳直达；onShow 经 ensureAuthenticated 后 load()，非死代码。
排除：页内原生 button(:172 彻底删除/:183 清空回收站)编译为 uni-button 自定义元素，属 uni-app 编译层另一族，按契约排除，不混入本轮伪按钮族。

## 决定性 CDP 证据（自起 uni H5 + Playwright headless 390x844 DPR2）

- 原始 DOM：六类锚点 role/tabindex/aria-label 全 null；getPartialAXTree 与 getFullAXTree 双通道 role=generic、name=null、ignored=false。
- 聚焦：DOM.focus 到锚点后 document.activeElement=BODY（isTarget=false），getFullAXTree 中 focused=true 非 root 节点数=0（无宿主 wrapper 假阳性）。
- 干净 Tab（未注入、30 次）：10 组 light/dark×五态全为 BODY×30，候选命中 cardTitle/retryBtn/emptyLink/detailClose/BUTTON 全 0；锚点数确认非零（列表 3 行、error retry=1、空态 link=2、详情 close=1、banner retry=1）。
- 注入对照：对锚点 setAttribute(role=button,tabindex=0,aria-label=R553-inj-*) 后，AX role=button、name=注入值，Tab30 命中 15 次，聚焦后 Enter 触发成功——证明缺口可修复、AX 通道正常。
- 跨页强对照：calendar.vue:295 role=button/aria-label=新建日历 但 tabindex=null → AX role=button/name=新建日历、Tab30 未命中（伪可达）；calendar.vue:296 完整范式 role=button/tabindex=0/aria-label=新建事件/@keydown.enter/.space.prevent → Tab30 命中。证明方法能分辨「有语义但键盘不可达」与「完整可达」。
- 鼠标对照（非死代码）：卡片点击 detail=true 且详情正文含造数行、关闭后 detail=false；空态去文档 URL 由回收站切至文档页；删除触发 banner retry=1、errorText=回收站加载失败：HTTP 500。
- 10 组主探针 + 5 组对照 + 10 组干净 Tab，全 fatals=0、pageErrors=0。

## 最小修复方向（业务零改）

逐锚点显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/@keydown.space.prevent 复用原 @tap handler，@tap 保留不动；类比 R544/R546/R550/R551/R549 逐页显式口径。

## 证据 SHA-256（证据文件全在 gitignored oa4rust/test-results/r553/，不入库）

| 文件 | sha256 |
| --- | --- |
| r553-report.json | 113f7a8cb05b66fe0f0bb22a28bc5f3d89dae7985d5de14eab7f6f16bcd12819 |
| r553-detail-report.json | 90334e70be2fd9be65fe01271abc1e5e8921c4b5eb83535b7d94dc2b0fcd575d |
| r553-tab-report.json | ccdf66ffd25d258d1adb0d46b83cd87e824847154d679a8779231c84bacc7496 |
| 列表态截图(light=dark) | 66bfd2702909f0fdbe233b4f67faa9e4a118e77b23f5f8bebe5ca2de98964ad6 |
| 空态截图(light=dark) | fbeb2c17a56e40d72a1b399452743db3212c754ce624432ef0c7a12dc5ea3f9a |
| 错误态截图(light=dark) | 97a034f901132d84fe694ad6665259a98f19a5f50d1e1a259ccc359755b66fd4 |

测试基址 <TEST_HOST>:<PORT> 为占位符，真值仅存在于 gitignored 证据与运行环境变量，未入库。