
## 不可靠的内核锁指南


:Author: Rusty Russell

## 引言


欢迎阅读 Rusty 编写、相当不可靠的《内核锁问题指南》。本文档描述了
Linux 2.6 内核中的锁机制。

随着超线程（HyperThreading）的广泛普及，以及 Linux 内核中抢占（preemption）的引入，
每一位在内核上做开发的人都需要了解 SMP 下并发与锁的基本原理。

## 并发的问题


（如果你已经知道什么是竞态条件，可以跳过本节。）

在一个普通程序中，你可以像下面这样递增一个计数器：

```

          very_important_count++;


```
这是人们期望发生的情况：


  +------------------------------------+------------------------------------+
  | Instance 1                         | Instance 2                         |
  +====================================+====================================+
  | read very_important_count (5)      |                                    |
  +------------------------------------+------------------------------------+
  | add 1 (6)                          |                                    |
  +------------------------------------+------------------------------------+
  | write very_important_count (6)     |                                    |
  +------------------------------------+------------------------------------+
  |                                    | read very_important_count (6)      |
  +------------------------------------+------------------------------------+
  |                                    | add 1 (7)                          |
  +------------------------------------+------------------------------------+
  |                                    | write very_important_count (7)     |
  +------------------------------------+------------------------------------+

这是实际可能发生的情况：


  +------------------------------------+------------------------------------+
  | Instance 1                         | Instance 2                         |
  +====================================+====================================+
  | read very_important_count (5)      |                                    |
  +------------------------------------+------------------------------------+
  |                                    | read very_important_count (5)      |
  +------------------------------------+------------------------------------+
  | add 1 (6)                          |                                    |
  +------------------------------------+------------------------------------+
  |                                    | add 1 (6)                          |
  +------------------------------------+------------------------------------+
  | write very_important_count (6)     |                                    |
  +------------------------------------+------------------------------------+
  |                                    | write very_important_count (6)     |
  +------------------------------------+------------------------------------+


### 竞态条件与临界区


上述这种重叠，其最终结果依赖于多个任务的相对时序，被称为竞态条件
（race condition）。包含并发问题的那段代码称为临界区（critical region）。
尤其是自从 Linux 开始运行在 SMP 机器上以来，它们成了内核设计与实现中的
主要问题之一。

即便只有一个 CPU，抢占也会产生同样的效果：如果在临界区内抢占了一个任务，
我们就得到了完全相同的竞态条件。在这种情况下，抢占我们的那个线程可能
自己就运行了临界区。

解决办法是识别这些同时发生的访问，并使用锁来确保任意时刻只有一个实例能够
进入临界区。Linux 内核中有许多友好的原语可以帮助你做到这一点。当然也
有一些不那么友好的原语，不过我会假装它们不存在。

## Linux 内核中的锁


如果关于锁我只能给你一条建议，那就是：**保持简单**。

不要随意引入新的锁。

### 内核锁的两大类型：自旋锁与互斥体


内核锁主要有两种类型。最基本的是自旋锁（`include/asm/spinlock.h`），
它是一种非常简单的单人持有锁：如果你拿不到自旋锁，就会一直尝试（自旋）
直到拿到为止。自旋锁非常小巧且快速，可在任何地方使用。

第二种是互斥体（`include/linux/mutex.h`）：它很像自旋锁，但持有互斥体时
你可以睡眠。如果拿不到互斥体，你的任务会挂起自己，并在互斥体被释放时被唤醒。
这意味着在你等待期间 CPU 可以去做别的事情。很多时候你根本无法睡眠
（参见 `What Functions Are Safe To Call From Interrupts?`_），
因此不得不改用自旋锁。

这两种锁都不是可重入的：参见 `Deadlock: Simple and Advanced`_。

### 锁与单处理器内核


对于没有开启 `CONFIG_SMP`、也没有开启 `CONFIG_PREEMPT` 而编译的内核，
自旋锁根本不存在。这是一个出色的设计决策：当没有其他人能同时运行时，
就没有理由使用锁。

如果内核没有开启 `CONFIG_SMP`，但设置了 `CONFIG_PREEMPT`，那么自旋锁
仅仅是禁用抢占，这已足以防止任何竞态。在大多数情况下，我们可以把抢占
等同于 SMP，而不必单独考虑它。

你应该始终在开启 `CONFIG_SMP` 和 `CONFIG_PREEMPT` 的情况下测试你的锁代码，
即便你手头没有 SMP 测试机，因为它仍然能捕获某些类型的加锁错误。

互斥体依然存在，因为它们是用户上下文之间同步所必需的，正如我们下面
将看到的那样。

### 仅在用户上下文中加锁


如果你有一个数据结构，只会在用户上下文中被访问，那么你可以使用一个简单的
互斥体（`include/linux/mutex.h`）来保护它。这是最平凡的情形：你初始化
互斥体。然后你可以调用 mutex_lock_interruptible() 来获取互斥体，
调用 mutex_unlock() 来释放它。还有一个 mutex_lock()，
应该避免使用它，因为一旦收到信号它就不会返回。

示例：`net/netfilter/nf_sockopt.c` 允许注册新的 setsockopt() 和
getsockopt() 调用，通过 nf_register_sockopt()。注册与注销
只在模块加载和卸载时（以及启动时，那时没有并发）进行，而注册列表只在
遇到未知的 setsockopt() 或 getsockopt() 系统调用时才被查阅。
`nf_sockopt_mutex` 非常适合用来保护它，尤其是因为 setsockopt 和
getsockopt 调用很可能会睡眠。

### 用户上下文与软中断之间的加锁


如果软中断与用户上下文共享数据，你会面临两个问题。首先，当前的用户上下文
可能被软中断打断；其次，临界区也可能从另一个 CPU 进入。这时就要用到
spin_lock_bh()（`include/linux/spinlock.h`）。它先在该 CPU 上
禁用软中断，然后再获取锁。spin_unlock_bh() 做相反的事情。
（'_bh' 后缀是对“底半部”（Bottom Halves，软件中断的旧称）的历史指称。
在理想世界里它其实应该叫 spin_lock_softirq()。）

