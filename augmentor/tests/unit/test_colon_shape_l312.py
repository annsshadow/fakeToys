# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L312 守卫：冒号形状（A374）+ 三笔契约口径写进文档（A336① / A342② / A318③ / A284②）

**这一轮收的是哪一格**：

1. **A374 —— 冒号档**：`pathlib` 把 `C:foo` / `a:b` 解析成**盘符相对**的形状 ⇒
   `root / candidate` 会把基座悄悄丢掉（`C:foo` 落到 `C:\\base\\foo`，`a:b` 落到
   `a:\\base\\b`）⇒ **落点名与调用方给的 id 不是同一个东西**，调用方拿回执里的 id 再去找
   就找不到了。本轮把这一档**下推到 `require_within_dir`** ⇒ 九个模块的调用面一起获得。
2. **A336① —— 契约写了一档而它永远不来**：三只**无请求体**的备份端点（list / DELETE /
   verify）身上，框架仍自动挂一档 422，而它们只有字符串路径段与查询参数（恒合法）⇒ 这一档
   不可达。本轮**显式覆盖**那一档的说明（只改说明、不改可达性、不改公开响应的键集）。
3. **A342② / A318③ / A284② —— 三处「文档面写实话」**：`_output_path` 交回的是**拼接**
   而不是解析后的落点（两个形状并存）；`validate-config` 的判决**只由 `errors` 决定**
   （`warnings` 不参与）；`unique_instructions` 这个键名数的是「所选字段的唯一值数」。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走具名用例，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_colon_shape_l312.py -q --no-cov`
"""
import pathlib

import pytest

from augmentor.exceptions import DataValidationError
from augmentor.validation import require_within_dir

ROOT = pathlib.Path(__file__).resolve().parents[2]
#: 仓库根（`docs/` 在子项目之外）
REPO = ROOT.parent
API_DOC = REPO / "docs" / "augmentor" / "API.md"

#: 盘符相对形状 —— `pathlib` 会把它们解析成带 drive 的路径 ⇒ 基座被丢掉。
COLON_SHAPES = ["C:foo", "C:", "a:b"]

#: 带冒号但**不带 drive**的名字 —— 落点名不变形 ⇒ 这一档不该被误伤。
#: （`pathlib` 的 Windows 盘符只认**单字母 + 冒号**：`foo:bar` 的 drive 是空的，
#: `a:b:c` 的 drive 是 `a:` ⇒ 后者落在盘符那一档，不在这份名单里。）
NOT_A_DRIVE = ["foo:bar"]


class TestTheColonShapeIsRefused:
    """A374：冒号形状 ⇒ 400，且判决点名「名字里的冒号会被当成盘符」"""

    @pytest.fixture()
    def base(self, tmp_path):
        root = tmp_path / "base"
        root.mkdir()
        return root

    def test_every_colon_shape_is_refused(self, base):
        for shape in COLON_SHAPES:
            with pytest.raises(DataValidationError) as exc:
                require_within_dir("名称", shape, base)
            assert "不能含冒号" in str(exc.value), (shape, str(exc.value))
            assert "名称" in str(exc.value), shape

    def test_the_verdict_does_not_leak_a_path(self, base):
        """判决只含名字与目录标签，不含解析后的绝对落点（A155① 那一族）"""
        with pytest.raises(DataValidationError) as exc:
            require_within_dir("名称", "C:foo", base)
        assert str(base) not in str(exc.value), str(exc.value)

    def test_a_name_without_a_drive_is_not_over_reached(self, base):
        """`pathlib` 不把它当 drive ⇒ 落点名不变形 ⇒ 这一档不该被误伤"""
        for name in NOT_A_DRIVE:
            landed = require_within_dir("名称", name, base)
            assert landed.name == name, (name, landed)

    def test_the_landing_never_differs_from_the_given_name(self, base):
        """**这一档的实质**：落点与给的名字永远是同一个东西（放行也不再变形）"""
        for name in ("plain", "sub_dir", "a.b.json", *NOT_A_DRIVE):
            landed = require_within_dir("名称", name, base)
            assert landed.name == name, (name, landed.name)


class TestTheAuthorityCallSitesInheritIt:
    """A374：下推到权威 ⇒ 九个模块的调用面一起获得（抽样两处真调用面）"""

    def test_the_export_face_refuses_a_colon_name(self, tmp_path):
        """`augmentor/export.py` 的「数据集名称」那一档：冒号名字不许落盘"""
        from augmentor.export import Exporter

        out = tmp_path / "out"
        out.mkdir()
        with pytest.raises(DataValidationError) as exc:
            Exporter().export_batch({"C:evil": [{"instruction": "a", "output": "b"}]},
                                    str(out), ["jsonl"])
        assert "不能含冒号" in str(exc.value), str(exc.value)

    def test_the_version_face_refuses_a_colon_id(self, tmp_path):
        """`versioning.py` 的「版本 ID」那一档：冒号 id 不许落盘"""
        from augmentor.versioning import VersionManager

        manager = VersionManager(storage_dir=str(tmp_path / "versions"))
        with pytest.raises(DataValidationError) as exc:
            manager.load_version("C:evil")
        assert "不能含冒号" in str(exc.value), str(exc.value)


class TestTheContractSaysTheTruth:
    """A336① / A342② / A318③ / A284②：契约与文档面写实话"""

    def test_the_three_body_less_backup_endpoints_declare_the_unreachable_422(self):
        """三只无请求体的备份端点：422 那一档**显式覆盖**成「不可达」的实话"""
        from api.routes.system_ops import router

        body_less = {
            ("GET", "/api/system/backups"),
            ("DELETE", "/api/system/backups/{backup_id}"),
            ("GET", "/api/system/backups/verify"),
        }
        seen = {}
        for route in router.routes:
            key = (next(iter(route.methods - {"HEAD"})), route.path)
            if key in body_less:
                seen[key] = route.responses.get(422)
        assert set(seen) == body_less, sorted(seen)
        for key, desc in seen.items():
            assert desc is not None, key
            assert "不可达" in desc["description"], (key, desc)

    def test_the_api_doc_says_which_shape_the_visualize_face_returns(self):
        """A342②：文档里写明这一面交回「拼接」而不是解析后的落点"""
        text = API_DOC.read_text(encoding="utf-8")
        assert "两个形状并存" in text, "API.md 里少了「两个形状并存」那一句"

    def test_the_api_doc_says_warnings_do_not_decide_the_verdict(self):
        """A318③：文档里写明 `validate-config` 的判决只由 `errors` 决定"""
        text = API_DOC.read_text(encoding="utf-8")
        assert "warnings` 与 `info` 两档不参与判决" in text, "API.md 里少了「warnings 不参与判决」那一句"

    def test_the_api_doc_says_what_unique_instructions_counts(self):
        """A284②：文档里写明那个键名数的是「所选字段的唯一值数」"""
        text = API_DOC.read_text(encoding="utf-8")
        assert "unique_instructions` 这个键名是既成契约" in text, "API.md 里少了 unique_instructions 那一句"
