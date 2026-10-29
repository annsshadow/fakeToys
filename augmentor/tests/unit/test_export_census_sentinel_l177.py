# -*- coding: utf-8 -*-
"""L177 / B247：SDK 导出面死符号哨兵（`__all__` 逐条必须可核到消费方）。

L177 普查把 `__all__` 里的导出符号逐个对全仓消费面扫了一遍，撞出 1 个零消费方的
死别名（`get_supported_export_formats`，已删）。本哨兵把普查**机器化**成常驻守卫。

**guard-integrity 自查点名的是这把尺子的读数面**，本轮改的就是这一处。旧判据把
「符号名在某一行文本里出现过」当成消费，而那一步既不看语法位置也不排散文：

* 产品面里 `class DataInsight:` / `def http_status_code(...)` 是**定义自身**，
  旧过滤只摘 `^def 符号名` 那一行，其余同文件的散文照样喂饱判据；
* `from .indexer import (\\n    DatasetIndexer, QueryResult,\\n)` 这类**跨行 import**
  的续行不以 `import ` 开头 ⇒ 旧过滤放过它，于是「导进测试文件但没人读」也算消费；
* 注释与 docstring 里的散文提到名字同样算一处引用。

三条叠起来的现量后果：判据退化成「这串字符在仓库里出现过吗」，221 条导出**不可能**
被判死（量不到东西），而 `test_the_dead_alias_stays_out` 是对一个不存在的键做
`not in`，且只盯 `__all__` 一张面——别名若不以导出形态回来、只以 `augmentor.X`
的命名空间形态回来，旧判据一声不响。

现在换成 AST 读数：**只有 `ast.Name(ctx=Load)` 与 `ast.Attribute.attr` 算读取**，
按构造排除 import 语句、定义行、`__all__` 清单行与一切散文（注释与 docstring
不是 Name 节点）。语料与普查口径不变（`augmentor/__init__.py` 那一处再导出点自身
不算消费），现量仍是 0 条死符号——但这次是**量出来的 0**：`TestRulerHasTeeth`
四支注入臂（造一条零读取的导出 / 只在散文里出现的名字 / 跨行 import 续行 /
别名回到命名空间）逐臂点名喊红，注入全在内存文本与 `monkeypatch` 上，磁盘零写入。
"""
import ast
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
#: 唯一的再导出点：它的 import 行与 `__all__` 清单行都不算消费，整份文件不进语料
_EXPORT_SITE = _ROOT / "augmentor" / "__init__.py"
_PRODUCTS = (sorted(_ROOT.joinpath("augmentor").rglob("*.py"))
             + sorted(_ROOT.joinpath("api").rglob("*.py"))
             + [_ROOT / "cli.py"])
_TESTS = sorted(_ROOT.joinpath("tests").rglob("*.py"))

#: L177 删掉的那只死别名（原版 `export_enhanced.get_supported_formats` 才是正主）
DEAD_ALIAS = "get_supported_export_formats"

#: `__all__` 现量 221 条（L177 那轮普查记的是 220，此后名单又长过一格）。
#: 这一格漂了就在名字上方的常量里按现量重钉，别顺手放宽逐条判据。
ROSTER_SCALE = 221


def _all_symbols() -> list:
    import augmentor
    return list(augmentor.__all__)


def ast_reads(text: str) -> set:
    """一段源码里被**读取**到的名字（真实消费的唯一证据形态）。

    `ast.Name(ctx=Load)` 覆盖调用、构造、注解、右侧取值；`ast.Attribute.attr` 覆盖
    `augmentor.DatasetBackup` 这一形（测试面打包级导出就走这条路）。import 语句、
    `def`/`class` 的定义头、`__all__` 里的字符串字面量、注释与 docstring 都不产出
    这两类节点 ⇒ 散文与自我指涉按构造被排除，不需要再写文本过滤。
    """
    try:
        tree = ast.parse(text)
    except SyntaxError:  # 非 Python 或写坏的桩文件：不产读数，由规模判据抓
        return set()
    out = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Name) and isinstance(node.ctx, ast.Load):
            out.add(node.id)
        elif isinstance(node, ast.Attribute):
            out.add(node.attr)
    return out


