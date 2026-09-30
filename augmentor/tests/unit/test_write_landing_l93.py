# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L93：A163 写侧落点收口 —— 「不替调用方挑目录」在纯写侧等于「新文件写不进去」

改前的形状（`api/deps.py` 的 `resolve_within_roots`，实测见
`Temp/l93q/repro_ui_write.py`）：相对路径的候选按「白名单根在前、工作目录兜底」
排列，取**第一个存在**的解释；**一个都不存在**时取的是 `candidates[-1]`，也就是
工作目录解释。而出厂默认 `web.data_roots = ["data"]` 是**相对**工作目录的，所以
工作目录恰好在闸外 ⇒ 所有「写一个新文件」的请求必 403：

| 同一份配置（根 = `<cwd>/data`） | 改前 | 改后 |
| --- | --- | --- |
| 读 `in.json`（根内存在） | 200 | 200（一字未动） |
| 写 `augmented_output.json`（UI 的默认输出名） | **403 路径超出允许的数据目录范围** | 200，落 `<cwd>/data/` |
| 读 `nope.json`（不存在） | **403**（真相是「不存在」） | 404 文件不存在 |
| 写 `data/xxx.json`（docstring 教的写法） | 200 | 200 |
| 绝对路径越界 / `..` | 403 / 400 | 403 / 400（一字未动） |

`web/src/pages/Augmentation.tsx` 的 `outputFile` 初值就是裸名
`augmented_output.json`，`api.ts` 原样 POST 给 `/api/augment/start`，该端点在入队
**之前**调 `resolve_data_path(..., for_write=True)` ⇒ 用户打开页面什么都不改、点下
「启动增强」拿到 403。这与 L91 的 A160 是同一族缺陷往深一层：上传面当时修的是
「裸名上传落不进根」，而增强/导出/切分/备份那**十处 `for_write=True` 的调用点**
走的是同一把尺子的另一头（`ast` 实测：`api/deps.py` 1 + `augment.py` 1 +
`dataset_tools.py` 5 + `quality.py` 1 + `system_ops.py` 3 = 11 处写点，同一条链上另有 27 处读点）。

修法只有一行，但它必须同时守住三格，缺一格就是另一种撒谎：

1. **不放宽可达范围** —— 新的兜底是**首个根解释**，它按构造就在白名单内，而
   `_assert_within_roots` 照旧过一遍（符号链接逃逸仍 403）。
2. **不搬走本来能用的落点** —— 工作目录解释**自己在闸内**时仍然优先用它，否则
   L87 那格「客户端说的落点要真落在那儿」会被静默改掉（本轮第一版就是改宽成
   无条件 `candidates[0]`，`tests/unit/test_upload_ceiling_l87.py` 三条当场判红）。
3. **读写同一条优先级** —— 读侧一直是「根内第一个存在的」，写侧的兜底也用根优先，
   同一份字符串不再有两种落点故事。
