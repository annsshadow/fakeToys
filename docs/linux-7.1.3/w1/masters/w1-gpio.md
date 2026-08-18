## 鍐呮牳椹卞姩w1-gpio


浣滆€咃細Ville Syrjala <syrjala@sci.fi>


### 鎻忚堪


GPIO 1 绾挎€荤嚎涓婚┍鍔ㄥ櫒銆傞┍鍔ㄧ▼搴忎娇鐢?GPIO API 鏉ユ帶鍒?
鍙互浣跨敤 GPIO 鏈哄櫒鎻忚堪绗﹁〃鎸囧畾绾胯矾鍜?GPIO 寮曡剼銆?
涔熷彲浠ヤ娇鐢ㄨ澶囨爲瀹氫箟涓昏澶囷紝璇峰弬瑙?
鏂囨。/devicetree/bindings/w1/w1-gpio.yaml


### 绀轰緥锛坢ach-at91锛?


```

  #include <linux/gpio/machine.h>
  #include <linux/w1-gpio.h>

  static struct gpiod_lookup_table foo_w1_gpiod_table = {
	.dev_id = "w1-gpio",
	.table = {
		GPIO_LOOKUP_IDX("at91-gpio", AT91_PIN_PB20, NULL, 0,
			GPIO_ACTIVE_HIGH|GPIO_OPEN_DRAIN),
	},
  };

  static struct w1_gpio_platform_data foo_w1_gpio_pdata = {
	.ext_pullup_enable_pin	= -EINVAL,
  };

  static struct platform_device foo_w1_device = {
	.name			= "w1-gpio",
	.id			= -1,
	.dev.platform_data	= &foo_w1_gpio_pdata,
  };

  ...
	at91_set_GPIO_periph(foo_w1_gpio_pdata.pin, 1);
	at91_set_multi_drive(foo_w1_gpio_pdata.pin, 1);
	gpiod_add_lookup_table(&foo_w1_gpiod_table);
	platform_device_register(&foo_w1_device);

```