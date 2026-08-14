
## XArray


:Author: Matthew Wilcox

## 概述


XArray 是一种抽象数据类型，行为类似于一个非常大的指针数组。它满足了哈希或
常规可调整大小数组的许多相同需求。与哈希不同，它允许你以缓存高效的方式合理地
前往缓存中下一个或上一个条目。与可调整大小的数组相比，不需要为了扩展数组而复制
数据或更改 MMU 映射。它比双向链表更节省内存、可并行化且对缓存友好。它利用
RCU 来执行无需加锁的查找。

当所使用的索引密集聚集时，XArray 的实现是高效的；对对象进行哈希并将哈希用作
索引的表现不会很好。XArray 针对小索引进行了优化，但在大索引时仍具有良好的性能。
如果你的索引可能大于 `ULONG_MAX`，那么 XArray 不适合你。XArray 最重要的用户
是页缓存（page cache）。

普通指针可以直接存储在 XArray 中。它们必须按 4 字节对齐，这对于 kmalloc() 和
alloc_page() 返回的任何指针都是成立的。对于任意的用户空间指针或函数指针则不成立。
只要这些对象具有至少 4 的对齐，你就可以存储指向静态分配对象的指针。

你也可以在 XArray 中存储介于 0 和 `LONG_MAX` 之间的整数。你必须首先使用
xa_mk_value() 将其转换为一个条目。当你从 XArray 检索一个条目时，可以通过调用
xa_is_value() 检查它是否为值条目，并通过调用 xa_to_value() 将其转换回整数。

一些用户想要标记他们存储在 XArray 中的指针。你可以调用 xa_tag_pointer() 来
创建一个带标记的条目，使用 xa_untag_pointer() 将带标记的条目转回未标记的指针，
使用 xa_pointer_tag() 来检索一个条目的标记。带标记的指针使用与区分值条目和普通
指针相同的位，因此对于每个特定的 XArray，你必须决定是要存储值条目还是带标记的
指针。

XArray 不支持存储 IS_ERR() 指针，因为其中一些与值条目或内部条目冲突。

XArray 的一个不寻常的特性能够创建占据一个索引范围的条目。一旦存储到该范围，查找
该范围内的任何索引都将返回与查找该范围内任何其他索引相同的条目。存储到任何索引
都会存储到它们全部。多索引条目可以被显式拆分为更小的条目。清除（使用 xa_erase()
或使用 `NULL` 调用 xa_store()）任何条目都会导致 XArray 忘记该范围。

## 普通 API


首先初始化一个 XArray，对静态分配的 XArray 使用 DEFINE_XARRAY()，对动态分配的
使用 xa_init()。一个刚初始化的 XArray 在每个索引处都包含一个 `NULL` 指针。

然后你可以使用 xa_store() 设置条目，使用 xa_load() 获取条目。xa_store() 将用
新条目覆盖任何条目，并返回存储在该索引处的先前条目。你可以使用 xa_erase() 或
使用 xa_store() 将条目设置为 `NULL` 来取消设置条目。从未存储过的条目与被
xa_erase() 擦除的条目之间没有区别；最近存储了 `NULL` 的条目也是等价的，除非
XArray 是用 `XA_FLAGS_ALLOC` 初始化的。

你可以使用 xa_cmpxchg() 有条件地替换某个索引处的条目。与 cmpxchg() 类似，只有
当该索引处的条目具有 'old' 值时才会成功。它还会返回该索引处的条目；如果它返回与
作为 'old' 传入的相同条目，则 xa_cmpxchg() 成功。

如果你只想在该索引处的当前条目为 `NULL` 时向该索引存储一个新条目，可以使用
xa_insert()，如果条目不为空则返回 `-EBUSY`。

你可以通过调用 xa_extract() 将条目从 XArray 复制到一个普通数组中。或者你可以
通过调用 xa_for_each()、xa_for_each_start() 或 xa_for_each_range() 来迭代
XArray 中存在的条目。你可能更喜欢使用 xa_find() 或 xa_find_after() 移动到
XArray 中的下一个存在条目。

调用 xa_store_range() 在一段索引范围内存储相同的条目。如果你这样做，某些其他
操作的行为会略有不同。例如，标记一个索引处的条目可能导致该条目在某些（而非全部）
其他索引处被标记。存储到一个索引可能导致某些（而非全部）其他索引检索到的条目
发生改变。

