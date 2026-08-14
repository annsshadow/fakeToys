## PINCTRL锛堝紩鑴氭帶鍒讹級瀛愮郴缁?

鏈枃妗ｆ杩颁簡 Linux 涓殑寮曡剼鎺у埗瀛愮郴缁?
璇ュ瓙绯荤粺澶勭悊锛?
- 鏋氫妇骞跺懡鍚嶅彲鎺х殑寮曡剼

- 寮曡剼銆佺剨鐩樸€佺鑴氾紙finger锛岀瓑锛夌殑澶氳矾澶嶇敤锛坢ultiplexing锛岃瑙佷笅鏂囷級

- 寮曡剼銆佺剨鐩樸€佺鑴氾紙绛夛級鐨勯厤缃紝渚嬪杞欢鎺у埗鐨勫亸缃紙biasing锛夊拰椹卞姩
  鐗瑰畾寮曡剼鐨勯┍鍔ㄦā寮忥紝渚嬪涓婃媺銆佷笅鎷夈€佸紑婕忋€佽礋杞界數瀹圭瓑銆?
## 椤跺眰鎺ュ彛


瀹氫箟锛?
- 涓€涓紩鑴氭帶鍒跺櫒锛圥IN CONTROLLER锛夋槸涓€鍧楃‖浠讹紝閫氬父鏄竴缁勫瘎瀛樺櫒锛屽彲浠?  鎺у埗寮曡剼锛圥IN锛夈€傚畠鍙互涓哄崟涓紩鑴氭垨寮曡剼缁勮繘琛屽璺鐢ㄣ€佸亸缃€佽缃?  璐熻浇鐢靛銆佽缃┍鍔ㄥ己搴︾瓑銆?
- 寮曡剼锛圥INS锛夌瓑鍚屼簬鐒婄洏銆佺鑴氥€佺悆锛坆all锛夋垨浠讳綍浣犳兂鎺у埗鐨勫皝瑁呰緭鍏ユ垨
  杈撳嚭绾匡紝瀹冧滑鐢辫寖鍥?0..maxpin 鍐呯殑鏃犵鍙锋暣鏁拌〃绀恒€傝繖涓紪鍙风┖闂村浜庢瘡涓?  寮曡剼鎺у埗鍣ㄦ槸灞€閮ㄧ殑锛屽洜姝ょ郴缁熶腑鍙兘瀛樺湪澶氫釜杩欐牱鐨勭紪鍙风┖闂淬€傝繖涓紩鑴氱┖闂?  鍙兘鏄█鐤忕殑鈥斺€斿嵆绌洪棿涓彲鑳藉瓨鍦ㄦ病鏈夊紩鑴氱殑缂栧彿闂撮殭銆?
褰撳疄渚嬪寲涓€涓紩鑴氭帶鍒跺櫒鏃讹紝瀹冧細鍚戝紩鑴氭帶鍒舵鏋舵敞鍐屼竴涓弿杩扮锛岃鎻忚堪绗?鍖呭惈涓€涓紩鑴氭弿杩扮鏁扮粍锛屾弿杩拌鐗瑰畾寮曡剼鎺у埗鍣ㄦ墍澶勭悊鐨勫紩鑴氥€?
```

        A   B   C   D   E   F   G   H

   8    o   o   o   o   o   o   o   o

   7    o   o   o   o   o   o   o   o

   6    o   o   o   o   o   o   o   o

   5    o   o   o   o   o   o   o   o

   4    o   o   o   o   o   o   o   o

   3    o   o   o   o   o   o   o   o

   2    o   o   o   o   o   o   o   o

   1    o   o   o   o   o   o   o   o

```
瑕佹敞鍐屼竴涓紩鑴氭帶鍒跺櫒骞跺懡鍚嶆灏佽涓婄殑鎵€鏈夊紩鑴氾紝鎴戜滑鍙互鍦ㄦ垜浠殑椹卞姩涓繖鏍峰仛锛?

	#include <linux/pinctrl/pinctrl.h>

	const struct pinctrl_pin_desc foo_pins[] = {
		PINCTRL_PIN(0, "A8"),
		PINCTRL_PIN(1, "B8"),
		PINCTRL_PIN(2, "C8"),
		...
		PINCTRL_PIN(61, "F1"),
		PINCTRL_PIN(62, "G1"),
		PINCTRL_PIN(63, "H1"),
	};

	static struct pinctrl_desc foo_desc = {
		.name = "foo",
		.pins = foo_pins,
		.npins = ARRAY_SIZE(foo_pins),
		.owner = THIS_MODULE,
	};

	int __init foo_init(void)
	{
		int error;

		struct pinctrl_dev *pctl;

		error = pinctrl_register_and_init(&foo_desc, <PARENT>, NULL, &pctl);
		if (error)
			return error;

		return pinctrl_enable(pctl);
	}

瑕佸惎鐢?pinctrl 瀛愮郴缁熶互鍙?PINMUX 鍜?PINCONF 鐨勫瓙缁勫拰閫夊畾鐨勯┍鍔紝浣犻渶瑕?浠庝綘鏈哄櫒鐨?Kconfig 鏉＄洰涓€夋嫨瀹冧滑锛屽洜涓哄畠浠笌鎵€浣跨敤鏈哄櫒鐨勯泦鎴愰潪甯哥揣瀵嗐€?鍙傝 `arch/arm/mach-ux500/Kconfig` 浣滀负绀轰緥銆?
寮曡剼閫氬父姣旇繖鏈夋洿鑺卞摠鐨勫悕瀛椼€備綘鍙互鍦ㄨ姱鐗囩殑鏁版嵁鎵嬪唽涓壘鍒拌繖浜涖€傛敞鎰忔牳蹇?pinctrl.h 鏂囦欢鎻愪緵浜嗕竴涓悕涓?`PINCTRL_PIN()` 鐨勪究鎹峰畯鏉ュ垱寤虹粨鏋勪綋鏉＄洰銆傚浣犳墍瑙侊紝
寮曡剼浠庡乏涓婅鐨?0 鏋氫妇鍒板彸涓嬭鐨?63銆傝繖涓灇涓炬槸闅忔剰閫夋嫨鐨勶紝瀹為檯涓婁綘闇€瑕佹兂濂?浣犵殑缂栧彿绯荤粺锛屼娇鍏朵笌椹卞姩涓瘎瀛樺櫒绛変簨鐗╃殑甯冨眬鐩稿尮閰嶏紝鍚﹀垯浠ｇ爜鍙兘浼氬彉寰楀鏉傘€?浣犺繕蹇呴』鑰冭檻涓庡紩鑴氭帶鍒跺櫒鍙兘澶勭悊鐨?GPIO 鑼冨洿鐨勫亸绉诲尮閰嶃€?
瀵逛簬涓€涓湁 467 涓剨鐩橈紙pad锛夎€岄潪瀹為檯寮曡剼鐨勫皝瑁咃紝鏋氫妇灏嗗儚杩欐牱锛岀粫鐫€鑺墖鐨?杈圭紭璧帮紝杩欎技涔庢槸琛屼笟


     0 ..... 104
   466        105
     .        .
     .        .
   358        224
    357 .... 225


## 寮曡剼缁勶紙Pin groups锛?

璁稿鎺у埗鍣ㄩ渶瑕佸鐞嗗紩鑴氱粍锛屽洜姝ゅ紩鑴氭帶鍒跺櫒瀛愮郴缁熸湁涓€绉嶆満鍒舵潵鏋氫妇寮曡剼缁勫苟
妫€绱㈠睘浜庢煇涓壒瀹氱粍鐨勫疄闄呮灇涓惧紩鑴氥€?
渚嬪锛屽亣璁炬垜浠湁涓€涓鐞?SPI 鎺ュ彛鐨勫紩鑴氱粍锛屼綅浜?{ 0, 8, 16, 24 }锛屼互鍙婁竴涓?澶勭悊 I2C 鎺ュ彛鐨勫紩鑴氱粍锛屼綅浜?{ 24, 25 }銆?
杩欎袱涓粍閫氳繃瀹炵幇涓€浜涢€氱敤鐨?`pinctrl_ops` 鍛堢幇缁欏紩鑴氭帶鍒跺瓙绯荤粺锛屽儚杩欐牱锛?

	#include <linux/pinctrl/pinctrl.h>

	static const unsigned int spi0_pins[] = { 0, 8, 16, 24 };
	static const unsigned int i2c0_pins[] = { 24, 25 };

	static const struct pingroup foo_groups[] = {
		PINCTRL_PINGROUP("spi0_grp", spi0_pins, ARRAY_SIZE(spi0_pins)),
		PINCTRL_PINGROUP("i2c0_grp", i2c0_pins, ARRAY_SIZE(i2c0_pins)),
	};

	static int foo_get_groups_count(struct pinctrl_dev *pctldev)
	{
		return ARRAY_SIZE(foo_groups);
	}

	static const char **foo_get_group_name(struct pinctrl_dev **pctldev,
					      unsigned int selector)
	{
		return foo_groups[selector].name;
	}

	static int foo_get_group_pins(struct pinctrl_dev *pctldev,
				      unsigned int selector,
				      const unsigned int **pins,
				      unsigned int *npins)
	{
		*pins = foo_groups[selector].pins;
		*npins = foo_groups[selector].npins;
		return 0;
	}

	static struct pinctrl_ops foo_pctrl_ops = {
		.get_groups_count = foo_get_groups_count,
		.get_group_name = foo_get_group_name,
		.get_group_pins = foo_get_group_pins,
	};

	static struct pinctrl_desc foo_desc = {
		...
		.pctlops = &foo_pctrl_ops,
	};

寮曡剼鎺у埗瀛愮郴缁熷皢璋冪敤 `.get_groups_count()` 鍑芥暟鏉ョ‘瀹氬悎娉曢€夋嫨鍣紙selector锛夌殑鎬?鏁帮紝鐒跺悗瀹冨皢璋冪敤鍏朵粬鍑芥暟鏉ヨ幏鍙栫粍鐨勫悕瀛楀拰寮曡剼銆傜淮鎶ょ粍鐨勬暟鎹粨鏋勬槸椹卞姩鐨勮矗浠伙紝杩?鍙槸涓€涓畝鍗曠殑渚嬪瓙鈥斺€斿疄闄呬笂浣犲彲鑳介渶瑕佸湪浣犵殑缁勭粨鏋勪腑鍔犲叆鏇村鏉＄洰锛屼緥濡備笌姣忎釜缁?鍏宠仈鐨勭壒瀹氬瘎瀛樺櫒鑼冨洿绛夌瓑銆?

## 寮曡剼閰嶇疆锛圥in configuration锛?

寮曡剼鏈夋椂鍙互浠ュ悇绉嶆柟寮忚繘琛岃蒋浠堕厤缃紝涓昏涓庡叾鐢ㄤ綔杈撳叆鎴栬緭鍑烘椂鐨勭數瀛愮壒鎬х浉鍏炽€?渚嬪锛屼綘鍙互浣夸竴涓緭鍑哄紩鑴氫负楂橀樆锛圚i-Z锛夛紝鎴栤€滀笁鎬侊紙tristate锛夆€濓紝鎰忓懗鐫€瀹冨疄闄呬笂琚?鏂紑杩炴帴銆備綘鍙互浣跨敤鏌愪釜鐗瑰畾鐨勭數闃诲€煎皢杈撳叆寮曡剼杩炴帴鍒?VDD 鎴?GND鈥斺€斾笂鎷夊拰涓嬫媺鈥斺€?杩欐牱褰撴病鏈変笢瑗块┍鍔ㄥ畠鎵€杩炴帴鐨勭嚎璺紝鎴栬€呭畠鏈繛鎺ユ椂锛屽紩鑴氭湁涓€涓ǔ瀹氬€笺€?
寮曡剼閰嶇疆鍙互閫氳繃灏嗛厤缃潯鐩坊鍔犲埌鏄犲皠琛ㄤ腑鏉ョ紪绋嬶紱鍙傝涓嬫枃 `Board/machine
configuration`_ 涓€鑺傘€?
涓婇潰鎻愬埌鐨勯厤缃弬鏁?PLATFORM_X_PULL_UP 鐨勬牸寮忓拰鍚箟瀹屽叏鐢卞紩鑴氭帶鍒跺櫒椹卞姩瀹氫箟銆?
寮曡剼閰嶇疆椹卞姩瀹炵幇鐢ㄤ簬鏇存敼寮曡剼鎺у埗鍣?ops 涓紩鑴氶厤缃殑鍥炶皟锛屽儚杩欐牱锛?

	#include <linux/pinctrl/pinconf.h>
	#include <linux/pinctrl/pinctrl.h>

	#include "platform_x_pindefs.h"

	static int foo_pin_config_get(struct pinctrl_dev *pctldev,
				      unsigned int offset,
				      unsigned long *config)
	{
		struct my_conftype conf;

		/** ... Find setting for pin @ offset ... **/

		*config = (unsigned long) conf;
	}

	static int foo_pin_config_set(struct pinctrl_dev *pctldev,
				      unsigned int offset,
				      unsigned long config)
	{
		struct my_conftype **conf = (struct my_conftype **) config;

		switch (conf) {
			case PLATFORM_X_PULL_UP:
			...
			break;
		}
	}

	static int foo_pin_config_group_get(struct pinctrl_dev *pctldev,
					    unsigned selector,
					    unsigned long *config)
	{
		...
	}

	static int foo_pin_config_group_set(struct pinctrl_dev *pctldev,
					    unsigned selector,
					    unsigned long config)
	{
		...
	}

	static struct pinconf_ops foo_pconf_ops = {
		.pin_config_get = foo_pin_config_get,
		.pin_config_set = foo_pin_config_set,
		.pin_config_group_get = foo_pin_config_group_get,
		.pin_config_group_set = foo_pin_config_group_set,
	};

	/** Pin config operations are handled by some pin controller **/
	static struct pinctrl_desc foo_desc = {
		...
		.confops = &foo_pconf_ops,
	};

