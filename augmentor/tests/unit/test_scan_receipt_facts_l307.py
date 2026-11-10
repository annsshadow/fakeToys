# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A403 / A401 守卫：回执口径与「被吃掉的东西」的面（L307）

**这一轮收两格**：

* **A403**（L291 立）—— `merge_files(config=None)` 的回执口径：产物侧按
  `MergeConfig()` 默认去重，回执却记 `removed_duplicates: 0` ⇒ **同一件事两种读法**
  （实测 5 条进、产物 1 条出、回执说「没去重」）。收口：**改回执不改产物** ——
  回执报真数，与产物、与 HTTP / CLI 两个产品面（两者都显式传 `MergeConfig`）同口径。
* **A401**（L291 立）—— `multimodal.process_directory` 按 `file_path.stem` 分组 ⇒
  同名**同模态**的后一只**静默覆盖**前一只、子目录**整只静默跳过**，而回执里各占
  0 条 ⇒ 「扫了 4 只、只处理 3 只」不可见。收口：回执加 `warning_count` /
  `warnings` 两格，**逐条点名**（谁被谁覆盖、哪个子目录没扫）。

**这一把尺为什么这样摆**：

1. **A403 的口径断言要独立复算**：断言不用产品自己报的那几个数，而是**重读输入
   文件与产物文件**算一遍真数再比对 ⇒ 产品码退回「报 0」时当场红（不靠「产品报的
   数 == 产品报的数」那种恒真形状）。
2. **A403 的产品面零影响要静态钉住**：HTTP 路由与 CLI 两处 `merge_files` 调用若
   哪天改成传 `None`，那一路的读数就变了 ⇒ 用 AST 咬住「config 实参是
   `MergeConfig(...)`」（与本仓既有那几支接线普查同一形状）。
3. **A401 的告警要点名到具体文件**：只判「有告警」会放过「随便报一条」；断言里
   写死是哪一只被哪一只覆盖、哪个子目录没扫。
4. **A401 的承重臂在回执那一侧**：`generate_report(records, notes=[])` 必须给出
   0 条告警 ⇒ 证明告警**来自扫描**而不是回执自造（否则「回执里硬写一条」也能过）。
5. **A401 的跨模态同名不算告警**：同名**跨模态**融合成一条记录是 docstring 承诺的
   **特性**，把它算成告警就是把特性当缺陷。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走
