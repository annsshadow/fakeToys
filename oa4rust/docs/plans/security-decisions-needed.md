# 安全裁决清单（2026-11-06）

> 用途：把「已查清、但改代码前必须先定口径」的条目一次性摆到桌上。每条都能当场回答，不需要翻审计报告。
>
> 本文只做裁决，不改业务代码。默认原则：**没有裁决就不动**；必须动时按 fail-closed（拒绝）处理，历史空归属视为无权。

## 默认裁决（已采用，业务方可推翻）

> 2026-11-06 由业务方先行确定，作为实施默认值。**可推翻**：推翻时只改本节，下文章节保留原始证据与备选方案，不改判据。本节生效后，第 2–5 节降级为证据附录，真正待开会项见本节末表。

1. **读权限统一「本人 OR 管理员」**；历史行无归属值的，按「仅管理员可读」兜底（fail-closed）。
   - 对应：`2.1` 的 12 条、`2.2` 的 5 条（补列后生效）、`2.3` 中能落到主体列的条目、`5` 的全部 E13–E24。
   - 「管理员」按 `admin_policy` 两分：`allow` 的端点 admin 是合法读者；`deny` 的端点（个人 PII、考勤明细等）admin 也不放行。E18–E24 分组就是用来把 `unknown` 收敛进这两档。
2. **cms 口径**：`published` = 全员可读（匿名端点同样）；`draft` 只本人与管理员可见。owner 同时认 `creator` 与 `author_id`。
   - 对应：`3` 的 C1–C3，以及 `5` 的 E13、E18。
3. **软门统一改 403**：不再返回 `200 + ActionResult{success:false}`。前端已能处理 `PermissionError`，不保留兼容层。
   - 对应：`4` 的 S1、S2。
4. **5 条「列从不写入」**：新增行在写入时补归属列（附件 = 上传人）；**历史行不回填**，按第 1 条兜底「仅管理员可读」。
   - 不回填的理由：`x_cms_fileinfo.upload_person` 历史上全部为 NULL，没有任何可信来源能反推上传人；改用「父文档 ACL」反推，等于把授权绑到可能已迁移/删除的 `doc_id` 上，是用猜测建立授权。先拒绝、后按真实需求逐行人工回填，风险更低。
   - 对应：`2.2` 的 #21–#25。
5. **E13–E24**：`admin` 视为合法读者（可读），但非 admin 必须按归属过滤（`owner OR 业务关系`），不允许只按「登录」放行。
   - 对应：`5` 的 12 组（E13、E14–E16、E17、E18–E24）。

### 采用默认后仍需业务回答的（不因本节关闭）

| 剩余问题 | 条数 | 默认值覆盖不到的原因 |
|---|---:|---|
| 真·全局表「归谁」 | 14（`2.3` #1–#14） | 这些表**没有主体列**，`本人 OR 管理员` 无从落值，必须先在「应用命名空间 / 全局配置 / 组织范围」里选一档 |
| 关联边归属 | 1（`2.1` #17） | `source_id`/`target_id` 指向**实体**而非人，要决定「按端点实体所有者」还是「仅平台 admin」 |
| 写侧留痕/审批 | 2（#13 会议室照片、#31 admin 代改） | 读侧已定，但「谁能写」和「代改是否留痕」是流程问题，不是权限谓词问题 |
| `deny` 端点清单 | 待收敛 | 第 1 条的「admin 也放行」只对 `allow` 成立；`deny` 需逐条点名，未点名的默认按 `unknown` 继续进基线 |

**合计仍待业务回答 17 条 + `deny` 清单**（14 + 1 + 2）。

## 0. 怎么用这份清单

每条只需要业务回答三个问题（后文简称「三问」）：

1. **读权限**：登录即可读 / 只有本人和 admin / 只有有业务关系的人（成员、参与者、同单位）。
2. **写权限**：本人 / 业务管理员 / 平台 admin。
3. **历史行没有归属值**：拒绝（推荐）/ 归给 admin / 回填系统账号。

