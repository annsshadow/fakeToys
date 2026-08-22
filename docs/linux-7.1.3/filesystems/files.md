
## Linux 内核中的文件管理


本文档介绍文件（`struct file`）与文件描述符表（`struct files`）的加锁机制是如何工作的
2.6.12 之前，文件描述符表由一把锁（files->file_lock）和引用计数（files->count）保护->file_lock 保护对表中所有与文件相关字段的访问>count 用于在通过 CLONE_FILES 标志克隆的任务之间共文件描述符表。对POSIX 线程通常就是这种情况。与内核中常见的引用计数模型一样，最后一个执put_files_struct() 的任务释放文件描述符（fd）表。文件（`struct file`）本身使用引用计数（->f_count）保护
在文件描述符管理新的无锁模型中，引用计数方式类似，但加锁基于 RCU。文件描述符表包含多个元素—fd 集合（open_fds close_on_exec）、文件指针数组、集合与数组的大小等。为了使更新对无锁读取呈现原子性，文件描述符表的所有元素都放在一个独立的结构struct fdtable 中。files_struct 包含一指向 struct fdtable 的指针，实际fd 表通过该指针访问。最fdtable 嵌入files_struct 自身中在后fdtable 扩展时，会分配一个新fdtable 结构，files->fdtab 指向新结构。fdtable 结构通过
RCU 释放，无锁读取者要么看到旧fdtable，要么看到新fdtable，从而使更新呈现原子性以下fdtable 结构的加锁规则：

1. 所有对 fdtable 的引用都必须通过

```

	struct fdtable *fdt;

	rcu_read_lock();

	fdt = files_fdtable(files);
	....
	if (n <= fdt->max_fds)
		....
	...
	rcu_read_unlock();

   files_fdtable() 使用 rcu_dereference() 宏，该宏负责处理无锁解引用所需的内存屏障要求   fdtable 指针必须在读端临界区内部读取
```

2. 上述fdtable 的读取必须由 rcu_read_lock()/rcu_read_unlock() 保护
3. 对于任何fd 表的更新，必须持files->file_lock
4. 给定一fd 查找 file 结构时，读取者必须使lookup_fdget_rcu() files_lookup_fdget_rcu() API   它们负责处理因无锁查找而产生的屏障要求
```

	struct file *file;

	rcu_read_lock();
	file = lookup_fdget_rcu(fd);
	rcu_read_unlock();
	if (file) {
		...
                fput(file);
	}
	....

```

5. 由于 fdtable file 结构都可以无锁查找，它们必须使用 rcu_assign_pointer() API 安装   如果它们被无锁查找，则必须使rcu_dereference()。不过建议使files_fdtable() 以及
   lookup_fdget_rcu()/files_lookup_fdget_rcu()，它们会处理这些问题
6. 在更新时，fdtable 指针必须在持files->file_lock 的情况下查找。如果释放了 ->file_lock，则
   另一个线程可能扩files，从而创建一个新fdtable 并使先前fdtable 指针失效
```

	spin_lock(&files->file_lock);
	fd = locate_fd(files, file, start);
	if (fd >= 0) {
		/* locate_fd() 可能已扩fdtable，加载该指针 */
		fdt = files_fdtable(files);
		__set_open_fd(fd, fdt);
		__clear_close_on_exec(fd, fdt);
		spin_unlock(&files->file_lock);
	.....

   由于 locate_fd() 可能释放 ->file_lock（并重新获取 ->file_lock），fdtable 指针（fdt）必须在
   locate_fd() 之后加载
```
在较新的内核中，基于 RCU 的文件查找已切换为依SLAB_TYPESAFE_BY_RCU 而非 call_rcu()。仅仅在 RCU 使用 atomic_long_inc_not_zero() 获取相关文件的引用已经不够，因为该文件可能已经被回收，而其他人可能已经
增加了引用计数。换句话说，调用者可能看到来自较新用户的引用计数增加。出于这个原因，有必要在引用计数
增加前后验证指针是相同的。这一模式可见get_file_rcu() __files_get_rcu()
此外，在 RCU 查找下，若未先在文件上获取引用，就无法访问或检struct file 中的字段。不这样做一直非不可靠，并且它只适用struct file 中的非指针数据。有SLAB_TYPESAFE_BY_RCU，调用者有必要要么获取一个引用，要么必须持有 fdtable files_lock