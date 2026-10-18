# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集验证模块

提供数据集格式验证、完整性检查等功能，以及跨模块共用的标量入参判据
（`require_count` 计数旋钮 / `require_seconds` 时长旋钮 /
`require_positive` 无量纲倍率旋钮 / `require_ratio` 0-1 比例旋钮 /
`require_bool` 开关旋钮 / `require_string` 非空字符串旋钮 /
`require_string_list` 字符串列表旋钮 / `require_choice` 封闭清单旋钮）。字符串
那一族的「整串是空白」只有一个产地 `is_blank_string`，跨解释器边界的判据也住这里：电子表格「这一格会不会被当公式执行」只有一个产地 `SHEET_FORMULA_LEADERS`。
"""

import json
import logging
import math
import re
from typing import List, Dict, Optional, Any, Set
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum

from augmentor.exceptions import DataValidationError, DatasetTextError

#: L199（B265）：`DataSanitizer.remove_duplicates(keep=)` 的保留策略封闭清单
#: （A77 单一权威）。改前只跟 `"last"` 比一次，`"Last"` / `"last "` / `"firts"`
#: 全部静默按 first 走——用户以为「保留最后一次」，实际保留第一次，且无任何提示。
#: B205 / L135 两轮都在优化这条路径的复杂度，却没补这道判据。
DEDUP_KEEP_MODES = ("first", "last")

logger = logging.getLogger(__name__)

# 空白折叠模式。编译一次放在模块级：`re.sub(字面模式, ...)` 每次都要走一遍
# `re._compile()` 的缓存查找，而 `_sanitize_item` 对每条数据的每个字符串字段
# 各调一次（真实 6902 条 × 3 字段 = 20706 次）。
_WHITESPACE_PATTERN = re.compile(r'\s+')


def require_count(name: str, value: Any, minimum: int = 0,
                  maximum: Optional[int] = None) -> Optional[int]:
    """校验「取前 N 条 / 向后移 N 条」这类计数旋钮，返回原值。

    口径：这类旋钮只有一种合法读法——**要多少条**。它们在实现里几乎都落到
    下标切片，而 Python 的切片对越界值不是报错，是换语义：

    - 负数：`seq[:-1]` 是「丢掉最后一条」，于是「要 1 条」变成「要 N-1 条」；
      `seq[-limit:]` 在 `limit` 为负时又变成「从倒数第 |limit| 条往后全要」。
    - 零：走 `value or default` 回落的调用点会把「要 0 条」当成「没传参数」，
      静默给出默认条数。

    两种都是**静默错答**：调用方拿到一个形状合法、内容却与请求相反的结果，
    没有任何信号。所以在 SDK 的入参处一次判掉，而不是让每个调用点各自小心。

    Args:
        name: 参数名，直接出现在报错里（调用方看到的就是自己写的那个名字）
        value: 传入的值
        minimum: 允许的下界。默认 0——「一条都不要」是合法请求（只要计数、
            只要格式信息），必须与「没传参数」区分开。需要强制正数的调用点
            （如每轮必须选出样本的主动学习）显式传 1。
        maximum: 允许的上界（含），默认 `None` 表示不设上界。这一维是给**配置对象**
            用的（L51 / A77）：计数旋钮越上界的代价不是「换语义」而是「量级失控」——
            改前实测 `AugmentationConfig(max_retries=10**6)` 无任何判据地构造成功，
            按 §3.24 的等待公式它就是 `999999 × 300 s ≈ 83,333 h` 的最坏等待预算，
            而这条天花板此前只写在 `config_validator`（`max_retries ≤ 20`）里，
            只有显式跑过 `validate-config` 的人才拿得到反馈。

    Returns:
        校验通过后的原值。`None` 直接放行——「未提供」由各调用点自己决定
        回落哪个默认值，这里不替它决定。

    Raises:
        DataValidationError: 值不是整数（`bool` 也不算），或小于 `minimum`，
            或大于 `maximum`
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, int):
        raise DataValidationError(
            f"{name} 必须是整数，当前是 {value!r}（{type(value).__name__}）"
        )
    if value < minimum:
        raise DataValidationError(f"{name} 必须是不小于 {minimum} 的整数，当前是 {value}")
    if maximum is not None and value > maximum:
        raise DataValidationError(
            f"{name} 必须是不大于 {maximum} 的整数，当前是 {value}"
        )
    return value


def require_seconds(name: str, value: Any, minimum: float = 0.0,
                    maximum: Optional[float] = None) -> Optional[float]:
    """校验「等待多少秒」这类时长旋钮，返回原值。

    `require_count` 的浮点对应物。计数旋钮的越界症状是「换语义」，时长旋钮一样，
    而且更隐蔽——`retry.compute_delay` 首尾各有一道夹逼（`min(max_delay, …)` 与
    `max(0.0, …)`），实测下来它**不会**把坏值抛出来，而是悄悄换成另一档等待：

    - `base_delay=-5.0` → `0.0`（尾部 `max()` 夹掉负号）：想要「等 5 秒」拿到「立刻重试」
    - `base_delay=NaN` → `30.0`（`30.0 < nan` 为假，`min()` 交出第一个参数）：等到上限
    - `max_delay=NaN` → `0.0`（尾部 `max()` 交出第一个参数）

    三条都不报错，所以判据只能放在入参处。YAML 侧 `.nan` / `.inf` 是合法浮点字面量
    （实测 `yaml.safe_load("a: .nan")` 给出 `float('nan')`），模板没填值就可能留下 NaN，
    这不是纯理论输入。

    正无穷不在拒绝之列：它被 `min(max_delay, …)` 夹住，行为有界且与「等很久」的意图一致。

    `maximum` 是给**旋钮本身就是封顶**的那些参用的（默认 `None` ⇒ 与既有全部调用点
    逐字同答）。上面那句「正无穷无所谓」的前提是"实现里另有一道夹逼兜住它"，而
    `retry.with_retries` 的 `max_retry_wait` 恰恰**就是**那道夹逼（L49 / A73）：它自己
    越界时无处可夹，加判据前实测 `max_retry_wait=inf` 让 `Retry-After: 3000` 原样睡着
    3000 s，
    把 `MAX_RETRY_AFTER` 对外承诺的「服务端指令一支封顶 300 s」直接作废。所以对这一类
    参，上界不是"防手滑的天花板"（那是校验器对 `retry_delay ≤ 60` 的用法），而是
    **承诺本身**，必须放在运行时入口。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值。`int` 也接受（YAML 的 `retry_delay: 1` 读进来是整数）
        minimum: 允许的下界。默认 0——「不等，失败就立刻重试」是合法请求
        maximum: 允许的上界（含），默认 `None` 表示不设上界

    Returns:
        校验通过后的原值。`None` 直接放行——「未提供」由各调用点决定回落哪个默认值
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值秒数，当前是 {value!r}（{type(value).__name__}）"
        )
    # 整数按定义不是 NaN：`math.isnan(10**400)` 抛的是 OverflowError（裸异常 ⇒ 500），
    # 而区间比较对 int 是精确的，所以 NaN 判据只在 float 档跑。
    if isinstance(value, float) and math.isnan(value):
        raise DataValidationError(f"{name} 不能是 NaN")
    if value < minimum:
        raise DataValidationError(f"{name} 必须是不小于 {minimum} 的秒数，当前是 {value}")
    if maximum is not None and value > maximum:
        raise DataValidationError(
            f"{name} 必须是不大于 {maximum} 的秒数，当前是 {value}"
        )
    return value


def require_positive(name: str, value: Any) -> Optional[float]:
    """校验「指数因子」这类**无量纲倍率**旋钮，返回原值。

    与 `require_seconds` 同族，但下界固定为「严格大于 0」、没有 `minimum` 可配：
    这类值只有「每档乘多少」一种合法读法，而实现里的幂运算对 0 与负数都不报错，
    只是换形状。实测（`retry.compute_delay`，`base_delay=1`、`max_delay=30`，
    退避取第 1/2/3 档）：

    - `factor=0` → ``[1.0, 0.0, 0.0]``（``0 ** 0`` 是 1，之后每档都是 0 ⇒ 忙重试）
    - `factor=-2` → ``[1.0, 0.0, 4.0]``（负幂次的小数被尾部 `max(0.0, …)` 与正负
      交替搅在一起，得到一个非单调、隔档完全不等待的序列）
    - `factor=NaN` → ``[1.0, 30.0, 30.0]``（首档照常，之后 `min()` 交出第一个参数）
    - `factor='2'` / `None` → 不报「参数写错」，而是 ``TypeError: unsupported
      operand type(s) for ** or pow()`` —— 且它发生在**第一次真实调用失败之后**
      （见 `with_retries` 里 `raise last_exc` 根本轮不到执行），原始异常整个丢失。

    `+inf` 与 `int` 照常接受：实测 `factor=inf` 给 ``[1.0, 30.0, 30.0]``，行为有界
    且与「一路上限」的意图一致（与 `require_seconds` 同口径）。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值

    Returns:
        校验通过后的原值。`None` 视为「没传参数」，与 `require_count` /
        `require_seconds` 一致，由各调用点自己决定回落哪个默认值。

    Raises:
        DataValidationError: 值不是数值（`bool` 也不算）、是 NaN，或不大于 0
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值，当前是 {value!r}（{type(value).__name__}）"
        )
    # 整数按定义不是 NaN：`math.isnan(10**400)` 抛的是 OverflowError（裸异常 ⇒ 500），
    # 而区间比较对 int 是精确的，所以 NaN 判据只在 float 档跑。
    if isinstance(value, float) and math.isnan(value):
        raise DataValidationError(f"{name} 不能是 NaN")
    if value <= 0:
        raise DataValidationError(f"{name} 必须是大于 0 的数值，当前是 {value}")
    return value


