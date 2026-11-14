<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- 本页在 mkdocs.yml 的 exclude_docs 名单内，不发布到文档站。 -->

# 全仓现量快照（2026-11-15 复核）

> **定位**：本页是各子项目「今天跑出来是多少」的**唯一权威入口**，不是台账、不是计划、不写预测。
> 分册若与本页冲突，以本页为准；决策依据与重开规则见 `docs/plans/2026-11-14-l331-reopen-decisions.md`，
> 逐轮实装记录见 `augmentor/OPTIMIZATION_LOOP_2.md` 的 L330–L343 日志块。
> 快照分支 `docs/ledger-l330-closure`；`main` 未改动。

## 一、门禁与验证读数

| 面 | 现量 | 取证方式 |
| --- | --- | --- |
| augmentor 全量门禁 | **10729 passed / 2 skipped / 0 failed**（733.61 s） | `cd augmentor && python -m pytest tests -q -p no:cacheprovider` |
| augmentor 覆盖率 | 门禁汇总 **99.37%**；`coverage.xml` 行覆盖 **99.51%**（16071 行 / 78 未覆盖 / 4478 分支 / 52 未覆盖），门槛 99% | pytest-cov 汇总与 `augmentor/coverage.xml` |
| 账本与名册守卫族 | 13 把账本守卫 + 3 份 L342 名册守卫，终态全绿 | 定点复跑；声明面按现量重钉 |
| augmentor 前端 | **430 passed**；`tsc --noEmit` / `eslint --max-warnings 0` / `prettier --check src` 全绿 | `cd augmentor/web && npm test` |
| laiyipao H5 | 单测 **1264 passed / 2 skipped**；`vue-tsc` 通过；`build:h5` exit 0 | `cd laiyipao/miniapp` |
| oa4rust | `cargo check -p openapi --offline` exit 0（1m18s，完全离线） | `cd oa4rust/backend` |
| 跨项目告警门禁 | **5 项 5 通过 / 0 跳过**（rust-lints / llm-lab / llm-lab-tests / augmentor-web / license） | 注入 Python 3.12 后跑 `node scripts/check-warnings.mjs --only ...` |
| SPDX 门禁 | 2469 个源码文件，缺失 0 | `node scripts/check-license-headers.mjs` |
| 文档站 | `mkdocs build --strict` exit 0 | `.venv-docs/Scripts/python.exe -m mkdocs build --strict` |

**口径提醒**：`scripts/check-warnings.mjs` 的 `CHECKS` 共 **19 项**；上表 5 项是本轮抽查子集，
不是全量 19 项的替代。两支 skip 是作者工作树条件（export 节无开关、无真实数据集可定标），
与历史同族，不是跳过失败。

## 二、各子项目现量

| 子项目 | 交付形态 | 关键现量 |
| --- | --- | --- |
| **augmentor** | Python + FastAPI + React | **81 个 REST 端点**全部声明 `response_model`；卷二 Backlog A 27 笔全部有终局结论；`stage_gates()` 已挂到管道；三份判据名册（点号主张 / 哈希 / 被引行内容）改严 |
| **laiyipao** | Go + uni-app **仅 H5** + Vue3 admin | 小程序端已移除：构建脚本、manifest 平台段、appid 占位符、部署章节均已删；`miniapp/` 目录就是 H5 源码；已知边界 15 条分类结案，第 1 / 2 条随平台范围关闭 |
| **oa4rust** | Rust workspace + Vue3 桌面端 + uni-app mobile | `utoipa-swagger-ui` 启用 `vendored`，`Cargo.lock` 引入 `utoipa-swagger-ui-vendored v0.2.0`，编译期不再联网；P0-1 与 P2-1 关闭；**Swagger UI 已挂 `/swagger-ui/`**（L345，页面指向既有带缓存的 `/openapi.json`） |
| **llm-lab** | Python 教学双轨 | 原理轨（只依赖 numpy）与实战轨（需 torch）；11 篇讲义 00–10；95 个验证用例 |
| **auto-checkin** | Node + Playwright | 3 站点自动签到 + 9 项模块覆盖率棘轮 |
| **linux-7.1.3**（仓外） | 内核 gcov 覆盖率 POC | 源码在 Docker 容器；仓内 `docs/linux-7.1.3/` 只留工件缓存，不计入上述 5 个子项目 |

