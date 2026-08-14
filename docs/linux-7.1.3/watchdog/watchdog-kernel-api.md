## Linux WatchDog Timer 驱动核心内核 API


最后审阅：2013-02-12

Wim Van Sebroeck <wim@iguana.be>

### 简介


本文档并不描述什么是看门狗定时器（WDT）驱动或设备，也不描述用户空间可用于与
看门狗定时器通信的 API。如果你想知道这些，请阅读以下文件：
Documentation/watchdog/watchdog-api.rst 。

那么本文档描述什么？它描述了希望使用看门狗定时器驱动核心框架的看门狗定时器
驱动所能使用的 API。该框架提供了所有面向用户空间的接口，因此同样的代码无需
每次都重复编写。这也意味着看门狗定时器驱动只需要提供控制看门狗定时器（WDT）
的不同例程（操作）。

### API


每个希望使用看门狗定时器驱动核心的看门狗定时器驱动都必须 #include
<linux/watchdog.h>（在编写看门狗设备驱动时无论如何你都得这么做）。该头文件
包含如下
```

	extern int watchdog_register_device(struct watchdog_device *);
	extern void watchdog_unregister_device(struct watchdog_device *);

```
watchdog_register_device 例程注册一个看门狗定时器设备。该例程的参数是一个指向
watchdog_device 结构的指针。该例程成功时返回零，失败时返回负的 errno 码。

watchdog_unregister_device 例程注销一个已注册的看门狗定时器设备。该例程的
参数是已注册的 watchdog_device 结构的指针。

看门狗子系统包含一个注册延迟机制，允许你在启动过程中尽早地注册一个看门狗。

```

  struct watchdog_device {
	int id;
	struct device *parent;
	const struct attribute_group **groups;
	const struct watchdog_info *info;
	const struct watchdog_ops *ops;
	const struct watchdog_governor *gov;
	unsigned int bootstatus;
	unsigned int timeout;
	unsigned int pretimeout;
	unsigned int min_timeout;
	unsigned int max_timeout;
	unsigned int min_hw_heartbeat_ms;
	unsigned int max_hw_heartbeat_ms;
	struct notifier_block reboot_nb;
	struct notifier_block restart_nb;
	void *driver_data;
	struct watchdog_core_data *wd_data;
	unsigned long status;
	struct list_head deferred;
  };

```
它包含以下字段：

- id：由 watchdog_register_device 设置，id 0 是特殊的。它同时拥有 /dev/watchdog0
  cdev（动态主设备号，次设备号 0）以及旧的 /dev/watchdog miscdev。调用
  watchdog_register_device 时会自动设置该 id。
- parent：在调用 watchdog_register_device 之前，将其设置为父设备（或 NULL）。
- groups：创建看门狗设备时要创建的 sysfs 属性组列表。
- info：一个指向 watchdog_info 结构的指针。该结构给出关于看门狗定时器自身的一些
  附加信息（如其唯一名称）。
- ops：一个指向看门狗所支持的操作列表的指针。
- gov：一个指向已分配的看门狗设备 pretimeout 管理器（governor）的指针，或 NULL。
- timeout：看门狗定时器的超时值（以秒为单位）。如果设置了 WDOG_ACTIVE，这是
  在用户空间不发送心跳请求的情况下系统将会重启的时间。
- pretimeout：看门狗定时器的 pretimeout 值（以秒为单位）。
- min_timeout：看门狗定时器的最小超时值（以秒为单位）。若设置，则为 'timeout'
  可配置的最小值。
- max_timeout：看门狗定时器的最大超时值（以秒为单位），从用户空间可见。若设置，
  则为 'timeout' 可配置的最大值。当 max_hw_heartbeat_ms 非零时不使用。
- min_hw_heartbeat_ms：心跳之间最小时间间隔的硬件限制，以毫秒为单位。该值通常为
  0；只有当硬件无法容忍更短的心跳间隔时才应提供。
