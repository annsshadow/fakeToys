## 内核驱动 i2c-ismt


支持的适配器：
  - Intel S12xx 系列 SOCs

作者：
	Bill Brown <bill.e.brown@intel.com>


### 模块参数


- bus_speed（无符号整型
用于更改总线速度。通常总线速度BIOS 设定，无需更改。但在调试期间，某些 SMBus 分析器速度过慢，无法监测总线，因此需要此模块参数。请kHz 为单位指定总线速度
可用的总线频率设置
  ====   =========
  0      无变  80     kHz
  100    kHz
  400    kHz
  1000   kHz
  ====   =========


### 描述


S12xx 系列 SOCs 集成了一SMBus 2.0 控制器，主要面向微服务器与存储市场
S12xx 系列包含一PCI functions。lspci 的输出将显示```
  00:13.0 System peripheral: Intel Corporation Centerton SMBus 2.0 Controller 0
  00:13.1 System peripheral: Intel Corporation Centerton SMBus 2.0 Controller 1
```
