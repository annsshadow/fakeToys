# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L318（A295）：RAG 目标格式的「出厂默认」只留**一份**权威

**这一格原来错在哪**：同一个「出厂默认的 RAG 目标格式」在仓里有三处字面量、
其中一处写的还是**另一个格式名**：

| 位置 | 改前写的 | 说的是不是同一件事 |
| --- | --- | --- |
| `RAGConfig.default_format`（配置出厂值） | `llamaindex` | 是 |
| `RAGFormatter.format` 的形参默认 | `llamaindex` | 是 |
| **端点那一行**的第三档兜底 | **`langchain`** | **不是** ⇒ 真兜到它时产物是 langchain 形状，而配置与文档都说 llamaindex |

今天那一档**不可达**（配置那格非空），但它是一只**埋着的谎**：一旦配置被显式写成
空串（配置校验只判 `type: str`，不判非空），端点就会悄悄产出**另一种**格式的产物，
而回执里的 `format` 字段会写「langchain」——调用方拿到的东西与文档承诺的不一样。

**修法（自决，记理由）**：取 A295 记的修法②「把它改成与出厂默认同源的一个常数」——
新增 `augmentor.rag.DEFAULT_RAG_FORMAT`（写在**文件末尾**，既有行号一格未动，
与 A285 那只常数的处置同款），端点那一行的第三档改引它。**不取修法①**
（「删掉那一档兜底」）：配置那格可以被显式写成空串（校验器只判类型），删掉之后
空串会一路走到 SDK 的「不支持的 RAG 格式: 」，回执比「悄悄换成另一种格式」更难读。

**三面判据**：

1. **单一权威面**：`DEFAULT_RAG_FORMAT` == `RAGFormatter.format` 的形参默认
   == `RAGConfig().default_format`（三者**逐字相等**，任何一处改了而另两处没改 ⇒ 红）。
2. **端点那一行**：源码里那一行的第三档必须引那只常量，且**不许**再出现
   `langchain` 这个字面量（它就是那只谎本身）。
3. **行为面**：① 省略 `format` ⇒ 产物是出厂默认那一种形状；② 配置被显式写成空串
   ⇒ 落到第三档（出厂默认）而**不是** langchain 形状（这一格就是本轮真正修掉的那一格，
   改前会产出一份 langchain 形状的产物）；③ 显式 `format` 仍然赢。

**注入臂**（内存里改，**零写入**）：把常量换成 `langchain` ⇒ 权威面那一支红。

跑法：`python -m pytest tests/unit/test_rag_default_format_l318.py -q --no-cov`
"""

import inspect
import json
import pathlib

import pytest

AI_DIR = pathlib.Path(__file__).resolve().parent.parent.parent
ROUTE = AI_DIR / "api" / "routes" / "dataset_tools.py"


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
def seed(tmp_path):
    """落在 CWD（= tmp_path）里的两条样本"""
    name = "seed_l318.json"
    pathlib.Path(name).write_text(
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


class TestOneAuthority:
    """单一权威面：常量、形参默认、配置出厂值三者逐字相等"""

    def test_the_constant_matches_the_signature_default(self):
        from augmentor.rag import DEFAULT_RAG_FORMAT, RAGFormatter

        default = inspect.signature(RAGFormatter.format).parameters["fmt"].default
        assert DEFAULT_RAG_FORMAT == default, (DEFAULT_RAG_FORMAT, default)

    def test_the_constant_matches_the_config_factory_default(self):
        from augmentor.config import RAGConfig
        from augmentor.rag import DEFAULT_RAG_FORMAT

        assert DEFAULT_RAG_FORMAT == RAGConfig().default_format

    def test_injection_a_flipped_constant_turns_this_face_red(self):
        """注入臂：把常量换成另一个格式名 ⇒ 权威面当场红"""
        from augmentor.config import RAGConfig
        from augmentor.rag import DEFAULT_RAG_FORMAT, RAGFormatter

        injected = "langchain"
        default = inspect.signature(RAGFormatter.format).parameters["fmt"].default
        assert injected != DEFAULT_RAG_FORMAT
        assert injected != default
        assert injected != RAGConfig().default_format


class TestTheRouteLine:
    """端点那一行：第三档引那只常量，且不再写着另一个格式名"""

    def _fmt_line(self) -> str:
        for line in ROUTE.read_text(encoding="utf-8").split("\n"):
            if "fmt = request.format" in line:
                return line
        raise AssertionError("端点里找不到 `fmt = request.format` 那一行 ⇒ 判据要跟着换形状")

    def test_the_third_tier_cites_the_constant(self):
        line = self._fmt_line()
        assert "DEFAULT_RAG_FORMAT" in line, line
        assert 'or "langchain"' not in line, line
        assert 'or "llamaindex"' not in line, (
            "第三档不许再写一个字面量（写出来就是第二份权威）"
        )

    def test_the_module_imports_the_constant(self):
        source = ROUTE.read_text(encoding="utf-8")
        assert "from augmentor.rag import DEFAULT_RAG_FORMAT, RAGFormatter" in source, (
            "那一行必须从 SDK 侧 import 那只常量（从别处 import 等于又一份权威）"
        )


class TestBehavior:
    """行为面：省略 / 空配置 / 显式三档"""

    def test_omitted_format_produces_the_factory_shape(self, client, pipeline, seed):
        resp = client.post(
            "/api/dataset/rag", json={"input_file": seed, "output_file": "rag_a.json"}
        )
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "llamaindex"
        records = json.loads(pathlib.Path("rag_a.json").read_text(encoding="utf-8"))
        assert records and "text" in records[0]

    def test_an_explicitly_emptied_config_falls_back_to_the_factory_default(
        self, client, pipeline, seed
    ):
        """**本轮真正修掉的那一格**：配置被显式写成空串 ⇒ 落到出厂默认，
        而**不是**改前那样悄悄产出一份 langchain 形状的产物"""
        pipeline.config.rag.default_format = ""
        resp = client.post(
            "/api/dataset/rag", json={"input_file": seed, "output_file": "rag_b.json"}
        )
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "llamaindex", (
            "配置那格为空时第三档必须是出厂默认；改前那一档写的是 langchain"
        )
        records = json.loads(pathlib.Path("rag_b.json").read_text(encoding="utf-8"))
        assert records and "text" in records[0], (
            "产物必须是 llamaindex 形状（langchain 形状的键是 page_content / metadata）"
        )

    def test_an_explicit_format_still_wins(self, client, pipeline, seed):
        resp = client.post(
            "/api/dataset/rag",
            json={
                "input_file": seed,
                "output_file": "rag_c.json",
                "format": "langchain",
            },
        )
        assert resp.status_code == 200, resp.text
        assert resp.json()["format"] == "langchain"