证据图例：

- `HTTP`：审计时实际发过请求，引号内是实测响应原文。
- `STATIC`：只做了代码/静态扫描（`status=None`、`marks=[]`），**没有发 HTTP 请求**。这类条目上会前仍建议补一次实测。

## 1. 总览

| 分组 | 条数 | 性质 | 需要谁拍板 |
|---|---:|---|---|
| NO-OWNER-COL：可修 | 12 | 工程活，只需确认「本人 or admin」 | 安全 + 业务确认口径 |
| NO-OWNER-COL：列从不写入 | 5 | 先定「谁在什么时点写归属列」 | 业务 + 数据/后端 |
| NO-OWNER-COL：真·全局 | 14 | 定「这张表归谁」 | 业务 |
| cms 假门 | 3 | 已收口为 403，剩「published 是否等于公开」 | 业务 + 产品 |
| 软门 | 2 | HTTP 语义：403 还是 200+body | 前端/客户端负责人 |
| E13–E24 读越权 | 12 组 | 分组定「admin 是否是合法读者」 | 业务按域拍板 |

**合计 48 个可裁决项**（31 + 3 + 2 + 12）。其中 31 条的读侧、cms、软门、E13–E24 已由文首「默认裁决」覆盖；实际仍需上会的是该节末表的 17 条 + `deny` 清单。

---

## 2. NO-OWNER-COL：31 条

### 2.1 第一档：12 条可直接修（有归属列且生产路径确实写入）

这些不需要业务重新设计，只需要确认一句：**读 = 本人 OR admin；写 = 本人（管理员操作走管理端点）**。历史空值 fail-closed。

| # | 端点 | 归属列 | 当前行为（证据） | 风险 | 方案①（推荐）/ 方案② | 需业务回答 |
|---|---|---|---|---|---|---|
| 15 | `GET /api/attendance/assemble/control/v2/shift/{id}` | `x_attendance_v2_shift.creator_person` | `STATIC`：SELECT 未取 `creator_person`；登录即可读 | 班次模板、排班规则跨部门可见 | ① `creator_person = 会话人 OR admin`；② 全局模板，登录可读 | 班次模板是否允许跨部门复用？ |
| 16 | `GET /api/attendance/assemble/control/v2/group/{id}` | `creator_person` / `participate_list` / `assist_admin_list` / `top_unit` / `unit_list` | 登录即可读；样本含全部成员列 | 泄露他人排班与成员名单 | ① 成员/协管/创建人可读；② `top_unit`/`unit_list` 组织范围内可读 | 考勤组可见性按「成员」还是「组织范围」？ |
| 17 | `DELETE /api/correlation/core/entity/delete/{id}` | `x_corr_c_correlation.source_id` / `target_id` | `STATIC`：`marks=[]`；登录即可删 | 可拆任意关联，破坏审计链 | ① 按 `type` 分派到端点实体所有者；② 仅平台 admin | 关联边归端点实体，还是全局数据？ |
| 18 | `GET .../authentication/person/{id}/icon` | `auth_person.id`（本人） | 登录即可读；低敏 | 可枚举组织成员头像 | ① 头像视为公开资料；② 仅本人/通讯录可见 | 头像是否组织内公开？ |
| 19 | `GET .../personal/{id}/setting` | `auth_person.id` | 任何人按 id 读他人 `mobile/email`（PII） | 手机号/邮箱扩散 | ① 本人或 admin；② 本人 + 同单位可互查 | 手机号/邮箱是否允许同单位互查？（建议不允许） |
| 20 | `GET /api/process/task/{id}` | `x_process_task.assignee_id` | 登录即可读标题/状态 | 待办内容与流程信息泄露 | ① `assignee_id` 本人 + 流程发起人 + admin；② 同流程参与者可见 | 同流程协作者是否可见任务详情？ |
| 26 | `GET /api/program_center/warnlog/{id}` | `x_program_warn_log.creator_person` | 登录即可读 `host/port/message` | 内部拓扑、主机端口泄露 | ① 运维/admin 全局 + 对普通用户脱敏；② 仅触发行人可见 | 普通用户是否可看告警的 `host/port`？ |
| 27 | `PUT /api/organization/bind/{id}` | `x_org_bind.identity_id` | 登录即可改他人绑定 | **提权**（把自己绑进高权组） | ① 仅 `identity_id` 本人；② 组管理员/admin | 入组/退组是否允许本人自助？ |
| 28 | `DELETE /api/organization/bind/{id}` | `x_org_bind.identity_id` | 登录即可删他人绑定 | 踢人/破权 | ① 仅 `identity_id` 本人；② 组管理员/admin | 同上 |
| 29 | `PUT /api/organization/custom/{id}` | `x_org_custom.identity_id` | 登录即可改他人字段 | 伪造部门/职级 | ① 仅 `identity_id` 本人；② 字段定义管理员 | 部门/职级这类字段是否只允许管理员改？ |
| 30 | `DELETE /api/organization/custom/{id}` | `x_org_custom.identity_id` | 登录即可删他人字段 | 同上 | ① 仅本人；② 字段管理员 | 同上 |
| 31 | `POST .../personal/custom/{id}/mockputtopost` | `x_org_custom.identity_id` | `STATIC`：中间件已对普通用户 403；剩 admin 可改任何人 | admin 代改是否留痕 | ① 接受，补审计日志；② handler 级再校验本人 | 管理员代改是否需要审批/留痕？ |

