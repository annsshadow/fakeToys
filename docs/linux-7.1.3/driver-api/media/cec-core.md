
## CEC 内核支持


CEC 框架HDMI CEC 硬件提供了一个统一的内核接口。它旨在处理多种类型的硬件（接收器、发送器、USB 适配器）。该框架还提供了在内核驱动中做什么、以及在用户空间应用程序中处理什么的选项。此外，它将遥控器透传特性集成到了内核的遥控器框架中

### CEC 协议


CEC 协议使消费电子设备能够通过 HDMI 连接相互通信。该协议在通信中使用逻辑地址。逻辑地址与设备所提供的功能严格相关。充当通信枢纽的电视总是被分配地址 0。物理地址由设备之间的物理连接决定
此处描述CEC 框架CEC 2.0 规范保持同步。它HDMI 1.4 规范中有记载，新2.0 部分记录HDMI 2.0 规范中。但对于大多数特性而言，可免费获取HDMI 1.3a 规范已足够：

https://www.hdmi.org/spec/index


### CEC 适配器接

struct cec_adapter 表示 CEC 适配器硬件。它通过调用 cec_allocate_adapter() 创建，通过调用 cec_delete_adapter() 删除
   struct cec_adapter *cec_allocate_adapter(const struct cec_adap_ops *ops, \
					    void *priv, const char *name, \
					    u32 caps, u8 available_las);

   void cec_delete_adapter(struct cec_adapter *adap);

要创建一个适配器，你需要传入以下信息：

ops:
	CEC 框架调用、且你需要实现的适配器操作
priv:
	会被存储adap->priv 中，并可供适配器操作使用。使cec_get_drvdata(adap) 获取priv 指针
name:
	CEC 适配器的名称。注意：此名称会被复制
caps:
	CEC 适配器的能力。这些能力决定了硬件的能力，以及哪些部分由用户空间处理、哪些部分由内核空间处理。这些能力由 CEC_ADAP_G_CAPS 返回
available_las:
	该适配器能同时处理的逻辑地址数量。必须满1 <= available_las <= CEC_MAX_LOG_ADDRS
要获priv 指针，使用此辅助函数
	void *cec_get_drvdata(const struct cec_adapter *adap);

要注/dev/cecX 设备节点和遥控器设备（如果设置了 CEC_CAP_RC），你调用：

	int cec_register_adapter(struct cec_adapter *adap, \
				 struct device *parent);

其中 parent 是父设备
要注销设备，调用：

	void cec_unregister_adapter(struct cec_adapter *adap);

注意：如cec_register_adapter() 失败，则调用 cec_delete_adapter() 进行清理。但如果 cec_register_adapter() 成功，则只调cec_unregister_adapter() 清理，绝不要调用 cec_delete_adapter()。一旦该 /dev/cecX 设备的最后一个用户关闭了其文件句柄，注销函数将自动删除适配器

### 瀹炵幇搴曞眰 CEC 閫傞厤鍣。

以下底层适配器操作必须在你的驱动中实现：


	struct cec_adap_ops
	{
		/** Low-level callbacks **/
		int (*adap_enable)(struct cec_adapter *adap, bool enable);
		int (*adap_monitor_all_enable)(struct cec_adapter *adap, bool enable);
		int (*adap_monitor_pin_enable)(struct cec_adapter *adap, bool enable);
		int (*adap_log_addr)(struct cec_adapter *adap, u8 logical_addr);
		void (*adap_unconfigured)(struct cec_adapter *adap);
		int (*adap_transmit)(struct cec_adapter *adap, u8 attempts,
				      u32 signal_free_time, struct cec_msg *msg);
		void (*adap_nb_transmit_canceled)(struct cec_adapter *adap,
						  const struct cec_msg *msg);
		void (*adap_status)(struct cec_adapter *adap, struct seq_file *file);
		void (*adap_free)(struct cec_adapter *adap);

		/** Error injection callbacks **/
		...

		/** High-level callback **/
		...
	};

这些底层操作用于处理控制 CEC 适配器硬件的各个方面。它们都在持有互斥锁 adap->lock 的情况下被调用

