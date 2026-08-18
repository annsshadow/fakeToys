## lp855x 鍐呮牳椹卞姩


LP855x IC 鐨勮儗鍏夐┍鍔?
鏀寔鐨勮姱鐗囷細

	Texas Instruments LP8550, LP8551, LP8552, LP8553, LP8555, LP8556 浠ュ強
	LP8557

Author: Milo(Woogyom) Kim <milo.kim@ti.com>

### 鎻忚堪


- 浜害鎺у埗

  浜害鍙互閫氳繃 pwm 杈撳叆鎴?i2c 鍛戒护鎺у埗銆俵p855x 椹卞姩鏀寔杩欎袱绉嶆儏鍐点€?
- 璁惧灞炴€?
  1) bl_ctl_mode

  鑳屽厜鎺у埗妯″紡銆?
  鍙栧€硷細鍩轰簬 pwm 鎴栧熀浜庡瘎瀛樺櫒

  2) chip_id

  lp855x 鑺墖 id銆?
  鍙栧€硷細lp8550/lp8551/lp8552/lp8553/lp8555/lp8556/lp8557

### lp855x 鐨勫钩鍙版暟鎹?

涓烘敮鎸佸钩鍙扮浉鍏虫暟鎹紝鍙互浣跨敤 lp855x 骞冲彴鏁版嵁銆?
- name:
	鑳屽厜椹卞姩鍚嶇О銆傝嫢鏈畾涔夛紝鍒欒缃粯璁ゅ悕绉般€?- device_control:
	DEVICE CONTROL 瀵勫瓨鍣ㄧ殑鍊笺€?- initial_brightness:
	鑳屽厜浜害鍒濆鍊笺€?- period_ns:
	骞冲彴鐩稿叧鐨?PWM 鍛ㄦ湡鍊笺€傚崟浣嶄负绾崇銆?	浠呭湪浜害涓?pwm 杈撳叆妯″紡鏃舵湁鏁堛€?- size_program:
	lp855x_rom_data 鐨勬€诲ぇ灏忋€?- rom_data:
	鏂扮殑 eeprom/eprom 瀵勫瓨鍣ㄥ垪琛ㄣ€?
## 绀轰緥


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
