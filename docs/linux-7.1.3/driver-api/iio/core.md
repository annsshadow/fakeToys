## 核心要素


工业 I/O（Industrial I/O，IIO）核心既为编写多种不同类型嵌入式传感器的驱动提供了一个
统一的框架，也为操作用户空间传感器应用程序提供了标准接口。其实现可在
`drivers/iio/industrialio-*` 下找到。

### 工业 I/O 设备


- struct iio_dev - 工业 I/O 设备
- iio_device_alloc() - 从驱动分配一个 `iio_dev`
- iio_device_free() - 从驱动释放一个 `iio_dev`
- iio_device_register() - 向 IIO 子系统注册一个设备
- iio_device_unregister() - 从 IIO 子系统注销一个设备

一个 IIO 设备通常对应于单个硬件传感器，并提供处理该设备的驱动所需的全部信息。
让我们先了解一下嵌入在 IIO 设备中的功能，然后再展示设备驱动如何使用一个 IIO 设备。

用户空间应用程序可以通过两种方式与 IIO 驱动交互。

1. `/sys/bus/iio/devices/iio:device{X}/`，这代表一个硬件传感器，并将同一芯片的数据通道分组在一起。
2. `/dev/iio:device{X}`，用于缓冲数据传输和事件信息获取的字符设备节点接口。

一个典型的 IIO 驱动会将自己注册为 [I2C <../i2c>](I2C <../i2c>) 或
[SPI <../spi>](SPI <../spi>) 驱动，并创建 probe 与 remove 两个例程。

在 probe 时：

1. 调用 iio_device_alloc()，为 IIO 设备分配内存。
2. 用驱动特定的信息（例如设备名、设备通道）初始化 IIO 设备字段。
3. 调用 iio_device_register()，将设备注册到 IIO 核心。在此调用之后，设备即可接受来自用户空间应用程序的请求。

在 remove 时，我们以相反的顺序释放 probe 中分配的资源：

1. iio_device_unregister()，从 IIO 核心注销设备。
2. iio_device_free()，释放为 IIO 设备分配的内存。

## IIO 设备 sysfs 接口


属性是用于暴露芯片信息并允许应用程序设置各种配置参数的 sysfs 文件。对于索引为 X 的
设备，属性可在 /sys/bus/iio/devices/iio:deviceX/ 目录下找到。常见属性包括：

- `name`，对物理芯片的描述。
- `dev`，显示与 `/dev/iio:deviceX` 节点关联的 major:minor 对。
- `sampling_frequency_available`，设备可用的离散采样频率值集合。
- IIO 设备的可用标准属性在 Linux 内核源码的
  :file:Documentation/ABI/testing/sysfs-bus-iio 文件中有描述。

## IIO 设备通道


struct iio_chan_spec - 单个通道的规格说明

一个 IIO 设备通道是对一个数据通道的表示。一个 IIO 设备可以有一个或多个通道。例如：

- 温度计传感器有一个表示温度测量的通道。
- 一个光传感器有两个通道，分别表示可见光与红外光谱的测量值。
- 加速度计最多可有 3 个通道，分别表示 X、Y 与 Z 轴上的加速度。

一个 IIO 通道由 struct iio_chan_spec 描述。
上面示例中温度传感器的温度计驱动将

```

   static const struct iio_chan_spec temp_channel[] = {
        {
            .type = IIO_TEMP,
            .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
        },
   };

```
向用户空间暴露的通道 sysfs 属性以位掩码的形式指定。根据其共享信息的不同，属性可以
设置在以下掩码之一中：

- **info_mask_separate**，属性将特定于该通道
- **info_mask_shared_by_type**，属性由同一类型的所有通道共享
- **info_mask_shared_by_dir**，属性由同一方向的所有通道共享
- **info_mask_shared_by_all**，属性由所有通道共享

当每个通道类型有多个数据通道时，我们有两种方式区分它们：

- 将 `iio_chan_spec` 的 **.modified** 字段设为 1。修饰符通过同一 `iio_chan_spec`
  结构的 **.channel2** 字段指定，用于表示通道的某个物理上唯一的特征，例如其方向或
  光谱响应。例如，一个光传感器可以有两个通道，一个用于红外光，一个用于红外与可见光。
- 将 `iio_chan_spec` 的 **.indexed** 字段设为 1。在这种情况下，该通道只是另一个
  带有由 **.channel** 字段指定的索引的实例。

```

   static const struct iio_chan_spec light_channels[] = {
           {
                   .type = IIO_INTENSITY,
                   .modified = 1,
                   .channel2 = IIO_MOD_LIGHT_IR,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
                   .info_mask_shared = BIT(IIO_CHAN_INFO_SAMP_FREQ),
           },
           {
                   .type = IIO_INTENSITY,
                   .modified = 1,
                   .channel2 = IIO_MOD_LIGHT_BOTH,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
                   .info_mask_shared = BIT(IIO_CHAN_INFO_SAMP_FREQ),
           },
           {
                   .type = IIO_LIGHT,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
                   .info_mask_shared = BIT(IIO_CHAN_INFO_SAMP_FREQ),
           },
      }

```
该通道的定义将为原始数据获取生成两个独立的 sysfs 文件：

- `/sys/bus/iio/devices/iio:device{X}/in_intensity_ir_raw`
- `/sys/bus/iio/devices/iio:device{X}/in_intensity_both_raw`

一个用于处理后数据的文件：

- `/sys/bus/iio/devices/iio:device{X}/in_illuminance_input`

以及一个用于采样频率的共享 sysfs 文件：

- `/sys/bus/iio/devices/iio:device{X}/sampling_frequency`。

```

   static const struct iio_chan_spec light_channels[] = {
           {
                   .type = IIO_VOLTAGE,
		   .indexed = 1,
		   .channel = 0,
		   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
	   },
           {
	           .type = IIO_VOLTAGE,
                   .indexed = 1,
                   .channel = 1,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
           },
   }

```
这将为原始数据获取生成两个独立的属性文件：

- `/sys/bus/iio/devices/iio:device{X}/in_voltage0_raw`，表示通道 0 的电压测量值。
- `/sys/bus/iio/devices/iio:device{X}/in_voltage1_raw`，表示通道 1 的电压测量值。

## 更多细节

   :export:
