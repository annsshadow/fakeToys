# R304 W13/W14 清单治理：crud-view 发现口径对齐

## 来源与范围
- 任务：t46 [R304] 清单治理（attempt 2），依据 t41 用例13 与队长 §220 裁决（m01450/m01453）。
- 本轮只改测试守卫的发现口径与 manifest 元数据描述，不改任何 .vue 实现文件，不改 manifest 的 29 条裁决与 summary 统计。
- 来源声明：<YOUR_TEST_DOMAIN> 测试服（PC /oa4rust、H5 /oa4rust-h5）。截图与冒烟均来自该测试服真实浏览器，非物理真机；口令运行时从凭据文件读取，不入库、不回显。

## 背景与误判纠正
- §219 原描述「manifest 需补 6 个新增 crud-view 条目」方向有误。实测（oa4rust/test-results/r46-enum.cjs）：
  - manifest 中 W13/crud_view 裁决条目 29 条；
  - 按 class token 口径实际含 `crud-view` 的 .vue 页 29 个，与 manifest 一一对称（token_not_adjudicated=[]、adjudicated_not_token=[]）；
  - 旧守卫用精确闭合引号 `class="crud-view"` 只命中 24 页，漏 5 页（CmsDictDesignerApp.vue、DeploymentApp.vue、LogViewerApp.vue、NoteApp.vue、ThreeMemberApp.vue）。
- t41 用例13 曾把 vitest diff 的 expected 索引误读为 manifest 数，写出「实际29/清单23/多6」，是 §219 误判源头；队长 §220 已作废该描述并按本方案入账。

## 根因
- R300/R302 AI 重构给这 5 个真实 CRUD 页的根 div class 加了 AI 令牌后缀，例如：
  - CmsDictDesignerApp.vue:5 `class="crud-view ai-page-b5"`；
  - DeploymentApp.vue:5 `class="crud-view modern-page ai-panel ai-page-b5"`；
  - LogViewerApp.vue:5 `class="crud-view ai-page-b5"`；
  - NoteApp.vue:5 `class="crud-view modern-page ai-panel"`；
  - ThreeMemberApp.vue:5 `class="crud-view ai-page-b5"`。
- 页面仍是正确的 crud-view，只是 class 增加了合法后缀；守卫以「精确闭合引号」判定发现集合，导致发现口径与裁决口径失配。

## 改动（严格最小）
1. oa4rust/frontend/apps/desktop/src/views/w13-w14-closure.test.ts:41-47：
   - 新增 `hasCrudViewToken(source)`：遍历所有 `class="..."` 属性值，按空白 split，要求存在 token 精确等于 `crud-view`。
   - `exactCrudViews()` 的 filter 改用该函数。
   - 不接受子串：`crud-view-foo`、`xcrud-view` 不匹配；`foo crud-view bar` 匹配。避免把发现口径退化为宽泛子串。
   - 不变量保持：adjudicated 集合 == 发现集合、无重复条目、W14 家族、分类一致性、implemented 证据、薄 CRUD 不得标 implemented、W13 视图在 main.ts 可达等其余 5 个用例原样未动。
2. docs/audits/w13-w14-repository-closure.manifest.json 的 discovery.crudView：
   - 保留 original exactMarker 历史值；新增 `token: "crud-view"` 与 `matchRule` 文本，说明按 class token 精确匹配、覆盖 AI 后缀形态、子串不匹配。
   - entries 44（W13=29/W14=15）、summary total44/byWorkItem{W13:29,W14:15}/byStatus{implemented39,out_of_scope5} 完全不变。

## RED → GREEN 证据
- RED（改动前）：`pnpm exec vitest run frontend/apps/desktop/src/views/w13-w14-closure.test.ts` 1 failed / 5 passed，失败用例「covers every exact crud-view and rejects unadjudicated additions」，AssertionError: expected [ 'CmsColumnApp.vue', …(28) ] to deeply equal [ 'CmsColumnApp.vue', …(23) ]，失败点 :76 `expect(sorted(adjudicated)).toEqual(sorted(exactCrudViews()))`，diff +5 即上述带 AI 后缀的 5 页。
- 口径验证（oa4rust/test-results/r46-token-probe.cjs，gitignored）：FILES=75/TOKEN_MATCHES=29/EXACT_MATCHES=24/ADJUDICATED=29/TOKEN_NOT_ADJ=[]/ADJ_NOT_TOKEN=[]/TOKEN_EQ_ADJ=true；伪例 false/false/true 证明 token 精确而非子串。
- GREEN（改动后）：`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/w13-w14-closure.test.ts --reporter=verbose` exit 0，Test Files 1 passed、Tests 6 passed（6 用例全绿）。
- manifest JSON 校验：VALID_JSON=true、W13_CRUD=29、TOTAL=44、summary 不变。

## 测试服冒烟
- 本轮只改测试发现逻辑与文档元数据，不改任何运行时实现，测试服已部署产物无需重建。
- 真浏览器冒烟脚本 oa4rust/test-results/r46-smoke.cjs（gitignored）：Playwright Chromium headless 1440x900，运行时从凭据文件读口令；依次真 UI 登录后访问 /app/note、/app/admin/log-viewer、/app/admin/cms-dict-designer、/app/admin/deployment、/app/admin/three-member。结果 R46_SMOKE {"captured":5,"pass":true,"pagesWithCrudToken":5}：5/5 页 bodyLen>0、pageerror=0、apiFailure(≥400)=0，且 DOM 中 5/5 页均解析到精确 `crud-view` class token，佐证 token 发现口径与真实页面一致。
- 截图与诊断留 gitignored 证据目录 oa4rust/test-results/r46-smoke/（01-note.png 至 05-three-member.png + evidence.json），来源标注：<YOUR_TEST_DOMAIN> 测试服真实浏览器、非物理真机、非静态假数据。

## 脱敏与合规
- 文中不出现真实域名/IP/主机名/口令/私钥；统一用 <YOUR_TEST_DOMAIN> 占位。
- 口令与会话数据不入库、不回显；凭据仅运行时从本地凭据文件读取。
- 不声称项目完成；≥1000 轮持续迭代，本轮为其中一环。
