# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L329：A287 与 A335 两笔 S 级收口 —— 两个「对外契约面」的判决

这两笔的共同形状：**契约面里有一格，它要么恒着假值、要么缺了调用方给的那一格**。
两笔都不是「实现写错了」，而是「对外说的那句话不准确」⇒ 修法都在**契约面**，
不在逻辑面。

**A287 —— 质量面那两个恒零的相似度键**

* 改前：`augmentor/comparison.py` 的 `ComparisonResult` 上挂着 `similarity_to_a` /
  `similarity_to_b`，**全仓没有任何赋值点**（dataclass 默认值 + `to_dict()` 两处）
  ⇒ 它们恒为 `0.0`。实测两份**完全不同的**数据，回执里也是 `0.0`。
* 为什么恒零比缺格更坏：`0.0` 会被读成「A 与 B 毫不相似」，而真相是
  「这一档从未被计算」——**一个恒零的数比没有这个数更会骗人**。
* 判决（L329）：**删掉这两格**，相似度的唯一权威留在**重叠面**
  （`overlap_comparison.metrics.similarity_score`）。理由有三条：
  ① 重叠面已经有那一个数（匹配键集合的 Jaccard 相似度），在质量面再挂两个近义数
  就是「一个值两个权威」（A123 / A126 那一族的忌讳）；
  ② 质量面的职责是质量（分 / 通过率 / 长度 / 词汇量），相似度不归它管；
  ③ 接上计算（候选①）要先拍「`similarity_to_a` 是 B 相对 A 还是 A 相对 B」，
  而那正是本轮**不做**的那一格（拍板记录见仓库根 docs/plans 目录下 2026-11-13 那一份）。
* 本文件钉的是**反向棘轮 + 单一权威**：那两个名字不许回到 `comparison.py`、
  不许回到前端类型面；重叠面那一格必须**真在算**（部分重叠 ⇒ 0.5，不是恒 0）。

**A335 —— 备份回执里没有 `name`**

* 改前：回执七键（`backup_id` / `timestamp` / `source_path` / `backup_path` /
  `item_count` / `file_size` / `checksum`），而 `backup_id = name or backup_<时间戳>`
  ⇒ 调用方**分不清「这个 id 是我起的」还是「服务端编的」**，只能靠 `backup_` 前缀去猜
  ——而那只在**没给名字**时成立（自己起一个 `backup_xxx` 名字，前缀这招就认不出来了）。
* 判决（L329）：**回执加第八格 ame\** = **调用方自己起的那个名字**（没给就是 ull\）。
  它**不参与寻址**：`restore` / `delete` 仍只认 `backup_id`。
* 存量索引行（L329 之前写下的那份索引）没有这一格 ⇒ SDK 读边补 `None`、
  HTTP 读边回 `null`，**同一份清单里不许有的行有、有的行没有**。

**四把尺子**（每一把都要能失败）：

1. **契约面**（SDK dataclass + HTTP 回执 + TS 类型 + zod schema）逐格对上；
2. **单一权威**：A287 的相似度只剩重叠面那一格，A335 的 `name` 只从 SDK 那一处来；
3. **行为差**：A287 的部分重叠 ⇒ `similarity_score == 0.5`（真在算）；
   A335 的存量索引行 ⇒ `name is None`（不是缺键）；
4. **注入臂**（三支）：把死键塞回 `to_dict()`、把 `name` 从 `to_dict()` 里摘掉、
   把端点看到的 `create_backup` 换成假实现 ⇒ 三支各自让对应那一格**变样**，
   证明上面那些断言不是恒真。

