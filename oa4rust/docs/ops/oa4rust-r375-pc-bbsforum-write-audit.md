# R375 PC BBSForum 写操作链失败信封甄别（只读审计）

> 审计人：qa-e2e ｜ 轮次 R375 ｜ 任务 t194 ｜ 判定：**1 个 P1 真缺陷**
> 实测环境：Playwright Chromium headless 1440x900，**非物理真机**；测试服域名以 <YOUR_TEST_DOMAIN> 占位，测试主机以 <YOUR_TEST_HOST> 占位。
> 只读甄别，未修改任何源码/契约/后端/SDK；破坏性真状态翻转与真删除均未执行。

## 1. 结论与判定依据

PC 论坛页多个**有真实点击入口**的写函数，在后端返回业务失败信封（HTTP 200 + `type:error`，或 HTTP 200 + `type:success` 却带 `saved:false`/`deleted:false`）时，**仍无条件弹出成功 toast**，误导用户认为操作已生效。

真缺陷三全判据均满足：
1. **真 UI 可复现**：决定性对照轮只注入列表数据、写请求完全走真后端，真实点击重命名/删除/发帖/回复按钮复现误报。
2. **真请求响应**：假 ID 探针与决定性对照请求的真实响应为 `status 200, type:success, data.saved:false` / `data.deleted:false`、`status 200, type:error`。
3. **源码定位**：`frontend/apps/desktop/src/views/BBSForum.vue` 相关函数在 `await` 后直接 `toast.success()`，不读取响应 `type` 或权威 bool。

## 2. 源码定位（相对路径）

路由：`frontend/apps/desktop/src/main.ts:93` → `path=bbs`。

真实 `@click` 写入口（模板）：
- `BBSForum.vue:26` 发起主题 → `createTopic`
- `BBSForum.vue:35` 新建版块 → `openNewSection`
- `BBSForum.vue:48` 重命名版块 → `saveSection` edit 分支
- `BBSForum.vue:49` 删除版块 → `deleteSection`
- `BBSForum.vue:116` 发帖表单 `@submit.prevent=createTopic`
- `BBSForum.vue:179` 回复 → `submitReply`
- `BBSForum.vue:194` 版块表单 `@submit.prevent=saveSection`

误报函数群（await 后端响应后无条件成功提示）：
- `saveSection` `BBSForum.vue:363-384`：edit 分支 `:371 POST section/save/{id}` → `:372 toast「版块已重命名」`；create 分支 `:374 POST section/create` → `:375 toast「版块已创建」`。
- `deleteSection` `BBSForum.vue:385-402`：`:387 confirmMsg` 守卫 → `:391 POST section/delete/{id}` → `:395 toast「版块已删除」`。
- `createTopic` `BBSForum.vue:607-624`：经 `createMutation` `:584-596 POST /api/bbs/subject/create`，`onSuccess:614-618 toast「帖子已发布」`。
- `submitReply` `BBSForum.vue:643-656`：经 `replyMutation` `:628-641 POST reply/create`，`onSuccess:648-651 toast「回复已发送」`。

死代码（模板无 `@click`，不判缺陷）：`toggleMod` `:788-818`（8 个 GET setCream 等 → `:812 toast「操作成功」`）、`loadForums` `:848`、`loadBbsViews` `:868`、`loadBbsEntities` `:894`。

对比组（已正确处理）：列表 `useQuery` 的 `topicsLoadError/repliesLoadError` 用 `useListError` `:536-541/:574-579`，模板 `:73/:164 ListLoadError+retry`；空态 `:74「暂无主题」`/:165「暂无回复」；`listRows` `:249-256` 兼容单层/双层信封。

## 3. 后端与 SDK 契约依据

- `backend/crates/bbs_assemble_control/src/lib.rs` `section_update:335-356`：`crud_save` 返回 `saved` bool，假 ID → HTTP 200 + `type:success, data.saved:false`。
- 同文件 `section_delete:359-395`：`deleted_rows>0`，假 ID → HTTP 200 + `type:success, data.deleted:false`。
- `backend/crates/shared/src/crud.rs:83-137 crud_save` 假 ID 返回 `false`、`:141-157 crud_delete`。
- `backend/crates/shared/src/response.rs:188-200` `ActionResult::error` 设 `type:error/size:-1/prompt=ExceptionEntityNotExist` 但 HTTP 200。
- `frontend/packages/sdk/src/api.ts:82-88` `parseJsonEnvelope` 仅 `JSON.parse` 不检查 `type`；`:275-296` 仅非 2xx 抛错。故 `200+type:error` 会被当作成功 resolve。
- `confirmMsg` 为自定义 DOM 弹窗 `#oa4-confirm-overlay`（`frontend/apps/desktop/src/utils/toast.ts:182+`），按钮文本「取消」「确认」。
- R363 修复范式参考：`frontend/apps/desktop/src/views/MeetingApp.vue:634-640` `meetingWriteSucceeded(resp,flagKey)` 判 `typeOk=resp.type===success||(type undefined&&success!==false)` 且 `flag===true`。