## 涓?GPIO 瀛愮郴缁熺殑浜や簰


GPIO 椹卞姩鍙兘鎯宠鍦ㄥ悓鏍锋敞鍐屼负寮曡剼鎺у埗鍣ㄥ紩鑴氱殑鐩稿悓鐗╃悊寮曡剼涓婃墽琛屽悇绉嶇被鍨嬬殑鎿嶄綔銆?
棣栧厛涓旀渶閲嶈鐨勬槸锛岃繖涓や釜瀛愮郴缁熷彲浠ュ畬鍏ㄦ浜ゅ湴浣跨敤锛屽弬瑙佸悕涓?`Pin control requests
from drivers`_ 鍜?`Drivers needing both pin control and GPIOs`_ 鐨勫皬鑺備互浜嗚В璇︽儏銆?浣嗗湪鏌愪簺鎯呭喌涓嬶紝寮曡剼鍜?GPIO 涔嬮棿鐨勮法瀛愮郴缁熸槧灏勬槸闇€瑕佺殑銆?
鐢变簬寮曡剼鎺у埗鍣ㄥ瓙绯荤粺鐨勫紩鑴氱┖闂达紙pinspace锛夊浜庡紩鑴氭帶鍒跺櫒鏄眬閮ㄧ殑锛屾垜浠渶瑕佷竴涓?鏄犲皠锛屼互渚垮紩鑴氭帶鍒跺瓙绯荤粺鑳藉寮勬竻妤氬摢涓紩鑴氭帶鍒跺櫒澶勭悊鏌愪釜 GPIO 寮曡剼鐨勬帶鍒躲€傜敱浜?鍗曚釜寮曡剼鎺у埗鍣ㄥ彲鑳芥鍦ㄥ璺鐢ㄥ涓?GPIO 鑼冨洿锛堥€氬父鏄繖鏍蜂竴绉?SoC锛氬畠鏈変竴缁勫紩鑴氾紝
浣嗗唴閮ㄦ湁澶氫釜 GPIO 纭呮ā鍧楋紝姣忎釜閮借寤烘ā涓轰竴涓?struct gpio_chip锛夛紝鍥犳鍙互灏嗕换鎰?鏁伴噺鐨?GPIO 鑼冨洿娣诲姞鍒板紩鑴氭帶鍒跺櫒瀹炰緥涓紝鍍忚繖鏍凤細


	#include <linux/gpio/driver.h>

	#include <linux/pinctrl/pinctrl.h>

	struct gpio_chip chip_a;
	struct gpio_chip chip_b;

	static struct pinctrl_gpio_range gpio_range_a = {
		.name = "chip a",
		.id = 0,
		.base = 32,
		.pin_base = 32,
		.npins = 16,
		.gc = &chip_a,
	};

	static struct pinctrl_gpio_range gpio_range_b = {
		.name = "chip b",
		.id = 0,
		.base = 48,
		.pin_base = 64,
		.npins = 8,
		.gc = &chip_b;
	};

	int __init foo_init(void)
	{
		struct pinctrl_dev *pctl;
		...
		pinctrl_add_gpio_range(pctl, &gpio_range_a);
		pinctrl_add_gpio_range(pctl, &gpio_range_b);
		...
	}

鍥犳杩欎釜澶嶆潅鐨勭郴缁熸湁涓€涓紩鑴氭帶鍒跺櫒澶勭悊涓や釜涓嶅悓鐨?GPIO 鑺墖銆傗€渃hip a鈥濇湁 16 涓?寮曡剼锛屸€渃hip b鈥濇湁 8 涓紩鑴氥€傗€渃hip a鈥濆拰鈥渃hip b鈥濇湁涓嶅悓鐨?`pin_base`锛岃繖鎰忓懗鐫€ GPIO
鑼冨洿鐨勮捣濮嬪紩鑴氬彿銆?
鈥渃hip a鈥濈殑 GPIO 鑼冨洿浠?GPIO 鍩哄潃 32 寮€濮嬶紝瀹為檯鐨勫紩鑴氳寖鍥翠篃浠?32 寮€濮嬨€傜劧鑰?鈥渃hip b鈥濈殑 GPIO 鑼冨洿鍜屽紩鑴氳寖鍥存湁涓嶅悓鐨勮捣濮嬪亸绉汇€傗€渃hip b鈥濈殑 GPIO 鑼冨洿浠?GPIO 缂栧彿
48 寮€濮嬶紝鑰屸€渃hip b鈥濈殑寮曡剼鑼冨洿浠?64 寮€濮嬨€?
鎴戜滑鍙互浣跨敤杩欎釜 `pin_base` 灏嗕竴涓?gpio 缂栧彿杞崲涓哄疄闄呯殑寮曡剼缂栧彿銆傚畠浠湪鍏ㄥ眬 GPIO
寮曡剼绌洪棿涓槧灏勪负锛?
chip a:
 - GPIO range : [32 .. 47]
 - pin range  : [32 .. 47]
chip b:
 - GPIO range : [48 .. 55]
 - pin range  : [64 .. 71]

涓婇潰鐨勪緥瀛愬亣璁?GPIO 涓庡紩鑴氫箣闂寸殑鏄犲皠鏄嚎鎬х殑銆傚鏋滄槧灏勬槸绋€鐤忕殑鎴栭殢鎰忕殑锛屼竴涓?浠绘剰寮曡剼缂栧彿鏁扮粍鍙互鍍忚繖鏍风紪鐮佸埌鑼冨洿涓細


	static const unsigned int range_pins[] = { 14, 1, 22, 17, 10, 8, 6, 2 };

	static struct pinctrl_gpio_range gpio_range = {
		.name = "chip",
		.id = 0,
		.base = 32,
		.pins = &range_pins,
		.npins = ARRAY_SIZE(range_pins),
		.gc = &chip,
	};

鍦ㄨ繖绉嶆儏鍐典笅锛宍pin_base` 灞炴€у皢琚拷鐣ャ€傚鏋滃凡鐭ヤ竴涓紩鑴氱粍鐨勫悕瀛楋紝涓婅堪缁撴瀯鐨?pins 鍜?npins 鍏冪礌鍙互浣跨敤鍑芥暟 `pinctrl_get_group_pins()` 鏉ュ垵濮嬪寲锛屼緥濡傚浜庡紩鑴氱粍 鈥渇oo鈥濓細


	pinctrl_get_group_pins(pctl, "foo", &gpio_range.pins, &gpio_range.npins);

褰撳紩鑴氭帶鍒跺瓙绯荤粺涓笌 GPIO 鐩稿叧鐨勫嚱鏁拌璋冪敤鏃讹紝杩欎簺鑼冨洿灏嗚鐢ㄦ潵閫氳繃妫€鏌ュ苟灏嗗紩鑴氫笌鎵€鏈?鎺у埗鍣ㄤ笂鐨勫紩鑴氳寖鍥磋繘琛屽尮閰嶆潵鏌ユ壘鍚堥€傜殑寮曡剼鎺у埗鍣ㄣ€傚綋鎵惧埌澶勭悊鍖归厤鑼冨洿鐨勫紩鑴氭帶鍒跺櫒鏃讹紝
灏嗗湪璇ョ壒瀹氬紩鑴氭帶鍒跺櫒涓婅皟鐢ㄤ笌 GPIO 鐩稿叧鐨勫嚱鏁般€?
瀵逛簬鎵€鏈夋秹鍙婂紩鑴氬亸缃€佸紩鑴氬璺鐢ㄧ瓑鐨勫姛鑳斤紝寮曡剼鎺у埗鍣ㄥ瓙绯荤粺灏嗛€氳繃浼犲叆鐨?gpio 缂栧彿
鏌ユ壘鐩稿簲鐨勫紩鑴氱紪鍙凤紝骞朵娇鐢ㄨ鑼冨洿鐨勫唴閮ㄦ潵妫€绱竴涓紩鑴氱紪鍙枫€備箣鍚庯紝瀛愮郴缁熷皢鍏朵紶閫掔粰寮曡剼
鎺у埗椹卞姩锛屼互渚块┍鍔ㄥ皢寰楀埌涓€涓湪鍏跺鐞嗙紪鍙疯寖鍥村唴鐨勫紩鑴氱紪鍙枫€傛澶栬繕浼氫紶閫掕寖鍥?ID 鍊硷紝浠ヤ究
寮曡剼鎺у埗鍣ㄧ煡閬撳畠搴旇澶勭悊鍝釜鑼冨洿銆?
浠?pinctrl 椹卞姩璋冪敤 `pinctrl_add_gpio_range()` 鏄?*宸插純鐢紙DEPRECATED锛?*鐨勩€傝鍙傞槄
`Documentation/devicetree/bindings/gpio/gpio.txt` 鐨勭 2.1 鑺傦紝浜嗚В濡備綍缁戝畾 pinctrl 鍜?gpio 椹卞姩銆?

## PINMUX 鎺ュ彛


杩欎簺璋冪敤浣跨敤 pinmux_* 鍛藉悕鍓嶇紑銆傚叾浠栬皟鐢ㄩ兘涓嶅簲浣跨敤璇ュ墠缂€銆?

## 浠€涔堟槸寮曡剼澶氳矾澶嶇敤锛坧inmuxing锛夛紵


PINMUX锛屼篃绉颁负 padmux銆乥allmux銆佸鐢ㄥ姛鑳斤紙alternate functions锛夋垨浠诲姟妯″紡锛坢ission
modes锛夛紝鏄敓浜ф煇绉嶇數姘斿皝瑁呯殑鑺墖鍘傚晢浣跨敤鏌愪釜鐗瑰畾鐗╃悊寮曡剼锛堢悆銆佺剨鐩樸€佺鑴氱瓑锛夋潵鐢ㄤ簬
澶氫釜浜掓枼鍔熻兘鐨勪竴绉嶆柟寮忥紝鍏蜂綋鍙栧喅浜庡簲鐢ㄣ€傚湪杩欎釜涓婁笅鏂囦腑锛屾垜浠墍璇寸殑鈥滃簲鐢ㄢ€濋€氬父鏄寚灏?灏佽鐒婃帴鎴栧竷绾垮埌鐢靛瓙绯荤粺涓殑涓€绉嶆柟寮忥紝灏界璇ユ鏋朵篃浣垮緱鍦ㄨ繍琛屾椂鏀瑰彉鍔熻兘鎴愪负鍙兘銆?
```

        A   B   C   D   E   F   G   H
      +---+
   8  | o | o   o   o   o   o   o   o
      |   |
   7  | o | o   o   o   o   o   o   o
      |   |
   6  | o | o   o   o   o   o   o   o
      +---+---+
   5  | o | o | o   o   o   o   o   o
      +---+---+               +---+
   4    o   o   o   o   o   o | o | o
                              |   |
   3    o   o   o   o   o   o | o | o
                              |   |
   2    o   o   o   o   o   o | o | o
      +-------+-------+-------+---+---+
   1  | o   o | o   o | o   o | o | o |
      +-------+-------+-------+---+---+

