# 仍有价值未落地项 · 重新归集（2026-11-16）

> **这份文件是什么**：此前十一份计划/台账里写着「未做 / 待拍 / 进行中 / 遗留」的条目，
> 到 2026-11-16 有的已经做完、有的已经定性关闭、有的**仍真实开放且仍有价值**。
> 本文件把**第三类**重新落地为可续跑的计划；前两类只留结论索引，不重开。
>
> **口径（承 `docs/plans/2026-10-29-remaining-work-plan.md` §四）**：账本记载只作线索，
> 结论以**现量复核**为准。权威现量入口：`docs/ops/current-state.md`（2026-11-15 复核）。
> 本文件每一条都标注了复核方式与复核结果，不凭台账原文直接宣布「未做」。
>
> **状态**：待认领。每条含落点、验收判据与依赖；认领后按批次推进，不在这里假装已完成。

---

## 一、复核后确认「已关闭」的旧遗留（只留索引，不重开）

这些条目在旧计划里仍写着「未做/遗留」，但现量已变。**列在这里是为了防止重复劳动**。

| 旧条目 | 旧出处 | 现量结论 | 复核方式 |
|---|---|---|---|
| AU-P1 异步 LLM 后端（原记「大重构·未做」） | `2026-10-23-cross-subproject-optimization-backlog.md` §7.7 | **已完成**：Ollama / Claude / ERNIE / Gemini / OpenAI 五个后端均有 `_acall_api` + `_async_client_for`，重试取消传播与 `augment_async` 原生路径在批二 R1–R6 落地 | `grep -n "_acall_api\|_async_client_for" augmentor/augmentor/models/*.py` |
| LY-5 配置热加载（原记「未做·需改指针」） | `2026-10-23-cross-subproject-optimization-backlog.md` §7.5 | **已完成**：`Service.cfg` 改为原子指针，`ReloadConfig` 为 SIGHUP 入口，限定生效范围（TTL / 重算模式 / 微信凭据），构造期固化项不改 | `service.go` 的 `ReloadConfig` + `config_reload_test.go` |
| B2 invoke token 零验签（原「待拍板」） | `2026-10-30-b2-invoke-token-design.md` §四 | **已完成**：迁移 `114_add_program_invoke_token_hash.sql` 落表，`program_center/src/lib.rs` 有 `verify_invoke_token` 常量时间比对 | `ls oa4rust/backend/migrations/114_*` + grep 代码 |
| B4 bbs `create_reply` 功能桩 | `2026-10-29-remaining-work-plan.md` B4 | **已完成**：真 `INSERT INTO x_bbs_reply`，带 `require_session` 门，迁移 021/033/037/067 齐全 | 见 `2026-10-30-b2-invoke-token-design.md` §六 |
| `shared/src/crud.rs` CrudSpec 无 owner 谓词（A3-N9） | 安全审计账本 §五遗留 | **已收口**：`crud_owner_ok` + `crud_save_owned` / `crud_delete_owned`，owner_column 静态字面量传入，行不存在回 false 保 not-found 语义 | `crates/shared/src/crud.rs:163-224` |
| `row_to_json` 的 JSONB / 时间静默丢列 | 安全审计账本 §五（第十六批） | **已收口**：JSON / JSONB / TIMESTAMP / TIMESTAMPTZ / DATE / TIME / UUID 均已覆盖（注：**NUMERIC 仍残留**，见 §二） | `crates/shared/src/response.rs:10-55` |
| laiyipao 微信支付 / 激励广告 / 微信登录客户端 | `2026-10-21-...` 文末「仍未做」 | **已关闭**：小程序端 2026-11-14 整体移除，凭证类阻塞归零，不是待办 | `docs/ops/current-state.md` §三 |
| 卷二 Backlog A 27 笔 / oa4rust P0-1 / P2-1 / 历史债八类 | 台账族 | **全部终局**：27/27 有结论，P0-1、P2-1 关闭，真实仍开着的待办 0 | `docs/ops/current-state.md` §三、§六、§八 |

---

## 二、仍真实开放、仍有价值的条目（本计划的续跑入口）

