## 将驱动移植到新的驱动模型


Patrick Mochel

2003 骞?1 鏈?7 鏃。

概述（Overview
请参`Documentation/driver-api/driver-model/*.rst`，了解各种驱动类型和概念的定义
将设备驱动移植到新模型的大部分工作发生在总线驱动层。这是有意为之，以尽量减小对内核驱动的负面影响，并允许总线驱动逐步过渡
简而言之，驱动模型由一组可以嵌入到更大的、特定于总线的对象中的对象组成。这些通用对象中的字段可以取代特定于总线的对象中的字段
通用对象必须向驱动模型核心注册。这样做之后，它们会通过 sysfs 文件系统导出。sysfs 可以通过
```

	# mount -t sysfs sysfs /sys



```
这个过程（The Process
步骤 0：阅include/linux/device.h，了解对象和函数的定义
步骤 1：注册总线驱动

```
    struct bus_type pci_bus_type = {
          .name           = "pci",
    };


```
- 注册总线类型
  这应当在总线类型的初始化函数中完成，
```

    static int __init pci_driver_init(void)
    {
            return bus_register(&pci_bus_type);
    }

    subsys_initcall(pci_driver_init);


  The bus type may be unregistered (if the bus driver may be compiled
  as a module) by doing::

     bus_unregister(&pci_bus_type);


```
- 导出总线类型供其他代码使用
  其它代码可能希望引用该总线类型，因此应在共享头文件中声明它并导出该符号
```

  extern struct bus_type pci_bus_type;


```
```

  EXPORT_SYMBOL(pci_bus_type);



```
- 这将导致该总线出现/sys/bus/pci/ 下，包含两个
```

    # tree -d /sys/bus/pci/
    /sys/bus/pci/
    |-- devices
    `-- drivers



```
步骤 2：注册设备
struct device 表示单个设备。它主要包含描述该设备与其它实体之间关系的元数据

```

    struct pci_dev {
           ...
           struct  device  dev;            /* Generic device interface */
           ...
    };

  It is recommended that the generic device not be the first item in
  the struct to discourage programmers from doing mindless casts
  between the object types. Instead macros, or inline functions,
  should be created to convert from the generic object type::

    #define to_pci_dev(n) container_of(n, struct pci_dev, dev)

    or

    static inline struct pci_dev * to_pci_dev(struct kobject * kobj)
    {
	return container_of(n, struct pci_dev, dev);
    }

  This allows the compiler to verify type-safety of the operations
  that are performed (which is Good).


