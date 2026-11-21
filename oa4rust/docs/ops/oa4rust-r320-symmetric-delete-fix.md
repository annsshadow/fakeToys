# R320 section 删除与桥接 forum 对称清理修复

## §1 背景与来源

承接 t101 甄别（oa4rust/docs/ops/oa4rust-r320-section-delete-orphan-triage.md）：判定 B 真缺陷 P2——PC 真 UI 删版块后 section/list 已无该版块，但 t98 桥接（commit 93818d00c）写入的同 id x_bbs_forum 未同步清理，H5 forum/view-all 与发帖 picker 仍可见孤儿 forum，关联主题成孤儿。本轮按队长选定方向一实施对称桥接删除，test-first RED→GREEN。

## §2 实现（bbs_assemble_control/src/lib.rs section_delete）

修复前：section_delete（lib.rs:359-375）仅 shared::crud_delete(&pool, &section_create_spec(), &id)，section_create_spec.soft_delete=false → 硬删 section，无 forum 清理。
修复后（lib.rs:368 起）：
- 用 shared::db::acquire(&pool) 取连接并开启事务（let mut client / client.transaction()），两条 DML 同一事务提交，杜绝「section 删了、forum 还在」半状态。
- ① DELETE FROM x_bbs_assemble_control_section WHERE id=$1（与 crud_delete 同口径白名单静态 SQL + $1 占位，deleted_rows>0 决定返回 deleted）。
- ② UPDATE x_bbs_forum SET deleted_at=NOW() WHERE id=$1 AND deleted_at IS NULL（复用 u2.rs:1433 同口径，forum id 复用 section id，与 t98 建版块桥接对称）。
- 两条 DML 均 .map_err(|e| AppError::internal_with_source(...)) 真 DB 错误 fail loud；tx.commit() 同样 fail loud，事务回滚保证无半状态。
- 历史无桥接 forum 的 section：步骤② affected=0 属正常（不误判、不改 deleted 返回值）；admin 门（:365 is_admin→403）、creator=会话人语义、section/list 读取链、t98 建版块桥接、伪桥接第三表 x_bbs_section 均未改动。

## §3 RED→GREEN