"""

import json
import os
import pathlib

import pytest


@pytest.fixture
def shipped_layout(tmp_path, monkeypatch):
    """复刻出厂布局：根是**相对**的 `data`，进程跑在它的父目录

    `tests/conftest.py` 的 autouse 白名单会把工作目录与系统临时目录都放进来，
    于是「闸外的兜底解释」这一格在默认取证房里根本不存在 —— 必须显式把
    `AUGMENTOR_DATA_ROOTS` 撤掉，让白名单回到配置里那份相对写法。
    """
    import api.deps as deps

    monkeypatch.chdir(tmp_path)
    (tmp_path / "config.yaml").write_text(
        "web:\n  data_roots:\n  - data\n", encoding="utf-8")
    data = tmp_path / "data"
    data.mkdir()
    (data / "in.json").write_text(
        json.dumps([{"instruction": "问", "output": "答"}], ensure_ascii=False),
        encoding="utf-8")
    monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
    monkeypatch.setattr(deps, "config_file_path", lambda: tmp_path / "config.yaml")
    deps.invalidate_config_caches()
    monkeypatch.setattr(deps, "_pipeline", None)
    yield deps, tmp_path, data
    deps.invalidate_config_caches()


class TestTheShippedBugIsGone:
    """A163 的产品面：出厂布局下裸名写不再 403，且落点在根内"""

    def test_bare_new_name_resolves_inside_the_relative_root(self, shipped_layout):
        deps, _tmp, data = shipped_layout
        path = deps.resolve_data_path("augmented_output.json", for_write=True)
        assert path == (data / "augmented_output.json").resolve()

    def test_missing_read_is_404_not_403(self, shipped_layout):
        deps, _tmp, _data = shipped_layout
        from fastapi import HTTPException
        with pytest.raises(HTTPException) as got:
            deps.resolve_data_path("nope.json")
        assert got.value.status_code == 404
        assert got.value.detail == "文件不存在"

    def test_existing_read_still_resolves_the_same_way(self, shipped_layout):
        """读侧那一格一字未动：本体的 200 不是本轮改出来的"""
        deps, _tmp, data = shipped_layout
        assert deps.resolve_data_path("in.json") == (data / "in.json").resolve()

    def test_the_ui_default_output_name_starts_a_task(self, shipped_layout,
                                                      monkeypatch):
        """从真实中间件链走一遍：页面初值那个字符串必须能启动任务

        断言的不是「没报错」，而是**端点真的把根内的落点交给了 pipeline**——
        否则 200 也可能只是校验通过而产物写到别处。
        """
        deps, _tmp, data = shipped_layout
        seen = {}

        def fake_augment(input_path, output_path, **kwargs):
            seen["input"] = input_path
            seen["output"] = output_path

        instance = type("PipelineStub", (), {})()
        instance.augment_dataset = fake_augment
        monkeypatch.setattr(deps, "_pipeline", instance)

        from fastapi.testclient import TestClient
        from api.main import app
        http = TestClient(app)
        response = http.post("/api/augment/start",
                             json={"input_file": "in.json",
                                   "output_file": "augmented_output.json"})
        assert response.status_code == 200, response.json()
        assert pathlib.Path(seen["output"]) == (data / "augmented_output.json").resolve()
        assert pathlib.Path(seen["input"]) == (data / "in.json").resolve()


class TestTheGateStillRefusesWhatItAlwaysRefused:
    """不放宽那一半：本轮换的是「都不存在时选哪个候选」，不是闸的射程"""

    def test_absolute_path_outside_roots_is_still_403(self, shipped_layout):
        deps, tmp, _data = shipped_layout
        from fastapi import HTTPException
        outside = tmp.parent / "outside.json"
        with pytest.raises(HTTPException) as got:
            deps.resolve_data_path(str(outside), for_write=True)
        assert got.value.status_code == 403

    def test_parent_component_is_still_400(self, shipped_layout):
        deps, _tmp, _data = shipped_layout
        from fastapi import HTTPException
        with pytest.raises(HTTPException) as got:
            deps.resolve_data_path("data/../secret.json", for_write=True)
        assert got.value.status_code == 400

    def test_the_written_fallback_is_always_inside_a_root(self, shipped_layout):
        """形状守卫：兜底那一项**按构造**落在白名单内

        直接钉不变量而不是钉某个具体目录 —— 白名单可以有多项、可以是相对或绝对，
        而「不放宽」只要求：解析结果永远是某个根的后代。
        """
        deps, _tmp, _data = shipped_layout
        roots = deps.allowed_data_roots()
        for name in ("a.json", "sub/b.json", "deep/x/y/z.json"):
            resolved = deps.resolve_within_roots(name, "文件路径")
            assert any(resolved == r or str(resolved).startswith(str(r) + os.sep)
                       for r in roots), (name, resolved, roots)


class TestAWorkingDirectoryInsideARootIsNotMoved:
    """本轮第一版改宽后被既有判据判红的那一格，单独钉死

    根是 `tmp`、进程跑在 `tmp/sub`（在闸内）时，新文件仍按工作目录解释落在
    `tmp/sub/`，而不是被搬到 `tmp/`。这是 L87「客户端说的落点要真落在那儿」的
    同一条性质。
    """

    def test_inner_cwd_keeps_its_own_landing_spot(self, tmp_path, monkeypatch):
        import api.deps as deps

        inner = tmp_path / "sub"
        inner.mkdir()
        monkeypatch.chdir(inner)
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        deps.invalidate_config_caches()
        try:
            path = deps.resolve_data_path("brand_new.json", for_write=True)
        finally:
            deps.invalidate_config_caches()
        assert path == (inner / "brand_new.json").resolve()
        assert tmp_path in path.parents

    def test_outer_cwd_falls_back_to_the_root(self, tmp_path, monkeypatch):
        """对照格：同一条字符串，工作目录在闸外时落点是根而不是 cwd"""
        import api.deps as deps

        root = tmp_path / "ds"
        root.mkdir()
        elsewhere = tmp_path / "elsewhere"
        elsewhere.mkdir()
        monkeypatch.chdir(elsewhere)
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        deps.invalidate_config_caches()
        try:
            path = deps.resolve_data_path("brand_new.json", for_write=True)
        finally:
            deps.invalidate_config_caches()
        assert path == (root / "brand_new.json").resolve()
        assert elsewhere not in path.parents


class TestCandidateOrderIsUnchanged:
    """候选表本身一字未动： roots 在前、cwd 兜底在最后，去重照旧"""

    def test_roots_first_cwd_last(self, shipped_layout):
        deps, tmp, data = shipped_layout
        cands = deps._relative_candidates(pathlib.Path("x.json"),
                                          deps.allowed_data_roots())
        assert cands[0] == (data / "x.json").resolve()
        assert cands[-1] == (tmp / "x.json").resolve()

    def test_helper_is_the_assert_without_the_raise(self, shipped_layout):
        """`_is_within_roots` 与 `_assert_within_roots` 必须同判据（共用同一份白名单）

        钉的是「兜底用的判据与闸用的判据是同一个函数」，而不是各自写一遍
        `relative_to` 的循环 —— 两把尺子是本轮最容易漂回去的地方。
        """
        deps, tmp, _data = shipped_layout
        roots = deps.allowed_data_roots()
        inside = (tmp / "data" / "x.json").resolve()
        outside = (tmp.parent / "x.json").resolve()
        assert deps._is_within_roots(inside, roots) is True
        assert deps._is_within_roots(outside, roots) is False
        assert deps._assert_within_roots(inside, roots) == inside
        with pytest.raises(deps.HTTPException):
            deps._assert_within_roots(outside, roots)
