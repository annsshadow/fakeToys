## GPIO 鏄犲皠


鏈枃妗ｈ鏄庡浣曞皢 GPIO 鍒嗛厤缁欐寚瀹氱殑璁惧鍜屽姛鑳姐€?
鎵€鏈夊钩鍙伴兘鍙互鍚敤 GPIO 搴擄紝浣嗗鏋滄煇涓钩鍙颁弗鏍艰姹傚繀椤绘彁渚?GPIO 鍔熻兘锛屽垯闇€瑕佸湪鍏?Kconfig 涓€夋嫨 GPIOLIB銆備箣鍚庯紝GPIO 濡備綍鏄犲皠鍙栧喅浜庤骞冲彴浣跨敤浠€涔堟柟寮忔潵鎻忚堪鍏剁‖浠?甯冨眬銆傜洰鍓嶏紝鏄犲皠鍙互閫氳繃璁惧鏍戯紙device tree锛夈€丄CPI 鍜屽钩鍙版暟鎹紙platform data锛夋潵瀹氫箟銆?
### 璁惧鏍?
鍦ㄨ澶囨爲涓紝GPIO 鍙互寰堟柟渚垮湴鏄犲皠鍒拌澶囧拰鍔熻兘涓娿€傚叿浣撶殑鍋氭硶鍙栧喅浜庢彁渚涜繖浜?GPIO 鐨?GPIO 鎺у埗鍣紝璇峰弬鑰冧綘鐨勬帶鍒跺櫒瀵瑰簲鐨勮澶囨爲缁戝畾锛坉evice tree bindings锛夈€?
GPIO 鏄犲皠瀹氫箟鍦ㄦ秷璐硅澶囷紙consumer device锛夌殑鑺傜偣涓紝浣嶄簬涓€涓悕涓?<function>-gpios 鐨勫睘鎬ч噷锛屽叾涓?<function> 鏄┍鍔ㄥ皢璇锋眰鐨勯偅涓姛鑳?```


	foo_device {
		compatible = "acme,foo";
		...
		led-gpios = <&gpio 15 GPIO_ACTIVE_HIGH>, /* red */
			    <&gpio 16 GPIO_ACTIVE_HIGH>, /* green */
			    <&gpio 17 GPIO_ACTIVE_HIGH>; /* blue */

		power-gpios = <&gpio 1 GPIO_ACTIVE_LOW>;
	};


```
鍚嶄负 <function>-gpio 鐨勫睘鎬т篃琚涓烘湁鏁堬紝鏃х殑缁戝畾涓娇鐢ㄤ簡瀹冿紝浣嗕粎涓哄吋瀹规€ц€屼繚鐣欙紝
鐢变簬宸茶寮冪敤锛屾柊鐨勭粦瀹氫腑涓嶅簲鍐嶄娇鐢ㄣ€?
璇ュ睘鎬у皢浣?GPIO 15銆?6 鍜?17 閫氳繃浠ヤ笅鏂瑰紡瀵归┍鍔ㄥ彲鐢?```
	struct gpio_desc *red, *green, *blue, *power;

	red = gpiod_get_index(dev, "led", 0, GPIOD_OUT_HIGH);
	green = gpiod_get_index(dev, "led", 1, GPIOD_OUT_HIGH);
	blue = gpiod_get_index(dev, "led", 2, GPIOD_OUT_HIGH);

	power = gpiod_get(dev, "power", GPIOD_OUT_HIGH);

```
led 鐨?GPIO 灏嗕负楂樼數骞虫湁鏁堬紙active high锛夛紝鑰?power 鐨?GPIO 涓轰綆鐢靛钩鏈夋晥锛坅ctive low锛?锛堝嵆 gpiod_is_active_low(power) 灏嗚繑鍥?true锛夈€?
gpiod_get() 绯诲垪鍑芥暟鐨勭浜屼釜鍙傛暟锛屽嵆 con_id 瀛楃涓诧紝蹇呴』鏄?GPIO 鍚庣紑
锛?gpios" 鎴?"gpio"锛岀敱 gpiod 鍑芥暟鍦ㄥ唴閮ㄨ嚜鍔ㄦ煡鎵撅級鎵€瀵瑰簲鐨?<function>- 鍓嶇紑锛?璇ュ悗缂€鍦ㄨ澶囨爲涓娇鐢ㄣ€備互涓婇潰鐨?"led-gpios" 涓轰緥锛屼綔涓?con_id 鍙傛暟搴斾娇鐢ㄤ笉甯?"-"
鐨勫墠缂€锛?led"銆?
鍦ㄥ唴閮紝GPIO 瀛愮郴缁熶細鎶婁紶鍏?con_id 鐨勫瓧绗︿覆涓?GPIO 鍚庣紑锛?gpios" 鎴?"gpio"锛夋嫾鎺ワ紝
寰楀埌鏈€缁堢殑瀛楃涓诧紙`snprintf(... "%s-%s", con_id, gpio_suffixes[]`锛夈€?
### ACPI

