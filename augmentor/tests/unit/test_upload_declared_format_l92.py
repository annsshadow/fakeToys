# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L92：A162 前端上传面收口 —— 显式声明发得出去，失败文案说得出声

改前实测（`git show HEAD:./web/src/pages/DataManagement.tsx` / 同 `api.ts`，L92 取证）：

| 事实 | 改前 | 改后 |
| --- | --- | --- |
| `api.ts` 里 `input_format` 出现次数 | **0** | append 进 FormData，无条件 |
| 页面 `catch {` 的条数 | **7**（全部只 `message.error('…失败')`） | 0，七处都过 `apiErrorDetail` |
| 「显式声明 > 扩展名」这一档在 UI 的可达性 | **不可达**（选不到也发不出） | 下拉框 13 项 + 「自动」 |

L91 把端点做成了「认 `.csv`、认显式 `input_format`」，但能力在产品面只走完一半：
页面从不声明格式，也无条件把服务端的判决文案吞成一句「上传失败」。本文件钉的是**这一半**，
它与 `test_upload_table_formats_l91.py` 的分工是：那一档管「后端收不收、落不落盘」，
这一档管「前端发什么、说什么」。

判据分三层：

1. **跨语言漂移**（`TestDeclaredFormatChoicesDrift`）：页面那份 `UPLOAD_FORMAT_CHOICES`
   字面量必须与 `converter.INPUT_FORMAT_CHOICES` **逐项相等**。权威直接 import Python 常数，
   与 L91 的 `accept` 守卫同一口径 —— 搬在 Python 侧而不是 vitest 侧，因为本仓 `web/@types`
   里没有 node，跨语言读文件在 jsdom 侧要先动依赖表（L91 实测两次失败的经验）。
2. **接线形状**（`TestUploadFaceWiring`）：`api.ts` 真的把声明放进 FormData、页面真的
   把下拉框接到 `uploadData`、`beforeUpload` 真的用包装箭头（否则 antd 的第二个实参
   `fileList` 会被当成格式发出去）。文本扫描的局限如实说：不证明渲染结果。
3. **端点面回执**（`TestEndpointAcceptsWhatTheUiSends`）：UI 发的那三种形状（空串声明、
   真声明、非法声明）在真实 multipart + 真实中间件链下的行为，其中非法声明必须回
   **字符串** detail ⇒ 前端 `apiErrorDetail` 的主支不是想象出来的形状。

