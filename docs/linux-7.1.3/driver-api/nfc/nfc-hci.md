## NFC 核心HCI 后端


- Author: Eric Lapuyade, Samuel Ortiz
- Contact: eric.lapuyade@intel.com, samuel.ortiz@intel.com

### 概述


HCI 层实现了 ETSI TS 102 622 V10.2.0 规范的很大部分。它使得编写基于 HCI NFC 驱动变得容易HCI 层作NFC 核心的一个后端运行，实现一个抽象的 nfc 设备，并NFC 核心 API 转换HCI 命令
和事件
### HCI

HCI nfc 设备的身份向 NFC 核心注册。来自用户空间的请求通过 netlink 套接字路由到 NFC 核心然后HCI。从这一点开始，它们被转换为发往主机控制器（芯片）中 HCI 层的一系列 HCI 命令。命令可同步执行（发送上下文阻塞等待响应）或异步执行（响应从 HCI Rx 上下文返回）。HCI 事件也可以从主机
控制器接收。它们将被处理，并在需要时NFC 核心转发一个转换结果。有一些钩子让 HCI 驱动处理专有
事件或覆盖标准行为。HCI 使用 2 个执行上下文
- 一个用于执行命令：nfc_hci_msg_tx_work()。任何时刻只能有一个命令在执行- 一个用于分发接收到的事件和命令：nfc_hci_msg_rx_work()
### HCI 会话初始
会话初始化是一HCI 标准，但遗憾的是必须支持专有门（gate）。这就是为什么驱动将传递一个必须作会话一部分的专有门列表。HCI 将确保在 hci 设备建立时所有这些门都有管道连接。如果芯片支持预打开的门
和伪静态管道，驱动可以将该信息传递给 HCI 核心
### HCI 门与管道

一个门定义了可以找到某种服务的“端口”。为了访问一项服务，必须创建一个到该门的管道并打开它。在实现中，管道完全被隐藏。公API 只知道门。这与驱动需要向专有门发送命令而无需知道连接到它的管的需求是一致的
### 驱动接口


驱动通常分两部分编写：物理链路管理和 HCI 管理。这使得维护一个可以通过各种 phy（i2c、spi 等）连接芯片的驱动更容易
### HCI 管理

驱动通常会向 HCI 注册自己，并提供以下内容
```

  struct nfc_hci_ops {
	int (*open)(struct nfc_hci_dev *hdev);
	void (*close)(struct nfc_hci_dev *hdev);
	int (*hci_ready) (struct nfc_hci_dev *hdev);
	int (*xmit) (struct nfc_hci_dev *hdev, struct sk_buff *skb);
	int (*start_poll) (struct nfc_hci_dev *hdev,
			   u32 im_protocols, u32 tm_protocols);
	int (*dep_link_up)(struct nfc_hci_dev *hdev, struct nfc_target *target,
			   u8 comm_mode, u8 *gb, size_t gb_len);
	int (*dep_link_down)(struct nfc_hci_dev *hdev);
	int (*target_from_gate) (struct nfc_hci_dev *hdev, u8 gate,
				 struct nfc_target *target);
	int (*complete_target_discovered) (struct nfc_hci_dev *hdev, u8 gate,
					   struct nfc_target *target);
	int (*im_transceive) (struct nfc_hci_dev *hdev,
			      struct nfc_target *target, struct sk_buff *skb,
			      data_exchange_cb_t cb, void *cb_context);
	int (*tm_send)(struct nfc_hci_dev *hdev, struct sk_buff *skb);
	int (*check_presence)(struct nfc_hci_dev *hdev,
			      struct nfc_target *target);
	int (*event_received)(struct nfc_hci_dev *hdev, u8 gate, u8 event,
			      struct sk_buff *skb);
  };

```

