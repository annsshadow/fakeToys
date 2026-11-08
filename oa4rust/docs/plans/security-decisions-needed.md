# 安全裁决清单（2026-11-06）

> 用途：把「已查清、但改代码前必须先定口径」的条目一次性摆到桌上。每条都能当场回答，不需要翻审计报告。
>
> 本文只做裁决，不改业务代码。默认原则：**没有裁决就不动**；必须动时按 fail-closed（拒绝）处理，历史空归属视为无权。

## 默认裁决（已采用，业务方可推翻）

> 2026-11-06 由业务方先行确定，作为实施默认值。**可推翻**：推翻时只改本节，下文章节保留原始证据与备选方案，不改判据。本节生效后，第 2–5 节降级为证据附录，真正待开会项见本节末表。

1. **读权限统一「本人 OR 管理员」**；历史行无归属值的，按「仅管理员可读」兜底（fail-closed）。
   - 对应：`2.1` 的 12 条、`2.2` 的 5 条（补列后生效）、`2.3` 中能落到主体列的条目、`5` 的全部 E13–E24。
   - 「管理员」口径（2026-11-06 已裁决）：**管理端读/写一律 `allow`**；`deny` 只保留两类——(a) 匿名/未登录也能触达的端点；(b) 确实不应由管理员操作、且写明业务理由的少数端点。复核后 145 条 idor 的分布为 `allow 144 / deny 1 / unknown 0`（见 §6）。
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
| `deny` 例外清单 | 已收敛为 1 条（§6.2 的 `folder_delete`） | 其余一律 `allow`；例外必须带业务理由 |

**合计仍待业务回答 17 条**（14 + 1 + 2）；`deny` 清单已收敛，不再挂账。

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

## 6. admin_policy 裁决（allow 144 / deny 1 / unknown 0）

> 来源：`oa4rust/backend/tests/security_regression_cases.tsv` 的 `admin_policy` 列（idor 145 条）。2026-11-06 裁决后的最终分布：**`allow 144 / deny 1 / unknown 0`**。
>
> 裁决口径：管理端读/写一律 `allow`；`deny` 只保留 (a) 匿名/未登录可触达、(b) 业务明确禁止 admin 操作的端点，且逐条带理由。本次只改标签，**不改代码**，所以 `security_regression_baseline.txt` 仍为 156 条，LEAK 状态一条未少。

### 6.1 读侧 deny 8 条：全部改标 `allow`

理由：8 条都是**登录后才可达**的 idor 读端点，管理员是合法读者；无匿名触达项，不满足保留 `deny` 的 (a)。改标签不改变实测状态——其中 2 条仍 UNVERIFIED、6 条仍 LEAK，继续留基线，修复线照常按基线棘轮推进。

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

裁决落地：8 条全部改标 `allow`（`calendar_get`、`get_file`、`recycle_id`、`file_download`、`file_get`、`jpush`/`jpush_core` `device_get`、`collect_code_mobile`）。

### 6.2 写侧 deny 7 条：6 条改标 `allow`，1 条保留 `deny`（批次 5 真修）

| 用例 | 当前状态 | 实现口径 |
|---|---|---|
| `file_core_entity\|folder_delete\|DELETE` | LEAK | 未修；保留 `deny`，批次 5 真修为 `owner OR admin` 后再评估 |
| `general_core_entity\|file_delete\|POST` | PASS | 已修，`owner OR admin` |
| `general_core_entity\|file_update\|POST` | PASS | 已修，`owner OR admin` |
| `mind_core_entity\|update_folder\|POST` | PASS | 已修，`creator OR admin`（`mind_core_entity/src/lib.rs:35`） |
| `mind_core_entity\|update_mind\|POST` | PASS | 已修，同上 |
| `program_center\|collect_delete\|DELETE` | PASS | 已修，`person_id OR admin`（`program_center/src/lib.rs:9335`） |
| `program_center\|collect_save\|PUT` | PASS | 已修，同上 |

裁决落地：**admin 可代写他人数据**，与读侧同一口径。上表 6 条 PASS 用例的代码已经是 `owner OR admin`（依据列已注明文件/行），标签改 `allow`。

`file_core_entity|folder_delete` **保留 `deny`**，理由：这是破坏性删除，管理员不应直连该路由代删他人文件夹，应走管理端点或二次确认；**该条仍是 LEAK**，标签保留不掩盖缺口，走写 IDOR 批次 5 真修（改成 `owner OR admin` 后再评估是否改标 `allow`）。

### 6.3 unknown 19 条：全部落到 `allow`（unknown=0）

| 域 | 读（10 条，全部 LEAK） | 写（9 条） |
|---|---|---|
| attendance | `attendanceappealInfo`、`attendancedetail`、`attendancedetail/mobile`、`attendanceselfholiday`、`v2/appeal`、`v2/record` | `appeal` POST＝LEAK；`record/update`＝PASS |
| general | `invoice/get`、`securityclearance`、`general_core invoice` | `invoice/delete`＝PASS、`invoice/update`＝PASS |
| processplatform | `attachment` | `attachment/edit/text`＝UNVERIFIED |
| `t1` 重复行（4） | — | `appeal` POST/PUT＝LEAK×2；`record/update`＝PASS；`invoice/update`＝PASS |

裁决落地：19 条全部改标 `allow`。依据：这 15 个端点都要求登录，管理员是合法读者/写者；无一满足 `deny` 的 (a) 匿名触达或 (b) 业务禁止。改标签不改变实测——**13 LEAK + 1 UNVERIFIED + 5 PASS 原样保留在基线**，`unknown` 计数从 19 归零。

### 6.4 改标涉及的 20 条 LEAK（标签不掩盖缺口）

门禁实跑（2026-11-06，rebase 到 `origin/main` 后）：`cases=192 pass=36 leak=133 unverified=23 integrity=0`，**非 PASS 156，与基线一致，无可减行**。改标的 33 条里仍有 20 条 LEAK：

- 读（16）：`file_assemble_control|get_file`、`general_core_entity|file_download`、`general_core_entity|file_get`、`jpush|device_get`、`jpush_core_entity|device_get`、`program_center|collect_code_mobile`；attendance 6 条（`attendanceappealInfo`、`attendancedetail`、`attendancedetail/mobile`、`attendanceselfholiday`、`v2/appeal`、`v2/record`）；general 3 条（`invoice_get_id`、`securityclearance_get`、`invoice_get`）；`processplatform|attachment_id`。
- 写（4）：`file_core_entity|folder_delete`；attendance `appeal` POST、`t1|appeal` POST/PUT。
- 另有 3 条 UNVERIFIED 同批保留：`calendar_get`、`recycle_id`、`attachment/edit/text`。

这些全部留在基线，等对应修复批次按 `STALE-BASELINE` 点名后才减。

## 7. 建议的会议议程（约 45 分钟）

> 默认裁决与 `admin_policy` 均已定；议程只需处理默认裁决末表 17 条。

1. **确认默认裁决**（5 分钟）：读=本人 OR 管理员、空归属仅管理员、cms `published` 全员、软门 403、5 条只补新增。
2. **真·全局表归谁**（15 分钟）：`2.3` 的 14 条，按「应用命名空间 / 全局配置 / 组织范围」三选一，逐条勾。
3. **关联边 + 写侧留痕**（5 分钟）：`2.1` #17 归端点实体还是平台 admin；#13、#31 代改是否留痕。
4. **`admin_policy` 已定**（5 分钟）：确认分布 `allow 144 / deny 1 / unknown 0`，只核对 §6.2 的 `folder_delete` 例外理由。
5. **`deny` 例外**（5 分钟）：仅 `folder_delete` 1 条，处理路径是批次 5 真修，不是改标签。
6. **填裁决记录表**（5 分钟）。

## 8. 裁决记录（会议当场填）

| 编号 | 裁决 | 拍板人 | 日期 | 落地动作 |
|---|---|---|---|---|
|  |  |  |  |  |

## 9. 移交清单：改标后剩余 23 条（20 LEAK + 3 UNVERIFIED）

> 依据 2026-11-06 实跑（rebase `origin/main` 后）：`leak=133 unverified=23`。本清单只取「本次改标涉及的」23 条，其余 LEAK 不在改标范围内。
> 失败分类：**真漏**＝表里有归属列但查询/更新没按归属过滤；**缺归属列**＝表无归属列（本清单为 0 条）；**关联边**＝须经实体所有者间接判定（本清单为 0 条）；**证据漂移**＝种子缺行导致读不到 canary，不是代码缺陷。

### 9.1 读 IDOR 分片 A（8 条，全 `真漏`）

