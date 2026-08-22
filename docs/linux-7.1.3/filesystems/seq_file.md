
## seq_file 接口


	Copyright 2003 Jonathan Corbet <corbet@lwn.net>

	本文件最初来LWN.net 的驱动移植（Driver Porting）系列，位于
	https://lwn.net/Articles/driver-porting/


设备驱动（或其他内核组件）有许多方式向用户或系统管理员提供信息。一种有用的技术是debugfs/proc 或其他位置创建虚拟文件。虚拟文件可以提供人类可读的输出，无需任何特殊工具程序即可获取它们也能让脚本编写者的工作更轻松。虚拟文件的使用逐年增长并不令人意外
然而，正确地创建这些文件一直有点棘手。返回一个字符串的虚拟文件并不难做。但如果输出很长—超过应用程序单次操作可能读取的长度——事情就变得更复杂了。处理多次读取（和定lseek）需仔细关注读取者在虚拟文件中的位置——这个位置很可能位于某一行输出的中间。传统上，内核中有不实现在这方面犯了错
内核现在包含一组（Alexander Viro 实现）函数，旨在让虚拟文件的创建者轻松地把事情做对
seq_file 接口通过 `<linux/seq_file.h>` 提供。seq_file 有三个方面：

     - 一个迭代器（iterator）接口，让虚拟文件实现能够逐步遍历它所呈现的对象
     - 一些用于格式化对象以便输出的工具函数，无需担心输出缓冲区之类的事情
     - 一组预制的 file_operations，实现了虚拟文件上的大多数操作
我们将通过一个极其简单的示例来了seq_file 接口：一个可加载模块，它创建一个名/proc/sequence
的文件。该文件在被读取时，简单地产生一组递增的整数值，每行一个。这个序列会一直持续，直到用户
失去耐心去找点别的事情做。该文件是可定位的（seekable），也就是说可以执行类似如下的操
```

    dd if=/proc/sequence of=out1 count=1
    dd if=/proc/sequence skip=1 of=out2 count=1

```
然后拼接输出文件 out1 out2 就能得到正确的结果。是的，这是一个完全无用的模块，但重点是在
不迷失于其他细节的情况下展示该机制是如何工作的。（想要查看该模块完整源码的人可以在
https://lwn.net/Articles/22359/ 找到。）

## 已废弃的 create_proc_entry


请注意，上文文章使用的是 create_proc_entry，该函数已在

```

    -	entry = create_proc_entry("sequence", 0, NULL);
    -	if (entry)
    -		entry->proc_fops = &ct_file_ops;
    +	entry = proc_create("sequence", 0, NULL, &ct_file_ops);

```
## 迭代器接

使用 seq_file 实现虚拟模块的模块必须实现一个迭代器对象，该对象允许在一次“会话”（大致对应一read() 系统调用）期间逐步遍历感兴趣的数据。如果迭代器能够移动到特定位置——就像它们实现的文件
那样，不过可以自由地把位置编号映射到任意方便的序列位置——那么迭代器只需要在会话期间暂时存在如果迭代器无法轻易找到一个数字位置，但很适合 first/next 接口，则该迭代器可以存储在私有数据区中，
并在一个会话到下一个会话之间继续
例如，一个从表中格式化防火墙规则seq_file 实现，可以提供一个简单的迭代器，把位N 解释链中的第 N 条规则。一个呈现某个（可能是易变的）链表内容的 seq_file 实现，可能会记录一个指该链表的指针，前提是可以做到这一点而不会有当前位置被移除的风险
因此，定位可以以对数据生成者最有意义的方式来执行，而数据生成者无需知道位置如何转换为虚拟文件中
的偏移量。一个明显的例外是：位置为零应表示文件的开始
/proc/sequence 的迭代器只是把将要输出的下一个数字的计数作为其位置
必须实现四个函数才能使迭代器工作。第一个名start()，它启动一个会话，并以一个位置作为参数，
返回一个将从该位置开始读取的迭代器。传start() pos 总是要么为零，要么是前一个会话中使用最近一pos
对于我们的简单序列示例，