注意，这里你也可以使用 spin_lock_irq() 或 spin_lock_irqsave()，
它们同时会停止硬件中断：参见 `Hard IRQ Context`_。

这对于 UP 同样完美适用：自旋锁消失，这个宏简单地变成 local_bh_disable()
（`include/linux/interrupt.h`），它保护你不被软中断运行。

### 用户上下文与 Tasklet 之间的加锁


这与上面完全相同，因为 tasklet 实际上是从软中断中运行的。

### 用户上下文与定时器之间的加锁


这也与上面完全相同，因为定时器实际上是从软中断中运行的。从加锁的角度看，
tasklet 和定时器是完全相同的。

### Tasklet/定时器之间的加锁


有时一个 tasklet 或定时器可能想要与另一个 tasklet 或定时器共享数据。

#### 同一个 Tasklet/定时器


由于 tasklet 绝不会同时在两个 CPU 上运行，你不必担心你的 tasklet 会被
重入（同时运行两次），即使在 SMP 上也是如此。

#### 不同的 Tasklet/定时器


如果另一个 tasklet/timer 想要与你的 tasklet 或定时器共享数据，你们
二者都需要使用 spin_lock() 和 spin_unlock() 调用。
spin_lock_bh() 在这里是不必要的，因为你已经处于一个 tasklet 中，
同一个 CPU 上不会有其他 tasklet 运行。

### 软中断之间的加锁


软中断经常想要与自身或 tasklet/timer 共享数据。

#### 同一个软中断


同一个软中断可以在其他 CPU 上运行：你可以使用每-CPU 数组
（参见 `Per-CPU Data`_）来获得更好的性能。如果你都用到软中断这种程度了，
你大概足够关心可扩展性能，从而愿意承受额外的复杂度。

你需要对共享数据使用 spin_lock() 和 spin_unlock()。

#### 不同的软中断


你需要对共享数据使用 spin_lock() 和 spin_unlock()，
无论是定时器、tasklet、不同的软中断，还是相同或其他的软中断：它们中任何一个
都可能在不同的 CPU 上运行。

## 硬件 IRQ 上下文


硬件中断通常与一个 tasklet 或软中断通信。这通常涉及把工作放入一个队列，
由软中断取出。

### 硬件 IRQ 与软中断/Tasklet 之间的加锁


如果硬件 irq 处理程序与软中断共享数据，你有两个顾虑。首先，软中断处理
可能被硬件中断打断；其次，临界区可能被另一个 CPU 上的硬件中断进入。
这时就要用到 spin_lock_irq()。它被定义为先在该 CPU 上禁用中断，
然后再获取锁。spin_unlock_irq() 做相反的事情。

irq 处理程序不需要使用 spin_lock_irq()，因为软中断在 irq 处理程序
运行时不可能运行：它可以使用 spin_lock()，这样会稍快一些。
唯一的例外是如果另一个不同的硬件 irq 处理程序使用了同一把锁：
spin_lock_irq() 会阻止它打断我们。

这对于 UP 同样完美适用：自旋锁消失，这个宏简单地变成 local_irq_disable()
（`include/asm/smp.h`），它保护你不被软中断/tasklet/BH 运行。

spin_lock_irqsave()（`include/linux/spinlock.h`）是一个变体，
它把中断是开还是关保存在一个 flags 字中，该字会被传给
spin_unlock_irqrestore()。这意味着同样的代码既可以用在硬件 irq 处理程序
内部（中断已经关闭），也可以用在软中断中（需要禁用 irq）。

注意，软中断（因而也包括 tasklet 和定时器）是在从硬件中断返回时运行的，
所以 spin_lock_irq() 也会停止这些。从这个意义上说，spin_lock_irqsave()
是最通用、最强大的加锁函数。

### 两个硬件 IRQ 处理程序之间的加锁


在两个 IRQ 处理程序之间共享数据的情况很少见，但如果确实需要，应该使用
spin_lock_irqsave()：在 irq 处理程序自身内部是否禁用所有中断
是依赖体系结构的。

## 加锁速查表


Pete Zaitcev 给出了如下总结：

- 如果你处于进程上下文（任何系统调用）中，并且想要把其他进程排除在外，
   使用互斥体。你可以持有互斥体并睡眠
   （`copy_from_user()` 或 `kmalloc(x,GFP_KERNEL)`）。

- 否则（== 数据可能在中断中被触及），使用 spin_lock_irqsave() 和
   spin_unlock_irqrestore()。

- 避免持有自旋锁超过 5 行代码，并避免跨越任何函数调用
   （accessors 如 readb() 除外）。

### 最低要求表


下表列出了各种上下文之间的**最低**加锁要求。在某些情况下，同一上下文
一次只能在一个 CPU 上运行，因此该上下文不需要加锁（例如，某个特定线程
一次只能在一个 CPU 上运行，但如果它需要与另一个线程共享数据，就需要加锁）。

记住上面的建议：你始终可以使用 spin_lock_irqsave()，它是所有其他自旋锁
原语的超集。

============== ============= ============= ========= ========= ========= ========= ======= ======= ============== ==============
.              IRQ Handler A IRQ Handler B Softirq A Softirq B Tasklet A Tasklet B Timer A Timer B User Context A User Context B
============== ============= ============= ========= ========= ========= ========= ======= ======= ============== ==============
IRQ Handler A  None
IRQ Handler B  SLIS          None
Softirq A      SLI           SLI           SL
Softirq B      SLI           SLI           SL        SL
Tasklet A      SLI           SLI           SL        SL        None
Tasklet B      SLI           SLI           SL        SL        SL        None
Timer A        SLI           SLI           SL        SL        SL        SL        None
Timer B        SLI           SLI           SL        SL        SL        SL        SL      None
User Context A SLI           SLI           SLBH      SLBH      SLBH      SLBH      SLBH    SLBH    None
User Context B SLI           SLI           SLBH      SLBH      SLBH      SLBH      SLBH    SLBH    MLI            None
============== ============= ============= ========= ========= ========= ========= ======= ======= ============== ==============