| # | 端点 | crate | 表 | 归属列 | 实测证据 |
|---|---|---|---|---|---|
| A1 | `GET /api/file/assemble/control/file/{id}` | `file_assemble_control` | `x_file` | `creator` / `creator_person` | 2xx 含 canary `r6-get_file`（rev `rev-r6-get_file`） |
| A2 | `GET /api/general/file/download/{id}` | `general_core_entity` | `x_general_file` | `creator` | 2xx 含 canary `r6-filedownload`（rev 同） |
| A3 | `GET /api/general/file/{id}` | `general_core_entity` | `x_general_file` | `creator` | 2xx 含 canary `r6-filedownload` |
| A4 | `GET /api/jpush/device/{id}` | `jpush` | `x_jpush_device` | `user_id` | 2xx 含 canary `r6-deviceget` |
| A5 | `GET /api/jpush/core/entity/device/{id}` | `jpush_core_entity` | `x_jpush_device` | `user_id` | 2xx 含 canary `r6-deviceget` |
| A6 | `GET /api/program_center/collect/code/mobile/{id}` | `program_center` | `x_program_collect` | `person_id` | 2xx 含 canary `r6-collect_delete` |
| A7 | `GET /api/general/assemble/control/invoice/get/{id}` | `general_assemble_control` | `x_general_assemble_invoice` | `creator` | 2xx 含 canary `r6-invoicegetid` |
| A8 | `GET /api/general/assemble/control/securityclearance/{id}` | `general_assemble_control` | `x_general_assemble_security_clearance` | `creator` | 2xx 含 canary `r6-securityclearanceget` |

### 9.2 读 IDOR 分片 B（8 条，全 `真漏`）

| # | 端点 | crate | 表 | 归属列 | 实测证据 |
|---|---|---|---|---|---|
| B1 | `GET /api/general/invoice/{id}` | `general_core_entity` | `x_general_invoice` | `creator` | 2xx 含 canary `r6-invoiceget` |
| B2 | `GET /api/processplatform/service/processing/attachment/{id}` | `processplatform_service_processing` | `x_attachment` | `creator` | 2xx 含 canary `r6-attachment_edit_id_t` |
| B3 | `GET /api/attendance/assemble/control/attendanceappealInfo/{id}` | `attendance_assemble_control` | `x_attendance_appeal_info` | `person_id` | 2xx 含 canary `appeal-admin`（rev `rev-appeal-admin`） |
| B4 | `GET /api/attendance/assemble/control/attendancedetail/{id}` | `attendance_assemble_control` | `x_attendance_detail` | `person_id` | 2xx 含 canary `r5-ad-admin`（rev `rev-r5-ad-admin`） |
| B5 | `GET /api/attendance/assemble/control/attendancedetail/mobile/{id}` | `attendance_assemble_control` | `x_attendance_detail` | `person_id` | 2xx 含 canary `r5-ad-admin` |
| B6 | `GET /api/attendance/assemble/control/attendanceselfholiday/{id}` | `attendance_assemble_control` | `x_attendance_selfholiday` | `person_id` | 2xx 含 canary `hol-admin` |
| B7 | `GET /api/attendance/assemble/control/v2/appeal/{id}` | `attendance_assemble_control` | `x_attendance_v2_appeal_info` | `user_id` | 2xx 含 canary `r6-v2_appeal_get` |
| B8 | `GET /api/attendance/assemble/control/v2/record/{id}` | `attendance_assemble_control` | `x_attendance_record` | `user_id` | 2xx 含 canary `r6-record_update` |

### 9.3 写 IDOR（4 条，全 `真漏`）

| # | 端点 | crate | 表 | 归属列 | 实测证据 |
|---|---|---|---|---|---|
| W1 | `DELETE /api/file/core/entity/folder/{id}` | `file_core_entity` | `file_folder` | `person` | 2xx 且受害者行被删（保留 `deny`，修成 `owner OR admin`） |
| W2 | `POST /api/attendance/assemble/control/attendanceappealInfo/appeal/{id}` | `attendance_assemble_control` | `x_attendance_appeal_info` | `person_id` | 2xx 且受害者行被改 |
| W3 | 同 W2（`t1` 重复用例） | 同 | 同 | 同 | 同 |
| W4 | `PUT /api/attendance/assemble/control/attendanceappealInfo/appeal/{id}`（`t1`） | 同 | 同 | 同 | 2xx 且受害者行被改 |

### 9.4 不计入派活：2 条 `证据漂移` + 1 条写谓词缺失

| # | 端点 | 表 | 证据 | 处理 |
|---|---|---|---|---|
| D1 | `GET /api/calendar/core/entity/calendar/{id}` | `x_cal_calendar` | 2xx `{"type":"error","message":"calendar not found"}`，无 canary | 夹具缺行：补种子后重测，不改代码 |
| D2 | `GET /api/recycle/{id}` | `file_file` | 2xx `"file not in recycle bin"` | 夹具缺「回收站标记」行，同上 |
| D3 | `POST /api/processplatform/service/processing/attachment/edit/text/{id}` | `x_attachment` | 2xx 但行未变（body `data.creator` 仍是 victim） | **真漏**：更新谓词没按 id/归属落库，随写 IDOR 一并修 |

派活口径：分片 A → 读 IDOR 会话 A；分片 B → 读 IDOR 会话 B；9.3 → 写 IDOR 会话；9.4 的 D1/D2 → 夹具会话（补种子，不是代码缺陷），D3 → 写 IDOR。

## 10. 证据来源与复跑

- 31 条静态扫描：`docs/audits/behavioral-audit-2026-11-05/evidence-round7.txt §B`（`NO-OWNER-COL count = 31`）。
- 31 条的两种归属定义与后果：`docs/audits/behavioral-audit-2026-11-05/report-7.md §2`。
- E13 实测：`docs/audits/behavioral-audit-2026-11-05/report-2.md:170`。
- my-1..3 实测：`docs/audits/behavioral-audit-2026-11-05/report-2.md:174-181`。
- 117 条按 crate：`docs/audits/behavioral-audit-2026-11-05/report-6.md:136-149`；逐条 `evidence-round6.txt §B`。
- 考勤双向实测：`docs/audits/behavioral-audit-2026-11-05/report-6.md:253-262`。
- 当前代码行号以 `origin/main`（合并 commit `5fbc15326`）为准。
- 复跑门禁：`cd oa4rust/backend && python scripts/security_regression_gate.py run`（本地 preflight 严格；CI 跳过 preflight）。

## 11. 扫描规则教训（写规则前先读）

本轮 limit-scan 误报 #201，根因不只是行号漂移，而是**判定单位与风险单位不一致**：

- 风险单位是「端点 × 限额键」，扫描却按「SQL 字面量 + 行号」计数，插一行代码整体漂移；
- 更本质的一层：归属谓词被抽成 `const CREATOR_OWNER_PREDICATE` 后，SQL 里只剩
  `WHERE {CREATOR_OWNER_PREDICATE}`（或 `WHERE {}` + `u2_helpers::CREATOR_OWNER_PREDICATE`），
  扫描器看不到 `id = $1`，5 条单行查询被误判为 `unbounded`：
  `message|mass_id`、`organization_authentication|identity_id`、
  `organization_control|identity_id / identity_flag / unit_flag`。

写静态扫描规则的硬要求：

1. 判定键必须稳定：`端点标识（fn/route）+ 风险键（限额名/bucket）`；行号只作定位提示。
2. 允许代码把 SQL 抽成常量或 `format!` 占位符：规则必须解析 `{NAME}` / `{}` 的实参，
   把谓词内联后再分类；解析不了时应报 `UNVERIFIED`，**不能默认成最坏桶**。
3. 「新增/未登记」必须基于键集合差，不能把整个文件的行号列出来当证据。
4. 规则要配 sabotage：新增一个真·无 LIMIT 端点必须判红，否则改键之后灵敏度可能已归零。

实现位置：`oa4rust/backend/scripts/limit_scan.py`（端点级计数 + 谓词常量内联）；
实测记录见 PR #206。

## 12. 全量核对：每个已合并 PR 的改动是否真的进了 `origin/main`（2026-11-06）

### 12.1 为什么做

#208 是已发生的实例：它合并时 base 指向 `sec/read-b2` 而不是 `main`，b3 的内容只落在 b2 分支，
`main` 首父链里根本没有 `#208` 的合并提交。最后由 #219 重新补合才进 `main`。
因此「PR 被合并过」「`STALE-BASELINE` 消失了」都不能证明改动真的在 `main`。

### 12.2 方法（三重证据，逐 PR）

1. **声称改动**：对每个 PR 取分支自身 diff（合并提交用三点 `c^1...c^2`，直推提交用 `c^1..c`），
   得到文件清单与新增行；**不以合并结果（`c^1..c`）为准**，否则合并时被丢弃的内容无法暴露。
2. **实际存在**：文件必须存在于 `origin/main` 对应路径；新增行逐条与 `origin/main:path` 比对。
   逐字漂移（被后续 PR 重写）再人工确认语义等价——测试辅助函数是否存在、用例 id 是否存在、
   调度分支是否仍在、夹具字段是否只是扩列。
