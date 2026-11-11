---
title: "逐处处置完毕 = 逐处写下处置结论：45 处逐处量、10 处保持 500 并把契约写进文档"
date: 2026-11-11
last_refreshed: 2026-11-11
category: best-practices
module: augmentor/OPTIMIZATION_LOOP_2.md + augmentor/api/deps.py + docs/augmentor/API.md
problem_type: a_family_is_closed_only_when_every_site_has_a_written_verdict_not_just_a_code_change
component: close_a_family_by_writing_every_verdict
severity: medium
symptoms:
  - "一族「下游异常一刀切收敛成 500」的收尾分支散在六个路由文件里，改了一部分，剩下的没人知道是漏了还是量过"
  - "名册上的 45 处只有一处代码改动；没有逐处结论，下一轮会把它当成漏网之鱼再改一遍"
  - "契约只钉在代码里：客户端读不到「域判决 400 / 资源不存在 404 / 其余 500」这条三档语义"
  - "门禁红数从基线多出一条，追出来是账本文件里一枚被文本模式读写折掉的 CR，不是产品码的问题"
root_cause: a_family_is_closed_by_changing_code_but_not_by_writing_a_verdict_for_every_site
resolution_type: measure_every_site_then_write_the_verdict_into_a_roster_guard_and_the_public_doc
tags: [error-contract, api-contract, roster-guard, closure, ledger, byte-fidelity, fail-loud]
applies_when:
  - "一个待办格的措辞是「一族收尾分支形状不统一」，要判它收没收口"
  - "有些格子该收窄、有些格子该保持现状，而「保持」的理由只有实测能给出"
  - "账本文件里有特殊字节（孤立 CR），任何文本模式读写都会静默改掉它"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/augment.py
  - augmentor/api/routes/config.py
  - augmentor/api/routes/data.py
  - augmentor/api/routes/export.py
  - augmentor/api/routes/multimodal.py
  - augmentor/api/routes/privacy.py
  - augmentor/api/routes/dataset_tools.py
  - augmentor/api/routes/system_ops.py
  - augmentor/api/routes/language.py
  - augmentor/api/routes/vector.py
  - augmentor/tests/unit/test_error_routing_roster_l315.py
  - augmentor/OPTIMIZATION_LOOP_2.md
---

# 逐处处置完毕 = 逐处写下处置结论：45 处逐处量、10 处保持 500 并把契约写进文档

## 一、收口的判据不是「代码改了」，是「每一处都有结论」

A280 那一格写的是「8 个路由文件把下游异常一刀切收敛成 500」。前三轮（L311 / L313 / L314）
按**文件**收窄，改的是代码。L315 是最后一格，判据换了：**按**站点**逐处处置**，而且处置的
结论有两种——收窄，或者**保持现状并写下理由**。

两扇门口一共 45 处：

| 门口 | 处数 | 逐处实测的结论 | 处置 |
| --- | --- | --- | --- |
| 一刀切（`raise_internal_error`） | **10**：`augment.py` 2 / `config.py` 3 / `data.py` 2 / `export.py` 1 / `multimodal.py` 1 / `privacy.py` 1 | **每一处的下游都抛不出域判决**（域判决早已被 `HTTPException` 接住；写边 `OSError` 属服务端状态；那几支没有客户端入参；配置坏了是服务端自己的问题） | **保持 500 + 固定文案** |
| 分档（`to_http_error`） | **35**：`dataset_tools.py` 14 / `system_ops.py` 14 / `language.py` 2 / `vector.py` 2 / `config.py` 1 / `data.py` 2 | 那扇门**已经分档**：`FileNotFoundError` → 404、`JSONDecodeError` → 400（中文判决）、`ValueError` → 400 + 原文、其余 → 500 + 固定文案 | **保持分档** |

⇒ 本轮**零产品码改动**。这不是「没做完」，是**逐处量完之后的结论**。

## 二、为什么「保持 500」也算收口

收窄的前提是**下游有域判决可放行**。那 10 处的下游一句域判决都抛不出 ⇒ 把它们改成 400
只会给「本来该 500 的内部故障」多开一个 400 的口子。而 A280 记的那一格「资源不存在拿不到
404」在这一族**本来就不成立**：

- 读操作查存在性 ⇒ 404（`resolve_data_path`）
- 落点越界 ⇒ 403（`resolve_within_roots`）