- max_hw_heartbeat_ms：最大硬件心跳，以毫秒为单位。若设置，当 'timeout' 大于
  max_hw_heartbeat_ms 时，基础设施会向看门狗驱动发送心跳，除非设置了 WDOG_ACTIVE
  且用户空间至少在 'timeout' 秒内未能发送一次心跳。如果驱动没有实现 stop 函数，
  则必须设置 max_hw_heartbeat_ms。
- reboot_nb：为重启通知注册的 notifier 块，仅供内部使用。如果驱动调用
  watchdog_stop_on_reboot，看门狗核心会在收到此类通知时停止看门狗。
- restart_nb：为机器重启注册的 notifier 块，仅供内部使用。如果看门狗能够重启机器，
  它应定义 ops->restart。优先级可通过 watchdog_set_restart_priority 更改。
- bootstatus：启动后设备的状态（以看门狗 WDIOF_* 状态位报告）。
- driver_data：指向看门狗设备驱动私有数据的指针。该数据应仅通过 watchdog_set_drvdata
  与 watchdog_get_drvdata 例程访问。
- wd_data：指向看门狗核心内部数据的指针。
- status：该字段包含一些状态位，提供关于设备状态的额外信息（例如：看门狗定时器
  是否正在运行/激活，或 nowayout 位是否已设置）。
- deferred：wtd_deferred_reg_list 中的一项，用于注册提前初始化的看门狗。

```

  struct watchdog_ops {
	struct module *owner;
	/* mandatory operations */
	int (*start)(struct watchdog_device *);
	/* optional operations */
	int (*stop)(struct watchdog_device *);
	int (*ping)(struct watchdog_device *);
	unsigned int (*status)(struct watchdog_device *);
	int (*set_timeout)(struct watchdog_device *, unsigned int);
	int (*set_pretimeout)(struct watchdog_device *, unsigned int);
	unsigned int (*get_timeleft)(struct watchdog_device *);
	int (*restart)(struct watchdog_device *);
	long (*ioctl)(struct watchdog_device *, unsigned int, unsigned long);
  };

```
首先定义看门狗定时器驱动操作的模块所有者非常重要。该模块所有者用于在看门狗
激活时锁定模块（这是为了避免在卸载模块而 /dev/watchdog 仍打开时造成系统崩溃）。

有些操作是强制的，有些是可选的。强制的操作是：

- start：这是一个指向启动看门狗定时器设备例程的指针。该例程需要以看门狗定时器
  设备结构为参数。成功时返回零，失败时返回负的 errno 码。

并非所有看门狗定时器硬件都支持相同的功能。这就是为什么所有其他例程/操作都是
可选的。它们只需要在受支持时才需要提供。这些可选的例程/操作是：

- stop：通过该例程停止看门狗定时器设备。该例程需要以看门狗定时器设备结构为
  参数。成功时返回零，失败时返回负的 errno 码。有些看门狗定时器硬件只能启动而
  不能停止。支持此类硬件的驱动无需实现 stop 例程。如果驱动没有 stop 函数，看门狗
  核心会设置 WDOG_HW_RUNNING，并在看门狗设备关闭后开始调用驱动的 keepalive ping
  函数。如果看门狗驱动没有实现 stop 函数，它必须设置 max_hw_heartbeat_ms。
- ping：这是向看门狗定时器硬件发送 keepalive ping 的例程。该例程需要以看门狗
  定时器设备结构为参数。成功时返回零，失败时返回负的 errno 码。大多数不支持
  将其作为独立功能的硬件会使用 start 函数来重启看门狗定时器硬件。而这正是看门狗
  定时器驱动核心所做的：为了向看门狗定时器硬件发送 keepalive ping，它要么使用
  ping 操作（可用时），要么使用 start 操作（ping 操作不可用时）。（注意：WDIOC_KEEPALIVE
  ioctl 调用仅在看门狗 info 结构的 option 字段中设置了 WDIOF_KEEPALIVEPING 位时
  才会生效）。
