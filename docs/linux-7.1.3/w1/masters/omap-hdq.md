## 面向 omap HDQ/1-wire 模块的内核驱动


## 支持的芯片：

TI OMAP 2430/3430 平台上的 HDQ/1-wire 控制器。

## 关于 HDQ 基础的有用链接：

http://focus.ti.com/lit/an/slua408a/slua408a.pdf

## 描述：

TI OMAP2430/3430 平台上的 HDQ/1-Wire 模块实现了 Benchmark HDQ 与 Dallas Semiconductor 1-Wire 协议的主设备（master）功能的硬件协议。这些协议使用单根线在主机（HDQ/1-Wire 控制器）与从机（HDQ/1-Wire 外部兼容设备）之间进行通信。

HDQ/1-Wire 模块的一个典型应用是与电池监视器（电量计，gas gauge）集成电路通信。

该控制器支持 HDQ 与 1-wire 两种模式运行。HDQ 与 1-wire 模式之间的本质区别在于从设备如何响应初始化脉冲。在 HDQ 模式下，固件不要求主机向从机创建初始化脉冲。不过，可以通过使用初始化脉冲（也称为 break 脉冲）来复位从机。与 1-Wire 协议不同，从机不会以存在脉冲（presence pulse）响应。

## 备注：

该驱动（drivers/w1/masters/omap_hdq.c）支持控制器的 HDQ 模式。在此模式下，由于我们无法读取遵循 W1 规范（family:id:crc）的 ID，可以向驱动传递一个模块参数，用于计算 CRC 并向 W1 核心回传一个合适的从机 ID。

默认情况下，主设备驱动与 BQ 从机接口驱动（drivers/w1/slaves/w1_bq27000.c）将 ID 设为 1。请注意，如有必要，可以用不同的 ID 加载这两个模块，但要注意主设备与从设备驱动加载时使用的 ID 应当相同。

```

  insmod omap_hdq.ko W1_ID=2
  insmod w1_bq27000.ko F_ID=2

```
该驱动也支持 1-wire 模式。在此模式下，无需将从机 ID 作为参数传递。驱动会使用 SEARCH_ROM 过程自动检测连接到总线上的从机。可以通过将 DT 中的 "ti,mode" 属性设为 "1w" 来选择 1-wire 模式（详见 Documentation/devicetree/bindings/w1/omap-hdq.txt）。默认情况下驱动处于 HDQ 模式。
