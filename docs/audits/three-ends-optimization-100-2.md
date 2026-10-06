<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# oa4rust 三端全面优化 100 轮账本·第二本（2026-10-06 启动）

> 目标：oa4rust 三端（Rust 后端 + desktop + mobile）的**功能扩展**与**性能质量提升**全面优化第二轮 100 轮。
> 承接第一本（`docs/audits/three-ends-optimization-100.md`，2026-10-01 收官 100/100）。
> 每轮 = 真实扫描 → 处置 → 验证 → 记账。结果三类：**FEAT**（功能扩展）、**FIX**（发现并修复）、
> **PERF/IMPROVE**（性能/不改行为的质量提升）。
> 节奏：每轮轻量门禁（biome lint + tsc + 受影响测试）；每 10 轮中型门禁（全量 vitest + 双端 build）；
> 第 50/100 轮完整门禁（另加 cargo fmt/clippy --all-targets -D warnings/parity/reconcile）。
> **提交尾注用「（优化二轮 N）」与第一本的「（优化轮 N）」区分**；仍只暂存本人文件
> （工作区有并行 CI/augmentor/laiyipao 会话在途改动，绝不越界暂存）。

## 状态：进行中（轮 8/100）

## 启动基线（2026-10-06 实测）

- 分支 `augmentor-opt100`（与并行会话共用的马拉松分支），起点 HEAD 含第一本收官后 augmentor/terrain 交错轮次。
- **三端门禁复跑全绿**：桌面+SDK vitest 998/998、mobile vitest 114/114（与第一本收官基线一致）。
- 遗留甄别：工作区 `.github/workflows/oa4rust-{ci,web}.yml`（并行 CI 会话）、`augmentor/*`、
  `laiyipao/*`（各自会话）、`docs/audits/three-ends-2026-09-20/api_reconcile.json`（行号漂移，10-02
  契约会话在途产物）、`backend_routes.json`（CRLF 假修改零内容变化）——均不属本循环，不处理。

## 第一本移交的候选方向（续用）

1. desktop bundle 分片（首包 40KB 后的懒加载块优化）；
2. mobile 业务域扩面（recycle/search 页化等）；
3. 前端未消费 useQuery 复扫；
4. 软删泄漏续扫（第一本清了 query/org/portal 等/bbs 四集群 64 处，本本轮1 续出写面新集群）；
5. 读路径 N+1 / 慢查询复扫。

## 轮记

