# R394 PC 极光推送写操作链失败信封甄别报告

- 轮次：R394（qa-e2e 只读甄别，不改码）
- 甄别对象：PC 端 `JPushApp.vue` 消息推送页建/存/删推送、绑/解绑设备、新版解绑、控制绑定、发送/测试发送、注册/删实体设备写链在后端 HTTP 200 业务失败信封下的 UI 反馈一致性
- 实测环境：Playwright Chromium headless，视口 1440x900，针对已部署测试服；**非物理真机**。测试域名/主机均为占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，真值不入库。
- git 基线：`bb9b70a702ecd98b69130ab63857254a7187f5e1`（HEAD 与上游一致，工作树干净）

## 一、任务与范围

甄别极光推送页写链是否存在与 R366/R368/R373/R381/R386/R390/R392 同型的缺陷：后端返回 HTTP 200 + type:error 业务失败时，前端 await 后无条件弹成功 toast，误导用户。判真缺陷需三全：真 UI 可复现 + 真请求响应 + 源码定位。破坏性真状态翻转写（真推送群发、真解绑、真删设备）未授权不执行，用假 ID + 写响应不拦截走真后端替代。

覆盖甄别：docs/ops 命中 jpush 相关历史文档均为端点可达性/视觉/插槽类审计，非失败信封 UI 反馈专项；同批次 QueryViewApp(R390)/QueryManagerDeep(R392) 口径可参照但不可套用结论。本轮首次专测该页失败信封 UI 反馈，有效增量。

## 二、源码甄别（真实入口 / 只读探测 / 门控三分）

文件相对路径 `frontend/apps/desktop/src/views/JPushApp.vue`；路由 `main.ts:146-149` path=jpush/name=JPushApp，页面标题「消息推送」。无 EndpointFold / devToolsEnabled 门控（与 R372/R388 门控型不同），写按钮在测试服生产构建真渲染：DOM 审计 21 个按钮，含建推送/存推送/删推送/绑设备/解绑设备/新版解绑/控制绑设备/发送消息/注册实体设备各命中，fold=0。

**裸成功提示且有真实 @click 入口的写路径（均 prompt 取参 + await 后无条件 toast.success，不读 type / 权威 bool；catch 仅覆盖非 2xx）**：
- jpushWrite（:172-218，:214「推送操作已提交」）10 个分支：jpushCreate(:174-177)、jpushSave(:178-180)、jpushDelete(:181-184，:183 confirmMsg 守卫)、deviceBind(:185-187)、deviceUnbindAll(:188-190，:189 confirmMsg)、deviceUnbind(:191-195，:194 confirmMsg)、deviceRegister(:196-204)、messageSend(:205-206)、messageTest(:207-208)、末分支实体设备删除(:210-212，:211 confirmMsg)
- jpushUnbindNew（:220-230，:226「设备已解绑」，GET device/unbind/new/{dn}/{dt}/{pt}）
- jpushCtrlWrite（:269-280，:276「控制操作已提交」，POST jpush/assemble/control/device/bind）
- sendWxTest（:247-264，:259-260 判 d.accepted/queued 否则「微信模板消息已受理」，POST mpweixin/menu/test/send/to/{person}）
- delDevice（:282-290，confirmMsg 守卫后 await DELETE core/entity/device/{id}，成功即本地 filter 删行，无成功 toast；catch 仅非 2xx）

只读探测：loadDevices(:107-119)、loadTemplates(:121-133)、loadPushConfig(:236-245)、loadJpushEntities(:138) 等为 load/read 类，不判缺陷。模板 :19-57 的 Tab 切换（设备管理/推送模板/实体明细）与查询动作属只读。

后端 ironclad：`jpush_assemble_control/src/lib.rs` save_jpush(:384-420，属主门:392 与 0 行:408 均 200+type:error「jpush not found」)、delete_jpush(:424-455，:431/:445 200+error)、device_unbind_new(:652-702，:674/:689 200+error「device not found or already unbound」)、device_unbind(:705-734，:722 200+error)、device_unbind_all(:460-501，:480 200+error「personId is required」)等失败统一 ActionResult::error → HTTP 200 + type:error + message；成功 success(json) 无权威 bool。

SDK 铁证：frontend/packages/sdk/src/api.ts:82-88 parseJsonEnvelope 仅 JSON.parse 不查 type；:275-296 仅 !resp.ok 抛错（非 2xx 进 catch），2xx + type:error 原样 resolve → 上述函数落入无条件成功路径。

## 三、真后端信封实测（假 ID / 缺参探针，不注入，不群发）

在测试服真浏览器内 fetch 直打（API base `/oa4rust-api`），6 个端点信封形状全部坐实：
- 存推送假 ID → 200 + type:error「jpush not found」
- 删推送假 ID → 200 + type:error「jpush not found or already deleted」
- 新版解绑假三元组 → 200 + type:error「device not found or already unbound」
- 旧版解绑假二元组 → 200 + type:error「device not found or already unbound」
- 解绑全部缺 personId → 200 + type:error「personId is required」
- 建推送空标题/内容 → 200 + type:success + data.id（产品语义差异，见第五节）