def _scan(files) -> dict:
    hits = {}
    for path in files:
        if path == _EXPORT_SITE or "__pycache__" in path.parts:
            continue
        for name in ast_reads(path.read_text(encoding="utf-8", errors="replace")):
            hits.setdefault(name, []).append(str(path.relative_to(_ROOT)))
    return hits


_PRODUCT_READS = _scan(_PRODUCTS)
_TEST_READS = _scan(_TESTS)


def consumer_files(symbol: str) -> list:
    """产品面与测试面上真实读取到该符号的文件（相对路径，逐个点名用）"""
    return sorted(set(_PRODUCT_READS.get(symbol, [])) | set(_TEST_READS.get(symbol, [])))


def dead_exports(symbols=None) -> list:
    """`__all__` 里零真实读取的条目 —— 空集才算活，非空即死导出"""
    roster = _all_symbols() if symbols is None else list(symbols)
    return sorted(s for s in roster if not consumer_files(s))


def alias_returned(pkg) -> list:
    """死别名的两张回归面：`__all__` 清单，以及包命名空间里的公开名。

    旧判据只看第一张，于是 `from .export_enhanced import get_supported_formats as
    get_supported_export_formats` 不写进 `__all__` 也能让人打到 `augmentor.别名` ——
    那正是 L177 想拦的东西换了个壳。
    """
    faces = []
    if DEAD_ALIAS in list(pkg.__all__):
        faces.append("__all__ 清单")
    if DEAD_ALIAS in vars(pkg):
        faces.append("包命名空间")
    return faces


def test_export_roster_scale():
    """`__all__` 现量 221 条且无重复条目 —— 语料与名单的规模都先钉住"""
    roster = _all_symbols()
    assert len(roster) == ROSTER_SCALE, f"__all__ 现量 {ROSTER_SCALE} 条，实得 {len(roster)} 条"
    assert len(set(roster)) == len(roster), "有重复条目，逐条判据会漏量"


