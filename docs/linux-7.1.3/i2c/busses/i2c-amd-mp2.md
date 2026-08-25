## 内核驱动 i2c-amd-mp2


支持的适配器：
  - AMD MP2 PCIe interface

数据手册：未公开

作者：
 - Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
 - Nehal Shah <nehal-bakulchandra.shah@amd.com>
 - Elie Morisse <syniurge@gmail.com>

### 描述


MP2 是一个被编程I2C 控制器的 ARM 处理器，通过 PCI x86 主机通信

```

  03:00.7 MP2 I2C controller: Advanced Micro Devices, Inc. [AMD] Device 15e6

```
出现在你`lspci -v` 输出中，则该驱动适用于你的设备
