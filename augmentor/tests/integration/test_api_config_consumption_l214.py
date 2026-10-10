# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0

"""L214（A138 ②③ 档接线·第一批）：配置键必须是**行为差**，不是回显

这一族的旧形状是「写进配置对象、GET /api/config 里看得见、但没有任何产品代码读它」——
② 档 6 键只回显，③ 档 5 键连回显都没有。回显面钉死（L184）只证明「键在响应里」，
不证明「键有用」。本轮把六只键接到既有端点上，判据一律取**行为差**：改配置 ⇒ 端点
的产物/回应跟着变；配置与请求参数同时给 ⇒ 请求参数赢。

权威序（本轮定稿，与 quality/dedup 的管道阶段同一口径）：
**请求参数 > 配置键 > 代码内置默认**。

覆盖面：
- `export.formats` → `/api/data/export` 与 `/api/export/batch` 的「省略 formats」支路
  （原先无条件导全部 13 族，配置的五族清单在 API 面零读者）
- `export.default_format` → `/api/export/preview` 的省略 `format` 支路
- `rag.default_format` → `/api/dataset/rag` 的省略 `format` 支路
- `multimodal.image_extensions` / `multimodal.audio_extensions` → `/api/multimodal/formats`
  与 `/api/multimodal/scan`（内置默认认 `.gif`，配置默认不认 ⇒ 「配置改变识别面」
  这一格是可证的，也是本轮最强的消费证据）

**`enabled` 三只键不在本文件**：它们是管道阶段的开关，住在 `AugmentorPipeline`
（与 quality/dedup 同族语义），拿端点拒绝一次调用方点名要的独立能力属于改契约而非
消费配置；`enabled` 与 vector 四键的接线见 L215。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """真实 AugmentorPipeline，读写面全部指向临时目录"""
    from augmentor import AugmentorPipeline, load_config

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.augmentation.output_dir = str(tmp_path / "out")
    config.augmentation.checkpoint_dir = str(tmp_path / "ckpt")
    instance = AugmentorPipeline(config)
    import api.deps as deps
    monkeypatch.setattr(deps, "_pipeline", instance)
    monkeypatch.chdir(tmp_path)
    yield instance


@pytest.fixture
def client():
    from fastapi.testclient import TestClient

    from api.main import app
    return TestClient(app)


@pytest.fixture
def seed(pipeline, tmp_path):
    """落在 CWD（= tmp_path）里的两条样本"""
    name = "seed_l214.json"
    Path(name).write_text(
        json.dumps(
            [
                {"instruction": "租房押金多少", "input": "", "output": "一个月"},
                {"instruction": "合同期多长", "input": "", "output": "一年"},
            ],
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    return name


class TestExportFormatsIsConsumed:
    def test_omitted_formats_follows_config(self, client, pipeline, seed, tmp_path):
        """配置收窄到 jsonl 一族后，省略 formats 的导出只产出一族

        接不上线的旧形状：这一脚永远走 `export_all_formats`，13 族全导，
        `config.export.formats` 无论写什么都没人读。
        """
        pipeline.config.export.formats = ["jsonl"]
        out = str(tmp_path / "exp_cfg")
        resp = client.post(
            "/api/data/export", json={"input_file": seed, "output_dir": out}
        )
        assert resp.status_code == 200, resp.text
        assert set(resp.json()["files"]) == {"jsonl"}

    def test_request_formats_beat_config(self, client, pipeline, seed, tmp_path):
        """请求参数点名 csv 时配置的五族让位——两级序的另一半"""
        pipeline.config.export.formats = ["jsonl", "alpaca"]
        out = str(tmp_path / "exp_req")
        resp = client.post(
            "/api/data/export",
            json={"input_file": seed, "output_dir": out, "formats": ["csv"]},
        )
        assert resp.status_code == 200, resp.text
        assert set(resp.json()["files"]) == {"csv"}

    def test_empty_config_falls_back_to_all_formats(self, client, pipeline, seed, tmp_path):
        """配置写空列表按「没配」处理，回落既有的全格式支路（空 ≠ 一条都不导）"""
        pipeline.config.export.formats = []
        out = str(tmp_path / "exp_empty")
        resp = client.post(
            "/api/data/export", json={"input_file": seed, "output_dir": out}
        )
        assert resp.status_code == 200, resp.text
        assert len(resp.json()["files"]) > 1

    def test_batch_export_follows_config(self, client, pipeline, seed, tmp_path):
        pipeline.config.export.formats = ["jsonl", "sharegpt"]
        out = str(tmp_path / "batch_cfg")
        resp = client.post(
            "/api/export/batch",
            json={"datasets": {"ds1": seed}, "output_dir": out},
        )
        assert resp.status_code == 200, resp.text
        assert set(resp.json()["results"]["ds1"]) == {"jsonl", "sharegpt"}


class TestPreviewFormatFollowsConfig:
    def test_omitted_preview_format_reads_default_format(self, client, pipeline, seed):
        """预览的默认格式跟着配置走（旧形状：路由写死 "jsonl"，配置里改天改地也无妨）"""
        pipeline.config.export.default_format = "alpaca"
        resp = client.post("/api/export/preview", json={"input_file": seed, "size": 1})
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "alpaca"

    def test_explicit_preview_format_still_wins(self, client, pipeline, seed):
        pipeline.config.export.default_format = "alpaca"
        resp = client.post(
            "/api/export/preview", json={"input_file": seed, "format": "jsonl", "size": 1}
        )
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "jsonl"


class TestRagDefaultFormatIsConsumed:
    def test_omitted_rag_format_reads_config(self, client, pipeline, seed, tmp_path):
        """省略 format ⇒ config.rag.default_format（旧形状：路由写死 langchain）"""
        pipeline.config.rag.default_format = "custom"
        out = "rag_out.json"
        resp = client.post(
            "/api/dataset/rag", json={"input_file": seed, "output_file": out}
        )
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "custom"
        records = json.loads(Path(out).read_text(encoding="utf-8"))
        # custom 的产物是 {id, query, answer, context}，与 llamaindex 的 text 形状不同
        # ⇒ 读的是「配置真的决定了转换器」，不是「响应里 echo 了这个字符串」
        assert records and {"id", "query", "answer"} <= set(records[0])

    def test_explicit_rag_format_beats_config(self, client, pipeline, seed, tmp_path):
        pipeline.config.rag.default_format = "custom"
        resp = client.post(
            "/api/dataset/rag",
            json={
                "input_file": seed,
                "output_file": "rag_req.json",
                "format": "langchain",
            },
        )
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "langchain"


class TestMultimodalExtensionsAreConsumed:
    def test_formats_endpoint_reads_config(self, client, pipeline):
        """配置里的扩展名直接决定能力清单（旧形状：清单永远来自类的内置默认）"""
        pipeline.config.multimodal.image_extensions = [".gif"]
        pipeline.config.multimodal.audio_extensions = [".m4a"]
        body = client.get("/api/multimodal/formats").json()
        assert body["image_extensions"] == [".gif"]
        assert body["audio_extensions"] == [".m4a"]

    def test_extensions_are_normalized(self, client, pipeline):
        """大写、缺点号的写法归一成 `Path.suffix` 口径（A138 把这条界留给消费方）"""
        pipeline.config.multimodal.image_extensions = ["GIF", "Png"]
        body = client.get("/api/multimodal/formats").json()
        assert body["image_extensions"] == [".gif", ".png"]

    def test_config_widens_recognition_beyond_builtin_default(self, client, pipeline, tmp_path):
        """配置认 .gif 而内置默认不认 ⇒ 扫描真的多收一条，这是「配置有用」的强证据"""
        media = tmp_path / "media_cfg"
        media.mkdir()
        # 最小 GIF 头：GIF89a + 2x2 逻辑屏幕描述符
        (media / "clip.gif").write_bytes(
            b"GIF89a\x02\x00\x02\x00\x00\x00\x00\x00,"
        )
        pipeline.config.multimodal.image_extensions = [".gif"]
        resp = client.post("/api/multimodal/scan", json={"directory": str(media)})
        assert resp.status_code == 200, resp.text
        assert resp.json()["total_records"] == 1

    def test_default_config_does_not_pick_up_gif(self, client, pipeline, tmp_path):
        """反向对照：同一份文件在默认配置（不认 .gif）下不收——两臂互证"""
        media = tmp_path / "media_default"
        media.mkdir()
        (media / "clip.gif").write_bytes(b"GIF89a\x02\x00\x02\x00\x00\x00\x00\x00,")
        resp = client.post("/api/multimodal/scan", json={"directory": str(media)})
        assert resp.status_code == 200, resp.text
        assert resp.json()["total_records"] == 0

    def test_request_extensions_beat_config(self, client, pipeline, tmp_path):
        """请求点名的扩展面覆盖配置面（两级序在扫描端点同样成立）"""
        media = tmp_path / "media_req"
        media.mkdir()
        (media / "clip.gif").write_bytes(b"GIF89a\x02\x00\x02\x00\x00\x00\x00\x00,")
        (media / "shot.jpg").write_bytes(b"fake")
        pipeline.config.multimodal.image_extensions = [".jpg"]
        resp = client.post(
            "/api/multimodal/scan",
            json={"directory": str(media), "image_extensions": [".gif"]},
        )
        assert resp.status_code == 200, resp.text
        stems = {r["text"] for r in resp.json()["records"]}
        assert stems == {"clip"}, f"请求参数应当盖掉配置，实际收到 {stems}"