Table: Table of Locking Requirements

+--------+----------------------------+
| SLIS   | spin_lock_irqsave          |
+--------+----------------------------+
| SLI    | spin_lock_irq              |
+--------+----------------------------+
| SL     | spin_lock                  |
+--------+----------------------------+
| SLBH   | spin_lock_bh               |
+--------+----------------------------+
| MLI    | mutex_lock_interruptible   |
+--------+----------------------------+

Table: Legend for Locking Requirements Table

## trylock 函数


有一些函数只尝试获取一次锁，并立即返回一个值表示获取成功或失败。如果你在
其他线程持有锁时不需要访问被该锁保护的数据，就可以使用它们。如果你之后需要
访问被该锁保护的数据，应该稍后再去获取锁。

spin_trylock() 不会自旋，如果第一次尝试就拿到了自旋锁则返回非零，
否则返回 0。这个函数可以像 spin_lock() 一样用于所有上下文：你
必须已经禁用了可能打断你的那些上下文并获取了自旋锁。

mutex_trylock() 不会挂起你的任务，如果第一次尝试就能锁定互斥体则返回非零，
否则返回 0。尽管它并不睡眠，但这个函数不能在硬件或软件中断上下文中安全使用。

## 常见示例


让我们逐步看一个简单的例子：一个“数字到名称”映射的缓存。缓存记录了每个
对象被使用的频率，并在缓存满时丢弃使用最少的那个。

### 全部在用户上下文中


在我们的第一个例子中，我们假设所有操作都在用户上下文（即来自系统调用）中，
因此我们可以睡眠。这意味着我们可以使用互斥体。

```
    #include <linux/list.h>
    #include <linux/slab.h>
    #include <linux/string.h>
    #include <linux/mutex.h>
    #include <asm/errno.h>

    struct object
    {
            struct list_head list;
            int id;
            char name[32];
            int popularity;
    };

    /* Protects the cache, cache_num, and the objects within it */
    static DEFINE_MUTEX(cache_lock);
    static LIST_HEAD(cache);
    static unsigned int cache_num = 0;
    #define MAX_CACHE_SIZE 10

    /* Must be holding cache_lock */
    static struct object *__cache_find(int id)
    {
            struct object *i;

            list_for_each_entry(i, &cache, list)
                    if (i->id == id) {
                            i->popularity++;
                            return i;
                    }
            return NULL;
    }

    /* Must be holding cache_lock */
    static void __cache_delete(struct object *obj)
    {
            BUG_ON(!obj);
            list_del(&obj->list);
            kfree(obj);
            cache_num--;
    }

    /* Must be holding cache_lock */
    static void __cache_add(struct object *obj)
    {
            list_add(&obj->list, &cache);
            if (++cache_num > MAX_CACHE_SIZE) {
                    struct object *i, *outcast = NULL;
                    list_for_each_entry(i, &cache, list) {
                            if (!outcast || i->popularity < outcast->popularity)
                                    outcast = i;
                    }
                    __cache_delete(outcast);
            }
    }

    int cache_add(int id, const char *name)
    {
            struct object *obj;

            if ((obj = kmalloc(sizeof(*obj), GFP_KERNEL)) == NULL)
                    return -ENOMEM;

            strscpy(obj->name, name, sizeof(obj->name));
            obj->id = id;
            obj->popularity = 0;

            mutex_lock(&cache_lock);
            __cache_add(obj);
            mutex_unlock(&cache_lock);
            return 0;
    }

    void cache_delete(int id)
    {
            mutex_lock(&cache_lock);
            __cache_delete(__cache_find(id));
            mutex_unlock(&cache_lock);
    }

    int cache_find(int id, char *name)
    {
            struct object *obj;
            int ret = -ENOENT;

            mutex_lock(&cache_lock);
            obj = __cache_find(id);
            if (obj) {
                    ret = 0;
                    strcpy(name, obj->name);
            }
            mutex_unlock(&cache_lock);
            return ret;
    }

```
注意，我们在添加、删除或查找缓存时总是确保持有 cache_lock：缓存基础设施
本身以及对象的内容都由这把锁保护。在这种情况下这很容易，因为我们把数据
复制给用户，从不让他们直接访问对象。

这里有一个细微（也很常见）的优化：在 cache_add() 中，我们在获取锁之前
就设置好了对象的各个字段。这是安全的，因为在把对象放进缓存之前，没有其他人
能访问它。

### 从中断上下文访问


现在考虑 cache_find() 可能从中断上下文被调用的情况：要么是硬件中断，
要么是软中断。一个例子是某个定时器会从缓存中删除对象。

下面的改动以标准补丁格式展示：`-` 开头的行是被删除的行，`+` 开头的行是
被添加的行。

```
    --- cache.c.usercontext 2003-12-09 13:58:54.000000000 +1100
    +++ cache.c.interrupt   2003-12-09 14:07:49.000000000 +1100
    @@ -12,7 +12,7 @@
             int popularity;
     };

    -static DEFINE_MUTEX(cache_lock);
    +static DEFINE_SPINLOCK(cache_lock);
     static LIST_HEAD(cache);
     static unsigned int cache_num = 0;
     #define MAX_CACHE_SIZE 10
    @@ -55,6 +55,7 @@
     int cache_add(int id, const char *name)
     {
             struct object *obj;
    +        unsigned long flags;

             if ((obj = kmalloc(sizeof(*obj), GFP_KERNEL)) == NULL)
                     return -ENOMEM;
    @@ -63,30 +64,33 @@
             obj->id = id;
             obj->popularity = 0;

    -        mutex_lock(&cache_lock);
    +        spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);
    -        mutex_unlock(&cache_lock);
    +        spin_unlock_irqrestore(&cache_lock, flags);
             return 0;
     }

     void cache_delete(int id)
     {
    -        mutex_lock(&cache_lock);
    +        unsigned long flags;
    +
    +        spin_lock_irqsave(&cache_lock, flags);
             __cache_delete(__cache_find(id));
    -        mutex_unlock(&cache_lock);
    +        spin_unlock_irqrestore(&cache_lock, flags);
     }

     int cache_find(int id, char *name)
     {
             struct object *obj;
             int ret = -ENOENT;
    +        unsigned long flags;

    -        mutex_lock(&cache_lock);
    +        spin_lock_irqsave(&cache_lock, flags);
             obj = __cache_find(id);
             if (obj) {
                     ret = 0;
                     strcpy(name, obj->name);
             }
    -        mutex_unlock(&cache_lock);
    +        spin_unlock_irqrestore(&cache_lock, flags);
             return ret;
     }

```
注意，spin_lock_irqsave() 会在中断开启时关闭中断，否则什么也不做
（如果我们已经在中断处理程序中），因此这些函数可以安全地从任何上下文中调用。