| 轮 | 维度 | 结果 | 处置 |
|---|---|---|---|
| 1 | FIX（后端数据正确性，软删写面集群五） | FIX | **x_portal_dict 写面 4 处 + pp_e_form 游标列表 2 处**（commit `3da2ef7ab`）：方法学=crud spec soft_delete（18 表）× 手写 SQL 交叉扫描，读侧已全过滤（第一本成果），本轮清写侧漏网——①surface 字典数据 3 处 UPDATE 无软删过滤：已删行被写穿（数据落进永远读不到的行；mockputtopost 端点随后 query_one 必查无行 **500**）→ 改复活式 upsert（`SET ... deleted_at = NULL`，表无唯一约束安全，PUT 语义=重存即恢复）；②`..._data_delete` 端点物理 DELETE 与 designer 侧 crud 软删语义分裂 → 改软删幂等（`SET deleted_at=NOW() ... AND deleted_at IS NULL`，第一本轮28 惯例）；③pp_e_form 游标分页 next/prev 已删表单仍列出 → 补 `AND deleted_at IS NULL`。甄别记档：pp_e_mapping 同款游标 SELECT **不过滤**（该表全仓零 deleted_at 写入方、删除端点为物理删=有意设计，非泄漏）；cms tests_u2 的 5 处物理 DELETE 是测试清理非缺陷；`update_time = NOW()` 赋 TEXT 列合法（PG I/O 转换 assignment cast，全仓惯例）。新增真库往返守卫 2 例（软删幂等含二次 delete 计数 0 / 重存复活 deleted_at 归空），live PG 实跑。**坑（重踩第一本第十五批）：python heredoc 写含反斜杠 needle 必坏（`pp_e_form \`+LF 永不失配）——含反斜杠编辑一律 Edit 工具**。验证：surface 24/24（含 2 新守卫）、pp_designer 28/28、clippy 0、fmt 0 |
| 2 | FIX（后端安全/数据正确性，IDOR+语义分裂） | FIX | **collect 族属主边界收口**（commit `a02dfc1b3`）：软删表物理 DELETE 续扫 7 处甄别出真缺陷——①`GET collect/list` 全量列出所有人收藏（mobile 轮91 的「按登录人过滤」是前端假过滤，API 层任何人可枚举他人收藏 URL）→ 补 session 属主过滤 `person_id = $1`；②`DELETE collect/delete/{id}` 走 crud_delete 无属主校验——持他人收藏 id 即可横向越权软删（**真 IDOR**）→ 属主门禁+软删幂等手写 UPDATE（`WHERE id AND person_id AND deleted_at IS NULL`），`deleted=false` 不区分不存在/非本人/已删不泄漏存在性；③顺删 `collect_remove` 未注册死 handler（我按扫描命中先改它，守卫测试立功暴露真路由挂的是 `collect_delete`——**扫描甄别必须以 routes.rs 注册表为准，不能信函数名相似**）。甄别记档：pp_e_applicationdict 删除端点注释自证「物理删除=有意」、program_center u3 admin 清理有 require_admin、u2_closures 3 处 query 表物理删（不泄漏不写穿仅无回收）记档不扩大。新增越权守卫真库实跑（bob 删 alice 收藏 deleted=false+行原样 / alice 删 deleted=true+deleted_at 非空）。验证：program_center 256/256（+1 守卫）、clippy 0、fmt 0 |
| 3 | FIX（desktop 安全/隐私，跨账号缓存泄漏） | FIX | **登录人变化即清 vue-query 缓存**（commit `9e92549d`）：链路=①SDK `logout()/switchUser()` 只清 user state 不清查询缓存；②QueryClient `staleTime: 5min + refetchOnWindowFocus: false`；③视图 queryKey 普遍不含登录人（如 CollectApp `['Collect','list']`）——换账号登录后 staleTime 窗口内上一账号数据直接渲染给当前账号（跨账号数据泄漏，CollectApp 仅为首例，全视图同构）。修法=装配层单一事实源：utils/sessionCache.ts `watchSessionCacheReset(queryClient, watchSource)`，main.ts 在 pinia 激活后接线 watch `session.user.unique` 变化（覆盖登出/切换/换账号重登全路径；SDK 不依赖 app 的 queryClient）。新守卫 3 例（换人清/登出清+重登再清/同人不触发），3 例先红（watcher 默认异步调度须 `await nextTick()`）后绿=判别力实证。**坑：SDK `session.state` 是 `readonly(state)` 解包对象（非 ref），`.state.value` tsc TS2339**。顺甄别：bundle 分片主题确认充分（入口 44KB gz12、echarts 524/codemirror 376 已独立懒加载块、EChartsView 已按需），desktop RecycleApp 四端点对 generated_routes 精确比对无错配。验证：desktop vitest 1001/1001、tsc 6 包 0、biome 0、desktop build ✓ |
| 4 | IMPROVE（跨端同构甄别，轮3 主题收尾） | IMPROVE | **mobile 端跨账号缓存泄漏面甄别=不存在**：①mobile 不用 vue-query（pnpm 依赖与源码零命中，自有 http.ts+pinia 方案）；②业务数据零 storage 落盘（`setStorageSync` 全仓仅 tab/theme 等 UI 偏好，页面数据内存态+onShow 重拉）；③会话恢复走服务端 `/who`（store 纯内存 ref，无持久化）——登出/换号后不存在上一账号数据复现路径。SDK 侧 `clearLegacyStorage()` 已有。结论：轮3 修复无需 mobile 同构改动，跨端缓存/存储泄漏主题双端闭环。验证：依赖清单+源码扫描+store 实读三重证据 |
| 5 | FIX（后端安全/数据正确性，IDOR+语义分裂跨 8 端点） | FIX | **query designer 删除族口径统一收口**（commit `1ce67a79d`）：u2_closures.rs 内 8 个删除端点三类问题一次收口——①裸奔无守卫：importmodel_delete/stat_delete/statement_delete/neural_delete/table_delete/table_row_delete/view_delete 全部无 session 校验，任何登录人可删任意数据 → 补 guard_write（owner=COALESCE(creator_person,creator,'')，空 owner 回退 admin，同族既有惯例）；②物理 DELETE 与 crud spec soft_delete: true 分裂（7 表有 deleted_at 列）→ 软删幂等 `UPDATE SET deleted_at=NOW() ... AND deleted_at IS NULL`（statement 表无软删列维持物理删仅补守卫）；③guard SQL 的 OR 优先级 bug：`id=$1 OR model_flag=$1 AND deleted_at IS NULL` 实际等价 id=$1 OR (model_flag=$1 AND 未删)——已删行按 id 仍可通过守卫 → 括号化修正。**甄别记档（勿重扫）：mind 域 tree/my 列全部 = o2 原始契约语义（docs/oa/modules/o2server/x_mind_assemble_control.md 明确记录查 parent_id IS NULL 全量），共享目录树设计非 IDOR；mind create 的 creator 落 "system" 为低危展示字段质量问题记档**。验证：query_assemble_designer 46/46、clippy 0、fmt 0 |
| 6 | FIX（后端安全，越权管理面） | FIX | **bbs assemble/control 删除族补 admin 门禁**（commit `411aeb822`）：delete_forum/reply/subject 的 Path 版+body 版共 6 端点 + section_delete（crud_delete 版）=7 端点此前任何登录人可调（BBSForum.vue 调用点全部在管理面板上下文，与禁言/角色并列=管理动作语义）→ 补 `shared::middleware::is_admin` 门禁，非 admin 返 Forbidden 信封。**消费面甄别：delete/subject 零前端消费（desktop 测试还断言不调用）、forum/reply/section 删除仅管理面板**——无误伤。section CRUD 往返测试改造为守卫双断言（非 admin 403 + 临时 admin 探针身份 200 硬删+清理）；坑：auth_person.password_hash 与 auth_person_role.unit_id NOT NULL，探针 INSERT 须带占位值。验证：bbs 51/51、clippy 0、fmt 0 |
| 7 | FIX（后端安全，归属伪造+IDOR） | FIX | **bbs 发帖/回帖 creator 事实源收口 + 昵称 IDOR**（commit `a22dea854`）：①create_topic（mobile 发帖在用）/u2_subject_save/u2_user_reply_save 三端点 creator 从 body 取值（可伪造=冒名发帖/回帖，且漏发时落 "anonymous" 垃圾归属）→ 强制取 session.person_unique，客户端传值忽略；②u2_userinfo_update_nick 路径含 person 却无校验=任何人可改任意人昵称 → 本人或 admin（`person != session.person_unique → u2_require_admin`）。轮98 mobile roundtrip 测试同步带 session（creator 断言值与会话人一致）。**方法学教训：扫描器的守卫模式必须先枚举本域真实守卫原语（u2_require_admin/gate_topic_owner/gate_reply_owner）再扫，本轮两度误报「无守卫」实为模式漏配**。甄别排除：vote_submit 已用 session、role_list_by_section 为纯读。验证：bbs 51/51、desktop vitest 1001/1001、mobile 114/114、clippy 0、fmt 0 |
| 8 | FIX（后端安全，归属事实源） | FIX | **collect 归属事实源收口**（commit `eb6e221fb`，轮2/7 同主题收尾）：①collect_create 的 personId 从 body 取（客户端可伪造=替别人创建收藏）→ 强制 session.person_unique 覆盖 body 值；②collect_save 双缺陷：无属主校验（可改他人收藏内容）+ personId 可经 save 改写 → 前置属主校验（WHERE id AND person_id AND deleted_at IS NULL，不匹配返 saved=false 不泄漏）+ payload 剥离 personId。desktop CollectApp 发的 personId=登录人（轮97），后端覆盖后行为不变仅堵伪造面。验证：program_center 256/256、clippy 0、fmt 0 |

## 记账纪律（沿用第一本）

- 提交信息带「（优化二轮 N）」尾注；只暂存本轮文件。
- 跨工具核实以 od/python 字节级输出为权威；含反斜杠内容的编辑用 Edit 工具（heredoc 必坏）。
- 断言 needle 从真实源码字节抄；批量 SQL 变换必须逐条真实 DB 验证。
