# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L215：`enabled` 族第一批的消费证据——出厂四只 `true` 键接进管道阶段入口

**为什么是这四只**：A138 / B5 量到全仓 16 只 `config.<节>.enabled` 在产品代码里零读者，
「接上」因此不是给一条既有语义找位置，而是新建一条消费路径。新建路径就得先量默认值：
出厂 config 里 `quality` / `dedup` / `versioning` / `visualization` 四节是 `true`
⇒ 接上零行为漂移，本轮做掉；其余十二只是 `false` ⇒ 接上等于把当前始终在跑的行为关掉，
是对外破坏面，留在 Backlog 单独拍。

**判据一律取行为差**（L214 定的规矩：接线必须先举出「改配置 ⇒ 产物跟着变」这一格，
举不出就是不许接）：
- `quality.enabled`：关掉 15 条、打开 0 条（假后端产出的变体与原问题不相似，全被滤掉）
- `dedup.enabled`：关掉 5 条、打开 1 条（五条种子各出同一个变体）
- `versioning.enabled`：打开 1 个快照、关掉 0 个
- `visualization.enabled`：打开进入出图阶段、关掉不进入（判据取「阶段是否被进入」而不是
  「图表数量」——本机没装 wordcloud/matplotlib，两臂的图表都是空，数量臂不可证伪）
- 权威序 调用参数 > 配置 > 内置默认：每节都有一臂「配置与调用参数相反」，
  证明配置臂没有反过来压掉调用方

