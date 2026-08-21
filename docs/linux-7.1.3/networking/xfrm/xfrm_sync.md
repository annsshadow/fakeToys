

## XFRM 同步


该同步补丁工作基Krisztian <hidden@balabit.hu> 及其他人提供的初始补丁，
以及 Jamal <hadi@cyberus.ca> 提供的额外补丁

同步的最终目标是能够插入属性并生成事件，从而可以将 SA 安全地从一台机
迁移到另一台机器以实现高可用（HA）目的
其思路是同SA，这样接管机器在能够访问SA 时，可以尽可能精确地处理它

我们已经具备生成 SA add/del/upd 事件的能力
这些补丁增加了同步能力，并提供了精确的生存期字节计数（以确保 SA 正确衰减
以及重放计数器，从而在故障切换时尽可能减少损失，避免重放攻击
这样，备份机就能保持与主用成员尽可能接近的最新状态

由于上述各项会随着 SA 收到的每一个数据包而变化，因此有可能产生大量事件
出于这个原因，我们还加入了一种类nagle 的算法来限制事件数量。即我们
设置阈值，例如“当重放序列号阈值达到或已过10 秒时通知我”
这些阈值可通过 sysctl 进行系统级设置，也可以按 SA 更新

需要同步的项包括：
- 生存期字节计数器
注意：如果你假设故障切换机器是预先已知的，则生存期时间限制并不重要，因为
时间倒计时的衰减并不是由数据包到达驱动的
- 入向与出向的重放序列

### 1) 消息结构


nlmsghdr:aevent_id:optional-TLVs銆。

netlink 消息类型包括

XFRM_MSG_NEWAE 涓?XFRM_MSG_GETAE銆。

XFRM_MSG_GETAE 不带 TLV

XFRM_MSG_NEWAE 至少会包含两TLV（如下文进一步讨论）

```

   struct xfrm_aevent_id {
	     struct xfrm_usersa_id           sa_id;
	     xfrm_address_t                  saddr;
	     __u32                           flags;
	     __u32                           reqid;
   };

```
唯一SA xfrm_usersa_id、reqid saddr 的组合来标识

flags 用于指示不同的含义。可能的

```

	XFRM_AE_RTHR=1, /* replay threshold*/
	XFRM_AE_RVAL=2, /* replay value */
	XFRM_AE_LVAL=4, /* lifetime value */
	XFRM_AE_ETHR=8, /* expiry timer threshold */
	XFRM_AE_CR=16, /* Event cause is replay update */
	XFRM_AE_CE=32, /* Event cause is timer expiry */
	XFRM_AE_CU=64, /* Event cause is policy update */

```
这些 flags 如何使用取决于消息的方向（kernel<->user）以及起因（配置、查询或事件）
下文在不同的消息中会加以说明

pid 会在 netlink 中被适当设置以识别方向（发往内核时为 0，从内核到用户空间时
pid = 创建该事件的进程 ID

程序需要订阅多播组 XFRMNLGRP_AEVENTS 才能收到这些事件的通知

### 2) TLV 反映不同的参


a) 字节值（XFRMA_LTIME_VAL

   TLV 携带自上次事件以来字节生存期的运当前计数器

b) 重放值（XFRMA_REPLAY_VAL

   TLV 携带自上次事件以来重放序列号的运当前计数器

c) 重放阈值（XFRMA_REPLAY_THRESH

   TLV 携带内核用于在重放序列号超出时触发事件的阈值

d) 过期定时器（XFRMA_ETIMER_THRESH

   这是一个以毫秒为单位的定时器值，用作限制事件速率nagle 值

### 3) 参数的默认配


默认情况下这些事件应当是关闭的，除非至少有一个监听器注册以监听多播组
XFRMNLGRP_AEVENTS銆。

安装 SA 的程序需要指定这两个阈值，但是，为了不改变诸如 racoon 之类
现有应用程序，我们也针对这些不同参数提供了默认阈值，以防它们未被指定

两个 sysctl/proc 项为

a) /proc/sys/net/core/sysctl_xfrm_aevent_etime

   用于100ms 为递增时间单位提供 XFRMA_ETIMER_THRESH 的默认值。默认是 10 秒）

b) /proc/sys/net/core/sysctl_xfrm_aevent_rseqth

   用于以递增的数据包计数提供 XFRMA_REPLAY_THRESH 参数的默认值。默认是两个数据包

### 4) 消息类型


a) XFRM_MSG_GETAE 由用户空--> 内核发出
   XFRM_MSG_GETAE 不携带任TLV

   响应是一XFRM_MSG_NEWAE，其格式取决XFRM_MSG_GETAE 所查询的内容

   响应始终带有 XFRMA_LTIME_VAL XFRMA_REPLAY_VAL TLV

     - 如果设置XFRM_AE_RTHR 标志，则也会取回 XFRMA_REPLAY_THRESH
     - 如果设置XFRM_AE_ETHR 标志，则也会取回 XFRMA_ETIMER_THRESH

b) XFRM_MSG_NEWAE 既可由用户空间发出以进行配置
   也可由内核发出以宣告事件或响XFRM_MSG_GETAE

   i) 用户 --> 内核，用于配置某个特定的 SA

      可以通过传递相应的 TLV 来更新任意值或阈值参数

      会向用户空间中的发送方回送响应，指示成功或失败

      在成功的情况下，还会额外向任何监听器发出一个带XFRM_MSG_NEWAE 的事件，iii) 所述

   ii) 内核 -> 用户方向，作为对 XFRM_MSG_GETAE 的响

       响应始终带有 XFRMA_LTIME_VAL XFRMA_REPLAY_VAL TLV

       如果 XFRM_MSG_GETAE 消息中显式请求，则会包含阈TLV

   iii) 内核 -> 用户，用于报告事件：如果有人使用 XFRM_MSG_NEWAE（如上文 #i 所述）
        为某SA 设置了任意值或阈值。在这种情况下会设置 XFRM_AE_CU 标志
        以告知用户该变更是由一次更新引起的
        该消息始终带XFRMA_LTIME_VAL XFRMA_REPLAY_VAL TLV

   iv) 内核 -> 用户，用于在重放阈值或超时超出时报告事件

在这种情况下会设XFRM_AE_CR（重放超出）XFRM_AE_CE（发生超时）之一
以告知用户发生了什么。注意这两个标志是互斥的
该消息始终带XFRMA_LTIME_VAL XFRMA_REPLAY_VAL TLV

### 5) 阈值设置的例外情况


如果你有一SA，其流量是突发式命中，以至于存在一段定时器阈值已过期但未看到
任何数据包的时期，那么会出现如下的异常行为：
定时器过期后的第一个数据包到达时会触发一个超时事件；即我们不会等待超
周期或数据包阈值达到。这样做是出于简单性与效率的考虑

-JHS