def require_ratio(name: str, value: Any,
                  minimum: float = 0.0,
                  maximum: float = 1.0) -> Optional[float]:
    """校验「随机抖动比例」这类**落在闭区间内的无量纲比例**旋钮，返回原值。

    与 `require_positive` 同族，区别只有一句话：比例**有上界**，而实现里没有任何
    一处会替调用方夹住它（`require_seconds` 的 `maximum` 是 L49 才加的，且只给
    「旋钮本身就是封顶」的那一类参用；比例两界都是内在属性，所以本员的界做成必填、
    默认即 0-1）。以 `retry.compute_delay` 的 `jitter` 为例（实测，
    `max_delay=30`；贴顶档取 `base_delay=20`、`factor=2`、`attempt=3` ⇒ 夹完正好 30.0）：

    - 抖动是**加在夹好之后**的（``delay += rng.uniform(0, jitter * delay)``），所以
      退避一旦贴到 `max_delay`，jitter 就成了那一支真正的上限来源：同一档位实测
      `jitter=0.5` 抽出 30.02 ~ 44.94 s、`1.0` 抽出 30.05 ~ 59.87 s、
      `2.0` 抽出 30.09 ~ 89.74 s、`50.0` 抽出 32.27 ~ **1523.54 s** —— 全部
      100% 越过 `max_delay=30`。把比例判在 1 以内，那一支的最坏等待就可证地封在
      2 × `max_delay`（60 s）；判在外面，等于交出一个上界随用户手滑线性放大的旋钮。
    - **下界 0 不是多余的**：负数与 NaN 都过不了 `if jitter > 0` 这道门，于是
      「传了一个参数」与「什么都没传」逐字同答（实测 `jitter=-5.0` / `NaN` 都交出
      ``[1.0, 2.0, 4.0]``，与 `jitter=0` 完全相同）。这类静默不报错的读法正是
      L33 禁止 `or` 回落时抱怨过的同一形状。
    - `bool` 一并拒：`True` 会被当成 1.0 ⇒ 一个本想写「打开抖动」的参数打开的是
      **最大档**抖动（实测 `jitter=True` 与 `jitter=1.0` 五次抽样逐字相同）。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值
        minimum: 下界（含），默认 0.0
        maximum: 上界（含），默认 1.0

    Returns:
        校验通过后的原值。`None` 视为「没传参数」，与 `require_count` /
        `require_seconds` / `require_positive` 一致，由调用点自己决定回落哪个默认值。

    Raises:
        DataValidationError: 值不是数值（`bool` 也不算）、是 NaN 或无穷，或不在闭区间内
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值，当前是 {value!r}（{type(value).__name__}）"
        )
    # 同上：整数不可能是 NaN/无穷，裸 `math.isfinite` 对超大 int 抛 OverflowError
    if isinstance(value, float) and not math.isfinite(value):
        raise DataValidationError(f"{name} 必须是有限数值，当前是 {'NaN' if math.isnan(value) else '无穷'}")
    if not minimum <= value <= maximum:
        raise DataValidationError(
            f"{name} 必须是 {minimum} 到 {maximum} 之间的比例，当前是 {value}"
        )
    return value


#: 「权重族之和的容差」的唯一权威（A320，L238）。三元（`quality.weights` 的配置层与
#: 组件层）与四元（`DatasetHealthScore` 的健康分权重）都取这一个值——判据分抄两处，
#: 同一份 YAML 就会在一面能加载、在另一面被判拒。
WEIGHT_SUM_TOLERANCE = 0.01
#: 判和之前先把浮点噪声量化掉的位数（A320 的现场）。留两位小数凑 1.0 是最自然的用法，
#: 而 `[0.24, 0.25, 0.25, 0.25]` 的十进制差是 0.01、IEEE-754 求和之后是
#: **0.010000000000000009** —— 直接比 `> 0.01` 就把这一档最该有的用法判红。
#: 量化到 9 位之后，「±0.01（含端点）算 1.0」这句才真的是它的尺子。
WEIGHT_SUM_QUANTIZE_DIGITS = 9


def require_ratio_list(name: str, value: Any,
                       length: int = 3,
                       minimum: float = 0.0,
                       maximum: float = 1.0,
                       sum_tolerance: float = WEIGHT_SUM_TOLERANCE) -> Optional[Any]:
    """校验「权重列表」旋钮（A124，L153）：逐项 bool/NaN/越界 + 长度 + 和为 1±容差（含端点）。

    `require_ratio` 的列表形：单旋钮判一道，权重族判三件套（每项、长度、总和）。
    改前这道判据的权威住在 `QualityScorer.__init__` 的两行手抄（只判长度与和），
    字符串 `'abc'`（长度恰 3）死在 `sum()` 的 `TypeError` 上、`[-1.0, 2.0, 0.0]`
    两层面静默放行（权重符号反了排序整个反过来不出声）；且配置层只判 null ⇒
    加载面与直构面对同一个键不同判。本员把三道判据收进判据族，配置层 / 组件层 /
    静态回放三面共引同一份（A77：一条权威只住一处）。

    `None` 视为「没传参数」（家族惯例，由调用点决定回落默认或拒绝）。
    字符串是逐字符可迭代的，必须显式挡掉（`'abc'` 的 `len` 恰好等于 3）。
    `bool` 是 `int` 的子类（`isinstance(True, int)` 恒真），逐项点名拒。
    """
    if value is None:
        return None
    if isinstance(value, (str, bytes)) or not isinstance(value, (list, tuple)):
        raise DataValidationError(
            f"{name} 必须是 {length} 个元素的数值列表，当前是 {type(value).__name__}"
        )
    if len(value) != length:
        raise DataValidationError(
            f"{name} 必须是 {length} 个元素，当前 {len(value)} 个"
        )
    for i, w in enumerate(value):
        if isinstance(w, bool):
            raise DataValidationError(f"{name}[{i}] 不能是布尔，当前是 {w!r}")
        if not isinstance(w, (int, float)):
            raise DataValidationError(
                f"{name}[{i}] 必须是数值，当前是 {w!r}（{type(w).__name__}）"
            )
        if math.isnan(w):
            raise DataValidationError(f"{name}[{i}] 不能是 NaN")
        if not minimum <= w <= maximum:
            raise DataValidationError(
                f"{name}[{i}] 必须在 {minimum} 到 {maximum} 之间，当前是 {w}"
            )
    total = sum(float(w) for w in value)
    if round(abs(total - 1.0), WEIGHT_SUM_QUANTIZE_DIGITS) > sum_tolerance:
        raise DataValidationError(
            f"{name} 之和必须是 1.0（容差 {sum_tolerance}，含端点），当前 {total}"
        )
    return value


def require_item_field(name: str, items: Any, field: Any, label: str = "数据集") -> str:
    """校验「这个字段名在数据集里真的存在」，返回字段名（A321 / A322，L238）。

    下游那些按字段取文本的评分器一律写成 `item.get(field, "")`：字段名整体拼错时
    **每条都取到空串**，于是「唯一值 = 1、重复率 = 100%」——一份根本没被读过的数据
    被报告成「多样性极差」，而这条假判决与「数据真的全重复」在响应里逐字节同形
    （L235 实测：`text_field` 送空串、纯空白、`no_such_field`、`Instruction` 四种都回
    **200**，`duplicate_rate` 全是 **1.0**，`failed_rules` 全含 `duplicate_rate`）。
    静默降级比报错危险，所以在门口拦下。

    两刀各有分工，缺一不可：

    1. `require_string` 管「这个名字本身不成立」——空串、纯空白、非字符串；报错点名
       **参数名**（`name`），调用方一看就知道是哪一格写错了。
    2. 存在性管「名字成立但这里没有这一列」——报错带上**首条实际字段**，因为拼错的人
       真正需要的是「那一份里到底有什么」。

    存在性判的是「**至少一条**含该字段」而不是「每条都含」：数据集异构是常态（增强与
    合并的产物天然多列，见 `augmentor/converter.py` 那条注释），逐条要求会把合法数据判死；
    而取不到那一列的条目由下游按各自口径处理（评估一侧另有逐条带下标的 `_as_column`）。

    Raises:
        DataValidationError: 字段名不是非空字符串，或整个数据集没有一条含该字段
    """
    require_string(name, field)
    if items and not any(field in item for item in items):
        raise DataValidationError(
            f"{label} 里没有任何一条含字段 '{field}'（大概率是字段名拼错了）；"
            f"首条实际字段: {', '.join(sorted(items[0])) or '（无）'}"
        )
    return field


def require_bool(name: str, value: Any) -> Optional[bool]:
    """校验「打开 / 关掉某一节」这类**开关**旋钮，返回原值。

    家族里其他成员判的是数值形状，本员判的是「你到底是不是个布尔」——因为开关
    在下游一律落到 `if config.xxx.enabled:`，而 Python 的真值判断对**任何**非空
    对象都答「是」。实测分两面，别把两面混着读：

    **Python 字典面**（`Temp/l76q/before.json`，跑在提交态 b63648018：把
    `{section: {field: 值}}` 直接喂给 `_load_section`）：

    - `enabled: 'no'` ⇒ 加载放行，字段值是字符串 `'no'`，`bool('no')` 是 `True`
      ⇒ 用户写的是「关掉」，拿到的是「质量闸门照样打开」，且**没有任何一处报错**。
      同一条配置送进 `validate_config` 却是红的（`_validate_known_fields` 的
      内联 bool 判据对非 bool 一律报「类型错误: 期望 bool, 实际 str」；该消息是
      L126 删掉六个死 `_validate_*` 方法后的现行措辞，历史上由 _validate_bool
      报）⇒ 校验红 / 加载绿，正是 A77 那一族缝隙的形状。
    - `enabled: 0` ⇒ 加载放行，真值恰好是假 ⇒ 与 `false` 逐字同答。这一条**碰巧**
      对上了用户意图，但「碰巧」不是判据：`0` / `'0'` / `'false'` / `[]` 都是
      「看起来像关掉」的写法，实测真值依次是 **假 / 真 / 真 / 假**（`bool('0')`
      与 `bool('false')` 都是 `True`，因为非空字符串恒真）⇒ 四种写法两种结果，
      且其中两种与用户写的字面意思**相反**。

    **YAML 面**（`Temp/l76q/pyyaml_bool_forms.json`，15 种拼法 × 解析结果，与产品
    路径同用 `yaml.safe_load`）：上面那四种写法**只有加引号才成立**。
    - 6 档是**真布尔**、本函数不拒：裸 `no` / `false` / `off` → `False`，
      裸 `yes` / `true` / `on` → `True` ⇒ 「关掉」的正常写法都合法。
    - 4 档落到 `str`、本函数拒：`'no'` / `'false'` / `'0'`（加引号即字符串）与
      裸 `None`（注意它**不是** null，是四字母字符串 `'None'`，真值还是 `True`）。
    - 3 档落到 `int` / `list`、本函数拒：`0` / `1` / `[]`。
    - 2 档是 `None`（`~` 与「写了键没给值」）⇒ 由 `_reject_null_fields` 先拒，
      轮不到本函数。
    一句话口径：**拒的是「解析到 Python 还不是布尔」的值，不是某种 YAML 拼法**。

    拒绝一切非布尔（含 `int`、`str`、`list`），而不是「夹一下」或「`bool(value)`」：
    开关只有一真一假两种合法读法，把「像假」的值折成假就是把用户的笔误翻译成
    一个他没写过的决定。报错文案点名 `true` / `false` 两种合法写法，让改的人
    一步就能改对。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值

    Returns:
        校验通过后的原值。`None` 视为「没传参数」，与 `require_count` /
        `require_seconds` / `require_ratio` / `require_positive` 一致，由调用点
        自己决定回落哪个默认值 —— 但**配置对象不走这条短路**：那里的 `None` 是
        「写了键没给值」，由各节的 `_reject_null_fields` 先拒掉。

    Raises:
        DataValidationError: 值不是 `True` 或 `False`
    """
    if value is None:
        return None
    if not isinstance(value, bool):
        raise DataValidationError(
            f"{name} 必须是布尔值 true 或 false，当前是 {value!r}"
            f"（{type(value).__name__}）"
        )
    return value


def is_blank_string(value: str) -> bool:
    """「看起来有值、其实一个字符也没有」的唯一判据（A142 / L83）。

    家族里所有字符串形状判据的**第二刀**都从这里出：`require_string`、
    `require_string_list` 的每一项、以及 `config_validator` 的 `non_empty` 与
    `items` 两格（共引同一个函数对象，由 `is` 身份的守卫钉住，承 A77）。

    存在理由不是「空格也算错」这种口味，而是一族实测过的静默症状：`''` 谁都拒，
    于是所有人都以为「非空」判完了，但 `'   '` 在过去**四面全绿**（L82 现量
    `Temp/l83q/blank_census_before.json`：10 个键拒 `''` 而放行纯空白）。落到下游
    各不一样，且没有一种会出声 —— `data_roots: ["  "]` 得到一个**名叫空格的目录根**
    （`api/deps.py` 的 `Path(str(p))`，与 A118 里那个 `None` 变成 `None` 目录同形，
    只是这次连 `None` 都没写）；`model: '   '` 直发后端，症状换回一条服务端 400；
    `logging.file: '   '` 会创建一个名叫三个空格的文件（空串倒是合法的 = 不落文件）。

    判「整串都是空白」，**不**裁剪值也不拒「含空格的值」：`static_dir: "my docs"`
    照旧合法，`"  0.0.0.0  "` 也照原样通过 —— 静默 `.strip()` 等于替客户改配置，
    与本仓「判据不改变值」的口径一致（L51 起）。空白档按 `str.isspace()`，所以
    NBSP（`\\xa0`，从网页或文档里复制 YAML 时最常见的隐形字符）也在内。

    **空串返回 `False`**，这一点本轮踩过一次（写完全族第一个跑的就是它）：本函数判
    的是「有字符，但字符全是空白」，而 `''` 是「一个字符也没有」。两种错的修法不同，
    报错文案也不同（「不能是空字符串」对「不能是纯空白」），更要紧的是有一格配置
    **只能区分它们** —— `logging.file: ''` 是「不落文件」这一档设计，纯空白却是一个
    名叫空格的文件；如果把空串也算成空白，`AppConfig()` 连默认值都构造不出来。
    调用点因此一律写成「先判空串、再判空白」两刀，`logging.file` 只挂第二刀。

    Args:
        value: 已知是字符串的值（类型那一刀由调用点先判）

    Returns:
        `True` = 非空且整串是空白
    """
    return value != "" and not value.strip()


def require_string(name: str, value: Any) -> str:
    """校验「主机名 / 目录名」这类**必须是非空字符串**的旋钮，返回原值。

    与标量家族的其他成员有一处关键差别：**这里 `None` 不放行**。那一条是为
    函数入参设计的（「没传参数」由各调用点回落默认值），而配置对象的字段没有
    「没传」这种状态 —— YAML 里写了键却没给值，读进来就是 `None`，实测
    `load_config` 会把它原样放进字段（`port=None`、`data_roots=None`）。这一
    步判掉的正是「字段值是 `None` 却被下游按字符串用」：`api/deps.py` 里
    `Path(str(p))` 会把 `None` 变成一个**名叫 `None` 的目录根**（实测改前
    `data_roots=[None]` 得到 `<cwd>/None`），而这类根目录是白名单，静默可用
    比静默不可用更坏。

    空串一并拒：`host: ""` 与 `static_dir: ""` 都不是「未设置」而是「设置成了
    无意义值」，留给下游去猜。纯空白同样拒（A142 / L83）：`host: "   "` 与
    `host: ""` 是同一件事，只是它**看起来**有值，于是过去四面全绿。

    Returns:
        校验通过后的原值（**不放行 `None`**，与同族其他判据不同，理由见上；
        也**不裁剪**空白，见 `is_blank_string`）

    Raises:
        DataValidationError: 值不是字符串（`bool` 也不算）、是空串，或整串是空白
    """
    if isinstance(value, bool) or not isinstance(value, str):
        raise DataValidationError(
            f"{name} 必须是字符串，当前是 {value!r}（{type(value).__name__}）"
        )
    if not value:
        raise DataValidationError(f"{name} 不能是空字符串")
    if is_blank_string(value):
        raise DataValidationError(f"{name} 不能是纯空白字符串，当前是 {value!r}")
    return value


def require_string_list(name: str, value: Any) -> list:
    """校验「目录白名单 / 跨源白名单」这类**字符串列表**旋钮，返回原值。

    同样不放行 `None`（见 `require_string` 的理由）。要判掉的是两类实测过的
    「形状合法、语义完全不同」的写法：

    - **写成标量**：`data_roots: data` 与 `cors_origins: "https://api.corp.example"`。
      前者被逐字符迭代成 `d/a/t/a` 四个根（实测 4 个），数据端点全 403，症状长得
      像后端坏了；后者的后果不是「不生效」而是**放宽** —— starlette 1.6.0 与
      1.2.1 实测同形：`allow_origins` 是字符串时 `origin in allow_origins` 走的是
      **子串**匹配，配置 `"https://api.corp.example"` 会放行 `https://api.corp` 与
      `https://api` 两个完全不同的主机，而写成列表时两个都拒。访问控制项被写成
      标量就静默变成前缀匹配，这是本判据存在的最硬理由。
    - **列表里混进非字符串**：`[None]` / `[123]` —— 下游 `Path(str(p))` 把它们
      变成名叫 `None` / `123` 的根目录，同样静默。

    空列表放行：`cors_origins: []` 是 L50 的出厂默认（一个跨源也不放行），
    `rate_limit_exempt_paths: []` 是「不豁免」，两者都是有意义的显式选择。
    但**纯空白的项**不放行（A142 / L83）：`data_roots: ["  "]` 是一个名叫空格的
    目录根，与上面那两类「形状合法、语义完全不同」的写法同形，而且它比 `''` 更阴 ——
    空串至少谁都看得出来是漏填。

    这里**不**判断元素内容（URL 是否合法、路径是否存在、豁免前缀以 `/` 开头
    才可能命中请求路径）—— 那些是各消费点自己的语义，判在这里会把「配置形状」
    与「业务语义」两件事焊死。

    Returns:
        校验通过后的原列表

    Raises:
        DataValidationError: 值不是 list、元素不是字符串、元素是空串，或元素是纯空白
    """
    if not isinstance(value, list):
        raise DataValidationError(
            f"{name} 必须是列表（YAML 里要用 `- 项` 写成序列，不能写成标量），"
            f"当前是 {value!r}（{type(value).__name__}）"
        )
    for item in value:
        if isinstance(item, bool) or not isinstance(item, str):
            raise DataValidationError(
                f"{name} 的每一项必须是字符串，当前含 {item!r}"
                f"（{type(item).__name__}）"
            )
        if not item:
            raise DataValidationError(f"{name} 的每一项不能是空字符串")
        if is_blank_string(item):
            raise DataValidationError(
                f"{name} 的每一项不能是纯空白字符串，当前含 {item!r}"
            )
    return value


def require_choice(name: str, value: Any, choices) -> Optional[str]:
    """校验「从一张**封闭清单**里选一个」这类旋钮，返回原值（A118 / L82）。

    家族里其他成员判数值与布尔的形状，本员判的是「选中的那一项存不存在」。它的
    存在理由是：这类旋钮在消费方**往往已经有一道同样的判决**，但那一发生在
    「用到它的那一天」，实测三档改前症状（`Temp/l82q/before.json`）——

    - `create_vector_db('nonsense')` ⇒ `VectorError: 不支持的向量数据库后端`，
      而配置里写 `vector.backend: nonsense` 时三面向（SDK 直构 / `load_config` /
      `POST /api/config`）全部放行，端点还回 200。
    - `Exporter('xls')` ⇒ 抛的是 `ValueError: 'xls' is not a valid ExportFormat`，
      **不是**领域异常 ⇒ 走 API 时它是 500 而不是 400。
    - `rag.default_format: pinecone` 今天无人读（`rag.py` 只判自己入参），所以
      它连「晚一天炸」都没有，是一份纯静默。

    所以本员**不新造任何清单**：清单一律由调用点传进来，且那个清单在仓内必须有
    权威产地（A77）。类型、空串、纯空白三刀先由 `require_string` 判掉，于是
    `backend: ''`、`backend: '   '`、`backend: 5`、`backend: ['faiss']` 四档都在
    membership 之前先出更准确的形状错（最后那一档正是 L82 注入 m8 撞出来的
    「名字写着先报形状、其实挂在 membership 上」，A142 本轮收口）。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值
        choices: 封闭清单（tuple / list / set 皆可，报错时按迭代序列出）

    Returns:
        校验通过后的原值。`None` 放行（同家族口径：「没传」由调用点回落默认值；
        配置对象那一侧的 `None` 由各节 `_reject_null_fields` 先拒）

    Raises:
        DataValidationError: 值不是非空字符串，或不在清单里
    """
    if value is None:
        return None
    require_string(name, value)
    if value not in choices:
        raise DataValidationError(
            f"{name} 必须是 {' / '.join(str(choice) for choice in choices)} 之一，"
            f"当前是 {value!r}"
        )
    return value


def require_chunk_window(size_name: str, size: Any,
                         overlap_name: str, overlap: Any) -> None:
    """成对校验「窗口大小 / 窗口重叠」两个互相约束的旋钮（A118 / L82）。

    为什么需要一个判两个值的成员：这条界**不可拆**。`rag.RAGFormatter` 改前只判
    `overlap >= size` 而不判正负，实测 `chunk_size=-1` 配 `chunk_overlap=-5` 构造
    成功，随后 `chunk_text()` 对 200 字符的输入产出 **1 块 199 字符** —— 分块整件
    失效，而症状长得像「这篇文档就是一个块」。单值判据（`require_count`）各自都
    拦得住这一档，但拦的顺序不对就先漏：`-1` 与 `-5` 相比是「小的那个」，
    只比大小关系永远看不出两个都是负数。

    三刀顺序：`size` 必须是不小于 1 的整数、`overlap` 必须是不小于 0 的整数、
    然后才比 `overlap < size`。前两刀复用 `require_count`，所以「`chunk_size` 写成
    `'512'`」这种类型错与全仓其他计数旋钮同一句文案。

    Args:
        size_name: 窗口大小的名字
        size: 窗口大小的值
        overlap_name: 窗口重叠的名字
        overlap: 窗口重叠的值

    Raises:
        DataValidationError: 任一值形状不对，或重叠不小于窗口大小
    """
    require_count(size_name, size, minimum=1)
    require_count(overlap_name, overlap, minimum=0)
    # 两值都必须到场：这条界是一次比较，缺任何一侧都比不了。改前 `RAGFormatter(
    # chunk_size=None)` 在 `64 >= None` 上抛 `TypeError`（响亮），若沿用家族的
    # 「None 放行」口径就会变成静默构造成功 —— 那是把响亮换成静默，不是对齐。
    if size is None:
        raise DataValidationError(f"{size_name} 必须是整数，当前是 None")
    if overlap is None:
        raise DataValidationError(f"{overlap_name} 必须是整数，当前是 None")
    if overlap >= size:
        raise DataValidationError(
            f"{overlap_name} 必须小于 {size_name}，当前是 {overlap} 对 {size}")


class ValidationSeverity(Enum):
    """验证严重程度"""
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


@dataclass
class ValidationIssue:
    """验证问题"""
    field: str
    message: str
    severity: ValidationSeverity
    index: Optional[int] = None
    value: Any = None


@dataclass
class ValidationResult:
    """验证结果"""
    is_valid: bool
    total_items: int
    valid_items: int
    issues: List[ValidationIssue] = field(default_factory=list)
    
    @property
    def error_count(self) -> int:
        return sum(1 for i in self.issues if i.severity == ValidationSeverity.ERROR)
    
    @property
    def warning_count(self) -> int:
        return sum(1 for i in self.issues if i.severity == ValidationSeverity.WARNING)
    
    def to_dict(self) -> Dict:
        return {
            "is_valid": self.is_valid,
            "total_items": self.total_items,
            "valid_items": self.valid_items,
            "error_count": self.error_count,
            "warning_count": self.warning_count,
            "issues": [
                {
                    "field": i.field,
                    "message": i.message,
                    "severity": i.severity.value,
                    "index": i.index
                }
                for i in self.issues[:100]  # 限制返回数量
            ]
        }


class DatasetValidator:
    """数据集验证器
    
    支持多种验证规则和自定义验证。
    """
    
    # 预定义的验证规则
    PRESET_RULES = {
        "basic": {
            "required_fields": ["instruction", "output"],
            "optional_fields": ["input"],
            "max_instruction_length": 1000,
            "max_output_length": 5000
        },
        "strict": {
            "required_fields": ["instruction", "output"],
            "optional_fields": ["input"],
            "min_instruction_length": 5,
            "max_instruction_length": 500,
            "min_output_length": 10,
            "max_output_length": 2000,
            "forbidden_patterns": [r"<script>", r"javascript:"]
        },
        "chat": {
            "required_fields": ["instruction", "output"],
            "optional_fields": ["input", "history"],
            "max_instruction_length": 500,
            "max_output_length": 1000,
            "check_history_format": True
        }
    }
    
    def __init__(self, rules: Optional[Dict] = None, preset: str = "basic"):
        """初始化验证器
        
        Args:
            rules: 自定义验证规则
            preset: 预设规则名称
        """
        # L189①：`rules` 用 `is not None` 判型——显式空 dict {} 是「零规则」的合法意图，
        # 不是「没传」；旧写法 `if rules:` 把 {} 读成没传而静默落到 preset（strict 档拿到
        # basic 规则面），falsy 假零。None 才是「用 preset」。
        if rules is not None:
            self.rules = rules
        # L189②：preset 走封闭清单——未知预设不再静默回落 basic（拼错 strict/chat 会被读成
        # basic，规则面整档静默放宽，checkpoint 症状族语义漂移档）；API/CLI 面早有 400/choices
        # 收口（VALIDATION_PRESETS / parser choices），SDK 直构面这一格补齐。
        elif preset in self.PRESET_RULES:
            # 浅拷贝：把类级字典直接挂到实例上，任一实例改规则就会污染全部预设
            self.rules = dict(self.PRESET_RULES[preset])
        else:
            valid = " / ".join(self.PRESET_RULES)
            raise DataValidationError(
                f"未知验证预设: {preset!r}（可选 {valid}）"
            )
        # 禁止模式的编译缓存，元素是 `(源列表, 编译结果)`；见 `_compiled_forbidden()`
        self._forbidden_cache = None

    def _compiled_forbidden(self) -> tuple:
        """把 `forbidden_patterns` 编译一次并复用

        `re.search(字符串模式, ...)` 每次调用都要在 `re` 模块内部重做一遍
        「按 `(pattern, flags)` 查编译缓存」；真实 6902 条数据的 strict 校验里，
        禁止模式那一段占了整档耗时的 53%，预编译实测快 1.69×。

        缓存键用**模式列表对象本身**而不是布尔「编译过了」：调用方换掉
        `rules["forbidden_patterns"]` 时会触发重编译，而不是静默沿用旧结果。
        惰性编译同时保留了报错时机——非法模式仍在第一条数据上抛，而不是构造时。
        """
        patterns = self.rules.get("forbidden_patterns") or ()
        cache = self._forbidden_cache
        if cache is None or cache[0] is not patterns:
            cache = (patterns, tuple(
                re.compile(p, re.IGNORECASE) for p in patterns))
            self._forbidden_cache = cache
        return cache[1]
    
    def validate(self, items: List[Dict]) -> ValidationResult:
        """验证数据集
        
        Args:
            items: 数据列表
        
        Returns:
            验证结果
        """
        issues = []
        valid_count = 0
        
        for idx, item in enumerate(items):
            item_issues = self._validate_item(item, idx)
            issues.extend(item_issues)
            
            # 如果没有ERROR级别问题，计为有效
            has_error = any(i.severity == ValidationSeverity.ERROR for i in item_issues)
            if not has_error:
                valid_count += 1
        
        return ValidationResult(
            is_valid=valid_count == len(items),
            total_items=len(items),
            valid_items=valid_count,
            issues=issues
        )
    
    def _validate_item(self, item: Dict, index: int) -> List[ValidationIssue]:
        """验证单条数据
        
        Args:
            item: 数据项
            index: 索引
        
        Returns:
            问题列表
        """
        issues = []
        
        # 检查是否为字典
        if not isinstance(item, dict):
            issues.append(ValidationIssue(
                field="root",
                message="数据项必须是字典类型",
                severity=ValidationSeverity.ERROR,
                index=index
            ))
            return issues
        
        # 检查必填字段
        for field_name in self.rules.get("required_fields", []):
            if field_name not in item:
                issues.append(ValidationIssue(
                    field=field_name,
                    message=f"缺少必填字段: {field_name}",
                    severity=ValidationSeverity.ERROR,
                    index=index
                ))
            elif not isinstance(item[field_name], str):
                issues.append(ValidationIssue(
                    field=field_name,
                    message=f"字段 {field_name} 必须是字符串类型",
                    severity=ValidationSeverity.ERROR,
                    index=index
                ))
            elif len(item[field_name].strip()) == 0:
                issues.append(ValidationIssue(
                    field=field_name,
                    message=f"字段 {field_name} 不能为空",
                    severity=ValidationSeverity.WARNING,
                    index=index
                ))
        
        # 检查字段长度
        for field_name in ["instruction", "output", "input"]:
            if field_name in item and isinstance(item[field_name], str):
                value = item[field_name]
                min_key = f"min_{field_name}_length"
                max_key = f"max_{field_name}_length"
                
                if min_key in self.rules and len(value) < self.rules[min_key]:
                    issues.append(ValidationIssue(
                        field=field_name,
                        message=f"字段 {field_name} 长度不能小于 {self.rules[min_key]}",
                        severity=ValidationSeverity.WARNING,
                        index=index,
                        value=len(value)
                    ))
                
                if max_key in self.rules and len(value) > self.rules[max_key]:
                    issues.append(ValidationIssue(
                        field=field_name,
                        message=f"字段 {field_name} 长度不能大于 {self.rules[max_key]}",
                        severity=ValidationSeverity.WARNING,
                        index=index,
                        value=len(value)
                    ))
        
        # 检查禁止的模式
        forbidden = self._compiled_forbidden()
        for field_name in ["instruction", "output"]:
            if field_name in item and isinstance(item[field_name], str):
                for pattern in forbidden:
                    if pattern.search(item[field_name]):
                        issues.append(ValidationIssue(
                            field=field_name,
                            message=f"字段 {field_name} 包含禁止的模式: {pattern.pattern}",
                            severity=ValidationSeverity.ERROR,
                            index=index
                        ))
        
        # 检查历史记录格式
        if self.rules.get("check_history_format") and "history" in item:
            history = item["history"]
            if not isinstance(history, list):
                issues.append(ValidationIssue(
                    field="history",
                    message="history 字段必须是列表类型",
                    severity=ValidationSeverity.ERROR,
                    index=index
                ))
            else:
                for turn_idx, turn in enumerate(history):
                    if not isinstance(turn, dict):
                        issues.append(ValidationIssue(
                            field="history",
                            message=f"history[{turn_idx}] 必须是字典类型",
                            severity=ValidationSeverity.ERROR,
                            index=index
                        ))
                    elif "role" not in turn or "content" not in turn:
                        issues.append(ValidationIssue(
                            field="history",
                            message=f"history[{turn_idx}] 缺少 role 或 content 字段",
                            severity=ValidationSeverity.WARNING,
                            index=index
                        ))
        
        return issues
    
    def validate_file(self, file_path: str) -> ValidationResult:
        """验证文件
        
        Args:
            file_path: 文件路径
        
        Returns:
            验证结果
        """
        with open(file_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        return self.validate(items)


class DataSanitizer:
    """数据清洗器
    
    提供数据清洗和修复功能。
    """
    
    def __init__(self):
        """初始化清洗器"""
        pass
    
    def sanitize(self, items: List[Dict], fix: bool = True) -> List[Dict]:
        """清洗数据集
        
        Args:
            items: 数据列表
            fix: 是否尝试修复问题
        
        Returns:
            清洗后的数据列表
        """
        result = []
        
        for item in items:
            sanitized = self._sanitize_item(item, fix) if fix else item
            if sanitized is not None:
                result.append(sanitized)
        
        return result
    
    def _sanitize_item(self, item: Dict, fix: bool) -> Optional[Dict]:
        """清洗单条数据
        
        Args:
            item: 数据项
            fix: 是否尝试修复
        
        Returns:
            清洗后的数据项，或 None（如果应删除）
        """
        if not isinstance(item, dict):
            return None
        
        result = item.copy()
        
        # 清洗字符串字段
        for field_name in ["instruction", "output", "input"]:
            if field_name in result and isinstance(result[field_name], str):
                # 去除首尾空白
                result[field_name] = result[field_name].strip()
                
                # 去除多余空白并移除控制字符。原恒真守卫已在 L126 删除：本方法只在
                # sanitize 的 fix=True 支被调用（:794），全量偏支审计的 822->826 /
                # 826->816 两条假支随守卫一起消失
                result[field_name] = _WHITESPACE_PATTERN.sub(' ', result[field_name])
                # 移除控制字符
                result[field_name] = ''.join(
                    c for c in result[field_name] 
                    if c.isprintable() or c in '\n\r\t'
                )
        
        # 移除空的必填字段
        if not result.get("instruction") or not result.get("output"):
            return None
        
        return result
    
    def remove_duplicates(self, 
                         items: List[Dict], 
                         key: str = "instruction",
                         keep: str = "first") -> List[Dict]:
        """移除重复项
        
        Args:
            items: 数据列表
            key: 去重依据的字段
            keep: 保留策略 (first/last)
        
        Returns:
            去重后的数据列表

        Raises:
            DataValidationError: `keep` 不是 first/last 之一
        """
        if keep is None:
            raise DataValidationError(
                f"keep 不能为 null，合法取值: {' / '.join(DEDUP_KEEP_MODES)}"
            )
        require_choice("keep", keep, choices=DEDUP_KEEP_MODES)

        seen = {}
        result = []
        
        for idx, item in enumerate(items):
            value = item.get(key, "")
            if value not in seen:
                seen[value] = len(result)
                result.append(item)
            elif keep == "last":
                # 替换之前的记录：seen[value] 存的是 result 下标，O(1) 直接覆盖；
                # 语义＝「保留最后一次的内容，落在首次出现的位置」（与原逐条扫描等价）
                result[seen[value]] = item
        
        return result


def require_within_dir(label: str, value: Any, base: Path) -> Path:
    """把调用方给的**名字**限制在 `base` 目录之内，返回已 resolve 的落点。

    这一族判据要钉的是「名字被当成路径片段用」的形状：版本 ID、任务 ID、数据
    集名称、分片前缀都会拼进 `base / f"{name}..."`，而 `pathlib` 对 `..` 与绝对
    路径是**照字面拼**的 —— 实测 `VersionManager(storage_dir=<tmp>/versions)
    .delete_version("../victim/important")` 返回 `None`（成功）并把版本根之外
    那个整目录 `shutil.rmtree` 掉，`load_version("../secrets")` 交出目录外的
    `data.json`，`CheckpointManager.create_checkpoint("../../escape", 3)` 把断点
    文件写到 `checkpoint_dir` 之外，`DiskCache(cache_dir=<数据目录>).clear()`
    连目录里别人的 `*.json` 一起 unlink。L286 补最后一档：名字是 `.` 时落点**就是基座目录本身**（`path_lands_within` 判形状，「相等」也算落在之内），于是 `delete_version(".")` 一次 `rmtree` 掉整个版本根（全部版本 + `history.jsonl` + `current.txt`），而 HTTP 面 `DELETE /api/versions/%2E` 实测能把 `'.'` 原样送进路径参数。

    与 `api/deps.py` 的 `resolve_within_roots` 是同一条口径的两个尺度：那一把
    管「HTTP 传入的路径只能落在数据白名单之内」，这一把管「一个**名字**不许
    逃出它自己的目录」。SDK 面必须自带这一把，因为 CLI 与直接调用 SDK 的
    调用方**不经过** HTTP 那道闸。

    Returns:
        `base / value` 规范化之后的绝对路径（严格落在 `base` 之内，不含 `base` 本身）

    Raises:
        DataValidationError: 名字为空、是绝对路径、含 `..` 组件，或规范化之后落到
            `base` 之外（符号链接逃逸）或**就是 `base` 本身**。判据写成两次 `path_lands_within` 而不是 `resolved == root`，是因为 `resolve()` 的扩展长度前缀形状在两次调用之间可以不一致（L273 现量那一格），归一交给权威自己做。错误文案**只含名字与目录标签，
            不含解析后的绝对落点** —— 那是 `INTERNAL_ERROR_DETAIL` 那一族要防的
            部署路径外泄面。
    """
    # 「必须是非空字符串」这一维不重抄：交回同族的 `require_string`。自己写
    # `not isinstance(value, str)` 会把 `5` 判成「不能为空」——那是句假话。
    value = require_string(label, value)

    candidate = Path(value.strip())
    if candidate.is_absolute() or ".." in candidate.parts:
        raise DataValidationError(f"{label} 不能指向 {base.name} 目录之外")

    root = Path(base).resolve()
    resolved = (root / candidate).resolve()
    if not path_lands_within(root, resolved) or path_lands_within(resolved, root):
        raise DataValidationError(f"{label} 不能指向 {base.name} 目录之外或它本身")
    return resolved


def path_lands_within(base: Path, landing: Path) -> bool:
    r"""`landing` 是否等于 `base` 或落在它之内 —— 判**形状**，且先抹平 `resolve()` 的前缀

    住在 `require_within_dir` 之后而不是之前，是为了不搬动它上面那些被账本与守卫点名的行号；
    名字是公开的，因为 `augmentor/backup.py` 的落点判据与它是同一件事，不该有两套写法。

    为什么不写成 `base in landing.parents`（L273 现量出来的坑）：`resolve()` 的输出**形状**
    取决于调用那一刻哪一段存在 —— Windows 上拿到句柄的那一次会多出 `\\?\` 这个扩展长度前缀，
    没拿到的那一次不带。同一条表达式里两次 `resolve()`，只要目录正好在两次之间被别的线程建
    出来，两侧就拿到两套形状，于是**一个普通名字被判成目录之外**：并发首用 20 次实测 4—12 次
    误判，两侧读数由 `probe2_resolve.py` 抓在案（左 `C:\<临时>\c00`、
    右 `\\?\C:\<临时>\c00\b002.json`）。那不是并发才有的怪事，而是「拿两次读盘的结果比字符串」
    这件事本身的口径不成立；判形状不比句柄 ⇒ 剥掉扩展长度前缀，再交给 `Path` 自己那一套
    比较（Windows 折叠大小写、POSIX 不折叠）问「相等，或是它的某一级父目录」。

    Args:
        base: 已解析的目录
        landing: 已解析的落点

    Returns:
        `landing` 是否在 `base` 之内（含两者相等的「就是目录本身」那一档）
    """
    left = Path(str(base).removeprefix("\\\\?\\"))
    right = Path(str(landing).removeprefix("\\\\?\\"))
    return right == left or left in right.parents


def validate_dataset(file_path: str, preset: str = "basic") -> Dict:
    """验证数据集文件
    
    Args:
        file_path: 文件路径
        preset: 预设规则
    
    Returns:
        验证结果字典
    """
    validator = DatasetValidator(preset=preset)
    result = validator.validate_file(file_path)
    return result.to_dict()


def sanitize_dataset(items: List[Dict], remove_duplicates: bool = True) -> List[Dict]:
    """清洗数据集
    
    Args:
        items: 数据列表
        remove_duplicates: 是否移除重复项
    
    Returns:
        清洗后的数据列表
    """
    sanitizer = DataSanitizer()
    result = sanitizer.sanitize(items)
    
    if remove_duplicates:
        result = sanitizer.remove_duplicates(result)
    
    return result


def generate_validation_report(items: List[Dict], preset: str = "basic") -> Dict:
    """生成批量验证报告（增强功能）
    
    Args:
        items: 数据列表
        preset: 验证预设
    
    Returns:
        包含验证结果、统计信息和建议的完整报告
    """
    validator = DatasetValidator(preset=preset)
    result = validator.validate(items)
    
    issues_by_severity = {}
    for issue in result.issues:
        severity_name = issue.severity.value
        issues_by_severity.setdefault(severity_name, 0)
        issues_by_severity[severity_name] += 1
    
    return {
        "summary": result.to_dict(),
        "issues_by_severity": issues_by_severity,
        "total_items": len(items),
        "passed_items": result.valid_items,
        "failed_items": len(items) - result.valid_items,
        "pass_rate": result.valid_items / len(items) if items else 0.0,
        "suggestions": [
            f"发现 {issues_by_severity.get('error', 0)} 个严重错误",
            f"发现 {issues_by_severity.get('warning', 0)} 个警告",
            "建议根据严重程度优先修复错误"
        ]
    }


def require_item_text(items: List[Dict], *fields: str,
                      label: str = "数据集") -> List[Dict]:
    """校验「数据集条目里被当作文本用的字段，值确实是字符串」，原样返回入参（A358 / L274）。

    `require_item_field` 判的是「这一列存在吗」，本员判的是「这一列里的**值**是不是文本」。
    缺口是实测出来的：`[{"instruction": 1, "output": 2}]` 在形状门口（`api/deps.py` 的
    `assert_dataset_shape` 只判「顶层是 list、元素是 dict」）完全合法，于是它一路走进下游
    那些把 `item.get(key, "")` 直接当字符串用的写法——`"为什么" in 1` 抛 `TypeError`、
    `1.lower()` 抛 `AttributeError`、`1.encode()` 抛 `AttributeError`，端点把它们一律折成
    **500**。L274 定向探针（仓外数据根，`Temp/l274/probe3.py`）实测改前 **28 格 500 /
    6 枚栈帧**：`GET /api/analyze/{filename}`（`sampler.py` 的 `_analyze_question_type`
    与 `_analyze_topic_distribution`）、`GET /api/data/load/{filename}?search=`
    （`api/routes/data.py` 的搜索支）、`POST /api/quality/report` 与 `/evaluate` 与
    `POST /api/dataset/compare`（`quality.py` 的 `_char_ngrams`）、
    `POST /api/quality/dedup`（`dedup.py` 的 `_ngram_scan`）、`POST /api/dataset/merge`
    （`dataset_ops.py` 的 `_deduplicate`）。**调用方自己的文件让服务端报内部错误**：
    客户端只拿到固定文案（A155①），可行动信息全留在服务端日志里，而这一形可以批量制造。

    **为什么不 `str()` 归一**：本仓已有八处手抄的 `str(item[key])`（`statistics.py`、
    `profiling.py` 五处、`health_score.py` 两处等），归一在**只读测量**一侧顶多是含糊读数；
    但在**会改写数据**的一侧它是静默删行——`POST /api/dataset/merge` 带 `deduplicate=true`
    时，`instruction` 为 `1` / `None` / `["a","b"]` 的条目会被折成同一份文本而互为重复，
    于是只留一条。`str(None)` 还会产出一个名叫 `None` 的四字符假样本。本族「静默降级比报错
    危险」的判据已由 `require_item_field`（A321 / L238）拍定，本轮沿用而不是另立一套。
    两侧口径并存（评估一侧 `_as_column` 仍 `str()` 归一）记在 A358 待拍。

    **缺列不判**：`{"output": "b"}` 没有 `instruction` 今天得到 200（下游一律 `.get(key, "")`
    回落空串），而增强与合并的产物天然多列，异构数据集是合法数据；把「缺列」一并拒掉会把一批
    **现在就成功**的请求改成 400——那是 A292 / A354 那一维要拍的事，不在本轮。空数据集与
    在场且为字符串的值一律放行，所以对合法数据**零行为差**（本轮以逐字节响应相同取证）。

    返回入参本身是为了让接线落成**同行包裹**（`items = require_item_text(items, ...)`、
    `for item in require_item_text(items, ...)`）而不是插行：本仓的历史定位引用钉在
    `dataset_ops.py` 与 `api/routes/data.py` 的插入点之下，插一行就把它们平移出被点名的构造
    （纪律第 5 节）。

    Args:
        items: 数据集条目列表
        *fields: 要逐个检查的字段名，如 `"instruction", "output"`
        label: 判决文案里的数据集称呼（两侧对比时写「生成侧」/「参考侧」）

    Returns:
        进入时的同一个列表对象（不改写、不裁剪、不补空串）

    Raises:
        DatasetTextError: 某一条里该字段**在场且不是字符串**（`bool` 也不算）。
            落码走既有契约：`to_http_error` 的 ``ValueError`` 支与 `raise_boundary_error`
            都回 **400** 并把判决原文照回客户端；`raise_internal_error` 额外认这一个
            具名窄口（L274），所以六只收尾走它的端点也回 400，而它的父类
            `DataValidationError` 的其他成员在那六只端点上仍按 **A280** 回 500。
    """
    for index, item in enumerate(items):
        for field_name in fields:
            if field_name not in item:
                continue
            value = item[field_name]
            if not isinstance(value, str):
                raise DatasetTextError(
                    f"{label} 第 {index} 条的字段 '{field_name}' 不是字符串，"
                    f"当前是 {type(value).__name__}: {repr(value)[:120]}"
                )
    return items


def require_path_segment(label: str, value: Any) -> str:
    r"""校验「一个可被 URL 寻址的 id 只占一个路径段」，返回原值（A366 / L277）。

    **为什么立这一条**：`augmentor/backup.py` 的备份名由请求方给，它同时是**落盘文件名**
    和 HTTP 路径段（`DELETE /api/system/backups/{backup_id}`、
    `POST /api/system/backups/{backup_id}/restore`）。原有的落点判据 `path_lands_within`
    只管「别写出备份目录之外」，于是 `sub/bak` 这种**落在根内、却含分隔符**的名字完全放行：
    文件照样建、索引照样登记、`GET /api/system/backups` 照样列得出来，但 Starlette 的
    `{backup_id}` 只匹配 `[^/]+` ⇒ restore / delete 恒 404（percent 编码成 `sub%2Fdeep` 也一样 404，
    路由按原始路径切段）。实测形状是「创建回 200、清单里有这一行、这两个入口用尽办法也够不着」——
    数据不是被拒，是**失踪**。同一格在 Windows 上还多一挡：尾随的点与空白会被文件系统
    抹掉，落点名 ≠ 回执 id，按回执 id 去寻址同样 404。

    「建得出来却寻不了址」比直接报错危险，因为它把一份看起来成功的持久状态留在了系统里，
    而所有能操作它的入口都到不了。判据设在**创建那一刻**（名字唯一的出生点），
    不在路由里补——路由只认 HTTP 面，SDK 与 CLI 用户代码传的同一个名字会走同一条落盘路径。

    拒绝的八档（每档都对应一个「回执与可寻址名不成立」的具体形状）：

    1. 非字符串、空串、纯空白：空名落盘成 `.json`，与目录里其它文件无从区分。
    2. 含 `/` 或 `\`（**两种都拒，不分平台**）：`/` 在 URL 段里是分隔符，`\` 在 Windows 上
       是 `os.sep`；POSIX 下它虽然只是普通文件名字符，但一份用 Windows 客户端建的索引
       拿到 POSIX 上读就是两套读法，跨平台的 id 面按更严的一边收。
    3. `.` 与 `..`：单点/双点各自都是「相对父目录」的写法，`..` 会走出备份目录。
    4. `#` 与 `?`：URL 的片段与查询分隔符。实测名字含 `#` 的备份创建 **200**、清单里躺着它，
       而照回执原样拼 URL 的客户端拿到 restore **405** / delete **404**（`#` 之后整段被客户端
       当片段丢掉），只有编码成 `frag%23ment` 才寻得着；`?` 在 Windows 上已被文件系统判死
       （`errno 22` ⇒ 400），在 POSIX 上却是合法文件名 ⇒ 两个平台一套口径，出生时就拒。
    5. 控制字符（`\x00`—`\x1f`、`\x7f`）：NUL 让 `open()` 直接 `ValueError`，旧归宿是 500。
    6. 尾随的点或空白：Windows 静默裁剪 ⇒ 落点名与回执 id 不同形。
    7. 超过 255 字符：单个路径段的长度上限，越界时 `open()` 报 `ENAMETOOLONG`。这一档只是
       **必要**条件——总长还叠加备份目录那一段，Windows 上更早的一刀由操作系统落下
       （实测 200 字符可成、240 字符 `errno 22` ⇒ 400，且**不留残骸**），由 L271 的
       `is_rejected_path_name` 接住，不在这里冒充本闸判的。
    8. 含 `:`（**同样不分平台**）：Windows 上它不是合法文件名字符，而 `pathlib` 会把名字开头
       那一段当**盘符前缀**吞掉——实测 `<备份目录> / "C:foo.json"` 的落点是 `foo.json`：
       回执 id 是 `C:foo`，盘上那份叫 `foo`。同目录里再创建一个 `foo`，两份名字落到同一只
       文件上，清单里**两行并存**、各自自称一条备份；按 `C:foo` 删除（**200**）会把那只共用文件
       unlink，剩下的 `foo` 那一行指向已经不存在的文件，恢复它回 **404**——一次跨 id 的数据损毁
       （与档 2/4 同一族「回执说它在、入口找不到它」，只是坏在盘名而不是 URL）。换别的盘符
       （`D:evil`）则整个走出备份目录，靠 `path_lands_within` 那道第二线接住（400）；`"C:"`
       更落到 `.json`，正是档 1 要避免的那个形状。POSIX 上 `C:foo.json` 是合法文件名 ⇒
       同一个 id 在两个平台上是两个落点，按更严的一边收。

    Args:
        label: 判决文案里的名称称呼（如「备份名称」），调用方看到的仍是自己写的那一格
        value: 传入的名字

    Returns:
        进入时的同一个字符串（不改写、不裁剪、不剥空白——裁剪会造出第二个 id 面）

    Raises:
        DataValidationError: 八档任一命中。落码沿用既有契约：HTTP 面由 `to_http_error` /
            `raise_boundary_error` 回 **400** 并把判决原文照回客户端。
    """
    if not isinstance(value, str) or is_blank_string(value):
        raise DataValidationError(f"{label}必须是非空字符串，当前是 {repr(value)[:120]}")
    if "/" in value or chr(92) in value:
        raise DataValidationError(f"{label}不能含路径分隔符，否则寻不了址: {value[:120]}")
    # 冒号档（L279 / A366 的续格）：`pathlib` 把 `C:foo` 当盘符前缀吞掉 ⇒ 落点名 ≠ 回执 id，
    # 同目录里 `C:foo` 与 `foo` 两份名字落到同一只文件，索引里两行并存，按其中一个 id 删除就毁掉另一条的载荷。
    if ":" in value:
        raise DataValidationError(
            f"{label}不能含冒号：pathlib 会把开头的盘符前缀吞掉，落点名与回执 id 不同: {value[:120]}"
        )
    if value in (".", ".."):
        raise DataValidationError(f"{label}不能是相对父目录的写法: {value}")
    if "#" in value or "?" in value:
        raise DataValidationError(
            f"{label}不能含 URL 结构字符 # 与 ?，它们会把 id 截成前半段: {value[:120]}"
        )
    if any(ord(ch) < 32 or ord(ch) == 127 for ch in value):
        raise DataValidationError(f"{label}不能含控制字符: {repr(value)[:120]}")
    if value != value.rstrip(" ."):
        raise DataValidationError(
            f"{label}不能以点或空白结尾，文件系统会静默裁剪它: {repr(value)[:120]}"
        )
    if len(value) > 255:
        raise DataValidationError(f"{label}超过单个路径段上限 255 字符: {len(value)}")
    return value


# ---------- 模式串（A345 / L278）----------

#: 模式串长度上限。`method="regex"` 的 `query` 是「编译一次、对每条候选字段的值各跑
#: 一遍 `pattern.search`」的输入，长度直接乘进回溯状态数；改前 `SearchRequest.query`
#: 是无约束 `str`，实测 200,000 字照收、模型层不拦（A345 由 L267 立项）。
#: 取 1024 的理由：数据集检索里真正在用的模式（关键词、`.*` 分隔的短语、字段名）在
#: 几十到几百字符量级，1024 已经容得下把整份小词表写进一个 `(a|b|c|…)` 里；超过它的
#: 模式串更可能是构造品而不是请求。这一刀只治「长度乘进状态数」那一维，
#: **不治**形状——`(a+)+b` 只有 7 个字符，所以还需要下面那把静态闸。
MAX_PATTERN_LENGTH = 1024

#: 组扩展记号里合法的 inline flag 集（`(?aiLmsux-imsx:…)`）。
_GROUP_FLAGS = "aiLmsux-"

#: 重复计数里合法的数字：`re` 只吃 ASCII 0-9，`str.isdigit()` 会把 `²` 也算数字，
#: 而 `int("²")` 当场炸 `ValueError`。
_REPEAT_DIGITS = "0123456789"


def _skip_char_class(source: str, start: int) -> Optional[int]:
    """跳过 `[...]` 字符类，返回 `]` 之后的下标。

    Args:
        source: 模式串
        start: `[` 的下标

    Returns:
        类结束后的下标；**类未闭合**时返回 `None`（语法错交给 `re.compile` 报，
        本静态闸不下判断）。首位置的 `]` 与 `[^]` 里的 `]` 按 `re` 的口径算字面量。

    类里再出现 `[` 时**不**当成嵌套类：`re` 没有 POSIX 类，`[[:alpha:]]` 在它眼里是
    字符类 `[:alpha:]` 加一枚字面 `]`（实测 `findall` 在 `"a]:b[c"` 上返回 `['a]']`）。
    改前这里去找 `]]`，于是把整条模式扫到结尾并回 `None` ⇒ 「类里含 `[`」的指数形
    （实测 `([ab[]+)+b` 与基准 `(a+)+b` 同曲线）整族漏判。
    """
    i = start + 1
    if source[i:i + 1] == "^":
        i += 1
    if source[i:i + 1] == "]":
        i += 1
    while i < len(source):
        ch = source[i]
        if ch == "\\":
            i += 2
            continue
        if ch == "]":
            return i + 1
        i += 1
    return None


def _read_repeat_bound(source: str, start: int) -> Optional[tuple]:
    """从 `{` 处读一个**重复计数**量词。

    只认 `re` 真正当量词的四种写法：`{n}` / `{n,}` / `{n,m}` / `{,m}`（`{,}` 实测等价
    `{0,}`）。改前这里用 `int(body)` 收尾，而 `body` 只验过「字符是数字或 `, `」⇒
    `{1,2,3}` 把 `int("2,3")`、`{ }` 把 `int(" ")` 炸成 `ValueError`：它既不是
    `DataValidationError` 也不是 `re.error`，`_search_regex` 的 `except re.error`
    接不住 ⇒ 域外异常逃出产品入口（HTTP 面 500）。

    Args:
        source: 模式串
        start: `{` 的下标

    Returns:
        `(结束下标, 下界, 上界)`，上界 `None` 表示开区间 `{n,}`；**不是合法重复计数**
        时返回 `None`（`re` 会把 `{` 当字面量：`{}` / `{a}` / `{1, 2}` 这类含空格 /
        两个逗号 / 非 ASCII 数字）。`{3,2}` 那一形 `re` 直接报错，这里也回 `None`，
        报错由编译那把闸出声。
    """
    end = source.find("}", start + 1)
    if end < 0 or end - start > 21:
        return None
    body = source[start + 1:end]
    if not body or body.count(",") > 1:
        return None
    if any(ch not in _REPEAT_DIGITS and ch != "," for ch in body):
        return None
    low, comma, high = body.partition(",")
    lo = int(low) if low else 0
    hi = int(high) if high else (None if comma else lo)
    if hi is not None and hi < lo:
        return None
    return end + 1, lo, hi


def _read_quantifier(source: str, start: int) -> Optional[tuple]:
    """读 `start` 处的一个量词，判它的两维。

    两维是本静态闸唯一用到的：
    - **能重复到 ≥2 次**（`+` / `*` / `{n,}` 含 `{0,}` 与 `{,}` / `{n,m}`(m≥2) / `{k}`(k≥2)）：
      只有重复到两次以上，组内的可变长才会变成「同一串换一种切法」的组合爆炸。
      改前开区间还额外要求 `n != 0`，于是把等价于 `*` 的 `{0,}` 打成了不可重复 ⇒
      `((a+)){0,}b`、`(a+){0,}b` 实测与基准 `(a+)+b` 同曲线（n=14/16/18/20/22 为
      0.00074/0.00242/0.00864/0.03659/0.14244 秒，每 +2 字 ×4）却整族漏判。
    - **可变长**（`+` / `*` / `{n,}` / `{n,m}`(m>n)）：切点不唯一才叫可变长。
      **`?` 与 `{0,1}` / `{,1}` 不算**：`{0,1}` 就是 `?` 的另一写法，它在 CPython 的
      空匹配推进护栏下实测线性（`(a?)+b` 与 `(a{0,1})+b` 在 22 字上都是 0.00000 秒），
      把它算进来只会把「本闸明知线性且刻意放行」的那一族变成误伤；`{k}` 是定长，
      它不产生切点歧义。

    Args:
        source: 模式串
        start: 量词起始下标

    Returns:
        `(结束下标, 能重复到≥2次, 可变长, 量词文本)`；`{` 读不出重复计数时返回 `None`
        （由调用方按字面量处理）。
    """
    ch = source[start]
    if ch in "+*":
        return start + 1, True, True, ch
    if ch == "?":
        return start + 1, False, False, ch
    bound = _read_repeat_bound(source, start)
    if bound is None:
        return None
    end, lo, hi = bound
    text = source[start:end]
    can_repeat = hi is None or hi >= 2
    variable = (hi is None or hi > lo) and (lo, hi) != (0, 1)
    return end, can_repeat, variable, text


def _flag_directive_end(source: str, start: int) -> Optional[int]:
    """`(?aiLmsux)` 这一形（**纯开关指令**，不是包住内容的组）的结束下标。

    `(?i)` 后面没有 `:` 也没有内容，所以它不该推一个组帧进去。改前它落到
    `_group_extension_end` 的「认不出」支路并回 `None` ⇒ **整条模式放弃判断**，
    于是一个 5 字符的前缀就绕过了整把静态闸：实测 `(?i)(a+)+b` 与 `(?s)(a*)*c`
    都是合法正则、读数为 `None`。

    Args:
        source: 模式串
        start: `(` 的下标

    Returns:
        指令之后的下标（`)` 之后）；不是这一形时返回 `None`。
    """
    if not source.startswith("(?", start):
        return None
    i = start + 2
    while i < len(source) and source[i] in _GROUP_FLAGS:
        i += 1
    if i == start + 2 or source[i:i + 1] != ")":
        return None
    return i + 1


def _group_extension_end(source: str, start: int) -> Optional[int]:
    """跳过 `(?` 之后的组扩展记号，返回记号结束后的下标。

    认得的形状：`(?:` / `(?=` / `(?!` / `(?<=` / `(?<!` / `(?P<name>` / `(?P=name)` /
    `(?<name>` / `(?flags:`。纯开关指令 `(?flags)` 由 `_flag_directive_end` 先行认走。

    Args:
        source: 模式串
        start: `(` 的下标

    Returns:
        记号之后的下标；普通捕获组直接返回 `(` 之后。**认不出来**时返回 `start + 2`，
        即把 `(?` 当成一个普通组的开头**继续扫**；只有「名字没写完就到底」
        （`(?P<` / `(?<x` 缺 `>`）才返回 `None` 让调用方不下判断。

        改前认不出就回 `None`，而调用方据此**放弃整条模式** ⇒ 攻击者在危险形状前面
        挂一枚合法但不认识的扩展记号就能废掉整把闸：实测 `()(?(1)a)(a+)+b` 与
        `(?i)(a+)+b` 都是合法正则、改前读数都是 `None`。按组开头继续扫不会让栈失衡
        （吃掉的 `(` 总有一个 `)` 来配），所以「认不出」只可能变成「这一形判得不准」，
        不可能再变成「整条不判」。
    """
    if not source.startswith("(?", start):
        return start + 1
    i = start + 2
    head = source[i:i + 1]
    if head == "P":
        i += 1
        marker = source[i:i + 1]
        if marker == "<":
            end = source.find(">", i)
            return None if end < 0 else end + 1
        if marker == "=":
            return i + 1
        return start + 2
    if head in ("=", "!"):
        return i + 1
    if head == "<":
        nxt = source[i + 1:i + 2]
        if nxt in ("=", "!"):
            return i + 2
        end = source.find(">", i)
        return None if end < 0 else end + 1
    while i < len(source) and source[i] in _GROUP_FLAGS:
        i += 1
    if source[i:i + 1] == ":":
        return i + 1
    return start + 2


def find_repeated_ambiguous_group(source: str) -> Optional[tuple]:
    r"""静态找出「**可变长的组被重复到 ≥2 次**」这一族形状（栈式单遍扫描）。

    判据：一个括号组在它的某条路径上含可变长片段（`x+` / `x*` / `x{n,}` / `x{n,m}`(m>n)，
    或一个本身可变长的子组），而它自己又被能重复到 ≥2 次的量词套住 ⇒ 判危险。
    `(a+)+b`、`(.*)*c`、`(a*){2,}`、`((a+))+`、`(a|a+)+b`、`(\w+\s?)*`、`([0-9]+)+x`
    都在这一族里；`(foo|bar)+`、`(abc){3}`、`(?:\d{2}){4}`、`colou?r` 不在。

    复杂度对**模式串本身**线性（每个字符至多看两次），不看语料、不跑 `re` 引擎，
    所以它自己不可能成为新的 DoS 面。

    Args:
        source: 模式串（调用方给的字面量，未经编译）

    Returns:
        命中标记形状时返回 `(组的起始列, 组内可变长片段, 被重复的那个片段, 量词文本)`；
        无命中返回 `None`。`None` 里还混着**极少数**「扫不下去就不下判断」的形状：
        括号不配对、字符类未闭合、组名没写完就到底、结尾裸反斜杠——它们全都同时是
        `re.compile` 的语法错，所以到不了匹配那一步。认不出的 `(?` 扩展**不在**这一列
        （改前在，于是一枚扩展记号就能废掉整把闸，见 `_group_extension_end`）。
    """
    root = {"col": -1, "ambiguous": None, "unit": None}
    stack = [root]
    i = 0
    n = len(source)
    while i < n:
        ch = source[i]
        frame = stack[-1]
        if ch == "\\":
            if i + 1 >= n:
                return None
            frame["unit"] = (False, False, source[i:i + 2], i, None)
            i += 2
            continue
        if ch == "[":
            end = _skip_char_class(source, i)
            if end is None:
                return None
            frame["unit"] = (False, False, source[i:end], i, None)
            i = end
            continue
        if ch == "(":
            if source.startswith("(?#", i):
                end = source.find(")", i + 4)
                if end < 0:
                    return None
                frame["unit"] = (False, False, source[i:end], i, None)
                i = end + 1
                continue
            directive = _flag_directive_end(source, i)
            if directive is not None:
                frame["unit"] = (False, False, source[i:directive], i, None)
                i = directive
                continue
            end = _group_extension_end(source, i)
            if end is None:
                return None
            stack.append({"col": i, "ambiguous": None, "unit": None})
            i = end
            continue
        if ch == ")":
            if len(stack) == 1:
                return None
            closed = stack.pop()
            group_src = source[closed["col"]:i + 1]
            variable = closed["ambiguous"] is not None
            parent = stack[-1]
            if variable and parent["ambiguous"] is None:
                # 可变长的子组本身就是「父组里那个可变长片段」：`((a+))+` 要在最外层
                # 那把 `+` 上红，所以父帧的歧义源得从子组继承，不能只认直接写出的量词。
                parent["ambiguous"] = group_src
            parent["unit"] = (variable, True, group_src, closed["col"],
                              closed["ambiguous"])
            i += 1
            continue
        if ch == "|":
            frame["unit"] = None
            i += 1
            continue
        if ch in "+*?{":
            quant = _read_quantifier(source, i)
            if quant is None:
                frame["unit"] = (False, False, ch, i, None)
                i += 1
                continue
            end, can_repeat, variable, text = quant
            if frame["unit"] is None:
                # 前面没有可重复的单元（`^+` 这类）⇒ 语法错，交给 re.compile 报
                i = end
                continue
            was_variable, is_group, unit_src, unit_col, inner_src = frame["unit"]
            if can_repeat and is_group and was_variable:
                return (unit_col, inner_src, unit_src, text)
            if variable:
                if frame["ambiguous"] is None:
                    frame["ambiguous"] = inner_src or source[unit_col:end]
                frame["unit"] = (True, is_group, source[unit_col:end], unit_col,
                                 inner_src or source[unit_col:end])
            else:
                frame["unit"] = (was_variable, is_group, unit_src, unit_col, inner_src)
            i = end
            continue
        frame["unit"] = (False, False, ch, i, None)
        i += 1
    if len(stack) != 1:
        return None
    return None


def require_search_pattern(name: str, value: Any) -> str:
    """校验「被当成正则模式来用」的入参，返回原串。

    两把静态闸，都在**编译之前**、都不看语料：

    1. 非空字符串：走 `require_string`（`None` / 非串 / 空串 / 纯空白一律拒）。
       改前 `query=None` 会让 `re.compile` 抛 `TypeError` —— 那是 `except re.error`
       接不住的域外异常，HTTP 面降成 500 固定文案；空串则是「匹配一切」的隐式全量请求。
    2. 长度上限 `MAX_PATTERN_LENGTH`，再排掉 `find_repeated_ambiguous_group`
       认出的「可变长的组被重复」形状。

    这是**缓解不是根除**：它不看语料，所以判的是模式串的形状；换引擎（`re2`）与
    请求级超时那一半仍待拍（见账本 A345 的判决）。

    Args:
        name: 参数名，直接出现在报错里
        value: 传入的值

    Returns:
        校验通过后的原模式串

    Raises:
        DataValidationError: 不是非空字符串、超过长度上限，或含被重复的可变长组
    """
    text = require_string(name, value)
    if len(text) > MAX_PATTERN_LENGTH:
        raise DataValidationError(
            f"{name} 作为正则模式的长度 {len(text)} 超过上限 {MAX_PATTERN_LENGTH}；"
            f"模式串长度会乘进每条候选的回溯状态数"
        )
    hit = find_repeated_ambiguous_group(text)
    if hit is not None:
        col, inner, repeated, quant = hit
        raise DataValidationError(
            f"{name} 作为正则模式有指数级回溯风险：第 {col + 1} 列的 {repeated!r} "
            f"被量词 {quant!r} 重复，而它内部已含可变长片段 {inner!r}；"
            f"请把内层改成确切计数（`{{k}}`）或把重复挪到组外"
        )
    return text


#: 电子表格读回时会被当**公式**解释的前导字符（CWE-1236 触发面，A389 于 L287 按拍板① 立案）：
#: `=` 是公式出生符；`+` / `-` / `@` 在 Excel 与 LibreOffice 里同样把一串字面量当公式或函数调用
#: 执行（`-2+3` 打开即算、`@SUM(1)` 在 Apple Numbers 与旧版 Excel 里就是函数）；制表与回车前导
#: 先躲过「以数字或文本开头」的肉眼判断，再被解析器剥掉空白后长成正好那一族形状。
#: 本常数只有这一处产地，三条 CSV/TSV 写边与 xlsx 写边共引它，谁都不许手抄字符集。
SHEET_FORMULA_LEADERS = ("=", "+", "-", "@", "\t", "\r")


def is_sheet_formula_text(value: Any) -> bool:
    """这一格会不会被电子表格当成公式执行（判据只此一处，写边不许各自实现）

    只判**字符串**：数字、布尔、`None` 这些原生类型到了表格里就是原生类型，不会长成公式；
    列表与字典在 xlsx 写边已由 `excel_write.excel_cell` 先 `str()` 化，`"['a']"` 这类字面量
    不以判据里的任何字符开头，因此不需要在这一层重复处理。

    Args:
        value: 待写入单元格的值

    Returns:
        该值是否需要转义
    """
    return isinstance(value, str) and value[:1] in SHEET_FORMULA_LEADERS


def sheet_text_cell(value: Any) -> Any:
    """CSV / TSV 写边的转义：会被执行的字面串前面加一个单引号，其余原样返回

    为什么 CSV 只能加前缀：纯文本容器里没有「类型」这一维，`"=1+1"` 加引号也一样是公式，
    客户端唯一认的既有约定就是前导单引号（Excel / LibreOffice / Numbers 都把它当「这是文本」
    的标记而不显示出来）。**这一改写会进交付字节**：读回那份 CSV 拿到的是 `'=1+1`，
    这是 A389 拍板① 明确接受的面，不是疏漏。xlsx 那侧相反，见 `is_sheet_formula_text`
    在 `excel_write` 里的用法——它有真正的类型位可改，因此一个字都不必加。

    Args:
        value: 待写入 CSV / TSV 某一格的值

    Returns:
        转义后的值（需要转义时为带前导单引号的新字符串，否则是进入时的同一个对象）
    """
    return "'" + value if is_sheet_formula_text(value) else value


def sheet_text_row(row: Dict[str, Any]) -> Dict[str, Any]:
    """整行的**值**过 `sheet_text_cell`，键一字不动，返回新字典（不改调用方的记录）

    键不在本轮的判据面里：给表头加前导单引号会改变列名，而列名是读回侧用来配行的键——
    一处转义就得两处同步，否则 `csv.DictWriter` 报 extraneous keys。那一面量到之后另立
    待拍格处置（卷二 A391），这里刻意不顺手。

    Args:
        row: 一条对象记录

    Returns:
        新字典：键与进入时同序同值，值按 `sheet_text_cell` 过一遍
    """
    return {key: sheet_text_cell(item) for key, item in row.items()}


# ---------------------------------------------------------------------------
# 电子表格容器的解压放大判据（A390 · CWE-409）
# ---------------------------------------------------------------------------

#: 一个 zip 容器（xlsx 就是一个 zip）解压后允许的最大字节数：512 MiB。
#: 三条阈值一起定标，读数全部是本轮现量（样本构造与逐格数字见卷二 L288 块）：合法侧
#: 20000 行 × 50 列同字符填满的工作簿解压后 83.8 MiB，炸弹侧 114.4 MiB 与 1525.9 MiB
#: ⇒ 上限给「真而大」的档案留 6 倍余地，同时把单次展开量钉在上传上限
#: （`web.max_upload_bytes` 默认 256 MiB，量的是**压缩后**）的两倍以内。
MAX_SHEET_EXPANDED_BYTES = 512 * 1024 * 1024

#: 允许的最大解压比率（解压后字节 ÷ 压缩字节）：100。合法侧现量最高 36.9，炸弹侧现量
#: 最低 1025.4 ⇒ 两侧各留一个数量级的余地。比率只在解压后尺寸超过下面那道地板时才判。
MAX_SHEET_EXPANSION_RATIO = 100.0

#: 比率判据的地板：解压后不足 32 MiB 的容器**不判比率**。少了这道地板，一个 4 MiB 的
#: 工作簿（同字符填满那种天然比率就在 1000 以上）会被拒，而它整个展开完也只占 4 MiB
#: 内存。比率是「放大」的度量，能吃到内存的只有放大之后的绝对量。
MIN_SHEET_RATIO_JUDGMENT_BYTES = 32 * 1024 * 1024


def sheet_expansion_verdict(compressed: int, expanded: int, name: str, face: str) -> None:
    """按「解压后尺寸 + 解压比率」两条臂判一份容器会不会放大成灾，超阈值就抛

    两条臂一起用是这一格的拍板，各自管一种形状：上限那一臂管「怎么压都会吃光内存」的
    绝对量（炸弹侧现量 1525.9 MiB），比率那一臂管「压缩体很小但展开很大」的相对量
    （同族另一格 114.4 MiB 在上限之下，只有比率读得出它）。比率带地板，理由见
    `MIN_SHEET_RATIO_JUDGMENT_BYTES`。

    `face` 不是装饰：同一对判据要跑两遍，第一遍读**中央目录声明**的尺寸（不碰压缩流，
    诚实声明的炸弹在这一步就停下），第二遍读**实际解压出来的字节数**（中央目录可以谎报
    声明尺寸，实测谎报档在只查声明值的那一臂上是能过闸的，见 `safe_sheet_source` 的
    docstring）。抛出的文案点名是哪一遍读到的，调用方因此分得清「档案自己招了」和
    「档案撒了谎」。

    抛 `DataValidationError` 而不是新立一档：这条判据的语义正是那句 docstring 里的
    「取值越界」，而它在门口拿到的归宿（`api/deps.py::to_http_error` 的 `ValueError`
    支 ⇒ 400 且原文照回）与既有 Excel 读边的拒收同形。新立类型要动 `__all__` 与异常
    契约那把尺子的面，收益为零。

    Args:
        compressed: 压缩字节数（本轮量到的是中央目录里各员压缩尺寸之和）
        expanded: 解压后字节数（声明值或实际读出的字节数，取决于 `face` 那一遍）
        name: 容器名，只进报错文案
        face: 这一遍读数的来源，进报错文案（「中央目录声明」/「实际字节流」）

    Raises:
        DataValidationError: 超过上限，或（解压后大于地板且）比率超过上限
    """
    over_ceiling = expanded > MAX_SHEET_EXPANDED_BYTES
    over_ratio = (
        expanded > MIN_SHEET_RATIO_JUDGMENT_BYTES
        and expanded > compressed * MAX_SHEET_EXPANSION_RATIO
    )
    if not (over_ceiling or over_ratio):
        return
    mib = 1024 * 1024
    reason = (
        f"解压后 {expanded / mib:.1f} MiB 超过上限 {MAX_SHEET_EXPANDED_BYTES // mib} MiB"
        if over_ceiling else
        f"压缩 {compressed / mib:.1f} MiB 却解压出 {expanded / mib:.1f} MiB，"
        f"比率 {expanded / max(compressed, 1):.1f} 超过上限 "
        f"{MAX_SHEET_EXPANSION_RATIO:.0f}（解压后已大于地板 "
        f"{MIN_SHEET_RATIO_JUDGMENT_BYTES // mib} MiB）"
    )
    raise DataValidationError(
        f"{name} 的{face}读数：{reason}。电子表格容器解压放大超过判据，"
        f"请拆小工作簿或改交 CSV（三条阈值住 `augmentor/validation.py`，"
        f"本轮现量定标见循环账本 L288）"
    )
