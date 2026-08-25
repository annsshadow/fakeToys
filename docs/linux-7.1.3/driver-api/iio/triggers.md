## 触发器（Triggers

- struct iio_trigger 工业 I/O 触发器设- `devm_iio_trigger_alloc` 资源托管iio_trigger_alloc
- `devm_iio_trigger_register` 资源托管iio_trigger_register
  iio_trigger_unregister
- `iio_trigger_validate_own_device` 检查触发器IIO
  设备是否属于同一个设
在许多情况下，驱动能够基于某些外部事件（触发器）来捕获数据，而不是周期性地轮询
数据，这非常有用。IIO 触发器可以由一个同时拥有基于硬件生成事件（例如数据就绪超过阈值）IIO 设备的设备驱动提供，也可以由一个来自独立中断源（例如连接到某个
外部系统GPIO 线路、定时器中断，或用户空间写入 sysfs 中的某个特定文件）的单独
驱动提供。一个触发器可以为多个传感器发起数据捕获，并且它也可能与传感器本身完无关
## IIO 触发器的 sysfs 接口


sysfs 中与触发器相关的位置有两处：

- `/sys/bus/iio/devices/trigger{Y}/*`，该文件IIO 触发器注册到 IIO 核心时创建，
  对应于索引为 Y 的触发器。由于触发器根据类型可能大不相同，这里只有少数标准属  可以描述
  - `name`，触发器的名称，之后可用于与设备关联  - `sampling_frequency`，某些基于定时器的触发器使用此属性来指定触发调用的频率
- `/sys/bus/iio/devices/iio:device{X}/trigger/*`，该目录在设备支持触发缓冲区  创建。我们可以通过`current_trigger` 文件中写入触发器的名称来将触发器与我们的
  设备关联
## IIO 触发器设

```

      struct iio_trigger_ops trigger_ops = {
          .set_trigger_state = sample_trigger_state,
          .validate_device = sample_validate_device,
      }

      struct iio_trigger *trig;

      /* 首先，为我们的触发器分配内存 */
      trig = iio_trigger_alloc(dev, "trig-%s-%d", name, idx);

      /* 设置触发器操作字*/
      trig->ops = &trigger_ops;

      /* 现在将触发器注册IIO 核心 */
      iio_trigger_register(trig);

```
## IIO 触发ops


- struct iio_trigger_ops iio_trigger 的操作结构体
注意触发器附带了一组操作：

- `set_trigger_state`，按需打开/关闭触发器- `validate_device`，当当前触发器被更改时用于验证设备的函数
## 更多细节


   :export:
