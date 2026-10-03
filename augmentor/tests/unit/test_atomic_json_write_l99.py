# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L99 守卫：整份 JSON 落盘的**原子性**（读者永不看见半份）

**这一格为什么是真缺陷**（L99 实测，两条独立取证）：

1. 上传面（`Temp/l99q/fin_after_drain_l99.py`）：240 000 行的 xlsx 在服务端要处理 7.44 s，
   客户端中途放弃之后服务端照样写完，而轮询读者在 t=5.76 s 与 6.05 s 两次读到
   `JSONDecodeError`，磁盘上分别是 9 239 624 B 与 39 668 789 B（最终 45 840 001 B）。
   ⇒ `open(path,'w') + json.dump` 的**整个写窗口是一份坏数据集**。
2. 断点面（`augmentor/checkpoint.py` 的读侧）：`load_checkpoint()` 收尾是
   `except Exception: logger.error(...); return None`，所以断点文件被截断时它**不报错**，
   而是回「这个任务没有断点」。已完成的整段进度就此消失 —— 同一族缺陷，后果更重一档。

本文件的判据因此不是「函数返回了什么」，而是**写窗口里第三方读者看见什么**：要么旧内容、
要么新内容，不存在中间态；失败时旧内容必须原样还在，且目录里不留临时件。