### 2.2 第二档：5 条「列存在但生产路径从不写」

共同前提：`x_cms_fileinfo.upload_person` 除「从已有行复制」外，所有 INSERT 都不带该列（`cms_assemble_control/src/lib.rs:4277/4754/4791/4828/4855/11604`；`:4700` 有注释说明）。当前实际为 NULL。**先有列值，才谈得上按列收口。**

| # | 端点 | 当前行为（证据） | 风险 | 方案①（推荐）/ 方案② | 需业务回答 |
|---|---|---|---|---|---|
| 21 | `GET /api/document/{id}/view/count` | `STATIC`：只 `SELECT 1` 判存在；登录即可读 | 可枚举文档存在性 | ① 跟随文档 ACL（`x_cms_permission` + `creator` + `published`）；② 仅 `published` 可读 | 未发布文档的作者本人能否看计数？ |
| 22 | `GET /api/fileinfo/{id}` | `STATIC`：`upload_person` 恒 NULL；任何人按 id 读他人附件元数据 | 原文件名/大小/类型泄露 | ① 跟随父文档 `doc_id` 的 ACL（现在就能修）；② 先补写 `upload_person`，再按上传人私有 | 附件归属是「上传人」还是「父文档」？ |
| 23 | `GET /api/fileinfo/{id}/online/info` | 同 22 | 在线预览元信息泄露 | 同 22 | 同 22 |
| 24 | `GET /api/fileinfo/{id}/preview/pdf` | 同 22；SELECT 不取 `upload_person` | **任何人可渲染他人附件 → 内容泄露（最高危读）** | ① 父文档 ACL + `downloadable`；② 登录即可（现状，不推荐） | 预览是否必须继承下载权限？ |
| 25 | `POST /api/fileinfo/update/{id}/content` | 同 22；登录即可改他人附件元数据 | 污染/伪造附件元数据 | ① 父文档编辑者 + admin；② 补列后按上传人 | 元数据编辑权归父文档编辑者还是上传人？ |

### 2.3 第三档：14 条真·全局（无主体列，必须业务定「归谁」）

