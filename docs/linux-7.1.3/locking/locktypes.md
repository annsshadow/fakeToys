
## 锁类型及其规

## 引言

内核提供了多种锁原语，可分为三类
 - 睡眠锁（Sleeping locks - CPU 本地锁（CPU local locks - 自旋锁（Spinning locks
本文档从概念上描述这些锁类型，并提供它们的嵌套规则，包括PREEMPT_RT 下使用的规则

## 锁类

### 鐫＄湢閿。
睡眠锁只能在可抢占的任务上下文中获取
尽管实现允许从其它上下文进行 try_lock()，但有必要仔细评unlock() 以及 try_lock() 的安全性此外，也有必要评估这些原语的调试版本。简而言之，不要从其它上下文获取睡眠锁，除非别无选择
睡眠锁类型：

 - mutex
 - rt_mutex
 - semaphore
 - rw_semaphore
 - ww_mutex
 - percpu_rw_semaphore

PREEMPT_RT 内核上，以下锁类型被转换为睡眠锁
 - local_lock
 - spinlock_t
 - rwlock_t


### CPU 本地
 - local_lock

在非 PREEMPT_RT 内核上，local_lock 函数是禁用抢占和中断原语的封装。与其它锁机制相反，禁用抢占中断是纯粹的 CPU 本地并发控制机制，并不适合用于 CPU 间并发控制

### 自旋
 - raw_spinlock_t
 - 位自旋锁（bit spinlocks
在非 PREEMPT_RT 内核上，以下锁类型也是自旋锁
 - spinlock_t
 - rwlock_t

自旋锁隐式禁用抢占，并且解锁函数可以带有后缀以应用进一步的保护
 ===================  ====================================================
 _bh()                Disable / enable bottom halves（软中断 _irq()               Disable / enable 中断
 _irqsave/restore()   保存并禁/ 恢复中断禁用状 ===================  ====================================================


## 所有者语
除了信号量之外，上述锁类型都具有严格的所有者语义：

  获取锁的上下文（任务）必须释放它
rw_semaphore 有一个特殊接口，允许读者进行非所有者释放

## rtmutex

RT-mutex 是支持优先级继承（PI）的 mutex
由于抢占和中断禁用段的存在，PI 在非 PREEMPT_RT 内核上受到限制
即使PREEMPT_RT 内核上，PI 显然也无法抢占禁用抢占或禁用中断的代码段。相反，PREEMPT_RT 内核可抢占的任务上下文中执行大多数此类代码段，特别是中断处理程序和软中断。这种转换使spinlock_t rwlock_t 能够通过 RT-mutex 实现

## semaphore

semaphore 是一个计数信号量的实现
信号量常常既用于序列化又用于等待，但新的用例应当改用独立的序列化和等待机制，例如 mutex completion

### semaphore 涓?PREEMPT_RT

PREEMPT_RT 不改变信号量的实现，因为计数信号量没有所有者的概念，从而阻PREEMPT_RT 为信号量提供
优先级继承。毕竟，未知的所有者无法被提升优先级。因此，在信号量上阻塞可能导致优先级反转

## rw_semaphore

rw_semaphore 是一种多读者单写者锁机制
在非 PREEMPT_RT 内核上，实现是公平的，从而防止写者饥饿
rw_semaphore 默认遵守严格的所有者语义，但存在允许读者非所有者释放的特殊用途接口。这些接口独立于
内核配置工作

### rw_semaphore 涓?PREEMPT_RT

PREEMPT_RT 内核rw_semaphore 映射到基rt_mutex 的单独实现，从而改变了公平性：

  因为 rw_semaphore 写者无法将其优先级授予多个读者，一个被抢占的低优先级读者将继续持有其锁，从  使即使是高优先级的写者也会饥饿。相反，因为读者可以将其优先级授予写者，一个被抢占的低优先级写  将使其优先级被提升，直到它释放锁，从而防止该写者使读者饥饿

## local_lock