```

	static void *ct_seq_start(struct seq_file *s, loff_t *pos)
	{
	        loff_t *spos = kmalloc(sizeof(loff_t), GFP_KERNEL);
	        if (! spos)
	                return NULL;
	        *spos = *pos;
	        return spos;
	}

```
这个迭代器的整个数据结构就是一个保存当前位置的单一 loff_t 值。序列迭代器没有上限，但大多数其seq_file 实现并非如此；在大多数情况下，start() 函数应该检查“超过文件末尾”的情况，并在必要时
返回 NULL
对于更复杂的应用，seq_file 结构private 字段可用于在会话之间保存状态。start() 函数还可以返一个特殊SEQ_START_TOKEN；如果你希望指示你的 show() 函数（下文描述）在输出顶部打印一个头部，
可以使用它。不SEQ_START_TOKEN 只应在偏移量为零时使用。SEQ_START_TOKEN 对核seq_file
代码没有特殊含义。它作为一种便利提供，用于 start() 函数next() show() 函数之间的通信
接下来要实现的函数，令人惊讶地，叫做 next()；它的工作是把迭代器向前移动到序列中的下一个位置示例模块可以简单地将位置加一；更有用的模块会做必要的工作来遍历某数据结构。next() 函数返回一新迭代器，如果序列结束则返回

```

	static void *ct_seq_next(struct seq_file *s, void *v, loff_t *pos)
	{
	        loff_t *spos = v;
	        *pos = ++*spos;
	        return spos;
	}

```
next() 函数应该`*pos` 设置为一start() 可以用来在序列中找到新位置的值。当迭代器被存储私有数据区中、而不是在每次 start() 时重新初始化时，仅仅`*pos` 设置为任意非零值（零总是告诉
start() 要重启序列）似乎就足够了。但由于历史问题，这并不充分
历史上，许多 next() 函数**没有**在文件末尾更`*pos`。如果该值随后被 start() 用来初始化迭代器就可能导致边界情况，即序列中的最后一个条目在文件中被报告两次。为了阻止这bug 死灰复燃，核seq_file 代码现在会在 next() 函数不改`*pos` 的值时产生一个警告。因此，next() 函数**必须**
改变 `*pos` 的值，并且当然必须把它设置为一个非零值
stop() 函数关闭一个会话；它的工作当然是清理。如果为迭代器分配了动态内存，stop() 就是释放它的
地方；如start() 获取了一个锁，stop() 必须释放那个锁。在 stop() 之前最后一next() 调用所
设置`*pos` 值会被记住，并用于下一会话的第一start() 调用，除非对该文件调用了 lseek()；在
那种情况
```

	static void ct_seq_stop(struct seq_file *s, void *v)
	{
	        kfree(v);
	}

```
最后，show() 函数应该格式化当前指向的对象

```

	static int ct_seq_show(struct seq_file *s, void *v)
	{
	        loff_t *spos = v;
	        seq_printf(s, "%lld\n", (long long)*spos);
	        return 0;
	}

```
如果一切正常，show() 函数应该返回零。以常规方式返回一个负的错误码表示出了点问题；它会被传用户空间。这个函数也可以返回 SEQ_SKIP，这会导致跳过当前条目；如果 show() 函数在返SEQ_SKIP
之前已经产生了输出，那么那部分输出会被丢弃
我们稍后会看 seq_printf()。但首先，通过创建一seq_operations 结构来完seq_file 迭代器的
定义

```

	static const struct seq_operations ct_seq_ops = {
	        .start = ct_seq_start,
	        .next  = ct_seq_next,
	        .stop  = ct_seq_stop,
	        .show  = ct_seq_show
	};

```
稍后我们将需要这个结构来把我们的迭代器与 /proc 文件绑定起来
值得一提的是，start() 返回并被其他函数操作的迭代器值，对于 seq_file 代码来说被视为完全不透明
（opaque）。因此它可以是任何有助于逐步遍历待输出数据的东西。计数器可能有用，但它也可以是一直接指向数组或链表的指针。只要程序员意识到在两次调用迭代器函数之间可能发生任何事情，怎么都行不过，seq_file 代码（按设计）不会在 start() stop() 的调用之间休眠，因此在这段时间内持有是合理的。seq_file 代码在迭代器处于活动状态时也会避免获取任何其他锁
start() next() 返回的迭代器值保证会被传递给后续next() stop() 调用。这使得诸如所
获取的锁等资源能够被可靠地释放。但*没有**保证该迭代器会被传递给 show()，尽管在实践中它通常
会被传递

## 格式化输