```
杩欎笉鏄縿缃楁柉鏂瑰潡銆傝鑱旀兂鐨勬父鎴忔槸璞℃銆傚苟闈炴墍鏈夌殑 PGA/BGA 灏佽閮藉儚妫嬬洏涓€鏍凤紝澶х殑灏佽
浼氭牴鎹笉鍚岀殑璁捐妯″紡鏈変竴浜涒€滅┖娲炩€濓紝浣嗘垜浠繖閲岀敤浣滀竴涓畝鍗曠殑渚嬪瓙銆傚湪浣犺兘鐪嬪埌鐨勫紩鑴氫腑锛?鏈変簺浼氳璇稿鍑犱釜 VCC 鍜?GND 鐢ㄦ潵缁欒姱鐗囦緵鐢碉紝杩樻湁鐩稿綋澶氫細琚ぇ鐨勭鍙ｏ紙濡傚閮ㄥ瓨鍌ㄥ櫒鎺ュ彛锛?鍗犵敤銆傚墿涓嬬殑寮曡剼閫氬父浼氬彈鍒板紩鑴氬璺鐢ㄧ殑褰卞搷銆?
涓婇潰杩欎釜 8x8 鐨?PGA 灏佽灏嗕负鍏剁墿鐞嗗紩鑴氬垎閰嶅紩鑴氱紪鍙?0 鍒?63銆傚畠灏嗕娇鐢?pinctrl_register_pins() 鍜屽鍓嶉潰鎵€绀虹殑涓€缁勫悎閫傛暟鎹紝灏嗗紩鑴氬懡鍚嶄负 { A1, A2, A3 ...
H6, H7, H8 }銆?
鍦ㄨ繖涓?8x8 鐨?BGA 灏佽涓紝寮曡剼 { A8, A7, A6, A5 } 鍙互鐢ㄤ綔涓€涓?SPI 绔彛锛堣繖鏄洓涓紩鑴氾細
CLK銆丷XD銆乀XD銆丗RM锛夈€傚湪杩欑鎯呭喌涓嬶紝寮曡剼 B5 鍙互鐢ㄤ綔鏌愪釜閫氱敤鐨?GPIO 寮曡剼銆傜劧鑰岋紝鍦ㄥ彟涓€绉?璁剧疆涓紝寮曡剼 { A5, B5 } 鍙互鐢ㄤ綔涓€涓?I2C 绔彛锛堣繖鍙槸涓や釜寮曡剼锛歋CL銆丼DA锛夈€備笉鐢ㄨ锛屾垜浠?涓嶈兘鍚屾椂浣跨敤璇?SPI 绔彛鍜?I2C 绔彛銆傜劧鑰屽湪灏佽鍐呴儴锛屾墽琛?SPI 閫昏緫鐨勭鍙互鏀归亾杈撳嚭鍒板紩鑴?{ G4, G3, G2, G1 }銆?
鍦ㄦ渶涓嬮潰涓€琛?{ A1, B1, C1, D1, E1, F1, G1, H1 } 鎴戜滑鏈変竴浜涚壒鍒殑涓滆タ鈥斺€斿畠鏄竴涓閮ㄧ殑
MMC 鎬荤嚎锛屽彲浠ユ槸 2銆? 鎴?8 浣嶅锛屽苟浼氬垎鍒秷鑰?2銆? 鎴?8 涓紩鑴氾紝鍥犳瑕佷箞鍗犵敤 { A1, B1 }锛?瑕佷箞鍗犵敤 { A1, B1, C1, D1 }锛岃涔堝叏閮ㄥ崰鐢ㄣ€傚鏋滄垜浠娇鐢ㄥ叏閮?8 浣嶏紝鎴戜滑褰撶劧灏变笉鑳戒娇鐢?寮曡剼 { G4, G3, G2, G1 } 涓婄殑 SPI 绔彛銆?
閫氳繃杩欑鏂瑰紡锛岃姱鐗囧唴閮ㄥ瓨鍦ㄧ殑纭呮ā鍧楀彲浠ヨ澶氳矾澶嶇敤锛堚€渕uxed鈥濓級杈撳嚭鍒颁笉鍚岀殑寮曡剼鑼冨洿銆傚綋浠?SoC锛堢墖涓婄郴缁燂級閫氬父鍖呭惈澶氫釜 I2C銆丼PI銆丼DIO/MMC 绛夌妯″潡锛屽彲浠ラ€氳繃 pinmux 璁剧疆璺敱鍒颁笉鍚岀殑
寮曡剼銆?
鐢变簬閫氱敤杈撳叆/杈撳嚭寮曡剼锛圙PIO锛夐€氬父鎬绘槸鐭己锛岄€氬父濡傛灉鏌愪釜寮曡剼褰撳墠娌℃湁琚叾浠?I/O 绔彛浣跨敤锛?灏卞彲浠ュ皢瀹冪敤浣?GPIO 寮曡剼銆?

## Pinmux 绾﹀畾


寮曡剼鎺у埗鍣ㄥ瓙绯荤粺涓?pinmux 鍔熻兘鐨勭洰鐨勬槸锛屼负浣犻€夋嫨鍦ㄦ満鍣ㄩ厤缃腑瀹炰緥鍖栫殑璁惧鎶借薄骞舵彁渚?pinmux 璁剧疆銆傚畠鍙楀埌 clk銆丟PIO 鍜?regulator 瀛愮郴缁熺殑鍚彂锛屽洜姝よ澶囧皢璇锋眰瀹冧滑鐨?mux 璁剧疆锛?浣嗕篃鍙互涓轰緥濡?GPIO 璇锋眰鍗曚釜寮曡剼銆?
绾﹀畾濡備笅锛?
- 鍑芥暟锛團UNCTION锛夊彲浠ョ敱椹荤暀鍦ㄥ唴鏍?`drivers/pinctrl` 鐩綍涓殑寮曡剼鎺у埗瀛愮郴缁熷唴鐨勯┍鍔?  鍒囨崲杩涘嚭銆傚紩鑴氭帶鍒堕┍鍔ㄧ煡閬撳彲鑳界殑鍔熻兘銆傚湪涓婇潰鐨勪緥瀛愪腑锛屼綘鍙互璇嗗埆鍑轰笁涓?pinmux 鍔熻兘锛?  涓€涓敤浜?spi锛屼竴涓敤浜?i2c锛屼竴涓敤浜?mmc銆?
- 鍑芥暟锛團UNCTION锛夎鍋囧畾涓轰粠涓€涓竴缁存暟缁勪腑浠庨浂寮€濮嬪彲鏋氫妇銆傚湪杩欑鎯呭喌涓嬶紝鏁扮粍鍙兘鏄儚
  { spi0, i2c0, mmc0 } 杩欐牱鐨勪笢瑗匡紝瀵瑰簲涓変釜鍙敤鐨勫姛鑳姐€?
- 鍑芥暟锛團UNCTION锛夊叿鏈夊湪閫氱敤灞傞潰瀹氫箟鐨勫紩鑴氱粍锛圥IN GROUP锛夆€斺€斿洜姝ゆ煇涓壒瀹氬嚱鏁?*鎬绘槸**
  涓庢煇涓壒瀹氱殑寮曡剼缁勯泦鍚堢浉鍏宠仈锛屽彲鑳藉彧鏈変竴涓紝浣嗕篃鍙兘鏈夊緢澶氥€傚湪涓婇潰鐨勪緥瀛愪腑锛屽嚱鏁?i2c
  涓庡紩鑴?{ A5, B5 } 鐩稿叧鑱旓紝鍦ㄦ帶鍒跺櫒寮曡剼绌洪棿涓灇涓句负 { 24, 25 }銆?
  鍑芥暟 spi 涓庡紩鑴氱粍 { A8, A7, A6, A5 } 鍜?{ G4, G3, G2, G1 } 鐩稿叧鑱旓紝鍒嗗埆鏋氫妇涓?  { 0, 8, 16, 24 } 鍜?{ 38, 46, 54, 62 }銆?
  缁勫悕瀵逛簬姣忎釜寮曡剼鎺у埗鍣ㄥ繀椤绘槸鍞竴鐨勶紝鍚屼竴涓帶鍒跺櫒涓婁笉鑳芥湁涓や釜鍚屽悕鐨勭粍銆?
- 鍑芥暟锛團UNCTION锛夊拰寮曡剼缁勶紙PIN GROUP锛夌殑缁勫悎鍐冲畾浜嗘煇缁勫紩鑴氱殑鏌愪釜鐗瑰畾鍔熻兘銆傚嚱鏁板拰寮曡剼缁?  鍙婂叾鏈哄櫒鐗瑰畾缁嗚妭鐨勭煡璇嗕繚瀛樺湪 pinmux 椹卞姩鍐呴儴锛屼粠澶栭儴鍙煡閬撴灇涓惧櫒锛岄┍鍔ㄦ牳蹇冨彲浠ヨ姹傦細

  - 鍏锋湁鏌愪釜閫夋嫨鍣紙>= 0锛夌殑鍑芥暟鍚?  - 涓庢煇涓壒瀹氬嚱鏁板叧鑱旂殑缁勫垪琛?  - 璇ュ垪琛ㄤ腑鏌愪釜鐗瑰畾缁勮婵€娲讳互鐢ㄤ簬鏌愪釜鐗瑰畾鍑芥暟

  濡備笂鎵€杩帮紝寮曡剼缁勬湰韬張鏄嚜鎻忚堪鐨勶紝鍥犳鏍稿績灏嗕粠椹卞姩涓绱㈡煇涓粍涓疄闄呯殑寮曡剼鑼冨洿銆?
- 鏌愪釜寮曡剼鎺у埗鍣ㄤ笂鐨勫嚱鏁帮紙FUNCTION锛夊拰缁勶紙GROUP锛夐€氳繃鏉挎枃浠躲€佽澶囨爲鎴栫被浼肩殑鏈哄櫒璁剧疆閰嶇疆
  鏈哄埗琚槧灏勶紙MAP锛夊埌鏌愪釜鐗瑰畾璁惧锛岀被浼间簬 regulator 濡備綍杩炴帴鍒拌澶囷紝閫氬父鎸夊悕瀛椼€傚畾涔夊紩鑴?  鎺у埗鍣ㄣ€佸嚱鏁板拰缁勪粠鑰屽敮涓€鏍囪瘑鏌愪釜璁惧瑕佷娇鐢ㄧ殑寮曡剼闆嗗悎銆傦紙濡傛灉璇ュ嚱鏁板彧鏈変竴涓彲鑳界殑寮曡剼缁?  鍙敤锛屽垯鏃犻渶鎻愪緵缁勫悕鈥斺€旀牳蹇冨皢绠€鍗曞湴閫夋嫨绗竴涓篃鏄敮涓€鍙敤鐨勭粍銆傦級

  鍦ㄤ緥瀛愪腑锛屾垜浠彲浠ュ畾涔夎繖鍙扮壒瀹氱殑鏈哄櫒搴斾娇鐢ㄨ澶?spi0锛岄厤鍚?pinmux 鍑芥暟 fspi0銆佺粍 gspi0锛?  浠ュ強 i2c0锛岄厤鍚堝嚱鏁?fi2c0銆佺粍 gi2c0锛屽湪涓诲紩鑴氭帶鍒跺櫒涓婏紝鎴戜滑寰楀埌濡備笅鏄犲皠锛?
  .. code-block:: c

	{
		{"map-spi0", spi0, pinctrl0, fspi0, gspi0},
		{"map-i2c0", i2c0, pinctrl0, fi2c0, gi2c0},
	}

  姣忎釜鏄犲皠閮藉繀椤昏鍒嗛厤涓€涓姸鎬佸悕銆佸紩鑴氭帶鍒跺櫒銆佽澶囧拰鍑芥暟銆傜粍涓嶆槸寮哄埗鐨勨€斺€斿鏋滅渷鐣ワ紝椹卞姩
  鎵€鍛堢幇鐨勯€傜敤浜庤鍑芥暟绗竴涓粍灏嗚閫変腑锛岃繖瀵圭畝鍗曟儏鍐靛緢鏈夌敤銆?
  鍙互灏嗗涓粍鏄犲皠鍒扮浉鍚岀殑璁惧銆佸紩鑴氭帶鍒跺櫒鍜屽嚱鏁扮殑缁勫悎銆傝繖鏄拡瀵规煇涓紩鑴氭帶鍒跺櫒涓婄殑鏌愪釜
  鐗瑰畾鍔熻兘鍦ㄤ笉鍚岄厤缃腑鍙互浣跨敤涓嶅悓寮曡剼闆嗗悎鐨勬儏鍐点€?
- 鏌愪釜寮曡剼鎺у埗鍣ㄤ笂銆佷娇鐢ㄦ煇涓紩鑴氱粍鐨勬煇涓嚱鏁帮紙FUNCTION锛夌殑寮曡剼锛圥INS锛夋寜鍏堝埌鍏堝緱锛?  first-come first-serve锛夌殑鍘熷垯鎻愪緵锛屽洜姝ゅ鏋滄煇涓叾浠栬澶?mux 璁剧疆鎴?GPIO 寮曡剼璇锋眰宸茬粡
  鍗犵敤浜嗕綘鐨勭墿鐞嗗紩鑴氾紝浣犲皢鏃犳硶浣跨敤瀹冦€傝鑾峰彇锛堟縺娲伙級涓€涓柊璁剧疆锛屽繀椤诲厛灏嗘棫鐨勯噴鏀撅紙鍋滅敤锛夈€?
鏈夋椂鏂囨。鍜岀‖浠跺瘎瀛樺櫒浼氬洿缁曠剨鐩橈紙pad锛屾垨鈥滅鑴?finger鈥濓級鑰屼笉鏄紩鑴氭潵缁勭粐鈥斺€旇繖浜涙槸灏佽鍐?纭呬笂鐨勭剨鎺ラ潰锛屽彲鑳戒笌澶栧３涓嬮潰鐨勫疄闄呭紩鑴?鐞冩暟閲忓尮閰嶆垨涓嶅尮閰嶃€傞€夋嫨瀵逛綘鏈夋剰涔夌殑鏌愮鏋氫妇鏂瑰紡銆?濡傛灉璁插緱閫氾紝鍙负浣犺兘澶熸帶鍒剁殑寮曡剼瀹氫箟鏋氫妇鍣ㄣ€?
鍋囪锛?
鎴戜滑鍋囪鍙兘鐨勫姛鑳藉埌寮曡剼缁勭殑鏄犲皠鏁伴噺鍙楃‖浠堕檺鍒躲€傚嵆鎴戜滑鍋囪涓嶅瓨鍦ㄦ煇涓姛鑳藉彲浠ヨ鏄犲皠鍒颁换浣?寮曡剼鐨勭郴缁燂紝灏卞儚鐢佃瘽浜ゆ崲鏈洪偅鏍枫€傚洜姝ゆ煇涓壒瀹氬姛鑳界殑鍙敤寮曡剼缁勫皢闄愪簬灏戞暟鍑犵閫夋嫨锛堟瘮濡傛渶澶?鍏釜宸﹀彸锛夛紝鑰屼笉鏄暟鐧剧鎴栦换鎰忔暟閲忕殑閫夋嫨銆傝繖鏄垜浠€氳繃妫€鏌ュ彲鐢ㄧ殑 pinmux 纭欢鎵€鍙戠幇鐨勭壒鎬э紝
骞朵笖鏄竴涓繀瑕佺殑鍋囪锛屽洜涓烘垜浠湡鏈?pinmux 椹卞姩鍚戝瓙绯荤粺鍛堢幇**鎵€鏈?*鍙兘鐨勫姛鑳戒笌寮曡剼缁勭殑
鏄犲皠銆?

## Pinmux 椹卞姩


pinmux 鏍稿績璐熻矗闃叉寮曡剼涓婄殑鍐茬獊锛屽苟璋冪敤寮曡剼鎺у埗鍣ㄩ┍鍔ㄦ潵鎵ц涓嶅悓鐨勮缃€?
pinmux 椹卞姩鏈夎矗浠绘柦鍔犺繘涓€姝ョ殑闄愬埗锛堜緥濡傛帹鏂敱浜庤礋杞界瓑甯︽潵鐨勭數瀛愰檺鍒讹級锛屼互纭畾鎵€璇锋眰鐨?鍔熻兘鏄惁纭疄琚厑璁革紝骞朵笖鍦ㄥ彲浠ユ墽琛屾墍璇锋眰鐨?mux 璁剧疆鐨勬儏鍐典笅锛屽幓鎷ㄥ姩锛坧oke锛夌‖浠朵互浣垮叾鍙戠敓銆?
Pinmux 椹卞姩闇€瑕佹彁渚涗竴浜涘洖璋冨嚱鏁帮紝鏈変簺鏄彲閫夌殑銆傞€氬父瀹炵幇 `.set_mux()` 鍑芥暟锛屽皢鍊煎啓鍏ユ煇浜?鐗瑰畾瀵勫瓨鍣ㄤ互婵€娲绘煇涓紩鑴氱殑鐗瑰畾 mux 璁剧疆銆?
瀵逛笂杩颁緥瀛愮殑涓€涓畝鍗曢┍鍔ㄥ皢閫氳繃璁剧疆浣?0銆?銆?銆?銆? 鎴?5 鍒版煇涓悕涓?MUX 鐨勫瘎瀛樺櫒鏉ラ€夋嫨鏌愪釜
鍏锋湁鐗瑰畾寮曡剼缁勭殑鍔熻兘锛屽ぇ鑷村儚杩欐牱锛?

	#include <linux/pinctrl/pinctrl.h>
	#include <linux/pinctrl/pinmux.h>

	static const unsigned int spi0_0_pins[] = { 0, 8, 16, 24 };
	static const unsigned int spi0_1_pins[] = { 38, 46, 54, 62 };
	static const unsigned int i2c0_pins[] = { 24, 25 };
	static const unsigned int mmc0_1_pins[] = { 56, 57 };
	static const unsigned int mmc0_2_pins[] = { 58, 59 };
	static const unsigned int mmc0_3_pins[] = { 60, 61, 62, 63 };

	static const struct pingroup foo_groups[] = {
		PINCTRL_PINGROUP("spi0_0_grp", spi0_0_pins, ARRAY_SIZE(spi0_0_pins)),
		PINCTRL_PINGROUP("spi0_1_grp", spi0_1_pins, ARRAY_SIZE(spi0_1_pins)),
		PINCTRL_PINGROUP("i2c0_grp", i2c0_pins, ARRAY_SIZE(i2c0_pins)),
		PINCTRL_PINGROUP("mmc0_1_grp", mmc0_1_pins, ARRAY_SIZE(mmc0_1_pins)),
		PINCTRL_PINGROUP("mmc0_2_grp", mmc0_2_pins, ARRAY_SIZE(mmc0_2_pins)),
		PINCTRL_PINGROUP("mmc0_3_grp", mmc0_3_pins, ARRAY_SIZE(mmc0_3_pins)),
	};

	static int foo_get_groups_count(struct pinctrl_dev *pctldev)
	{
		return ARRAY_SIZE(foo_groups);
	}

	static const char **foo_get_group_name(struct pinctrl_dev **pctldev,
					      unsigned int selector)
	{
		return foo_groups[selector].name;
	}

	static int foo_get_group_pins(struct pinctrl_dev *pctldev, unsigned int selector,
				      const unsigned int **pins,
				      unsigned int *npins)
	{
		*pins = foo_groups[selector].pins;
		*npins = foo_groups[selector].npins;
		return 0;
	}

	static struct pinctrl_ops foo_pctrl_ops = {
		.get_groups_count = foo_get_groups_count,
		.get_group_name = foo_get_group_name,
		.get_group_pins = foo_get_group_pins,
	};

	static const char * const spi0_groups[] = { "spi0_0_grp", "spi0_1_grp" };
	static const char * const i2c0_groups[] = { "i2c0_grp" };
	static const char * const mmc0_groups[] = { "mmc0_1_grp", "mmc0_2_grp", "mmc0_3_grp" };

	static const struct pinfunction foo_functions[] = {
		PINCTRL_PINFUNCTION("spi0", spi0_groups, ARRAY_SIZE(spi0_groups)),
		PINCTRL_PINFUNCTION("i2c0", i2c0_groups, ARRAY_SIZE(i2c0_groups)),
		PINCTRL_PINFUNCTION("mmc0", mmc0_groups, ARRAY_SIZE(mmc0_groups)),
	};

	static int foo_get_functions_count(struct pinctrl_dev *pctldev)
	{
		return ARRAY_SIZE(foo_functions);
	}

	static const char **foo_get_fname(struct pinctrl_dev **pctldev, unsigned int selector)
	{
		return foo_functions[selector].name;
	}

	static int foo_get_groups(struct pinctrl_dev *pctldev, unsigned int selector,
				  const char * const **groups,
				  unsigned int * const ngroups)
	{
		*groups = foo_functions[selector].groups;
		*ngroups = foo_functions[selector].ngroups;
		return 0;
	}

	static int foo_set_mux(struct pinctrl_dev *pctldev, unsigned int selector,
			       unsigned int group)
	{
		u8 regbit = BIT(group);

		writeb((readb(MUX) | regbit), MUX);
		return 0;
	}

	static struct pinmux_ops foo_pmxops = {
		.get_functions_count = foo_get_functions_count,
		.get_function_name = foo_get_fname,
		.get_function_groups = foo_get_groups,
		.set_mux = foo_set_mux,
		.strict = true,
	};

	/** Pinmux operations are handled by some pin controller **/
	static struct pinctrl_desc foo_desc = {
		...
		.pctlops = &foo_pctrl_ops,
		.pmxops = &foo_pmxops,
	};

鍦ㄤ緥瀛愪腑锛屽悓鏃舵縺娲?muxing 0 鍜?2 璁剧疆浣?0 鍜?2锛屽叡鐢ㄤ簡寮曡剼 24锛屽洜姝ゅ畠浠細鍐茬獊銆傚浜?muxes 1 鍜?5 涔熶竴鏍凤紝瀹冧滑鍏辩敤浜嗗紩鑴?62銆?
pinmux 瀛愮郴缁熺殑缇庡涔嬪鍦ㄤ簬锛岀敱浜庡畠璺熻釜鎵€鏈夊紩鑴氫互鍙婅皝鍦ㄤ娇鐢ㄥ畠浠紝瀹冩棭宸叉嫆缁濅簡杩欐牱涓€涓?涓嶅彲鑳界殑璇锋眰锛屽洜姝ら┍鍔ㄦ棤闇€鎷呭績杩欑被浜嬫儏鈥斺€斿綋瀹冧紶鍏ヤ竴涓€夋嫨鍣ㄦ椂锛宲inmux 瀛愮郴缁熺‘淇濇病鏈夊叾浠?璁惧鎴?GPIO 鍒嗛厤宸茬粡鍦ㄤ娇鐢ㄦ墍閫夌殑寮曡剼銆傚洜姝ゆ帶鍒跺瘎瀛樺櫒涓殑浣?0 鍜?2锛屾垨 1 鍜?5锛屾案杩滀笉浼氳
鍚屾椂璁剧疆銆?
浠ヤ笂鎵€鏈夊嚱鏁板浜?pinmux 椹卞姩鏉ヨ閮芥槸蹇呴』瀹炵幇鐨勩€?

## 寮曡剼鎺у埗涓?GPIO 瀛愮郴缁熺殑浜や簰


娉ㄦ剰锛屼互涓嬪唴瀹规殫绀轰簡浣跨敤鍦烘櫙鏄湪 Linux 鍐呮牳涓娇鐢?`<linux/gpio/consumer.h>` 涓殑 API锛?閰嶅悎 gpiod_get() 鍙婄被浼煎嚱鏁般€傛湁浜涙儏鍐典笅浣犲彲鑳芥鍦ㄤ娇鐢ㄤ綘鐨勬暟鎹墜鍐岀О涓衡€淕PIO 妯″紡鈥濈殑涓滆タ锛?浣嗗疄闄呬笂鍙槸鏌愪釜璁惧鐨勭數姘旈厤缃€傚弬瑙佷笅鏂囩殑 `GPIO mode pitfalls`_ 灏忚妭锛屼簡瑙ｆ洿澶氬叧浜庢
鍦烘櫙鐨勭粏鑺傘€?
鍏叡 pinmux API 鍖呭惈涓や釜鍚嶄负 `pinctrl_gpio_request()` 鍜?`pinctrl_gpio_free()` 鐨勫嚱鏁般€?杩欎袱涓嚱鏁?*鍙兘**浠庡熀浜?gpiolib 鐨勯┍鍔ㄤ腑璋冪敤锛屼綔涓哄畠浠殑 `.request()` 鍜?`.free()` 璇箟
鐨勪竴閮ㄥ垎銆傚悓鏍峰湴锛宍pinctrl_gpio_direction_input()` / `pinctrl_gpio_direction_output()`
鍙兘鍒嗗埆鍦ㄥ悇 gpiolib 鐨?`.direction_input()` / `.direction_output()` 瀹炵幇鍐呴儴璋冪敤銆?
娉ㄦ剰锛屽钩鍙板拰鍚勪釜椹卞姩**涓嶅簲**璇锋眰 GPIO 寮曡剼鍙楁帶锛屼緥濡傝 mux 杩涙潵銆傜浉鍙嶏紝瀹炵幇涓€涓悎閫傜殑
gpiolib 椹卞姩锛屽苟璁╄椹卞姩涓哄畠鐨勫紩鑴氳姹傚悎閫傜殑 muxing 鍜屽叾浠栨帶鍒躲€?
鍑芥暟鍒楄〃鍙兘浼氬彉寰楀緢闀匡紝鐗瑰埆鏄鏋滀綘鑳藉皢姣忎釜鍗曠嫭鐨勫紩鑴氳浆鎹负涓€涓?GPIO 寮曡剼鑰屼笉渚濊禆浜庝换浣曞叾浠?寮曡剼锛岀劧鍚庡皾璇曠敤瀹氫箟姣忎釜寮曡剼涓轰竴涓嚱鏁扮殑鏂规硶銆?
鍦ㄨ繖绉嶆儏鍐典笅锛屽嚱鏁版暟缁勫皢鍙樻垚姣忎釜 GPIO 璁剧疆鏈?64 涓潯鐩紝鐒跺悗鏄澶囧姛鑳姐€?
鍥犳鏈変袱涓嚱鏁板彲渚涘紩鑴氭帶鍒堕┍鍔ㄥ疄鐜帮紝浠ヤ粎鍦ㄥ崟涓紩鑴氫笂鍚敤 GPIO锛歚.gpio_request_enable()`
鍜?`.gpio_disable_free()`銆?
杩欎釜鍑芥暟灏嗕紶鍏ョ敱寮曡剼鎺у埗鍣ㄦ牳蹇冩爣璇嗙殑鍙楀奖鍝嶇殑 GPIO 鑼冨洿锛屽洜姝や綘鐭ラ亾鍝簺 GPIO 寮曡剼鍙楀埌璇?璇锋眰鎿嶄綔鐨勫奖鍝嶃€?
濡傛灉浣犵殑椹卞姩闇€瑕佹潵鑷鏋剁殑鍏充簬 GPIO 寮曡剼搴旂敤浜庤緭鍏ヨ繕鏄緭鍑虹殑鎸囩ず锛屼綘鍙互瀹炵幇
`.gpio_set_direction()` 鍑芥暟銆傚鍓嶆墍杩帮紝杩欏皢浠?gpiolib 椹卞姩涓璋冪敤锛屽彈褰卞搷鐨?GPIO 鑼冨洿銆?寮曡剼鍋忕Щ鍜屾湡鏈涙柟鍚戝皢琚紶閫掔粰璇ュ嚱鏁般€?
浣滀负浣跨敤杩欎簺鐗规畩鍑芥暟鐨勬浛浠ｆ柟妗堬紝瀹屽叏鍏佽涓烘瘡涓?GPIO 寮曡剼浣跨敤鍛藉悕鍑芥暟锛宍pinctrl_gpio_request()`
灏嗗皾璇曡幏鍙栧悕涓?鈥済pioN鈥?鐨勫嚱鏁帮紝鍏朵腑 鈥淣鈥?鏄叏灞€ GPIO 寮曡剼缂栧彿锛屽墠鎻愭槸娌℃湁娉ㄥ唽鐗规畩鐨?GPIO 澶勭悊鍣ㄣ€?

