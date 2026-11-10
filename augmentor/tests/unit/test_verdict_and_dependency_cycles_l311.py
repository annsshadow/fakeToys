# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L311 守卫：依赖图要报环（A310）+ 版本 id 不存在要 404（A360）+ 导出并行支路不许吞判决（A340）

**这一轮收的是哪一格**（三格同族：都是「客户端拿到的东西说不清事实」）：

1. **A310 —— 依赖图不查环**：`validate_dependencies()` 只发 `missing_source` /
   `missing_target` 两种 issue，成环时 `issues` 里零报告 ⇒ 页面上「校验问题 0 就绿」
   那一格会被读成「依赖关系没问题」，而一个环会让下游任何按依赖顺序重建的流程死循环。
   本轮加 DFS 染色的环检测，`issues` 多一档 `type: "cycle"`（响应模型的三键不变）。
2. **A360 —— 版本 id 不存在回 500**：同一份路由文件里三种归宿并存（源文件缺失 ⇒ 404、
   版本 id 缺失 ⇒ 500、删不存在的版本 ⇒ 200 `success:true`）。本轮把「版本不存在」
   那一类收窄成 **404**（`raise_boundary_error` 新增一只与 `to_http_error` 同形的
   `not_found_types` 窄口 ⇒ A77「判据住一处」），而**其余域错误仍落 500** ——
   这是**收窄**不是放宽（改前 `VersionError` 因为不是 `DataValidationError` 而被
   一刀切折成 500）。
