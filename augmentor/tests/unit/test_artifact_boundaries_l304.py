# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""产物边界两格：A407（GET 落盘跟着白名单走）与 A398（校验回执脱敏）

**这一轮收的是哪两格**：

* A407 —— `GET /api/visualize/{filename}` 名义上是读，实际按
  `AugmentorPipeline.visualize_dataset` 的默认参数 `"visualizations"` 在**进程当前
  工作目录**建目录并写 PNG；落点随启动目录漂移，也在数据白名单之外。改法是把
  `deps.default_visualization_dir()`（白名单首根下的 `visualizations`，与
  `default_registry_dir` / `default_backup_dir` 同构）显式传给管道，并把回执里的
  绝对落点折回 `visualizations/<名>`。同轮收 A363：`visualize_dataset` 只改
  `copy.copy(self.visualizer)` 的局部视图，不再把共享单例的 `output_dir` 搬走。
* A398 —— `POST /api/system/validate-config` 的 200 回执把 PyYAML 异常里的
  **服务器绝对落点**原样回显。改法只在回执层脱敏：`message` 里的绝对落点换成
  调用方给的原字符串（缺省时用文件名），行列号与 `path` 逻辑键一字不动；鉴权档位
  不变（只读面继续不挂 `verify_api_key`）。

**这一把尺为什么这样摆**：

1. 白名单根与工作目录**分开**建（`tmp/data` 与 `tmp/cwd`），否则「产物落在根下」
   与「什么也没落到 CWD」两支会互相掩护。
2. 回执断言直接读 `message` 字符串。`str(tmp) in json.dumps(payload)` 会被
   JSON 的反斜杠转义骗成假阴性，不能这么写。
3. 两支「脱敏/折回承重」的牙齿：把 `_redact_path` 换成恒等函数、把
   `_public_chart_paths` 换成恒等函数，相应的绝对落点必须立刻出现在回执里。
   没有这两支，上面那几支可能只是空转。
4. A363 用「记录器可视化器」而非真出图：当前 venv 没有 matplotlib / wordcloud，
   真实 PNG 不可观测；`copy.copy` 会共享 `__dict__` 里的列表引用，因此局部视图
   在调用时看到的 `output_dir` 能被记下来。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走
