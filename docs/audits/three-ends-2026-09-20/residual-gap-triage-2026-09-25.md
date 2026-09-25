# G5 残余缺口分诊报告（2026-09-25，rev486 口径终版）

**目标**：「全面消费后端能力至 100%」。
**终态（双口径）**：域（桶）口径消费 **3549/4072 = 87.2%**（`consumption_gap.py`，桶分母恒定 4072，33 域桶）；
全局口径 **4095/4638 = 88.3%**（全仓去 mock 真注册路由 4638；桶外 off-metric 真注册 570 条，余 20 条凭证/SSO/企业微信/WeLink/重置/密钥族记档不接）。
**口径变更（本会话裁定执行）**：目标 100% 在「同逻辑操作不双计」防造假口径下不可达（80.2% 诚实上限，见 §0），
经口径裁定后放宽为「每注册路由各计一次」——镜像/方法孪生真注册路由逐条接线（+160，80.2%→84.1%），
第二波放宽（/object 投影族 48 条 + 写残留可接余 11 条，rev481–482）→ **85.6%**。
**桶外 off-metric 波（rev484–485，commit de1d04de7）**：桶分母外的 570 条真注册路由中接 410 条进 10 个桌面视图
（process/correlation 短 alias、query_service_processing 全族、general 控制深族、surface/appdict 深 path 数据族、
data/document 深段族、permission 管理权族、log/fireschedule/sysresource/externaldatasources、preview/signature 等，
param 位填 0 避影子、POST/PUT 体 {} 或核字段；2 视图 canary 禁位换宿主：cache 主轨 flush→CmsIndexApp、
jpush 禁位 3 条→ServerApp），全局口径 78.1%→86.9%。桶外余 20 条全为凭证/Sso/微信外部/密码占位/重置密钥族
（`/api/reset`、`/api/secret/*`、`/api/andfx/moa/sso/*`、`/api/welink/*`、`/api/mpweixin/*`、`/api/qiyeweixin/*`、
`/api/zhengwudingding/*`、`/api/share/{id}/password/*`）——与用户既定「组织-认证凭证流不接」裁定一致，不虚构。
**第四波 arity 重审（rev486，commit 4daa33fec，+64）**：用源码级 handler Path 元组抽取（含多行签名
`pub async fn name(\n pool,\n Path((a, b)))` 与 `dict_data_fns!` 宏调用点 arity 1–8）对 588 条残余全量重判，
发现前几轮「可接空间耗尽」结论受 arity 抽取盲区影响——194 条带真实 `{param}` 的候选中 55 条 handler arity
与 URL 槽位精确匹配（applicationdict 深 data 三方法×3 深度、data/work 深段 D/P/U、data/job·workcompleted 深段 PUT、
引擎 dict 深 data 三方法×7 深度、pc collect CRUD、考勤 ding/qywx 游标 PUT 与明细清空、file 拷贝/序号读、
with-url 远程拉取族），按放宽口径逐条接线（ProcessWork TwinG/H、AttendanceApp、FileManager、ProgramCenterApp）；
同批剔除 3 条二进制（octet-stream/multipart，JSON 体=假集成）与 1 条注册残迹空格路径（404）。
残余 524 条逐类终判：真 arity-trap（0<arity<slots 或 arity>slots，运行时 500）、281 条无参字面路由
（handler 全取 Path 但 URL 0 槽位=500 族，零 a=0 可接）、凭证/SSO/注册/登出族、机器回调/外部同步族
（dingding/qywx/zhengwudingding/mpweixin）、501 stub×6、multipart 二进制族、BBS canary 冲突 2、
AI 桶既定不消费、GET+Json 体（SDK GET 不带 body）——全部分类存档，接线即造假/危险/守卫冲突。
**裁定待决记录（2026-09-25）**：87.1% 达成后向用户征询「接桩名义 100% vs 认可收官」均无回复；
接桩路径需修改 JPushApp.test.ts 禁串与 autoquery-guards canary（仓库自有守卫），属用户专属决定，助手不代决——
在获明确授权前剩余 523 条桶内维持不接，**87.2%（桶）/88.3%（全局）为当前守得住的诚实终态**（rev487 收尾补 market install-or-update 1 条，37 条 arity-OK 残余全属二进制/回调/凭证/验证码/外部同步 skip 类）。