3. **基线不虚减**：对每个 PR 声称从 `security_regression_baseline.txt` 删除的行，
   在 `main` 上重跑门禁，该用例当前必须是 `[PASS]`。若仍是 `LEAK`/`UNVERIFIED`，即基线虚减。
4. **门禁接线**：对减基线的批次跑对应 sabotage（摘归属谓词），必须变红并逐条点名。

最终 `main` 实测：`cases=242 pass=114 leak=111 unverified=17 integrity=0`，
非 PASS 128 = 基线 128。

### 12.3 声称 vs 实际（窗口 #196–#222，`#208` 单列）

| PR | `main` 内提交 | 声称文件 | 声称新增代码行 | 缺失文件 | 未逐字命中行 | 实际状态 |
|---|---|---:|---:|---:|---:|---|
| #196 | `dc5ee5fc` | 6 | 392 | 0 | 3 | 被后续批次覆盖的 sabotage 调度串与旧跟踪表文段；`program_center` 分支仍在（`gate.py:694`） |
| #197 | `5fbc1532` | 10 | 2618 | 0 | 58 | 187/187 新增用例 id 全在 `cases.tsv`；其余为被 #202/#210 重写的测试辅助函数/调度串，等价形态存在 |
| #200 | `bf0ab7b8` | 2 | 245 | 0 | 0 | 全部逐字在 `main` |
| #201 | `1fa98c7b` | 9 | 138 | 0 | 1 | 调度串被后续并集覆盖（b5/b6/b7 均在） |
| #202 | `cf8c3905` | 12 | 849 | 0 | 26 | 全部为 read-a 裁决文段；#217 已改写为「第五批按两源并集修复」，语义保留 |
| #203 | `b2c341e4` | 5 | 423 | 0 | 2 | 调度串被 #205 的 b6 并集覆盖；b5 分支存在（`gate.py:722`） |
| #204 | `ef2cf972` | 2 | 227 | 0 | 0 | 全部逐字在 `main` |
| #205 | `ccbff183` | 5 | 381 | 0 | 0 | 全部逐字在 `main` |
| #206 | `92625f81` | 2 | 172 | 0 | 0 | 全部逐字在 `main` |
| #207 | `3f5d19eb` | 11 | 243 | 0 | 9 | 6 条文档被 #219 扩为「批次 2/3/4」重写；3 条夹具行已扩列（`gate-person-a` 等仍在） |
| **#208** | **无** | — | — | — | — | **真实缺失**：base=`sec/read-b2`，内容只在 b2 分支；已由 #219 补合 |
| #209 | `a02c49c6` | 13 | 627 | 0 | 0 | 全部逐字在 `main` |
| #210 | `5eac2c4f` | 1 | 38 | 0 | 2 | 调度串并集（b5/b6/b7 均在） |
| #211 | `2dca955d` | 39 | 132 | 0 | 0 | 全部逐字在 `main` |
| #212 | `4323c60c` | 1 | 0 | 0 | 0 | 2 行文档（推送纪律） |
| #213 | `ca925e0e` | 4 | 275 | 0 | 0 | 全部逐字在 `main` |
| #214 | `619a4f28` | 8 | 238 | 0 | 0 | 全部逐字在 `main` |
| #215 | `cc742985` | 3 | 283 | 0 | 0 | 全部逐字在 `main` |
| #216 | `e80d75b3` | 6 | 109 | 0 | 0 | 原 base=`sec/read-b3`，改 base 前已验证；全部逐字在 `main` |
| #217 | `cb8cc367` | 3 | 273 | 0 | 0 | 全部逐字在 `main` |
| #218 | `4019eaea` | 2 | 13 | 0 | 0 | 全部逐字在 `main` |
| #219 | `750418ec` | 5 | 186 | 0 | 6 | 1 条为 `fn owner_scope` 后续改成 `pub(crate) fn`；5 条文档被 #220/#221 扩写；语义保留 |
| #220 | `c751ccc7` | 6 | 100 | 0 | 0 | 全部逐字在 `main` |
| #221 | `2fb7a3fe` | 4 | 79 | 0 | 0 | 全部逐字在 `main` |
| #222 | `3e0847fc` | 1 | 42 | 0 | 0 | 全部逐字在 `main` |

「未逐字命中行」不等于缺失：上表每一处都已人工确认语义存在（用例 id、函数、调度分支、夹具字段、
裁决结论），没有一处的功能内容在 `main` 中消失。

### 12.4 重点六项（堆叠分支 / base 非 main / 合并后又 rebase）

| PR | 核对的实质内容 | 结果 |
|---|---|---|
| #213 | `mind_assemble_control/src/u2.rs` 165 行归属谓词改动 + 新增 `security_gate_tests.rs` | 275 行新增全部逐字在 `main` |
| #216 | `organization_assemble_control` 归属收紧 + 用例 + 夹具 | 改 base 后 109 行全部在 `main`；未减基线 |
| #217 | `meeting_assemble_control` 两源并集谓词 + 用例 | 273 行全部在 `main`；未减基线 |
| #209 | 5 个 crate 读归属谓词 + b7 用例 + 删 5 行基线 | 627 行全部在 `main`；5 条删除用例当前全 `[PASS]`；`sabotage write-idor-b7` 变红点名 5 条 |
| #203 | `folder_delete` + cms 三个 POST 读端点 + 删 4 行基线 | 423 行全部在 `main`（2 条调度串被 b6 并集覆盖）；4 条删除用例全 `[PASS]`；`sabotage write-idor-b5` 变红点名 4 条 |
| #205 | `appeal` + processplatform 两把 + 删 5 行基线 | 381 行全部在 `main`；5 条删除用例全 `[PASS]`；`sabotage write-idor-b6` 变红点名 5 条 |

### 12.5 基线虚减核对

窗口内 8 个 PR 共声称删除基线行 **32 条**（#196 4、#201 3、#202 5、#203 4、#205 5、
#207 1、#209 5、#215 5）。在 `main`（`a02c49c6`）重跑门禁，**32/32 当前均为 `[PASS]`**，
没有一条仍处于 `LEAK`/`UNVERIFIED`。

结论：**本轮基线 128 不是虚减**，减行数与实际修复一致（156 − 28 = 128；#196 的 4 行属于窗口前）。
唯一真实缺失是 #208，已由 #219 补合，且补合未被计入任何额外减行。

### 12.6 做成常规动作（合入减基线 PR 后必做）

不再以 `STALE-BASELINE` 消失作为「已修复」的证据，改为固定三步：

1. **行是否真在 main**：`git diff <merge>^1...<merge>^2 --stat` 的文件清单逐一对
   `git show origin/main:<path>` 比对新增行；被后续 PR 重写的，确认语义等价形态存在。
2. **基线是否虚减**：该 PR 删除的每个基线 id，在 `main` 上跑门禁必须是 `[PASS]`；
   任何一条仍为 `LEAK`/`UNVERIFIED` 即回滚该减行。
3. **门禁是否接线**：跑该批次对应 sabotage，必须变红并逐条点名；
   若脚本调度串被并集覆盖导致分支缺失，等同减行无效。

三项全过才算这笔 PR「进了 main」。

## 13. 剩余 111 条 LEAK 分批清单（按 crate 聚类，可派三条线）

### 13.1 分级口径与「是否定死」的判定

判定只用两条硬标准：

1. **用例能构造 victim canary 行** → 目标表必须存在可写的归属列（或可从父行 join 得到归属）；
2. **归属谓词在裁决文档里有出处**：`§9.1/§9.2/§9.3`、`§2.1`、`§3 cms`、`§4 meeting invite` 属「显式」；
   `默认裁决 1（读 = 本人 OR 管理员；NULL fail-closed → 仅管理员）` 且分组 `E13–E24` 明确覆盖的属「默认（已采用）」。
   「显式」与「默认」都算**已定死、可动手**；只有「表无主体列且默认值覆盖不到」才算未定。

**结论：111 条全部已定死，未定 0 条。** 这不是宽松口径，而是结构性事实：门禁的 IDOR 用例必须能写入
victim 归属值，表没有主体列就**无法成为 canary 用例**，所以裁决文档里仍待业务回答的 17 条
（14 张真·全局表 + 1 条关联边 + 2 条写追溯）**本来就不在这 111 条里**。它们没有混进来，
也就不需要在本清单里「只登记」。逐条出处见各批次「口径」列。

### 13.2 三条线的 crate 划界（整 crate 归属，禁止跨线并发）

原 A（mind/meeting）、B（organization/message/calendar）的读面已清空，故按**整 crate** 重新划界，
保证同一 crate 不会同时出现在两条线上：

| 线 | 独占 crate | 条数 |
|---|---|---|
| 分片 A | `cms_*`、`portal_*`、`ai_assemble_control` | 30 |
| 分片 B | `attendance_*`、`processplatform_*` | 36 |
| 写 IDOR | `general*`、`file*`、`bbs`、`mind_core`、`program_center`、`query*` | 45 |

