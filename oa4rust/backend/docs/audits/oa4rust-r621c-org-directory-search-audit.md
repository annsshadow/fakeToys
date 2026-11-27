# R621c OrgDirectory 人员搜索筛选失效甄别审计

- 任务: t249 / 轮次: R621c / 类型: verification(零源码改)
- 真父 HEAD: e78561a995dfcf9c9ed08aa71ea1379f71442ed5(执行时 HEAD 同; @{u}=994c9dea0 本地落后未合并)
- 判级: P2(据实三分裁定, 详见第五节)
- runnerSha(sha256(紧凑JSON meta)): 208ccd5db3fe6a3415403c1ab2317aa3dfb5c22b3c307f1780bf594cd8b8fe0e
- 探针日志: <PROBE_LOG_PATH>  sha256=f7f7328749c9872eb3968cb03f897dfecabc51811f53f739b8f1e1a85f7b149c

## 一. 甄别目标与范围
- 唯一交付: 本文档; OrgDirectoryApp.vue / u2_person.rs / 兄弟视图 / main.ts 全部零改(git diff --stat HEAD 空)。
- 四个事实锚点均已逐行读回, 源码 sha: OrgDirectoryApp.vue=e4260eb944e04faa3ae30051e076888d23197179e9d4cbd9ea5e1ef8c443765f, u2_person.rs=97f75cda0c895e0e4abe4a9436c286fa1da15d9ddf0c07c0ce278e9dcc7c0481。

## 二. 事实(经 vitest+jsdom 真探针坐实, 4 用例 EXIT0 4passed)
- 事实1 请求体恒空: loadPersonFilter(OrgDirectoryApp.vue:285-292) 调用 api.post(/api/organization/assemble/control/person/list/filter/${personPage.value}/size/${personPageSize}, {}) , 第二参数恒为空对象; 探针模拟 api.post 捕获 payload 的 Object.keys 为 [], URL 仅含页码; 搜索词从未进入请求参数。
- 事实2 客户端过滤只作用当前页: :96 对「本页已返回的行」做 JSON.stringify(row).toLowerCase().includes(关键词) 匹配并 .slice(0,20) 截断。跨页命中(在其它页的行)永远不会进入当前页 rows, 搜索结果漏报; 即使本页命中多于 20 条也会被截断。
- 事实3 部门快捷不触发重载: :50 点击部门节点仅把 $refs.personSearch.value 设为部门名并写入 lastAction, 只改输入框 DOM 值, 未调用 loadPersonFilter, 也不重置页码。
- 事实4 页数口径矛盾: :148-149 源码注释明写服务端分页设计(页码/页大小/总数由后端 person_list_filter_paging 负责); :150-153 personPageTotalPages 却按后端全量 personCount 计算。空 body 使后端失去筛选语义, 搜索后「共 N 人 · 第 p / 总页数」与可见行不一致, 翻页跳变(换页仍取全量)。

## 三. 后端能力确实存在(缺陷在前端接线, 非后端缺失)
- u2_person.rs:778-820 person_list_filter_paging 读取 name/mobile/email/status/unitFlag/unitId, 经 escape_like+normalize_key 后构造五条件 ILIKE, count 与 data 使用同一谓词并带 owner_scope 收窄。
- 前端把关键词放入相应请求字段即可启用服务端筛选, 当前未传。

## 四. 兄弟正确写法对照
- BBSForum.vue:484-498 关键词进 PUT body.keyword, 页码跟随当前页并进入 queryKey 触发 refetch。
- DocumentApp.vue:219 构造 params.keyword 交服务端过滤, 不靠前端逐行匹配。
- main.ts:56-61 path directory 指向 OrgDirectoryApp.vue, onMounted(:309-312) 自动加载, 入口无门槛可达。

## 五. 三分判级(据实裁定)
- 可达性: 登录用户经 /app/directory 直达, 搜索框与部门树均无权限门槛; 但需用户主动输入关键词或点部门并搜索才显现, 非默认破坏。
- 危害: 搜索承诺(搜索框 placeholder 与人员搜索区提示)失效; 跨页漏搜、本页截断、翻页跳变、总页数与可见行不符, 属功能可见错误。纯前端行为, 不越权、不写库、不删数据、不泄漏额外数据(后端 owner_scope 仍在), 无安全或数据破坏后果, 排除 P0/P1。
- 限制: 部门/工号到后端 name/mobile/email/status/unitFlag/unitId 的字段映射需产品确认(部门可能对应单位标识, 工号可能对应凭据字段), 不能盲目映射; 无准确字段对应前不在此 prescribe 具体修法。影响面限于该视图的人员搜索, 其它视图不受影响。
- 结论: P2(功能可见错误, 需主动触发, 局部影响, 纯前端无数据破坏)。排除 P0(无安全/破坏)、P1(非默认触发且无数据损坏/越权)、P3(搜索为该视图核心入口能力, 跨页漏搜属明显功能失效, 高于纯文案问题)。

## 六. 证据与据实披露
- vitest v5.0.5 + jsdom 真探针: 4 用例全部通过(EXIT0, 4passed), 探针执行后已删除, 日志经占位符 <PROBE_LOG_PATH> 引用, sha256=f7f7328749c9872eb3968cb03f897dfecabc51811f53f739b8f1e1a85f7b149c。
- 探针用源码字符串断言与模拟 api.post 捕获 payload, 未起真实浏览器, 未做跨页命中/翻页稳定性的浏览器界面取证(未跑 GUI)。
- 台账: oa4rust/backend/docs 下对 OrgDirectory 搜索/筛选的既有专项审计命中 0(本次为首次甄别, 不与其它议题重复计数)。
- typecheck 盲区: desktop 的 tsc --noEmit 不解析 .vue 模板表达式, 该缺陷可被类型检查放过(据实披露)。
- 后续修复提示(非本次改动): 若把关键词放入请求体, 需同步在搜索时把 personPage 重置为 1 并让分页跟随筛选后的 count, 否则仍有跳页。

## 七. sha 清单
- runnerSha: 208ccd5db3fe6a3415403c1ab2317aa3dfb5c22b3c307f1780bf594cd8b8fe0e
- 文件 sha256: <由提交时 sha256sum 磁盘结果给出; 本文档生成时不含自身 sha>。
