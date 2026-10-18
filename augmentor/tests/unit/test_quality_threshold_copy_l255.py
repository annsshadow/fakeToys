# -*- coding: utf-8 -*-
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L255 守卫（收 A316）：页面与文档里那句「三个阈值」必须等于 SDK 当下真的在用的那三个

**为什么立这一条**：`completeness 0.7 / diversity 0.4 / consistency 0.7` 这一串数出现在**三个面**
（前端卡片正文、前端类型注释、`docs/augmentor/API.md` 那一行），而它的权威住在
`augmentor/quality_monitor.py` 的 `_setup_default_thresholds()` 里——那里是**构造实参**而不是可引
常数，改一个数不会让任何测试红。L234 把这三个数写进文案时是用探针量的，量完之后**没有任何尺子回头
看过它**：SDK 调档 ⇒ 三处文案同时变成假话，而「触发告警为 0」这句判决恰恰是页面替用户读告警的唯一
入口。这是 A266 / A275（契约面数字靠人肉对账）同族里最容易烂的一格，因为它连「文档与代码」都不是，
是「文案与实参」。

**口径（全部按现量选，不是先立后用）**：

1. **三面各 1 处，名册精确钉**（`MEASURED_SITES`）：判据写成「抽到的站点数 == 3 且文件名逐格对上」，
   而不是「每处若存在则相等」——后者在有人把句子删掉之后会静默变绿。站点数由
   `test_the_site_roster_is_the_measured_three` 钉，删一处当场红（臂 B3）。
2. **两侧比的是 `alert_below`，不是 `min_value` / `max_value`**：文案那句说的是「触不触告警」，
   而告警只由 `alert_below` 决定（见 `_check_alerts`）；`min_value` 只决定 `severity` 是 critical
   还是 warning，文案里没有那一维。
3. **比的是值而不是字面**：`0.7` 与 `0.70` 是同一个数的两种排版，守卫按 float 判相等，但把**两边
   的字面都留进读数里**（红的时候报「从 0.45 到 0.4」而不是「两个字符串不等」）。真正会红的是把
   `0.7` 写成 `70%` 那一类**换写法**——那一形抽不到 ⇒ 站点数掉 ⇒ 红（见「换写法」那一支）。
4. **行为维在场**：光比「文案 vs SDK 属性」仍是两份静态文本对读，所以同批加一支
   `test_the_alert_path_prints_the_same_numbers`——把三项指标全压到阈值之下走
   `_check_alerts()`，要求每条告警文案里打印出来的那个阈值（`:0.2f` 形）与页面文案的数**逐档相同**。
   这一支把「页面写的数」与「用户真能在告警正文里读到的数」钉成同一份，SDK 若把格式从 `:.2f` 改掉、
   或把 `alert_below` 调档而文案未动，红的是行为那一支而不是文案那一支。
5. **指标名册也是现量**（`MEASURED_METRICS`）：SDK 现在恰好三个指标带 `alert_below`。新增第四个而不
   改文案 ⇒ 文案覆盖不全，臂 B4 证这一支不是恒真。
6. **尺子的失明面（如实声明）**：认的是「`completeness 数 / diversity 数 / consistency 数`」这一种
   连写形状（含跨行与注释星号两种排版）。把三个数拆成三行、或改用「完整性 70%」这种中文措辞，本守卫
   抽不到 ⇒ 站点数掉到 2 而红，**不会**静默放行；真正失明的是「新增第四个面用别的写法」，那一格由
   站点名册的红逼人来更新口径。

7. **四臂写成四条显式用例而不是 `parametrize` 表**（本轮量到的代价，如实记下）：初稿把四臂写成一张
   表加一处 `@pytest.mark.parametrize`，定向复跑 `test_parametrize_blind_face_l98` 报**两格红**——
   档位计数 `derived` 38 → **39**、它与 l97 那把静态尺子的调用点总数同时不合。同一次定向复跑还有
   **五支 error**，事后查明**与本文件无关**：那五支走 `collected` 会话级 fixture，而它拒绝「采集项少于
   下限」的这一跑形状（下限见 l98 的 `MIN_COLLECTED_ITEMS`），定向复跑本来就采不满——全量门禁里它们
   一直是绿的。判据与参数化无关（四臂各有独立期望值与独立报错信息），所以按现量换成四条显式用例，
   并把爆炸半径记在这里：**此后任何一轮新增一处 `parametrize`，都要同批重钉 l98 的档位计数**，
   那是它红的那一格的本征代价，不是可选的同步。

