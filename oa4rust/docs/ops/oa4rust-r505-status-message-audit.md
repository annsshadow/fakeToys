# R505 WCAG 2.1 SC 4.1.3 状态消息甄别（Dashboard / ProcessWork 自定义错误横幅）

> 轮次 R505；只读甄别，不改任何源码、不改台账。真父 HEAD=500cf57907101efa7acc9f11b2cd2712faa03cef。
> 执行者 qa-e2e；与 design-lead 的 R504 并行，写域不重叠。项目 ≥1000 轮持续迭代，本轮非终态。

## 一、判据与前序不重复说明

本轮依据 WCAG 2.1 SC 4.1.3 Status Messages（AA）：状态变化须能被辅助技术播报，不得仅以视觉呈现。反预判判据：
1. 错误态须在真浏览器真实出现并截图坐实（构造查询失败）；
2. 区分动态状态消息与常驻内容：横幅须在页面加载后异步动态出现（before=0 → after=1）才算 status message，首屏即存在不算；
3. 区分 toast 共触发：若 toast.error 同时播报同一错误则属重复播报判正常，无 toast 才是真候选；
4. 区分全局增强器覆盖：loadingA11y 仅 .loading-state/.loading-indicator/.loading-spinner/.skeleton-loader；formErrorA11y 仅 data-oa-field-error；ListLoadError 已修 role=alert/aria-live=assertive；routeAnnouncer 仅路由导航。四者均不覆盖自定义错误横幅与 useQuery 错误态。
5. 三分判定：可达真缺陷 / 不可达 L1 / 正常规范预期，逐项给实测依据，0 缺陷须据实判 0，严禁臆造。

前序已覆盖、本轮不再列为候选：R215 加载指示器、R226 ListLoadError.vue（全库 40+ 视图 role=alert/aria-live=assertive）、R228 登录失败播报、R231 路由播报。
队长已去伪排除、不再复列：OrgDirectoryApp loadError/deptTreeError/unitCreateError（toast.error 共触发同文案，重复播报判正常）；FileManager error-banner（内嵌 ListLoadError 已播报）；main.ts 全局 toast.error（仅未捕获异常非业务横幅）；mobile bbs.vue tip-error（无源码低价值）。

## 二、候选清单与去伪

| 候选 | 源码锚点 | 是否自定义错误态 | 是否前序/全局已覆盖 | 是否进入三分判定 |
| --- | --- | --- | --- | --- |
| C1 统计计数错误横幅 | Dashboard.vue:17-20（countError computed :250-253，msg 查询带 refetchInterval:30000） | 是 | 否（无 role/live，无 toast 共触发，增强器不覆盖） | 是 |
| C2 待办加载失败错误态 | Dashboard.vue:60-63（pendingError 于 loadPending catch :335-337 赋值，onMounted :371 触发） | 是 | 否 | 是 |
| C3 工作流待办列表错误态 | ProcessWork.vue:34-37（query.error.value = useQuery :289-304，GET task/list/my/paging） | 是 | 否 | 是 |
| C4 工作详情弹窗错误态 | ProcessWork.vue:106（detailError 于 openWork catch :1138 赋值） | 是 | 否 | 是 |
| 去伪：ListLoadError 全系列 | ListLoadError.vue | 否（已修） | 是 role=alert/aria-live=assertive | 否（前序覆盖） |
| 去伪：loading/formError/route 三增强器 | loadingA11y.ts / formErrorA11y.ts / routeAnnouncer.ts | 否 | 是（选择器不匹配本四候选） | 否（不覆盖） |
| 去伪：toast.error | toast.ts:60-61 | 否（独立通知组件） | 组件自身 role=alert/aria-live=assertive，但本四候选无共触发 | 否（无共触发，见第三节） |

## 三、三分判定逐项依据（真浏览器 1440x900 headless 双主题，非物理真机）

