## Supporting Legacy Boards


鍐呮牳涓殑璁稿椹卞姩锛屼緥濡?`leds-gpio` 鍜?`gpio-keys`锛屾閫愭笎浠庝娇鐢ㄦ澘鐗瑰畾鐨?`platform_data` 杩佺Щ鍒?
缁熶竴鐨勮澶囧睘鎬э紙device properties锛夋帴鍙ｃ€傝鎺ュ彛璁╅┍鍔ㄦ洿绠€鍗曘€佹洿閫氱敤锛屽洜涓哄畠浠彲浠ヤ互鏍囧噯鍖栫殑
鏂瑰紡鏌ヨ灞炴€с€?

鍦ㄧ幇浠ｇ郴缁熶笂锛岃繖浜涘睘鎬ч€氳繃璁惧鏍戞彁渚涖€傜劧鑰岋紝涓€浜涜緝鏃х殑骞冲彴灏氭湭杞崲涓鸿澶囨爲锛岃€屾槸渚濊禆鏉挎枃浠?
鏉ユ弿杩板叾纭欢閰嶇疆銆備负浜嗗讥鍚堣繖涓€宸窛锛屽苟璁╄繖浜涗紶缁熸澘鑳藉閰嶅悎鐜颁唬鐨勯€氱敤椹卞姩宸ヤ綔锛屽唴鏍告彁渚涗簡涓€绉?
绉颁负**杞欢鑺傜偣**锛坰oftware node锛夌殑鏈哄埗銆?

鏈枃妗ｆ彁渚涗簡濡備綍灏嗕紶缁熸澘鏂囦欢浠庝娇鐢?`platform_data` 鍜?`gpiod_lookup_table` 杞崲涓虹幇浠ｇ殑杞欢
鑺傜偣鏂规硶鏉ユ弿杩?GPIO 杩炴帴璁惧鐨勬寚鍗椼€?

### The Core Idea: Software Nodes


杞欢鑺傜偣鍏佽鏉跨壒瀹氫唬鐮佷娇鐢?struct software_node 鍜?struct property_entry 鏋勫缓鍐呭瓨涓殑銆佺被浼?
璁惧鏍戠殑缁撴瀯銆傝缁撴瀯闅忓悗鍙互涓庡钩鍙拌澶囧叧鑱旓紝浣块┍鍔ㄨ兘澶熶娇鐢ㄦ爣鍑嗙殑璁惧灞炴€?API锛堜緥濡?
device_property_read_u32()銆乨evice_property_read_string()锛夋煡璇㈤厤缃紝灏卞儚鍦?ACPI 鎴栬澶囨爲绯荤粺涓?
涓€鏍枫€?

gpiolib 浠ｇ爜鏀寔澶勭悊杞欢鑺傜偣锛屽洜姝ゅ鏋?GPIO 琚纭弿杩帮紙濡備笅鑺傝杩帮級锛岄偅涔堝父瑙勭殑 gpiolib API锛?
濡?gpiod_get()銆乬piod_get_optional() 绛夛紝閮借兘姝ｅ父宸ヤ綔銆?

#### Requirements for GPIO Properties


浣跨敤杞欢鑺傜偣鎻忚堪 GPIO 杩炴帴鏃讹紝蹇呴』婊¤冻浠ヤ笅瑕佹眰锛孏PIO 鏍稿績鎵嶈兘姝ｇ‘瑙ｆ瀽寮曠敤锛?

1. **GPIO 鎺у埗鍣ㄧ殑杞欢鑺傜偣蹇呴』宸叉敞鍐岋紝骞朵綔涓轰富鍥轰欢鑺傜偣鎴栨鍥轰欢鑺傜偣鎸傝浇鍒版帶鍒跺櫒鐨?`struct
    device` 涓娿€?* gpiolib 鏍稿績浣跨敤鍥轰欢鑺傜偣鐨勫湴鍧€鍦ㄨ繍琛屾椂鏌ユ壘瀵瑰簲鐨?`struct gpio_chip`銆?