## 4. 真浏览器实测（1440x900 headless 非物理真机）

证据目录（gitignored）：`test-results/r375-pc-bbsforum-write/`；脚本 `r375-pc-bbsforum-write.cjs` 及 `-v2.cjs/-v3.cjs/-v4.cjs` + 对应 run.sh。全程 `0 pageerror`。

### 轮 1（假 ID 探针 + DOM 审计）
- DOM 77 个按钮，含真新建版块/发起主题/重命名/删除入口。
- 真后端假 ID 探针：`sectionSave POST=200 type:success data.saved:false`；`sectionDelete POST=200 type:success data.deleted:false`；`subjectCreateSpoof=200 type:success`（冒名 authorId 被后端以会话身份覆盖，属既有安全设计，非缺陷）。

### 轮 2（决定性对照，写请求走真后端不拦截）
- 注入 `section/list` 1 行「R375 注入版块」+ posts list 1 行；**写响应完全来自真后端**。
- 点重命名：真 `POST section/save/R375-INJECT-* → 200 type:success saved:false`，UI toast「✓版块已重命名」→ 误报。
- 删除取消守卫：点「取消」→ `writesBefore=3/writesAfter=3`，`zeroWrites=true`（弹窗文本含「确认删除版块」）。
- 删除确认：真 `POST section/delete/* → 200 type:success deleted:false`，UI toast「✓版块已删除」→ 误报。
- 发帖：`POST subject/create → 200 type:error`，UI toast「✓帖子已发布」→ 误报，`composeError` 为空。
- 截图：`bbs-v2-injected.png / bbs-v2-rename-toast.png / bbs-v2-delete-toast.png / bbs-v2-post-toast.png`。

### 轮 3（非 2xx / 空态 / 回复链）
- 非 2xx `saveSection 500`：`toasts=[]`，`sectionError=["HTTP 500: Internal Server Error"]` → 正确错误提示，无误报。
- 空态注入 `data:[]` → 「暂无主题可见」，与失败态可区分。
- 回复注入 `reply/create → 200 type:error` → toast「✓回复已发送」→ 误报（同型）。

### 轮 4（非 2xx delete 403 + 失败态补测）
- `deleteSection 403`：真点删除→确认→UI toast「✗没有权限执行该操作」→ **正确错误提示**（SDK 非 2xx 抛错进 catch）。
- 失败态补测：注入 `POST index/list 500` 后页面主题列表仍为「加载中...」、`retryButtons=0`，未观察到预期失败提示文案；**据实记录为取证限制**，不虚报通过。空态已可区分。

## 5. 三分判定

**真缺陷（1 个 P1）**：`saveSection / deleteSection / createTopic / submitReply` 等有真实 `@click` 入口的写函数在后端 HTTP 200 + `type:error` 或 `200+type:success+saved/deleted:false` 下无条件 `toast.success`，成功误报。最小修复方向对齐 R363 方案 B：新增 `writeSucceeded(resp,flagKey)`（判 `type=success` 且权威 bool 严格 `true`），各写函数 `await` 后先判再提示，否则 `toast.error` 透传 `message`。

**通过项**：非 2xx `save 500` 写 `sectionError` 不弹成功、`delete 403` 正确 toast.error、`confirmMsg` 取消守卫 0 写、空态可区分、0 pageerror。

**取证限制**：破坏性真状态翻转/真删未授权未执行（用假 ID + 只注入列表行走真后端替代）；全量写函数按代表性覆盖，未逐一 UI 触发；主题列表 500 失败态文案未坐实（见轮 4）。

**产品语义差异**：`ActionResult::error` 使用 HTTP 200 是 o2server 兼容既有设计（非本轮改动）；`subject/create` 冒名 authorId 被会话身份覆盖属既有安全设计。

## 6. 覆盖甄别

`docs/ops` 命中 38 处，均为视觉（R305）/建表修复（R302）/版块 forum 跨表（R317/R318）/删除孤儿（R320）/H5（R321/R322）等写链无关主题；从未专项甄别写操作链失败信封 UI 反馈，本轮首次专测为有效增量。
