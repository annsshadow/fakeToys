
## 内核驱动 gigabyte_waterforce


支持的设备：

- Gigabyte AORUS WATERFORCE X240
- Gigabyte AORUS WATERFORCE X280
- Gigabyte AORUS WATERFORCE X360

作者：Aleksa Savic

### 描述


该驱动为所列的 Gigabyte Waterforce 一体式 CPU 水冷散热器提供硬件监控支持可用传感器包括以 RPM 为单位的泵与风扇转速，以及冷却液温度。通过 debugfs 可获取固件版本
连接风扇是可选的，并允许从设备控制风扇。如果未连接，与风扇相关的传感器报告零值
可寻址 RGB LED LCD 屏幕不受本驱动支持，应通过用户空间工具控制
### 使用说明


由于这些USB HID，驱动可由内核自动加载，并支持热插拔
### Sysfs 条目


=========== =============================================
fan1_input  风扇转速（RPMfan2_input  泵转速（RPMtemp1_input 冷却液温度（毫摄氏度=========== =============================================

### Debugfs 条目


================ =======================
firmware_version 设备固件版本
================ =======================