有时你需要确保随后对 xa_store() 的调用不需要分配内存。xa_reserve() 函数会在
指定索引处存储一个保留条目。普通 API 的用户会将该条目视为包含 `NULL`。如果你
不需要使用该保留条目，可以调用 xa_release() 来移除未使用的条目。如果另一个用户
在此期间已存储到该条目，xa_release() 将不执行任何操作；相反，如果你希望该条目
变为 `NULL`，应该使用 xa_erase()。在保留条目上使用 xa_insert() 将失败。

如果数组中的所有条目都是 `NULL`，xa_empty() 函数将返回 `true`。

最后，你可以通过调用 xa_destroy() 从 XArray 中移除所有条目。如果 XArray 条目是
指针，你可能希望先释放这些条目。你可以通过使用 xa_for_each() 迭代器迭代 XArray
中所有存在的条目来完成此操作。

### 搜索标记


数组中的每个条目都有三个与之关联的位，称为标记。每个标记可以独立于其他标记进行
设置或清除。你可以使用 xa_for_each_marked() 迭代器来迭代带标记的条目。

你可以通过使用 xa_get_mark() 查询某个条目上是否设置了标记。如果条目不是 `NULL`，
你可以使用 xa_set_mark() 在它上面设置标记，并通过调用 xa_clear_mark() 从条目中
移除标记。你可以通过调用 xa_marked() 询问 XArray 中是否有任何条目设置了特定标记。
从 XArray 中擦除一个条目会导致与该条目关联的所有标记被清除。

在多索引条目的任何索引上设置或清除标记都会影响该条目所覆盖的所有索引。在任何索引
上查询标记将返回相同的结果。

没有办法迭代未被标记的条目；数据结构不允许高效地实现这一点。目前没有迭代器来搜索
位的逻辑组合（例如迭代所有同时设置了 `XA_MARK_1` 和 `XA_MARK_2` 的条目，或迭代
所有设置了 `XA_MARK_0` 或 `XA_MARK_2` 的条目）。如果有用户出现，可以添加这些。

### 分配 XArray


如果你使用 DEFINE_XARRAY_ALLOC() 定义 XArray，或通过向 xa_init_flags() 传递
`XA_FLAGS_ALLOC` 来初始化它，则 XArray 会更改以跟踪条目是否在使用中。

你可以调用 xa_alloc() 将条目存储在 XArray 中一个未使用的索引处。如果你需要从中断
上下文修改数组，可以使用 xa_alloc_bh() 或 xa_alloc_irq() 在分配 ID 时禁用中断。

使用 xa_store()、xa_cmpxchg() 或 xa_insert() 也会将该条目标记为已分配。与普通
XArray 不同，存储 `NULL` 会将该条目标记为在使用中，就像 xa_reserve() 一样。要
释放一个条目，使用 xa_erase()（或者如果你只想在该条目为 `NULL` 时释放它，则使用
xa_release()）。

默认情况下，从 0 开始分配最低空闲条目。如果你想从 1 开始分配条目，使用
DEFINE_XARRAY_ALLOC1() 或 `XA_FLAGS_ALLOC1` 会更高效。如果你想分配 ID 直到一个
最大值，然后回绕到最低空闲 ID，可以使用 xa_alloc_cyclic()。

你不能将 `XA_MARK_0` 用于分配型 XArray，因为此标记用于跟踪条目是否为空闲。其他
标记可供你使用。

### 内存分配


xa_store()、xa_cmpxchg()、xa_alloc()、xa_reserve() 和 xa_insert() 函数接受
一个 gfp_t 参数，以防 XArray 需要分配内存来存储此条目。如果要删除条目，则不需要
执行内存分配，指定的 GFP 标志将被忽略。

可能分配不到内存，特别是在你传递了一组限制性的 GFP 标志时。在这种情况下，函数返回
一个特殊值，可以使用 xa_err() 将其转换为 errno。如果你不需要确切知道发生了哪个
错误，使用 xa_is_err() 会稍微更高效一些。

### 加锁


使用普通 API 时，你无需担心加锁。XArray 使用 RCU 和一个内部自旋锁来同步访问：

不需要加锁：
 - xa_empty()
 - xa_marked()

