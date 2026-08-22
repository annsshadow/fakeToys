## Linux 硬件监控内核 API


Guenter Roeck

### 简

本文档描述了希望使用硬件监控框架的硬件监控驱动所能使用的 API
本文档并不描述什么是硬件监控（hwmon）驱动或设备，也不描述用户空间可用于与硬件监控设通信API。如果你想知道这些，请阅读以下文件：Documentation/hwmon/sysfs-interface.rst
关于如何编写和改hwmon 驱动的更多指南，也请阅读 Documentation/hwmon/submitting-patches.rst
### API

每个硬件监控驱动必须 `#include <linux/hwmon.h>`，在某些情况下还`#include <linux/hwmon-sysfs.h>`linux/hwmon.h 声明了以下内容：

```

  struct device *
  hwmon_device_register_with_info(struct device *dev,
				  const char *name, void *drvdata,
				  const struct hwmon_chip_info *info,
				  const struct attribute_group **extra_groups);

  struct device *
  devm_hwmon_device_register_with_info(struct device *dev,
				       const char *name,
				       void *drvdata,
				       const struct hwmon_chip_info *info,
				       const struct attribute_group **extra_groups);

  void hwmon_device_unregister(struct device *dev);

  char *hwmon_sanitize_name(const char *name);

  char *devm_hwmon_sanitize_name(struct device *dev, const char *name);

  void hwmon_lock(struct device *dev);
  void hwmon_unlock(struct device *dev);

```

hwmon_device_register_with_info 注册一个硬件监控设备。它在硬件监控核心中创建标准sysfs
属性，让驱动专注于读写芯片，而不必操sysfs 属性。父设备参数以及芯片参数都不能为 NULL。其
参数在下面更详细地描述
devm_hwmon_device_register_with_info hwmon_device_register_with_info 类似。不过，它是
设备托管的（device managed），意味着硬件监控设备无需由移除函数显式移除
所有其他硬件监控设备注册函数都已弃用，不得在新驱动中使用
hwmon_device_unregister 注销一个已注册的硬件监控设备。该函数的参数是指向已注册硬件监控设结构的指针。如果硬件监控设备是通过 hwmon_device_register_with_info 注册的，则必须从驱动remove 函数中调用此函数
所有受支持hwmon 设备注册函数只接受有效的设备名称。包含无效字符（空白、`*` `-`）的设备
名称将被拒绝。如果以 NULL 作为 name 参数传入，硬件监控设备名称将从父设备名称派生
如果驱动不使用静态设备名称（例如它使dev_name()），因此无法确保名称只包含有效字符，可以hwmon_sanitize_name。此便捷函数会复制字符串并将任何无效字符替换为下划线。它会为新字符串
分配内存，调用者有责任在设备移除时释放该内存
devm_hwmon_sanitize_name hwmon_sanitize_name 的资源托管版本；内存将在设备移除时自动释放
当使`[devm_]hwmon_device_register_with_info()` 注册硬件监控设备时，使用相关访问函数的访由硬件监控核心串行化。如果驱动需要为其他函数（例如中断处理程序，或完全在驱动中实现的属性）加锁可以使用 hwmon_lock() hwmon_unlock() 来确保对这些函数的调用被串行化
### 使用 devm_hwmon_device_register_with_info()

hwmon_device_register_with_info() 注册一个硬件监控设备。该函数的参数是

=============================================== ===============================================
`struct device *dev`			指向父设备的指针
`const char *name`			设备名称
`void *drvdata`				驱动私有数据
`const struct hwmon_chip_info *info`	指向芯片描述的指针`const struct attribute_group **extra_groups` 	NULL 结尾的附加非标准
						sysfs 属性组列表=============================================== ===============================================

此函数在成功时返回指向所创建硬件监控设备的指针，失败则返回负的错误码
```

	struct hwmon_chip_info {
		const struct hwmon_ops *ops;
		const struct hwmon_channel_info * const *info;
	};

```

它包含以下字段：

- ops:
	指向设备操作的指针- info:
	NULL 结尾的设备通道描述符列表