local_lock 为通过禁用抢占或中断保护的临界区提供一个具名作用域
在非 PREEMPT_RT 内核上，local_lock 操作映射到禁用和启用抢占及中断的原语
 ===============================  ======================
 local_lock(&llock)               preempt_disable()
 local_unlock(&llock)             preempt_enable()
 local_lock_irq(&llock)           local_irq_disable()
 local_unlock_irq(&llock)         local_irq_enable()
 local_lock_irqsave(&llock)       local_irq_save()
 local_unlock_irqrestore(&llock)  local_irq_restore()
 ===============================  ======================

local_lock 的具名作用域相对于常规原语有两个优点
  - 锁名允许静态分析，同时也是对保护范围的清晰文档说明，而常规原语是无作用域且不透明的
  - 如果启用lockdep，local_lock 会获得一lockmap，可用于验证保护的正确性。这可以检测例如使    preempt_disable() 作为保护机制的函数从中断或软中断上下文被调用的情况。除此之外，
    lockdep_assert_held(&llock) 与任何其它锁原语一样工作

### local_lock 涓?PREEMPT_RT

PREEMPT_RT 内核local_lock 映射到每-CPU spinlock_t，从而改变语义：

  - spinlock_t 的所有变更同样适用local_lock

### local_lock 的用
local_lock 应当用于这样的情形：在非 PREEMPT_RT 内核上，禁用抢占或中断是保护CPU 数据结构的合并发控制形式
由于 PREEMPT_RT 特定spinlock_t 语义，local_lock 不适合PREEMPT_RT 内核上用于防范抢占或中断

### CPU 本地作用域与 bottom-half

仅在内核软中断（softirq）上下文中访问的CPU 变量，不应依赖于“该上下文因不可抢占而受到隐式保护这一假设。在 PREEMPT_RT 内核上，软中断上下文是可抢占的，通过隐式上下文同步每个禁bottom-half 段会导致一个隐式的CPU“大内核锁”
local_lock_t 配合 local_lock_nested_bh() local_unlock_nested_bh() 用于加锁操作，有助于
标识加锁作用域
当启lockdep 时，这些函数验证对数据结构的访问发生在软中断上下文中。与 local_lock() 不同local_unlock_nested_bh() 不禁用抢占，并且在不使用 lockdep 时不增加开销
PREEMPT_RT 内核上，local_lock_t 表现为一把真实的锁，local_unlock_nested_bh() 对数据结构访进行序列化，从而可以移除通过 local_bh_disable() 进行的序列化

## raw_spinlock_t 涓?spinlock_t


### raw_spinlock_t

raw_spinlock_t 在所有内核（包括 PREEMPT_RT 内核）中都是严格的自旋锁实现。仅在真正的临界核心代码底层中断处理以及需要禁用抢占或中断的地方（例如，为安全访问硬件状态）使用 raw_spinlock_t。当临界非常小时，有时也可以使用 raw_spinlock_t，从而避RT-mutex 的开销

### spinlock_t

spinlock_t 的语义随 PREEMPT_RT 的状态而变化
在非 PREEMPT_RT 内核上，spinlock_t 被映射到 raw_spinlock_t，并具有完全相同的语义

### spinlock_t 涓?PREEMPT_RT

PREEMPT_RT 内核上，spinlock_t 被映射到基于 rt_mutex 的单独实现，从而改变语义：

 - 不禁用抢占
 - 与硬中断相关的后缀（spin_lock / spin_unlock 操作_irq、_irqsave / _irqrestore）不影响 CPU    中断禁用状态
 - 与软中断相关的后缀（_bh()）仍禁用软中断处理程序
   PREEMPT_RT 内核通过禁用抢占来获得此效果
   PREEMPT_RT 内核使用CPU 锁进行序列化，同时保持抢占启用。该锁禁用软中断处理程序，并防止由于
   任务抢占而导致的重入
