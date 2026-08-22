
## sysfs —用于导出内核对象的文件系

Patrick Mochel	<mochel@osdl.org>

Mike Murphy <mamurph@cs.clemson.edu>

:Revised:    16 August 2011
:Original:   10 January 2003


#### 它是什

sysfs 是一个基RAM 的文件系统，最初基ramfs。它提供了一种将内核数据结构、其属性以及它们之间的链接导出到用户空间的方式
sysfs 本质上与 kobject 基础设施绑定。有kobject 接口的更多信息，请阅Documentation/core-api/kobject.rst

#### 使用 sysfs


如果定义CONFIG_SYSFS，sysfs 总是被编译进内核。你可以通过以下方式访问

```

    mount -t sysfs sysfs /sys


```
#### 目录的创

对于系统中注册的每个 kobject，都会在 sysfs 中为其创建一个目录。该目录作为 kobject 父目录的子目录创建，从而向用户空间表达内部对象层级。sysfs 中的顶层目录表示对象层级的共同祖先；即对象所属的子系统
sysfs 在与该目录关联的 kernfs_node 对象内部存储一个指向实现该目录kobject 的指针。过去，sysfs 在文件打开或关闭时直接使用kobject 指针kobject 进行引用计数。在当前 sysfs 实现中，kobject 引用计数仅由 sysfs_schedule_callback() 函数直接修改

#### 属

可以以文件系统中普通文件的形式kobject 导出属性。sysfs 将文I/O 操作转发给为属性定义的方法，从而提供一种读写内核属性的手段
属性应ASCII 文本文件，最好每个文件只有一个值。需要注意的是，每个文件只包含一个值可能效率不高，因此表达同一类型的数值数组在社会层面上是可接受的
混合类型、表达多行数据以及花哨地格式化数据是被强烈反对的。做这些事情可能会让你当众出丑，并且你的代码会在未通知的情况下被重写
```

    struct attribute {
	    char                    *name;
	    struct module           *owner;
	    umode_t                 mode;
    };


    int sysfs_create_file(struct kobject * kobj, const struct attribute * attr);
    void sysfs_remove_file(struct kobject * kobj, const struct attribute * attr);


```
一个裸属性不包含读或写属性值的手段。鼓励子系统定义自己的属性结构以及用于为特定对象类型添加和移除属性的包装函数
```

    struct device_attribute {
	    struct attribute	attr;
	    ssize_t (*show)(struct device *dev, struct device_attribute *attr,
			    char *buf);
	    ssize_t (*store)(struct device *dev, struct device_attribute *attr,
			    const char *buf, size_t count);
    };

    int device_create_file(struct device *, const struct device_attribute *);
    void device_remove_file(struct device *, const struct device_attribute *);

```
```

    #define DEVICE_ATTR(_name, _mode, _show, _store) \
    struct device_attribute dev_attr_##_name = __ATTR(_name, _mode, _show, _store)

```
```

    static DEVICE_ATTR(foo, S_IWUSR | S_IRUGO, show_foo, store_foo);

```
```

    static struct device_attribute dev_attr_foo = {
	    .attr = {
		    .name = "foo",
		    .mode = S_IWUSR | S_IRUGO,
	    },
	    .show = show_foo,
	    .store = store_foo,
    };

```
注意，如 include/linux/sysfs.h 中所述，“OTHER_WRITABLE？通常被认为是个坏主意。”因此尝试将 sysfs 文件设置为对所有人都可写会失败，并回退为对“Others”的只读模式
对于常见情况，sysfs.h 提供了便利宏，使定义属性更容易，同时让代码更简洁易读。上述情况可简写为
static struct device_attribute dev_attr_foo = __ATTR_RW(foo);

可用于定义你包装函数的辅助宏列表如下
__ATTR_RO(name)		 假设默认 name_show 且模式为 0444
__ATTR_WO(name)		 假设只有 name_store，并限制为模0200，即root 可写__ATTR_RO_MODE(name, mode)	         用于更严格的只读访问；目前唯一用例EFI 系统资源	         （见 drivers/firmware/efi/esrt.c__ATTR_RW(name)	         假设默认 name_show、name_store，并将模式设0644__ATTR_NULL	         将名称设NULL，用作列表结束指示符（见：kernel/workqueue.c
#### 子系统特定的回调


