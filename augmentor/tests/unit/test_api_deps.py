# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""api.deps 依赖层单元测试

覆盖惰性 get_pipeline、reset_pipeline、同步/异步 JSON 读写、
require_file 校验与 run_in_thread 线程池执行。
"""

import ast
import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent
API_ROOT = AI_DIR / "api"


def api_source_files() -> list:
    """禁令面 = 整个 `api/` 包（递归，含 `main.py` / `deps.py` / `middleware/` / `routes/`）

    L180 那句裁定管的是「服务端代码不许把异常原文回进 500 响应体」，它的单位是**状态码**
    而不是某个目录里的某一行写法。旧尺子把扫描面写成 `api/routes/*.py`（非递归）＋
    单行字面量 `HTTPException(status_code=500, detail=str(`，于是折行写法、关键字换序、
    位置参数、`repr(...)`、f-string 内插 except 变量，以及 routes 之外新立的 500 支
    全都从读数面外面走过去 —— 那是「量不到」而不是「没有」。
    """
    return sorted(p for p in API_ROOT.rglob("*.py") if "__pycache__" not in p.parts)


def _call_args_by_position_and_keyword(call):
    """`HTTPException(500, "x")` 与 `HTTPException(status_code=500, detail="x")` 同一张表"""
    table = {}
    for index, node in enumerate(call.args[:2]):
        table[("status_code", "detail")[index]] = node
    for kw in call.keywords:
        if kw.arg in ("status_code", "detail"):
            table[kw.arg] = kw.value
    return table


def _httpexception_calls(tree):
    for node in ast.walk(tree):
        if (isinstance(node, ast.Call) and isinstance(node.func, ast.Name)
                and node.func.id == "HTTPException"):
            yield node


def _handler_names_around(call, handlers) -> set:
    """这个调用点被哪些 `except ... as 名字` 罩着（f-string 内插原文就是从这里漏的）"""
    return {h.name for h in handlers
            if h.name and any(node is call for node in ast.walk(h))}


def exception_text_sites(source: str, name: str = "<memory>") -> list:
    """所有「detail 取自异常原文」的 HTTPException 调用点 → `[(状态码, '名字:行号')]`

    只认有语法位置的读取（AST 节点），散文按构造进不来：注释与 docstring 里把禁令原文
    抄一遍不算一处违规，这也是为什么这一族判据不能靠放宽文本 `in line` 来扩面。
    泄漏形态两式：① `detail` 里出现 `str(...)` / `repr(...)` 调用；
    ② `detail` 引用了所在 `except X as e` 绑定的那个名字（`detail=f"...{e}"` 同罪）。
    """
    tree = ast.parse(source)
    handlers = [n for n in ast.walk(tree) if isinstance(n, ast.ExceptHandler)]
    sites = []
    for call in _httpexception_calls(tree):
        table = _call_args_by_position_and_keyword(call)
        status = table.get("status_code")
        code = status.value if isinstance(status, ast.Constant) else None
        detail = table.get("detail")
        if detail is None:
            continue
        leaked = False
        for node in ast.walk(detail):
            if (isinstance(node, ast.Call) and isinstance(node.func, ast.Name)
                    and node.func.id in ("str", "repr")):
                leaked = True
            elif isinstance(node, ast.Name) and node.id in _handler_names_around(call, handlers):
                leaked = True
        if leaked:
            sites.append((code, f"{name}:{call.lineno}"))
    return sites


def raw_500_leaks(source: str, name: str = "<memory>") -> list:
    """500 档那几处：禁令的判据（现量必须为空集）"""
    return [loc for code, loc in exception_text_sites(source, name) if code == 500]


@pytest.fixture(autouse=True)
def _isolate_pipeline_state():
    """每个测试前后重置全局管道单例，避免状态泄漏

    Yields:
        None
    """
    import api.deps as deps
    saved = deps._pipeline
    deps.reset_pipeline()
    yield
    deps._pipeline = saved


class TestLazyPipeline:
    def test_get_pipeline_lazy_init(self, tmp_path, monkeypatch):
        """_pipeline 为空时 get_pipeline 需惰性构造（离线可降级）"""
        import api.deps as deps

        monkeypatch.chdir(tmp_path)
        # CWD 无 config.yaml 时 load_config 走默认配置，不得抛异常
        pipeline = deps.get_pipeline()
        assert pipeline is not None
        assert pipeline is deps._pipeline

    def test_reset_pipeline(self):
        import api.deps as deps

        deps._pipeline = object()
        deps.reset_pipeline()
        assert deps._pipeline is None


class TestFileHelpers:
    def test_require_file_missing_raises_404(self, tmp_path):
        from fastapi import HTTPException

        import api.deps as deps

        with pytest.raises(HTTPException) as exc_info:
            deps.require_file(str(tmp_path / "nope.json"))
        assert exc_info.value.status_code == 404

    def test_require_file_existing_returns_path(self, tmp_path):
        import api.deps as deps

        f = tmp_path / "ok.json"
        f.write_text("[]", encoding="utf-8")
        assert deps.require_file(str(f)) == f

    def test_save_items_roundtrip(self, tmp_path):
        import api.deps as deps

        target = tmp_path / "sub" / "items.json"
        items = [{"instruction": "q", "input": "", "output": "a"}]
        deps.save_items(str(target), items)
        assert json.loads(target.read_text(encoding="utf-8")) == items

    def test_read_json_file_async(self, tmp_path):
        import asyncio

        import api.deps as deps

        f = tmp_path / "a.json"
        f.write_text(json.dumps([{"instruction": "q1"}, {"instruction": "q2"}]),
                     encoding="utf-8")

        async def run():
            return await deps.read_json_file(f)

        assert asyncio.run(run()) == [{"instruction": "q1"}, {"instruction": "q2"}]

    def test_read_json_file_async_applies_shape_gate(self, tmp_path):
        """异步读盘同样受形态校验约束

        原夹具写的是 ``[1, 2]`` 并断言原样返回 —— 那恰好把「非对象数据项也照收」
        这个缺陷语义钉住了：`read_json_file` 只是 `read_items` 的执行器包装，
        两者形态判据必须一致，否则异步路径会重新漏出 500。
        """
        import asyncio

        import api.deps as deps
        from fastapi import HTTPException

        f = tmp_path / "shape.json"
        f.write_text(json.dumps([1, 2]), encoding="utf-8")

        with pytest.raises(HTTPException) as exc:
            asyncio.run(deps.read_json_file(f))
        assert exc.value.status_code == 400
        assert "必须是 JSON 对象" in exc.value.detail

    def test_write_json_file_async(self, tmp_path):
        import asyncio

        import api.deps as deps

        f = tmp_path / "b.json"

        async def run():
            await deps.write_json_file(f, [{"x": 1}])

        asyncio.run(run())
        assert json.loads(f.read_text(encoding="utf-8")) == [{"x": 1}]

    def test_run_in_thread_executes_func(self):
        import asyncio

        import api.deps as deps

        result = asyncio.run(deps.run_in_thread(lambda: 40 + 2))
        assert result == 42


class TestConfigDataRootsMissingFile:
    def test_missing_config_file_skips_cache(self):
        """config 文件 stat 不了时走降级键：默认根照常返回，且不落缓存（L102，A202）

        `_config_data_roots` 的 docstring 口径：「文件不存在（stat 不了）时不缓存，
        免得把降级路径一起冻住」——若这一支把 ``(path, None)`` 也写进缓存，配置文件
        修好之前 mtime 永远不会跳变，降级结果会被冻住，缓存失效机制对它整体失明。
        """
        import api.deps as deps

        deps._config_roots_cache.clear()
        missing = Path("l102-definitely-not-present.yaml")
        roots = deps._config_data_roots(missing)
        assert len(roots) == 1 and roots[0].name == "data"
        assert deps._config_roots_cache.get((str(missing), None)) is None


class TestToHttpError500LeakClosed:
    """L179（A155①）：500 档回固定文案、不再转发异常原文（部署路径 / 文件系统状态）。

    判据钉三格：① 未分类异常（非 ValueError）⇒ 500 + 固定 ``INTERNAL_ERROR_DETAIL``，
    且部署绝对路径与 errno 文案**不得**再出现在响应体；② 原文改走服务端日志（ERROR 级）；
    ③ 400 档（ValueError 可行动文案）保持原样，防修过头把客户端可行动信息也抹掉。
    """

    def test_unclassified_error_returns_fixed_detail_not_str(self):
        import api.deps as deps

        exc = OSError(13, "Permission denied", "D:/deploy/sensitive/secret.json")
        err = deps.to_http_error(exc)
        assert err.status_code == 500
        assert err.detail == deps.INTERNAL_ERROR_DETAIL
        for leaked in ("D:/deploy/sensitive", "Permission denied", "Errno 13", "secret.json"):
            assert leaked not in err.detail, leaked

    def test_unclassified_error_goes_to_server_log(self, caplog):
        import api.deps as deps

        with caplog.at_level("ERROR", logger="api.deps"):
            deps.to_http_error(RuntimeError("boom-detail"))
        assert any("未分类异常收敛为 500" in rec.message for rec in caplog.records), \
            "500 档必须 logger.exception 落服务端日志"

    def test_value_error_still_400_with_actionable_detail(self):
        import api.deps as deps

        err = deps.to_http_error(ValueError("比例须在 [0,1] 区间"))
        assert err.status_code == 400
        assert err.detail == "比例须在 [0,1] 区间"


class TestRaiseInternalErrorL180:
    """L180（A155②）：路由裸 500 统一走 raise_internal_error——原文落日志、客户端拿固定文案。

    三格：① 助手 raise 的是 500 + 固定文案、异常原文不进 detail；② 原文进服务端日志；
    ③ **静态形状棘轮**——整个 `api/` 包里不得再出现「500 档把异常原文回进 detail」的写法
    （谁把某路由的收尾分支改回转发原文 ⇒ 当场红）。

    ③ 这一格在 guard-integrity 自查里被判**读数面不对**：旧尺子只扫 `api/routes/*.py`
    （非递归，25 只源文件里只看得到 16 只）且只认一行字面量
    `HTTPException(status_code=500, detail=str(`。折行、换序、位置参数、`repr(...)`、
    f-string 内插 except 变量，以及 `api/main.py` / `api/middleware/` 里新立的 500 支
    全在读数面之外 ⇒ 它绿是因为看不见，不是因为真没有。现在按禁令面（状态码 500 +
    原文来源）走 AST 扫描，负面对照同时在场（400/409 档回原文是 L179 定下的可行动文案，
    不许一起拦）。
    """

    def test_helper_raises_fixed_500_not_str(self, caplog):
        import api.deps as deps
        from fastapi import HTTPException

        with caplog.at_level("ERROR", logger="api.deps"):
            with pytest.raises(HTTPException) as ei:
                deps.raise_internal_error(OSError(13, "Permission denied", "D:/deploy/x.json"))
        assert ei.value.status_code == 500
        assert ei.value.detail == deps.INTERNAL_ERROR_DETAIL
        for leaked in ("D:/deploy/x.json", "Permission denied"):
            assert leaked not in ei.value.detail
        assert any("未分类异常收敛为 500" in r.message for r in caplog.records)

    def test_no_raw_500_detail_str_leak_in_routes(self):
        """禁令主判据：`api/` 全包 25 只源文件里，500 档的原文泄漏点恰好 0 处"""
        files = api_source_files()
        assert len(files) == 25, f"扫描面现量 25 只源文件，实得 {len(files)} 只 ⇒ 语料变了先在这里红"
        offenders = []
        for path in files:
            rel = path.relative_to(AI_DIR).as_posix()
            offenders += raw_500_leaks(path.read_text(encoding="utf-8"), rel)
        assert offenders == [], "裸 500 回异常原文（逐格点名）：\n" + "\n".join(offenders)

    def test_the_scanner_is_not_blind_on_the_allowed_statuses(self):
        """正控制：同一条判据摘掉状态码那一格，现量 10 处、逐文件点名

        这一支证明 ③ 的 0 是**量出来的 0**——判据确实读到了这批调用点，只是它们都是
        400/409 档（L179 拍定「可行动文案照回」）。哪天有人把其中一支的 400 改成 500，
        上一支当场点名它。
        """
        per_file = {}
        statuses = set()
        for path in api_source_files():
            rel = path.relative_to(AI_DIR).as_posix()
            sites = exception_text_sites(path.read_text(encoding="utf-8"), rel)
            for code, _ in sites:
                statuses.add(code)
            if sites:
                per_file[rel] = len(sites)
        assert per_file == {
            "api/deps.py": 3,
            # L277 / A367：`load_data` 门口那两判把 SDK `require_count` 的原话回成 400，
            # 用的是本仓既有的 `raise HTTPException(400, str(exc))` 形状（不是 500 档）。
            "api/routes/data.py": 1,
            "api/routes/dataset_tools.py": 1,
            "api/routes/multimodal.py": 1,
            "api/routes/quality.py": 3,
            "api/routes/system_ops.py": 1,
        }, f"原文回 detail 的调用点现量 10 处（3/1/1/1/3/1），实得 {per_file}"
        assert statuses == {400, 409}, f"现量只该有 400/409 两档，实得 {statuses} ⇒ 500 档混进来了"

    def test_injected_leak_forms_each_shout(self):
        """注入臂（内存文本，磁盘零写入）：旧字面量抓不到的形状，每一形都要点名喊红"""
        arms = {
            "一行字面量（旧尺子唯一认的那形）": (
                "raise HTTPException(status_code=500, detail=str(exc))", 1),
            "关键字换序并折行": (
                "def f():\n    raise HTTPException(\n        detail=str(exc),\n"
                "        status_code=500,\n    )\n", 2),
            "位置参数": ("def f():\n    raise HTTPException(500, str(exc))\n", 2),
            "repr 而不是 str": (
                "def f():\n    raise HTTPException(status_code=500, detail=repr(exc))\n", 2),
            "f-string 内插 except 绑定的名字": (
                "def f():\n    try:\n        read()\n    except OSError as err:\n"
                "        raise HTTPException(status_code=500, detail=f'内部错误：{err}')\n", 5),
        }
        for label, (source, lineno) in arms.items():
            got = raw_500_leaks(source, "arm")
            assert got == [f"arm:{lineno}"], f"{label} 没被抓到（实得 {got}）"

    def test_the_ban_does_not_swallow_prose_or_the_allowed_forms(self):
        """反面对照：散文里抄禁令原文不算违规，400 档与 `raise_internal_error` 不算违规

        第三格是**这一族判据为什么不能靠放宽文本 `in line` 来扩面**的实测：旧那种
        逐行字面量匹配在 `api/deps.py` 的 docstring 上就会误伤（那里就写着这句话），
        而 AST 读数按构造只看调用点的实参。
        """
        allowed = [
            "raise HTTPException(status_code=400, detail=str(exc))",
            "raise HTTPException(status_code=500, detail=INTERNAL_ERROR_DETAIL)",
            "def helper():\n    \"\"\"收尾分支别再写 HTTPException(status_code=500, "
            "detail=str(e))\"\"\"\n    return 1\n",
            "try:\n    run()\nexcept OSError as err:\n    raise_internal_error(err)\n",
        ]
        for source in allowed:
            assert raw_500_leaks(source) == [], f"允许的形状被判红了：{source!r}"
        # 散文那一形，旧的逐行字面量尺子确实会误伤（写下差异，免得日后退回文本匹配）
        prose = allowed[2]
        assert "HTTPException(status_code=500, detail=str(" in prose
        assert raw_500_leaks(prose) == []
