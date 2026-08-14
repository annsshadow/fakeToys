


######## ioctl MEDIA_IOC_DEVICE_INFO


## 名称


MEDIA_IOC_DEVICE_INFO - 查询设备信息

## 概要



`int ioctl(int fd, MEDIA_IOC_DEVICE_INFO, struct media_device_info *argp)`

## 参数



`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向结构体 `media_device_info` 的指针。

## 描述


所有媒体设备都必须支持 `MEDIA_IOC_DEVICE_INFO` ioctl。要查询设备信息，应用程序以指向结构体 `media_device_info` 的指针调用该 ioctl。驱动填充该结构并将信息返回给应用程序。该 ioctl 永远不会失败。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  char
       - `driver`\ [^16^]
       - 实现媒体 API 的驱动名称，为 NUL 结尾的 ASCII 字符串。驱动版本存储在
	  `driver_version` 字段中。

	  特定于驱动的应用程序可以使用此信息来验证驱动身份。它也有助于规避已知缺陷，
	  或在错误报告中识别驱动。

    - -  char
       - `model`\ [^32^]
       - 设备型号名称，为 NUL 结尾的 UTF-8 字符串。设备版本存储在 `device_version`
	  字段中，且不附加到型号名称之后。

    - -  char
       - `serial`\ [^40^]
       - 序列号，为 NUL 结尾的 ASCII 字符串。

    - -  char
       - `bus_info`\ [^32^]
       - 设备在系统中的位置，为 NUL 结尾的 ASCII 字符串。这包括总线类型名称
	  （PCI、USB 等）以及总线特定的标识符。

    - -  __u32
       - `media_version`
       - 媒体 API 版本，使用 `KERNEL_VERSION()` 宏格式化。

    - -  __u32
       - `hw_revision`
       - 硬件设备修订号，采用驱动特定的格式。

    - -  __u32
       - `driver_version`
       - 媒体设备驱动版本，使用 `KERNEL_VERSION()` 宏格式化。与 `driver` 字段一起
	  用于标识特定的驱动。

    - -  __u32
       - `reserved`\ [^31^]
       - 保留以备将来扩展。驱动和应用程序都必须将该数组置零。

`serial` 与 `bus_info` 字段可用于区分多个其他方面相同的硬件实例。在提供序列号时，序列号优先，且可假定为唯一。如果序列号为空字符串，则可改用 `bus_info` 字段。`bus_info` 字段保证唯一，但可能在重启或设备拔插之间变化。

## 返回值



成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误码 <gen-errors> 一章中描述。