遗憾的是，cache_add() 调用了 kmalloc()，并带有 `GFP_KERNEL` 标志，
这只在用户上下文中合法。我已经假设 cache_add() 仍然只在用户上下文中被调用，
否则它应该成为 cache_add() 的一个参数。

### 把对象暴露到本文件之外


如果我们的对象包含更多信息，仅仅在内外复制信息可能就不够了：代码的其他部分
可能想要保留指向这些对象的指针，例如，而不是每次都按 id 去查找。这带来了
两个问题。

第一个问题是，我们使用 `cache_lock` 来保护对象：我们需要让这个锁变成非
静态的，以便代码的其余部分可以使用它。这让加锁变得更棘手，因为它不再全都
集中在一个地方了。

第二个问题是生命周期问题：如果另一个结构体保留了一个指向对象的指针，它大概
期望该指针保持有效。遗憾的是，这只在你持有锁期间才得到保证，否则有人可能
调用 cache_delete()，甚至更糟，添加另一个对象，复用同一地址。

由于只有一把锁，你不可能永远持有它：其他人都没法干活了。

这个问题的解决办法是使用引用计数：每个持有对象指针的人，在第一次拿到对象时
增加计数，用完时减少计数。谁把它减到零，谁就知道它已无人使用，就可以真正
删除它。

```
    --- cache.c.interrupt   2003-12-09 14:25:43.000000000 +1100
    +++ cache.c.refcnt  2003-12-09 14:33:05.000000000 +1100
    @@ -7,6 +7,7 @@
     struct object
     {
             struct list_head list;
    +        unsigned int refcnt;
             int id;
             char name[32];
             int popularity;
    @@ -17,6 +18,35 @@
     static unsigned int cache_num = 0;
     #define MAX_CACHE_SIZE 10

    +static void __object_put(struct object *obj)
    +{
    +        if (--obj->refcnt == 0)
    +                kfree(obj);
    +}
    +
    +static void __object_get(struct object *obj)
    +{
    +        obj->refcnt++;
    +}
    +
    +void object_put(struct object *obj)
    +{
    +        unsigned long flags;
    +
    +        spin_lock_irqsave(&cache_lock, flags);
    +        __object_put(obj);
    +        spin_unlock_irqrestore(&cache_lock, flags);
    +}
    +
    +void object_get(struct object *obj)
    +{
    +        unsigned long flags;
    +
    +        spin_lock_irqsave(&cache_lock, flags);
    +        __object_get(obj);
    +        spin_unlock_irqrestore(&cache_lock, flags);
    +}
    +
     /* Must be holding cache_lock */
     static struct object *__cache_find(int id)
     {
    @@ -35,6 +65,7 @@
     {
             BUG_ON(!obj);
             list_del(&obj->list);
    +        __object_put(obj);
             cache_num--;
     }

    @@ -63,6 +94,7 @@
             strscpy(obj->name, name, sizeof(obj->name));
             obj->id = id;
             obj->popularity = 0;
    +        obj->refcnt = 1; /* The cache holds a reference */

             spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);
    @@ -79,18 +111,15 @@
             spin_unlock_irqrestore(&cache_lock, flags);
     }

    -int cache_find(int id, char *name)
    +struct object *cache_find(int id)
     {
             struct object *obj;
    -        int ret = -ENOENT;
             unsigned long flags;

             spin_lock_irqsave(&cache_lock, flags);
             obj = __cache_find(id);
    -        if (obj) {
    -                ret = 0;
    -                strcpy(name, obj->name);
    -        }
    +        if (obj)
    +                __object_get(obj);
             spin_unlock_irqrestore(&cache_lock, flags);
    -        return ret;
    +        return obj;
     }

```
我们把引用计数封装在标准的两个 'get' 和 'put' 函数中。现在我们可以让
cache_find() 直接返回对象本身，这样做的好处是，用户现在可以在持有对象时
睡眠（例如用 copy_to_user() 把名字复制到用户空间）。

另一点要注意的是，我说过每个指向对象的指针都应该持有一个引用：因此当对象
首次插入缓存时，引用计数为 1。在某些版本中，该框架并不持有引用计数，但那样
会更复杂。

#### 使用原子操作实现引用计数


在实践中，`atomic_t` 通常用于 refcnt。`include/asm/atomic.h` 中定义了一组
原子操作：它们保证能从系统中的所有 CPU 以原子方式看到，因此不需要锁。在这种
情况下，它比使用自旋锁更简单，尽管对于任何不平凡的情况，使用自旋锁会更清晰。
这里使用 atomic_inc() 和 atomic_dec_and_test() 来替代标准的
递增和递减运算符，并且不再用锁来保护引用计数本身。

