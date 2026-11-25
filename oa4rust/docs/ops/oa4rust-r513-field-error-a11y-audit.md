# R513 WCAG 3.3.1 / 4.1.2 表单错误程序化关联甄别

## 1. 依据与判据

WCAG 3.3.1（Error Identification）要求错误以文本呈现并能被用户识别；WCAG 4.1.2（Name, Role, Value）要求用户代理可程序化确定组件状态。本轮甄别字段保存失败错误是否仅视觉呈递：错误文本须有 role=alert 或等价语义，并与出错控件建立 aria-invalid / aria-describedby 程序化关联。

已有基础设施：oa4rust/frontend/apps/desktop/src/utils/formErrorA11y.ts（R188）全局增强器，扫描带 data-oa-field-error 标记的错误文本，补 role=alert + 稳定 id，并给同字段控件补 aria-invalid=true / aria-describedby（findControls :52-72 从错误节点向上找含控件的容器），MutationObserver 驱动；main.ts:614 已安装。Personal.vue:50（R489 修复）已带该标记，可作正面对照。

反预判铁律：严禁仅凭源码是否带标记就判缺陷，必须真浏览器坐实错误态真实出现、DOM 属性与 CDP AXTree 语义。区分「检测器错误」与真实缺陷，前者据实披露不作证据。

## 2. 方法与基线

- 真浏览器：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，非物理真机。
- 取证：CDP DOM + Accessibility.getPartialAXTree + DOM 属性（marker / role / id / aria-invalid / aria-describedby / label for），四轮（主轮4组+补采2组）共 0 pageerror / 0 fatal。
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，真父 commit b156b03bcd807a58f23916b93fa50d6734974d72，HEAD 与远端对齐，工作树仅非本人未跟踪文件。
- 部署域名与凭据真值仅存在于 gitignored 本地 runner（环境变量注入），本报告用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，0 真值。
- 源码 sha256：BBSForum.vue=78955ea8303e564ca72cb6311052c39a29b882a0fb68543273302f120030bb84；Personal.vue=8a323a15ea9e3c66f8114dede3fbb38f403502a131c23c7fe75bdfe58088d9b1；formErrorA11y.ts=11e8cbb32f50e7d699a0eb14ad69c966ed7182b09e1a87d6455552de04613b05。

## 3. 逐项程序化证据

### 3.1 BBSForum 版块保存失败（主轮 report.json）

- 源码：BBSForum.vue:194 form @submit.prevent=saveSection；:196-198 label for=bbs-section-name + input id=bbs-section-name；:200 <div v-if="sectionError" class="error-msg">{{ sectionError }}</div>，无 data-oa-field-error；:400 catch 分支 sectionError.value=e instanceof Error?e.message:保存失败。
- 主轮用真实 UI 两次新建同名版块：两次均 toast「✓版块已创建」（role=status / aria-live=polite），因后端允许重名未返回错误，sectionError 未出现（error=null、input=null、cdp nodes=[]）。据实披露：重复名不是该后端的失败路径，不能据此判定错误态语义。
- 主轮正面对照 Personal（见 3.3）正常，说明检测器与增强器工作正常。

### 3.2 BBSForum sectionError 真实触发（补采 supplement.json，故障注入）

- 方法：真浏览器内 page.route 拦截 section/create 请求并 fulfill HTTP 500（type:error），使 api.post 抛异常走 saveSection catch 分支，sectionError 真实出现——这是真实可达的异常路径（网络/服务端故障会发生），非静态推断。injectedRequests=1/主题，0 pageerror。
- 实测（light/dark 一致）：modalLabel=新建版块、modalTitle=新建版块；error div tag=div / cls=error-msg / text="HTTP 500: Internal Server Error" / visible=true / marker(data-oa-field-error)=false / role="" / id="" / aria-live=""；input id=bbs-section-name / value=R513-故障注入-{theme} / labelFor=true / ariaInvalid=null / aria-describedby=""（空）。
- CDP AXTree：error 节点 axRole=generic / axName 空 / axIgnored=false；input axRole=textbox / axName=版块名称 / axIgnored=false；modal axRole=dialog / axName=新建版块 / axIgnored=false。
- 目视（1440x900 非物理真机）：新建版块浮层内红色错误文本 HTTP 500: Internal Server Error，仅视觉呈递；读屏用户无法从错误节点获得 alert 语义，name 输入框也无无效态/描述关联。
- 字段级判定：错误 div 位于单字段 modal 表单内、紧邻 name 字段 form-group 之后，全局增强器 findControls 从错误节点向上可达 form.modal-body 内唯一 input，符合字段级关联条件（队长确认此处按字段级，非多字段提交级 createError）。

