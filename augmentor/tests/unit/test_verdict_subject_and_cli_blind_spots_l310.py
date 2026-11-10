# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L310 守卫：判决要点名主语（A291）+ CLI 三只盲区接线（A359）+ 五格补标的形状与证据

**这一轮收的是哪一格**（三格同族：都是「用户拿到的那句话/那个格子说不清事实」）：

1. **A291 —— 判决缺主语**：转换端点 `source_format` 的默认是 `None`（＝按扩展名推断），
   但**空串不是 `None`**：实测 `{"source_format": ""}` ⇒ **400**，detail 逐字是
   「无法从  转换到 JSON」——主语位置插进去的就是那个空串，句子里留着两个连续空格，
   **说不出是什么格式转不了**。本轮取账本记的候选②：**保留 400**，只把文案换成能读的
   （点名输入文件 + 说清「省略」与「显式声明」两种形状的差别）。候选①（空串归一成
   `None`）是行为变化，本轮不做。
2. **A359 —— CLI 侧给的是裸 SDK 消息**：`analyze` / `quality-report` 两个子命令在退化
   语料上回 rc=1 + stderr 一句 `错误: object of type 'int' has no len()`，**不点名第几条、
   哪个字段**。本轮取候选①：`require_item_text`（HTTP 面同一权威）接到那三只 SDK 成员上，
   判决变成点名条目下标、字段名与当前类型。接线点不是本轮自创的——l274 那把尺子早就把
   它们写成 `BLIND_ROSTER`（声明盲区），并明写「谁去接线，这一格当场红，必须把该条从
   盲区挪进 `WIRING_ROSTER` 并同步账本」。本守卫与那把尺子对账：两处名册不许漂移。
3. **五格补标**（A29 / A30 / A276 / A288 / A294）：这五行的**工作在早轮就落地了**
   （正文各自点名了落地那一轮与证据），但**定级格从没转成规范的闭合形状** ⇒ 账本自己的
   尺子（l253 的 `MEASURED_LETTERS`）一直把它们算作「未判决」。本轮逐条核过证据之后
   补标，并让守卫把「形状」与「证据」两格都钉住：**形状对了但证据没了，判红**。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走名册字典循环，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_verdict_subject_and_cli_blind_spots_l310.py -q --no-cov`
"""
import ast
import pathlib
import re

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
#: 仓库根（账本与 `docs/` 在子项目之外）—— 证据路径按**仓库根相对**写
REPO = ROOT.parent

#: 五格补标的「证据仍在」清单：`(账本行 ID) -> (证据文件（仓内相对路径）, 证据片段)`
#: 片段按**在文件里出现**判，不按行号——行号会被插行平移，而「这格证据还在不在」
#: 这一事实不该跟着物理行号走。
RESTAMP_EVIDENCE = {
    "A29": ("augmentor/augmentor/search_enhanced.py", "fuzzy_threshold"),
    "A30": ("augmentor/tests/integration/test_cli_merged_commands.py",
            "class TestSearchFilterSurface"),
    "A276": ("docs/augmentor/README.md", "A276"),
    "A288": ("augmentor/web/src/types/api.ts", "size_ratio: number | null"),
    "A294": ("docs/augmentor/API.md", "weighted"),
}

#: 账本两个卷的路径
LEDGER_VOLUMES = ("augmentor/OPTIMIZATION_LOOP.md", "augmentor/OPTIMIZATION_LOOP_2.md")

#: 规范闭合形状：`~~<字母>~~ 已收口（L<轮号>）`。轮号必须**在本卷声明过**
#: （清单行或日志块标题），否则 l253 的可追溯性那一格会把它算进「两卷都查不到」。
CANONICAL_CLOSURE = re.compile(r"^~~[A-Z]{1,2}~~\s*已收口（(L\d+[^）]*)）\s*$")


def _backlog_rows():
    """`(卷名, 行号, 行首 ID, 定级格)` —— 定级格取表格行的最后一格"""
    out = []
    for rel in LEDGER_VOLUMES:
        for lineno, line in enumerate(
                (REPO / rel).read_text(encoding="utf-8").split("\n"), 1):
            hit = re.match(r"^\|\s*([AB]\d{1,3})\s*\|", line)
            if not hit:
                continue
            cells = [c.strip() for c in line.split("|")]
            while cells and cells[-1] == "":
                cells.pop()
            out.append((rel, lineno, hit.group(1), cells[-1] if cells else ""))
    return out


def _declared_rounds(rel):
    """本卷声明过的轮号（清单行 `- [x] **L<n>**` 或日志块标题 `### L<n>`）"""
    text = (REPO / rel).read_text(encoding="utf-8")
    found = set()
    for m in re.finditer(r"^- \[x\] \*\*L(\d+)", text, re.M):
        found.add(int(m.group(1)))
    for m in re.finditer(r"^### L(\d+)", text, re.M):
        found.add(int(m.group(1)))
    return found


def _grade_cells():
    return {ident: (rel, lineno, cell) for rel, lineno, ident, cell in _backlog_rows()}


class TestTheVerdictNamesItsSubject:
    """① A291：空 `source_format` 的判决要点名主语，状态码仍是 400"""

    def test_the_sdk_level_verdict_names_the_input_file(self, tmp_path):
        """SDK 面同一句：判决点名输入文件，且不再是那句留两个连续空格的「无法从  转换到」"""
        from augmentor.converter import DatasetConverter
        from augmentor.exceptions import DataFormatError

        src = tmp_path / "p.json"
        src.write_text('[{"instruction": "a", "output": "b"}]', encoding="utf-8")

        with pytest.raises(DataFormatError) as exc:
            DatasetConverter().convert_file(str(src), str(tmp_path / "out.jsonl"),
                                           source_format="")

        detail = str(exc.value)
        assert "源格式不能是空串" in detail, detail
        assert "p.json" in detail, detail
        assert "无法从  转换到" not in detail, detail

    def test_the_empty_string_is_still_rejected(self, tmp_path):
        """**只改文案，不改可达性**：空串仍是被拒的那一档（不是被悄悄当成「自动」）"""
        from augmentor.converter import DatasetConverter
        from augmentor.exceptions import DataFormatError

        src = tmp_path / "p.json"
        src.write_text('[{"instruction": "a", "output": "b"}]', encoding="utf-8")

        with pytest.raises(DataFormatError):
            DatasetConverter().convert_file(str(src), str(tmp_path / "out.jsonl"),
                                            source_format="")

    def test_the_omitted_source_format_still_goes_through_extension_inference(
            self, tmp_path):
        """「省略」与「显式声明」仍是两种形状：省略 ⇒ 按扩展名推断并成功转换"""
        from augmentor.converter import DatasetConverter

        src = tmp_path / "p.json"
        src.write_text('[{"instruction": "a", "output": "b"}]', encoding="utf-8")

        result = DatasetConverter().convert_file(str(src), str(tmp_path / "out.jsonl"))

        assert result["source_format"] == "json", result


