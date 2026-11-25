# R499 WCAG 2.4.6 标题层级（Headings and Labels）甄别审计

> 轮次：R499 / 任务 t392（只读甄别，不改码）
> 判据：WCAG 2.4.6 Headings and Labels（AA）
> 基线真父：59cf70bc86a9e73952b099ba1f328e3ba0fa7fbb（台账 §494 据实立项）
> 分支：opt1000/oa4rust-pc-ai-refactor（非 main）
> 数据来源：测试服真浏览器 Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，CDP DOM + Accessibility.getPartialAXTree。截图为 headless 非物理真机，非真机手拍。
> 测试地址：<YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，真值仅 gitignored runner 与 .local.keys，本文零真值。

## 1. 甄别范围与判据

静态侦察（subagent r499-scout）给出四类候选，均仅作候选、非缺陷结论：

1. 跳级候选：DeploymentApp.vue:42 h3「新建部署记录」紧接 :8 h1「部署管理」；NoteApp.vue:43 h3「新建笔记」紧接 :8 h1「笔记管理」。两处 h3 均在 div.modal-overlay（v-if showCreate/showEdit，DeploymentApp:40-41 / NoteApp:41-42）浮层内，打开才渲染，须 CDP 区分浮层独立上下文与同页主标题。
2. 缺 h1 候选：45 文件；B1 主标题用 h2（11 文件，含 CmsIndexApp:10 / IMChat:9 / ProcessManagerApp:11）、B2 首标题 h3/h4（34 文件，多为浮层内；ProcessDesigner:221 首 h4）。
3. 泛化标题候选：AppInfoApp:40 / ViewApp:42 / DocumentApp:82 / TemplateApp:45「详情」。
4. div 浮层标题：80 命中 / 13 文件，role=dialog 仅 5 处（BBSForum:111/145/189/213 + ProgramCenterApp:384）；ProcessManagerApp:5 / ProcessTaskCenterApp:6 AdminPageHeader title 为组件封装标签须 AXTree 核实。

三分判定：可达真缺陷（层级或名称缺失影响导航且无等价，须实证）/不可达 L1（候选当前状态不可达，不预判缺陷）/正常规范预期（浮层独立上下文、组件封装标题、语义等价或描述性名称，据实判 0）。

## 2. 实测方法与程序化自洽

- 主轮 test-results/r499-e2e.cjs + r499-run.sh：12 页面配置 × 双主题 = 24 组，R499_DONE groups=24 fatals=0 errs=0，0 pageerror。
- 补采 test-results/r499-sup.cjs + r499-sup-run.sh：5 页面配置 × 双主题 = 10 组，R499_SUP_DONE groups=10 fatals=0 errs=0，0 pageerror。
- 每次采 DOM 全部 h1-h6/[role=heading]（tag/文本/可见/是否浮层内/最近 dialog 的 aria-label）+ 全部 [role=dialog]/.modal-overlay/.modal；再 CDP DOM.querySelectorAll 定位后 getPartialAXTree 取 axRole/axName/level/ignored。截图 crypto sha256 与命令行 sha256sum 双算一致。
- 证据文件（gitignored）：test-results/r499-heading-level/report.json（主轮 24 组）、supplement.json（补采 10 组）。本文逐项表与这两份 JSON 同源，无 R487 式文档/report 矛盾。
- 首跑 app-info 用错路由 /app/admin/app-info/ 落到登录页（h1 OA4RUST + h2 智慧办公·无限可能），据实披露并在补采以正确路由 /app/admin/appinfo/ 重采（h1 应用管理），不以错误路由数据下结论。

## 3. 逐页程序化证据（DOM 可见标题 → CDP AX 层级）

表内 AX 记法：tag:text L<level> ign=<ignored>；@DLG[label] 表示标题位于浮层及其 aria-label。全部 heading 均 axRole=heading、axIgnored=false（除注明）。

### 3.1 主轮（report.json，24 组）