- status：该例程检查看门狗定时器设备的状态。设备状态以看门狗 WDIOF_* 状态标志/
  位报告。WDIOF_MAGICCLOSE 与 WDIOF_KEEPALIVEPING 由看门狗核心报告；无需从驱动
  报告这些位。此外，如果驱动未提供 status 函数，看门狗核心会报告 struct
  watchdog_device 的 bootstatus 变量中提供的状态位。
- set_timeout：该例程检查并更改看门狗定时器设备的超时。成功时返回 0，“参数超出
  范围”返回 -EINVAL，“无法将值写入看门狗”返回 -EIO。成功时，该例程应将
  watchdog_device 的超时值设置为实际达到的超时值（可能与请求值不同，因为看门狗
  不一定具有 1 秒的分辨率）。实现了 max_hw_heartbeat_ms 的驱动会将硬件看门狗心跳
  设置为 timeout 与 max_hw_heartbeat_ms 中的较小者。这些驱动将 watchdog_device 的
  超时值设置为请求的超时值（如果它大于 max_hw_heartbeat_ms），或者设置为实际达到
  的超时值。（注意：需要在看门狗 info 结构的 options 字段中设置 WDIOF_SETTIMEOUT）。
  如果看门狗驱动除了设置 watchdog_device.timeout 之外无需执行任何动作，则可以省略
  此回调。如果未提供 set_timeout 但设置了 WDIOF_SETTIMEOUT，看门狗基础设施会在
  内部将 watchdog_device 的超时值更新为请求值。如果使用了 pretimeout 特性
  （WDIOF_PRETIMEOUT），那么 set_timeout 还必须负责检查 pretimeout 是否仍然有效，
  并相应地设置定时器。这在核心中无法在无竞争的情况下完成，因此是驱动的职责。
- set_pretimeout：该例程检查并更改看门狗的 pretimeout 值。它是可选的，因为并非
  所有看门狗都支持 pretimeout 通知。该超时值并非绝对时间，而是距离实际超时发生
  之前的秒数。成功时返回 0，“参数超出范围”返回 -EINVAL，“无法将值写入看门狗”
  返回 -EIO。值 0 表示禁用 pretimeout 通知。（注意：需要在看门狗 info 结构的
  options 字段中设置 WDIOF_PRETIMEOUT）。如果看门狗驱动除了设置
  watchdog_device.pretimeout 之外无需执行任何动作，则可以省略此回调。这意味着如果
  未提供 set_pretimeout 但设置了 WDIOF_PRETIMEOUT，看门狗基础设施会在内部将
  watchdog_device 的 pretimeout 值更新为请求值。
- get_timeleft：该例程返回重启之前剩余的时间。
- restart：该例程重启机器。成功时返回 0，失败时返回负的 errno 码。
- ioctl：如果存在此例程，那么它会在我们自己的内部 ioctl 调用处理之前首先被调用。
  当命令不受支持时，该例程应返回 -ENOIOCTLCMD。传递给 ioctl 调用的参数是：
  watchdog_device、cmd 与 arg。

状态位（最好）应使用 set_bit 与 clear_bit 之类的位操作来设置。所定义的状态位
如下：

- WDOG_ACTIVE：该状态位从用户角度指示看门狗定时器设备是否处于活动状态。在此标志
  被设置期间，用户空间应向驱动发送心跳请求。
- WDOG_NO_WAY_OUT：该位存储看门狗的 nowayout 设置。如果设置了该位，则看门狗定时器
  将无法停止。
