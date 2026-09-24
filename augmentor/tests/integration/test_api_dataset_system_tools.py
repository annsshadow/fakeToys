# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""dataset / system 两组新增端点的业务分支集成测试

`tests/integration/test_api_openapi_contract.py` 覆盖的是**响应形态**
（每个端点的键是否与 `response_model` 一致）。本模块覆盖的是**业务分支**：
枚举白名单、路径越界、写盘产物、库层容错语义、备份与依赖注册表生命周期。

写这些用例前先对全部端点打了真实请求（探针脚本），所有断言都来自实测输出，
而不是读代码推断。几个**不显然但必须钉住**的点：

* **坏 JSON → 400 且不泄漏解析器英文原文**。`to_http_error` 之前会把
  ``"Expecting property name enclosed in double quotes: line 1 column 3"``
  原样回给客户端；现在只回「数据文件不是合法 JSON（第 N 行第 M 列）」。
* **枚举白名单**。`DatasetValidator` / `EnhancedSearcher` / `DatasetMigrator`
  对未知取值是**静默降级或静默过滤**的，所以 API 边界必须显式拦下——
  否则调用方会以为「strict 校验通过了」「fuzzy 搜不到」。
* **`ValidateRequest.preset` 的默认值**。此前是 ``"default"``，而
  `PRESET_RULES` 里根本没有这个名字，于是**默认请求也在静默回退到 basic**。
  现在默认是 basic，且 `default == basic` 由用例锁定。
* **恢复不存在的备份 → 404**。此前是 400，而同一组路由里
  ``DELETE /api/system/backups/{id}`` 对同样的情况回 404——同一个
  「备份不存在」不该有两种状态码。
* **流式处理对坏输入的两套语义**：JSONL 是逐行格式，单行语法错误跳过并
  `logger.warning`（不是静默吞掉）；而 JSON **数组**的结构是明确的，截断
  必须报 400。`/api/system/stream` 因此**不**参与「坏 JSON 一律 400」的
  跨端点参数化用例，它的三种边界由 `TestSystemStream` 单独钉住。
