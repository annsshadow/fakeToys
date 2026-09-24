"""比例 → 条数的配额分配：一处算清「余数归谁」

`int(total * 比例)` 逐份下取整会同时犯两种错：份数少交（4 组等权要 7 条 → 每组 1 条
只交 4 条），以及余数偏给某一份（`test = data[a:]` 让名义 10% 的测试集实际拿到 28.6%）。
这里用最大余数法：先落下取整，剩下的名额按小数部分从大到小补，和恰好等于 `total`；
平局按输入顺序，所以同一份输入永远得到同一份分配。

代价按**份数**收，不按条数收：`weights` 有几千份时（分层采样用 `instruction` 作组键，
每组只有 1 条 ⇒ 份数 == 行数），逐段过一遍取整 + 排序就是毫秒级。所以「每份上限都是 1」
这种最常见的形状单独走一条等价快路，见 `_unit_cap_quota`。
"""
from typing import List, Optional, Sequence

__all__ = ["largest_remainder"]


def largest_remainder(total: int,
                      weights: Sequence[float],
                      caps: Optional[Sequence[int]] = None,
                      minimum_each: int = 0) -> List[int]:
    """把 `total` 按 `weights` 切成整数份，返回与 `weights` 等长的配额列表

    Args:
        total: 要分配的条数
        weights: 每份的相对权重（负数按 0 读）
        caps: 每份的上限（例如「这一组只有 5 条数据」），装不下的名额让给其它份
        minimum_each: 非 0 时，只要名额够，每份至少给这么多条

    Returns:
        各份配额。正常情况下和 == `total`；当 `caps` 之和不足时和 < `total`
        （没有更多东西可分，这是唯一诚实的答案）
    """
    count = len(weights)
    if count == 0:
        return []
    if caps is not None and caps.count(1) == count:
        return _unit_cap_quota(total, weights, count)

    limits = list(caps) if caps is not None else [total] * count
    if minimum_each and total >= count:
        alloc = [minimum_each if limits[i] >= minimum_each else limits[i]
                 for i in range(count)]
        for i in range(count):
            limits[i] -= alloc[i]
        remaining = total - sum(alloc)
    else:
        alloc = [0] * count
        remaining = total

    # 一轮 = 一次带上限的最大余数法。装不下的那份在本轮就被取满并冻结，
    # 所以轮数不超过份数：耗时与 `total` 无关，只与份数平方相关。
    while remaining > 0:
        open_idx = [i for i, room in enumerate(limits) if room > 0]
        if not open_idx:
            break
        shares = [w if w > 0 else 0.0 for w in map(weights.__getitem__, open_idx)]
        weight_sum = sum(shares)
        if weight_sum <= 0:
            # 全零权重（或只剩零权重段）时按份数均分，比「一条都不给」诚实
            shares = [1.0] * len(shares)
            weight_sum = float(len(shares))
        take = [0] * len(shares)
        # (小数部分的相反数, 段序) 与下取整在同一次遍历里落下：`remaining` 与
        # 权重都非负，故 `exact >= 0`，下取整不会再被 0 夹一次。
        keys = [(0.0, 0)] * len(shares)
        for pos, share in enumerate(shares):
            exact = remaining * share / weight_sum
            floor = int(exact)
            room = limits[open_idx[pos]]
            take[pos] = floor if floor < room else room
            keys[pos] = (floor - exact, pos)
        left = remaining - sum(take)
        # 平局（小数部分相同）按段序升序补，保证同一份输入永远得到同一份分配
        keys.sort()
        for _, pos in keys:
            if left == 0:
                break
            i = open_idx[pos]
            if take[pos] >= limits[i]:
                continue
            take[pos] += 1
            left -= 1
        for pos, i in enumerate(open_idx):
            alloc[i] += take[pos]
            limits[i] -= take[pos]
        remaining = left

    return alloc


def _unit_cap_quota(total: int, weights: Sequence[float], count: int) -> List[int]:
    """每份上限都是 1 时的配额：一般路径的答案，但只排一次序、不逐段补名额

    可这样推：上限 1 ⇒ 配额只能是 0 或 1。一般路径里「本轮取满」⇔ `exact >= 1`，
    这些段必然各占掉 `total` 里的一个名额；没取满的那批 `exact < 1`，其小数部分就是
    `exact` 本身，所以「补名额」的次序 == 按 `exact` 降序、平局按段序。两支合起来，
    选中集恰是「按 `exact` 降序、平局按输入顺序的前 `total` 段」，一次排序取阈值即可。

    排序键必须是 `exact` 而**不是**原始权重：`total * 权重 / 权重和` 会把两个不同的
    float 权重舍入成同一个 `exact`（相邻 double 的权重族就会撞，见用例
    `test_fast_path_compares_the_same_float_as_the_general_path`），那时一般路径按
    平局处理（先到先得），按权重排序却会挑后出现的那个。
    `minimum_each` 在上限 1 下只能是「每份都给 1」或「无效」，已由 `total >= count` 一支盖住。

    等权重是「全体 `exact` 相同 ⇒ 全体平局 ⇒ 前 `total` 份」的特例，可以在排序前短路掉。
    真实分层采样正落在这条支路上：`weights` 与 `caps` 都是组大小，每份上限都是 1 就意味着
    每组只有 1 条，于是权重必然全相等。注意 `list.count` 带身份快路径，`NaN` 只有在
    **不同对象**之间才判不出等权 —— 两种写法的答案都与一般路径同（见
    `test_nan_weight_is_read_as_zero_like_the_general_path`），但走的支路不同。
    """
    if total >= count:
        return [1] * count
    if total <= 0:
        return [0] * count
    if weights.count(weights[0]) == count:
        return [1] * total + [0] * (count - total)
    shares = [w if w > 0 else 0.0 for w in weights]
    weight_sum = sum(shares)
    if weight_sum <= 0:
        shares = [1.0] * count
        weight_sum = float(count)
    exacts = [total * s / weight_sum for s in shares]
    cut = sorted(exacts, reverse=True)[total - 1]
    quotas = [1 if e > cut else 0 for e in exacts]
    left = total - sum(quotas)
    if left:
        # 与阈值相等的那些是平局，按输入顺序补到刚好 `total` 份
        for i, exact in enumerate(exacts):
            if left == 0:
                break
            if exact == cut:
                quotas[i] = 1
                left -= 1
    return quotas