ACPI 涔熶互绫讳技浜?DT 鐨勬柟寮忔敮鎸?GPIO 鐨勫姛鑳藉悕銆備笂闈㈢殑 DT 绀轰緥鍙互杞崲涓虹瓑浠风殑 ACPI 鎻忚堪
```

	Device (FOO) {
		Name (_CRS, ResourceTemplate () {
			GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 15 } // red
			GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 16 } // green
			GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 17 } // blue
			GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 1 } // power
		})

		Name (_DSD, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () {
					"led-gpios",
					Package () {
						^FOO, 0, 0, 1,
						^FOO, 1, 0, 1,
						^FOO, 2, 0, 1,
					}
				},
				Package () { "power-gpios", Package () { ^FOO, 3, 0, 0 } },
			}
		})
	}

```
鏈夊叧 ACPI GPIO 缁戝畾鐨勬洿澶氫俊鎭紝璇峰弬瑙?Documentation/firmware-guide/acpi/gpio-properties.rst銆?
### 杞欢鑺傜偣


杞欢鑺傜偣锛坰oftware nodes锛夊厑璁告澘绾х壒瀹氱殑浠ｇ爜浣跨敤 struct software_node 鍜?struct
property_entry 鏋勯€犱竴涓唴瀛樹腑銆佺被璁惧鏍戠殑缁撴瀯銆傞殢鍚庤缁撴瀯鍙互涓庡钩鍙拌澶囧叧鑱旓紝浣块┍鍔?鑳藉浣跨敤鏍囧噯鐨勮澶囧睘鎬э紙device properties锛堿PI 鏉ユ煡璇㈤厤缃紝灏卞儚鍦?ACPI 鎴栬澶囨爲绯荤粺
涓婁竴鏍枫€?
鐢辫蒋浠惰妭鐐规敮鎸佺殑 GPIO 浣跨敤 `PROPERTY_ENTRY_GPIO()` 瀹忔潵鎻忚堪锛岃瀹忓皢浠ｈ〃 GPIO 鎺у埗鍣ㄧ殑
杞欢鑺傜偣涓庢秷璐硅澶囧叧鑱旇捣鏉ャ€傚畠鍏佽娑堣垂鏂逛娇鐢ㄥ父瑙勭殑 gpiolib API锛屼緥濡?gpiod_get()銆乬piod_get_optional()銆?
浠ｈ〃 GPIO 鎺у埗鍣ㄧ殑杞欢鑺傜偣蹇呴』鎸傛帴鍒?GPIO 鎺у埗鍣ㄨ澶?鈥斺€?鏃㈠彲浠ヤ綔涓轰富鍥轰欢鑺傜偣锛屼篃鍙互浣滀负
娆＄骇鍥轰欢鑺傜偣銆?
渚嬪锛屼笅闈㈡槸濡備綍鎻忚堪涓€涓敱鍗曚釜 GPIO 杩炴帴鐨?LED銆傝繖鏄湪鏃х郴缁熶笂浣跨敤 platform_data 鐨?鏇夸唬鏂规銆?

	#include <linux/property.h>
	#include <linux/gpio/machine.h>
	#include <linux/gpio/property.h>

	/*
  - 1. 瀹氫箟 GPIO 鎺у埗鍣ㄧ殑鑺傜偣銆?	 */
	static const struct software_node gpio_controller_node = {
		.name = "gpio-foo",
	};

	/** 2. 瀹氫箟 LED 璁惧鐨勫睘鎬с€?**/
	static const struct property_entry led_device_props[] = {
		PROPERTY_ENTRY_STRING("label", "myboard:green:status"),
		PROPERTY_ENTRY_STRING("linux,default-trigger", "heartbeat"),
		PROPERTY_ENTRY_GPIO("gpios", &gpio_controller_node, 42, GPIO_ACTIVE_HIGH),
		{ }
	};

	/** 3. 瀹氫箟 LED 璁惧鐨勮蒋浠惰妭鐐广€?**/
	static const struct software_node led_device_swnode = {
		.name = "status-led",
		.properties = led_device_props,
	};

	/*
  - 4. 娉ㄥ唽杞欢鑺傜偣鍜屽钩鍙拌澶囥€?	 */
	const struct software_node *swnodes[] = {
		&gpio_controller_node,
		&led_device_swnode,
		NULL
	};
	software_node_register_node_group(swnodes);

	/*
  - 5. 灏?GPIO 鎺у埗鍣ㄧ殑杞欢鑺傜偣鎸傛帴鍒拌澶囧苟娉ㄥ唽瀹冦€?	 */
	 static void gpio_foo_register(void)
	 {
		struct platform_device_info pdev_info = {
			.name = "gpio-foo",
			.id = PLATFORM_DEVID_NONE,
			.swnode = &gpio_controller_node
		};

		platform_device_register_full(&pdev_info);
	 }

	// 鐒跺悗涓?"leds-gpio" 娉ㄥ唽涓€涓?platform_device锛屽苟閫氳繃 .fwnode
	// 灏嗗叾涓?&led_device_swnode 鍏宠仈銆?
