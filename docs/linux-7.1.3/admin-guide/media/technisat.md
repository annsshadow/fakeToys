
## 如何设置 Technisat/B2C2 Flexcop 设备


   本文档已过时。

Author: Uwe Bugla <uwe.bugla@gmx.de> August 2009

### 确认你拥有的设备


重要提示：该驱动不支持 Technisat USB 2 设备！

首先用发行版自带的内核启动你的 Linux 主机：


	lspci -vvv （对 PCI 设备）或 lsusb -vvv （对 USB 设备）会显示例如：
	02:0b.0 Network controller: Techsan Electronics Co Ltd B2C2 FlexCopII DVB chip /
	Technisat SkyStar2 DVB card (rev 02)

	dmesg | grep frontend 可能显示例如：
	DVB: registering frontend 0 (Conexant CX24123/CX24109)...

### 内核编译：


如果 Flexcop / Technisat 是你主机中唯一的 DVB / TV / Radio 设备，请去掉不必要的模块并选中以下项：

`Multimedia support` => `Customise analog and hybrid tuner modules to build`

在此目录中取消勾选其中所有已激活的驱动（除用于第三代 ATSC 的 `Simple tuner support` 外 —— 见情形 9）。

然后请激活：

- 主模块部分：

  `Multimedia support` => `DVB/ATSC adapters` => `Technisat/B2C2 FlexcopII(b) and FlexCopIII adapters`

  #) => `Technisat/B2C2 Air/Sky/Cable2PC PCI` （PCI 卡）或
  #) => `Technisat/B2C2 Air/Sky/Cable2PC USB` （USB 1.1 适配器）
     以及用于故障排除：
  #) => `Enable debug for the B2C2 FlexCop drivers`

- 前端 / 调谐器 / 解调器模块部分：

  `Multimedia support` => `DVB/ATSC adapters`
   => `Customise the frontend modules to build` `Customise DVB frontends` =>

  - SkyStar DVB-S 修订版 2.3：

    #) => `Zarlink VP310/MT312/ZL10313 based`
    #) => `Generic I2C PLL based tuners`

  - SkyStar DVB-S 修订版 2.6：

    #) => `ST STV0299 based`
    #) => `Generic I2C PLL based tuners`

  - SkyStar DVB-S 修订版 2.7：

    #) => `Samsung S5H1420 based`
    #) => `Integrant ITD1000 Zero IF tuner for DVB-S/DSS`
    #) => `ISL6421 SEC controller`

  - SkyStar DVB-S 修订版 2.8：

    #) => `Conexant CX24123 based`
    #) => `Conexant CX24113/CX24128 tuner for DVB-S/DSS`
    #) => `ISL6421 SEC controller`

  - AirStar DVB-T 卡：

    #) => `Zarlink MT352 based`
    #) => `Generic I2C PLL based tuners`

  - CableStar DVB-C 卡：

    #) => `ST STV0297 based`
    #) => `Generic I2C PLL based tuners`

  - AirStar ATSC 卡第一代：

    #) => `Broadcom BCM3510`

  - AirStar ATSC 卡第二代：

    #) => `NxtWave Communications NXT2002/NXT2004 based`
    #) => `Generic I2C PLL based tuners`

  - AirStar ATSC 卡第三代：

    #) => `LG Electronics LGDT3302/LGDT3303 based`
    #) `Multimedia support` => `Customise analog and hybrid tuner modules to build` => `Simple tuner support`
