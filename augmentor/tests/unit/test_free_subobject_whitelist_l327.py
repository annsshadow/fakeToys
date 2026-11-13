# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L327（A278 收口）：A277 那把尺子加一档「**自由子对象白名单**」

**A278 记的是什么**：「OpenAPI 里 `additionalProperties: true` 的自由子对象 …—
契约面不是全知的。响应模型里凡是 `Dict[str, Any]` 的子对象，在 OpenAPI 上就是
`additionalProperties: true` —— **键名压根不在 schema 里**，机器推不出来」。

**为什么文档会写错、而机器判不出来**：A277 那把尺子有五档证据（契约面 / 字面量 / 符号 /
配置键 / CLI 子命令）。自由子对象的键**只能靠散文说明**，而它们今天多半是被**字面量档**
接住的 —— 那一档是**逐格限作用域**的（路由文件 + 两跳 import 闭包）⇒
**一旦某个键来自更深的模块（超过两跳），它就会被报成无据，而那不是文档错**
（实测形状：`ImpactResponse.before` / `after` 的键来自 SDK 那份统计画像）。

**A278 选②**：**不给自由子对象立最小形状**（改响应模型是对外契约变化）⇒
而是由这把尺子加一档「自由子对象白名单」，**把「这一格只能靠散文」写成一份可证伪的名单**
而不是含糊过去。名单按 **(端点, 自由子对象)** 逐格登记那些**确实被文档写出来**的键。

**这份名单的契约（三条，各自都有牙齿）**：
- **① 不许陈旧**：名单里的每一项都必须真的出现在该端点的 `API.md` 那一格里
  （文档删了它 ⇒ 名单当场红 ⇒ 不留「早已不存在的白名单」）。
- **② 不许给非自由的东西开路**：名单所在的 (端点, 自由子对象) 在契约里**必须真的自由**
  （`additionalProperties: true`，或其数组项如此）⇒ 否则「用白名单掩盖一个契约里明明
  有名字的字段」当场红。
- **③ 白名单**只**增加**一档证据，**不豁免别的**：名单外的键，证据档仍是原来那五档 ⇒
  文档写漏照样红。而名单里的键**不再依赖代码的局部性** ⇒ 这正是 A278 要解决的那一格
  （「键来自更深的模块时被报成无据」）。