> 口径：只收**有明确缺口 + 有落点 + 不靠假设就能开工**的条目。需要产品拍板或真机的单独标注，
> 不混进「可直接做」那一档。每条的「复核」列写的是 2026-11-16 实际查到的读数。

### 批 A · oa4rust 安全审计残留（缺口明确、尺子可复跑）

| # | 条目 | 复核（2026-11-16） | 落点 / 修法 | 验收判据 | 依赖 |
|---|---|---|---|---|---|
| A1 | **第三方回调真验签族**（钉钉 / 政务钉钉 / 企微的 sync、regist、callback） | 代码内**找不到任何签名校验**；此前只把伪造面收 admin 档，真验签明确「本批不冒充做了」 | 增加 timestamp + nonce + 签名比对（常量时间）；保留 admin 兜底；按回调源分配置密钥 | 每源三组用例：合法签名放行 / 重放 nonce 拒绝 / 篡改体 403；无会话与伪造回调仍 401/403 | 需各平台签名密钥来源口径（配置项，不入库真值） |
| A2 | **`row_to_json` 的 NUMERIC 残留** | `response.rs` 注释明写 NUMERIC 仍走静默丢列（`postgres-types` 移除了 decimal feature） | 在调用面 SQL 显式 `::float8` / `::text` 转写，或统一转字符串；**不改公共函数签名** | 对含 NUMERIC 列的通用 SELECT，响应形状与列集合一致；有反向用例钉住「未知类型仍省略键」 | 无；需逐调用面确认金额精度口径（钱用字符串还是 float） |
| A3 | **GET 写面残余门禁** | `write_routes_get_writes.tsv` 现量 **2 条 K1 匿名**（`auth` 的 `qiyeweixin_update_person_detail`、`program_init` 的 `secret/set/cancel`）。旧文档「auth 6 条」口径已过期 | 前者属第三方回调族（与 A1 同批，加门与验签分开记）；后者是首次引导有意保留匿名，需定性后再决定收不收 | GET 写面归零或每条有显式定性说明；加门不得打断首次初始化 | A1 的密钥口径；引导族需产品定性 |
| A4 | **只读挂写方法全仓收口** | `write_routes_readonly_on_writemethod.tsv` 现量 **651 条**（含表头 652 行）；其中大量是 touch 时间戳的「读形状写」，已被守卫点名 | 按 crate 递减：先分辨「真落笔」与「只读挂写」，真落笔逐批补归属门；只读挂写只在确属语义错位时改方法 | 三向恒等式 `real + readonly + unparsable == total` 保持成立；每批离线 401 臂交叉验证 | 无；但必须逐批回归，不能只看「含写」差值 |
| A5 | **尺子判据债**（`GATE_RE` 的 `require_actor` 冗余词、门证据不剥注释） | `GATE_RE` 仍含 `require_actor`（审计账本记为「无独立见证的冗余词」）；注释里的门名可能被当真门 | 删词要连注入见证一起改（账本 §三.15 明确要求）；门证据扫描先剥注释再匹配 | 注入臂红、反向臂绿；R7 / 函数级读数在改词前后**按现量重钉**，不假设不变 | 无；纯尺子轴，零产品码 |

### 批 B · 1000 轮路线图 R155 候选（台账有候选、未写代码）

