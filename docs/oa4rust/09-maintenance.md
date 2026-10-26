# 09 · 后续维护与更新约定

## 1. 文档分层与"单一信息源"原则

| 信息 | 权威来源 | 本文档集的角色 |
| --- | --- | --- |
| 迁移完成度 / 端点数 | `oa4rust/backend/docs/brainstorms/oa4rust-endpoint-inventory.md`、`oa4rust-migration-status.md`（脚本生成） | 引用，不手工改数字 |
| API 明细 | 运行时 `/openapi.json` + 各 crate `routes.rs` | 讲结构与模式，不抄全量清单 |
| 安全审计 | `oa4rust/backend/docs/audits/YYYY-MM-DD-security-audit.md` | 引用结论 |
| 部署 / 回滚 | `backend/docs/ops/REALIZE_RUNBOOK.md` §4.2（待确认：该文件当前不在仓库中）、`deploy/nginx.conf` | 引用上线判据与回退口径 |
| 本文档集 | `docs/oa4rust/` | **入口与导读**：架构、流程、约定、FAQ |

**规则**：生成类数字（crate 数、handler 数、迁移数）在本文档集中出现时必须标注"以 XX 为准"；发现不一致时，改权威源 + 重新生成，而不是手改本文档。

## 2. 代码变更的强制同步项

改动落在下表左列时，右列动作**必须**完成（属于"完成"的定义，不是可选优化）：

| 变更 | 同步动作 |
| --- | --- |
| 新增/修改/删除后端路由 | 更新三端契约 fixture（`oa4rust/tests/contracts/`）+ 前端 `@oa4rust/apis`；移动端按需暴露；跑 `pnpm test` |
| 新增数据库对象 | `migrations/NNN_xxx.sql` **成对**提供 `NNN_xxx_rollback.sql`；命名连续编号 |
| 新增/删除 crate | workspace `Cargo.toml` members + `lib.rs` merge + 重新跑 `backend/scripts/regen_endpoints.py` 刷新 inventory（待确认：该脚本当前实际产出 `tests/behavior_comparison/endpoints.rs`，清单自述生成器 `scripts/gen_inventory.py` 不在 `backend/scripts/`） |
| 安全例外 | `security/rustsec-exceptions.toml` 带到期日 + `rustsec-db.rev` 抬版 + 审计记录 |
| 配置项（env） | `.env.example` 同步 + 本目录 `05-environment.md` 更新 |
| 公开 API 行为变化 | 审计对齐记录（`docs/audits/`）+ OpenAPI 注解（utoipa） |
| 豁免路径 / 限流目标变更 | `shared/src/middleware/constants.rs` + 安全审计确认（**不得静默**） |

## 3. 提交前门禁（Definition of Done）

后端（`oa4rust/backend/`）：

```bash
cargo check --workspace --locked
cargo test --workspace --lib --locked
```

前端（`oa4rust/` workspace 根）：

```bash
pnpm test && pnpm typecheck && pnpm lint:check && pnpm build
```

全绿才可提交。任何被跳过的测试必须显式说明（fail loud，不允许静默 skip）。

## 4. Git 工作流约定

- 仓库：`fakeToys`（oa4rust 为其子目录），提交遵循 Conventional Commits + 中文描述，scope 用 `oa4rust`，例：
  - `fix(oa4rust/backend): ...`、`sec(oa4rust/frontend): ...`、`test(oa4rust/backend): ...`、`docs(oa4rust): ...`
- 分组分批提交：后端 / 前端 / 规范 / memory 分开提交，不混无关变更（vendor/README 遗留不入功能提交）。
- 安全审计在独立 worktree（如分支 `oa4rust-vuln-audit`）进行，避免污染主工作区（2026-10-14 审计即此模式）。
- 大改造先落计划文档于 `fakeToys/docs/plans/`（目录按需新建，命名 `YYYY-MM-DD-NNN-*.md`；已完结计划移入 `docs/archived-plans/` 归档）。