```
    --- cache.c.refcnt  2003-12-09 15:00:35.000000000 +1100
    +++ cache.c.refcnt-atomic   2003-12-11 15:49:42.000000000 +1100
    @@ -7,7 +7,7 @@
     struct object
     {
             struct list_head list;
    -        unsigned int refcnt;
    +        atomic_t refcnt;
             int id;
             char name[32];
             int popularity;
    @@ -18,33 +18,15 @@
     static unsigned int cache_num = 0;
     #define MAX_CACHE_SIZE 10

    -static void __object_put(struct object *obj)
    -{
    -        if (--obj->refcnt == 0)
    -                kfree(obj);
    -}
    -
    -static void __object_get(struct object *obj)
    -{
    -        obj->refcnt++;
    -}
    -
     void object_put(struct object *obj)
     {
    -        unsigned long flags;
    -
    -        spin_lock_irqsave(&cache_lock, flags);
    -        __object_put(obj);
    -        spin_unlock_irqrestore(&cache_lock, flags);
    +        if (atomic_dec_and_test(&obj->refcnt))
    +                kfree(obj);
     }

     void object_get(struct object *obj)
     {
    -        unsigned long flags;
    -
    -        spin_lock_irqsave(&cache_lock, flags);
    -        __object_get(obj);
    -        spin_unlock_irqrestore(&cache_lock, flags);
    +        atomic_inc(&obj->refcnt);
     }

     /* Must be holding cache_lock */
    @@ -65,7 +47,7 @@
     {
             BUG_ON(!obj);
             list_del(&obj->list);
    -        __object_put(obj);
    +        object_put(obj);
             cache_num--;
     }

    @@ -94,7 +76,7 @@
             strscpy(obj->name, name, sizeof(obj->name));
             obj->id = id;
             obj->popularity = 0;
    -        obj->refcnt = 1; /* The cache holds a reference */
    +        atomic_set(&obj->refcnt, 1); /* The cache holds a reference */

             spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);
    @@ -119,7 +101,7 @@
             spin_lock_irqsave(&cache_lock, flags);
             obj = __cache_find(id);
             if (obj)
    -                __object_get(obj);
    +                object_get(obj);
             spin_unlock_irqrestore(&cache_lock, flags);
             return obj;
     }

```
### 保护对象自身


在这些例子中，我们假设对象（引用计数除外）一旦创建就再也不会改变。如果我们
想允许 name 发生改变，有三种可能：

- 你可以让 `cache_lock` 变成非静态的，并告诉人们在修改任何对象中的 name
   之前先获取那把锁。

- 你可以提供一个 cache_obj_rename()，它获取这把锁并替调用者修改 name，
   并告诉大家使用那个函数。

- 你可以让 `cache_lock` 只保护缓存本身，而用另一把锁来保护 name。

理论上，你可以把锁做得非常细粒度，细到每个对象每个字段一把锁。实践中，最
常见的变体是：

- 一把锁保护基础设施（本例中的 `cache` 列表）和所有对象。这是我们目前
   所做的。

- 一把锁保护基础设施（包括对象内部的列表指针），对象内还有一把锁保护
   该对象的其余部分。

- 多把锁保护基础设施（例如每条哈希链一把锁），可能再配合一把独立的每对象锁。

下面是“每对象锁”的实现：

```
    --- cache.c.refcnt-atomic   2003-12-11 15:50:54.000000000 +1100
    +++ cache.c.perobjectlock   2003-12-11 17:15:03.000000000 +1100
    @@ -6,11 +6,17 @@

     struct object
     {
    +        /* These two protected by cache_lock. */
             struct list_head list;
    +        int popularity;
    +
             atomic_t refcnt;
    +
    +        /* Doesn't change once created. */
             int id;
    +
    +        spinlock_t lock; /* Protects the name */
             char name[32];
    -        int popularity;
     };

     static DEFINE_SPINLOCK(cache_lock);
    @@ -77,6 +84,7 @@
             obj->id = id;
             obj->popularity = 0;
             atomic_set(&obj->refcnt, 1); /* The cache holds a reference */
    +        spin_lock_init(&obj->lock);

             spin_lock_irqsave(&cache_lock, flags);
             __cache_add(obj);

```
注意，我决定让 popularity 计数由 `cache_lock` 而非每对象锁来保护：
这是因为它（像对象内部的 `struct list_head <list_head>` 一样）在逻辑上
属于基础设施。这样一来，在 __cache_add() 寻找最不常用对象时，我就不必去获取
每个对象的锁。

我还决定 id 成员不可更改，因此在 __cache_find() 中检查 id 时不需要去获取
每个对象的锁：对象锁只被想要读写 name 字段的调用者使用。

还要注意，我加了一条注释，说明哪些数据由哪些锁保护。这极其重要，因为它描述了
代码的运行时行为，而且仅靠阅读很难看出来。正如 Alan Cox 所说，“锁住数据，
而不是代码（Lock data, not code）”。

## 常见问题


### 死锁：简单的与高级的


有一种编码错误，是一段代码试图两次获取同一个自旋锁：它会永远自旋，等待锁被
释放（自旋锁、rwlock 和互斥体在 Linux 中都不是可重入的）。这很容易诊断：
不是那种“连续五个晚上不睡觉、和毛茸茸的代码兔子对话”才能搞定的问题。

稍微复杂一点的情况是，假设你有一个被软中断和用户上下文共享的区域。如果你
使用 spin_lock() 调用来保护它，用户上下文有可能在持有锁时被软中断打断，
而软中断随后会永远自旋，试图获取同一把锁。

这两种情况都叫做死锁，如上所示，即使只有一个 CPU 也可能发生（尽管在 UP 编译
下不会，因为在 `CONFIG_SMP`\ =n 的内核编译中自旋锁会消失。但在第二个例子里
你仍然会得到数据损坏）。

这种完全的锁死很容易诊断：在 SMP 机器上，看门狗定时器，或者编译时设置
`DEBUG_SPINLOCK`（`include/linux/spinlock.h`），会在它发生时立即暴露出来。

一个更复杂的问题是所谓的“致命拥抱”（deadly embrace），涉及两把或更多锁。
假设你有一个哈希表：表中的每一项都是一个自旋锁，以及一条哈希对象的链。在
一个软中断处理程序中，你有时想把某个对象从哈希表的一处移到另一处：你获取
旧哈希链的锁和新哈希链的锁，把对象从旧链删除，再插入新链。

这里有两个问题。第一，如果你的代码试图把对象移到同一条链，它会与自己死锁，
因为它试图加锁两次。第二，如果另一个 CPU 上同一个软中断正试图把另一个对象
反向移动，可能会发生如下情况：

