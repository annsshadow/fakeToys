

## Maple Tree


:Author: Liam R. Howlett

## 概述


Maple Tree 是一种 B-Tree 数据类型，针对存储不重叠的范围（包括大小为 1 的范围）进行了优化。该树被设计为易于使用，不需要用户编写搜索方法。它支持以缓存高效的方式遍历一段条目范围，以及转到缓存中的上一个或下一个条目。该树还可以被置于 RCU 安全的操作模式下，从而允许并发读写。写者必须在锁上同步，锁可以是默认的自旋锁，也可以由用户设置为不同类型的外部锁。

Maple Tree 保持较小的内存占用，并且被设计为能高效地利用现代处理器缓存。大多数用户可以使用普通 API。还存在一个 maple-tree-advanced-api 用于更复杂的场景。Maple Tree 最重要的用途是跟踪虚拟内存区域。

Maple Tree 可以存储介于 `0` 和 `ULONG_MAX` 之间的值。Maple Tree 保留底部两位被设为 “10” 且小于 4096（即 2、6、10 .. 4094）的值供内部使用。如果条目可能使用保留条目，那么用户可以使用 xa_mk_value() 转换条目，并通过调用 xa_to_value() 转换回来。如果用户需要使用保留值，那么在使用 maple-tree-advanced-api 时可以对值进行转换，但普通 API 不允许这样做。

Maple Tree 也可以被配置为支持搜索给定大小（或更大）的空隙（gap）。

也支持使用 maple-tree-advanced-api 进行节点的预分配。这对于那些在无法分配时，必须保证在给定代码段内成功执行 store 操作的用户很有用。节点的分配相对较小，约为 256 字节。


## 普通 API


首先初始化一个 Maple Tree：对于静态分配的 Maple Tree 使用 DEFINE_MTREE()，对于动态分配的则使用 mt_init()。一个刚刚初始化的 Maple Tree 对范围 `0` - `ULONG_MAX` 包含一个 `NULL` 指针。目前支持两种类型的 Maple Tree：分配树（allocation tree）和常规树（regular tree）。常规树对内部节点具有更高的分支因子。分配树的分支因子较低，但允许用户从 `0` 向上或 `ULONG_MAX` 向下搜索给定大小或更大的空隙。在初始化树时传入 `MT_FLAGS_ALLOC_RANGE` 标志即可使用分配树。

然后您可以使用 mtree_store() 或 mtree_store_range() 设置条目。mtree_store() 会用新条目覆盖任何已有条目，成功时返回 0，否则返回错误码。mtree_store_range() 工作方式相同，但接受一个范围。mtree_load() 用于检索存储在给定索引处的条目。您可以使用 mtree_erase() 通过仅知道该范围内的一个值来擦除整个范围，或者使用带有 NULL 条目的 mtree_store() 调用来部分擦除一个范围或一次擦除多个范围。

如果您只想在某个范围（或索引）当前为 `NULL` 时才存储新条目，可以使用 mtree_insert_range() 或 mtree_insert()，如果范围不为空，它们会返回 -EEXIST。

您可以使用 mt_find() 从某个索引向上搜索条目。

您可以通过调用 mt_for_each() 遍历一个范围内的每个条目。您必须提供一个临时变量来存储游标。如果您想遍历树的每个元素，则可以使用 `0` 和 `ULONG_MAX` 作为范围。如果调用者要在遍历期间持有锁，那么值得查看 maple-tree-advanced-api 章节中的 mas_for_each() API。

有时有必要确保对 Maple Tree 的下一次 store 调用不分配内存，请为此用例参见 maple-tree-advanced-api。

您可以使用 mtree_dup() 复制整个 Maple Tree。这比将元素逐个插入新树更高效。

最后，您可以通过调用 mtree_destroy() 移除 Maple Tree 中的所有条目。如果 Maple Tree 的条目是指针，您可能希望先释放这些条目。

### 分配节点


分配由内部树代码处理。其他选项请参见 maple-tree-advanced-alloc。

### 加锁


您无需担心加锁问题。其他选项请参见 maple-tree-advanced-locks。

