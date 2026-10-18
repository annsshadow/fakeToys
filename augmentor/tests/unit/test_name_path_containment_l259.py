# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L259 / 续 L258：把「名字拼进路径」那一族的**剩余落点**接进同一把权威，并给这一族装上闭族普查

L258 装上了权威（`augmentor/validation.require_within_dir`）并接了十个落点，但它的名册是
**那一轮那五个文件**的名册（`test_name_path_containment_l258.py` 按文件逐格量，10 处 / 5 个文件），
不是这一族的闭包。本轮先把闭包量出来，再补落点，最后把「闭包」本身钉成判据。

改前实测（仓外探针 `Temp/l259_probe.py`，改后复量 `Temp/l259_after.py`，只在沙箱目录读写）：

- `FAISSDB(collection="../escaped", storage_dir=<tmp>/db).persist()` 把 `escaped_vectors.npy`
  与 `escaped_meta.json` 写到 **`storage_dir` 之外**（实测目录内 `[]`、目录外那两份文件），
  `load()` 从目录外读回内容（实测 `faiss_load_outside_ids == ["x"]`）。
  这一处 HTTP **两步可达**：`vector` 在 `WRITABLE_SECTIONS` 里（`api/routes/config.py:101`，
  L277 给该节新添的目录值白名单闸排在它下面两行，所以行号由 100 重钉为 101），
  而 `VectorConfig` 对 `vector.collection` **只过 `require_string`**（`augmentor/config.py`
  那一节：判非空、不判包含）⇒ `POST /api/config` 写 `collection: ../escaped` 拿 200，
  之后 `POST /api/augment` 在 `vector.enabled` 为真时走
  `augment_dataset` → `build_vector_index` → `semantic.persist_index` → `FAISSDB.persist`。
  读侧同一条名字走 `GET /api/dataset/vector-status` → `index_status` → `load()`。
  **L277 起这一条只剩半截**：同一节里「目录语义」的键 `storage_dir` 已经被 A341 接进
  数据根白名单（越界回 400/403，见 `tests/unit/test_config_dir_containment_l277.py`），
  本守卫的可达性主张因此**只坐在 `collection` 这只名字键上** —— 那正是 SDK 侧这四道权威
  仍然承重、不许被摘掉的原因。
  改后：`persist` 的判决排在 `mkdir` **之前**（实测坏集合名之后目录根本没被创建），
  `load` 与 `index_status` 一并出声，磁盘上零字节越界。
- `ExperimentTracker(storage_dir=<tmp>/exp).start_experiment("../trapped", ...)` 把
  `trapped.json` 写到 `storage_dir` **之外**（实测目录外一份、`exp` 目录压根没长出来）。
  这一处的 `DataValidationError` 落在 `_save_experiment` 那句 `except Exception: logger.error`
  的 **try 之外**（`experiment_path = self._get_experiment_path(...)` 在 try 之前一行），
  所以坏 ID 是出声而不是「保存失败」被咽下——不是 A340 那一族，本守卫按 AST 钉住这个位置。
- `DataVisualizer._output_path("../../evil/plot.png")` 交出 `<output_dir>\\..\\..\\evil\\plot.png`。
  **这一档是给「自己写包含判据」的人准备的陷阱**：实测那个**未规范化**的路径对
  `output_dir` 的 `is_relative_to` 判 `True`，而它的真实落点在沙箱根之外。包含判据必须
  在 `resolve()` **之后**做，这正是权威里那一格的作用；手抄一份浅判据会写出一个恒真的尺子。
  当前请求面与 CLI 都只给目录、不给 `output_file`（`cli/commands/analysis.py` 传 `--output-dir`）
  ⇒ 这一处按现量是**潜在**缺口而不是可利用的洞，补的是那道缺失的闸。
- `pipeline.export_dataset` 的 `formats` 元素**直接来自 HTTP 请求体**
  （`api/routes/export.py:56` 的请求模型 + `:92-94` 原样传下去），进
  `Path(output_dir) / f"train_data_{name}.json"`。改前实测挡住越界的**不是这一处自己的闸**，
  而是下游的枚举强制（`ValueError: '../../evilfmt' is not a valid ExportFormat`）：
  `ExportFormat` 13 个成员的值**无一含 `/`、`\\` 或 `..`**（本守卫逐档点名），所以名字拼出去
  的那一档今天恰好不可利用；只要有人新增一只带分隔符的成员，或把枚举强制改成透传，
  这一处立刻写出去。改后判决换成 `DataValidationError`（同为 `ValueError` 子类，
  捕获面不变），HTTP 面那条门口仍走 `raise_internal_error` ⇒ **仍回 500**，
  A280（一刀切收敛成 400）不在本轮处置面，本守卫把这一格按实测钉住而不是顺手改掉。