seq_file 代码管理迭代器所创建输出中的位置，并将其送入用户的缓冲区。但为了让它工作，该输出必须
被传递给 seq_file 代码。已经定义了一些工具函数来使这项任务变得容易
大多数代码将直接使用 seq_printf()，它的工作方式与 printk() 非常相似，但需seq_file 指针作为
参数
```

	seq_putc(struct seq_file *m, char c);
	seq_puts(struct seq_file *m, const char *s);
	seq_escape(struct seq_file *m, const char *s, const char *esc);

```
前两个分别输出单个字符和字符串，正如人们所期望的那样。seq_escape() 类似seq_puts()，不之处在于 s 中任何属于字符串 esc 的字符在输出中将以八进制形式表示
```

	int seq_path(struct seq_file *m, const struct path *path,
		     const char *esc);
	int seq_path_root(struct seq_file *m, const struct path *path,
			  const struct path *root, const char *esc)

```
这里，path 指示感兴趣的文件，esc 是一组应在输出中转义的字符。调seq_path() 将输出相对于
当前进程文件系统根的路径。如果需要不同的根，可以seq_path_root() 一起使用。如果最终发现无root 到达 path，seq_path_root() 返回 SEQ_SKIP
```

	bool seq_has_overflowed(struct seq_file *m);

```
如果返回 true，则避免进一步调seq_<output>
seq_has_overflowed 返回 true 意味着 seq_file 缓冲区将被丢弃，并且 seq_show 函数将尝试分配一更大的缓冲区并重试打印

## 让一切运转起

到目前为止，我们有一组不错的函数，它们可以在 seq_file 系统中产生输出，但我们还没有把它们变用户可见的文件。在内核中创建一个文件当然需要创建一file_operations 来实现该文件上的操作seq_file 接口提供了一组预制操作，完成了大部分工作。不过，虚拟文件的作者仍然必须实open()
方法来把一切都挂接起来。open 函数通常很简
```

	static int ct_open(struct inode *inode, struct file *file)
	{
		return seq_open(file, &ct_seq_ops);
	}

```
这里，对 seq_open() 的调用接受我们之前创建的 seq_operations 结构，并设置为遍历虚拟文件
在成功打开时，seq_open() struct seq_file 指针存储file->private_data 中。如果你有某个应用，
其中同一个迭代器可用于多个文件，你可以把任意指针存储seq_file 结构private 字段中；该值随可被迭代器函数取回
还有一seq_open() 的包装函数叫 seq_open_private()。它 kmalloc 一块填零的内存，并把指向它指针存储seq_file 结构private 字段中，成功时返0。该

```

	static int ct_open(struct inode *inode, struct file *file)
	{
		return seq_open_private(file, &ct_seq_ops,
					sizeof(struct mystruct));
	}

```
还有一个变体函__seq_open_private()，它功能上完全相同，只是如果成功，它会返回指向所分配内存指针

```

	static int ct_open(struct inode *inode, struct file *file)
	{
		struct mystruct *p =
			__seq_open_private(file, &ct_seq_ops, sizeof(*p));

		if (!p)
			return -ENOMEM;

		p->foo = bar; /* 初始化我的东*/
			...
		p->baz = true;

		return 0;
	}

```
有一个对应的 close 函数 seq_release_private() 可用，它会释放在对应 open 中分配的内存
其他感兴趣的操作——read()、llseek() release()——全部由 seq_file 代码本身实现。因此一个虚文件
```

	static const struct file_operations ct_file_ops = {
	        .owner   = THIS_MODULE,
	        .open    = ct_open,
	        .read    = seq_read,
	        .llseek  = seq_lseek,
	        .release = seq_release
	};

```
还有一seq_release_private()，它在释放结构之前把 seq_file private 字段的内容传kfree()
最后一步是创建 /proc 文件本身。在示例
```

	static int ct_init(void)
	{
	        struct proc_dir_entry *entry;

	        proc_create("sequence", 0, NULL, &ct_file_ops);
	        return 0;
	}

	module_init(ct_init);

```
而这基本上就是全部了

## seq_list


如果你的文件要遍历一个链表，你可能会用到这些

```

	struct list_head *seq_list_start(struct list_head *head,
	       		 		 loff_t pos);
	struct list_head *seq_list_start_head(struct list_head *head,
			 		      loff_t pos);
	struct list_head *seq_list_next(void *v, struct list_head *head,
					loff_t *ppos);

```
这些辅助函数会把 pos 解释为链表中的一个位置，并相应地进行迭代。你start() next() 函数只需
要用一个指向相list_head 结构的指针来调用 `seq_list_*` 辅助函数

## 极简版本


对于极其简单的虚拟文件，有一个更简单的接口。一个模块可以只定义 show() 函数，它应该创建虚拟文件
将包含的所有输出。该文件open() 方法随后

```

	int single_open(struct file *file,
	                int (*show)(struct seq_file *m, void *p),
	                void *data);

```
当输出时刻到来时，show() 函数会被调用一次。传single_open() data 值可以在 seq_file 结构private 字段中找到。使single_open() 时，程序员应该在 file_operations 结构中使single_release()
而非 seq_release()，以避免内存泄漏