## 通用通知机制


通用通知机制构建在标准管道（pipe）驱动之上，它实际上将来自内核的通知消息拼接到管道中
```

  * Key/keyring notifications


```
通知缓冲区可通过以下方式启用
	"General setup"/"General notification queue"
	(CONFIG_WATCH_QUEUE)

本文档包含以下章节：



## 概述


该机制表现为一个以特殊模式打开的管道。管道内部的环形缓冲区用于保存由内核生成的消息。这些消息随后由 read() 读出。此类管道上禁用splice 及类似操作，因为它们在某些情况下可能希望撤销对环形缓冲区的添加——而这最终可能会与通知消息交错在一起
管道的所有者必须告诉内核它希望通过该管道监视哪些来源。只有已连接到某个管道的来源才会向其中插入消息。请注意，一个来源可以绑定到多个管道，并同时向所有这些管道插入消息
也可以在管道上放置过滤器，以便在不感兴趣时忽略某些来源类型和子事件
如果环形缓冲区中没有可用的槽位，或者没有可用的预分配消息缓冲区，则消息将被丢弃。在这两种情况下，read() 会在缓冲区中当前最后一条消息被读取之后，向输出缓冲区插入一WATCH_META_LOSS_NOTIFICATION 消息
请注意，在产生通知时，内核不会等待消费者来收集它，而是直接继续。这意味着通知可以在持有自旋锁的情况下生成，同时也保护内核不会被用户空间的故障无限期地卡住

## 消息结构


```

	struct watch_notification {
		__u32	type:24;
		__u32	subtype:8;
		__u32	info;
	};

```
「type」表示通知记录的来源，"subtype" 表示该来源的记录类型（参见下面的“监视来源”一节）。type 也可能是 "WATCH_TYPE_META"。这是一种由监视队列自身在内部生成的特殊记录类型。它有两个子类型
  - WATCH_META_REMOVAL_NOTIFICATION
  - WATCH_META_LOSS_NOTIFICATION

前者表示安装了监视的对象被移除或销毁，后者表示某些消息已丢失
「info」表示多项内容，包括
  - 消息的长度（以字节为单位，含头部）（WATCH_INFO_LENGTH 掩码，并右移 WATCH_INFO_LENGTH__SHIFT）。这表示记录的大小，介于 8 127 字节之间
  - 监视 ID（用 WATCH_INFO_ID 掩码，并右移 WATCH_INFO_ID__SHIFT）。这表示监视的调用ID，介0 255 之间。多个监视可以共享一个队列，这提供了一种区分它们的方法
  - 类型特定的字段（WATCH_INFO_TYPE_INFO）。它由通知生产者设置，用于表示特定于该类型和子类型的某些含义
info 中除长度之外的所有内容都可用于过滤
头部之后可以跟随补充信息。其格式由类型和子类型自行定义

## 监视列表（通知来源）API


「watch list」（监视列表）是订阅了某个通知来源的监视者列表。一个列表可以附加到某个对象上（例如密钥或超级块），也可以是全局的（例如用于设备事件）。从用户空间的角度来看，非全局的监视列表通常通过其所属对象的引用来指代（例如使用 KEYCTL_NOTIFY 并给定一个密钥序列号来监视那个特定的密钥）
要管理监视列表，提供了以下函数：


```

	void init_watch_list(struct watch_list *wlist,
			     void (*release_watch)(struct watch *wlist));

    初始化一个监视列表。如``release_watch`` 不为 NULL，则它表示在
    watch_list 对象被销毁时应调用的一个函数，用于释放监视列表对被监视对象
    持有的任何引用
  * ``void remove_watch_list(struct watch_list *wlist);``

    移除订阅到某watch_list 的所有监视并释放它们，然后销    watch_list 对象本身

```
## 监视队列（通知输出）API


「watch queue」（监视队列）是应用程序分配的一段缓冲区，通知记录将被写入其中。其运作完全隐藏在管道设备驱动内部，但要设置监视，必须获取对它的引用。可以通过以下方式管理
  - `struct watch_queue *get_watch_queue(int fd);`

    由于监视队列是通过实现该缓冲区的管道的 fd 向内核标识的，用户空间必须通过系统调用传递该 fd。这可用于从系统调用中查找到监视队列的不透明指针
  - `void put_watch_queue(struct watch_queue *wqueue);`

    这将丢弃`get_watch_queue()` 获得的引用

## 监视订阅 API


