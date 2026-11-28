# R638b NoteApp 笔记成员列字段错配甄别（P3）

## 结论
可达真缺陷 1 项，判 P3：NoteApp 笔记列表「成员」列恒显示占位符「团队成员」，用户填写的成员值丢失。根因为纯前端字段错配——模板渲染读 `item.personName`，而表单、接口与后端只提供 `personId`，值域无交集。数据正确入库并返回，无越权、无主流程阻断，属低优先级显示/可用性缺陷。

## 真父与写域
- 真父 HEAD=`2b22ce40f`（分支 opt1000/oa4rust-pc-ai-refactor）。
- 零源码改；唯一产出本文 `oa4rust/backend/docs/audits/oa4rust-r638b-note-personname.md`。临时 vitest 夹具测后即删，git 工作树零残留。

## 机制锚点（逐行核实）
- `oa4rust/frontend/apps/desktop/src/views/NoteApp.vue:26` 表头「成员」；`:31` td[data-testid=note-member-cell] 渲染 `{{ item.personName || '团队成员' }}`。
- 同文件 `:46` 表单 input `v-model="form.personId"`，placeholder「成员姓名/工号」；`:69-77` interface Item 仅 id/title/content/personId/status/updateTime/createTime，**无 personName**。
- 同文件 `:82` listEp=`/api/cms/core/entity/note/list`；`:101` `api.get(listEp)`。
- 后端 `oa4rust/backend/crates/cms_core_entity/src/lib.rs:424-428` x_cms_note 分支：`SELECT id, title, content, person_id AS personId, status, create_time::text AS createTime, update_time::text AS updateTime FROM x_cms_note WHERE deleted_at IS NULL AND (person_id=$1 OR $2::boolean) ORDER BY create_time DESC, id`——返回 `personId`，**无 personName**（create:552-555 / save:601-603 亦写 person_id）。
- 兄弟正确渲染：同文件 `:29` `item.title`、`:32` `fmtTime(item.updateTime)`；`Personal.vue:63` 有 `sm.personName||sm.name||sm.id` 兜底链，本处缺。
- 路由 `main.ts:114 /app/note` 普通登录可达；create/save 需 admin（后端 require_admin_opt）但 list 不需，普通用户可见此列。台账无专项甄别，与 R637 note 列表归属门安全修复不同维度，不重复报。

## 决定性真测试（vitest + jsdom + ts.transpileModule + new Function）
夹具 `r638b_note_personname_audit.test.ts`：readFileSync 读取源码，从 `:31` 提取 `{{}}` 内表达式，经 ts.transpileModule→new Function **真实执行**渲染表达式，非静态比对。命令（`oa4rust` 根目录）：
`./node_modules/.bin/vitest run frontend/apps/desktop/src/views/r638b_note_personname_audit.test.ts --reporter=verbose`
结果 EXIT=0，Test Files 1 passed，Tests 5 passed / 0 failed。决定性输出：
- `R638B_EXPR=item.personName || '团队成员'`；`R638B_IFACE` 仅 personId 无 personName；`R638B_BACKEND_BRANCH` 含 `person_id AS personId` 无 personName。
- 臂2 `R638B_ARM2_CELL_PERSONID_ONLY=团队成员`：mock api.get 返回 `{personId:'u-123'}` → 单元格显示占位，不含 u-123（填写值丢失）。
- 臂3 `R638B_ARM3_CELL_WITH_PERSONNAME=张三`：返回 personName 字段则正确显示，证明仅字段错配。
- 臂4 `R638B_ARM4_DOM_TEXTCONTENT=团队成员`：jsdom 真实 td[data-testid] 的 textContent 为占位。
- 臂5 兄弟 Personal.vue 兜底链存在、本处缺失。

## 三分裁决
- 可达真缺陷：1 项 P3——字段错配致成员列恒占位、填写成员值在 UI 丢失；数据正确入库与返回、无越权、无主流程阻断，普通用户可见（create/save 需 admin 不影响 list）。
- 不可达 L1：0。
- 正常：3 类——标题/正文/更新时间按 item.title、item.content、fmtTime(item.updateTime) 正确渲染；有 personName 时成员列正确显示；兄弟 Personal.vue 兜底链范本存在。
- 减级理由：非越权、非阻断、无数据损坏；与 R637 归属门安全修复不重复报；不预支 P1，亦不降为 L1。

## 行为现状与复现
登录用户访问 /app/note，表单填写成员后提交（提交需 admin），列表「成员」列恒显示「团队成员」。直调 GET /api/cms/core/entity/note/list 响应中该行含 personId 而无 personName，前端按 personName 取值落空回退占位。测试环境以 `<TEST_HOST>:<PORT>` 占位，未写入任何真实域名/IP/凭据。

## Fail Loud
未做 GUI 端到端点击，改用 vitest+jsdom 真实执行渲染表达式取证（等价覆盖字段取值与 DOM 落位）；未跑全量前端回归，仅定点 5 用例，据实披露。

## SHA（分列，据 meta 不可复算者据实分列）
- docSha=文档全文 sha256，由提交后复算，不在文内自引用（见报告）。
- runnerSha=`6d2e5baf7b1540cd9bd9f8da2edad3d60a12a6f911c2fa40aab3209434eb0f6d`（sha256(JSON.stringify(meta))）。
- fixtureSha(内容)=`d6031281fd7f545efb7a31364d8a099cecfbb87e925b42f9380ecb531a74d895`；oneshotLogSha=`50858265b1fa1e2f9a73755b09c45cdc55db681ca73cac617a8c1b573481d60b`。
- 0 真值，占位 `<TEST_HOST>:<PORT>`，LEAK=false。

## 修复方向
前端将成员列改读 `item.personId`（并按需 JOIN 映射可读姓名），或后端 x_cms_note 分支补 `person_name` 字段；二者择一即可闭合值域。本文仅甄别，不改源码。