2. **GPIO 灞炴€у繀椤绘槸涓€涓紩鐢ㄣ€?* `PROPERTY_ENTRY_GPIO()` 瀹忓鐞嗕簡杩欎竴鐐癸紝鍥犱负瀹冩槸
   `PROPERTY_ENTRY_REF()` 鐨勫埆鍚嶃€?

3. **璇ュ紩鐢ㄥ繀椤绘伆濂芥湁涓や釜鍙傛暟锛?*

    - 绗竴涓弬鏁版槸鎺у埗鍣ㄥ唴鐨?GPIO 鍋忕Щ閲忋€?
    - 绗簩涓弬鏁版槸璇?GPIO 绾跨殑鏍囧織锛堜緥濡?GPIO_ACTIVE_HIGH銆丟PIO_ACTIVE_LOW锛夈€?

`PROPERTY_ENTRY_GPIO()` 瀹忔槸鍦ㄨ蒋浠惰妭鐐逛腑瀹氫箟 GPIO 灞炴€х殑棣栭€夋柟寮忋€?

### Conversion Example


璁╂垜浠€氳繃涓€涓皢瀹氫箟 GPIO 杩炴帴鐨?LED 鍜屾寜閽殑鏉挎枃浠惰繘琛岃浆鎹㈢殑绀轰緥鏉ラ€愭璇存槑銆?

#### Before: Using Platform Data


涓€涓吀鍨嬬殑浼犵粺鏉挎枃浠跺彲鑳藉涓嬫墍绀猴細


  #include <linux/platform_device.h>
  #include <linux/leds.h>
  #include <linux/gpio_keys.h>
  #include <linux/gpio/machine.h>

  #define MYBOARD_GPIO_CONTROLLER "gpio-foo"

  /** LED 璁剧疆 **/
  static const struct gpio_led myboard_leds[] = {
  	{
  		.name = "myboard:green:status",
  		.default_trigger = "heartbeat",
  	},
  };

  static const struct gpio_led_platform_data myboard_leds_pdata = {
  	.num_leds = ARRAY_SIZE(myboard_leds),
  	.leds = myboard_leds,
  };

  static struct gpiod_lookup_table myboard_leds_gpios = {
  	.dev_id = "leds-gpio",
  	.table = {
  		GPIO_LOOKUP_IDX(MYBOARD_GPIO_CONTROLLER, 42, NULL, 0, GPIO_ACTIVE_HIGH),
  		{ },
  	},
  };

  /** 鎸夐挳璁剧疆 **/
  static struct gpio_keys_button myboard_buttons[] = {
  	{
  		.code = KEY_WPS_BUTTON,
  		.desc = "WPS Button",
  		.active_low = 1,
  	},
  };

  static const struct gpio_keys_platform_data myboard_buttons_pdata = {
  	.buttons = myboard_buttons,
  	.nbuttons = ARRAY_SIZE(myboard_buttons),
  };

  static struct gpiod_lookup_table myboard_buttons_gpios = {
  	.dev_id = "gpio-keys",
  	.table = {
  		GPIO_LOOKUP_IDX(MYBOARD_GPIO_CONTROLLER, 15, NULL, 0, GPIO_ACTIVE_LOW),
  		{ },
  	},
  };

  /** 璁惧娉ㄥ唽 **/
  static int __init myboard_init(void)
  {
  	struct platform_device_info pdev_info = {
  		.name = MYBOARD_GPIO_CONTROLLER,
  		.id = PLATFORM_DEVID_NONE,
  		.swnode = &gpio_controller_node
  	};

  	gpiod_add_lookup_table(&myboard_leds_gpios);
  	gpiod_add_lookup_table(&myboard_buttons_gpios);

  	platform_device_register_full(&pdev_info);
  	platform_device_register_data(NULL, "leds-gpio", -1,
  				      &myboard_leds_pdata, sizeof(myboard_leds_pdata));
  	platform_device_register_data(NULL, "gpio-keys", -1,
  				      &myboard_buttons_pdata,
  				      sizeof(myboard_buttons_pdata));

  	return 0;
  }