A/B/W 之间 crate 集合两两不相交；同一批内可以跨多个 crate，但那些 crate 都只归本线。

### 13.3 分片 A（30 条，5 批）

**A-b1（6）** 口径：`cms §3`（published 公开；draft 本人+管理员；owner=creator OR author_id）

| 用例 | 表 |
|---|---|
| `cms_assemble_control|anonymous_document_id_view|GET` | `x_cms_data_document` |
| `cms_assemble_control|document_persons_u3|GET` | `x_cms_data_document` |
| `cms_assemble_control|file_id|GET` | `x_cms_file` |
| `cms_assemble_control|file_id_content|GET` | `x_cms_file` |
| `cms_assemble_control|file_id_download|GET` | `x_cms_file` |
| `cms_assemble_control|comment_id|GET` | `x_cms_comment` |

**A-b2（6）** 口径：`cms §3`

| 用例 | 表 |
|---|---|
| `cms_assemble_control|anonymous_form_id|GET` | `x_cms_form` |
| `cms_assemble_control|form_id|GET` | `x_cms_form` |
| `cms_assemble_control|formversion_id|GET` | `x_cms_form_v2` |
| `cms_assemble_control|script_id|GET` | `x_cms_script` |
| `cms_assemble_control|scriptversion_id|GET` | `x_cms_form_v2` |
| `cms_assemble_control|templateform_id|GET` | `x_cms_form_v2` |

**A-b3（6）** 口径：`cms §3`（appinfo/categoryinfo/commend 无 published 概念 → creator OR admin）

| 用例 | 表 |
|---|---|
| `cms_assemble_control|appinfo_id|GET` | `x_cms_appinfo` |
| `cms_assemble_control|appinfo_id_control|GET` | `x_cms_appinfo` |
| `cms_assemble_control|application_id|GET` | `x_cms_appinfo` |
| `cms_assemble_control|categoryinfo_id|GET` | `x_cms_categoryinfo` |
| `cms_assemble_control|categoryinfo_id_control|GET` | `x_cms_categoryinfo` |
| `cms_assemble_control|commend_id|GET` | `x_cms_commend` |

**A-b4（6）** 口径：前 2 条 `cms §3`；后 4 条 `默认1 / E23`

| 用例 | 表 |
|---|---|
| `cms_core_entity|article_get|GET` | `x_cms_article` |
| `cms_core_express|content_detail|GET` | `x_cms_content` |
| `portal|portal_id|GET` | `x_portal` |
| `portal_assemble_designer|portal_id_icon|GET` | `x_portal` |
| `portal_assemble_designer|portal_id_permission|GET` | `x_portal` |
| `portal_assemble_surface|get_surface|GET` | `x_portal_surface` |

**A-b5（6）** 口径：前 5 条 `默认1 / E23`；`config_get_mcp_flag` 为 `默认1 / E24`（`x_ai_mcp_config`
不在 14 张无主体列表内，有 `creator` 列）

| 用例 | 表 |
|---|---|
| `portal_assemble_surface|portal_id_icon|GET` | `x_portal` |
| `portal_assemble_surface|portal_id_icon_base64|GET` | `x_portal` |
| `portal_assemble_surface|preview_surface|GET` | `x_portal_surface` |
| `portal_assemble_surface|surface_preview|GET` | `x_portal_surface` |
| `portal_core_entity|page_get|GET` | `x_portal_page` |
| `ai_assemble_control|config_get_mcp_flag|GET` | `x_ai_mcp_config` |

### 13.4 分片 B（36 条，6 批）

**B-b1（6）** 口径：`默认1 / E19`（配置表 = creator OR admin）

| 用例 | 表 |
|---|---|
| `attendance_assemble_control|attendanceadmin_id|GET` | `x_attendance_admin` |
| `attendance_assemble_control|attendanceemployeeconfig_id|GET` | `x_attendance_employee_config` |
| `attendance_assemble_control|attendanceimportfileinfo_id|GET` | `x_attendance_import_file_info` |
| `attendance_assemble_control|attendanceschedulesetting_id|GET` | `x_attendance_schedule_setting` |
| `attendance_assemble_control|attendancesetting_id|GET` | `x_attendance_setting` |
| `attendance_assemble_control|attendancestatisticalcycle_id|GET` | `x_attendance_statistical_cycle` |

**B-b2（6）** 口径：`默认1 / E19`；`v2_record_get`、`v2_appeal_get`、`attendancedetail_mobile_my`
按 `person_id` 本人过滤

| 用例 | 表 |
|---|---|
| `attendance_assemble_control|attendancestatisticrequirelog_id|GET` | `x_attendance_statistic_require_log` |
| `attendance_assemble_control|attendanceworkdayconfig_id|GET` | `x_attendance_workday_config` |
| `attendance_assemble_control|workplace_id|GET` | `x_attendance_workplace` |
| `attendance_assemble_control|v2_appeal_get|GET` | `x_attendance_v2_appeal_info` |
| `attendance_assemble_control|v2_record_get|GET` | `x_attendance_record` |
| `t1|attendancedetail_mobile_my` | `x_attendance_detail` |

**B-b3（4）** 口径：`§9.2 B3–B8 / E19`（本人 OR admin；历史 NULL → 仅 admin，fail-closed）

| 用例 | 表 |
|---|---|
| `attendance_assemble_control|attendanceappealInfo_id|GET` | `x_attendance_appeal_info` |
| `attendance_assemble_control|attendancedetail_id|GET` | `x_attendance_detail` |
| `attendance_assemble_control|attendancedetail_mobile_id|GET` | `x_attendance_detail` |
| `attendance_assemble_control|attendanceselfholiday_id|GET` | `x_attendance_selfholiday` |

**B-b4（6）** 口径：`默认1 / E20`（assignee/发起人 OR admin）

| 用例 | 表 |
|---|---|
| `processplatform_service_processing|task_id|GET` | `x_task` |
| `processplatform_service_processing|task_v2_id|GET` | `x_task` |
| `processplatform_service_processing|get_process|GET` | `x_work` |
| `processplatform_service_processing|data_workcompleted_id|GET` | `x_workcompleted` |
| `processplatform_service_processing|data_workcompleted_id_path|GET` | `x_workcompleted` |
| `processplatform_service_processing|taskcompleted_id|GET` | `x_workcompleted` |

**B-b5（6）** 口径：`E20`；`attachment_id` 另有 `§9.2 B2`（显式）；`snap_id` 按 `x_snap.creator/person`
直接归属（E20 已明确列 `/snap/{id}` = ① assignee/发起人/admin）

| 用例 | 表 |
|---|---|
| `processplatform_service_processing|readcompleted_id|GET` | `x_readcompleted` |
| `processplatform_service_processing|record_id|GET` | `x_record` |
| `processplatform_service_processing|review_id|GET` | `x_review` |
| `processplatform_service_processing|snap_id|GET` | `x_snap`（间接） |
| `processplatform_service_processing|attachment_id|GET` | `x_attachment` |
| `processplatform_service_processing|applicationdict_id|GET` | `x_process_definition` |

**B-b6（8）** 口径：`默认1 / E20`

| 用例 | 表 |
|---|---|
| `processplatform_core_express|task_processing|GET` | `x_task` |
| `processplatform_core_express|work_processing|GET` | `x_work` |
| `processplatform_core_express|work_retract|GET` | `x_work` |
| `processplatform_core_entity|work_get|GET` | `x_process_work` |
| `processplatform_assemble_surface|get_surface|GET` | `x_process_surface` |
| `processplatform_assemble_surface|preview_surface|GET` | `x_process_surface` |
| `processplatform_assemble_designer|get_flow|GET` | `x_process_definition` |
| `processplatform_assemble_designer|preview_flow|GET` | `x_process_definition` |

### 13.5 写 IDOR（45 条，7 批）

**W-b1（5）** 口径：前 3 条 `默认1 / E24`；`invoice_get_id` = `§9.1 A7`；`securityclearance_get` = `§9.1 A8`

| 用例 | 表 |
|---|---|
| `general_assemble_control|area_get|GET` | `x_general_assemble_area` |
| `general_assemble_control|attendscope_get|GET` | `x_general_attend_scope` |
| `general_assemble_control|invoice_get_id|GET` | `x_general_assemble_invoice` |
| `general_assemble_control|qrcode_get|GET` | `x_general_assemble_qrcode` |
| `general_assemble_control|securityclearance_get|GET` | `x_general_assemble_security_clearance` |

**W-b2（6）** 口径：前 3 条读 `默认1 / E24`；后 3 条写 `写=会话身份`（管理端可写他人，`§6.2`）

| 用例 | 表 |
|---|---|
| `general_core_entity|file_download|GET` | `x_general_file` |
| `general_core_entity|file_get|GET` | `x_general_file` |
| `general_core_entity|invoice_get|GET` | `x_general_invoice` |
| `t2|general_core|file_create` | `x_general_file` |
| `t2|general_core|invoice_create` | `x_general_invoice` |
| `t2|general_assemble|invoice_create` | `x_general_assemble_invoice` |

