# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L89 / A152 附带的一格：`save_config` 写出的那些 `null` 键，加载侧必须全部收得下

立项理由（不是猜测，是 L89 量 A152① 的往返差异时撞见的）：`save_config` 走
`dataclasses.asdict` 整树展开，凡是**当前值为 `None`** 的字段都会被具象成磁盘上一个
真实的 `null` 键。出厂那份 `config.yaml` 往返一次实测多出 **7 个** `null` 键
（`Temp/l89q/roundtrip_diff.py`，形状 B）：五个 `models.<名>.request_timeout` 加
两个 `models.{ernie,openai}.base_url`。

这 7 个键今天**无害**，但无害的理由是一条没写下来的巧合：`ModelConfig` 里恰好只有
「`None` 是合法档位」的那两键会是 `None`，而 `__post_init__` 对采样三键（`temperature`
/ `top_p` / `max_output_tokens`）的 `null` 是**拒收**的（config.py 里那段「`None` 的读法
按键分档」），它们又都有非 `None` 默认值，所以永远进不了那 7 个里。

于是真正的风险是**将来**：给任何一个配置节加一个默认值为 `None`、而加载侧拒收 `null`
的字段，`POST /api/config` 就会当场把「写进去 → 下次重启起不来」这条路接通 —— 症状与
A117 那一族完全相同（当次请求 200，进程照跑，重启即抛），而且那时磁盘上已经没有一份
能改回来的配置了。这不靠加判据解决（写侧的行为没错），靠一条**会翻红的守卫**盯住：

- 判据①「往返闭合」：往返出来的那份文件必须**能加载**，且加载结果与往返前深相等
  （`null` 不许改变语义）；
- 判据②「逐键隔离」：那 7 个键（以及将来任何新增的 `null` 键）**逐个单独**注入出厂
  配置，每一次加载都不许抛「不能是 null」。这一条比①更严：①只保证「这一批一起写」
  能过，②保证每一个键各自都是「写了键不给值也认」的键 ⇒ 新增字段一旦落进拒收 `null`
  那一档，这里当场红，而不是等某个用户的重启。

两份判据都用产品同一条路（`load_config` / `save_config`），不复制第二份判据。