"""

import json
import logging
import os
import tempfile
from pathlib import Path
from types import SimpleNamespace

import pytest
from fastapi.testclient import TestClient

AI_DIR = Path(__file__).resolve().parent.parent.parent

# 5 条数据，其中「如何申请公租房」重复一次 —— 便于同时验证去重与采样。
# 所有 input 字段为空 —— 便于验证稀疏字段检测。
# 「物业费怎么算」的 output 只有 8 个字符 —— 触发 strict 预设的长度告警。
ITEMS = [
    {"instruction": "如何申请公租房", "input": "", "output": "需满足收入条件并提供社保证明。"},
    {"instruction": "二手房交易流程", "input": "", "output": "签约、缴税、过户三步走。"},
    {"instruction": "贷款利率是多少", "input": "", "output": "首套房当前 3.1%。"},
    {"instruction": "物业费怎么算", "input": "", "output": "按建筑面积计费。"},
    {"instruction": "如何申请公租房", "input": "", "output": "需满足收入条件并提供社保证明。"},
]
# 上游数据集的子集，用于合并 / 对比 / 聚合
SECOND_ITEMS = ITEMS[:3]
# 字段结构不同，用于迁移
MIGRATION_ITEMS = [{"question": "q1", "answer": "a1"}, {"question": "q2", "answer": "a2"}]


def _write_json(path: Path, items) -> Path:
    """写入 JSON 数据集文件

    Args:
        path: 目标路径
        items: 数据列表

    Returns:
        目标路径
    """
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return path


def _as_jsonl(items) -> str:
    """把数据列表渲染成 JSONL 文本（每行一条，结尾带换行）

    Args:
        items: 数据列表

    Returns:
        JSONL 文本
    """
    return "".join(json.dumps(x, ensure_ascii=False) + "\n" for x in items)


@pytest.fixture(scope="module")
def shared_pipeline():
    """从仓库配置构造的管道实例（模块内复用，避免逐用例重复构造）

    钉住 `api.deps._pipeline` 的意义：它是**模块级单例**，若不钉住，
    测试会共享上一用例残留的状态（本项目已因此踩过坑）。

    Returns:
        AugmentorPipeline 实例
    """
    from augmentor import AugmentorPipeline, load_config

    return AugmentorPipeline(load_config(str(AI_DIR / "config.yaml")))


@pytest.fixture
def tools_env(tmp_path, monkeypatch, shared_pipeline):
    """隔离 CWD、数据白名单与管道单例，并准备样本文件

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture
        shared_pipeline: 模块级管道实例

    Returns:
        SimpleNamespace，含 client 与各样本文件路径
    """
    import api.deps as deps
    from api.main import app

    # 白名单显式包含：临时目录、仓库目录（便于校验仓库自带 config.yaml）。
    # 不依赖 conftest 的取值，避免 fixture 顺序影响断言。
    monkeypatch.setenv(
        "AUGMENTOR_DATA_ROOTS",
        os.pathsep.join([str(tmp_path), str(AI_DIR), tempfile.gettempdir()]),
    )
    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.chdir(tmp_path)
    monkeypatch.setattr(deps, "_pipeline", shared_pipeline)

    data = _write_json(tmp_path / "items.json", ITEMS)
    second = _write_json(tmp_path / "items_b.json", SECOND_ITEMS)
    migration = _write_json(tmp_path / "migration.json", MIGRATION_ITEMS)
    broken = tmp_path / "broken.json"
    broken.write_text("{ 这不是合法 JSON", encoding="utf-8")

    jsonl = tmp_path / "items.jsonl"
    jsonl.write_text(_as_jsonl(ITEMS), encoding="utf-8")

    # 5 行合法 + 1 行坏 —— 验证流式处理的「跳过无效行」容错
    jsonl_broken = tmp_path / "items_broken.jsonl"
    jsonl_broken.write_text(
        _as_jsonl(ITEMS) + "{这一行不是合法 JSON\n", encoding="utf-8"
    )

    # 被截断的 JSON **数组**：结构明确，语法错误必须报出来（与 JSONL 的容错相反）
    broken_array = tmp_path / "broken_array.json"
    broken_array.write_text('[ {"instruction": "x", "output": "y"}, ', encoding="utf-8")

    return SimpleNamespace(
        client=TestClient(app),
        tmp=tmp_path,
        data=data,
        second=second,
        migration=migration,
        broken=broken,
        broken_array=broken_array,
        jsonl=jsonl,
        jsonl_broken=jsonl_broken,
        out=tmp_path / "out.json",
        out_dir=tmp_path / "outdir",
        backup_dir=tmp_path / "backups",
        registry=tmp_path / "registry",
    )


def _read_json(path: Path):
    """读取 JSON 文件

    Args:
        path: 文件路径

    Returns:
        解析结果
    """
    return json.loads(path.read_text(encoding="utf-8"))


# ============================================================
# 只读分析类
# ============================================================

class TestDatasetStats:
    """/api/dataset/stats"""

    def test_returns_per_field_statistics(self, tools_env):
        """逐字段统计与整体质量指标都在响应里"""
        response = tools_env.client.post(
            "/api/dataset/stats", json={"input_file": str(tools_env.data)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_items"] == len(ITEMS)
        assert payload["dataset_name"] == "items"
        # summary 是给人看的多行文本，field_statistics 才是结构化结果
        assert isinstance(payload["summary"], str)
        assert set(payload["field_statistics"]) == {"instruction", "input", "output"}
        assert set(payload["quality_metrics"]) == {"completeness", "consistency", "diversity"}

    def test_fields_filter_limits_output(self, tools_env):
        """fields 只保留指定字段的统计"""
        response = tools_env.client.post(
            "/api/dataset/stats",
            json={"input_file": str(tools_env.data), "fields": ["instruction"]},
        )
        assert response.status_code == 200, response.text
        assert set(response.json()["field_statistics"]) == {"instruction"}


class TestDatasetValidate:
    """/api/dataset/validate"""

    def test_default_preset_equals_basic(self, tools_env):
        """不传 preset 等价于 basic

        修复前 `ValidateRequest.preset` 默认值是 ``"default"``，而
        `PRESET_RULES` 里只有 basic / strict / chat —— 默认请求实际走了
        「未知预设静默回退 basic」这条路径。本用例锁定默认值已改正。
        """
        client, payload = tools_env.client, {"input_file": str(tools_env.data)}
        default = client.post("/api/dataset/validate", json=payload)
        basic = client.post("/api/dataset/validate", json={**payload, "preset": "basic"})
        assert default.status_code == basic.status_code == 200
        assert default.json() == basic.json()

    def test_strict_preset_surfaces_extra_warnings(self, tools_env):
        """strict 预设真的比 basic 严 —— 而不是静默回退到 basic"""
        client = tools_env.client
        payload = {"input_file": str(tools_env.data)}
        basic = client.post("/api/dataset/validate", json=payload).json()
        strict = client.post(
            "/api/dataset/validate", json={**payload, "preset": "strict"}
        ).json()
        assert basic["warning_count"] == 0
        assert strict["warning_count"] > basic["warning_count"]
        # error_count 是 dataclass 上的计算属性，漏声明会被 response_model 裁掉
        assert "error_count" in strict

    def test_unknown_preset_rejected(self, tools_env):
        """未知预设必须 400 —— 不能静默回退到 basic"""
        response = tools_env.client.post(
            "/api/dataset/validate",
            json={"input_file": str(tools_env.data), "preset": "no_such_preset"},
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "未知的验证预设" in detail
        # 报错要告诉调用方合法取值，否则只能去翻源码
        for preset in ("basic", "strict", "chat"):
            assert preset in detail


class TestDatasetSearch:
    """/api/dataset/search"""

    def test_contains_matches_instruction(self, tools_env):
        """contains 在 instruction 上命中两条重复项"""
        response = tools_env.client.post(
            "/api/dataset/search",
            json={"input_file": str(tools_env.data), "query": "公租房"},
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_matches"] == 2
        assert len(payload["items"]) == 2
        assert payload["method"] == "contains"
        assert payload["query_time_ms"] >= 0

    @pytest.mark.parametrize(
        "method,query,expected",
        [
            ("exact", "如何申请公租房", 2),   # 整字段精确匹配
            ("exact", "公租房", 0),           # 子串不匹配
            ("ngram", "交易", 1),
            ("regex", "^如何", 2),
            ("fuzzy", "公租屋", 0),           # 编辑距离不足
        ],
    )
    def test_each_method_has_its_own_semantics(self, tools_env, method, query, expected):
        """五种检索方法各自语义不同 —— 不是都退化成 contains"""
        response = tools_env.client.post(
            "/api/dataset/search",
            json={"input_file": str(tools_env.data), "query": query, "method": method},
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["method"] == method
        assert payload["total_matches"] == expected

    def test_fields_narrows_search_scope(self, tools_env):
        """限定 fields 后不搜 instruction，故「公租房」命中 0"""
        response = tools_env.client.post(
            "/api/dataset/search",
            json={
                "input_file": str(tools_env.data),
                "query": "公租房",
                "fields": ["output"],
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["total_matches"] == 0

    def test_limit_and_offset_paginate_but_total_is_full(self, tools_env):
        """total_matches 是命中总数，limit/offset 只裁剪 items"""
        response = tools_env.client.post(
            "/api/dataset/search",
            json={
                "input_file": str(tools_env.data),
                "query": "公租房",
                "limit": 1,
                "offset": 1,
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_matches"] == 2
        assert len(payload["items"]) == 1

    def test_unknown_method_rejected(self, tools_env):
        """未知检索方法必须 400 —— 不能静默回退到 contains"""
        response = tools_env.client.post(
            "/api/dataset/search",
            json={
                "input_file": str(tools_env.data),
                "query": "公租房",
                "method": "no_such",
            },
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "未知的搜索方法" in detail
        for method in ("exact", "contains", "ngram", "fuzzy", "regex"):
            assert method in detail

    def test_broken_json_returns_400(self, tools_env):
        """坏 JSON → 400（本端点需要 query，故不并入跨端点参数化用例）"""
        response = tools_env.client.post(
            "/api/dataset/search",
            json={"input_file": str(tools_env.broken), "query": "公租房"},
        )
        assert response.status_code == 400, response.text
        assert "数据文件不是合法 JSON" in response.json()["detail"]


class TestDatasetCompareFeaturesAutoConfig:
    """/api/dataset/compare、features、auto-config"""

    def test_compare_returns_both_conclusions(self, tools_env):
        """质量向与重叠度两份结论缺一不可"""
        response = tools_env.client.post(
            "/api/dataset/compare",
            json={"dataset_a": str(tools_env.data), "dataset_b": str(tools_env.second)},
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        quality = payload["quality_comparison"]
        overlap = payload["overlap_comparison"]
        # 名字缺省取文件名 stem
        assert quality["dataset_a_name"] == "items"
        assert quality["dataset_b_name"] == "items_b"
        assert quality["dataset_a_count"] == len(ITEMS)
        assert quality["dataset_b_count"] == len(SECOND_ITEMS)
        assert "metrics" in overlap

    def test_compare_honours_explicit_names(self, tools_env):
        """显式传名字时不取 stem"""
        response = tools_env.client.post(
            "/api/dataset/compare",
            json={
                "dataset_a": str(tools_env.data),
                "dataset_b": str(tools_env.second),
                "name_a": "线上集",
                "name_b": "候选集",
            },
        )
        assert response.status_code == 200, response.text
        quality = response.json()["quality_comparison"]
        assert quality["dataset_a_name"] == "线上集"
        assert quality["dataset_b_name"] == "候选集"

    def test_features_reports_sparse_fields(self, tools_env):
        """全空的 input 字段必须被标成稀疏"""
        response = tools_env.client.post(
            "/api/dataset/features", json={"input_file": str(tools_env.data)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_items"] == len(ITEMS)
        assert {f["name"] for f in payload["field_features"]} == {
            "instruction",
            "input",
            "output",
        }
        assert "input" in payload["sparse_fields"]
        # FieldFeature 是嵌套模型，字段名对不上会被静默裁掉
        assert set(payload["field_features"][0]) == {
            "name",
            "feature_type",
            "coverage",
            "unique_ratio",
            "description",
        }

    def test_auto_config_returns_thresholds(self, tools_env):
        """推荐结果必须包含可直接喂给管道的一组阈值"""
        response = tools_env.client.post(
            "/api/dataset/auto-config", json={"input_file": str(tools_env.data)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert 0.0 <= payload["quality_threshold"] <= 1.0
        assert 0.0 <= payload["dedup_threshold"] <= 1.0
        assert payload["recommended_sample_size"] >= 0
        assert payload["source_stats"]["total_items"] == len(ITEMS)

    def test_compare_missing_second_dataset_returns_404(self, tools_env):
        """两个输入任一不存在都 → 404（本端点有两个文件入参）"""
        response = tools_env.client.post(
            "/api/dataset/compare",
            json={
                "dataset_a": str(tools_env.data),
                "dataset_b": str(tools_env.tmp / "nope.json"),
            },
        )
        assert response.status_code == 404, response.text

    def test_compare_broken_json_returns_400(self, tools_env):
        """坏 JSON → 400（本端点有两个文件入参，故不并入跨端点参数化用例）"""
        response = tools_env.client.post(
            "/api/dataset/compare",
            json={"dataset_a": str(tools_env.broken), "dataset_b": str(tools_env.second)},
        )
        assert response.status_code == 400, response.text
        assert "数据文件不是合法 JSON" in response.json()["detail"]


# ============================================================
# 写盘变换类：响应只回「写到哪、写了多少」，产物必须真落盘
# ============================================================

class TestDatasetConvert:
    """/api/dataset/convert"""

    def test_converts_to_jsonl_and_writes_file(self, tools_env):
        """产物真的落盘，且条数与响应一致"""
        response = tools_env.client.post(
            "/api/dataset/convert",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "target_format": "jsonl",
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["input_count"] == len(ITEMS)
        assert payload["output_count"] == len(ITEMS)
        assert payload["source_format"] == "json"
        assert payload["target_format"] == "jsonl"
        assert tools_env.out.is_file()
        lines = [ln for ln in tools_env.out.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert len(lines) == len(ITEMS)

    def test_unknown_target_format_rejected(self, tools_env):
        """未知目标格式必须 400"""
        response = tools_env.client.post(
            "/api/dataset/convert",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "target_format": "no_such",
            },
        )
        assert response.status_code == 400, response.text
        assert "不支持的转换" in response.json()["detail"]

    def test_declared_source_format_converts_container_file(self, tools_env):
        """`source_format` 让对话类文件（落盘也是 .json）能转回规范形再导出

        用 sharegpt 而不是 alpaca：alpaca 记录本身就带 `instruction`/`output`，
        按 json 读走也能出对的结果，测不出这条边；`conversations` 只有声明了源格式
        才会被折叠。断言值是手写字面量。
        """
        src = _write_json(tools_env.tmp / "sharegpt.json", [{
            "conversations": [
                {"from": "system", "value": "你是租房顾问"},
                {"from": "user", "value": "可以月付吗"},
                {"from": "assistant", "value": "支持月付"},
            ]}])
        response = tools_env.client.post(
            "/api/dataset/convert",
            json={
                "input_file": str(src),
                "output_file": str(tools_env.out),
                "target_format": "chatml",
                "source_format": "sharegpt",
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["source_format"] == "sharegpt"
        assert json.loads(tools_env.out.read_text(encoding="utf-8")) == [{
            "messages": [
                {"role": "system", "content": "你是租房顾问"},
                {"role": "user", "content": "可以月付吗"},
                {"role": "assistant", "content": "支持月付"},
            ]}]

    def test_unknown_source_format_rejected(self, tools_env):
        """未知源格式 → 400，而不是 500 或静默按 json 读"""
        response = tools_env.client.post(
            "/api/dataset/convert",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "target_format": "jsonl",
                "source_format": "tsv",
            },
        )
        assert response.status_code == 400, response.text
        assert "无法从 tsv 转换到 JSON" in response.json()["detail"]

    def test_dirty_container_row_reports_row_number(self, tools_env):
        """源文件里有坏记录：400 的错误信息带条目下标，便于调用方定位"""
        src = _write_json(tools_env.tmp / "bad_sharegpt.json", [
            {"conversations": [{"from": "human", "value": "q"}, {"from": "gpt", "value": "a"}]},
            {"conversations": [{"from": "tool", "value": "外部返回"},
                               {"from": "gpt", "value": "a"}]},
        ])
        response = tools_env.client.post(
            "/api/dataset/convert",
            json={
                "input_file": str(src),
                "output_file": str(tools_env.out),
                "target_format": "json",
                "source_format": "sharegpt",
            },
        )
        assert response.status_code == 400, response.text
        assert "第 2 条 sharegpt 记录含未认识的角色: tool" in response.json()["detail"]
        assert not tools_env.out.exists()


class TestDatasetMerge:
    """/api/dataset/merge"""

    def test_merge_deduplicates_by_default(self, tools_env):
        """默认去重：8 条进 → 4 条出"""
        response = tools_env.client.post(
            "/api/dataset/merge",
            json={"inputs": [str(tools_env.data), str(tools_env.second)],
                  "output_file": str(tools_env.out)},
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["input_files"] == 2
        assert payload["total_input"] == len(ITEMS) + len(SECOND_ITEMS)
        assert payload["total_output"] == 4
        assert payload["removed_duplicates"] == payload["total_input"] - payload["total_output"]
        assert len(_read_json(tools_env.out)) == 4

    def test_deduplicate_can_be_disabled(self, tools_env):
        """关掉去重后一条不丢"""
        response = tools_env.client.post(
            "/api/dataset/merge",
            json={
                "inputs": [str(tools_env.data), str(tools_env.second)],
                "output_file": str(tools_env.out),
                "deduplicate": False,
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_output"] == payload["total_input"]
        assert payload["removed_duplicates"] == 0

    def test_empty_inputs_rejected(self, tools_env):
        """空 inputs 必须 400，而不是当成「合并了 0 个文件」成功返回"""
        response = tools_env.client.post(
            "/api/dataset/merge", json={"inputs": [], "output_file": str(tools_env.out)}
        )
        assert response.status_code == 400, response.text
        assert "inputs" in response.json()["detail"]

    def test_missing_input_returns_404(self, tools_env):
        """任一输入不存在 → 404"""
        response = tools_env.client.post(
            "/api/dataset/merge",
            json={
                "inputs": [str(tools_env.data), str(tools_env.tmp / "nope.json")],
                "output_file": str(tools_env.out),
            },
        )
        assert response.status_code == 404, response.text


class TestDatasetSample:
    """/api/dataset/sample"""

    def test_sample_by_size(self, tools_env):
        """按数量采样，产物条数与响应一致"""
        response = tools_env.client.post(
            "/api/dataset/sample",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "size": 2,
                "seed": 7,
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["input_count"] == len(ITEMS)
        assert payload["output_count"] == 2
        assert len(_read_json(tools_env.out)) == 2

    def test_sample_by_ratio(self, tools_env):
        """按比例采样：5 * 0.4 → 2 条"""
        response = tools_env.client.post(
            "/api/dataset/sample",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "ratio": 0.4,
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["output_count"] == 2

    def test_size_larger_than_dataset_is_clamped(self, tools_env):
        """size 超过数据量时截到全量 —— 库层既定语义，不是错误"""
        response = tools_env.client.post(
            "/api/dataset/sample",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "size": 99999,
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["output_count"] == len(ITEMS)

    @pytest.mark.parametrize("method", ["random", "systematic", "stratified"])
    def test_supported_methods(self, tools_env, method):
        """三种采样方法都可用"""
        response = tools_env.client.post(
            "/api/dataset/sample",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "size": 3,
                "method": method,
                "seed": 7,
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["output_count"] == 3

    def test_unknown_method_rejected(self, tools_env):
        """未知采样方法必须 400"""
        response = tools_env.client.post(
            "/api/dataset/sample",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "size": 3,
                "method": "no_such",
            },
        )
        assert response.status_code == 400, response.text
        assert "不支持的采样方法" in response.json()["detail"]


class TestDatasetSplit:
    """/api/dataset/split"""

    def test_split_writes_three_parts(self, tools_env):
        """三份产物都落盘，条数之和等于输入"""
        response = tools_env.client.post(
            "/api/dataset/split",
            json={
                "input_file": str(tools_env.data),
                "output_dir": str(tools_env.out_dir),
                "seed": 7,
            },
        )
        assert response.status_code == 200, response.text
        splits = response.json()["splits"]
        assert set(splits) == {"train", "val", "test"}
        counts = {name: part["count"] for name, part in splits.items()}
        assert sum(counts.values()) == len(ITEMS)
        assert counts["train"] == 4
        for name in ("train", "val", "test"):
            assert Path(splits[name]["file"]).is_file(), f"{name} 产物缺失"

    def test_ratios_must_sum_to_one(self, tools_env):
        """比例之和不为 1 必须 400"""
        response = tools_env.client.post(
            "/api/dataset/split",
            json={
                "input_file": str(tools_env.data),
                "output_dir": str(tools_env.out_dir),
                "train_ratio": 0.5,
                "val_ratio": 0.1,
                "test_ratio": 0.1,
            },
        )
        assert response.status_code == 400, response.text
        assert "分割比例之和" in response.json()["detail"]


class TestDatasetAggregate:
    """/api/dataset/aggregate"""

    def test_union_deduplicates(self, tools_env):
        """union 策略下产物落盘且去重"""
        response = tools_env.client.post(
            "/api/dataset/aggregate",
            json={
                "datasets": {"a": str(tools_env.data), "b": str(tools_env.second)},
                "output_file": str(tools_env.out),
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["aggregated_count"] == 4
        assert payload["source_counts"] == {"a": len(ITEMS), "b": len(SECOND_ITEMS)}
        assert payload["conflicts"] == []
        assert len(_read_json(tools_env.out)) == 4

    def test_intersection_keeps_common_items(self, tools_env):
        """intersection 只留两边都有的条目"""
        response = tools_env.client.post(
            "/api/dataset/aggregate",
            json={
                "datasets": {"a": str(tools_env.data), "b": str(tools_env.second)},
                "output_file": str(tools_env.out),
                "strategy": "intersection",
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["aggregated_count"] == 4

    def test_weighted_respects_target_size(self, tools_env):
        """weighted 策略按 target_size 截断"""
        response = tools_env.client.post(
            "/api/dataset/aggregate",
            json={
                "datasets": {"a": str(tools_env.data)},
                "output_file": str(tools_env.out),
                "strategy": "weighted",
                "target_size": 3,
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["aggregated_count"] == 3

    def test_empty_datasets_rejected(self, tools_env):
        """空 datasets 必须 400"""
        response = tools_env.client.post(
            "/api/dataset/aggregate",
            json={"datasets": {}, "output_file": str(tools_env.out)},
        )
        assert response.status_code == 400, response.text
        assert "datasets" in response.json()["detail"]

    def test_unknown_strategy_rejected(self, tools_env):
        """未知聚合策略必须 400"""
        response = tools_env.client.post(
            "/api/dataset/aggregate",
            json={
                "datasets": {"a": str(tools_env.data)},
                "output_file": str(tools_env.out),
                "strategy": "no_such",
            },
        )
        assert response.status_code == 400, response.text
        assert "不支持的聚合策略" in response.json()["detail"]


class TestDatasetRag:
    """/api/dataset/rag"""

    @pytest.mark.parametrize("fmt", ["langchain", "llamaindex", "custom"])
    def test_supported_formats(self, tools_env, fmt):
        """三种 RAG 格式都能转换并落盘"""
        response = tools_env.client.post(
            "/api/dataset/rag",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "format": fmt,
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["format"] == fmt
        assert payload["input_count"] == len(ITEMS)
        assert payload["record_count"] >= 1
        assert len(_read_json(tools_env.out)) == payload["record_count"]

    def test_unknown_format_rejected(self, tools_env):
        """未知 RAG 格式必须 400，且列出支持项"""
        response = tools_env.client.post(
            "/api/dataset/rag",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "format": "no_such",
            },
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "不支持的 RAG 格式" in detail
        assert "langchain" in detail


# ============================================================
# system 组
# ============================================================

class TestSystemDependencies:
    """/api/system/dependencies"""

    def test_reports_installed_and_degraded(self, tools_env):
        """必须同时给出「装了什么」与「因缺什么而降级」"""
        response = tools_env.client.get("/api/system/dependencies")
        assert response.status_code == 200, response.text
        payload = response.json()
        assert isinstance(payload["installed"], dict)
        assert isinstance(payload["missing"], list)
        assert isinstance(payload["degraded_features"], list)
        # 这两个是 DiagnosticsReport 上的计算属性，漏声明会被 response_model 裁掉
        assert isinstance(payload["all_required_present"], bool)
        assert payload["available_count"] == sum(
            1 for ok in payload["installed"].values() if ok
        )


class TestSystemValidateConfig:
    """/api/system/validate-config"""

    def test_valid_config_passes(self, tools_env):
        """最小合法配置（只含必填的 models.default）应通过"""
        cfg = tools_env.tmp / "min.yaml"
        cfg.write_text("models:\n  default: fallback\n", encoding="utf-8")
        response = tools_env.client.post(
            "/api/system/validate-config", json={"path": str(cfg)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["is_valid"] is True
        assert payload["errors"] == []
        # summary 是三个桶的计数，不是文本
        assert payload["summary"]["errors"] == 0

    def test_repo_config_passes(self, tools_env):
        """仓库自带的 config.yaml 必须自洽 —— 否则「保存后校验」会失败"""
        response = tools_env.client.post(
            "/api/system/validate-config", json={"path": str(AI_DIR / "config.yaml")}
        )
        assert response.status_code == 200, response.text
        assert response.json()["is_valid"] is True

    def test_missing_required_field_reported(self, tools_env):
        """缺 models 段 → is_valid False 且 errors 指向该字段"""
        cfg = tools_env.tmp / "no_models.yaml"
        cfg.write_text("version: '2.0'\n", encoding="utf-8")
        response = tools_env.client.post(
            "/api/system/validate-config", json={"path": str(cfg)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["is_valid"] is False
        assert any(e["path"] == "models" for e in payload["errors"])
        # ConfigValidationIssue 的字段名必须完整保留
        assert set(payload["errors"][0]) == {"path", "message", "severity", "line"}

    def test_broken_yaml_reported_not_raised(self, tools_env):
        """YAML 语法错误应作为一条 error 返回，而不是 500"""
        cfg = tools_env.tmp / "broken.yaml"
        cfg.write_text("this: [is: not: valid\n", encoding="utf-8")
        response = tools_env.client.post(
            "/api/system/validate-config", json={"path": str(cfg)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["is_valid"] is False
        assert payload["errors"][0]["path"] == "yaml"

    def test_missing_file_returns_404(self, tools_env):
        """文件不存在 → 404"""
        response = tools_env.client.post(
            "/api/system/validate-config",
            json={"path": str(tools_env.tmp / "nope.yaml")},
        )
        assert response.status_code == 404, response.text


class TestSystemMonitor:
    """/api/system/monitor"""

    def test_returns_snapshot_with_metrics(self, tools_env):
        """快照必须带指标与告警列表"""
        response = tools_env.client.post(
            "/api/system/monitor", json={"input_file": str(tools_env.data)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["snapshot_id"]
        assert payload["timestamp"]
        assert set(payload["metrics"]) >= {"completeness", "consistency", "diversity"}
        assert isinstance(payload["alerts"], list)


class TestSystemAutoTest:
    """/api/system/auto-test"""

    def test_default_suite_runs(self, tools_env):
        """默认套件可运行，且逐条结果与汇总自洽"""
        response = tools_env.client.post(
            "/api/system/auto-test", json={"input_file": str(tools_env.data)}
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["name"] == "default"
        # 四个计数是 TestSuite 上的计算属性，漏声明会被 response_model 裁掉
        assert payload["total_tests"] == len(payload["results"])
        assert payload["passed_tests"] + payload["failed_tests"] == payload["total_tests"]
        assert 0.0 <= payload["pass_rate"] <= 1.0
        assert set(payload["results"][0]) == {
            "test_id",
            "test_name",
            "passed",
            "message",
            "execution_time_ms",
            "timestamp",
        }

    def test_explicit_default_suite_accepted(self, tools_env):
        """显式传 default 也应通过白名单"""
        response = tools_env.client.post(
            "/api/system/auto-test",
            json={"input_file": str(tools_env.data), "suite": "default"},
        )
        assert response.status_code == 200, response.text

    def test_unknown_suite_rejected(self, tools_env):
        """未知套件必须 400

        `run_dataset_tests()` 每次新建 runner，其 `_test_suites` 初始为空且
        没有注册入口 —— 传任何名字都会静默回退到 default。静默回退会让调用方
        误以为自定义套件跑了。
        """
        response = tools_env.client.post(
            "/api/system/auto-test",
            json={"input_file": str(tools_env.data), "suite": "no_such_suite"},
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "未知的测试套件" in detail
        assert "default" in detail


class TestSystemMigrate:
    """/api/system/migrate"""

    def test_migrates_and_writes_file(self, tools_env):
        """默认用全部内置规则，产物落盘"""
        response = tools_env.client.post(
            "/api/system/migrate",
            json={
                "input_file": str(tools_env.migration),
                "output_file": str(tools_env.out),
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_items"] == len(MIGRATION_ITEMS)
        assert payload["migrated_items"] == len(MIGRATION_ITEMS)
        assert payload["failed_items"] == 0
        assert len(payload["rules_applied"]) == 3
        assert tools_env.out.is_file()

    def test_explicit_rules_are_applied_verbatim(self, tools_env):
        """显式传规则时，rules_applied 必须就是那一份 —— 不能被静默过滤掉"""
        response = tools_env.client.post(
            "/api/system/migrate",
            json={
                "input_file": str(tools_env.migration),
                "output_file": str(tools_env.out),
                "rules": ["rename_instruction"],
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["rules_applied"] == ["rename_instruction"]

    def test_empty_rules_fall_back_to_all_builtins(self, tools_env):
        """空规则等价于「用全部内置规则」"""
        response = tools_env.client.post(
            "/api/system/migrate",
            json={
                "input_file": str(tools_env.migration),
                "output_file": str(tools_env.out),
                "rules": [],
            },
        )
        assert response.status_code == 200, response.text
        assert len(response.json()["rules_applied"]) == 3

    def test_unknown_rules_rejected(self, tools_env):
        """未知规则必须 400

        `DatasetMigrator.migrate()` 用 `r.rule_id in rules` 过滤，未知 id 被
        **静默丢弃**；全写错时 `rules_applied` 是空数组，等于「迁移跑了个寂寞」。
        """
        response = tools_env.client.post(
            "/api/system/migrate",
            json={
                "input_file": str(tools_env.migration),
                "output_file": str(tools_env.out),
                "rules": ["rename_instruction", "no_such_rule"],
            },
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "未知的迁移规则" in detail
        assert "no_such_rule" in detail
        assert "rename_instruction" in detail


class TestSystemStream:
    """/api/system/stream"""

    def test_none_operation_copies_everything(self, tools_env):
        """operation=none 原样搬运"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.jsonl),
                "output_file": str(tools_env.out),
                "operation": "none",
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_input"] == len(ITEMS)
        assert payload["total_output"] == len(ITEMS)
        assert payload["processed"] == len(ITEMS)

    def test_dedup_operation_drops_duplicate(self, tools_env):
        """去重：5 条里有一对重复 → 4 条"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.jsonl),
                "output_file": str(tools_env.out),
                "operation": "dedup",
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["total_output"] == 4

    def test_quality_operation_filters_low_quality(self, tools_env):
        """质量过滤：短输出那条被丢掉"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.jsonl),
                "output_file": str(tools_env.out),
                "operation": "quality",
            },
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["processed"] == len(ITEMS)
        assert payload["total_output"] < payload["total_input"]

    def test_clean_operation_keeps_clean_data(self, tools_env):
        """清洗：样本本来就是干净的，一条不丢"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.jsonl),
                "output_file": str(tools_env.out),
                "operation": "clean",
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["total_output"] == len(ITEMS)

    def test_json_array_input_also_supported(self, tools_env):
        """JSON 数组格式走增量解析路径，同样可用"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.out),
                "operation": "none",
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["total_output"] == len(ITEMS)

    def test_broken_jsonl_lines_are_skipped_with_warning(self, tools_env, caplog):
        """坏 JSONL 行被跳过并留下 warning —— 是既定容错，不是静默吞掉

        输入是「逐行」格式，单行语法错误不该让整批失败；但必须留痕。
        CLI 走的是同一个 `StreamAugmentor`，两边语义必须一致。
        """
        with caplog.at_level(logging.WARNING, logger="augmentor.streaming"):
            response = tools_env.client.post(
                "/api/system/stream",
                json={
                    "input_file": str(tools_env.jsonl_broken),
                    "output_file": str(tools_env.out),
                    "operation": "none",
                },
            )
        assert response.status_code == 200, response.text
        payload = response.json()
        # 6 行进（5 合法 + 1 坏），坏行不进产物
        assert payload["total_input"] == 6
        assert payload["total_output"] == len(ITEMS)
        assert any("跳过无效行" in r.message for r in caplog.records), caplog.text

    def test_entirely_invalid_input_yields_empty_output(self, tools_env, caplog):
        """整份文件都非法（且以 `{` 开头，被当成单行 JSONL）→ 200 且产物为空

        这是 `StreamAugmentor` 的既定语义：它按首字符判断格式，`{` 开头即视作
        JSONL，于是唯一那行被跳过。产物为空 + warning，调用方不会误判成成功。
        本用例把这个边界**明确钉住**，避免日后被当成「静默吞异常」误改。
        """
        with caplog.at_level(logging.WARNING, logger="augmentor.streaming"):
            response = tools_env.client.post(
                "/api/system/stream",
                json={
                    "input_file": str(tools_env.broken),
                    "output_file": str(tools_env.out),
                    "operation": "none",
                },
            )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["total_input"] == 1
        assert payload["total_output"] == 0
        assert any("跳过无效行" in r.message for r in caplog.records), caplog.text

    def test_broken_json_array_returns_400(self, tools_env):
        """JSON **数组**格式的语法错误必须 400（与 JSONL 的容错相反）

        数组的结构是明确的：逗号后意外结束意味着文件被截断，继续下去只会
        悄悄丢数据，所以库层直接报错。这里确认它没被降级成 500，也没泄漏
        `json` 模块的英文原文。
        """
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.broken_array),
                "output_file": str(tools_env.out),
                "operation": "none",
            },
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "Expecting" not in detail
        assert "double quotes" not in detail

    def test_unknown_operation_rejected(self, tools_env):
        """未知 operation 必须 400，且列出合法取值"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.jsonl),
                "output_file": str(tools_env.out),
                "operation": "no_such",
            },
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "不支持的 operation" in detail
        for operation in ("quality", "dedup", "clean", "none"):
            assert operation in detail


class TestSystemDependencyRegistry:
    """/api/system/dependency/*"""

    def _register(self, env, **overrides):
        """登记一个数据集

        Args:
            env: tools_env
            **overrides: 覆盖默认 payload

        Returns:
            httpx.Response
        """
        payload = {"name": "ds1", "input_file": str(env.data)}
        payload.update(overrides)
        return env.client.post(
            "/api/system/dependency/datasets",
            params={"registry_path": str(env.registry)},
            json=payload,
        )

    def test_register_then_list(self, tools_env):
        """登记后能在列表里查到"""
        empty = tools_env.client.get(
            "/api/system/dependency/datasets",
            params={"registry_path": str(tools_env.registry)},
        )
        assert empty.status_code == 200, empty.text
        assert empty.json()["datasets"] == []

        created = self._register(tools_env, description="示例集", tags=["t1"])
        assert created.status_code == 200, created.text
        info = created.json()
        assert info["name"] == "ds1"
        assert info["item_count"] == len(ITEMS)
        assert info["description"] == "示例集"
        assert info["tags"] == ["t1"]
        assert info["dataset_id"]

        listed = tools_env.client.get(
            "/api/system/dependency/datasets",
            params={"registry_path": str(tools_env.registry)},
        )
        assert listed.status_code == 200, listed.text
        names = [d["name"] for d in listed.json()["datasets"]]
        assert names == ["ds1"]

    def test_graph_contains_registered_node(self, tools_env):
        """依赖图里应有已登记的节点"""
        assert self._register(tools_env).status_code == 200
        response = tools_env.client.get(
            "/api/system/dependency/graph",
            params={"registry_path": str(tools_env.registry)},
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert [n["name"] for n in payload["nodes"]] == ["ds1"]
        assert payload["edges"] == []
        assert payload["issues"] == []

    def test_register_missing_file_returns_404(self, tools_env):
        """登记不存在的文件 → 404"""
        response = self._register(tools_env, input_file=str(tools_env.tmp / "nope.json"))
        assert response.status_code == 404, response.text

    def test_register_requires_api_key_when_configured(self, tools_env, monkeypatch):
        """配置了密钥后，登记属于写操作，必须带 X-API-Key"""
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        assert self._register(tools_env).status_code == 401
        assert self._register(tools_env, name="ds2").status_code == 401

        ok = tools_env.client.post(
            "/api/system/dependency/datasets",
            params={"registry_path": str(tools_env.registry)},
            json={"name": "ds1", "input_file": str(tools_env.data)},
            headers={"X-API-Key": "s3cret"},
        )
        assert ok.status_code == 200, ok.text


class TestSystemBackups:
    """/api/system/backups 的创建 / 列表 / 恢复 / 删除全生命周期"""

    def _create(self, env, name="snap"):
        """创建备份

        Args:
            env: tools_env
            name: 备份名

        Returns:
            httpx.Response
        """
        return env.client.post(
            "/api/system/backups",
            params={"backup_dir": str(env.backup_dir)},
            json={"input_file": str(env.data), "name": name},
        )

    def _list(self, env):
        """列出备份

        Args:
            env: tools_env

        Returns:
            httpx.Response
        """
        return env.client.get(
            "/api/system/backups", params={"backup_dir": str(env.backup_dir)}
        )

    def test_lifecycle(self, tools_env):
        """创建 → 列表 → 恢复 → 删除，每一步的产物都可验证"""
        assert self._list(tools_env).json()["backups"] == []

        created = self._create(tools_env, "snap")
        assert created.status_code == 200, created.text
        info = created.json()
        assert info["backup_id"] == "snap"
        assert info["item_count"] == len(ITEMS)
        assert info["file_size"] > 0
        assert info["checksum"]
        assert Path(info["backup_path"]).is_file()

        listed = self._list(tools_env)
        assert [b["backup_id"] for b in listed.json()["backups"]] == ["snap"]

        restored = tools_env.client.post(
            "/api/system/backups/snap/restore",
            params={"backup_dir": str(tools_env.backup_dir)},
            json={"output_file": str(tools_env.out)},
        )
        assert restored.status_code == 200, restored.text
        assert restored.json()["item_count"] == len(ITEMS)
        assert _read_json(tools_env.out) == ITEMS

        deleted = tools_env.client.delete(
            "/api/system/backups/snap",
            params={"backup_dir": str(tools_env.backup_dir)},
        )
        assert deleted.status_code == 200, deleted.text
        assert deleted.json()["success"] is True
        assert self._list(tools_env).json()["backups"] == []

    def test_restore_unknown_backup_returns_404(self, tools_env):
        """恢复不存在的备份 → 404

        与 ``DELETE`` 同一语义必须同一状态码；此前这里是 400。
        """
        response = tools_env.client.post(
            "/api/system/backups/nope/restore",
            params={"backup_dir": str(tools_env.backup_dir)},
            json={"output_file": str(tools_env.out)},
        )
        assert response.status_code == 404, response.text
        assert response.json()["detail"] == "备份不存在"

    def test_delete_unknown_backup_returns_404(self, tools_env):
        """删除不存在的备份 → 404"""
        response = tools_env.client.delete(
            "/api/system/backups/nope",
            params={"backup_dir": str(tools_env.backup_dir)},
        )
        assert response.status_code == 404, response.text

    def test_create_backup_missing_source_returns_404(self, tools_env):
        """源文件不存在 → 404"""
        response = tools_env.client.post(
            "/api/system/backups",
            params={"backup_dir": str(tools_env.backup_dir)},
            json={"input_file": str(tools_env.tmp / "nope.json")},
        )
        assert response.status_code == 404, response.text

    def test_write_operations_require_api_key(self, tools_env, monkeypatch):
        """创建 / 恢复 / 删除都属写操作"""
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        assert self._create(tools_env).status_code == 401
        assert tools_env.client.delete(
            "/api/system/backups/x", params={"backup_dir": str(tools_env.backup_dir)}
        ).status_code == 401
        assert tools_env.client.post(
            "/api/system/backups/x/restore",
            params={"backup_dir": str(tools_env.backup_dir)},
            json={"output_file": str(tools_env.out)},
        ).status_code == 401

    def test_backup_dir_outside_roots_rejected(self, tools_env, monkeypatch):
        """备份目录同样受白名单约束"""
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tools_env.tmp))
        response = tools_env.client.get(
            "/api/system/backups",
            params={"backup_dir": str(tools_env.tmp.parent / "escaped")},
        )
        assert response.status_code == 403, response.text


# ============================================================
# 跨端点的错误语义
# ============================================================

READ_ONLY_ENDPOINTS = [
    ("/api/dataset/stats", {"input_file": "F"}),
    ("/api/dataset/validate", {"input_file": "F"}),
    ("/api/dataset/features", {"input_file": "F"}),
    ("/api/dataset/auto-config", {"input_file": "F"}),
    ("/api/system/monitor", {"input_file": "F"}),
    ("/api/system/auto-test", {"input_file": "F"}),
]
WRITE_ENDPOINTS = [
    ("/api/dataset/convert", {"input_file": "F", "output_file": "O"}),
    ("/api/dataset/sample", {"input_file": "F", "output_file": "O", "size": 2}),
    ("/api/dataset/rag", {"input_file": "F", "output_file": "O"}),
    ("/api/system/migrate", {"input_file": "F", "output_file": "O"}),
]
# 注意：`/api/system/stream` **不**在这里。它对坏 JSONL 是容忍语义
# （跳过 + warning），对坏 JSON 数组才是 400 —— 见 TestSystemStream。
ALL_FILE_ENDPOINTS = READ_ONLY_ENDPOINTS + WRITE_ENDPOINTS


def _substitute(payload, *, data, out):
    """把占位符换成真实路径（递归处理嵌套的 list / dict）

    Args:
        payload: 含 F / O 占位符的 payload
        data: 替换 F 的路径
        out: 替换 O 的路径

    Returns:
        新 payload
    """
    def convert(value):
        if isinstance(value, dict):
            return {key: convert(item) for key, item in value.items()}
        if isinstance(value, list):
            return [convert(item) for item in value]
        if value == "F":
            return str(data)
        if value == "O":
            return str(out)
        return value

    return {key: convert(value) for key, value in payload.items()}


class TestSharedErrorSemantics:
    """每个端点的 400 / 403 / 404 语义必须一致"""

    @pytest.mark.parametrize("route,payload", ALL_FILE_ENDPOINTS)
    def test_missing_file_returns_404(self, tools_env, route, payload):
        """文件不存在 → 404（而不是 500 或 400）"""
        body = _substitute(payload, data=tools_env.tmp / "nope.json", out=tools_env.out)
        assert tools_env.client.post(route, json=body).status_code == 404

    @pytest.mark.parametrize("route,payload", ALL_FILE_ENDPOINTS)
    def test_broken_json_returns_400_without_parser_leak(self, tools_env, route, payload):
        """坏 JSON → 400，且**不出现** json 模块的英文解析措辞

        修复前有 6 个端点把
        ``"Expecting property name enclosed in double quotes: line 1 column 3"``
        原样回给了客户端 —— 既泄漏实现细节，又对调用方毫无增量信息。
        """
        body = _substitute(payload, data=tools_env.broken, out=tools_env.out)
        response = tools_env.client.post(route, json=body)
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "数据文件不是合法 JSON" in detail
        # 只回行列号，不回解析器措辞
        assert "Expecting" not in detail
        assert "double quotes" not in detail

    @pytest.mark.parametrize("route,payload", ALL_FILE_ENDPOINTS)
    def test_dotdot_path_returns_400(self, tools_env, route, payload):
        """含 .. 的路径 → 400（而不是尝试解析后再报错）"""
        body = _substitute(payload, data="../escape.json", out=tools_env.out)
        response = tools_env.client.post(route, json=body)
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    @pytest.mark.parametrize("route,payload", ALL_FILE_ENDPOINTS)
    def test_path_outside_roots_returns_403(self, tools_env, route, payload, monkeypatch):
        """白名单之外的绝对路径 → 403"""
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tools_env.tmp))
        outside = tools_env.tmp.parent / "outside.json"
        body = _substitute(payload, data=outside, out=tools_env.out)
        response = tools_env.client.post(route, json=body)
        assert response.status_code == 403, response.text

    def test_write_path_outside_roots_returns_403(self, tools_env, monkeypatch):
        """写路径同样受约束，避免借变换端点写到任意位置"""
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tools_env.tmp))
        response = tools_env.client.post(
            "/api/dataset/convert",
            json={
                "input_file": str(tools_env.data),
                "output_file": str(tools_env.tmp.parent / "escaped.json"),
            },
        )
        assert response.status_code == 403, response.text


class TestHttpExceptionPassthrough:
    """handler 内部抛出的 HTTPException 必须原样透传，不能被降级成 500

    每个 handler 的 `except HTTPException: raise` 都排在最前面，就是为了这个。
    这里用**非 input_file 的入参**（output_dir / output_file / registry_path /
    backup_dir）触发，覆盖参数化用例照不到的那几条分支。
    """

    def test_split_rejects_traversal_output_dir(self, tools_env):
        """output_dir 含 .. → 400"""
        response = tools_env.client.post(
            "/api/dataset/split",
            json={"input_file": str(tools_env.data), "output_dir": "../escaped"},
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_stream_rejects_traversal_output_file(self, tools_env):
        """output_file 含 .. → 400"""
        response = tools_env.client.post(
            "/api/system/stream",
            json={
                "input_file": str(tools_env.jsonl),
                "output_file": "../escaped.jsonl",
            },
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_restore_rejects_traversal_output_file(self, tools_env):
        """恢复目标路径含 .. → 400（而不是被当成「备份不存在」回 404）"""
        response = tools_env.client.post(
            "/api/system/backups/snap/restore",
            params={"backup_dir": str(tools_env.backup_dir)},
            json={"output_file": "../escaped.json"},
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_dependency_list_rejects_traversal_registry(self, tools_env):
        """registry_path 含 .. → 400"""
        response = tools_env.client.get(
            "/api/system/dependency/datasets", params={"registry_path": "../escaped"}
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_dependency_graph_rejects_traversal_registry(self, tools_env):
        """依赖图端点同样约束 registry_path"""
        response = tools_env.client.get(
            "/api/system/dependency/graph", params={"registry_path": "../escaped"}
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_dependency_register_rejects_traversal_registry(self, tools_env):
        """登记端点同样约束 registry_path"""
        response = tools_env.client.post(
            "/api/system/dependency/datasets",
            params={"registry_path": "../escaped"},
            json={"name": "ds1", "input_file": str(tools_env.data)},
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_backup_create_rejects_traversal_dir(self, tools_env):
        """backup_dir 含 .. → 400"""
        response = tools_env.client.post(
            "/api/system/backups",
            params={"backup_dir": "../escaped"},
            json={"input_file": str(tools_env.data)},
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]

    def test_backup_list_rejects_traversal_dir(self, tools_env):
        """列备份同样约束 backup_dir"""
        response = tools_env.client.get(
            "/api/system/backups", params={"backup_dir": "../escaped"}
        )
        assert response.status_code == 400, response.text
        assert "非法组件" in response.json()["detail"]


# ============================================================
# 意外失败 → 500
# ============================================================
#
# 25 个端点的收尾分支都是 `except Exception as e: raise to_http_error(e)`。
# 正常路径与 400/403/404 都覆盖不到它，因为那条路径上的失败都被
# `read_items` / `resolve_data_path` 提前转成了 HTTPException。
# 这里把底层依赖换成「调用即抛」的假对象，确定性命中该分支，
# 并锁住「必须带上原始信息、而不是丢一个空白的 500」。
#
# 每条记录是 (端点, payload, 模块名, 属性名)。属性名可以是函数，也可以是类
# （handler 里的 `from X import Y` 每次调用都会重读模块属性，因此补丁生效）。

POST_FAULT_INJECTIONS = [
    ("/api/dataset/stats", {"input_file": "F"},
     "augmentor.statistics", "calculate_statistics"),
    ("/api/dataset/validate", {"input_file": "F"},
     "augmentor.validation", "DatasetValidator"),
    ("/api/dataset/search", {"input_file": "F", "query": "x"},
     "augmentor.search_enhanced", "search_dataset"),
    ("/api/dataset/compare", {"dataset_a": "F", "dataset_b": "F"},
     "augmentor.compare_enhanced", "compare_datasets_enhanced"),
    ("/api/dataset/features", {"input_file": "F"},
     "augmentor.feature_detect", "FeatureDetector"),
    ("/api/dataset/auto-config", {"input_file": "F"},
     "augmentor.auto_config", "AutoConfig"),
    ("/api/dataset/convert", {"input_file": "F", "output_file": "O"},
     "augmentor.converter", "convert_file"),
    ("/api/dataset/merge", {"inputs": ["F"], "output_file": "O"},
     "augmentor.dataset_ops", "DatasetOperations"),
    ("/api/dataset/sample", {"input_file": "F", "output_file": "O", "size": 1},
     "augmentor.dataset_ops", "DatasetOperations"),
    ("/api/dataset/split", {"input_file": "F", "output_dir": "O"},
     "augmentor.dataset_ops", "DatasetOperations"),
    ("/api/dataset/aggregate", {"datasets": {"a": "F"}, "output_file": "O"},
     "augmentor.aggregator", "DataAggregator"),
    ("/api/dataset/rag", {"input_file": "F", "output_file": "O"},
     "augmentor.rag", "RAGFormatter"),
    ("/api/system/validate-config", {"path": "F"},
     "augmentor.config_validator", "validate_config_file"),
    ("/api/system/monitor", {"input_file": "F"},
     "augmentor.quality_monitor", "monitor_quality"),
    ("/api/system/auto-test", {"input_file": "F"},
     "augmentor.auto_test", "run_dataset_tests"),
    ("/api/system/migrate", {"input_file": "F", "output_file": "O"},
     "augmentor.migration", "migrate_file"),
    ("/api/system/stream", {"input_file": "F", "output_file": "O"},
     "augmentor.streaming", "StreamAugmentor"),
    ("/api/system/dependency/datasets", {"name": "ds1", "input_file": "F"},
     "augmentor.dependency", "DependencyManager"),
    ("/api/system/backups", {"input_file": "F"},
     "augmentor.backup", "create_backup"),
]

FAULT_MESSAGE = "注入的底层故障"


class TestUnexpectedFailureBecomes500:
    """底层意外抛错 → 500，且带上原始信息"""

    @pytest.mark.parametrize("route,payload,module_name,attr", POST_FAULT_INJECTIONS)
    def test_post_endpoint_returns_500(
        self, tools_env, monkeypatch, route, payload, module_name, attr
    ):
        """每个 POST 端点的收尾分支都能把意外异常转成 500"""
        import importlib

        def boom(*args, **kwargs):
            raise RuntimeError(FAULT_MESSAGE)

        monkeypatch.setattr(importlib.import_module(module_name), attr, boom)
        body = _substitute(payload, data=tools_env.data, out=tools_env.out)
        response = tools_env.client.post(route, json=body)
        assert response.status_code == 500, response.text
        assert FAULT_MESSAGE in response.json()["detail"]

    def test_restore_backup_returns_500(self, tools_env, monkeypatch):
        """恢复备份的收尾分支"""
        import augmentor.backup as backup_mod

        def boom(*args, **kwargs):
            raise RuntimeError(FAULT_MESSAGE)

        monkeypatch.setattr(backup_mod, "restore_backup", boom)
        response = tools_env.client.post(
            "/api/system/backups/snap/restore",
            params={"backup_dir": str(tools_env.backup_dir)},
            json={"output_file": str(tools_env.out)},
        )
        assert response.status_code == 500, response.text
        assert FAULT_MESSAGE in response.json()["detail"]

    def test_delete_backup_returns_500(self, tools_env, monkeypatch):
        """删除备份的收尾分支"""
        import augmentor.backup as backup_mod

        def boom(*args, **kwargs):
            raise RuntimeError(FAULT_MESSAGE)

        monkeypatch.setattr(backup_mod, "delete_backup", boom)
        response = tools_env.client.delete(
            "/api/system/backups/snap",
            params={"backup_dir": str(tools_env.backup_dir)},
        )
        assert response.status_code == 500, response.text
        assert FAULT_MESSAGE in response.json()["detail"]

    def test_list_backups_returns_500(self, tools_env, monkeypatch):
        """列备份的收尾分支"""
        import augmentor.backup as backup_mod

        def boom(*args, **kwargs):
            raise RuntimeError(FAULT_MESSAGE)

        monkeypatch.setattr(backup_mod, "list_backups", boom)
        response = tools_env.client.get(
            "/api/system/backups", params={"backup_dir": str(tools_env.backup_dir)}
        )
        assert response.status_code == 500, response.text
        assert FAULT_MESSAGE in response.json()["detail"]

    @pytest.mark.parametrize("path", ["/api/system/dependency/datasets",
                                     "/api/system/dependency/graph"])
    def test_dependency_reads_return_500(self, tools_env, monkeypatch, path):
        """依赖注册表读取的收尾分支"""
        import augmentor.dependency as dependency_mod

        def boom(*args, **kwargs):
            raise RuntimeError(FAULT_MESSAGE)

        monkeypatch.setattr(dependency_mod, "DependencyManager", boom)
        response = tools_env.client.get(
            path, params={"registry_path": str(tools_env.registry)}
        )
        assert response.status_code == 500, response.text
        assert FAULT_MESSAGE in response.json()["detail"]


# ============================================================
# 数据文件形态校验（read_items 的单点收敛）
# ============================================================

SHAPE_REJECTIONS = [
    ("顶层是对象", '{"backups": []}', "顶层必须是 JSON 数组"),
    ("顶层是字符串", '"just a string"', "顶层必须是 JSON 数组"),
    ("顶层是数字", "42", "顶层必须是 JSON 数组"),
    ("顶层是空值", "null", "顶层必须是 JSON 数组"),
    ("元素是标量", "[1, 2, 3]", "第 1 个数据项必须是 JSON 对象"),
    ("元素是字符串", '["a"]', "第 1 个数据项必须是 JSON 对象"),
    ("元素是嵌套数组", '[[{"instruction": "q"}]]', "第 1 个数据项必须是 JSON 对象"),
    ("第二个元素非对象",
     '[{"instruction": "q", "output": "a"}, 42]', "第 2 个数据项必须是 JSON 对象"),
]

# 参与形态校验的端点，以及各端点除 `input_file` 外的必填字段
# （`/api/dataset/search` 的 `query` 是必填，缺了会先撞上 422 而不是形态校验）
SHAPE_ENDPOINTS = {
    "/api/dataset/stats": {},
    "/api/dataset/search": {"query": "q", "method": "contains"},
    "/api/dataset/features": {},
    "/api/quality/evaluate": {},
}


class TestShapeRejection:
    """形态不合法的数据文件必须 400，而不是深入库内部炸成 500

    审计证据（修复前实测，非推断）：

    * `POST /api/dataset/stats {"input_file": "web/package.json"}` → **500**
      `{"detail": "'str' object has no attribute 'items'"}`
    * `POST /api/quality/evaluate` 同输入 → **500** `"'int' object has no attribute 'get'"`
    * 读 `.backups/index.json`（内容是 `{"backups": []}`）→ **500**
      `"'str' object has no attribute 'keys'"`

    两个问题叠在一起：状态码把「调用方传错了」说成「服务挂了」（违反本项目
    §2.8 定下的错误语义），且把 Python 属性错误的原文回显给了客户端。

    根因是 `read_items` 声明返回列表却不校验形态：顶层是对象时下游拿到键
    （字符串），元素是标量时拿到标量，都要深入到 `augmentor/` 里按 dict 用才炸。
    """

    @pytest.mark.parametrize("path", SHAPE_ENDPOINTS)
    @pytest.mark.parametrize("label,body,expected", SHAPE_REJECTIONS,
                             ids=[c[0] for c in SHAPE_REJECTIONS])
    def test_rejected_with_400(self, tools_env, label, body, expected, path):
        # 用序号而非 hash：hash() 每进程加盐，会让失败用例的文件名不可复现
        index = SHAPE_REJECTIONS.index((label, body, expected))
        target = tools_env.tmp / f"shape_{index}.json"
        target.write_text(body, encoding="utf-8")

        payload = {"input_file": str(target), **SHAPE_ENDPOINTS[path]}
        response = tools_env.client.post(path, json=payload)

        assert response.status_code == 400, (
            f"{label} 在 {path} 上应 400，实际 {response.status_code}："
            f"{response.text[:160]}"
        )
        detail = response.json()["detail"]
        assert expected in detail, f"{label}：{detail}"
        # 内部措辞不得外泄——这是本组用例区别于「只看状态码」的部分
        assert "attribute" not in detail and "object has no" not in detail

    def test_empty_array_still_accepted(self, tools_env):
        """空数组是合法的空数据集，不能被形态校验误伤

        加元素级校验时的边界：`[]` 迭代不到任何元素，因此必须仍然 200。
        """
        target = tools_env.tmp / "shape_empty.json"
        target.write_text("[]", encoding="utf-8")

        response = tools_env.client.post(
            "/api/dataset/stats", json={"input_file": str(target)}
        )
        assert response.status_code == 200, response.text

    def test_validate_endpoint_reports_instead_of_rejecting(self, tools_env):
        """`/api/dataset/validate` 对畸形数据项必须**报告**而不是 400

        它的职责就是告诉调用方「这些项不合法」，所以不经 `read_items`，
        自己走 `DatasetValidator.validate_file`。这条用例钉住这个例外，
        避免后来者把形态校验「统一」进去而砍掉验证端点的意义。
        """
        target = tools_env.tmp / "shape_scalars.json"
        target.write_text("[1, 2, 3]", encoding="utf-8")

        response = tools_env.client.post(
            "/api/dataset/validate", json={"input_file": str(target)}
        )
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["total_items"] == 3
        assert body["error_count"] > 0, "标量数据项必须被报成问题，而不是静默通过"


class TestDatasetImpact:
    """/api/dataset/impact —— 增强前后的四项增益必须可核对

    断言全部来自**手算**，不是拿 `ImpactEvaluator` 再跑一遍当预期（同源预言机什么也
    测不出来）。样本挑得能被口算：

    | | before | after |
    |---|---|---|
    | 条数 | 2 | 4 |
    | 唯一文本 | 1（`a` `a`） | 3（`a` `a` `b` `cc`） |
    | 重复率 | 2/2 = 1.0 | 2/4 = 0.5 |
    | 长度 | 1, 1 → std 0 | 1, 1, 1, 2 → std = sqrt(0.1875) |

    → scale_gain = (4-2)/2 = **1.0**；diversity_gain = (3-1)/1 = **2.0**；
    dedup_gain = 1.0-0.5 = **0.5**；length_spread_gain = **0.4330127018922193**。
    """

    BEFORE = [{"instruction": "a", "output": "x"}, {"instruction": "a", "output": "y"}]
    AFTER = [
        {"instruction": "a", "output": "x"},
        {"instruction": "a", "output": "y"},
        {"instruction": "b", "output": "z"},
        {"instruction": "cc", "output": "w"},
    ]

    def _post(self, env, before, after, **extra):
        """写两份数据集并打一次 impact

        Args:
            env: tools_env
            before: 基线数据列表
            after: 增强后数据列表
            **extra: 透传给请求体的其它字段

        Returns:
            httpx.Response
        """
        _write_json(env.tmp / "before.json", before)
        _write_json(env.tmp / "after.json", after)
        return env.client.post(
            "/api/dataset/impact",
            json={
                "before_file": str(env.tmp / "before.json"),
                "after_file": str(env.tmp / "after.json"),
                **extra,
            },
        )

    def test_gains_match_hand_computed(self, tools_env):
        """四项增益逐项对上上面那张表"""
        response = self._post(tools_env, self.BEFORE, self.AFTER)
        assert response.status_code == 200, response.text

        body = response.json()
        assert body["gains"] == pytest.approx(
            {
                "scale_gain": 1.0,
                "diversity_gain": 2.0,
                "dedup_gain": 0.5,
                "length_spread_gain": 0.4330127018922193,
            }
        )
        assert body["before"]["total_items"] == 2
        assert body["after"]["unique_instructions"] == 3
        assert body["before"]["duplicate_rate"] == pytest.approx(1.0)
        assert body["after"]["duplicate_rate"] == pytest.approx(0.5)
        assert body["beneficial"] is True

    def test_min_scale_gain_is_not_decorative(self, tools_env):
        """`min_scale_gain` 必须真的参与判定，而不是个装饰性参数

        同一份数据只挪门槛：数字不变、结论翻转。少了这条，参数写错成「传进去但没人读」
        也能全绿。
        """
        loose = self._post(tools_env, self.BEFORE, self.AFTER)
        strict = self._post(tools_env, self.BEFORE, self.AFTER, min_scale_gain=2.0)

        assert loose.json()["gains"] == strict.json()["gains"]
        assert loose.json()["beneficial"] is True
        assert strict.json()["beneficial"] is False

    def test_empty_baseline_is_400_not_zero_gains(self, tools_env):
        """空基线 → 400：四项增益的分母都是基线，「增益 0.0」会被读成「增强无效」"""
        response = self._post(tools_env, [], self.AFTER)
        assert response.status_code == 400, response.text
        assert "基线数据集为空" in response.json()["detail"]

    def test_empty_after_is_a_verdict_not_an_error(self, tools_env):
        """增强后为空是**结论**：规模增益 -1.0、判定不划算，不能报 400"""
        response = self._post(tools_env, self.BEFORE, [])
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["gains"]["scale_gain"] == pytest.approx(-1.0)
        assert body["beneficial"] is False

    def test_typo_text_field_rejected_instead_of_degrading(self, tools_env):
        """字段名拼错必须 400，而不是把每条文本静默读成空串

        `ImpactEvaluator.measure()` 取字段用 `item.get(field, "")`：拼错时
        「唯一文本 = 1、重复率 = 100%」，一份**根本没被读过**的数据集会被报告成
        「多样性极差」。静默降级比报错危险。
        """
        response = self._post(
            tools_env, self.BEFORE, self.AFTER, text_field="instructionn"
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "instructionn" in detail, "要回显拼错的字段名"
        assert "instruction" in detail, "要给出该数据集实际有的字段"


class TestDatasetEvaluate:
    """/api/dataset/evaluate —— 指标数值必须能被手算复核

    两条样本（全小写英文单词，分词结果就是按空格切开）：

    * A：生成 == 参考 `how do i sort a list` → bleu / rouge_l / similarity 全 1.0；
    * B：生成同上、参考 `how do i filter a list` →
      rouge_l：LCS = `how do i a list` 共 5 个词元，P = R = 5/6 → F1 = 5/6；
      similarity：交集 5 / 并集 7 = 5/7；
      bleu：4 元文法**零重叠**，`compute_bleu` 对「完全无重叠」直接判 0。

    均值：bleu = (1+0)/2 = 0.5；rouge_l = (1+5/6)/2 = 11/12；similarity = (1+5/7)/2 = 6/7。
    """

    GENERATED = [
        {"output": "how do i sort a list"},
        {"output": "how do i sort a list"},
    ]
    REFERENCE = [
        {"output": "how do i sort a list"},
        {"output": "how do i filter a list"},
    ]

    def _post(self, env, generated, reference, **extra):
        """写两侧数据集并打一次 evaluate

        Args:
            env: tools_env
            generated: 生成侧数据列表
            reference: 参考侧数据列表
            **extra: 透传给请求体的其它字段

        Returns:
            httpx.Response
        """
        _write_json(env.tmp / "generated.json", generated)
        _write_json(env.tmp / "reference.json", reference)
        return env.client.post(
            "/api/dataset/evaluate",
            json={
                "generated_file": str(env.tmp / "generated.json"),
                "reference_file": str(env.tmp / "reference.json"),
                **extra,
            },
        )

    def test_batch_metrics_match_hand_computed(self, tools_env):
        """三项指标均值逐项对上注释里的推导"""
        response = self._post(tools_env, self.GENERATED, self.REFERENCE)
        assert response.status_code == 200, response.text

        body = response.json()
        assert body["metrics"] == pytest.approx(
            {"bleu": 0.5, "rouge_l": 11 / 12, "similarity": 6 / 7}
        )
        assert body["sample_count"] == 2
        assert [d["index"] for d in body["details"]] == [0, 1]
        assert body["details"][1]["scores"]["rouge_l"] == pytest.approx(5 / 6)

    def test_metrics_subset(self, tools_env):
        """`metrics` 只要 rouge_l 时，产物里就只有它，均值口径不变"""
        response = self._post(
            tools_env, self.GENERATED, self.REFERENCE, metrics=["rouge_l"]
        )
        assert response.status_code == 200, response.text
        assert response.json()["metrics"] == pytest.approx({"rouge_l": 11 / 12})

    def test_unknown_metric_lists_options(self, tools_env):
        """未知指标 400，且**在读盘之前**就拒掉

        库里的 `ModelEvaluator.__init__` 也会拒绝未知指标（`DataValidationError` → 400），
        所以只断言状态码抓不到这道路由守卫——实测撤掉守卫后本用例仍绿。这条守卫的真正
        契约是「参数校验先于 I/O」：这里故意传一个不存在的 `generated_file`，若顺序颠倒
        就会先撞出 404，等于为一个格式就不对的请求去读盘。
        """
        response = tools_env.client.post(
            "/api/dataset/evaluate",
            json={
                "generated_file": str(tools_env.tmp / "never-created.json"),
                "reference_file": str(tools_env.tmp / "never-created.json"),
                "metrics": ["bleu", "meteor"],
            },
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "未知的评估指标" in detail, "必须是路由守卫的报错，不是透传库内异常"
        assert "rouge_l" in detail, "可选项由 METRIC_FUNCTIONS 反推，不能手抄"

    def test_length_mismatch_rejected_with_counts(self, tools_env):
        """两侧条数不等时按索引配对没有意义 → 400，且把两个数都回出来"""
        response = self._post(tools_env, self.GENERATED, self.REFERENCE[:1])
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "生成侧 2 条" in detail and "参考侧 1 条" in detail

    def test_missing_field_names_the_offending_index(self, tools_env):
        """某一条缺字段 → 400 并带**下标**，不能静默补空串

        补空串等于凭空造一条 0 分样本：均值被稀释，却完全看不出来源。
        """
        response = self._post(
            tools_env, self.GENERATED, [{"output": "how do i sort a list"}, {"text": "x"}]
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "参考侧 第 1 条" in detail, "必须带下标，否则调用方不知道是哪条数据坏了"
        assert "output" in detail

    def test_empty_side_rejected(self, tools_env):
        """空数据集 → 400：空集合上「各项 0.0」会被误读成模型差，实际是没数据"""
        response = self._post(tools_env, [], [])
        assert response.status_code == 400, response.text
        assert "参与评估的数据集为空" in response.json()["detail"]

    def test_include_details_false_keeps_sample_count(self, tools_env):
        """`include_details=false` 只裁 details，`sample_count` 仍是真实条数"""
        response = self._post(
            tools_env, self.GENERATED, self.REFERENCE, include_details=False
        )
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["details"] == []
        assert body["sample_count"] == 2
        assert body["metrics"]["bleu"] == pytest.approx(0.5)

    def test_two_fields_of_the_same_file(self, tools_env):
        """两侧传同一路径、不同字段：这是最常见的「同一份语料的两个版本」用法

        也是「同路径只读一遍盘」那条分支的功能证据（读取次数由
        `test_api_event_loop_blocking.py` 从另一侧盯着）。
        """
        both = [
            {
                "output": "how do i sort a list",
                "reference_output": "how do i sort a list",
            },
            {
                "output": "how do i sort a list",
                "reference_output": "how do i filter a list",
            },
        ]
        _write_json(tools_env.tmp / "both.json", both)
        response = tools_env.client.post(
            "/api/dataset/evaluate",
            json={
                "generated_file": str(tools_env.tmp / "both.json"),
                "reference_file": str(tools_env.tmp / "both.json"),
                "generated_field": "output",
                "reference_field": "reference_output",
            },
        )
        assert response.status_code == 200, response.text
        assert response.json()["metrics"] == pytest.approx(
            {"bleu": 0.5, "rouge_l": 11 / 12, "similarity": 6 / 7}
        )
