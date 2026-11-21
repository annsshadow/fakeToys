# R317 M2 系统消息写读列不同源修复报告

- 任务：t93 [R317][A4]（M2 系统消息写读列不同源 test-first 修复）
- 基线：t91 oa4rust-r317-static-latent-audit.md（静态候选 M2）、t92 oa4rust-r317-message-candidate-verify.md（真浏览器坐实）
- 仓库分支：opt1000/oa4rust-pc-ai-refactor
- 测试入口：测试服现有产物；测试域名用 <YOUR_TEST_DOMAIN> 占位，本报告 0 真值

## 1. 修复范围与纪律

- 仅改 inScope 三文件：backend/crates/message/src/lib.rs、backend/crates/message/src/tests.rs、本文档。
- 不改 consume_list 的 SELECT SQL、admin 门禁、前端 IMChat.vue；xcreateTime 容错仅在行映射层。
- 两处真缺陷均 test-first：先写回归测试跑 RED，再改源码跑 GREEN，保留输出证据。
- 口令/连接串不入库：运行期从测试服凭据文件读取，真值不回显；deploy/harness 脚本落 gitignored 目录。

## 2. 缺陷一：custom_create 写读列不同源（M2 主缺陷）

### 2.1 根因

- 写端点：POST /api/message/custom/create（message/src/routes.rs 绑定；lib.rs custom_create，约 :151-177，require_admin 门禁）。
- 修复前 INSERT：INSERT INTO x_msg_message (id, title, body, type, consumer, is_read, xid, xtitle, xbody, xtype, xconsumed) VALUES ($1,$2,$3,$4,$5,false,$6,$2,$3,$4,false)。$5 = req.consumer.unwrap_or_default()。
- 读端点：GET /api/message/consume/list/{consume}/count/{count}（lib.rs:50-62 consume_list）WHERE xconsumer=$1 AND xconsumed=false；unread_count（约 :219-230）同样按 xconsumer 过滤。
- 漂移：写规范列 consumer，读遗留列 xconsumer——INSERT 从未写 xconsumer，故新消息 xconsumer=NULL，对任何 consumer 不可见。这比 t91 静态候选「consumer 缺省空串」更强：即使传具体 consumer 也查不到。
- t92 真浏览器证据：create 200 但 consume/list data=[]；DB 回读 x_msg_message 两行 xconsumer=NULL。**判定确凿真缺陷**。

### 2.2 test-first RED→GREEN

- RED：tests.rs 新增 test_custom_create_insert_writes_xconsumer_column（断言 INSERT 列清单含 xconsumer）与 test_custom_create_xconsumer_reuses_consumer_placeholder（断言 xconsumer 与 consumer 复用同一占位符 $5）。首跑 2 failed，报 INSERT column list must include xconsumer。
- 修复：lib.rs INSERT 补 xconsumer 列并复用 $5（两列均 text，无 E42P08；id varchar 与 xid text 主键仍绑两次）。
- GREEN：touch tests.rs 强制重编后 cargo test -p message --lib 15 passed / 0 failed。

## 3. 缺陷二：consume_list 行映射 xcreateTime NULL panic（潜伏，补写 xconsumer 后暴露）

### 3.1 根因

- 补写 xconsumer 后，consume/list 开始返回新行，但 HTTP 500。后端 run.log：thread tokio-rt-worker panicked at crates/message/src/lib.rs:97:39: error retrieving column xcreateTime: error deserializing column 6。
- 源码 lib.rs:58 SELECT ... "xcreateTime" ...；:97 原用 Value::String(row.get("xcreateTime")) 非 Option 读取，同函数其他 6 列均 Option<String> 容错。
- 远端只读探针坐实：xcreateTime 列 text nullable 无默认值，新消息与历史数据 xcreateTime 全 NULL——读取 NULL 行触发 panic。
- 这是独立潜伏映射 bug，直接阻塞「consume/list 可见」验收，且位于 inScope lib.rs，故一并修复。

### 3.2 test-first RED→GREEN

- RED：tests.rs 新增 test_consume_list_xcreatetime_null_tolerant（断言 consume_list 用 get::<_, Option<String>>("xcreateTime")）。首跑 panicked at tests.rs:197。
- 修复：lib.rs:97 改为 Value::String(row.get::<_, Option<String>>("xcreateTime").unwrap_or_default())。
- GREEN：touch lib.rs+tests.rs 强制重编后 cargo test -p message --lib 16 passed / 0 failed。

## 4. 构建与部署口径

- Linux ELF：Docker rust:1.98-bullseye，cargo +1.98.0 build --release -p oa4rust --offline --target-dir /workspace/target/linux（内层 bash -lc 单引号）。强制重编先 touch lib.rs+tests.rs，确保链接进最新修复。
- 原子部署：scp 到测试服 → 备份 → install + mv 原子替换 → systemctl restart + is-active 健康检查 → 失败自动回滚。远端命令用单引号避免本地展开 $()。
- 复验：真登录 admin → /app/im 页面内 fetch（credentials: include）走真实端点 → create 带具体 consumer → GET consume/list/{consumer}/count/50 可见 → GET unread/count/{consumer} > 0 → DB 只读回读 xconsumer=consumer 同值。
- 证据落 gitignored 目录 oa4rust/test-results/r317-m2fix/（evidence.json + 1440x900 PNG），来源标注 Playwright Chromium headless against deployed test server; not a physical device（非物理真机）。

## 5. 复验结果（2026-11-21 真浏览器实测）

- 构建：新 Linux ELF 15:16:59 / 118336104 字节（旧 14:48:26 / 118337448），BuildID 由 3233fb3f 变为 81e7973c，确认为含 xcreateTime 修复的重新链接；file 确认 ELF 64-bit x86-64。
- 部署：原子替换测试服后端，systemctl is-active=active，健康检查通过；远端 ELF 大小 118336104、mtime 15:19:09、BuildID 81e7973c。
- 真浏览器（Playwright Chromium headless，/app/im 真登录 admin，1440x900）：
  - POST /api/message/custom/create {title, body, consumer=R317-M2FIX-...-CONSUMER} → HTTP 200，body.data.created=true。
  - GET /api/message/consume/list/{consumer}/count/50 → HTTP 200，data 含新建消息行（标题 marker 匹配，visible=true）。
  - GET /api/message/unread/count/{consumer} → HTTP 200，body.data.count=1（>0）。
  - DB 只读回读 x_msg_message：新行 consumer 与 xconsumer 同为 marker-CONSUMER 值（写读列同源坐实）。
  - 0 pageerror、0 API 失败（errors=[]、apiFailures=[]）。
- 证据：oa4rust/test-results/r317-m2fix/（evidence.json + 01-im-before.png / 02-m2fix-after-create.png / 03-m2fix-after-reload.png 共 3 张 1440x900 PNG），来源标注 Playwright Chromium headless against deployed test server; not a physical device（非物理真机），已 gitignore 不入库。
- 结论：两处缺陷均修复并经真浏览器复验通过；create→consume/list 回显、刷新持久、unread 计数、DB 写读列同源全部满足验收。