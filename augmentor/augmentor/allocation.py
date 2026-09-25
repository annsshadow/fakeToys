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
        minimum_each: 非 0 时，只要「各份保底之和」（先被各自上限夹过）装得下 `total`，
            每份至少给这么多条；装不下时保底整条不生效（退回纯按权重摊派），绝不超发

    Returns:
        各份配额。正常情况下和 == `total`；当 `caps` 之和不足时和 < `total`
        （没有更多东西可分，这是唯一诚实的答案）。任何情况下和都不大于 `total`——
        保底只在装得下时生效，不是超发的理由
    """
    count = len(weights)
    if count == 0:
        return []
    if caps is None:
        if not minimum_each:
            return _no_cap_quota(total, weights, count)
    elif caps.count(1) == count:
        return _unit_cap_quota(total, weights, count)

    limits = list(caps) if caps is not None else [total] * count
    if minimum_each:
        # 保底的准入读「保底之和装不装得下」，不读「名额盖不盖得住份数」：上限小于
        # 保底的段吃不下 `minimum_each`，把它算进份数会既放过真正装不下的请求（超发），
        # 又误拒真正装得下的请求（份数里有吃不下保底的段）。
        floor = [minimum_each if lim >= minimum_each else lim for lim in limits]
        floor_total = sum(floor)
        if floor_total <= total:
            alloc = floor
            limits = [lim - given for lim, given in zip(limits, alloc)]
            remaining = total - floor_total
        else:
            alloc = [0] * count
            remaining = total
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


def _no_cap_quota(total: int, weights: Sequence[float], count: int) -> List[int]:
    """没有上限、没有保底时的配额：一般路径的答案，但只跑一轮、不建上限表

    可这样推：通用路径里「本轮取满并冻结」的条件是 `floor >= limits[i]`，而 `caps=None`
    把上限读成 `total`，于是 `exact = remaining * share / weight_sum <= remaining <= total`
    —— 永不撞线，`while` 必然在一轮后结束。剩下的只有两件事：落下取整，再把
    `left = remaining - sum(floor)` 个名额按小数部分从大到小发下去（平局按段序）。
    补发时也不用查「这一份是否已取满」：`left > 0` 蕴含所有 `floor < total`
    （若某份 `floor == total` 则 `sum(floor) >= total`，`left` 就非正），而
    `left < count`（各份小数部分都小于 1 ⇒ `sum(floor) > total - count`），所以份数一定够发。
    负权重按 0 读、全零权重均分，这两条与一般路径用的是同一条式子。`total` 非正时一般
    路径的 `while` 一步都不走（交出全 0，而不是负配额），这里同答。
    """
    if total <= 0:
        return [0] * count
    shares = [w if w > 0 else 0.0 for w in weights]
    weight_sum = sum(shares)
    if weight_sum <= 0:
        shares = [1.0] * count
        weight_sum = float(count)
    alloc = [0] * count
    # (小数部分的相反数, 段序)：升序排即「小数部分降序、平局按段序」
    keys = [(0.0, 0)] * count
    for pos, share in enumerate(shares):
        exact = total * share / weight_sum
        floor = int(exact)
        alloc[pos] = floor
        keys[pos] = (floor - exact, pos)
    left = total - sum(alloc)
    if left:
        keys.sort()
        # 这里每份都还有余量（`left` 非负且小于 `count`，见上面的推导），所以「取前
        # `left` 个键」与「逐段补到 `left` 归零才停」是同一批段 —— 不必在循环里再守一次
        for _, pos in keys[:left]:
            alloc[pos] += 1
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
    `minimum_each` 在上限 1 下不改变答案：保底之和 == `count` × 1（每份的上限就是 1），
    于是「保底装得下」与下面的 `total >= count` 同判，快路不必读那个参数。

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
    # 与阈值相等的那些是平局，按输入顺序补到刚好 `total` 份。不预判 `left` 是否非零：
    # 下面的守卫第一步就 `break`，外层再套一层 `if left:` 是不可达的第二道同一判断
    for i, exact in enumerate(exacts):
        if left == 0:
            break
        if exact == cut:
            quotas[i] = 1
            left -= 1
    return quotas