| # | 端点 | 数据 | 当前行为（证据） | 风险 | 方案①（推荐）/ 方案② | 需业务回答 |
|---|---|---|---|---|---|---|
| 1 | `GET /api/general/dict/{id}` | 应用级键值字典，`application` 是命名空间 | 登录即可读；写无门 | 跨应用字典互相可见/被改 | ① 按 `application` 归属（写=该应用管理员）；② 全局共享（写=平台 admin） | 字典是否允许跨应用复用？ |
| 2 | `GET /api/general/dict/item/{id}` | 字典项，`dict_id` 指向父字典 | 登录即可读 | 泄露所有业务字典 | ① 随父字典；② 全局共享 | 同上 |
| 3 | `POST /api/general/dict/delete/{id}` | 同 1（软删） | **登录即可删（当前真实可利用）** | 删他应用字典 | ① 应用管理员；② 平台 admin | 应用管理员能否自删本应用字典？ |
| 4 | `POST /api/general/dict/item/delete/{id}` | 同 2 | 登录即可删 | 同上 | ① 随父字典；② 平台 admin | 同上 |
| 5 | `POST /api/general/dict/item/update/{id}` | 同 2 | 登录即可改 `name/value` | 篡改业务字典 | ① 随父字典；② 平台 admin | 同上 |
| 6 | `POST /api/general/dict/update/{id}` | 同 1 | 登录即可改 `name/application` | 改命名空间逃逸权限 | ① 应用管理员 + 禁止改 `application`；② 平台 admin | 是否允许改 `application` 字段？ |
| 7 | `GET /api/cms/category/{id}` | 栏目树节点，`parent_id` 层级 | 登录即可读；写无门 | 栏目结构泄露/被改 | ① 全局栏目树，读=登录、写=admin；② 按节点归属（需新增 `creator_person` 并回填） | 栏目树是否全局？父栏目管理员能否管子栏目？ |
| 8 | `GET /api/jpush/template/{id}` | 推送模板库 | 1 行样本；登录即可读 | 模板内容跨应用可见 | ① 全局模板库（读=登录、写=admin）；② 创建人私有（需新增 creator） | 模板是否允许跨应用复用？ |
| 9 | `GET /api/jpush/core/entity/template/{id}` | 同 8（core 别名） | 登录即可读 | 同 8 | 同 8 | 同 8 |
| 10 | `GET /api/meeting/assemble/control/building/{id}` | 楼宇目录 | 1 行；登录即可读 | 跨单位楼宇信息可见 | ① 全局资源目录（读=登录、写=admin）；② 按 `unit_id` 归属 | 楼宇是否按单位隔离？ |
| 11 | `GET /api/meeting/assemble/control/room/{id}` | 会议室，`open_meeting` 是否开放 | 5 行；登录即可读（含未开放房间元数据） | 未开放房间名称/容量/设备泄露 | ① 全局目录；② `open_meeting=true` 或预订人/管理员才可见 | 未开放会议室是否从目录隐藏？ |
| 12 | `GET /api/meeting/core/entity/room/{id}` | 同 11（core 别名） | 登录即可读 | 同 11 | 同 11 | 同 11 |
| 13 | `POST /api/meeting/assemble/control/room/{id}/photo` | `x_meeting_room.photo` | 只 `SELECT room WHERE id`，无门；登录可写 | 任意人往任意房间塞图（污染/钓鱼） | ① 会议室管理员；② 任意登录用户（现状） | 谁可以改会议室照片？ |
| 14 | `POST /api/attendance/core/entity/rule/{id}/update` | 全局考勤规则（上下班时间） | `main` 上无门；修复线工作树已加 `require_admin`（未合入） | 一人改规则影响全公司 | ① 全局配置，写=admin；② 按考勤组 `group_id` 归属 | 考勤规则是全局还是每组独立？ |

### 2.4 31 条的裁决模板

按顺序回答，能一次覆盖 12+5+14：

1. 这张表的读权限：登录 / 本人+admin / 有业务关系的人。
2. 这张表的写权限：本人 / 业务管理员 / 平台 admin。
3. 历史空归属：拒绝（推荐）/ 归 admin / 回填系统账号。

