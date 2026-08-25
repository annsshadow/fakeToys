## 设备驱动

参见 struct device_driver kerneldoc
#### 分配

设备驱动是静态分配的结构体。尽管系统中可能存在某个驱动所支持的多个设备，struct device_driver 代表的是整个驱动（而不是某个特定的设备实例）
#### 初始
驱动必须至少初始name bus 字段。它还应当初始化 devclass 字段（当它出现时），以便能够在内部获得正确的链接关系。它还应当尽可能多地初始化各个回调，不过每个回调都是可选的
#### 声明

如上所述，struct device_driver 对象是静态分配的。下面是一eepro100 驱动的声明示例。该声明仅是假设性的；它依赖于驱```

  static struct device_driver eepro100_driver = {
         .name		= "eepro100",
         .bus		= &pci_bus_type,

         .probe		= eepro100_probe,
         .remove		= eepro100_remove,
         .suspend		= eepro100_suspend,
         .resume		= eepro100_resume,
  };

```
大多数驱动无法被完全转换到新模型，因为它们所属的总线有一个总线特定的结构体，其中带有无法被泛化的总线特定字段
这方面最常见的例子是设备 ID 结构体。驱动通常会定义一个它所支持的设ID 数组。这些结构体的格式以及比较设ID 的语义完全是总线特定的。如果将它们定义为总线特定的实体，会牺牲类型安全，因此我们将总线特定的结构体保留下来
总线特定的驱动应当包含一个通用struct device_driver，方式如```

  struct pci_driver {
         const struct pci_device_id *id_table;
         struct device_driver	  driver;
  };

```
一个包含总线特定字段的定义看起来像这```

  static struct pci_driver eepro100_driver = {
         .id_table       = eepro100_pci_tbl,
         .driver	       = {
		.name		= "eepro100",
		.bus		= &pci_bus_type,
		.probe		= eepro100_probe,
		.remove		= eepro100_remove,
		.suspend	= eepro100_suspend,
		.resume		= eepro100_resume,
         },
  };

```
有些人可能会觉得内嵌结构体初始化的语法有些别扭，甚至有点难看。迄今为止，这是我们发现的最能实现我们目标的做法…
#### 注册

```

  int driver_register(struct device_driver *drv);

```
驱动在启动时注册该结构体。对于没有总线特定字段（即没有总线特定驱动结构体）的驱动，它们会使driver_register 并传入指向其 struct device_driver 对象的指针
然而，大多数驱动都会有一个总线特定的结构体，并且需要使用类pci_driver_register 的函数向总线注册
驱动尽早注册其驱动结构体这一点很重要。向核心注册时会初始struct device_driver 对象中的若干字段，包括引用计数和锁。这些字段在任何时候都被假定为有效，并可能被设备模型核心或总线驱动使用
#### 过渡阶段的总线驱动

通过定义包装（wrapper）函数，可以更容易地过渡到新模型。驱动可以完全忽略通用结构体，而让总线包装函数来填写这些字段。对于回调，总线可以定义通用的回调，将调用转发给驱动的总线特定回调
该方案仅打算作为临时措施。为了获得驱动中class 信息，无论如何都必须修改驱动。由于将驱动转换到新模型应当能减少一些基础设施的复杂度和代码量，因此建议在添加 class 信息时一并完成转换
#### 访问

对象一旦注册，就可以访问以下通用字段
```

  int driver_for_each_dev(struct device_driver *drv, void *data,
			  int (*callback)(struct device *dev, void *data));

```
devices 字段是一个列表，包含已绑定到该驱动的所有设备。LDM（Linux 设备模型）核心提供了一个辅助函数来操作一个驱动所控制的所有设备。该辅助函数在每次访问节点时会对驱动加锁，并在访问每个设备时对其做正确的引用计数
#### sysfs

当注册一个驱动时，会在其总线的目录中创建一sysfs 目录。在该目录中，驱动可以向用户空间导出一个接口，以全局方式控制驱动的运行；例如开关驱动中的调试输出
该目录的一个未来特性将是一'devices' 目录。该目录将包含指向它所支持设备的目录的符号链接
#### 回调