**本轮接的四只文件**（全部引同一把权威，零手抄）：`vector/faiss.py`（4 处）、
`tracker.py`（1 处）、`pipeline.py`（1 处）、`visualizer.py`（1 处）⇒ 全族 17 处 / 9 个文件。

**判据八组**：

1. **闭族普查**：`require_within_dir` 的调用点按「文件 → 次数」与实测名册相等（17 处 / 9 个文件）；
   残余的「目录 / 名字」拼接（AST 里 `Div`，左侧名字以 `dir`/`root`/`base`/`path` 收尾、右侧是
   f-string 或裸名字）**恰好四处且逐处点名处置**——权威自己那一行、L231 备份名的手判、
   `cache` 的 md5 十六进制摘要（形状上不可越界）、`visualizer` 那句**上一行刚判过**的返回拼接。
   扫描器自己有一支合成源文的牙齿臂：新的 `dir / f"{name}"` 必须被它看见，
   而 `require_within_dir("x", f"{name}.json", dir)` 那种写法必须**不**被它当成残余。
2. 版本/向量面：`persist` 与 `load` 对越界集合名一起出声，且拦下之后目录内外**零字节**；
   合法集合名照旧落在 `storage_dir` 内并可回读（`count` 不变，零行为漂移）。
3. 断点/实验面：`start_experiment("../trapped")` 出声且目录外零字节；合法 ID 写读往返照旧；
   `_get_experiment_path` 的调用点在 `_save_experiment` 的吞异常 try **之外**（AST 判）。
4. 可视化面：越界文件名出声且**不建目录**；合法名的返回形状**一字不变**——绝对 `output_dir`
   仍给绝对、相对 `output_dir` 仍给相对（本轮只借那句判决，不改响应面）。
5. 管道导出面：`formats` 越界出声、目录外零字节；两档合法格式的键与落点照旧；
   `ExportFormat` 十三档值逐一验「不含分隔符」（这一档是「本轮之前为什么不可利用」的证据）。
6. 配置面（可达性，结构判据）：`vector` 确在可写节名册里；沙箱 YAML 的 `collection: ../escaped`
   经 `load_config` **原样存活**（那道门只判非空）；`augment_dataset` 里 `build_vector_index`
   确实坐在 `if self.config.vector.enabled` 那一档之内（AST 判，不是读到就信）。
   **L277 起再加一格**：`vector` 节里被接进数据根白名单的键**恰好只有** `storage_dir`
   （`collection` 不在其中）——这一格钉的是「配置侧关上的是目录那扇门，名字那扇门仍然开着」，
   所以 SDK 侧那四道权威不许跟着被摘。
7. HTTP 边界：`GET /api/dataset/vector-status` 在越界集合名下拿 **400**（既有 `to_http_error`
   的 `ValueError→400` 映射，本轮没有新造状态码），正文是 SDK 那句判决且**不含部署路径**；
   合法档位仍 200 且键集不变；`POST /api/data/export` 的 `formats` 越界按实测仍 **500**
   （A280 待拍）且输出目录外零产物，合法档 200。
8. 尺子有牙齿：四只新接线各做一次**内存里的**零写入注入（把模块属性 `require_within_dir`
   换成裸拼接），对应判据当场红——证明那一处调用点真的在读这把权威。

