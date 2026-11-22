# R346 PC 未审计页批量甄别（首轮：CMS 设计器五页）

## 来源与范围
- 任务：t153 [R346] 只读深度审计，20 个未审计页首轮聚焦 CMS 设计器五页。
- 对象：frontend/apps/desktop/src/views/ 下 CmsColumnApp.vue / CmsDictDesignerApp.vue / CmsFormDesignerApp.vue / CmsViewDesignerApp.vue / CmsXformApp.vue。
- 路由（main.ts，均在 /app/admin/ 下）：cms-column / cms-dict-designer / cms-form-designer / cms-view-designer / cms-xform。
- inScope：逐页真实写端点静态梳理、测试服真浏览器只读与代表性安全写闭环（新建回显／编辑 save 持久／删除确认守卫取消不真删）、list 信封形状探针、加载失败兜底、三分判定。
- 硬约束：只读审计，未改源码/契约/后端/配置/组件/台账/其它页；唯一入库物为本文档；脚本与证据落 gitignored test-results/r346-unaudited-cms/。
- 凭据仅运行期由 ssh oa4rust-test 读取、不回显；测试域名用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，仓库无真值。
- 截图与 evidence 均来自本地 Playwright headless Chromium 1440x900 真浏览器实测，非物理真机。

## 覆盖甄别（首轮五页此前为 0 审计覆盖）
- docs/ops 全库对上述 5 个 view 名 grep 命中 total=0，确认无八项写路径审计覆盖。
- R306 oa4rust-r306-admin-a-visual.md 仅视觉改造基线；R314 契约漂移为静态枚举；R332 默认插槽甄别点名部分页但非写闭环。
- 结论：五页写闭环此前 0 真浏览器覆盖，本轮为首次。

## 静态梳理（五页写端点，均同构）
- CmsColumnApp：list GET /api/cms/core/entity/column/list(:102)、create POST .../create(:103)、save PUT .../save/${editingId}(:160)、delete POST .../delete/${id}(:177)，单层 queryFn(:121-123)。
- CmsDictDesignerApp：list GET /api/cms/assemble/control/dict/list(:89)、create POST .../create(:90)、save POST .../save/${editingId}(:151)、delete POST .../delete/${id}(:168)，单层(:109)。
- CmsFormDesignerApp：list GET /api/cms/assemble/control/form/list(:94)、create POST .../create(:95)、save POST .../save/${editingId}(:155)，单层(:114)。
- CmsViewDesignerApp：list GET /api/cms/assemble/control/view/list、create POST .../create、save POST .../save/${editingId}，单层。
- CmsXformApp：list GET /api/cms/assemble/control/xform/list(:94)、create POST .../create(:95)、save POST .../save/${editingId}(:155)、delete POST .../delete/${id}(:172)，单层(:115)。
- 五页共同结构：Admin(create-label=+ 新建) + ListLoadError banner/empty + AdminTable；具名插槽 #cell-name/#cell-app_id/#cell-actions 等（非默认插槽丢弃缺陷）；useQuery + useMutation + useQueryClient；loadData()=reload()。

## 真浏览器实测（1440x900 headless Chromium，非物理真机）
- 两轮：首轮脚本 r346-unaudited-cms.cjs 行选择器误用 table.data-table tbody tr（真实行 class=.admin-table-row）致 rows=0；有效证据仍含 S0 裸数组探针、S1 新建回显、S4 500 兜底。
- 补测 r346-unaudited-cms-v2.cjs 修正为 .admin-table-row，五页全通过：
  - CmsColumn：rows 1→2、createEcho=true、编辑 modal=1 且 reload 后 echo=true、删除守卫 overlay=1+canceled+delCalls=0。
  - CmsDict：rows 2→3、createEcho=true、编辑 echo=true、删除守卫全 OK。
  - CmsForm：rows 13→14、createEcho=true、编辑 R346V...CmsForm-EDIT reload 后持久可见、删除守卫全 OK。
  - CmsView：rows 1→2、createEcho=true、编辑 echo=true、删除守卫全 OK。
  - CmsXform：rows 14→15、createEcho=true、编辑 echo=true、删除守卫全 OK。
- 原始写端点状态（evidence-v2.json）：五页 create 与 save 全 HTTP 200（column/create=200、PUT column/save/{id}=200、dict/create=200、POST dict/save/{id}=200、form/create=200、POST form/save/{id}=200、view/create=200、POST view/save/{id}=200、xform/create=200、POST xform/save/{id}=200）。
- 首轮 status=null 系脚本 endsWith(/save/) 未匹配带 id 后缀的真实 URL，非业务问题，原始日志已坐实 200。
- S0 list 原始信封探针：五页 dataType=array / dataIsArray=true / innerIsArray=false，即后端返回裸数组 data[]，前端单层 Array.isArray(r?.data) 解析正确，排除 R340 双层信封致列表恒空缺陷。
- S4 500 兜底：route fulfill 500 后 .list-error-empty 出现，文案含加载失败语义，与空态可区分，PASS。
- 0 pageerror（两轮均为 []）。
- 截图证据：CmsColumn-v2-created.png（列表 2 行新建名可见）、CmsForm-v2-guard.png（编辑名 R346V...CmsForm-EDIT 持久可见）、以及各页 list/created/guard/fail500 截图，均 1440x900 headless。

## 三分判定
- 【真缺陷】0 个：五页 CRUD 闭环、信封解析、具名插槽、加载兜底、0 pageerror 全部通过。
- 【取证限制】破坏性删除仅验证确认守卫（#oa4-confirm-overlay 出现 + 取消后 delCalls=0），未对真实数据执行 DELETE；写端点只走代表性新建/编辑，未逐条覆盖五页全部端点变体。
- 【产品语义差异】CmsXform 与 CmsForm 列表展示同源种子数据（seed-leave-form 同 uuid/definition），xform/save 与 form/save 用同 id（da3e1fb7...），属后端数据同源设计，非前端缺陷；五页普通 useQuery + 正确 queryKey，实测回显正常，无 R342 queryKey 不匹配缺陷；list 裸数组单层解析正确，无 R340 双层信封/字段大小写失配缺陷；具名插槽正常消费，无 R332 默认插槽丢弃缺陷。

## 结论
- 首轮 CMS 设计器五页批量甄别完成：0 新增真缺陷、0 新增 P0/P1。
- 最小修复方向：无需修复；后续轮次继续覆盖剩余 15 个未审计页。