**本文件自己也被变异探针打过**（`Temp/l89q/mutation_probe_nulls.py`，两解释器读数一致）：
M1 = 在装配模型条目处注入「写了键、值为 null 就拒」（只判 `kwargs` 里在场的键）⇒
**5 failed / 6 passed**，红的恰好是预判那 5 条（三条往返闭合 + 两条 `base_url` 注入），
`request_timeout` 那 5 条与射程锚点照常绿；M2 = dump 前递归剥掉 `None` 叶子 ⇒
**1 failed / 10 passed**，红的只有锚点。两次还原后均 11 passed、产品文件逐字节相等。
⇒ 判据②与锚点各有各的牙，不是同一件事写两遍。
"""

import copy
import sys
from pathlib import Path

import pytest
import yaml

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor.config import load_config, save_config  # noqa: E402

SHIPPED_YAML = REPO / "config.yaml"

#: `save_config` 往返出厂配置后写出来的全部 `null` 键（L89 现量，两解释器一致）。
#: 它既是判据①的锚点（清单变了这条就红），也是判据②的射程（逐键注入）。
WRITTEN_NULL_KEYS = [
    "models.claude.request_timeout",
    "models.ernie.base_url",
    "models.ernie.request_timeout",
    "models.gemini.request_timeout",
    "models.ollama.request_timeout",
    "models.openai.base_url",
    "models.openai.request_timeout",
]


def _leaf_nulls(node, prefix=""):
    """往返产物里值为 `null` 的叶子路径（`models.ernie.base_url` 这种点分形状）"""
    out = []
    if isinstance(node, dict):
        for key, value in node.items():
            out.extend(_leaf_nulls(value, f"{prefix}{key}."))
    elif node is None:
        out.append(prefix[:-1])
    return sorted(out)


def _inject(doc, dotted, value):
    """在出厂配置的副本上把一条点分路径写成给定值（父级缺失时补出中间层）"""
    cursor = doc
    for part in dotted.split(".")[:-1]:
        cursor = cursor.setdefault(part, {})
    cursor[dotted.split(".")[-1]] = value


@pytest.fixture
def roundtrip(tmp_path):
    """形状 B 的一次往返：就地覆盖一份出厂配置的副本（与 `POST /api/config` 同形状）

    刻意不写成「往一个不存在的路径保存」—— 那种形状下 `save_config` 里「保留既有文件
    里 AppConfig 不建模的顶层段落」与「沿用 `${ENV}` 占位符」两段都走不到，量出来的
    差异会把「没有原文件可抄」误读成缺陷（L89 本轮第一次就踩在这里，见
    `Temp/l89q/roundtrip_diff.py` 的两种形状对账）。
    """
    target = tmp_path / "config.yaml"
    target.write_bytes(SHIPPED_YAML.read_bytes())
    before = load_config(str(SHIPPED_YAML))
    save_config(copy.deepcopy(before), str(target))
    return before, target


class TestRoundTripCloses:
    """判据①：往返出来的文件能加载，且语义与往返前一致"""

    def test_the_written_file_still_loads(self, roundtrip):
        _, target = roundtrip
        load_config(str(target))

    def test_roundtrip_is_semantically_idempotent(self, roundtrip):
        before, target = roundtrip
        after = load_config(str(target))
        assert after == before, "往返改变了加载结果 ⇒ `null` 键不再无害，A152 那格要重开"

    def test_second_roundtrip_adds_no_more_nulls(self, roundtrip):
        """`null` 键必须**一次成型**：第二次往返不许继续长（长个 = 写入面在自我放大）"""
        _, target = roundtrip
        first = yaml.safe_load(target.read_text(encoding="utf-8"))
        config = load_config(str(target))
        save_config(config, str(target))
        second = yaml.safe_load(target.read_text(encoding="utf-8"))
        assert _leaf_nulls(second) == _leaf_nulls(first)


class TestEveryWrittenNullIsAcceptableToTheLoader:
    """判据②：每一个被写出来的 `null` 键，单独注入出厂配置也要能被加载收下"""

    def test_the_measured_null_set_is_not_empty(self, roundtrip):
        """反证锚点：这条用例的射程由这个数决定 ⇒ 它必须真的在测东西

        出厂配置往返出来的 `null` 键实测 7 个。若哪天一个都不剩（比如写入面改成跳过
        `None` 字段），上面两条判据与下面那批注入用例会集体退化成自等，所以这里先把
        「探针本身有载荷、且载荷恰好是 `WRITTEN_NULL_KEYS` 这一份清单」钉住。
        """
        _, target = roundtrip
        assert _leaf_nulls(yaml.safe_load(target.read_text(encoding="utf-8"))) == WRITTEN_NULL_KEYS

    @pytest.mark.parametrize("dotted", WRITTEN_NULL_KEYS)
    def test_injecting_a_single_null_still_loads(self, dotted, tmp_path):
        """把这个键单独写成 `null` 塞进出厂配置 ⇒ 加载侧必须收下（不许抛「不能是 null」）"""
        doc = yaml.safe_load(SHIPPED_YAML.read_text(encoding="utf-8")) or {}
        _inject(doc, dotted, None)
        path = tmp_path / "one_null.yaml"
        path.write_text(yaml.safe_dump(doc, allow_unicode=True, sort_keys=False), encoding="utf-8")
        try:
            load_config(str(path))
        except Exception as e:  # noqa: BLE001 —— 这条用例的判决就是「有没有抛」
            pytest.fail(f"{dotted} 被 `save_config` 写成 null，但加载侧拒收：{type(e).__name__}: {e}")