| 组（主题/键） | 页面 | 可见 h1-h4 DOM | CDP AX | 浮层上下文 | 截图 sha 前 12 |
|---|---|---|---|---|---|
| light/deployment-base | 部署管理 | h1 部署管理 | 部署管理 L1 ign=false | 无 | 612f84e58105 |
| light/deployment-modal | 部署管理·新建浮层 | h1 部署管理 + h3 新建部署记录 | 新建部署记录 L3 ign=false | div.modal-overlay role=dialog + div.modal | 15ab2dafa910 |
| light/note-base | 笔记管理 | h1 笔记管理 | 笔记管理 L1 ign=false | 无 | 3073d4e69a41 |
| light/note-modal | 笔记管理·新建浮层 | h1 笔记管理 + h3 新建笔记 | 新建笔记 L3 ign=false | div.modal-overlay role=dialog + div.modal | b9ab49694132 |
| light/cms-index | CMS 索引设计 | h1 CMS索引设计 + h3 业务工作台 + h2 CMS 索引设计 | L1/L3/L2 全 ign=false | 无 | 8789d54a3653 |
| light/im | 即时通讯 | h2 消息 + h2 选择会话 | 消息 L2 / 选择会话 L2 ign=false | 无 | 4305f68b1b16 |
| light/bbs-base | 团队论坛 | h1 团队论坛 + h2 版块导航 + h2 主题列表 | L1/L2/L2 ign=false | 无 | 888c846de221 |
| light/bbs-compose | 团队论坛·发帖浮层 | + h2 发起新主题 | 发起新主题 L2 ign=false | section role=dialog aria-label=发帖区 | b0767b036478 |
| light/process-designer | 流程设计器 | h1 流程设计器 + h3 业务工作台 | L1/L3 ign=false | 无 | 1f4a124f7654 |
| light/document | 文档管理 | h1 文档管理 | L1 ign=false | 无 | d04912d4e664 |
| light/view | 视图管理 | h1 视图管理 | L1 ign=false | 无 | 3d852ab3bd19 |
| light/app-info¹ | 路由错→登录页 | h1 OA4RUST + h2 智慧办公·无限可能 | L1/L2 ign=false | 无 | 85cf0ea18eeb |
| dark/deployment-base | 部署管理 | h1 部署管理 | L1 ign=false | 无 | 13f0f1a4130b |
| dark/deployment-modal | 部署管理·新建浮层 | h1 + h3 新建部署记录 | L3 ign=false | div.modal-overlay role=dialog | 75794a5c9bd9 |
| dark/note-base | 笔记管理 | h1 笔记管理 | L1 ign=false | 无 | 5b7c9fb60998 |
| dark/note-modal | 笔记管理·新建浮层 | h1 + h3 新建笔记 | L3 ign=false | div.modal-overlay role=dialog | faab1bcb0d19 |
| dark/cms-index | CMS 索引设计 | h1 + h3 + h2（同 light） | L1/L3/L2 ign=false | 无 | bf3c842ffd24 |
| dark/im | 即时通讯 | h2 消息 + h2 选择会话 | L2/L2 ign=false | 无 | a8af6cf0cbb1 |
| dark/bbs-base | 团队论坛 | h1 + h2 + h2 | L1/L2/L2 ign=false | 无 | 6971592b8637 |
| dark/bbs-compose | 团队论坛·发帖浮层 | + h2 发起新主题 | L2 ign=false | section role=dialog aria-label=发帖区 | 3904eb2117dd |
| dark/process-designer | 流程设计器 | h1 + h3 业务工作台 | L1/L3 ign=false | 无 | 9acd6ddd82ad |
| dark/document | 文档管理 | h1 文档管理 | L1 ign=false | 无 | 46ee4ce775e3 |
| dark/view | 视图管理 | h1 视图管理 | L1 ign=false | 无 | f94c0491e2b7 |
| dark/app-info¹ | 路由错→登录页 | h1 OA4RUST + h2 智慧办公·无限可能 | L1/L2 ign=false | 无 | 4b27d27f70c7 |

