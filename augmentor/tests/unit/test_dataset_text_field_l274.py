# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L274 守卫：数据集条目的「文本字段」装着非字符串值时，服务端不许回答内部错误

**这一轮收的是哪一格**（改前 91 格定向探针实测，28 格 **500**、6 枚本仓栈帧）：
`[{"instruction": 1, "output": 2}]` 在形状门口是合法的（`assert_dataset_shape` 只判
「顶层是 list、元素是 dict」），于是它一路走进下游那些把 `item.get(key, "")` 当字符串
用的写法——`"为什么" in 1` 抛 `TypeError`、`1.lower()` 抛 `AttributeError`、
`1.encode()` 抛 `AttributeError`，而六只端点的收尾是 `raise_internal_error` ⇒
**调用方自己的文件让服务端报内部错误**，客户端只拿到固定文案（A155①），可行动信息全
埋在服务端日志里。落点：`GET /api/analyze/{{fn}}`（`sampler._analyze_question_type`
与 `_analyze_topic_distribution`）、`GET /api/data/load/{{fn}}?search=`、
`POST /api/quality/report` 与 `/evaluate` 与 `POST /api/dataset/compare`
（`quality.batch_score` 的 `_char_ngrams`）、`POST /api/quality/dedup`
（`dedup.deduplicate`）、`POST /api/dataset/merge`（`dataset_ops._deduplicate`）。

**为什么按具名子类放行而不是把 `DataValidationError` 整体放行**：门口三个收敛函数对
「SDK 已判出来的参数错误」给了两种归宿，`to_http_error` 与 `raise_boundary_error` 落
400，`raise_internal_error` 将其余一律折成 500。后者那一族由 **A280**（待拍，状态码
是对外契约）按 L272 现量钉死（`dedup` / `benchmark` 的非有限浮点四形全部 500），按父类
放行会把那些格子一并改掉——所以下面 `TestFunnelNarrowness` 既钉「新类型落 400」，也钉
「父类成员仍落 500」，两格缺一格都读不到本轮的边界。

**为什么不 `str()` 归一**：归一在只读测量一侧顶多是含糊读数，在会改写数据的一侧是
静默删行——`POST /api/dataset/merge` 带 `deduplicate=true` 时 `instruction` 为
`1` / `None` / `["a","b"]` 的条目会被折成同一份文本而互为重复，于是只留一条；
`str(None)` 还产出一个名叫 `None` 的四字符假样本。判据沿用 `require_item_field`
（A321 / L238）那句「静默降级比报错危险」。评估一侧 `_as_column` 仍 `str()` 归一，
两侧口径并存记在 **A358** 待拍，本文件把它钉成可见的盲区名册而不是抹平。

**零漂移取证**（Temp 沙箱 `probe3.py` 的 91 格 before/after）：28 格 **500→400**，
改前的 56 格 200 改后仍 200 且响应体逐字节相同——只有 `auto_test` 那 7 格不同，差量
仅 `execution_time_ms` 与 `timestamp` 两个天生不确定的字段。缺列（`no_instruction`）
与空数据集（`empty`）**仍按改前交 200**：本轮只拒「在场且非字符串」，把「缺列」一并
拒掉会把一批现在就成功的请求改成 400，那是 A292 / A354 那一维要拍的事。