---

## 3. cms 假门（3 条）

> 审计时实测是 `200 + 布尔`，任何登录用户都能探测任意文档；修复线已把不可读改为 `403`。**下面写的是当前 main 行为**，剩下的只是口径裁决。

| # | 端点 | 当前行为（证据） | 风险 | 方案①（推荐）/ 方案② | 需业务回答 |
|---|---|---|---|---|---|
| C1 | `GET /api/cms/document/control/{id}` | 代码：非 admin、非 `creator`、非 `published` → 403；放行后 `allowRead=true`，`allowEdit/Manage = admin OR creator`（`cms_assemble_control/src/lib.rs:11173-11188`）。审计实测：200 + `allowRead:true`（`evidence-round6.txt:43`、`evidence-round7.txt:641-665`） | 控制位可被探测；`published` 对所有登录用户放开；owner 只认 `creator`，忽略 `author_id` | ① 保持 `published` 公开，但控制位仅 creator/author/admin；② 控制位收紧为 admin 专用 | `published` 是否等于「全员可读」？文档 owner 是 `creator` 还是 `author_id`？ |
| C2 | `GET /api/cms/document/permission/read/{id}` | 代码：`readable = admin OR creator OR author_id OR published`；不可读 → 403（`:11241-11251`）。审计实测：200 + `readable` 布尔（`evidence-round6.txt:43`） | 权限探针暴露文档存在性与 ACL 结果 | ① 与正文同一个 ACL（读权限者才能探）；② 探针仅 admin | 普通用户是否需要调用权限探针？ |
| C3 | `GET /api/cms/document/cipher/permission/read/{id}/{person}` | 代码：非 admin 且路径 `person != 会话人` → 403；放行后返回 cipher 布尔（`:11039-11045`）。审计实测：200 + `readable` 布尔，可传 `person=admin` 探测（`evidence-round7.txt:598-610`） | 可探测他人密级/加密配置 | ① 保留本人可探自己，admin 可探全部（已实现）；② cipher 仅 admin 可探 | 密级/加密信息是否只有 admin 可查？ |

---

## 4. 软门（2 条）：HTTP 语义

| # | 端点 | 当前行为（证据） | 风险 | 方案①（推荐）/ 方案② | 需业务回答 |
|---|---|---|---|---|---|
| S1 | `POST /api/empower/manager/create` | `200` + `ActionResult{success:false,error:"forbidden: admin required"}`（`empower/src/lib.rs:350-351`） | 只认 HTTP 2xx 的客户端/扫描器会把拒绝当成功；门禁若只看状态码会漏 | ① 改 `403`（语义正确，需排查只认 2xx 的客户端）；② 保持 `200 + body code`（兼容，但所有消费方必须读 body） | 是否有客户端只检查 HTTP 状态码、不读 body？ |
| S2 | `POST /api/empower/manager/update/{id}` | 同上（`empower/src/lib.rs:406-407`） | 同上 | 同上 | 同上 |

---

## 5. E13–E24：读越权分组

### 5.1 编号说明

台账现有 `E1–E13`，其后是 `my-1..my-3` 与「117 条读 IDOR 拆 E18–E24」。本清单为会议索引做一次归并：

- `E13`：匿名文档列表（1 条）。
- `E14–E16`：对应台账 `my-1..my-3`（不改变台账原行，仅本清单编号）。
- `E17`：无会话读 3 条（`work_processing` / `task_processing` / `work_retract`）。
- `E18–E24`：117 条按 crate 分 7 批。

> 117 条逐条清单在 `evidence-round6.txt §B`；本清单只给分组与代表端点，够开会拍板。