#### After: Using Software Nodes


浠ヤ笅鏄浣曚娇鐢ㄨ蒋浠惰妭鐐硅〃杈剧浉鍚岀殑閰嶇疆銆?

######## 姝ラ 1锛氬畾涔?GPIO 鎺у埗鍣ㄨ妭鐐?


棣栧厛锛屽畾涔変竴涓唬琛?LED 鍜屾寜閽墍杩炴帴 GPIO 鎺у埗鍣ㄧ殑杞欢鑺傜偣銆傝鑺傜偣鐨?`name` 鏄彲閫夌殑銆?


  #include <linux/property.h>
  #include <linux/gpio/property.h>

  #define MYBOARD_GPIO_CONTROLLER "gpio-foo"

  static const struct software_node myboard_gpio_controller_node = {
  	.name = MYBOARD_GPIO_CONTROLLER,
  };

######## 姝ラ 2锛氬畾涔夋秷璐硅澶囪妭鐐逛笌灞炴€?


鎺ヤ笅鏉ワ紝瀹氫箟娑堣垂璁惧锛圠ED 鍜屾寜閽級鐨勮蒋浠惰妭鐐广€傝繖娑夊強涓烘瘡涓澶囩被鍨嬪垱寤轰竴涓埗鑺傜偣锛屽苟涓烘瘡涓?
鍗曠嫭鐨?LED 鎴栨寜閽垱寤哄瓙鑺傜偣銆?


  /** LED 璁剧疆 **/
  static const struct software_node myboard_leds_node = {
  	.name = "myboard-leds",
  };

  static const struct property_entry myboard_status_led_props[] = {
  	PROPERTY_ENTRY_STRING("label", "myboard:green:status"),
  	PROPERTY_ENTRY_STRING("linux,default-trigger", "heartbeat"),
  	PROPERTY_ENTRY_GPIO("gpios", &myboard_gpio_controller_node, 42, GPIO_ACTIVE_HIGH),
  	{ }
  };

  static const struct software_node myboard_status_led_swnode = {
  	.name = "status-led",
  	.parent = &myboard_leds_node,
  	.properties = myboard_status_led_props,
  };

  /** 鎸夐挳璁剧疆 **/
  static const struct software_node myboard_keys_node = {
  	.name = "myboard-keys",
  };

  static const struct property_entry myboard_wps_button_props[] = {
  	PROPERTY_ENTRY_STRING("label", "WPS Button"),
  	PROPERTY_ENTRY_U32("linux,code", KEY_WPS_BUTTON),
  	PROPERTY_ENTRY_GPIO("gpios", &myboard_gpio_controller_node, 15, GPIO_ACTIVE_LOW),
  	{ }
  };

  static const struct software_node myboard_wps_button_swnode = {
  	.name = "wps-button",
  	.parent = &myboard_keys_node,
  	.properties = myboard_wps_button_props,
  };



######## 姝ラ 3锛氬垎缁勫苟娉ㄥ唽鑺傜偣


