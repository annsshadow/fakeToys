## lp855x 内核驱动


LP855x IC 的背光驱动

支持的芯片：

	Texas Instruments LP8550, LP8551, LP8552, LP8553, LP8555, LP8556 以及
	LP8557

Author: Milo(Woogyom) Kim <milo.kim@ti.com>

### 描述


- 亮度控制

  亮度可以通过 pwm 输入或 i2c 命令控制。lp855x 驱动支持这两种情况。

- 设备属性

  1) bl_ctl_mode

  背光控制模式。

  取值：基于 pwm 或基于寄存器

  2) chip_id

  lp855x 芯片 id。

  取值：lp8550/lp8551/lp8552/lp8553/lp8555/lp8556/lp8557

### lp855x 的平台数据


为支持平台相关数据，可以使用 lp855x 平台数据。

- name:
	背光驱动名称。若未定义，则设置默认名称。
- device_control:
	DEVICE CONTROL 寄存器的值。
- initial_brightness:
	背光亮度初始值。
- period_ns:
	平台相关的 PWM 周期值。单位为纳秒。
	仅在亮度为 pwm 输入模式时有效。
- size_program:
	lp855x_rom_data 的总大小。
- rom_data:
	新的 eeprom/eprom 寄存器列表。

## 示例


```

    #define EEPROM_A5_ADDR	0xA5
    #define EEPROM_A5_VAL	0x4f	/* EN_VSYNC=0 */

    static struct lp855x_rom_data lp8552_eeprom_arr[] = {
	{EEPROM_A5_ADDR, EEPROM_A5_VAL},
    };

    static struct lp855x_platform_data lp8552_pdata = {
	.name = "lcd-bl",
	.device_control = I2C_CONFIG(LP8552),
	.initial_brightness = INITIAL_BRT,
	.size_program = ARRAY_SIZE(lp8552_eeprom_arr),
	.rom_data = lp8552_eeprom_arr,
    };

```
```

    static struct lp855x_platform_data lp8556_pdata = {
	.device_control = PWM_CONFIG(LP8556),
	.initial_brightness = INITIAL_BRT,
	.period_ns = 1000000,
    };

```
