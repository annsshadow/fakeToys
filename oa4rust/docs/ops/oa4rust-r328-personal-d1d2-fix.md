# R328 PC 个人中心 D1/D2/D3 修复与验证

## 背景与范围

- 分支：opt1000/oa4rust-pc-ai-refactor（禁止 main）。
- 测试环境：测试服，域名占位 <YOUR_TEST_DOMAIN>；真值不入库。
- 截图来源：Playwright Chromium headless 1440x900 对已部署测试服的实测，**非物理真机**。
- 提交范围（8 文件，含队长 revision1 扩 scope 的 D3 两文件）：

| 文件 | 作用 |
| --- | --- |
| frontend/apps/desktop/src/views/Personal.vue | D1 错误信封两分支 + D2 签名解包 + D2 次生 watch 响应式 |
| frontend/apps/desktop/src/views/Personal.business.contract.test.ts | 前端契约（12 it，含 watch 响应式回归） |
| backend/crates/personal_extend/src/personal.rs | 保存签名桥接 x_custom + D2 次生 bridge UPDATE 刷 created_at |
| backend/crates/personal_extend/src/tests.rs | 真 DB 写读测试（含 created_at 刷新断言） |
| backend/crates/personal_extend/src/tests_generated.rs | 静态契约（2 it） |
| backend/crates/personal/src/signature.rs | D3：list 的 created_at 由 String 解码改 NaiveDateTime（仅此一处） |
| backend/crates/personal/src/tests.rs | D3：真 TIMESTAMP 行→list 200 的 RED→GREEN 回归测试 |
| docs/ops/oa4rust-r328-personal-d1d2-fix.md | 本文档 |

## 缺陷与修复

### D1[P2] HTTP 200 业务错误信封误判成功

- 根因：Personal.vue pwdMutation onSuccess 与 mutate options onSuccess 均直接清空表单/弹成功提示，未检查 resp.type === error；错误旧密码旧行为仍显示“密码修改成功”。
- 修复：两处 onSuccess 统一先判 resp?.type === error，命中则写 pwdError + toast.error 并 return，不进成功分支、不清空表单。不改 SDK api.ts，不影响成功路径。

### D2[P2] 保存签名后个人签名回显为空

- 根因双端：① 前端 queryFn 把 resp.data 当数组且取 content，真信封为 {data:{signatures:[{value}]}}；② 后端 save_signature 只写 x_person_signature，而 signature/list/upload 读写 x_custom（value 列），保存与列表不同表。
- 修复：前端解包 resp.data.signatures[0].value，空列表回空串；后端 save_signature 原 x_person_signature 写链保留，新增桥接写 x_custom（SIGNATURE_{final_id}，person=session.person_unique，重复保存 UPDATE 不膨胀），与 list 的归属口径同源。

### D3[P1]（扩 scope revision1）signature/list 有行即 500

- 发现：D2 桥接保存返回 saved:true 后 /api/person/signature/list 500、签名无法回显；用 upload 端点独立写一行 x_custom 后同样 500，坐实与桥接格式无关。t123 审计时 x_custom 为空表故未暴露。
- 根因：personal/src/signature.rs:130 用 row.get::<_, Option<String>>(created_at) 解码 TIMESTAMP 列（migrations/015_create_x_custom.sql:9 created_at TIMESTAMP），token-postgres 的 FromSql 类型不匹配；空表不解析行故 200，一旦有真实行即 500。
- 修复：仅将 list() 该行改为 row.get::<_, Option<chrono::NaiveDateTime>>(created_at).map(|dt| dt.to_string())（chrono 已是 crate 依赖）；manager_list 的同形解码按队长限定不动；归属谓词/删除分支/其它字段均不动。

### D2 次生一[P2] watch 静态快照致签名回显为空

- 发现：D3 修复后 listOnReload HTTP 200 且含新签名，但 textarea 仍为空。
- 根因：Personal.vue:132 signature.value = sigData.value ?? "" 在 setup 期执行，此时 useQuery 尚未 resolve，取到静态 undefined，后续 query 数据到达不再同步。
- 修复：import watch，改为 watch(sigData, (v) => { signature.value = v ?? "" }, { immediate: true })。契约新增第 12 it 断言响应式同步（RED 1 failed → GREEN 12/12）。

### D2 次生二[P2] bridge UPDATE 未刷 created_at 致回显旧签名

- 发现：watch 修复后 textarea 回显旧图片签名（iVBORw==），而非刚保存的新文本签名。
- 根因：personal_extend/personal.rs bridge UPDATE 只刷 updated_at 不刷 created_at；signature.rs:116 list 按 created_at DESC 排序，Personal.vue:129 取 sigs[0]，旧签名排在新签名之前。
- 修复：仅 bridge UPDATE SET 增加 created_at = NOW() 一行；不动 signature.rs ORDER BY / 归属谓词 / delete / upload。真 DB 测试先把已有行 created_at 调旧 1 小时再保存，断言刷新至 1 分钟内（RED exit101 → GREEN personal_extend --lib 22 passed）。

## 验证台账

- RED：前端 3 failed/8 passed（+ watch 1 failed/11 passed）；personal_extend 2 failed/1 passed（+ created_at 1 failed）；D3 cargo test -p personal --lib 定点 1 failed（signature.rs:130:35 error deserializing column 4）。
- GREEN：前端定点 12/12；personal_extend --lib 22 passed/0 failed/1 ignored；personal --lib 38 passed/0 failed（含 r328_signature_list_decodes_timestamp_created_at）；全量 vitest 267 files/2159 tests 0 failed；pnpm run typecheck exit 0。
- 部署：PC 入口 index-BhYD-jCK.js / index-BMFkaD73.css，DEPLOY_R328_PERSONAL_PC_HASH_OK；后端 Linux ELF 经 docker rust:1.98-bullseye 编译（含桥接、D3 解码、created_at 刷新三处），原子替换并健康检查 active。
- 真浏览器：见 test-results/r328-personal-fix/evidence.json（非物理真机标注）；截图 01-wrong-password.png（D1）、02-signature-persist.png（D2/D3 最终复验）。
- 安全：本文无真域名/IP/凭据，均用占位符；口令仅由 extract-remote-creds.py 运行期从远端凭据文件读取后注入页面，不落代码/文档/日志。