### augmentor 本轮改动的对外契约速查

| 轮次 | 落地 |
| --- | --- |
| L332 | 同名再登记回 409，原有 `created_at` 不动 |
| L333 | 严格 JSON（拒 NaN / Infinity）、阈值收 `[0,1]`、管道降级 503、坏路径名 400 |
| L334 | 合并四档旋钮 + 六键回执；聚合三档 + 一份权威回执，冲突明细收窄到 20 条 |
| L335 | 成功面回显 UTF-8 安全化；评估侧 / 统计侧在场非字符串即拒 |
| L336 | 分层采样全值分组并回执真组数；搜索请求级超时 5 秒回 504（不换 re2） |
| L337 | 两支 shuffle 口径统一；读侧瞬时失败有限重试，耗尽回 503 |
| L338 | 注销数据集与建立依赖边两条 HTTP 写入口（端点总数 79 → 81） |
| L339 | 出图清单 `types` 接线（timeline 第一次真出图）；`log_metric` 加可选 run 把手 |
| L340 | 模型维度与配置冲突取「拒配置」；chromadb 状态面改只读探查（400 → 200） |
| L341 | 缩 `AugmentorPipeline` 公开面；SDK 的 expander / tracker 能力仍由包导出 |
| L342 | 点号主张、423 枚哈希、121 格被引行内容三份名册改严 |
| L343 | 10 只 `config.*.enabled` 出厂值由关改开（与既有行为一致），`stage_gates()` 统一读 |

## 三、台账终态

| 台账 | 终态 |
| --- | --- |
| 卷二 Backlog A | **27 / 27 笔有终局结论**：24 笔实装改判「已修」，A355 / A376 / A384 三笔按「生产单进程」判为纸面风险、零产品码 |
| laiyipao 已知边界 | 15 条分类结案（要凭证 / 要真机 / 维持现状 / 设计意图），凭证类阻塞随小程序端移除归零 |
| oa4rust 交接 §4 | 6 条全部有结论；P0-1、P2-1 已关闭 |
| 拍板议程 | 10 条全部有结论；27 笔重开 8 题答案见决策台账 §六 |
| 决策单 / 盘点台账 | `docs/laiyipao/DECISIONS.md` D1–D5 全部实施；`docs/plans/2026-11-12-remaining-ledger-inventory.md` 批次表 1–8 全数完成 |

**「清零」的含义**：每条都有终局结论、阻塞条件与推进人，**不等于消灭所有潜在问题**。
后续真出现线上问题按约定另立新台账，不重开已结案的行。A271 当时留的「关掉哪只端点」
属新需求，已于 2026-11-15 按 L345 单独收口，见下节。

## 四、文档维护约定（改文档前必读）

1. **净零行**：`docs/augmentor/API.md` 前 1190 行有 6 组带行号的历史引用，
   `docs/augmentor/ARCHITECTURE.md` 有 29 格被引行内容被 A357 名册钉住。
   这两份文件**只能同行替换**；要插行只能追加在文件末尾，且末尾追加会让 `docs/index.md`
   那类自指行号主张平移，必须同批重钉定位器。
2. **名册重钉**：A346（45 格真主张 / 30 格形状示例）、A349（423 枚唯一哈希）、
   A357（121 格被引行原文）三份名册此后每轮落账都要按现量重钉。
   讲坏引用时**不要把引用本身写进日志块**，否则会自造新主张或新哈希格。
3. **守卫钉住的数字**：根 `README.md` 的端点总数 / 功能页数 / 多格式导出种数 /
   Python 版本、`docs/augmentor/API.md` 的端点总数、CLI 索引表、门禁卡影响面文案，
   改这些必须同批复跑对应守卫。
