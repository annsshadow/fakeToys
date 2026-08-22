
## 内核驱动 sht4x


支持的设备：

  - Sensirion SHT4X

    Prefix: 'sht4x'

    Addresses scanned: None

    Datasheet:

      English: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/2_Humidity_Sensors/Datasheets/Sensirion_Humidity_Sensors_SHT4x_Datasheet.pdf

Author: Navin Sankar Velliangiri <navin@linumiz.com>


### 描述


本驱动实现对 Sensirion SHT4x 芯片（一款湿度与温度传感器）的支持。温度以摄氏度度量，相对湿度以百分比表示。在 sysfs 接口中，所有数值均乘以 1000，即 31.5 摄氏度对应的值为 31500
### 使用说明


该器件通过 I2C 协议通信。传感器可使I2C 地址 0x44。关于实例化该器件的方法，请参阅 Documentation/i2c/instantiating-devices.rst
### Sysfs 条目


=============== ============================================
temp1_input     测量的温度，单位毫摄氏度
humidity1_input 测量的湿度，单位 %H
update_interval 轮询传感器的最小间隔，单位毫秒。可写。必须至少为 2000heater_power	请求的加热器功率，单位毫瓦		可用值：201000（默认：200）heater_time	请求的加热器工作时间，单位毫秒		可用值：100000（默1000）heater_enable	以所选功率、在所选时间内启用加热器，以去除传感器表面的冷凝水。一旦启用便无法手动关闭（完成操作后自动关闭）
   - 0: 关闭（只读值）
   - 1: 开=============== ============================================
