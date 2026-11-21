# R314 D1 已办不同步修复（page-core）

## 现象
桌面「工作流待办 → 已办」页在审批通过后仍显示「暂无已办任务」。审批动作成功（x_task 状态变为 completed、x_record 落 complete 记录），但已办列表计数恒为 0。

## 根因核对（方向选择依据）
- 读端点已就绪：`processplatform_assemble_surface/src/lib.rs:19010-19060` 的 `taskcompleted_list_my_paging_page_size_size_core` 直接 `SELECT ... FROM PP_C_TASKCOMPLETED`，面相已办列表。
- 表结构真实存在：`backend/migrations/032_create_assemble_control_tables.sql:580-607` 定义 `pp_c_taskcompleted`（含 xperson / xtitle / xprocessName / id 等列），`101_add_missing_handler_columns.sql:123-125` 补 `xwork` 列。
- 写侧全局缺失：全仓 `INSERT INTO PP_C_TASKCOMPLETED` 命中 0；`task_complete` 只写 x_task / x_record / x_work，从不写已办表。
- 前端无缺陷：`ProcessWork.vue:298-299` 读 `/api/processplatform/assemble/surface/taskcompleted/list/my/paging/1/size/20`，`submit()` 在 :3186 已有 `queryClient.invalidateQueries({ queryKey: ['process-work'] })`。

**方向**：后端补写（在 task_complete 事务内同步插一条已办记录）。理由：读端点与表均已就绪、写侧是唯一缺口；若改前端「改读自有任务」会偏离 o2server 的 taskcompleted 契约投影，破坏与 H5/其他端的一致性。

## 字段映射
| 已办表列 | 来源 | 说明 |
| --- | --- | --- |
| xperson | task_complete 的 task.person | 任务负责人；集成测试据此计数 |
| xcreatorPerson | session.person_unique | 真实审批人（与 x_record.creator 同口径） |
| xid / xjob | task.id / work.id | 任务与工作标识 |
| xtitle / xprocessName / xapplicationName | x_work.title / process / application | 业务可读名 |
| xstartTime / xcreateTime | x_work.start_time / create_time | 时间文本 |
| xcompletedTime / xupdateTime | 本地当前时间 | 完成时间 |
| id | 新 UUID | 表主键 |
| creator / creator_person | x_record.creator / x_work.creator | 审计列 |

写入点与 x_task 状态更新、x_record 插入同处一个事务，`tx.commit()` 前完成，保证「审批成功 ⇒ 已办可见」原子性。

## RED → GREEN
- RED：`tests/taskcompleted_sync.rs` 在 scratch DB seed x_work/x_task，直接调 `task_complete` 后查 pp_c_taskcompleted.xperson 计数；实证 `审批完成后已办列表必须有记录（pp_c_taskcompleted count），实收 0`（exit=101）。
- GREEN：在事务内补写后，同一测试 1 passed（exit=0）。
- 踩坑：`ActionResult` 无 `success` 字段，改断言 `r#type == "success"`；`fmt_ts` 闭包签名需匹配 `Option::map` 期望的 `NaiveDateTime -> String`。

## 验证
- `cargo test -p processplatform_service_processing`：101 lib passed + 1 nullable + 1 taskcompleted_sync，0 failed（20 ignored）。
- `pnpm exec vitest run`：261 files / 2119 tests，0 failed。
- `pnpm run typecheck`：Scope 6/7 全 Done，exit 0。

## 追加发现：已办读端点 0 基分页
- `processplatform_assemble_surface/src/lib.rs:19015-19021` 用 `page_offset(page)` 把 page 当 0 基 OFFSET；待办端点 `task_list_my_paging_page_size_size_core` 用 `page_window(page,size)`（1 基）。
- 桌面原请求 `taskcompleted/list/my/paging/1/size/20` → OFFSET=1，跳过刚写入的唯一一条已办记录，UI 仍为空。
- 真浏览器实测：`paging/0/size/20` → count=1；`paging/1/size/20` → count=0。
- 修复：桌面 `ProcessWork.vue` 已办端点改 `paging/0/size/20`（inScope），契约同步断言 0 基路径；后端语义（23 处同类端点）未改，避免一次性改动其它分页语义。

## 测试服真浏览器复验
- 时序：t82（D2 上传 415 修复）完成后放行；基于含双方改动的 HEAD 先 Linux ELF 后端原子部署（strings 校验含 D1 SQL），再 web 原子部署（入口 index-95EPWgTt.js / index-BMFkaD73.css）。
- 流程：Playwright Chromium headless 登录 → /app/process → 记录已办基线 → 待办办理 → 审批通过 → 已办计数 → reload 持久 → H5 390x844。
- 结果（evidence.json，source=Playwright Chromium headless against deployed test server; not a physical device）：completedBefore=1、completedAfter=2、completedAfterReload=2、completeStatus=200、pageErrors=[]、apiFailures=[]；completedGteOne/completedIncremented/persistAfterReload/complete200/noPageErrors/noApiFailures 六项断言全部 true。
- 截图（位于 gitignored oa4rust/test-results/r314-taskcompleted-sync/，1440x900 除末项 390x844）：01-pending-before.png sha256:75bdc66513a7a8fafae768f093b042332382da20f7f4a0fea9220cb5d56a89ee；02-approved.png（右上角「审批通过」toast）sha256:59b96924beeb461b9a51ee3439857e8517b8d059c6e06dabe30e1c5e47bb07e7；03-completed-after.png（已办 tab 两条记录）sha256:4012177ab3d4be7a423266b52ff262f8fcb9721ba6ffca9be3beebad208dc8d1；04-process-h390.png sha256:4ea20d6c42a156da8dae38c42de25f6230e8fd792f3c7697c2548aaa57e4b90d。

## 证据索引
- 后端集成测试：`oa4rust/backend/crates/processplatform_service_processing/tests/taskcompleted_sync.rs`
- 桌面契约：`oa4rust/frontend/apps/desktop/src/views/TaskCompleted.R314.contract.test.ts`
- 浏览器证据（签名/截图片段由运行期注入，仓库内不写真域名或凭据）：`oa4rust/test-results/r314-taskcompleted-sync/`
- 真域名 / SSH 主机 / 凭据一律从本机环境或远端凭据文件读取，本文不含真值，仅用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位。