+-----------------------+-----------------------+
| CPU 1                 | CPU 2                 |
+=======================+=======================+
| Grab lock A -> OK     | Grab lock B -> OK     |
+-----------------------+-----------------------+
| Grab lock B -> spin   | Grab lock A -> spin   |
+-----------------------+-----------------------+

Table: Consequences

两个 CPU 会永远自旋，等待对方放弃自己的锁。它看起来、闻起来、摸起来都像一次崩溃。

### 预防死锁


教科书会告诉你，如果你总是按相同的顺序加锁，就永远不会出现这类死锁。实践会
告诉你，这种方法无法扩展：当我新创建一把锁时，我对内核了解得还不够多，无法
弄清它该放在那 5000 层锁的层级结构中的什么位置。

最好的锁是封装起来的：它们永远不会出现在头文件中，也永远不会在调用同一文件
之外的非平凡函数时持有。你可以通读这段代码并看出它永远不会死锁，因为它在
持有这把锁时从不试图去获取另一把锁。使用你代码的人甚至不需要知道你在使用锁。

这里一个典型的问题是，当你提供回调或钩子时：如果你在持有锁的情况下调用它们，
你就有简单死锁或致命拥抱的风险（谁知道回调会做什么？）。

#### 过度积极地预防死锁


死锁固然成问题，但不如数据损坏严重。一段代码先获取读锁、搜索列表、发现找不到
想要的、释放读锁、再获取写锁并插入对象，这就有竞态条件。

### 竞速的定时器：内核的一项消遣


定时器自身也会产生它特有的竞态问题。考虑一组对象（列表、哈希表等），其中
每个对象都有一个定时器，到期时销毁它。

如果你想销毁整个集合（例如在模块移除时），

```

            /* THIS CODE BAD BAD BAD BAD: IF IT WAS ANY WORSE IT WOULD USE
               HUNGARIAN NOTATION */
            spin_lock_bh(&list_lock);

            while (list) {
                    struct foo *next = list->next;
                    timer_delete(&list->timer);
                    kfree(list);
                    list = next;
            }

            spin_unlock_bh(&list_lock);


```
迟早，这在 SMP 上会崩溃，因为定时器可能恰好在 spin_lock_bh() 之前已经触发，
它只会在我们 spin_unlock_bh() 之后才拿到锁，然后试图释放那个元素
（而它已经被释放掉了！）。

这可以通过检查 timer_delete() 的返回值来避免：如果返回 1，说明定时器已被删除。
如果返回 0，则意味着（在此例中）它当前正在运行，因此我们可以

```

            retry:
                    spin_lock_bh(&list_lock);

                    while (list) {
                            struct foo *next = list->next;
                            if (!timer_delete(&list->timer)) {
                                    /* Give timer a chance to delete this */
                                    spin_unlock_bh(&list_lock);
                                    goto retry;
                            }
                            kfree(list);
                            list = next;
                    }

                    spin_unlock_bh(&list_lock);


```
另一个常见问题是删除那些会自我重启的定时器（在定时器函数末尾调用 add_timer()）。
因为这是一个相当常见、又容易出现竞态的情况，你应该使用 timer_delete_sync()
（`include/linux/timer.h`）来处理这种情况。

在释放定时器之前，应该调用 timer_shutdown() 或 timer_shutdown_sync()，
它们会防止定时器被重新唤起。随后任何重新唤起定时器的尝试都会被核心代码静默忽略。

## 加锁速度


在考虑某些加锁代码的性能时，有三个主要方面需要担心。第一是并发：当别人持有
锁时，会有多少东西在等待。第二是实际获取和释放一把无竞争锁所花的时间。第三是
使用更少或更聪明的锁。我假定这把锁使用得相当频繁：否则，你不会关心效率。

并发取决于锁通常被持有多长时间：你应该按需持有锁，但绝不要更久。在缓存例子中，
我们总是在不持有锁的情况下创建对象，然后只在准备把它插入列表时才获取锁。

获取时间取决于锁操作对流水线造成了多大破坏（流水线停顿），以及这个 CPU 是否
最有可能就是上一个拿到锁的 CPU（即这把锁对该 CPU 是否是缓存热的）：在 CPU
更多的机器上，这种可能性下降得很快。以一台 700MHz 的 Intel Pentium III 为例：
一条指令约需 0.7ns，一次原子递增约需 58ns，一把对该 CPU 缓存热的锁约需 160ns，
而从另一个 CPU 做一次缓存行传输还要额外 170 到 360ns。（这些数字来自 Paul
McKenney 的 `Linux Journal RCU 文章
<http://www.linuxjournal.com/article.php?sid=6993>`__）。

这两个目标相互冲突：把锁持有很短的时间，可以通过把锁拆分成多个部分来实现
（例如我们最后的每对象锁例子），但这会增加锁获取的次数，结果往往比使用单一
锁更慢。这是主张“锁要简单”的另一个理由。

第三个方面的顾虑在下面讨论：有一些方法可以减少需要进行的加锁量。

### 读写锁变体


自旋锁和互斥体都有读写变体：`rwlock_t` 和 `struct rw_semaphore
<rw_semaphore>`。它们把使用者分成两类：读者和写者。如果你只是读取数据，
你可以获取读锁，但要写数据就需要写锁。许多人可以持有读锁，但写者必须是唯一的
持有者。

如果你的代码能清晰地按读者/写者划分（就像我们的缓存代码那样），而且锁被读者
持有的时间较长，使用这些锁会有帮助。不过它们比普通锁稍慢，所以实践中 `rwlock_t`
通常并不划算。

### 避免加锁：读-复制-更新（RCU）


有一种特殊的读写锁方法叫做读-复制-更新（Read Copy Update，RCU）。使用 RCU，
读者可以完全避免获取锁：因为我们预期缓存被读取的次数多于被更新的次数
（否则缓存就是浪费时间），它是一个进行这种优化的候选者。

我们如何去掉读锁？去掉读锁意味着，写者可能在读者正在遍历列表时修改它。这其实
相当简单：如果写者非常小心地添加元素，我们就可以在元素被添加的同时读取链表。
例如，