鍏充簬濡備綍灏嗘澘鏂囦欢杞崲涓轰娇鐢ㄨ蒋浠惰妭鐐圭殑瀹屾暣鎸囧崡锛岃鍙傝
Documentation/driver-api/gpio/legacy-boards.rst銆?
### 骞冲彴鏁版嵁

鏈€鍚庯紝GPIO 杩樺彲浠ラ€氳繃骞冲彴鏁版嵁缁戝畾鍒拌澶囧拰鍔熻兘銆傛澘绾т唬鐮?```

	#include <linux/gpio/machine.h>


```
GPIO 閫氳繃鏌ユ壘琛紙tables of lookups锛夋潵鏄犲皠锛岃〃涓寘鍚涓嬪疄渚?```

	GPIO_LOOKUP(key, chip_hwnum, con_id, flags)
	GPIO_LOOKUP_IDX(key, chip_hwnum, con_id, idx, flags)


```
鍏朵腑

  - key 鏄彁渚涜 GPIO 鐨?gpiod_chip 瀹炰緥鐨勬爣绛撅紝鎴栬€呮槸 GPIO 绾垮悕绉?  - chip_hwnum 鏄?GPIO 鍦ㄨ姱鐗囧唴鐨勭‖浠剁紪鍙凤紝鎴栬€?U16_MAX 琛ㄧず key 鏄竴涓?GPIO 绾垮悕绉?  - con_id 鏄粠璁惧瑙嗚鐪嬪埌鐨?GPIO 鍔熻兘鍚嶇О銆傚畠鍙互涓?NULL锛屾鏃跺皢鍖归厤浠绘剰鍔熻兘銆?  - idx 鏄?GPIO 鍦ㄥ姛鑳藉唴鐨勭储寮曘€?  - flags 鐢ㄤ簬鎸囧畾浠ヤ笅灞炴€э細
 - GPIO_ACTIVE_HIGH	- GPIO 绾夸负楂樼數骞虫湁鏁? - GPIO_ACTIVE_LOW	- GPIO 绾夸负浣庣數骞虫湁鏁? - GPIO_OPEN_DRAIN	- GPIO 绾胯閰嶇疆涓哄紑婕忥紙open drain锛? - GPIO_OPEN_SOURCE	- GPIO 绾胯閰嶇疆涓哄紑婧愶紙open source锛? - GPIO_PERSISTENT	- GPIO 绾垮湪鎸傝捣/鎭㈠锛坰uspend/resume锛夋湡闂翠繚鎸?				  鍏跺彇鍊间笉鍙? - GPIO_TRANSITORY	- GPIO 绾挎槸鏆傛椂鎬х殑锛屽湪鎸傝捣/鎭㈠鏈熼棿鍙兘涓㈠け
				  鍏剁數姘旂姸鎬?
灏嗘潵锛岃繖浜涙爣蹇楀彲鑳戒細鎵╁睍浠ユ敮鎸佹洿澶氬睘鎬с€?
娉ㄦ剰锛?  1. GPIO 绾垮悕绉颁笉淇濊瘉鍏ㄥ眬鍞竴锛屽洜姝や細閲囩敤鎵惧埌鐨勭涓€涓尮閰嶉」銆?  2. GPIO_LOOKUP() 鍙槸 GPIO_LOOKUP_IDX() 鍦?idx = 0 鏃剁殑绠€渚垮啓娉曘€?
鐒跺悗鍙互鎸夊涓嬫柟寮忓畾涔夋煡鎵捐〃锛屼互涓€涓┖鏉＄洰琛ㄧず琛ㄧ殑缁撴潫銆傝〃涓殑 'dev_id' 瀛楁鏄皢浣跨敤
杩欎簺 GPIO 鐨勮澶囩殑鏍囪瘑绗︺€傚畠鍙互涓?NULL锛屾鏃跺皢鍖归厤浠?NULL 璁惧璋冪敤 gpiod_get() 鐨勬儏鍐点€?

        struct gpiod_lookup_table gpios_table = {
                .dev_id = "foo.0",
                .table = {
                        GPIO_LOOKUP_IDX("gpio.0", 15, "led", 0, GPIO_ACTIVE_HIGH),
                        GPIO_LOOKUP_IDX("gpio.0", 16, "led", 1, GPIO_ACTIVE_HIGH),
                        GPIO_LOOKUP_IDX("gpio.0", 17, "led", 2, GPIO_ACTIVE_HIGH),
                        GPIO_LOOKUP("gpio.0", 1, "power", GPIO_ACTIVE_LOW),
                        { },
                },
        };