class TestTheCliBlindSpotsAreWired:
    """② A359：那三只 SDK 成员现在给域判决，不是裸异常"""

    def test_the_three_members_reject_the_degenerate_corpora(self):
        """退化语料 ⇒ 判决点名条目下标、字段名与当前类型（不再是 `TypeError`）"""
        from augmentor.analytics import analyze_dataset
        from augmentor.quality_report import generate_quality_report
        from augmentor.visualizer import generate_statistics

        cases = (
            (analyze_dataset, [{"instruction": 1, "output": 2}]),
            (generate_quality_report, [{"instruction": ["a", "b"], "output": "c"}]),
            (generate_statistics, [{"instruction": 1, "output": 2}]),
        )
        for call, corpus in cases:
            with pytest.raises(ValueError) as exc:
                call(corpus) if call is not generate_quality_report else call(corpus)
            detail = str(exc.value)
            assert "不是字符串" in detail, detail
            assert "第 0 条" in detail, detail
            assert "instruction" in detail, detail

    def test_the_legal_corpus_is_untouched(self):
        """合法语料零行为差：仍返回报告，不抛"""
        from augmentor.analytics import analyze_dataset
        from augmentor.quality_report import generate_quality_report
        from augmentor.visualizer import generate_statistics

        good = [{"instruction": "为什么A", "output": "b"}]
        assert analyze_dataset(good) is not None
        assert generate_quality_report(good) is not None
        assert generate_statistics(good)["total_items"] == 1

    def test_the_l274_rosters_agree_with_this_round(self):
        """与 l274 那把尺子对账：那三只成员在接线名册里、不在盲区名册里"""
        from tests.unit.test_dataset_text_field_l274 import (
            BLIND_ROSTER, WIRING_ROSTER,
        )

        wired = {
            ("augmentor/analytics.py", "analyze"),
            ("augmentor/quality_report.py", "generate_report"),
            ("augmentor/visualizer.py", "generate_statistics"),
        }
        assert wired <= set(WIRING_ROSTER), wired - set(WIRING_ROSTER)
        assert not (wired & set(BLIND_ROSTER)), wired & set(BLIND_ROSTER)

    def test_the_authority_has_exactly_the_rostered_number_of_call_sites(self):
        """出厂面里 `require_item_text` 的调用点数 = 接线名册的总和（新增接线必然两处同红）"""
        total = 0
        for base in ("augmentor", "api", "cli"):
            root = ROOT / base
            if not root.is_dir():
                continue
            for py in sorted(root.rglob("*.py")):
                if "__pycache__" in str(py):
                    continue
                tree = ast.parse(py.read_text(encoding="utf-8"), filename=str(py))
                total += sum(1 for node in ast.walk(tree)
                             if isinstance(node, ast.Call)
                             and getattr(node.func, "id", None) == "require_item_text")
        from tests.unit.test_dataset_text_field_l274 import WIRING_ROSTER
        assert total == sum(WIRING_ROSTER.values()), total


@pytest.fixture(scope="module")
def cells():
    """账本两卷的定级格名册（模块级 fixture —— 类级 fixture 定义成实例方法在新版 pytest 会告警）"""
    return _grade_cells()


class TestTheRestampedRows:
    """③ 补标五格：形状是规范闭合形，证据仍在，轮号本卷可查"""

    def test_every_restamped_row_has_the_canonical_closure_shape(self, cells):
        """定级格 = `~~<字母>~~ 已收口（L<轮号>）`，不是裸字母"""
        for ident in sorted(RESTAMP_EVIDENCE):
            assert ident in cells, ident
            _rel, _ln, cell = cells[ident]
            hit = CANONICAL_CLOSURE.match(cell)
            assert hit, (ident, cell)
            assert not cell.startswith("**"), (ident, cell)

    def test_the_cited_round_is_declared_in_the_same_volume(self, cells):
        """轮号必须**本卷可查**（l253 的可追溯性那一格：两卷都查不到 ⇒ 缺陷档）"""
        for ident in sorted(RESTAMP_EVIDENCE):
            rel, _ln, cell = cells[ident]
            hit = CANONICAL_CLOSURE.match(cell)
            assert hit, (ident, cell)
            rounds = [int(x) for x in re.findall(r"L(\d+)", hit.group(1))]
            assert rounds, (ident, cell)
            declared = _declared_rounds(rel)
            for r in rounds:
                assert r in declared, (ident, cell, r, rel)

    def test_the_evidence_is_still_in_the_tree(self):
        """形状对了但证据没了 ⇒ 判红（补标要有据，不是抄一遍字面）"""
        for ident in sorted(RESTAMP_EVIDENCE):
            rel, needle = RESTAMP_EVIDENCE[ident]
            path = REPO / rel
            assert path.is_file(), (ident, rel)
            assert needle in path.read_text(encoding="utf-8"), (ident, rel, needle)
