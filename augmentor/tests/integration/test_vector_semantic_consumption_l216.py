# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L216：vector 五键的消费证据——`augmentor/vector/` 从能力孤岛变成有产品的路径

**本轮补的是哪一格**：A138 的 ③ 档三只键（`vector.dimension` / `storage_dir` /
`collection`）连回显都没有，根因不是「忘了回显」而是**整个向量子系统没有产品读者**：
`create_vector_db` 的调用者只有测试。所以本轮接的是消费者（`augmentor/semantic.py` +
两条端点 + 管道建库阶段），五只键一起落地。

**判据一律取行为差**（L214 立的规矩）：
- `dimension` ⇒ 嵌入宽度与相似度分值跟着变（小维度哈希碰撞会改排名，实测两臂分值不同）；
- `backend` ⇒ 值真的流到工厂分派（spy 收 kwargs，喂的是非默认值才可区分），未知值在工厂
  那道门抛 `VectorError`，没有 `persist`/`load` 面的后端在构造前被闸拒；
- `collection` ⇒ 持久化文件名 `{collection}_vectors.npy` 与向量 ID 前缀跟着变；
- `storage_dir` ⇒ 文件落点跟着变，另一侧目录保持空；
- `enabled` ⇒ 增强结束时建不建库、报告里 `vector_index` 是 `null` 还是回执。
  出厂 `false` ⇒ 默认完全不建，本轮之前之后一字不差（零漂移那一档）。

