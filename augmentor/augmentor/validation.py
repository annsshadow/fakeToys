# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集验证模块

提供数据集格式验证、完整性检查等功能，以及跨模块共用的标量入参判据
（`require_count` 计数旋钮 / `require_seconds` 时长旋钮 /
`require_positive` 无量纲倍率旋钮 / `require_ratio` 0-1 比例旋钮 /
`require_bool` 开关旋钮 / `require_string` 非空字符串旋钮 /
`require_string_list` 字符串列表旋钮 / `require_choice` 封闭清单旋钮）。字符串
那一族的「整串是空白」这一刀只有一个产地 `is_blank_string`，静态校验器共引它。
"""

import json
import logging
import math
import re
from typing import List, Dict, Optional, Any, Set
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum

from augmentor.exceptions import DataValidationError

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
    if math.isnan(value):
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
    if math.isnan(value):
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
        DataValidationError: 值不是数值（`bool` 也不算）、是 NaN，或不在闭区间内
    """
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DataValidationError(
            f"{name} 必须是数值，当前是 {value!r}（{type(value).__name__}）"
        )
    if math.isnan(value):
        raise DataValidationError(f"{name} 不能是 NaN")
    if not minimum <= value <= maximum:
        raise DataValidationError(
            f"{name} 必须是 {minimum} 到 {maximum} 之间的比例，当前是 {value}"
        )
    return value


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
        if rules:
            self.rules = rules
        elif preset in self.PRESET_RULES:
            # 浅拷贝：把类级字典直接挂到实例上，任一实例改规则就会污染全部预设
            self.rules = dict(self.PRESET_RULES[preset])
        else:
            self.rules = dict(self.PRESET_RULES["basic"])
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
        """
        seen = {}
        result = []
        
        for idx, item in enumerate(items):
            value = item.get(key, "")
            if value not in seen:
                seen[value] = idx
                result.append(item)
            elif keep == "last":
                # 替换之前的记录
                old_idx = seen[value]
                for i, r in enumerate(result):
                    if r.get(key, "") == value:
                        result[i] = item
                        break
        
        return result


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