构造方式：Playwright Chromium headless，viewport 1440x900，localStorage oa4rust_theme 切 dark/light；route 谓词注入目标查询 HTTP 500 {type:error}，CDP Accessibility.getFullAXTree + DOM.describeNode 取 axRole/axLive，DOM 取 role/aria-live，统计 .oa4-toast 可见数。useQuery 三重试累计约 7 秒，故错误态等待 10 秒。证据：gitignored oa4rust/test-results/r505-status-message/{evidence.json,fix.json}。

### C1 统计计数错误横幅（Dashboard.vue:17-20）—— 可达真缺陷（AA 违规）
- 动态性：dark/light 双主题 before selectorCount=0 → after=1（fix.json count:dark/count:light），页面加载后异步出现，属 status message。
- 播报语义：DOM role=null、aria-live=null、aria-atomic=null；CDP axRole=none、axLive=[]。读屏无法播报。
- toast 共触发：toast.visible=0（错误态出现时无 toast.error 播报同一错误），非重复播报，构成真候选。
- 全局增强器：error-banner stats-error 不匹配 loadingA11y 四个选择器，非 data-oa-field-error，非 ListLoadError，非路由。无覆盖。
- 结论：可达真缺陷。视觉红横幅「统计计数加载失败：HTTP 500」+ 重新加载按钮，但读屏零播报。

### C2 待办加载失败错误态（Dashboard.vue:60-63）—— 可达真缺陷（AA 违规）
- 动态性：dark/light 双主题 baseline=0 → after=1（evidence.json dashboard-pending），onMounted 中 loadPending 失败后异步出现。
- 播报语义：DOM role=null/aria-live=null；CDP axRole=paragraph（仅为段落语义，非 status/alert），axLive=[]。
- toast 共触发：toast.visible=0，无共触发。
- 全局增强器：.empty-state .error-text 不匹配任何增强器，非 ListLoadError 组件。无覆盖。
- 结论：可达真缺陷。

### C3 工作流待办列表错误态（ProcessWork.vue:34-37）—— 可达真缺陷（AA 违规）
- 动态性：dark/light 双主题 before=0 → after=1（fix.json list），点「刷新待办」触发 query 失败后异步出现。
- 播报语义：DOM role=null/aria-live=null；CDP axRole=none、axLive=[]。
- toast 共触发：toast.visible=0，无共触发。
- 全局增强器：.state.error-state 不匹配加载增强器（非 .loading-state），非 formError/ListLoadError/路由。无覆盖。
- 结论：可达真缺陷。

### C4 工作详情弹窗错误态（ProcessWork.vue:106）—— 可达真缺陷（AA 违规）
- 动态性：dark/light 双主题 baseline=0 → after=1（evidence.json process-detail），注入假待办行后点击打开详情，form/v2/lookup 注入 500，openWork catch 赋值 detailError 后异步出现。
- 播报语义：DOM role=null/aria-live=null；CDP axRole=none、axLive=[]。
- toast 共触发：toast.visible=0，无共触发。
- 全局增强器：.work-dialog .state.error-state 无覆盖。
- 结论：可达真缺陷。

三分汇总：可达真缺陷 4 项（C1/C2/C3/C4，均 AA 违规）；不可达 L1 0 项；正常规范预期 0 项。无 0 缺陷据实判 0 的情形——四候选均满足动态出现 + 无 toast 共触发 + 无全局增强器覆盖全部三条件，非误报。

## 四、证据 sha256（命令行原始值，非 read_image 重编码值）

