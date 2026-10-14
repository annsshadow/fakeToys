# 规划文档状态校准与知识沉淀对应报告

**日期**：2026-10-14　**范围**：`docs/plans/` 全部 19 份　**执行**：状态标记逐一与代码实测核对 → 更新 → 长期价值内容沉淀至 `docs/solutions/`

> 结构漂移总说明：计划中的 `oa4rust/crates/` 现为 `oa4rust/backend/crates/`（96 crates），`oa4rust-web/` 现为 `oa4rust/frontend/`（桌面 84 视图 + 移动 16 页），`ai/` 现为仓库根 `augmentor/`。所有"在档"证据均按新结构实测（2026-10-14）。

---

## 一、状态变更总表

| # | 规划文档 | 原状态 | 新状态 | 更新依据（代码证据，2026-10-14 实测） |
|---|---------|--------|--------|--------------------------------------|
| 1 | 2026-08-25-001 residual-gaps-closure | completed | **不变** | 收官记录与现状无冲突；96 crates、REPLACEABLE-08-25、fail_baseline 门禁均在档 |
| 2 | 2026-08-26-001 parity-convergence-phase2 | completed | **不变** | U1/U2（聚类脚本、CI 真实化）完成状态由 #8 执行进度报告与根 `.github/workflows/`（oa4rust-ci 等 4 个 workflow）印证 |
| 3 | 2026-08-26-002 fail-closure-master | completed | **不变** | 信封统一与三线工作已被后续 comparator 容忍改进 + 006 收敛（FAIL 193→136）覆盖印证 |
| 4 | 2026-08-26-002 envelope-unification | superseded | **不变** | superseded_by 指向已在档 |
| 5 | 2026-08-26-003 express-post | superseded | **不变** | 同上 |
| 6 | 2026-08-26-004 deep-logic-gaps | superseded | **不变** | 同上 |
| 7 | 2026-08-28-005 residual-fail-fix | active | **completed** | R6（FAIL ≤400）超额达成：006 进度报告实测 1821 PASS/193 FAIL（-73.9%），R500J200=0、R401J200 已豁免、R200J415=0；剩余收敛转入 #9/#15 轨道（已降至 136 ≤ 基线 170） |
| 8 | 2026-08-28-006-execution-progress | （无 frontmatter） | **不变**（加关联注记） | 该文件是 #7 的收官证据，非计划本体 |
| 9 | 2026-08-28-006 full-parity-closure-master | active | **superseded** | 由 2026-09-11-001（rev15 收官）与 2026-09-20-001（rev120 进行中）接管。**未达成项已显式记录**：R-test-coverage ≥95% 未达成（前端 42.97%，`docs/solutions/testing/final-achieve-95-coverage-report.md` 明确"未达成"；Rust crate 无 95% 证据），当前无活跃承接计划，如需继续须另立计划。R1 生产切流按 rev15 正式移除（deploy/ 下 shadow-traffic.sh、rollback-playbook.md 已删，实测） |
| 10 | 2026-09-04-001 frontend-o2web-next | （无 frontmatter，正文自称完成） | **completed**（补 frontmatter） | 桌面 84 视图实测、ProcessDesigner.vue 11,829 行、移动 16 页、backend 96 crates；09-09 安全修订由专项计划落地（Biome 2.5.11 于 `oa4rust/biome.json` 实测在档） |
| 11 | 2026-09-04-002 web-full-replacement | in_progress | **completed** | 结构目标 84/84 达成（`oa4rust/frontend/apps/desktop/src/views/` 实测 84 个）；48 个新视图抽查 11/11 在档（LogViewer/Selector/CmsDictDesigner/DesignCenter/Note/Template/Ann/Bam/QueryQuery/ProcessTaskCenter/CmsXform）；功能深度与壳裁决由 #15 rev15 闭环 |
| 12 | 2026-09-04-003 frontend-completion-audit | （无 status） | **superseded** | 2026-09-04 时效快照，"功能 ~8%"结论已被 #15 §三 重审取代（PD 807→11,807 行等）；保留作文审链 |
| 13 | 2026-09-06-001 stub-replacement | in_progress | **completed** | stub 2,194/2,194=100%（文末执行记录 + 2026-10-14 前端无 `async function api_/call_` 残留）；P1 parity 防回归意图由 fail_baseline=170 门禁承接（`backend/tests/behavior_comparison/fail_baseline` 实测）；P2/P3 由 091 种子 + `oa4rust/e2e/`（designer-roundtrip/workflow-runtime/oa-flows-live/app-coverage 四个 spec 实测）承接 |
| 14 | 2026-09-09-001 httponly-cookie/rustsec/biome | active | **不变（active）** | 外部未闭合项未变；新增 2026-10-14 定期复核节：RSA 例外已于 2026-10-09 再分类续期至 2026-11-08、lopdf-genpdf 例外有效至 2026-10-28（下次复核触发点）；SDK session.ts 无 token 持久化（仅 legacy 清理循环）；Java↔Rust 互操作项按 #15 rev15"无旧系统"实质失效 |
| 15 | 2026-09-11-001 assess-full-replacement | active | **completed** | rev16 收官：验收项 1/2/3/5 已达成（fail_baseline=170 门禁 + e2e 5/5 + W13 manifest + 书面边界），项 4 按 rev15 移除；2026-09-15 批次闭环（KNOWN_BACKEND_GAPS 注释留档于 backend 源码、091 迁移、realtime crate、w13-w14 manifest 均在档）；长期消费率移交 #16（rev120，609/4638=13.1%） |
| 16 | 2026-09-20-001 three-ends-completeness | active | **不变（active）** | 与代码一致：G5 长期推进中（frontmatter rev120，桌面消费 609/4638=13.1%，三端门禁 vitest 953/38 文件全绿）；无需改动 |
| 17 | 2026-09-21-001 augmentor-fullstack-audit | active | **不变（active）** | Phase 0/1 与代码一致；Phase 2 确认未启动（`augmentor/augmentor/capabilities.py` 与 `docs/CAPABILITY_MATRIX.md` 实测不存在）；加 2026-10-14 复核节（审计后 web 新增 Security/System 页、文档实位于 `docs/augmentor/`） |
| 18 | 2026-09-16-001 ai-data-augmentation-tool | active | **completed** | U1-U13 全部模块实测在档：models/（base/ernie/openai_model/**ollama**/factory）、quality、dedup、export、**context**、checkpoint、versioning、sampler、expander、tracker、visualizer、pipeline、api/main.py、web/；测试 3,100+、语句覆盖 99.50%/分支 98.63% |
| 19 | 2026-09-16-001 enterprise-ai-platform | active | **completed** | U1-U18 全部落地实测：**claude/gemini**、benchmark+performance_benchmark、preview/report、data/ 子包、multilingual/rag/evaluation、vector/（FAISS/ChromaDB）、active_learning、frameworks/、cli/ 包、REST 44 路由 11 router、web 11 页（9+Security/System）、tests 三层、`docs/augmentor/` 六件套、docker/、ai-platform-ci/cd.yml。**余项**：能力清单（capabilities.py）未建，归 #17 Phase 2 |