## GPIO 妯″紡闄烽槺锛圙PIO mode pitfalls锛?

鐢变簬纭欢宸ョ▼甯堜娇鐢ㄧ殑鍛藉悕绾﹀畾锛屽叾涓?鈥淕PIO鈥?鐨勫惈涔変笌鍐呮牳鎵€鍋氱殑涓嶅悓锛屽紑鍙戣€呭彲鑳戒細琚暟鎹墜鍐?璋堣鏌愪釜寮曡剼鍙互琚缃负 鈥淕PIO 妯″紡鈥?鎵€鍥版儜銆傜湅璧锋潵纭欢宸ョ▼甯堟墍璇寸殑 鈥淕PIO 妯″紡鈥?骞朵笉涓€瀹?鏄唴鏍告帴鍙?`<linux/gpio/consumer.h>` 鎵€鏆楃ず鐨勪娇鐢ㄥ満鏅細涓€涓綘浠庡唴鏍镐唬鐮佷腑鎶撳彇銆佺劧鍚庤涔?鐩戝惉杈撳叆銆佽涔堥┍鍔ㄩ珮/浣庝互鏂█/鍙栨秷鏂█鏌愪釜澶栭儴绾胯矾鐨勫紩鑴氥€?
鐩稿弽锛岀‖浠跺伐绋嬪笀璁や负 鈥淕PIO 妯″紡鈥?鎰忓懗鐫€浣犲彲浠ヨ蒋浠舵帶鍒跺紩鑴氱殑涓€浜涚數姘旂壒鎬э紝鑰屽鏋滃紩鑴氬浜?鍏朵粬妯″紡锛堜緥濡傝 mux 杩涙煇涓澶囷級鏃朵綘灏嗘棤娉曟帶鍒惰繖浜涚壒鎬с€?
涓€涓紩鑴氱殑 GPIO 閮ㄥ垎鍙婂叾涓庢煇涓紩鑴氭帶鍒跺櫒閰嶇疆鍜?muxing 閫昏緫鐨勫叧绯诲彲浠ョ敤鍑犵鏂瑰紡鏋勫缓銆傝繖閲?鏈変袱涓緥瀛愩€?
```

                       pin config
                       logic regs
                       |               +- SPI
     Physical pins --- pad --- pinmux -+- I2C
                               |       +- mmc
                               |       +- GPIO
                               pin
                               multiplex
                               logic regs

```
杩欓噷寮曡剼鐨勪竴浜涚數姘旂壒鎬ф棤璁哄紩鑴氭槸鍚︾敤浜?GPIO 閮藉彲浠ヨ閰嶇疆銆傚鏋滀綘灏嗕竴涓?GPIO 澶氳矾澶嶇敤鍒颁竴涓?寮曡剼涓婏紝浣犱篃鍙互浠?鈥淕PIO鈥?瀵勫瓨鍣ㄩ┍鍔ㄥ畠楂?浣庛€傛垨鑰咃紝璇ュ紩鑴氬彲浠ヨ鏌愪釜鐗瑰畾澶栬鎺у埗锛屽悓鏃朵粛鐒?搴旂敤鎵€闇€鐨勫紩鑴氶厤缃睘鎬с€傚洜姝?GPIO 鍔熻兘涓庝娇鐢ㄨ寮曡剼鐨勪换浣曞叾浠栬澶囨槸姝ｄ氦鐨勩€?
鍦ㄨ繖绉嶅畨鎺掍腑锛屽紩鑴氭帶鍒跺櫒鐨?GPIO 閮ㄥ垎瀵勫瓨鍣紝鎴?GPIO 纭欢妯″潡鐨勫瘎瀛樺櫒锛屽彲鑳戒綅浜庝竴涓粎渚?GPIO
椹卞姩鐨勫崟鐙唴瀛樿寖鍥翠腑锛岃€屽鐞嗗紩鑴氶厤缃拰寮曡剼澶氳矾澶嶇敤鐨勫瘎瀛樺櫒鑼冨洿琚斁鍦ㄤ竴涓笉鍚岀殑鍐呭瓨鑼冨洿鍜屾暟鎹?鎵嬪唽鐨勪笉鍚岀珷鑺備腑銆?
鍦?struct pinmux_ops 涓湁涓€涓爣蹇?鈥渟trict鈥濓紝鍙敤浜庢鏌ュ拰鎷掔粷鏉ヨ嚜 GPIO 鍜屽紩鑴氬璺鐢?娑堣垂鑰呭湪鍚屼竴绫诲瀷纭欢涓婂悓鏃惰闂悓涓€涓紩鑴氥€俻inctrl 椹卞姩搴旂浉搴斿湴璁剧疆姝ゆ爣蹇椼€?
```

                       pin config
                       logic regs
                       |               +- SPI
     Physical pins --- pad --- pinmux -+- I2C
                       |       |       +- mmc
                       |       |
                       GPIO    pin
                               multiplex
                               logic regs

```
鍦ㄨ繖绉嶅畨鎺掍腑锛孏PIO 鍔熻兘鎬绘槸鍙互琚惎鐢紝渚嬪涓€涓?GPIO 杈撳叆鍙互鐢ㄦ潵鈥滅鎺紙spy锛夆€濇鍦ㄨ剦鍐茶緭鍑?鐨?SPI/I2C/MMC 淇″彿銆傜敱浜庡畠浠庢湭鐪熸鏂紑杩炴帴锛岄€氳繃 GPIO 鍧椾笂鍋氶敊浜嬫潵骞叉壈寮曡剼涓婄殑娴侀噺鏄彲鑳?鐨勩€侴PIO銆佸紩鑴氶厤缃拰寮曡剼澶氳矾澶嶇敤瀵勫瓨鍣ㄥ彲鑳借鏀惧湪鍚屼竴涓唴瀛樿寖鍥村拰鏁版嵁鎵嬪唽鐨勫悓涓€涓珷鑺備腑锛屽敖绠?骞朵笉涓€瀹氶潪寰楀姝ゃ€?
鍦ㄤ竴浜涘紩鑴氭帶鍒跺櫒涓紝铏界劧鐗╃悊寮曡剼鐨勮璁′笌锛圔锛夌浉鍚岋紝浣?GPIO 鍔熻兘浠嶇劧涓嶈兘涓庡璁惧姛鑳藉悓鏃跺惎鐢ㄣ€?鍥犳鍚屾牱搴旇璁剧疆 鈥渟trict鈥?鏍囧織锛屾嫆缁?GPIO 鍜屽叾浠?mux 杩涙潵鐨勮澶囧悓鏃舵縺娲汇€?
鐒惰€岋紝浠庡唴鏍哥殑瑙掑害鏉ョ湅锛岃繖浜涙槸纭欢鐨勪笉鍚屾柟闈紝搴旇琚斁鍒颁笉鍚岀殑瀛愮郴缁熶腑锛?
- 鎺у埗寮曡剼鐢垫皵鐗规€э紙濡傚亸缃拰椹卞姩寮哄害锛夌殑瀵勫瓨鍣紙鎴栧瘎瀛樺櫒涓殑瀛楁锛夊簲璇ラ€氳繃 pinctrl 瀛愮郴缁?  浣滀负鈥滃紩鑴氶厤缃紙pin configuration锛夆€濊缃毚闇层€?
- 鎺у埗鏉ヨ嚜鍚勭鍏朵粬纭欢妯″潡锛堜緥濡?I2C銆丮MC 鎴?GPIO锛夌殑淇″彿鍒板紩鑴氱殑澶氳矾澶嶇敤鐨勫瘎瀛樺櫒锛堟垨瀵勫瓨鍣?  涓殑瀛楁锛夊簲璇ラ€氳繃 pinctrl 瀛愮郴缁熶綔涓?mux 鍔熻兘鏆撮湶銆?
- 鎺у埗 GPIO 鍔熻兘锛堜緥濡傝缃?GPIO 鐨勮緭鍑哄€笺€佽鍙?GPIO 鐨勮緭鍏ュ€硷紝鎴栬缃?GPIO 寮曡剼鏂瑰悜锛夌殑瀵勫瓨鍣?  锛堟垨瀵勫瓨鍣ㄤ腑鐨勫瓧娈碉級搴旇閫氳繃 GPIO 瀛愮郴缁熸毚闇诧紝濡傛灉瀹冧滑杩樻敮鎸佷腑鏂兘鍔涳紝鍒欓€氳繃 irqchip 鎶借薄
  鏆撮湶銆?