```

	int	(*probe)	(struct device *dev);

```
probe() 入口在任务上下文（task context）中被调用，此时总线rwsem 处于加锁状态，且驱动已部分绑定到设备。驱动通常会在 probe() 以及其他例程中使container_of() "dev" 转换为总线特定的类型。该类型通常提供设备资源数据，例pci_dev.resource[] platform_device.resources，它们与 dev->platform_data 一起用于初始化驱动
该回调包含将驱动绑定到给定设备的驱动特定逻辑。这包括验证设备是否存在、它是否是驱动能够处理的版本、驱动数据结构能否被分配并初始化，以及任何硬件能否被初始化。驱动通常会用 dev_set_drvdata() 保存一个指向其状态的指针。当驱动成功地绑定到该设备时，probe() 返回零，设备模型代码将完成其绑定该驱动到该设备的部分
驱动probe() 可以返回一个负errno 值，以表明该驱动没有绑定到这个设备，此时它应当释放它所分配的所有资源
可选地，如果驱动依赖于尚未可用的资源（例如由尚未初始化的某个驱动所提供的资源），probe() 可以返回 -EPROBE_DEFER。驱动核心会将该设备放入延迟探测（deferred probe）列表，并在稍后再次尝试调用它。如果驱动必须延迟，它应当尽早返-EPROBE_DEFER，以减少花在那些稍后需要撤销并重做的初始化工作上的时间
      -EPROBE_DEFER 不得probe() 已经创建了子设备之后返回，即便那些子设备又在清理路径中被移除。如果在子设备已经注册之后返-EPROBE_DEFER，可能会导致对同一驱动无限循环地调.probe()
```

	void	(*sync_state)	(struct device *dev);

```
sync_state 对某个设备只调用一次。它在该设备的所有消费者（consumer）设备都成功探测后被调用。设备的消费者列表是通过查看将该设备连接到其消费者设备的设备链接（device links）得到的
第一次尝试调sync_state() 是在 late_initcall_sync() 期间进行的，以便给固件和驱动留出将设备互相链接的时间。在第一次尝试调sync_state() 时，如果此时该设备的所有消费者都已经成功探测，sync_state() 会被立即调用。如果在第一次尝试时该设备没有消费者，这也被视为“该设备的所有消费者都已探测”，sync_state() 会被立即调用
如果在第一次尝试为某个设备调用 sync_state() 时，仍有消费者尚未成功探测，sync_state() 调用会被推迟，只有在将来该设备的一个或多个消费者成功探测时才会重试。如果在重试时，驱动核心发现该设备还有一个或多个消费者尚未探测，sync_state() 调用会再次被推迟
sync_state() 的一个典型用例是让内核从引导加载程序（bootloader）干净地接管对设备的管理。例如，如果某个设备被引导加载程序保持在开启状态并处于某个特定的硬件配置，该设备的驱动可能需要将该设备保持在引导配置下，直到该设备的所有消费者都已探测。一旦该设备的所有消费者都已探测，设备驱动就可以将设备的硬件状态同步为匹配所有消费者所请求的聚合软件状态。这正是 sync_state() 名称的由来
虽然regulator 这样的资源是能从 sync_state() 获益的显而易见例子，sync_state() IOMMU 这类复杂资源也很有用。例如，拥有多个消费者（其地址IOMMU 重映射的设备）的 IOMMU，可能需要将其映射固定在引导配置（或在其基础上叠加），直到其所有消费者都已探测
虽然 sync_state() 的典型用例是让内核从引导加载程序干净地接管对设备的管理，sync_state() 的用法并不限于此。只要在某个动作在所有消费者都探测之后才有意义时，都可以使用它
```

	int 	(*remove)	(struct device *dev);

```
remove 被调用以将一个驱动从设备上解绑。如果某个设备被从系统中物理移除、驱动模块正在被卸载、在重启过程中，或其他情况下，都可能调用它
由驱动来决定设备是否存在。它应当释放专门为设备分配的任何资源；即设备 driver_data 字段中的任何内容
如果设备仍然存在，它应当让设备静默（quiesce）并将其置于一个受支持的低功耗状态
```

	int	(*suspend)	(struct device *dev, pm_message_t state);

```
suspend 被调用以将设备置于低功耗状态
```

	int	(*resume)	(struct device *dev);

```
Resume 用于将设备从低功耗状态恢复
#### 属
```

  struct driver_attribute {
          struct attribute        attr;
          ssize_t (*show)(struct device_driver *driver, char *buf);
          ssize_t (*store)(struct device_driver *, const char *buf, size_t count);
  };

```
设备驱动可以通过sysfs 目录导出属性。驱动可以使DRIVER_ATTR_RW DRIVER_ATTR_RO 宏来声明属性，这两个宏的工作方式与 DEVICE_ATTR_RW DEVICE_ATTR_RO 宏完全相同
```

	DRIVER_ATTR_RW(debug);

```
```

	struct driver_attribute driver_attr_debug;

```
随后可以用它来向驱动添加和移除该属```

  int driver_create_file(struct device_driver *, const struct driver_attribute *);
  void driver_remove_file(struct device_driver *, const struct driver_attribute *);

```
