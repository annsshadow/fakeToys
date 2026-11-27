# R621a mind_save 身份盲写 IDOR 甄别报告

## 一、结论与判级
- 甄别对象：POST /api/mind/assemble/control/mind/save（crate mind_assemble_control，src/u2.rs 的 mind_save）。
- 结论：确认身份盲写 IDOR，判级 P1（逻辑缺陷确凿，非预设），本轮仅甄别、零源码改动。
- 三分：可达性需有效会话且知他人脑图 id（UUID 非自增、非匿名）；危害为覆写他人私有脑图的名称与正文、将私有置为共享并把攻击者身份写入受害者版本链；限制为当前迁移 schema 下另有两处独立阻断缺陷会使更新分支整体 500 回滚，夹具隔离后方暴露越权写（见第四节）。

## 二、锚点与根因
- src/u2.rs:422 require_session 仅验会话存在；:450 person 取自 sess.person_unique，但只用于版本 INSERT 的 creator（:494），不参与更新归属门控。
- :452-457 body.id 非空走更新分支；:462-479 UPDATE x_mind SET name/content/folder_id/description/shared/parent_id WHERE id=$1 AND deleted_at IS NULL，无 creator 归属谓词、无 person_can_manage_mind 门；:474 shared 由请求体自证可将他人私有脑图置公开。
- :480-501 更新成功后追加版本 INSERT（creator=$5=会话人），攻击者身份被写入受害者版本链；:507 之后 body.id 为空走新建分支属合法插入，不构成越权。
- 同文件 person_can_manage_mind（:67-90，先 admin 判定再比对 creator，None 返回 false fail-closed）被 folder_delete/mind_restore/mind_share/mind_share_cancel/mind_destroy/mind_destroy_recycle 等写端点调用；mind_save 是唯一漏门写端点。路由在 src/routes.rs:80-83 真实注册。

## 三、真执行 RED 探针
- 独占临时 PG + 迁移后的干净库，一次性契约-two-cases；会话经 Router.layer(Extension(test_session(p))) 注入，裸请求不带会话。
- alice 凭 bob 的脑图 id 调 mind_save：实测 status=200 OK，body data.fileVersion=2/updated=1；落库 name=hacked、content=hacked-content、shared=true，版本链 creator=r621a-alice。越权覆写与污染全部落库，RED 断言 403 失败（panic 于契约 :101，left=200 right=403）。
- 反臂：bob 本人改自己 200 通过；admin 代改 200 通过；裸会话 401 通过；兄弟 mind_share 归属门对冒名返回 forbidden 反证通过。1 passed / 1 failed，PIPE_EXIT=101。
- 说明：探针用后即删，不入库；连接串仅运行时以本地回环占位注入，文档不写真值。

## 四、两处独立阻断缺陷（据实披露）
- 缺陷甲：029 迁移把 x_mind_version_info.file_version 建成 INTEGER(INT4)，u2.rs:483 用 i64 读 COALESCE(MAX(file_version),0)+1，反序列化 panic（error deserializing column 0），更新事务回滚。夹具把该列升 BIGINT 后方能继续。
- 缺陷乙：029 的 x_mind_version_info.folder_id 为 NOT NULL 且无默认值，但 u2.rs:492 版本 INSERT 不含该列，真实更新必 500 回滚（body message=internal server error，DB 未改）。夹具给该列默认值空串后方能落库。
- 二者属迁移与源码错配的独立问题，非身份绑定范畴；本轮不修、只隔离，以免掩盖 IDOR 本身。当前产物库上 mind_save 更新分支实际返回 500，越权未必落库；但身份盲写逻辑缺陷本身确凿，一旦上述 schema 修复即成真实越权。

## 五、兄弟门对照与限制
- 读谓词 MIND_ROW_READ_PREDICATE（:43-49）允许 creator 本人或 shared=true 或共享接收人或 admin 读取，与写侧严格区分；读侧共享接收合法，不代表可写。
- 写侧约定仅属主或 admin 可改（由 person_can_manage_mind 表达），mind_save 未遵守，构成越权写入。
- 利用前提：有效会话 + 知道他人 mind id（UUID 非自增，非匿名可扫）；body.id 空的新建分支不构成越权，admin 代改属授权行为。
- 修订建议（不在本轮实施）：更新前调用 person_can_manage_mind(pool, sess.person_unique, id)，非属主非 admin 返回 Forbidden，门置于首个 DB 写之前；并单独修复两处 schema 与源码错配。

## 六、证据与 sha 分列
- 真父（派发时）：e78561a995dfcf9c9ed08aa71ea1379f71442ed5；采集证据时本地 HEAD 已前移至 c09ebf1a360f0bc764e779626f2b97faba642a42，区间仅队友 t249 的 R621c OrgDirectory 审计文档提交，未触及本轮写域与目标文档，据实披露、不回滚队友工作。
- runnerSha=9075d133d7ad400ee60d24b77699c6b7cb59a286e7df6f203f0594a3394842b7。
- 契约 sha256=6a219649e1a450bfde5a232e67c414827e9f7073de14c1456bc992bad6d18b99（取证后删除）。
- src/u2.rs sha256=22e6afe3f3ea012da2396360b802b3ebc86cf699613861a3b03ba23aa9770523。
- RED 日志 sha256=7b395874167dc1c03a92575eed8561c95db3eab5891135b4a0fecde3e0855606。
- runnerSha 由 node:crypto 对元数据 JSON 计算，与文件 sha256 分列；全文零密钥口令真域名真 IP 个人路径入库。
