# R352 H5 搜索页双来源信封解包修复

> 环境：测试服 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 均为占位符，0 真值入库。真浏览器证据为 Playwright Chromium headless 390x844 isMobile/hasTouch/DPR2 对已部署测试服的真实渲染，非物理真机。

## 1. 范围与根因

- 页面：mobile/src/pages/search/search.vue 的 doSearch() 两个来源。
- R351 审计与真浏览器双重确证：全局 POST /api/queryview/search 返回 data 为 designList/viewList 分组对象，论坛 PUT 论坛主题搜索端点返回 data 为内层 data 数组加 total 的对象；页面用 x.data ?? 空数组 直接 map 遍历对象，任何关键词均抛 map is not a function，被各自 try/catch 转成晦涩错误文案，真实结果恒 0 卡片、空态被错误掩盖。

## 2. 方案 A 裁决（fail-loud）

- 全局来源：env 取 g.data 或空对象，断言 designList/viewList 可选数组，rowsG 由两数组各自 Array.isArray 判空后展开拼接，再 map 加 _from 全局。
- 论坛来源：rowsB 先判 b.data 是否数组，是则直接用；否则判其内层 data 是否数组，是则取内层；否则空数组。再 map 加 _from 论坛，兼容后端偶发单层数组。
- 两来源仍各自 try/catch，失败文案 describeLoadError 与两来源都失败保留旧结果、有成功以本次为准的语义不变；空态模板与论坛深链 bbs/topic 不变。
- 硬边界：未改 services/index.ts 的 list 传输层（类型断言层影响十余端点）、http.ts、后端、SDK、desktop、其它页、migrations；端点路径与 payload 未变。实测信封形状与裁决一致（R351 v2 探针已坐实），无改向。

## 3. 修复内容

- mobile/src/pages/search/search.vue 两处解包替换原直接 map，仅新增局部变量与 Array.isArray 判空。
- 契约 mobile/src/pages/search/search.envelope.contract.test.ts（新建）：源码级 bodyOf 花括号配对提取 doSearch 函数体，正则剥离 TS 断言后 new AsyncFn 真执行注入 searchApi.global/bbsSubject/describeLoadError 桩；6 it 覆盖双层解包与 _from 标记、坏数据与单层兼容、单源失败不阻断另一源且无 map 报错文案、双空结果无错误（空态不被掩盖）、关键词 trim 与端点参数不变、写读链保护。自定义函数参数均带冒号 string 标注防 TS7006。

## 4. RED → GREEN 台账

- RED：cd oa4rust/mobile 后跑定点契约 exit=1，5 failed / 1 passed（原始遍历对象，错误文案含 map is not a function）。首轮契约按字面 split 剥离断言，源码加解包后残留 as 断言致 Unexpected identifier as，改通用正则后业务断言生效。
- GREEN：同命令 exit=0，6/6 passed。
- 全量：mobile 全量 vitest exit=0，73 files / 469 tests 全 passed；根全量 vitest exit=0，276 files / 2210 tests 全 passed；mobile tsc --noEmit exit=0。

## 5. 真浏览器复验（390x844 非物理真机）

- 脚本 test-results/r352-h5-search-check.cjs（gitignored）：登录 → 搜索页 → 关键词 a → 空结果词 → 再搜 a 点论坛卡深链；两来源真实 fetch 均 200。
- 10 项断言全 true：pageLoaded/keywordACards/noMapError/globalTagVisible/forumRowsVisible/emptyStateShown/searchApisOk/deepLinkReached/noPageErrors/noApiFailures，failed 为空。
- 关键词 a：卡片 7（全局 3 + 论坛 4），body 无 map 报错；空结果词：卡片 0，出现未找到结果空态且无错误文案；论坛深链补测（R351 取证限制点）：点到 bbs/topic 且主题 view 与 reply/list 端点 200；pageErrors 空、API 失败 0。
- 截图 test-results/r352-h5-search-envelope/01-keyword-a-cards.png、02-empty-state.png、03-forum-deeplink.png（均 390x844 headless 非物理真机，evidence.json source 明标 NOT physical device）。

## 6. 取证限制

- 空态关键词按真实 UI 输入随机不存在词取证；未覆盖 confirm 触发路径（tap 已坐实，R351 同口径）。
- 论坛深链仅验证跳转与主题只读端点 200，未做发贴/回复等破坏性写（未授权）。
