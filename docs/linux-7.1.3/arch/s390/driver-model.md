## S/390 驱动模型接口


### 1. CCW 设备


所有可以通过 ccw 寻址的设备都被称为 “CCW 设备”——即使它们实际上并非由 ccw 驱动。

所有 ccw 设备都通过一个子通道（subchannel）访问，这反映在

```

  devices/
     - system/
     - css0/
	   - 0.0.0000/0.0.0815/
	   - 0.0.0001/0.0.4711/
	   - 0.0.0002/
	   - 0.1.0000/0.1.1234/
	   ...
	   - defunct/

```
在本例中，设备 0815 通过子通道集 0 中的子通道 0 访问，设备 4711 通过子通道集 0 中的子通道 1 访问，而子通道 2 是一个非 I/O 子通道。设备 1234 通过子通道集 1 中的子通道 0 访问。

名为 “defunct” 的子通道不代表系统上任何真实的子通道；它是一个伪子通道，当断开连接的 ccw 设备被另一个在其原子通道上变为可操作的 ccw 设备挤占时，这些断开连接的 ccw 设备会被移到那里。如果那些 ccw 设备在该子通道上再次变为可操作，它们会被再次移回合适的子通道。

您应该通过其 bus id（例如 0.0.4711）来寻址一个 ccw 设备；该设备可以在 bus/ccw/devices/ 下找到。

所有 ccw 设备都通过 sysfs 导出一些数据。

cutype:
	控制部件（control unit）类型 / 型号。

devtype:
	设备类型 / 型号（如果适用）。

availability:
	      可以是 “good” 或 “boxed”；对于断开连接的设备则是
	      “no path” 或 “no device”。

online:
	   一个用于将设备上线（online）和下线（offline）的接口。
	   在设备断开连接的特殊情况下（见 1.2 下的 notify 函数），
	   向 online 写入 0 将强制删除该设备。

设备驱动可以添加条目来导出每设备的数据和接口。

还有一些数据是按子通道导出的（见 bus/css/devices/）：

chpids:
	设备通过哪些 chpid 连接。

pimpampom:
	路径已安装、路径可用和路径可操作的掩码。

可能还有额外的数据，例如针对块设备。


### 1.1 启动一个 ccw 设备


这分几个步骤完成。

a. 每个驱动可以提供一个或多个参数接口，用于指定参数。这些接口也由驱动负责。
b. 在执行 a. 之后，如有必要，最终通过 “online” 接口启动设备。


### 1.2 为 ccw 设备编写驱动


基本的 struct ccw_device 和 struct ccw_driver 数据结构可以在

```

  struct ccw_device {
	spinlock_t *ccwlock;
	struct ccw_device_private *private;
	struct ccw_device_id id;

	struct ccw_driver *drv;
	struct device dev;
	int online;

	void (*handler) (struct ccw_device *dev, unsigned long intparm,
			 struct irb *irb);
  };

  struct ccw_driver {
	struct module *owner;
	struct ccw_device_id *ids;
	int (*probe) (struct ccw_device *);
	int (*remove) (struct ccw_device *);
	int (*set_online) (struct ccw_device *);
	int (*set_offline) (struct ccw_device *);
	int (*notify) (struct ccw_device *, int);
	struct device_driver driver;
	char *name;
  };

```
“private” 字段只包含内部 I/O 操作所需的数据，设备驱动不可访问。

每个驱动应在 MODULE_DEVICE_TABLE 中声明它感兴趣哪些 CU 类型/型号和/或设备类型/型号。此信息之后可以在

```

  struct ccw_device_id {
	__u16   match_flags;

	__u16   cu_type;
	__u16   dev_type;
	__u8    cu_model;
	__u8    dev_model;

	unsigned long driver_info;
  };

```
ccw_driver 中的函数应按如下方式使用：

probe:
	 设备层为每个该驱动感兴趣的设备调用此函数。驱动应只分配私有结构放入 dev->driver_data 并创建属性（如果需要）。同时，应在此处设置中断处理程序（见下文）。

```

  int (*probe) (struct ccw_device *cdev);

```
参数：
		cdev
   - 要 probe 的设备。


remove:
	 设备层在移除驱动、设备或模块时调用此函数。驱动应在此处执行清理。

```

  int (*remove) (struct ccw_device *cdev);

```
参数：
		cdev
   - 要移除的设备。


set_online:
	    公共 I/O 层在通过 “online” 属性激活设备时调用此函数。驱动应最终在此处设置和激活设备。

```

  int (*set_online) (struct ccw_device *);

```
参数：
		cdev
   - 要激活的设备。公共层已验证该设备尚未 online。