- WDOG_HW_RUNNING：如果硬件看门狗正在运行，由看门狗驱动设置。如果看门狗定时器硬件
  无法停止，则必须设置该位。如果看门狗定时器在启动后、看门狗设备被打开之前就在
  运行，也可以设置该位。如果设置，看门狗基础设施会在 WDOG_ACTIVE 未设置时向看门狗
  硬件发送 keepalive。注意：当你带着该位被设置来注册看门狗定时器设备时，打开
  /dev/watchdog 将跳过 start 操作，而是发送一个 keepalive 请求。

  要设置 WDOG_NO_WAY_OUT 状态位（在注册你的看门狗定时器设备之前），你可以：

  - 在你的 watchdog_device 结构中静态设置

	.status = WATCHDOG_NOWAYOUT_INIT_STATUS,

    （这会将值设置为与 CONFIG_WATCHDOG_NOWAYOUT 相同）或
```

	static inline void watchdog_set_nowayout(struct watchdog_device *wdd,
						 int nowayout)

```
注意：
   看门狗定时器驱动核心支持 magic close 特性与 nowayout 特性。要使用 magic close
   特性，你必须在看门狗 info 结构的 options 字段中设置 WDIOF_MAGICCLOSE 位。

nowayout 特性会覆盖 magic close 特性。

要获取或设置驱动特定数据，应使用以下两个辅助函数
```

  static inline void watchdog_set_drvdata(struct watchdog_device *wdd,
					  void *data)
  static inline void *watchdog_get_drvdata(struct watchdog_device *wdd)

```
watchdog_set_drvdata 函数允许你添加驱动特定数据。该函数的参数是你要向其添加驱动
特定数据的看门狗设备，以及指向数据本身的指针。

watchdog_get_drvdata 函数允许你取回驱动特定数据。该函数的参数是你要从中取回数据
的看门狗设备。该函数返回指向驱动特定数据的指针。

```

  extern int watchdog_init_timeout(struct watchdog_device *wdd,
                                   unsigned int timeout_parm,
                                   const struct device *dev);

```
watchdog_init_timeout 函数允许你使用模块 timeout 参数，或从设备树获取 timeout-sec
属性（如果模块 timeout 参数无效）来初始化 timeout 字段。最佳实践是先将默认超时值
设为 watchdog_device 中的超时值，然后使用此函数设置用户“偏好”的超时值。该例程
成功时返回零，失败时返回负的 errno 码。

```

  static inline void watchdog_stop_on_reboot(struct watchdog_device *wdd);

```
要在注销看门狗时禁用它，用户必须调用以下辅助函数。注意，只有当 nowayout 标志
未设置时，这才会停止看门狗。

```

  static inline void watchdog_stop_on_unregister(struct watchdog_device *wdd);

```
要更改重启处理程序的优先级，应使用以下辅助函数
```

  void watchdog_set_restart_priority(struct watchdog_device *wdd, int priority);

```
用户应遵循以下设置优先级的准则：

- 0：应在最后的手段中调用，重启能力有限
- 128：默认重启处理程序，在预期没有其他处理程序可用，和/或重启足以重启整个系统
  时使用
- 255：最高优先级，将抢占所有其他重启处理程序

```

  void watchdog_notify_pretimeout(struct watchdog_device *wdd)

```
该函数可以在中断上下文中调用。如果启用了看门狗 pretimeout 管理器框架（kbuild
CONFIG_WATCHDOG_PRETIMEOUT_GOV 符号），则由预先分配给看门狗设备的、预先配置好的
pretimeout 管理器采取行动。如果未启用看门狗 pretimeout 管理器框架，
watchdog_notify_pretimeout() 会向内核日志缓冲区打印一条通知消息。

要设置看门狗最后一次已知的硬件 keepalive 时间，使用以下函数
```

  int watchdog_set_last_hw_keepalive(struct watchdog_device *wdd,
                                     unsigned int last_ping_ms)

```
该函数必须在看门狗注册之后立即调用。它将最后一次已知的硬件心跳设置为在当前时间
之前 last_ping_ms 毫秒时发生。只有当 probe 被调用时看门狗已经在运行，且看门狗只能
在自上次 ping 起经过 min_hw_heartbeat_ms 时间之后才能被 ping 时，才需要调用此函数。
