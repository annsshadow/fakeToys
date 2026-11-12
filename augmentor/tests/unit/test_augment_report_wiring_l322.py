# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L322：两只 augment 端点进前端面 + l233 的门禁口径拍板

**这一轮收的是哪一族**：L321 把两只 augment 端点（`GET /api/augment/report/{task_id}` 与
`DELETE /api/augment/checkpoints/{task_id}`）第一次写进文档面，于是 l224 / l233 / l252
那三把尺子的名册被顶高一格而**从此一直红**（13 支）。L321 自己把名册那一半与门禁那一半
分开放，说「门禁那一半要单独拍」⇒ 这一轮拍的就是它。

**拍板（自决，记理由）**：门禁只管**写面**，读面原则上不挂门禁 —— **唯一的例外是
`GET /api/augment/report/{task_id}`**。

理由不是「读端点都该挂」也不是「这一只特殊」，而是**仓库里早就拍过了**：
`tests/integration/test_api_security.py` 的 `TestWriteRouteAuthCoverage.PROTECTED`
名单里，从第 35 轮那两只端点上线起就一直写着这一条，注释是「报告含失败明细」⇒
**「报告端点属于密面」是既有裁定**，而 l233 那句「读操作一律不挂门禁」是后来写下的
**更宽的口径**，它与既有裁定冲突 ⇒ 这一轮是把 l233 那句**收窄**成与既有裁定一致，
不是反过来放宽产品面。

另一条路（把门禁从报告端点上摘掉，让 l233 那句原样成立）本轮**不取**：任务 id 是
「输入文件解析后路径的 md5 前 12 位」⇒ 知道文件名的人**推得出**别人的任务 id
⇒ 不挂门禁等于把落盘路径与逐条失败预览敞开给任何猜得到 id 的人。

**代价与兜底**：开了鉴权以后页面读这只端点要带 key ⇒ 「开了鉴权后页面仍然读得到东西」
这句话对**它**不再成立。兜底是现成的：L233 建的那条通道对**所有**请求生效（含 GET），
所以配了 key 之后页面照样读得到 ⇒ 这一轮前端那道门（`test_api_key_channel_l233.py`）
把前提从「一条 GET 都不许挂」收紧成「挂门禁的 GET **恰好**是这一只」。

**四面判据**：
1. **接线面**（静态）：服务层有一只 `getAugmentReport` 打的是 `/augment/report/…`，
   增强页**用**了它（不是只导出）。跑法：读源码文本，零运行。
2. **跨语言契约面**：TS 接口 `AugmentReport` 的字段名集合、zod 面
   `augmentReportResponseSchema` 的键集、契约 `AugmentReportResponse` 的属性集，
   **三者逐格相等**；`deleteCheckpoint` 的返回类型必须是**具名**的 `MessageResponse`
   （内联对象字面量 ⇒ `l237` 那把尺子解析不动它）。
3. **门禁口径面**（含真链路）：报告端点**确实**在门禁面里；门禁面里挂门禁的**读**操作
   恰好是它一只；真打一次 HTTP —— 门禁升起且没带 key ⇒ **401** + 那句判决，
   带了 key ⇒ 落到 **404**（不是 401）。
4. **非恒真面**（注入臂）：把服务层的路径改错、把 zod 面的一格删掉，分别当场喊红。