⇒ 「保持」必须把**理由**写下来，否则下一轮读账本的人无法区分「量过，保持」与「漏了，没改」。

## 三、契约要写进文档，不只钉在代码里

`docs/augmentor/API.md` 新增「错误语义（三档）」一节：三档的表 + 三扇门口的逐处现状 +
**已知的一格**（泛档那一格归 A386 / A155①，不在 A280 那一行）。

点名「已知的一格」是关键：把不属于本行的东西显式指到它自己的账目上，下一轮才不会顺手把它
一起改掉。守卫里对应一支：那一段必须出现在文档里，且必须点名 A386。

## 四、守卫怎么写才有牙：名册面 + 判决面 + 契约面

1. **名册面**（AST）：一刀切那 10 处与分档那 35 处**逐文件**点名调用点数 ⇒ 谁再塞一处一刀切、
   或把某一处挪了地方，这张名册当场红。**名册是「这一族处置完毕」的机器可读形态**——
   它同时也是「逐处结论」的载体（点名即结论）。
2. **判决面**：一刀切那 10 处**逐处**注入一个内部异常 ⇒ 必须 500 + 固定文案，且**响应里不含
   内部原文**；分档那扇门的**四档**逐档点名。
3. **契约面**：文档里那一段必须在。

零 `parametrize`：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98` 把全仓
parametrize 三面冻成现量数 ⇒ 新守卫走具名用例，那两把尺子一格未动。

## 五、这一轮真正咬人的不是产品码，是账本文件的一枚 CR

L315 的全量门禁比基线**多一条红**：`test_scratch_pair_matches_its_ceiling`（工件对 1484 vs
常数 1485，**−1**）。逐层追下去：

1. `SCRATCH_PAIR` 那把尺子量的不是「死引用」，是**指向不入仓工件的引用数**（`scratch` +
   `scratch_missing` 两档之和）。
2. −1 的来源是账本卷二里少了一个 `` `index.json` `` 匹配——那枚引用被一枚 **CR** 顶开了
   （CR 插在 `index.json` 与闭引号之间 ⇒ 引用形状不再匹配 ⇒ 少收一条）。
3. 那枚 CR 是**本轮写入时误插的**：卷二账本里另有一枚**历史遗留**的孤立 CR（用来演示
   「每个 `\n` 都前接一个 `\n\r`」的那一句），本轮用文本模式读写把它**折成了 LF**，同时
   在另一行**插了一枚新的 CR**。

⇒ 两条纪律：

- **账本文件里有特殊字节时，一律二进制读写**（`io.open(p,'rb')` + `split(b'\n')` + `join` +
  `'wb'`），写完用独立脚本复核**孤立 CR 计数**。
- **门禁多一条红先查字节，再查代码**。本轮的红与产品码无关；若是产品码问题，那 45 处的
  判决面早就该红。

修法按**内容守恒**做，不是「把数字改对」：A332 行的 CR 去掉后，那一行与 HEAD **逐字节
相同**；HEAD 原有的那枚孤立 CR 按原字节复位（`` `\n\r` `` 而不是折成的 `` `\n\n` ``）。
⇒ 修完工件对回到 1485，`test_scratch_pair_matches_its_ceiling` 转绿，全量门禁回到基线。

## 六、泛档那一格归谁

分档那扇门把 `ValueError` **泛档**放行成 400 + **原文回传** ⇒ 内部异常也会被回成 400 判决、
且内部原文会发出去。那是 **A386**（`api/routes/vector.py::semantic_search` 是它点名的那一处）
与 **A155①**（泄漏面）的账。**A280 那一行记的是「一刀切收敛」这一族** ⇒ 本轮把那一族处置
完毕，那一格照旧归 A386 / A155①，守卫里按现量钉住（不是悄悄留着，也不是顺手改掉）。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L315 清单行与日志块；A280 行的 `~~S~~ 已收口（L315）`
- 守卫：`augmentor/tests/unit/test_error_routing_roster_l315.py`（零 parametrize，20 支）
- 上一轮：`docs/solutions/best-practices/measure-then-route-per-endpoint.md`（逐处先实测再决定
  放行口径：收窄 8 格、保持 10 格）
- 同族：`docs/solutions/best-practices/narrow-by-type-not-by-superclass.md`、
  `docs/solutions/best-practices/verdict-narrowing-and-failure-honesty.md`