**W-b3（7）** 口径：`写=会话身份`（先例 `§9.3 W1` 把硬编码 system 改为会话人）
注意 `mind_core` 的 `creator_unit` 目前仍取自请求体（见 §14），修复时必须一并收口。

| 用例 | 表 |
|---|---|
| `t2|general|office` | `x_general_assemble_office` |
| `t2|general|office_html_to_word` | `x_general_assemble_office` |
| `t1|bbs|create_reply` | `x_bbs_reply` |
| `t1|bbs|create_subject` | `bbs_subject_info` |
| `t1|mind_core|create_mind` | `x_mind_base_info` |
| `t1|mind_core|create_folder` | `x_mind_folder_info` |
| `t2|mind_core|create_version` | `x_mind_version_info` |

**W-b4（8）** 口径：`默认1 / E22`（creator OR admin）

| 用例 | 表 |
|---|---|
| `program_center|application_get_by_id|GET` | `x_applications` |
| `program_center|collect_code_mobile_mobile|GET` | `x_program_collect` |
| `program_center|deploy_id|GET` | `x_program_deploy` |
| `program_center|dict_id|GET` | `x_program_dict` |
| `program_center|module_id_compare|GET` | `x_program_module` |
| `program_center|module_output_flag_file|GET` | `x_program_module` |
| `program_center|prompterrorlog_id|GET` | `x_program_prompt_error_log` |
| `program_center|unexpectederrorlog_id|GET` | `x_program_unexpected_error_log` |

**W-b5（6）** 口径：`warnlog_create` 写 `写=会话身份`；其余 `默认1 / E21`

| 用例 | 表 |
|---|---|
| `t2|program_center|warnlog_create` | `x_program_warn_log` |
| `query_assemble_designer|get_designer|GET` | `x_query_design` |
| `query_assemble_designer|query_id_permission|GET` | `x_query_design` |
| `query_assemble_designer|stat_id|GET` | `x_query_stat` |
| `query_assemble_designer|stat_id_permission|GET` | `x_query_stat` |
| `query_assemble_designer|stat_id_simulate|GET` | `x_query_stat` |

**W-b6（8）** 口径：`默认1 / E21`

| 用例 | 表 |
|---|---|
| `query_assemble_designer|view_id_bundle|GET` | `x_query_view` |
| `query_assemble_designer|view_id_permission|GET` | `x_query_view` |
| `query_assemble_designer|view_id_simulate|GET` | `x_query_view` |
| `query_assemble_surface|get_surface|GET` | `x_query_surface` |
| `query_assemble_surface|view_id_bundle|GET` | `x_query_view` |
| `query_assemble_surface|view_id_bundle_v2|GET` | `x_query_view` |
| `query_assemble_surface|view_id_excel|GET` | `x_query_view` |
| `query_assemble_surface|view_id_execute|GET` | `x_query_view` |

**W-b7（5）** 口径：`table_id_permission`、`view_get` 读 `默认1 / E21`；`view_create` 写 `写=会话身份`；
`get_file` = `§9.1 A1`（显式）；`folder_create` = `§9.3 W1`（显式）

| 用例 | 表 |
|---|---|
| `query_assemble_designer|table_id_permission|GET` | `x_query_table` |
| `query_core_entity|view_get|GET` | `query_view` |
| `t2|query|view_create` | `query_view` |
| `t2|file|folder_create` | `file_folder` |
| `file_assemble_control|get_file|GET` | `x_file` |

### 13.6 每批的准入与减行条件

1. 合入前该批必须有对应 sabotage 模式；**没有 sabotage 的批次不得删基线行**（否则第 3 步无法执行）。
2. 按 `§12.6` 三步核对：行在 `main` → 被删行全 `[PASS]` → sabotage 变红并逐条点名。
3. 一批只减该批实际转 `[PASS]` 的行；出现 `STALE-BASELINE` 只减被点名的行，不做连带清理。
4. **并行用例共用同一夹具行会互相删行**：`cargo test` 默认多线程，同文件两条用例若按宽口径
   （如按 `creator`）清理同一张表的夹具，A 的清理会删掉 B 正在用的行，表现为**偶发 403/404/UNVERIFIED**。
   排查顺序是先做用例隔离（单线程跑、或让用例独占行 id），**不要先怀疑归属谓词**；复现可用临时
   `tokio::time::sleep(2s)` 放大竞态，拿到证据后把睡眠移除。#253 的 W2 假红即用此法定位
   （根因是两用例按 creator 宽口径互删，改为精确 id 清理后 25 用例 ×30 轮无失败）。

### 13.7 写 IDOR 批次执行记录（§12.6 三步核对）

七批均从 `origin/main` 单独开分支、`base=main`、单批单 PR，无堆叠。

| 批次 | PR | merge commit | 减行 | sabotage | 三步核对 |
|---|---|---|---|---|---|
| W-b1 | #228 | `12179d371` | 5 | `write-idor-w1` | 通过 |
| W-b2 | #229 | `fe4a74aac` | 6 | `write-idor-w2` | 通过 |
| W-b3 | #231 | `621d0e02a` | 7 | `write-idor-w3` | 通过 |
| W-b4 | #235 | `40cd8eab7` | 8 | `write-idor-w4` | 通过 |
| W-b5 | #239 | `577305025` | 5 | `write-idor-w5` | 通过 |
| W-b6 | #240 | `255f5064b` | 8 | `write-idor-w6` | 通过 |
| W-b7 | #241 | `adc14abc0` | 5 | `write-idor-w7` | 通过 |

每批三步均为：`git diff <merge>^1 <merge>^2 --stat` 与 `<merge>^1 <merge>` 一致；
被删行在 `main` 上实测 `[PASS]`；对应 sabotage 变红并逐条点名。七批合计减 44 行。

**FALSE-GREEN（已收口）**：`t2|program_center|warnlog_create` 的门禁用例 `field=level`
不是归属列；写侧实际落 `creator_person = sess.person_unique`。2026-11-08「假绿根修批」已把
`field` 改为 `creator_person`，行由真实 `POST /api/program_center/warnlog` 产出并回表核对，
基线按 `STALE-BASELINE` 减行；证据与剩余假绿项见 `f2e-endpoint-tracker.md`「假绿根修批」。

### 13.8 W-b8：5 条未认领 UNVERIFIED 收口（file / bbs / mind）

从 `origin/main` 单开 `fix/w8-unverified-owner-gates`，`base=main`，单批单 PR；
sabotage `w8-unverified`，本批减 5 行（分支创建时基线 37→32）。门禁实测（`ec5d915a4`）与根因：

| 用例 | 实测 | 根因 | 修法 |
|---|---|---|---|
| `file_assemble_control\|recycle_id\|GET` | 200 `file not in recycle bin` | 种子 `file_file.deleted_at` 为 NULL，`deleted_at IS NOT NULL` 不命中；handler 无会话/无归属门 | 种子补 `deleted_at=NOW()`；handler `require_session` + `require_owner(recycle_owner)` |
| `t1\|bbs\|create_topic` | **422** | body 同时给 `author_id` 与 `authorId`，`CreateTopicRequest` 用 `alias` 指向同一字段 → serde 重复字段 | 用例 `field` 改 `authorId`；handler 对非管理员 `author_id != creator` → 403 |
| `t1\|mind\|create_mind` | 200，表内无 canary | 用例查 `x_mind_base_info`，handler 写并存的 `mind_base_info` | cases 表名改 `mind_base_info`；`creatorUnit` 与会话派生不一致且非管理员 → 403 |
| `t1\|mind\|create_folder` | 200，同上 | 用例查 `x_mind_folder_info`，handler 写 `mind_folder_info` | 同上，表名改 `mind_folder_info` |
| `t2\|file\|file_create` | **422** | `FileCreateRequest` 要求 `reference_type` 非空，冒名 body 未给 → 参数提取失败 | cases body 列补 `{"reference_type":"GATE-HACK"}`（runner 合并进冒名 body）；handler 校验 `person == 会话人`，否则 403 |

表中的 `422` 与 `200 无 canary` 都不是「谓词漏了」，而是**判据行没走到真实路径**：
要么请求先被反序列化拦下，要么回表查了另一张同名表，要么夹具行不在谓词范围内。
先修路径、再收归属门，避免用「改断言」迁就实现。

## 14. 17 条 UNVERIFIED 逐条定性

结论先写：**16 条可补证据得到确定判定，1 条（`t2|auth|code`）属用例语义错配、门禁无法判定，保留。**

分类与证据动作：

### 14.1 A 类：请求体缺必填字段（422/400/invalid request）— 7 条

