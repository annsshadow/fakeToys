## 基于 connector 的用户空间通信协议


## 消息类型


w1 核心与用户空间之间有三种类型的消息：

1. 事件。每当通过自动或请求式搜索发现一个新的主设备或从设备时生成。
2. 用户空间命令。
3. 对用户空间命令的回复。


## 协议


```

  [struct cn_msg] - connector 头部。
	其 length 字段等于附带数据的大小
  [struct w1_netlink_msg] - w1 netlink 头部。
	__u8 type 	- 消息类型。
			W1_LIST_MASTERS
				列出当前的总线主设备
			W1_SLAVE_ADD/W1_SLAVE_REMOVE
				从设备添加/移除事件
			W1_MASTER_ADD/W1_MASTER_REMOVE
				主设备添加/移除事件
			W1_MASTER_CMD
				面向总线主设备的用户空间命令
				（搜索/报警搜索）
			W1_SLAVE_CMD
				面向从设备的用户空间命令
				（读/写/触摸）
	__u8 status	- 来自内核的错误指示
	__u16 len	- 附加到此头部数据的数据大小
	union {
		__u8 id[8];			 - 从设备唯一 id
		struct w1_mst {
			__u32		id;	 - 主设备 id
			__u32		res;	 - 保留
		} mst;
	} id;

  [struct w1_netlink_cmd] - 给定主设备或从设备的命令。
	__u8 cmd	- 命令操作码。
			W1_CMD_READ 	- 读命令
			W1_CMD_WRITE	- 写命令
			W1_CMD_SEARCH	- 搜索命令
			W1_CMD_ALARM_SEARCH - 报警搜索命令
			W1_CMD_TOUCH	- 触摸命令
				（写数据并将采样结果返回用户空间）
			W1_CMD_RESET	- 发送总线复位
			W1_CMD_SLAVE_ADD	- 将 slave 添加到内核列表
			W1_CMD_SLAVE_REMOVE	- 从内核列表移除 slave
			W1_CMD_LIST_SLAVES	- 从内核获取 slave 列表
	__u8 res	- 保留
	__u16 len	- 此命令的数据长度
		For read command data must be allocated like for write command
	__u8 data[0]	- 此命令的数据


```
每个 connector 消息可以包含一个或多个 w1_netlink_msg，以及零个或多个附带的 w1_netlink_cmd 消息。

对于事件消息，没有嵌入的 w1_netlink_cmd 结构，只有 connector 头部和 w1_netlink_msg 结构，其中 "len" 字段为零，并填充了类型（事件类型之一）和 id：要么是主机字节序的 8 字节从设备唯一 id，要么是主设备的 id（在将其添加到 w1 核心时分配给总线主设备）。

目前仅对读命令请求生成对用户空间命令的回复。每个 w1_netlink_cmd 读请求恰好生成一个回复。发送时回复不会合并——即典型的回复
```

  [cn_msg][w1_netlink_msg][w1_netlink_cmd]
  cn_msg.len = sizeof(struct w1_netlink_msg) +
	     sizeof(struct w1_netlink_cmd) +
	     cmd->len;
  w1_netlink_msg.len = sizeof(struct w1_netlink_cmd) + cmd->len;
  w1_netlink_cmd.len = cmd->len;

```
对 W1_LIST_MASTERS 的回复应向用户空间发回一条消息，其中包含以下形式的所有已注册主设备 id 列表
```

	cn_msg (CN_W1_IDX.CN_W1_VAL 作为 id，len 等于 sizeof(struct
	w1_netlink_msg) 加上主设备数量乘以 4)
	w1_netlink_msg (type: W1_LIST_MASTERS, len 等于
		主设备数量乘以 4 (u32 大小))
	id0 ... idN

```
每条消息最大为 4k，因此如果主设备数量超过此值，它将被拆分为多条消息。

