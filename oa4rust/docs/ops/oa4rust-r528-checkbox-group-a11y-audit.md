# R528 表单设计器预览多选组分组语义无障碍甄别（WCAG 1.3.1，只读）

- 甄别编号：R528；真父提交 6d3c1aac0e98610f4b15761888416247a3cb9629（台账 §519）。
- 性质：只读甄别，未修改任何业务源码；证据脚本与截图均入 gitignored 目录 oa4rust/test-results/r528-（.gitignore:28）。
- 方法：本地 Vite dev server 加载仓库源码 + context.route API stub（占位数据，无真值、无真实写库），Playwright 自带 Chromium headless，1440×900 DPR1，light/dark 双主题。
- 域名占位：`<TEST_HOST>`；真域名/账号/口令一律不入本文档与对话。

## 一、结论与三分判定

**判定：可达真缺陷 1 项 —— WCAG 2.1 SC 1.3.1 Info and Relationships（Level A）。**

- 缺陷描述：表单设计器「表单预览」中，多选字段 .pv-cb-group 把 3 个复选框（A/B/C）在视觉上组织为一个字段，但容器无 role=group、无 aria-label/aria-labelledby，字段标题「多选」的裸 label 未与容器或选项程序化关联；CDP AXTree 坐实容器为 role=none / ignored=true / 无 name，视觉分组与字段归属关系不可被辅助技术确定。
- 关联而非另计：每个复选框自身有可访问名 A/B/C 且 ignored=false，属 SC 4.1.2 通过项，不作为缺陷上报；「多选」裸 label 的关联缺失是同一 1.3.1 缺陷的组成证据，不重复计数。
- 三分归类：可达真缺陷（非不可达 L1、非正常）。纯前端入口可达（/app/admin/form-designer → 新建 → 选「多选」字段 → 预览），无需后端真实数据。0 缺陷假设不成立，据实判 1。

## 二、源码锚点（逐行）

- oa4rust/frontend/apps/desktop/src/views/FormDesigner.vue（sha256 `4903c8b3772fb2c2978b6f7b7f81f962fd696190ebe6e2126cfe948387549435`，142970 bytes）：
  - :119 `<div v-if="mode==='preview' && currentForm" class="preview-overlay" @click.self="mode='edit'">`；:120 预览弹层 .preview-modal；:122 标题 currentForm.name。
  - :127 `<div v-for="(field,i) in currentForm.fields" class="pv-field">`；:128 `<label v-if="!['row_start','row_end'].includes(field.type)">{{field.label}}<span v-if="field.required" class="req">*</span></label>` —— 裸 label，无 for、不包裹输入，是游离文本标签。
  - :149–153 多选组分支：`<div v-else-if="field.type==='checkbox_group'" class="pv-cb-group"><label v-for="opt in parseOptions(field.optionsStr)" :key="opt.value" class="pv-cb-label"><input type="checkbox" :value="opt.value" :disabled="field.disabled" class="pv-cb"/>{{opt.label}}</label></div>` —— 容器仅 class，无 role / aria-label / aria-labelledby。
  - :14 预览按钮 @click=togglePreview；:12 新建按钮 title=新建表单；:966 makeField 默认 checkbox_group 选项 a|A / b|B / c|C；:1078 resetForm 置空 fields。
  - :161 另一分支 section 用 fieldset + legend（对比：该分支有原生分组语义，checkbox_group 分支没有）。
- 正面对照 oa4rust/frontend/apps/desktop/src/components/XformRuntime.vue（sha256 `00c16db74ff6b94177d5319a63a86197e461c883a6cc3c3d6218d9ae83281993`，6241 bytes）：:38 `<div v-else-if="kind(module)==='checkboxes'" class="xform-options" role="group" :aria-required="module.required?'true':undefined">`，:39 label v-for，:40 input type=checkbox —— 同一「复选组」模式在运行时组件明确加了 role=group，并有 R225 契约。
- 全域 role=group 仅 4 处：AttendanceApp.vue:235、BBSForum.vue:66、CalendarApp.vue:11、XformRuntime.vue:38；FormDesigner 预览多选组不在其中。

## 三、判定依据（SC 1.3.1）