类内独立用例，两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_artifact_boundaries_l304.py -q --no-cov`
"""
import json
from pathlib import Path
from types import SimpleNamespace

import pytest

BAD_YAML = "models: [is: not: valid\nsecret: sk-SECRET\n"


def _seed_dataset(path: Path) -> None:
    """写一份最小合法数据集（`visualize_data` 只判存在，不判内容）

    Args:
        path: 落盘路径
    """
    path.write_text(
        json.dumps([{"instruction": "问题", "input": "", "output": "答案"}], ensure_ascii=False),
        encoding="utf-8",
    )


@pytest.fixture
def artifact_env(tmp_path, monkeypatch):
    """白名单根、工作目录、管道替身三件套

    根目录与 CWD 刻意不同：A407 要同时证明「产物落在根下」与「CWD 零痕迹」。

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        SimpleNamespace，含 client / root / cwd / calls / deps
    """
    from fastapi.testclient import TestClient

    import api.deps as deps
    from api.main import app

    root = tmp_path / "data"
    cwd = tmp_path / "cwd"
    root.mkdir()
    cwd.mkdir()
    _seed_dataset(root / "a.json")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(cwd)

    calls = []

    def fake_visualize(file_path, output_dir):
        """按端点传入的目录真建一张图，证明那个入参被使用"""
        calls.append((file_path, output_dir))
        chart = Path(output_dir) / "wordcloud.png"
        chart.parent.mkdir(parents=True, exist_ok=True)
        chart.write_bytes(b"png")
        return {"wordcloud": str(chart)}

    monkeypatch.setattr(
        deps, "_pipeline", SimpleNamespace(visualize_dataset=fake_visualize)
    )
    yield SimpleNamespace(
        client=TestClient(app), root=root, cwd=cwd, calls=calls, deps=deps
    )


class TestVisualizeLandsInsideTheGate:
    """A407：产物跟着白名单首根走，回执不出现服务器绝对路径"""

    def test_default_chart_dir_follows_the_first_whitelist_root(self, artifact_env):
        """默认目录不是 CWD 相对路径，而是白名单首根下的 visualizations"""
        chart_dir = artifact_env.deps.default_visualization_dir()
        assert chart_dir == artifact_env.root / "visualizations"
        assert chart_dir.is_absolute()

    def test_pipeline_receives_the_root_relative_chart_dir(self, artifact_env):
        """端点必须显式传目录；只传一个入参就是 A407 的旧形状"""
        response = artifact_env.client.get("/api/visualize/a.json")
        assert response.status_code == 200, response.text

        assert artifact_env.calls == [
            (str(artifact_env.root / "a.json"), str(artifact_env.root / "visualizations"))
        ]

    def test_artifacts_land_under_the_root_and_leave_cwd_clean(self, artifact_env):
        """图真落在根下；CWD 下不许出现 visualizations 目录"""
        response = artifact_env.client.get("/api/visualize/a.json")
        assert response.status_code == 200, response.text

        assert (artifact_env.root / "visualizations" / "wordcloud.png").read_bytes() == b"png"
        assert not (artifact_env.cwd / "visualizations").exists()

    def test_receipt_stays_relative_and_never_exposes_the_root(self, artifact_env):
        """整份响应文本里搜不到服务器根目录，值仍是 visualizations/<名>"""
        response = artifact_env.client.get("/api/visualize/a.json")
        payload = response.json()

        assert payload["charts"]["wordcloud"] == str(
            Path("visualizations") / "wordcloud.png"
        )
        assert str(artifact_env.root) not in response.text
        assert str(artifact_env.cwd) not in response.text


class TestPublicChartPaths:
    """回执折回的两种形状与承重证明"""

    def test_in_dir_paths_keep_the_relative_shape(self, artifact_env):
        """闸内路径按「目录名 / 相对名」返回"""
        from api.routes.data import _public_chart_paths

        charts = {"a": str(artifact_env.root / "visualizations" / "a.png")}
        public = _public_chart_paths(charts, artifact_env.root / "visualizations")

        assert public == {"a": str(Path("visualizations") / "a.png")}

    def test_out_of_dir_paths_fall_back_to_the_bare_name(self, artifact_env):
        """目录外路径只回文件名，任何一支都不写绝对路径"""
        from api.routes.data import _public_chart_paths

        outside = artifact_env.cwd / "escaped.png"
        charts = {"a": str(outside)}
        public = _public_chart_paths(charts, artifact_env.root / "visualizations")

        assert public == {"a": "escaped.png"}
        assert str(artifact_env.root) not in json.dumps(public, ensure_ascii=False)

    def test_folding_is_load_bearing(self, artifact_env, monkeypatch):
        """把折回换成恒等函数，绝对落点立刻进回执 ⇒ 上面那支不是空转"""
        import api.routes.data as data_module

        monkeypatch.setattr(
            data_module,
            "_public_chart_paths",
            lambda charts, chart_dir: charts,
        )
        response = artifact_env.client.get("/api/visualize/a.json")

        # 直接读值，不读 `json.dumps`：JSON 会把 `\` 转义成 `\\`，子串比对会假阴。
        assert response.status_code == 200, response.text
        assert str(artifact_env.root) in response.json()["charts"]["wordcloud"]


class RecordingVisualizer:
    """记录每次出图时自己看到的 output_dir（`copy.copy` 与单例共享 seen 列表）"""

    def __init__(self, output_dir):
        """初始化

        Args:
            output_dir: 构造时的落盘目录
        """
        self.output_dir = Path(output_dir)
        self.seen = []

    def generate_statistics(self, items):
        """本组用例不经过统计入口"""
        raise AssertionError("visualize_dataset 不该调用统计入口")

    def generate_all_visualizations(self, items):
        """记录调用时的目录并返回空清单

        Args:
            items: 数据列表

        Returns:
            空字典
        """
        self.seen.append(self.output_dir)
        return {}


class TestSharedVisualizerIsNotRelocated:
    """A363：`visualize_dataset` 只改局部视图，不搬共享单例的落点"""

    def _bare_pipeline(self, tmp_path):
        """只装 `visualize_dataset` 需要的最小属性，避开真管道构造

        Args:
            tmp_path: pytest 临时目录

        Returns:
            `(pipeline, visualizer, seed_file)`
        """
        from augmentor.pipeline import AugmentorPipeline

        pipeline = object.__new__(AugmentorPipeline)
        pipeline.config = SimpleNamespace(
            visualization=SimpleNamespace(enabled=True)
        )
        visualizer = RecordingVisualizer(tmp_path / "singleton")
        pipeline.visualizer = visualizer
        seed = tmp_path / "seed.json"
        _seed_dataset(seed)
        return pipeline, visualizer, seed

    def test_singleton_output_dir_survives_two_calls(self, tmp_path):
        """两次不同目录的调用都只活在局部视图里"""
        pipeline, visualizer, seed = self._bare_pipeline(tmp_path)
        first = tmp_path / "first"
        second = tmp_path / "second"

        assert pipeline.visualize_dataset(str(seed), str(first)) == {}
        assert pipeline.visualize_dataset(str(seed), str(second)) == {}

        assert visualizer.output_dir == tmp_path / "singleton"
        assert visualizer.seen == [first, second]

    def test_the_local_view_is_what_gets_the_requested_dir(self, tmp_path):
        """调用时看到的目录就是本次入参（不是单例默认值）"""
        pipeline, visualizer, seed = self._bare_pipeline(tmp_path)
        requested = tmp_path / "requested"

        pipeline.visualize_dataset(str(seed), str(requested))

        assert visualizer.seen == [requested]
        assert visualizer.output_dir == tmp_path / "singleton"


class TestValidateConfigRedaction:
    """A398：回执保留行列号，不保留服务器绝对落点"""

    def test_parse_error_keeps_position_but_drops_absolute_path(self, artifact_env):
        """YAML 解析错误：line / column 在，根目录不在，文件内容不外泄"""
        (artifact_env.root / "bad.yaml").write_text(BAD_YAML, encoding="utf-8")
        response = artifact_env.client.post(
            "/api/system/validate-config", json={"path": "bad.yaml"}
        )

        assert response.status_code == 200, response.text
        payload = response.json()
        issue = payload["errors"][0]
        message = issue["message"]
        assert payload["is_valid"] is False
        assert message.startswith("YAML解析错误:")
        assert "line 1" in message and "column 9" in message
        assert "bad.yaml" in message
        assert issue["path"] == "yaml"
        assert str(artifact_env.root) not in message
        assert "sk-SECRET" not in response.text

    def test_default_config_path_uses_the_bare_filename(self, artifact_env, monkeypatch):
        """不传 path 时用文件名替代绝对落点，非 YAML 错误同样走脱敏"""
        cfg_dir = artifact_env.root / "cfg"
        cfg_dir.mkdir()
        missing = cfg_dir / "no-such-config.yaml"
        monkeypatch.setenv("AUGMENTOR_CONFIG_PATH", str(missing))

        response = artifact_env.client.post("/api/system/validate-config", json={})

        assert response.status_code == 200, response.text
        message = response.json()["errors"][0]["message"]
        assert "文件不存在" in message
        assert "no-such-config.yaml" in message
        assert str(cfg_dir) not in message

    def test_redaction_is_load_bearing(self, artifact_env, monkeypatch):
        """把脱敏换成恒等函数，绝对落点立刻回显 ⇒ 上面几支不是空转"""
        import api.routes.system_ops as system_ops

        monkeypatch.setattr(
            system_ops, "_redact_path", lambda payload, path, label: payload
        )
        (artifact_env.root / "bad.yaml").write_text(BAD_YAML, encoding="utf-8")

        response = artifact_env.client.post(
            "/api/system/validate-config", json={"path": "bad.yaml"}
        )

        message = response.json()["errors"][0]["message"]
        assert str(artifact_env.root) in message

    def test_the_endpoint_stays_off_the_write_tier(self, artifact_env):
        """只读面继续不挂 verify_api_key：静态读依赖 + 行为双证"""
        from api.deps import verify_api_key
        from api.routes.system_ops import router

        route = next(
            r
            for r in router.routes
            if getattr(r, "path", "") == "/api/system/validate-config"
        )
        assert route.methods == {"POST"}
        assert not any(
            dep.call is verify_api_key for dep in route.dependant.dependencies
        )

    def test_no_key_still_passes_when_api_key_is_configured(
        self, artifact_env, monkeypatch
    ):
        """设了 AUGMENTOR_API_KEY，该端点仍无需请求头"""
        (artifact_env.root / "ok.yaml").write_text(
            "models:\n  default: fallback\n", encoding="utf-8"
        )
        monkeypatch.setenv("AUGMENTOR_API_KEY", "secret-key")

        response = artifact_env.client.post(
            "/api/system/validate-config", json={"path": "ok.yaml"}
        )

        assert response.status_code == 200, response.text
        assert response.json()["is_valid"] is True