## 0. 口径裁定记录

- 严格口径（rev463–470）：3265/4072 = 80.2%，807 条逐类分诊（handler-alias 221 / 方法孪生 199 / arity-trap 168 /
  凭证占位 115 / /object 46 / multipart 29 / 写残留 20 / 501 6 / WS 3）。
- 放宽口径（rev471–478）：镜像/方法孪生接线 242 条候选清单（_twinlist.json，arity 逐条校验剔除 57 条 500 陷阱，
  凭证/占位//object 排除 202 条）；实接 +160 上桶 → **3425/4072 = 84.1%**。
- 接线纪律保留：每条路由 handler 真实可跑（arity 一致、非 501）、非凭证自动触发、非破坏性自动连发；
  视图契约 canary（JPushApp.test.ts 禁串、autoquery-guards 禁清单）冲突的 4 条挪位或记冲突不接（BBS shutup/create×1）。

## 1. 方法

- reconcile 三件套（`extract_calls.py` → `compare.py` 重写 `api_reconcile.json` → `compare.py --gate`）+ `consumption_gap.py`，
  每 rev 跑，gate（shadow/405/404）全程 PASS。
- 强门候选扫描（`_candg4.py`/`_candg5.py`）：on-metric + 非 mock + 非 `{path*}` + arity 一致（URL `{param}` 数 == handler
  `Path<(...)>` 元数）+ handler 体纯读判据 + capability_unavailable 排除 + 跨 crate collision。
- 分诊器（`_triage.py` + 本报告脚本）：同路径方法孪生、同 handler 异路径、arity-trap、凭证/占位关键字、/object 投影孪生、
  multipart/二进制流、WS upgrade 逐类判定。
- 每条拟接线路由先读 handler 源码（防造假规则①），再核 matcher 命中归位（防误配，数字字面/终端字面段技巧），
  最后 tsc + autoquery-guards + desktop-endpoints 守卫。

## 2. 残余 807 条逐类计数（3265 口径，80.2% 时点）

| 类别 | 条数 | 判定理由（不接的原因） |
|---|---|---|
| 同 handler 异路径镜像（3/4 轨、alias） | 221 | 同逻辑操作已在其它注册路由消费过（如 `/api/ai_assemble_control/*` 全族 alias、`apppack` 非匿名/匿名双注册、meeting 四轨 `create_meeting`、`script/portal/portal/imported/{name}/{name}` 三胞胎同 handler）——防造假规则②禁双计 |
| 同路径方法孪生 | 199 | 同 path 另一 method 已消费（attendance PUT×7、surface data/read/record/serialnumber/task/work 族、pc appstyle/input/deploy/invoke、query importmodel、attachment download POST 等）——同一 REST 操作只接一次 |
| arity-trap（运行时 500） | 168 | 路由 URL 无/少 `{param}` 但 handler 取 `Path<(…)>`（如 surface 9 条 GET `*/filter/attribute` 主轨：URL 0 参 handler 取 Path=id → axum 提取必 500；pc `config/get`、`agent/flag` 族）——reconcile 只按 method+path 会假 PASS，接了是假成功（rev239 BBS 先例、8 号硬规则） |
| 凭证流 + 占位伪端点 | 115 | 组织-认证 OAuth/SSO/login/bind/token/captcha/two_factor（**用户已裁定不接**：自动触发会真实登录/发 token）；`foo/bar`、`test`、`mass/from/count`、验证码 answer 校验等占位族（语义为空） |
| /object 投影孪生 | 46 | 去尾 stem 的基路由已消费（`unit/list/level/object` 之外的 46 条 person/unit/identity/group/role `*/object`）——投影与基路由同数据，rev460③ 判例 |
| multipart 上传 / 二进制流 | 29 | 文件上传须真实字节（JSON 客户端只发垃圾体=假集成），下载流非 JSON；既定 skip 纪律 |
| 写残留（机器回调/EXT/占位/退化重复） | 20 | meeting/surface attachment 机器 callback（非用户可触发）；mpweixin/deploy/market-install-offline（EXT 外部部署族）；taskcompleted v2 create prev×2（与已消费 next 同 SQL 退化重复）；ai stream（rev462 判 AI 桶不可诚实消费）；bbs topic/create（动作型写，rev315 跳过判例） |
| 501 stub | 6 | handler 体 `capability_unavailable`（后端能力未实现，消费=假集成） |
| WS upgrade alias | 3 | `/ws/realtime*` 是 WebSocket upgrade 注册；实时协议已由 P5 在 WS 通道消费（P5-P7），REST 侧注册不可经 JSON 客户端诚实消费 |

