## LP5521/LP5523/LP55231/LP5562/LP8501 公共驱动


作者：Milo(Woogyom) Kim <milo.kim@ti.com>

### 描述


LP5521、LP5523/55231、LP5562 LP8501 具有以下共同特性：

  通过 I2C 访问寄存  设备的初始化/反初始化
  为多个输出通道创建 LED 类设  用于用户空间接口的设备属  用于运行 LED 模式的程序存储器

LP55xx 公共驱动使用导出的函数提供这些特性
  lp55xx_init_device() / lp55xx_deinit_device()
  lp55xx_register_leds() / lp55xx_unregister_leds()
  lp55xx_regsister_sysfs() / lp55xx_unregister_sysfs()

（驱动结构数据）

lp55xx 公共驱动中，使用了两种不同的数据结构
- lp55xx_led
    控制多输LED 通道，例LED 电流、通道索引- lp55xx_chip
    通用芯片控制，例I2C 和平台数据
例如，LP5521 最多有 3 LED 通道```

  lp55xx_chip for LP5521 ... lp55xx_led #1
			     lp55xx_led #2
			     lp55xx_led #3

  lp55xx_chip for LP5523 ... lp55xx_led #1
			     lp55xx_led #2
				   .
				   .
			     lp55xx_led #9

```
（依赖芯片的代码
为了支持设备特定的配置，使用了一个特殊结‘lpxx_device_config’
  - 最大通道  - 复位命令、芯片使能命  - 芯片特定初始  - 亮度控制寄存器访  - 设置 LED 输出电流
  - 用于运行模式的程序存储器地址访问
  - 附加的设备特定属
（固件接口）

LP55xx 系列设备拥有用于运行各种 LED 模式的内部程序存储器
此模式数据作为文件保存在用户空间，或者通过 I2C 将十六进制字节串写入存储器
LP55xx 公共驱动支持固件接口
LP55xx 芯片有三个程序引擎
要加载并运行模式，编程顺序如下
  (1) 选择一个引擎号/2/3  (2) 模式切换load（加载）
  (3) 将模式数据写入所选区  (4) 模式切换run（运行）

LP55xx 公共驱动提供如下简单接口
select_engine:
	选择用于运行程序的引run_engine:
	启动通过固件接口加载的程firmware:
	加载程序数据

LP5523 的情况下，还需要一个命‘enginex_leds’。它用于在每个引擎号上选择 LED
输出。更多细节请参阅 ‘leds-lp5523.txt’
```

	echo 1 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp5521/loading
	echo "4000600040FF6000" > /sys/class/firmware/lp5521/data
	echo 0 > /sys/class/firmware/lp5521/loading
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
例如，在 LP55231 的引#3 中运行闪烁模
```

	echo 3 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp55231/loading
	echo "9d0740ff7e0040007e00a0010000" > /sys/class/firmware/lp55231/data
	echo 0 > /sys/class/firmware/lp55231/loading
	echo "000001100" > /sys/bus/i2c/devices/xxxx/engine3_leds
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
```

	for idx in 2 3
	do
	echo $idx > /sys/class/leds/red/device/select_engine
	sleep 0.1
	echo 1 > /sys/class/firmware/lp5521/loading
	echo "4000600040FF6000" > /sys/class/firmware/lp5521/data
	echo 0 > /sys/class/firmware/lp5521/loading
	done
	echo 1 > /sys/class/leds/red/device/run_engine

```
这是 LP5523 的另一个示例
```

	echo 2 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp5523/loading
	echo "9d80400004ff05ff437f0000" > /sys/class/firmware/lp5523/data
	echo 0 > /sys/class/firmware/lp5523/loading
	echo "111111111" > /sys/bus/i2c/devices/xxxx/engine2_leds
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
一‘loading被设0，注册的回调就会被调用。在回调内部，所选引擎被加载，存储器
被更新。要运行已编程的模式，应启用 ‘run_engine属性
LP8501 的模式顺序与 LP5523 类似
不过模式数据是特定的
```

	echo 1 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp8501/loading
	echo "9d0140ff7e0040007e00a001c000" > /sys/class/firmware/lp8501/data
	echo 0 > /sys/class/firmware/lp8501/loading
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
```

	echo 2 > /sys/bus/i2c/devices/xxxx/select_engine
	sleep 1
	echo 1 > /sys/class/firmware/lp8501/loading
	echo "9d0140ff7e0040007e00a001c000" > /sys/class/firmware/lp8501/data
	echo 0 > /sys/class/firmware/lp8501/loading
	sleep 1
	echo 3 > /sys/bus/i2c/devices/xxxx/select_engine
	sleep 1
	echo 1 > /sys/class/firmware/lp8501/loading
	echo "9d0340ff7e0040007e00a001c000" > /sys/class/firmware/lp8501/data
	echo 0 > /sys/class/firmware/lp8501/loading
	sleep 1
	echo 1 > /sys/class/leds/d1/device/run_engine

```
（‘run_engine‘firmware_cb’）

运行程序数据的顺序是通用的
但每个设备都有自己特定的命令寄存器地址
为此，‘run_engine‘firmware_cb在每个驱动中是可配置的
run_engine:
	控制所选引firmware_cb:
	固件加载完成后的回调函数
	用于加载和更新程序存储器的芯片特定命令
（预定义模式数据
如果没有固件接口，LP55xx 驱动提供另一种加LED 模式的方法。那就是“预定义”模式
预定义模式定义在平台数据中，并在需要时通过 sysfs 加载它（或它们）
要使用预定义模式的概念，应配‘patterns‘num_patterns’
```

  /* mode_1: blinking data */
  static const u8 mode_1[] = {
		0x40, 0x00, 0x60, 0x00, 0x40, 0xFF, 0x60, 0x00,
		};

  /* mode_2: always on */
  static const u8 mode_2[] = { 0x40, 0xFF, };

  struct lp55xx_predef_pattern board_led_patterns[] = {
	{
		.r = mode_1,
		.size_r = ARRAY_SIZE(mode_1),
	},
	{
		.b = mode_2,
		.size_b = ARRAY_SIZE(mode_2),
	},
  }

  struct lp55xx_platform_data lp5562_pdata = {
  ...
	.patterns      = board_led_patterns,
	.num_patterns  = ARRAY_SIZE(board_led_patterns),
  };

```
```

  echo 1 > /sys/bus/i2c/devices/xxxx/led_pattern    # 红色 LED 闪烁模式
  echo 2 > /sys/bus/i2c/devices/xxxx/led_pattern    # 蓝色 LED 常亮

```
```

  echo 0 > /sys/bus/i2c/devices/xxxx/led_pattern

```
