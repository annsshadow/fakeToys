
## Netlink 简

Netlink 常被描述ioctl() 的替代品。它旨在用一种便于方便地添加或扩参数的格式，来替代提供给 ioctl() 的固定格C 结构体
为此，Netlink 使用一个最小的固定格式元数据头，其后跟随多个采TLV
（类型、长度、值）格式的属性
遗憾的是，该协议多年来以有机且未文档化的方式演变，使得很难连贯地解释为了最切合实际，本文档首先描述今天所使用netlink，并在后面的章节深入
探讨更具“历史”用途的用法
## 鎵撳紑濂楁帴瀛。

Netlink 通信通过套接字进行，首先需要打开一个套接字

  fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);

套接字的使用提供了一种自然的方式在双向（发往内核与来自内核）交换信息当应用程send() 请求时，操作仍然是同步执行的，但需要单独的 recv()
系统调用来读取回复
Netlink “调用”的一个非常简化的流程大致如下

  fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);

  /** format the request **/
  send(fd, &request, sizeof(request));
  n = recv(fd, &response, RSP_BUFFER_SIZE);
  /** interpret the response **/

Netlink 还天然支持“dumping”（转储），即向用户空间传递某一类型的所有对（例如转储所有的网络接口）

  fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);

  /** format the dump request **/
  send(fd, &request, sizeof(request));
  while (1) {
    n = recv(fd, &buffer, RSP_BUFFER_SIZE);
    /** one recv() call can read multiple messages, hence the loop below **/
    for (nl_msg in buffer) {
      if (nl_msg.nlmsg_type == NLMSG_DONE)
        goto dump_finished;
      /** process the object **/
    }
  }
  dump_finished:

socket() 调用的前两个参数无需太多解释——它打开一Netlink 套接字，所头部由用户提供（因此NETLINK、RAW）。最后一个参数是 Netlink 内部的协议该字段用于标识套接字将与之通信的子系统
### 经典 Netlink 与通用 Netlink


Netlink 的最初实现依赖于向子系统静态分ID，并提供很少的支持基础设施我们将这些协议统称为 **Classic Netlink（经Netlink*。它们的列表定义`include/uapi/linux/netlink.h` 文件之上，其中包括通用网络
（NETLINK_ROUTE）、iSCSI（NETLINK_ISCSI）和审计（NETLINK_AUDIT）等
**Generic Netlink（通用 Netlink*（于 2005 年引入）允许动态注册子系统
（以及子系统 ID 分配）、自省，并简化了接口内核侧的实现
下一节描述如何使Generic Netlink，因为使Generic Netlink 的子系统数量
比旧协议多出一个数量级。内核也没有计划添加更多 Classic Netlink 协议。关如何Linux 内核的核心网络部分（或使Classic Netlink 的另20 个子系统
之一）通信、以及它Generic Netlink 的区别，本文档后面会提供基本信息
## 通用 Netlink


除了 Netlink 固定元数据头之外，每Netlink 协议都定义了自己的固定元数据
头。（类似于网络头部的堆叠——Ethernet > IP > TCP，我们有
Netlink > Generic N. > Family。）

一Netlink 消息总是struct nlmsghdr 开始，其后跟随一个协议特定的头部Generic Netlink 的情况下，该协议头部struct genlmsghdr
Generic Netlink 的情况下，各字段的实际含义如下：


  struct nlmsghdr {
	__u32	nlmsg_len;	/** Length of message including headers **/
	__u16	nlmsg_type;	/** Generic Netlink Family (subsystem) ID **/
	__u16	nlmsg_flags;	/** Flags - request or dump **/
	__u32	nlmsg_seq;	/** Sequence number **/
	__u32	nlmsg_pid;	/** Port ID, set to 0 **/
  };
  struct genlmsghdr {
	__u8	cmd;		/** Command, as defined by the Family **/
	__u8	version;	/** Irrelevant, set to 1 **/
	__u16	reserved;	/** Reserved, set to 0 **/
  };
  /** TLV attributes follow... **/