def test_scanned_corpus_scale():
    """尺子读的语料规模钉住：产品面 133 只 + 测试面 415 只（现量，不含再导出点自身）

    扫描面缩水（换目录、加过滤）会把「量不到的 0」重新变成假绿，所以这里先量自己。
    413 = 412 只已跟踪 + 1 只未跟踪（L291 那一支已转入已跟踪侧，本轮 L294 新增的那一支守卫尚未提交，`git ls-files tests` 与 `--others` 两侧现量；
    提交之后它换到已跟踪侧，总数一格不动 —— 这一维本来就只钉总数）。
    改数之前那一份读数是 405 = 386 只已跟踪 + 19 只**当时**尚未跟踪的本轮系列新增测试文件 ⇒
    下面那串名单逐只点名的十九次计数是历史账，「19 只未跟踪」这一维到本轮已过期（往后的批次把
    它们逐只提交了，所以分解变了而总数 405 在那一段里没动）：
    `test_vuln_audit_l277.py` / `test_empty_scope_http_l277.py` /
    `test_backup_name_path_segment_l277.py` / `test_config_dir_containment_l277.py` /
    `test_delta_remerge_l277.py`（第二次数 392 → 393 就是它）/
    `test_async_checkpoint_resume_l277.py` / `tests/integration/test_data_page_slice_l277.py`
    / `test_pattern_blowup_l278.py`（第三次数 393 → 394 就是它）/
    `test_shared_cache_lock_l279.py` / `test_memory_cache_lock_l279.py`（L279 两面罩锁守卫，
    第四次数 394 → 396 就是这两只）/ `test_disk_cache_missing_file_l280.py`（L280 缺文件语义与自逐出守卫，第五次数 396 → 397 就是它）/ `test_missing_file_family_l281.py`（L281 把同一族推到全仓产品语料的常驻守卫，第六次数 397 → 398 就是它；产品面 133 一格未动，本轮只改产品文件内容与新增这一支测试）。/ `test_path_lock_registry_cap_l283.py` + `test_status_degrade_logs_l283.py`（L283 锁表封顶与状态端点出声两支守卫，第七次数 398 → 400 就是这两只；同轮产品面改了 `atomic_write.py` 与 `api/routes/status.py` 的内容，产品文件**只数文件不数行** ⇒ 133 仍不动）。/ `test_validation_finite_echo_l284.py`（L284 错误回执渲染兜底那一族的守卫，第八次数 400 → 401 就是它；本轮产品面只改 `api/main.py` 的内容、没新增文件 ⇒ 133 第四次确认不动）。/ `test_log_forging_l285.py`（L285 日志注入族那一支守卫，第九次数 401 → 402 就是它；同轮产品面改了 `logging_setup.py`（尾部新增消毒三件）与 `api/main.py`（两处同行改写）的内容、没新增产品文件 ⇒ 133 第五次确认不动；该守卫刻意**零 `parametrize`**（`grep -c parametrize` 整文件现量 0），l97 / l98 那两把尺子的量面因此没有新增成员，而不是「跑过了所以没变」。/ `test_dir_self_landing_l286.py` + `test_export_enhanced_empty_set_l286.py`（L286 自指落点与空集静默零写入两支守卫，第十次数 402 → 404 就是这两只；同轮产品面改了 `validation.py`（权威同行改写 + docstring 三处同行改写，净零行）与 `export_enhanced.py`（两个格式分支压成「文件必建、空集 0 字节」，净 -5 行）的内容、没新增产品文件 ⇒ 133 第六次确认不动；这两支守卫**都带 `parametrize`** ⇒ l97 / l98 的量面新增成员，改的是成员数而不是恒真，本轮按各自红读数重钉）。/ `test_sheet_formula_injection_l287.py`（L287 电子表格公式注入那一族的守卫，第十一次数 404 → 405 就是它；同轮产品面改了 `validation.py`（尾部新增判据四件）与 `export.py` / `export_enhanced.py` / `converter.py` / `excel_write.py` 的内容、没新增产品文件 ⇒ 133 第七次确认不动；这支守卫带 3 处 `parametrize`（`grep -c` 整文件现量）⇒ l97 / l98 的量面新增成员，本轮同样按各自红读数重钉）。/ `test_xlsx_zipbomb_l288.py`（L288 电子表格容器解压放大那一族的守卫，第十二次数 405 → 406 就是它；同轮产品面改了 `validation.py`（尾部新增三条阈值与判决函数，净 +72 行）、`csv_excel_import.py`（第 79 行同行改写 + 尾部新增闸门，净 +61 行）与 `converter.py`（两处同行改写，`git diff --numstat` 复验 5/5 净零行）的内容、没新增产品文件 ⇒ 133 第八次确认不动；插行全部落在三只文件的尾部或既有行之内，`converter.py:999`/`:1005` 与 `csv_excel_import.py:61` 这些活引用逐处复验未平移。这支守卫带 2 处 `parametrize` ⇒ l97 侧 `CALLS 466 → 468`、`READABLE 341 → 343`、`BLIND 125` 未动，l98 侧档位与两张名册未动，逐格读数见 `Temp/l288q/census.py`）。/ `test_config_write_lock_l289.py` + `test_logging_apply_concurrency_l289.py` + `test_module_collectable_l289.py`（L289 配置写面两把锁、日志装配锁、全仓可收集三面守卫，第十三次数 406 → 409 就是这三支；同轮产品面改了 `api/deps.py`（装配双检锁 + 复位同锁，净 +17 行）、`api/routes/config.py`（端点整段罩 `path_lock`，净 +8 行）与 `augmentor/logging_setup.py`（A393 锁壳拆分，净 +20 行）的内容、没新增产品文件 ⇒ 133 第九次确认不动。**但本轮插行落在三只文件的中部**，把在册活引用整体顶移 ⇒ a184 硬 0 档与 l79 同时红，按各自红读数逐格重钉、判据一字未松：`api/deps.py:59` 是 `verify_api_key` 的比较行（旧落点 58）、`api/deps.py:511` 是 `def to_http_error(`（旧 494）、`augmentor/logging_setup.py:226` 是 `if keys and not root.handlers:`（旧 206）、`augmentor/logging_setup.py:172` 是 `def apply_logging_config(`（旧 166，括弧点名符号的那两行 A102 / A106）；l217 那三条归档读取引用同样从 190/191/192 顶到 210/211/212。三支守卫**都零 `parametrize`**（`grep -c parametrize` 整文件现量各 0）⇒ l97 / l98 的量面没有新增成员，而不是「跑过了所以没变」。第三只守卫来自本轮全量定标当场撞出的 A394：`test_validation_finite_echo_l284.py` 的模块 docstring 不是 raw，里面那句孤立代理字面量在 3.13 上让整支守卫 `compile` 不过去（`Interrupted: 1 error during collection`），此前那份读数出自 3.12（`__pycache__` 里 312 / 313 两种标签并存为证）⇒ 那只文件加 `r` 前缀收口，测试面总数因它不变（改的是既有文件的内容）。 / `test_dependency_registry_lock_l290.py` + `test_versioning_current_lock_l290.py`（L290 依赖登记表跨请求丢登记、版本面 rollback/delete 锁外 check-then-act 两支守卫，第十四次数 409 → 411 就是这两只；同轮产品面改了 `augmentor/dependency.py`（三处写点进 `path_lock` + 锁内重读，新方法 `_load_datasets` / `_load_dependencies` 落在文件中部）与 `augmentor/versioning.py`（`delete_version` 整段罩锁净零行、`rollback` 整段罩锁净 -1 行）的内容、没新增产品文件 ⇒ 133 第十次确认不动。两支守卫**都带 2 处 `parametrize`** 且值位全是 `sorted(<模块级常数>)` ⇒ l97 侧 `CALLS 468 → 472`、`READABLE 343` 一格未动、`BLIND 125 → 129`，l98 侧 `plain 63 → 67`、档位和 129、盲面用例名册 120 → **124**，`MEASURED_EXEMPT_SITES` 34 未动（四格都不写字面 `ids=`）；逐格读数出自仓外探针 `Temp/l290q/l97census.py` 与 `Temp/l290q/l98blindtests.py` 的同一次现量，非推断。**插行落点的活引用复验**：本轮 `dependency.py` 净 +32 行（`git diff --numstat` 现量 54/22，`__init__` 内的注释扩写与新增读面 `_load_datasets` 都落在文件中部），`grep -n` 逐处复验在册引用 `dependency.py:89`（`__init__` 仍收在 106）与 `:142`（`register_dataset` 定义行，新方法插在其后）未平移；`versioning.py` 被引的 86 / 185 / 205 / 283-284 / 289 / 372 六处同样逐处复验未出代码跨度。） / `test_merge_truncation_attribution_l291.py`（L291 合并面截断归因那一族的守卫，第十五次数 411 → 412 就是它；同轮产品面只改了 `augmentor/dataset_ops.py` 第 219–220 行两格的内容（同行改写，`git diff --numstat` 现量 2/2 净零行）、没新增产品文件 ⇒ 133 第十一次确认不动。这支守卫**零 `parametrize`**（`grep -c parametrize` 整文件现量 0）⇒ l97 / l98 的量面没有新增成员，而不是「跑过了所以没变」。红读数逐字 `AssertionError: 语料规模现量 (133, 411)，实得 (133, 412)`，两面之差由 `git ls-files tests`（411）与 `--others`（1）两侧现量对上，非推断。） / `test_stream_retry_backoff_l294.py`（L294 流式增量解析的重试节奏翻倍那一族的计数守卫，第十六次数 412 → 413 就是它；同轮产品面只改 `augmentor/streaming.py` 的内容——新增 `_next_retry_gap` 与两处「攒够 gap 个新字符才重试」的节奏，`git diff --numstat` 现量 28/6 净 +22 行、产品文件**只数文件不数行** ⇒ 133 第十二次确认不动。**但本轮插行落在文件中段**，把在册活引用 `augmentor/streaming.py`（`read_chunks` 定义行）由 296 顶到 318 ⇒ a184 硬 0 档报 2 处（卷一 A147 那一格、`test_system_ops_wiring_l234.py` 的模块 docstring），两处按处方 3 同行重钉为 318，`git diff --numstat` 复验各 1/1 净零行。这支守卫**零 `parametrize`**（`grep -c parametrize` 整文件现量 0）⇒ l97 / l98 的量面没有新增成员，而不是「跑过了所以没变」。红读数逐字 `AssertionError: 语料规模现量 (133, 412)，实得 (133, 413)`。） / `test_save_config_comments_auq1.py` + `test_narrow_excepts_auq2.py`（AU-Q1 配置保存 round-trip 保注释、AU-Q2 except 收窄成真实失败族两支守卫，第十七次数 413 → 415 就是这两只；同轮产品面改了 `augmentor/config.py`（save_config 尾部接 round-trip 支路 + 两个模块级 helper）、`augmentor/checkpoint.py`（六处 except 收窄）与 `augmentor/config_validator.py`（一处 except 收窄）的内容、没新增产品文件 ⇒ 133 第十四次确认不动。**但两只产品文件的插行落在中段**，把在册活引用顶移 ⇒ a184 硬 0 档报 9 处、l79 `code_but_no_name_match` +2，按现量逐格重钉（checkpoint 三档 321/372/443 → 332/385/462、`:592` → `:611`、`:501` → `:520`，config_validator 的 880-900 → 883-903），判据一字未松。两支守卫**都零 `parametrize`** ⇒ l97 / l98 的量面没有新增成员。红读数逐字 `AssertionError: 语料规模现量 (133, 413)，实得 (133, 415)`。） / `test_async_generate_au_p1.py`（AU-P1 异步链路守卫：`async_with_retries` 的策略与同步版逐位一致、`agenerate` 的缓存/统计口径、OpenAI 原生 httpx 路径与缺 httpx 回退，第十八次数 415 → 416 就是它；同轮产品面改了 `models/base.py`、`models/openai_model.py`、`retry.py` 的内容、没新增产品文件 ⇒ 133 第十五次确认不动。**产品文件的插行落在中段**，在册活引用按现量重钉：`openai_model.py:71 → 73`（payload 的 `model` 行）、`base.py:75/93/278 → 76/94/279`（`_DEFAULT_ATTEMPTS` / `DEFAULT_RESPONSE_CACHE_MAX_BYTES` / `_cache_key`），`git diff --numstat` 复验 `base.py` 净 +1 行（`import asyncio`）之外的两段插入都落在被引行之后。这支守卫**零 `parametrize`** ⇒ l97 / l98 的量面没有新增成员。）
    """
    n_products = len([p for p in _PRODUCTS if p != _EXPORT_SITE])
    n_tests = len(_TESTS)
    assert (n_products, n_tests) == (133, 416), \
        f"语料规模现量 (133, 416)，实得 {(n_products, n_tests)}"