当子系统定义新的属性类型时，它必须实现一sysfs 操作，用于将写调用转发给

```

    struct sysfs_ops {
	    ssize_t (*show)(struct kobject *, struct attribute *, char *);
	    ssize_t (*store)(struct kobject *, struct attribute *, const char *, size_t);
    };

```
[ 子系统应当已经定义了一struct kobj_type 作为该类型的描述符，sysfs_ops 指针就存储于此。更多信息请参见 kobject 文档]

当文件被读或写时，sysfs 会调用该类型的适当方法。该方法随后将通用struct kobject struct attribute 指针转换为适当的指针类型，并调用关联的方法
```

    #define to_dev_attr(_attr) container_of(_attr, struct device_attribute, attr)

    static ssize_t dev_attr_show(struct kobject *kobj, struct attribute *attr,
				char *buf)
    {
	    struct device_attribute *dev_attr = to_dev_attr(attr);
	    struct device *dev = kobj_to_dev(kobj);
	    ssize_t ret = -EIO;

	    if (dev_attr->show)
		    ret = dev_attr->show(dev, dev_attr, buf);
	    if (ret >= (ssize_t)PAGE_SIZE) {
		    printk("dev_attr_show: %pS returned bad count\n",
				    dev_attr->show);
	    }
	    return ret;
    }



```
#### 写属性数

要读或写属性，必须在声明属性时指定 show() store() 方法。方法类型应如下

```

    ssize_t (*show)(struct device *dev, struct device_attribute *attr, char *buf);
    ssize_t (*store)(struct device *dev, struct device_attribute *attr,
		    const char *buf, size_t count);

```
换言之，它们应只接受一个对象、一个属性和一个缓冲区作为参数
sysfs 分配一个大小为 (PAGE_SIZE) 的缓冲区并传递给方法。sysfs 对每次读或写只调用该方法一次。这强制方法实现遵循以下行为
- read(2) 时，show() 方法应填满整个缓冲区。回想一下，一个属性只应导出一个值或一组相似值，因此这不应太昂贵。这允许用户空间进行部分读取，并在整个文件上随意前向定位。如果用户空间定位回零或用偏移量 '0' 进行 pread(2)，show() 方法会被再次调用（重新装填）以填充缓冲区
  这允许用户空间进行部分读取并在整个文件上随意前向定位。如果用户空间定位回零或用偏移量 '0' 进行 pread(2)，show() 方法会被再次调用，重新装填，以填充缓冲区
- write(2) 时，sysfs 期望在第一次写入时传递整个缓冲区。sysfs 随后将整个缓冲区传递给 store() 方法。在 store 时，数据之后会添加一个终止空字符。这使得 sysfs_streq() 等函数可以安全使用
  写入 sysfs 文件时，用户空间进程应首先读取整个文件，修改其希望改变的值，然后将整个缓冲区写回
  属性的方法实现在读写值时应在相同的缓冲区上操作
其他注意事项
- 写入会导show() 方法被重新装填，无论当前文件位置如何
- 缓冲区长度始终为 PAGE_SIZE 字节。在 x86 上，这是 4096
- show() 方法应返回打印到缓冲区中的字节数
- show() 方法的新实现在格式化要返回给用户空间的值时，应只使sysfs_emit() sysfs_emit_at()
- store() 应返回从缓冲区中使用的字节数。如果整个缓冲区都已被使用，直接返回 count 参数即可
- show() store() 总是可以返回错误。如果传入了错误的值，务必返回一个错误
- 传递给方法的对象会通过 sysfs 对其内嵌对象的引用计数被固定在内存中。然而，该对象所代表的物理实体（例如设备）可能并不存在。如有必要，务必有办法对此进行检查
```

    static ssize_t show_name(struct device *dev, struct device_attribute *attr,
			    char *buf)
    {
	    return sysfs_emit(buf, "%s\n", dev->name);
    }

    static ssize_t store_name(struct device *dev, struct device_attribute *attr,
			    const char *buf, size_t count)
    {
	    snprintf(dev->name, sizeof(dev->name), "%.*s",
		    (int)min(count, sizeof(dev->name) - 1), buf);
	    return count;
    }

    static DEVICE_ATTR(name, S_IRUGO, show_name, store_name);


```
（注意，真实实现不允许用户空间设置设备的名称。）