鏍规嵁纭垏鐨勭‖浠跺瘎瀛樺櫒璁捐锛孏PIO 瀛愮郴缁熸毚闇茬殑涓€浜涘姛鑳藉彲鑳戒細璋冪敤 pinctrl 瀛愮郴缁燂紝浠ュ崗璋冭法纭欢
妯″潡鐨勫瘎瀛樺櫒璁剧疆銆傜壒鍒槸锛屽浜庡叿鏈夌嫭绔?GPIO 鍜屽紩鑴氭帶鍒跺櫒纭欢妯″潡銆佸叾涓緥濡?GPIO 鏂瑰悜鐢卞紩鑴?鎺у埗鍣ㄧ‖浠舵ā鍧椾腑鐨勫瘎瀛樺櫒鑰屼笉鏄?GPIO 纭欢妯″潡鍐冲畾鐨勭‖浠讹紝杩欏彲鑳藉氨鏄繀闇€鐨勩€?
寮曡剼鐨勭數姘旂壒鎬э紝渚嬪鍋忕疆鍜岄┍鍔ㄥ己搴︼紝鍦ㄦ墍鏈夋儏鍐典笅鍙兘琚斁鍦ㄦ煇涓紩鑴氱壒瀹氱殑瀵勫瓨鍣ㄤ腑锛屾垨鑰呭湪锛圔锛?鎯呭喌涓挨鍏舵槸浣滀负 GPIO 瀵勫瓨鍣ㄧ殑涓€閮ㄥ垎銆傝繖骞朵笉鎰忓懗鐫€姝ょ被鐗规€у繀鐒跺睘浜?Linux 鍐呮牳鎵€绉扮殑 鈥淕PIO鈥濄€?
渚嬪瓙锛氫竴涓紩鑴氶€氬父琚?mux 杩涙潵鐢ㄤ綔 UART TX 绾裤€備絾鍦ㄧ郴缁熶紤鐪犳湡闂达紝鎴戜滑闇€瑕佸皢杩欎釜寮曡剼缃簬
鈥淕PIO 妯″紡鈥濆苟灏嗗叾鎺ュ湴銆?
濡傛灉浣犱负杩欎釜寮曡剼鍋氫竴瀵逛竴鏄犲皠鍒?GPIO 瀛愮郴缁燂紝浣犲彲鑳戒細寮€濮嬭涓轰綘闇€瑕佹兂鍑烘煇绉嶇湡姝ｅ鏉傜殑涓滆タ锛屽嵆
璇ュ紩鑴氳鍚屾椂鐢ㄤ簬 UART TX 鍜?GPIO锛屼綘灏嗘姄鍙栦竴涓紩鑴氭帶鍒跺彞鏌勫苟灏嗗叾璁剧疆涓烘煇涓姸鎬佷互鍚敤 UART TX
琚?mux 杩涙潵锛岀劧鍚庡皢鍏跺垏鎹㈠埌 GPIO 妯″紡骞朵娇鐢?gpiod_direction_output() 鍦ㄤ紤鐪犳湡闂村皢鍏堕┍鍔ㄤ负浣庯紝
鐒跺悗鍦ㄥ敜閱掓椂鍐嶅皢鍏?mux 鍥?UART TX锛岀敋鑷冲彲鑳藉湪杩欎釜寰幆涓敤鍒?gpiod_get() / gpiod_put()銆傝繖涓€鍒?鍙樺緱闈炲父澶嶆潅銆?
瑙ｅ喅鏂规鏄笉瑕佽涓烘暟鎹墜鍐屾墍绉扮殑 鈥淕PIO 妯″紡鈥?蹇呴』鐢?`<linux/gpio/consumer.h>` 鎺ュ彛鏉ュ鐞嗐€傜浉鍙?灏嗗叾瑙嗕负鏌愪釜鐗瑰畾鐨勫紩鑴氶厤缃缃€備緥濡傜湅鐪?`<linux/pinctrl/pinconf-generic.h>`锛屼綘浼氬湪鏂囨。涓壘鍒?杩欎釜锛?
  PIN_CONFIG_LEVEL:
     杩欏皢鎶婂紩鑴氶厤缃负杈撳嚭锛屼娇鐢ㄥ弬鏁?1 琛ㄧず楂樼數骞筹紝鍙傛暟 0 琛ㄧず浣庣數骞炽€?