| 组 | 范围 | 条数 | 代表端点 | 当前行为（证据） | 风险 | 方案①（推荐）/ 方案② | 需业务回答 |
|---|---:|---|---|---|---|---|---|
| E13 | 匿名文档列表 | 1 | `PUT /api/anonymous/document/filter/list/1/size/10` | `HTTP`：`ids=[doc-draft-1]`，`status:"draft"`，body 为 secret（`report-2.md:170`） | 登录用户可读他人草稿 | ① 无会话只 `published`；登录 = `published OR creator/author_id=本人`；② 仅 creator/author（匿名不可读） | 匿名端是否必须能读 `published`？草稿 owner 是 `creator` 还是 `author_id`？ |
| E14 | `my-1` 草稿列表 | 1 族 | `POST .../draft/list/my/paging/1/size/1` | `HTTP`：返回 `audit-d2`（`xperson=admin`），`count=1`（库中 3 行） | `/my/` 语义丢失，管理员调用时返回他人数据 | ① 无条件按会话人过滤；② 管理员代查走独立管理端点 | 管理员是否需要 `/my/` 代查？（建议不需要） |
| E15 | `my-2` 工作列表 | 1 族 | `POST .../work/list/my/paging/1/size/10` | `HTTP`：返回 `work-admin-1`（`creator=person-admin`） | 同上 | 同上 | 同上 |
| E16 | `my-3` 任务列表 | 1 族（6 路由同构） | `GET .../task/list/my/paging/1/size/10` | `HTTP`：`dataLen=2 ids=[task-admin-2,task-admin-1]`（均 `person=person-admin`）；`taskcompleted/read/review/handover` 同构 | 同上 | 同上 | 同上 |
| E17 | 无会话读 | 3 | `/api/processplatform/.../work_processing:91`、`task_processing:125`、`work_retract:58` | `STATIC`：3 个 handler 不取 `Session`，匿名可读 | 匿名读流程数据 | ① 加 `Session` + 本人/发起人/admin；② 若产品需要公开分享，改成显式 allowlist token | 这些端点是否允许匿名访问？ |
| E18 | cms 读 | 18 | `/api/document/{id}/view`、`/api/file/{id}/content`、`/api/file/{id}/download`、`/api/document/{id}/persons`、`/api/comment/{id}` | `HTTP`（report-6 §3.2）：117 条 EXPLOIT 中含 cms 18；`anonymous_document_id_view:657` 对登录用户可读他人 `draft` | 文档正文/附件/PII 可读 | ① `creator/author OR admin`，`published` 例外；② 仅 owner/admin（`published` 也不公开） | 文档/附件公开范围：`published` 是否对全员？owner 用 `creator` 还是 `author_id`？ |
| E19 | 考勤读 | 15 | `/attendancedetail/{id}`、`/attendancedetail/mobile/{id}`、`/attendanceappealInfo/{id}`、`/v2/appeal/{id}`、`/v2/record/{id}` + 10 条配置 | `HTTP`：A→B `mobilepreview` 200 返回 `r6-ad-b\|audit-u2\|LATE`；`GET /mobile/{id}` 200 返回 B 的行（`report-6.md:255-257`） | 他人考勤明细/申诉/记录可读 | ① `person_id = 会话人 OR admin`；② 考勤配置读仅 admin | 考勤配置（规则/班次/组）普通用户是否需要读？ |
| E20 | 流程读 | 12 | `/task/{id}`、`/task/v2/{id}`、`/workcompleted/{id}`、`/review/{id}`、`/snap/{id}`、`/record/{id}` | `HTTP`（report-6 §3.2）：EXPLOIT 12 条 | 流程/办理内容泄露 | ① `assignee` / 发起人 / admin；② 同流程参与者可见 | 同流程协作者是否算合法读者？ |
| E21 | 查询读 | 14 | designer：`/get/{id}`、`/view/permission/{id}`、`/stat/simulate/{id}`；surface：`/execute/{view}/{id}`、`/excel/{view}/{id}` | `HTTP`（report-6 §3.2）：EXPLOIT 9 + 5 | 查询定义、权限串、执行结果泄露 | ① creator/admin；② 有查询执行权的人 | 查询定义是否按创建人私有？权限串是否允许普通用户看？ |
| E22 | program_center 读 | 8 | `/application/{id}`、`/module/output/{id}/file`、`/dict/{id}`、错误日志 | `HTTP`（report-6 §3.2）：EXPLOIT 8 | 应用数据/错误日志泄露 | ① 应用管理员；② creator/admin | 应用数据归应用管理员，还是创建人私有？ |
| E23 | portal 读 | 12 | designer：`/get/{id}`、`/portal/icon/{id}`、`/file/download/{id}`；surface：`/preview/{id}`、`/portal/icon/base64/{id}` | `HTTP`（report-6 §3.2）：EXPLOIT 7 + 5 | 门户配置/图标/文件泄露 | ① creator/admin；② `published` 门户公开 | 门户是否公开？未发布门户是否仅创建人可见？ |
| E24 | general + 其余 crate 读 | 38 | `general`：`/invoice/get/{id}`、`/securityclearance/{id}`、`/qrcode/{id}`；其余：org identity、meeting、mind、jpush device、calendar、message | `HTTP`（report-6 §3.2）：general 5 + 其余 26 crate 33 | 财务/身份 PII、消息/日历数据泄露 | ① 按域：财务=财务管理员、身份=本人/同单位、消息=收件人；② 统一 `owner OR admin` | 财务/身份/消息分别按什么业务关系开放？（建议按域裁决，不统一） |

