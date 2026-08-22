## spi_lm70llp：LM70-LLP 并口SPI 适配

支持的板芯片
  - National Semiconductor LM70 LLP 评估
    数据手册: https://www.ti.com/lit/gpn/lm70

作
        Kaiwan N Billimoria <kaiwan@designergraphix.com>

### 描述

该驱动提供粘合代码，National Semiconductor LM70 LLP 温度传感器评估板连接到内核的 SPI 核心子系统
这是一SPI 主控制器（master controller）驱动。它可以与（作为下层）LM70 逻辑驱动（一个“SPI 协议驱动”）配合使用。实际上，该驱动将评估板上的并行端口接口转换为一条带单个设备SPI 总线，该设备将由通用 LM70 驱动（drivers/hwmon/lm70.c）驱动
### 硬件接口

此特定板卡（LM70EVAL-LLP）的原理图（4 页）可在此处获取
  https://download.datasheets.com/pdfs/documentation/nat/kit&board/lm70llpevalmanual.pdf

LM70 LLP 评估板上的硬件接口如下：

   ======== == =========   ==========
   并行端口              LM70 LLP
     端口    .  方向      JP2 排针
   ======== == =========   ==========
      D0     2      -         -
      D1     3     -->      V+   5
      D2     4     -->      V+   5
      D3     5     -->      V+   5
      D4     6     -->      V+   5
      D5     7     -->      nCS  8
      D6     8     -->      SCLK 3
      D7     9     -->      SI/O 5
     GND    25      -       GND  7
   Select  13     <--      SI/O 1
   ======== == =========   ==========

注意，由LM70 使用 SPI 的 线”变体，SI/SO 引脚通过一种让并口LM70 任一拉低该引脚的接法，同时连接到引脚 D7（作为主Master Out）和 Select（作为主Master In）。这不能与普SPI 设备共享，但其他 3 线设备可能共享同一SI/SO 引脚
该驱动中bitbanger 例程（lm70_txrx）由其绑定的“hwmon/lm70”协议驱动通过 sysfs 钩子，使spi_write_then_read() 调用回调。它执行 Mode 0（SPI/Microwire）位脉冲（bitbanging）。然lm70 驱动解释所得的数字温度值并通过 sysfs 导出
一个“陷阱（gotcha）”：National Semiconductor LM70 LLP 评估板电路原理图显示，来LM70 芯片SI/O 线连接到晶体Q1 的基极（还有一个上拉电阻，以及一个到 D7 的齐纳二极管）；而集电极接到 VCC
解释该电路：LM70 SI/O 线为高电平（或三态且未被主机通过 D7 拉低）时，晶体管导通并将集电极切换为零，这反映DB25 并口连接器的引脚 13 上。另一方面，当 SI/O 为低电平（由 LM70 或主机驱动）时，晶体管截止，接在其集电极上的电压作为高电平反映在引脚 13 上
因此：该驱动中的 getmiso 内联例程考虑了这一事实，对引脚 13 读取的值进行取反
### 致谢


- David Brownell，感谢其SPI 侧驱动开发上的指导- Dr.Craig Hollabaugh，感谢其（早期）的“手动”位脉冲驱动版本- Nadir Billimoria，感谢其在解释电路原理图上的帮助