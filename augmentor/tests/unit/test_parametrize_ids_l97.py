# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L97：A177 的普查与判据 —— parametrize 的默认 id 不许随解释器劈叉

**为什么这一支该住在仓库里**：pytest 的参数化 id 默认由**参数对象现造**，而对象的渲染
随解释器变。L96 收尾做名册对账时实测撞出：3.14 给 PEP 604 联合（`int | str`）挂了
`__name__` 等于 `Union`，3.13 没有这个属性 ⇒ 同一条 `test_label` 在两侧长成两个节点名，
于是「两侧新增集合相同」这条对账判据被一次解释器差异无声劈开。L96 就地修了撞出的那一支
（七档显式命名 id）并写下「**下一轮第一件事：先普查再定修法**」。本轮先兑现普查，再把修法
冻成判据。

普查读数（由本文件的尺子在终态树上现量，四格都被 `test_census_shape_is_pinned` 钉住）：
  - 全仓 `parametrize(...)` 调用 **355** 支；
  - 可判面 **262** 支：值写成字面 list ⇒ 能静态看到喂了什么对象；
  - 盲面 **93** 支：值来自变量 / 生成器 / 函数调用 ⇒ 本尺子判不出，**记成盲而不是记成安全**
    （A171 那一族的第三种形状：尺子看不见的那一格最容易读成「干净」）；
  - 可判面里值位含类型对象 / 联合 / 命名空间属性的 **1** 支，正是 L96 已显式命名 id 的
    `test_schema_type_spec_l96.py` 那一支 ⇒ **违规集为空**。