跑法：`python -m pytest tests/unit/test_contract_faces_l329.py -q --no-cov`
"""

import ast
import json
import pathlib
import re

import pytest
from tests.unit.test_wired_dropdown_choices_l226 import (
    needs_web_src,
    strip_comments,
)

from augmentor.backup import BackupInfo, create_backup
from augmentor.comparison import ComparisonResult

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
TYPES_TS = SRC / "types" / "api.ts"
SCHEMAS_TS = SRC / "schemas" / "full-schemas.ts"
COMPARISON_PY = ROOT / "augmentor" / "comparison.py"
BACKUP_PY = ROOT / "augmentor" / "backup.py"
API_MD = ROOT.parent / "docs" / "augmentor" / "API.md"

#: A287 那两个名字（**不许再出现**）
DEAD_KEYS = ("similarity_to_a", "similarity_to_b")

#: A335：回执的八键
RECEIPT_KEYS = {
    "backup_id", "timestamp", "source_path", "backup_path",
    "item_count", "file_size", "checksum", "name",
}


def _items(prefix: str, count: int) -> list:
    return [{"instruction": f"{prefix}{i}", "input": "", "output": f"a{i}"}
            for i in range(count)]


@pytest.fixture
def http_env(tmp_path, monkeypatch):
    """白名单数据根：两份语料（4 条与 2 条，后者与前者前两条重叠）"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "p.json").write_text(json.dumps(_items("q", 4), ensure_ascii=False),
                                 encoding="utf-8")
    (root / "n.json").write_text(json.dumps(_items("q", 2), ensure_ascii=False),
                                 encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root


class TestA287TheDeadKeysAreGone:
    """A287：质量面那两个恒零键不许再出现"""

    def test_the_sdk_dataclass_has_no_such_fields(self):
        fields = set(ComparisonResult.__dataclass_fields__)
        for dead in DEAD_KEYS:
            assert dead not in fields, f"{dead} 还在 dataclass 上（恒零死面）"

    def test_the_sdk_receipt_has_no_such_keys(self):
        payload = ComparisonResult(
            dataset_a_name="A", dataset_b_name="B",
            dataset_a_count=1, dataset_b_count=1,
        ).to_dict()
        for dead in DEAD_KEYS:
            assert dead not in payload, f"{dead} 还在回执里（恒零死面）"

    def test_the_source_mentions_neither_name(self):
        """反向棘轮：那两个名字**不许回到 `comparison.py` 的活代码里**

        口径：**扫的是活代码，不是注释**。`comparison.py` 里那几行注释必须提到这两个名字
        （解释「为什么删」），所以这一支不能扫整份文本 ⇒ 用 AST 取**标识符与字典键**
        （`ast.Name` / `ast.Attribute` / `ast.arg` / `ast.keyword` / 字典字面量的字符串键），
        注释与 docstring 不在其中。
        """
        tree = ast.parse(COMPARISON_PY.read_text(encoding="utf-8"))
        names = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.Name):
                names.add(node.id)
            elif isinstance(node, ast.Attribute):
                names.add(node.attr)
            elif isinstance(node, ast.arg):
                names.add(node.arg)
            elif isinstance(node, ast.keyword) and node.arg:
                names.add(node.arg)
            elif isinstance(node, ast.Dict):
                for key in node.keys:
                    if isinstance(key, ast.Constant) and isinstance(key.value, str):
                        names.add(key.value)
        for dead in DEAD_KEYS:
            assert dead not in names, f"{dead} 回到了 comparison.py ⇒ 恒零死面复活了"

    def test_the_http_receipt_has_no_such_keys(self, http_env):
        http, _ = http_env
        quality = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "n.json"}).json(
                            )["quality_comparison"]
        for dead in DEAD_KEYS:
            assert dead not in quality, f"{dead} 还在 HTTP 回执里"

    def test_the_similarity_authority_is_the_overlap_face_and_it_computes(self, http_env):
        """单一权威 + 行为差：相似度只剩重叠面那一格，而且**真在算**

        两份数据 4 条 vs 2 条、前两条重叠 ⇒ 匹配键集合 `{q0..q3}` 与 `{q0,q1}`
        ⇒ Jaccard = 2 / 4 = **0.5**（恒 0 的那一面早就被删了）。
        """
        http, _ = http_env
        payload = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "n.json"}).json()
        metrics = payload["overlap_comparison"]["metrics"]
        assert metrics["common_items"] == 2, metrics
        assert metrics["similarity_score"] == 0.5, metrics

    def test_the_quality_face_still_carries_its_own_metrics(self, http_env):
        """删的是那两个死键，**不是**把质量面掏空：质量那几格原样在"""
        http, _ = http_env
        quality = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "n.json"}).json(
                            )["quality_comparison"]
        for key in ("quality_a", "quality_b", "quality_diff", "dedup_a", "dedup_b",
                    "length_a", "length_b", "vocabulary_a", "vocabulary_b",
                    "winner", "summary"):
            assert key in quality, f"{key} 不在质量面里了"

    @needs_web_src
    def test_the_ts_face_has_no_such_fields(self):
        types = TYPES_TS.read_text(encoding="utf-8")
        block = re.search(r"export interface QualityComparisonFace \{(.*?)\n\}",
                          types, re.S)
        assert block, "前端类型面里没有 QualityComparisonFace ⇒ 判据要跟着换形状"
        for dead in DEAD_KEYS:
            assert dead not in block.group(1), f"{dead} 还在前端类型面里"

    @needs_web_src
    def test_the_ts_doc_points_at_the_overlap_face(self):
        """写实：类型面的注释要指明「相似度去重叠面取」，而不是留一句旧口径"""
        types = TYPES_TS.read_text(encoding="utf-8")
        assert "similarity_score" in types, "类型面没提重叠面那一格"


