
## 延迟与睡眠机制


本文档试图回答一个常见问题：“插入延迟的正确方式（TM）是什么？”

这个问题最常出现在驱动开发者面前，他们必须处理硬件延迟，却可能对 Linux 内核的内部运作并不十分熟悉。

下表粗略地概述了现有的函数“族”及其限制。该概览表不能替代使用前对函数说明的阅读！

   :widths: 20 20 20 20 20
   :header-rows: 2

   - -
     - `*delay()`
     - `usleep_range*()`
     - `*sleep()`
     - `fsleep()`
   - -
     - 忙等待循环
     - 基于 hrtimer
     - 基于定时器列表定时器
     - 综合上述其他函数
   - - 是否在原子上下文使用
     - 是
     - 否
     - 否
     - 否
   - - 在“短间隔”下精确
     - 是
     - 是
     - 取决于情况
     - 是
   - - 在“长间隔”下精确
     - 不要使用！
     - 是
     - 最多 12.5% 松弛
     - 是
   - - 可中断版本
     - 否
     - 是
     - 是
     - 否

对于非原子上下文，一般建议如下：

#. 不确定时一律使用 `fsleep()`（因为它综合了其他函数的所有优点）
#. 尽可能使用 `*sleep()`
#. 当 `**sleep()` 的精度不足时使用 `usleep_range**()`
#. 用于非常、非常短的延迟时使用 `*delay()`

关于这些函数“族”的更详细信息，请见后续章节。

### `*delay()` 函数族


这些函数利用对时钟速度的 jiffy 估算，并通过忙等待足够的循环次数来实现所需延迟。udelay() 是基础实现，ndelay() 和 mdelay() 是其变体。

这些函数主要用于在原子上下文中添加延迟。在原子上下文中添加延迟前，请务必先问自己：这真的有必要吗？

	:identifiers: udelay ndelay

	:identifiers: mdelay


### `usleep_range*()` 与 `*sleep()` 函数族


这些函数使用 hrtimer 或定时器列表定时器来提供所请求的睡眠时长。为了决定使用哪个函数才合适，请考虑一些基本信息：

#. hrtimer 开销更大，因为它使用 rb 树（而非哈希）
#. 当所请求的睡眠时长是第一个定时器（意味着需要编程真实硬件）时，hrtimer 开销更大
#. 定时器列表定时器由于基于 jiffy，总是提供某种程度的松弛

这里再次重复通用建议：

#. 不确定时一律使用 `fsleep()`（因为它综合了其他函数的所有优点）
#. 尽可能使用 `*sleep()`
#. 当 `**sleep()` 的精度不足时使用 `usleep_range**()`

首先请查阅 fsleep() 函数说明；要进一步了解精度，请查阅 msleep() 函数说明。


#### `usleep_range*()`


	:identifiers: usleep_range usleep_range_idle

	:identifiers: usleep_range_state


#### `*sleep()`


       :identifiers: msleep msleep_interruptible

	:identifiers: ssleep fsleep