- open() close() 应当打开和关闭硬件- hci_ready() 是一个可选的入口点，hci 会话建立后立即调用。驱动可以使用它来进行必须使HCI
  命令完成的额外初始化- xmit() 应当简单地向物理链路写入一帧- start_poll() 是一个可选的入口点，应当将硬件设置为轮询模式。仅当硬件使用专有门或与 HCI 标准
  略有不同的机制时才必须实现- dep_link_up() 在检测到 p2p 目标后被调用，以使用需要回传给 nfc 核心的硬件参数完p2p 连接设置- dep_link_down() 被调用以断开 p2p 链路- target_from_gate() 是一个可选的入口点，用于返回与专有门对应nfc 协议- complete_target_discovered() 是一个可选的入口点，让驱动执行自动激活已发现目标所需的额外专  处理- im_transceive() 如果向标签发送数据需要专HCI 命令，则必须由驱动实现。某些标签类型需要自定义命令  其他可以使用标准 HCI 命令写入。驱动可以检查标签类型，要么进行专有处理，要么返1 以请求标准处理  数据交换命令本身必须异步发送- tm_send() p2p 连接的情况下被调用以发送数据- check_presence() 是一个可选的入口点，核心会定期调用它来检查已激活的标签是否仍在场。如果未实现  核心将无法向用户空间推tag_lost 事件- event_received() 被调用以处理来自芯片的事件。驱动可以处理该事件，或返回 1 HCI 尝试标准处理
rx 路径上，驱动负责使用 nfc_hci_recv_frame() 将传入的 HCP 帧推送给 HCI。HCI 将负责重新聚和处理。这必须在一个可以休眠的上下文中完成
### PHY 管理

```

  struct nfc_phy_ops {
	int (*write)(void *dev_id, struct sk_buff *skb);
	int (*enable)(void *dev_id);
	void (*disable)(void *dev_id);
  };

```

enable():
	打开 phy（上电），使其准备好传输数据disable():
	关闭 phywrite():
	向芯片发送一个数据帧。注意，为了llc 等更高层能够存储该帧以便重发，此函数不得改变 skb	它也不得返回正数结果（成功返0，失败返回负数）
来自芯片的数据应直接发送到 nfc_hci_recv_frame()
### LLC

CPU 与芯片之间的通信通常需要某种链路层协议。这些协议被隔离为由 HCI 层管理的模块。目前有两个模块nop（原始传输）shdlc
```

  struct nfc_llc_ops {
	void *(*init) (struct nfc_hci_dev *hdev, xmit_to_drv_t xmit_to_drv,
		       rcv_to_hci_t rcv_to_hci, int tx_headroom,
		       int tx_tailroom, int *rx_headroom, int *rx_tailroom,
		       llc_failure_t llc_failure);
	void (*deinit) (struct nfc_llc *llc);
	int (*start) (struct nfc_llc *llc);
	int (*stop) (struct nfc_llc *llc);
	void (*rcv_from_drv) (struct nfc_llc *llc, struct sk_buff *skb);
	int (*xmit_from_hci) (struct nfc_llc *llc, struct sk_buff *skb);
  };

```

init():
	分配并初始化你的私有存储deinit():
	清理start():
	建立逻辑连接stop():
	终止逻辑连接rcv_from_drv():
	处理来自芯片、发往 HCI 的数据xmit_from_hci():
	处理HCI 发送、发往芯片的数据
llc 必须在使用前注册nfc。通过以下方式完成
```

	nfc_llc_register(const char *name, const struct nfc_llc_ops *ops);

```

再次注意，llc 不处理物理链路。因此，对于任何给定的芯片驱动，很容易将任何物理链路与任llc 混合
### 包含的驱

包含一个基HCI NXP PN544 驱动，通过 I2C 总线连接，并使用 shdlc
### 执行上下
执行上下文如下：
- IRQ 处理程序（IRQH）：
  快速，不能休眠。将传入帧发送到 HCI，在那里它们被传递给当前llc。在使用 shdlc 的情况下，该  被排shdlc rx 队列
- SHDLC 状态机工作线程（SMW
  仅在使用 llc_shdlc 时：处理 shdlc rx tx 队列
  分发 HCI 命令响应
- HCI Tx 命令工作线程（MSGTXWQ
  串行HCI 命令的执行
  在响应超时时完成执行