def test_every_all_symbol_has_a_consumer():
    """`__all__` 逐条有真实读取方（产品面或测试面）——死导出禁入库"""
    dead = dead_exports()
    assert dead == [], (
        f"__all__ 里的 {dead} 全仓零真实读取（只有定义行、import 语句或散文提到它）——"
        "死导出按 L177 纪律删除或接消费方，二选一。"
    )


def test_the_dead_alias_stays_out():
    """`get_supported_export_formats` 不许以 `__all__` 条目**或**包命名空间公开名回来。

    反向护栏同步在场：禁令不是靠删能力达成的——正主 `export_enhanced.get_supported_formats`
    必须仍然可 import、仍然被测试面读取。
    """
    import augmentor
    from augmentor.export_enhanced import get_supported_formats

    assert alias_returned(augmentor) == [], f"死别名回流：{alias_returned(augmentor)}"
    assert callable(get_supported_formats), "正主被顺手删了 ⇒ 禁令变成假绿"
    # 正主的消费方确实在场（不是靠「没人引用」通过的）：测试面里至少有一处真实读取
    assert consumer_files("get_supported_formats"), "正主零读取 ⇒ 这一族已被整体删掉"


class TestRulerHasTeeth:
    """注入臂：每一臂都必须让对应判据点名喊红（全走内存文本 / monkeypatch，磁盘零写入）"""

    def test_fabricated_dead_export_turns_the_census_red(self, monkeypatch):
        """往 `__all__` 里塞一条没有任何读取方的名字 ⇒ 主判据当场非空"""
        import augmentor

        assert dead_exports() == [], "基线必须先为 0，否则这一臂证明不了牙齿"
        ghost = "l177_ghost_export_no_consumer"
        monkeypatch.setattr(augmentor, "__all__", list(augmentor.__all__) + [ghost])
        assert dead_exports() == [ghost], f"注入的死导出没被点名：{dead_exports()}"

    def test_prose_mention_does_not_count_as_a_consumer(self):
        """只在注释与 docstring 里出现的名字不算消费（旧文本判据在这一格是绿的）"""
        text = (
            "# DataInsight 在这一版被弃用了\n"
            'def helper():\n    """改见 DataInsight 的替代者"""\n    return 1\n'
        )
        assert "DataInsight" not in ast_reads(text), "散文喂饱了 AST 判据 ⇒ 读数面退化"
        # 旧口径（词边界文本命中）在这一格是「有引用」的读数，写下差异以免日后退回
        import re
        assert re.search(r"\bDataInsight\b", text), "注入样本失效：散文里根本没有这个名字"

    def test_multiline_import_continuation_does_not_count(self):
        """`from x import (` 的续行读不到消费（旧过滤按行首 `import ` 判断，正是漏的那一处）"""
        text = "from augmentor.indexer import (\n    DatasetIndexer,\n    QueryResult,\n)\n"
        assert ast_reads(text) == set(), f"跨行 import 的续行被当成消费：{ast_reads(text)}"

    def test_definition_line_alone_does_not_count(self):
        """定义行自己不算消费：这正是旧尺子「永远量不到东西」的那条结构性根因

        旧判据的过滤器只摘 `from `/`import ` 行首与 `__all__` 清单行，`class DataInsight:`
        这样一行照样喂饱它 ⇒ **任何在产品模块里定义的导出符号，哪怕全仓没人读它，也恒绿**。
        现量抽查（`tests/unit/test_indexer.py:11` 那条跨行 import 续行、`augmentor/analytics.py:53`
        那条 class 定义行）就是旧读数与真消费的错位处。
        """
        import re

        text = "class DataInsight:\n    pass\n"
        assert ast_reads(text) == set(), "定义行被当成消费 ⇒ 主判据退化成恒真"
        assert re.search(r"\bDataInsight\b", text), "注入样本失效"

    def test_real_consumer_line_does_count(self):
        """正控制：同一条语句后面真的读一次，判据必须收 —— 收窄不是把尺子收成恒红"""
        text = (
            "from augmentor.indexer import (\n    DatasetIndexer,\n    QueryResult,\n)\n"
            "result = QueryResult()\n"
        )
        assert "QueryResult" in ast_reads(text)

    def test_alias_back_on_the_namespace_is_caught(self, monkeypatch):
        """别名不以导出形态回来、只挂在包命名空间上 ⇒ 两表面判据仍要响"""
        import augmentor

        assert alias_returned(augmentor) == [], "基线必须先干净"
        monkeypatch.setattr(augmentor, DEAD_ALIAS, lambda: ["json"], raising=False)
        assert alias_returned(augmentor) == ["包命名空间"], "命名空间那一面没牙"
        monkeypatch.delattr(augmentor, DEAD_ALIAS)
        monkeypatch.setattr(augmentor, "__all__", list(augmentor.__all__) + [DEAD_ALIAS])
        assert alias_returned(augmentor) == ["__all__ 清单"], "清单那一面没牙"