4. **文档站白名单**：`mkdocs.yml` 只发布 `docs/laiyipao`、`docs/llm-lab`、
   `docs/oa4rust` 与首页 `docs/index.md`。新增 `docs/` 顶层目录或顶层 md 必须同步
   `exclude_docs`，否则 CI 的产物白名单校验会拦下。本页在 `docs/ops/`，不发布。
5. **根目录入口约定**：`README.md`（GitHub 首页 + 六道守卫的判据源）与
   `AGENTS.md`（Agent 工具链按约定从根读取）必须留在仓库根，不要搬进 `docs/`。
   原根 `MEMORY.md` 已归档为 `docs/ops/history-milestones-2026-09-07.md`。

## 五、容易混写的口径

| 容易混 | 正确口径 |
| --- | --- |
| 99.51% vs 99.37% | 前者是 `coverage.xml` 的核心包行覆盖，后者是门禁汇总；两个都要标注来源，不能互相替换 |
| 96 vs 97 crates | 96 是 `[workspace].members`，97 是连同根包的 `Cargo.toml` 总数；不是矛盾 |
| 79 vs 81 端点 | 79 是 A309 两条写入口之前；现量 81，且全部声明 `response_model` |
| 「小程序端」 | laiyipao 已移除；**oa4rust mobile 的微信小程序构建仍在**，两者不要混 |
| 「待拍」 | 27 笔已全部结案；文档里保留的「待拍」字眼若指历史叙述，须带轮次号说明当时状态 |
| 13 项检查 | `CHECKS` 现量 19 项；根 README 的清单已按脚本顺序补齐 |
| docs 篇数 | 约 270 篇 md / 360 余个文件；本页自身也在其中，写死精确数会自指，需要时用 `find docs -name "*.md" | wc -l` 现查 |

## 六、遗留项收口（2026-11-15，L345）

上一轮明确留的两项按「后续有问题另立新台账」处理，**没有重开卷二已结案的 27 笔**：

| 项 | 结论 | 落地 |
| --- | --- | --- |
| Swagger UI 挂载 | 已挂 `/swagger-ui/` | `oa4rust/backend/src/main.rs` 的 `docs_router()`：`Config::new(["/openapi.json"])` 指向既有缓存端点，不重复注册文档路由；3 支 Rust 行为面测试；离线 `cargo check` 与 `openapi-paths` 门禁通过 |
| A271 关闸语义 | 十只闸真正生效 | `augmentor/api/stage_gate.py` 的 ASGI 中间件 + `STAGE_ENDPOINTS` 映射：5 节覆盖 7 个端点，关掉回 400 固定中文文案；另 5 节明确登记「产品侧无消费者可关」。默认全 True ⇒ 零漂移，实时读配置，读数失败 fail-open |

**A271 的口径**：中间件不注册路由、不改 OpenAPI 契约、不动前端 fetcher。
关闸映射见 `api/stage_gate.py` 的 `STAGE_ENDPOINTS`，守卫逐格钉住表形状、
零漂移反证、隔离反证、热更新实时性、fail-open 与真身 app 行为面。
`context` / `expander` / `tracker` / `active_learning` / `frameworks` 五节今天
没有可关的端点（L341 缩面、产品侧零实例化），**明确登记而非遗漏**；
将来真接出消费者时补映射并同批重钉守卫。

### L345 本轮新增验证

| 面 | 读数 |
| --- | --- |
| Python 新守卫 | 8 支 `test_stage_gate_enforcement_l345.py`（映射 / 零漂移 / 隔离 / 回执 / 未映射跳过 / 热更新 / fail-open / 真身 app） |
| Rust 新守卫 | 3 支 `docs_router_tests`（补斜杠 / 页面与静态资源 / 反证不重复注册文档路由） |
| 定点守卫 | 引用尺与文档守卫 305 passed；l281 重钉后 57 passed；三处语料面常数按红读数重钉 |
| 离机构建 | `cargo check -p oa4rust --offline` exit 0；`openapi-paths` / `rust-lints` 门禁 PASS |
