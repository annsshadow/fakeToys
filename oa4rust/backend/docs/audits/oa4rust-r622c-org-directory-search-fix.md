# R622c OrgDirectory 人员搜索漏筛真修复审计

- 任务: t253 / 轮次: R622c / 类型: implementation(真修复)
- 真父 HEAD: 56476ec5e5069e7201f3334c7c257c6d9bff9dff(执行时 HEAD 同; @{u}=b4903f31 本地领先 1)
- 判级: P2(据实裁定, 甄别轮 t249/R621c 已定 P2, 本轮为真修复)
- runnerSha(sha256(紧凑JSON meta)): a2d6edef82eb90fa3d6e7c24c93cb244343270bef47adcdb8bcb9d55d1270ae5
- 证据日志: <PROBE_LOG_PATH>(red/green/typecheck/biome 四日志执行后留存于本地 test-results, 均为本地路径占位)

## 一. 路径纠正与范围
- 任务描述写 src/components/OrgDirectoryApp.vue; 真实文件为 oa4rust/frontend/apps/desktop/src/views/OrgDirectoryApp.vue(与 t249 甄别一致), 据实按真实路径修复。
- 写域精确三文件: M OrgDirectoryApp.vue + A OrgDirectoryApp.searchfix.test.ts(契约) + A 本文档。后端 u2_person.rs 零改(sha=97f75cda0c895e0e4abe4a9436c286fa1da15d9ddf0c07c0ce278e9dcc7c0481)。

## 二. 缺陷与修复(真父 e4260eb9 → 修复版 06d38864)
- 原 :285-292 loadPersonFilter 调 api.post(..., {}) body 恒空, 搜索词从不进请求; :96 v-for 只对本页 rows 做 JSON.stringify 客户端过滤并 slice(0,20) 截断; :50 部门快捷仅 $refs.personSearch.value=dept 不重载; :153 总页数按全量 personCount 算。后端 u2_person.rs:778-820 person_list_filter_paging 读 name/mobile/email/status/unitFlag(unitId) 五条件 ILIKE+escape_like+normalize_key+owner_scope, count/data 同谓词分页, 能力完整。
- 修复7处: ①搜索输入加 v-model=personQuery ②部门点击改为 applyDepartmentFilter(dept) ③模板 v-for 改为直接遍历 rows, 移除客户端过滤和 slice(0,20) ④新增 personQuery/unitFilter 两个 ref ⑤loadPersonFilter 构造 body: 部门优先写 unitId, 否则关键词写 name, 替换原空对象 ⑥新增 applyDepartmentFilter 设部门/页码=1 并重载 ⑦搜索 form 改 onPersonSearch, 重置部门与页码后重载, 避免翻页后搜索残留。
- 总页数仍按 personCount(后端服务端 count)计算: body 修复后 count 即筛选后总数, 页数与可见行一致, 无需改动计算式。成功返回结构与其它端点行为零改。

## 三. RED→GREEN 真测试
- vitest v5.0.5 + jsdom 契约 OrgDirectoryApp.searchfix.test.ts 4 用例不 ignore, 用 node:fs 读源码断言(真父/修复版可区分): body 带 name/unitId 不再空对象、模板无客户端过滤与截断、部门快捷设 unitId 并第一页重载、总页数按服务端 count。
- 真父备份覆盖下 RED: EXIT=1, 3 failed | 1 passed(日志 sha256=e74f932cbd5d31b4640157d420539b62618041f8b2f302689af519b27a666f07); 修复版 GREEN: EXIT=0, 4 passed(日志 sha256=c2803a498b5c7dc93d04f5f7ebd183137788ce47a5c02a26e57858605b077270)。
- 真父备份 oa4rust/test-results/r622c-parent.bak.vue(gitignored), 修复后源码 sha256=06d3886487bc8f5f0a29592954d0b64e2cedefef7d51a925cd18b2cf7ce10248。

## 四. 三 verify
- V1 定点: node node_modules/vitest/vitest.mjs run frontend/apps/desktop/src/views/OrgDirectoryApp.searchfix.test.ts → EXIT0 4 passed。
- V2 typecheck: pnpm typecheck(tsc --noEmit) EXIT0; 据实披露 tsc 不解析 .vue 模板, 该缺陷可被类型检查放过, typecheck 通过不等于模板正确性证明。
- V3 biome: 新契约文件单独 biome check EXIT0(合规); OrgDirectoryApp.vue biome 真父 3 errors → 修复版 3 errors(持平, 均为既有基线未修改区 269/274/509/511 行, 非本次引入)。组合日志 sha256=67fe9d734efd331ffefb5870c5426a18e61d8c678175c35db0a690a3cd7f4a51, 据实披露 vue 基线未清零。

## 五. 据实披露与限制
- 部门到 unitId 的映射: 后端仅接受 unitFlag/unitId 字符串精确匹配(unit_id = $5), 本实现把部门名直接作为 unitId 值传入; 部门名到真实 unit_id 的映射需产品/后端确认, 若不一致需后续适配, 不在此盲目转换。
- 工号: 后端 person_list_filter_paging 无工号专属字段契约, 现按姓名/部门语义把输入放入 name(ILIKE), 工号精确搜索需后端补字段契约后另行对接, 据实披露未覆盖。
- 未跑浏览器 GUI 端到端取证, 契约以源码断言 + vitest 坐实; 后端 ILIKE/owner_scope 契约已逐行读回确认存在。
- 行为变化: 普通用户输入关键词/点部门后由前端伪过滤改为服务端真实分页筛选, 属功能修复; 搜索命中不再受本页 20 行截断与跨页漏搜影响。

## 六. sha 清单
- 契约 sha256: ab7078b9f3503d9952c30b9a1acd3b83284c9bfbf28d1d33bb7783f2f1d59ddd
- runnerSha: a2d6edef82eb90fa3d6e7c24c93cb244343270bef47adcdb8bcb9d55d1270ae5
- 文档文件 sha256: <由提交时 sha256sum 磁盘结果给出; 本文档生成时不含自身 sha>。
