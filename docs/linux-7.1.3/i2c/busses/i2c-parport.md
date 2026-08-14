## 内核驱动 i2c-parport


作者：Jean Delvare <jdelvare@suse.de>

这是一个用于多种 i2c-over-parallel-port（并行口上的 I2C）适配器的统一驱动，
例如 Philips、Velleman 或 ELV 制造的那些。该驱动意在取代旧有的、各自独立的驱动：

 - i2c-philips-par
 - i2c-elv
 - i2c-velleman
 - video/i2c-parport
   （与此驱动不同，它专用于自制的图文电视 teletext 适配器）

它目前支持以下设备：

 - (type=0) Philips 适配器
 - (type=1) 自制图文电视 teletext 适配器
 - (type=2) Velleman K8000 适配器
 - (type=3) ELV 适配器
 - (type=4) Analog Devices ADM1032 评估板
 - (type=5) Analog Devices 评估板：ADM1025、ADM1030、ADM1031
 - (type=6) Barco LPT->DVI（K5800236）适配器
 - (type=7) One For All JP1 并行口适配器
 - (type=8) VCT-jig

这些设备使用不同的引脚配置，因此必须通过 type 模块参数告诉驱动你所使用的
是什么。无法自动探测设备。如有需要，可以很容易地加入对不同引脚配置的支持。

较早的内核默认使用 type=0（Philips）。但现在，如果缺少 type 参数，驱动将
直接初始化失败。

在那些将 SMBus alert 中断线正确连接到并行口中端引脚的适配器上，可以使用
SMBus alert 支持。


### 构建你自己的适配器


如果你想自己构建 i2c-over-parallel-port 适配器，可参考下面的电路：
```
   Device                                                      PC
   Side          ___________________Vdd (+)                    Side
                  |    |         |
                 ---  ---       ---
                 | |  | |       | |
                 |R|  |R|       |R|
                 | |  | |       | |
                 ---  ---       ---
                  |    |         |
                  |    |    /|   |
   SCL  ----------x--------o |-----------x-------------------  pin 2
                       |    \|   |       |
                       |         |       |
                       |   |\    |       |
   SDA  ----------x----x---| o---x---------------------------  pin 13
                  |        |/            |
                  |                      |
                  |         /|           |
                  ---------o |----------------x--------------  pin 3
                            \|           |    |
                                         |    |
                                        ---  ---
                                        | |  | |
                                        |R|  |R|
                                        | |  | |
                                        ---  ---
                                         |    |
                                        ###  ###
                                        GND  GND

```
说明：
 - 这正是 Analog Devices 评估板所使用的引脚定义与电路。
```
                  /|
                -o |-
                  \|

   必须使用 74HC05，且必须是开集电极（open collector）输出。
 - 所有电阻均为 10k。
 - 并行口的引脚 18-25 连接到 GND。
 - 引脚 4-9（D2-D7）可用作 VDD，由驱动将它们驱动为高电平。
   ADM1032 评估板使用了 D4-D7。注意从并行口可吸取的电流是受限的。
   还需注意，所有相连的线路必须被驱动为相同状态，否则会短路输出缓冲！
   因此，在加载 i2c-parport 模块之后再插入 I2C 适配器可能更安全，因为
   初始化前数据线状态可能是未知的。
 - 这是 5V 电平！
 - 显然你无法读取 SCL（因此它并非真正符合标准）。要加上很容易，只需
   复制 SDA 部分并使用另一个输入引脚即可。这样会得到（ELV 兼容引脚定义）：：


      Device                                                      PC
      Side          ______________________________Vdd (+)         Side
                     |    |            |    |
                    ---  ---          ---  ---
                    | |  | |          | |  | |
                    |R|  |R|          |R|  |R|
                    | |  | |          | |  | |
                    ---  ---          ---  ---
                     |    |            |    |
                     |    |      |\    |    |
      SCL  ----------x--------x--| o---x------------------------  pin 15
                          |   |  |/         |
                          |   |             |
                          |   |   /|        |
                          |   ---o |-------------x--------------  pin 2
                          |       \|        |    |
                          |                 |    |
                          |                 |    |
                          |      |\         |    |
      SDA  ---------------x---x--| o--------x-------------------  pin 10
                              |  |/              |
                              |                  |
                              |   /|             |
                              ---o |------------------x---------  pin 3
                                  \|             |    |
                                                 |    |
                                                ---  ---
                                                | |  | |
                                                |R|  |R|
                                                | |  | |
                                                ---  ---
                                                 |    |
                                                ###  ###
                                                GND  GND


```
如果可能，你应当使用与现有适配器相同的引脚配置，这样就无需修改代码。


### 相似（但不同）的驱动


本驱动与 i2c 软件包中的 i2c-pport 驱动并不相同。i2c-pport 驱动利用现代
并行口的特性，因此不需要额外的电子电路。但它也有其他限制，并且尚未
移植到 Linux 2.6（截至目前）。

本驱动也与 lm_sensors 软件包中的 i2c-pcf-epp 驱动不相同。i2c-pcf-epp 驱动
并不是将并行口直接用作 I2C 总线，而是用它来控制外部的 I2C 总线主设备。
该驱动同样尚未移植到 Linux 2.6（截至目前）。


### Velleman 适配器的历史文档


有用的链接：

- Velleman                http://www.velleman.be/
- Velleman K8000 Howto    http://howto.htlw16.ac.at/k8000-howto.html

该项目催生了用于 Velleman K8000 和 K8005 的新库：

  LIBK8000 v1.99.1 和 LIBK8005 v0.21

借助这些库，你可以使用原始 Velleman 软件中的简单命令，例如
SetIOchannel、ReadADchannel、SendStepCCWFull 等许多命令，来控制 K8000
接口卡和 K8005 步进电机卡，使用 /dev/velleman。

  - http://home.wanadoo.nl/hihihi/libk8000.htm
  - http://home.wanadoo.nl/hihihi/libk8005.htm
  - http://struyve.mine.nu:8080/index.php?block=k8000
  - http://sourceforge.net/projects/libk8005/


### One For All JP1 并行口适配器


JP1 项目围绕一组遥控器展开，这些遥控器通过电池仓中的一个 6 针跳线，
将其内部配置 EEPROM 所连接的 I2C 总线暴露出来。更多细节可参见：

http://www.hifi-remote.com/jp1/

简单的并行口硬件细节可参见：

http://www.hifi-remote.com/jp1/hardware.shtml