```
- 在注册时初始化设备
  当设备被发现或向总线类型注册时，总线驱动应当初始化通用设备。最需要初始化的字段是 bus_id、parent bus
  bus_id 是一ASCII 字符串，包含设备在该总线上的地址。该字符串的格式是特定于总线的。这对于sysfs 中表示设备是必要的
  parent 是设备的物理父设备。总线驱动正确设置该字段非常重要
  驱动模型维护一个有序的设备列表，用于电源管理。该列表必须是有序的，以保证设备在其物理父设备之前被关闭，反之亦然。该列表的顺序由已注册设备的 parent 决定
  此外，设备的 sysfs 目录的位置取决于设备parent。sysfs 导出一个镜像设备层次的目录结构。准确地设置 parent 可以保证 sysfs 准确地表示这个层次
  设备bus 字段是一个指向该设备所属总线类型的指针。它应当被设置为之前已声明并初始化的 bus_type
  可选地，总线驱动可以设置设备name release 字段
  name 字段是一个描述该设备ASCII 字符串，例如

     "ATI Technologies Inc Radeon QD"

  release 字段是一个回调函数，当设备已被移除、且对它的所有引用都已被释放时，驱动模型核心会调用它。稍后将对此做更多说明

- 注册设备
  一旦通用设备被初始化，就可以注册```

       device_register(&dev->dev);

  It can later be unregistered by doing::

       device_unregister(&dev->dev);

  This should happen on buses that support hotpluggable devices.
  If a bus driver unregisters a device, it should not immediately free
  it. It should instead wait for the driver model core to call the
  device's release method, then free the bus-specific object.
  (There may be other code that is currently referencing the device
  structure, and it would be rude to free the device while that is
  happening).


  当设备被注册时，会在 sysfs 中创建一个目录。sysfs 中的 PCI 树形如：
    /sys/devices/pci0/
    |-- 00:00.0
    |-- 00:01.0
    |   `-- 01:00.0
    |-- 00:02.0
    |   `-- 02:1f.0
    |       `-- 03:00.0
    |-- 00:1e.0
    |   `-- 04:04.0
    |-- 00:1f.0
    |-- 00:1f.1
    |   |-- ide0
    |   |   |-- 0.0
    |   |   `-- 0.1
    |   `-- ide1
    |       `-- 1.0
    |-- 00:1f.2
    |-- 00:1f.3
    `-- 00:1f.5

  此外，在总线'devices' 目录中会创建指向设备物理层次目录的符号链接：
    /sys/bus/pci/devices/
    |-- 00:00.0 -> ../../../devices/pci0/00:00.0
    |-- 00:01.0 -> ../../../devices/pci0/00:01.0
    |-- 00:02.0 -> ../../../devices/pci0/00:02.0
    |-- 00:1e.0 -> ../../../devices/pci0/00:1e.0
    |-- 00:1f.0 -> ../../../devices/pci0/00:1f.0
    |-- 00:1f.1 -> ../../../devices/pci0/00:1f.1
    |-- 00:1f.2 -> ../../../devices/pci0/00:1f.2
    |-- 00:1f.3 -> ../../../devices/pci0/00:1f.3
    |-- 00:1f.5 -> ../../../devices/pci0/00:1f.5
    |-- 01:00.0 -> ../../../devices/pci0/00:01.0/01:00.0
    |-- 02:1f.0 -> ../../../devices/pci0/00:02.0/02:1f.0
    |-- 03:00.0 -> ../../../devices/pci0/00:02.0/02:1f.0/03:00.0
    `-- 04:04.0 -> ../../../devices/pci0/00:1e.0/04:04.0



```
步骤 3：注册驱动
struct device_driver 是一个简单的驱动结构，包含一组驱动模型核心可能会调用的操作

- 在特定于总线的驱动中嵌入一struct device_driver
```

    struct pci_driver {
           ...
           struct device_driver    driver;
    };


```
- 初始化通用驱动结构
  当驱动向总线注册时（例如调用 pci_register_driver()），初始化驱动的必要字段：name bus 字段

- 注册驱动
```

	driver_register(&drv->driver);

  to register the driver with the core.

  When the driver is unregistered from the bus, unregister it from the
  core by doing::

        driver_unregister(&drv->driver);

  Note that this will block until all references to the driver have
  gone away. Normally, there will not be any.


```
- sysfs 表示
  驱动通过 sysfs 在其总线'drivers' 目录中导出```

    /sys/bus/pci/drivers/
    |-- 3c59x
    |-- Ensoniq AudioPCI
    |-- agpgart-amdk7
    |-- e100
    `-- serial


```
步骤 4：为驱动定义通用方法
struct device_driver 定义了一组驱动模型核心会调用的操作。这些操作中的大多数可能与总线已经为驱动定义的操类似，但参数不同
强制让总线上的每一个驱动同时将它们自己的驱动转换为通用格式，会是困难且繁琐的。相反，总线驱动应当定义通用方法的单一实例，由
```


  static int pci_device_remove(struct device * dev)
  {
          struct pci_dev * pci_dev = to_pci_dev(dev);
          struct pci_driver * drv = pci_dev->driver;

          if (drv) {
                  if (drv->remove)
                          drv->remove(pci_dev);
                  pci_dev->driver = NULL;
          }
          return 0;
  }


```
通用驱动应当用这些方法初始化，然```

        /* initialize common driver fields */
        drv->driver.name = drv->name;
        drv->driver.bus = &pci_bus_type;
        drv->driver.probe = pci_device_probe;
        drv->driver.resume = pci_device_resume;
        drv->driver.suspend = pci_device_suspend;
        drv->driver.remove = pci_device_remove;

        /* register with core */
        driver_register(&drv->driver);