```

	int (*adap_enable)(struct cec_adapter *adap, bool enable);

```
此回调启用或禁用 CEC 硬件。启CEC 硬件意味着将其上电到一个未声明任何逻辑地址的状态。如果设置了 CEC_CAP_NEEDS_HPD，物理地址将始终有效。如果未设置该能力，则物理地址可能CEC 硬件启用期间发生变化。CEC 驱动不应设置 CEC_CAP_NEEDS_HPD，除非硬件设计有此要求，因为这会使无法唤醒在待机模式下将 HPD 拉低的显示器。调cec_allocate_adapter() CEC 适配器的初始状态是禁用的
注意，如enable false，adap_enable 必须返回 0

```

	int (*adap_monitor_all_enable)(struct cec_adapter *adap, bool enable);

```
如果启用，则适配器应被置于一种也监视非发送给本机的消息的模式。并非所有硬件都支持此功能，且只有在设置CEC_CAP_MONITOR_ALL 能力时才会调用此函数。此回调是可选的（某些硬件可能始终处于“monitor all”模式）
注意，如enable false，adap_monitor_all_enable 必须返回 0

```

	int (*adap_monitor_pin_enable)(struct cec_adapter *adap, bool enable);

```
如果启用，则适配器应被置于一种也监视 CEC 引脚变化的模式。并非所有硬件都支持此功能，且只有在设置CEC_CAP_MONITOR_PIN 能力时才会调用此函数。此回调是可选的（某些硬件可能始终处于“monitor pin”模式）
注意，如enable false，adap_monitor_pin_enable 必须返回 0

```

	int (*adap_log_addr)(struct cec_adapter *adap, u8 logical_addr);

```
如果 logical_addr == CEC_LOG_ADDR_INVALID，则所有已编程的逻辑地址都应被擦除。否则应编程给定的逻辑地址。如果超过了可用逻辑地址的最大数量，则应返回 -ENXIO。一旦某个逻辑地址被编程，CEC 硬件就能接收发往该地址的定向消息
注意，如logical_addr CEC_LOG_ADDR_INVALID，adap_log_addr 必须返回 0

```

	void (*adap_unconfigured)(struct cec_adapter *adap);

```
适配器已取消配置。如果驱动在取消配置后必须采取特定操作，则可以通过此可选回调来完成

```

	int (*adap_transmit)(struct cec_adapter *adap, u8 attempts,
			     u32 signal_free_time, struct cec_msg *msg);

```
这会发送一条新消息。attempts 参数是建议的发送尝试次数
signal_free_time 是适配器在线路空闲时、尝试发送消息前应等待的数据位周期数。该值取决于本次发送是重试、来自新发起者的消息，还是同一发起者的新消息。大多数硬件会自动处理这一点，但在某些情况下需要此信息
CEC_FREE_TIME_TO_USEC 宏可用于signal_free_time 转换为微秒（一个数据位周期2.4 ms）

```

	void (*adap_nb_transmit_canceled)(struct cec_adapter *adap,
					  const struct cec_msg *msg);

```
此可选回调可用于获取序列号为 msg->sequence 的被取消非阻塞发送的结果。在以下情况下调用：发送被中止、发送超时（即硬件从未发出发送完成的信号），或者发送成功但等待预期回复时要么被中止要么超时

```

	void (*adap_status)(struct cec_adapter *adap, struct seq_file *file);

```
此可选回调可用于显示 CEC 硬件的状态。该状态可通过 debugfs 获取：cat /sys/kernel/debug/cec/cecX/status


```

	void (*adap_free)(struct cec_adapter *adap);

```
此可选回调可用于释放驱动可能已分配的任何资源。它cec_delete_adapter 调用

你的适配器驱动还必须在以下情况下（通常是中断驱动）通过调用框架来响应事件：

```

	void cec_transmit_done(struct cec_adapter *adap, u8 status,
			       u8 arb_lost_cnt,  u8 nack_cnt, u8 low_drive_cnt,
			       u8 error_cnt);

```
```

	void cec_transmit_attempt_done(struct cec_adapter *adap, u8 status);

```
   状态可以是以下之一
CEC_TX_STATUS_OK:
	发送成功