```

  struct hwmon_ops {
	umode_t (*is_visible)(const void *, enum hwmon_sensor_types type,
			      u32 attr, int);
	int (*read)(struct device *, enum hwmon_sensor_types type,
		    u32 attr, int, long *);
	int (*write)(struct device *, enum hwmon_sensor_types type,
		     u32 attr, int, long);
  };

```

它定义了以下操作
- is_visible:
    指向一个函数的指针，返回每个受支持属性的文件模式。此函数是必需的
- read:
    指向一个函数的指针，用于从芯片读取值。此函数是可选的，但若存在任何可读属性则必须提供
- write:
    指向一个函数的指针，用于向芯片写入值。此函数是可选的，但若存在任何可写属性则必须提供
每个传感器通道都使struct hwmon_channel_info 描述，即

```

	struct hwmon_channel_info {
		enum hwmon_sensor_types type;
		u32 *config;
	};

```

它包含以下字段：

- type:
    硬件监控传感器类型
    受支持的传感器类型有

     ================== ==================================================
     hwmon_chip		一个虚拟传感器类型，用于描述不绑定到特定输入或输出的属     hwmon_temp		温度传感     hwmon_in		电压传感     hwmon_curr		电流传感     hwmon_power	功率传感     hwmon_energy	能量传感     hwmon_energy64	能量传感器，64 位有符号值报     hwmon_humidity	湿度传感     hwmon_fan		风扇转速传感器
     hwmon_pwm		PWM 控制
     ================== ==================================================

- config:
    指向给定类型的每个传感器0 结尾的配置值列表的指针。每个值是指示单个传感器所支持的属    的位值的组合
作为一个例子，这是 LM75 兼容传感器芯片的完整描述文件。该芯片具有单个温度传感器。驱动希向热子系统注册（HWMON_C_REGISTER_TZ），并且支持 update_interval 属性（HWMON_C_UPDATE_INTERVAL）该芯片支持读取温度（HWMON_T_INPUT），它有一个最高温度寄存器（HWMON_T_MAX）以及一个最高温度迟寄存器（HWMON_T_MAX_HYST
```

	static const u32 lm75_chip_config[] = {
		HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL,
		0
	};

	static const struct hwmon_channel_info lm75_chip = {
		.type = hwmon_chip,
		.config = lm75_chip_config,
	};

	static const u32 lm75_temp_config[] = {
		HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST,
		0
	};

	static const struct hwmon_channel_info lm75_temp = {
		.type = hwmon_temp,
		.config = lm75_temp_config,
	};

	static const struct hwmon_channel_info * const lm75_info[] = {
		&lm75_chip,
		&lm75_temp,
		NULL
	};

	HWMON_CHANNEL_INFO() 宏可以且应当在可能时优先使用	借助此宏，上面的示例可以简化成

	static const struct hwmon_channel_info * const lm75_info[] = {
		HWMON_CHANNEL_INFO(chip,
				HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL),
		HWMON_CHANNEL_INFO(temp,
				HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST),
		NULL
	};

	其余的声明如下
	static const struct hwmon_ops lm75_hwmon_ops = {
		.is_visible = lm75_is_visible,
		.read = lm75_read,
		.write = lm75_write,
	};

	static const struct hwmon_chip_info lm75_chip_info = {
		.ops = &lm75_hwmon_ops,
		.info = lm75_info,
	};

```

指示各个属性支持的位值的完整列表定义include/linux/hwmon.h 中。定义前缀如下
=============== =================================================
HWMON_C_xxxx	芯片属性，hwmon_chip 一起使用HWMON_T_xxxx	温度属性，hwmon_temp 一起使用HWMON_I_xxxx	电压属性，hwmon_in 一起使用HWMON_C_xxxx	电流属性，hwmon_curr 一起使用		注意此前缀与芯片属性前缀重叠HWMON_P_xxxx	功率属性，hwmon_power 一起使用HWMON_E_xxxx	能量属性，hwmon_energy 一起使用HWMON_H_xxxx	湿度属性，hwmon_humidity 一起使用HWMON_F_xxxx	风扇转速属性，hwmon_fan 一起使用HWMON_PWM_xxxx	PWM 控制属性，hwmon_pwm 一起使用=============== =================================================