¹ app-info 主轮路由错误，已由补采 appinfo-correct 纠正，不据此组下结论。

### 3.2 补采（supplement.json，10 组）

| 组（主题/键） | 页面 | 可见 h1-h4 DOM | CDP AX | 浮层上下文 | 截图 sha 前 12 |
|---|---|---|---|---|---|
| light/appinfo-correct | 应用管理 | h1 应用管理 | L1 ign=false | 无 | bfa2b9206fc8 |
| light/document-detail | 文档管理·详情浮层 | h1 文档管理 + h3 文档详情 | h3 L3 ign=false | div.modal-overlay role=dialog | 0150ab8605a6 |
| light/view-detail | 视图管理·详情浮层 | h1 视图管理 + h3 视图详情 | h3 L3 ign=false | div.modal-overlay role=dialog | 6bd76c3fc7e2 |
| light/template-detail | 模板管理·详情浮层 | h1 模板管理 + h3 模板详情 | h3 L3 ign=false | div.modal-overlay role=dialog | 62b1c3459ab2 |
| light/bbs-topic-detail² | 团队论坛 | h1 + h2 + h2（详情浮层未触发） | L1/L2/L2 ign=false | 无 | 888c846de221 |
| dark/appinfo-correct | 应用管理 | h1 应用管理 | L1 ign=false | 无 | 45e801ad6429 |
| dark/document-detail | 文档管理·详情浮层 | h1 + h3 文档详情 | h3 L3 ign=false | div.modal-overlay role=dialog | c9d0fbc857d4 |
| dark/view-detail | 视图管理·详情浮层 | h1 + h3 视图详情 | h3 L3 ign=false | div.modal-overlay role=dialog | 53a7440aada3 |
| dark/template-detail | 模板管理·详情浮层 | h1 + h3 模板详情 | h3 L3 ign=false | div.modal-overlay role=dialog | 857c9d90c327 |
| dark/bbs-topic-detail² | 团队论坛 | h1 + h2 + h2（详情浮层未触发） | L1/L2/L2 ign=false | 无 | 6971592b8637 |

² 补采点击主题列表未打开主题详情浮层（role=dialog aria-label=主题详情），浮层内 h2/h3 层级未实证，据实判 L1，不预判缺陷。

## 4. 三分判定

### 可达真缺陷：0 项

- DeploymentApp:42 / NoteApp:43 的 h3「跳级」：CDP 证实仅点击「+ 新建」后随 div.modal-overlay role=dialog 浮层渲染，背景 h1 属主文档上下文、h3 属浮层独立上下文，非同层级序列真跳级。截图目视浮层确实出现，名实相符。判正常。
- cms-index / process-designer 的 h1→h3（业务工作台）：AX 证实 h1=AdminPageHeader 组件封装标题、h3=AdminFormSection 组件封装 title、h2=CMS 索引设计为卡片内手写标题；h1 与 h2 文本等价。属组件封装标题 + 语义等价，按反预判铁律不算候选，判正常（据实披露 DOM 顺序 h1→h3→h2 的层级倒退系组件封装所致）。
- IMChat 无 h1、仅 h2 消息 / h2 选择会话：双栏 IM 布局两个并列面板标题，页面顶栏面包屑「工作台 / 即时通讯」提供上下文；缺 h1 不必然缺陷，判正常（据实披露无 h1）。
- 「详情」泛化标题：实测为「文档详情」「视图详情」「模板详情」及 AppInfoApp:40 动态数据项名称，CDP axName 均具描述性，静态「泛化」预判被实证推翻，判正常。
- BBSForum 发帖浮层 h2 发起新主题：位于 section role=dialog aria-label=发帖区 内，独立上下文 + 可访问名称，判正常。

### 不可达 / 不适用 L1：1 类

- BBSForum 主题详情浮层（静态 BBSForum:145 role=dialog aria-label=主题详情，含 h2 标题 / h3 回复）：补采真实点击未打开该浮层，内部标题层级未实证，当前数据/交互路径不可达，据实判 L1，不预判缺陷。

