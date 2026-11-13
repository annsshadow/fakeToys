# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L327（A272 收口）：`multilingual` 那一节的**口径写实**——可见，但**不可改**

**A272 原来记的是什么**：「`api/routes/config.py` 的可写四节 / `multilingual` 节 …—
可写面至今只有 export / rag / vector / multimodal 四节，`multilingual` 要改只能改
`config.yaml` 重启」。**两件事**：

- **「可写四节」这句已过时**：L217 写下它之后，`augmentation` / `quality` / `dedup`
  三节也拿到了运行时判据（A118 / A341），**今天的可写清单是七节** ⇒ 账本那句按现量订正。
- **`multilingual` 的口径要写实**：它是**只读**节（L217 接上读者、L327 进回显面），
  **不在**可写清单里 ⇒ 送它回 `POST /api/config` 会落进 `ignored_keys`
  （**出声、不静默、也不写入**）。

**这一支钉的四面**：
① **可写清单**是七节，且 `multilingual` **不在**其中；
② **`multilingual` 在回显面里**，四只键逐只可见（按「**有读者 ⇒ 必须可见**」）；
③ **写不进**：送 `{"multilingual": …}` 回 `POST /api/config` ⇒ 落进 `ignored_keys`，
  且**配置对象的生效值一格未动**（`enabled` / `default_target_lang` / `supported_langs` /
  `translate_batch_size` 逐只对比）；
④ **契约面**（OpenAPI）里 `ConfigResponse` 有 `multilingual` 这一节 ⇒ 前端与文档对得上。

**零 `parametrize`**（仓里「新守卫一律零 parametrize」的惯例 ⇒ l97 / l98 两把
参数化普查尺的读数一格未动）。
"""

import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from api.main import app  # noqa: E402
from api.routes.config import WRITABLE_SECTIONS, ConfigResponse  # noqa: E402
from fastapi.testclient import TestClient  # noqa: E402


#: 今天 `POST /api/config` 的可写清单（A118 / A341 之后是**七节**，不是账本旧句里的「四节」）。
#: 这份名单与 `ConfigUpdateRequest` 的七个 dict 字段一一对应，由
#: `tests/unit/test_upload_ceiling_l87.py::test_writable_list_matches_the_request_schema` 钉住；
#: 本支再钉一遍「**`multilingual` 不在其中**」——那正是 A272 的口径。
EXPECTED_WRITABLE = (
    "augmentation", "quality", "dedup", "export", "vector", "rag", "multimodal",
)

#: `multilingual` 的四只键：L217 接上读者、L327 进回显面。
MULTILINGUAL_KEYS = (
    "enabled", "default_target_lang", "supported_langs", "translate_batch_size",
)

#: 只读节（不在可写清单里、但在回显面里）：`versioning` / `visualization` / `multilingual`。
READ_ONLY_SECTIONS = ("versioning", "visualization", "multilingual")


@pytest.fixture()
def client():
    return TestClient(app)


class TestTheWritableListIsSevenSections:
    def test_the_list_is_exactly_these_seven(self):
        assert tuple(WRITABLE_SECTIONS) == EXPECTED_WRITABLE, WRITABLE_SECTIONS

    def test_multilingual_is_not_writable(self):
        assert "multilingual" not in WRITABLE_SECTIONS, (
            "A272 的口径：`multilingual` 是只读节，不在 `POST /api/config` 的可写清单里")

    def test_the_read_only_sections_are_not_writable(self):
        for section in READ_ONLY_SECTIONS:
            assert section not in WRITABLE_SECTIONS, section


class TestMultilingualIsVisible:
    def test_the_response_model_carries_the_section(self):
        assert "multilingual" in ConfigResponse.model_fields, sorted(ConfigResponse.model_fields)

    def test_the_openapi_contract_carries_the_section(self):
        schema = app.openapi()["components"]["schemas"]["ConfigResponse"]
        assert "multilingual" in schema["properties"], sorted(schema["properties"])

    def test_the_get_face_shows_all_four_keys(self, client):
        resp = client.get("/api/config")
        assert resp.status_code == 200, resp.text[:200]
        section = resp.json()["multilingual"]
        for key in MULTILINGUAL_KEYS:
            assert key in section, (key, sorted(section))


class TestMultilingualIsNotWritable:
    def test_posting_the_section_is_reported_as_ignored(self, client):
        resp = client.post(
            "/api/config",
            json={"multilingual": {"enabled": True, "translate_batch_size": 7}},
        )
        assert resp.status_code == 200, resp.text[:200]
        body = resp.json()
        assert body["ignored_keys"] == ["multilingual"], body["ignored_keys"]
        assert "配置已保存" not in body["message"], body["message"]

    def test_posting_the_section_changes_nothing(self, client):
        before = client.get("/api/config").json()["multilingual"]
        client.post(
            "/api/config",
            json={"multilingual": {"enabled": True, "translate_batch_size": 7}},
        )
        after = client.get("/api/config").json()["multilingual"]
        assert after == before, (before, after)
        for key in MULTILINGUAL_KEYS:
            assert after[key] == before[key], key


class TestTheCaliberIsWrittenDown:
    """口径要写在**人读的那一面**（API.md），不是只活在代码与守卫里"""

    def test_the_api_doc_says_the_section_is_read_only(self):
        doc = (ROOT.parent / "docs" / "augmentor" / "API.md").read_text(encoding="utf-8")
        assert "只读节看得见、但改不了" in doc, "API.md 里没有那一段口径说明"
        assert "`multilingual`" in doc, "API.md 里没有点名 multilingual"
        assert "可写的是**七节**" in doc, "API.md 里没有把可写节数按现量写准"

    def test_the_ledger_no_longer_claims_four_writable_sections(self):
        ledger = (ROOT / "OPTIMIZATION_LOOP_2.md").read_text(encoding="utf-8")
        row = next(line for line in ledger.splitlines() if line.startswith("| A272 "))
        assert "可写四节" not in row, "A272 那一行还写着过时的「可写四节」"
        assert "已收口（L327）" in row, row[-80:]
