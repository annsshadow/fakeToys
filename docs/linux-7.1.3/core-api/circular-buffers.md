## 环形缓冲区


:Author: David Howells <dhowells@redhat.com>
:Author: Paul E. McKenney <paulmck@linux.ibm.com>


Linux 提供了若干可用于实现环形缓冲（circular buffering）的特性。这类特性有两组：

 (1) 用于确定 2 的幂大小缓冲区相关信息的便捷函数。

 (2) 当缓冲区的生产者与消费者不想共享锁时所用的内存屏障。

如下文所述，要使用这些设施，需要且仅需要一个生产者与一个消费者。通过把它们串行化，也可以处理多个生产者；通过串行化，也可以处理多个消费者。


 (*) 什么是环形缓冲区？

 (*) 测量 2 的幂缓冲区。

 (*) 在环形缓冲区中使用内存屏障。
     - 生产者。
     - 消费者。


## 什么是环形缓冲区？


首先，什么是环形缓冲区？环形缓冲区是一种固定、有限大小的缓冲区，其中包含两个索引：

 (1) 'head'（头）索引——生产者向缓冲区插入条目的位置。

 (2) 'tail'（尾）索引——消费者在缓冲区中找到下一个条目的位置。

通常当 tail 指针等于 head 指针时，缓冲区为空；当 head 指针比 tail 指针小 1 时，缓冲区为满。

添加条目时 head 索引递增，移除条目时 tail 索引递增。tail 索引绝不应超过 head 索引，并且两个索引到达缓冲区末尾时都应回绕到 0，从而允许无限量的数据流经该缓冲区。

通常，条目都具有相同的单位大小，但使用下述技巧并不严格要求如此。如果要向缓冲区放入多个条目或变长条目，索引可以一次增加多于 1，前提是两个索引都不会超过对方。不过实现者必须小心，因为大于一个单位大小的区域可能会在缓冲区末尾回绕，从而被分割成两段。

## 测量 2 的幂缓冲区


计算任意大小环形缓冲区的占用情况或剩余容量通常是一项较慢的操作，需要使用取模（除法）指令。不过，如果缓冲区大小为 2 的幂，就可以改用快得多的按位与（bitwise-AND）指令。

Linux 提供了一组用于处理 2 的幂环形缓冲区的宏。这些
```
	#include <linux/circ_buf.h>

```
这些宏包括：

```
	CIRC_SPACE(head_index, tail_index, buffer_size);

     This returns the amount of space left in the buffer[1] into which items
     can be inserted.


 (#) Measure the maximum consecutive immediate space in a buffer::

	CIRC_SPACE_TO_END(head_index, tail_index, buffer_size);

     This returns the amount of consecutive space left in the buffer[1] into
     which items can be immediately inserted without having to wrap back to the
     beginning of the buffer.


 (#) Measure the occupancy of a buffer::

	CIRC_CNT(head_index, tail_index, buffer_size);

     This returns the number of items currently occupying a buffer[2].


 (#) Measure the non-wrapping occupancy of a buffer::

	CIRC_CNT_TO_END(head_index, tail_index, buffer_size);

     This returns the number of consecutive items[2] that can be extracted from
     the buffer without having to wrap back to the beginning of the buffer.


```

这些宏名义上都会返回介于 0 与 buffer_size-1 之间的值，但是：

 (1) CIRC_SPACE*() 用于生产者一端。对生产者而言，它们返回的是下界，因为生产者控制着 head 索引，但消费者可能仍在另一个 CPU 上消耗缓冲区并移动 tail 索引。对消费者而言，它显示的是上界，因为生产者可能正忙于消耗空间。

 (2) CIRC_CNT*() 用于消费者一端。对消费者而言，它们返回的是下界，因为消费者控制着 tail 索引，但生产者可能仍在另一个 CPU 上填充缓冲区并移动 head 索引。对生产者而言，它显示的是上界，因为消费者可能正忙于清空缓冲区。

 (3) 对第三方而言，生产者和消费者对索引的写入何时变得可见，是无法保证顺序的，因为两者相互独立，且可能发生在不同的 CPU 上——因此这种情况下的结果只能算猜测，甚至可能为负数。

## 在环形缓冲区中使用内存屏障


通过在环形缓冲区中结合使用内存屏障，你可以避免：

 (1) 使用单个锁来管理缓冲区两端的访问，从而允许缓冲区同时被填充和清空；以及

 (2) 使用原子计数器操作。

这有两方：填充缓冲区的生产者，以及清空它的消费者。任何时刻应当只有一个主体在填充缓冲，也应当只有一个主体在清空缓冲，但两方可以同时操作。


### 生产者


```
	spin_lock(&producer_lock);

	unsigned long head = buffer->head;
	/* The spin_unlock() and next spin_lock() provide needed ordering. */
	unsigned long tail = READ_ONCE(buffer->tail);

	if (CIRC_SPACE(head, tail, buffer->size) >= 1) {
		/* insert one item into the buffer */
		struct item *item = buffer[head];

		produce_item(item);

		smp_store_release(buffer->head,
				  (head + 1) & (buffer->size - 1));

		/* wake_up() will make sure that the head is committed before
		 * waking anyone up */
		wake_up(consumer);
	}

	spin_unlock(&producer_lock);

```
这会指示 CPU：新条目的内容必须在 head 索引将其对消费者可见之前写入；随后指示 CPU：修改后的 head 索引必须在唤醒消费者之前写入。

注意，wake_up() 并不能保证任何形式的内存屏障，除非确实有对象被唤醒。因此我们不能依赖它来保证顺序。不过，数组中总会留一个元素为空。因此，生产者必须先生产两个元素，才可能破坏消费者当前正在读取的元素。因此，消费者连续两次调用之间的解锁-加锁对，提供了必要的顺序保证：它介于"读取表明消费者已腾出某元素的索引"与"生产者向该同一元素写入"之间。


### 消费者


```
	spin_lock(&consumer_lock);

	/* Read index before reading contents at that index. */
	unsigned long head = smp_load_acquire(buffer->head);
	unsigned long tail = buffer->tail;

	if (CIRC_CNT(head, tail, buffer->size) >= 1) {

		/* extract one item from the buffer */
		struct item *item = buffer[tail];

		consume_item(item);

		/* Finish reading descriptor before incrementing tail. */
		smp_store_release(buffer->tail,
				  (tail + 1) & (buffer->size - 1));
	}

	spin_unlock(&consumer_lock);

```
这会指示 CPU：在读取新条目之前先确保索引是最新的；随后确保 CPU 已完成对该条目的读取，再写入新的 tail 指针——该指针会抹掉该条目。

注意这里使用 READ_ONCE() 和 smp_load_acquire() 来读取对方的索引。这可以防止编译器丢弃并重新加载其缓存的值。如果你能确定对方索引只会使用一次，那么严格来说这并非必需。smp_load_acquire() 还会强制 CPU 对后续的内存访问进行排序。类似地，两种算法中都使用 smp_store_release() 来写入本线程的索引。这记录了"我们正在写入一个可能被并发读取的对象"这一事实，防止编译器对写入进行拆分（tearing），并强制相对于先前的访问进行排序。


## 延伸阅读


另见 Documentation/memory-barriers.txt，其中描述了 Linux 的内存屏障设施。