---

## 6. admin_policy 点名（deny 15 / unknown 19）

> 来源：`oa4rust/backend/tests/security_regression_cases.tsv` 的 `admin_policy` 列（idor 共 145 条：allow 111 / deny 15 / unknown 19）。「当前状态」对 `security_regression_baseline.txt`（156 条）取差集：不在基线即 PASS。
>
> 本节是文首「默认裁决」的配套：默认说「读 = 本人 OR 管理员」，与 `deny` 的原始定义（admin 也不该看）**直接冲突**。冲突不取平均，二选一，见 6.1。

### 6.1 读侧 deny：8 条（默认裁决要求改标 `allow`，除非业务点名例外）

| 用例 | 端点 | 当前状态 |
|---|---|---|
| `calendar_core_entity\|calendar_get\|GET` | `/api/calendar/core/entity/calendar/{id}` | UNVERIFIED |
| `file_assemble_control\|get_file\|GET` | `/api/file/assemble/control/file/{id}` | LEAK |
| `file_assemble_control\|recycle_id\|GET` | `/api/recycle/{id}` | UNVERIFIED |
| `general_core_entity\|file_download\|GET` | `/api/general/file/download/{id}` | LEAK |
| `general_core_entity\|file_get\|GET` | `/api/general/file/{id}` | LEAK |
| `jpush\|device_get\|GET` | `/api/jpush/device/{id}` | LEAK |
| `jpush_core_entity\|device_get\|GET` | `/api/jpush/core/entity/device/{id}` | LEAK |
| `program_center\|collect_code_mobile_mobile\|GET` | `/api/program_center/collect/code/mobile/{id}` | LEAK |

裁决二选一：① 按默认，admin 是合法读者 → 这 8 条 `deny` 改 `allow`；② 保留 `deny` → 必须在例外清单里写明「即使 admin 也不得读」，并据此实现。

### 6.2 写侧 deny：7 条（6 条已修但实现为「本人或 admin」，与 deny 矛盾）

| 用例 | 当前状态 | 实现口径 |
|---|---|---|
| `file_core_entity\|folder_delete\|DELETE` | LEAK | 未修，仍可越权删他人文件夹；若 deny 成立，修时 admin 也要拒 |
| `general_core_entity\|file_delete\|POST` | PASS | 已修，`owner OR admin` |
| `general_core_entity\|file_update\|POST` | PASS | 已修，`owner OR admin` |
| `mind_core_entity\|update_folder\|POST` | PASS | 已修，`creator OR admin`（`mind_core_entity/src/lib.rs:35`） |
| `mind_core_entity\|update_mind\|POST` | PASS | 已修，同上 |
| `program_center\|collect_delete\|DELETE` | PASS | 已修，`person_id OR admin`（`program_center/src/lib.rs:9335`） |
| `program_center\|collect_save\|PUT` | PASS | 已修，同上 |