- SC 1.3.1 要求：通过呈现传达的信息、结构与关系必须可由程序确定或以文本提供。本例「三个复选框属于名为『多选』的同一字段」是视觉结构与归属关系，必须程序化可确定。
- 判据：容器 AX 若已被浏览器推断为 group（R525 去伪教训），则关系可确定、判 0；若容器 role=none/ignored=true 且无 aria 关联，则字段结构与标题归属不可确定，判可达 1.3.1 缺陷。以下用 CDP getPartialAXTree / getFullAXTree 双向坐实。

## 四、决定性 CDP / AXTree 证据矩阵（双主题一致）

- 报告：oa4rust/test-results/r528/r528-ax-report.json，sha256 `6e49c0c27f18236b6c89e3c0a4ec595a694fb96686dbf997ea2ec7bfb0123f53`。groups=2，fatals=0，evidenceGroups=2，pageErrors=[]（两组）。

| 观测项 | light | dark | 判定意义 |
| --- | --- | --- | --- |
| DOM .pv-cb-group role / aria-label / aria-labelledby | null / null / null | null / null / null | 源码层无任何分组语义标记 |
| DOM groupCount / cbCount / fieldLabel / previewTitle | 1 / 3 / 多选 / 表单预览 | 1 / 3 / 多选 / 表单预览 | 视觉字段与三个选项确实存在 |
| 容器 partial AX | role=none, ignored=true, childIds=3, properties=[] | 同 | 未被推断为 group |
| 容器 full AX | role=none, ignored=true, childIds=3, fullMatched=true | 同 | 全树交叉验证一致，非 partial 采样误差 |
| 三个 checkbox partial/full AX | role=checkbox, name=A/B/C, ignored=false | 同 | 4.1.2 通过，不另计缺陷 |
| DOM 同构对照 partial/full AX | role=group, name=「R528对照分组」, ignored=false | 同 | 证明 AXTree 能暴露 group；容器 role=none 是缺失标记而非工具盲点 |
| 键盘 Tab（首选项→次选项） | a→b, reachedSecond=true | 同 | 原生 checkbox 键盘可达，缺陷不在 2.1.1 |
| pageerror | [] | [] | 无运行时异常干扰 |

- 去伪铁律执行结果：R525 式「浏览器已推断分组语义」假设被证伪 —— 容器 partial 与 full 双通道均 role=none / ignored=true / name 缺失，故不能判 0，指向真实 1.3.1 缺陷。
- AXTree 节点 id（backendNodeId 642/639 等）为会话内临时值，不作为证据引用；以 role/name/ignored/childIds 语义为准。

## 五、截图与取证限制

- 截图 oa4rust/test-results/r528/r528-preview-light.png：1440×900，172910 bytes，sha256 `0992e2e3b78b34401a6148275354d13b2e41a2bfc1c43518845e61dc19828903`。
- 截图 oa4rust/test-results/r528/r528-preview-dark.png：1440×900，216508 bytes，sha256 `ee34a38188a802cbca1c4f14e25a918008c0782ede9afb7a52662bc6db0b6102`。
- 两图均可见「表单预览」弹层内字段标题「多选」与 A/B/C 三个复选框，视觉分组明确；与 AX 层 role=none/ignored=true 形成对比。
- 取证限制：预览弹层为纯前端交互，API 用 stub 占位，不验证后端持久化；AXTree properties 在 Chromium 不返回 aria 属性明细，故以 DOM 属性（role/aria-label/aria-labelledby 全 null）+ partial/full role/name/ignored 三者交叉为决定性依据，不夸大。

## 六、覆盖甄别（未混入其它维度）

- SC 4.1.2：每个复选框有自有可访问名与 checkbox role，通过，不报。
- label 全局增强器角度：未发现为裸 label 自动补关联的机制，但本甄别只认源码与运行时 AX 事实，不臆造增强器行为。
- 2.1.1 键盘：Tab 可达且能在选项间移动，通过，不报；role=button/嵌套交互等其它族不在本任务范围。
- 死代码排除：showFieldTemplates modal（:170/:172/:899 无置 true 入口）不是本判据依赖，预览路径为真实可达入口。
- 修复建议（不在本只读任务内执行）：对 checkbox_group 容器补 role=group 与 aria-labelledby 指向字段标题，或将字段标题改为语义标签并关联容器，参照 XformRuntime.vue:38 与 R225 契约。