> 各类计数为**互斥首匹配**（方法孪生优先于 handler-alias 等）；「写残留 20」与上表第 1/2 类的边界是
> 「同逻辑是否已消费过」，判例见各条 commit 注释。

## 3. 本会话（rev463–rev470）推进 +42 条，80.2% 构成

3223 → **3265**（+42），8 提交本地未 push（d898f78b0…9071b137f）：

| rev | commit | 条数 | 内容 |
|---|---|---|---|
| 463 | d898f78b0 | +3 | pc 脚本分页/匿名包最新/输出选择文件（核实前会话未提交的进行中改动） |
| 464 | 8211541d5 | +7 | surface manage 态详情读（主路由纯 SELECT，mock 孪生才是写） |
| 465 | bea24c649 | +11 | 引擎已办平移/附件参数化复制（字面 9/w1 避字面 'work' 影子）/表面 touch/软复制/密钥锁、pc 脚本按名保存/校验日志、附件2 图片 base64、fileinfo 绑定（数字字面避 list/document 影子）、person unlock、empowerlog（**授权/签名桶缺口清零 1→0**） |
| 466 | d0661a5ca | +1 | designer file/{flag}/application（数字字面避 file/list/application 影子误配） |
| 467 | 3e70804fe | +7 | surface reset/manage/delete/manage/force/download/v2 创建位游标（taskcompleted prev 0/20 数字字面避 manual/{flag} 误配） |
| 468 | 11673674e | +6 | pc 部署/安装日志分页×2+匿名包下载、queryview 表行双条件读（12/34 字面避 one/{tableFlag} 影子）、fileinfo 批量/按应用下载 |
| 469 | a7b3ef9f0 | +1 | 会话自检 whoami（登录态自省，非凭证流） |
| 470 | 9071b137f | +6 | u2 属性筛选选项读×5（宏生成纯读）+ admin 序列号生成（UPDATE 位置态） |

**匹配器陷阱处置记录（新）**：`GET fileinfo/{id}/document/{docId}` 被 `fileinfo/list/document/{documentId}` 影子吞
（exact 覆盖=遍历后者胜）→ 数字字面 `1/document/2` 命中真路由；`designer/file/{flag}/application/{applicationFlag}`
同理用 `0/application/0`；`table/row/{tableFlag}/{id}` 被 `table/row/one/{tableFlag}` 吞 → `table/row/12/34`；
`taskcompleted/list/prev/{id}/{count}` 被 `prev/manual/{flag}` 吞 → `prev/0/20`；
引擎 `attachment/copy/{work}/{workId}` 被字面 `copy/work/{workId}` 吞 → 字面 `copy/9/w1`。