W1 搜索和报警搜索命令。
```

  [cn_msg]
    [w1_netlink_msg type = W1_MASTER_CMD
	id is equal to the bus master id to use for searching]
    [w1_netlink_cmd cmd = W1_CMD_SEARCH or W1_CMD_ALARM_SEARCH]

```
```

  [cn_msg, ack = 1 and increasing, 0 means the last message,
	seq is equal to the request seq]
  [w1_netlink_msg type = W1_MASTER_CMD]
  [w1_netlink_cmd cmd = W1_CMD_SEARCH or W1_CMD_ALARM_SEARCH
	len is equal to number of IDs multiplied by 8]
  [64bit-id0 ... 64bit-idN]

```
每个头部中的长度对应于其后面数据的大小，因此
w1_netlink_cmd->len = N * 8；其中 N 是本消息中 ID 的数量。可以为零。
```

  w1_netlink_msg->len = sizeof(struct w1_netlink_cmd) + N * 8;
  cn_msg->len = sizeof(struct w1_netlink_msg) +
	      sizeof(struct w1_netlink_cmd) +
	      N*8;

```
```

  [cn_msg]
    [w1_netlink_msg type = W1_MASTER_CMD
	id is equal to the bus master id to use for searching]
    [w1_netlink_cmd cmd = W1_CMD_RESET]


```
## 命令状态回复


每个命令（无论是 root、master 还是 slave，无论是否带有 w1_netlink_cmd 结构）都会被 w1 核心“确认”（acked）。回复的格式与请求消息相同，只是长度参数不计入用户请求的数据，即读/写/触摸 IO 请求将不包含数据，因此 w1_netlink_cmd.len 将为 0，w1_netlink_msg.len 将为 w1_netlink_cmd 结构的大小，而 cn_msg.len 将等于 sizeof(struct w1_netlink_msg) 与 sizeof(struct w1_netlink_cmd) 之和。如果回复是为 master 或 root 命令（不带 w1_netlink_cmd）生成的，则回复仅包含 cn_msg 和 w1_netlink_msg 结构。

w1_netlink_msg.status 字段将携带正的错误值（例如 EINVAL）或成功时的 0。

每个结构中的所有其他字段将镜像请求消息中的相同参数（除上述长度外）。

会为 w1_netlink_msg 中嵌入的每个 w1_netlink_cmd 生成状态回复；如果没有 w1_netlink_cmd 结构，则将为 w1_netlink_msg 生成回复。

在每个 w1_netlink_msg 中，所有 w1_netlink_cmd 命令结构都会被处理，即使存在错误，只有长度不匹配才会中断消息处理。


## 当接收到新命令时 w1 核心中的操作步骤


当接收到新消息（w1_netlink_msg）时，w1 核心根据 w1_netlink_msg.type 字段检测它是 master 还是 slave 请求。然后搜索 master 或 slave 设备。找到后，master 设备（被请求的，或是找到 slave 设备的那个）被锁定。如果请求的是 slave 命令，则启动复位/选择（reset/select）过程以选择给定设备。

然后 w1_netlink_msg 中请求的所有操作被逐一执行。如果命令需要回复（如读命令），则在命令完成时发送。

当所有命令（w1_netlink_cmd）处理完毕后，master 设备被解锁，并开始处理下一个 w1_netlink_msg 头部。


## Connector [1] 特定文档


每个 connector 消息包含两个 u32 字段作为“地址”。w1 使用在 include/linux/connector.h 头文件中定义的 CN_W1_IDX 和 CN_W1_VAL。每个消息还包含序列号和确认号。

事件消息的序列号是相应的总线主设备序列号，每通过该主设备发送一条事件消息就递增。用户空间请求的序列号由用户空间应用程序设置。回复的序列号与请求中的相同，确认号设为 seq+1。


## 附加文档、源代码示例


1. Documentation/driver-api/connector.rst
2. https://github.com/bioothod/w1

   此归档包含用户空间应用程序 w1d.c，它使用读/写/搜索命令操作总线上找到的所有 master/slave 设备。
