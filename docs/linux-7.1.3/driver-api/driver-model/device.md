## 基本设备结构

请参struct device kerneldoc

#### 编程接口

发现该设备的总线驱动使用此接口来注册

```
  int device_register(struct device * dev);

```
总线应当初始化以下字段：

    - parent
    - name
    - bus_id
    - bus

当引用计数降为以下值时，设备将从核心中移除

```
  struct device * get_device(struct device * dev);
  void put_device(struct device * dev);

```
如果引用计数还不0（即设备正在被移除的过程中），get_device() 将返传入struct device 指针
```
  void lock_device(struct device * dev);
  void unlock_device(struct device * dev);

```
#### 属

```
  struct device_attribute {
	struct attribute	attr;
	ssize_t (*show)(struct device *dev, struct device_attribute *attr,
			char *buf);
	ssize_t (*store)(struct device *dev, struct device_attribute *attr,
			 const char *buf, size_t count);
  };

```
设备的属性可以由设备驱动通过 sysfs 导出
请参Documentation/filesystems/sysfs.rst 以了解更多关sysfs
工作原理的信息
Documentation/core-api/kobject.rst 所解释，设备属性必须在生成
KOBJ_ADD uevent 之前创建。实现这一点的唯一方式是定义一个属性组
```
  #define DEVICE_ATTR(name,mode,show,store)

```
```
  static DEVICE_ATTR(type, 0444, type_show, NULL);
  static DEVICE_ATTR(power, 0644, power_show, power_store);

```
对于 mode 的常见取值，提供了辅助宏，因此上述示例可以改写为

```
  static DEVICE_ATTR_RO(type);
  static DEVICE_ATTR_RW(power);

```
这会声明两个类型struct device_attribute 的结构体，名称分别为
'dev_attr_type' 'dev_attr_power'。这两个属性可以通过

```
  static struct attribute *dev_attrs[] = {
	&dev_attr_type.attr,
	&dev_attr_power.attr,
	NULL,
  };

  static struct attribute_group dev_group = {
	.attrs = dev_attrs,
  };

  static const struct attribute_group *dev_groups[] = {
	&dev_group,
	NULL,
  };

```
对于单一组的常见情况，提供了一个辅助宏，因此上述代码可以改写为

```
  ATTRIBUTE_GROUPS(dev);

```
随后可以通过将以下指针关联到设备来将该组数组与设备关联：

```
        dev->groups = dev_groups;
        device_register(dev);

```
device_register() 函数将使'groups' 指针来创建设备属性，device_unregister() 函数将使用该指针来移除设备属性
警告：虽然内核允许在任意时刻对设备调device_create_file() device_remove_file()，但用户空间对属性的创建时机有严格的预期。当
一个新设备在内核中注册时，会生成一uevent 来通知用户空间（如 udev有一个新设备可用。如果在设备注册之后才添加属性，那么用户空间将不收到通知，也就不会知道这些新属性
这对于需要在驱动探测时为设备发布额外属性的设备驱动十分重要。如设备驱动只是对其传入的设备结构体调用 device_create_file()，那用户空间将永远收不到新属性的通知