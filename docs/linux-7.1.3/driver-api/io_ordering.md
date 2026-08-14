
## 对映射到内存地址的 I/O 写入进行排序


在某些平台上，所谓的内存映射 I/O（memory-mapped I/O）是弱序的。在这类平台上，驱动开发者有责任确保其设备上对内存映射地址的 I/O 写入按预期的顺序到达。这通常通过读取一个“安全”的设备或桥接寄存器来实现，从而迫使 I/O 芯片组在任何读操作发起之前，将挂起的写入刷新到设备。驱动通常会在受自旋锁保护的临界区代码退出之前立即使用此技术。这可确保所有后续的 I/O 空间写入都仅在所有先前的写入之后到达（很像内存屏障操作 mb()，只是针对 I/O 而言）。

```

		...
	CPU A:  spin_lock_irqsave(&dev_lock, flags)
	CPU A:  val = readl(my_status);
	CPU A:  ...
	CPU A:  writel(newval, ring_ptr);
	CPU A:  spin_unlock_irqrestore(&dev_lock, flags)
		...
	CPU B:  spin_lock_irqsave(&dev_lock, flags)
	CPU B:  val = readl(my_status);
	CPU B:  ...
	CPU B:  writel(newval2, ring_ptr);
	CPU B:  spin_unlock_irqrestore(&dev_lock, flags)
		...

```
在上述情形下，设备可能会在收到 newval 之前先收到 newval2，

```

		...
	CPU A:  spin_lock_irqsave(&dev_lock, flags)
	CPU A:  val = readl(my_status);
	CPU A:  ...
	CPU A:  writel(newval, ring_ptr);
	CPU A:  (void)readl(safe_register); /* maybe a config register? */
	CPU A:  spin_unlock_irqrestore(&dev_lock, flags)
		...
	CPU B:  spin_lock_irqsave(&dev_lock, flags)
	CPU B:  val = readl(my_status);
	CPU B:  ...
	CPU B:  writel(newval2, ring_ptr);
	CPU B:  (void)readl(safe_register); /* maybe a config register? */
	CPU B:  spin_unlock_irqrestore(&dev_lock, flags)

```
此处，对 safe_register 的读取将促使 I/O 芯片组在实际向芯片组发起读操作之前，刷新任何挂起的写入，从而防止可能的数据损坏。