鍥犳瀹屽叏鍙互灏嗕竴涓紩鑴氭帹鍏?鈥淕PIO 妯″紡鈥?骞朵綔涓洪€氬父寮曡剼鎺у埗鏄犲皠鐨勪竴閮ㄥ垎灏嗙嚎璺┍鍔ㄤ负浣庛€傛墍浠ヤ緥濡?浣犵殑 UART 椹卞姩鍙兘鐪嬭捣鏉ュ儚杩欐牱锛?

	#include <linux/pinctrl/consumer.h>

	struct pinctrl          *pinctrl;
	struct pinctrl_state    *pins_default;
	struct pinctrl_state    *pins_sleep;

	pins_default = pinctrl_lookup_state(uap->pinctrl, PINCTRL_STATE_DEFAULT);
	pins_sleep = pinctrl_lookup_state(uap->pinctrl, PINCTRL_STATE_SLEEP);

	/** Normal mode **/
	retval = pinctrl_select_state(pinctrl, pins_default);

	/** Sleep mode **/
	retval = pinctrl_select_state(pinctrl, pins_sleep);

鑰屼綘鐨勬満鍣ㄩ厤缃彲鑳界湅璧锋潵鍍忚繖鏍凤細


	static unsigned long uart_default_mode[] = {
		PIN_CONF_PACKED(PIN_CONFIG_DRIVE_PUSH_PULL, 0),
	};

	static unsigned long uart_sleep_mode[] = {
		PIN_CONF_PACKED(PIN_CONFIG_LEVEL, 0),
	};

	static struct pinctrl_map pinmap[] __initdata = {
		PIN_MAP_MUX_GROUP("uart", PINCTRL_STATE_DEFAULT, "pinctrl-foo",
				  "u0_group", "u0"),
		PIN_MAP_CONFIGS_PIN("uart", PINCTRL_STATE_DEFAULT, "pinctrl-foo",
				    "UART_TX_PIN", uart_default_mode),
		PIN_MAP_MUX_GROUP("uart", PINCTRL_STATE_SLEEP, "pinctrl-foo",
				  "u0_group", "gpio-mode"),
		PIN_MAP_CONFIGS_PIN("uart", PINCTRL_STATE_SLEEP, "pinctrl-foo",
				    "UART_TX_PIN", uart_sleep_mode),
	};

	foo_init(void)
	{
		pinctrl_register_mappings(pinmap, ARRAY_SIZE(pinmap));
	}

杩欓噷鎴戜滑瑕佹帶鍒剁殑寮曡剼鍦?鈥渦0_group鈥?涓紝骞朵笖鏈変竴涓悕涓?鈥渦0鈥?鐨勫姛鑳藉彲浠ュ湪杩欎釜寮曡剼缁勪笂鍚敤锛岀劧鍚?涓€鍒囧氨鍍忓钩甯哥殑 UART 涓氬姟銆備絾杩樻湁涓€涓悕涓?鈥済pio-mode鈥?鐨勫姛鑳藉彲浠ヨ鏄犲皠鍒扮浉鍚岀殑寮曡剼锛屽皢瀹冧滑绉诲叆
GPIO 妯″紡銆?
杩欏皢浜х敓鏈熸湜鐨勬晥鏋滐紝鑰屾棤闇€浠讳綍涓?GPIO 瀛愮郴缁熺殑铏氬亣浜や簰銆傚畠鍙槸璇ヨ澶囧湪杩涘叆浼戠湢鏃朵娇鐢ㄧ殑鐢垫皵
閰嶇疆锛屽畠鍙兘鎰忓懗鐫€璇ュ紩鑴氳璁剧疆涓烘暟鎹墜鍐屾墍绉扮殑 鈥淕PIO 妯″紡鈥濓紝浣嗚繖涓嶆槸閲嶇偣锛氬畠浠嶇劧琚偅涓?UART
璁惧鐢ㄦ潵鎺у埗灞炰簬璇?UART 椹卞姩鍣ㄧ殑寮曡剼锛屽皢瀹冧滑缃簬 UART 鎵€闇€鐨勭姸鎬併€侺inux 鍐呮牳鎰忎箟涓婄殑 GPIO 鍙槸
鏌愮 1 浣嶇殑绾匡紝鏄竴涓笉鍚岀殑浣跨敤鍦烘櫙銆?
瀵勫瓨鍣ㄥ浣曡鎷ㄥ姩锛坧oke锛変互杈惧埌鎺ㄦ垨鎷夈€佽緭鍑轰綆閰嶇疆浠ュ強灏?鈥渦0鈥?鎴?鈥済pio-mode鈥?缁?mux 鍒拌繖浜涘紩鑴?涓婏紝鏄┍鍔ㄨ瑙ｅ喅鐨勯棶棰樸€?
涓€浜涙暟鎹墜鍐屼細鏇存湁甯姪锛屽皢 鈥淕PIO 妯″紡鈥?绉颁负 鈥滀綆鍔熻€楁ā寮忊€濓紝鑰屼笉鏄换浣曚笌 GPIO 鏈夊叧鐨勪笢瑗裤€傚悗鑰呭湪
鐢垫皵涓婇€氬父鎰忓懗鐫€鐩稿悓鐨勪笢瑗匡紝浣嗗湪鍚庝竴绉嶆儏鍐典笅锛岃蒋浠跺伐绋嬪笀閫氬父浼氳繀閫熻瘑鍒嚭杩欐槸鏌愪釜鐗瑰畾鐨?muxing
鎴栭厤缃紝鑰屼笉鏄笌 GPIO API 鐩稿叧鐨勪换浣曚笢瑗裤€?

## 鏉?鏈哄櫒閰嶇疆锛圔oard/machine configuration锛?

鏉垮拰鏈哄櫒瀹氫箟浜嗘煇涓畬鏁寸殑杩愯绯荤粺鏄浣曠粍鍚堝湪涓€璧风殑锛屽寘鎷?GPIO 鍜岃澶囨槸濡備綍 mux 鐨勩€乺egulator
鏄浣曞彈绾︽潫鐨勶紝浠ュ強鏃堕挓鏍戞槸浠€涔堟牱瀛愩€傚綋鐒?pinmux 璁剧疆涔熸槸鍏朵腑鐨勪竴閮ㄥ垎銆?
涓€涓満鍣ㄧ殑寮曡剼鎺у埗鍣ㄩ厤缃湅璧锋潵闈炲父鍍忎竴涓畝鍗曠殑 regulator 閰嶇疆锛屽洜姝ゅ浜庝笂闈㈢殑绀轰緥鏁扮粍锛屾垜浠?鎯宠鍦ㄧ浜屼釜鍔熻兘鏄犲皠涓婂惎鐢?i2c 鍜?spi锛?

	#include <linux/pinctrl/machine.h>

	static const struct pinctrl_map mapping[] __initconst = {
		{
			.dev_name = "foo-spi.0",
			.name = PINCTRL_STATE_DEFAULT,
			.type = PIN_MAP_TYPE_MUX_GROUP,
			.ctrl_dev_name = "pinctrl-foo",
			.data.mux.function = "spi0",
		},
		{
			.dev_name = "foo-i2c.0",
			.name = PINCTRL_STATE_DEFAULT,
			.type = PIN_MAP_TYPE_MUX_GROUP,
			.ctrl_dev_name = "pinctrl-foo",
			.data.mux.function = "i2c0",
		},
		{
			.dev_name = "foo-mmc.0",
			.name = PINCTRL_STATE_DEFAULT,
			.type = PIN_MAP_TYPE_MUX_GROUP,
			.ctrl_dev_name = "pinctrl-foo",
			.data.mux.function = "mmc0",
		},
	};

杩欓噷鐨?dev_name 鍖归厤鍙敤浜庢煡鎵捐澶?struct 鐨勫敮涓€璁惧鍚嶏紙灏卞儚 clockdev 鎴?regulator 閭ｆ牱锛夈€?鍑芥暟鍚嶅繀椤诲尮閰嶅鐞嗘寮曡剼鑼冨洿鐨?pinmux 椹卞姩鎻愪緵鐨勫嚱鏁般€?
濡備綘鎵€瑙侊紝鎴戜滑绯荤粺涓婂彲鑳芥湁澶氫釜寮曡剼鎺у埗鍣紝鍥犳鎴戜滑闇€瑕佹寚瀹氬叾涓寘鍚垜浠鏄犲皠鐨勫姛鑳界殑閭ｄ竴涓€?
浣犲彧闇€閫氳繃浠ヤ笅鏂瑰紡灏嗚繖涓?pinmux 鏄犲皠娉ㄥ唽鍒?pinmux 瀛愮郴缁燂細


       ret = pinctrl_register_mappings(mapping, ARRAY_SIZE(mapping));

