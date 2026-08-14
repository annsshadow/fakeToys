## 内核驱动 lp3944


  - National Semiconductor LP3944 Fun-light 芯片

    Prefix: 'lp3944'

    Addresses scanned: 无（见下文说明部分）

    Datasheet:

	Publicly available at the National Semiconductor website
	http://www.national.com/pf/LP/LP3944.html

Authors:
	Antonio Ospite <ospite@studenti.unina.it>


### 描述

LP3944 是一个辅助芯片，可驱动多达 8 个 LED，具有两种可编程 DIM 模式；它甚至可以用作 gpio 扩展器，但本驱动假定它被用作 LED 控制器。

DIM 模式用于为 LED 设置 _闪烁_ 模式，该模式通过提供两个参数来指定：

  - period（周期）：
	从 0s 到 1.6s
  - duty cycle（占空比）：
	LED 点亮时间占周期的百分比，从 0 到 100

将 LED 设置为 DIM0 或 DIM1 模式会使其按照该模式闪烁。详见数据手册。

LP3944 可在 Motorola A910 智能手机中找到，它驱动 rgb LED、相机闪光灯以及 lcd 的电源。


### 说明

该芯片主要用于嵌入式环境，因此本驱动期望它通过 i2c_board_info 机制注册。

要在适配器 0 的地址 0x60 处注册该芯片，请设置平台数据
```

	static struct i2c_board_info a910_i2c_board_info[] __initdata = {
		{
			I2C_BOARD_INFO("lp3944", 0x60),
			.platform_data = &a910_lp3944_leds,
		},
	};

```
```

	i2c_register_board_info(0, a910_i2c_board_info,
			ARRAY_SIZE(a910_i2c_board_info));

```