PREEMPT_RT 内核保留所有其spinlock_t 语义
 - 持有 spinlock_t 的任务不会迁移。非 PREEMPT_RT 内核通过禁用抢占来避免迁移。PREEMPT_RT 内核则改   禁用迁移，这确保即使任务被抢占，指向CPU 变量的指针仍然有效
 - 任务状态在获取 spinlock_t 期间被保留，确保任务状态规则适用于所有内核配置。非 PREEMPT_RT 内核
   保持任务状态不变。然而，如果任务在获取期间阻塞，PREEMPT_RT 必须改变任务状态。因此，它在阻塞   保存当前任务状态，相应的锁唤醒
```

    task->state = TASK_INTERRUPTIBLE
     lock()
       block()
         task->saved_state = task->state
	 task->state = TASK_UNINTERRUPTIBLE
	 schedule()
					lock wakeup
					  task->state = task->saved_state

   其它类型的唤醒通常会无条件将任务状态设RUNNING，但这在这里不起作用，因为任务必须保持阻塞直到锁
   可用。因此，当一次非锁唤醒尝试唤醒一个阻塞等待自旋锁的任务时，它改为将保存的状态设RUNNING   然后，当锁获取完成时，锁唤醒将任务状态设为保存的状态，在此例中将其设为 RUNNING::

    task->state = TASK_INTERRUPTIBLE
     lock()
       block()
         task->saved_state = task->state
	 task->state = TASK_UNINTERRUPTIBLE
	 schedule()
					non lock wakeup
					  task->saved_state = TASK_RUNNING

					lock wakeup
					  task->state = task->saved_state

   这确保真正的唤醒不会丢失

```
## rwlock_t

rwlock_t 是一种多读者单写者锁机制
PREEMPT_RT 内核rwlock_t 实现为自旋锁，spinlock_t 的后缀规则相应适用。实现是公平的，从而防写者饥饿

### rwlock_t 涓?PREEMPT_RT

PREEMPT_RT 内核rwlock_t 映射到基rt_mutex 的单独实现，从而改变语义：

 - spinlock_t 的所有变更同样适用rwlock_t
 - 因为 rwlock_t 写者无法将其优先级授予多个读者，一个被抢占的低优先级读者将继续持有其锁，从而使即使   高优先级的写者也会饥饿。相反，因为读者可以将其优先级授予写者，一个被抢占的低优先级写者将使其优先   被提升，直到它释放锁，从而防止该写者使读者饥饿

## PREEMPT_RT 注意事项


### RT 上的 local_lock

local_lock PREEMPT_RT 内核上映射到 spinlock_t 有一些影响。例如，在非 PREEMPT_RT 内核上，以下代码
```

  local_lock_irq(&local_lock);
  raw_spin_lock(&lock);

```
```

   raw_spin_lock_irq(&lock);

```
PREEMPT_RT 内核上，这段序列会出错，因为 local_lock_irq() 被映射到CPU spinlock_t，它既不禁用
中断也不禁用抢占。以下代码序列在两者上都能完全正确地工```

  local_lock_irq(&local_lock);
  spin_lock(&lock);

```
local_lock 的另一个注意事项是，每local_lock 都有一个特定的
```

  func1()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock_1, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock_1, flags);
  }

  func2()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock_2, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock_2, flags);
  }

  func3()
  {
    lockdep_assert_irqs_disabled();
    access_protected_data();
  }

```
在非 PREEMPT_RT 内核上这能正确工作，但在 PREEMPT_RT 内核local_lock_1 local_lock_2 是互不相同的无法func3() 的调用者进行序列化。并且由local_lock_irqsave() 不禁用中断，lockdep 断言也会PREEMPT_RT 内核上触发，因为
```

  func1()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock, flags);
  }

  func2()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock, flags);
  }

  func3()
  {
    lockdep_assert_held(&local_lock);
    access_protected_data();
  }


```
### spinlock_t 涓?rwlock_t