默认裁决**没有覆盖写侧**。要回答的只有一句：**admin 能否代写他人数据？** 答「不能」则上表 6 条已修用例的 admin 分支要改成 403，`admin_policy` 才有意义；答「能」则这 6 条改标 `allow`。

### 6.3 unknown：19 条（10 读 + 9 写，去重后 15 个端点）

| 域 | 读（10 条，全部 LEAK） | 写（9 条） |
|---|---|---|
| attendance | `attendanceappealInfo`、`attendancedetail`、`attendancedetail/mobile`、`attendanceselfholiday`、`v2/appeal`、`v2/record` | `appeal` POST＝LEAK；`record/update`＝PASS |
| general | `invoice/get`、`securityclearance`、`general_core invoice` | `invoice/delete`＝PASS、`invoice/update`＝PASS |
| processplatform | `attachment` | `attachment/edit/text`＝UNVERIFIED |
| `t1` 重复行（4） | — | `appeal` POST/PUT＝LEAK×2；`record/update`＝PASS；`invoice/update`＝PASS |

读数：**13 LEAK + 1 UNVERIFIED + 5 PASS**。这 19 条是「还没想清楚的有多少」的显式台账；不裁决就继续留在基线，`unknown` 不阻断门禁。

## 7. 建议的会议议程（约 45 分钟）

> 文首「默认裁决」已采用，议程只需处理：默认裁决末表 17 条 + `6.1`（读侧 deny 是否改 allow）+ `6.2`（admin 能否代写）。

1. **确认默认裁决**（5 分钟）：读=本人 OR 管理员、空归属仅管理员、cms `published` 全员、软门 403、5 条只补新增。
2. **真·全局表归谁**（15 分钟）：`2.3` 的 14 条，按「应用命名空间 / 全局配置 / 组织范围」三选一，逐条勾。
3. **关联边 + 写侧留痕**（5 分钟）：`2.1` #17 归端点实体还是平台 admin；#13、#31 代改是否留痕。
4. **admin_policy 冲突**（10 分钟）：`6.1` 读侧 deny 8 条改不改 `allow`；`6.2` admin 能否代写他人数据。
5. **`deny` 例外清单**（5 分钟）：非默认的「admin 也不得看/写」逐条点名，未点名的一律按 `allow`。
6. **填裁决记录表**（5 分钟）。

## 8. 裁决记录（会议当场填）

| 编号 | 裁决 | 拍板人 | 日期 | 落地动作 |
|---|---|---|---|---|
|  |  |  |  |  |

## 9. 证据来源与复跑

- 31 条静态扫描：`docs/audits/behavioral-audit-2026-11-05/evidence-round7.txt §B`（`NO-OWNER-COL count = 31`）。
- 31 条的两种归属定义与后果：`docs/audits/behavioral-audit-2026-11-05/report-7.md §2`。
- E13 实测：`docs/audits/behavioral-audit-2026-11-05/report-2.md:170`。
- my-1..3 实测：`docs/audits/behavioral-audit-2026-11-05/report-2.md:174-181`。
- 117 条按 crate：`docs/audits/behavioral-audit-2026-11-05/report-6.md:136-149`；逐条 `evidence-round6.txt §B`。
- 考勤双向实测：`docs/audits/behavioral-audit-2026-11-05/report-6.md:253-262`。
- 当前代码行号以 `origin/main`（合并 commit `5fbc15326`）为准。
- 复跑门禁：`cd oa4rust/backend && python scripts/security_regression_gate.py run`（本地 preflight 严格；CI 跳过 preflight）。
