
## Folio Queue


:Author: David Howells <dhowells@redhat.com>


 - 概述（Overview）
 - 初始化（Initialisation）
 - 添加与移除 folio（Adding and removing folios）
 - 查询 folio 的信息（Querying information about a folio）
 - 查询 folio_queue 的信息（Querying information about a folio_queue）
 - folio 队列迭代（Folio queue iteration）
 - folio 标记（Folio marks）
 - 无锁的同步生产/消费问题（Lockless simultaneous production/consumption issues）


## 概述


folio_queue 结构体构成了 folio 分段链表中的一段，该链表可用于构成一个 I/O 缓冲区。
因此，该链表可以使用 ITER_FOLIOQ 类型的 iov_iter 进行迭代。

```

	struct folio_queue {
		struct folio_queue *next;
		struct folio_queue *prev;
		...
	};

```
提供了一对指针 `next` 与 `prev`，分别指向被访问段两侧的段。虽然这是一个双向链表，但它
故意不是一个环形链表；末端段中向外的兄弟指针应为 NULL。

链表中的每个段还存储：

 - 一组有序的 folio 指针序列，
 - 每个 folio 的大小，以及
 - 每个 folio 三个 1 位的标记，

但这些不应被直接访问，因为底层数据结构可能会变化，而应使用下面列出的访问函数。

```

	#include <linux/folio_queue.h>

```
```

	#include <linux/uio.h>


```
## 初始化


```

	void folioq_init(struct folio_queue *folioq);

```
并传入指向待初始化段的指针。注意，这不一定会初始化所有的 folio 指针，因此必须小心检查
所添加的 folio 数量。


## 添加与移除 folio


可以通过调用以下函数在段结构体的下一个未使用槽位中设置 folio：

```

	unsigned int folioq_append(struct folio_queue *folioq,
				   struct folio *folio);

	unsigned int folioq_append_mark(struct folio_queue *folioq,
					struct folio *folio);

```
这两个函数都会更新所存储的 folio 计数、存储该 folio 并记录其大小。第二个函数还会为所添加
的 folio 设置第一个标记。两个函数都返回所用槽位的编号。[!] 注意，不会尝试检查容量是否
被溢出，链表也不会自动扩展。

```

	void folioq_clear(struct folio_queue *folioq, unsigned int slot);

```
这会清空数组中的该槽位，并清空该 folio 的所有标记，但不会改变 folio 计数——因此未来
访问该槽位时必须检查该槽位是否被占用。


## 查询 folio 的信息


可以使用以下函数查询特定槽位中 folio 的信息：

```

	struct folio *folioq_folio(const struct folio_queue *folioq,
				   unsigned int slot);

```
如果某个槽位中尚未设置 folio，这可能产生未定义的结果：

```

	unsigned int folioq_folio_order(const struct folio_queue *folioq,
					unsigned int slot);

	size_t folioq_folio_size(const struct folio_queue *folioq,
				 unsigned int slot);

```
第一个函数以 order 形式返回大小，第二个函数以字节数形式返回大小。


## 查询 folio_queue 的信息


可以使用以下函数检索特定段的信息：

```

	unsigned int folioq_nr_slots(const struct folio_queue *folioq);

	unsigned int folioq_count(struct folio_queue *folioq);

	bool folioq_full(struct folio_queue *folioq);

```
第一个函数返回段的最大容量。不得假设它在不同段之间不会变化。第二个函数返回添加到段中的
folio 数量，第三个函数是一个简写，用于指示该段是否已被填充到容量上限。

注意，计数与填充状态不受从段中清空 folio 的影响。它们更多地用于表示数组中有多少个槽位
已被初始化，并且假定槽位不会被复用，而是当队列被消费时该段会被丢弃。


## folio 标记


队列中的 folio 也可以被赋予标记。这些标记可用于记录诸如某个 folio 是否需要对其调用
folio_put() 之类的信息。每个 folio 可设置三个标记。

```

	void folioq_mark(struct folio_queue *folioq, unsigned int slot);
	void folioq_mark2(struct folio_queue *folioq, unsigned int slot);

```
```

	void folioq_unmark(struct folio_queue *folioq, unsigned int slot);
	void folioq_unmark2(struct folio_queue *folioq, unsigned int slot);

```
```

	bool folioq_is_marked(const struct folio_queue *folioq, unsigned int slot);
	bool folioq_is_marked2(const struct folio_queue *folioq, unsigned int slot);

```
这些标记可用于任何用途，本 API 不会对其作出解释。


## folio 队列迭代


可以使用 I/O 迭代器设施，通过一个 `ITER_FOLIOQ` 类型的 `iov_iter` 迭代器来迭代段链表。
该迭代器可以

```

	void iov_iter_folio_queue(struct iov_iter *i, unsigned int direction,
				  const struct folio_queue *folioq,
				  unsigned int first_slot, unsigned int offset,
				  size_t count);

```
可以告知它从队列中特定的段、槽位与偏移处开始。iov 迭代器函数在前进时会跟随 next 指针，
在回退时会跟随 prev 指针（在需要时）。


## 无锁的同步生产/消费问题


如果管理得当，链表可以由生产者在头部端扩展，同时由消费者在尾部端缩短，而无需加锁。
ITER_FOLIOQ 迭代器会插入适当的屏障来辅助这一点。

同时生产与消费一个链表时必须小心。如果到达最后一个段，并且 IOV 迭代器已完全消费它所
引用的 folio，那么 iov_iter 结构体将指向最后一个段，其槽位编号等于该段的容量。当该
迭代器再次被使用时，它会尝试从此处继续（如果有另一个可用段），但必须小心，以免该段在
迭代器前进之前已被消费者移除并释放。

建议队列始终至少包含一个段，即使该段从未被填充或已被完全耗尽。这可以防止头指针与尾
指针发生重叠。


## API 函数参考