**校准后状态分布**：completed 10 ｜ superseded 4 ｜ active 3（#14 安全计划外部项、#16 三端长期、#17 augmentor Phase 2）｜ 非计划报告 2（无 frontmatter）。`grep -r "status: active" docs/plans/` 现在只返回真实在途计划。

---

## 二、沉淀文档与来源对应关系

| 沉淀文档（docs/solutions/） | 来源规划文档 | 沉淀内容 |
|---------------------------|-------------|---------|
| `security-issues/browser-session-cookie-httponly-migration.md` | #14（U2-U5、U12、Completion Evidence） | Cookie 契约七属性表、认证优先级（无效 Cookie 不回退 Bearer）、CSRF Origin 只卡危险方法、refresh 单飞/并发共享 Promise、四阶段迁移与回滚、Pinia 解包/requestSubmit 实测坑 |
| `security-issues/rustsec-pinned-audit-double-track-gate.md` | #14（U6-U8）+ 2026-10-14 例外再分类实测 | cargo-audit 独占 advisory / cargo-deny 三职拆分、pinned PR + floating daily 双轨、cache SHA 校验、例外七字段与 30 天到期硬失败、rsa 续期与 lopdf 不可达论证范式 |
| `security-issues/api-path-whitelist-and-write-authz.md` | #17（F1-F5、F12、F13、Phase 0） | resolve_data_path 三层白名单（绝对路径/非 .json/符号链接三逃逸）、写操作鉴权元测试（42 路由固化防漏挂）、限流"已实现未注册"防回归断言、np.bool_ 构造点收敛、__del__ getattr 兜底 |
| `architecture-patterns/greenfield-cutover-replaceability-criteria.md` | #15（§〇 判据重定义、§三 P0、§四、§六、§七 rev13-rev16） | 绿地切流 = 能力替代非数据迁移；四项验收判据 + S4 的立与废；「功能可替换 vs 100% 逐字节替代」边界声明；引擎 activities 键决定设计器契约方向；路由三档不闭合（404/405/静默影子）分级处置；壳视图 manifest 机器守卫 |
| `development-workflow/machine-audit-extraction-methodology.md` | #16（§〇、§6.8、§九 rev6/rev11/rev116） | 提取器 9 处缺陷"反例证伪再修"范式、只降不升基线门禁 + fail-injection 可红证明、消费率双口径（域桶 4072 vs 全局 4638）与 mock* 兼容层不可清理、注释勿写可截断 /api/ 字面量（rev116）、URL 一律模板字面量 |
| `testing/behavior-compare-tolerance-and-evidence-discipline.md` | #7/#8/#9（七项容忍+收益）+ #2（U2-U3 评审门）+ #15 rev9（S2） | 容忍规则量化收益表与方法冻结、聚类只产候选/三态评审/≤20 条小批重测、登出族请求前 SKIP 防 ~2000 条会话毒化、双侧全种子 fail_baseline=170 门禁语义（165 过/175 拦）、W12 双栈 A/B 实测度量闭环 |
| `testing/mutation-testing-operational-pitfalls.md` | #17（T1.4 及发现 1/2/3） | 裸 python test-command 假 100% killed（验尸官先自证 + `{python}`→sys.executable + shlex posix 正斜杠）、等价变异体五类清单与人工甄别、逐模块门槛防合计掩盖、断言被错误来源满足的三种典型 |
| `best-practices/test-hygiene-ast-triage-and-coverage-traps.md` | #17（F2/F6/F7/F10/F13、Phase 0/1） | 覆盖率度量执行≠行为验证（删 68 假测试覆盖率反升）、AST 分诊两次误报校准（变异绑定非常量；"能否失败"判据）、红→绿 stash 验证、同名二次导入符号遮蔽与跨枚举成员身份不认、版本号单一来源 + 文档数字不漂移 |
| `integration-issues/vue-spa-shell-routing-pitfalls.md` | #14（blank-shell 缺陷节） | /app/:appId + 空路径子路由无限重定向、v-for/v-if 同元素（Vue 3 B2）、TanStack Query v5 单参数签名、四断言浏览器黑盒验证清单 |