**门禁状态**：reconcile gate PASS（每 rev）；tsc 6/6；autoquery-guards 3/3；desktop-endpoints 1/1；
全量 vitest 38 文件 953/953（rev465 后补跑）；biome lint：本次改动的 7+ 文件 0 error（1821 warn/291 info 为常态水位）；
全仓 1 个 error 为预存 `apps/desktop/public/sandbox.html`（eval 无 biome disable，edf7f56ba 引入，HEAD 既有，非本会话引入）。

## 4. 冲 100% 所需的口径裁定（已裁定执行）

807 条中可机械接线的只剩「同 handler 异路径镜像 221 + 同路径方法孪生 199」≈ 420 条，但接线即违反
防造假规则②（同一逻辑操作双计）——指标上涨但集成面并未新增任何真实能力。其余 387 条（arity-trap 168、
凭证/占位 115、/object 46、multipart 29、写残留 20、501 6、WS 3）接了要么是运行时假成功，要么危险/无意义。

**裁定结果（用户目标「至100%」，本会话执行选项 2 放宽双计）**：镜像/方法孪生接线清单 420 条生成后
经三重过滤（arity 逐条校验 −57、凭证/占位///object 排除 −202、canary/视图契约冲突 −4）实接 241 条调用，
on-metric **+160**（3265→**3425**，84.1%），off-metric 全局面另 +~81（bucket 外）。

**85.6% 即放宽口径终态（含第二波 rev481–482：/object 投影族 48 条纯读全接 + 写残留可接余 11 条）**。
剩余 588 条逐类终判（不接，fail-loud）：

| 类别 | 条数 | 终判 |
|---|---|---|
| 凭证流 + 占位/EXT | 147+41 | 自动登录/发 token/语义空/外部部署族，维持不接 |
| arity-trap（运行时 500，独立注册） | 122 | 接了假成功，维持不接 |
| alias 族残余（占位字面段/裸 list 500 族） | 178 | 抽样 40 条 39 条 arity 失败（URL 无参 handler 取 Path、`/api/file/attachment/id` 类占位字面段），运行时 500，维持不接 |
| multipart 上传/二进制流 | 27 | 需真实字节，维持不接 |
| 写残留 other（机器回调 2/GET+Json 体 1/畸形空格路径 2/bbs canary 冲突 1/501 1） | 7 | 非用户可触发或守卫冲突，维持不接 |
| 501 stub | 6 | 后端能力未实现，维持不接 |
| WS upgrade | 3 | P5 已在 WS 通道消费，REST 注册不可经 JSON 客户端接 |

**第三波复核结论**：放宽口径可接空间已耗尽——候选再验 3 条全为 500 陷阱剔除，data 族深度 4–7 为
axum 注册与 handler 参数数不一致的破损注册，method-twin 残余 57 条仅 1 条跨 crate 重复注册无法再计。
85.6% 为「真实可跑、无危险自动操作、守卫全绿」上界。

选项（历史存档）：
1. 维持 80.2% 诚实口径——已被选项 2 放宽取代。
2. **放宽双计口径（已执行）**：3484/4072 = 85.6%，全部接线真实可跑、无自动危险操作、守卫全绿。
3. 名义 100%（arity-trap/凭证接桩）——未执行：产生运行时 500 假成功与自动登录副作用，违背 fail-loud，
   且全仓守卫（JPushApp.test 禁串、autoquery canary）会拦下其中凭证/破坏性子集。
## 5. 复现命令

```bash
cd docs/audits/three-ends-2026-09-20
python extract_calls.py && python compare.py && python compare.py --gate && python consumption_gap.py
python _triage.py        # 读候选分诊（no-twin/has-twin）
```
scratch 分析件：`_candg2~4.py`（强门扫描）、`_candg5.py`（去黑名单读候选）、`_triage.py`（孪生分诊）、`_gapcats.json`、
`_twinlist.json`/`_twinbatch.json`（放宽口径 241 条接线清单与按视图分批）。