Classic Netlink 中，:c`nlmsghdr.nlmsg_type` 用于标识消息所指的是子系统
内的哪个操作（例如获取关于某netdev 的信息）。Generic Netlink 需要在一协议里多路复用多个子系统，因此它用该字段来标识子系统，而由
:c`genlmsghdr.cmd` 来标识操作。（关于如何找到感兴趣子系统Family ID请参res_fam。）请注意，Classic Netlink Generic Netlink 中，该字的前 16 个值（0 - 15）都保留用于控制消息。更多细节请参阅 nl_msg_type
Netlink 套接字上通常3 种消息交换类型：

 - 执行单个动作（`do`）；
 - 转储信息（`dump`）；
 - 获取异步通知（`multicast`）
Classic Netlink 非常灵活，大概也允许其他类型的交换发生，但在实践中用到的
就是这三类
异步通知由内核发送，由订阅了它们的用户套接字接收。`do` `dump` 请求用户发起c`nlmsghdr.nlmsg_flags` 应按如下方式设置
 - 对于 `do`：`NLM_F_REQUEST | NLM_F_ACK`
 - 对于 `dump`：`NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP`

:c`nlmsghdr.nlmsg_seq` 应设置为一个单调递增的值。该值会在响应中被回显，
实践中并不重要，但为发送的每条消息设置一个递增的值被视为良好的习惯。该
字段的目的是将响应与请求匹配。异步通知:c`nlmsghdr.nlmsg_seq` 将为
`0`銆。
:c`nlmsghdr.nlmsg_pid` Netlink 中相当于地址的字段。与内核通信时该字段
可设`0`。关于该字段（不常见）的用途，请参nlmsg_pid
:c`genlmsghdr.version` 的预期用途是允许对子系统提供API 进行版本管理迄今为止没有任何子系统大量使用该字段，因此将其设`1` 似乎是稳妥的选择

### Netlink 消息类型


如前所述，:c`nlmsghdr.nlmsg_type` 携带协议特定的值，但前 16 个标识符保留的（第一个子系统特定的消息类型应等于 `NLMSG_MIN_TYPE`，即 `0x10`）
只定义了 4 Netlink 控制消息
 - `NLMSG_NOOP` - 忽略该消息，实践中未使用 - `NLMSG_ERROR` - 携带操作的返回码 - `NLMSG_DONE` - 标记一dump 的结束；
 - `NLMSG_OVERRUN` - 套接字缓冲区已溢出，至今未使用
`NLMSG_ERROR` `NLMSG_DONE` 具有实际重要性。它们携带操作的返回码。请
注意，除非在请求上设置了 `NLM_F_ACK` 标志，否则如果没有错误，Netlink 不会
`NLMSG_ERROR` 回应。为了避免必须为这种怪异行为特殊处理，建议始终设`NLM_F_ACK`
```

  ----------------------------------------------
  | struct nlmsghdr - response header          |
  ----------------------------------------------
  |    int error                               |
  ----------------------------------------------
  | struct nlmsghdr - original request header |
  ----------------------------------------------
  | ** optionally (1) payload of the request   |
  ----------------------------------------------
  | ** optionally (2) extended ACK             |
  ----------------------------------------------

```
这里有两struct nlmsghdr 实例，第一个属于响应，第二个属于请求`NLMSG_ERROR` 携带导致错误的请求的信息。这在尝试将请求与响应匹配，或重解析请求以转储到日志中时可能很有用
请求的有效载荷不会在报告成功的消息（`error == 0`）中回显，如果设置了
`NETLINK_CAP_ACK` setsockopt() 也不会回显。后者很常见，或许也值得推荐因为不得不从内核读回每个请求的副本是相当浪费的。请求有效载荷的缺失:c`nlmsghdr.nlmsg_flags` 中的 `NLM_F_CAPPED` 指示
`NLMSG_ERROR` 的第二个可选元素是扩展 ACK 属性。更多细节请参阅 ext_ack扩展 ACK 的存在由 :c`nlmsghdr.nlmsg_flags` 中的 `NLM_F_ACK_TLVS` 指示
`NLMSG_DONE` 更简单，请求永远不会被回显，但扩```

  ----------------------------------------------
  | struct nlmsghdr - response header          |
  ----------------------------------------------
  |    int error                               |
  ----------------------------------------------
  | ** optionally extended ACK                 |
  ----------------------------------------------

```
请注意，某些实现可能会发出自定义`NLMSG_DONE` 消息来回`do` 动作请求在这种情况下，有效载荷是实现特定的，也可能不存在