### 3.3 Personal 正面对照（report.json）

- 源码：Personal.vue:50 <div v-if="pwdError" class="error-msg" data-oa-field-error>，R489 已修。
- 实测（light/dark 一致）：错误 text="两次密码不一致" / marker=true / role=alert / id=oa-field-error-000001；CDP axRole=alert / axIgnored=false；三个密码框 oldPwd / newPwd / confirmPwd 均 aria-invalid=true、aria-describedby=oa-field-error-000001。
- 结论：同一增强器在带标记时正确工作，反证 BBSForum 缺陷根因是缺少标记，而非增强器失效或检测器误判。

## 4. 三分判定

- **可达真缺陷：1 项 P2。** BBSForum.vue:200 版块保存失败错误（sectionError）缺 data-oa-field-error 标记：错误真实出现时仅 role 空 / axRole=generic / 无 aria-live，name 输入框无 aria-invalid / aria-describedby，读屏用户无法程序化识别错误与出错字段；light/dark 双主题故障注入实证。非破坏性、不影响 sighted 用户操作，故 P2 而非更高。
- **不可达 / 未覆盖 L1：1 类。** BBSForum.vue:133 createError（发主题多字段场景）本轮不判：队长明确不一并纳入，且该场景与字段级单字段关联不同型，据实披露为未覆盖，不臆判为缺陷。另：主轮重复名因后端允许重名未触发 sectionError，属数据/服务端行为，据实披露不作缺陷证据。
- **正常规范预期：4 类。** ① Personal.vue:50 R489 修复生效（marker+role=alert+三框程序化关联）；② BBSForum label for=bbs-section-name 与 input id 正确、axName=版块名称；③ modal role=dialog / axName=新建版块 语义正确；④ 成功/业务失败 toast role=status / aria-live=polite 语义正确（属 3.3.1 toast 范畴，非本轮缺陷）。
- 反证边界：仅抽测 BBSForum 版块保存与 Personal 密码修改，全库其他保存失败点不作外推。

## 5. 修复建议

BBSForum.vue:200 的 <div v-if="sectionError" class="error-msg"> 开标签加 data-oa-field-error（与 Personal.vue:50 一致），即可纳入已安装的全局增强器：自动补 role=alert + 稳定 id，并给 bbs-section-name 输入框补 aria-invalid=true / aria-describedby；业务逻辑零改。回归验证：故障注入保存失败后，CDP AXTree 应见 error axRole=alert 且输入框 aria-invalid=true、describedby 指向该 error id，错误解除后属性还原。契约测试可复用 R489 personalErrorA11y.contract.test.ts 范式。

## 6. 证据与 sha256

全部 PNG 与 JSON 经命令行 sha256sum 核验一致：

- report.json=14d3464e00db53248b96f5c85505b6ec696cdb370e37d20d8bf9a020c3fadeb6；supplement.json=dd7b72cdaa40cfd2e06a0d7cc2a9c11a051803447eb16e7e70c6594101d86e40
- pc-bbs-section-light-r513.png=9d523d0fda663006b7221992147e2a02cb25d9f531585348e8317c82a2ed52a3 / dark=574c06ada247c41bb9d0b2e5e31f01cff7130ce204fbfd382e260eb1a020caf6
- pc-personal-error-light-r513.png=277e439be976d482a8fede8db3ce28cf01d7d79d1ac8928481ca1982e7a8f665 / dark=4396d28e94c8567c90086b26c65d74f973e72c772a72a273efa159217bdf2c4b
- pc-bbs-section-error-light-r513-sup.png=a5a2c1fd47d654db19fadbeca11ab7ec49c9ff20db530954d77da2eff1873f94 / dark=e82ffe598414f50f9b7cf95cb72bf3beaae4695e4def6d9beaccf306ac4a933d

## 7. 脱敏边界

本报告与证据分离：真域名、账号、口令、SSH 信息仅存在于 gitignored 本地 runner 与证据 JSON，文档仅用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，无任何真值入库。截图均为 headless 非物理真机，据实标注。故障注入为浏览器内请求拦截，未修改服务端数据，据实披露。