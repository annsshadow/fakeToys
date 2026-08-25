## Driver Binding


驱动绑定（driver binding）是将一个设备与能够控制它的设备驱动关联起来的过程。总线驱动通常负责处理这一工作，因为以往都是用总线特定的结构来表示设备和驱动。有了通用device device_driver 结构之后，大部分绑定工作都可以通过公共代码来完成

#### Bus


总线类型（bus type）结构包含一个系统中属于该总线类型的所有设备的列表。当为某个设备调device_register 时，它会被插入到该列表的末尾。总线对象还包含一个该总线类型所有驱动的列表。当为某个驱动调driver_register 时，它会被插入到该列表的末尾。这两个事件会触发驱动绑定

#### device_register


当添加一个新设备时，会遍历该总线的驱动列表，以找到一个支持它的驱动。为了确定这一点，设备device ID 必须与驱动所支持的某device ID 相匹配。比ID 的格式和语义是总线特定的。总线驱动需要提供回调函数，将设备与驱动ID 进行比对，而不是去推导一个复杂的状态机和匹配算法。如果找到匹配，总线返回 1；否则返0
int match(struct device ** dev, struct device_driver ** drv);

如果找到匹配，设备的 driver 字段会被设置为该驱动，并调用驱动probe 回调。这让驱动有机会验证它确实支持该硬件，并且硬件处于可用状态

#### Device Class


probe 成功完成后，设备会注册到它所属的类（class）。设备驱动属于且仅属于一个类，该信息设置在驱动的 devclass 字段中。会调用 devclass_add_device 在该类中枚举设备，并通过类的 register_dev 回调实际将设备注册到类中

#### Driver


当驱动被附加到一个设备时，会调用驱动probe() 函数。在 probe() 内部，驱动初始化设备，并分配、初始化每设备（per-device）的数据结构。只要驱动保持与该设备的绑定，这些每设备状态就与设备对象相关联。从概念上讲，这些每设备数据与对设备的绑定一起，可以看作驱动的一个实例

#### sysfs


会在总线'devices' 目录下创建一个符号链接，指向设备在物理层次结构中的目录
会在驱动'devices' 目录下创建一个符号链接，指向设备在物理层次结构中的目录
会在类的目录下为设备创建一个目录。在该目录中会创建一个符号链接，指向设备sysfs 树中的物理位置
可以在设备的物理目录中创建一个符号链接（尽管目前尚未这样做），指向它的类目录或类的顶层目录。也可以创建一个符号链接指向它的驱动目录

#### driver_register


添加新驱动时的过程几乎完全相同。会遍历总线的设备列表以寻找匹配。已经拥有驱动的设备会被跳过。会遍历所有设备，以便将尽可能多的设备绑定到该驱动

#### Removal


当移除一个设备时，它的引用计数最终会变为 0。当变为 0 时，会调用驱动的 remove 回调。设备会从驱动的的设备列表中移除，驱动的引用计数递减。两者之间的所有符号链接都会被删除
当移除一个驱动时，会遍历它支持的设备列表，并对每个设备调用驱动的 remove 回调。设备会从该列表中移除，符号链接也会被删除

#### Driver Override


用户空间可以通过向设备的 `driver_override` sysfs 属性写入驱动名称，来覆盖标准的匹配规则。一旦设置，在绑定时只会考虑名称与覆盖值相匹配的驱动。这会绕过所有总线特定的匹配（OF、ACPI、ID 表等）
可以通过写入空字符串来清除覆盖，使设备恢复到标准匹配规则。写`driver_override` 并不会自动将设备从其当前驱动解绑，也不会尝试加载指定的驱动
总线通过`bus_type` 中设`driver_override` 标志来启用这一机制
```

  const struct bus_type example_bus_type = {
      ...
      .driver_override = true,
  };

```
当该标志被设置后，驱动核心会自动为该总线上的每个设备创建 `driver_override` sysfs 属性
总线`match()` 回调应在执行具体匹配之前先检查覆盖值：

```

  static int example_match(struct device *dev, const struct device_driver *drv)
  {
      int ret;

      ret = device_match_driver_override(dev, drv);
      if (ret >= 0)
          return ret;

      /* Fall through to bus-specific matching... */
  }

```
`device_match_driver_override()` 在覆盖值匹配给定驱动时返回 > 0；若覆盖值已设置但不匹配则返0；若完全未设置覆盖值则返回 < 0
还提供以下辅助函数：

- `device_set_driver_override()` - 从内核代码中设置或清除覆盖值- `device_has_driver_override()` - 检查是否已设置覆盖值