### 正常规范预期：全数其余组

- 各 CRUD 页 h1 主标题（部署管理 / 笔记管理 / 文档管理 / 视图管理 / 应用管理 / 团队论坛 / 流程设计器）axName 具描述性、ignored=false。
- 浮层 h3（部署记录 / 笔记 / 文档详情 / 视图详情 / 模板详情）均在 dialog 独立上下文内，标题描述具体操作对象。
- 全部 34 组 0 pageerror，heading 均 axRole=heading 且未被 AX 忽略。

## 5. 反证边界与静态预判纠正

- 静态 196 处 heading / 45 文件缺 h1 / 80 处浮层标题为候选全集，本轮择 12 配置 + 补采 5 配置实证；其余未采样页不预判。
- 静态「泛化详情」预判被 AX 实证纠正：实际 axName 均带对象前缀（文档/视图/模板）或数据名，具描述性。
- 静态「DeploymentApp/NoteApp h3 跳级」预判被浮层上下文实证纠正：浮层独立上下文，非同页跳级。
- ProcessDesigner:221 h4「源字段」位于 showDataMappingEditor 浮层（未打开），未纳入实证，不预判。
- role=dialog 仅 5 处为静态统计；CDP 坐实 BBSForum 发帖浮层 section role=dialog aria-label=发帖区，其余 dialog 未逐一触发，不预判。

## 6. 截图与 SHA 命令行核验

主轮 24 张 PNG（pc-*-r499.png）+ 补采 10 张 PNG（sup-*-r499.png）经 sha256sum 命令行核验，与 report.json / supplement.json 的 crypto sha 逐张一致。完整 64 位 sha 见下：