class TestA335TheReceiptCarriesTheName:
    """A335：回执里那一格 `name` 说清「这个 id 是谁起的」"""

    def test_the_sdk_receipt_has_the_eight_keys(self, tmp_path):
        src = tmp_path / "p.json"
        src.write_text(json.dumps(_items("q", 2), ensure_ascii=False), encoding="utf-8")
        info = create_backup(str(src), str(tmp_path / "bk"), "mybak")
        payload = info.to_dict()
        assert set(payload) == RECEIPT_KEYS, sorted(payload)
        assert payload["name"] == "mybak", payload

    def test_no_name_given_reads_as_none_not_empty_string(self, tmp_path):
        src = tmp_path / "p.json"
        src.write_text(json.dumps(_items("q", 2), ensure_ascii=False), encoding="utf-8")
        info = create_backup(str(src), str(tmp_path / "bk"))
        payload = info.to_dict()
        assert payload["name"] is None, payload
        assert re.fullmatch(r"backup_\d{8}_\d{6}", payload["backup_id"]), payload

    def test_the_index_row_carries_the_name(self, tmp_path):
        """落盘的那一行也要有这一格 —— 否则重启后读边就丢了它"""
        src = tmp_path / "p.json"
        src.write_text(json.dumps(_items("q", 2), ensure_ascii=False), encoding="utf-8")
        target = tmp_path / "bk"
        create_backup(str(src), str(target), "mybak")
        rows = json.loads((target / "index.json").read_text(encoding="utf-8"))["backups"]
        assert len(rows) == 1, rows
        assert rows[0]["name"] == "mybak", rows

    def test_a_legacy_row_without_the_key_reads_as_none(self, tmp_path):
        """存量索引行（L329 之前写下的）没有这一格 ⇒ 补 `None`，不是缺键

        「同一份清单里有的行有、有的行没有」是两种形状 ⇒ 调用方每读一行都得先问
        「这行有没有那格」。
        """
        target = tmp_path / "bk"
        target.mkdir()
        legacy = {
            "backup_id": "legacy",
            "timestamp": "2026-01-01T00:00:00",
            "source_path": "p.json",
            "backup_path": str(target / "legacy.json"),
            "item_count": 1,
            "file_size": 2,
            "checksum": "0" * 64,
        }
        (target / "index.json").write_text(json.dumps({"backups": [legacy]}),
                                           encoding="utf-8")
        from augmentor.backup import list_backups

        rows = list_backups(str(target))
        assert len(rows) == 1, rows
        assert "name" in rows[0], rows
        assert rows[0]["name"] is None, rows

    def test_the_http_receipt_carries_the_name(self, http_env):
        http, _ = http_env
        named = http.post("/api/system/backups",
                          json={"input_file": "p.json", "name": "mybak"}).json()
        assert named["name"] == "mybak", named
        assert named["backup_id"] == "mybak", named
        anonymous = http.post("/api/system/backups", json={"input_file": "p.json"}).json()
        assert anonymous["name"] is None, anonymous

    def test_the_http_list_carries_the_name(self, http_env):
        http, _ = http_env
        http.post("/api/system/backups", json={"input_file": "p.json", "name": "mybak"})
        rows = http.get("/api/system/backups").json()["backups"]
        assert [r["name"] for r in rows] == ["mybak"], rows

    def test_the_id_is_still_the_addressing_key(self, http_env):
        """加一格**不动**寻址：`restore` / `delete` 仍只认 `backup_id`"""
        http, _ = http_env
        http.post("/api/system/backups", json={"input_file": "p.json", "name": "mybak"})
        restored = http.post("/api/system/backups/mybak/restore",
                             json={"output_file": "restored.json"})
        assert restored.status_code == 200, restored.text
        assert restored.json()["backup_id"] == "mybak", restored.text
        deleted = http.delete("/api/system/backups/mybak")
        assert deleted.status_code == 200, deleted.text

    @needs_web_src
    def test_the_ts_face_declares_the_nullable_key(self):
        types = TYPES_TS.read_text(encoding="utf-8")
        block = re.search(r"export interface BackupInfo \{(.*?)\n\}", types, re.S)
        assert block, "前端类型面里没有 BackupInfo ⇒ 判据要跟着换形状"
        assert re.search(r"name:\s*string \| null", block.group(1)), block.group(1)

    @needs_web_src
    def test_the_zod_schema_accepts_null(self):
        """zod schema 漏了这一格 ⇒ 前端把回执**验**过一遍就把它丢了（静默）"""
        schemas = SCHEMAS_TS.read_text(encoding="utf-8")
        block = re.search(r"export const backupInfoResponseSchema = z\.object\(\{(.*?)\n\}\)",
                          schemas, re.S)
        assert block, "前端 schema 里没有 backupInfoResponseSchema ⇒ 判据要跟着换形状"
        assert re.search(r"name:\s*z\.string\(\)\.nullable\(\)", block.group(1)), \
            block.group(1)

    @needs_web_src
    def test_the_api_doc_mentions_the_name_key(self):
        doc = API_MD.read_text(encoding="utf-8")
        assert "回执八键" in doc, "API 文档还写着回执七键 ⇒ 口径过时"
        assert "`name`" in doc, "API 文档没提 `name` 那一格"


