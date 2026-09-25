# G5 残余缺口分诊报告（2026-09-25，rev470 口径）

**目标**：「全面消费后端能力至 100%」。
**现状**：域口径消费 **3265/4072 = 80.2%**（`consumption_gap.py`，桶分母恒定 4072）；全局口径约 3265/4638 ≈ 70.4%。
**结论（fail-loud）**：在现行 G5 防造假口径下（同逻辑操作不双计、凭证流不接、运行时 500/501 假成功不算消费、
matcher 误配不计），**80.2% 是诚实上限**。残余 807 条逐条归类如下；冲 100% 需要用户裁定放宽口径（见 §4）。

## 1. 方法

- reconcile 三件套（`extract_calls.py` → `compare.py` 重写 `api_reconcile.json` → `compare.py --gate`）+ `consumption_gap.py`，
  每 rev 跑，gate（shadow/405/404）全程 PASS。
- 强门候选扫描（`_candg4.py`/`_candg5.py`）：on-metric + 非 mock + 非 `{path*}` + arity 一致（URL `{param}` 数 == handler
  `Path<(...)>` 元数）+ handler 体纯读判据 + capability_unavailable 排除 + 跨 crate collision。
- 分诊器（`_triage.py` + 本报告脚本）：同路径方法孪生、同 handler 异路径、arity-trap、凭证/占位关键字、/object 投影孪生、
  multipart/二进制流、WS upgrade 逐类判定。
- 每条拟接线路由先读 handler 源码（防造假规则①），再核 matcher 命中归位（防误配，数字字面/终端字面段技巧），
  最后 tsc + autoquery-guards + desktop-endpoints 守卫。

## 2. 残余 807 条逐类计数（3265 口径）

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

## 4. 冲 100% 所需的口径裁定（需用户决定）

807 条中可机械接线的只剩「同 handler 异路径镜像 221 + 同路径方法孪生 199」≈ 420 条，但接线即违反
防造假规则②（同一逻辑操作双计）——指标上涨但集成面并未新增任何真实能力。其余 387 条（arity-trap 168、
凭证/占位 115、/object 46、multipart 29、写残留 20、501 6、WS 3）接了要么是运行时假成功，要么危险/无意义。

选项：
1. **维持 80.2% 诚实口径（现状）**——807 条按上表存档为不可诚实消费，G5 收官。
2. **放宽双计口径**：接受「每注册路由各计一次」（同 handler 镜像/方法孪生也逐条接线）→ 可再 +~420 至 ~90.5%；
   再放宽 arity-trap/凭证占位（接桩即 500/假登录）可名义 100%，但指标与真实集成面脱钩。
3. 折中：只接「同 handler 异路径」中语义确有独立价值者（如匿名/登录双轨、u2/u3 双轨各算一轨）——需逐条人工裁定，
   量级约几十条，收益有限且需重开逐条判例。

## 5. 复现命令

```bash
cd docs/audits/three-ends-2026-09-20
python extract_calls.py && python compare.py && python compare.py --gate && python consumption_gap.py
python _triage.py        # 读候选分诊（no-twin/has-twin）
```
scratch 分析件：`_candg2~4.py`（强门扫描）、`_candg5.py`（去黑名单读候选）、`_triage.py`（孪生分诊）、`_gapcats.json`。