spinlock_t rwlock_t PREEMPT_RT 内核上语义的变化有一些影响。例如，在非 PREEMPT_RT 内核```

   local_irq_disable();
   spin_lock(&lock);

```
```

   spin_lock_irq(&lock);

```
同样的道理适用rwlock_t _irqsave() 后缀变体
PREEMPT_RT 内核上，这段序列会出错，因为 RT-mutex 需要一个完全可抢占的上下文。相反，应使spin_lock_irq() spin_lock_irqsave() 及其对应的解锁函数。在中断禁用和加锁必须保持分离的情况下，
PREEMPT_RT 提供了一local_lock 机制。获local_lock 将任务固定到某个 CPU，从而允许获取每-CPU
的禁用中断锁等。然而，这种方法只应在绝对必要时使用
```

  struct foo *p = get_cpu_ptr(&var1);

  spin_lock(&p->lock);
  p->count += this_cpu_read(var2);

```
在非 PREEMPT_RT 内核上这是正确的代码，但PREEMPT_RT 内核上这会出错。spinlock_t PREEMPT_RT 特定
语义变化不允许获p->lock，因get_cpu_ptr() 隐式地禁```

  struct foo *p;

  migrate_disable();
  p = this_cpu_ptr(&var1);
  spin_lock(&p->lock);
  p->count += this_cpu_read(var2);

```
migrate_disable() 确保任务被固定到当前 CPU，进而保证只要任务保持可抢占，对 var1 var2 的每-CPU 访问
就停留在同一CPU 上
migrate_disable() 替换对于以下情况无效
```

  func()
  {
    struct foo *p;

    migrate_disable();
    p = this_cpu_ptr(&var1);
    p->val = func2();

```
这会出错，因migrate_disable() 无法防范来自
```

  func()
  {
    struct foo *p;

    local_lock(&foo_lock);
    p = this_cpu_ptr(&var1);
    p->val = func2();

```
在非 PREEMPT_RT 内核上，这通过禁用抢占来防范重入。在 PREEMPT_RT 内核上，这通过获取底层的每-CPU
自旋锁来实现

### RT 上的 raw_spinlock_t

获取 raw_spinlock_t 会禁用抢占，可能还会禁用中断，因此临界区必须避免获取常规spinlock_t rwlock_t，例如，临界区必须避免分配内存。因此，在非 PREEMPT_RT 内核上，以下代码
```

  raw_spin_lock(&lock);
  p = kmalloc(sizeof(*p), GFP_ATOMIC);

```
但在 PREEMPT_RT 内核上这段代码会失败，因为内存分配器是完全可抢占的，因此无法从真正的原子上下文调用然而，在持有常规的raw 自旋锁时调用内存分配器是完全没问题的，因为它们不会禁```

  spin_lock(&lock);
  p = kmalloc(sizeof(*p), GFP_ATOMIC);


```
### 位自旋锁

PREEMPT_RT 无法替换位自旋锁，因为单个位太小，容纳不RT-mutex。因此，位自旋锁的语义在 PREEMPT_RT
内核上被保留，从而使raw_spinlock_t 的注意事项同样适用于位自旋锁
一些位自旋锁在 PREEMPT_RT 下被替换为常规的 spinlock_t，这通过在调用点使用条件ifdef）代码变来实现。相比之下，spinlock_t 的替换不需要调用点变更。相反，头文件中的条件判断和核心锁实现使得编译器
能够透明地完成替换

## 锁类型嵌套规
最基本的规则是
  - 同一锁类别（睡眠、CPU 本地、自旋）的锁类型可以任意嵌套，只要它们遵守通用的锁排序规则以防止死锁
  - 睡眠锁类型不能嵌套在 CPU 本地锁和自旋锁类型内部
  - CPU 本地锁和自旋锁类型可以嵌套在睡眠锁类型内部
  - 自旋锁类型可以嵌套在所有锁类型内部

这些约束PREEMPT_RT 和其它情况下都适用
PREEMPT_RT spinlock_t rwlock_t 的锁类别从自旋改为睡眠，并将 local_lock 替换为每-CPU spinlock_t，这意味着它们不能在持raw spinlock 时获取。这导致以下嵌套顺序
  1) 睡眠  2) spinlock_t、rwlock_t、local_lock
  3) raw_spinlock_t 和位自旋
如果违反这些约束，lockdep 会在 PREEMPT_RT 和其它情况下都发出告警