# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置写入路径的运行时判据测试（L73 / A117）

L45–L72 七轮接上的判据一律住在各配置节的 `__post_init__` 里，而 `setattr` 不会
再次触发它。本文件锁的是那条旁路被堵上后的三件事：

1. `apply_section_update` 写后立刻复查该节自己的判据，越界即抛且整批回滚；
2. `POST /api/config` 对越界值回 400，内存与磁盘都停在原值（不再「当次 200、
   重启即抛」）；
3. 路由结构上不再存在裸 `setattr` 写配置节，且 `DataValidationError` 分支排在
   兜底 `Exception` 之前（否则 400 会静默退化成 500）。
"""

import ast
import dataclasses
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent
ROUTE_FILE = AI_DIR / "api" / "routes" / "config.py"

# 与 `AugmentationConfig.__post_init__` 同源的一条明确越界值：
# `max_retry_wait` 的界是 `retry.MAX_RETRY_AFTER`（300 s），9999 必判负。
BAD_KEY = "max_retry_wait"
BAD_VALUE = 9999
GOOD_KEY = "retry_delay"
GOOD_VALUE = 2.5

# `POST /api/config` 可写的七节（与路由里的清单一致）。
WRITABLE_SECTIONS = [
    "augmentation", "quality", "dedup", "export", "vector", "rag", "multimodal",
]


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """构造使用临时目录的管道桩（同 `test_api_config_extended.py`）

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        带真实 `AppConfig` 的管道桩
    """
    from augmentor import load_config

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = type("PipelineStub", (), {})()
    instance.config = config
    monkeypatch.chdir(tmp_path)  # save_config 写 CWD 的 config.yaml，隔离到临时目录
    import api.deps as deps
    monkeypatch.setattr(deps, "_pipeline", instance)
    yield instance


def _client():
    from fastapi.testclient import TestClient

    from api.main import app
    return TestClient(app)


def _seed_cwd_config() -> bytes:
    """把出厂 config.yaml 复制到 CWD 并返回其字节，供「磁盘未被改动」比对"""
    import os

    payload = (AI_DIR / "config.yaml").read_bytes()
    Path(os.getcwd(), "config.yaml").write_bytes(payload)
    return payload


def _snapshot(section) -> dict:
    """配置节的全字段快照，用于断言「抛异常后一个字都没变」"""
    return dataclasses.asdict(section)


class TestApplySectionUpdateSdk:
    """直接打 `apply_section_update`，不经 HTTP"""

    def test_out_of_range_value_raises(self):
        from augmentor.config import AugmentationConfig, apply_section_update
        from augmentor.exceptions import DataValidationError

        section = AugmentationConfig()
        with pytest.raises(DataValidationError, match="max_retry_wait"):
            apply_section_update(section, {BAD_KEY: BAD_VALUE})

    def test_section_is_untouched_after_a_rejected_write(self):
        """抛负后必须逐字段回到调用前状态——只回滚「本条」是不够的"""
        from augmentor.config import AugmentationConfig, apply_section_update
        from augmentor.exceptions import DataValidationError

        section = AugmentationConfig()
        before = _snapshot(section)
        with pytest.raises(DataValidationError):
            apply_section_update(section, {BAD_KEY: BAD_VALUE})
        assert _snapshot(section) == before

    def test_batch_is_all_or_nothing(self):
        """同批里合法的那条也不能留下：端点契约是 `success` 才代表保存过"""
        from augmentor.config import AugmentationConfig, apply_section_update
        from augmentor.exceptions import DataValidationError

        section = AugmentationConfig()
        before = _snapshot(section)
        with pytest.raises(DataValidationError):
            apply_section_update(
                section, {GOOD_KEY: GOOD_VALUE, BAD_KEY: BAD_VALUE}
            )
        assert getattr(section, GOOD_KEY) == before[GOOD_KEY]
        assert _snapshot(section) == before

    def test_rollback_reaches_keys_applied_before_the_offender(self):
        """坏值夹在批次**中间**：它前面已写入的 3 条一条都不许留下。

        这条是 L73 注入分判 m3（`applied[-1:]`，只回滚最后一条）逼出来的：
        单键批次下 `applied[-1:] == applied`，「坏值在首」的批次里回滚尾部等于
        什么都没做 —— 只有夹在中间才有 ≥2 条前缀可漏。首/尾两种排法都咬不住部分回滚。
        """
        from augmentor.config import AugmentationConfig, apply_section_update
        from augmentor.exceptions import DataValidationError

        section = AugmentationConfig()
        before = _snapshot(section)
        with pytest.raises(DataValidationError):
            apply_section_update(section, {
                "max_retries": 7,
                GOOD_KEY: GOOD_VALUE,
                "retry_jitter": 0.5,
                BAD_KEY: BAD_VALUE,
            })
        assert _snapshot(section) == before

    def test_reversal_order_rolls_back_the_earliest_key_too(self):
        """回滚必须逆序且覆盖整批：把坏值放第一条，后面三条好值也一条不留"""
        from augmentor.config import AugmentationConfig, apply_section_update
        from augmentor.exceptions import DataValidationError

        section = AugmentationConfig()
        before = _snapshot(section)
        with pytest.raises(DataValidationError):
            apply_section_update(section, {
                BAD_KEY: BAD_VALUE,
                GOOD_KEY: GOOD_VALUE,
                "retry_jitter": 0.5,
                "max_retries": 7,
            })
        assert _snapshot(section) == before

    def test_legal_batch_lands_and_reports_nothing_ignored(self):
        from augmentor.config import AugmentationConfig, apply_section_update

        section = AugmentationConfig()
        ignored = apply_section_update(section, {GOOD_KEY: GOOD_VALUE, "max_retries": 2})
        assert ignored == []
        assert section.retry_delay == GOOD_VALUE
        assert section.max_retries == 2

    def test_unknown_keys_are_returned_not_raised(self):
        """丢弃未知键是既有契约（不崩），出声交给调用方"""
        from augmentor.config import AugmentationConfig, apply_section_update

        section = AugmentationConfig()
        ignored = apply_section_update(section, {"no_such_knob": 1, "max_retries": 4})
        assert ignored == ["no_such_knob"]
        assert section.max_retries == 4
        assert not hasattr(section, "no_such_knob")

    def test_empty_updates_is_a_noop(self):
        from augmentor.config import AugmentationConfig, apply_section_update

        section = AugmentationConfig()
        before = _snapshot(section)
        assert apply_section_update(section, {}) == []
        assert _snapshot(section) == before

    @pytest.mark.parametrize("key,value", [
        ("variants_per_seed", None),
        ("num_threads", None),
        ("retry_delay", None),
        ("request_timeout", None),
    ])
    def test_none_is_rejected_for_typed_knobs(self, key, value):
        """`require_count`/`require_seconds` 对 `None` 短路，null 由节判据挡下"""
        from augmentor.config import AugmentationConfig, apply_section_update
        from augmentor.exceptions import DataValidationError

        section = AugmentationConfig()
        before = _snapshot(section)
        with pytest.raises(DataValidationError, match="不能是 null"):
            apply_section_update(section, {key: value})
        assert _snapshot(section) == before

    @pytest.mark.parametrize("key,value", [
        (BAD_KEY, BAD_VALUE),
        ("variants_per_seed", 10**6),
        ("retry_jitter", 50.0),
        ("request_timeout", float("nan")),
    ])
    def test_naive_setattr_still_bypasses_every_gate(self, key, value):
        """钉住「为什么必须有本函数」：裸 `setattr` 至今不触发 `__post_init__`。

        这条不是回归测试而是机制测试 —— 若哪天 `setattr` 自己会判（或本函数被
        换回裸写入），它会红，逼着改写这一族文档而不是留下一个假守卫。
        """
        from augmentor.config import AugmentationConfig

        section = AugmentationConfig()
        setattr(section, key, value)  # noqa: B010 - 故意绕开判据
        # `is` 而不是 `==`：本批里 NaN 也要能钉住（`nan != nan`，用 `==` 会假红）
        assert getattr(section, key) is value

    def test_ratchet_names_sections_without_runtime_gates(self):
        """七节**全部**接上运行时判据 ⇒ 本函数的守卫对每一节都真的在做事

        用精确集合而不是「至少有一条没判据」：任何一节接上或拆掉 `__post_init__`
        都会让本用例红，那时应同步改这个集合并说明理由。

        **本条是第二次翻转**（L82 / A118 余四节）：`export` / `rag` / `vector` /
        `multimodal` 四节接上判据后，集合从四节缩到**空**。L73 写下这条时它守的是
        「四节的 `apply_section_update` 是空转」，L76 收了 quality/dedup，本轮收完
        最后四节 —— 于是断言的形状也从「等于那四节」换成「与七节交集为空」的硬条件，
        加上全 20 节现量名单一条（计数来自 `Temp/l82q/section_gates.json`，不靠回忆）：
        还有 11 节没有判据，但它们**不在** `POST /api/config` 的可写清单里，所以本节
        的守卫不欠它们账。
        """
        from augmentor import load_config

        config = load_config(str(AI_DIR / "config.yaml"))
        ungated = sorted(
            name for name in WRITABLE_SECTIONS
            if not hasattr(type(getattr(config, name)), "__post_init__")
        )
        assert ungated == [], "七节里有节的运行时判据被拆掉了：%s" % ungated

        all_sections = sorted(
            f.name for f in dataclasses.fields(config)
            if dataclasses.is_dataclass(getattr(config, f.name))
            and not hasattr(type(getattr(config, f.name)), "__post_init__")
        )
        assert all_sections == [
            "active_learning", "benchmark", "context", "evaluation", "expander",
            "frameworks", "multilingual", "sampler", "tracker", "versioning",
            "visualization",
        ], "A118 现量变了：按本轮同款推导重测再改这里，别照抄"

    def test_sections_without_runtime_gates_still_write_through(self):
        """设计选择 3 的行为面：没有 `__post_init__` 的节照旧写入，不抛也不吞键

        为什么本轮补它：覆盖报表里 `config.py` 的 `if post_init is not None` 是本轮
        唯一新增的偏支 —— 七节全部接上判据后，假臂只余上面那 11 节会走，而它们全在
        `POST /api/config` 的可写清单之外，只有 SDK 直构能到这里。没有这条用例，下一轮
        把它当死代码删掉就是一次对外行为变更（A140 接节时这一支还要继续走）。
        """
        from augmentor import load_config
        from augmentor.config import apply_section_update

        config = load_config(str(AI_DIR / "config.yaml"))
        all_sections = sorted(
            f.name for f in dataclasses.fields(config)
            if dataclasses.is_dataclass(getattr(config, f.name))
            and not hasattr(type(getattr(config, f.name)), "__post_init__")
        )
        assert all_sections, "全 20 节都接上了判据 ⇒ 假臂无来源，本用例连同设计选择 3 一起退役"
        flipped = 0
        for name in all_sections:
            section = getattr(config, name)
            assert not hasattr(type(section), "__post_init__"), \
                "%s 节接上了判据，本用例的靶子消失，改走七节那条路" % name
            old = getattr(section, dataclasses.fields(section)[0].name)
            # 首字段是 bool ⇒ 写一个**不同**的值，断言才不是自等
            new = (not old) if isinstance(old, bool) else old
            flipped += isinstance(old, bool)
            ignored = apply_section_update(
                section, {dataclasses.fields(section)[0].name: new, "no_such_knob": 1}
            )
            assert ignored == ["no_such_knob"], name
            assert getattr(section, dataclasses.fields(section)[0].name) == new, name
        assert flipped == len(all_sections), \
            "有节的首字段不再是 bool ⇒ 那一节的写入断言退化成自等，换字段选择再改这里"


class TestApiRejectsOutOfRange:
    """端点级：越界值 ⇒ 400，且内存与磁盘都停在原值"""

    def test_400_for_out_of_range_value(self, pipeline, monkeypatch):
        client = _client()
        response = client.post("/api/config", json={"augmentation": {BAD_KEY: BAD_VALUE}})
        assert response.status_code == 400

    def test_400_detail_names_the_key_and_the_offending_value(self, pipeline, monkeypatch):
        """文案要能直接定位（哪个键、什么值、界在哪），且不回 Python 栈"""
        client = _client()
        response = client.post("/api/config", json={"augmentation": {BAD_KEY: BAD_VALUE}})
        detail = response.json()["detail"]
        assert f"augmentation.{BAD_KEY}" in detail
        assert str(BAD_VALUE) in detail
        assert "Traceback" not in detail

    def test_400_leaves_live_config_untouched(self, pipeline, monkeypatch):
        client = _client()
        before = getattr(pipeline.config.augmentation, BAD_KEY)
        response = client.post("/api/config", json={"augmentation": {BAD_KEY: BAD_VALUE}})
        assert response.status_code == 400
        assert getattr(pipeline.config.augmentation, BAD_KEY) == before

    def test_400_leaves_the_config_file_untouched(self, pipeline, monkeypatch):
        """A117 的本体：坏值曾经能被 `save_config` 写进 YAML，之后重启即抛"""
        import os

        client = _client()
        original = _seed_cwd_config()
        response = client.post("/api/config", json={"augmentation": {BAD_KEY: BAD_VALUE}})
        assert response.status_code == 400
        on_disk = Path(os.getcwd(), "config.yaml").read_bytes()
        assert on_disk == original
        assert str(BAD_VALUE).encode() not in on_disk

    def test_400_leaves_no_partial_mutation_across_sections(self, pipeline, monkeypatch):
        """跨节批次也一样：`quality.threshold` 合法也不能留在内存里"""
        client = _client()
        before_threshold = pipeline.config.quality.threshold
        response = client.post(
            "/api/config",
            json={"quality": {"threshold": 0.42}, "augmentation": {BAD_KEY: BAD_VALUE}},
        )
        assert response.status_code == 400
        assert pipeline.config.quality.threshold == before_threshold

    def test_legal_update_still_returns_200_and_persists(self, pipeline, monkeypatch):
        """收紧不能伤到正常写入：200 + 落盘 + 能读回同一个值"""
        import os

        from augmentor.config import load_config

        client = _client()
        _seed_cwd_config()
        response = client.post(
            "/api/config", json={"augmentation": {GOOD_KEY: GOOD_VALUE}}
        )
        assert response.status_code == 200
        assert response.json()["success"] is True
        assert pipeline.config.augmentation.retry_delay == GOOD_VALUE
        reloaded = load_config(str(Path(os.getcwd(), "config.yaml")))
        assert reloaded.augmentation.retry_delay == GOOD_VALUE

    def test_unknown_key_contract_unchanged(self, pipeline, monkeypatch):
        """L72 那一半（丢弃要出声）不能被本轮改动带跑"""
        client = _client()
        response = client.post("/api/config", json={"quality": {"no_such_key": 123}})
        assert response.status_code == 200
        assert response.json()["ignored_keys"] == ["quality.no_such_key"]

    def test_the_three_link_chain_now_breaks_at_link_one(self, pipeline, monkeypatch):
        """复现 L72 取证链（`probe_setattr_bypass.json`）并证明它断在第一环。

        改前三段：坏值挂上内存 → `save_config` 原样落盘 → 下次加载抛。
        现在第一段就抛，所以后两段都不成立 —— 磁盘上留的仍是合法配置。
        """
        import os

        from augmentor.config import load_config

        client = _client()
        _seed_cwd_config()
        response = client.post(
            "/api/config", json={"augmentation": {GOOD_KEY: GOOD_VALUE,
                                                  BAD_KEY: BAD_VALUE}}
        )
        assert response.status_code == 400
        reloaded = load_config(str(Path(os.getcwd(), "config.yaml")))
        assert reloaded.augmentation.retry_delay != GOOD_VALUE


class TestRouteStructure:
    """判据型守卫走 AST（L56 立的规矩：正则匹配的是文本，不是结构）"""

    @staticmethod
    def _update_config_node():
        tree = ast.parse(ROUTE_FILE.read_text(encoding="utf-8"))
        for node in tree.body:
            if isinstance(node, ast.AsyncFunctionDef) and node.name == "update_config":
                return node
        raise AssertionError("update_config 不在了，守卫需要重写")

    def test_validation_handler_precedes_the_generic_one(self):
        """`except DataValidationError` 必须排在 `except Exception` 前，否则 400 变 500"""
        handlers = []
        for node in ast.walk(self._update_config_node()):
            if isinstance(node, ast.ExceptHandler):
                handlers.append(node.type.id if isinstance(node.type, ast.Name) else None)
        assert handlers.index("DataValidationError") < handlers.index("Exception")

    def test_route_writes_sections_only_through_the_shared_helper(self):
        """路由里不再出现裸 `setattr(节, ...)`；写入统一经 `apply_section_update`"""
        calls = set()
        for node in ast.walk(self._update_config_node()):
            if isinstance(node, ast.Call):
                if isinstance(node.func, ast.Name):
                    calls.add(node.func.id)
                elif isinstance(node.func, ast.Attribute):
                    calls.add(node.func.attr)
        assert "setattr" not in calls
        assert "apply_section_update" in calls

    def test_every_writable_section_reaches_the_helper(self):
        """路由的可写节清单与本文件的清单一致：加节必须同时加守卫"""
        sections = None
        for node in ast.walk(self._update_config_node()):
            if isinstance(node, ast.For) and isinstance(node.target, ast.Name) \
                    and node.target.id == "section":
                value = node.iter
                if isinstance(value, (ast.List, ast.Tuple)):
                    sections = [
                        elt.value for elt in value.elts if isinstance(elt, ast.Constant)
                    ]
                break
        assert sections == WRITABLE_SECTIONS