3. **A340 —— 导出的并行支路把判决咽成「失败冒充成功」**：写边（`require_within_dir`）
   拦住了，回执却看不见那次拦截 ⇒ 调用方拿到 `{}` 而 HTTP 面照样回 `success: true`。
   本轮并行支路整批 raise，路由随之回 400（写边抛的是 `DataValidationError`）。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98`
把全仓 parametrize 三面冻成现量数，本守卫走具名用例，那两把尺子一格未动。

跑法：`python -m pytest tests/unit/test_verdict_and_dependency_cycles_l311.py -q --no-cov`
"""
import json
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

from augmentor.dependency import DependencyManager
from augmentor.exceptions import DataValidationError
from augmentor.export import Exporter

ITEMS = [{"instruction": "写一首诗。", "output": "落叶知秋。"}]


# ---------------------------------------------------------------- A310 那一族


class TestTheDependencyGraphReportsCycles:
    """A310：成环要报，且只报一次"""

    @pytest.fixture()
    def manager(self, tmp_path):
        m = DependencyManager(str(tmp_path / "registry.json"))
        m.register_dataset("dsA", "a.json", 1)
        m.register_dataset("dsB", "b.json", 1)
        return m

    def _cycle(self, manager):
        manager.add_dependency("dsA", "dsB", "derived")
        manager.add_dependency("dsB", "dsA", "merged")
        return manager.validate_dependencies()

    def test_a_cycle_is_reported(self, manager):
        """成环 ⇒ `issues` 里有一档 `type: "cycle"`"""
        issues = self._cycle(manager)
        kinds = [i["type"] for i in issues]
        assert "cycle" in kinds, issues

    def test_the_cycle_issue_names_the_path(self, manager):
        """判决点名环上的数据集（不是一句「有环」）"""
        issues = self._cycle(manager)
        cycle = [i for i in issues if i["type"] == "cycle"][0]
        assert "dsA" in cycle["message"] and "dsB" in cycle["message"], cycle
        assert "→" in cycle["message"], cycle

    def test_the_cycle_issue_has_the_same_three_keys(self, manager):
        """issue 的三键形状不变（`type` / `dependency` / `message`）——前端那支守卫按形状对账"""
        issues = self._cycle(manager)
        for issue in issues:
            assert sorted(issue) == ["dependency", "message", "type"], issue

    def test_a_dag_has_no_cycle_issue(self, manager):
        """不成环 ⇒ 没有 `cycle` 那一档（别把「有边」当成「有环」）"""
        manager.add_dependency("dsA", "dsB", "derived")
        kinds = [i["type"] for i in manager.validate_dependencies()]
        assert "cycle" not in kinds, kinds

    def test_one_cycle_is_reported_once(self, manager):
        """同一个环只报一条（DFS 染色会撞到两次同一条回边）"""
        issues = self._cycle(manager)
        cycles = [i for i in issues if i["type"] == "cycle"]
        assert len(cycles) == 1, cycles

    def test_the_dangling_arms_are_untouched(self, manager):
        """悬空端点那两档一字未动（改前就在册）"""
        manager.add_dependency("dsA", "ghost", "derived")
        kinds = [i["type"] for i in manager.validate_dependencies()]
        assert "missing_target" in kinds, kinds


# ---------------------------------------------------------------- A360 那一族


class TestTheVersionRoutesAnswer404:
    """A360：不存在的版本 id ⇒ 404；其余域错误仍落 500"""

    @pytest.fixture()
    def client(self, tmp_path, monkeypatch):
        from augmentor import AugmentorPipeline, load_config
        from augmentor.checkpoint import CheckpointManager
        from augmentor.versioning import VersionManager

        import api.deps as deps
        from api.main import app

        monkeypatch.chdir(tmp_path)
        config = load_config(str(Path(__file__).resolve().parents[2] / "config.yaml"))
        config.versioning.storage_dir = str(tmp_path / "versions")
        instance = AugmentorPipeline(config)
        instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
        instance.checkpoint_manager = CheckpointManager(
            checkpoint_dir=str(tmp_path / "checkpoints"))
        monkeypatch.setattr(deps, "_pipeline", instance)
        return TestClient(app)

    def test_the_detail_route_answers_404(self, client):
        resp = client.get("/api/versions/v_ghost")
        assert resp.status_code == 404, resp.text
        assert resp.json()["detail"] == "版本不存在"

    def test_the_data_route_answers_404(self, client):
        resp = client.get("/api/versions/v_ghost/data")
        assert resp.status_code == 404, resp.text

    def test_the_diff_route_answers_404(self, client):
        resp = client.post("/api/versions/diff",
                            json={"version1": "nope-1", "version2": "nope-2"})
        assert resp.status_code == 404, resp.text

    def test_the_verdict_does_not_echo_the_given_id(self, client):
        """固定文案 ⇒ 不回显调用方给的名字（l274 那一族的要求）"""
        resp = client.post("/api/versions/diff",
                           json={"version1": "nope-1", "version2": "nope-2"})
        assert "nope-1" not in resp.text, resp.text
        assert "nope-2" not in resp.text, resp.text

    def test_the_delete_route_keeps_its_silent_success(self, client):
        """`delete_version` 的 `VersionError` 语义是「不能删当前版本」而不是「不存在」
        ⇒ 那一档**不进**404 窄口（改前也是 200 `success:true`）"""
        resp = client.delete("/api/versions/v_ghost")
        assert resp.status_code == 200, resp.text
        assert resp.json()["success"] is True


# ---------------------------------------------------------------- A340 那一族


class TestTheExportArmStopsSwallowing:
    """A340：并行支路不再把判决咽成一片空 dict"""

    def test_the_parallel_arm_raises_the_verdict(self, tmp_path):
        """改前：并行档回执是 `{"../../silent/x": {}}`（假成功）；改后整批 raise"""
        out = tmp_path / "parout"
        out.mkdir()
        with pytest.raises(DataValidationError):
            Exporter().export_batch({"../../silent/x": ITEMS}, str(out), ["jsonl"])
        assert not (tmp_path.parent / "silent").exists()

    def test_the_serial_arm_still_raises(self, tmp_path):
        """两档现在同码（改前两档判决不同）"""
        out = tmp_path / "serout"
        out.mkdir()
        with pytest.raises(DataValidationError):
            Exporter().export_batch({"../../silent/x": ITEMS}, str(out), ["jsonl"],
                                    use_parallel=False)

    def test_a_legal_batch_still_succeeds(self, tmp_path):
        """合法输入零行为差：回执形状不变，产物落盘"""
        out = tmp_path / "okout"
        out.mkdir()
        results = Exporter().export_batch({"train": ITEMS}, str(out), ["jsonl"])
        assert list(results) == ["train"]
        assert "jsonl" in results["train"], results
        produced = results["train"]["jsonl"]
        with open(produced, encoding="utf-8") as fh:
            # jsonl = 每条一行（不是一个 JSON 数组）；导出面把记录补成三键形状
            # （`input` 缺省补空串），那是导出格式自己的契约，本轮不动
            row = json.loads(fh.readline())
            assert row["instruction"] == ITEMS[0]["instruction"], row
            assert row["output"] == ITEMS[0]["output"], row


class TestTheExportRouteAnswers400:
    """A340 的 HTTP 面那一格：失败不再是 `success: true`"""

    @pytest.fixture()
    def client(self, tmp_path, monkeypatch):
        import api.deps as deps
        from api.main import app

        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        monkeypatch.setattr(deps, "_pipeline", None)
        return TestClient(app, raise_server_exceptions=False)

    def test_the_batch_route_answers_400_not_a_false_success(self, client, tmp_path):
        """名字被当成路径片段用 ⇒ 400（写边的判决），不再是 200 + 空 results"""
        data = tmp_path / "d.json"
        data.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        resp = client.post("/api/export/batch", json={
            "datasets": {"../../evil/x": "d.json"},
            "output_dir": "out",
            "formats": ["jsonl"],
        })
        assert resp.status_code == 400, resp.text
