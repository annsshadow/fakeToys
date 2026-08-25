## 内核驱动 ftsteutates


支持的芯片：

  - FTS Teutates

    Prefix: 'ftsteutates'

    Addresses scanned: I2C 0x73 (7-Bit)

Author: Thilo Cestonaro <thilo.cestonaro@ts.fujitsu.com>


### 描述


BMC Teutates Superior System 监控和热管理的第十一代解决方案。它建立BMC Theseus 的基本功能之上，并包含若干新特性和增强。它可以监控最4 路电压6 个温度和 8 个风扇。它还包含一个当前由该驱动实现的集成看门狗

`pwmX_auto_channels_temp` 属性显示当前是哪个温度传感器在驱动哪个风扇通道。该值可能会在运行时动态变化，取决于风扇控制电路所选的温度传感器

4 路电压需要板级特定的倍乘系数，因BMC 只能测量最3.3V 的电压，因此依赖于分压器。详情请参阅你的主板手册

要清除温度或风扇报警，请执行以下命令
```
	echo 0 >XXXX_alarm
```
该芯片的规格书可以在 `Kontron FTP Server <http://ftp.kontron.com/>`_（用户名 = "anonymous"，无需密码）下找到，位于以下路径：

  /Services/Software_Tools/Linux_SystemMonitoring_Watchdog_GPIO/BMC-Teutates_Specification_V1.21.pdf