```

	gpiod_add_lookup_table(&gpios_table);


```
```
	struct gpio_desc *red, *green, *blue, *power;

	red = gpiod_get_index(dev, "led", 0, GPIOD_OUT_HIGH);
	green = gpiod_get_index(dev, "led", 1, GPIOD_OUT_HIGH);
	blue = gpiod_get_index(dev, "led", 2, GPIOD_OUT_HIGH);

	power = gpiod_get(dev, "power", GPIOD_OUT_HIGH);


```
鐢变簬 "led" 鐨?GPIO 琚槧灏勪负楂樼數骞虫湁鏁堬紝鏈ず渚嬪皢鎶婂畠浠俊鍙风疆涓?1锛屽嵆鐐逛寒 LED銆傝€屽浜庤
鏄犲皠涓轰綆鐢靛钩鏈夋晥鐨?"power" GPIO锛岃繖娈典唬鐮佹墽琛屽悗鍏跺疄闄呬俊鍙峰皢涓?0銆備笌鏃х殑鏁村瀷 GPIO 鎺ュ彛
涓嶅悓锛屼綆鐢靛钩鏈夋晥锛坅ctive-low锛夊睘鎬ф槸鍦ㄦ槧灏勮繃绋嬩腑澶勭悊鐨勶紝鍥犳瀵?GPIO 娑堣垂鏂规槸閫忔槑鐨勩€?
涓€缁勮濡?gpiod_set_value() 涔嬬被鐨勫嚱鏁板彲鐢ㄤ簬鎿嶄綔杩欎釜鏂扮殑銆佷互鎻忚堪绗︿负瀵煎悜鐨勬帴鍙ｃ€?
### 寮曡剼鏁扮粍

闄や簡閫愪釜璇锋眰灞炰簬鏌愪釜鍔熻兘鐨勫紩鑴氬锛岃澶囦篃鍙互璇锋眰鍒嗛厤缁欒鍔熻兘鐨勪竴缁勫紩鑴氥€傝繖浜涘紩鑴?濡備綍鏄犲皠鍒拌澶囷紝鍐冲畾浜嗚鏁扮粍鏄惁鏈夎祫鏍艰繘琛屽揩閫熺殑浣嶅浘澶勭悊銆傚鏋滃彲浠ワ紝浣嶅浘灏嗛€氳繃
get/set 鏁扮粍鍑芥暟鍦ㄨ皟鐢ㄦ柟涓?GPIO 鑺墖鐩稿簲鐨?.get/set_multiple() 鍥炶皟涔嬮棿鐩存帴浼犻€掋€?
涓轰簡绗﹀悎蹇€熶綅鍥惧鐞嗙殑鏉′欢锛屾暟缁勫繀椤绘弧瓒充互涓嬭姹傦細

- 鏁扮粍鎴愬憳 0 鐨勫紩鑴氱‖浠剁紪鍙蜂篃蹇呴』涓?0锛?- 涓庢垚鍛?0 灞炰簬鍚屼竴鑺墖鐨勮繛缁暟缁勬垚鍛樼殑寮曡剼纭欢缂栧彿锛屼篃蹇呴』涓庡叾鏁扮粍绱㈠紩鐩稿尮閰嶃€?
鍚﹀垯涓嶄細浣跨敤蹇€熶綅鍥惧鐞嗚矾寰勶紝浠ラ伩鍏嶅睘浜庡悓涓€鑺墖浣嗙‖浠堕『搴忎笉杩炵画鐨勫紩鑴氳鍒嗗紑澶勭悊銆?
濡傛灉鏁扮粍绗﹀悎蹇€熶綅鍥惧鐞嗚矾寰勶紝閭ｄ箞涓庢垚鍛?0 涓嶅悓鑺墖鐨勫紩鑴氾紝浠ュ強绱㈠紩涓庡叾纭欢寮曡剼缂栧彿
涓嶅悓鐨勫紩鑴氾紝閮戒細琚帓闄ゅ湪蹇€熻矾寰勪箣澶栵紝鏃犺杈撳叆杩樻槸杈撳嚭銆傛澶栵紝寮€婕忓拰寮€婧愬紩鑴氫細琚?鎺掗櫎鍦ㄥ揩閫熶綅鍥捐緭鍑哄鐞嗕箣澶栥€?