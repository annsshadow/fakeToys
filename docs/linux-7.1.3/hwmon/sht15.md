## 内核驱动 sht15


Authors:

  - Wouter Horre
  - Jonathan Cameron
  - Vivien Didelot <vivien.didelot@savoirfairelinux.com>
  - Jerome Oufella <jerome.oufella@savoirfairelinux.com>

支持的设备：

  - Sensirion SHT10

    Prefix: 'sht10'

  - Sensirion SHT11

    Prefix: 'sht11'

  - Sensirion SHT15

    Prefix: 'sht15'

  - Sensirion SHT71

    Prefix: 'sht71'

  - Sensirion SHT75

    Prefix: 'sht75'

Datasheet: 可在 Sensirion 网站公开获取

	http://www.sensirion.ch/en/pdf/product_information/Datasheet-humidity-sensor-SHT1x.pdf

### 描述


SHT10、SHT11、SHT15、SHT71 SHT75 是湿度与温度传感器
这些器件使用两条 GPIO 线进行通信
支持的测量分辨率为温14 位、湿12 位，或温12 位、湿8 位
湿度校准系数被烧录在芯片OTP 存储器中。这些系数用于对内来自传感器信号进行内部校准。禁用这些系数的重新加载可以为每次测量节10ms 并降功耗，但会损失精度
一些选项可以通过 sysfs 属性设置
注意  - 调节器电源名称被设置“vcc”  - 如果 CRC 校验失败，会发送一个软复位命令，将状态寄存器重置为其硬件
    默认值，但驱动会尝试恢复先前的设备配置
### 平台数据


- checksum  设为 true 以启用读数的 CRC 校验（默认为 false）- no_otp_reload  指示不从 OTP 重新加载的标志（默认false）- low_resolution  指示要使用的温度/湿度分辨率的标志（默认为 false）
### Sysfs 接口


================== ==========================================================
temp1_input        温度输入
humidity1_input    湿度输入
heater_enable      向该属性写1 以启用片内加热器，写0 以禁用		    注意不要将加热器启用过久temp1_fault        若为 1，表示电压过低（低于 2.47V），测量可能无效humidity1_fault    temp1_fault================== ==========================================================