| 用例 | 现状 | 证据动作 |
|---|---|---|
| `t1|bbs|create_topic` | 422 | 补 `forumId`/`title`/`content` 到 `body` 列，按 `author_id` 判定 |
| `t2|file|file_create` | 422 | 补 `folderId`/`name`/`type` 等必填，按 `person` 判定 |
| `t2|meeting|create_meeting` | 400 | 补 `roomId`/`title`/`startTime`/`endTime`，按 `creator` 判定 |
| `t2|organization|identity_create` | 422 | 补 `personId`/`type`/`value`，按 `person_id` 判定 |
| `t2|organization|bind_create` | 422 | 补 `identityId`/`groupId`，按 `person_id` 判定 |
| `t1|attendancedetail_mobile_mobilepreview` | 2xx `data:null` | 补查询键（`personId`+日期/移动端标识），按 `person_id` 判定 |
| `t1|attendancedetail_mobile_recive` | 2xx `received:0` | 补明细/接收键；写侧需同时补行级比对（见 14.3） |

### 14.2 B 类：夹具种子漂移（行/关联不存在）— 5 条

| 用例 | 现状 | 证据动作 |
|---|---|---|
| `file_assemble_control|recycle_id` | 2xx「file not in recycle bin」 | 夹具把 victim 文件置回收站标记，再判 403/他人可见 |
| `t2|attendance|v2_groupschedule_list_filter` | 2xx `data:[]` | 补 filter 的 `userId`/日期 + 种子排班行 |
| `t2|organization|unit_check_unit_has_person` | 2xx `value:false` | 种子把 victim 放入目标单位，再补 `personId`/`unitId` |
| `t2|organization|group_list_object` | 2xx `data:[]` | 补过滤参数 + 种子组成员关系 |
| `t2|organization|unit_list_level_name_object` | 2xx `data:[]` | 补层级参数 + 种子单位层级 |

### 14.3 C 类：门禁观测不到副作用（先修观测能力，仍保留在基线）— 4 条

| 用例 | 原因（已核实到代码） | 证据动作 |
|---|---|---|
| `t1|mind|create_mind` | 用例 `table=x_mind_base_info`，但该 handler 写的是 `mind_base_info`（无 `x_` 前缀，`mind/src/lib.rs:408`）；且 `creator_unit` 仍取自请求体（`lib.rs:392`） | 修正用例 `table` 映射 → 预计转 `LEAK`（真实缺陷，不是「已修」） |
| `t1|mind|create_folder` | 同上，实际写 `mind_folder_info`（`lib.rs:564`），`creator_unit` 取自请求体（`lib.rs:556`） | 修正 `table` → 预计 `LEAK` |
| `t2|mind|create_version` | 实际写 `mind_version_info`（`lib.rs:722`）；`creator` 已取会话（`lib.rs:705`），但 `creator_unit` 仍取自请求体（`lib.rs:706`） | 修正 `table` → 本条 person 维度预计转 `PASS`；另需新增 unit 维度用例覆盖 `creator_unit`，**不能因此说该端点已安全** |
| `t1|oauth|generate_code` | 响应 id 字段是 `data.code` 而非 `data.id`，门禁按 `id` 回查不到行；handler 把请求体 `personId` 直接写入 `x_org_oauth_code.person_id`（`organization_assemble_authentication/src/lib.rs:1297-1299`） | 门禁支持按 `code` 回查 → 预计 `LEAK`（真实缺陷） |

**注意：这 4 条在门禁修好前一律不得从基线删除。** 其中 `create_mind`/`create_folder` 的
`creator_unit`、`oauth_generate_code` 的 `person_id` 都直接取自请求体，是**真实缺陷**；
`create_version` 仅 person 维度已收口，unit 维度仍有洞。

### 14.4 语义错配、保留待裁决 — 1 条

| 用例 | 理由 | 处置 |
|---|---|---|
| `t2|auth|code` | `POST /api/authentication/code` 是**双因素登录第二阶段**（`auth/src/lib.rs:750`），不是资源读端点；不存在「读他人资源」的归属谓词，门禁也无法构造「已签发验证码 + 有效登录态」的前置条件 | 保留在基线；建议单独立项确认是否应从 IDOR 用例集移除。**移除即减行，需另行给出证据，本次不动** |

### 14.5 计数核对

| 类别 | 条数 |
|---|---|
| 可补证据（A 7 + B 5 + C 4） | 16 |
| 语义错配、保留 | 1 |
| **合计** | **17** |

17 条全部继续计入基线 128；只有拿到 `[PASS]` 证据的条目才按 §12.6 三步减行。

### 14.6 已落地：organization 剩余可补证据项（U-org）

| 用例 | 处置 | 证据 |
|---|---|---|
| `t2|organization|identity_create` | **已收归属转 PASS，基线点名删行** | `person_id` 必须等于会话主体或调用者为管理员，鉴权先于必填校验；`type_` 加 `serde(default)` 让 spoof body 能进 handler |
| `t2|organization|bind_create` | **已收归属转 PASS，基线点名删行** | 用例承载字段改 `identity_id`；父资源 `x_org_identity.creator` 按 `CREATOR_OWNER_PREDICATE` 判定，再区分 403/404；`group_id` 加 `serde(default)` |
| `t2|organization|unit_check_unit_has_person` | 保留基线，登记 | 该路径在 `AUTH_EXEMPT_PATHS` 且 RBAC 注册为 `PermissionLevel::Public`（匿名前置查询，`shared/src/middleware/constants.rs:156`、`rbac.rs:288`），与双锁一致性绑定；加会话门会把公开端点变成对所有人 401 的死端点，属「公开 vs 私有」产品口径，需裁决后再动 |
| `t2|organization|group_list_object` | 保留基线，登记 | runner 的 `build_spoof_body` 只能注入单字段，handler 实际读 body `groupList` 数组；群组可见性无归属裁决 |
| `t2|organization|unit_list_level_name_object` | 保留基线，登记 | 同上，handler 读 body `unitList` 数组；组织层级可见性无归属裁决 |

本批基线 31 → 29；破坏实验短路两处新门后 `NEW-FAIL` 恰好点名 2 条（identity_create / bind_create）。

## 15. 基线剩余 8 条裁决（B8，2026-11-08）

基线从 8 条（7 `UNVERIFIED` + 1 `LEAK`）按四类口径逐条裁决。目标不是把数字刷成 0，而是消除
`UNVERIFIED` 挡箭牌：能公开的升级断言、能私有的收会话归属、不属于 IDOR 的单独立项、证据缺口先补
观测再按实测转 `PASS/LEAK`。每条移出基线都必须留下「判据 / 影响面 / 如何回滚」三件事。

| # | 用例 | 类别 | 实际动作 | 基线变化 |
|---|---|---|---|---|
| 1 | `t1\|oauth\|generate_code` | ② 该私有（写归属） | 生产代码已 `person_id=sess`、响应回传 `data.id`；门禁按 id 回查，伪造 owner 与会话 actor 分开判 | 移除 |
| 2 | `t2\|auth\|code` | ③ 非 IDOR | 移出用例集，单独立项 `AUTH-2FA-01`（authentication） | 移除 |
| 3 | `t2\|meeting\|create_meeting` | ④→② | 补必填 body 后可观测；实测 `creator` 采信 body，改为会话派生 | 移除 |
| 4 | `t2\|mind\|create_version` | ④ 表名错配 | `table` 修为 `mind_version_info`；`creator/creator_unit` 已会话派生 | 移除 |
| 5 | `t2\|organization\|unit_check_unit_has_person` | ① PUBLIC-BY-DESIGN | 响应已仅 `value:boolean`；用例升级为匿名 `public-read` + 无 PII 断言 | 移除 |
| 6 | `t2\|organization\|group_list_object` | ④→① | 用例承载字段修为 `groupList` 并补种子；匿名正控制 + 无姓名/手机 canary | 移除 |
| 7 | `t2\|organization\|unit_list_level_name_object` | ④→① | 用例承载字段修为 `unitList` 并补种子；匿名正控制 + 无 PII canary | 移除 |
| 8 | `t2\|program_center\|warnlog_create` | FALSE-GREEN | 2026-11-08 假绿根修批：`field` 改真实归属列 `creator_person`，行由真实 POST 产出 | 移除 |

### 15.1 批 1（PR-A）：oauth 授权码、2FA 登录、mind 版本

**判据**

- `t1|oauth|generate_code`：handler 在 `organization_assemble_authentication/src/lib.rs:1290-1306`
  已把 `person_id` 固定为 `sess.person_unique`，响应同时回传表主键 `id` 与授权码 `code`。门禁改为
  「按 `data.id` 回查行：含伪造 owner → LEAK；不含伪造 owner 但含会话 actor → PASS；
  两者都无 → UNVERIFIED」。这是正证据，不是「2xx 且 canary 增加即 LEAK」的粗判。