### 解析 Family ID


本节说明如何找到子系统的 Family ID。它同时也作Generic Netlink 通信一个示例
Generic Netlink 本身就是一个通过 Generic Netlink API 暴露的子系统。为避免
循环依赖，Generic Netlink 有一个静态分配的 Family ID（`GENL_ID_CTRL`，等`NLMSG_MIN_TYPE`）。Generic Netlink family 实现了一个用于查询其family
信息的命令（`CTRL_CMD_GETFAMILY`）
要获取例如名`"test1"` Generic Netlink family 的信息，我们需要在之前
打开Generic Netlink 套接字上发送一条消息。该消息应指Generic Netlink
Family），是对 `CTRL_CMD_GETFAMILY`）的一`do`）调用。此调用`dump` 版本会让内核以其所知的 **所* family 的信息来回应。最后但同样重要
的是，相family 的名称包```

  struct nlmsghdr:
    __u32 nlmsg_len:	32
    __u16 nlmsg_type:	GENL_ID_CTRL               // (1)
    __u16 nlmsg_flags:	NLM_F_REQUEST | NLM_F_ACK  // (2)
    __u32 nlmsg_seq:	1
    __u32 nlmsg_pid:	0

  struct genlmsghdr:
    __u8 cmd:		CTRL_CMD_GETFAMILY         // (3)
    __u8 version:	2 /* or 1, doesn't matter */
    __u16 reserved:	0

  struct nlattr:                                   // (4)
    __u16 nla_len:	10
    __u16 nla_type:	CTRL_ATTR_FAMILY_NAME
    char data: 		test1\0

  (padding:)
    char data:		\0\0

```
Netlink 中的长度字段c`nlmsghdr.nlmsg_len` :c`nlattr.nla_len`）总是
**包含** 头部。Netlink 中的属性头部必须从消息起始位置对齐4 字节，因此在
`CTRL_ATTR_FAMILY_NAME` 之后有额外的 `\0\0`。属性长**不包* 填充
如果找到了该 family，内核会用两条消息回应，即响```

  /* Message #1 - reply */
  struct nlmsghdr:
    __u32 nlmsg_len:	136
    __u16 nlmsg_type:	GENL_ID_CTRL
    __u16 nlmsg_flags:	0
    __u32 nlmsg_seq:	1    /* echoed from our request */
    __u32 nlmsg_pid:	5831 /* The PID of our user space process */

  struct genlmsghdr:
    __u8 cmd:		CTRL_CMD_GETFAMILY
    __u8 version:	2
    __u16 reserved:	0

  struct nlattr:
    __u16 nla_len:	10
    __u16 nla_type:	CTRL_ATTR_FAMILY_NAME
    char data: 		test1\0

  (padding:)
    data:		\0\0

  struct nlattr:
    __u16 nla_len:	6
    __u16 nla_type:	CTRL_ATTR_FAMILY_ID
    __u16: 		123  /* The Family ID we are after */

  (padding:)
    char data:		\0\0

  struct nlattr:
    __u16 nla_len:	9
    __u16 nla_type:	CTRL_ATTR_FAMILY_VERSION
    __u16: 		1

  /* ... etc, more attributes will follow. */

```
```

  /* Message #2 - the ACK */
  struct nlmsghdr:
    __u32 nlmsg_len:	36
    __u16 nlmsg_type:	NLMSG_ERROR
    __u16 nlmsg_flags:	NLM_F_CAPPED /* There won't be a payload */
    __u32 nlmsg_seq:	1    /* echoed from our request */
    __u32 nlmsg_pid:	5831 /* The PID of our user space process */

  int error:		0

  struct nlmsghdr: /* Copy of the request header as we sent it */
    __u32 nlmsg_len:	32
    __u16 nlmsg_type:	GENL_ID_CTRL
    __u16 nlmsg_flags:	NLM_F_REQUEST | NLM_F_ACK
    __u32 nlmsg_seq:	1
    __u32 nlmsg_pid:	0

```
属性的顺序（struct nlattr）不保证，因此用户必须遍历属性并解析它们
请注意，Generic Netlink 套接字并不关联或绑定到单一 family。一个套接字可用与许多不同的 family 交换消息，通过 :c`nlmsghdr.nlmsg_type` 字段逐条消息选择接收family