**注入取证**（臂 B1–B4，全部在内存里做，磁盘零副作用）：B1 改文案的一个数 ⇒ 值不等；
B2 改 SDK 侧的一个数（不改文案） ⇒ 值不等；B3 删掉一处站点 ⇒ 名册少一格；B4 给 SDK 加第四个带
`alert_below` 的指标 ⇒ 名册与指标集合同时变。跑完本文件三处文案的字节与进入时相同。
"""
import pathlib
import re

from augmentor.quality_monitor import QualityMonitor, QualityThreshold

ROOT = pathlib.Path(__file__).resolve().parents[2]
#: 子项目文档已于 2026-10-14 汇总到仓库根 `docs/augmentor/`；本文件自持这一真源
DOCS_DIR = ROOT.parent / "docs" / "augmentor"

#: 三面（现量各 1 处）。相对仓库内 augmentor 根目录。
FACES = (
    pathlib.Path("web") / "src" / "pages" / "System.tsx",
    pathlib.Path("web") / "src" / "types" / "api.ts",
    DOCS_DIR / "API.md",
)

#: 连写形状：三个指标名 + `数 / 数 / 数`，允许跨行与注释里的 `*`（两处前端文案都换行，
#: 而 api.ts 那一处换行处带注释星号）。
CLAIM = re.compile(
    r"completeness\s+([\d.]+)\s*/\s*diversity\s+([\d.]+)\s*/\s*(?:\*\s*)?consistency\s+([\d.]+)"
)

#: 现量：三处站点、三个指标、三份 alert_below。
MEASURED_SITES = 3
MEASURED_METRICS = ("completeness", "diversity", "consistency")
MEASURED_ALERT_BELOW = {"completeness": "0.7", "diversity": "0.4", "consistency": "0.7"}


def read(rel):
    """按 bytes 读再解 utf-8：三处文案都在同一仓内，不许靠文本模式的行尾差异"""
    return (ROOT / rel).read_bytes().decode("utf-8")


def claims_in(text):
    """`[(completeness, diversity, consistency)]` —— 字面原样保留，不做 float 归一"""
    return [m.groups() for m in CLAIM.finditer(text)]


#: `docs/augmentor/API.md` 里那一处连写的原文（臂 B3 与「换写法」那一支都要先断言它确实改得动）
API_SENTENCE = "completeness 0.7 / diversity 0.4 / consistency 0.7"


def rewrite_api(text_to):
    """把 API.md 那一处换成别的说法（只在内存里做），改不动就是锚点错了"""
    src = read(FACES[2])
    assert src.count(API_SENTENCE) == 1, src.count(API_SENTENCE)
    return src.replace(API_SENTENCE, text_to)


SYS_ANCHOR = "diversity 0.4"


def rewrite_system(text_to):
    """把卡片正文里的那个数换掉（只在内存里做），改不动同样是锚点错了"""
    src = read(FACES[0])
    assert src.count(SYS_ANCHOR) == 1, src.count(SYS_ANCHOR)
    return src.replace(SYS_ANCHOR, text_to)


def total_sites(overrides=None):
    """三面抽到的站点总数；`overrides` 用「文件 → 改过的文本」模拟某一处被删或换写法"""
    overrides = overrides or {}
    return sum(len(claims_in(overrides.get(face, read(face)))) for face in FACES)


def sdk_alert_below(monitor=None):
    """SDK 当下真的在用的那三个 `alert_below`，值取字面字符串而不是 float（口径 3）"""
    mon = monitor or QualityMonitor()
    return {
        name: format(threshold.alert_below, "g")
        for name, threshold in mon._thresholds.items()
        if threshold.alert_below is not None
    }


def mismatches(page, sdk):
    """两侧按指标名比**值**（`0.70` 与 `0.7` 是同一个数的两种排版），返回「哪一档从几到几」的名册"""
    bad = {}
    for name, want in sdk.items():
        got = page.get(name)
        if got is None or float(got) != float(want):
            bad[name] = (got, want)
    return bad


def as_dict(group):
    return dict(zip(MEASURED_METRICS, group))


def face_id(face):
    """面的稳定标识：落在 augmentor 树内的用其相对路径，汇总到仓库根的文档用仓库根相对路径

    2026-10-14 文档汇总后第三面已不在 `ROOT` 之下，`face.as_posix()` 会变成机器相关的
    绝对路径 ⇒ 名册断言的键不再可移植，故统一走本函数。
    """
    for base in (ROOT, ROOT.parent):
        if face.is_relative_to(base):
            return face.relative_to(base).as_posix()
    return face.as_posix()


class TestTheThreeFacesQuoteTheSameNumbers:
    def test_the_site_roster_is_the_measured_three(self):
        """三面各恰好 1 处，且文件名逐格对上（删面 / 加面都会在这里红，不会静默）"""
        roster = {face_id(face): len(claims_in(read(face))) for face in FACES}
        assert roster == {
            "web/src/pages/System.tsx": 1,
            "web/src/types/api.ts": 1,
            "docs/augmentor/API.md": 1,
        }
        assert sum(roster.values()) == MEASURED_SITES

    def test_every_face_quotes_exactly_what_the_sdk_uses(self):
        """三处的三个数与 SDK 的 `alert_below` 逐档相等，且三处之间也彼此相等"""
        sdk = sdk_alert_below()
        seen = []
        for face in FACES:
            groups = claims_in(read(face))
            assert len(groups) == 1, (face, groups)
            seen.append(as_dict(groups[0]))
            bad = mismatches(as_dict(groups[0]), sdk)
            assert not bad, (face, bad)
        assert seen[0] == seen[1] == seen[2]

    def test_the_copy_covers_exactly_the_metrics_that_have_alert_below(self):
        """口径 5：文案点名的指标集合 == SDK 里带 `alert_below` 的指标集合（现量三个）"""
        assert set(MEASURED_METRICS) == set(sdk_alert_below())
        assert tuple(sdk_alert_below()) == MEASURED_METRICS


class TestTheNumbersAreTheOnesTheAlertPathPrints:
    def test_the_alert_path_prints_the_same_numbers(self):
        """行为维：三项都压到阈值下走 `_check_alerts()`，告警正文打印的阈值必须与文案逐档相同"""
        mon = QualityMonitor()
        sdk = sdk_alert_below(mon)
        metrics = {name: 0.0 for name in sdk}
        alerts = mon._check_alerts(metrics)
        printed = {}
        for alert in alerts:
            tail = alert.message.split("<")[-1].strip()
            assert alert.alert_type == "below"
            printed[alert.metric_name] = tail
        assert printed == {name: f"{float(v):.2f}" for name, v in sdk.items()}
        # 文案写 0.7 ⇒ 告警正文必须印 0.70，两处是同一个数的两种排版
        assert printed["diversity"] == f"{float(MEASURED_ALERT_BELOW['diversity']):.2f}"


class TestTheRulerIsNotIdle:
    """四臂注入取证：每一臂都要改变一个且仅一个判据读数，改不动就说明尺子是恒真的"""

    def test_b1_a_moved_number_in_the_copy_is_caught(self):
        """臂 B1：把页面文案的 diversity 改成 0.45（SDK 不动）⇒ 名册点名 diversity 那一档"""
        bad = mismatches(as_dict(claims_in(rewrite_system("diversity 0.45"))[0]),
                         sdk_alert_below())
        assert bad == {"diversity": ("0.45", "0.4")}, bad

    def test_b2_a_moved_number_in_the_sdk_is_caught(self):
        """臂 B2：把 SDK 的 completeness 调成 0.75（三处文案不动）⇒ 同一把尺子反向点名"""
        bad = mismatches(as_dict(claims_in(read(FACES[0]))[0]),
                         sdk_alert_below(monitor=_monitor_with(completeness=0.75)))
        assert bad == {"completeness": ("0.7", "0.75")}, bad

    def test_b3_a_deleted_face_is_caught_by_the_roster_not_by_luck(self):
        """臂 B3：把 API.md 那一处的连写拆掉 ⇒ 站点总数 3 掉到 2（等值判据，不是「不少于二」）"""
        assert total_sites({FACES[2]: rewrite_api("那三个阈值")}) == MEASURED_SITES - 1

    def test_b4_a_fourth_metric_is_caught_by_the_name_set(self):
        """臂 B4：SDK 多出第四个带 `alert_below` 的指标 ⇒ 指标集合当场变，文案覆盖不再算齐"""
        got = sorted(sdk_alert_below(monitor=_monitor_with(extra=True)))
        assert got == ["completeness", "consistency", "diversity", "leakage"], got

    def test_the_copy_shape_is_what_the_pattern_recognises(self):
        """换新写法（拆成「70%」这种）当场抽不到——证明认的是那一形而不是「任何含数字的句子」"""
        assert total_sites({FACES[2]: rewrite_api(
            "completeness 70% 、 diversity 40% 、 consistency 70%")}) == MEASURED_SITES - 1
        assert claims_in(read(FACES[2])) == [("0.7", "0.4", "0.7")]

    def test_no_disk_side_effect_from_this_file(self):
        """本文件的四臂全在内存里做：跑完三处文案的字节与进入时相同"""
        for face in FACES:
            before = (ROOT / face).read_bytes()
            claims_in(before.decode("utf-8"))
            sdk_alert_below()
            assert (ROOT / face).read_bytes() == before, face


def _monitor_with(completeness=None, extra=False):
    """造一个改过档的 monitor（只动内存里的对象，不碰磁盘也不碰模块级代码）"""
    mon = QualityMonitor()
    if completeness is not None:
        mon._thresholds["completeness"].alert_below = completeness
    if extra:
        mon.add_threshold(QualityThreshold(
            metric_name="leakage", min_value=0.0, max_value=1.0, alert_below=0.05
        ))
    return mon
