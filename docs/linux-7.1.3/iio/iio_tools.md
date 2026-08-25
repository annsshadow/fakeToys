## IIO 接口工具


## 1. Linux 内核工具


Linux 内核提供了一些可用于IIO sysfs 获取数据的用户空间工具：

- lsiio：提IIO 设备与触发器列表的示例应
- iio_event_monitor：从 IIO 设备读取事件并打印的示例应用
- iio_generic_buffer：从缓冲区读取数据的示例应用
- iio_utils：一组通常用于访问 sysfs 文件API

## 2. LibIIO


LibIIO 是一C/C++ 库，提供IIO 设备的通用访问。该库抽象了硬件的低层细节，并提供了一套简单而完整的编程接口，可用于高级项目

有关 LibIIO 的更多信息，请参见：
https://github.com/analogdevicesinc/libiio
