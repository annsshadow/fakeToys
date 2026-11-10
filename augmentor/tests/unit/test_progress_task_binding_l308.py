# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A408 守卫：进度回执要能绑到「问的那一次运行」（L308）

**这一轮收的是哪一格**：A408（L294 立）—— `GET /api/augment/progress` 调
`p.checkpoint_manager.get_progress()` 且**不传**断点对象 ⇒ 走 `None` 分支回落到
`self._current_checkpoint` ⇒ 回执里那格 `task_id` 可能是**最后加载断点的那一次
运行**，而不是调用方问的那一次。任务身份的权威在
`AugmentorPipeline.build_task_id`（按**解析后**的输入路径取 md5 前 12 位）。

**这一轮收口的形状**（两层）：

* **读侧**：HTTP 面接受 `?task_id=`，读**那一份**断点的进度；且这条读法是
  **只读**的 —— 不消费增量（不把正在跑的增量吃掉）、不动环境态（不把别的调用方
  的指针搬走）。为此 `CheckpointManager` 新增 `peek_checkpoint`（纯读），
  `load_checkpoint` 拆成「纯读合并」+「消费」两层（消费那一半的行为一字未动）。
* **写侧**：`POST /api/augment/start` 的回执**带上 `task_id`** —— 客户端算不出
  那个 id（它只知道自己送的文件名，不知道服务端解析后的路径），要问「我这次
  任务的进度」就得由服务端把 id 递回来。

**这一把尺为什么这样摆**：

1. **点名读法必须是承重的**：盘上放两份断点、环境态指着其中一份，点名读**另一份**
   ⇒ 回执里的 `task_id` 必须是点名的那一份。少了这一支，「点名」只是把环境态
   读法换了个名字。
2. **只读性要单独钉**：`peek_checkpoint` 之后增量文件**仍在盘上**、环境态**没被
   搬走** —— 这两件事都是「看一眼进度」绝不该做的。
3. **消费那一半不能被顺手改掉**：`load_checkpoint` 之后增量文件**必须消失**
   （否则断点续传会重复累加），这一支钉住「读侧拆分没有把消费读改成只读读」。
4. **404 与空态要分得开**：问一个不存在的任务 ⇒ 404；「当下没有任务在跑」⇒
   仍是既有的 `{"status": "no_checkpoint"}` —— 两件事不该用同一句话回答。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走