**`backend` 的证据边界画在哪里**：本仓 CI 与本机都没装 `chromadb` 包，`ChromaDB` 构造即抛
`ImportError`，于是「换成 chromadb 会出另一套检索产物」在这一台机器上没有可现量的对象——
写一条假的「两后端结果不同」断言就是本仓一直在收的不可证伪形状。能量到的换成三条：配置值进
工厂（spy，且喂进去的值与默认值**可区分**）、未知值被工厂拒（异常）、没有 `persist`/`load`
面的后端在构造**之前**被能力闸拒（`TestPersistenceSurfaceIsGated`）。第三条是本轮读
`augmentor/vector/chromadb.py` 才浮出来的：那侧压根没有显式持久化面，而它的构造会写盘。
"""

import json
from pathlib import Path

import pytest

from augmentor import AugmentorPipeline, load_config
from augmentor.checkpoint import CheckpointManager
from augmentor.config import ModelConfig
from augmentor.exceptions import VectorError
from augmentor.models.base import ModelBackend
from augmentor.semantic import (
    build_index,
    embed_texts,
    index_status,
    persist_index,
    search_similar,
)
from augmentor.vector import FAISSDB

AI_DIR = Path(__file__).resolve().parent.parent.parent

ITEMS = [
    {"instruction": "猫喜欢吃什么食物", "output": "猫喜欢吃鱼肉和猫粮"},
    {"instruction": "狗怎么训练定点排便", "output": "用零食引导到固定地点"},
    {"instruction": "仓鼠笼要怎么清理", "output": "每周换一次垫料并消毒"},
    {"instruction": "今天的会议安排是什么", "output": "上午十点在产品会议室"},
    {"instruction": "如何写一份周报", "output": "按进展、风险、下周计划三段"},
]


def vector_items_file(tmp_path):
    """把 `ITEMS` 写成白名单内的数据文件，返回绝对路径字符串"""
    path = tmp_path / "l216_items.json"
    path.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return str(path)


def vector_config(tmp_path, **overrides):
    """按 `节.键` 覆盖出厂配置，返回一份指向临时目录的 `AppConfig`

    Args:
        tmp_path: pytest 临时目录
        **overrides: `vector.dimension` 形式的点号键

    Returns:
        AppConfig 实例
    """
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.vector.storage_dir = str(tmp_path / "vectors")
    for dotted, value in overrides.items():
        section, _, field = dotted.partition(".")
        setattr(getattr(config, section), field, value)
    return config


class TestDimensionIsConsumed:
    """`vector.dimension` 决定嵌入宽度，进而决定相似度与排名"""

    def test_dimension_sets_vector_width(self):
        vectors = embed_texts(["猫喜欢吃什么食物"], dimension=8)
        assert len(vectors) == 1
        assert vectors[0].shape == (8,)
        assert embed_texts(["猫喜欢吃什么食物"], dimension=1024)[0].shape == (1024,)

    def test_dimension_changes_scores_and_ranking(self):
        """同一份数据、同一个查询，换维度就换分值——这只键不是装饰"""
        narrow = search_similar(
            "猫爱吃的食物", ITEMS, backend="faiss", dimension=4,
            collection="default", top_k=3,
        )
        wide = search_similar(
            "猫爱吃的食物", ITEMS, backend="faiss", dimension=1024,
            collection="default", top_k=3,
        )
        assert [h["score"] for h in narrow] != [h["score"] for h in wide]
        # 4 维必然碰撞（二元组种类远多于 4 个桶），宽维度才把「猫吃什么」那条排到首位
        assert wide[0]["item"]["instruction"] == "猫喜欢吃什么食物"

    def test_bad_dimension_raises_at_the_store_gate(self):
        with pytest.raises(VectorError):
            embed_texts(["文本"], dimension=0)


class TestBackendIsDispatched:
    """`vector.backend` 的值真的进工厂，而不是被写死的默认值吃掉"""

    def test_configured_backend_reaches_the_factory(self, monkeypatch):
        """喂进来的配置值必须**与默认值可区分**，否则「写死默认值」这类缺陷测不出来。

        取 `chromadb` 而不是 `faiss` 做输入：工厂的默认参数就是 `"faiss"`，用同名值做
        校验时，把调用写成 `create_vector_db("faiss", ...)` 的缺陷臂与正确实现读到同一个
        字符串，判据就空转了。本机没装 chromadb 包，真构造会 ImportError，所以 spy 用
        `FAISSDB` 当替身只接住返回值——本例判「请求了哪只后端」，产物归
        `test_faiss_is_the_backend_actually_constructed`。
        """
        import augmentor.semantic as semantic

        seen = []

        def spy(backend, **kwargs):
            seen.append(backend)
            return FAISSDB(**kwargs)

        monkeypatch.setattr(semantic, "create_vector_db", spy)
        build_index(ITEMS, backend="chromadb", dimension=16, collection="default")
        assert seen == ["chromadb"]

    def test_unknown_backend_is_rejected_by_the_factory(self):
        """配置能写的取值受工厂那道约束：未知后端在建模时出声，不是静默换后端"""
        with pytest.raises(VectorError):
            build_index(
                ITEMS, backend="pinecone", dimension=16,
                collection="default",
            )

    def test_faiss_is_the_backend_actually_constructed(self):
        db = build_index(ITEMS, backend="faiss", dimension=16, collection="default")
        assert isinstance(db, FAISSDB)
        assert db.count() == len(ITEMS)


class TestPersistenceSurfaceIsGated:
    """落盘与读回两面对「没有 `persist`/`load` 的后端」在**构造之前**出声

    `vector.backend` 的合法取值不止 `faiss`（清单推导自 `SUPPORTED_BACKENDS`，`chromadb`
    也在内），但 `ChromaDB` 类里没有 `persist`/`load`，且它的构造阶段就 `mkdir` +
    `PersistentClient` + 注册集合。不加这道闸的话，两个后果都是本轮造成的：
    ① 配 `chromadb` 的调用方拿到 `AttributeError`（→ 500），而不是一句能读的能力说明；
    ② 一条 GET（`vector-status`）凭空创建目录 = 未鉴权写盘。

    判据取「错误类型 + 目录未被创建」两条，都能在本机现量：`pytest.raises(VectorError)`
    本身就排除了缺包那条路（没装 chromadb 时真走到工厂构造抛的是 `ImportError`，
    不是 `VectorError`），所以这只断言顺带证明了闸发生在工厂**之前**。
    """

    def test_persist_refuses_a_backend_without_the_surface(self, tmp_path):
        target = tmp_path / "chroma_store"
        with pytest.raises(VectorError) as exc:
            persist_index(
                ITEMS, backend="chromadb", dimension=16,
                collection="default", storage_dir=str(target),
            )
        assert "chromadb" in str(exc.value)
        assert not target.exists()

    def test_status_refuses_it_before_creating_the_dir(self, tmp_path):
        """`index_status` 是 GET 的那条路：闸必须先于构造，否则承诺不成立"""
        target = tmp_path / "status_dir"
        with pytest.raises(VectorError):
            index_status(
                backend="chromadb", dimension=16,
                collection="default", storage_dir=str(target),
            )
        assert not target.exists()

    def test_faiss_keeps_the_surface(self, tmp_path):
        """反面对照：同一个 storage_dir 下 faiss 会真的落盘，防「闸把两条都关掉」"""
        target = tmp_path / "faiss_store"
        receipt = persist_index(
            ITEMS, backend="faiss", dimension=16,
            collection="default", storage_dir=str(target),
        )
        assert receipt["count"] == len(ITEMS)
        assert target.exists()


class TestCollectionNamesTheArtifacts:
    """`vector.collection` 决定持久化文件名与向量 ID 前缀"""

    def test_collection_changes_file_names(self, tmp_path):
        base = tmp_path / "by_collection"
        for name in ("alpha", "beta"):
            persist_index(
                ITEMS, backend="faiss", dimension=16,
                collection=name, storage_dir=str(base / name),
            )
        for name in ("alpha", "beta"):
            directory = base / name
            assert (directory / f"{name}_vectors.npy").exists()
            assert (directory / f"{name}_meta.json").exists()
            # 另一只集合的文件不许出现在这个目录里
            other = "beta" if name == "alpha" else "alpha"
            assert not (directory / f"{other}_vectors.npy").exists()

    def test_collection_prefixes_vector_ids(self):
        hits = search_similar(
            "猫爱吃的食物", ITEMS, backend="faiss", dimension=64,
            collection="alpha", top_k=1,
        )
        assert hits and hits[0]["index"] == 0
        # ID 前缀由集合名生成（`VectorDB._generate_ids`），换集合就是换命名空间
        db = build_index(ITEMS, backend="faiss", dimension=64, collection="alpha")
        assert db._ids[0].startswith("alpha-")


class TestStorageDirIsWhereFilesLand:
    """`vector.storage_dir` 决定落点：这侧有、那侧空"""

    def test_persist_writes_only_into_configured_dir(self, tmp_path):
        left = tmp_path / "left"
        right = tmp_path / "right"
        receipt = persist_index(
            ITEMS, backend="faiss", dimension=16,
            collection="default", storage_dir=str(left),
        )
        assert receipt["count"] == len(ITEMS)
        assert list(left.iterdir()), "配置的目录里必须有产物"
        assert not right.exists(), "没配置过的目录不许被写"

    def test_index_status_reads_back_the_configured_dir(self, tmp_path):
        directory = tmp_path / "status"
        before = index_status(
            backend="faiss", dimension=16, collection="default",
            storage_dir=str(directory),
        )
        assert before["exists"] is False
        assert before["count"] == 0

        persist_index(
            ITEMS, backend="faiss", dimension=16, collection="default",
            storage_dir=str(directory),
        )
        after = index_status(
            backend="faiss", dimension=16, collection="default",
            storage_dir=str(directory),
        )
        assert after["exists"] is True
        assert after["count"] == len(ITEMS)
        assert after["dimension"] == 16
        assert after["storage_dir"] == str(directory)


class TestEnabledGatesPipelineIndex:
    """`vector.enabled` 是「增强结束时自动建库」的闸，出厂 false ⇒ 不建（零漂移）"""

    class OneVariantBackend(ModelBackend):
        """假后端：每条种子产出一条可预测的变体"""

        def __init__(self):
            super().__init__(ModelConfig(type="ollama"))

        def _call_api(self, prompt: str) -> str:
            return json.dumps(
                [{"instruction": "猫喜欢吃什么食物", "output": "鱼"}],
                ensure_ascii=False,
            )

    def _pipeline(self, tmp_path, tag, **overrides):
        config = vector_config(tmp_path, **overrides)
        config.augmentation.num_threads = 1
        instance = AugmentorPipeline(config)
        instance.model_backend = self.OneVariantBackend()
        instance.checkpoint_manager = CheckpointManager(
            checkpoint_dir=str(tmp_path / f"ckpt_{tag}"),
            auto_save_interval=1,
        )
        return instance

    def _seed(self, tmp_path, tag):
        path = tmp_path / f"seed_{tag}.json"
        # 写 bytes：默认文本模式会在 Windows 上把这份文件写成 CRLF
        path.write_text(
            json.dumps(ITEMS[:2], ensure_ascii=False), encoding="utf-8"
        )
        return str(path)

    def test_disabled_builds_nothing_and_reports_none(self, tmp_path):
        instance = self._pipeline(tmp_path, "off", **{"vector.enabled": False})
        report = instance.augment_dataset(
            self._seed(tmp_path, "off"), str(tmp_path / "out_off.json"),
            use_checkpoint=False, use_quality_check=False, use_dedup=False,
        )
        assert report["vector_index"] is None
        assert not Path(instance.config.vector.storage_dir).exists()

    def test_enabled_builds_and_reports_the_receipt(self, tmp_path):
        instance = self._pipeline(tmp_path, "on", **{"vector.enabled": True})
        report = instance.augment_dataset(
            self._seed(tmp_path, "on"), str(tmp_path / "out_on.json"),
            use_checkpoint=False, use_quality_check=False, use_dedup=False,
        )
        receipt = report["vector_index"]
        assert receipt["count"] == report["output_count"]
        assert receipt["backend"] == instance.config.vector.backend
        assert receipt["dimension"] == instance.config.vector.dimension
        assert Path(receipt["storage_dir"], f"{receipt['collection']}_vectors.npy").exists()

    def test_async_entry_follows_the_same_gate(self, tmp_path):
        """两个入口同一道判（L215 立的规矩），否则同一份配置出两种产物"""
        import asyncio

        instance = self._pipeline(tmp_path, "async", **{"vector.enabled": True})
        report = asyncio.run(instance.augment_async(
            self._seed(tmp_path, "async"), str(tmp_path / "out_async.json"),
            use_quality_check=False, use_dedup=False,
        ))
        assert report["vector_index"]["count"] == report["output_count"]


class TestSemanticSearchEndpoint:
    """`POST /api/dataset/semantic-search` 按配置生效值出结果，并回显生效值"""

    def _client(self, monkeypatch, config):
        import api.deps as deps
        from api.main import app
        from fastapi.testclient import TestClient

        stub = type("PipelineStub", (), {})()
        stub.config = config
        monkeypatch.setattr(deps, "_pipeline", stub)
        return TestClient(app)

    def test_returns_hits_and_echoes_effective_values(self, tmp_path, monkeypatch):
        config = vector_config(tmp_path, **{"vector.dimension": 64})
        client = self._client(monkeypatch, config)
        response = client.post(
            "/api/dataset/semantic-search",
            json={
                "input_file": vector_items_file(tmp_path),
                "query": "猫爱吃的食物",
                "top_k": 2,
            },
        )
        assert response.status_code == 200, response.text
        body = response.json()
        assert body["dimension"] == 64
        assert body["backend"] == config.vector.backend
        assert body["total"] == 2
        assert len(body["results"]) == 2
        assert body["results"][0]["item"]["instruction"] == "猫喜欢吃什么食物"

    def test_dimension_from_config_changes_the_ranking(self, tmp_path, monkeypatch):
        """同一请求、两份配置：排名跟着 `dimension` 走，端点面也拿得到行为差"""
        narrow = vector_config(tmp_path, **{"vector.dimension": 4})
        wide = vector_config(tmp_path, **{"vector.dimension": 1024})
        payload = {
            "input_file": vector_items_file(tmp_path),
            "query": "猫爱吃的食物",
            "top_k": 5,
        }
        first = self._client(monkeypatch, narrow).post(
            "/api/dataset/semantic-search", json=payload
        ).json()
        second = self._client(monkeypatch, wide).post(
            "/api/dataset/semantic-search", json=payload
        ).json()
        assert first["dimension"] == 4 and second["dimension"] == 1024
        assert [r["score"] for r in first["results"]] != [r["score"] for r in second["results"]]

    def test_text_key_selects_the_embedded_field(self, tmp_path, monkeypatch):
        """`text_key` 换成 `output`，命中的就是另一批条目——检索面真的按字段走"""
        config = vector_config(tmp_path, **{"vector.dimension": 256})
        client = self._client(monkeypatch, config)
        by_instruction = client.post(
            "/api/dataset/semantic-search",
            json={
                "input_file": vector_items_file(tmp_path),
                "query": "上午十点产品会议室",
                "top_k": 1,
                "text_key": "instruction",
            },
        ).json()
        by_output = client.post(
            "/api/dataset/semantic-search",
            json={
                "input_file": vector_items_file(tmp_path),
                "query": "上午十点产品会议室",
                "top_k": 1,
                "text_key": "output",
            },
        ).json()
        assert by_output["results"][0]["item"]["instruction"] == "今天的会议安排是什么"
        assert by_instruction["results"][0]["index"] != 3

    def test_negative_top_k_is_400_from_the_sdk_gate(self, tmp_path, monkeypatch):
        client = self._client(monkeypatch, vector_config(tmp_path))
        response = client.post(
            "/api/dataset/semantic-search",
            json={
                "input_file": vector_items_file(tmp_path),
                "query": "猫",
                "top_k": -1,
            },
        )
        assert response.status_code == 400

    def test_outside_whitelist_input_is_403(self, tmp_path, monkeypatch):
        # 平台无关的越界判据：白名单只留 tmp_path，它之外的绝对路径 → 403。
        # ⚠️ 不能用 Windows 路径字面量：POSIX 上反斜杠不是分隔符，
        # "D:\Windows\..." 是白名单内的相对名 → 404 而非 403（CI 实测）。
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
        client = self._client(monkeypatch, vector_config(tmp_path))
        response = client.post(
            "/api/dataset/semantic-search",
            json={
                "input_file": str(tmp_path.parent / "not_a_dataset.json"),
                "query": "猫",
            },
        )
        assert response.status_code == 403


class TestVectorStatusEndpoint:
    """`GET /api/dataset/vector-status` 让五只键的生效值与建库结果可见"""

    def _client(self, monkeypatch, config):
        import api.deps as deps
        from api.main import app
        from fastapi.testclient import TestClient

        stub = type("PipelineStub", (), {})()
        stub.config = config
        monkeypatch.setattr(deps, "_pipeline", stub)
        return TestClient(app)

    def test_reports_configured_values_before_any_build(self, tmp_path, monkeypatch):
        config = vector_config(
            tmp_path, **{"vector.dimension": 32, "vector.collection": "ready"}
        )
        body = self._client(monkeypatch, config).get(
            "/api/dataset/vector-status"
        ).json()
        assert body["enabled"] == config.vector.enabled
        assert body["dimension"] == 32
        assert body["collection"] == "ready"
        # `backend` 三面同义（配置选择），不是 `FAISSDB.backend` 那个「实际引擎」属性：
        # 本机没装 faiss-cpu 时后者是 "numpy"，把它当作本面的读数会让同一个键名有两套含义。
        assert body["backend"] == config.vector.backend
        assert body["exists"] is False
        assert body["count"] == 0

    def test_exists_flips_after_the_pipeline_builds(self, tmp_path, monkeypatch):
        config = vector_config(tmp_path, **{"vector.enabled": True})
        persist_index(
            ITEMS,
            backend=config.vector.backend,
            dimension=config.vector.dimension,
            collection=config.vector.collection,
            storage_dir=config.vector.storage_dir,
        )
        body = self._client(monkeypatch, config).get(
            "/api/dataset/vector-status"
        ).json()
        assert body["exists"] is True
        assert body["count"] == len(ITEMS)

    def test_backend_without_the_surface_is_400_and_creates_nothing(
            self, tmp_path, monkeypatch):
        """配 `chromadb` 的 GET 收 400 而不是 500，且临时目录里什么都没长出来

        这条是 `TestPersistenceSurfaceIsGated` 的端点面：SDK 那道闸若位置不对（写在
        构造之后），这里就会看到目录；若漏了，就是 500。
        """
        config = vector_config(tmp_path, **{"vector.backend": "chromadb"})
        response = self._client(monkeypatch, config).get(
            "/api/dataset/vector-status"
        )
        assert response.status_code == 400
        assert "chromadb" in response.json()["detail"]
        assert not Path(config.vector.storage_dir).exists()
