## 锁的经验之谈


## 第一课：自旋


```

  static DEFINE_SPINLOCK(xxx_lock);

	unsigned long flags;

	spin_lock_irqsave(&xxx_lock, flags);
	... critical section here ..
	spin_unlock_irqrestore(&xxx_lock, flags);

```
上述做法始终是安全的。它会在局部禁用中断，但自旋锁本身会保证全局锁，因此它能保证在由该锁保护的区域中只有一个控制线程。这即使UP 下也能良好工作，因此代码不需要担UP SMP 的问题：自旋锁在两种情况下都能正确工作

  注意！自旋锁对内存的影响在以下文档中有进一步描述：

    Documentation/memory-barriers.txt

       (5) ACQUIRE operations.

       (6) RELEASE operations.

上述通常相当简单（对于大多数事情你通常只需要且只想要一个自旋锁——使用多个自旋锁会让事情复杂得多甚至更慢，通常只对于你**确定**需要拆分开的序列才值得：如果不确定就无论如何都要避免）

这确实是自旋锁唯一真正困难的部分：一旦你开始使用自旋锁，它们往往会扩展到你可能之前没有注意到的领域，因为你必须确保自旋锁在它们被使用*每一个地*都正确地保护共享数据结构。自旋锁最容易添加到完全独立于其他代码的地方（例如，没有其他人会触碰的内部驱动数据结构）

   注意！自旋锁只有在你**同时**使用锁本身来CPU 进行加锁时才是安全的，这意味着每个触碰共享变量的部分都必须就它们想要使用的自旋锁达成一致

----

## 第二课：读写自旋锁


如果你的数据访问具有非常自然的模式，即你通常大多只是读取共享变量，那么自旋锁的读写锁（rw_lock）版本有时很有用。它们允许多个读者同时处于同一个临界区内，但如果有人想要修改变量，就必须获取独占的写锁

   注意！读写锁比简单的自旋锁需要更多的原子内存操作。除非读者临界区很长，否则你最好只使用自旋锁

```

   rwlock_t xxx_lock = __RW_LOCK_UNLOCKED(xxx_lock);

	unsigned long flags;

	read_lock_irqsave(&xxx_lock, flags);
	.. critical section that only reads the info ...
	read_unlock_irqrestore(&xxx_lock, flags);

	write_lock_irqsave(&xxx_lock, flags);
	.. read and write exclusive access to the info ...
	write_unlock_irqrestore(&xxx_lock, flags);

```
上述这类锁对于像链表这样的复杂数据结构可能很有用，特别是用于搜索条目而不改变链表本身的情况。读锁允许多个并发读者。任*改变**链表的动作都必须获取写锁

   注意！RCU 更适合链表遍历，但需要仔细关注设计细节（Documentation/RCU/listRCU.rst）

此外，你不能将读锁“升级”为写锁，所以如果你在_任何_时候需要做任何修改（即使你并非每次都做），你都必须在最开始就获取写锁

   注意！我们正在努力在大多数情况下移除读写自旋锁，因此如果没有共识请不要新增一个。（作为替代，有关完整信息请参见 Documentation/RCU/rcu.rst。）

----

## 第三课：再谈自旋锁


上面这些单一的自旋锁原语绝不是唯一的。它们是最安全的，也是在一切情况下都能工作的，但部*因为**它们安全，它们也相当慢。它们比需要的更慢，因为它们确实必须禁用中断（x86 上这只是一条指令，但却是一条代价高昂的指令——而在其他架构上可能更糟）

如果你有一种情况，必须跨多CPU 保护一个数据结构，并且你想使用自旋锁，你就有可能使用更廉价的自旋锁版本。如果（IFF）你知道自旋

```

	spin_lock(&lock);
	...
	spin_unlock(&lock);

```
（当然，还有等价的读写版本）。自旋锁将保证同类型的独占访问，并且会快得多。如果你知道所讨论的数据只在“进程上下文”中被操纵，即不涉及中断，这就很有用

如果你有中断，就绝不

```

	spin_lock(&lock);
	...
		<- interrupt comes in:
			spin_lock(&lock);

```
使用这些版本，因为中断会试图锁定一个已被锁定的变量。如果另一个中断发生在另一CPU 上，这没有问题；但如果该中断发生在已经持有该锁的同一CPU 上，就_不_可以，因为该锁显然永远不会被释放（因为中断正在等待该锁，而锁的持有者被该中断打断，在中断被处理完之前不会继续执行）

（这也是 irq 版本的自旋锁只需要禁用局部中断的原因——在其他 CPU 上的中断中使用自旋锁是可以的，因为另一CPU 上的中断不会打断持有锁的 CPU，所以锁的持有者可以继续并最终释放该锁）

		Linus

----

## 参考信息：


对于动态初始化，请使用 spin_lock_init() rwlock_init() 视情况而定

```

   spinlock_t xxx_lock;
   rwlock_t xxx_rw_lock;

   static int __init xxx_init(void)
   {
	spin_lock_init(&xxx_lock);
	rwlock_init(&xxx_rw_lock);
	...
   }

   module_init(xxx_init);

```
对于静态初始化，请视情况使DEFINE_SPINLOCK() / DEFINE_RWLOCK() __SPIN_LOCK_UNLOCKED() / __RW_LOCK_UNLOCKED()