```

            new->next = list->next;
            wmb();
            list->next = new;


```
wmb() 是一个写内存屏障。它确保第一个操作（设置新元素的 `next` 指针）已完成，
并且会被所有 CPU 看到，然后才进行第二个操作（把新元素放入列表）。这很重要，
因为现代编译器和现代 CPU 都可能在未被明确告知的情况下重排指令：我们希望读者
要么完全看不到新元素，要么看到带有正确指向链表其余部分的 `next` 指针的新元素。

幸运的是，有一个函数可以为标准的 `struct list_head <list_head>` 列表做这件事：
list_add_rcu()（`include/linux/list.h`）。

从列表中删除一个元素更简单：我们用指向其后继的指针替换指向旧元素的指针，读者
要么看到它，要么跳过它。

```

            list->next = old->next;


```
有 list_del_rcu()（`include/linux/list.h`）来做这件事（普通版本会污染旧对象，
而那不是我们想要的）。

读者也必须小心：某些 CPU 会顺着 `next` 指针提前开始读取下一个元素的内容，但当
`next` 指针在它们脚下改变时，却没有意识到预取的内容是错误的。再一次，有
list_for_each_entry_rcu()（`include/linux/list.h`）来帮助你。当然，写者只
需使用 list_for_each_entry() 即可，因为不可能有两个同时的写者。

我们最终的困境是：我们究竟什么时候才能真正销毁被删除的元素？记住，读者现在
可能正在遍历列表中的这个元素：如果我们释放了这个元素，而 `next` 指针随之改变，
读者就会跳进垃圾并崩溃。我们需要等到我们知道，在我们删除该元素时所有正在遍历
列表的读者都已完成。我们使用 call_rcu() 注册一个回调，一旦所有既存的读者都完成，
它就会真正销毁对象。或者，也可以使用 synchronize_rcu() 来阻塞，直到所有既存的
读者都完成。

但读-复制-更新怎么知道读者已经完成了呢？方法是这样的：首先，读者总是在
rcu_read_lock()/rcu_read_unlock() 对内部遍历列表：它们仅仅是禁用抢占，这样
读者在读取列表时就不会去睡眠。

然后 RCU 等待，直到每个其他 CPU 至少睡眠过一次：由于读者不能睡眠，我们就知道，
在删除期间正在遍历列表的任何读者都已经完成，于是触发回调。真正的读-复制-更新
代码比这优化得稍好一些，但这就是基本思想。

```

    --- cache.c.perobjectlock   2003-12-11 17:15:03.000000000 +1100
    +++ cache.c.rcupdate    2003-12-11 17:55:14.000000000 +1100
    @@ -1,15 +1,18 @@
     #include <linux/list.h>
     #include <linux/slab.h>
     #include <linux/string.h>
    +#include <linux/rcupdate.h>
     #include <linux/mutex.h>
     #include <asm/errno.h>

     struct object
     {
    -        /* These two protected by cache_lock. */
    +        /* This is protected by RCU */
             struct list_head list;
             int popularity;

    +        struct rcu_head rcu;
    +
             atomic_t refcnt;

             /* Doesn't change once created. */
    @@ -40,7 +43,7 @@
     {
             struct object *i;

    -        list_for_each_entry(i, &cache, list) {
    +        list_for_each_entry_rcu(i, &cache, list) {
                     if (i->id == id) {
                             i->popularity++;
                             return i;
    @@ -49,19 +52,25 @@
             return NULL;
     }

    +/* Final discard done once we know no readers are looking. */
    +static void cache_delete_rcu(void *arg)
    +{
    +        object_put(arg);
    +}
    +
     /* Must be holding cache_lock */
     static void __cache_delete(struct object *obj)
     {
             BUG_ON(!obj);
    -        list_del(&obj->list);
    -        object_put(obj);
    +        list_del_rcu(&obj->list);
             cache_num--;
    +        call_rcu(&obj->rcu, cache_delete_rcu);
     }

     /* Must be holding cache_lock */
     static void __cache_add(struct object *obj)
     {
    -        list_add(&obj->list, &cache);
    +        list_add_rcu(&obj->list, &cache);
             if (++cache_num > MAX_CACHE_SIZE) {
                     struct object *i, *outcast = NULL;
                     list_for_each_entry(i, &cache, list) {
    @@ -104,12 +114,11 @@
     struct object *cache_find(int id)
     {
             struct object *obj;
    -        unsigned long flags;

    -        spin_lock_irqsave(&cache_lock, flags);
    +        rcu_read_lock();
             obj = __cache_find(id);
             if (obj)
                     object_get(obj);
    -        spin_unlock_irqrestore(&cache_lock, flags);
    +        rcu_read_unlock();
             return obj;
     }

```
注意，读者会在 __cache_find() 中修改 popularity 成员，而现在它并不持有锁。
一种解决办法是把它改成 `atomic_t`，但对于这种用法，我们其实并不关心竞态：
一个近似的结果就足够了，所以我没改它。

结果是，cache_find() 不需要与任何其他函数同步，因此在 SMP 上几乎和在 UP 上
一样快。

这里还有进一步的优化可能：回想我们最初的缓存代码，那时没有引用计数，调用者
只要在使用对象时就简单地持有锁？这仍然是可能的：如果你持有锁，就没有人能删除
对象，所以你不需要去增减引用计数。

现在，由于 RCU 中的“读锁”仅仅是禁用抢占，一个始终在调用 cache_find() 和
object_put() 之间禁用抢占的调用者，实际上不需要去增减引用计数：我们可以让
__cache_find() 变成非静态的来暴露它，这样的调用者直接调用它即可。

这样做的好处是引用计数不会被写入：对象不会被以任何方式修改，由于缓存的缘故，
这在 SMP 机器上要快得多。

### 每-CPU 数据


另一种被广泛使用的避免加锁的技术，是为每个 CPU 复制信息。例如，如果你想为一个
常见条件保存一个计数，你可以用一个自旋锁和一个单一的计数器。简单又干净。

