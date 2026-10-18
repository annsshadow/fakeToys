# oa4rust 项目长期备忘

## 审计 campaign（分支 oa4rust-vuln-audit-r4 → 已合入 optimism20261012）
- 四轮安全审计主账本：`oa4rust/backend/docs/audits/2026-10-14-security-audit-round4.md`（§三 逐批记账，§五 处置与遗留）。
- **R7 战役已收尾**：基线 820 → 629（−191，第 65–84 批）；ORM 盲区复查队列（b65→b84）已清空。各批 commit 在 `oa4rust-vuln-audit-r4`，**已于 2026-10-18 合并进 `optimism20261012`**（合并提交 `e65238424` 并 push；合并后尺子复读 R7=629，与 r4 分支一致）。
- 进行中主题：GET 写面逐 crate 加门（尺子 `scripts/audit_write_gating.mjs` 产 `docs/audits/artifacts/*.tsv`）。**下一批入口 = auth 6 条**（OAuth 回调 GET 写，与「第三方回调零验签」族重叠，加门与验签分开记）。
- 遗留待修大项：第三方回调零验签族、`shared/src/crud.rs` CrudSpec 无归属、`GATE_RE` 里 `require_actor` 冗余词、尺子 `gateEvidence` 不剥注释（尺子缺陷 26）、`row_to_json` JSONB/时间盲口、只读挂写 764 等（详见账本 §五）。

## 本地真实库验证（Windows + Docker）
- Windows 主机上 Docker 原生 `5432:5432` 端口代理不通；用 `docker run -p 15432:5432 postgres:16`（`POSTGRES_USER=o2server/POSTGRES_PASSWORD=password/POSTGRES_DB=oa4rust`）发布到 **15432**。
- 迁移：`DATABASE_URL=postgres://o2server:password@localhost:15432/oa4rust ./target/debug/oa4rust.exe --migrate-only`（迁移文件在 `migrations/`，运行时从盘读、非编译期嵌入；追踪表 `schema_migrations`）。
- 单元测试：`shared::testing::test_pool()` 已支持 `DATABASE_URL` 覆盖（缺省回退 5432）。跑测试设 `DATABASE_URL=...@localhost:15432/oa4rust`。
- cargo：`/c/Users/Administrator/.cargo/bin/cargo`。整 workspace 高并发编译会撞 Windows 资源抖动（os error 5 写 rmeta / os error 1455 page file too small）；必要时 `CARGO_INCREMENTAL=0` + 降 `-j`，或定向 `-p <crate>` 跑。

## 关键代码约定
- `require_owner(pool, session, owner_id)`：admin 放行 + 身份归一化比较（trim+lower）。
- attendance `normalize_key` = `trim().to_lowercase()`，写库前身份归一化。
- 迁移命名 `NNN_desc.sql` + `NNN_desc_rollback.sql`（`_rollback.sql` 不会被运行器自动应用）。