class TestTheRulersCanFail:
    """三支注入臂：把上面那些判据拆掉时，它们各自要变样

    没有这一节，「回执里没有那两格」这类断言可能是**恒真**的（判据自己红了也没人知道）。
    """

    def test_arm_putting_the_dead_keys_back_makes_them_appear(self, monkeypatch, http_env):
        """臂 1：把两个死键塞回 `to_dict()` ⇒ HTTP 回执里就有它们了

        这证明「回执里没有那两格」读的是**真源头**（dataclass 的序列化），
        而不是路由层顺手把它们过滤掉了。
        """
        original = ComparisonResult.to_dict

        def patched(self):
            payload = original(self)
            payload["similarity_to_a"] = 0.0
            payload["similarity_to_b"] = 0.0
            return payload

        monkeypatch.setattr(ComparisonResult, "to_dict", patched)
        http, _ = http_env
        quality = http.post("/api/dataset/compare",
                            json={"dataset_a": "p.json", "dataset_b": "n.json"}).json(
                            )["quality_comparison"]
        assert "similarity_to_a" in quality and "similarity_to_b" in quality, quality

    def test_arm_dropping_the_name_from_the_dict_changes_the_receipt(self, monkeypatch,
                                                                     http_env):
        """臂 2：把 `name` 从 `to_dict()` 里摘掉 ⇒ 回执那一格退回 `null`

        （FastAPI 按响应模型补默认值 ⇒ 缺键的那一次会变成 `null`，不是「键不见了」；
        所以这一格读的是模型默认值而不是 SDK 的值 ⇒ 与臂 3 咬合。）
        """
        original = BackupInfo.to_dict

        def patched(self):
            payload = original(self)
            payload.pop("name", None)
            return payload

        monkeypatch.setattr(BackupInfo, "to_dict", patched)
        http, _ = http_env
        named = http.post("/api/system/backups",
                          json={"input_file": "p.json", "name": "mybak"}).json()
        assert named["name"] is None, named

    def test_arm_faking_the_sdk_changes_the_receipt(self, monkeypatch, http_env):
        """臂 3：把端点看到的 `create_backup` 换成假实现 ⇒ 回执的 `name` 跟着变

        这证明回执里那一格**是 SDK 那一处的值**，不是路由把请求体原样回显。
        """
        from augmentor.backup import BackupInfo as RealBackupInfo

        def fake(source_path, backup_dir, name=None):
            return RealBackupInfo(
                backup_id="fake", timestamp="t", source_path="s", backup_path="b",
                item_count=0, file_size=0, checksum="c", name="tampered",
            )

        monkeypatch.setattr("augmentor.backup.create_backup", fake)
        http, _ = http_env
        named = http.post("/api/system/backups",
                          json={"input_file": "p.json", "name": "mybak"}).json()
        assert named["name"] == "tampered", named
