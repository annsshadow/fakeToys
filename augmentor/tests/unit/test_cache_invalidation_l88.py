# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L88：写入面显式作废内容类配置缓存（关闭 A156 的「本进程那一半」）

取证读数（`Temp/l88q/invalidation_cost.py`，两解释器各跑一遍，200 轮中位数）：

- `invalidate_config_caches()` 本体 **0.0001 ms**（两次 `dict.clear()`）。
- 它换来的一次重解析：**7.34–8.13 ms**（`data_roots` 与 `max_upload_bytes` 两读边），
  而缓存命中只要 **0.005 ms** —— 也就是说「作废」的真实代价是那一次重解析，
  不是那次 `clear()`。
- 关键在于：**`POST /api/config` 这条路径本来就要重解析**。它末尾的 `save_config`
  实测 12.0–22.5 ms（本机 mtime tick 0.5 ms 的 24–45 倍），连续两次 `save_config`
  的 mtime 落在同一个 tick 的概率 **0/300**（两个解释器各 300 轮，都是 0）⇒
  作废动作在这条路上**没有多付一次重解析**，它只是把「下一次读必然重解析」这件事
  从「由操作系统时钟决定」改成「由代码决定」。
- 由此纠正上一轮 A156 里的一个**口径错**：那条写的「连续两次写入落同一 tick 的概率
  61–65%」量的是测试里连续的 `write_text`（单次几微秒），不是产品写配置的
  `save_config`。生产这条路上那个概率实测是 0，剩下的窗口来自「写得很快的别人」
  和「不改 mtime 的别人」（另立 A158），不是来自这里。