### 扩展 ACK


扩展 ACK 控制 `NLMSG_ERROR` `NLMSG_DONE` 消息中额外错警告 TLV 报告。为了保持向后兼容，此功能必须通过`NETLINK_EXT_ACK` setsockopt()
设为 `1` 来显式启用
扩展 ack 属性的类型定义enum nlmsgerr_attrs。最常用的属性是
`NLMSGERR_ATTR_MSG`、`NLMSGERR_ATTR_OFFS` `NLMSGERR_ATTR_MISS_*`
`NLMSGERR_ATTR_MSG` 携带一条描述所遇问题的英文消息。这些消息比通过标准
UNIX 错误码所能表达的详细得多
`NLMSGERR_ATTR_OFFS` 指向导致问题的属性
`NLMSGERR_ATTR_MISS_TYPE` `NLMSGERR_ATTR_MISS_NEST` 报告缺失的属性
扩展 ACK 既可在出错时报告，也可在成功时报告。后者应被视为警告
扩展 ACK 极大地提升了 Netlink 的可用性，应当始终启用、恰当地解析并报告给
用户
## 高级主题


### Dump 一致

内核用于存储对象的部分数据结构，使得难以提供一dump 中所有对象的原子
快照（同时不影响更新它们的快速路径）
如果 dump 被中断并可能不一致（例如缺少对象），内核可能dump 中的任何
消息上（包括 `NLMSG_DONE` 消息）设`NLM_F_DUMP_INTR` 标志。用户空间在
看到该标志时应重dump
### 自省


基本自省能力通过访问 res_fam 中报告的 Family 对象来启用。用户可以查询关Generic Netlink family 的信息，包括内核支持哪些操作、内核理解哪些属性Family 信息包含内核可解析的属性的最ID，一个单独的命令
（`CTRL_CMD_GETPOLICY`）提供关于受支持属性的详细信息，包括内核接受的范围
当用户空间需要在发出请求之前确认内核是否支持某个功能时，查询 family 信息
很有用

### nlmsg_pid


:c`nlmsghdr.nlmsg_pid` Netlink 中相当于地址的字段。它被称Port ID有时也叫 Process ID，因为出于历史原因，如果应用程序未选择（bind() 到）一显式Port ID，内核会自动将其分配为等于其 Process ID ID（由 getpid()
系统调用报告）
TCP/IP 网络协议bind() 语义类似，零值表示“自动分配”，因此应用程序
通常会将 :c`nlmsghdr.nlmsg_pid` 字段初始化为 `0`
该字段今天在罕见情况下仍在使用，即内核需要发送单播通知时。用户空间应程序可以使用 bind() 将其套接字与特定PID 关联，然后将它的 PID 告知内核这样内核就能联系到特定的用户空间进程
这类通信用于类似 UMH（User Mode Helper）的场景，即内核需要触发用户空处理或向用户空间询问策略决策时
### 组播通知