- 契约 1（行为，live-gated）：tests_generated.rs test_section_create_bridges_to_forum 之后新增 test_section_delete_symmetric_bridged_forum_cleanup——admin 探针建版块（section/create）→ forum/view/all 前置可见且 id 一致 → 真 DELETE section/delete/{id} → 断言 forum/view-all 不再含该 id、DB 回读 deleted_at 非空 → 清理两表。修复前 RED：panicked at tests_generated.rs:406「RED: section/delete must soft-delete bridged x_bbs_forum with SAME id; id=... still visible in forum/view/all」。
- 契约 2（静态不变量，防回归）：test_section_delete_bridge_cleanup_invariants 断言 section_delete 函数体含 UPDATE x_bbs_forum SET deleted_at = NOW()、&[&id]（同 id 不误删）、map_err(|e| AppError::internal_with_source（fail loud）。
- 编译坑：client.transaction() 需 let mut client（沿用 t82 教训）；修后 GREEN。
- 全量：cd oa4rust/backend && cargo test -p bbs_assemble_control --lib → 49 passed; 0 failed; 0 ignored; 0 measured（基线 47 + 新增 2，既有 section/forum 用例全部不回退）。

## §4 部署与真浏览器复验

- Linux ELF 构建：gitignored oa4rust/test-results/build-linux-r320bbs.sh（docker rust:1.98-bullseye 内层 bash -lc 单引号、touch lib.rs+tests_generated.rs 强制重编、cargo +1.98.0 build --release -p oa4rust --offline --target-dir /workspace/target/linux），后台 12m39s 完成 R320BBS_BUILD_DONE，新 ELF 118342728 字节（旧 118343112），`file`=ELF executable（Windows PE32+ 在测试服 203/EXEC，禁用）。
- 原子部署：oa4rust/test-results/deploy-backend-r320bbs.sh（scp→备份 bak-r320bbs→install→mv 原子替换→healthcheck 失败回滚，远端命令单引号避免本地展开）输出 SERVICE_ACTIVE=active / BACKEND_DEPLOY_OK，exit 0。
- 真浏览器：oa4rust/test-results/r320-bbs-delete-verify.cjs + r320-bbs-delete-verify-run.sh（Playwright Chromium headless、1440x900 PC + 390x844 H5、凭据 ssh 运行期读取不回显、base 从既有脚本提取），证据 oa4rust/test-results/r320-bbs-delete-fix/（4 PNG + evidence.json，gitignored；source=本地无头 Chromium 渲染，非物理真机）。
- 流程与判定：PC 真 UI 建版块（sectionId 383c6e59…）→ section/list found=true → H5 forum/view-all visible=true + picker 含新名 → PC 真 UI hover+confirm 删版块 → section/list present=false、forum/view-all visible=false → H5 硬重载后 picker 不再含该名。
- 首轮 NOT_VERIFIED 的根因：脚本对相同 hash URL（#/pages/bbs/new）二次 goto，uni-app H5 路由不重走 onLoad→loadForums（mobile/src/pages/bbs/new.vue:69-75），picker 保留删除前缓存数组；后端 forum/view-all 删除后 visible=false 已证明修复生效。将二次 goto 改为 h5.reload() 硬重载后重跑：verdict=SYMMETRIC_DELETE_VERIFIED，pcSectionFound=true、forumBefore=true、pickerBefore=true、sectionAfterPresent=false、forumAfter=false、pickerAfter=false、apiFailures=0、errors=0。

## §5 脱敏

真域名/测试服地址一律以 <YOUR_TEST_DOMAIN> 占位，0 真值；口令仅运行期经 ssh 从 .local.keys 读取、不回显不入库；版块 id 仅以首 8 位展示，完整 id 见 gitignored 证据。本文档不含任何真域名、IP、口令、私钥或个人路径。

## §6 已知边界

- 关联主题不自动删除：本修复只清理桥接 forum（H5 picker/forum 不再可见已删版块），该 forum_id 下的历史主题仍保留，属队长明确的已知边界；主题归属策略（随版块归档 / 标记原版块已删除 / 列表过滤）另轮评估。
- 历史遗留：t98 之前创建的无桥接 section，其同 id x_bbs_forum 不存在，删除时步骤② affected=0 属正常；历史孤儿 forum 不在本轮迁移范围（禁历史数据迁移）。
- 不削弱 admin 门/creator 语义、不改 PC section/list 读链、不改伪桥接第三表、不改 t98 建版块桥接。

## §7 证据索引

- 契约：oa4rust/backend/crates/bbs_assemble_control/src/tests_generated.rs（test_section_delete_symmetric_bridged_forum_cleanup / test_section_delete_bridge_cleanup_invariants）。
- 源码：bbs_assemble_control/src/lib.rs:359-375（section_delete）、u2.rs:1433（同口径软删 SQL）、routes.rs:42-43（section/delete/{id}）。
- 对照：oa4rust/docs/ops/oa4rust-r320-section-delete-orphan-triage.md（t101 判定）、oa4rust/docs/ops/oa4rust-r318-bbs-section-bridge-fix.md（t98 建版块桥接）。
- 真浏览器证据目录：oa4rust/test-results/r320-bbs-delete-fix/（gitignored，本地无头 Chromium 渲染，非物理真机）。

## §8 结论

- 对称桥接删除修复成立：section_delete 在单事务内硬删 section + 同 id 软删桥接 x_bbs_forum，真 DB 错误 fail loud、无半状态、affected=0 不误判；admin 门/creator/section 读链/t98 建版块桥接/伪桥接第三表均未改。
- 真浏览器双端复验 SYMMETRIC_DELETE_VERIFIED（R320删除专测_1795267654306，sectionId 383c6e59…），PC 删版块后 H5 forum/view-all 与发帖 picker 均不再可见该版块，0 API 失败/0 pageerror。
- 关联主题不随版块删除自动清理（队长明确的已知边界，见 §6），历史无桥接 forum 与历史孤儿不在本轮迁移范围。
- 项目 ≥1000 轮持续迭代，本轮为其中一环，非终态。