截图均 1440x900 PNG，测试服真浏览器渲染，非物理真机；存 gitignored oa4rust/test-results/r505-status-message/。
| 截图 | sha256 | 说明 |
| --- | --- | --- |
| r505-fix-count-dark.png | 71904f82ffeddb9c045c73c8544aa9d4e5bc8e014e7f13c76858b277af8f151c | C1 深色横幅真渲染（已目视） |
| r505-fix-count-light.png | 047a195af315c556adf9e605f9a516b761c0ceb83cfb7329da90e15a78acd997 | C1 浅色 |
| r505-dashboard-pending-dark.png | dd54f298919b36bcd8be0386e9b930791d7081ef51644b96cf8d8fab5a90172a | C2 深色错误态（已目视） |
| r505-dashboard-pending-light.png | 40069c878052e35d350fe8be2251676931e6172b0638fefe6040abafa7eaddcb | C2 浅色 |
| r505-fix-list-dark.png | 2309917ebfdd973153b2ca38a25c8a8ba4e97aec6fff7e411c71acc9c7b35d2a | C3 深色错误态（已目视） |
| r505-fix-list-light.png | c48930ba6454b05352f3606fe25955f255ee74c953a7150cca95ff8b54b9f672 | C3 浅色 |
| r505-process-detail-dark.png | 9084bbfb1496be66e6d77f3b6c9f5721b454e599dd5f9b4cf8ae1c9e4d3800e3 | C4 深色弹窗错误态（已目视） |
| r505-process-detail-light.png | a9d22aa58ae28f783998334c314dcd396e16e9cc8f260da8491350f3260e3974 | C4 浅色 |
| r505-dashboard-count-dark.png | ec0baa737be846d20602e63cb9f9a8b6f087cac4985fe8767af5564664d45858 | C1 首轮注入未生效（无横幅），取证过程留档 |
| r505-process-list-dark.png | 619755d0491e21e6c58454f3e674386942fabf680e984fd5fe825590815307a7 | C3 首轮注入未生效，取证过程留档 |
| r505-dashboard-count-light.png | 91f36b2bfc82b3fd81a5c8d011f2925573d38bf0599edfb24ba8b0c50d763f42 | C1 首轮浅色留档 |
| r505-process-list-light.png | a95c47b73d0f3a18c375bfc0aa2077fc28236d02581f29caa7d5eaf4da0ea70e | C3 首轮浅色留档 |

取证过程说明：首轮 dashboard-count / process-list 两候选因 useQuery 默认 3 次重试累计约 7 秒、等待仅 2.8 秒，错误态尚未渲染，见首轮 4 张留档截图（selectorCount=0）；延长等待至 10 秒后重跑 fix.json，四处均 selectorCount=1。C2/C4 首轮即坐实。两轮证据一致指向同一结论。pageErrors=0（两轮均无页面异常）。

## 五、脱敏声明

- 来源与证据域名一律用 <YOUR_TEST_DOMAIN> / <HOST> 占位，真实测试服域名仅存在于 gitignored 的 test-results/r505-status-message/ 证据文件中，不入库、不回显。
- 凭据运行时从远端凭据脚本读取，仅存 shell 变量，未写入任何文件、文档或提交信息。
- 截图为测试服真浏览器无头渲染（1440x900，非物理真机），已如实标注，不冒充真机。
- 文档无密钥、口令、私钥、真域名、个人环境路径等真值。

## 六、结论与修复建议

结论：本轮在 Dashboard 与 ProcessWork 的 4 处自定义错误横幅/useQuery 错误态上坐实 4 个可达 WCAG 2.1 SC 4.1.3（AA）真缺陷。均动态出现、无 toast 共触发、无全局增强器覆盖，读屏无法播报错误状态。

最小修复建议（本轮不改码，供后续 RED→GREEN 轮）：
1. C1 Dashboard.vue:17-20 错误横幅加 role=alert（或 role=status + aria-live=assertive）；
2. C2 Dashboard.vue:60-63 待办错误态容器加 role=alert；
3. C3 ProcessWork.vue:34-37 列表错误态加 role=alert；
4. C4 ProcessWork.vue:106 弹窗错误态加 role=alert；
5. 或统一复用已修的 ListLoadError 组件 / 扩展全局增强器选择器覆盖 .error-banner/.error-state/.error-text。
每处修复需配套契约测试（断言 role/aria-live），并真浏览器 CDP AXTree 复验 axRole/axLive 后关单。

取证限制：错误态通过 route 注入 HTTP 500 构造（非真实后端故障），注入仅用于复现 UI 渲染与播报语义，不改变本轮判定（判定对象是横幅自身的无障碍语义）；光主题与深主题均应覆盖，已覆盖。

≥1000 轮持续迭代，本轮非终态，待队长独立复核与后续修复轮。