涓轰簡鍙淮鎶ゆ€э紝閫氬父灏嗘墍鏈夎蒋浠惰妭鐐瑰垎缁勫埌涓€涓暟缁勪腑骞剁敤涓€娆¤皟鐢ㄦ敞鍐屽畠浠槸鏈夌泭鐨勩€?


  static const struct software_node * const myboard_swnodes[] = {
  	&myboard_gpio_controller_node,
  	&myboard_leds_node,
  	&myboard_status_led_swnode,
  	&myboard_keys_node,
  	&myboard_wps_button_swnode,
  	NULL
  };

  static int __init myboard_init(void)
  {
  	int error;

  	error = software_node_register_node_group(myboard_swnodes);
  	if (error) {
  		pr_err("Failed to register software nodes: %d\n", error);
  		return error;
  	}

  	// ... 闅忓悗鏄钩鍙拌澶囨敞鍐?
  }

  褰撴寜鎵€浠ｈ〃鐨勮澶囨媶鍒嗚妭鐐规敞鍐屾椂锛屽繀椤诲厛娉ㄥ唽浠ｈ〃 GPIO 鎺у埗鍣ㄦ湰韬殑杞欢鑺傜偣锛岀劧鍚庢墠鑳芥敞鍐屼换浣?
  寮曠敤瀹冪殑鑺傜偣銆?

######## 姝ラ 4锛氫娇鐢ㄨ蒋浠惰妭鐐规敞鍐屽钩鍙拌澶?


鏈€鍚庯紝娉ㄥ唽骞冲彴璁惧锛屽苟浣跨敤 struct platform_device_info 涓殑 `fwnode` 瀛楁灏嗗畠浠笌鍚勮嚜鐨勮蒋浠?
鑺傜偣鍏宠仈銆?


  static struct platform_device *leds_pdev;
  static struct platform_device *keys_pdev;

  static int __init myboard_init(void)
  {
  	struct platform_device_info pdev_info;
  	int error;

  	error = software_node_register_node_group(myboard_swnodes);
  	if (error)
  		return error;

  	memset(&pdev_info, 0, sizeof(pdev_info));
  	pdev_info.name = MYBOARD_GPIO_CONTROLLER;
  	pdev_info.id = PLATFORM_DEVID_NONE;
  	pdev_info.swnode = &myboard_gpio_controller_node;
  	gpio_pdev = platform_device_register_full(&pdev_info);
  	if (IS_ERR(gpio_pdev)) {
  		error = PTR_ERR(gpio_pdev);
  		goto err_unregister_nodes;
  	}

  	memset(&pdev_info, 0, sizeof(pdev_info));
  	pdev_info.name = "leds-gpio";
  	pdev_info.id = PLATFORM_DEVID_NONE;
  	pdev_info.fwnode = software_node_fwnode(&myboard_leds_node);
  	leds_pdev = platform_device_register_full(&pdev_info);
  	if (IS_ERR(leds_pdev)) {
  		error = PTR_ERR(leds_pdev);
  		platform_device_unregister(gpio_pdev);
  		goto err_unregister_nodes;
  	}

  	memset(&pdev_info, 0, sizeof(pdev_info));
  	pdev_info.name = "gpio-keys";
  	pdev_info.id = PLATFORM_DEVID_NONE;
  	pdev_info.fwnode = software_node_fwnode(&myboard_keys_node);
  	keys_pdev = platform_device_register_full(&pdev_info);
  	if (IS_ERR(keys_pdev)) {
  		error = PTR_ERR(keys_pdev);
  		platform_device_unregister(gpio_pdev);
  		platform_device_unregister(leds_pdev);
  		goto err_unregister_nodes;
  	}

  	return 0;

  err_unregister_nodes:
  	software_node_unregister_node_group(myboard_swnodes);
  	return error;
  }

  static void __exit myboard_exit(void)
  {
  	platform_device_unregister(keys_pdev);
  	platform_device_unregister(leds_pdev);
  	platform_device_unregister(gpio_pdev);
  	software_node_unregister_node_group(myboard_swnodes);
  }

閫氳繃杩欎簺鏇存敼锛岄€氱敤鐨?`leds-gpio` 鍜?`gpio-keys` 椹卞姩灏嗚兘澶熸垚鍔熸帰娴嬶紝骞朵粠杞欢鑺傜偣涓畾涔夌殑灞炴€?
鑾峰彇鍏堕厤缃紝浠庤€屼笉鍐嶉渶瑕佹澘鐗瑰畾鐨?platform data銆?