## 5. 文档集自身的维护约定

- **触发时机**：架构决策变化、模块增删、配置项增删、门禁命令变化、迁移里程碑（如 todo 清零）时，同步修订对应分册并在文首快照日期更新（README 的"基于当前代码实测梳理"行）。
- **修订原则**：只改必要分册；生成类数字回写权威源；新增 FAQ 条目须有可复现根因（来自真实踩坑或审计结论，不写推测性问题）。
- **废弃流程**：文档失真且短期无法核实时，在对应小节标注"待核实（日期）"，禁止删除后留空引用。

## 6. 例行维护节奏

| 周期 | 动作 | 依据 |
| --- | --- | --- |
| 每日（CI） | `cargo audit`（浮动门，`--deny unsound/vulnerable`）；`pnpm audit` | `docs/security/rustsec-audit.md` |
| 每次真实化推进 | `regen_endpoints.py` 刷新 inventory / migration-status（待确认：该脚本当前实际产出 `tests/behavior_comparison/endpoints.rs`）；更新本文档集速览数字 | 迁移状态文档头部说明 |
| 安全例外到期前 | 复验可达性：升版本或续期（必须留审计记录） | rustsec-exceptions.toml 到期日 |
| 每次发布 | 静态门禁全绿 + live E2E 通过即上线；回滚 = 回退上一 Rust 版本 | `backend/docs/ops/REALIZE_RUNBOOK.md` §4.2（待确认：该文件当前不在仓库中） |

## 7. 已知待办与风险登记（2026-10-26 快照）

| 项 | 状态 | 位置 |
| --- | --- | --- |
| `sms`、`captcha_store` crate 真实化 | todo（最后 2 个业务 crate） | endpoint-inventory 汇总 |
| 移动端原生/小程序 token 鉴权通道 | 待后端提供（Cookie 不跨容器） | `mobile/README.md` 鉴权模型 |
| Swagger UI 挂载 | zip 已备（swagger-ui-v5.17.12.zip）未接线 | `backend/` 根目录 |
| lopdf 0.26.0 例外 | 2026-10-28 到期（genpdf 生成 PDF 用，无解析入口） | security/rustsec-exceptions.toml |
| rsa 0.9.10 例外 | 2026-11-08 到期（仅 dev-dependency 可达） | 同上 |
| `spent` 精确耗时 | 未实现（信封恒 0，对齐 o2server v9 行为） | response.rs 注释 |
| 平均 DB 真实化率 13.8% | done crate 仍有深层 DB 化空间（handler 已无桩） | endpoint-inventory |
| FI-002 列完整性扫描 | **已收口**：`scripts/fi002_column_scan.py` 固化（WHERE/ORDER BY/引号列/视图投影列/大小写关系全盲补），基线 7→0；CI 棘轮 `fi002-column-scan` job 每轮跑，新增违规即红。2026-10-26 批次 105–112 迁移收口 fresh-DB 500 端点（含 `PP_E_APPLICATION` 视图、`x_program_field.sort_order`、`x_work.category` 等），`schema_diff.py` 对账脚本入库 | `backend/scripts/fi002_column_scan.py`、`backend/scripts/schema_diff.py`、`migrations/105–112` |
| fresh-DB 500 端点收口台账 | 第三轮全量收口完成，台账存档于 `fakeToys/docs/plans/2026-10-26-fresh-db-500-closure-round.md`（F-1~F-8/FX-1 缺陷 + B-1~B-7 扫描器盲区，B 族全收口） | 该计划文档 |

## 8. 求助路径

1. 先查本文档集 [08-faq.md](08-faq.md)；
2. 后端专题：`oa4rust/backend/docs/`（audits / security / solutions / ops）；
3. 历史计划与需求：`fakeToys/docs/archived-plans/`、`fakeToys/docs/brainstorms/`；
4. 仍无结论 → 按第 4 节 Git 约定开计划文档（`docs/plans/`，按需新建）登记问题与验证标准。