CEC_TX_STATUS_ARB_LOST:
	仲裁失败：另一CEC 发起者控制了 CEC 线路，你失去了仲裁
CEC_TX_STATUS_NACK:
	消息nack（对于定向消息）ack（对于广播消息）。需要重传
CEC_TX_STATUS_LOW_DRIVE:
	CEC 总线上检测到 low drive。这表明某个跟随者检测到总线上的错误并请求重传
CEC_TX_STATUS_ERROR:
	发生了某种未指定的错误：如果硬件无法区分，这可能ARB_LOST LOW_DRIVE 之一，或者是完全不同的情况。某些硬件只支持OK FAIL 作为发送结果，即无法区分不同的可能错误。在这种情况下，FAIL 映射CEC_TX_STATUS_NACK 而非 CEC_TX_STATUS_ERROR
CEC_TX_STATUS_MAX_RETRIES:
	尝试多次后仍无法发送消息。应仅由具有消息重试硬件支持的驱动设置。如果设置，框架会假定它无需再次尝试发送该消息，因为硬件已经这样做了
硬件必须能够区分 OK、NACK 和“其他情况”
\*_cnt 参数是所观察到的错误条件数量。如果没有可用信息，可以0。不支持硬件重试的驱动只需将与发送错误对应的计数器设1；如果硬件确实支持重试，则当硬件不提供发生了哪些错误以及发生次数的反馈时，将这些计数器设0，否则填入硬件报告的正确值
请注意，如果存在排队中待发送的消息，调用这些函数可能会立即开始一次新的发送。因此，在调用这些函*之前**，请确保硬件处于可以开始新发送的状态
cec_transmit_attempt_done() 函数是一个辅助函数，用于硬件从不重试的情况，因此发送总是只有单次尝试。它会接着调用 cec_transmit_done()，将对应状态的 count 参数填为 1。如果状态为 OK，则全部0
当接收到一CEC 消息时：

	void cec_received_msg(struct cec_adapter *adap, struct cec_msg *msg);

不言自明
### 实现中断处理程序


通常，CEC 硬件会提供中断，用于指示发送何时完成以及是否成功，并在接收CEC 消息时提供中断
CEC 驱动应始终先处理发送中断，再处理接收中断。框架期望在 cec_received_msg 调用之前看到 cec_transmit_done 调用，否则如果接收到的消息是对已发送消息的回复，框架可能会混淆
### 可选：实现错误注入支持


如果 CEC 适配器支持错误注入（Error Injection）功能，则可以通过错误注入回调将其暴露出来

	struct cec_adap_ops {
		/** Low-level callbacks **/
		...

		/** Error injection callbacks **/
		int (*error_inj_show)(struct cec_adapter *adap, struct seq_file *sf);
		bool (*error_inj_parse_line)(struct cec_adapter *adap, char *line);

		/** High-level CEC message callback **/
		...
	};

如果两个回调都被设置，则会在 debugfs 中出现一`error-inj` 文件。基本语法如下：

   前导空格/制表符会被忽略。如果下一个字符是 `#` 或到达了行尾，则整行被忽略。否则预期是一条命令
   此基本解析在 CEC 框架中完成。由驱动决定实现哪些命令。唯一的要求是必须实现不带任何参数`clear` 命令，并且它会移除所有当前的错误注入命令
   这确保你始终可以执行 `echo clear >error-inj` 来清除任何错误注入，而无需了解驱动特定命令的细节
   注意 `error-inj` 的输出应可作`error-inj` 的输入。因此这必须有效

	$ cat error-inj >einj.txt
	$ cat einj.txt >error-inj

第一个回调在读取此文件时被调用，它应显示
```

	int (*error_inj_show)(struct cec_adapter *adap, struct seq_file *sf);

```
   建议它以一个带有基本用法信息的注释块开头。成功时返回 0，否则返回错误```

	bool (*error_inj_parse_line)(struct cec_adapter *adap, char *line);

```
   `line` 参数指向命令的起始位置。任何前导空格或制表符都已被跳过。它只是一行（因此没有内嵌的换行符），并以 0 结尾。该回调可以自由修改缓冲区的内容。它仅对包含命令的行调用，因此对于空行或注释行永远不会调用此回调
   如果命令有效则返true，如果存在语法错误则返回 false
