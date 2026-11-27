# R599 文档筛选失效验证型甄别审计

- 标识: oa4rust-r599-document-filter-audit / 轮次 R599 / 任务 t161
- 类型: 验证型甄别（零源码改动，唯一脱敏审计文档）
- 父提交: a454ae4b5cf68e6632382a8346e5dfa2bba9e6e5
- 结论: P2 —— 文档筛选（状态页签 + 关键词搜索）失效；统计数字随页签变化但列表数据不变，构成界面误导。非安全问题、非越权。

## 一、立项与范围
台账 docs/ops/current-state.md 仅在 3969 行记录 R302 改端点修 400、4824/4826 行记录 R331 补 deleted_at IS NULL（commit 7769a170）、5608 行记录 R599 立项；全量 grep 确认从未对 type/keyword 筛选语义做过专项甄别。本次仅甄别，不改任何业务源码。

## 二、机制锚点（逐行核实）
- 前端 oa4rust/frontend/apps/desktop/src/views/DocumentApp.vue:9-10 页签按钮 @click 只改 tab 变量，全文件无 watch(tab)，切页签不触发 doSearch。
- 同文件 :188-191 stats 计算属性 label 随 tab 变、value 恒为 items.value.length，界面误导。
- 同文件 :208-220 doSearch 发 params.keyword(:215) 与 params.type=draft(:216)，POST /api/document/filter/list/1/size/50/manager(:219)。
- 后端 crates/cms_assemble_control/src/lib.rs:11606-11618 document_filter_paging_manager_u3(pool:Extension<Pool>, session:Extension<Session>)：先 u2_require_admin，再 list_from_table_filtered_legacy 传硬编码 where（deleted_at IS NULL ORDER BY create_time DESC LIMIT 200）与空参数数组；无 Json 提取器，请求体被 Axum 丢弃。
- 路由 crates/cms_assemble_control/src/routes.rs:645 挂载该 POST manager 端点。
- 决定性根因：非 manager 变体 document_filter_paging_u3(:11592-11602) 同样无 Json 提取器、同样硬编码 where，改用该端点也无效。根因是后端两变体都不读 body、都无 type/keyword 过滤语义，而非前端选错端点。
- 辅助函数 list_from_table_filtered_inner(:401-462) 按 ORDER BY/LIMIT 切分 filter_only 与 tail，绑定参数为空数组。

## 三、方法
1. 静态逐行核实上述锚点与台账历史。
2. 临时 PostgreSQL（唯一容器名 + 高位端口，真值不入仓）运行 migrations，seed 管理员 person/role 与三篇文档（draft / published / 含关键词）。
3. 用测试 Session 注入 Router Extension，oneshot POST 两变体端点并带请求体 keyword + type=draft，断言应只返回匹配项。
4. 真浏览器 E2E：Playwright Chromium headless 加载真实前端 bundle，stub 认证与后端（后端全量返回行为由第 3 步真测试坐实），核实页签切换与搜索的界面表现。

## 四、决定性实测
后端真测试：DATABASE_URL 指向临时库（占位），cargo test -p cms_assemble_control --test r599_document_filter_audit -- --nocapture --test-threads=1 → 2 passed / 0 failed（10.82s）。

| 用例 | 状态 | 返回 ID | 全量返回 |
| --- | --- | --- | --- |
| manager_list_ignores_type_and_keyword | 200 | r599-doc-keyword、r599-doc-pub、r599-doc-draft | true |
| non_manager_variant_also_ignores_filters | 200 | 同上三篇 | true |

注入 type=draft 本应过滤掉 published，keyword 本应只留含关键词文档，但两变体均返回全部三篇，坐实过滤被忽略。
真浏览器 E2E（证据 evidence.json）：s0/s1/s2 行数为 3、标题恒为 PUB_MARK 普通已发布 / DRAFT_MARK 普通草稿 / KEYWORD_MARK 关键词文档；页签 active 从已发布切到草稿（tabActiveChanged=true），但 tabClickTriggeredRequest=false、sameRowsAfterTab=true；搜索请求携带 lastReqKeyword=R599_IMPOSSIBLE_KEYWORD_998877、lastReqType=draft（searchPayloadHasKeyword=true），但 sameRowsAfterSearch=true；pageErrors 为空。即界面状态变化而数据不变，形成误导。

## 五、三分裁决（据实）
1. 根因：后端两变体都缺请求体提取与过滤语义，非前端选错端点（已实测改用 u3 亦无效）。据此写定，不归咎前端。
2. 口径：manager 变体先做 u2_require_admin，管理员看全量本身符合其权限口径；本问题是筛选条件对全量也无效应，不构成越权、不包装为安全问题。
3. 严重度：P2，限于筛选失效与界面统计误导；非鉴权绕过、非数据泄露。

## 六、Fail Loud（失败均据实披露，未采信后修正）
- 首跑 EXIT101 E42P10：there is no unique or exclusion constraint matching the ON CONFLICT specification——auth_role.name 无唯一约束，ON CONFLICT(name) 非法，改 ON CONFLICT(id)。
- 二跑同错且输出只见 Finished 无 Compiling：ops_edit_file 保留 mtime 致 cargo 编译缓存未刷新，执行 cargo clean -p cms_assemble_control 后重编。
- 三跑 EXIT101：ToSql(3) WrongType{postgres:Timestamp,rust:&str}，create_time 传字符串，改 chrono::NaiveDateTime::parse_from_str。
- E2E 首跑 stub 顺序瑕疵：宽泛 **/api/** 覆盖具体 who/manager 路由致跳登录页找不到 .tabs；改统一 route handler 按 URL 判断后重跑通过。属自身脚本瑕疵，非环境阻塞。

## 七、法医证据（runnerSha 与文件 sha256sum 分列，勿混用）
- runnerSha = sha256(JSON.stringify(meta)) = d8d73fdf11016f0c329ec85e4e96bb84fc3946ae62c78612553f3a8a14b54cc0
- 文件字节 sha256：r599-test.log = de50b20ffc96031b780429cc5e2417fd1d41147e6938e758cd0c3a4a1e202ff3
- r599_document_filter_audit.rs = a3923ab3e1a46fe651b1576c70aa5fa17690b47603ebf10922c7fbf2f14a14c4
- evidence.json = c1ea5d51fdb62671998544b2ba647a5dbad4dd5cf468eb2438740d5d0db02600
- lib.rs = 9a1b9348b78f95c34fcd86415dc2338a15b3a1d595cca85c594a4348e678f1ce
- routes.rs = 7f0a88fde98e3b501fbdb3d02b62c55bc69a696526d24ae179e1b9f00aa790ce
- DocumentApp.vue = 021e6c1d16206aa72708a22db650fd903c083e17a57f365d85e324dd046e6a0c
- 测试连接串与页面地址均为占位：postgres://<TEST_USER>:<TEST_PASS>@<TEST_HOST>:<PORT>/<TEST_DB>、<YOUR_TEST_DOMAIN>/oa4rust/app/document。

## 八、修复方向（另案，不改本轮源码）
后端两变体增加 Json<Value> 提取器，按 type/keyword 构造带参数的 where（如 status=$n 与 title ILIKE），并让 manager/非 manager 共用同一过滤实现；前端页签切换应触发带 type 的重新查询，统计数字取真实过滤后条数。