**覆盖面不是只手挑的**：Temp `probe7.py` 按 `app.openapi()` 自动枚举全部 POST/GET 端点、
**每格新立一份语料文件**（`probe5.py` 第一版把 `input_file` 与 `output_file` 填成同一个名字，
前一支把语料吃掉，后一支量到的是「文件格式不对」——同一轮 `compare.py` 撞过的键冲突形状），
146 格里这一族仍 500 的只剩一枚：`POST /api/versions/diff` 的不存在 id。它与本轮那一形不同
（异常是 `VersionError`，`raise_boundary_error` 的放行名单按 `DataValidationError` 判），
`TestResidualMembersOfTheSameFunnel` 把它按观测行为钉成现量而不顺手改——拍「哪一类域错误进 400」
是 **A280** 的事。同一类里另一格是 `dataset_filter` 那一臂：那条路径上没有端点，非 GET 的码
由静态挂载决定（见 `PHANTOM_CODE` 的注释），所以它按 `app.routes` 现算而不是抄 405。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走名册字典循环，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_dataset_text_field_l274.py -q --no-cov`
"""

import ast
import json
import uuid
from pathlib import Path

import pytest
from fastapi import HTTPException
from fastapi.testclient import TestClient

from api.deps import (
    INTERNAL_ERROR_DETAIL,
    INVALID_PATH_NAME_DETAIL,
    is_rejected_path_name,
    raise_boundary_error,
    raise_internal_error,
    to_http_error,
)
from api.main import app
from augmentor.exceptions import AugmentorError, DataValidationError, DatasetTextError
from augmentor.validation import require_item_text

ROOT = Path(__file__).resolve().parents[2]
PRODUCT_ROOT = ROOT / "augmentor"

#: 本轮接线的五只咽喉：`(产品文件（仓内相对路径）, 所在函数) -> require_item_text 调用点数`。
#: 按「文件 + 函数」而不是按行号——行号会被插行平移（L217 / L219 / L266 三次同一缺陷类），
#: 而「这一层有没有判」这一事实不该跟着物理行号走。
WIRING_ROSTER = {
    ("augmentor/sampler.py", "analyze_coverage"): 1,
    ("augmentor/quality.py", "batch_score"): 1,
    ("augmentor/dedup.py", "deduplicate"): 1,
    ("augmentor/dataset_ops.py", "_deduplicate"): 1,
    ("api/routes/data.py", "load_data"): 1,
}

#: 声明盲区（写明，不装）：本轮量到「同一形仍会炸」或「口径与本轮相反」的点。
#: 每一格都要求 `require_item_text` **没有**出现在那个函数里——谁去接线，这一格当场红，
#: 必须把该条从盲区挪进 `WIRING_ROSTER` 并同步账本，而不是悄悄留着。
BLIND_ROSTER = {
    ("augmentor/analytics.py", "analyze"):
        "CLI-only 入口（`cli/commands/analysis.py` 独家 import），不在本轮 HTTP 面上；"
        "实测同一形退化语料抛 `TypeError: object of type 'int' has no len()`",
    ("augmentor/quality_report.py", "generate_report"):
        "CLI-only 入口（`cli/commands/quality.py`）；实测抛 "
        "`TypeError: unhashable type: 'list'`",
    ("augmentor/visualizer.py", "generate_statistics"):
        "CLI-only 入口（`cli/commands/analysis.py`）；HTTP 的 `GET /api/visualize/{filename}` 走的是 "
        "`pipeline.visualize_dataset`，本机量不到这里 ⇒ 同一形实测仍抛 "
        "`TypeError: object of type 'int' has no len()`",
    ("api/routes/dataset_tools.py", "_as_column"):
        "口径相反的一侧：在场非字符串走 `str()` 归一而不是拒绝（评估两侧按索引配对，"
        "补空串会凭空造 0 分样本）⇒ 与本轮判据并存，记在 **A358** 待拍",
    ("augmentor/converter.py", "read_conversation_turns"):
        "读入侧的 `str()` 归一（把 JSON 里的数字转成文本再落库），同样是 A358 那一格"
        "的另一半，本轮不改口径",
}

#: `POST /api/dataset/filter` 不是这一族的端点，而是「本轮一格未动」那一档的锚：`/api/dataset/*`
#: 上压根没有 `filter` 这一条，非 GET 请求拿到的码由 `api/main.py:216` 那条静态挂载决定——挂载在
#: （`web/dist` 存在）⇒ `StaticFiles` 部分匹配给出 405，挂载不在 ⇒ 404。把 405 写成常数等于把
#: 「作者机构建过前端」写进判据，干净克隆里会自己变红，所以这一格按 `app.routes` 现算而不是抄读数。
STATIC_MOUNTED = any(type(route).__name__ == "Mount" for route in app.routes)
PHANTOM_CODE = "405" if STATIC_MOUNTED else "404"

#: 现量表：`(探针臂名) -> (方法, 路径模板, 退化那一形的期望码, 合法那一形的期望码, 点名字段)`。
#: 路径模板里的 `{fn}` 换成语料文件名。退化那一形 = 在场且非字符串（四份语料），
#: 合法那一形 = 好数据 / 缺列 / 空数据集（三份语料，按改前形状钉住）。
#: `point_at` 为 None 表示该臂本轮**一格未动**（反向证据：谁把它改成 400 就是越界）。
FACE = {
    "analyze": ("get", "/api/analyze/{fn}", "400", "200", "instruction"),
    "data_load_search": ("get", "/api/data/load/{fn}", "400", "200", "instruction"),
    "quality_report": ("post", "/api/quality/report", "400", "200", "generated"),
    "quality_evaluate": ("post", "/api/quality/evaluate", "400", "200", "generated"),
    "quality_dedup": ("post", "/api/quality/dedup", "400", "200", "instruction"),
    "dataset_compare": ("post", "/api/dataset/compare", "400", "200", "original"),
    "dataset_merge": ("post", "/api/dataset/merge", "400", "200", "instruction"),
    "visualize": ("get", "/api/visualize/{fn}", "200", "200", None),
    "dataset_sample": ("post", "/api/dataset/sample", "200", "200", None),
    "sample_stratified": ("post", "/api/dataset/sample", "200", "200", None),
    "dataset_filter": ("post", "/api/dataset/filter", PHANTOM_CODE, PHANTOM_CODE, None),
    "auto_test": ("post", "/api/system/auto-test", "200", "200", None),
    "profiling": ("post", "/api/quality/profiling", "200", "200", None),
}

#: 四份「在场且非字符串」语料与三份合法语料（与 Temp 沙箱 `probe3.py` 逐字同一批）
BAD_CORPORA = {
    "numeric": [{"instruction": 1, "output": 2}],
    "null_field": [{"instruction": None, "output": {"nested": 1}}],
    "list_field": [{"instruction": ["a", "b"], "output": "c"}],
    "bool_field": [{"instruction": True, "output": False}],
}
LEGAL_CORPORA = {
    "ok_str": [{"instruction": "为什么A", "output": "b"}],
    "no_instruction": [{"output": "b"}],
    "empty": [],
}


def _parse(rel_path):
    return ast.parse((ROOT / rel_path).read_text(encoding="utf-8"))


def _call_sites(rel_path, func_name):
    """AST 数「某个产品函数的函数体里」对 `require_item_text` 的调用点

    不含 import 行与函数定义行：`_names` 只认 `ast.Call` 且函数名是 `ast.Name`。
    """
    tree = _parse(rel_path)
    target = None
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and node.name == func_name:
            target = node
            break
    if target is None:
        raise AssertionError("%s 里找不到函数 %s" % (rel_path, func_name))
    hits = 0
    for node in ast.walk(target):
        if isinstance(node, ast.Call) and getattr(node.func, "id", None) == "require_item_text":
            hits += 1
    return hits


def _write_corpora(tmp_path):
    """七份语料落 pytest tmp 目录（`_roots` 已把该目录设成唯一的允许数据根）"""
    paths = {}
    for name, payload in list(BAD_CORPORA.items()) + list(LEGAL_CORPORA.items()):
        path = Path(tmp_path) / (name + ".json")
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        paths[name] = path.name
    return paths


def _send(client, tmp_path, arm, corpus):
    """按 `FACE` 的模板发一支请求，返回 `(状态码字符串, 响应体文本)`

    语料一律以**裸文件名**进出：`_roots` 把 `AUGMENTOR_DATA_ROOTS` 钉成 `tmp_path`，
    于是路径参数与请求体两侧的解析都落在同一个根里，不必把带 `:` 与反斜杠的绝对路径
    塞进 URL 的路径段（那是探针在沙箱里踩过的另一格）。
    """
    method, template, _bad, _legal, _marker = FACE[arm]
    fn = "%s.json" % corpus
    url = template.replace("{fn}", fn)
    out = "out_%s.json" % corpus
    ok = "ok_str.json"
    if method == "get":
        params = {"search": "a"} if arm == "data_load_search" else None
        resp = client.request("GET", url, params=params)
        return str(resp.status_code), resp.text
    if arm == "dataset_compare":
        body = {"dataset_a": fn, "dataset_b": ok}
    elif arm == "dataset_merge":
        body = {"inputs": [fn, ok], "output_file": out, "deduplicate": True}
    elif arm in ("dataset_sample",):
        body = {"input_file": fn, "output_file": out, "method": "random", "size": 1}
    elif arm == "sample_stratified":
        body = {"input_file": fn, "output_file": out, "method": "stratified",
                "ratio": 0.5, "stratify_key": "instruction"}
    elif arm == "dataset_filter":
        body = {"input_file": fn, "output_file": out, "keyword": "a"}
    else:
        body = {"input_file": fn}
    resp = client.request(method.upper(), url, json=body)
    return str(resp.status_code), resp.text


class TestWiringRoster:
    """接线名册：五只咽喉恰好各一处，且逐只点名"""

    def test_the_roster_is_exact(self):
        seen = {(rel, func): _call_sites(rel, func) for (rel, func) in WIRING_ROSTER}
        assert seen == WIRING_ROSTER

    def test_product_root_files_exist(self):
        for rel in {rel for (rel, _) in list(WIRING_ROSTER) + list(BLIND_ROSTER)}:
            assert (ROOT / rel).is_file(), rel

    def test_blind_roster_is_still_blind(self):
        """盲区名册逐格仍然成立：那一格里没有 `require_item_text`（接线了就当场红）"""
        for (rel, func), reason in BLIND_ROSTER.items():
            assert _call_sites(rel, func) == 0, (rel, func, reason)
            assert reason, (rel, func)

    def test_the_authority_is_called_nowhere_else(self):
        """出厂面（`augmentor/` + `api/` + `cli/`）里 `require_item_text` 的调用点恰好 5 处

        写成总数而不是逐文件枚举：新增接线必然让这一格与 `WIRING_ROSTER` 同时动，
        两道锁一起红，不会出现「名册说一处、实际两处」那种自欺。
        """
        total = 0
        for base in ("augmentor", "api", "cli"):
            for py in sorted((ROOT / base).rglob("*.py")):
                if "__pycache__" in str(py):
                    continue
                rel = py.relative_to(ROOT).as_posix()
                tree = ast.parse(py.read_text(encoding="utf-8"), filename=str(py))
                total += sum(1 for node in ast.walk(tree)
                             if isinstance(node, ast.Call)
                             and getattr(node.func, "id", None) == "require_item_text")
        assert total == sum(WIRING_ROSTER.values()), total


class TestRulerItself:
    """尺子自证：`_call_sites` 数的正是函数体里的调用，import 行与定义行都不算"""

    SAMPLE = (
        "from .validation import require_item_text\n"
        "def other(items):\n"
        "    return require_item_text(items, 'instruction')\n"
        "def target(items):\n"
        "    for x in require_item_text(items, 'output'):\n"
        "        pass\n"
        "    other(items)\n"
    )

    def test_counts_only_body_call_sites(self):
        tree = ast.parse(self.SAMPLE)
        hits = 0
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef,)) and node.name == "target":
                for inner in ast.walk(node):
                    if (isinstance(inner, ast.Call)
                            and getattr(inner.func, "id", None) == "require_item_text"):
                        hits += 1
        assert hits == 1

    def test_a_missing_function_raises_instead_of_passing(self):
        """名册里的函数被改名或删除时，尺子响而不是静默数出 0——否则盲区名册会自己变绿"""
        real = _parse("augmentor/sampler.py")
        assert real is not None
        with pytest.raises(AssertionError):
            _call_sites("augmentor/sampler.py", "no_such_function_at_all")


class TestAuthorityFace:
    """SDK 权威本身：只拒「在场且非字符串」，其余原样放行并返回同一个对象"""

    def test_legal_items_pass_through_by_identity(self):
        items = [{"instruction": "a", "output": "b"}, {"instruction": "c"}]
        assert require_item_text(items, "instruction", "output") is items

    def test_empty_dataset_is_legal(self):
        items = []
        assert require_item_text(items, "instruction") is items

    def test_missing_field_is_not_rejected(self):
        """缺列是合法数据（增强与合并的产物天然多列），本轮只判在场的值"""
        items = [{"output": "b"}, {"instruction": "a"}]
        assert require_item_text(items, "instruction", "output") is items

    def test_every_non_string_shape_is_rejected_and_naming(self):
        shapes = {
            "int": 1, "NoneType": None, "list": ["a"], "dict": {"k": "v"},
            "bool": True, "float": 1.5,
        }
        for kind, value in shapes.items():
            with pytest.raises(DatasetTextError) as caught:
                require_item_text([{"instruction": value}], "instruction")
            text = str(caught.value)
            assert "不是字符串" in text, kind
            assert "'instruction'" in text, kind
            assert "第 0 条" in text, kind
            assert kind in text, (kind, text)

    def test_the_index_of_the_offending_record_is_named(self):
        items = [{"instruction": "a"}, {"instruction": "b"}, {"instruction": 3}]
        with pytest.raises(DatasetTextError) as caught:
            require_item_text(items, "instruction")
        assert "第 2 条" in str(caught.value)

    def test_fields_are_checked_in_the_order_given(self):
        items = [{"instruction": "a", "output": 7}]
        with pytest.raises(DatasetTextError) as caught:
            require_item_text(items, "output", "instruction")
        assert "'output'" in str(caught.value)

    def test_label_appears_in_the_verdict(self):
        with pytest.raises(DatasetTextError) as caught:
            require_item_text([{"original": 1}], "original", label="生成侧")
        assert str(caught.value).startswith("生成侧")

    def test_verdict_is_capped_so_a_huge_value_cannot_flood_the_response(self):
        big = {"instruction": {"blob": "x" * 5000}}
        with pytest.raises(DatasetTextError) as caught:
            require_item_text([big], "instruction")
        assert len(str(caught.value)) < 400

    def test_exception_is_compatible_with_the_existing_hierarchy(self):
        err = DatasetTextError("boom")
        assert isinstance(err, DataValidationError)
        assert isinstance(err, ValueError)
        assert isinstance(err, AugmentorError)
        assert not issubclass(DatasetTextError, TypeError)

    def test_exported_from_package_root(self):
        import augmentor
        assert "DatasetTextError" in augmentor.__all__
        assert augmentor.DatasetTextError is DatasetTextError


class TestFunnelNarrowness:
    """门口三兄弟的归宿：新类型 400，父类成员仍 500（本轮的边界就钉在这一格）"""

    def test_raise_internal_error_lets_the_new_type_through_as_400(self):
        exc = DatasetTextError("数据集 第 0 条的字段 'instruction' 不是字符串，当前是 int: 1")
        with pytest.raises(HTTPException) as caught:
            raise_internal_error(exc)
        assert caught.value.status_code == 400
        assert caught.value.detail == str(exc)

    def test_raise_internal_error_still_folds_the_parent_type_into_500(self):
        """A280 一格未松：`DataValidationError` 的其余成员（非有限浮点旋钮那一族）仍 500"""
        with pytest.raises(HTTPException) as caught:
            raise_internal_error(DataValidationError("threshold 必须在 0 到 1 之间"))
        assert caught.value.status_code == 500
        assert caught.value.detail == INTERNAL_ERROR_DETAIL

    def test_raise_internal_error_does_not_echo_an_unclassified_error(self):
        marker = "D:/deploy/" + uuid.uuid4().hex + "/secret.json"
        with pytest.raises(HTTPException) as caught:
            raise_internal_error(TypeError(marker))
        assert caught.value.status_code == 500
        assert marker not in str(caught.value.detail)

    def test_the_l271_path_name_branch_is_untouched(self):
        """L271 那一支的位次不变：名字形状的判决仍先于未分类 500，且落 400 固定文案"""
        bad = OSError(22, "Invalid argument", "a?b.json")
        assert is_rejected_path_name(bad)
        with pytest.raises(HTTPException) as caught:
            raise_internal_error(bad)
        assert caught.value.status_code == 400
        assert caught.value.detail == INVALID_PATH_NAME_DETAIL

    def test_the_two_older_funnels_already_answer_400(self):
        """另外两只收敛函数天生认得它（父类支），本轮没有为它们写第二套判据"""
        exc = DatasetTextError("数据集 第 1 条的字段 'output' 不是字符串，当前是 bool: True")
        mapped = to_http_error(exc)
        assert mapped.status_code == 400
        assert "第 1 条" in str(mapped.detail)
        with pytest.raises(HTTPException) as caught:
            raise_boundary_error(exc)
        assert caught.value.status_code == 400
        assert "第 1 条" in str(caught.value.detail)


class TestHttpFace:
    """13 臂 × 7 语料 = 91 格的归宿整表（与 Temp 沙箱 before/after 同一批语料）"""

    @pytest.fixture(autouse=True)
    def _roots_and_client(self, monkeypatch, tmp_path):
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        _write_corpora(tmp_path)
        self.client = TestClient(app, raise_server_exceptions=False)

    def _run(self, tmp_path):
        rows = {}
        for arm in FACE:
            for corpus in list(BAD_CORPORA) + list(LEGAL_CORPORA):
                rows[(arm, corpus)] = _send(self.client, tmp_path, arm, corpus)
        return rows

    def test_the_whole_face_matches_the_pinned_table(self, tmp_path):
        rows = self._run(tmp_path)
        assert len(rows) == 13 * 7
        wrong = []
        for (arm, corpus), (code, text) in rows.items():
            want = FACE[arm][2] if corpus in BAD_CORPORA else FACE[arm][3]
            if code != want:
                wrong.append((arm, corpus, code, want, text[:120]))
        assert wrong == [], wrong

    def test_no_cell_of_the_family_is_a_500_anymore(self, tmp_path):
        rows = self._run(tmp_path)
        fifty = [(arm, corpus, rows[(arm, corpus)][1][:120])
                 for (arm, corpus), (code, _t) in rows.items() if code == "500"]
        assert fifty == [], fifty

    def test_the_400_names_record_field_and_type(self, tmp_path):
        """客户端拿到的是能行动的判决：第几条、哪个字段、实际是什么类型"""
        for arm in FACE:
            marker = FACE[arm][4]
            if marker is None:
                continue
            for corpus, payload in BAD_CORPORA.items():
                code, text = _send(self.client, tmp_path, arm, corpus)
                assert code == "400", (arm, corpus, text[:160])
                assert "不是字符串" in text, (arm, corpus)
                assert "'%s'" % marker in text, (arm, corpus, marker)

    def test_legal_and_missing_column_data_keep_answering_200(self, tmp_path):
        """零漂移那一半：本轮没有把任何现在成功的格子改成 400"""
        for arm in FACE:
            want = FACE[arm][3]
            for corpus in LEGAL_CORPORA:
                code, text = _send(self.client, tmp_path, arm, corpus)
                assert code == want, (arm, corpus, text[:160])

    def test_merge_with_deduplicate_does_not_silently_drop_a_record(self, tmp_path):
        """`str()` 归一为什么不行：把两条不同的记录折成同一份文本会静默删行

        两条 `instruction` 分别是 `1` 与 `"1"`，归一后逐字相同 ⇒ 去重只留一条；
        本轮的判决把它们都在门口拒掉（400），产物文件因此根本不会被写。
        """
        path = Path(tmp_path) / "collapse.json"
        path.write_text(json.dumps([{"instruction": 1, "output": "a"},
                                    {"instruction": "1", "output": "b"}],
                                   ensure_ascii=False), encoding="utf-8")
        out = Path(tmp_path) / "collapsed_out.json"
        resp = self.client.post("/api/dataset/merge", json={
            "inputs": [str(path), str(path)], "output_file": str(out),
            "deduplicate": True})
        assert resp.status_code == 400
        assert not out.exists()


class TestRulerBites:
    """注入臂：逐处削掉接线或削掉放行，当场证明对应的钉子会喊红（全部零写盘）"""

    @pytest.fixture(autouse=True)
    def _roots_and_client(self, monkeypatch, tmp_path):
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        _write_corpora(tmp_path)
        self.client = TestClient(app, raise_server_exceptions=False)

    def test_unwiring_the_sampler_sink_returns_the_500(self, monkeypatch, tmp_path):
        import augmentor.sampler as mod
        monkeypatch.setattr(mod, "require_item_text", lambda items, *f, **kw: items)
        code, _text = _send(self.client, tmp_path, "analyze", "numeric")
        assert code == "500"

    def test_unwiring_the_batch_score_sink_returns_the_500(self, monkeypatch, tmp_path):
        import augmentor.quality as mod
        monkeypatch.setattr(mod, "require_item_text", lambda items, *f, **kw: items)
        for arm in ("quality_report", "quality_evaluate", "dataset_compare"):
            code, _text = _send(self.client, tmp_path, arm, "numeric")
            assert code == "500", arm

    def test_unwiring_the_dedup_sink_returns_the_500(self, monkeypatch, tmp_path):
        import augmentor.dedup as mod
        monkeypatch.setattr(mod, "require_item_text", lambda items, *f, **kw: items)
        code, _text = _send(self.client, tmp_path, "quality_dedup", "numeric")
        assert code == "500"

    def test_unwiring_the_merge_sink_returns_the_500(self, monkeypatch, tmp_path):
        import augmentor.dataset_ops as mod
        monkeypatch.setattr(mod, "require_item_text", lambda items, *f, **kw: items)
        code, _text = _send(self.client, tmp_path, "dataset_merge", "numeric")
        assert code == "500"

    def test_unwiring_the_search_branch_returns_the_500(self, monkeypatch, tmp_path):
        """路由里那一处是**延迟 import**（同行包裹落不进去），所以削的是模块属性本身"""
        import augmentor.validation as mod
        monkeypatch.setattr(mod, "require_item_text", lambda items, *f, **kw: items)
        code, _text = _send(self.client, tmp_path, "data_load_search", "numeric")
        assert code == "500"

    def test_widening_the_funnel_back_to_the_parent_breaks_a280_pin(self, monkeypatch, tmp_path):
        """把放行改成按父类判 ⇒ 本轮的 400 仍绿，但 A280 那一格当场红

        这一支证明 `test_raise_internal_error_still_folds_the_parent_type_into_500`
        不是恒真：只要有人图省事按 `DataValidationError` 放行，500 那一格就守不住。
        """
        import api.deps as deps
        monkeypatch.setattr(deps, "DatasetTextError", DataValidationError)
        with pytest.raises(HTTPException) as caught:
            deps.raise_internal_error(DataValidationError("threshold 非有限"))
        assert caught.value.status_code == 400
        code, _text = _send(self.client, tmp_path, "quality_dedup", "numeric")
        assert code == "400"

    def test_narrowing_the_funnel_branch_returns_the_500(self, monkeypatch, tmp_path):
        """削掉 `raise_internal_error` 那一支放行 ⇒ 六只端点回到 500"""
        import api.deps as deps

        class NeverMatch(DatasetTextError):
            pass

        real = deps.DatasetTextError
        monkeypatch.setattr(deps, "DatasetTextError", NeverMatch)
        # 抛出点仍是真类型，`isinstance(真, NeverMatch)` 为假 ⇒ 那一支等于不存在
        assert real is not NeverMatch
        code, _text = _send(self.client, tmp_path, "analyze", "numeric")
        assert code == "500"

    def test_dropping_the_verdict_wording_breaks_the_naming_test(self, monkeypatch, tmp_path):
        """文案臂：判决不点名字段 ⇒ `test_the_400_names_record_field_and_type` 会红"""
        import augmentor.sampler as mod

        def silent(items, *fields, **kwargs):
            raise DatasetTextError("数据有问题")

        monkeypatch.setattr(mod, "require_item_text", silent)
        code, text = _send(self.client, tmp_path, "analyze", "numeric")
        assert code == "400"
        assert "'instruction'" not in text


class TestBlindSpotsAreRealOnTheCliFace:
    """盲区不是自我安慰：逐格量出它今天仍抛非 ValueError 的那一句"""

    @pytest.fixture(autouse=True)
    def _roots(self, monkeypatch, tmp_path):
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))

    def test_analytics_still_raises_type_error_on_the_same_shape(self):
        from augmentor.analytics import analyze_dataset
        with pytest.raises(TypeError):
            analyze_dataset(BAD_CORPORA["numeric"])

    def test_quality_report_still_raises_type_error_on_the_same_shape(self):
        from augmentor.quality_report import generate_quality_report
        with pytest.raises(TypeError):
            generate_quality_report(BAD_CORPORA["list_field"])

    def test_visualizer_statistics_still_raises_type_error_on_the_same_shape(self):
        from augmentor.visualizer import generate_statistics
        with pytest.raises(TypeError):
            generate_statistics(BAD_CORPORA["numeric"])

    def test_the_as_column_side_still_coerces_instead_of_rejecting(self, tmp_path):
        """A358 的另一半今天仍成立：评估一侧把非字符串 `str()` 归一后照常算分（200）"""
        before = Path(tmp_path) / "before.json"
        after = Path(tmp_path) / "after.json"
        before.write_text(json.dumps([{"instruction": "为什么A", "output": "b"}]),
                          encoding="utf-8")
        after.write_text(json.dumps([{"instruction": 1, "output": 2}]), encoding="utf-8")
        resp = TestClient(app, raise_server_exceptions=False).post(
            "/api/dataset/impact", json={"before_file": "before.json",
                                         "after_file": "after.json"})
        assert resp.status_code == 200, resp.text[:200]


class TestResidualMembersOfTheSameFunnel:
    """同一只咽喉上「本轮一格未动」的两格：按观测行为钉成现量，不顺手改（与 A280 同批待拍）

    自动普查（Temp `probe7.py`，逐格独立语料，146 格）之后仍剩一枚 **500**：
    `POST /api/versions/diff` 拿不存在的版本 id。它的异常是 `VersionError`——
    **`ValueError` 的子孙，但不是 `DataValidationError`**，而 `raise_boundary_error`
    放行名单按后者判，于是它落到 `raise_internal_error`。本轮不动它：拍「哪一类域错误进 400」
    是 A280 那一格的事，本轮只把新成员与它的判据维钉住。
    """

    @pytest.fixture(autouse=True)
    def _roots_and_client(self, monkeypatch, tmp_path):
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        _write_corpora(tmp_path)
        self.client = TestClient(app, raise_server_exceptions=False)

    def test_the_phantom_arm_reads_its_code_from_the_app(self, tmp_path):
        """`/api/dataset/filter` 与「任何不存在的 POST 路径」同码 ⇒ 那一格量的不是判决而是挂载形状"""
        phantom = self.client.post("/api/dataset/filter", json={"input_file": "numeric.json"})
        unknown = self.client.post("/api/definitely-not-a-path-xyz", json={})
        assert phantom.status_code == unknown.status_code == int(PHANTOM_CODE)
        assert _send(self.client, tmp_path, "dataset_filter", "numeric")[0] == PHANTOM_CODE

    def test_versions_diff_with_unknown_ids_is_still_500(self):
        """A280 的新成员（本轮不动）：不存在的版本 id ⇒ 500 固定文案，且不回显调用方给的那两个名字"""
        resp = self.client.post("/api/versions/diff",
                                json={"version1": "nope-1", "version2": "nope-2"})
        assert resp.status_code == 500
        assert resp.json()["detail"] == INTERNAL_ERROR_DETAIL
        assert "nope-1" not in resp.text

    def test_version_error_is_a_value_error_yet_answered_500(self):
        """判据维钉死：门口按 `DataValidationError` 放行而不是按 `ValueError` ⇒ A280 要拍的正是这一维"""
        from augmentor.exceptions import VersionError
        assert issubclass(VersionError, ValueError)
        assert not issubclass(VersionError, DataValidationError)
        with pytest.raises(HTTPException) as caught:
            raise_boundary_error(VersionError("版本不存在: nope-1"))
        assert caught.value.status_code == 500
        assert caught.value.detail == INTERNAL_ERROR_DETAIL
        assert "nope-1" not in caught.value.detail