Netlink 的优势之一是能够向用户空间发送事件通知。这是一种单向通信形式
（内-> 用户），不涉及任何像 `NLMSG_ERROR` `NLMSG_DONE` 这样的控消息
例如，Generic Netlink family 自身就定义了一组关于已注册 family 的组通知。当添加一个新family 时，
```

  struct nlmsghdr:
    __u32 nlmsg_len:	136
    __u16 nlmsg_type:	GENL_ID_CTRL
    __u16 nlmsg_flags:	0
    __u32 nlmsg_seq:	0
    __u32 nlmsg_pid:	0

  struct genlmsghdr:
    __u8 cmd:		CTRL_CMD_NEWFAMILY
    __u8 version:	2
    __u16 reserved:	0

  struct nlattr:
    __u16 nla_len:	10
    __u16 nla_type:	CTRL_ATTR_FAMILY_NAME
    char data: 		test1\0

  (padding:)
    data:		\0\0

  struct nlattr:
    __u16 nla_len:	6
    __u16 nla_type:	CTRL_ATTR_FAMILY_ID
    __u16: 		123  /* The Family ID we are after */

  (padding:)
    char data:		\0\0

  struct nlattr:
    __u16 nla_len:	9
    __u16 nla_type:	CTRL_ATTR_FAMILY_VERSION
    __u16: 		1

  /* ... etc, more attributes will follow. */

```
该通知包含与对 `CTRL_CMD_GETFAMILY` 请求的响应相同的信息
通知Netlink 头部大多0 且无关紧要c`nlmsghdr.nlmsg_seq` 可以是零也可以是family 维护的单调递增的通知序列号
要接收通知，用户套接字必须订阅相关的通知组。与 Family ID 非常相似，给组播组的 Group ID 是动态的，可以在 Family 信息中找到。`CTRL_ATTR_MCAST_GROUPS`
属性包含嵌套，其中有各组的名称（`CTRL_ATTR_MCAST_GRP_NAME`）和 ID
（`CTRL_ATTR_MCAST_GRP_ID`）
一旦知道了 Group ID，一setsockopt() 调用就会将该套接字加入该组：


  unsigned int group_id;

  /** .. find the group ID... **/

  setsockopt(fd, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP,
             &group_id, sizeof(group_id));

该套接字现在将接收通知
建议为接收通知和向内核发送请求使用单独的套接字。通知的异步特性意味着它们
可能会与响应混在一起，使得消息处理困难得多
### 缓冲区大

Netlink 套接字是数据报套接字而非流套接字，这意味着每条消息都必须由单次
recv()/recvmsg() 系统调用完整地接收。如果用户提供的缓冲区太短，消息将被
截断，并struct msghdr 中设`MSG_TRUNC` 标志（struct msghdr recvmsg()
系统调用的第二个参数*不是** Netlink 头部）
截断后，消息的剩余部分将被丢弃
Netlink 期望用户缓冲区至少为 8kB，或 CPU 架构的页大小，取两者中较大者然而，特定Netlink family 可能要求更大的缓冲区。为最高效地处dump推荐使用 32kB 缓冲区（更大的缓冲区可容纳更多被 dump 的对象，因此需要的
recvmsg() 调用更少）

## 经典 Netlink


Classic Generic Netlink 的主要区别在于子系统标识符的动态分配以及自省的
可用性。理论上该协议没有显著差异，然而在实践中，Classic Netlink 试验了一
些在 Generic Netlink 中被废弃的概念（实际上，它们通常只在单个子系统的一小角落里使用过）。本节旨在解释其中几个概念，明确目标是让 Generic Netlink
用户在阅uAPI 头部时能有信心忽略它们
这里的大多数概念和示例都涉及 `NETLINK_ROUTE` family，它涵盖Linux 网络
栈的大部分配置。对family 的真正文档值得单独写一章（或一本书）
### Families


Netlink 将子系统称为 families。这是使用套接字和协议族概念的遗留产物，协议族是 `NETLINK_ROUTE` 中消息解复用的组成部分
遗憾的是，每一层封装都喜欢把它所承载的东西称为“families”，使得这个术语
非常令人困惑
 1. AF_NETLINK 是一个名副其实的套接字协议族
 2. AF_NETLINK 的文档将消息中它自身头部（struct nlmsghdr）之后的内容称为
    “Family Header 3. Generic Netlink AF_NETLINK 的一family（struct genlmsghdr 跟随
    struct nlmsghdr），但它也称其用户为“Families”
请注意，Generic Netlink Family ID 处于不同的“ID 空间”中，并且与 Classic
Netlink 协议号重叠（例如 `NETLINK_CRYPTO` Classic Netlink 协议 ID 21Generic Netlink 也会很乐意将其分配给它的某个 family）
### 严格检