- `t2|mind|create_version`：原 `table` 写成 `x_mind_version_info`（实际写入 `mind_version_info`），
  表名错配导致永远观测不到。修正后 `creator`/`creator_unit` 均为会话派生，`creator_unit` 维度按
  `unit-gate-*` 断言。
- `t2|auth|code`：`/api/authentication/code` 是双因素登录第二阶段（`auth/src/lib.rs:750-817`），
  输入是登录凭据、输出是新会话，不存在「某一行属于谁」的 IDOR 归属语义；IDOR 用例集的
  victim/owner 模型不适用。移出用例集，登记为 `AUTH-2FA-01`。

**影响面**

- 仅改门禁判据、用例表与种子，不改生产鉴权逻辑；oauth/mind 的会话派生行为不变。
- 门禁判据变更会让「正确按会话归属落库」的写用例首次具备 PASS 证据；对既有用例只影响
  spoof-write 分支，未回查到行时仍 UNVERIFIED。

**如何回滚**

- 回滚本 PR 即可恢复原判据与 3 条基线行；`AUTH-2FA-01` 条目随 §15 一并回退。
- 若后续发现 oauth `clientId` 验签缺失（已登记在 handler 注释），属独立问题，不回滚本条归属修复。

破坏实验 `sabotage b8-session-owner-write`：把 oauth `person_id`、mind `creator_unit` 改回请求体，
门禁变红并逐条点名 `t1|oauth|generate_code`、`t2|mind|create_version`（两者均 `[LEAK] ... 冒名落库`）。

### 15.2 批 2（PR-B）：unit_check 登记为 PUBLIC-BY-DESIGN

**判据**

- `/api/unit/check/unit/has/person` 同时命中 `AUTH_EXEMPT_PATHS`（`shared/src/middleware/constants.rs`）
  与 RBAC `PermissionLevel::Public`（`rbac.rs`）；这是 o2server 契约的匿名前置查询，客户端在登录前
  用它判定「某人是否属于某组织」。改成必须登录会把公开端点变成对所有人 401 的死端点。
- 响应只有 `{"value": boolean}`，不含姓名/手机/邮箱/组织名等 PII；最小必要字段已经满足。
- 用例从 `spoof-read` 升级为 `public-read`：匿名请求必须 2xx；`victim` 作为正控制必须出现
  （`"value":true`，证明查询真的命中种子关系）；`leak` 里的姓名/手机 canary 任一出现即 LEAK。
  靠断言升级移出基线，而不是删断言。
- 种子补齐：`x_org_unit('unit-gate-v')` + `x_org_identity(person_id='gate-person-v', unit_id='unit-gate-v')`，
  否则匿名请求只能得到 `value:false`（空结果假绿）。
- 修观测时暴露真实缺陷：handler 的 `x_org_person` / `x_org_unit` 查询只用 `$1` 一个占位符，却绑定了
  `&[&person, &person]` / `&[&unit, &unit]` 两个参数，PostgreSQL 直接报 `Parameters(2,1)` → 任何
  合法 `person+unit` 请求都是 500（旧用例只传 `person_id`，person 为空，恰好绕过 SQL 才显得「2xx
  value:false」）。本次修为单参数绑定，让公开前置查询真正可用。

**影响面**

- 生产代码只改参数绑定数量（等价于原 SQL 语义），不改变查询条件、返回字段与匿名策略；
  500 死端点恢复为可用。
- 其余仅新增夹具关系与门禁用例类型；匿名契约不变，新增断言只约束「不得返回 PII」。

**如何回滚**

- 回滚本 PR 即恢复原 `spoof-read` 用例与基线行；夹具里的 `unit-gate-v`/`ident-gate-v` 可一并删除。
- 参数绑定修复不要单独回滚：回滚会让该公开端点重新变成对合法输入 500 的死端点；
  若确需回退，必须同时恢复旧用例的 `person_id`（不触达 SQL）行为并登记新的证据缺口。
- 若产品口径后来要求该端点必须登录，另行产品裁决并同时改 `AUTH_EXEMPT_PATHS` 与 RBAC 注册
  （双锁一致性测试会拦单边修改），不在本条回滚范围内。

破坏实验 `sabotage b8-public-pii-check`：把 `unit_check` 响应改成夹带 `personName=GATE-PERSON-V`，
匿名用例变红并点名 `t2|organization|unit_check_unit_has_person`。

### 15.3 批 3（PR-C）：meeting、group/unit list、warnlog

**判据**

- `t2|meeting|create_meeting`：补 `roomId/title/startTime/endTime` 后才能观测，实测发现两个真实缺陷：
  ① `creator` 直接采信请求体（默认 `"system"`），任意登录用户可冒名落库；② `start_time/end_time`
  以 `&str` 绑定到 TIMESTAMP 参数，`tokio_postgres` 报 `WrongType`，任何缺失/合法请求都是 500（旧用例
  因必填缺失停在 400/UNVERIFIED，从未触达 INSERT）。修复：`creator` 只取 `sess.person_unique`；
  时间参数显式 `$5::text::timestamp` 转换后绑定。
- `t2|organization|group_list_object`：原用例把承载字段写成 `person_id`，而 handler 读 `groupList`，
  永远空查询。修正为 `groupList` 并指向夹具群 `grp-gate-b`；该端点为 o2server 契约的匿名群目录查询
  （`AUTH_EXEMPT_PATHS` + RBAC Public），返回仅群 id/name/type/unitId/personList，`personList` 是成员
  关系的最小不透明标识，不含姓名/手机/邮箱。用例升级为 `public-read`：正控制 `grp-gate-b` 必须出现，
  姓名/手机 PII canary 不得出现。
- `t2|organization|unit_list_level_name_object`：同理，原字段 `level_name` 错误，修正为 `unitList` 并
  指向 `unit-gate-v`。修观测时暴露真实缺陷：handler 复用 `row_to_map`，后者把非 bool 列一律按
  `Option<String>` 取，`BIGINT` 计数列直接 panic → 任何非空 `unitList` 都是 500。改为按列显式构造
  对象，保持原 JSON 形状。
- `t2|program_center|warnlog_create`：2026-11-08「假绿根修批」已收口。原用例 `field=level`
  不是归属列，冒名判定来源与真实归属无关；现改为 `creator_person`，新行由真实写入路径产出，
  写受害者值后门禁实测 `[LEAK]`，基线减 1 行。证据见 `f2e-endpoint-tracker.md`「假绿根修批」。

**影响面**

- `meeting`：写归属改为会话本人（与其它写面一致）；时间戳绑定修正把 500 死端点恢复为可用。
  正常前端本来就传字符串时间，修复对合法请求是纯可用性提升，不改变响应字段。
- `group/unit list`：只改用例承载字段与断言、修一个 BIGINT panic；匿名契约与返回字段不变。
  新增断言只约束「不得返回姓名/手机 PII」。
- `warnlog`：不动生产代码。

**如何回滚**

- 回滚本 PR 恢复 meeting 旧插入 SQL / body creator、恢复两条 list 用例与三条基线行。
  `meeting` 的时间戳转换与 `unit_list` 的 BIGINT panic 修复不建议单独回滚——回滚会让端点重新 500。
- 若产品口径要求群/单位目录必须登录，另行裁决并同时改 `AUTH_EXEMPT_PATHS` 与 RBAC（双锁一致性
  测试会拦单边修改），不在本条回滚范围。
- `warnlog` FALSE-GREEN 已于 2026-11-08「假绿根修批」收口（用例改真实归属列，不动写语义）。

破坏实验 `sabotage b8-b3`：把 meeting `creator` 改回 body、在两条匿名目录响应里注入
`mobile=GATE-V-MOBILE`，门禁变红并逐条点名上述 3 条。

### 15.4 合入后 §12.6 三步核对（#267 / #268 / #269）

对 `main@ee5c9551e`（三笔 PR 依次合入后的 HEAD）逐项核对：

1. **行是否真在 main**：`HEAD` 与 `origin/main` 均为 `ee5c9551e032497e9e68026442889725033046da`，
   `git diff HEAD origin/main` 为空；三笔合入提交的文件清单与各自 PR 一致（#267 门禁/用例/基线/文档；
   #268 门禁/用例/夹具/`endpoints_org.rs`/脚本/文档；#269 `meeting/lib.rs`/`endpoints_unit2.rs`/脚本/
   用例/基线/文档）。
2. **基线是否虚减**：在 `main` 跑 `cargo test --test security_regression -- --nocapture`
   → `cases=280 pass=279 leak=1 unverified=0 integrity=0`，唯一非 PASS 是
   `t2|program_center|warnlog_create`（FALSE-GREEN，基线保留行）。本次移出的 7 条
   （oauth / auth|code / mind / unit_check / meeting / group_list_object / unit_list_level_name_object）
   在 `main` 上全部 `[PASS]`，`unverified=0`，不存在靠 UNVERIFIED 占位挡枪。