获取 RCU 读锁：
 - xa_load()
 - xa_for_each()
 - xa_for_each_start()
 - xa_for_each_range()
 - xa_find()
 - xa_find_after()
 - xa_extract()
 - xa_get_mark()

内部获取 xa_lock：
 - xa_store()
 - xa_store_bh()
 - xa_store_irq()
 - xa_insert()
 - xa_insert_bh()
 - xa_insert_irq()
 - xa_erase()
 - xa_erase_bh()
 - xa_erase_irq()
 - xa_cmpxchg()
 - xa_cmpxchg_bh()
 - xa_cmpxchg_irq()
 - xa_store_range()
 - xa_alloc()
 - xa_alloc_bh()
 - xa_alloc_irq()
 - xa_reserve()
 - xa_reserve_bh()
 - xa_reserve_irq()
 - xa_destroy()
 - xa_set_mark()
 - xa_clear_mark()

假定进入时已持有 xa_lock：
 - __xa_store()
 - __xa_insert()
 - __xa_erase()
 - __xa_cmpxchg()
 - __xa_alloc()
 - __xa_set_mark()
 - __xa_clear_mark()

如果你想利用该锁来保护你存储在 XArray 中的数据结构，可以在调用 xa_load() 之前
调用 xa_lock()，然后在你找到的对象上增加引用计数，再调用 xa_unlock()。这将防止
在查找对象与增加引用计数之间，存储操作将对象从数组中移除。你也可以使用 RCU 来
避免解引用已释放的内存，但对此的解释超出了本文档的范围。

XArray 在修改数组时不会禁用中断或软中断。由于 RCU 锁提供了足够的保护，从中断或
软中断上下文读取 XArray 是安全的。

例如，如果你想在进程上下文中将条目存储在 XArray 中
```

    void foo_init(struct foo *foo)
    {
        xa_init_flags(&foo->array, XA_FLAGS_LOCK_BH);
    }

    int foo_store(struct foo *foo, unsigned long index, void *entry)
    {
        int err;

        xa_lock_bh(&foo->array);
        err = xa_err(__xa_store(&foo->array, index, entry, GFP_KERNEL));
        if (!err)
            foo->count++;
        xa_unlock_bh(&foo->array);
        return err;
    }

    /* foo_erase() 仅从软中断上下文调用 */
    void foo_erase(struct foo *foo, unsigned long index)
    {
        xa_lock(&foo->array);
        __xa_erase(&foo->array, index);
        foo->count--;
        xa_unlock(&foo->array);
    }

```
如果你要从中断或软中断上下文修改 XArray，需要使用 xa_init_flags() 通过传递
`XA_FLAGS_LOCK_IRQ` 或 `XA_FLAGS_LOCK_BH` 来初始化数组。

上面的示例还展示了一个常见模式：希望扩展存储侧的 xa_lock 覆盖范围以保护与数组
关联的一些统计信息。

与中断上下文共享 XArray 也是可能的，既可以在中断处理程序和进程上下文中都使用
xa_lock_irqsave()，也可以在进程上下文中使用 xa_lock_irq() 而在中断处理程序中使用
xa_lock()。一些更常见的模式有辅助函数，例如 xa_store_bh()、xa_store_irq()、
xa_erase_bh()、xa_erase_irq()、xa_cmpxchg_bh() 和 xa_cmpxchg_irq()。

有时你需要用互斥体保护对 XArray 的访问，因为该锁在锁层次结构中位于另一个互斥体
之上。这并不赋予你在不获取 xa_lock 的情况下使用 __xa_erase() 等函数的权利；
xa_lock 用于 lockdep 验证，将来还会用于其它目的。

__xa_set_mark() 和 __xa_clear_mark() 函数也可用于你查找一个条目并希望原子地
设置或清除标记的情况。在这种情况下使用高级 API 可能更高效，因为它可以省去两次
遍历树。

## 高级 API


高级 API 提供了更大的灵活性和更好的性能，代价是接口更难使用且保障措施更少。高级
API 不会为你做任何加锁，你需要在修改数组时使用 xa_lock。在数组上执行只读操作时，
你可以选择使用 xa_lock 或 RCU 锁。你可以在同一数组上混合使用高级和普通操作；实际上
普通 API 是基于高级 API 实现的。高级 API 仅对具有 GPL 兼容许可证的模块可用。