类内独立用例，两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_progress_task_binding_l308.py -q --no-cov`
"""
import json
from pathlib import Path

import pytest

from augmentor.checkpoint import CheckpointManager


def _manager(tmp_path, name="ckpts", interval=1):
    """造一个断点管理器（增量每 1 条落一次盘）

    Args:
        tmp_path: pytest 临时目录
        name: 断点目录名
        interval: 增量落盘间隔

    Returns:
        CheckpointManager 实例
    """
    return CheckpointManager(checkpoint_dir=str(tmp_path / name),
                             auto_save_interval=interval)


def _delta_path(mgr, task_id):
    """增量文件路径

    Args:
        mgr: 断点管理器
        task_id: 任务 ID

    Returns:
        Path
    """
    return Path(mgr.checkpoint_dir) / f"{task_id}_delta.jsonl"


class TestThePeekIsReadOnly:
    """`peek_checkpoint` 只读：不消费增量、不动环境态"""

    def test_peek_merges_the_delta_without_consuming_it(self, tmp_path):
        """只读面看得到增量里的进度，而增量文件**仍在盘上**"""
        mgr = _manager(tmp_path)
        ck = mgr.create_checkpoint("taskA", 10)
        mgr.update_progress(0, True, checkpoint=ck)

        peeked = mgr.peek_checkpoint("taskA")

        assert peeked is not None
        assert peeked.processed_items == 1, "只读面没看到增量里的进度"
        assert _delta_path(mgr, "taskA").exists(), \
            "只读面把增量消费了 ⇒ 正在跑的增量被吃掉"

    def test_peek_does_not_move_the_environment_state(self, tmp_path):
        """只读面不搬环境态指针（`get_progress()` 仍读原来那一份）"""
        mgr = _manager(tmp_path)
        ck_a = mgr.create_checkpoint("taskA", 10)
        mgr.load_checkpoint("taskA")  # 把环境态指到 taskA

        mgr.peek_checkpoint("taskA")

        assert mgr.get_progress()["task_id"] == "taskA"

    def test_load_still_consumes_the_delta(self, tmp_path):
        """消费那一半没被顺手改掉：`load_checkpoint` 之后增量文件必须消失

        （否则断点续传会把同一批增量再累加一遍 —— 那正是 L100 量到的那一形。）

        """
        mgr = _manager(tmp_path)
        ck = mgr.create_checkpoint("taskA", 10)
        mgr.update_progress(0, True, checkpoint=ck)
        assert _delta_path(mgr, "taskA").exists(), "前置条件不成立：增量没落盘"

        mgr.load_checkpoint("taskA")

        assert not _delta_path(mgr, "taskA").exists(), \
            "消费型读没有清增量 ⇒ 续传会重复累加"


class TestTheNamedReadIsLoadBearing:
    """HTTP 面：点名读法必须真的读那一份"""

    @pytest.fixture
    def api_env(self, tmp_path, monkeypatch):
        """钉住管道实例与断点目录，并把增强任务切成空实现

        Args:
            tmp_path: pytest 临时目录
            monkeypatch: pytest fixture

        Returns:
            SimpleNamespace，含 client 与 manager
        """
        from types import SimpleNamespace

        from fastapi.testclient import TestClient

        import api.deps as deps
        from api.main import app
        from augmentor import AugmentorPipeline, load_config

        ai_dir = Path(__file__).resolve().parents[2]
        config = load_config(str(ai_dir / "config.yaml"))
        pipeline = AugmentorPipeline(config)
        manager = _manager(tmp_path, "http_ckpts")
        pipeline.checkpoint_manager = manager
        pipeline.augment_dataset = lambda *args, **kwargs: {}
        monkeypatch.setattr(deps, "_pipeline", pipeline)
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        return SimpleNamespace(client=TestClient(app), manager=manager)

    def test_the_named_task_read_reports_that_task(self, api_env):
        """环境态指着 B，点名读 A ⇒ 回执里的 `task_id` 必须是 A"""
        manager = api_env.manager
        ck_a = manager.create_checkpoint("taskA", 10)
        ck_b = manager.create_checkpoint("taskB", 10)
        manager.load_checkpoint("taskB")  # 环境态指到 B

        response = api_env.client.get("/api/augment/progress",
                                      params={"task_id": "taskA"})

        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["task_id"] == "taskA", \
            "点名读法读回来的不是点名的那一份 ⇒ 这一格没修"

    def test_the_unnamed_read_is_unchanged(self, api_env):
        """不带 `task_id` ⇒ 仍是环境态读法（既有行为一格未动）"""
        manager = api_env.manager
        manager.create_checkpoint("taskB", 10)
        manager.load_checkpoint("taskB")

        response = api_env.client.get("/api/augment/progress")

        assert response.status_code == 200, response.text
        assert response.json()["task_id"] == "taskB"

    def test_an_unknown_task_is_404(self, api_env):
        """问一个不存在的任务 ⇒ 404（与「当下没有任务在跑」分得开）"""
        response = api_env.client.get("/api/augment/progress",
                                      params={"task_id": "nope"})

        assert response.status_code == 404, response.text

    def test_start_hands_back_the_task_id(self, api_env, tmp_path):
        """`POST /api/augment/start` 的回执带上**这次运行**的任务身份"""
        src = tmp_path / "in.json"
        src.write_text(json.dumps([{"instruction": "x", "input": "", "output": "o"}]),
                       encoding="utf-8")
        import api.deps as deps
        expected = deps.get_pipeline().build_task_id(str(src))

        response = api_env.client.post(
            "/api/augment/start",
            json={"input_file": str(src), "output_file": str(tmp_path / "out.json")},
        )

        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["task_id"] == expected, \
            "回执里的任务身份与 build_task_id 算出来的不一致"
        assert payload["success"] is True
