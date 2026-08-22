## 缓冲区（Buffers

- struct iio_buffer 通用缓冲区结- `iio_validate_scan_mask_onehot` 校验是否恰好选中了一个通道
- `iio_buffer_get` 获取对缓冲区的引- `iio_buffer_put` 释放对缓冲区的引
Industrial I/O 核心提供了一种基于触发源（trigger source）进行连续数据采集的方式。可以从 `/dev/iio:device{X}` 字符设备节点一次性读取多个数据通道，从而降CPU 负载
## IIO 缓冲sysfs 接口

一IIO 缓冲区在 `/sys/bus/iio/devices/iio:device{X}/buffer/*` 下有一个关联的 attributes 目录。以下是一些已有属性：

- `length`，缓冲区可存储的数据样本总数（容量）- `enable`，激活缓冲区采集
## IIO 缓冲区设

与放入缓冲区中的某次通道读取相关的元信息称为扫描元素（scan element）。配置扫描元素的重要位通过 `/sys/bus/iio/devices/iio:device{X}/scan_elements/` 目录暴露给用户空间应用程序。该目录包含以下形式的属性：

- `enable`，用于启用某个通道。当且仅当其属性非 *** 时，触发式采集才会包含该通道的数据样本- `index`，该通道scan_index- `type`，描述扫描元素数据在缓冲区中的存储方式，以及因此从用户空间读取它的形式  格式[be|le]:[s|u]bits/storagebits[Xrepeat][>>shift] 
  - **be** **le**，指定大端或小端  - **s** **u**，指定有符号（补码）或无符号  - **bits**，是有效数据位数  - **storagebits**，是数据在缓冲区中占据的位数（含填充）  - **repeat**，指bits/storagebits 的重复次数。当 repeat 元素0 1 时，省略 repeat 值  - **shift**，若指定，则是在屏蔽掉未使用位之前需要应用的移位
例如，一12 位分辨率3 轴加速度计驱动，其中
```

        7   6   5   4   3   2   1   0
      +---+---+---+---+---+---+---+---+
      |D3 |D2 |D1 |D0 | X | X | X | X | (LOW byte, address 0x06)
      +---+---+---+---+---+---+---+---+

        7   6   5   4   3   2   1   0
      +---+---+---+---+---+---+---+---+
      |D11|D10|D9 |D8 |D7 |D6 |D5 |D4 | (HIGH byte, address 0x07)
      +---+---+---+---+---+---+---+---+

```
```

      $ cat /sys/bus/iio/devices/iio:device0/scan_elements/in_accel_y_type
      le:s12/16>>4

```
用户空间应用程序会把从缓冲区读取的数据样本解释为两字节小端有符号数据，需要在屏蔽12 位有效数据之前先右移 4 位
为实现缓冲区支持，驱动应初始化以下内```

   struct iio_chan_spec {
   /* other members */
           int scan_index
           struct {
                   char sign;
                   u8 realbits;
                   u8 storagebits;
                   u8 shift;
                   u8 repeat;
                   enum iio_endian endianness;
                  } scan_type;
          };

```
上述加速度计的驱动将具```

   struct iio_chan_spec accel_channels[] = {
           {
                   .type = IIO_ACCEL,
		   .modified = 1,
		   .channel2 = IIO_MOD_X,
		   /* other stuff here */
		   .scan_index = 0,
		   .scan_type = {
		           .sign = 's',
			   .realbits = 12,
			   .storagebits = 16,
			   .shift = 4,
			   .endianness = IIO_LE,
		   },
           }
           /* similar for Y (with channel2 = IIO_MOD_Y, scan_index = 1)
            * and Z (with channel2 = IIO_MOD_Z, scan_index = 2) axis
            */
    }

```
此处 **scan_index** 定义了已启用通道在缓冲区内放置的顺序。较低的 **scan_index** 的通道会被放在较高索引的通道之前。每个通道都需要有唯一**scan_index**
**scan_index** 设为 -1 可用于表示该特定通道不支持缓冲采集。这种情况下，scan_elements 目录中不会为该通道创建任何条目
## 更多细节（More details
   :export:
