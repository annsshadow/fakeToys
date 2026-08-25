
## lm3556 内核驱动


- Texas Instrument  1.5 A 同步升压 LED 闪光灯驱动，带高边电流源
- 数据手册：http://www.national.com/ds/LM/LM3556.pdf

作者：
      - Daniel Jeong

	联系方式：Daniel Jeong(daniel.jeong-at-ti.com, gshark.jeong-at-gmail.com)

### 描述

LM3556 3 种功能：闪光（Flash）、手电（Torch）和指示（Indicator）
##### 闪光模式


在闪光模式下，LED 电流源（LED）提16 个目标电流等级，93.75 mA 1500 mA。闪光电流通过 CURRENT CONTROL REGISTERx09）调节。闪光模式由 ENABLE REGISTERx0A）激活，或通过拉高 STROBE 引脚激活
LM3556 闪光可通过 /sys/class/leds/flash/brightness 文件控制

- STROBE 引脚已使能，以下示例仅控制亮度，
  ON/OFF STROBE 引脚控制
闪光示例
```

	#echo 0 > /sys/class/leds/flash/brightness

```
```

	#echo 1 > /sys/class/leds/flash/brightness

```
...
```

	#echo 16 > /sys/class/leds/flash/brightness

```
##### 手电模式


在手电模式下，电流源（LED）通过 CURRENT CONTROL REGISTERx09）编程。手电模式由 ENABLE REGISTERx0A）或硬件 TORCH 输入激活
LM3556 手电可通过 /sys/class/leds/torch/brightness 文件控制- TORCH 引脚已使能，以下示例仅控制亮度，
  ON/OFF TORCH 引脚控制
手电示例
```

	#echo 0 > /sys/class/leds/torch/brightness

```
```

	#echo 1 > /sys/class/leds/torch/brightness

```
...
```

	#echo 8 > /sys/class/leds/torch/brightness

```
##### 指示模式


指示模式可通过 /sys/class/leds/indicator/pattern 文件设置，indicator_pattern 数组中预定义4 种模式
根据 N-blank、脉冲时间和 N 周期的取值，会生成不同的模式。如果你想为自有设备定义新模式，请使用自己的值修indicator_pattern 数组INDIC_PATTERN_SIZE
有关 N-blank、脉冲时间和 N 周期的更多细节，请参阅数据手册
指示模式示例
```

	#echo 0 > /sys/class/leds/indicator/pattern

```
...
```

	#echo 3 > /sys/class/leds/indicator/pattern

```
指示亮度可通过 sys/class/leds/indicator/brightness 文件控制
示例
```

	#echo 0 > /sys/class/leds/indicator/brightness

```
```

	#echo 1 > /sys/class/leds/indicator/brightness

```
...
```

	#echo 8 > /sys/class/leds/indicator/brightness

```
### 注意事项

驱动期望通过 i2c_board_info 机制注册。要在特定适配器上以地址 0x63 注册该芯片，请根include/linux/platform_data/leds-lm3556.h 设置平台数据，设i2c 板信
```

	static struct i2c_board_info board_i2c_ch4[] __initdata = {
		{
			 I2C_BOARD_INFO(LM3556_NAME, 0x63),
			 .platform_data = &lm3556_pdata,
		 },
	};

```
并在平台 init 函数中注册它

```

	board_register_i2c_bus(4, 400,
				board_i2c_ch4, ARRAY_SIZE(board_i2c_ch4));

```
