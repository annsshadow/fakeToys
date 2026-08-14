## 通用互斥体子系统


started by Ingo Molnar <mingo@redhat.com>

updated by Davidlohr Bueso <davidlohr@hp.com>

### 什么是互斥体？

在 Linux 内核中，互斥体（mutex）指的是一种特定的加锁原语，用于在
共享内存系统上强制串行化，而不仅仅是在学术或类似理论教科书中表示
“互斥”（mutual exclusion）的通用术语。互斥体是一种睡眠锁，其行为
类似于二值信号量，于 2006 年[^1^] 作为信号量的替代方案被引入。这种
新的数据结构带来了若干优势，包括更简单的接口，以及当时更小的代码量
（见“缺点”一节）。

[^1^] https://lwn.net/Articles/164802/

### 实现

互斥体由 `struct mutex` 表示，定义于 include/linux/mutex.h，实现于
kernel/locking/mutex.c。这些锁使用一个原子变量（->owner）来在锁的整个
生命周期内追踪其状态。字段 owner 实际上包含指向当前锁持有者的
`struct task_struct *`，因此在当前未被持有时为 NULL。由于 task_struct
指针至少按 L1_CACHE_BYTES 对齐，其低 3 位被用来存储额外的状态（例如，
等待者链表是否非空）。在最基本的形式下，它还包括一个等待队列以及用于
串行化访问该队列的自旋锁。此外，配置了 CONFIG_MUTEX_SPIN_ON_OWNER=y
的系统会使用一个旋转（spinner）MCS 锁（->osq），详见下文（ii）。

获取互斥体时，根据锁的状态有三条可能路径：

(i) 快速路径（fastpath）：尝试通过 cmpxchg() 将 owner 替换为当前任务
    来原子地获取锁。这仅在无竞争的情况下有效（cmpxchg() 与 0UL 比较，
    因此上面 3 个状态位都必须为 0）。如果锁存在竞争，则进入下一条
    可能的路径。

(ii) 中速路径（midpath）：又称乐观自旋（optimistic spinning），在锁的
    持有者正在运行、且没有其他优先级更高（need_resched）且已就绪的
    任务时，尝试自旋以获取锁。其理由是，如果锁持有者正在运行，它很
    可能很快便会释放锁。互斥体的自旋者使用 MCS 锁排队，因此同一时刻
    只有一个自旋者能与互斥体竞争。

    MCS 锁（由 Mellor-Crummey 和 Scott 提出）是一种简单的自旋锁，具有
    公平、且每个 cpu 都自旋在本地变量上来获取锁的良好特性。它避免了
    常见的 test-and-set 自旋锁实现所带来的昂贵的缓存行颠簸（cacheline
    bouncing）。一种类 MCS 锁被专门定制用于睡眠锁实现的乐观自旋。定制
    MCS 锁的一个重要特性是：自旋者在需要重新调度时能够退出 MCS 自旋锁
    队列。这进一步有助于避免这样一种情况：需要重新调度的 MCS 自旋者在
    获得 MCS 锁之前会继续等待自旋在互斥体持有者上，结果一拿到 MCS 锁
    就直接进入慢速路径。

(iii) 慢速路径（slowpath）：最后手段，如果仍然无法获取锁，该任务会被
    加入等待队列并睡眠，直到被解锁路径唤醒。通常情况下它以
    TASK_UNINTERRUPTIBLE 状态阻塞。

虽然从形式上看内核互斥体是睡眠锁，但正是路径（ii）使它们实际上更
接近于一种混合类型。通过简单地不中断任务、忙等几个周期而非立即睡眠，
这种锁的性能在许多工作负载上都得到了显著提升。请注意，rw-semaphores
也使用了这一技术。

### 语义

互斥体子系统检查并强制执行以下规则：

    - 同一时刻只能有一个任务持有互斥体。
    - 只有持有者才能解锁互斥体。
    - 不允许多次解锁。
    - 不允许递归加锁/解锁。
    - 互斥体只能通过 API 进行初始化（见下文）。
    - 任务不得持有互斥体退出。
    - 持有锁所在的内存区域不得被释放。
    - 已持有的互斥体不得被重新初始化。
    - 互斥体不能用于硬件或软件中断上下文，例如 tasklets 和定时器。

这些语义在启用 CONFIG_DEBUG_MUTEXES 时被完全强制执行。此外，互斥体
调试代码还实现了若干其他特性，使锁调试更轻松、更快速：

    - 在调试输出中打印互斥体时，使用其符号名。
    - 获取点跟踪、函数名的符号查找、系统中所有已持有锁的列表及其打印。
    - 持有者跟踪。
    - 检测自我递归的锁并打印所有相关信息。
    - 检测多任务循环死锁并打印所有受影响的锁与任务（且仅这些任务）。

互斥体——以及大多数其他睡眠锁如 rwsems——并不会为它们所占用的内存
提供隐式引用，该引用会随 mutex_unlock() 一起释放。

[ 这与 spin_unlock() [或 completion_done()] 不同，后者可用于保证在
  spin_unlock()/completion_done() 释放锁之后，该内存不会被锁实现再
  访问。 ]

mutex_unlock() 即使在内部已经释放锁之后，仍可能访问互斥体结构——因而
另一个上下文获取该互斥体并假定 mutex_unlock() 上下文不再使用该结构是
不安全的。

互斥体使用者必须确保互斥体不会在释放操作仍在进行时被销毁——换言之，
mutex_unlock() 的调用者必须确保互斥体在 mutex_unlock() 返回之前一直
保持有效。

### 接口

```

   DEFINE_MUTEX(name);

```
```
   mutex_init(mutex);

```
```
   void mutex_lock(struct mutex *lock);
   void mutex_lock_nested(struct mutex *lock, unsigned int subclass);
   int  mutex_trylock(struct mutex *lock);

```
```
   int mutex_lock_interruptible_nested(struct mutex *lock,
				       unsigned int subclass);
   int mutex_lock_interruptible(struct mutex *lock);

```
```
   int atomic_dec_and_mutex_lock(atomic_t *cnt, struct mutex *lock);

```
```
   void mutex_unlock(struct mutex *lock);

```
```
   int mutex_is_locked(struct mutex *lock);

```
### 缺点

与其最初的设计与目的不同，`struct mutex` 是内核中最大的锁之一。例如：
在 x86-64 上它是 32 字节，而 `struct semaphore` 是 24 字节，rw_semaphore
是 40 字节。更大的结构尺寸意味着更多的 CPU 缓存与内存占用。

### 何时使用互斥体

除非互斥体的严格语义不合适，和/或临界区导致该锁无法被共享，否则应始终
优先使用互斥体而非其他任何加锁原语。
