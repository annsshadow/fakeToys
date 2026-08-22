
## saa7134 驱动

本文面向驱动开发者，记录 saa7134 多媒体视频采集芯片驱动的实现细节，涵盖不同芯片型号（saa7130/7133/7134/7135）的差异、晶振配置，以及 LifeView 等板卡的 GPIO 接线说明



Author Gerd Hoffmann


### 芯片型号差异


电视卡可以使用以下两种晶振（xtal）之一

- 32.11 MHz -> .audio_clock=0x187de7
- 24.576MHz -> .audio_clock=0x200000 (xtal * .audio_clock = 51539600)

关于 30/34/35 的一些细节：

- saa7130 - 低价芯片，没有静音功能，因此所有这
  卡的 tuner 结构体中应定.mute 字段

- saa7134 - 常见芯片

- saa7133/35 - saa7135 可能是一个市场决策，因为所有这
  芯片pci 上标识自身为 33

### LifeView GPIO


本节Peter Missel <peter.missel@onlinehome.de> 撰写

- LifeView FlyTV Platinum FM (LR214WF)

    - GP27    MDT2005 PB4 pin 10
    - GP26    MDT2005 PB3 pin 9
    - GP25    MDT2005 PB2 pin 8
    - GP23    MDT2005 PB1 pin 7
    - GP22    MDT2005 PB0 pin 6
    - GP21    MDT2005 PB5 pin 11
    - GP20    MDT2005 PB6 pin 12
    - GP19    MDT2005 PB7 pin 13
    - nc      MDT2005 PA3 pin 2
    - Remote  MDT2005 PA2 pin 1
    - GP18    MDT2005 PA1 pin 18
    - nc      MDT2005 PA0 pin 17 strap low
    - GP17    Strap "GP7"=High
    - GP16    Strap "GP6"=High

 - 0=Radio 1=TV
 - 驱动 SA630D ENCH1 HEF4052 A1 引脚，通过
	  SIF 输入实现 FM 收音

    - GP15    nc
    - GP14    nc
    - GP13    nc
    - GP12    Strap "GP5" = High
    - GP11    Strap "GP4" = High
    - GP10    Strap "GP3" = High
    - GP09    Strap "GP2" = Low
    - GP08    Strap "GP1" = Low
    - GP07.00 nc

### 致谢


andrew.stevens@philips.com + werner.leeb@philips.com 鎻愪緵浜。
saa7134 硬件规格和样例板