**零 `parametrize`**（仓里「新守卫一律零 parametrize」的惯例 ⇒ l97 / l98 两把
参数化普查尺的读数一格未动）。
"""

import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))


#: 自由子对象白名单：`(方法, 路径) → {自由子对象名: 那一格里被文档写出来的键}`
#:
#: **只登记「文档确实写了」的键**（契约①）—— 白名单不是「这一节什么都能写」的通行证，
#: 而是一份「这些键名在这份文档里是这么说的」的逐格登记。
WHITELIST = {
    ("POST", "/api/dataset/impact"): {
        # `ImpactResponse.before` / `after` = SDK 那份统计画像的原始 dict
        # （`api/routes/dataset_tools.py` 声明为 `Dict[str, Any]`）⇒ 键名不在 schema 里。
        "before": ("duplicate_rate", "unique_instructions"),
        "after": ("duplicate_rate", "unique_instructions"),
    },
    ("POST", "/api/dataset/aggregate"): {
        # `AggregateResponse.conflicts` 是**数组**，项是自由字典（`additionalProperties: true`）
        # ⇒ 文档写的那一项 `{key, values, sources, same_source}` 里，
        # 只有 `same_source` 是「像字段名」的反引号片段（其余三个要么太短、要么与别的键
        # 写在同一个反引号片段里 ⇒ 尺子不收）。
        "conflicts": ("same_source",),
    },
}


def _openapi():
    from api.main import app

    return app.openapi()


def _response_model(method, path):
    """该端点的 200 响应模型名（找不到就 None）"""
    spec = _openapi()
    op = (spec.get("paths", {}).get(path) or {}).get(method.lower())
    if not op:
        return None
    for code, resp in (op.get("responses") or {}).items():
        if not str(code).startswith("2"):
            continue
        for media in (resp.get("content") or {}).values():
            ref = ((media.get("schema") or {}).get("$ref") or "")
            if ref:
                return ref.rsplit("/", 1)[-1]
    return None


def free_fields(method, path):
    """该端点响应里的**自由子对象**字段名 → `{"字段名": "free" | "free-items"}`

    两种形状都算自由：
    - `free`：`Dict[str, Any]` ⇒ OpenAPI 上 `additionalProperties: true`；
    - `free-items`：`List[Dict[str, Any]]` ⇒ 数组项 `additionalProperties: true`
      （**项**是自由的 ⇒ 键名同样不在 schema 里）。
    """
    spec = _openapi()
    model = _response_model(method, path)
    if not model:
        return {}
    schema = (spec.get("components", {}).get("schemas", {}) or {}).get(model) or {}
    out = {}
    for field, sub in (schema.get("properties") or {}).items():
        if not isinstance(sub, dict):
            continue
        if sub.get("additionalProperties") is True:
            out[field] = "free"
        elif sub.get("type") == "array":
            items = sub.get("items") or {}
            if isinstance(items, dict) and items.get("additionalProperties") is True:
                out[field] = "free-items"
    return out


def whitelist_evidence(method, path, token):
    """该 token 在这一格里有没有「自由子对象白名单」那一档证据"""
    entry = WHITELIST.get((method, path)) or {}
    for keys in entry.values():
        if token in keys:
            return True
    return False


class TestTheRosterMatchesTheContract:
    """契约① 不许陈旧 + 契约② 不许给非自由的东西开路"""

    def test_the_free_field_is_really_free(self):
        """契约②：名单所在的 (端点, 自由子对象) 在契约里**必须真的自由**"""
        for (method, path), fields in WHITELIST.items():
            frees = free_fields(method, path)
            for field in fields:
                assert field in frees, (
                    "%s %s 的 %s 在契约里**不是**自由子对象（现量 %s）⇒ 白名单不许给它开路"
                    % (method, path, field, sorted(frees)))

    def test_every_rostered_key_is_actually_written_down(self):
        """契约①：名单里的每一项都必须真的出现在该端点的 `API.md` 那一格里"""
        from tests.unit.test_api_doc_field_hints_l223 import hint_rows

        rows = {(m, p): text for m, p, text in hint_rows()}
        for (method, path), fields in WHITELIST.items():
            text = rows.get((method, path))
            assert text is not None, "API.md 里没有 %s %s 那一行" % (method, path)
            for field, keys in fields.items():
                for key in keys:
                    assert "`%s`" % key in text, (
                        "%s %s 的白名单项 %s.%s 在文档里已经不存在 ⇒ 名单陈旧"
                        % (method, path, field, key))


class TestTheWhitelistIsAnIndependentChannel:
    """契约③：名单里的键**不再依赖字面量档的作用域** —— 这正是 A278 要解决的那一格"""

    def test_the_rostered_keys_survive_a_narrowed_literal_scope(self):
        """把字面量档的作用域收窄到**一跳** ⇒ 名单里的键**仍有据**（靠白名单这一档）"""
        from tests.unit import test_api_doc_field_hints_l223 as ruler

        original = ruler.closure_of
        try:
            ruler.closure_of = lambda route_file, hops=2: original(route_file, hops=1)
            for (method, path), fields in WHITELIST.items():
                for keys in fields.values():
                    for key in keys:
                        kinds = ruler.evidence_for(method, path, key)
                        assert "free" in kinds, (
                            "%s %s 的 %s 在字面量档收窄后没了白名单这一档 ⇒ "
                            "A278 要解决的那一格（键来自更深的模块时被报成无据）没解决"
                            % (method, path, key))
        finally:
            ruler.closure_of = original

    def test_a_key_outside_the_roster_is_not_rescued(self):
        """白名单**不豁免别的**：名单外的键，证据档仍是原来那五档"""
        from tests.unit import test_api_doc_field_hints_l223 as ruler

        for (method, path), fields in WHITELIST.items():
            rostered = {key for keys in fields.values() for key in keys}
            for token in sorted(ruler.tokens_in(
                    next(t for m, p, t in ruler.ROWS if (m, p) == (method, path)))):
                if token in rostered:
                    continue
                assert not whitelist_evidence(method, path, token), (
                    "%s %s 的 %s 不在白名单里却得了白名单证据" % (method, path, token))