| 截图 | sha256 |
|---|---|
| pc-deployment-base-light-r499.png | 612f84e58105254d62bde5be7e0db5d94ef7dcd4e856fd2239650ce934baba34 |
| pc-deployment-base-dark-r499.png | 13f0f1a4130b893f5424d0cd0899c8f53c33469c172fce53e34719b4417d3af2 |
| pc-deployment-modal-light-r499.png | 15ab2dafa910706ad46aa6bb41e9c0d544042a5e7bdde7cf2cc95b46dcb9ead3 |
| pc-deployment-modal-dark-r499.png | 75794a5c9bd95290be0f829ad39bde27423275b34a1b793d0e01758a31bb9fe2 |
| pc-note-base-light-r499.png | 3073d4e69a41e8a9713c65923d08c8af13fb0a2b0911df0f6abc568be55bd8c0 |
| pc-note-base-dark-r499.png | 5b7c9fb6099896699a23f5222eaac51ce596ca7a26638f3de9a2283ee45ebf6b |
| pc-note-modal-light-r499.png | b9ab49694132342cc40142c4ce494598ebfb591ecf1997d876ebfed17bdb3da7 |
| pc-note-modal-dark-r499.png | faab1bcb0d1931565fbfad321f8996b2dd376ddb26b4fbc36ab5b1160d675b2e |
| pc-cms-index-light-r499.png | 8789d54a3653b6916d2d95bfc43a33a9cdf1ab0b6202cf4ff3c1652c118da6bf |
| pc-cms-index-dark-r499.png | bf3c842ffd244a485abab64e7a36f1d5a17456c28a86b6915481670a0b888c16 |
| pc-im-light-r499.png | 4305f68b1b168138bead0dc7d973c0af31aed7bcb79ebe6e8b338c1a6cbdc24a |
| pc-im-dark-r499.png | a8af6cf0cbb10f13634c758aa04ca23b46c5adca2c12107bc0efdc045f2a21cd |
| pc-bbs-base-light-r499.png | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| pc-bbs-base-dark-r499.png | 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c |
| pc-bbs-compose-light-r499.png | b0767b03647892d760fbd50e9395c81d0f3914337ec21f4b458c95934551bfb5 |
| pc-bbs-compose-dark-r499.png | 3904eb2117dd2b78904d07e23a8682962a18632697bd1a3803313590a7238b78 |
| pc-process-designer-light-r499.png | 1f4a124f765404b974003e5179eb145cf835dd5f3132dd36e12ca3eb38414610 |
| pc-process-designer-dark-r499.png | 9acd6ddd82ad57c0bfea5cf3f5ecb3025d2209ea8ce45b6bcbe27918c18c7df2 |
| pc-document-light-r499.png | d04912d4e66442967f89a2ef283eccbdf94cd129d42a495fcf95a4d2061bed1f |
| pc-document-dark-r499.png | 46ee4ce775e3e01314c2c3ab6d2518e5f2e7849d38526bf0774c0c20fe96b69d |
| pc-view-light-r499.png | 3d852ab3bd191e6acc19f333475fe1168e4544a8a33e79ea11c52c4e12e7f129 |
| pc-view-dark-r499.png | f94c0491e2b7103d1f2b5d5472d8e6c711410d62bf376a92966548b887b54183 |
| pc-app-info-light-r499.png（路由错，登录页） | 85cf0ea18eeb83d2b51f0487f68fb29300f3315375fca9ada3de48cda8f4afb3 |
| pc-app-info-dark-r499.png（路由错，登录页） | 4b27d27f70c7e4bd00c4a72008cebfe651158c8e0734f1f2499a001e7592cb48 |
| sup-appinfo-correct-light-r499.png | bfa2b9206fc84f8447eb6f37aa291abbf3faf95a0ba66ccb174210798c217007 |
| sup-appinfo-correct-dark-r499.png | 45e801ad64291b7cff6fdeb3f48f0e1cc7a10672d089f987e1a9f50876258319 |
| sup-document-detail-light-r499.png | 0150ab8605a68c3e5eac458618f631b203d8d1b66150e95c8e8b8b289d5473f8 |
| sup-document-detail-dark-r499.png | c9d0fbc857d4be8ac4069eb1731c9f3b42e8c13e6ed32e331171b1b2b0ebde65 |
| sup-view-detail-light-r499.png | 6bd76c3fc7e2e155675e29dc7c524f106d2c963efd377ddec2732659ab349c24 |
| sup-view-detail-dark-r499.png | 53a7440aada30e76b03414a5754dc481b6b5ec4847f15965812c4f0a5cb4adc0 |
| sup-template-detail-light-r499.png | 62b1c3459ab227c4d99f737801fcea5cca8939362ccba0b0e7723950d6c5f0c5 |
| sup-template-detail-dark-r499.png | 857c9d90c3278fa89cbca0baac2824eb335e43c6c70365e52b6d53a5dbd1b8be |
| sup-bbs-topic-detail-light-r499.png | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| sup-bbs-topic-detail-dark-r499.png | 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c |

AXTree 证据文件 sha256（命令行）：report.json = d0f9c85bd100c050f1d373307622e3860fde52c0edbf6091e0c5d81970ce6c08；supplement.json = e119c56154783b7faa90a3146768371580254368fc5120afd4d3c732db42d0ae。内含全部 heading 的 axRole/axName/level/ignored 原始值。

## 7. 修复建议与脱敏边界

- 本维度（WCAG 2.4.6）无需源码修复：可达真缺陷 0 项。跳级/缺 h1/泛化标题候选均被浮层独立上下文、组件封装标题、语义等价或描述性 axName 实证消解。
- 非阻断观察（不属本轮修复）：ProcessDesigner:221 浮层内 h4、BBSForum 主题详情浮层内部层级未实证，留待可复现数据轮次；AdminFormSection 组件 h3 与手写 h2 的层级倒退若需统一可由组件层优化，但当前不构成 2.4.6 缺陷。
- 脱敏：本文零真值，测试域名/主机名一律占位 <YOUR_TEST_DOMAIN> / <TEST_HOST>；凭据仅由 runner 从测试服 .local.keys 运行时读取并注入环境变量，未写入文档或证据 JSON；截图为 headless 非物理真机，未冒充真机；本报告为只读甄别，未改任何业务源码。