类内独立用例，两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_scan_receipt_facts_l307.py -q --no-cov`
"""
import ast
import io
import json
from pathlib import Path

import pytest

from augmentor.dataset_ops import DatasetOperations, MergeConfig
from augmentor.data.multimodal import MultimodalProcessor

REPO = Path(__file__).resolve().parents[2]
#: 项目根（`augmentor/`）：下面有 `api/`（HTTP 面）、`augmentor/`（产品包）、`tests/`
AI_DIR = REPO

ITEM = {"instruction": "样例", "input": "", "output": "输出"}


def _write_inputs(tmp_path, datasets):
    """把若干份数据集写成 JSON 文件

    Args:
        tmp_path: pytest 临时目录
        datasets: 数据集列表（每份是一个 dict 列表）

    Returns:
        文件路径列表
    """
    paths = []
    for i, data in enumerate(datasets):
        path = tmp_path / ("in%d.json" % i)
        path.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")
        paths.append(str(path))
    return paths


def _read_json(path):
    """读一份 JSON 文件

    Args:
        path: 文件路径

    Returns:
        解析后的对象
    """
    return json.loads(Path(path).read_text(encoding="utf-8"))


def _make_scan_dir(tmp_path, name="scan"):
    """造一个扫描目录：同名同模态两只 + 同名跨模态一对 + 一只独苗 + 一个子目录

    Args:
        tmp_path: pytest 临时目录
        name: 目录名

    Returns:
        目录路径
    """
    d = tmp_path / name
    d.mkdir(parents=True, exist_ok=True)
    # 同名**同模态**：都是 image ⇒ 后者覆盖前者
    (d / "sample.jpg").write_bytes(b"not-a-real-jpeg")
    (d / "sample.png").write_bytes(b"not-a-real-png")
    # 同名**跨模态**：image + audio ⇒ 融合成一条记录（特性，不是告警）
    (d / "pair.jpg").write_bytes(b"not-a-real-jpeg")
    (d / "pair.wav").write_bytes(b"not-a-real-wav")
    # 独苗
    (d / "only.gif").write_bytes(b"not-a-real-gif")
    # 子目录：整只跳过
    nested = d / "nested"
    nested.mkdir()
    (nested / "deep.png").write_bytes(b"not-a-real-png")
    return d


class TestTheMergeReceiptTellsTheTruth:
    """A403：`config=None` 时回执与产物同口径"""

    def test_config_none_reports_the_true_removed_count(self, tmp_path):
        """SDK 直调 `config=None` ⇒ 回执报真数（改前报 0）"""
        paths = _write_inputs(tmp_path, [[ITEM] * 3, [ITEM] * 2])
        out = tmp_path / "merged.json"

        report = DatasetOperations().merge_files(paths, str(out), None)

        assert report["total_input"] == 5
        assert report["total_output"] == 1, "产物侧照旧去重"
        assert report["removed_duplicates"] == 4, \
            "回执说没去重而产物去了 ⇒ 同一件事两种读法"

    def test_the_receipt_matches_the_artifacts_independently(self, tmp_path):
        """**独立复算**：重读输入与产物算一遍真数，再跟回执比（承重臂）

        这一支不拿产品自己报的那几个数当依据 ⇒ 产品码退回「报 0」时当场红。

        """
        paths = _write_inputs(tmp_path, [[ITEM] * 3, [ITEM] * 2])
        out = tmp_path / "merged.json"

        report = DatasetOperations().merge_files(paths, str(out), None)

        total_in = sum(len(_read_json(p)) for p in paths)
        total_out = len(_read_json(out))
        assert report["removed_duplicates"] == total_in - total_out, \
            "回执的去重数与「输入条数 - 产物条数」对不上"

    def test_the_truncation_cell_stays_zero_when_nothing_was_truncated(self,
                                                                      tmp_path):
        """`config=None` ⇒ `max_items` 是 None ⇒ 截断根本没发生 ⇒ 那一格恒 0"""
        paths = _write_inputs(tmp_path, [[ITEM] * 3, [ITEM] * 2])
        out = tmp_path / "merged.json"

        report = DatasetOperations().merge_files(paths, str(out), None)

        assert report["truncated_by_max_items"] == 0

    def test_the_explicit_config_face_reads_the_same_true_count(self, tmp_path):
        """显式传 `MergeConfig()`（HTTP / CLI 两个产品面走的就是这一条路）⇒ 同一读数"""
        paths = _write_inputs(tmp_path, [[ITEM] * 3, [ITEM] * 2])
        out = tmp_path / "merged.json"

        report = DatasetOperations().merge_files(paths, str(out), MergeConfig())

        assert report["removed_duplicates"] == report["total_input"] - report["total_output"]

    def test_both_product_faces_pass_a_merge_config(self):
        """**接线普查**：HTTP 路由与 CLI 两处 `merge_files` 的 config 实参都不是 None

        哪天有人把某一处改成传 `None`，那一路的读数就与另一路分家 ⇒ 这一支当场红。

        """
        targets = {
            AI_DIR / "api" / "routes" / "dataset_tools.py": "merge_files",
            AI_DIR / "augmentor" / "cli" / "commands" / "data_ops.py": "merge_files",
        }
        for path in targets:
            tree = ast.parse(path.read_text(encoding="utf-8"))
            calls = [
                node for node in ast.walk(tree)
                if isinstance(node, ast.Call)
                and isinstance(node.func, ast.Attribute)
                and node.func.attr == "merge_files"
            ]
            assert calls, "%s 里找不到 merge_files 调用" % path.name
            for call in calls:
                config_node = None
                if len(call.args) > 2:
                    config_node = call.args[2]
                else:
                    for kw in call.keywords:
                        if kw.arg == "config":
                            config_node = kw.value
                assert config_node is not None, \
                    "%s 的 merge_files 调用没传 config" % path.name
                assert _mentions_merge_config(config_node, tree), \
                    "%s 的 merge_files 调用传的 config 不是 MergeConfig(...)" % path.name


def _mentions_merge_config(node, tree):
    """一个表达式最终是不是 `MergeConfig(...)`

    直接是 `MergeConfig(...)` 调用 ⇒ 是；是个名字 ⇒ 在同一棵树里找它的赋值，
    递归看那个赋值的右边是不是。

    Args:
        node: 表达式 AST 节点
        tree: 所在模块的语法树

    Returns:
        bool
    """
    if isinstance(node, ast.Call):
        return isinstance(node.func, ast.Name) and node.func.id == "MergeConfig"
    if isinstance(node, ast.Name):
        for sub in ast.walk(tree):
            if isinstance(sub, ast.Assign):
                for tgt in sub.targets:
                    if isinstance(tgt, ast.Name) and tgt.id == node.id:
                        if _mentions_merge_config(sub.value, tree):
                            return True
    return False


class TestTheScanReceiptNamesWhatWasSwallowed:
    """A401：被覆盖 / 被忽略的东西在回执里被点名"""

    def test_the_overwritten_file_is_named(self, tmp_path):
        """同名同模态 ⇒ 回执点名「谁被谁覆盖」"""
        d = _make_scan_dir(tmp_path)

        proc = MultimodalProcessor()
        records = proc.process_directory(str(d))
        report = proc.generate_report(records, proc.scan_notes)

        assert report["warning_count"] == 2, report["warnings"]
        joined = " | ".join(report["warnings"])
        assert "sample.jpg" in joined and "sample.png" in joined, \
            "被覆盖的那一对没有被点名：%r" % report["warnings"]

    def test_the_skipped_subdirectory_is_named(self, tmp_path):
        """子目录 ⇒ 回执点名「哪个子目录没扫」"""
        d = _make_scan_dir(tmp_path)

        proc = MultimodalProcessor()
        records = proc.process_directory(str(d))
        report = proc.generate_report(records, proc.scan_notes)

        joined = " | ".join(report["warnings"])
        assert "nested" in joined, "被跳过的子目录没有被点名：%r" % report["warnings"]

    def test_the_cross_modality_pair_is_not_a_warning(self, tmp_path):
        """同名**跨模态**（jpg + wav）是融合特性，不算告警"""
        d = _make_scan_dir(tmp_path)

        proc = MultimodalProcessor()
        records = proc.process_directory(str(d))
        report = proc.generate_report(records, proc.scan_notes)

        joined = " | ".join(report["warnings"])
        assert "pair.jpg" not in joined and "pair.wav" not in joined, \
            "跨模态融合被当成了告警：%r" % report["warnings"]

    def test_the_report_does_not_invent_warnings(self, tmp_path):
        """**承重臂**：告警必须来自扫描，回执不许自造（`notes=[]` ⇒ 0 条）"""
        d = _make_scan_dir(tmp_path)

        proc = MultimodalProcessor()
        records = proc.process_directory(str(d))
        report = proc.generate_report(records, [])

        assert report["warning_count"] == 0
        assert report["warnings"] == []

    def test_the_notes_are_reset_by_each_scan(self, tmp_path):
        """告警只描述**那一次**扫描：第二次扫干净目录 ⇒ 0 条"""
        d = _make_scan_dir(tmp_path)
        clean = tmp_path / "clean"
        clean.mkdir()

        proc = MultimodalProcessor()
        proc.process_directory(str(d))
        assert proc.scan_notes, "第一次扫描该有告警"

        records = proc.process_directory(str(clean))
        assert proc.scan_notes == [], "上一批的告警被带到了这一批"
        assert proc.generate_report(records, proc.scan_notes)["warning_count"] == 0

    def test_the_http_face_carries_the_warning_face(self, tmp_path, monkeypatch):
        """HTTP 面：回执带 `warning_count` / `warnings`，且点名了被覆盖的那一只"""
        from fastapi.testclient import TestClient

        from api.main import app

        d = _make_scan_dir(tmp_path)
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))

        response = TestClient(app).post(
            "/api/multimodal/scan", json={"directory": str(d)}
        )

        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["warning_count"] >= 1, payload
        joined = " | ".join(payload["warnings"])
        assert "sample.jpg" in joined and "sample.png" in joined, payload["warnings"]
        assert "nested" in joined, payload["warnings"]