- HCI Rx 工作线程（MSGRXWQ
  分发传入HCI 命令或事件
- 来自用户空间调用的系统调用上下文（SYSCALL
  HCI 中从 NFC 核心调用的任何入口点
### 执行 HCI 命令的工作流（使shdlc
执行一HCI 命令可以很容易地通过以下方式同步执行
```

  int nfc_hci_send_cmd (struct nfc_hci_dev *hdev, u8 gate, u8 cmd,
			const u8 *param, size_t param_len, struct sk_buff **skb)

```

API 必须从一个可以休眠的上下文中调用。大多数情况下，这将是系统调用上下文。skb 将返回在响应接收到的结果
在内部，执行是异步的。所以此 API 所做的只是HCI 命令入队，在栈上建立一个本地等待队列，wait_event() 等待完成。该等待不可中断，因为无论如何都保证命令会在某个较短的超时后完成
MSGTXWQ 上下文随后被调度并调nfc_hci_msg_tx_work()。此函数将出队下一个挂起的命令，并将其 HCP
分片发送到恰好shdlc 的下一层。然后它将启动一个定时器，以便在没有响应到达时以超时错误完成命令
SMW 上下文被调度并调nfc_shdlc_sm_work()。此函数处理 shdlc 帧的收发。它使用驱动 xmit 发送帧并从驱动 IRQ 处理程序填充skb 队列中接收传入帧。SHDLC I（信息）帧的有效负载HCP 分片。它被聚合以形成完整HCI 帧，可以是响应、命令或事件
HCI 响应从此上下文立即分发以解除等待中的命令执行。响应处理涉及调用由 nfc_hci_msg_tx_work() 发送命令时提供的完成回调。完成回调随后唤醒系统调用上下文
```

  static int nfc_hci_execute_cmd_async(struct nfc_hci_dev *hdev, u8 pipe, u8 cmd,
				       const u8 *param, size_t param_len,
				       data_exchange_cb_t cb, void *cb_context)

```

工作流相同，只是 API 调用立即返回，并且回调将SMW 上下文带上结果被调用
### 接收 HCI 事件或命令的工作
HCI 命令或事件不SMW 上下文分发。相反，它们被排HCI rx_queue，并将从 HCI rx 工作线程上下（MSGRXWQ）分发。这样做是为了允cmd 或事件处理程序也执行其他命令（例如，处理来自 PN544 NFC_HCI_EVT_TARGET_DISCOVERED 事件需要向 reader A 门发ANY_GET_PARAMETER 以获取关于已发现
目标的信息）
通常，此类事件将MSGRXWQ 上下文传播到 NFC 核心
### 错误管理

NFC 核心请求执行同步发生的错误，简单地作为请求的执行结果返回。这些很容易处理
异步发生的错误（例如，在后台协议处理线程中）必须被报告，以便上层不会在下面出了问题的情况下仍蒙在
鼓里，并知道预期的事件很可能永远不会发生。这些错误的处理如下
- 驱动（pn544）未能递送传入帧：它存储错误，使得任何后续对驱动的调用都会导致此错误。然后它调用
  标准nfc_shdlc_recv_frame() 并传NULL 参数，以向更上层报告问题。shdlc 存储一EREMOTEIO
  粘滞状态，这将依次触发 SMW 向上报告
- SMW 本质上是一个处理传入和传出 shdlc 帧的后台线程。此线程还会检shdlc 粘滞状态，并在发现由于
  shdlc 或其下层中发生的不可恢复错误而无法再运行时，HCI 报告
- HCI：如果发生内HCI 错误（帧丢失），HCI 从下层收到错误，HCI 要么以该错误完成当前正在执行  命令，要么在没有命令执行时直接通知 NFC 核心
- NFC 核心：当 NFC 核心从下层收到错误通知且轮询处于活动状态时，它将向用户空间发送一个带有空标签
  列表的标签发现事件，让用户空间知道轮询操作将永远无法检测到标签。如果轮询不活动且错误是粘滞的，
  下层将在下次调用时返回它