跑法：`python -m pytest tests/unit/test_upload_declared_format_l92.py -q --no-cov`
"""

import io
import json
import pathlib
import re

import pytest

from augmentor.converter import INPUT_FORMAT_CHOICES

SRC = pathlib.Path(__file__).resolve().parents[2] / "web" / "src"
PAGE_TSX = SRC / "pages" / "DataManagement.tsx"
API_TS = SRC / "services" / "api.ts"

needs_web_src = pytest.mark.skipif(
    not (PAGE_TSX.is_file() and API_TS.is_file()),
    reason="前端源码不在场（只带构建产物的安装没有 web/）",
)

ROWS = [{"instruction": "问1", "input": "", "output": "答1"},
        {"instruction": "问2", "input": "", "output": "答2"}]
CSV_BYTES = "instruction,input,output\n问1,,答1\n问2,,答2\n".encode("utf-8")
ALPACA_BYTES = b"\n".join(
    json.dumps({"instruction": r["instruction"], "input": r["input"],
                "output": r["output"]},
               ensure_ascii=False).encode("utf-8")
    for r in ROWS
)

CHOICES_RE = re.compile(r"const UPLOAD_FORMAT_CHOICES = \[(.*?)\]", re.S)
QUOTED_RE = re.compile(r"'([^']+)'")
BEFORE_UPLOAD_RE = re.compile(r"beforeUpload=\{([^}]*)\}")

# 「把后端判决端给用户」这一族的包装器。A167 新增了 `bulkErrorDetail`（只给超时那一支
# 换文案，其余仍委托 `apiErrorDetail`），所以下面两条扫描认的是**这一族**而不是一个名字；
# 认了就必须坐实它确实还在委托 —— 见 `test_bulk_wrapper_delegates_to_detail_helper`。
DETAIL_WRAPPERS = ("apiErrorDetail", "bulkErrorDetail")
_WRAPPED_RE = re.compile(
    r"message\.error\((?:%s)\(err," % "|".join(DETAIL_WRAPPERS)
)
_NAKED_RE = re.compile(
    r"message\.error\(\s*(?!(?:%s))" % "|".join(DETAIL_WRAPPERS)
)


def strip_comments(source: str) -> str:
    """剥掉块注释与 `//` 行注释，只留代码

    **本档必须这么做**（实测教训）：页面里那段解释「为什么 `beforeUpload` 要包一层箭头」
    的注释**原样写着** `beforeUpload={(file) => handleUpload(file)}`，纯文本扫描的第一次
    命中就是它 —— 于是把 JSX 改回直传 `beforeUpload={handleUpload}` 时守卫依然绿，
    判据被自己的案文骗过去了。`web/src/services/api.wiring.test.ts` 早就带同名工具，
    口径一致（注释里提到名字不算已接线）。
    """
    return re.sub(r"/\*[\s\S]*?\*/", "", re.sub(r"^[ \t]*//.*$", "", source, flags=re.M))


@pytest.fixture
def api_env(tmp_path, monkeypatch):
    """白名单收到 `tmp_path/ds`、工作目录 `tmp_path`（与 L91 同形，防磁盘产物改判）"""
    root = tmp_path / "ds"
    root.mkdir()
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root, tmp_path


def post_upload(http, filename, content, input_format=None):
    files = {"file": (filename, io.BytesIO(content), "application/octet-stream")}
    data = {} if input_format is None else {"input_format": input_format}
    return http.post("/api/data/upload", files=files, data=data)


@needs_web_src
class TestDeclaredFormatChoicesDrift:
    """页面那份可选格式的字面量不许与读边权威表漂移（双向）"""

    @staticmethod
    def _choices() -> list:
        page = strip_comments(PAGE_TSX.read_text(encoding="utf-8"))
        match = CHOICES_RE.search(page)
        assert match, "DataManagement 里没有 UPLOAD_FORMAT_CHOICES 常量 ⇒ 判据要跟着换形状"
        return QUOTED_RE.findall(match.group(1))

    def test_the_choice_list_is_scanned_and_not_vacuous(self):
        """扫描本身要有效：只抠到一两项说明正则在空转，下面的相等断言就是摆设"""
        choices = self._choices()
        assert len(choices) >= 10, choices
        assert len(choices) == len(set(choices)), f"页面里有重复项: {choices}"

    def test_nothing_is_offered_that_the_read_side_cannot_parse(self):
        """UI 选得到而读边认不出 ⇒ 选了必 400，等于骗人"""
        choices = self._choices()
        extra = [name for name in choices if name not in INPUT_FORMAT_CHOICES]
        assert not extra, f"下拉框放开这些格式，但读边不认: {extra}"

    def test_every_read_side_format_can_be_declared(self):
        """读边认的格式在 UI 声明不到 ⇒ 那一档能力对用户不存在（A162 ① 的形状）"""
        choices = self._choices()
        missing = [name for name in INPUT_FORMAT_CHOICES if name not in choices]
        assert not missing, f"读边认这些格式，但页面选不到: {missing}"

    def test_the_order_matches_the_authority_table(self):
        """只比顺序不比内容：加一种源格式时页面应当**跟着动**，而不是各自排序凑巧相等"""
        assert self._choices() == list(INPUT_FORMAT_CHOICES)


@needs_web_src
class TestUploadFaceWiring:
    """接线：声明真的进 FormData、错误文案真的出声、`beforeUpload` 真的用包装箭头"""

    @staticmethod
    def _page() -> str:
        return strip_comments(PAGE_TSX.read_text(encoding="utf-8"))

    @staticmethod
    def _api() -> str:
        return strip_comments(API_TS.read_text(encoding="utf-8"))

    def test_the_comment_strip_is_what_makes_the_scan_trustworthy(self):
        """非空转元断言：页面**确实**有一段注释原样写着被扫的那个写法

        这一条是本轮变异取证抓出来的：M2 把 JSX 改回 `beforeUpload={handleUpload}`
        时守卫没红，因为第一次命中的是那段解释「为什么要包一层」的注释。若哪天注释
        被改写、这条就失去意义 ⇒ 它会先响，提醒判据的形状变了。
        """
        raw = PAGE_TSX.read_text(encoding="utf-8")
        assert len(BEFORE_UPLOAD_RE.findall(raw)) >= 2, "案文里已不再出现该写法，本条可删"
        assert len(BEFORE_UPLOAD_RE.findall(self._page())) == 1

    def test_service_layer_appends_the_declaration(self):
        api_src = self._api()
        assert "formData.append('input_format', inputFormat)" in api_src, \
            "api.ts 没把 input_format 放进 FormData ⇒ 显式声明这一档发不出去"

    def test_upload_data_takes_an_optional_format(self):
        """签名里有第二个参数：它是「默认按扩展名」的旧行为与新声明之间唯一的桥"""
        assert re.search(r"export const uploadData = async \(\s*file: File,\s*inputFormat = ''",
                         self._api()), "uploadData 的签名变了 ⇒ 本档判据要跟着换形状"

    def test_the_page_passes_the_declared_format_into_the_request(self):
        page = self._page()
        assert "uploadData(file, format)" in page, "页面没把格式声明交给服务层"
        assert "format = uploadFormat" in page, "handleUpload 的默认值没接上下拉框状态"

    def test_before_upload_uses_a_wrapping_arrow(self):
        """antd 的 `beforeUpload(file, fileList)` 会把第二个实参也传进去

        直接写 `beforeUpload={handleUpload}` 时，`fileList` 会顶掉 `format` 形参，
        `FormData.append` 把数组 coerce 成字符串发出去 ⇒ 每次上传都 400。
        这条断言钉的就是那个「看着等价、其实不等价」的写法。
        """
        match = BEFORE_UPLOAD_RE.findall(self._page())
        assert match, "Upload 控件没有 beforeUpload 属性 ⇒ 判据要跟着换形状"
        assert match == ["(file) => handleUpload(file)"], match

    def test_the_select_is_bound_to_the_state(self):
        page = self._page()
        assert "value={uploadFormat}" in page
        assert "setUploadFormat(e.target.value)" in page, "下拉框没接到状态"
        assert '<option value="">自动（按扩展名）</option>' in page, \
            "少了「自动」那档：用户一旦不能声明回退，就等于强制声明"

    def test_every_error_toast_surfaces_the_server_detail(self):
        """A162 ②：这个页面里**没有**一处 `message.error('字面量')`

        判据写成「两头的数量相等」而不是「至少 N 处」，所以新加的 `catch` 忘了包
        判决包装器会当场变红；非空转由 `>= 6` 那条兜住（本页面有七处 catch）。
        认的是 `DETAIL_WRAPPERS` 这一族（A167 起有 `bulkErrorDetail`）。
        """
        page = self._page()
        total = len(re.findall(r"message\.error\(", page))
        wrapped = len(_WRAPPED_RE.findall(page))
        assert total >= 6, f"只扫到 {total} 处 message.error，扫描可能已空转"
        assert wrapped == total, f"{total - wrapped} 处错误提示没走判决包装器"

    def test_no_error_toast_anywhere_swallows_the_server_detail(self):
        """A162 ② 的**同构面**：全仓 `pages/` + `components/` 一处不剩

        `apiErrorDetail` 一就位，「吞掉判决」就成了一个可以机械扫描判负的形状，所以守卫
        不做「本页面」而做「所有 UI 源文件」。本轮清扫的实测规模：14 个文件 / 35 处
        （24 处 `} catch {` 块 + 11 处 `.catch(() => …)` 箭头），清扫后未包装数 = 0。
        非空转由「至少扫到 35 处」兜住：扫不到就说明形状换了，本条要先跟着换。
        豁免名单只有 `DETAIL_WRAPPERS`，而它每一员都必须自己落到 `apiErrorDetail`
        （下一条守卫盯这个），否则「加一个壳」就成了绕过本判据的后门。
        """
        files = [p for p in sorted((SRC / "pages").glob("*.tsx"))
                 + sorted((SRC / "components").glob("*.tsx")) if ".test." not in p.name]
        sources = {p.name: strip_comments(p.read_text(encoding="utf-8")) for p in files}
        total = sum(len(re.findall(r"message\.error\(", text)) for text in sources.values())
        naked = {
            name: len(_NAKED_RE.findall(text)) for name, text in sources.items()
        }
        assert total >= 35, f"全仓只扫到 {total} 处 message.error，扫描可能已空转"
        bad = {name: n for name, n in naked.items() if n}
        assert not bad, f"这些文件里的错误提示没走判决包装器: {bad}"

    def test_bulk_wrapper_delegates_to_detail_helper(self):
        """A167 那层壳不许变成新的吞判决口子

        上面两条扫描认了 `bulkErrorDetail`，前提是它在「非超时」那一支仍然落到
        `apiErrorDetail`。哪天有人把那句委托删了，4xx 的判决就又只剩一句字面量。
        """
        src = API_TS.read_text(encoding="utf-8")
        _, _, body = src.partition("export const bulkErrorDetail")
        assert body, "api.ts 里的 bulkErrorDetail 没了 ⇒ 上面两条扫描的豁免名单要跟着收口"
        assert "apiErrorDetail(error, fallback)" in body[:400], \
            "bulkErrorDetail 不再委托 apiErrorDetail ⇒ 它成了吞掉后端判决的新壳"

    def test_the_service_layer_exports_the_detail_helper(self):
        assert "export const apiErrorDetail" in API_TS.read_text(encoding="utf-8")


class TestEndpointAcceptsWhatTheUiSends:
    """端点回执：UI 实际会发的三种 `input_format` 形状，各自的行为与文案形状"""

    def test_empty_declaration_is_the_same_as_no_declaration(self, api_env):
        """无条件 append 的代价核对：空串必须与「不传」完全同路（L91 口径 `Form("")`）"""
        http, root, _ = api_env
        sent = post_upload(http, "train.csv", CSV_BYTES, input_format="")
        omitted = post_upload(http, "train2.csv", CSV_BYTES)
        assert sent.status_code == omitted.status_code == 200, (sent.text, omitted.text)
        assert json.loads((root / "train.json").read_text(encoding="utf-8")) == ROWS
        assert json.loads((root / "train2.json").read_text(encoding="utf-8")) == ROWS

    def test_declaration_rescues_a_name_the_extension_cannot_help(self, api_env):
        """浏览器只给裸文件名 ⇒ 「无扩展名的表格」是 UI 上最常见的一档，只能靠声明救"""
        http, root, _ = api_env
        response = post_upload(http, "mystery", CSV_BYTES, input_format="csv")
        assert response.status_code == 200, response.text
        assert json.loads((root / "mystery.json").read_text(encoding="utf-8")) == ROWS

    def test_a_schema_name_is_uploadable_not_only_a_container(self, api_env):
        """alpaca 这类 schema 名在上传面同样可用（改前它连 `.jsonl` 都进不来）"""
        http, root, _ = api_env
        response = post_upload(http, "train.txt", ALPACA_BYTES, input_format="alpaca")
        assert response.status_code == 200, response.text
        assert json.loads((root / "train.json").read_text(encoding="utf-8")) == ROWS

    def test_bad_declaration_detail_is_a_string_the_ui_can_show(self, api_env):
        """前端主支的形状凭据：detail 必须是**字符串**而不是对象/数组

        `apiErrorDetail` 只有两支（字符串、422 数组）。本条钉住「非法声明」这一档落
        在第一支，且文案里带着可选项 ⇒ 用户看到的那句确实可 actions。
        """
        http, root, _ = api_env
        response = post_upload(http, "x.csv", CSV_BYTES, input_format="xml")
        assert response.status_code == 400
        detail = response.json()["detail"]
        assert isinstance(detail, str), type(detail)
        assert "input_format" in detail and "alpaca" in detail
        assert list(root.glob("*")) == []

    def test_a_missing_file_is_422_and_its_detail_is_the_array_the_ui_handles(self, api_env):
        """跨语言形状凭据：422 的 detail 确实是 `[{msg: 字符串}]`，vitest 那支不是想象"""
        http, root, _ = api_env
        response = http.post("/api/data/upload", data={"input_format": ""})
        assert response.status_code == 422, response.text
        detail = response.json()["detail"]
        assert isinstance(detail, list) and detail, type(detail)
        assert all(isinstance(item.get("msg"), str) for item in detail)
        assert list(root.glob("*")) == []
