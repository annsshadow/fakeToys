
## Kernel Connector


内核连接器（Kernel connector）——一种基于 netlink 的、用户空间 <-> 内核空间之间易于使用的通信模块。

连接器驱动使得使用基于 netlink 的网络连接各种代理变得容易。使用者必须注册一个回调和一个标识符。当驱动收到带有相应标识符的特殊 netlink 消息时，就会调用相应的回调。

从用户空间的角度来看，这相当直接：

 - socket();
 - bind();
 - send();
 - recv();

但如果内核空间想要充分利用这种连接的威力，驱动编写者必须创建特殊的套接字，必须了解 struct sk_buff 的处理等等……连接器驱动允许任何内核空间代理以显著简化的方式使用基于 netlink 的网络进行进程间通信：

```

  int cn_add_callback(const struct cb_id *id, char *name, void (*callback) (struct cn_msg *, struct netlink_skb_parms *));
  void cn_netlink_send_mult(struct cn_msg *msg, u16 len, u32 portid, u32 __group, int gfp_mask);
  void cn_netlink_send(struct cn_msg *msg, u32 portid, u32 __group, int gfp_mask);

  struct cb_id
  {
	__u32			idx;
	__u32			val;
  };

```
idx 和 val 是唯一标识符，必须在 connector.h 头文件中注册以供内核内部使用。`void (**callback) (void **)` 是一个回调函数，当连接器核心收到带有上述 idx.val 的消息时会被调用。该函数的参数必须是：

```

  struct cn_msg
  {
	struct cb_id		id;

	__u32			seq;
	__u32			ack;

	__u16			len;	/* 后续数据的长度 */
	__u16			flags;
	__u8			data[0];
  };

```
## Connector interfaces


 .. kernel-doc:: include/linux/connector.h

 注意：
   在注册新的回调用户时，连接器核心会分配给该用户一个 netlink 组，其值等于它的 id.idx。

## Protocol description


当前框架提供了一个带有固定头的传输层。使用该头部的推荐协议如下：

msg->seq 和 msg->ack 用于确定消息的谱系。当某方发送一条消息时，它会使用一个本地唯一的序列号和随机的确认号。该序列号也可以复制到 nlmsghdr->nlmsg_seq 中。

序列号随每条发送的消息递增。

如果你期望收到对该消息的回复，那么接收到的消息中的序列号必须与原消息相同，且确认号必须是原序列号 + 1。

如果我们收到一条消息，其序列号与我们期望的不相等，那么它就是一条新消息。如果我们收到一条消息，其序列号与我们期望的相同，但确认号不等于原消息中的序列号 + 1，那么它也是一条新消息。

显然，协议头部包含了上述的 id。

连接器允许以如下形式进行事件通知：内核驱动或用户空间进程可以请求连接器在选定的 id 被打开或关闭（注册或注销其回调）时通知它。这是通过向连接器驱动发送一条特殊命令来完成的（它自身也以 id={-1, -1} 注册）。

关于这种用法的示例可以在 cn_test.c 模块中找到，该模块使用连接器来请求通知并发送消息。

## Reliability


Netlink 本身并不是一个可靠的协议。这意味着消息可能会由于内存压力或进程的接收队列溢出而丢失，因此调用者被警告必须有所准备。这就是为什么 struct cn_msg（连接器的主要消息头）包含 u32 seq 和 u32 ack 字段。

## Userspace usage


2.6.14 引入了一种新的 netlink 套接字实现，默认情况下不允许向除 1 以外的 netlink 组发送数据。
因此，如果你希望使用具有不同组号的 netlink 套接字（例如使用连接器），用户空间应用程序必须订阅：

```

  s = socket(PF_NETLINK, SOCK_DGRAM, NETLINK_CONNECTOR);

  l_local.nl_family = AF_NETLINK;
  l_local.nl_groups = 12345;
  l_local.nl_pid = 0;

  if (bind(s, (struct sockaddr *)&l_local, sizeof(struct sockaddr_nl)) == -1) {
	perror("bind");
	close(s);
	return -1;
  }

  {
	int on = l_local.nl_groups;
	setsockopt(s, 270, 1, &on, sizeof(on));
  }

```
其中上面的 270 是 SOL_NETLINK，1 是 NETLINK_ADD_MEMBERSHIP 套接字选项。要取消多播订阅，应使用定义为 0 的 NETLINK_DROP_MEMBERSHIP 参数调用上述套接字选项。

2.6.14 的 netlink 代码只允许选择小于或等于最大组号的组，该最大组号在 netlink_kernel_create() 时使用。对于连接器而言，它是 CN_NETLINK_USERS + 0xf，因此如果你想使用组号 12345，必须将 CN_NETLINK_USERS 增加到该数值。额外的 0xf 个编号分配给内核外部的用户使用。

由于这一限制，组 0xffffffff 目前无法工作，因此不能使用添加/删除连接器的组通知，但据我所知，只有 cn_test.c 测试模块使用过它。

netlink 领域的一些工作仍在进行中，因此在 2.6.15 期间可能会有变动，如果发生，将会更新对应内核的文档。

## Code samples


连接器测试模块和用户空间的示例代码可以在 samples/connector/ 中找到。要构建这些代码，请启用 CONFIG_CONNECTOR 和 CONFIG_SAMPLES。