### 驱动回调函数


每个驱动提供 is_visible、read write 函数。参
```

  umode_t is_visible_func(const void *data, enum hwmon_sensor_types type,
			  u32 attr, int channel)

```

参数	data:
		指向设备私有数据结构的指针	type:
		传感器类型	attr:
		与特定属性关联的属性标识符		例如，HWMON_T_INPUT 的属性值将hwmon_temp_input。关于位字段		属性值的完整映射，请参阅 include/linux/hwmon.h	channel:
		传感器通道号
返回值：
	此属性的文件模式。通常，这将是 0（不会创建该属性）444 0644
```

	int read_func(struct device *dev, enum hwmon_sensor_types type,
		      u32 attr, int channel, long *val)

```

参数	dev:
		指向硬件监控设备的指针	type:
		传感器类型	attr:
		与特定属性关联的属性标识符		例如，HWMON_T_INPUT 的属性值将hwmon_temp_input。完整映射请参阅
		include/linux/hwmon.h銆?	channel:
		传感器通道号	val:
		指向属性值的指针		对于 hwmon_energy64，`'val`' 作为 `long *` 传入，但需要类型转换为 `s64 *`
返回值：
	成功0，否则为负错误号
```

	int write_func(struct device *dev, enum hwmon_sensor_types type,
		       u32 attr, int channel, long val)

```

参数	dev:
		指向硬件监控设备的指针	type:
		传感器类型	attr:
		与特定属性关联的属性标识符		例如，HWMON_T_INPUT 的属性值将hwmon_temp_input。完整映射请参阅
		include/linux/hwmon.h銆?	channel:
		传感器通道号	val:
		要写入芯片的值
返回值：
	成功0，否则为负错误号

### 驱动提供sysfs 属

在大多数情况下，驱动不需要提sysfs 属性，因为硬件监控核心会在内部创建这些属性。只需要提额外的非标准 sysfs 属性
头文linux/hwmon-sysfs.h 提供了一些有用的宏来声明和使用硬件监sysfs 属性
在许多情况下，你可以使用现有的定DEVICE_ATTR 或其变体 DEVICE_ATTR_{RW,RO,WO} 来声明此属性。如果一个属性没有额外的上下文，这是可行的。然而，在许多情况下会有附加信息（例如传感器索引需要传递给 sysfs 属性处理函数
SENSOR_DEVICE_ATTR SENSOR_DEVICE_ATTR_2 可用于定义需要此类附加上下文信息的属性SENSOR_DEVICE_ATTR 需要一个附加参数，SENSOR_DEVICE_ATTR_2 需要两个
如果标准的属性权限和函数名可行，应当使用 SENSOR_DEVICE_ATTR SENSOR_DEVICE_ATTR_2 的简变体。标准权限为：SENSOR_DEVICE_ATTR[_2]_RW 0644，SENSOR_DEVICE_ATTR[_2]_RO 0444SENSOR_DEVICE_ATTR[_2]_WO 0200。标准函数类似于 DEVICE_ATTR_{RW,RO,WO}，在所提供的函数名附加 _show _store
SENSOR_DEVICE_ATTR 及其变体定义了一struct sensor_device_attribute

```

	struct sensor_device_attribute {
		struct device_attribute dev_attr;
		int index;
	};

```

你可以使to_sensor_dev_attr 从属性的读或写函数中获取指向此结构的指针。其参数是该属性所附加设备
SENSOR_DEVICE_ATTR_2 及其变体定义了一struct sensor_device_attribute_2

```

	struct sensor_device_attribute_2 {
		struct device_attribute dev_attr;
		u8 index;
		u8 nr;
	};

```

使用 to_sensor_dev_attr_2 获取指向此结构的指针。其参数是该属性所附加的设备