发送消息/测试发送/微信测试发送属真群发端点，未做假 ID 探测（避免真实推送副作用），据实记为取证限制。shared/response.rs ActionResult::error 用 HTTP 200 是 o2server 兼容既有设计（非本轮改）。

## 四、E2E 决定性对照（写响应不拦截，走真后端）

1440x900 headless，5 个真实 UI 点击 + prompt 填假 ID，写响应完全走真后端不拦截，逐个捕获真请求响应体：
1. 「存推送」填 R394-NOSUCH-ID → 真 POST /api/jpush/save/假 = 200 type:error「jpush not found」→ UI 弹「✓推送操作已提交」（误报）
2. 「删推送」填假 ID + confirm 确认 → 真 POST /api/jpush/delete/假 = 200 type:error → UI 弹「✓推送操作已提交」（误报）
3. 「新版解绑设备」填假三元组 → 真 GET device/unbind/new/假 = 200 type:error「device not found or already unbound」→ UI 弹「✓设备已解绑」（误报）
4. 「解绑设备」填假二元组 + confirm 确认 → 真 DELETE device/unbind/假 = 200 type:error → UI 弹「✓推送操作已提交」（误报）
5. 「控制绑设备」填假设备名 → 真 POST assemble/control/device/bind = 200 type:success data:{}（成功响应，不误报，仅证入口可达）

每次 writesDelta=1，newWrites 响应体为铁证；截图 jp-decisive-save.png / jp-decisive-unbind-new.png / jp-decisive-unbind.png 目视确认绿 toast 与 200+error 并存（已排除人造注入伪造质疑）。

## 五、通过项 / 取证限制 / 产品语义差异

通过项：confirm 取消守卫（删推送填值后点取消）writesDelta=0、无 toast、零写；非 2xx 注入 500→「✗推送操作失败: HTTP 500: Internal Server Error」、403→「✗推送操作失败: 没有权限执行该操作」正确 toast.error；设备列表 500 → 「设备列表加载失败: HTTP 500: Internal Server Error」+重试按钮 1；空态「暂无设备」可见；0 pageerror；totalWrites=13。

取证限制：
- 破坏性真状态翻转写（真推送群发、真解绑、真删设备、注册真设备）未授权未执行，用假 ID + 不拦截真后端写响应替代。
- S1 建推送空标题/内容探针（jpushCreateEmpty）真实创建 1 条空推送配置记录（空 title/content/target，后端 type:success + data.id）——为探针副作用，非破坏性写但确属真实新增；无逆向删除权限，据实记录不隐瞒。该条数据仅影响本次测试服设备统计计数，不入库。
- 发送消息/测试发送/微信测试发送为真群发端点，未做假 ID 探测；sendWxTest 的 :259-260 判 accepted/queued 否则「已受理」、未实测其失败信封形状，按源码隐患记录不判真缺陷（三全缺真请求响应）。
- 全量 12+ 写按钮按代表性覆盖（4 个误报决定性 UI 点击 + 1 个成功入口 + 6 个探针 + 源码行号），未逐一 UI 触发。

产品语义差异：建推送空标题/内容返回 200+type:success+data.id（后端不拒空字段），响应为成功故不构成误报前提；控制绑设备假名返回 200+type:success+data:{}（INSERT 无 0 行失败口径），响应成功不构成误报。二者据实记录不虚报缺陷。

## 六、三分判定与最小修复方向

判定：**1 个 P1 真缺陷**——存推送/删推送/新版解绑/旧版解绑四个有真实 UI 入口的写路径，在后端 HTTP 200 + type:error 业务失败下无条件弹成功 toast，三全满足（真 UI 可复现 + 真请求响应 + 源码定位）。同一裸成功模式的 jpushWrite 其余分支、jpushCtrlWrite、sendWxTest（仅源码隐患未实测）、delDevice（删除失败仍本地删行致状态矛盾）列为同型隐患，一并纳入修复范围。

最小修复方向（对齐 R363 方案 B，本轮不改码）：新增 jpushWriteSucceeded(resp, flagKey?) 判 type 为 success（或 type 缺省且权威 bool 为 true）为主；各写函数 await 后先判响应再提示，失败时 toast.error 透传 resp.message，不再无条件 toast.success；delDevice 仅后端成功后再本地 filter 删行。

## 七、交付证据

- 文档：`docs/ops/oa4rust-r394-pc-jpush-write-audit.md`（本文件），脱敏占位 0 真值、相对路径、非物理真机标注。
- E2E 脚本与证据（gitignored）：`test-results/r394-pc-jpush-write.cjs`、`test-results/r394-pc-jpush-write/evidence.json` 与截图 jp-dom-audit / jp-probe-done / jp-decisive-save / jp-decisive-delete / jp-decisive-unbind-new / jp-decisive-unbind / jp-decisive-ctrl-bind / jp-cancel-guard / jp-http500 / jp-http403 / jp-fail-state / jp-empty-state.png。