#### 顶层目录布局


sysfs 的目录排列展现了内核数据结构之间的关系
```

    block/
    bus/
    class/
    dev/
    devices/
    firmware/
    fs/
    hypervisor/
    kernel/
    module/
    power/

```
devices/ 包含设备树在文件系统中的表示。它直接映射到内部内核设备树，即 struct device 的层级结构
bus/ 包含系统中各种总线类型的扁平目录布局
```

	devices/
	drivers/

```
devices/ 包含为系统中发现的每个设备所建立的指向其/sys/devices 下目录的符号链接
drivers/ 包含为该特定总线上的设备所加载的每个设备驱动对应一个目录（这假设驱动不会跨越多种总线类型）
fs/ 包含某些文件系统的目录。目前每个希望导出属性的文件系统必须fs/ 下创建自己的层级（示例见 fuse/fuse.rst）
module/ 包含所有已加载系统模块（包括内建与可加载模块）的参数值与状态信息
dev/ 包含两个目录：char/ block/。这两个目录内有名为 <major>:<minor> 的符号链接。这些符号链接指向每个设备在 /sys/devices 下的目录sys/dev 提供了一种从 stat(2) 操作结果快速查找设sysfs 接口的方式
有关驱动模型特定特性的更多信息可在 Documentation/driver-api/driver-model/ 中找到
block/ 包含指向系统上发现的所有块设备的符号链接。这些符号链接指/sys/devices 下的目录
class/ 包含按功能类型分组的每个设备类的目录。class/ 中的每个目录包含指向 /sys/devices 目录中设备的符号链接
firmware/ 包含系统固件数据与配置，例如固件表、ACPI 信息与设备树数据
hypervisor/ 包含虚拟化平台信息，并提供到底层 hypervisor 的接口。仅在运行于虚拟机上时存在
kernel/ 包含运行时内核参数、配置设置与状态
power/ 包含电源管理子系统信息，包括睡眠状态、挂恢复能力以及策略

#### 当前的接

sysfs 当前存在以下接口层

### devices (include/linux/device.h)

```

    struct device_attribute {
	    struct attribute	attr;
	    ssize_t (*show)(struct device *dev, struct device_attribute *attr,
			    char *buf);
	    ssize_t (*store)(struct device *dev, struct device_attribute *attr,
			    const char *buf, size_t count);
    };

```
```

    DEVICE_ATTR(_name, _mode, _show, _store);

```
```

    int device_create_file(struct device *dev, const struct device_attribute * attr);
    void device_remove_file(struct device *dev, const struct device_attribute * attr);


```
### bus drivers (include/linux/device.h)

```

    struct bus_attribute {
	    struct attribute        attr;
	    ssize_t (*show)(const struct bus_type *, char * buf);
	    ssize_t (*store)(const struct bus_type *, const char * buf, size_t count);
    };

```
```

    static BUS_ATTR_RW(name);
    static BUS_ATTR_RO(name);
    static BUS_ATTR_WO(name);

```
```

    int bus_create_file(struct bus_type *, struct bus_attribute *);
    void bus_remove_file(struct bus_type *, struct bus_attribute *);


```
### device drivers (include/linux/device.h)


```

    struct driver_attribute {
	    struct attribute        attr;
	    ssize_t (*show)(struct device_driver *, char * buf);
	    ssize_t (*store)(struct device_driver *, const char * buf,
			    size_t count);
    };

```
```

    DRIVER_ATTR_RO(_name)
    DRIVER_ATTR_RW(_name)

```
```

    int driver_create_file(struct device_driver *, const struct driver_attribute *);
    void driver_remove_file(struct device_driver *, const struct driver_attribute *);


```
#### 文档


sysfs 目录结构以及每个目录中的属性定义了内核与用户空间之间的 ABI。对于任ABI 而言，该 ABI 保持稳定并得到妥善文档化都很重要。所有新sysfs 属性必须在 Documentation/ABI 中记录。更多信息另Documentation/ABI/README