**沉淀覆盖**：19 份规划中有长期价值的 9 个主题已沉淀；#1-#6、#10-#13 的方法论此前已由既有 solutions 覆盖（parity-closure-campaign、actionresult-9-field-contract、seaorm-dual-pool、nested-tokio-runtime、auto-generate-rust-handler-tests、single-source-of-truth、plan-status-lifecycle 等），本次未重复造文，仅在上述新文中交叉引用。

---

## 三、遗留风险与待办（fail loud）

| 项 | 等级 | 说明 |
|----|------|------|
| 测试覆盖率 ≥95% 目标无承接 | 中 | #9 的 R-test-coverage 未达成（前端 42.97%），当前无任何活跃计划推进；如需继续应立新计划，否则应在下次规划评审中正式关闭该目标 |
| lopdf-genpdf 例外 2026-10-28 到期 | 中 | 到期前必须再分类（替换 genpdf 或下线导出特性），触发点见 #14 复核节 |
| augmentor Phase 2 能力清单未启动 | 低 | #17 唯一未开工阶段（capabilities.py + CAPABILITY_MATRIX.md 缺失已实测确认） |
| T1.14 冗余数据文件清理待用户确认 | 低 | ~11.5 MB（train_data*.json md5 重复 + bak/ + 重复 enhanceTXT.py），属破坏性操作，须用户逐项确认 |
| three-ends G5 消费率 13.1% | 长期 | #16 rev120 持续推进中，非阻塞项 |
| 历史计划引用已删目录 | 低 | `2026-08-25-001` 有 7 处指向 2026-10-14 已删除的 `docs/oa4rust/` 模块卡片路径（本会话前已存在，随该计划 completed 状态保留为历史上下文，如需清理另起一笔） |

## 四、复现命令

```bash
# 状态总览
for f in docs/plans/*.md; do grep -m1 '^status:' "$f"; done
# 关键证据
cat oa4rust/backend/tests/behavior_comparison/fail_baseline      # 170
cat oa4rust/backend/security/rustsec-exceptions.toml             # 例外到期日
ls oa4rust/frontend/apps/desktop/src/views/*.vue | wc -l         # 84
ls -d oa4rust/backend/crates/*/ | wc -l                          # 96
ls oa4rust/backend/deploy/                                       # shadow-traffic.sh 已删
ls augmentor/augmentor/capabilities.py 2>&1                      # 不存在 = Phase 2 未启动
```