3. **门禁是否接线**：在 `main` 派生的临时分支（behind=0）复跑三组 sabotage，均变红并逐条点名：
   - `b8-session-owner-write` → `[LEAK] t1|oauth|generate_code`、`[LEAK] t2|mind|create_version`；
   - `b8-public-pii-check` → `[LEAK] t2|organization|unit_check_unit_has_person`；
   - `b8-b3` → `[LEAK] t2|meeting|create_meeting`、`[LEAK] t2|organization|group_list_object`、
     `[LEAK] t2|organization|unit_list_level_name_object`。
   脚本执行后文件均已 `[restore]`，临时分支已删除，工作树干净。

三项全过。基线由 8 条降为 1 条（仅 FALSE-GREEN `warnlog`）；若后续任一步复核失败，按 §15.1–§15.3
的回滚口径逐批回退对应基线行。

### 15.5 归属列写侧批 2：`message_assemble_communicate` sender

**问题（读/写面依赖 sender，写侧请求可控）**

- `send_message`（`/api/message/assemble/communicate/send`）与 `im_msg`
  （`/api/message/assemble/communicate/im/msg`）都把 `x_message.sender` 直接采信请求体，
  缺省 `"system"`；而 `mark_consume`、消息撤回等写面归属判定取 `sender`（见
  `security_gate_tests.rs` 的「不写 creator_person，归属事实取 sender」）。
  任意会话成员可冒名落库：消息在会话里显示为他人发送、并落入他人的归属面。

**改前为什么门禁是绿的**

- 门禁没有这两个写端点的用例；既有 `message_list_paging|GET` 只验证读列表，
  `sender` 的真伪无人断言。

**修法**

- 两处写侧改为 `sender = sess.person_unique.clone()`，忽略请求体同名键。
- 种子：`gate-conv-msg` 会话 + `audit-user`/`gate-victim` 两个成员（正反两向都需成员资格）。
- 用例：2 条 `spoof-write`（`field=sender`，`carrier=body`，`dir=both`）：
  2xx 后按响应 `data.id` 回表 `x_message`，含 victim → LEAK、含 actor → PASS。

**改后靠什么证明真防住了**

- 2 条实测 `[PASS]`：回表 `sender` 命中会话 actor 的正证据（冒名键被忽略）。
- 破坏实验：把两处写侧还原为请求体 `sender` → `[LEAK]` 2 条且 `NEW-FAIL` **恰好点名**
  `send_message` / `im_msg`（`owner=gate-victim（冒名落库）`）；
  还原后 `非 PASS 1`（仅 FALSE-GREEN `warnlog`），与基线一致。
- 基线不变（新增 2 条均为 PASS，不涉及减行）。

**登记（本批不做）**

- `x_message_mass.creator` 从不写入：`mass_create` 只写 `creator_person` 且为管理员档；
  读 `mass/{id}` 的归属谓词用 `creator`，靠 admin 旁路才 PASS，非管理员属主读不到自己的行。
  修复需补写 `creator`，但门禁无管理员攻击者档，无法取得 sabotage 正证据，登记独立立项。
- 会话成员 `role` 从不写入（`security_gate_tests.rs` 已注明），无读侧依赖。

### 15.6 归属列写侧盘点总表（本轮六 crate）

方向：写入侧把归属列写成常量/空/从不写，逐个对照读侧是否依赖。只列结论，不重复已收口读用例。

| crate | 结论 | 依据 |
|---|---|---|
| organization_core_entity | 已修（#263）：`identity_create`/`bind_create` 归属门 + 会话身份；`unit_check` 因 Public 双锁登记 | §14.6 |
| personal / personal_extend | 无缺口：请求体归属键零命中；人脸/签名 `person_id`/`creator` 均会话派生 | `face_create` 强制本人、`save` 窄白名单 |
| organization_assemble_personal / organization_assemble_authentication / auth | 无缺口：请求体 `personId`/`creator` 零命中；oauth 落库已会话派生 | `oauth_session_derived.rs` |
| message_assemble_communicate | 已修本批 `sender`；`x_message_mass.creator` 从不写入 → 登记 | §15.5 |
| calendar_core_entity | 已修（批 1）：create 写侧会话身份 + update/remove 归属门 | §15.7（PR #274） |
| attendance_assemble_control | 五张配置表 create 只写 `creator_person` 不写 `creator`；读侧 `creator OR creator_person` 已覆盖真实行 → 非当前缺陷，登记写侧收口 | 下节 |

#### 15.6.1 attendance 五张配置表：只堵写侧方案（不回填历史）

- 表：`x_attendance_employee_config` / `x_attendance_import_file_info` /
  `x_attendance_schedule_setting` / `x_attendance_setting` / `x_attendance_statistical_cycle`。
- 现状：create 只写 `creator_person`（会话派生），`creator` 为 NULL；读谓词同时认两列，
  真实新行通过 `creator_person` 命中本人，因此**当前无读写不一致**，不构成本轮可修缺陷。
- 上线前量化（只读 SQL，逐表执行，结果留档）：
  `SELECT count(*) FROM <table> WHERE creator IS NULL AND creator_person IS NOT NULL;`
- 修法（下一批）：五处 create 的 INSERT 增加 `creator = session.person_unique`，忽略请求体；
  **不回填历史行**；读谓词保持 `creator OR creator_person`。
- 证据缺口：该改动不会翻转任何门禁（改前改后读都命中 `creator_person`），无法做 sabotage 点名；
  按「无证据不改」纪律登记，待能构造新断言（如写后读一致性用例）再实施。
- attendance 其余写侧：`person_id`/`user_id` 请求可控处均已 `require_admin` 或
  `require_owner`/`normalize_key` 收口（`attendancedetail_create`、`selfholidaysimple_create`、
  `v2_groupschedule_post`、`mobile_check_impl`）。

### 15.7 归属列写侧不可用批 1：`calendar_core_entity`（B 线）

**问题（读侧已依赖、写侧不可用）**

- `calendar_create` / `event_create` 的 `createor` 直接采信请求体，缺省回落
  `"anonymous"`（`calendar_core_entity/src/lib.rs`）；而 `calendar_get` 的
  `CALENDAR_READ_PREDICATE` 用 `createor = $2` 判归属 → 任意登录用户可把新建行
  归属写成受害者，读侧谓词对真实写入行失效。
- `calendar_update` / `calendar_remove` / `event_update` / `event_remove` 只校验
  行存在 + `OPEN`，不校验归属 → 归属列即使写对也不生效。

**改前为什么门禁是绿的**

- 门禁只挂了 `calendar_get`（读，PASS），六个写端点没有任何用例；
  `/api/calendar/core/entity/*` 不在 `ADMIN_WRITE_PREFIXES`，RBAC 兜底为
  `Authenticated`，非管理员可达，因此缺陷真实可触发但无人断言。

**修法**

- 写侧：`calendar_create` / `event_create` 的 `createor = require_session()?.person_unique`，
  忽略请求体同名键。
- 写面：新增 `require_owner(db, owner, actor)`（`owner == actor` 或
  `actor_is_admin`，同源读谓词 admin EXISTS）；四处 update/remove 在取行后、写库前调用，
  NULL/空归属 fail-closed。
- 种子：`x_cal_event` 补 `xcal-victim-event`（`gate-victim`）与
  `rev-xcal-victim-event`（`audit-user`）。
- 用例：6 条 `spoof-write`（`field=createor`，`carrier=body`）。create 两条 `dir=both`
  走「2xx 后回表：含 victim → LEAK，含 actor → PASS」；update/remove 四条 `fwd-only`
  用 body 字面量带 `id`（runner 的 `build_write_body` 会剔除 `id`，不适用 `idor-write`）。

**改后靠什么证明真防住了**

- 6 条实测 `[PASS]`：create 是正证据（回表 `createor` 命中会话 actor），
  update/remove 是 403 拒绝；读侧正控制沿用既有 `calendar_get|own`。
- 破坏实验：create 写侧还原为 `req.createor.unwrap_or("anonymous")`、`require_owner`
  短路为恒放行（保留 `actor_is_admin` 调用避免 dead_code）后，`NEW-FAIL` **恰好点名**
  这 6 条；还原后复跑 `非 PASS 1`（仅 FALSE-GREEN `warnlog`），与基线一致，无
  `STALE-BASELINE` / `STATE-DRIFT`。
- 基线不变（本批 6 条均为新增 PASS，不涉及减行）。

**影响面与回滚**

- 只改写归属来源与写面归属门，不改表结构、不回填历史、不改响应字段；
  管理员行为不变。
- 回滚本 PR 即恢复请求体 `createor` 与四个无归属校验的写端点，同时移除 6 条用例与种子。

**登记（本批不做）**

- `event_list_by_calendar` 无归属谓词（读面，另批）。
- `event_create` 不校验 `calendarId` 所属日历归属（写入他人容器的完整性面，另批）。
- `calendar_list_my` 返回全部 OPEN 日历（读面，另批）。