`NETLINK_GET_STRICT_CHK` 套接字选项`NETLINK_ROUTE` 中启用严格的输入
检查。之所以需要它，是因为历史上内核不会验证它未处理的结构体的字段。这使得
后来不可能开始使用那些字段，而不冒那些错误地或未初始化它们的应用程序出现
回归的风险
`NETLINK_GET_STRICT_CHK` 声明应用程序正在正确初始化所有字段。它还选择验证
消息不包含尾随数据，并请求内核拒绝类型高于内核已知最大属性类型的属性
`NETLINK_GET_STRICT_CHK` 不在 `NETLINK_ROUTE` 之外使用
### 未知属

历史Netlink 忽略了所有未知属性。其想法是让应用程序不必去探查内核支什么。应用程序可以发出改变状态的请求，并检查请求的哪些部分“生效”了
对于新的 Generic Netlink family 以及选择严格检查的那些，情况已不再如此所执行的验证类型请参阅 enum netlink_validation
### 固定元数据与结构

Classic Netlink 在消息中大量使用固定格式的结构体。消息通常struct
nlmsghdr 之后带有一个具有大量字段的结构体。把具有多个成员的结构体放入
属性中、而不把每个成员拆成各自的属性，也是很常见的做法
这给验证和可扩展性带来了问题，因此对于新属性，不鼓励使用二进制结构体
### 请求类型


`NETLINK_ROUTE` 将请求分4 种类型：`NEW`、`DEL`、`GET` `SET`。每对象可以处理所有这些或其中部分请求（对象即 netdev、路由、地址、qdisc 等）请求类型由消息类型的最2 位定义，因此新对象的命令总是4 为步长分配
每个对象还会拥有自己的、由所有请求类型共享的固定元数据（例如 netdev 请求
使用 struct ifinfomsg，地址请求使用 struct ifaddrmsg，qdisc 请求使用
struct tcmsg）
尽管其他协议Generic Netlink 命令经常在它们的消息名中使用相同的动（`GET`、`SET`），但请求类型的概念并未得到更广泛的采用
### 通知回显


`NLM_F_ECHO` 请求将由该请求产生的通知排队到发起请求的套接字上。这有助发现该请求的影响
请注意，此功能并未被普遍实现
### 其他请求类型特定的标

Classic Netlink struct nlmsghdr nlmsg_flags 的高字节中为它的 `GET``NEW` `DEL` 请求定义了各种标志。由于请求类型尚未通用化，这些请求类型
特定的标志很少使用（并且对于新的 family 被视为已弃用）
对于 `GET` - `NLM_F_ROOT` `NLM_F_MATCH` 被合并为 `NLM_F_DUMP`，不单独
使用。`NLM_F_ATOMIC` 从未使用
对于 `DEL` - `NLM_F_NONREC` 仅被 nftables 使用，`NLM_F_BULK` 仅被 FDB 部分操作使用
用于 `NEW` 的标志在 classic Netlink 中最常用。遗憾的是，其含义并不十分清晰以下描述基于作者对意图的最佳猜测，而在实践中所family 都会以某种方式偏它。`NLM_F_REPLACE` 要求替换一个已存在的对象，如果不存在匹配的对象，操应当失败。`NLM_F_EXCL` 具有相反的语义，仅当对象已经存在时才成功`NLM_F_CREATE` 要求如果对象不存在就创建它，它可`NLM_F_REPLACE` `NLM_F_EXCL` 组合
```

   4.4BSD ADD		NLM_F_CREATE|NLM_F_EXCL
   4.4BSD CHANGE	NLM_F_REPLACE

   True CHANGE		NLM_F_CREATE|NLM_F_REPLACE
   Append		NLM_F_CREATE
   Check		NLM_F_EXCL

```
这似乎表明这些标志早于请求类型。`NLM_F_REPLACE` 在没`NLM_F_CREATE` 最初被用来代替 `SET` 命令。`NLM_F_EXCL` 在没`NLM_F_CREATE` 时用于检对象是否存在而不创建它，大概早于 `GET` 命令
`NLM_F_APPEND` 表示如果一个键可以关联多个对象（例如一条路由的多个下一对象），新对象应当被添加到列表中，而不是替换整个列表
## uAPI 参