「watch」（监视）是监视列表上的一个订阅，它指明了应将通知记录写入其中的监视队列（也就是缓冲区）。监视队列对象也可以携带该对象的过滤规则，这些规则由
```

	struct watch {
		union {
			u32		info_id;	/* ID to be OR'd in to info field */
			...
		};
		void			*private;	/* Private data for the watched object */
		u64			id;		/* Internal identifier */
		...
	};

```
`info_id` 值应是一个从用户空间获得8 位数，并左移 WATCH_INFO_ID__SHIFT。当通知被写入关联的监视队列缓冲区时，它会通过 OR 运算并入 **struct watch_notification** info 字段中的 WATCH_INFO_ID
`private` 字段是与 watch_list 关联的驱动数据，**``watch_list** : release_watch()`` 方法清理
`id` 字段是来源的 ID。以不同 ID 发布的通知会被忽略
提供以下函数来管理监视：

  - `void init_watch(struct watch **watch, struct watch_queue **wqueue);`

    初始化一个监视对象，将其指针设置为监视队列，并使用适当的屏障以避免 lockdep 警告
  - `int add_watch_to_object(struct watch **watch, struct watch_list **wlist);`

    将监视订阅到监视列表（通知来源）。在调用此函数之前，watch 结构体中驱动可设置的字段必须已经设置好
```

	int remove_watch_from_object(struct watch_list *wlist,
				     struct watch_queue *wqueue,
				     u64 id, false);

    从监视列表中移除一个监视，其中该监视必须匹配指定的监视队列
    （``wqueue``）和对象标识符（``id``）。会向监视队列发送一个通知
    （``WATCH_META_REMOVAL_NOTIFICATION``），指示该监视已被移除
  * ``int remove_watch_from_object(struct watch_list *wlist, NULL, 0, true);``

    移除监视列表中的所有监视。预计这将在销毁之前被调用，并且到此为    该监视列表对新的监视应当已不可访问。会向每个已订阅监视的监视队    发送一个通知（``WATCH_META_REMOVAL_NOTIFICATION``），指示该监    已被移除

```
## 通知发布 API


要将通知发布到监视列表，以便订阅的监视可以看到它```

	void post_watch_notification(struct watch_list *wlist,
				     struct watch_notification *n,
				     const struct cred *cred,
				     u64 id);

```
通知应预先格式化，并传入指向头部（`n`）的指针。通知可能大于此大小，以缓冲区槽位为单位的尺寸记录`n->info & WATCH_INFO_LENGTH` 中
`cred` 结构体表示来源（主体）的凭证，它被传递给 LSM（如 SELinux），以根据该队列（对象）的凭证允许或抑制在各队列中记录该通知
`id` 是来源对象的 ID（例如密钥上的序列号）。只有设置了相同 ID 的监视才能看到此通知

## 监视来源


任何特定的缓冲区都可以由多个来源提供数据。来源包括：

  - WATCH_TYPE_KEY_NOTIFY

    此类通知表示密钥和密钥环的变更，包括密钥环内容的变更或密钥属性的变更
    更多信息请参Documentation/security/keys/core.rst

## 事件过滤


一旦创建了监视队列，就可以应用一组过滤器来限```

	struct watch_notification_filter filter = {
		...
	};
	ioctl(fd, IOC_WATCH_QUEUE_SET_FILTER, &filter)

```
```

	struct watch_notification_filter {
		__u32	nr_filters;
		__u32	__reserved;
		struct watch_notification_type_filter filters[];
	};

```
其中 "nr_filters" filters[] 中过滤器的数量，"__reserved"
```

	struct watch_notification_type_filter {
		__u32	type;
		__u32	info_filter;
		__u32	info_mask;
		__u32	subtype_filter[8];
	};

```
其中
  - `type` 是要过滤的事件类型，应为类似
    "WATCH_TYPE_KEY_NOTIFY" 的
  - `info_filter` `info_mask` 用作info 字段的过滤器，其
```

	(watch.info & info_mask) == info_filter

    例如，这可用于忽略那些不在挂载树中被监视点上的事件
  * ``subtype_filter`` 是一个位掩码，指示感兴趣的子类型。subtype_filter[0]
    的第 0 位对应子类型 0，第 1 位对应子类型 1，依此类推
```
如果 ioctl() 的参数为 NULL，则过滤器将被移除，来自被监视来源的所有事件都将通过

## 用户空间代码示例


```

	pipe2(fds, O_TMPFILE);
	ioctl(fds[1], IOC_WATCH_QUEUE_SET_SIZE, 256);

```
```

	keyctl(KEYCTL_WATCH_KEY, KEY_SPEC_SESSION_KEYRING, fds[1], 0x01);

```
```

	static void consumer(int rfd, struct watch_queue_buffer *buf)
	{
		unsigned char buffer[128];
		ssize_t buf_len;

		while (buf_len = read(rfd, buffer, sizeof(buffer)),
		       buf_len > 0
		       ) {
			void *p = buffer;
			void *end = buffer + buf_len;
			while (p < end) {
				union {
					struct watch_notification n;
					unsigned char buf1[128];
				} n;
				size_t largest, len;

				largest = end - p;
				if (largest > 128)
					largest = 128;
				memcpy(&n, p, largest);

				len = (n->info & WATCH_INFO_LENGTH) >>
					WATCH_INFO_LENGTH__SHIFT;
				if (len == 0 || len > largest)
					return;

				switch (n.n.type) {
				case WATCH_TYPE_META:
					got_meta(&n.n);
				case WATCH_TYPE_KEY_NOTIFY:
					saw_key_change(&n.n);
					break;
				}

				p += len;
			}
		}
	}

```