跑法：`python -m pytest tests/unit/test_augment_report_wiring_l322.py -q --no-cov`
"""

import pathlib
import re

import pytest

from tests.unit.test_wired_dropdown_choices_l226 import strip_comments

ROOT = pathlib.Path(__file__).resolve().parents[2]
WEB_SRC = ROOT / "web" / "src"
API_TS = WEB_SRC / "services" / "api.ts"
AUGMENTATION_TSX = WEB_SRC / "pages" / "Augmentation.tsx"
TYPES_TS = WEB_SRC / "types" / "api.ts"
MAIN_SCHEMAS_TS = WEB_SRC / "schemas" / "main-schemas.ts"

needs_web_src = pytest.mark.skipif(
    not API_TS.is_file(),
    reason="前端源码不在场（只带构建产物的安装没有 web/）",
)

#: 报告端点的契约路径（`{task_id}` 是契约面的占位名）
REPORT_OP = ("GET", "/api/augment/report/{task_id}")
#: 删断点那只
DELETE_OP = ("DELETE", "/api/augment/checkpoints/{task_id}")


def _src(path: pathlib.Path) -> str:
    assert path.is_file(), f"{path.name} 不在场 ⇒ 本文件的判据全部失效"
    return path.read_text(encoding="utf-8")


def zod_keys(source: str, schema_name: str) -> set[str]:
    """zod `z.object({ … })` 的**顶层**键集

    只取顶层：嵌套块（`z.record(z.string(), z.unknown())` 这类）内部的键不是契约的键。
    逐行按两格缩进切，与 `l237` 那把尺子解析 TS 接口的口径同族。
    """
    m = re.search(
        rf"export const {schema_name} = z\.object\(\{{(.*?)\n\}}\)", source, re.S
    )
    assert m, f"{schema_name} 不在场"
    return set(re.findall(r"^  (\w+):", m.group(1), re.M))


def contract_props(schema_name: str) -> set[str]:
    """契约面某个响应模型的属性集"""
    from api.main import app

    schemas = app.openapi()["components"]["schemas"]
    assert schema_name in schemas, f"契约里没有 {schema_name}"
    return set(schemas[schema_name].get("properties", {}))


def ts_interface_fields(source: str, name: str) -> set[str]:
    """TS `export interface X { … }` 的字段名集合（复用 `l237` 那把解析器）"""
    from tests.unit.test_ts_contract_types_l237 import TsFace

    ts = TsFace(strip_comments(source))
    assert name in ts.fields, f"类型面没有 {name}"
    return set(ts.fields[name])


# ---------------------------------------------------------------------------
# 面 1：接线面
# ---------------------------------------------------------------------------


@needs_web_src
class TestTheWiringFace:
    """服务层有一只报告 fetcher，且增强页**用**了它"""

    def test_the_service_layer_calls_the_report_endpoint(self):
        code = strip_comments(_src(API_TS))
        assert re.search(
            r"export const getAugmentReport = async \(taskId: string\): Promise<AugmentReport> =>",
            code,
        ), "服务层没有那只报告 fetcher（或它的类型标注不是具名的 AugmentReport）"
        assert re.search(r"api\.get\(`/augment/report/\$\{[^}]+\}`\)", code), (
            "报告 fetcher 打的那条路径不对")

    def test_the_page_actually_uses_it(self):
        """**只导出没人用**是这一族最典型的漏法（`api.wiring.test.ts` 也在管，
        但那一把是前端门禁；这一格从仓外点名它确实接在增强页上）"""
        code = strip_comments(_src(AUGMENTATION_TSX))
        assert "getAugmentReport" in code, "增强页没有引用那只报告 fetcher"
        # 「引用」不等于「用」：至少要有一处**调用**形状
        assert re.search(r"await getAugmentReport\(", code), "增强页只是 import 了它"

    def test_the_delete_checkpoint_fetcher_is_named_too(self):
        """删断点那只的返回类型必须是具名的 `MessageResponse`

        改前写的是内联对象字面量 `{ success: boolean }` ⇒ `l237` 那把尺子解析不动它
        （`unresolved` 一员）⇒ 这里点名。
        """
        code = strip_comments(_src(API_TS))
        assert re.search(
            r"export const deleteCheckpoint = async \(taskId: string\): Promise<MessageResponse> =>",
            code,
        ), "删断点那只的返回类型不是具名的 MessageResponse"


# ---------------------------------------------------------------------------
# 面 2：跨语言契约面
# ---------------------------------------------------------------------------


@needs_web_src
class TestTheCrossLanguageContract:
    """TS 接口 / zod 面 / 契约面，三者逐格相等"""

    def test_the_three_faces_agree_on_the_report_keys(self):
        ts = ts_interface_fields(_src(TYPES_TS), "AugmentReport")
        zod = zod_keys(_src(MAIN_SCHEMAS_TS), "augmentReportResponseSchema")
        contract = contract_props("AugmentReportResponse")
        assert contract, "契约面读不到属性集"
        assert ts == contract, sorted(ts ^ contract)
        assert zod == contract, sorted(zod ^ contract)

    def test_the_three_faces_agree_on_the_delete_receipt_keys(self):
        ts = ts_interface_fields(_src(TYPES_TS), "MessageResponse")
        zod = zod_keys(_src(MAIN_SCHEMAS_TS), "deleteCheckpointSchema")
        contract = contract_props("MessageResponse")
        assert ts == contract, sorted(ts ^ contract)
        assert zod == contract, sorted(zod ^ contract)

    def test_the_delete_receipt_is_not_a_half_optional_shape(self):
        """`message` 在契约里**必填**（`api/schemas.py` 的 `MessageResponse`）⇒
        zod 面不许写「可选」——改前写的是 `message: z.string().optional()`，
        那是把「回执文案可能没有」这句猜测当成了契约。"""
        contract = contract_props("MessageResponse")
        assert "message" in contract
        src = _src(MAIN_SCHEMAS_TS)
        m = re.search(
            r"export const deleteCheckpointSchema = z\.object\(\{(.*?)\n\}\)", src, re.S
        )
        assert m, "deleteCheckpointSchema 不在场"
        assert not re.search(r"^  message\??:", m.group(1), re.M) or \
            "message: z.string()," in m.group(1), (
            "删断点那的回执把 message 写成了可选，与契约的必填不符")


# ---------------------------------------------------------------------------
# 面 3：门禁口径面
# ---------------------------------------------------------------------------


class TestTheGateDecision:
    """报告端点**挂着**门禁，且它是门禁面上唯一一只读操作"""

    def test_the_report_endpoint_is_on_the_gate_face(self):
        from tests.unit.test_api_key_channel_l233 import gated_operations

        gated = gated_operations()
        assert REPORT_OP in gated, sorted(gated)
        assert DELETE_OP in gated, sorted(gated)

    def test_it_is_the_only_gated_read(self):
        from tests.unit.test_api_key_channel_l233 import GATED_READ_EXCEPTION, gated_operations

        reads = {op for op in gated_operations() if op[0] == "GET"}
        assert reads == GATED_READ_EXCEPTION, sorted(reads)

    def test_the_standing_security_verdict_agrees(self):
        """仓里**早就有**这一条裁定：`test_api_security.py` 的 `PROTECTED` 名单里
        写着「报告含失败明细」⇒ 「报告端点属于密面」不是本轮的新发明，
        本轮只是把 l233 那句更宽的口径收窄到与它一致。"""
        from tests.integration.test_api_security import TestWriteRouteAuthCoverage

        assert REPORT_OP in TestWriteRouteAuthCoverage.PROTECTED
        assert DELETE_OP in TestWriteRouteAuthCoverage.PROTECTED


@needs_web_src
class TestTheRealHttpChain:
    """真打一次：门禁升起时，没带 key ⇒ 401；带了 key ⇒ 落到 404"""

    @pytest.fixture
    def client(self, tmp_path, monkeypatch):
        root = tmp_path / "ds"
        root.mkdir()
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        monkeypatch.setenv("AUGMENTOR_API_KEY", "l322-probe-key")
        monkeypatch.chdir(tmp_path)
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app)

    def test_without_the_key_it_is_401(self, client):
        resp = client.get("/api/augment/report/ghost_task")
        assert resp.status_code == 401, resp.text
        assert resp.json()["detail"] == "缺少或无效的 X-API-Key"

    def test_with_the_key_it_reaches_the_handler_and_says_404(self, client):
        resp = client.get("/api/augment/report/ghost_task",
                          headers={"X-API-Key": "l322-probe-key"})
        assert resp.status_code == 404, resp.text
        assert "报告不存在" in resp.json()["detail"]


# ---------------------------------------------------------------------------
# 面 4：非恒真面（注入臂）
# ---------------------------------------------------------------------------


@needs_web_src
class TestTheGuardCanStillFail:
    """注入臂：把判据的输入改坏，同一只判据必须当场喊红"""

    def test_a_wrong_path_in_the_service_layer_turns_the_wiring_face_red(self):
        code = strip_comments(_src(API_TS))
        doctored = code.replace("/augment/report/${taskId}", "/augment/reports/${taskId}")
        assert doctored != code, "锚点没找到 ⇒ 注入臂无效"
        assert not re.search(r"api\.get\(`/augment/report/\$\{[^}]+\}`\)", doctored), (
            "路径改错了，判据却没响 ⇒ 这只判据恒真")

    def test_a_dropped_zod_key_turns_the_contract_face_red(self):
        contract = contract_props("AugmentReportResponse")
        zod = zod_keys(_src(MAIN_SCHEMAS_TS), "augmentReportResponseSchema")
        doctored = zod - {"errors"}
        assert doctored != contract, sorted(doctored ^ contract)