高级 API 围绕 xa_state 构建。这是一个不透明的数据结构，你可以使用 XA_STATE() 宏
在栈上声明它。该宏初始化 xa_state 以准备开始遍历 XArray。它用作游标来维护在 XArray
中的位置，并让你将各种操作组合在一起，而无需每次都从头重新开始。xa_state 的内容
受 rcu_read_lock() 或 xas_lock() 保护。如果你需要释放那些保护你的状态和树的锁中的
任何一个，必须调用 xas_pause()，以便未来的调用不依赖于那些未被保护的状态部分。

xa_state 也用于存储错误。你可以调用 xas_error() 来检索错误。所有操作在继续之前
都会检查 xa_state 是否处于错误状态，因此你无需在每次调用后检查错误；你可以连续进行
多次调用，仅在方便的时机关检查。目前 XArray 代码本身生成的唯一错误是 `ENOMEM` 和
`EINVAL`，但它支持任意错误，以备你想自己调用 xas_set_err()。

如果 xa_state 持有 `ENOMEM` 错误，调用 xas_nomem() 将尝试使用指定的 gfp 标志分配
更多内存，并将其缓存在 xa_state 中供下一次尝试使用。其思路是你获取 xa_lock，尝试
该操作，然后释放锁。该操作在持有锁时尝试分配内存，但更可能失败。一旦你释放了锁，
xas_nomem() 可以更努力地尝试分配更多内存。如果有必要重试该操作（即存在内存错误
**并且** 分配了更多内存），它将返回 `true`。如果它先前已分配了内存，而该内存未被
使用，并且没有错误（或不是 `ENOMEM` 的某个错误），那么它将释放先前分配的内存。

### 内部条目


XArray 保留一些条目供自己使用。这些条目从不通过普通 API 暴露，但在使用高级 API 时
有可能看到它们。通常处理它们的最佳方式是将其传递给 xas_retry()，如果它返回 `true`
则重试该操作。

   :widths: 1 1 6

   - - 名称
     - 测试
     - 用途

   - - Node
     - xa_is_node()
     - 一个 XArray 节点。使用多索引 xa_state 时可能可见。

   - - Sibling
     - xa_is_sibling()
     - 多索引条目的非规范条目。该值指示此节点中的哪个槽位包含规范条目。

   - - Retry
     - xa_is_retry()
     - 此条目当前正被持有 xa_lock 的线程修改。包含此条目的节点可能在本 RCU 周期
       结束时被释放。你应该从数组的头部重新开始查找。

   - - Zero
     - xa_is_zero()
     - 零条目通过普通 API 显示为 `NULL`，但在 XArray 中占据一个条目，可用于为将来
       的使用保留索引。分配型 XArray 将其用于为 `NULL` 的已分配条目。

未来可能会添加其它内部条目。在可能的情况下，它们将由 xas_retry() 处理。

### 附加功能


xas_create_range() 函数分配所有必要的内存来存储一个范围内的每个条目。如果无法分配
内存，它将在 xa_state 中设置 ENOMEM。

你可以使用 xas_init_marks() 将条目的标记重置为默认状态。这通常为所有标记清除，除非
XArray 被标记为 `XA_FLAGS_TRACK_FREE`，在这种情况下标记 0 被设置而所有其他标记
清除。使用 xas_store() 将一个条目替换为另一个不会重置该条目上的标记；如果你希望
标记被重置，应显式地这样做。

xas_load() 将尽可能地把 xa_state 遍历到靠近条目的位置。如果你知道 xa_state 已经被
遍历到了该条目，并且需要检查该条目是否发生了变化，可以使用 xas_reload() 来节省一次
函数调用。

如果你需要移动到 XArray 中的不同索引，调用 xas_set()。这会将游标重置到树的顶部，
通常会使下一次操作将游标遍历到树中期望的位置。如果你想移动到下一个或上一个索引，调用
xas_next() 或 xas_prev()。设置索引不会在数组中遍历游标，因此不需要持有锁，而移动到
下一个或上一个索引则需要。

你可以使用 xas_find() 搜索下一个存在的条目。这等价于 xa_find() 和 xa_find_after()
两者；如果游标已被遍历到一个条目，那么它将找到当前引用的条目的下一个条目。如果不是，
它将返回 xa_state 索引处的条目。在大多数情况下，使用 xas_next_entry() 而不是
xas_find() 移动到下一个存在的条目将节省一次函数调用，代价是生成更多内联代码。