**L89 改的是本文件自己的载荷**：下面四条写面用例从前 POST 的是 `variants_per_item`
——**`AugmentationConfig` 没有这个字段**（真名 `variants_per_seed`），于是它们测的其实是
「一次什么都没写成的 POST」。它们当时能全绿，靠的正是 A152① 那一格：路由末尾无条件
`save_config`。A152① 落地之后这四条立刻变红，而红得对 —— 一条声称「写成功后要作废」
的用例必须先真的写成功。⇒ 与 L88 那两条变异探针同一族：**用例能失败还不够，还要问它
失败的是不是自己主张的那件事**。
"""

import inspect
import os
import re
import sys
from pathlib import Path

import pytest
import yaml

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor.config import load_config  # noqa: E402

SHIPPED_YAML = REPO / "config.yaml"


@pytest.fixture(autouse=True)
def _clean_content_caches():
    """内容类缓存前后各清一次：这些是模块级字典，跨用例带读数会互相骗"""
    import api.deps as deps

    deps._config_roots_cache.clear()
    deps._config_upload_limit_cache.clear()
    yield
    deps._config_roots_cache.clear()
    deps._config_upload_limit_cache.clear()


@pytest.fixture
def cfg(tmp_path, monkeypatch):
    """把服务配置指到临时目录里的一份可控 YAML（初始为「宽白名单 + 上限 4096」）

    两个键都要能改，是因为它们**共用同一个缓存键形状**（A156 的同构那一格）；
    只测一个键就等于没测「同构」。
    """
    import api.deps as deps

    path = tmp_path / "config.yaml"
    broad = tmp_path / "broad"
    narrow = tmp_path / "narrow"
    broad.mkdir()
    narrow.mkdir()
    (broad / "x.json").write_text("[]", encoding="utf-8")

    def write(roots, limit):
        path.write_text(
            yaml.safe_dump(
                {"web": {"data_roots": [str(r) for r in roots], "max_upload_bytes": limit}},
                allow_unicode=True,
            ),
            encoding="utf-8",
        )
        return path

    write([broad], 4096)
    monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
    monkeypatch.setattr(deps, "config_file_path", lambda: path)
    return deps, path, write, broad, narrow


@pytest.fixture
def client(tmp_path, monkeypatch):
    """`POST /api/config` 的取证房：chdir 到临时目录并逐字节复制出厂配置

    与 L87 那间同一个形状：写面的判决与产品同源，失败路径可以断言缓存没被碰。
    """
    monkeypatch.chdir(tmp_path)
    pristine = SHIPPED_YAML.read_bytes()
    (tmp_path / "config.yaml").write_bytes(pristine)
    import api.deps as deps
    from fastapi.testclient import TestClient
    from api.main import app

    instance = type("PipelineStub", (), {})()
    instance.config = load_config(str(SHIPPED_YAML))
    monkeypatch.setattr(deps, "_pipeline", instance)
    monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
    return TestClient(app), tmp_path


def _ghost(tmp_path):
    """一条**绝不可能来自磁盘配置**的白名单项：用来证明读边确实走的是缓存"""
    return tmp_path / "ghost_dir_that_never_exists"


class TestThisProcessSeesItsOwnWrite:
    """核心判据：同 tick 的改写对本进程**必须**可见 —— 这是 A156 收掉的那一半"""

    def test_same_tick_rewrite_is_invisible_without_invalidation(self, cfg):
        """先钉住缺陷本身：mtime 不动 ⇒ 缓存按设计看不见那次改写

        这条不是 endorsing，是 A156 立案的形状；下一条约它作废。
        """
        deps, path, write, broad, narrow = cfg
        assert deps.allowed_data_roots() == [broad]
        first = path.stat().st_mtime_ns
        write([narrow], 4096)
        os.utime(path, ns=(path.stat().st_atime_ns, first))
        assert deps.allowed_data_roots() == [broad], "mtime_ns 未变 ⇒ 旧边界仍在放行"

    def test_invalidation_makes_the_same_tick_rewrite_visible(self, cfg):
        deps, path, write, broad, narrow = cfg
        assert deps.allowed_data_roots() == [broad]
        first = path.stat().st_mtime_ns
        write([narrow], 4096)
        os.utime(path, ns=(path.stat().st_atime_ns, first))
        deps.invalidate_config_caches()
        assert deps.allowed_data_roots() == [narrow]

    def test_tightening_the_whitelist_reaches_the_gate(self, cfg):
        """不是「列表等于什么」，而是「闸放不放行」：危险方向是收紧被延迟放行

        作废之前，`broad/x.json` 仍被旧白名单放行；作废之后同一次调用 403。
        """
        deps, path, write, broad, narrow = cfg
        assert deps.resolve_within_roots("x.json", "文件路径") == broad / "x.json"
        first = path.stat().st_mtime_ns
        write([narrow], 4096)
        os.utime(path, ns=(path.stat().st_atime_ns, first))
        assert deps.resolve_within_roots("x.json", "文件路径") == broad / "x.json"
        deps.invalidate_config_caches()
        with pytest.raises(deps.HTTPException) as excinfo:
            deps.resolve_within_roots("x.json", "文件路径")
        assert excinfo.value.status_code == 403

    def test_limit_shares_the_same_key_shape_and_the_same_fix(self, cfg):
        """同构那一格：`max_upload_bytes` 用完全一样的键，也必须一起被作废覆盖"""
        deps, path, write, broad, _narrow = cfg
        assert deps.max_upload_bytes() == 4096
        first = path.stat().st_mtime_ns
        write([broad], 8192)
        os.utime(path, ns=(path.stat().st_atime_ns, first))
        assert deps.max_upload_bytes() == 4096
        deps.invalidate_config_caches()
        assert deps.max_upload_bytes() == 8192


class TestTheWriteFaceCallsIt:
    """`POST /api/config` 成功后必须作废；失败时不许作废"""

    def test_successful_post_empties_both_content_caches(self, client):
        import api.deps as deps

        http, _tmp_path = client
        assert deps.allowed_data_roots() and deps.max_upload_bytes()
        assert deps._config_roots_cache and deps._config_upload_limit_cache

        response = http.post("/api/config", json={"augmentation": {"variants_per_seed": 3}})
        assert response.status_code == 200
        assert deps._config_roots_cache == {}
        assert deps._config_upload_limit_cache == {}

    def test_successful_post_drops_a_same_mtime_stale_entry(self, client, monkeypatch):
        """写盘动作**即使没改 mtime**，本进程的缓存也必须作废

        这条是变异探针逼出来的形状。原写法（不 patch `save_config`，只摆一条旧值）
        在把路由里那句作废删掉之后**仍然绿** —— 因为真的 `save_config` 会把 mtime
        推到下一个 tick，缓存键自然就miss了。也就是说它在测「操作系统时钟动了一下」，
        不是在测作废。这里把 `save_config` 换成空操作（磁盘与 mtime 一字不动，
        但路由照旧 200），于是唯一能让旧值消失的东西就只剩那句显式失效。
        跨进程的 mtime-preserving 部署（`cp -p` / `rsync -t` / 配置管理工具）是同一格
        的另一半，那一格本函数管不到，另立 A158。
        """
        import api.deps as deps
        import augmentor.config as cfgmod

        http, tmp_path = client
        path = tmp_path / "config.yaml"
        ghost = _ghost(tmp_path)
        monkeypatch.setattr(cfgmod, "save_config", lambda *a, **k: None)
        deps._config_roots_cache[(str(path), path.stat().st_mtime_ns)] = [ghost]
        assert deps.allowed_data_roots() == [ghost], "缓存命中，磁盘说了不算"

        response = http.post("/api/config", json={"augmentation": {"variants_per_seed": 3}})
        assert response.status_code == 200
        assert path.stat().st_mtime_ns not in {k[1] for k in deps._config_roots_cache}, \
            "mtime 一字未动，唯一能让旧键消失的是那句显式失效"
        assert deps.allowed_data_roots() != [ghost], "旧边界不许在 mtime 没动的情况下继续放行"

    def test_failed_save_does_not_invalidate(self, client, monkeypatch):
        """作废只在「真的写成功了」之后发生：写失败时缓存保持原状

        这一条钉的是顺序（`save_config` 在前、`invalidate_config_caches` 在后）。
        顺序反了不会有任何可见故障，只有「什么都没改成功却把缓存洗了一遍」这种
        没人能复现的形状 —— 所以它必须被断言，而不是靠读代码。
        """
        import api.deps as deps
        import augmentor.config as cfgmod

        http, tmp_path = client
        path = tmp_path / "config.yaml"
        real = cfgmod.save_config
        calls = []

        def boom(*args, **kwargs):
            calls.append(1)
            raise OSError("disk full")

        monkeypatch.setattr(cfgmod, "save_config", boom)
        primed = deps.allowed_data_roots()
        assert calls == [], "读白名单不该触发 save_config"

        response = http.post("/api/config", json={"augmentation": {"variants_per_seed": 3}})
        assert response.status_code == 500
        assert calls == [1]
        assert deps._config_roots_cache and deps.allowed_data_roots() == primed

    def test_route_imports_the_invalidator_from_deps(self):
        """耦合方向：作废函数住在 `deps`（缓存的主人），路由只是调用方

        用 AST 而不是字符串，是因为字符串判据被变异探针抓住了：把那句调用注释掉
        之后，`"invalidate_config_caches()" in source` **照样为真**（注释里就有这串
        字符）⇒ 一条自称「守卫」的用例根本不会失败。这条教训与 A153（`dead_line`
        桶只认空行/越界、不认指错东西）同一族：**守卫的覆盖面要能被反例证伪**。
        """
        import ast
        import textwrap

        import api.deps as deps
        import api.routes.config as route

        tree = ast.parse(textwrap.dedent(inspect.getsource(route.update_config)))
        called = {
            node.func.id
            for node in ast.walk(tree)
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Name)
        }
        assert "invalidate_config_caches" in called, "必须是真的被调用，不是出现在注释里"
        assert "save_config" in called
        source = inspect.getsource(route.update_config)
        assert "_config_roots_cache" not in source
        assert "_config_upload_limit_cache" not in source
        assert hasattr(deps, "invalidate_config_caches")

    def test_invalidation_happens_after_the_save_statement(self, client, monkeypatch):
        """跨函数顺序：`save_config` 落盘的那一刻，磁盘内容才是新的

        注意被patch的是**路由模块里那个已绑定的名字**，不是 `deps` 上的原函数 ——
        路由用 `from ..deps import invalidate_config_caches` 绑过一次，改 `deps`
        上的属性对它无效（这条如果不写清楚，下一轮会有人「测不出顺序」而以为没发生）。
        """
        import api.deps as deps
        import api.routes.config as route
        import augmentor.config as cfgmod

        http, tmp_path = client
        events = []
        real_save = cfgmod.save_config
        real_invalidate = deps.invalidate_config_caches

        def save(*args, **kwargs):
            events.append("save")
            return real_save(*args, **kwargs)

        def invalidate():
            events.append("invalidate")
            return real_invalidate()

        monkeypatch.setattr(cfgmod, "save_config", save)
        monkeypatch.setattr(route, "invalidate_config_caches", invalidate)
        response = http.post("/api/config", json={"augmentation": {"variants_per_seed": 3}})
        assert response.status_code == 200
        assert events == ["save", "invalidate"]


class TestGranularityOfTheDrop:
    """作废的粒度：只清内容两张，路径/环境那两张不动"""

    def test_only_content_caches_are_dropped(self, cfg):
        deps, path, _write, _broad, _narrow = cfg
        # 路径缓存按真实读法建；环境白名单缓存直接调解析函数建；两张内容缓存各读一次
        deps.config_file_path()
        deps._env_data_roots(str(path.parent / "envroot"))
        deps.allowed_data_roots()
        deps.max_upload_bytes()
        assert deps._env_roots_cache
        assert deps._config_roots_cache and deps._config_upload_limit_cache
        if not deps._config_path_cache:  # `config_file_path` 被 fixture 换成 lambda，手动补一条
            deps._config_path_cache[(None, str(path.parent))] = path

        deps.invalidate_config_caches()
        assert deps._config_roots_cache == {}
        assert deps._config_upload_limit_cache == {}
        assert deps._config_path_cache, "路径缓存的键是环境变量+工作目录，配置内容与它无关"
        assert deps._env_roots_cache, "同上：环境变量原值没变，重解析只是白付 0.94 ms 的 resolve"

    def test_reparse_costs_exactly_one_load_after_invalidation(self, cfg):
        """作废一次 ⇒ 重解析一次，不是每请求作废每请求重解析"""
        deps, path, _write, _broad, _narrow = cfg
        assert deps.max_upload_bytes() == 4096
        calls = []
        real_load = deps.load_config

        def counting(*args, **kwargs):
            calls.append(1)
            return real_load(*args, **kwargs)

        deps.load_config = counting
        try:
            deps.invalidate_config_caches()
            assert deps.max_upload_bytes() == 4096
            assert deps.max_upload_bytes() == 4096
            assert len(calls) == 1
        finally:
            deps.load_config = real_load

    def test_invalidate_on_empty_caches_is_harmless(self, cfg):
        deps = cfg[0]
        deps.invalidate_config_caches()
        deps.invalidate_config_caches()
        assert deps._config_roots_cache == {}

    def test_invalidation_does_not_disturb_the_live_pipeline_config(self, cfg):
        """作废缓存 ≠ 回滚内存：正在跑的 `p.config` 不该被这条函数碰"""
        deps = cfg[0]
        instance = type("PipelineStub", (), {})()
        instance.config = "SENTINEL"
        old = deps._pipeline
        deps._pipeline = instance
        try:
            deps.invalidate_config_caches()
            assert deps._pipeline.config == "SENTINEL"
        finally:
            deps._pipeline = old


class TestEveryMtimeCacheIsCovered:
    """同构守卫：`deps` 里凡是按 mtime 建键的缓存，都必须被作废函数覆盖

    这条判据针对的是「第四张缓存」：本轮把两张 mtime 缓存接上失效，而下一轮任何人
    再加一张按 mtime 建键的缓存时，最省事的做法就是照抄 `_config_data_roots` ——
    照抄来的那张不会自动进作废名单，于是 A156 以新的形状复发（与 A123/A151 的
    「清单与权威两边漂移」同族，用 AST/正则把两边钉在一起）。
    """

    def test_mtime_keyed_caches_equal_the_invalidated_set(self):
        import api.deps as deps

        source = inspect.getsource(deps)
        mtime_caches, cleared = set(), set()
        for block in re.split(r"\n(?=def |async def |class )", source):
            names = set(re.findall(r"(\w+_cache)\.get\(", block))
            if names and "st_mtime_ns" in block:
                mtime_caches |= names
            if block.startswith("def invalidate_config_caches"):
                cleared |= set(re.findall(r"(\w+_cache)\.clear\(\)", block))

        assert mtime_caches == {"_config_roots_cache", "_config_upload_limit_cache"}
        assert cleared == mtime_caches, (
            f"按 mtime 建键的缓存 {sorted(mtime_caches)} 必须与作废集合 {sorted(cleared)} 一致"
        )

    def test_the_two_caches_really_share_the_key_shape(self):
        """同构的证据：两处键构造逐字同形，所以才由同一次作废一起覆盖"""
        import api.deps as deps

        bodies = []
        for name in ("_config_data_roots", "_config_upload_limit"):
            body = inspect.getsource(getattr(deps, name))
            bodies.append(re.search(r"key: Tuple\[str, Optional\[int\]\] = .*", body).group(0))
        assert len(set(bodies)) == 1

    def test_shipped_config_reads_the_same_values_through_both_edges(self):
        """出厂配置在「缓存读」与「直接读」两条边上必须同值

        作废之后走的就是缓存读那条边。如果哪天缓存的降级方向漂了
        （`_config_data_roots` 回出厂默认、`_config_upload_limit` 回 0 之类），
        这条会在真配置上先红，而不是等到某个请求 413。
        """
        import api.deps as deps

        shipped = str(SHIPPED_YAML)
        direct = load_config(shipped).web
        deps._config_roots_cache.clear()
        deps._config_upload_limit_cache.clear()
        roots = deps._config_data_roots(Path(shipped))
        assert roots == [Path(str(p)).resolve() for p in direct.data_roots]
        assert deps._config_upload_limit(Path(shipped)) == direct.max_upload_bytes