| # | 条目 | 复核（2026-11-16） | 落点 / 验收 | 依赖 |
|---|---|---|---|---|
| B1 | **弱网 / 错误态覆盖面**：桌面 `describeListError` / `useListError` / `ListLoadError.vue` 与移动 `services/http.ts` 文案归一化可复用 | 已于 R155 落地（提交 f8d6a48db）：桌面 api.ts 增 NETWORK_FAILURE_MESSAGE 等常量 + networkFailure() 工厂，6 处 catch + stream 守卫；移动 http.ts:106 transportFailureMessage 同口径。现量 §14 | 先以统一口径标识或行为用例取证「失败被折叠成空态」，再改；禁止只 grep「错误/失败」 | 无 |
| B2 | **破坏性操作重入守卫全仓普查** | 已于 R156 落地（提交 7dceab7f3）：26 页 CRUD 删除重入守卫 + crudDeleteGuards.test.ts 26 例；R157/R158 续补表单提交守卫 37 处。现量 §15.1/§16.1/§17.1 | 按同族（删除/清空/批量/退款/扣款/解绑/停用）扫描后合批，不做单点 | 无 |
| B3 | **`auth_person` 8 处 id≠uid** | 现量条目仍标 pending，收口方向已写明「无 arbiter + 插入后反查/真实冲突报错（#258 模式）」或统一 id 生成 | 按测试面分批，禁止跨面一把梭 | 需定 id 生成权威 |
| B4 | 分桶预案 / ④SLO 周期复核 / ⑤慢日志毫秒精度 / ⑥trace-pull 聚合 | 台账标「触发条件未到」或候选，未开工 | 触发一条做一条；本文件只登记，不预占轮次 | 各条触发条件 |
| B5 | `cmd/loadtest` / `cmd/playertest` 压测计划生成的纯函数测试 | 台账判定为**已知债务**：参数算错后果应从覆盖率里分离 | 给请求序列 / 并发档位 / 时长预算生成函数加测试；不追「假目标服务的 flag 覆盖率」 | 无 |
| B6 | laiyipao 文档站 P6-12 余项（侧边栏目录 / 搜索 / 深链 / `gems`/`skins`/`recipes` 展示 / 关卡地形与 BOSS 字段 / 移动端横向溢出） | `2026-10-21-...` 文末记「只落地装备图鉴页签，其余未做」；后续批次另在 miniapp 加了敌人/技能/装备/宝石/皮肤图鉴页，但**后台文档站这几项未见登记落地** | 先复核现量差集，再按「目录 → 搜索 → 深链」分批；`gems`/`skins` 若已被 miniapp 图鉴覆盖则同行销账，不重复做 | 需先出差集，避免与 miniapp 图鉴重复 |
| B7 | `.debug-tag` CSS 规则与 `rank.vue` 的 `formatScore` 同值死分支 | 已清：rank.vue:211-215 死分支已删、.debug-tag 命中 0；补 dead_code_guard.test.ts 两条防复活守卫（临时注入实测能变红）。现量 §381 | 直接删；带一个源码守卫防复活 | 无 |

### 批 C · 需要外部事实或产品口径，不擅自开工

| # | 条目 | 已知约束 | 解除条件 |
|---|---|---|---|
| C1 | `burn_cloud` 火区 | 会改 `replay_hash` → 存量战报永久失去可重放性，且要重跑 100 关平衡基线；规格已留档 | 产品明确接受「重放性断裂 + 重跑基线」代价 |
| C2 | oa4rust 路由补齐 ~73 条（OA-P0-1） | 旧计划记「多周工程量、需 Java 契约参照 + 产品决策」；起点 `oa4rust/backend/docs/plans/routing-completion-plan.md`。现量已有 2645 真实 handler，需先复核**到底还差哪些端点**再立战役 | 先出「现量差集」清单，再谈实现 |
| C3 | 真机视觉验收（laiyipao / 双端 UI） | 本地只有无头渲染；小程序端移除后仅 H5 | 有真机与人工标定窗口 |
| C4 | `A267` 的 `quality_distribution` 需要 `scores` 来源 | 两个产品调用点今天都不传 scores，该类恒跳过（明确边界） | 定 scores 权威来源，属新需求 |
| C5 | 部署改多 worker / 多实例后的三笔重评（A355 / A376 / A384） | 现为单进程事实，三笔判纸面风险；契约写明改形态必须重开 | 部署形态真的改变 |

---

## 三、本文件的边界

- **不重开已结案的行**：卷二 27 笔、oa4rust P0-1 / P2-1、laiyipao 已知边界 15 条、历史债八类，
  均按 `docs/ops/current-state.md` 的终局口径保留，不因本文件复活。
- **「清零」不等于没有问题**：每条都有终局结论、阻塞条件与推进人即为结案；
  真出现线上问题另立新台账。
- **轮次口径**：本文件不预支任何「第 N 轮」编号；认领后在各线自己的台账里按现量编号。
