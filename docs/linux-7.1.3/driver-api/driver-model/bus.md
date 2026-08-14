## 总线类型（Bus Types）


#### 定义

参见 struct bus_type 的内核文档（kerneldoc）。

int bus_register(struct bus_type * bus);


#### 声明


内核中的每种总线类型（PCI、USB 等）都应声明一个该类型的静态对象。它们必须初始化 name 字段，并且可以
```

   struct bus_type pci_bus_type = {
          .name	= "pci",
          .match	= pci_bus_match,
   };

```
该结构应在头文件中导出给驱动：

extern struct bus_type pci_bus_type;


#### 注册


当总线驱动被初始化时，它调用 bus_register。这会初始化总线对象中其余的字段，并将其插入到全局总线类型列表中。一旦总线对象被注册，其中的字段即可由总线驱动使用。


#### 回调


#### match()：将驱动附加到设备


设备 ID 结构的格式以及比较它们的语义本质上是总线相关的。驱动通常在总线相关的驱动结构中声明一个它们所支持设备的设备 ID 数组。

match 回调的目的是在不牺牲总线相关功能或类型安全的前提下，给总线一个机会，通过比较驱动所支持的设备 ID 与特定设备的设备 ID，来判断某个特定驱动是否支持某个特定设备。

当在某个总线上注册一个驱动时，会遍历该总线的设备列表，并对每个尚未关联驱动的设备调用 match 回调。


#### 设备和驱动列表


设备和驱动列表旨在取代许多总线维护的本地列表。它们分别是 struct device 和 struct device_driver 的列表。总线驱动可以随意使用这些列表，但可能需要将其转换为总线相关的类型。
```

  int bus_for_each_dev(struct bus_type * bus, struct device * start,
		       void * data,
		       int (*fn)(struct device *, void *));

  int bus_for_each_drv(struct bus_type * bus, struct device_driver * start,
		       void * data, int (*fn)(struct device_driver *, void *));

```
这些辅助函数遍历相应的列表，并为列表中的每个设备或驱动调用回调。所有列表访问都通过获取总线的锁（目前为读锁）进行同步。在调用回调之前，列表中每个对象的引用计数会递增；在获取下一个对象之后会递减。调用回调时不持有锁。


#### sysfs

存在一个名为 'bus' 的顶层目录。

每种总线在 bus 目录下都有一个目录，以及两个默认
```

	/sys/bus/pci/
	|-- devices
	`-- drivers

```
在总线上注册的驱动在总线的 drivers 目录下获得一个目录
```

	/sys/bus/pci/
	|-- devices
	`-- drivers
	    |-- Intel ICH
	    |-- Intel ICH Joystick
	    |-- agpgart
	    `-- e100

```
在该类型总线上发现的每个设备都会在总线的 devices 目录下获得一个指向该设备在物理
```

	/sys/bus/pci/
	|-- devices
	|   |-- 00:00.0 -> ../../../root/pci0/00:00.0
	|   |-- 00:01.0 -> ../../../root/pci0/00:01.0
	|   `-- 00:02.0 -> ../../../root/pci0/00:02.0
	`-- drivers


```
#### 导出属性


```

  struct bus_attribute {
	struct attribute	attr;
	ssize_t (*show)(const struct bus_type *, char * buf);
	ssize_t (*store)(const struct bus_type *, const char * buf, size_t count);
  };

```
总线驱动可以使用 BUS_ATTR_RW 宏导出属性，其工作方式类似于用于设备的 DEVICE_ATTR_RW 宏。例如，
```

	static BUS_ATTR_RW(debug);

```
```

	static bus_attribute bus_attr_debug;

```
随后可将其用于在总线上添加和删除属性
```

	int bus_create_file(struct bus_type *, struct bus_attribute *);
	void bus_remove_file(struct bus_type *, struct bus_attribute *);

```