鐢变簬涓婅堪鏋勯€犵浉褰撳父瑙侊紝鏈変竴涓緟鍔╁畯鍙互璁╁畠鏇寸揣鍑戯紝璇ュ畯鍋囪浣犳兂瑕佷娇鐢?pinctrl-foo 鍜屼綅缃?0
杩涜鏄犲皠锛屼緥濡傦細


	static struct pinctrl_map mapping[] __initdata = {
		PIN_MAP_MUX_GROUP("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				  "pinctrl-foo", NULL, "i2c0"),
	};

鏄犲皠琛ㄤ篃鍙兘鍖呭惈寮曡剼閰嶇疆鏉＄洰銆傛瘡涓紩鑴?缁勯€氬父鏈変竴缁勫奖鍝嶅叾鐨勯厤缃潯鐩紝鍥犳鐢ㄤ簬閰嶇疆鐨勮〃鏉＄洰
寮曠敤涓€涓厤缃弬鏁板拰鍊肩殑鏁扮粍銆備竴涓娇鐢ㄤ究鎹峰畯鐨勪緥瀛愬涓嬫墍绀猴細


	static unsigned long i2c_grp_configs[] = {
		FOO_PIN_DRIVEN,
		FOO_PIN_PULLUP,
	};

	static unsigned long i2c_pin_configs[] = {
		FOO_OPEN_COLLECTOR,
		FOO_SLEW_RATE_SLOW,
	};

	static struct pinctrl_map mapping[] __initdata = {
		PIN_MAP_MUX_GROUP("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				  "pinctrl-foo", "i2c0", "i2c0"),
		PIN_MAP_CONFIGS_GROUP("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				      "pinctrl-foo", "i2c0", i2c_grp_configs),
		PIN_MAP_CONFIGS_PIN("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				    "pinctrl-foo", "i2c0scl", i2c_pin_configs),
		PIN_MAP_CONFIGS_PIN("foo-i2c.0", PINCTRL_STATE_DEFAULT,
				    "pinctrl-foo", "i2c0sda", i2c_pin_configs),
	};

鏈€鍚庯紝涓€浜涜澶囨湡鏈涙槧灏勮〃鍖呭惈鏌愪簺鐗瑰畾鐨勫懡鍚嶇姸鎬併€傚綋杩愯鍦ㄤ笉闇€瑕佷换浣曞紩鑴氭帶鍒跺櫒閰嶇疆鐨勭‖浠朵笂鏃讹紝
鏄犲皠琛ㄤ粛鐒跺繀椤诲寘鍚偅浜涘懡鍚嶇姸鎬侊紝浠ユ槑纭〃鏄庤繖浜涚姸鎬佽鎻愪緵骞舵剰鍥句负绌恒€傝〃鏉＄洰瀹?`PIN_MAP_DUMMY_STATE()` 鐢ㄤ簬瀹氫箟涓€涓懡鍚嶇姸鎬佽€屼笉瀵艰嚧浠讳綍寮曡剼鎺у埗鍣ㄨ缂栫▼锛?

	static struct pinctrl_map mapping[] __initdata = {
		PIN_MAP_DUMMY_STATE("foo-i2c.0", PINCTRL_STATE_DEFAULT),
	};


## 澶嶆潅鏄犲皠锛圕omplex mappings锛?

鐢变簬鍙互灏嗕竴涓姛鑳芥槧灏勫埌涓嶅悓鐨勫紩鑴氱粍锛屽彲浠ュ儚杩欐牱鎸囧畾涓€涓彲閫夌殑 .group锛?

	...
	{
		.dev_name = "foo-spi.0",
		.name = "spi0-pos-A",
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "spi0",
		.group = "spi0_0_grp",
	},
	{
		.dev_name = "foo-spi.0",
		.name = "spi0-pos-B",
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "spi0",
		.group = "spi0_1_grp",
	},
	...

杩欎釜绀轰緥鏄犲皠鐢ㄤ簬鍦ㄨ繍琛屾椂鍦?spi0 鐨勪袱涓綅缃箣闂村垏鎹紝濡?`Runtime pinmuxing`_ 鏍囬涓嬭繘涓€姝?鎻忚堪銆?
姝ゅ锛屼竴涓懡鍚嶇姸鎬佸彲鑳藉奖鍝嶅涓紩鑴氱粍鐨?muxing锛屼緥濡備笂闈?mmc0 鐨勪緥瀛愪腑锛屼綘鍙互灏?mmc0 鎬荤嚎浠?2 浣嶅姞娉曞紡鎵╁睍鍒?4 浣嶅啀鍒?8 浣嶃€傚鏋滄垜浠兂瀵规€诲叡 2 + 2 + 4 = 8 涓紩鑴氾紙濡?8 浣?MMC 鎬荤嚎鐨勬儏鍐碉級
浣跨敤鍏ㄩ儴涓変釜缁勶紝鎴戜滑瀹氫箟涓€涓涓嬬殑鏄犲皠锛?

	...
	{
		.dev_name = "foo-mmc.0",
		.name = "2bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_1_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "4bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_1_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "4bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_2_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "8bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_1_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "8bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_2_grp",
	},
	{
		.dev_name = "foo-mmc.0",
		.name = "8bit"
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "mmc0",
		.group = "mmc0_3_grp",
	},
	...

鐢ㄥ儚杩欐牱锛堣涓嬩竴娈碉級浠庤澶囦腑鎶撳彇姝ゆ槧灏勭殑缁撴灉涓猴細


	p = devm_pinctrl_get(dev);
	s = pinctrl_lookup_state(p, "8bit");
	ret = pinctrl_select_state(p, s);

鎴栬€呮洿绠€鍗曪細


	p = devm_pinctrl_get_select(dev, "8bit");

缁撴灉灏嗘槸浣犱竴娆℃縺娲绘槧灏勪腑鍏ㄩ儴涓変釜搴曢儴璁板綍銆傜敱浜庡畠浠叡浜浉鍚岀殑鍚嶅瓧銆佸紩鑴氭帶鍒跺櫒璁惧銆佸姛鑳藉拰璁惧锛?骞朵笖鐢变簬鎴戜滑鍏佽灏嗗涓粍鍖归厤鍒板崟涓澶囷紝瀹冧滑鍏ㄩ儴琚€変腑锛屽苟涓斿叏閮ㄧ敱 pinmux 鏍稿績鍚屾椂鍚敤鍜?绂佺敤銆?

## 鏉ヨ嚜椹卞姩鐨勫紩鑴氭帶鍒惰姹傦紙Pin control requests from drivers锛?

褰撹澶囬┍鍔ㄥ嵆灏嗘帰娴嬶紙probe锛夋椂锛屽鏋滆澶囨爲涓畾涔変簡鏍囧噯鐘舵€侊紝璁惧鏍稿績浼氶€氳繃璋冪敤
`pinctrl_bind_pins()` 灏嗚繖浜涚姸鎬侀檮鍔犲埌杩欎簺璁惧涓娿€傚彲鑳界殑鏍囧噯鐘舵€佸悕鏈夛細鈥渄efault鈥濄€佲€渋nit鈥濄€?鈥渟leep鈥?鍜?鈥渋dle鈥濄€?
- 濡傛灉 `default` 鍦ㄨ澶囨爲涓畾涔夛紝瀹冧細鍦ㄨ澶囨帰娴嬩箣鍓嶈閫変腑銆?
- 濡傛灉 `init` 鍜?`default` 閮藉湪璁惧鏍戜腑瀹氫箟锛屽垯 鈥渋nit鈥?鐘舵€佸湪椹卞姩鎺㈡祴涔嬪墠琚€変腑锛屸€渄efault鈥?  鐘舵€佸湪椹卞姩鎺㈡祴涔嬪悗琚€変腑銆?
- `sleep` 鍜?`idle` 鐘舵€佺敤浜庣數婧愮鐞嗭紝鍙兘浣跨敤涓嬮潰鐨?PM API 閫変腑銆?
## PM 鎺ュ彛


PM 杩愯鏃舵寕璧?鎭㈠锛坮untime suspend/resume锛夊彲鑳介渶瑕佹墽琛屼笌鎺㈡祴鏈熼棿鐩稿悓鐨勫垵濮嬪寲搴忓垪銆傜敱浜庨瀹氫箟
鐘舵€佸凡缁忛檮鍔犲埌璁惧锛岄┍鍔ㄥ彲浠ヤ娇鐢ㄤ互涓嬭緟鍔╁嚱鏁版樉寮忔縺娲昏繖浜涚姸鎬侊細

- `pinctrl_pm_select_default_state()`
- `pinctrl_pm_select_init_state()`
- `pinctrl_pm_select_sleep_state()`
- `pinctrl_pm_select_idle_state()`

渚嬪锛屽鏋滄仮澶嶈澶囦緷璧栦簬鏌愪簺 pinmux 鐘舵€?

	foo_suspend()
	{
		/** suspend device **/
		...

		pinctrl_pm_select_sleep_state(dev);
	}

	foo_resume()
	{
		pinctrl_pm_select_init_state(dev);

		/** resuming device **/
		...

		pinctrl_pm_select_default_state(dev);
	}

杩欐牱椹卞姩缂栧啓鑰呮棤闇€娣诲姞涓嬮潰杩欑被鏍锋澘浠ｇ爜銆傜劧鑰岋紝褰撹繘琛岀粏绮掑害鐨勭姸鎬侀€夋嫨鑰屼笉浣跨敤 鈥渄efault鈥?鐘舵€佹椂锛?浣犲彲鑳介渶瑕佸仛涓€浜涜澶囬┍鍔ㄥ pinctrl 鍙ユ焺鍜岀姸鎬佺殑澶勭悊銆?
鎵€浠ュ鏋滀綘鍙槸鎯冲皢鏌愪釜璁惧鐨勫紩鑴氱疆浜庨粯璁ょ姸鎬佸苟灏辨浜嗕簨锛岄櫎浜嗘彁渚涙纭殑鏄犲皠琛ㄤ箣澶栵紝浣犳棤闇€鍋氬叾浠?浠讳綍浜嬫儏銆傝澶囨牳蹇冧細澶勭悊鍏朵綑閮ㄥ垎銆?
閫氬父涓嶅缓璁鍚勪釜椹卞姩鍘昏幏鍙栧拰鍚敤寮曡剼鎺у埗銆傚洜姝ゅ鏋滃彲鑳斤紝鍦ㄥ钩鍙颁唬鐮佹垨浣犺兘璁块棶鎵€鏈夊彈褰卞搷 struct
device * 鎸囬拡鐨勫叾浠栧湴鏂瑰鐞嗗紩鑴氭帶鍒躲€傚湪鏌愪簺鎯呭喌涓嬶紝褰撻┍鍔ㄩ渶瑕佸湪杩愯鏃跺垏鎹笉鍚岀殑 mux 鏄犲皠鏃讹紝杩?鏄笉鍙兘鐨勩€?
涓€涓吀鍨嬫儏鍐垫槸椹卞姩闇€瑕佸湪姝ｅ父鎿嶄綔鍜岃繘鍏ヤ紤鐪犱箣闂村垏鎹㈠紩鑴氱殑鍋忕疆锛屼粠 `PINCTRL_STATE_DEFAULT` 绉诲姩鍒?`PINCTRL_STATE_SLEEP`锛屽湪杩愯鏃堕噸鏂板亸缃敋鑷抽噸鏂?mux 寮曡剼浠ュ湪浼戠湢妯″紡涓嬭妭鐪佺數娴併€?
鍙︿竴绉嶆儏鍐垫槸 pinctrl 闇€瑕佸湪鎺㈡祴鏈熼棿鍒囨崲鍒版煇涓ā寮忥紝鐒跺悗鍦ㄦ帰娴嬬粨鏉熸椂鎭㈠鍒伴粯璁ょ姸鎬併€備緥濡傦紝涓€涓?PINMUX 鍙兘闇€瑕佸湪鎺㈡祴鏈熼棿琚厤缃负 GPIO銆傚湪杩欑鎯呭喌涓嬶紝浣跨敤 `PINCTRL_STATE_INIT` 鍦ㄦ帰娴嬪墠鍒囨崲鐘舵€侊紝
鐒跺悗鍦ㄦ帰娴嬬粨鏉熸椂绉诲姩鍒?`PINCTRL_STATE_DEFAULT` 浠ヨ繘琛屾甯告搷浣溿€?
椹卞姩鍙互鍍忚繖鏍疯姹傛縺娲绘煇涓帶鍒剁姸鎬侊紝閫氬父鍙槸榛樿鐘舵€侊細


	#include <linux/pinctrl/consumer.h>

	struct foo_state {
	struct pinctrl *p;
	struct pinctrl_state *s;
	...
	};

	foo_probe()
	{
		/** Allocate a state holder named "foo" etc **/
		struct foo_state *foo = ...;
		int ret;

		foo->p = devm_pinctrl_get(&device);
		if (IS_ERR(foo->p)) {
			ret = PTR_ERR(foo->p);
			foo->p = NULL;
			return ret;
		}

		foo->s = pinctrl_lookup_state(foo->p, PINCTRL_STATE_DEFAULT);
		if (IS_ERR(foo->s)) {
			devm_pinctrl_put(foo->p);
			return PTR_ERR(foo->s);
		}

		ret = pinctrl_select_state(foo->p, foo->s);
		if (ret < 0) {
			devm_pinctrl_put(foo->p);
			return ret;
		}
	}

杩欎釜鑾峰彇/鏌ユ壘/閫夋嫨/閲婃斁搴忓垪鍚屾牱鍙互鐢辨€荤嚎椹卞姩澶勭悊锛屽鏋滀綘涓嶆兂璁╂瘡涓┍鍔ㄩ兘澶勭悊瀹冿紝骞朵笖浣犳竻妤氫綘鐨?鎬荤嚎涓婄殑瀹夋帓銆?
pinctrl API 鐨勮涔夋槸锛?
- `pinctrl_get()` 鍦ㄨ繘绋嬩笂涓嬫枃涓璋冪敤锛屼互鑾峰彇缁欏畾瀹㈡埛绔澶囩殑鎵€鏈?pinctrl 淇℃伅鐨勫彞鏌勩€傚畠灏嗕粠
  鍐呮牳鍐呭瓨鍒嗛厤涓€涓?struct 鏉ヤ繚瀛?pinmux 鐘舵€併€傛墍鏈夋槧灏勮〃瑙ｆ瀽鎴栫被浼肩殑鎱㈤€熸搷浣滈兘鍦ㄦ API 鍐呭彂鐢熴€?
- `devm_pinctrl_get()` 鏄?pinctrl_get() 鐨勪竴涓彉浣擄紝瀹冨湪鍏宠仈鐨勮澶囪绉婚櫎鏃惰嚜鍔ㄨ皟鐢?  `pinctrl_put()` 浜庢墍鑾峰彇鐨勬寚閽堜笂銆傚缓璁娇鐢ㄦ鍑芥暟鑰岄潪鏅€氱殑 `pinctrl_get()`銆?
- `pinctrl_lookup_state()` 鍦ㄨ繘绋嬩笂涓嬫枃涓璋冪敤锛屼互鑾峰彇瀹㈡埛绔澶囨煇涓壒瀹氱姸鎬佺殑鍙ユ焺銆傝繖涓搷浣?  涔熷彲鑳藉緢鎱€?
- `pinctrl_select_state()` 鏍规嵁鏄犲皠琛ㄧ粰鍑虹殑鐘舵€佸畾涔夊寮曡剼鎺у埗鍣ㄧ‖浠惰繘琛岀紪绋嬨€傜悊璁轰笂锛岃繖鏄竴涓?  蹇€熻矾寰勬搷浣滐紝鍥犱负瀹冨彧娑夊強灏嗕竴浜涘瘎瀛樺櫒璁剧疆鍐欏叆纭欢銆傜劧鑰岋紝璇锋敞鎰忔煇浜涘紩鑴氭帶鍒跺櫒鐨勫瘎瀛樺櫒鍙兘浣嶄簬
  鎱㈤€?鍩轰簬 IRQ 鐨勬€荤嚎涓婏紝鍥犳瀹㈡埛绔澶囦笉搴斿亣璁惧畠浠彲浠ュ湪闈為樆濉炰笂涓嬫枃涓皟鐢?  `pinctrl_select_state()`銆?
- `pinctrl_put()` 閲婃斁涓?pinctrl 鍙ユ焺鍏宠仈鐨勬墍鏈変俊鎭€?
- `devm_pinctrl_put()` 鏄?`pinctrl_put()` 鐨勪竴涓彉浣擄紝鍙敤浜庢樉寮忛攢姣佺敱 `devm_pinctrl_get()`
  杩斿洖鐨?pinctrl 瀵硅薄銆傜劧鑰岋紝鐢变簬鍗充娇涓嶈皟鐢ㄥ畠涔熶細鍙戠敓鑷姩娓呯悊锛屼娇鐢ㄦ鍑芥暟鐨勬儏鍐典細寰堝皯銆?
  `pinctrl_get()` 蹇呴』涓庢櫘閫氱殑 `pinctrl_put()` 閰嶅銆?  `pinctrl_get()` 涓嶅緱涓?`devm_pinctrl_put()` 閰嶅銆?  `devm_pinctrl_get()` 鍙互閫夋嫨涓?`devm_pinctrl_put()` 閰嶅銆?  `devm_pinctrl_get()` 涓嶅緱涓庢櫘閫氱殑 `pinctrl_put()` 閰嶅銆?
閫氬父寮曡剼鎺у埗鏍稿績澶勭悊鑾峰彇/閲婃斁瀵癸紝骞惰皟鐢ㄨ澶囬┍鍔ㄧ殑璁拌处鎿嶄綔锛屽妫€鏌ュ彲鐢ㄥ姛鑳藉拰鍏宠仈鐨勫紩鑴氾紝鑰?`pinctrl_select_state()` 浼犻€掔粰寮曡剼鎺у埗鍣ㄩ┍鍔紝鐢卞悗鑰呰礋璐ｉ€氳繃蹇€熸嫧鍔ㄤ竴浜涘瘎瀛樺櫒鏉ユ縺娲诲拰/鎴?鍋滅敤 mux 璁剧疆銆?
褰撲綘鍙戝嚭 `devm_pinctrl_get()` 璋冪敤鏃讹紝浼氫负浣犵殑璁惧鍒嗛厤寮曡剼锛屼箣鍚庝綘搴旇鑳藉湪鎵€鏈夊紩鑴氱殑 debugfs
鍒楄〃涓湅鍒拌繖涓€鐐广€?
娉ㄦ剰锛氬鏋滄壘涓嶅埌璇锋眰鐨?pinctrl 鍙ユ焺锛屼緥濡?pinctrl 椹卞姩灏氭湭娉ㄥ唽锛宲inctrl 绯荤粺灏嗚繑鍥?`-EPROBE_DEFER`銆傚洜姝よ纭繚浣犵殑椹卞姩涓殑閿欒璺緞鑳藉浼橀泤鍦版竻鐞嗭紝骞跺噯澶囧ソ鍦ㄥ惎鍔ㄨ繃绋嬬殑鍚庢湡閲嶈瘯
鎺㈡祴銆?

## 鍚屾椂闇€瑕佸紩鑴氭帶鍒跺拰 GPIO 鐨勯┍鍔紙Drivers needing both pin control and GPIOs锛?

鍐嶆璇存槑锛屼笉寤鸿璁╅┍鍔ㄨ嚜宸辨煡鎵惧拰閫夋嫨寮曡剼鎺у埗鐘舵€侊紝浣嗗悓鏍锋湁鏃惰繖鏄笉鍙伩鍏嶇殑銆?
鎵€浠ュ亣璁句綘鐨勯┍鍔ㄥ儚杩欐牱鑾峰彇瀹冪殑璧勬簮锛?

	#include <linux/pinctrl/consumer.h>
	#include <linux/gpio/consumer.h>

	struct pinctrl *pinctrl;
	struct gpio_desc *gpio;

	pinctrl = devm_pinctrl_get_select_default(&dev);
	gpio = devm_gpiod_get(&dev, "foo");

杩欓噷鎴戜滑棣栧厛璇锋眰鏌愪釜寮曡剼鐘舵€侊紝鐒跺悗璇锋眰浣跨敤 GPIO 鈥渇oo鈥濄€傚鏋滀綘鍍忚繖鏍锋浜ゅ湴浣跨敤瀛愮郴缁燂紝浣犻€氬父搴旇
濮嬬粓鍦ㄨ姹?GPIO **涔嬪墠**鑾峰彇浣犵殑 pinctrl 鍙ユ焺骞堕€夋嫨鎵€闇€鐨?pinctrl 鐘舵€併€傝繖鏄竴涓涔夌害瀹氾紝鐢ㄤ簬
閬垮厤鍙兘鍦ㄧ數姘斾笂浠や汉涓嶅揩鐨勬儏鍐碉紝浣犺偗瀹氫細鎯冲湪 GPIO 瀛愮郴缁熷紑濮嬪鐞嗗畠浠箣鍓嶏紝浠ユ煇绉嶆柟寮?mux 杩涙潵骞?鍋忕疆寮曡剼銆?
浠ヤ笂鍙互闅愯棌锛氫娇鐢ㄨ澶囨牳蹇冿紝pinctrl 鏍稿績鍙兘浼氬湪璁惧鎺㈡祴涔嬪墠灏辫缃ソ寮曡剼鐨勯厤缃拰 muxing锛岀劧鑰岃繖涓?GPIO 瀛愮郴缁熸浜ゃ€?
浣嗕篃瀛樺湪杩欐牱鐨勬儏鍐碉紝GPIO 瀛愮郴缁熺洿鎺ヤ笌 pinctrl 瀛愮郴缁熼€氫俊鏄湁鎰忎箟鐨勶紝灏嗗悗鑰呯敤浣滃悗绔€傝繖灏辨槸 GPIO
椹卞姩鍙兘璋冪敤涓婃枃 `Pin control interaction with the GPIO subsystem`_ 涓€鑺備腑鎻忚堪鐨勫嚱鏁扮殑鏃跺€欍€傝繖鍙?娑夊強姣忓紩鑴氱殑澶氳矾澶嶇敤锛屽苟灏嗗畬鍏ㄩ殣钘忓湪 gpiod_*() 鍑芥暟鍛藉悕绌洪棿涔嬪悗銆傚湪杩欑鎯呭喌涓嬶紝椹卞姩鏍规湰涓嶉渶瑕佷笌
寮曡剼鎺у埗瀛愮郴缁熶氦浜掋€?
濡傛灉涓€涓紩鑴氭帶鍒堕┍鍔ㄥ拰涓€涓?GPIO 椹卞姩澶勭悊鐩稿悓鐨勫紩鑴氾紝骞朵笖浣跨敤鍦烘櫙娑夊強澶氳矾澶嶇敤锛屼綘蹇呴』灏嗗紩鑴氭帶鍒跺櫒
瀹炵幇涓?GPIO 椹卞姩鐨勫悗绔紝濡備笅鎵€绀猴紝闄ら潪浣犵殑纭欢璁捐浣垮緱 GPIO 鎺у埗鍣ㄥ彲浠ラ€氳繃纭欢瑕嗙洊寮曡剼鎺у埗鍣ㄧ殑
澶氳矾澶嶇敤鐘舵€侊紝鑰屾棤闇€涓庡紩鑴氭帶鍒剁郴缁熶氦浜掋€?
濡傛灉寮曡剼鎺у埗椹卞姩鍜?GPIO 椹卞姩澶勭悊鐩稿悓鐨勫紩鑴氾紝骞朵笖浣跨敤鍦烘櫙娑夊強澶氳矾澶嶇敤锛屼綘**蹇呴』**灏嗗紩鑴氭帶鍒跺櫒瀹炵幇
涓?GPIO 椹卞姩鐨勫悗绔紝闄ら潪浣犵殑纭欢璁捐浣垮緱 GPIO 鎺у埗鍣ㄥ彲浠ラ€氳繃纭欢瑕嗙洊寮曡剼鎺у埗鍣ㄧ殑澶氳矾澶嶇敤鐘舵€侊紝鑰?鏃犻渶涓庡紩鑴氭帶鍒剁郴缁熶氦浜掋€?

## 绯荤粺寮曡剼鎺у埗鍗犵敤锛圫ystem pin control hogging锛?

褰撳紩鑴氭帶鍒跺櫒琚敞鍐屾椂锛屽紩鑴氭帶鍒舵槧灏勬潯鐩彲浠ヨ鏍稿績鍗犵敤锛坔ogged锛夈€傝繖鎰忓懗鐫€鏍稿績灏嗗湪寮曡剼鎺у埗璁惧
娉ㄥ唽涔嬪悗绔嬪嵆灏濊瘯瀵瑰叾璋冪敤 `pinctrl_get()`銆乣pinctrl_lookup_state()` 鍜?`pinctrl_select_state()`銆?
杩欏彂鐢熷湪鏄犲皠琛ㄦ潯鐩腑瀹㈡埛绔澶囧悕绛変簬寮曡剼鎺у埗鍣ㄨ澶囧悕锛屼笖鐘舵€佸悕涓?`PINCTRL_STATE_DEFAULT` 鐨勬儏鍐碉細


	{
		.dev_name = "pinctrl-foo",
		.name = PINCTRL_STATE_DEFAULT,
		.type = PIN_MAP_TYPE_MUX_GROUP,
		.ctrl_dev_name = "pinctrl-foo",
		.function = "power_func",
	},

鐢变簬璇锋眰鏍稿績鍦ㄤ富寮曡剼鎺у埗鍣ㄤ笂鍗犵敤涓€浜涘缁堥€傜敤鐨?mux 璁剧疆鍙兘鏄父瑙佺殑锛屾湁涓€涓敤浜庢鐨勪究鎹峰畯锛?

	PIN_MAP_MUX_GROUP_HOG_DEFAULT("pinctrl-foo", NULL /** group **/,
				      "power_func")

杩欎細寰楀埌涓庝笂闈㈡瀯閫犲畬鍏ㄧ浉鍚岀殑缁撴灉銆?

## 杩愯鏃跺紩鑴氬璺鐢紙Runtime pinmuxing锛?

鍙互鍦ㄨ繍琛屾椂灏嗘煇涓姛鑳?mux 杩涙潵鍜屽嚭鍘伙紝渚嬪灏嗕竴涓?SPI 绔彛浠庝竴缁勫紩鑴氱Щ鍔ㄥ埌鍙︿竴缁勫紩鑴氥€備緥濡傚浜?涓婇潰鐨?spi0锛屾垜浠负鍚屼竴涓姛鑳芥毚闇蹭袱涓笉鍚岀殑寮曡剼缁勶紝浣嗗湪鏄犲皠涓娇鐢ㄤ笉鍚岀殑鍛藉悕锛屽涓婃枃 鈥淎dvanced
mapping鈥?鎵€杩般€傚洜姝ゅ浜庝竴涓?SPI 璁惧锛屾垜浠湁涓や釜鍚嶄负 鈥減os-A鈥?鍜?鈥減os-B鈥?鐨勭姸鎬併€?
杩欎釜鐗囨棣栧厛涓轰袱涓粍锛堝湪 foo_probe() 涓級鍒濆鍖栦竴涓姸鎬佸璞★紝鐒跺悗鍦ㄧ粍 A 瀹氫箟鐨勫紩鑴氫笂 mux 杩涜
鍔熻兘锛屾渶鍚庡湪缁?B 瀹氫箟鐨勫紩鑴氫笂 mux 杩涘畠锛?

	#include <linux/pinctrl/consumer.h>

	struct pinctrl *p;
	struct pinctrl_state **s1, **s2;

	foo_probe()
	{
		/** Setup **/
		p = devm_pinctrl_get(&device);
		if (IS_ERR(p))
			...

		s1 = pinctrl_lookup_state(p, "pos-A");
		if (IS_ERR(s1))
			...

		s2 = pinctrl_lookup_state(p, "pos-B");
		if (IS_ERR(s2))
			...
	}

	foo_switch()
	{
		/** Enable on position A **/
		ret = pinctrl_select_state(p, s1);
		if (ret < 0)
			...

		...

		/** Enable on position B **/
		ret = pinctrl_select_state(p, s2);
		if (ret < 0)
			...

		...
	}

涓婅堪蹇呴』鍦ㄨ繘绋嬩笂涓嬫枃涓畬鎴愩€傚紩鑴氱殑淇濈暀灏嗗湪鐘舵€佽婵€娲绘椂杩涜锛屽洜姝ゅ疄闄呬笂锛屽湪鏌愪釜杩愯涓殑绯荤粺涓婏紝
涓€涓壒瀹氬紩鑴氬彲浠ュ湪涓嶅悓鏃堕棿琚笉鍚屽姛鑳戒娇鐢ㄣ€?

## Debugfs 鏂囦欢


杩欎簺鏂囦欢鍦?`/sys/kernel/debug/pinctrl` 涓垱寤猴細

- `pinctrl-devices`锛氭墦鍗版瘡涓紩鑴氭帶鍒跺櫒璁惧浠ュ強鎸囩ず鏄惁鏀寔 pinmux 鍜?pinconf 鐨勫垪

- `pinctrl-handles`锛氭墦鍗版瘡涓凡閰嶇疆鐨勫紩鑴氭帶鍒跺櫒鍙ユ焺鍙婄浉搴旂殑 pinmux 鏄犲皠

- `pinctrl-maps`锛氭墦鍗版墍鏈?pinctrl 鏄犲皠

鍦?`/sys/kernel/debug/pinctrl` 鍐呴儴涓烘瘡涓紩鑴氭帶鍒跺櫒璁惧鍒涘缓涓€涓瓙鐩綍锛屽寘鍚繖浜涙枃浠讹細

- `pins`锛氫负寮曡剼鎺у埗鍣ㄤ笂娉ㄥ唽鐨勬瘡涓紩鑴氭墦鍗颁竴琛屻€俻inctrl 椹卞姩鍙互娣诲姞棰濆淇℃伅锛屼緥濡傚瘎瀛樺櫒鍐呭銆?
- `gpio-ranges`锛氭墦鍗板皢 gpio 绾挎槧灏勫埌鎺у埗鍣ㄤ笂寮曡剼鐨勮寖鍥?
- `pingroups`锛氭墦鍗板紩鑴氭帶鍒跺櫒涓婃敞鍐岀殑鎵€鏈夊紩鑴氱粍

- `pinconf-pins`锛氫负姣忎釜寮曡剼鎵撳嵃寮曡剼閰嶇疆璁剧疆

- `pinconf-groups`锛氭寜寮曡剼缁勬墦鍗板紩鑴氶厤缃缃?
- `pinmux-functions`锛氭墦鍗版瘡涓紩鑴氬姛鑳戒互鍙婃槧灏勫埌璇ュ紩鑴氬姛鑳界殑寮曡剼缁?
- `pinmux-pins`锛氶亶鍘嗘墍鏈夊紩鑴氬苟鎵撳嵃 mux 鎷ユ湁鑰呫€乬pio 鎷ユ湁鑰呬互鍙婅寮曡剼鏄惁鏄?hog

- `pinmux-select`锛氬啓鍏ユ鏂囦欢浠ユ縺娲绘煇涓粍鐨勫紩鑴氬姛鑳斤細

  .. code-block:: sh

        echo "<group-name function-name>" > pinmux-select
