## 鍐呮牳椹卞姩 i2c-mux-gpio


Author: Peter Korsgaard <peter.korsgaard@barco.com>

### 鎻忚堪


i2c-mux-gpio 鏄竴涓?i2c mux 椹卞姩锛岄€氳繃 GPIO 寮曡剼鎺у埗鐨勭‖浠?MUX锛屼粠涓?I2C 鎬荤嚎
鎻愪緵瀵?I2C 鎬荤嚎娈电殑璁块棶銆?
```

  ----------              ----------  Bus segment 1   - - - - -
 |          | SCL/SDA    |          |-------------- |           |
 |          |------------|          |
 |          |            |          | Bus segment 2 |           |
 |  Linux   | GPIO 1..N  |   MUX    |---------------   Devices
 |          |------------|          |               |           |
 |          |            |          | Bus segment M
 |          |            |          |---------------|           |
  ----------              ----------                  - - - - -

```
涓?I2C 鎬荤嚎鐨?SCL/SDA 鏍规嵁 GPIO 寮曡剼 1..N 鐨勮缃澶氳矾澶嶇敤鍒版€荤嚎娈?1..M銆?
### 鐢ㄦ硶


i2c-mux-gpio 浣跨敤 platform 鎬荤嚎锛屽洜姝や綘闇€瑕佹彁渚涗竴涓?struct platform_device锛?鍏?platform_data 鎸囧悜涓€涓?struct i2c_mux_gpio_platform_data锛屽叾涓寘鍚富鎬荤嚎鐨?I2C adapter 缂栧彿銆佽鍒涘缓鐨勬€荤嚎娈垫暟閲忥紝浠ュ強鐢ㄤ簬鎺у埗瀹冪殑 GPIO 寮曡剼銆傝鎯呰鍙傝
include/linux/platform_data/i2c-mux-gpio.h銆?
渚嬪锛屼负涓€涓彁渚?4 涓€荤嚎娈电殑 MUX锛屽彲浠ヨ繖鏍峰啓锛?
```

  #include <linux/platform_data/i2c-mux-gpio.h>
  #include <linux/platform_device.h>

  static const unsigned myboard_gpiomux_gpios[] = {
	AT91_PIN_PC26, AT91_PIN_PC25, AT91_PIN_PC24
  };

  static const unsigned myboard_gpiomux_values[] = {
	0, 1, 2, 3
  };

  static struct i2c_mux_gpio_platform_data myboard_i2cmux_data = {
	.parent		= 1,
	.base_nr	= 2, /* optional */
	.values		= myboard_gpiomux_values,
	.n_values	= ARRAY_SIZE(myboard_gpiomux_values),
	.gpios		= myboard_gpiomux_gpios,
	.n_gpios	= ARRAY_SIZE(myboard_gpiomux_gpios),
	.idle		= 4, /* optional */
  };

  static struct platform_device myboard_i2cmux = {
	.name		= "i2c-mux-gpio",
	.id		= 0,
	.dev		= {
		.platform_data	= &myboard_i2cmux_data,
	},
  };

```
濡傛灉浣犲湪娉ㄥ唽鏃朵笉鐭ラ亾缁濆鐨?GPIO 寮曡剼缂栧彿锛屽彲浠ユ敼涓烘彁渚涗竴涓姱鐗囧悕
锛?chip_name锛夊拰鐩稿鐨?GPIO 寮曡剼缂栧彿锛宨2c-mux-gpio 椹卞姩浼氭浛浣犲畬鎴愮浉鍏冲伐浣滐紝
鍖呮嫭鍦ㄨ GPIO 鑺墖涓嶈兘绔嬪嵆鍙敤鏃惰繘琛屽欢杩熸帰娴嬶紙deferred probing锛夈€?
### 璁惧娉ㄥ唽


娉ㄥ唽浣犵殑 i2c-mux-gpio 璁惧鏃讹紝浣犲簲褰撳皢鍏朵娇鐢ㄧ殑浠讳竴 GPIO 寮曡剼鐨勭紪鍙蜂綔涓鸿澶?ID 浼犲叆銆傝繖淇濊瘉浜嗘瘡涓疄渚嬮兘鏈変笉鍚岀殑 ID銆?
鎴栬€咃紝濡傛灉浣犱笉闇€瑕佺ǔ瀹氱殑璁惧鍚嶏紝鍙互鐩存帴浼犲叆 PLATFORM_DEVID_AUTO 浣滀负璁惧
ID锛宲latform 鏍稿績浼氫负浣犵殑璁惧鍒嗛厤涓€涓姩鎬?ID銆傚鏋滀綘鍦ㄦ敞鍐屾椂涓嶇煡閬撶粷瀵圭殑
GPIO 寮曡剼缂栧彿锛岃繖鐢氳嚦鏄敮涓€鐨勯€夋嫨銆?