Maple Tree 使用 RCU 和内部自旋锁来同步访问：

获取 RCU 读锁：
 - mtree_load()
 - mt_find()
 - mt_for_each()
 - mt_next()
 - mt_prev()

内部获取 ma_lock：
 - mtree_store()
 - mtree_store_range()
 - mtree_insert()
 - mtree_insert_range()
 - mtree_erase()
 - mtree_dup()
 - mtree_destroy()
 - mt_set_in_rcu()
 - mt_clear_in_rcu()

如果您想利用内部锁来保护存储在 Maple Tree 中的数据结构，可以在调用 mtree_load() 之前调用 mtree_lock()，然后在调用 mtree_unlock() 之前对找到的对象增加引用计数。这将防止在查找对象和增加引用计数之间，store 操作把对象从树中移除。您也可以使用 RCU 来避免解引用已释放的内存，但对此的解释超出了本文档的范围。


## 高级 API


高级 API 提供了更大的灵活性和更好的性能，代价是接口更难使用且保障更少。
使用高级 API 时，您必须自己负责加锁。
您可以使用 ma_lock、RCU 或外部锁进行保护。
只要加锁兼容，您可以在同一个数组上混合使用高级和普通操作。maple-tree-normal-api 是基于高级 API 实现的。

高级 API 围绕 ma_state 构建，这也是 “mas” 前缀的由来。ma_state 结构体跟踪树操作，以使内部和外部树用户都更轻松。

初始化 Maple Tree 与 maple-tree-normal-api 中相同。请参见上文。

maple state 在 mas->index 和 mas->last 中分别跟踪范围的起始和结束。

mas_walk() 会遍历树到 mas->index 的位置，并根据该条目的范围设置 mas->index 和 mas->last。

您可以使用 mas_store() 设置条目。mas_store() 会用新条目覆盖任何已有条目，并返回被覆盖的第一个已有条目。范围作为 maple state 的成员 index 和 last 传入。

您可以使用 mas_erase() 擦除整个范围，方法是将 maple state 的 index 和 last 设置为要擦除的期望范围。这将擦除在该范围内找到的第一个范围，将 maple state 的 index 和 last 设为被擦除的范围，并返回该位置原先存在的条目。

您可以使用 mas_for_each() 遍历一个范围内的每个条目。如果您想遍历树的每个元素，则可以使用 `0` 和 `ULONG_MAX` 作为范围。如果锁需要被周期性释放，请参见加锁章节中的 mas_pause()。

使用 maple state 可以让 mas_next() 和 mas_prev() 表现得像树是一个链表一样。由于分支因子如此之高，缓存优化带来的好处超过了平摊的性能代价。mas_next() 将返回位于 index 处条目之后的下一个条目。mas_prev() 将返回位于 index 处条目之前的上一条目。

mas_find() 在第一次调用时会找到存在于 index 处或之上的第一个条目，并在之后的每次调用中找到下一个条目。

mas_find_rev() 在第一次调用时会找到存在于 last 处或之下的第一个条目，并在之后的每次调用中找到上一个条目。

如果用户在操作期间需要让出锁，那么必须使用 mas_pause() 暂停 maple state。

使用分配树时提供了一些额外的接口。如果您想搜索一个范围内的空隙，则可以使用 mas_empty_area() 或 mas_empty_area_rev()。mas_empty_area() 从给定的最低索引开始搜索到该范围的最大值为止。mas_empty_area_rev() 从给定的最高索引开始搜索，并向下继续到该范围的下界。


### 高级分配节点


分配通常由树在内部处理，但是如果在写入发生之前就需要进行分配，那么调用 mas_expected_entries() 将分配插入所提供的范围数量所需的最坏情况节点数。这也会导致树进入批量插入模式。一旦插入完成，在 maple state 上调用 mas_destroy() 将释放未使用的分配。


### 高级加锁


Maple Tree 默认使用自旋锁，但外部锁也可以用于树的更新。要使用外部锁，树必须用 `MT_FLAGS_LOCK_EXTERN flag` 初始化，这通常通过宏 MTREE_INIT_EXT() 完成，它接受一个外部锁作为参数。

## 函数与结构体