**八组全部是结构判据、磁盘事实判据或状态码判据，不量墙钟**（L239 / A286 的口径）。
"""

import ast
import json
from pathlib import Path

import numpy as np
import pytest

from augmentor.config import load_config
from augmentor.exceptions import DataValidationError
from augmentor.pipeline import AugmentorPipeline
from augmentor.semantic import index_status, persist_index
from augmentor.tracker import ExperimentTracker
from augmentor.vector.faiss import FAISSDB
from augmentor.visualizer import DataVisualizer

REPO = Path(__file__).resolve().parents[2]
SDK = REPO / "augmentor"

#: 现量（L259）：逐文件的 `require_within_dir(...)` **调用点**数，不含 import 行与定义。
#: 前五行是 L258 那一份名册（一字未动），后四行是本轮接的落点。
MEASURED_CALL_SITES = {
    "augmentor/checkpoint.py": 3,
    "augmentor/dataset_ops.py": 1,
    "augmentor/export.py": 2,
    "augmentor/pipeline.py": 1,  # L259：`export_dataset` 的 `formats` 元素
    "augmentor/tracker.py": 1,  # L259：`_get_experiment_path`
    "augmentor/vector/faiss.py": 4,  # L259：`persist` 两份 + `load` 两份
    "augmentor/version_control.py": 3,
    "augmentor/versioning.py": 1,
    "augmentor/visualizer.py": 1,  # L259：`_output_path`
}

#: 现量（L259）：SDK 包内**没有**走权威的「目录 / 名字」拼接，逐处点名与其处置理由。
#: **L277 重量（键里的行号跟着产品码走）**：四处拼接**还是那四处**（文件、形状、处置一字未动），
#: 只是两处的**行号**被本轮的产品改动顶上去了 —— `cache.py:219 → 245`（DiskCache 的重入锁 +5 行、
#: MemoryCache 淘汰旧键的 `KeyError` 修复 +19 行、模块头 +2 行，合计 +26 行），
#: `validation.py:1021 → 1026`（`require_seconds` +2 / `require_positive` +2 / `require_ratio` +1，
#: 全是 L277 给超大整数补的 `OverflowError` 兜底，合计 +5 行）。两侧行号都用
#: `git show HEAD:…` 与 `ast` 现量对过，不是按「应该移了几行」推的。
#: **L279 重量**：`cache.py:245 → 262`（净 +17 行，全部落在 `_get_cache_path` **之前**：
#: `_synchronized` 整块从 176 行搬到 `class MemoryCache` 之前并重写 docstring、`MemoryCache`
#: 建实例锁、七个公开入口各加一枚装饰器；`_evict` 的口径改写走同行净零）。拼接本身一格未动，
#: 处置理由一字未改 —— 这一档红的归因只有「行号漂了」，没有「多了/少了拼接」。
#: 这一格记的是** census 的位置**而不是判据的松紧：名册仍按集合相等判，多出第六处拼接、
#: 或少一处、或行号漂了都红。**L280 重量**：`cache.py:262 → 268`（净 +6 行，同样全部落在
#: `_get_cache_path` **之前**：`_load_metadata` 把 `exists()` 后 `open()` 换成直接开文件 +
#: `except FileNotFoundError`，docstring 两行；拼接与处置理由一格未动，本行读数由守卫自己
#: 现量得到（红读数逐字是 `At index 1 diff: 'augmentor/cache.py:268' != 'augmentor/cache.py:262'`）。
MEASURED_RESIDUAL_JOINS = {
    "augmentor/backup.py:117": "L231 给备份名落下的手判（`.parents` 那一档），与权威同口径",
    "augmentor/cache.py:268": "键是 md5 十六进制摘要，形状上不含分隔符与 `..` ⇒ 惰性",
    "augmentor/validation.py:1026": "权威自己那一行 `root / candidate`（判完才拼）",
    "augmentor/visualizer.py:83": "返回的那句拼接：上一行刚刚用权威判过同一个 `output_file`",
}

#: 现量（L259）：`ExportFormat` 全部 13 个成员的值——**无一含 `/`、`\\` 或 `..`**。
#: 这一档是「`pipeline.export_dataset` 在接线之前为什么不可利用」的证据：挡住名字的是
#: 枚举强制，不是那一句拼接。
MEASURED_ENUM_VALUES = [
    "json", "jsonl", "csv", "tsv", "alpaca", "sharegpt", "chatml",
    "llama_factory", "vicuna", "belle", "openai", "huggingface", "raw",
]

ITEMS = [{"instruction": "猫喜欢吃什么食物", "output": "猫喜欢吃鱼肉和猫粮"}]


def _tree(rel: str) -> ast.Module:
    return ast.parse((REPO / rel).read_text(encoding="utf-8"))


def _call_count(rel: str) -> int:
    return sum(
        1
        for node in ast.walk(_tree(rel))
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name)
        and node.func.id == "require_within_dir"
    )


def _tail_name(node) -> str:
    if isinstance(node, ast.Name):
        return node.id
    if isinstance(node, ast.Attribute):
        return node.attr
    return ""


def _residual_joins(tree: ast.Module, rel: str) -> list:
    """返回这棵树里的「目录 / 名字」拼接（`dir / f"{name}.json"`、`dir / name`）"""
    hits = []
    for node in ast.walk(tree):
        if not (isinstance(node, ast.BinOp) and isinstance(node.op, ast.Div)):
            continue
        left = _tail_name(node.left).lower()
        if not left.endswith(("dir", "root", "base", "path")):
            continue
        if isinstance(node.right, (ast.JoinedStr, ast.Name)):
            hits.append(f"{rel}:{node.lineno}")
    return hits


def _all_residual_joins() -> list:
    hits = []
    for path in sorted(SDK.rglob("*.py")):
        hits += _residual_joins(
            ast.parse(path.read_text(encoding="utf-8")),
            path.relative_to(REPO).as_posix(),
        )
    return sorted(hits)


def _vector_items_file(tmp_path: Path) -> str:
    path = tmp_path / "l259_items.json"
    path.write_text(json.dumps(ITEMS), encoding="utf-8")
    return str(path)


def _add(db: FAISSDB) -> None:
    db.add_vectors(
        [np.array([1.0, 0.0, 0.0, 0.0], dtype=np.float32)],
        metadata=[{"text": "猫喜欢吃什么食物"}],
        ids=["i1"],
    )


class TestTheFamilyCensusIsClosed:
    def test_call_sites_equal_the_measured_roster_per_file(self):
        measured = {rel: _call_count(rel) for rel in MEASURED_CALL_SITES}
        assert measured == MEASURED_CALL_SITES, measured

    def test_total_is_seventeen_across_nine_files(self):
        assert sum(MEASURED_CALL_SITES.values()) == 17
        assert len(MEASURED_CALL_SITES) == 9

    def test_the_residual_joins_are_the_named_four(self):
        """闭族判据：SDK 内不走权威的「目录 / 名字」拼接只剩那四处，逐处带处置"""
        found = _all_residual_joins()
        assert found == sorted(MEASURED_RESIDUAL_JOINS), found

    def test_the_scanner_would_see_a_new_escape(self):
        """扫描器有牙齿：合成源文里那一档拼接必须命中，走了权威的写法必须不命中"""
        tree = ast.parse(
            "class X:\n"
            "    def p(self, name):\n"
            "        return self.output_dir / f\"{name}.json\"\n"
        )
        assert _residual_joins(tree, "synthetic.py") == ["synthetic.py:3"]
        guarded = ast.parse(
            "def q(name, output_dir):\n"
            "    return require_within_dir(\"名字\", f\"{name}.json\", output_dir)\n"
        )
        assert _residual_joins(guarded, "synthetic.py") == []

    @pytest.mark.parametrize("rel", sorted(MEASURED_CALL_SITES))
    def test_each_guarded_module_imports_the_authority_not_a_local_copy(self, rel):
        imported = [
            alias.name
            for node in ast.walk(_tree(rel))
            if isinstance(node, ast.ImportFrom)
            for alias in node.names
        ]
        assert "require_within_dir" in imported, rel
        source = (REPO / rel).read_text(encoding="utf-8")
        assert "def require_within_dir" not in source, f"{rel} 里手抄了第二份权威"


class TestTheVectorFace:
    def test_persist_cannot_write_outside_the_storage_dir(self, tmp_path):
        root = tmp_path / "db"
        db = FAISSDB(dimension=4, collection="../escaped", storage_dir=str(root))
        _add(db)
        with pytest.raises(DataValidationError) as exc:
            db.persist()
        assert "之外" in str(exc.value)
        assert list(tmp_path.glob("escaped*")) == []

    def test_a_bad_collection_creates_no_directory(self, tmp_path):
        """判决排在 `mkdir` 之前：坏集合名不该在盘上留下一个空目录（L45 的同一条纪律）"""
        root = tmp_path / "db"
        db = FAISSDB(dimension=4, collection="../escaped", storage_dir=str(root))
        _add(db)
        with pytest.raises(DataValidationError):
            db.persist()
        assert not root.exists()

    def test_load_cannot_read_outside_the_storage_dir(self, tmp_path):
        secrets = tmp_path / "secrets"
        secrets.mkdir()
        np.save(secrets / "v_vectors.npy", np.zeros((1, 4), dtype=np.float32))
        (secrets / "v_meta.json").write_text(
            json.dumps({"ids": ["secret-id"], "metadata": [], "dimension": 4}),
            encoding="utf-8",
        )
        db = FAISSDB(dimension=4, collection="../secrets/v", storage_dir=str(tmp_path / "db"))
        with pytest.raises(DataValidationError):
            db.load()

    def test_a_legitimate_collection_still_lands_inside_and_rereads(self, tmp_path):
        root = tmp_path / "db"
        db = FAISSDB(dimension=4, collection="default", storage_dir=str(root))
        _add(db)
        db.persist()
        assert sorted(p.name for p in root.glob("*")) == [
            "default_meta.json",
            "default_vectors.npy",
        ]
        back = FAISSDB(dimension=4, collection="default", storage_dir=str(root))
        assert back.load() is True
        assert back.count() == 1

    def test_index_status_follows_the_same_gate(self, tmp_path):
        root = tmp_path / "db"
        persist_index(
            ITEMS, backend="faiss", dimension=4, collection="default", storage_dir=str(root)
        )
        assert index_status(
            backend="faiss", dimension=4, collection="default", storage_dir=str(root)
        )["count"] == 1
        with pytest.raises(DataValidationError):
            index_status(
                backend="faiss", dimension=4, collection="../escaped", storage_dir=str(root)
            )


class TestTheTrackerFace:
    def test_start_experiment_cannot_write_outside_the_storage_dir(self, tmp_path):
        root = tmp_path / "exp"
        tracker = ExperimentTracker(storage_dir=str(root))
        with pytest.raises(DataValidationError) as exc:
            tracker.start_experiment("../trapped", "v1", "m1")
        assert "之外" in str(exc.value)
        assert list(tmp_path.glob("trapped*")) == []

    def test_a_legitimate_experiment_roundtrips(self, tmp_path):
        root = tmp_path / "exp"
        tracker = ExperimentTracker(storage_dir=str(root))
        tracker.start_experiment("run_1", "v1", "m1")
        tracker.log_metric("acc", 0.5)
        tracker.end_experiment()
        assert sorted(p.name for p in root.glob("*")) == ["run_1.json"]
        assert tracker.load_experiment("run_1").final_metrics == {"acc": 0.5}

    def test_the_escape_verdict_is_not_swallowed_by_the_saving_try(self):
        """坏 ID 出声而不是「保存失败」被咽下 ⇒ 不是 A340 那一族

        `_save_experiment` 的 `except Exception: logger.error` 会吞掉写失败；本判据钉的是
        「取路径」那一行**坐落在那个 try 之外**，所以判决能传到调用方。
        """
        tree = _tree("augmentor/tracker.py")
        savers = [
            node
            for node in ast.walk(tree)
            if isinstance(node, ast.FunctionDef) and node.name == "_save_experiment"
        ]
        assert len(savers) == 1
        inside_try = {
            child.lineno
            for node in ast.walk(savers[0])
            if isinstance(node, ast.Try)
            for child in ast.walk(node)
            if isinstance(child, (ast.Call, ast.Expr))
        }
        call_lines = [
            node.lineno
            for node in ast.walk(savers[0])
            if isinstance(node, ast.Call)
            and _tail_name(node.func) == "_get_experiment_path"
        ]
        assert call_lines, "改形：`_save_experiment` 不再调 `_get_experiment_path`"
        assert all(line not in inside_try for line in call_lines), (call_lines, inside_try)


class TestTheVisualizerFace:
    def test_a_bad_file_name_is_rejected_and_builds_nothing(self, tmp_path):
        root = tmp_path / "viz"
        viz = DataVisualizer(output_dir=str(root))
        with pytest.raises(DataValidationError) as exc:
            viz._output_path("../../evil/plot.png")
        assert "之外" in str(exc.value)
        assert not root.exists()

    def test_an_absolute_output_dir_keeps_an_absolute_answer(self, tmp_path):
        root = tmp_path / "viz"
        got = DataVisualizer(output_dir=str(root))._output_path("wordcloud.png")
        assert str(got) == str(root / "wordcloud.png")

    def test_a_relative_output_dir_still_answers_relative(self, tmp_path, monkeypatch):
        """本轮只借那句判决，不把响应面从相对改成绝对（响应面变更要拍板）"""
        monkeypatch.chdir(tmp_path)
        got = DataVisualizer(output_dir="rel_out")._output_path("wordcloud.png")
        assert str(got) == str(Path("rel_out") / "wordcloud.png")
        assert not got.is_absolute()

    def test_a_naive_containment_check_would_have_called_that_escape_inside(self, tmp_path):
        """浅判据的陷阱：未规范化的 `..` 路径对 base 的 `is_relative_to` 是 `True`

        这一支不判产品，判的是「为什么必须用权威而不是自己写一句包含」：
        `resolve()` 之后落点在根外，`resolve()` 之前看着在根内。
        """
        root = tmp_path / "viz"
        shallow = root / "../../evil/plot.png"
        assert shallow.is_relative_to(root), "浅判据今天不成立了，本支的立论要重写"
        assert root.resolve() not in shallow.resolve().parents


class TestThePipelineExportFace:
    def _pipeline(self) -> AugmentorPipeline:
        return AugmentorPipeline()

    def test_an_escaping_format_name_is_rejected_with_nothing_written(self, tmp_path):
        out = tmp_path / "pod"
        with pytest.raises(DataValidationError) as exc:
            self._pipeline().export_dataset(
                _vector_items_file(tmp_path), str(out), ["../../evilfmt"]
            )
        assert "之外" in str(exc.value)
        assert list(tmp_path.glob("train_data_..*")) == []

    def test_legitimate_formats_keep_their_keys_and_their_landing(self, tmp_path):
        out = tmp_path / "pod"
        results = self._pipeline().export_dataset(
            _vector_items_file(tmp_path), str(out), ["jsonl", "alpaca"]
        )
        assert sorted(results) == ["alpaca", "jsonl"]
        for path in results.values():
            assert Path(path).is_relative_to(out.resolve()), path
            assert Path(path).exists()

    def test_the_enum_values_carry_no_separator(self):
        """改前那一档不可利用的证据：挡住 `formats` 的是枚举强制，13 个成员值无一含分隔符

        如果将来有人新增一只带 `/` 的成员，这一支会当场红——那一刻 `export_dataset`
        里那句拼接就真的需要它现在这道闸（本轮补的就是它）。
        """
        from augmentor.export_enhanced import ExportFormat

        values = [f.value for f in ExportFormat]
        assert values == MEASURED_ENUM_VALUES, values
        assert [v for v in values if "/" in v or "\\" in v or ".." in v] == []

    def test_the_verdict_type_is_still_a_value_error(self, tmp_path):
        """换的是判决来源，不是异常族：既有 `except ValueError` 的调用面不受影响"""
        with pytest.raises(ValueError):
            self._pipeline().export_dataset(
                _vector_items_file(tmp_path), str(tmp_path / "pod"), ["../../evilfmt"]
            )


class TestTheConfigSurfaceStaysOpen:
    """可达性的四块结构证据：配置能写进这个键、加载不判它、增强链真会读它、配置侧只关目录那扇门"""

    def test_vector_is_a_writable_section(self):
        from api.routes.config import WRITABLE_SECTIONS

        assert "vector" in WRITABLE_SECTIONS, WRITABLE_SECTIONS

    def test_the_directory_key_is_gated_while_the_name_key_is_not(self):
        """L277 起：可写节里被接进数据根白名单的键**恰好只有** vector 的 storage_dir

        为什么这一格住在本守卫里而不是只交给新守卫：L259 这一族的立论是「名字拼进路径之后由
        SDK 那四道权威收口」，而 A341 关的是**目录**那扇门。哪天有人把 `collection` 也当成
        「配置侧已经管住了」顺手摘掉 SDK 的闸，这一格会先红，逼改的人回来看第 2 组。
        """
        from api.routes import config as config_routes

        gated = dict(config_routes._DIR_VALUED_KEYS)
        assert gated == {"vector": "storage_dir"}, gated
        assert "collection" not in set(gated.values())
        assert set(gated) <= set(config_routes.WRITABLE_SECTIONS)

    def test_the_loader_keeps_an_escaping_collection_as_is(self, tmp_path):
        """`VectorConfig` 对 `vector.collection` 只过 `require_string` ⇒ 门在 SDK 侧，不在配置侧"""
        cfg = tmp_path / "cfg.yaml"
        cfg.write_text(
            "vector:\n"
            "  enabled: true\n"
            "  backend: faiss\n"
            "  dimension: 4\n"
            "  storage_dir: vdb\n"
            "  collection: ../escaped\n",
            encoding="utf-8",
        )
        loaded = load_config(str(cfg))
        assert loaded.vector.collection == "../escaped"
        assert loaded.vector.enabled is True

    def test_augment_dataset_builds_the_index_under_the_enabled_gate(self):
        tree = _tree("augmentor/pipeline.py")
        bodies = [
            node
            for node in ast.walk(tree)
            if isinstance(node, ast.If)
            and "vector.enabled" in ast.unparse(node.test)
        ]
        assert bodies, "改形：`if self.config.vector.enabled` 不在了，可达性要重写"
        assert any(
            isinstance(child, ast.Call) and _tail_name(child.func) == "build_vector_index"
            for node in bodies
            for child in ast.walk(node)
        ), ast.unparse(bodies[0].test)


class TestTheHttpBoundary:
    """端点面：越界的集合名不再交出目录外的索引，合法档的键集与状态码一字不变

    配置一律用**内存里的** `AppConfig`（`load_config` 读仓库那份、再 setattr 指向 tmp），
    `deps._pipeline` 换成 stub ⇒ 不碰服务自己那份 `config.yaml`，也不往仓库 `data/` 写东西。
    """

    @staticmethod
    def _stub(tmp_path, collection, monkeypatch):
        import api.deps as deps

        config = load_config(str(REPO / "config.yaml"))
        config.vector.storage_dir = str(tmp_path / "vectors")
        config.vector.collection = collection
        stub = type("PipelineStub", (), {})()
        stub.config = config
        monkeypatch.setattr(deps, "_pipeline", stub)
        return config

    @staticmethod
    def _client():
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app, raise_server_exceptions=False)

    def test_vector_status_on_an_escaping_collection_is_400_not_a_disclosure(
        self, tmp_path, monkeypatch
    ):
        secrets = tmp_path / "secrets"
        secrets.mkdir()
        np.save(secrets / "v_vectors.npy", np.zeros((1, 4), dtype=np.float32))
        (secrets / "v_meta.json").write_text(
            json.dumps({"ids": ["secret"], "metadata": [], "dimension": 4}), encoding="utf-8"
        )
        self._stub(tmp_path, "../secrets/v", monkeypatch)
        response = self._client().get("/api/dataset/vector-status")
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "之外" in detail, detail
        assert str(tmp_path.resolve()) not in detail, detail
        assert not (tmp_path / "vectors").exists()

    def test_vector_status_legitimate_shape_is_untouched(self, tmp_path, monkeypatch):
        self._stub(tmp_path, "default", monkeypatch)
        body = self._client().get("/api/dataset/vector-status").json()
        assert sorted(body) == [
            "backend",
            "collection",
            "count",
            "dimension",
            "enabled",
            "exists",
            "storage_dir",
        ], body
        assert body["exists"] is False

    def test_the_export_route_still_answers_500_for_a_boundary_verdict(self, tmp_path, monkeypatch):
        """A280（把这类门口一刀切收敛成 400）不在本轮处置面：这一支钉本轮没改状态码契约

        合法输入先过白名单那道闸（`AUGMENTOR_DATA_ROOTS` 指向 tmp，零写入仓库 `data/`），
        越界的 `formats` 由 `api/routes/export.py` 那句 `raise_internal_error` 收敛成 500，
        正文不带部署路径；磁盘上输出目录之外零产物。
        """
        items = tmp_path / "in.json"
        items.write_text(json.dumps(ITEMS), encoding="utf-8")
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        response = self._client().post(
            "/api/data/export",
            json={
                "input_file": str(items),
                "output_dir": str(tmp_path / "pod"),
                "formats": ["../../evilfmt"],
            },
        )
        assert response.status_code == 500, response.text
        assert str(tmp_path.resolve()) not in response.text
        assert list(tmp_path.glob("train_data_..*")) == []

    def test_the_export_route_legitimate_arm_still_writes_inside(self, tmp_path, monkeypatch):
        items = tmp_path / "in.json"
        items.write_text(json.dumps(ITEMS), encoding="utf-8")
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        body = self._client().post(
            "/api/data/export",
            json={
                "input_file": str(items),
                "output_dir": str(tmp_path / "pod"),
                "formats": ["jsonl"],
            },
        ).json()
        assert body["success"] is True
        assert Path(body["files"]["jsonl"]).is_relative_to(
            (tmp_path / "pod").resolve()
        ), body



class TestTheRulerHasTeeth:
    """四只新接线各一次**内存里的**零写入注入：把权威换成裸拼接，对应判据必须当场失守"""

    @staticmethod
    def _raw(label, value, base):
        return Path(base) / value

    def test_faiss_persist_without_the_gate_writes_the_escape(self, tmp_path, monkeypatch):
        import augmentor.vector.faiss as faiss_mod

        root = tmp_path / "db"
        db = faiss_mod.FAISSDB(dimension=4, collection="../escaped", storage_dir=str(root))
        _add(db)
        monkeypatch.setattr(faiss_mod, "require_within_dir", self._raw)
        db.persist()
        assert (tmp_path / "escaped_vectors.npy").exists(), "注入后仍未写出 ⇒ 第 2 组没有牙齿"

    def test_faiss_load_without_the_gate_reads_the_escape(self, tmp_path, monkeypatch):
        import augmentor.vector.faiss as faiss_mod

        secrets = tmp_path / "secrets"
        secrets.mkdir()
        np.save(secrets / "v_vectors.npy", np.ones((1, 4), dtype=np.float32))
        (secrets / "v_meta.json").write_text(
            json.dumps({"ids": ["secret-id"], "metadata": [], "dimension": 4}), encoding="utf-8"
        )
        db = faiss_mod.FAISSDB(dimension=4, collection="../secrets/v", storage_dir=str(tmp_path / "db"))
        monkeypatch.setattr(faiss_mod, "require_within_dir", self._raw)
        assert db.load() is True
        assert db._ids == ["secret-id"], "注入后仍读不到目录外 ⇒ 第 2 组那一支是恒真的"

    def test_tracker_without_the_gate_writes_the_escape(self, tmp_path, monkeypatch):
        import augmentor.tracker as tracker_mod

        root = tmp_path / "exp"
        tracker = tracker_mod.ExperimentTracker(storage_dir=str(root))
        monkeypatch.setattr(tracker_mod, "require_within_dir", self._raw)
        tracker.start_experiment("../trapped", "v1", "m1")
        tracker.end_experiment()
        assert (tmp_path / "trapped.json").exists(), "注入后仍未写出 ⇒ 第 3 组没有牙齿"

    def test_visualizer_without_the_gate_hands_out_the_escape(self, tmp_path, monkeypatch):
        """`_output_path` 是**函数内**导入权威（见那里的注释），所以注入点在 `augmentor.validation` 上"""
        import augmentor.validation as validation_mod

        root = tmp_path / "viz"
        viz = DataVisualizer(output_dir=str(root))
        monkeypatch.setattr(validation_mod, "require_within_dir", self._raw)
        got = viz._output_path("../../evil/plot.png")
        assert root.resolve() not in got.resolve().parents, "注入没有还原出越界落点"

    def test_pipeline_without_the_gate_hands_the_escape_to_the_writer(self, tmp_path, monkeypatch):
        """闸在位时写手**收不到**路径；闸摘掉后写手收到的就是那句裸拼接的落点

        今天真正挡住「越界格式名写出去」的是 `Exporter.export` 里的枚举强制（第 5 组那一支），
        所以这一支量的是「调用点在读这把权威」，不是「没有权威就能写到目录外」。
        """
        import augmentor.pipeline as pipeline_mod

        items = tmp_path / "in.json"
        items.write_text(json.dumps(ITEMS), encoding="utf-8")
        out = tmp_path / "pod"
        seen = []
        pl = AugmentorPipeline()
        monkeypatch.setattr(
            pl.exporter, "export", lambda data, path, fmt: seen.append(path)
        )
        with pytest.raises(DataValidationError):
            pl.export_dataset(str(items), str(out), ["../../evilfmt"])
        assert seen == [], "闸失效时才会把路径交给写手"

        monkeypatch.setattr(pipeline_mod, "require_within_dir", self._raw)
        pl.export_dataset(str(items), str(out), ["../../evilfmt"])
        assert seen and ".." in Path(seen[0]).parts, seen

