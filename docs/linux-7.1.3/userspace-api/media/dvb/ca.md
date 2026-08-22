
# 数字电视 CA 设备


数字电视 CA 设备控制条件接收（conditional access）硬件。可通过 `/dev/dvb/adapter/ca` 访问
在应用程序中包含 `linux/dvb/ca.h` 即可获取数据类型ioctl 定义


   There are three ioctls at this API that aren't documented:
   CA_GET_MSG, CA_SEND_MSG and CA_SET_DESCR.
   Documentation for them are welcome.

- [ca_data_types](ca_data_types)
- [ca_function_calls](ca_function_calls)
- [ca_high_level](ca_high_level)
