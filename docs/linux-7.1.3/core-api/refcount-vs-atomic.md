## refcount_t API 与 atomic_t 的对比


## 简介


refcount_t API 的目标是提供一个最小的 API，用于实现对象的引用计数。虽然
lib/refcount.c 中通用的、与架构无关的底层实现使用了原子操作，但某些
`refcount_*()` 与 `atomic_*()` 函数在内存顺序（memory ordering）保证方面
存在若干差异。本文档概述这些差异并给出相应示例，以帮助维护者针对这些内存
顺序保证的变化校验他们的代码。

本文档中使用的术语试图遵循 tools/memory-model/Documentation/explanation.txt
中定义的正式 LKMM。

memory-barriers.txt 和 atomic_t.txt 提供了关于内存顺序（总体）以及原子操作
（具体）的更多背景信息。

## 相关的内存顺序类型


本文档中与被提及的原子操作和引用计数相关、并贯穿使用的内存顺序类型。更
全面的图景请参考 memory-barriers.txt 文档。

在没有任何内存顺序保证（即完全无序）的情况下，atomics 和 refcounters 只
提供原子性和程序顺序（po）关系（在同一 CPU 上）。它保证每个 `atomic_**()`
和 `refcount_**()` 操作都是原子的，且指令在同一 CPU 上按程序顺序执行。这
是使用 READ_ONCE()/WRITE_ONCE() 以及比较并交换（compare-and-swap）原语
实现的。

强（完整）内存顺序保证：同一 CPU 上所有先前的加载和存储（所有 po 更早的
指令）都在任何 po 更晚的指令于同一 CPU 上执行之前完成。它还保证同一 CPU 上
所有 po 更早的存储以及来自其他 CPU 的所有已传播存储，都必须在任何 po 更晚
的指令于原始 CPU 上执行之前传播到所有其他 CPU（A-cumulative 属性）。这是
使用 smp_mb() 实现的。

RELEASE 内存顺序保证：同一 CPU 上所有先前的加载和存储（所有 po 更早的指令）
都在该操作之前完成。它还保证同一 CPU 上所有 po 更早的存储以及来自其他 CPU
的所有已传播存储，都必须在 release 操作之前传播到所有其他 CPU（A-cumulative
属性）。这是使用 smp_store_release() 实现的。

ACQUIRE 内存顺序保证：同一 CPU 上所有后续的加载和存储（所有 po 更晚的指令）
都在 acquire 操作之后完成。它还保证同一 CPU 上所有 po 更晚的存储都必须在
acquire 操作执行之后传播到所有其他 CPU。这是使用 smp_acquire__after_ctrl_dep()
实现的。

引用计数的控制依赖（成功时）保证：如果成功获取了对象的引用（引用计数发生
了递增或加法，函数返回 true），则后续的存储都与此操作有序。存储上的控制
依赖不使用任何显式屏障实现，而是依赖 CPU 不会对存储进行推测执行。这仅是一
个单 CPU 关系，对其他 CPU 不提供任何保证。


## 函数对比


### 情形 1) - 非“读/修改/写”（RMW）操作


函数变化：

 - atomic_set() --> refcount_set()
 - atomic_read() --> refcount_read()

内存顺序保证变化：

 - 无（两者都完全无序）


### 情形 2) - 带 release 顺序的非“读/修改/写”（RMW）操作


函数变化：

 - atomic_set_release() --> refcount_set_release()

内存顺序保证变化：

 - 无（两者都提供 RELEASE 顺序）


### 情形 3) - 不返回值的基于递增的操作


函数变化：

 - atomic_inc() --> refcount_inc()
 - atomic_add() --> refcount_add()

内存顺序保证变化：

 - 无（两者都完全无序）

### 情形 4) - 不返回值的基于递减的 RMW 操作


函数变化：

 - atomic_dec() --> refcount_dec()

内存顺序保证变化：

 - 完全无序 --> RELEASE 顺序


### 情形 5) - 返回值、基于递增的 RMW 操作


函数变化：

 - atomic_inc_not_zero() --> refcount_inc_not_zero()
 - 无对应的 atomic 函数 --> refcount_add_not_zero()

内存顺序保证变化：

 - 完全有序 --> 成功时对存储的控制依赖


   取得对象指针的结果！

### 情形 6) - 返回值、带 acquire 顺序、基于递增的 RMW 操作


函数变化：

 - atomic_inc_not_zero() --> refcount_inc_not_zero_acquire()
 - 无对应的 atomic 函数 --> refcount_add_not_zero_acquire()

内存顺序保证变化：

 - 完全有序 --> 成功时的 ACQUIRE 顺序


### 情形 7) - 返回值、通用的基于 dec/sub 递减的 RMW 操作


函数变化：

 - atomic_dec_and_test() --> refcount_dec_and_test()
 - atomic_sub_and_test() --> refcount_sub_and_test()

内存顺序保证变化：

 - 完全有序 --> RELEASE 顺序 + 成功时的 ACQUIRE 顺序


### 情形 8) - 其他返回值、基于递减的 RMW 操作


函数变化：

 - 无对应的 atomic 函数 --> refcount_dec_if_one()
 - `atomic_add_unless(&var, -1, 1)` --> `refcount_dec_not_one(&var)`

内存顺序保证变化：

 - 完全有序 --> RELEASE 顺序 + 控制依赖


### 情形 9) - 基于锁的 RMW


函数变化：

 - atomic_dec_and_lock() --> refcount_dec_and_lock()
 - atomic_dec_and_mutex_lock() --> refcount_dec_and_mutex_lock()

内存顺序保证变化：

 - 完全有序 --> RELEASE 顺序 + 控制依赖 + 成功时持有 spin_lock()
