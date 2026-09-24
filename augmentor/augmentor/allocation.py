"""比例 → 条数的配额分配：一处算清「余数归谁」

`int(total * 比例)` 逐份下取整会同时犯两种错：份数少交（4 组等权要 7 条 → 每组 1 条
只交 4 条），以及余数偏给某一份（`test = data[a:]` 让名义 10% 的测试集实际拿到 28.6%）。
这里用最大余数法：先落下取整，剩下的名额按小数部分从大到小补，和恰好等于 `total`；
平局按输入顺序，所以同一份输入永远得到同一份分配。
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

    limits = [total if caps is None else caps[i] for i in range(count)]
    if minimum_each and total >= count:
        base = [min(minimum_each, limits[i]) for i in range(count)]
    else:
        base = [0] * count

    alloc = list(base)
    room = [limits[i] - base[i] for i in range(count)]
    remaining = max(0, total - sum(base))
    # 一轮 = 一次带上限的最大余数法。装不下的那份在本轮就被取满并冻结，
    # 所以轮数不超过份数：耗时与 `total` 无关，只与份数平方相关。
    while remaining > 0:
        open_idx = [i for i in range(count) if room[i] > 0]
        if not open_idx:
            break
        shares = [max(0.0, weights[i]) for i in open_idx]
        weight_sum = sum(shares)
        if weight_sum <= 0:
            # 全零权重（或只剩零权重段）时按份数均分，比「一条都不给」诚实
            shares = [1.0] * len(open_idx)
            weight_sum = float(len(open_idx))
        exact = [remaining * shares[pos] / weight_sum
                 for pos in range(len(open_idx))]
        take = [max(0, min(room[open_idx[pos]], int(exact[pos])))
                for pos in range(len(open_idx))]
        left = remaining - sum(take)
        # 平局（小数部分相同）按输入顺序补，保证同一份输入永远得到同一份分配
        order = sorted(range(len(open_idx)),
                       key=lambda pos: (-(exact[pos] - int(exact[pos])), pos))
        for pos in order:
            if left == 0:
                break
            if take[pos] >= room[open_idx[pos]]:
                continue
            take[pos] += 1
            left -= 1
        for pos, i in enumerate(open_idx):
            alloc[i] += take[pos]
            room[i] -= take[pos]
        remaining = left

    return alloc
