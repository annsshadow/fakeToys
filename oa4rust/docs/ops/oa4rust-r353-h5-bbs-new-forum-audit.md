# R353 H5 发帖页论坛版块列表信封甄别审计

> 环境：测试服 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 均为占位符，0 真值入库。真浏览器证据为 Playwright Chromium headless 390x844 isMobile/hasTouch/DPR2 对已部署测试服的真实渲染，非物理真机。

## 1. 任务与范围

- 任务 t163 / R353：只读甄别 mobile/src/pages/bbs/new.vue 的论坛版块列表数据源是否被分组信封破坏（R344/R345/R351 多 list 端点分组信封同型风险）。
- 唯一改动：本文档。未改源码 / 契约 / 后端 / SDK / 台账 / 其它页。破坏性真发帖未授权不执行（R317 已覆盖发帖 API 层）。
- 硬边界：凭据经 ssh oa4rust-test 运行期读取、不回显、不入库；文档域名 0 真值。

## 2. 覆盖甄别（任务描述称 0 命中不成立，实际命中但非本轮口径）

- docs/ops grep forumList / forum/view/all / bbs/new.vue 命中：R317 h5-deep-audit.md:28（记录发帖 picker 数据源走 GET forum/view/all，当时判后端返回空系数据表无行、非解析缺陷）；R318 bbs-section-forum-audit.md:24/49；R319 legacy-section-triage.md:25（头本地 picker 曾显示 6 个历史版块，非物理真机）；R320 symmetric-delete-fix.md:30（forum/view-all 删除后 picker 缓存需 h5.reload 硬重载）。
- 均为版块列表不同维度的记录，无一实测当前测试服 forumList 响应信封形状与 UI picker 的对应关系。本轮首次专测，有效增量。

## 3. 源码梳理

- mobile/src/pages/bbs/new.vue:12 forums ref；:16 forumIndex=-1；:22-32 loadForums() 中 :24 const resp = await bbsApi.forumList()，:25 forums.value = (resp.data ?? []).map(f => ({id, name})).filter(f => f.id)，:28-31 catch 置 forums.value=[] 并 uni.showToast(论坛列表加载失败)；:29 注释明确版块为发帖必选数据源、失败必须提示。
- :20 canSubmit = forumIndex>=0 && title.trim() && content.trim() && !submitting；:83 picker :range = forums.map(f => f.name) @change=onForumChange。
- mobile/src/services/index.ts:541 forumList = () => list(mapi.get(/api/bbs/assemble/control/forum/view/all))；list() 传输层仅类型断言不解包。
- 缺陷假设：若 forum/view/all 返回分组信封，resp.data 为对象，:25 的 .map 抛 TypeError，被 :28 catch 转成论坛列表加载失败 toast、picker 恒空。需实测证伪或坐实。

## 4. 真浏览器实测（390x844 headless 非物理真机）

- 脚本 test-results/r353-h5-bbs-new.cjs（gitignored）：ssh 取凭据登录 → 进入 #/pages/bbs/new 即刻轮询 uni-toast → S0 页面 body + 提示文本 → 页面上下文 fetch GET /oa4rust-api/api/bbs/assemble/control/forum/view/all 做 deep 探针 → S1 picker 当前值 + 点击展开读选项文本 → S2 填标题内容观察 submit 守卫 → 四张截图。
- 后端 deep 探针：status=200；topKeys=[data,type,message,date,spent,size,count,position]；dataType=object；dataIsArray=true；innerKeys=null；内层直取 data 数组 len=8，sample 含 id/name（如 R317H5正式版块、R318甄别版块系列）。即该端点返回裸数组信封，非分组对象。
- API 记录：/oa4rust-api/api/bbs/assemble/control/forum/view/all = 200（authentication/who、login 亦 200）；pageErrors=[]。
- UI S0：body=发布主题 / 版块 / 请选择版块 / 标题 / 请输入标题 / 内容 / 请输入内容 / 发布主题，toastTexts=[]，无加载失败提示。
- UI S1：picker 当前值=请选择版块；点击展开后选项文本读得 8 个真实版块名（R317H5正式版块、R318甄别版块 x3、R318桥接专测 x2、R320孤儿甄别 x2），与后端裸数组 8 条一一对应；hasLoadFail=false；hasSelectPlaceholder=true。截图 new-picker-expanded.png 肉眼可见 8 项真实版块名逐条渲染。
- UI S2：填入标题内容后 submit class=submit ai-brand-fill disabled（未选版块时 canSubmit 恒 false，守卫符合预期，未执行真发帖）。
- 截图 test-results/r353-h5-bbs-new/{new-initial,new-picker-before,new-picker-expanded,new-filled}.png，均 390x844 headless 非物理真机；evidence.json source 明标 NOT physical device。

## 5. 三分判定

- 通过（0 真缺陷）：forumList 返回裸数组（dataIsArray=true, len=8），前端 :25 单层解包 resp.data ?? [] 正确；picker 渲染 8 个真实版块名且与后端一一对应；无论坛列表加载失败提示；0 pageerror；提交守卫在未选版块时正确禁用。R344/R345/R351 分组信封同型缺陷在本页不成立（forum/view/all 是裸数组端点，与设计器 search 类端点信封不同）。无需修复。
- 取证限制：破坏性真发帖未执行（未授权，R317 已覆盖发帖 API 层），仅验证列表数据源与提交守卫；S1 读得 16 项文本系 uni-picker 双层 DOM（选择列 + 隐藏列），截图肉眼可见真实 8 项，非数据重复。
- 产品语义差异：无。

## 6. 结论

- R353 H5 发帖页论坛版块列表数据源甄别完成：0 真缺陷、0 P0/P1。任务假设的分组信封未解包缺陷被真浏览器探针证伪——该端点为裸数组、前端单层解析正确、picker 正常渲染真实版块名。
- 保持 R352 解包范式仅用于确属分组信封的端点，本页不适用，不改码。