xas_find_marked() 函数类似。如果 xa_state 尚未被遍历，它将返回 xa_state 索引处的
条目（如果被标记）。否则，它将返回 xa_state 引用的条目之后的第一个被标记的条目。
xas_next_marked() 函数等价于 xas_next_entry()。

当使用 xas_for_each() 或 xas_for_each_marked() 迭代 XArray 的一个范围时，可能
需要暂时停止迭代。xas_pause() 函数就是为此目的而存在的。在你完成必要的工作并希望
恢复后，xa_state 处于适合在你最后处理的条目之后继续迭代的状态。如果你在迭代时禁用了
中断，那么每 `XA_CHECK_SCHED` 个条目暂停迭代并重新启用中断是良好的做法。

xas_get_mark()、xas_set_mark() 和 xas_clear_mark() 函数要求 xa_state 游标已被
移动到 XArray 中的适当位置；如果你在之前立即调用了 xas_pause() 或 xas_set()，它们
将不执行任何操作。

你可以调用 xas_set_update() 让一个回调函数在 XArray 每次更新一个节点时被调用。页
缓存 workingset 代码使用它来维护其仅包含影子条目的节点列表。

### 多索引条目


XArray 有能力将多个索引绑定在一起，以便对一个索引的操作影响所有索引。例如，存储到
任何索引都会更改从任何索引检索到的条目的值。在任何索引上设置或清除标记都会设置或
清除绑定在一起的每个索引上的标记。当前的实现只允许将按 2 的幂对齐的范围绑定在一起；
例如索引 64-127 可以绑定在一起，但 2-6 不可以。这可以节省大量内存；例如将 512 个
条目绑定在一起将节省超过 4kB。

你可以使用 XA_STATE_ORDER() 或 xas_set_order() 后跟对 xas_store() 的调用来创建
多索引条目。使用多索引 xa_state 调用 xas_load() 会将 xa_state 遍历到树中的正确位置，
但返回值没有意义，即使在该范围内存储了条目，也可能是一个内部条目或 `NULL`。调用
xas_find_conflict() 将返回该范围内的第一个条目，如果范围内没有条目则返回 `NULL`。
xas_for_each_conflict() 迭代器将迭代与指定范围重叠的每个条目。

如果 xas_load() 遇到多索引条目，xa_state 中的 xa_index 不会改变。当迭代 XArray 或
调用 xas_find() 时，如果初始索引位于多索引条目的中间，它将不会被更改。后续的调用或
迭代会将索引移动到该范围内的第一个索引。每个条目只会被返回一次，无论它占据了多少个
索引。

不支持对多索引 xa_state 使用 xas_next() 或 xas_prev()。在多索引条目上使用这两个
函数中的任何一个都会暴露兄弟条目；这些应该由调用者跳过。

将 `NULL` 存储到多索引条目的任何索引都会将每个索引处的条目设置为 `NULL` 并解除绑定。
可以通过在不持有 xa_lock 的情况下调用 xas_split_alloc()，然后获取锁并调用 xas_split()
或持 xa_lock 调用 xas_try_split() 将多索引条目拆分为占据更小范围的条目。xas_split_alloc()
+ xas_split() 与 xas_try_alloc() 的区别在于，xas_split_alloc() + xas_split() 一次性
均匀地将条目从原始阶拆分为新阶，而 xas_try_split() 迭代地非均匀拆分包含该索引的条目。
例如，要拆分一个阶为 9 的条目，它占用 2^(9-6)=8 个槽位，假设 `XA_CHUNK_SHIFT` 为 6，
xas_split_alloc() + xas_split() 需要 8 个 xa_node。xas_try_split() 将阶为 9 的条目
拆分为 2 个阶为 8 的条目，然后根据给定索引，将一个阶为 8 的条目拆分为 2 个阶为 7 的
条目，……，并将一个阶为 1 的条目拆分为 2 个阶为 0 的条目。在拆分阶为 6 的条目且需要
一个新的 xa_node 时，xas_try_split() 会尽可能尝试分配一个。结果，xas_try_split() 只
需要 1 个 xa_node 而不是 8 个。

## 函数与结构