覆盖面：同步入口 `augment_dataset`、异步入口 `augment_async`（两个入口必须同一道判，
否则同一份配置下两条路径出两种结果）、可视化阶段方法、`POST /api/augment/start`
的缺席转发、CLI `augment` 的旗标转发。
"""

import asyncio
import json
from pathlib import Path
from types import SimpleNamespace

import pytest

from augmentor import AugmentorPipeline, load_config
from augmentor.checkpoint import CheckpointManager
from augmentor.config import ModelConfig
from augmentor.models.base import ModelBackend
from augmentor.versioning import VersionManager

AI_DIR = Path(__file__).resolve().parent.parent.parent


class VariantBackend(ModelBackend):
    """假模型后端：每条种子产出 `variants` 个「变体问题 i」

    变体文本与原问题故意不相似，这样质量检查那一臂才有东西可滤。
    """

    def __init__(self, variants: int = 3):
        """初始化

        Args:
            variants: 产出的变体数量
        """
        super().__init__(ModelConfig(type="ollama"))
        self.variants = variants

    def _call_api(self, prompt: str) -> str:
        """返回变体 JSON

        Args:
            prompt: 提示词

        Returns:
            变体列表文本
        """
        return json.dumps(
            [{"instruction": f"变体问题 {i}"} for i in range(self.variants)],
            ensure_ascii=False
        )


def make_pipeline(tmp_path, tag, variants=3, **overrides):
    """构造一份指向临时目录的管道，并按 `节.键` 覆盖配置

    每一臂拿**独立**的 versions / checkpoints 目录：`VersionManager` 的存储目录若在
    多臂之间共用，上一臂的快照会被算进这一臂（第一版探针就这样读出了 4 个版本，
    把「关掉之后应该是 0 个」那一臂读成了假红）。

    Args:
        tmp_path: pytest 临时目录
        tag: 臂名，用于隔离目录
        variants: 假后端每条种子的变体数
        **overrides: `节.键` 形式的配置覆盖

    Returns:
        AugmentorPipeline 实例
    """
    config = load_config(str(AI_DIR / "config.yaml"))
    storage_dir = str(tmp_path / f"versions_{tag}")
    config.versioning.storage_dir = storage_dir
    config.augmentation.num_threads = 2

    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=storage_dir)
    instance.checkpoint_manager = CheckpointManager(
        checkpoint_dir=str(tmp_path / f"ckpt_{tag}"),
        auto_save_interval=1
    )

    backend = VariantBackend(variants)
    instance.model_backend = backend
    instance.context_augmentor.model_backend = backend
    instance.expander.model_backend = backend

    # 阶段入口读的是 self.config.<节>.enabled，构造之后改这只键即可生效
    for dotted, value in overrides.items():
        section, key = dotted.split(".")
        setattr(getattr(instance.config, section), key, value)
    return instance


@pytest.fixture
def seed_file(tmp_path):
    """五条种子的样本文件（每条都能被假后端扩出变体）

    Args:
        tmp_path: pytest 临时目录

    Returns:
        文件路径字符串
    """
    items = [
        {
            "instruction": f"如何申请入住安居乐寓{i}？",
            "input": "",
            "output": "您可以通过官方 App 提交申请，上传身份证明后等待审核。"
        }
        for i in range(5)
    ]
    path = tmp_path / "seed_l215.json"
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(items, f, ensure_ascii=False)
    return str(path)


def run_sync(instance, tmp_path, tag, seed_file, **kwargs):
    """跑一次同步增强，输出文件按臂名落在临时目录

    Args:
        instance: 管道
        tmp_path: pytest 临时目录
        tag: 臂名
        seed_file: 种子文件路径
        **kwargs: 原样转给 `augment_dataset` 的调用参数

    Returns:
        处理报告
    """
    return instance.augment_dataset(
        seed_file, str(tmp_path / f"out_{tag}.json"), **kwargs
    )


class TestQualityEnabledIsConsumed:
    """`config.quality.enabled` 进质量阶段"""

    def test_false_in_config_disables_filtering(self, tmp_path, seed_file):
        """配置关掉质量检查 ⇒ 报告回显 False，且假后端的 15 条变体全部留下"""
        instance = make_pipeline(tmp_path, "q_off", **{"quality.enabled": False})
        report = run_sync(
            instance, tmp_path, "q_off", seed_file,
            use_checkpoint=False, use_dedup=False, use_parallel=False
        )

        assert report["quality_check"] is False
        assert report["output_count"] == 15

    def test_true_in_config_enables_filtering(self, tmp_path, seed_file):
        """同一份输入、只把配置改成打开 ⇒ 不相似的变体被滤光（0 条）

        这一臂同时是「随包配置零漂移」的证据：出厂 `config.yaml` 里这只键就是 true，
        所以省略调用参数的既有调用方，接上配置之后的行为与 L215 之前完全一致。
        """
        instance = make_pipeline(tmp_path, "q_on", **{"quality.enabled": True})
        report = run_sync(
            instance, tmp_path, "q_on", seed_file,
            use_checkpoint=False, use_dedup=False, use_parallel=False
        )

        assert report["quality_check"] is True
        assert report["output_count"] == 0

    def test_explicit_true_beats_config_false(self, tmp_path, seed_file):
        """配置 False 但调用参数点名 True ⇒ 调用参数赢（权威序，不是配置压人）"""
        instance = make_pipeline(tmp_path, "q_beat", **{"quality.enabled": False})
        report = run_sync(
            instance, tmp_path, "q_beat", seed_file,
            use_checkpoint=False, use_quality_check=True,
            use_dedup=False, use_parallel=False
        )

        assert report["quality_check"] is True
        assert report["output_count"] == 0


class TestDedupEnabledIsConsumed:
    """`config.dedup.enabled` 进去重阶段"""

    def test_false_in_config_keeps_duplicates(self, tmp_path, seed_file):
        """每条种子出同一个变体：关掉去重 ⇒ 5 条重复留在产物里"""
        instance = make_pipeline(tmp_path, "d_off", variants=1, **{"dedup.enabled": False})
        report = run_sync(
            instance, tmp_path, "d_off", seed_file,
            use_checkpoint=False, use_quality_check=False, use_parallel=False
        )

        assert report["dedup"] is False
        assert report["output_count"] == 5

    def test_true_in_config_merges_duplicates(self, tmp_path, seed_file):
        """同一份输入、只把配置改成打开 ⇒ 合并成 1 条"""
        instance = make_pipeline(tmp_path, "d_on", variants=1, **{"dedup.enabled": True})
        report = run_sync(
            instance, tmp_path, "d_on", seed_file,
            use_checkpoint=False, use_quality_check=False, use_parallel=False
        )

        assert report["dedup"] is True
        assert report["output_count"] == 1

    def test_explicit_false_beats_config_true(self, tmp_path, seed_file):
        """配置 True 但调用参数点名 False ⇒ 调用参数赢，重复保留"""
        instance = make_pipeline(tmp_path, "d_beat", variants=1, **{"dedup.enabled": True})
        report = run_sync(
            instance, tmp_path, "d_beat", seed_file,
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert report["dedup"] is False
        assert report["output_count"] == 5


class TestVersioningEnabledIsConsumed:
    """`config.versioning.enabled` 与 `auto_snapshot` 串联"""

    def test_enabled_creates_snapshot(self, tmp_path, seed_file):
        """总闸打开 ⇒ 增强收尾打一个快照"""
        instance = make_pipeline(tmp_path, "v_on")
        run_sync(
            instance, tmp_path, "v_on", seed_file,
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert len(instance.version_manager.list_versions()) == 1

    def test_disabled_creates_no_snapshot(self, tmp_path, seed_file):
        """总闸关掉 ⇒ 一个都不打，即使 `auto_snapshot` 仍是 True

        两键是串联而不是互相覆盖：用户关掉「版本管理」之后仍然自动打点，
        等于代码替用户回答「但自动打点你没关」。
        """
        instance = make_pipeline(tmp_path, "v_off", **{"versioning.enabled": False})
        run_sync(
            instance, tmp_path, "v_off", seed_file,
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert instance.config.versioning.auto_snapshot is True
        assert len(instance.version_manager.list_versions()) == 0

    def test_auto_snapshot_false_also_stops(self, tmp_path, seed_file):
        """既有那只 `auto_snapshot` 的判据不能被新总闸顶掉（回归臂）"""
        instance = make_pipeline(tmp_path, "v_auto", **{"versioning.auto_snapshot": False})
        run_sync(
            instance, tmp_path, "v_auto", seed_file,
            use_checkpoint=False, use_quality_check=False,
            use_dedup=False, use_parallel=False
        )

        assert instance.config.versioning.enabled is True
        assert len(instance.version_manager.list_versions()) == 0


class TestVisualizationEnabledIsConsumed:
    """`config.visualization.enabled` 进出图阶段"""

    @staticmethod
    def attach_recorder(instance, calls):
        """把出图那一层换成计数器

        本机未安装 wordcloud / matplotlib，两臂的真实图表都是空字典，
        「图表数量」那一臂不可证伪。判据因此取**阶段是否被进入**：
        配置关掉时那一层一次都不该被调用。

        Args:
            instance: 管道
            calls: 记录列表

        Returns:
            同一个管道实例
        """
        def fake(items):
            calls.append(items)
            return {}

        instance.visualizer.generate_all_visualizations = fake
        return instance

    def test_disabled_skips_stage(self, tmp_path, seed_file):
        """总闸关掉 ⇒ 不进阶段、返回空清单（不是抛错，返回形状不变）"""
        calls = []
        instance = make_pipeline(tmp_path, "viz_off", **{"visualization.enabled": False})
        self.attach_recorder(instance, calls)

        result = instance.visualize_dataset(seed_file, str(tmp_path / "viz_off"))

        assert result == {}
        assert calls == []

    def test_enabled_enters_stage(self, tmp_path, seed_file):
        """总闸打开 ⇒ 进入阶段一次，并把读到的五条数据交下去"""
        calls = []
        instance = make_pipeline(tmp_path, "viz_on")
        self.attach_recorder(instance, calls)

        result = instance.visualize_dataset(seed_file, str(tmp_path / "viz_on"))

        assert result == {}
        assert len(calls) == 1
        assert len(calls[0]) == 5


class TestAugmentAsyncFollowsConfig:
    """异步入口与同步入口同一道判"""

    def test_absent_params_follow_config(self, tmp_path, seed_file):
        """省略调用参数 ⇒ 异步入口也读配置"""
        instance = make_pipeline(
            tmp_path, "a_off", variants=1,
            **{"quality.enabled": False, "dedup.enabled": False}
        )
        report = asyncio.run(
            instance.augment_async(seed_file, str(tmp_path / "out_a_off.json"))
        )

        assert report["quality_check"] is False
        assert report["dedup"] is False
        assert report["output_count"] == 5

    def test_explicit_params_beat_config(self, tmp_path, seed_file):
        """配置两只 true、调用参数点名 False ⇒ 参数赢，产物回到未过滤未合并"""
        instance = make_pipeline(tmp_path, "a_beat", variants=1)
        report = asyncio.run(
            instance.augment_async(
                seed_file, str(tmp_path / "out_a_beat.json"),
                use_quality_check=False, use_dedup=False
            )
        )

        assert instance.config.quality.enabled is True
        assert report["quality_check"] is False
        assert report["dedup"] is False
        assert report["output_count"] == 5

    def test_both_entries_agree_on_the_same_config(self, tmp_path, seed_file):
        """同一份配置下同步与异步必须给同样的旗标（两入口一条判据的意义）"""
        overrides = {"quality.enabled": False, "dedup.enabled": False}
        sync_instance = make_pipeline(tmp_path, "same_sync", variants=1, **overrides)
        async_instance = make_pipeline(tmp_path, "same_async", variants=1, **overrides)

        sync_report = run_sync(
            sync_instance, tmp_path, "same_sync", seed_file,
            use_checkpoint=False, use_parallel=False
        )
        async_report = asyncio.run(
            async_instance.augment_async(
                seed_file, str(tmp_path / "out_same_async.json")
            )
        )

        assert sync_report["quality_check"] == async_report["quality_check"] is False
        assert sync_report["dedup"] == async_report["dedup"] is False


class TestApiStartForwardsAbsence:
    """`POST /api/augment/start` 只转发，不在端点里补默认值"""

    @pytest.fixture
    def recording_env(self, tmp_path, monkeypatch):
        """把管道挂进 API 依赖，并把 `augment_dataset` 换成参数记录器

        Args:
            tmp_path: pytest 临时目录
            monkeypatch: pytest fixture

        Returns:
            (TestClient, 输入文件名, 输出文件名, 记录列表)
        """
        from fastapi.testclient import TestClient

        import api.deps as deps
        from api.main import app

        instance = make_pipeline(tmp_path, "api")
        recorded = []
        instance.augment_dataset = lambda *args, **kwargs: recorded.append(kwargs)
        monkeypatch.setattr(deps, "_pipeline", instance)
        monkeypatch.chdir(tmp_path)

        items = [{"instruction": "问题？", "input": "", "output": "回答"}]
        (Path.cwd() / "in.json").write_text(
            json.dumps(items, ensure_ascii=False), encoding="utf-8"
        )
        return TestClient(app), "in.json", "out.json", recorded

    def test_omitted_fields_reach_pipeline_as_none(self, recording_env):
        """请求体不写这两只字段 ⇒ 到管道是 `None`，也就是「交给配置」

        端点若在缺席时补成 True，配置臂就永远拿不到控制权——那正是 L214 拆掉的
        「键写在请求模型里、读者只有代码默认值」的幽灵面形状。
        """
        client, in_name, out_name, recorded = recording_env
        response = client.post(
            "/api/augment/start",
            json={"input_file": in_name, "output_file": out_name, "use_checkpoint": False},
        )

        assert response.status_code == 200
        assert recorded[-1]["use_quality_check"] is None
        assert recorded[-1]["use_dedup"] is None

    def test_explicit_false_is_forwarded_verbatim(self, recording_env):
        """显式 false ⇒ 原样转发 False，不是 None"""
        client, in_name, out_name, recorded = recording_env
        response = client.post(
            "/api/augment/start",
            json={
                "input_file": in_name,
                "output_file": out_name,
                "use_quality": False,
                "use_dedup": False,
                "use_checkpoint": False,
            },
        )

        assert response.status_code == 200
        assert recorded[-1]["use_quality_check"] is False
        assert recorded[-1]["use_dedup"] is False


class TestCliForwardsFlagAbsence:
    """CLI `augment` 的旗标缺席同样交给配置"""

    @staticmethod
    def patch_recorder(monkeypatch, recorded):
        """把类上的 `augment_dataset` 换成记录器

        Args:
            monkeypatch: pytest fixture
            recorded: 记录列表
        """
        def recorder(self, *args, **kwargs):
            recorded.append(kwargs)
            return {}

        monkeypatch.setattr(AugmentorPipeline, "augment_dataset", recorder)

    def test_without_flags_passes_none(self, tmp_path, monkeypatch):
        """不给 `--no-quality` / `--no-dedup` ⇒ 传下去是 `None`（由配置定）"""
        from augmentor.cli.commands.pipeline import run_augment

        recorded = []
        self.patch_recorder(monkeypatch, recorded)
        args = SimpleNamespace(
            input="in.json", output=str(tmp_path / "out.json"),
            no_checkpoint=False, no_quality=False, no_dedup=False
        )

        run_augment(args, load_config(str(AI_DIR / "config.yaml")))

        assert recorded[-1]["use_quality_check"] is None
        assert recorded[-1]["use_dedup"] is None

    def test_flags_pass_false(self, tmp_path, monkeypatch):
        """给了旗标 ⇒ 传下去是 False，配置让位"""
        from augmentor.cli.commands.pipeline import run_augment

        recorded = []
        self.patch_recorder(monkeypatch, recorded)
        args = SimpleNamespace(
            input="in.json", output=str(tmp_path / "out.json"),
            no_checkpoint=True, no_quality=True, no_dedup=True
        )

        run_augment(args, load_config(str(AI_DIR / "config.yaml")))

        assert recorded[-1]["use_quality_check"] is False
        assert recorded[-1]["use_dedup"] is False
        assert recorded[-1]["use_checkpoint"] is False
