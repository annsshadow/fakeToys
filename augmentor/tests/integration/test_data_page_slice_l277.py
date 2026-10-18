# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""分页的坐标与边界（A367 / L277）

`GET /api/data/load/{filename}` 交回的是**过滤后切片**，而
`PUT /api/data/update` 与 `DELETE /api/data/delete` 读的是整份文件、按 `items[index]`
直接写盘。两个坐标系从来不等：

1. 行号 ≠ 盘上下标。第 2 页第 0 行的 antd 行号是 0，盘上却是第 10 条；`search` 生效时
   连「偏移一个整数」都不是（命中的是散落的第 2、7、13、20 条）。
2. `page` / `page_size` 落进 `items[start:end]`，而 Python 切片对越界值只换语义不报错。

实测改前（25 条夹具）：`page_size=-10` 回 **200 并交出 15 条**、`page=-1` 回 **200 并
交出第 5~14 条**、`page=0` / `page_size=0` 回 **200 交出空页**；拿着页内行号发
`PUT ?index=0` 回 **200「保存成功」**，改掉的是第 1 页那一行。两种都是静默错答。

本模块钉住修法后的两面：越界参数在门口落 400（判据共引 SDK 的 `require_count`），
`indices` 与 `items` 同长同序、逐格是盘上 0 基下标，写端点按它落子精确命中。
"""

import json
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

N_ITEMS = 25
NEEDLES = (2, 7, 13, 20)
LOAD = "/api/data/load/data.json"
UPDATE = "/api/data/update/data.json"
DELETE = "/api/data/delete/data.json"
WEB = Path(__file__).resolve().parents[2] / "web"


def _items():
    return [
        {
            "instruction": ("needle " if i in NEEDLES else "") + f"q{i}",
            "input": "",
            "output": f"a{i}",
        }
        for i in range(N_ITEMS)
    ]


@pytest.fixture
def api(tmp_path, monkeypatch):
    """把数据根收到 tmp_path、撤掉密钥，夹具数据 25 条、needle 散落在 4 个绝对位置"""
    from api.main import app

    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    monkeypatch.chdir(tmp_path)
    (tmp_path / "data.json").write_text(
        json.dumps(_items(), ensure_ascii=False), encoding="utf-8"
    )
    return TestClient(app), tmp_path


def _disk(tmp_path):
    return json.loads((tmp_path / "data.json").read_text(encoding="utf-8"))


def _load(client, **params):
    """打一次分页读，返回 (状态码, 页, 页内行, 坐标)"""
    response = client.get(LOAD, params=params)
    if response.status_code != 200:
        return response.status_code, response.text
    payload = response.json()
    return payload["items"], payload["indices"]


class TestTheCoordinatesAreDiskPositions:
    """`indices` 逐格是「这一条在盘上那份文件里的 0 基下标」"""

    def test_without_search_the_pages_tile_the_file(self, api):
        client, _ = api
        for page in (1, 2, 3):
            items, indices = _load(client, page=page, page_size=10)
            assert indices == list(range((page - 1) * 10, (page - 1) * 10 + len(items)))
            assert len(indices) == len(items)

    def test_the_second_page_is_not_numbered_from_zero(self, api):
        """尺子不许恒真：翻页时坐标必须**不等于**页内行号，否则本模块什么也没量到"""
        client, _ = api
        _items_on_page, indices = _load(client, page=2, page_size=10)
        assert indices != list(range(len(indices)))

    def test_with_search_the_rows_carry_their_own_absolute_positions(self, api):
        client, _ = api
        items, indices = _load(client, page=1, page_size=10, search="needle")
        assert indices == list(NEEDLES)
        assert [it["instruction"] for it in items] == [f"needle q{i}" for i in NEEDLES]

    def test_the_second_page_of_a_search_subset(self, api):
        """搜索 + 翻页同时生效：第 2 页（每页 2 条）拿到的是第 13、20 条"""
        client, _ = api
        items, indices = _load(client, page=2, page_size=2, search="needle")
        assert indices == [13, 20]
        assert [it["instruction"] for it in items] == ["needle q13", "needle q20"]

    def test_total_counts_the_filtered_subset(self, api):
        client, _ = api
        response = client.get(LOAD, params={"page": 1, "page_size": 10, "search": "needle"})
        assert response.json()["total"] == len(NEEDLES)


class TestTheReturnedCoordinateIsTheWriteCoordinate:
    """按 `indices` 发写请求，落子的就是页里看到的那一行——而且只那一行"""

    def test_update_changes_that_row_and_no_other(self, api):
        client, tmp_path = api
        before = _disk(tmp_path)
        items, indices = _load(client, page=2, page_size=2, search="needle")
        target = indices[0]

        response = client.put(
            UPDATE,
            params={"index": target},
            json={"instruction": "EDITED", "input": "", "output": "edited"},
        )
        assert response.status_code == 200, response.text

        after = _disk(tmp_path)
        assert len(after) == N_ITEMS
        assert after[target] == {"instruction": "EDITED", "input": "", "output": "edited"}
        assert items[0]["instruction"] == before[target]["instruction"]
        assert [i for i in range(N_ITEMS) if after[i] != before[i]] == [target]

    def test_delete_removes_that_row_and_no_other(self, api):
        client, tmp_path = api
        before = _disk(tmp_path)
        _items_on_page, indices = _load(client, page=1, page_size=10, search="needle")
        target = indices[-1]

        response = client.delete(DELETE, params={"index": target})
        assert response.status_code == 200, response.text

        after = _disk(tmp_path)
        assert len(after) == N_ITEMS - 1
        assert after == before[:target] + before[target + 1:]

    def test_the_page_local_number_is_a_real_wound(self, api):
        """反事实：把页内行号当坐标发出去，改掉的是**另一页**那一行且照样回 200

        这一支不是给旧行为留档，而是说明 `indices` 为什么是必需的一格——前端只看得到
        行号，坐标必须由后端随响应一起交回。
        """
        client, tmp_path = api
        before = _disk(tmp_path)
        items, indices = _load(client, page=2, page_size=10)
        assert items[0]["instruction"] == before[indices[0]]["instruction"]

        response = client.put(
            UPDATE,
            params={"index": 0},
            json={"instruction": "WRONG_ROW", "input": "", "output": "x"},
        )
        assert response.status_code == 200, response.text

        after = _disk(tmp_path)
        assert after[0]["instruction"] == "WRONG_ROW"
        assert after[indices[0]]["instruction"] == before[indices[0]]["instruction"]

    def test_out_of_range_writes_are_refused_both_ways(self, api):
        """越界写不落到 Python 的负索引语义上（`pop(-1)` 会删掉最后一条）"""
        client, tmp_path = api
        for endpoint, verb in ((UPDATE, "put"), (DELETE, "delete")):
            for index in (-1, N_ITEMS, 10**6):
                kwargs = {"params": {"index": index}}
                if verb == "put":
                    kwargs["json"] = {"instruction": "x", "input": "", "output": "y"}
                assert getattr(client, verb)(endpoint, **kwargs).status_code == 400, (
                    endpoint,
                    index,
                )
        assert len(_disk(tmp_path)) == N_ITEMS


class TestPagingParametersRefuseToSlice:
    """`page` / `page_size` 越界 ⇒ 门口 400，不再交出「形状合法、内容相反」的页"""

    def test_negative_page_size_no_longer_means_all_but_the_last_ten(self, api):
        client, _ = api
        response = client.get(LOAD, params={"page": 1, "page_size": -10})
        assert response.status_code == 400, response.text
        assert "page_size" in response.json()["detail"]

    def test_negative_page_no_longer_hands_back_a_middle_chunk(self, api):
        client, _ = api
        response = client.get(LOAD, params={"page": -1, "page_size": 10})
        assert response.status_code == 400, response.text
        assert "page" in response.json()["detail"]

    def test_zero_is_a_refusal_not_an_empty_page(self, api):
        """改前 `page=0` / `page_size=0` 都回 200 带空页——调用方无从知道是自己写错了"""
        client, _ = api
        for params in ({"page": 0, "page_size": 10}, {"page": 1, "page_size": 0}):
            response = client.get(LOAD, params=params)
            assert response.status_code == 400, (params, response.text)

    def test_the_judgment_is_the_sdks_one_copy(self, api):
        """文案来自 `require_count`，本端点不另立判据（A77）"""
        client, _ = api
        detail = client.get(LOAD, params={"page": 0, "page_size": 10}).json()["detail"]
        assert detail == "page 必须是不小于 1 的整数，当前是 0"

    def test_a_page_past_the_end_is_still_an_empty_page(self, api):
        """不收进 400 的那一格必须写明：页码越界是合法请求，只是没数据"""
        client, _ = api
        response = client.get(LOAD, params={"page": 99, "page_size": 10})
        assert response.status_code == 200, response.text
        assert response.json()["items"] == [] and response.json()["indices"] == []
        assert response.json()["total"] == N_ITEMS

    def test_the_door_guard_is_what_refuses(self, monkeypatch, api):
        """注入臂（零写入）：摘掉门口那两行判据，越界参数立刻回到静默错答

        证明上面四支量的确实是这道闸，而不是 FastAPI 的参数校验或别处的兜底。
        """
        import api.routes.data as route

        client, _ = api
        monkeypatch.setattr(route, "require_count", lambda name, value, minimum=0: value)
        assert client.get(LOAD, params={"page": -1, "page_size": 10}).status_code == 200
        assert client.get(LOAD, params={"page": 1, "page_size": 0}).status_code == 200


class TestTheClientSendsWhatItWasGiven:
    """前端接线：坐标来自 `indices`，不是 antd 给的行号"""

    def test_types_declare_the_coordinate(self):
        text = (WEB / "src" / "types" / "api.ts").read_text(encoding="utf-8")
        assert "indices: number[]" in text

    def test_the_page_maps_rows_through_the_coordinate(self):
        text = (WEB / "src" / "pages" / "DataManagement.tsx").read_text(encoding="utf-8")
        assert "setRowIndices(result.indices)" in text
        assert "const absoluteIndex = (rowIndex: number) => rowIndices[rowIndex]" in text
        # 三处消费点：索引列、编辑、删除——少一处就是那一列还在发行号
        assert text.count("absoluteIndex(") == 3
        assert "render: (_, __, index) => (page - 1) * pageSize + index + 1" not in text