如果那样太慢（通常并不会，但如果你有一台真正大型的机器来测试，并能证明它确实
慢），你可以改用每个 CPU 一个计数器，这样它们都不需要独占锁。参见
DEFINE_PER_CPU()、get_cpu_var() 和 put_cpu_var()
（`include/linux/percpu.h`）。

对于简单的每-CPU 计数器，特别有用的是 `local_t` 类型，以及 cpu_local_inc() 和
相关函数，在某些体系结构上它们比简单代码更高效
（`include/asm/local.h`）。

注意，在不引入更多锁的情况下，没有简单可靠的方法能得到这样一个计数器的精确值。
这对某些用途来说不是问题。

### 主要被 IRQ 处理程序使用的数据


如果数据总是从同一个 IRQ 处理程序内部访问，你根本不需要锁：内核已经保证该 irq
处理程序不会在多个 CPU 上同时运行。

Manfred Spraul 指出，即使数据极少情况下在用户上下文或软中断/tasklet 中被访问，
你仍然可以这样做：

```

        mutex_lock(&lock);
        disable_irq(irq);
        ...
        enable_irq(irq);
        mutex_unlock(&lock);


```
disable_irq() 阻止 irq 处理程序运行（如果它正在其他 CPU 上运行，则等待它
完成）。自旋锁阻止任何其他访问同时进行。自然，这比单独一次 spin_lock_irq()
调用要慢，因此只有当这类访问极其罕见时才有意义。

## 哪些函数可以从中断中安全调用？


内核中的许多函数会睡眠（即直接或间接调用 schedule()）：你绝不能在持有自旋锁
或禁用抢占时调用它们。这也意味着你需要处于用户上下文中：从中断调用它们是非法的。

### 一些会睡眠的函数


下面列出了最常见的一些，但通常你必须阅读代码才能搞清楚其他调用是否安全。如果
所有其他调用它的人都能睡眠，你大概也需要能够睡眠。特别是，注册和注销函数通常
期望从用户上下文调用，并且可能睡眠。

- 对 userspace 的访问：

   - copy_from_user()

   - copy_to_user()

   - get_user()

   - put_user()

- kmalloc(GP_KERNEL) <kmalloc>`

- mutex_lock_interruptible() 和 mutex_lock()

   有一个 mutex_trylock() 不会睡眠。尽管如此，它不能在中断上下文内部使用，因为
   它的实现对此并不安全。mutex_unlock() 也永远不会睡眠。它同样不能在中断上下文中
   使用，因为互斥体必须由获取它的同一个任务来释放。

### 一些不会睡眠的函数


有些函数可以安全地在任何上下文中调用，或者持有几乎任何锁时调用。

- printk()

- kfree()

- add_timer() 和 timer_delete()

## 互斥体 API 参考


   :internal:

   :export:

## Futex API 参考


   :internal:

   :internal:

   :internal:

   :internal:

   :internal:

## 延伸阅读


- `Documentation/locking/spinlocks.rst`：Linus Torvalds 在内核源码中的自旋锁
   教程。

- 《Unix Systems for Modern Architectures: Symmetric Multiprocessing and
   Caching for Kernel Programmers》（现代体系结构上的 Unix 系统：面向内核
   程序员的对称多处理与缓存）：

   Curt Schimmel 对内核级加锁非常好的入门介绍（并非为 Linux 所写，但几乎一切都
   适用）。这本书很贵，但要理解 SMP 加锁，每一分钱都值得。
   [ISBN: 0201633388]

## 致谢


感谢 Telsa Gwynne 进行 DocBooking、整理并添加风格。

感谢 Martin Pool、Philipp Rumpf、Stephen Rothwell、Paul Mackerras、Ruedi
Aschwanden、Alan Cox、Manfred Spraul、Tim Waugh、Pete Zaitcev、James Morris、
Robert Love、Paul McKenney、John Ashby 进行校对、纠正、吐槽和评论。

感谢那个秘密小团体对本文档没有产生任何影响。

## 术语表


preemption
  在 2.5 之前，或者当 `CONFIG_PREEMPT` 未设置时，处于内核中的用户上下文里的
  进程不会相互抢占（即你独占那个 CPU，直到你放弃它，中断除外）。随着 2.5.4 中
  加入 `CONFIG_PREEMPT`，这一点改变了：在用户上下文中时，更高优先级的任务可以
  “插队”：自旋锁被改成禁用抢占，即使在 UP 上也是如此。

bh
  底半部（Bottom Half）：由于历史原因，名称中带有 '_bh' 的函数现在通常指代任何
  软件中断，例如 spin_lock_bh() 会阻塞当前 CPU 上的任何软件中断。底半部已被弃用，
  并最终会被 tasklet 取代。任意时刻只会有一个底半部在运行。

Hardware Interrupt / Hardware IRQ
  硬件中断请求。in_hardirq() 在硬件中断处理程序中返回真。

Interrupt Context
  非用户上下文：处理一个硬件 irq 或软件 irq。由 in_interrupt() 宏返回真来指示。

SMP
  对称多处理器（Symmetric Multi-Processor）：为多 CPU 机器编译的内核。
  （`CONFIG_SMP=y`）。

Software Interrupt / softirq
  软件中断处理程序。in_hardirq() 返回假；in_softirq() 返回真。Tasklet 和
  软中断都属于“软件中断”这一类别。

  严格来说，softirq 是至多 32 个枚举软件中断之一，可以在多个 CPU 上同时运行。
  有时也用来指代 tasklet（即所有软件中断）。

tasklet
  一种可动态注册的软件中断，保证任意时刻只在一个 CPU 上运行。

timer
  一种可动态注册的软件中断，在给定的时间（或接近该时间）运行。运行时它就像一个
  tasklet（事实上，它们是从 `TIMER_SOFTIRQ` 调用的）。

UP
  单处理器（Uni-Processor）：非 SMP。（`CONFIG_SMP=n`）。

User Context
  代表某个特定进程（即一次系统调用或陷阱）或内核线程在内核中执行。你可以用
  `current` 宏知道是哪个进程。不要与 userspace 混淆。它可以被软件或硬件中断打断。

Userspace
  进程在自己的代码、处于内核之外执行。

