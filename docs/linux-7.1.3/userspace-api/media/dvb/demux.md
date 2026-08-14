
# 数字电视 Demux 设备


数字电视 demux 设备控制数字电视的 MPEG-TS 过滤器。如果驱动与硬件支持，这些过滤器
由硬件实现；否则由内核提供软件模拟。

可通过 `/dev/adapter?/demux?` 访问。在应用程序中包含 `linux/dvb/dmx.h` 即可获取
数据类型与 ioctl 定义。


- [dmx_types](dmx_types)
- [dmx_fcalls](dmx_fcalls)