判据口径沿用 L96 立行时写下的候选：「凡参数含类型对象或联合就必须显式 `ids`」。它比「只认
联合」严一档（裸 `int` 当值的 id 其实两侧一致），严出来的代价是后来人多写一个 `ids=`，
换来的是不必再逐对象论证「这个类型的渲染稳不稳定」。
"""

import ast
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

from tests.unit.test_measurement_rulers_l94 import RulerNotProven

TESTS_ROOT = Path(__file__).resolve().parent.parent
AI_ROOT = TESTS_ROOT.parent

#: 作为**值**喂进 parametrize 的内置类型名。判定看的是「名字出现在值位」，
#: 所以 `float("nan")` 里那个被调用的 `float` 不算（v1 探针正是栽在这里，见下面自证样本）。
TYPE_VALUE_NAMES = frozenset({
    "int", "str", "float", "bool", "bytes", "bytearray", "complex",
    "list", "dict", "tuple", "set", "frozenset", "object", "type", "memoryview", "range",
})

#: 命名空间前缀：`typing.Any` / `types.FunctionType` 这类值同样由对象现造 id。
NS_PREFIXES = frozenset({
    "typing", "types", "collections", "datetime", "decimal", "enum",
    "numbers", "abc", "dataclasses", "functools", "itertools", "re",
})

# 现量普查读数（改动全仓 parametrize 形状就要重跑本文件并改这里，不许照抄上一轮）
# **收官后 A184 入库回填（总数 +1、可读 +1，盲面一格未动）**：新守卫里那支按桶名参数化的用例
# 是「argvalues 全是字符串字面量」的可读形状 ⇒ 只抬两格读数，93 格盲面与 L98 的六档分布一字未动。
# 这里刻意不用反引号点名那支新守卫文件：本文件的注释也在 A184 普查的扫描面里，多写一条断言形状
# 就要多改一处账（A141「写账本身还税」那一族的第 N 次复现，本轮选择少欠一笔）。
# **第二本账 L114 回填（总数 +1、可读 +1，盲面不动）**：test_export_enhanced 新增一支
# TestConversationEmptyFieldBranches 的对话式格式空字段假支参数化，桶=plain（可读）⇒
# MEASURED_CALLS 356→357、MEASURED_READABLE 263→264，MEASURED_BLIND 93 与 L98 六档分布一字未动。
#: 第二本账 L151 回填（总数 +2、可读 +2，盲面不动）：test_quality_dedup_gates_l76 新增两支
#: QualityScorer 阈值参数化（坏值 7 档 / 合法 4 档，全字面量列表）⇒ CALLS 357→359、
#: READABLE 264→266，BLIND 93 一格未动。
#: 第二本账 L154 回填（总数 +18、盲 +8）：test_config_gates_l154 新增 18 处
#: parametrize（11 节坏值 × 四面 + 独立性 19 档 + 加载面 7 档）；值位含 list 对象的 8 处
#: 未自带 ids ⇒ 盲面 93 → **101**，CALLS 359 → **377**；READABLE 是差值口径
#: （总数 − 盲面）⇒ 266 → **276**。
#: L163（B233）：新增导出原子写守卫文件，两支 parametrize（字面格式清单，
#: 自带 literal-list）⇒ CALLS 377 → 379、READABLE 276 → 278，盲面（no-literal-list
#: 桶）101 一格未动。
#: **L204 回填（CALLS +2、盲面 +2、READABLE 一格未动）**：test_readme_cli_index_l204.py
#: 新增两处 parametrize，值位是**模块级名字引用** `PROMISED` 而不是内联字面量 ⇒
#: l97 的 no-literal-list 桶 +2、l98 的 plain 档同步 +2，CALLS 393 → 395，
#: READABLE = CALLS − 盲面 ⇒ **287 一格未动**（第一版按「字面量 ⇒ 可读」想当然写成
#: READABLE +2，实测当场红，已订正）。
#: L210 回填（CALLS +1、READABLE +1、盲面未动）：test_api_doc_endpoint_coverage_l210.py 新增一处
#: parametrize，值位是内联字面量列表 ["dataset", "system"] ⇒ 入可读档，盲面 108 一格未动。
#: **L217 回填（CALLS +3、盲面 +3、READABLE 一格未动）**：test_config_reader_census_l217.py
#: 三处 parametrize（正控制两支值位 `_MUST_BE_DIRECT` / `_MUST_BE_ALIAS_ONLY`，
#: 引用验真一支值位 `_LOADER_CONSUMED`），全都不带 ids ⇒ no-literal-list 桶 +3；
#: READABLE 是差值口径（总数 − 盲面），两边同增 ⇒ **288 未动**。
#: **L222 回填（CALLS +3、盲面 +3、READABLE 一格未动）**：test_config_three_side_l222.py
#: 三处无 ids parametrize（两支逐节判据值位 `sorted(SCHEMA)`，例外名单验真一支值位
#: `sorted(META_ROWS)`）⇒ no-literal-list 桶 +3；三格全落盲面，差值口径下 **288 未动**。
#: **L226 回填（CALLS +2、READABLE +2、盲面一格未动）**：`test_wired_dropdown_choices_l226.py`
#: 两支 parametrize（5 个检索方法 / 3 个验证预设，值位都是内联**字符串**字面清单，
#: pytest 直接把它们渲染成 `[exact]`、`[basic]` 这样的可读 id）⇒ 两格全落可读档，
#: 实测 CALLS 402 → **404**、READABLE 288 → **290**，l98 侧档位和一字未动。
#: L227：接线守卫一支 parametrize（两个 state 名，内联**字符串**字面清单）⇒
#: CALLS 404 → **405**、READABLE 290 → **291**，盲面一格未动。
#: L228：接线守卫一支 parametrize（两个 state 名，内联**字符串**字面清单，与 L227 同形）⇒
#: CALLS 405 → **406**、READABLE 291 → **292**，盲面一格未动。
#: **L229 回填（CALLS +5、盲面 +4、READABLE +1）**：`test_transform_wiring_l229.py` 五支
#: parametrize 实测分两族——一支（绝对路径 / `..` 那支）值位是内联字符串字面清单 ⇒ 入可读档；
#: 其余四支的值位都是「从两侧权威现造出来的东西」（`OUTPUT_FORMAT_CHOICES` 的引用、
#: 契约默认值的逐格表、响应模型的字段名册、TS 接口的键名册），静态读不出元素 ⇒
#: no-literal-list 桶 +4。实测 CALLS 406 → **411**、盲面 114 → **118**，
#: READABLE 是差值口径 ⇒ **292 → 293**。
#: **L230 回填（CALLS +8、READABLE +8、盲面一格未动）**：`test_sample_aggregate_rag_wiring_l230.py`
#: 八支 parametrize 的值位**全部是内联字面清单**（三张下拉各自的档、两条 ratio 臂、四策略、
#: 三只端点的路径判决、三张回执面），静态读得出元素 ⇒ 八格全落可读档。实测 CALLS 411 →
#: **419**、盲面仍 **118**、READABLE 293 → **301**。这一批刻意不引后端那份现造清单当值位
#: （L229 那四支就是这么进盲面的）：清单同源由 `dispatch_arms()` 读 AST 单独钉，不借 parametrize。
#: **L232 回填（CALLS +2、READABLE +2、盲面一格未动）**：`test_lineage_wiring_l232.py` 两支
#: parametrize（`input_file` 四档坏值 / `registry_path` 两档坏值），值位都是内联字符串字面清单
#: ⇒ 用本文件那把尺子现量两处都落 `plain` 桶（不在 `no-literal-list`），实测 CALLS 419 →
#: **421**、READABLE 301 → **303**、盲面仍 **118**。读数出自仓外探针 `l232_census.py`
#: （直接 `import` 本模块的 `param_sites(TESTS)` 逐处打印桶归属），不是按「字面量 ⇒ 可读」推的。
#: **L238 回填（CALLS +5、READABLE +5、盲面一格未动）**：`test_decision_gates_l238.py` 四支
#: parametrize（容差三档放行 / 两档拒 / `text_field` 四档两面同文 / 逐项界两档两面同句）与
#: `test_health_score.py` 同轮补的一支（`weights, idx` 四档，点名被拒下标）值位全是内联字面清单
#: ⇒ 五格全落可读档。实测 CALLS 421 → **426**、READABLE 303 → **308**、盲面仍 **118**，
#: l98 侧四档和（58+11+38+11=118）一字未动；读数出自仓外探针 `l238_census.py`
#: （`static_rows(TESTS_ROOT)` 逐处打印 `literal` 归属），不是按「字面量 ⇒ 可读」推的。
#: **L240 回填（CALLS +3、READABLE +3、盲面一格未动）**：`test_leakage_dense_route_l240.py` 三支
#: parametrize（判据零/负侧五档 / 阈值与示例数边角三档 / 两条路线各一支）值位全是内联字面清单
#: ⇒ 现量三格都落 `plain` 桶。实测 CALLS 426 → **429**、READABLE 308 → **311**、盲面仍 **118**，
#: l98 侧四档和一字未动；读数出自仓外探针（直接 `import` 本模块的 `param_sites(TESTS_ROOT)`
#: 逐处打印桶归属：303/346/455 三行全 `plain`），不是按「字面量 ⇒ 可读」推的。
#: **L241 回填（CALLS +1、READABLE +1、盲面一格未动）**：`test_cost_oracle_routing_l241.py` 只
#: 一支 parametrize（纯度判据两档，值位内联字面清单且自带 ids）⇒ 现量落 `plain` 桶。实测
#: CALLS 429 → **430**、READABLE 311 → **312**、盲面仍 **118**；读数同样出自仓外探针
#: （`param_sites(TESTS_ROOT)` 逐处打印桶归属），不是推的。
#: **L243 回填（CALLS +1、READABLE +1、盲面一格未动）**：`test_backup_integrity_l243.py` 只
#: 一支 parametrize（`cleanup` 保留份数的坏值两档 `zero` / `negative`，值位是内联字面清单，
#: 没有 `ids=`）⇒ 现量落 `plain` 桶。探针逐字：`test_backup_integrity_l243.py:322
#: bucket=plain ids=False`，桶分布 `plain 312 / no-literal-list 118 / type-valued 1`。
#: **L244 回填（CALLS +1、READABLE +1、盲面一格未动）**：`test_cli_backup_actions_l244.py` 只
#: 一支 parametrize（`cleanup` 保留份数的坏值两档 `zero` / `negative`，值位是内联字面清单，
#: 没有 `ids=`）⇒ 现量落 `plain` 桶。探针逐字：`test_cli_backup_actions_l244.py:338
#: bucket=plain ids=False`，桶分布 `plain 313 / no-literal-list 118 / type-valued 1`；l98 侧
#: `blind_rows` 仍 118、档位和一字未动（同那一格不进盲面名册，元素静态可见）。
MEASURED_CALLS = 468  # **L288 重量**：466 → **468**（新守卫 `test_xlsx_zipbomb_l288.py` 两只 parametrize，逐格点名：`test_actual_stream_is_counted_not_trusted`（2 档，值位 `(40 * MB, "比率")` 与 `(validation.MAX_SHEET_EXPANDED_BYTES + 5 * MB, "超过上限")` 的内联字面清单）/ `test_each_threshold_is_load_bearing`（3 档，`["ratio", "ceiling", "floor"]`）⇒ 两格都由本模块 `param_sites(TESTS_ROOT)` 现量判为 `bucket=plain ids=False`，全落可读面、盲面一格未动；`READABLE 343 + BLIND 125 == CALLS 468` 闭合。红读数 `assert 468 == 466`，三格读数与桶分布 `plain 342 / no-literal-list 125 / type-valued 1` 出自仓外探针 `Temp/l288q/census.py`（同一次读数把这两格逐格判档、其余 466 格与改前常数逐格对上 ⇒ 零既有漂移；l98 侧档位和 `63/39/12/11`、`MEASURED_BLIND_TESTS` 120、`MEASURED_EXEMPT_SITES` 34 各自未动，因这两格不写字面 `ids=` ⇒ 不进豁免名册），非推断。）**L287 重量**：463 → **466**（新守卫 `test_sheet_formula_injection_l287.py` 一只文件共 3 处 parametrize：`test_every_leader_gets_the_apostrophe_on_every_edge` 值位 `sorted(WRITERS)` ⇒ 进盲面，`test_tsv_says_the_same_as_csv` 与 `test_container_without_a_formula_interpreter_is_untouched` 值位是内联字面清单 ⇒ 进可读面 ⇒ 三面各自闭合：CALLS +3、READABLE +2、BLIND +1）。**L286 重量**：458 → **463**（L286 两支新守卫文件共 5 处 parametrize，逐格点名：`test_dir_self_landing_l286.py` 的 `test_ordinary_names_still_land_strictly_inside`（5 档，值位引用模块级常数 `STILL_ACCEPTED` ⇒ 落盲面）/ `test_the_read_faces_reject_the_same_shape`（2 档，内联字面清单 ⇒ 可读）/ `test_the_self_landing_id_is_a_four_hundred_and_costs_nothing`（3 档，内联字面清单 ⇒ 可读），加 `test_export_enhanced_empty_set_l286.py` 的 `test_the_delegated_exporter_writes_a_file_for_an_empty_dataset`（4 档，值位引用模块级常数 `DELEGATED_FORMATS` ⇒ 落盲面）/ `test_header_and_rows_are_still_written`（2 档，`(格式, 分隔符)` 内联字面清单 ⇒ 可读）；三格进可读、两格进盲面，`339 + 124 == 463` 闭合。红读数 `assert 463 == 458`，两侧读数由本模块 `param_sites(TESTS_ROOT)` 现量取出，非推断。**L284 重量**：453 → **458**（新守卫 `test_validation_finite_echo_l284.py` 的五处 parametrize，逐格点名：`test_nan_echo_returns_422_not_500`（5 档，值位引用模块级常数元组 `ENDPOINTS` ⇒ 无字面清单，唯一落盲面那一格）/ `test_default_handler_called_once`（2 档，值位内联字面清单**且写 literal `ids=`** ⇒ 可读档 + 整支进 l98 豁免名册）/ `test_surrogate_in_the_422_echo_is_also_escaped`（3 档，内联字面清单）/ `test_encodable_walks_each_shape`（7 档，内联字面清单）/ `test_each_shape`（12 档，内联字面清单）。红读数 `assert 458 == 453`，三格读数 `total 458 / blind 122 / readable 336` 由本模块 `param_sites(TESTS_ROOT)` 现量取出（仓外探针 `Temp/probe_l284_census_delta.py`，同一次读数把本文件五格逐格判档、其余 453 格与改前常数逐格对上 ⇒ 零既有漂移），非推断。**L281 重量**：447 → **453**（新守卫 `test_missing_file_family_l281.py` 的六处 parametrize，逐格点名：`test_checkpoint_unlink_sites_declare_missing_ok`（2 档）/ `test_processors_stat_the_size_under_file_not_found`（2 档）/ `test_processors_report_the_honest_missing_reading`（2 档）/ `test_the_corpus_ruler_names_the_pre_fix_shape`（3 档）/ `test_the_roster_names_a_flipped_missing_ok`（3 档）/ `test_the_roster_names_a_widened_except_face`（3 档）；六格值位全是内联字面清单**且都写字面 `ids=`** ⇒ 全落可读档、一格不落盲面，`MEASURED_BLIND` 121 一字未动。红读数 `assert 453 == 447`，三格读数 `total 453 / blind 121 / readable 332` 由本模块 `param_sites(TESTS_ROOT)` 现量取出（仓外探针 `Temp/l277_probes/_l281_ruler_readings.py`），非推断；同一次现量里 l98 侧盲面四档（61/38/11/11）与 `MEASURED_BLIND_TESTS` 116 各自未动，六格全进 `ids-literal` 档 ⇒ 与「盲面 121 未动」独立咬合。**L279 重量**：445 → **447**（覆盖率补口那一批在 `test_upload_gate_l90.py::TestDeclaredContentLengthReading` 新写两支 parametrize —— `test_unparseable_length_is_invisible`（6 档垃圾尺寸）与 `test_numeric_length_is_visible`（3 档数值），两支值位都是内联字面清单 ⇒ 全落可读档，盲面 121 一格未动。红读数 `assert 447 == 445`，三格读数 `total 447 / blind 121 / readable 326` 由本模块 `param_sites(TESTS_ROOT)` 现量取出，非推断。同轮新守卫 `test_shared_cache_lock_l279.py`（19 支）与 `test_memory_cache_lock_l279.py`（13 支）刻意零 parametrize ⇒ 三面判据不进这一面）。**L277 重量**：438 → **445**（本轮四支**新**守卫文件里三支共 7 处 parametrize，逐格点名：`test_config_dir_containment_l277.py::test_relative_escape_is_refused`（3 档，内联字面清单）/ `::test_blank_value_is_not_rejected_here`（3 档，内联字面清单）/ `test_vuln_audit_l277.py::test_top_level_non_mapping_is_actionable`（4 档）/ `::test_huge_int_never_raises_overflow`（5 档，`(函数, dict)` 字面清单）/ `::test_sample_ratio_out_of_zero_one_is_refused`（3 档）/ `test_backup_name_path_segment_l277.py::test_http_create_refuses_it_with_its_own_verdict`（10 档坏名，字面清单 + 字面 ids）/ `::test_create_restore_delete_all_reach_it`（6 档合法名，字面清单 + 字面 ids）；七格值位全是内联字面清单 ⇒ 全部落可读档，盲面 121 一格未动。其余**重写**的守卫（test_rate_limit / test_upload_gate_l90 / test_empty_container_defaults_l262 / test_export / test_partial_branches_l120 / test_stats_features_compare_wiring_l228 六支文件）与 HEAD 逐文件对账**装饰器数与桶归属一格未动**（本轮把 HEAD 那份源用只读方式取出、两棵树喂同一把尺子逐文件比对的脚本写在仓库外的临时目录里，不入仓，故此处不写成定位引用）。第四支新守卫文件 test_empty_scope_http_l277 零 parametrize。读数出自该探针两侧合计 `TOTAL HEAD 438 WT 445`，非推断。L262 那一步：437 → **438**（新守卫 `test_empty_container_defaults_l262.py` 那一支按 A 族十格实例化的 parametrize：值位是模块级常数 `CASES`、ids 写成推导式 `f"{c}.{a}"` ⇒ l97 记「有 ids」进可读档、l98 记进 `ids-derived` 档。读数出自本轮全量的红读数 `assert 438 == 437` 与 `assert 12 == 11`，非推断）。L259 重量：436 → **437**（新守卫 `test_name_path_containment_l259.py` 那一支值位引用模块级常数字典 `MEASURED_CALL_SITES` 的 parametrize ⇒ 与 BLIND 同涨 1；读数出自本轮全量的红读数 `assert 437 == 436`，非推断）。L258 那一步：434 → **436**（新守卫 `test_name_path_containment_l258.py` 两支 parametrize：`sorted(MEASURED_CALL_SITES)` 的值位引用模块级常数字典 ⇒ 落盲面 plain 档；`("method,url", [...])` 是内联字面清单 ⇒ 落可读档。读数出自仓外探针 `Temp/l258_census.py` 直接 `import` 本模块 `param_sites(TESTS_ROOT)` 现量，非推断。L257 那一步：432 → **434**（新守卫 `test_default_text_fields_l257.py` 两支 parametrize：CLI 三档那支的值位是模块级常数 `CLI_TRIO_SUBCOMMANDS` 引用 ⇒ 落盲面 plain 档；族外默认值那支是内联字面清单 ⇒ 落可读档。与 l98 侧档位和 +1（plain 58 → 59）独立咬合）
#: （L185：${ENV} 展开语义守卫两支 parametrize（坏形状 5 档 / 非串 4 档，全是字面清单）
#: ⇒ CALLS 382 → 384、READABLE 281 → 283，盲面 101 一格未动）
#: （L189：验证预设封闭清单守卫一支 parametrize（坏预设 5 档，全字面清单）⇒
#: CALLS 384 → 385、READABLE 283 → 284，盲面 101 一格未动）
#: （L190：leakage min_examples 计数旋钮守卫两支 parametrize（合法 4 档 / 坏形状 8 档，
#: 全字面清单）⇒ CALLS 385 → 387、READABLE 284 → 286，盲面 101 一格未动）
#: （L227：接线守卫一支字面清单 parametrize ⇒ CALLS 404 → 405、READABLE 290 → **291**）
#: （L228：同形一支 ⇒ CALLS 405 → 406、READABLE 291 → **292**）
#: （L229：五支里只有「绝对路径/`..`」那支的值位是内联字面清单 ⇒ READABLE 292 → **293**）
#: （L230：八支全是内联字面清单 ⇒ READABLE 293 → **301**，与 CALLS 同涨 8）
#: （L232：两支全是内联字符串字面清单 ⇒ READABLE 301 → **303**，与 CALLS 同涨 2）
#: （L238：五支全是内联字面清单 ⇒ READABLE 303 → **308**，与 CALLS 同涨 5，盲面不动）
#: （L240：三支全是内联字面清单 ⇒ READABLE 308 → **311**，与 CALLS 同涨 3，盲面不动）
#: （L241：一支内联字面清单 ⇒ READABLE 311 → **312**，与 CALLS 同涨 1，盲面不动）
#: （L243：一支内联字面清单 ⇒ READABLE 312 → **313**，与 CALLS 同涨 1，盲面不动）
#: （L244：一支内联字面清单 ⇒ READABLE 313 → **314**，与 CALLS 同涨 1，盲面不动）
MEASURED_READABLE = 343  # **L288 重量**：341 → **343**（L288 那两只 parametrize 的值位都是内联字面清单 ⇒ `test_actual_stream_is_counted_not_trusted` 与 `test_each_threshold_is_load_bearing` 同落可读面，桶 `plain` 340 → 342；现量与闭合式同批由 `Temp/l288q/census.py` 取出，`READABLE 343 + BLIND 125 == CALLS 468`）。**L287 重量**：339 → **341**（L287 那三格里值位写成内联字面清单的两处：`test_sheet_formula_injection_l287.py` 的 `test_tsv_says_the_same_as_csv`（`["enhanced", "converter"]`）与 `test_container_without_a_formula_interpreter_is_untouched`（`["json", "jsonl"]`）⇒ 与 CALLS 同涨但少一格，因为第三格走盲面；`READABLE 341 + BLIND 125 == CALLS 466` 由本文件那支闭合式当场验）。**L286 重量**：336 → **339**（与 CALLS 同涨那三格：L286 两支守卫里值位写内联字面清单的 `test_the_read_faces_reject_the_same_shape` / `test_the_self_landing_id_is_a_four_hundred_and_costs_nothing` / `test_header_and_rows_are_still_written` ⇒ 全落可读档；另两格引用模块级常数 ⇒ 进 `MEASURED_BLIND`，见那一格；`339 + 124 == 463` 闭合 ⇒ 尺子没被放宽。红读数由本模块现量与 l98 侧档位和独立咬合，非推断。**L284 重量**：332 → **336**（与 CALLS 同涨那四格：`test_validation_finite_echo_l284.py` 里 `test_default_handler_called_once` / `test_surrogate_in_the_422_echo_is_also_escaped` / `test_encodable_walks_each_shape` / `test_each_shape` 四支的值位全是内联字面清单 ⇒ 落可读档；`336 + 122 == 458` 闭合，另那一格 `test_nan_echo_returns_422_not_500` 值位引用常数元组 ⇒ 进盲面，与 l98 侧 plain 61 → 62 独立咬合。红读数 `assert 336 == 332`。**L281 重量**：326 → **332**（与 CALLS 同涨那六格：`test_missing_file_family_l281.py` 的六处 parametrize 值位内联字面清单且写字面 `ids=` ⇒ 全落可读档；`332 + 121 == 453` 闭合，盲面一格未动 ⇒ 尺子没被放宽）。**L279 重量**：324 → **326**（与 CALLS 同涨那两支：`test_upload_gate_l90.py` 的 `test_unparseable_length_is_invisible` / `test_numeric_length_is_visible`，值位内联字面清单 ⇒ 落可读档；`326 + 121 == 447` 闭合，盲面一格未动 ⇒ 尺子没被放宽）。**L277 重量**：317 → **324**（与 CALLS 同涨 7：`test_config_dir_containment_l277.py` 两支 + `test_vuln_audit_l277.py` 三支 + `test_backup_name_path_segment_l277.py` 两支的值位全是内联字面清单 ⇒ 七格全落可读档，`MEASURED_BLIND` 121 一格未动，`324 + 121 == 445` 闭合）。L262 那一步：316 → **317**（新守卫那支 parametrize 带 `ids=` ⇒ 进可读档，盲面 `MEASURED_BLIND` 121 一格未动，`317 + 121 == 438` 闭合）。L258 重量：315 → **316**（`test_name_path_containment_l258.py` 那支内联字面清单的 parametrize，与 CALLS 同涨 1；仓外探针现量 316）
#: L175：搜索方法封闭清单守卫两支 parametrize（坏值 6 档 / 合法 5 档，全是字面
#: 清单）⇒ CALLS 379 → 381、READABLE 278 → 279，盲面（no-literal-list 桶）101 不动。
#: L217：普查守卫三支无 ids parametrize ⇒ 盲面 108 → **111**（与 CALLS +3 同数，
#: 因为三格全落盲面；l98 的 plain 档同增 3，两边独立闭合）。
#: L222：三侧对账守卫三支无 ids parametrize ⇒ 盲面 111 → **114**（同样与 CALLS 同数）；
#: l98 侧档位和 +3（plain +1、derived +2）独立咬合。
#: **L229 回填（盲面 +4）**：`test_transform_wiring_l229.py` 四支的值位是引用/表达式
#: （`OUTPUT_FORMAT_CHOICES`、契约默认表、模型字段名册、TS 接口键名册）⇒ **114 → 118**；
#: l98 侧同四格拆成 opaque +1、ids-derived +3，档位和独立咬合。
#: **L230：盲面 118 一格未动**（八支新 parametrize 全落可读档，见 MEASURED_CALLS 那一段）。
#: **L238：盲面仍 118**（新五支全带内联字面清单；本轮没有把值位换成「从权威现造的名册」，
#: 同源仍靠 AST 读分发臂单独钉，与 L230 同一处置）。
#: **L241：盲面仍 118**（新那支自带内联字面清单与 ids；l98 侧档位和同样一字未动）。
#: **L243：盲面仍 118**（新那支也是内联字面清单；l98 侧档位和一字未动，探针 Counter 见上）。
#: **L244：盲面仍 118**（新那支同样是内联字面清单；l98 侧 `blind_rows` 与档位和一字未动）。
MEASURED_BLIND = 125  # **L288 复量：125 一格未动**（本轮新守卫那两只的值位全是内联字面清单 ⇒ 一格不落盲面；桶分布 `no-literal-list` 仍 125，与 l98 侧档位和 `63 + 39 + 12 + 11 = 125` 独立咬合，读数同上仓外探针）。**L287 重量**：124 → **125**（新守卫那一支把值位写成 `sorted(WRITERS)`、`ids=str` ⇒ 用例名看得见而格子值看不见，按名进 l98 的 `MEASURED_IDS_DERIVED_SITES` 第 12 格；四档 63/39/12/11 之和同为 125 ⇒ 与 l98 的档位表同数）。**L286 重量**：122 → **124**（L286 那两格值位引用模块级常数、无 `ids=` 的 parametrize：`test_dir_self_landing_l286.py::test_ordinary_names_still_land_strictly_inside`（常数清单 `STILL_ACCEPTED`）与 `test_export_enhanced_empty_set_l286.py::test_the_delegated_exporter_writes_a_file_for_an_empty_dataset`（常数清单 `DELEGATED_FORMATS`）⇒ 落盲面；同批另三格内联字面清单 ⇒ 进可读档，见 `MEASURED_READABLE`；`339 + 124 == 463` 闭合。红读数 `assert 463 == 458` 与 l98 侧 `plain 62 → 63` + `derived 38 → 39`（档位和 122 → 124）独立咬合，两侧都由现量取出。**L284 重量**：121 → **122**（`test_validation_finite_echo_l284.py::test_nan_echo_returns_422_not_500` 那一支值位引用模块级常数元组 `ENDPOINTS` 且无 ids ⇒ 落盲面 plain 档；其余四格值位内联字面清单 ⇒ 全落可读档，见 `MEASURED_READABLE` 那格；`336 + 122 == 458` 闭合）。红读数 `assert 122 == 121`。**L277 复量：121 一格未动**（本轮七处新 parametrize 的值位全是内联字面清单 ⇒ 一格都不落盲面；`test_empty_scope_http_l277.py` 零 parametrize；重写的那六支守卫文件与 HEAD 逐文件比装饰器数与桶归属相同，见 `MEASURED_CALLS` 那段）。L259 重量：120 → **121**（同一支 parametrize：值位是模块级常数引用、无 ids ⇒ 落盲面，且 `MEASURED_READABLE` 316 一字未动，`316 + 121 == 437` 闭合）。L258 那一步：119 → **120**（那支值位引用模块级常数字典 `MEASURED_CALL_SITES` 的新 parametrize ⇒ 与 CALLS 同涨 1；l98 侧同一格只进 plain 档，档位和独立闭合）。L257 那一步：118 → **119**（那支值位引用模块级常数 `CLI_TRIO_SUBCOMMANDS` 的新 parametrize ⇒ 与 CALLS 同涨 1）
#: 值位含类型对象的那些档（必须全部自带 ids，否则就是本判据的违规）
KNOWN_TYPED_SITES = frozenset({("tests/unit/test_schema_type_spec_l96.py", "test_label")})


def _value_shapes(node, hits: set) -> None:
    """把「处在**值位**上的类型形状」记进 `hits`

    三条下钻规则：`f(x)` 不进 `f`（那是被调用的函数，不是被喂的参数，所以
    `float("nan")` 与 `set()` 都不算类型值）；`a.b` 当叶子（`a` 是命名空间前缀，整条属性链
    才是那个值）；`X | Y` 记一次 `union` 后**不下钻**（联合的两边必然是类型，再记一遍
    `int` / `str` 只是把同一格数成两格）。
    """
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.BitOr):
        hits.add("union")
        return
    if isinstance(node, ast.Name):
        if node.id in TYPE_VALUE_NAMES:
            hits.add(node.id)
        return
    if isinstance(node, ast.Attribute):
        if isinstance(node.value, ast.Name) and node.value.id in NS_PREFIXES:
            hits.add(f"{node.value.id}.{node.attr}")
        return
    if isinstance(node, ast.Call):
        for arg in node.args:
            _value_shapes(arg, hits)
        for kw in node.keywords:
            _value_shapes(kw.value, hits)
        return
    for child in ast.iter_child_nodes(node):
        _value_shapes(child, hits)


def _hits_for(value_arg) -> Tuple[str, Tuple[str, ...]]:
    """判一支 parametrize 的值参数：返回 `(桶, 命中的类型值形状)`

    桶有三态，**没有「判不出就算安全」那一支**：字面 list 之外一律 `no-literal-list`。
    """
    if not isinstance(value_arg, ast.List):
        return "no-literal-list", ()
    hits: set = set()
    for elem in value_arg.elts:
        _value_shapes(elem, hits)
    return ("type-valued" if hits else "plain"), tuple(sorted(hits))


#: 已知答案的样本。`plain_case` 里 `float` / `set` 是**被调用**的函数，必须读成 plain
#: （v1 探针把这类当类型值收了 24 支假阳性）；`violation_case` 必须读成「类型值且无 ids」，
#: 所以这份样本自己就证明了判据能红 —— 一份读不出任何违规的样本无法证明检出能力。
SAMPLE_SRC = '''
import pytest
VALUES = [1, 2]

@pytest.mark.parametrize("a,b", [(float("nan"), 1), (set(), 2)])
def plain_case(a, b): pass

@pytest.mark.parametrize("t,want", [(int | str, "u"), (str, "s")], ids=["union", "str"])
def named_case(t, want): pass

@pytest.mark.parametrize("t", [typing.Any])
def violation_case(t): pass

@pytest.mark.parametrize("v", VALUES)
def blind_case(v): pass
'''

SAMPLE_ANSWER = {
    "plain_case": ("plain", ()),
    "named_case": ("type-valued", ("str", "union")),
    "violation_case": ("type-valued", ("typing.Any",)),
    "blind_case": ("no-literal-list", ()),
}


def _sample_sites() -> Dict[str, dict]:
    """跑样本源码，得到 `用例名 → 读数`，用于每次读数前自证"""
    return {r["test"]: r for r in _sites_from(ast.parse(SAMPLE_SRC), "<sample>")}


def _sites_from(tree: ast.AST, rel: str) -> List[dict]:
    out = []
    for node in ast.walk(tree):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        for dec in node.decorator_list:
            call = dec.value if isinstance(dec, ast.Attribute) else dec
            if not (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute)):
                continue
            if call.func.attr != "parametrize":
                continue
            value_arg = None
            for a in call.args[1:]:
                value_arg = a
                break
            if value_arg is None:
                for k in call.keywords:
                    if k.arg == "argvalues":
                        value_arg = k.value
            if value_arg is None:
                continue
            bucket, hits = _hits_for(value_arg)
            out.append({
                "file": rel, "line": node.lineno, "test": node.name,
                "ids": any(k.arg == "ids" for k in call.keywords),
                "bucket": bucket, "hits": hits,
            })
    return out


def _rel(p: Path, root: Path) -> str:
    """仓内文件给仓库相对路径（判据里的名册要能读），临时注入给裸名（不为它编一套前缀）"""
    try:
        return p.relative_to(AI_ROOT).as_posix()
    except ValueError:
        return p.name


def param_sites(root: Path) -> List[dict]:
    """普查 `root` 下所有 `@pytest.mark.parametrize` 调用及其值形状

    每次读数前先跑 `SAMPLE_SRC` 自证（A171 口径：自检写在读数函数里，不是另一支可选用例），
    对不上就抛 `RulerNotProven`。

    Args:
        root: 要普查的目录（通常是 `tests/`，注入档会指向临时目录）

    Returns:
        每支 parametrize 一条 dict：`file` / `line` / `test` / `ids` / `bucket` / `hits`

    Raises:
        RulerNotProven: 样本读数与已知答案不同，或 root 下一支 parametrize 都没读到
    """
    sample = _sample_sites()
    for name, expected in SAMPLE_ANSWER.items():
        got = sample[name]["bucket"], sample[name]["hits"]
        if got != expected:
            raise RulerNotProven(f"parametrize 尺子读错样本 {name}：{got} != {expected}")
    if not any(r["bucket"] == "type-valued" and not r["ids"] for r in sample.values()):
        raise RulerNotProven("样本里没有「类型值且无 ids」那一档，这份样本证明不了判据能红")

    files = sorted(p for p in Path(root).rglob("*.py") if "__pycache__" not in p.parts)
    rows: List[dict] = []
    for p in files:
        rows.extend(_sites_from(ast.parse(p.read_text(encoding="utf-8", errors="replace")),
                                _rel(p, Path(root))))
    if not rows:
        raise RulerNotProven(f"{root} 里一支 parametrize 都没读到：这是数不到，不是干净")
    return rows


def violations(rows: List[dict]) -> set:
    """值位含类型对象 / 联合却没写 `ids=` 的那些档，键为 `(文件, 用例名)`"""
    return {(r["file"], r["test"]) for r in rows if r["bucket"] == "type-valued" and not r["ids"]}


def blind_sites(rows: List[dict]) -> set:
    """静态判不出值形状的那些档（值来自变量 / 生成器 / 调用）"""
    return {(r["file"], r["test"]) for r in rows if r["bucket"] == "no-literal-list"}


@pytest.fixture(scope="module")
def rows() -> List[dict]:
    return param_sites(TESTS_ROOT)


class TestTheRulerProvesItself:
    """尺子自证：先证明它读得对，再拿它读数"""

    def test_sample_readings_match_known_answers(self):
        sample = _sample_sites()
        for name, expected in SAMPLE_ANSWER.items():
            assert (sample[name]["bucket"], sample[name]["hits"]) == expected, name

    def test_called_builtin_is_not_a_type_value(self):
        """v1 假阳性的那一格：`float("nan")` / `set()` 是被调用的函数，不是被喂的类型"""
        assert _sample_sites()["plain_case"]["bucket"] == "plain"

    def test_sample_contains_a_detectable_violation(self):
        """样本必须能让判据红一次 —— 读不出违规的样本证明不了检出能力"""
        assert violations(list(_sample_sites().values())) == {("<sample>", "violation_case")}

    def test_ruler_refuses_a_silence_shaped_like_success(self, tmp_path):
        """数不到东西要抛，而不是返回空集让「干净」看起来成立（A171 本体）"""
        empty = tmp_path / "t.py"
        empty.write_text("def nothing(): pass\n", encoding="utf-8")
        with pytest.raises(RulerNotProven):
            param_sites(tmp_path)


class TestCensusShapeIsPinned:
    """普查规模钉死：动到全仓 parametrize 形状就要重量并改常量"""

    def test_census_totals_match_this_rounds_measurement(self, rows):
        assert len(rows) == MEASURED_CALLS
        blind = sum(1 for r in rows if r["bucket"] == "no-literal-list")
        assert blind == MEASURED_BLIND
        assert len(rows) - blind == MEASURED_READABLE
        assert MEASURED_READABLE + MEASURED_BLIND == MEASURED_CALLS

    def test_typed_face_is_exactly_the_known_sites(self, rows):
        """可判面里喂了类型对象的就那几支：多出未知的一支必须被看见，而不是默默进违规集"""
        typed = [r for r in rows if r["bucket"] == "type-valued"]
        assert len(typed) == len(KNOWN_TYPED_SITES)
        assert {(r["file"], r["test"]) for r in typed} == KNOWN_TYPED_SITES


class TestNoInterpreterDependentIds:
    """判据本体"""

    def test_every_type_valued_parametrize_names_its_ids(self, rows):
        assert violations(rows) == set()

    def test_known_typed_site_still_carries_explicit_ids(self, rows):
        """删掉那支用例或摘掉它的 ids 都会让上一条判据变成空转 ⇒ 这里单独钉住"""
        site = ("tests/unit/test_schema_type_spec_l96.py", "test_label")
        assert site in KNOWN_TYPED_SITES
        hits = [r for r in rows if (r["file"], r["test"]) == site]
        assert len(hits) == 1, f"那一支的定位读到 {len(hits)} 份，名册已漂"
        assert hits[0]["ids"] is True
        assert "union" in hits[0]["hits"]


class TestInjectionMakesItRed:
    """反向证据：往临时目录写一支「联合当值且不写 ids」的用例，判据必须点名它"""

    def test_union_without_ids_is_flagged(self, tmp_path):
        bad = tmp_path / "injected.py"
        bad.write_text(
            "import pytest\n\n\n"
            '@pytest.mark.parametrize("t", [int | str])\n'
            "def injected_union(t):\n    pass\n",
            encoding="utf-8",
        )
        injected = param_sites(tmp_path)
        assert violations(injected) == {("injected.py", "injected_union")}

    def test_the_same_site_with_ids_is_clean(self, tmp_path):
        ok = tmp_path / "injected.py"
        ok.write_text(
            "import pytest\n\n\n"
            '@pytest.mark.parametrize("t", [int | str], ids=["int-or-str"])\n'
            "def injected_union(t):\n    pass\n",
            encoding="utf-8",
        )
        assert violations(param_sites(tmp_path)) == set()

    def test_blind_face_is_recorded_as_blind_not_as_safe(self, tmp_path):
        """值来自变量 ⇒ 落进盲桶，绝不落进「已判过且干净」"""
        src = tmp_path / "injected.py"
        src.write_text(
            "import pytest\nVALUES = [1]\n\n\n"
            '@pytest.mark.parametrize("v", VALUES)\n'
            "def blind(v):\n    pass\n",
            encoding="utf-8",
        )
        got = param_sites(tmp_path)
        assert got[0]["bucket"] == "no-literal-list"
        assert blind_sites(got) == {("injected.py", "blind")}
        assert violations(got) == set()