A/B 对照是刻意的：`_legacy_write_json` 是改造前那八行的副本，本文件对它提出与对
`atomic_write_json` 同样的要求（写失败后目标仍可读）并**断言它做不到**。没有这一档，
「守卫通过」就完全可能只是「探针压根没制造出中间态」—— 那是本仓第四次撞上的
「合法形状的零」（L94 / L97 / L98 各一次）。
"""

import json
import os
import re
import threading
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import pytest
from fastapi import HTTPException

from api import deps
from augmentor import atomic_write as atomic_write_module
from augmentor import checkpoint as checkpoint_module
from augmentor.atomic_write import atomic_write_json
from augmentor.checkpoint import CheckpointManager

# 真实 `os.replace`：探针要在换之前观察，换的动作本身必须照常发生
REAL_REPLACE = os.replace

# 剩余「非原子整份 JSON 写边」的棘轮上限（A182 的账面数，L99 现量）。
# 口径：产品码（`augmentor/**` + `api/**`）里出现 `json.dump(` 且其上四行内有
# `open(..., 'w')` 的位置。本轮只把两处**读侧会把截断变成错误答案**的边换成原子写
# （`api/deps.py` 与 `augmentor/checkpoint.py`），其余按判据留在 A182：读侧要么报异常、
# 要么按未命中重算，半份窗口是可恢复代价而非坏答案。
# L160（B230）：DiskCache 元数据写边收原子写（读侧无 except 且构造必调，
# 半份窗口 = 第二实例构造当场崩）；其缓存值写边按判据口径豁免（get 按未命中
# 重算，条件豁免已机器化进 l160 守卫）。cache.py 的 2 处里现量只降这 1 处。
# L161（B231）：VersionControl 索引两边（默认索引写 + _save_index）收原子写
# （读侧 _load_index 无 except 且 __init__ 必调，同 L160 形状）；其 data/version
# 快照两边豁免（读侧 get_version_data 响亮抛异常，条件豁免机器化进 l161 守卫）。
# version_control.py 的 4 处计数边现量降 2。
# L162（B232）：BackupManager 索引两边收原子写（同 L160/L161 形状）；快照与
# 恢复输出两边豁免（读侧响亮抛异常），其「同名覆盖写中途崩毁既有」窗口是判据
# 口径未覆盖的另一档，记档待裁。backup.py 的 4 处计数边现量降 2。
# L163（B233）：导出物交付面收边——export.py 的四条 JSON 边收原子写（输出落
# web.data_roots 白名单目录，数据管理端点按 *.json 扫同一目录 = 写窗口内读者
# 拿半份「文件不是合法 JSON」错误答案）；JSONL 文本边同收（工具面新增
# atomic_write_text，临时件命名同口径避开两种扫描）。四条边写盘字节从
# indent=2 变紧凑序列化，读侧只解析不读格式，既有测试面无格式断言（实证）。
# L164（B234）：数据集变换三边（merge/sample/split）收原子写——API 与 CLI 两链
# 输出经白名单落数据根，同 L163 判据必收。dataset_ops.py 的 3 处计数边现量降 3。
# L165（B235）：依赖登记两边收原子写（读侧 _load_file 无 except 且 __init__ 必调，
# L160 同构第四份；截断静默回落 default 的另一路也封死——登记凭空消失是 checkpoint
# 症状）。dependency.py 的 2 处计数边现量降 2。ops/io/analysis/quality 等 CLI 报告
# 一次性写边按判据豁免（同步等待、任意路径、无扫描链），A182 逐面分档记档。
# L166（B236）：数据集工具路由的落盘口 _dump 收原子写（写盘变换类六端点共用
# helper，产物落白名单数据根 = L163 同判据）。data_ops 命令 --output 报告边同轮
# 分档豁免（CLI 报告一次性写）。api/routes 现量降 1。
# L167（B237）：增强导出的九格式共用落盘口 _export_json 收原子写（导出物交付
# 面同 L163）；ensure_ascii/indent 是活选项（ExportOptions 契约），atomic_write_json
# 参数化（默认紧凑与既有调用零变化）、选项原样透传。export_enhanced.py 现量降 1。
# 现量 28 含 atomic_write.py 工具本体 2 处（豁免排除后 26）；L167 净收
# export_enhanced 1 处（27 + 1 拆分 - 2 排除 = 26）。
# L168（B238）：管线交付口两边（process/异步变体的最终输出）与迁移输出边收原子
# 写（API 链白名单数据根 = L163 同判据必收）；tracker/quality_trend 的「历史记录 +
# 静默吞」复合形状记档待单独轮裁；benchmark/comparison/faiss 等读侧响亮档有据豁免。
# migration.py 现量降 1、pipeline.py 现量降 2。
# L169（B239）：实验记录与趋势历史两写边收原子写——这两处读侧契约本来就是静默
# 的（损坏→None / 不应崩溃 / L146 检疫），收原子不翻案：半份窗口消失后损坏只剩
# 磁盘故障一途，写失败仍被 except 吞（契约保持），检疫保留为纵深防御。
# tracker.py 与 quality_trend.py 现量各降 1。
NON_ATOMIC_JSON_DUMP_SITES_MAX = 21


def _legacy_write_json(file_path: Path, data):
    """改造前 `api/deps.py` 的整个函数体副本 —— 只用于 A/B 对照，产品码已不走它"""
    file_path.parent.mkdir(parents=True, exist_ok=True)
    with open(file_path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, separators=(",", ":"))


class _Boom:
    """JSON 不可序列化的值：让写在中途失败，而不必依赖线程时序"""


def _payload(n: int):
    return [{"input": f"第{i}条输入内容", "output": json.dumps({"k": i, "v": "值" * 8})} for i in range(n)]


def _replace_spy(sink):
    """把 `atomic_write` 模块里的 `os` 换成一个只观察不干预的 shim"""

    def spy_replace(src, dst):
        dst_path = Path(dst)
        sink.append(
            {
                "target": dst_path.read_bytes() if dst_path.exists() else None,
                "json_glob": sorted(p.name for p in dst_path.parent.glob("*.json")),
                "all_names": sorted(p.name for p in dst_path.parent.iterdir()),
                "tmp_size": Path(src).stat().st_size,
            }
        )
        return REAL_REPLACE(src, dst)

    return type("OSShim", (), {"replace": staticmethod(spy_replace)})()


class TestMidWriteWindow:
    """写窗口内的读者读数：核心判据，全部走 `os.replace` 前的观察点"""

    def test_reader_sees_old_bytes_until_the_swap_and_new_ones_after(self, tmp_path, monkeypatch):
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(50))
        old_bytes = target.read_bytes()
        new_bytes = json.dumps(
            _payload(80), ensure_ascii=False, separators=(",", ":")
        ).encode("utf-8")
        assert len(new_bytes) > len(old_bytes)

        seen = []
        monkeypatch.setattr(atomic_write_module, "os", _replace_spy(seen))
        atomic_write_json(target, _payload(80))

        # 反空转：观察点必须真的被经过一次
        assert len(seen) == 1, "替换前的观察点没执行 ⇒ 这一整组断言是零读数"
        snap = seen[0]
        assert snap["target"] == old_bytes, "替换之前目标已被改写 ⇒ 存在中间态"
        assert snap["tmp_size"] == len(new_bytes), "新内容此刻应完整躺在临时件上"
        assert target.read_bytes() == new_bytes, "替换之后读者要看到新内容"

    def test_temp_file_never_enters_the_dataset_listing(self, tmp_path, monkeypatch):
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(20))
        seen = []
        monkeypatch.setattr(atomic_write_module, "os", _replace_spy(seen))
        atomic_write_json(target, _payload(30))

        snap = seen[0]
        assert snap["json_glob"] == ["ds.json"], "半份临时件被 `*.json` 扫到了"
        leftovers = [n for n in snap["all_names"] if n != "ds.json"]
        assert len(leftovers) == 1 and leftovers[0].startswith(".ds.json.tmp-")

    def test_no_temp_file_survives_a_successful_write(self, tmp_path):
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(10))
        atomic_write_json(target, _payload(11))
        assert [p.name for p in tmp_path.iterdir()] == ["ds.json"]


class TestFailureLeavesTargetAlone:
    """写失败（崩溃的确定形状）之后：旧内容原样、无临时件、异常上抛"""

    def test_atomic_writer_survives_an_unserializable_payload(self, tmp_path):
        target = tmp_path / "ds.json"
        good = _payload(40)
        atomic_write_json(target, good)
        good_bytes = target.read_bytes()

        bad = [{"input": "a", "output": "b"}, {"input": _Boom()}]
        with pytest.raises(TypeError):
            atomic_write_json(target, bad)

        assert target.read_bytes() == good_bytes, "写失败后旧内容必须原样还在"
        assert [p.name for p in tmp_path.iterdir()] == ["ds.json"], "失败后不许留临时件"
        assert json.loads(target.read_text(encoding="utf-8")) == good

    def test_the_legacy_shape_really_left_a_broken_dataset(self, tmp_path):
        """A/B 对照组：同一份坏进料喂给改造前的写法，读者拿到的就是坏答案

        这一档是「守卫能失败」的证明：如果它对 legacy 也绿，说明探针没制造出中间态，
        上面那档的绿就毫无内容。

        实测顺带量出一条与直觉相反的形状：`json.dump` **不是一次性写完的**，它在序列化
        过程中就分块落盘，所以 legacy 失败后目标不必须是 0 字节 —— 它可以是半份**非空**
        内容，而那更糟：一份 `[{"input":"a"...` 形状的前缀会被 `*.json` 扫成一份数据文件。
        """
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(40))
        assert json.loads(target.read_text(encoding="utf-8"))

        bad = [{"input": "a", "output": "b"}, {"input": _Boom()}]
        with pytest.raises(TypeError):
            _legacy_write_json(target, bad)

        raw = target.read_bytes()
        assert raw, "legacy 若不再清空目标，本对照就失效了"
        assert not raw.rstrip().endswith(b"]"), "legacy 写出了完整数组 ⇒ 中间态没被制造出来"
        with pytest.raises(json.JSONDecodeError):
            json.loads(raw.decode("utf-8"))
        # 产品侧的症状：读端点把「好好的文件被写坏」报成「文件不是合法 JSON」
        with pytest.raises(HTTPException) as exc:
            deps.read_items(target)
        assert exc.value.status_code == 400


class TestByteAndNamingContract:
    def test_bytes_match_the_compact_dumps_exactly(self, tmp_path):
        target = tmp_path / "ds.json"
        data = _payload(25)
        atomic_write_json(target, data)
        assert target.read_bytes() == json.dumps(
            data, ensure_ascii=False, separators=(",", ":")
        ).encode("utf-8"), "序列化格式漂了（缩进/转义/分隔符任一）"
        assert "第1条输入内容" in target.read_text(encoding="utf-8")

    def test_creates_missing_parents(self, tmp_path):
        target = tmp_path / "a" / "b" / "c" / "ds.json"
        atomic_write_json(target, _payload(3))
        assert target.exists()

    def test_same_name_is_overwritten_by_the_last_writer(self, tmp_path):
        """钉住 L99 对 A167 的订正：同名二次上传是**后写赢**，不是「不覆盖」"""
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(5))
        atomic_write_json(target, _payload(7))
        assert len(json.loads(target.read_text(encoding="utf-8"))) == 7
        assert [p.name for p in tmp_path.iterdir()] == ["ds.json"]

    def test_dict_payloads_are_supported_too(self, tmp_path):
        """断点与索引写的是 dict，不是 list：本原语不能只认数据集形状"""
        target = tmp_path / "index.json"
        atomic_write_json(target, {"backups": [{"id": 1}], "n": 0})
        assert json.loads(target.read_text(encoding="utf-8")) == {"backups": [{"id": 1}], "n": 0}


class TestConcurrentWritersNeverInterleave:
    def test_final_file_is_exactly_one_of_the_competing_payloads(self, tmp_path):
        """8 支线程同一时刻写同一个名字 —— Windows 的退避重试就是这一档逼出来的

        第一版没有重试时，这里必有 `PermissionError: [WinError 5]`（同名替换被瞬时拒绝）。
        旧写法不报这个错，因为它让两支线程同时往同一个文件里交错写：一份坏文件而没人出声。
        """
        target = tmp_path / "ds.json"
        payloads = {n: _payload(n) for n in (10, 20, 30, 40, 50, 60, 70, 80)}
        start = threading.Barrier(len(payloads))

        def write(n):
            start.wait()
            atomic_write_json(target, payloads[n])

        with ThreadPoolExecutor(max_workers=len(payloads)) as pool:
            list(pool.map(write, payloads))

        got = json.loads(target.read_text(encoding="utf-8"))
        assert len(got) in payloads, "读者最终看到的是一份哪一次写都不是的内容"
        assert got == payloads[len(got)]
        assert [p.name for p in tmp_path.iterdir()] == ["ds.json"]

    def test_replace_retries_and_gives_up_loudly(self, tmp_path, monkeypatch):
        """退避不许变成吞异常：上限内始终被拒就原样上抛，且目标与目录都不脏"""
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(10))
        good = target.read_bytes()

        calls = []

        class _AlwaysDenied:
            @staticmethod
            def replace(src, dst):
                calls.append(1)
                raise PermissionError(5, "拒绝访问")

        monkeypatch.setattr(atomic_write_module, "os", _AlwaysDenied)
        with pytest.raises(PermissionError):
            atomic_write_json(target, _payload(20))

        assert len(calls) == atomic_write_module._SWAP_ATTEMPTS, "重试次数与上限不符"
        assert target.read_bytes() == good
        assert [p.name for p in tmp_path.iterdir()] == ["ds.json"]

    def test_a_transient_refusal_is_retried_and_the_write_still_lands(self, tmp_path, monkeypatch):
        """退避重试的**正路**：前两次被瞬时拒绝、第三次换上 ⇒ 新内容落盘且一次抛错都没有

        这一档是 L100 把「最后一次替换」移出循环后才变得可测的那条路径。旧写法里末次替换藏在
        `if attempt == 末次: raise` 那一支里、循环的自然出口结构上不可达 ⇒ 那里永远挂着
        一条测不到的偏支（A185 病理的同族）。断言盯三件事：确实退避了、退避用的就是那个常数、
        退避之后写成功且目录不留临时件。
        """
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(10))

        tried = []
        sleeps = []

        class _Flaky:
            @staticmethod
            def replace(src, dst):
                tried.append(1)
                if len(tried) <= 2:
                    raise PermissionError(5, "拒绝访问（瞬时）")
                return REAL_REPLACE(src, dst)

        monkeypatch.setattr(atomic_write_module, "os", _Flaky)
        monkeypatch.setattr(
            atomic_write_module, "time",
            type("TimeShim", (), {"sleep": staticmethod(lambda s: sleeps.append(s))})(),
        )
        atomic_write_json(target, _payload(20))

        assert len(tried) == 3, "没重试到第三次就成功 ⇒ 退避那一档是零读数"
        assert sleeps == [atomic_write_module._SWAP_BACKOFF_S] * 2
        assert json.loads(target.read_text(encoding="utf-8")) == _payload(20)
        assert [p.name for p in tmp_path.iterdir()] == ["ds.json"]



class TestThroatsAreWired:
    """两处咽喉必须真的走本原语（漂移守卫）"""

    def test_sync_write_json_delegates(self, tmp_path, monkeypatch):
        calls = []
        monkeypatch.setattr(
            deps, "atomic_write_json", lambda p, d: calls.append((p, d))
        )
        target = tmp_path / "ds.json"
        deps._sync_write_json(target, [{"a": 1}])
        assert calls == [(target, [{"a": 1}])]

    def test_async_write_json_file_goes_through_the_same_throat(self, tmp_path, monkeypatch):
        import asyncio

        calls = []
        monkeypatch.setattr(
            deps, "atomic_write_json", lambda p, d: calls.append((p, d))
        )
        target = tmp_path / "ds.json"
        asyncio.run(deps.write_json_file(target, [{"a": 2}]))
        assert calls == [(target, [{"a": 2}])]

    def test_no_truncating_json_writer_remains_in_the_two_throats(self):
        """`deps.py` 与 `checkpoint.py` 里不许再出现「open(w) 紧跟 json.dump」"""
        pattern = re.compile(r"open\([^)]*['\"]w['\"]", re.S)
        for module in (deps, checkpoint_module):
            text = Path(module.__file__).read_text(encoding="utf-8")
            for i, line in enumerate(text.splitlines(), start=1):
                if "json.dump(" in line and "json.dumps(" not in line:
                    window = "\n".join(text.splitlines()[max(0, i - 6): i])
                    assert not pattern.search(window), (
                        f"{module.__name__} 第 {i} 行附近又长出一处非原子写"
                    )

    def test_remaining_sites_are_ratcheted_and_named(self):
        """A182 的账面：本轮未修的非原子整份写边数量不许悄悄增长"""
        root = Path(deps.__file__).resolve().parent.parent
        sites = []
        for pkg in ("augmentor", "api"):
            for path in sorted((root / pkg).rglob("*.py")):
                if path.name == "atomic_write.py":
                    # L167 工具本体豁免：atomic_write_json / atomic_write_text
                    # 写的是同目录临时件（.tmp- 中段），不是产品写边——工具自身
                    # 的 json.dump 属实现细节（L167 参数化后单行拆两支，+1 是
                    # 形状噪声）。
                    continue
                lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
                for i, line in enumerate(lines, start=1):
                    if "json.dump(" not in line or "json.dumps(" in line:
                        continue
                    window = "\n".join(lines[max(0, i - 5): i])
                    if re.search(r"open\([^)]*['\"]w['\"]", window):
                        sites.append((path.relative_to(root).as_posix(), i))
        assert len(sites) <= NON_ATOMIC_JSON_DUMP_SITES_MAX, (
            f"非原子 JSON 写边从 {NON_ATOMIC_JSON_DUMP_SITES_MAX} 涨到 {len(sites)}：{sites[-5:]}"
        )


class TestCheckpointProgressSurvives:
    """断点面：写失败时进度必须还在，且增量不许被删"""

    def _manager(self, tmp_path):
        mgr = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=5)
        cp = mgr.create_checkpoint("t1", total_items=100)
        cp.completed_indices = [1, 2, 3]
        cp.processed_items = 3
        cp.quality_scores = {1: 0.9}
        return mgr, cp

    def test_successful_save_returns_true_and_round_trips(self, tmp_path):
        mgr, cp = self._manager(tmp_path)
        assert mgr._save_checkpoint(cp) is True
        back = mgr.load_checkpoint("t1")
        assert back is not None and back.completed_indices == [1, 2, 3]

    def test_failed_save_keeps_the_previous_file_readable(self, tmp_path, monkeypatch):
        mgr, cp = self._manager(tmp_path)
        assert mgr._save_checkpoint(cp) is True

        def boom(_cp):
            return {"task_id": "t1", "total_items": 100, "processed_items": 3,
                    "failed_items": 0, "completed_indices": [1, 2, 3],
                    "failed_indices": [], "start_time": 0.0,
                    "last_update_time": 0.0, "quality_scores": {1: _Boom()}}

        monkeypatch.setattr(checkpoint_module, "asdict", boom)
        assert mgr._save_checkpoint(cp) is False
        back = mgr.load_checkpoint("t1")
        assert back is not None, "写失败后旧断点读不回来 ⇒ 进度消失"
        assert back.completed_indices == [1, 2, 3]

    def test_deltas_survive_a_failed_merge_and_go_when_it_succeeds(self, tmp_path, monkeypatch):
        """`save_checkpoint()` 的删除步必须看写盘判决 —— 否则「写失败 + 删增量」＝进度归零"""
        mgr, _cp = self._manager(tmp_path)
        delta_path = mgr._get_delta_path("t1")

        mgr.update_progress(7, True)
        mgr.update_progress(8, True)
        mgr.save_checkpoint()
        assert not delta_path.exists(), "成功合并后增量该删掉（否则下次恢复重复累加）"
        assert mgr.load_checkpoint("t1").completed_indices == [1, 2, 3, 7, 8]

        mgr.update_progress(9, True)
        monkeypatch.setattr(
            checkpoint_module,
            "atomic_write_json",
            lambda *a, **k: (_ for _ in ()).throw(OSError("盘满")),
        )
        mgr.save_checkpoint()
        assert delta_path.exists(), "主文件写失败却删了增量 ⇒ 那一批永久丢失"

    def test_legacy_shape_would_have_lost_the_progress(self, tmp_path, monkeypatch):
        """A/B 对照：旧写法把坏 JSON 留在盘上时，读侧的 `return None` 就是「进度消失」

        这里不复现写窗口（那是 `TestFailureLeavesTargetAlone` 的对照），复现的是它的
        **下游后果**：一份截断的断点文件经 `load_checkpoint()` 得到 None 而不是异常。
        """
        mgr, _cp = self._manager(tmp_path)
        path = mgr._get_checkpoint_path("t1")
        text = path.read_text(encoding="utf-8")
        path.write_text(text[: len(text) // 2], encoding="utf-8")  # 截断，像半份写
        assert mgr.load_checkpoint("t1") is None, "读侧若开始报错，本档的因果就要重写"
        assert mgr._get_delta_path("t1").exists() is False


class TestSelfProof:
    """探针必须能自证它真的观察到了中间态，而不是绿在一个空集合上"""

    def test_spy_hook_actually_observed_a_nonempty_partial_temp(self, tmp_path, monkeypatch):
        target = tmp_path / "ds.json"
        atomic_write_json(target, _payload(10))
        seen = []
        monkeypatch.setattr(atomic_write_module, "os", _replace_spy(seen))
        atomic_write_json(target, _payload(60))
        assert seen and seen[0]["tmp_size"] > 1000, "临时件没有内容 ⇒ 观察点是空操作"
        assert seen[0]["target"] is not None, "目标文件在观察点不存在 ⇒ 换的不是这一档"
