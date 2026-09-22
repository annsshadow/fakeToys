"""L72：__main__ 入口守卫（cli.py / api/main.py）

以 runpy 的 __main__ 身份执行两个入口模块，命中 `if __name__ == "__main__"`
块内的语句；api/main.py 注入假 uvicorn 阻止真实服务启动，
cli.py 无子命令调用会打印帮助并 exit(1)，二者均被捕获。
"""

import runpy
import sys
import types
from pathlib import Path

AI_DIR = Path(__file__).resolve().parent.parent.parent


class TestCliMainGuard:
    def test_cli_main_block_executes(self, monkeypatch):
        old_argv = sys.argv
        sys.argv = ["cli"]  # 无子命令 → print_help + sys.exit(1)
        try:
            try:
                runpy.run_path(str(AI_DIR / "cli.py"), run_name="__main__")
                raised = None
            except SystemExit as e:
                raised = e
        finally:
            sys.argv = old_argv
        # 入口块内的 main() 被执行，无子命令应以码 1 退出
        assert raised is not None and raised.code == 1


class TestApiMainGuard:
    def test_api_main_block_runs_fake_uvicorn(self, monkeypatch):
        import io
        from contextlib import redirect_stdout

        called = {}

        class _FakeUvicorn:
            def run(self, app, *args, **kwargs):
                called["ran"] = True

        fake = types.ModuleType("uvicorn")
        fake.run = _FakeUvicorn().run
        monkeypatch.setitem(sys.modules, "uvicorn", fake)

        out = io.StringIO()
        try:
            with redirect_stdout(out):
                runpy.run_path(str(AI_DIR / "api" / "main.py"), run_name="__main__")
        finally:
            pass
        # 入口块内的 uvicorn.run(app, ...) 被调用（假实现，无阻塞）
        assert called.get("ran") is True