set_offline: 公共 I/O 层在通过 “online” 属性停用设备时调用此函数。驱动应关闭设备，但不释放其私有数据。

```

  int (*set_offline) (struct ccw_device *);

```
参数：
		cdev
   - 要停用的设备。公共层已验证该设备处于 online。


notify:
	公共 I/O 层在设备的某些状态改变时调用此函数。

	向驱动发出的信号有：

 - 在 online 状态下，设备分离（CIO_GONE）或最后一条路径消失
	  （CIO_NO_PATH）。驱动必须返回 !0 以保留设备；对于
	   返回码 0，设备将照常被删除（即使没有注册 notify 函数时也是如此）。如果驱动想要保留
	   设备，它会被移入断开连接状态。
 - 在断开连接状态下，设备再次可操作（CIO_OPER）。公共 I/O 层对设备号和
	   Device / CU 执行一些完整性检查，以合理确信它是否仍是同一设备。
	   如果不是，旧设备被移除并注册一个新设备。通过 notify 函数的返回码，
	   设备驱动表明它是否想要回该设备：!0 表示保留，0 表示将设备移除并重新注册。

```

  int (*notify) (struct ccw_device *, int);

```
参数：
		cdev
   - 状态改变的设备。

		event
   - 发生的事件。可以是 CIO_GONE、
			  CIO_NO_PATH 或 CIO_OPER 之一。

struct ccw_device 的 handler 字段用于设置为该设备的中断处理程序。为了适应使用多个不同处理程序（例如多子通道设备）的驱动，这是 ccw_device 的成员而不是 ccw_driver 的成员。
handler 在 set_online() 处理期间、在调用驱动之前向公共层注册，并在 set_offline() 期间、在调用驱动之后注销。此外，在注册之后 / 注销之前，会执行路径分组（path grouping）或路径组的解散（如果适用）。

```

  void (*handler) (struct ccw_device *dev, unsigned long intparm, struct irb *irb);

```
参数：     dev     - 调用 handler 的设备
		intparm - 允许设备驱动识别中断所关联的 i/o，
			  或将中断识别为未请求的（unsolicited）。
		irb     - 包含累计状态的中断响应块（interruption response block）。

设备驱动从公共 ccw_device 层调用，并可以从 irb 参数检索有关中断的信息。


### 1.3 ccwgroup 设备


ccwgroup 机制设计用于处理由多个 ccw 设备组成的设备，例如 qeth 或 ctc。

ccw 驱动提供一个 “group” 属性。将 ccw 设备的 bus id 写入此属性会创建一个由这些 ccw 设备组成的 ccwgroup 设备（如果可能）。这个 ccwgroup 设备可以像普通的 ccw 设备一样上/下线。

每个 ccwgroup 设备还提供一个 “ungroup” 属性以再次销毁该设备（仅在下线时）。这是一个通用的 ccwgroup 机制（驱动不需要实现超出正常移除例程之外的任何东西）。

作为 ccwgroup 设备成员的 ccw 设备，在其 device 结构的 driver_data 中携带一个指向 ccwgroup 设备的指针。驱动不得触碰此字段——它应使用 ccwgroup 设备的 driver_data 来存放其私有数据。

要实现 ccwgroup 驱动，请参阅 include/asm/ccwgroup.h。请记住，大多数驱动都需要同时实现 ccwgroup 和 ccw 驱动。


### 2. 通道路径（Channel paths）


通道路径与子通道一样，出现在通道子系统根（css0）之下，被称为 “chp0.<chpid>”。它们没有驱动，也不属于任何总线。
请注意，与 2.4 中的 /proc/chpids 不同，通道路径对象只反映逻辑状态而不是物理状态，因为由于缺少机器支持，我们无法一致地跟踪后者（反正我们也不需要知道它）。

status
       - 可以是 “online” 或 “offline”。
	 写入 “on” 或 “off” 会将 chpid 逻辑地上/下线。
	 向一个已上线的 chpid 写入 “on” 会触发对其连接的所有设备的路径重新探测。这可用于强制内核复用一个用户知道已上线、但机器尚未为其创建机器检查的通道路径。

type
       - 通道路径的物理类型。

shared
       - 通道路径是否共享。

cmg
       - 通道测量组（channel measurement group）。

### 3. 系统设备


### 3.1 xpram


xpram 作为 “xpram” 出现在 devices/system/ 下。

### 3.2 cpus


对于每个 cpu，在 devices/system/cpu/ 下创建一个目录。每个 cpu 有一个属性 “online”，其值可以是 0 或 1。
