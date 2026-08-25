## 延迟 IO（Deferred IO

延迟 IO 是一种延迟并重用 IO 的方式。它使用主机内存作为缓冲区，并以 MMU 缺页（pagefault）作为执行设IO 的预触发条件。以下示例或许有助于解释这样一种设置是如何工作的：

- Xfbdev 这样的用户空间应用对帧缓冲（framebuffer）进mmap
- 延迟 IO 和驱动设fault page_mkwrite 处理函数
- 用户空间应用尝试写入mmap 的虚拟地址
- 我们收到 pagefault 并到fault 处理函数
- fault 处理函数找到并返回物理页
- 我们收到 page_mkwrite，在其中将此页加入一个链- 调度一workqueue 任务在延迟后运行
- 应用继续写入该页，无需额外开销。这是关键收益- workqueue 任务进入并清除（mkclean）链表上各页的脏标记，然后完成与更新帧缓冲相关的工作。这才是真正与设备对话的工作- 应用尝试写入该地址（此时该页已mkclean- 再次收到 pagefault，上述序列再次发
如上可见，其一项收益是大致允许以最小代价进行突发的帧缓冲写入。然后在经过一段时间后，当希望一切已安静下来时，我们才去真正更新帧缓冲，而这将是一个相对更昂贵的操作
对于某些类型的非易失性高延迟显示器，期望的图像是最终图像而非中间阶段，因此无需为每次发生的写入都进行更新
这种情况也可能在其他场景中很有用。Paul Mundt 提到了一个例子：利用页计数来决定是合并并发出 SG DMA，还是进行内存突发（memory burst）是有益的
另一个例子是，如果你有一个格式特殊的设备帧缓冲，比如对角移位RGB，那么这就可以成为一种机制，让你的应用假装拥有正常的帧缓冲，但在 vsync 时根据被写入的页链表为设备帧缓冲重新调整（reswizzle）
### 如何使用：（针对应用

无需任何改动。像平常一mmap 帧缓冲并直接使用它
### 如何使用：（针对 fbdev 驱动

以下示例可能有帮助
```

	static struct fb_deferred_io hecubafb_defio = {
		.delay		= HZ,
		.deferred_io	= hecubafb_dpy_deferred_io,
	};

```

delay page_mkwrite 触发发生deferred_io 回调被调用之间的最小延迟。deferred_io 回调将在下面解释
```

	static void hecubafb_dpy_deferred_io(struct fb_info *info,
					     struct list_head *pagelist)

```

deferred_io 回调是你对所有显示设备执IO 的地方。你会收pagelist，它是在延迟期间被写入的页的链表。你不得修改此链表。该回调从一workqueue 中被调用
```

	info->fbdefio = &hecubafb_defio;
	fb_deferred_io_init(info);

```

```

	fb_deferred_io_cleanup(info);

```
