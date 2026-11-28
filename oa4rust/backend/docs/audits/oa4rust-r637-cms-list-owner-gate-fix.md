# R637 CMS 文章列表/笔记列表归属门真修复审计

## 结论
- 双 P2（沿用 R636b 文章列表 / R636c 笔记列表甄别与终审判级，非新判级）。
- 修复前：/api/cms/article/list 仅按 deleted_at IS NULL 裸查全表，任意登录者及匿名可见他人私有草稿；/api/cms/core/entity/note/list 匿名 200 裸读全表，任意登录者可见他人笔记。
- 修复后：文章列表按「published 公开 OR 本人草稿 OR admin 全量」可见范围过滤；笔记列表要求登录并仅本人可见（admin 全量）。

## 锚点与根因
- oa4rust/backend/crates/cms_core_entity/src/lib.rs
  - article_list（原 227-272）：SeaORM Entity::find 仅 filter DeletedAt.is_null，无归属谓词。x_cms_article 有 author_id/status 列，与既有 article_read_gate（ARTICLE_READ_ALLOWED_SQL，55-59）口径不一致——详情门有归属校验，列表门缺失。
  - list_cms_entity_table（原 386-429）+ cms_entity_note_list（原 463-469）：note 分支 SQL 的 WHERE 仅 deleted_at IS NULL，无 person_id 归属；handler 仅接 pool，无 session。x_cms_note 有 person_id 列（DDL 093:55-65）。
  - 对照：同族写端点（801/818/836）已用 require_admin_opt；列表读门缺失。
- 根因：列表端点在数据模型有明确归属列时未施加可见范围过滤，属身份盲读，不涉越权写库。

## RED→GREEN 证据
- 契约：crates/cms_core_entity/tests/r637_cms_list_owner_fix_contract.rs（implementation 入库，用 router oneshot 使签名变化前后均可编译）。
- 真父 RED：cargo test -p cms_core_entity --test r637_cms_list_owner_fix_contract -- --test-threads=1
  - 文章臂失败：alice 可见 bob 私有草稿 got=["r637-alice-draft","r637-bob-draft","r637-public"]（契约 :98）。
  - 源码守卫失败：article_list 必须提取会话（契约 :147）。
  - test result: FAILED. 0 passed; 2 failed，EXIT=101。
- GREEN：修复后同命令 test result: ok. 2 passed; 0 failed，EXIT=0。

## 修复实现
- article_list：签名加 session:Option<Extension<Session>> + Extension<Pool>；用 sea_orm::Condition::any() 构造可见范围（published OR AuthorId=本人，admin 追加 Id.is_not_null 全量），逻辑等价 ARTICLE_READ_ALLOWED_SQL；回传八字段形状零变化。
- cms_entity_note_list：签名加 session，ok_or(Unauthorized) 实现匿名 401；经 is_admin 判管理员。
- list_cms_entity_table 增 actor/admin 参数，仅 note 分支 SQL 加 AND (person_id=$1 OR $2::boolean) 并传绑定；其余四表保持空绑定不变。

## 三 verify
- V1 定点契约：cargo test -p cms_core_entity --test r637_cms_list_owner_fix_contract → 2/2 passed，EXIT=0。
- V2 包级：cargo test -p cms_core_entity -- --test-threads=1 → lib 20 passed + 契约 2 passed + doc 0 passed，0 failed，EXIT=0（无写域外基线失败，无需 skip）。
- V3a cargo fmt -p cms_core_entity -- --check → EXIT=0（首跑仅契约长行 diff，cargo fmt 后通过）。
- V3b cargo clippy -p cms_core_entity --all-targets --no-deps -- -D warnings → EXIT=0，零告警。

## 行为变化
- 文章列表：匿名/非作者不再可见他人私有草稿；本人可见自己草稿；published 全员可见；admin 全量不变。
- 笔记列表：匿名由 200 变 401；非本人不再可见他人笔记；admin 全量不变。
- legacy_success 信封（data/type/count 等）形状零变化，仅过滤范围收紧。

## SHA（分列，据实不可由单文件复算 runnerSha）
- 修复对象 lib.rs sha256=3db4d4ea970cd87fd9a9b706ac2f5c34193560f63196902001e56387749a38cd（37937 bytes / 1023 行；基线 a95bf0b0 → 修复后 3db4d4ea）
- 契约 sha256=70f0327e0df2179cf54feb6bb2c35d4e92353856eef9879ae8602198b34ac52b（7829 bytes / 215 行）
- runnerSha = sha256(JSON.stringify(meta)) = e0b9af0aa75d2b2702406ba79071aecd8f0ea2879ef8357a5f42526723738b83（运行元数据哈希，与两文件 sha 分列，不可由文件直接复算）
- 真父 HEAD = ff0e28185142e616fdc4cae17bfd48c919bac185

## Fail Loud 披露
- 写端点 person_id 取自请求体 payload（cms_entity_create/save，492-575），与会话身份存在语义张力，属另一路冒名族；本次仅修列表读门，未触碰写端点，另记待修。
- 未跑 GUI 人工点验；仅后端 oneshot + 定点/包级 cargo verify。
- 测试连接串为本地回环 + 占位口令 postgres://user:pass@localhost:<TEST_PORT>/db，无真值入库（占位域名 <TEST_HOST>）。