### 瀹炵幇楂樺眰 CEC 閫傞厤鍣?

底层操作驱动硬件，高层操作由 CEC 协议驱动。高层回调在未持adap->lock 互斥锁的情况下被调用。可用的高层回调如下

	struct cec_adap_ops {
		/** Low-level callbacks **/
		...

		/** Error injection callbacks **/
		...

		/** High-level CEC message callback **/
		void (*configured)(struct cec_adapter *adap);
		int (*received)(struct cec_adapter *adap, struct cec_msg *msg);
	};

```

	void (*configured)(struct cec_adapter *adap);

```
   适配器已完全配置，即所有逻辑地址都已成功声明。如果驱动在配置后必须采取特定操作，则可以通过此可选回调来完成

received() 回调允许驱动可选地处理一条新
```

	int (*received)(struct cec_adapter *adap, struct cec_msg *msg);

```
   如果驱动想要处理一CEC 消息，则可以实现此回调。如果它不想处理该消息，则应返回 -ENOMSG，否CEC 框架会假定它已处理该消息，并且不会再对其做任何处理

### CEC 框架函数


CEC 适配器驱动可以调用以CEC 框架函数
   int cec_transmit_msg(struct cec_adapter *adap, struct cec_msg *msg, \
			bool block);

   发送一CEC 消息。如block true，则等待消息被发送完毕，否则只将其入队并返回
   void cec_s_phys_addr(struct cec_adapter *adap, u16 phys_addr, bool block);

   更改物理地址。此函数会设adap->phys_addr，并在其发生变化时发送一个事件。如果已调用 cec_s_log_addrs() 且物理地址已变为有效，CEC 框架将开始声明逻辑地址。如block true，则此函数在该过程完成之前不会返回
   当物理地址被设为有效值时，CEC 适配器将被启用（参见 adap_enable 操作）。当它被设为 CEC_PHYS_ADDR_INVALID 时，CEC 适配器将被禁用。如果你将有效的物理地址更改为另一个有效的物理地址，则此函数会先将地址设为 CEC_PHYS_ADDR_INVALID，再启用新的物理地址
   void cec_s_phys_addr_from_edid(struct cec_adapter *adap, \
				  const struct edid *edid);

   一个辅助函数，edid 结构体中提取物理地址，并用该地址调用 cec_s_phys_addr()，或者如EDID 不包含物理地址edid NULL 指针，则使用 CEC_PHYS_ADDR_INVALID 调用
	int cec_s_log_addrs(struct cec_adapter *adap, \
			    struct cec_log_addrs *log_addrs, bool block);

   声明 CEC 逻辑地址。如果设置了 CEC_CAP_LOG_ADDRS，则绝不应调用。如block true，则等待逻辑地址被声明，否则只将其入队并返回。要取消配置所有逻辑地址，可log_addrs 设为 NULL，或log_addrs->num_log_addrs 设为 0 来调用此函数。取消配置时会忽block 参数。如果物理地址无效，此函数将直接返回。一旦物理地址变为有效，框架将尝试声明这些逻辑地址

### CEC 引脚框架


大多CEC 硬件基于完整CEC 消息工作：软件提供消息，硬件处理底层 CEC 协议。但有些硬件只驱CEC 引脚，软件必须处理底CEC 协议。CEC 引脚框架正是为处理此类设备而创建的
注意，由于接近实时的要求，永远无法保证其 100% 工作。该框架在内部使用高精度定时器（highres timers），但如果定时器晚触发超300 微秒，就可能出现错误结果。实际上它似乎相当可靠
这种底层实现的一个优势是它可以作为一种廉价的 CEC 分析仪使用，特别是在可以使用中断来检CEC 引脚从低到高（或反之）的跳变时

### CEC 通知器框

大多drm HDMI 实现都有集成CEC 实现，不需要通知器支持。但有些具有独立CEC 实现，它们拥有自己的驱动。这可能SoC 的一IP 块，或者是处理 CEC 引脚的完全独立的芯片。对于这些情况，drm 驱动可以安装一个通知器（notifier），并使用该通知器将物理地址的变化告CEC 驱动