```
理想情况下，总线只应在这些字段尚未被设置时才初始化它们。这样允许驱动实现它们自己的通用方法

步骤 5：支持通用驱动绑定
该模型假设设备或驱动可以在任何时刻动态地注册到总线上。当注册发生时，设备必须绑定到一个驱动，或者驱动必须绑定到它所支持的所有设备
驱动通常包含一个它所支持的设ID 列表。总线驱动将这ID 与注册到它上面的设备ID 进行比较。设ID 的格式，以及比较它们的语义，是特定于总线的，因此通用模型并不试图对它们进行泛化
相反，总线可以struct bus_type 中提供一个方法，```

  int (*match)(struct device * dev, struct device_driver * drv);

```
如果驱动支持该设备，match 应当返回一个正值，否则返回 0。如果无法确定给定驱动是否支持该设备，它也可以返回错误码（例-EPROBE_DEFER）
当设备被注册时，会遍历总线的驱动列表。对每个驱动调用 bus->match()，直到找到匹配
当驱动被注册时，会遍历总线的设备列表。对每一个尚未被某个驱动认领的设备调bus->match()
当一个设备成功地绑定到一个驱动时，会设置 device->driver，将该设备添加到该驱动的每驱动设备列表中，并在该驱动sysfs 目录中创建一个指```

  /sys/bus/pci/drivers/
  |-- 3c59x
  |   `-- 00:0b.0 -> ../../../../devices/pci0/00:0b.0
  |-- Ensoniq AudioPCI
  |-- agpgart-amdk7
  |   `-- 00:00.0 -> ../../../../devices/pci0/00:00.0
  |-- e100
  |   `-- 00:0c.0 -> ../../../../devices/pci0/00:0c.0
  `-- serial


```
这种驱动绑定应当取代总线当前使用的现有驱动绑定机制

步骤 6：提供热插拔回调
每当一个设备被注册到驱动模型核心时，用户空间程/sbin/hotplug 会被调用，以通知用户空间。用户可以定义在设备或插移除时要执行的动作
驱动模型核心通过环境变量向用户空间传递若干参数，包括

- ACTION：设置为 'add' 'remove'
- DEVPATH：设置为设备sysfs 中的物理路径
总线驱动也可以提供额外的参数供用户空间使用。为此，总线必须```

     int (*hotplug) (struct device *dev, char **envp,
                     int num_envp, char *buffer, int buffer_size);

```
中实'hotplug' 方法。这会在 /sbin/hotplug 执行之前立即被调用

步骤 7：清理总线驱动
通用bus、device driver 结构提供了若干字段，可以取代总线驱动私下定义的那些字段
- 设备列表
struct bus_type 包含一个注册到该总线类型的所有设备的列表。这包括该总线类型所有实例上的所有设备。总线使用的内部列表可以被移除，转而使用这一个
```

  int bus_for_each_dev(struct bus_type * bus, struct device * start,
                       void * data, int (*fn)(struct device *, void *));


```
- 驱动列表
struct bus_type 还包含一个注册到它的所有驱动的列表。总线驱动维护的驱动内部列表可以被移除，转而使用通用的那一个
```

  int bus_for_each_drv(struct bus_type * bus, struct device_driver * start,
                       void * data, int (*fn)(struct device_driver *, void *));


```
更多相关信息请参drivers/base/bus.c

- rwsem銆。
struct bus_type 包含一rwsem，用于保护对设备和驱动列表的所有核心访问。总线驱动可以在内部使用它，并且在访问总线维护的设备或驱动列表时应当使用它

- 设备和驱动字段
struct device struct device_driver 中的某些字段与这些对象的特定于总线的表示中的字段重复。可以随意移除特定于总线的字段，转而使用通用字段。不过请注意，这很可能意味着要修复所有引用了这些特定于总线的字段的驱动（尽管这些应该都只是一行改动）