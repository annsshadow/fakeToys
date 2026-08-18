## Linux 涓嬬殑闂厜 LED 澶勭悊


鏌愪簺 LED 璁惧鎻愪緵涓ょ妯″紡鈥斺€旀墜鐢电瓛锛坱orch锛変笌闂厜锛坒lash锛夈€傚湪 LED 瀛愮郴缁熶腑锛岃繖涓ょ妯″紡鍒嗗埆鐢?LED 绫伙紙瑙?Documentation/leds/leds-class.rst锛変笌 LED 闂厜绫绘敮鎸併€傛墜鐢电瓛妯″紡鐩稿叧鐗规€ч粯璁ゅ惎鐢紝鑰岄棯鍏夋ā寮忕壒鎬т粎鍦ㄩ┍鍔ㄩ€氳繃璁剧疆 `LED_DEV_CAP_FLASH` 鏍囧織澹版槑鏀寔鏃舵墠鍚敤銆?

涓轰簡鍚敤瀵归棯鍏?LED 鐨勬敮鎸侊紝蹇呴』鍦ㄥ唴鏍搁厤缃腑瀹氫箟 `CONFIG_LEDS_CLASS_FLASH` 绗﹀彿銆侺ED 闂厜绫婚┍鍔ㄥ繀椤讳娇鐢?led_classdev_flash_register 鍑芥暟鍦?LED 瀛愮郴缁熶腑娉ㄥ唽銆?

涓烘帶鍒堕棯鍏?LED 璁惧锛屾毚闇蹭簡浠ヤ笅 sysfs 灞炴€э細
锛堣 Documentation/ABI/testing/sysfs-class-led-flash锛?

 - flash_brightness
 - max_flash_brightness
 - flash_timeout
 - max_flash_timeout
 - flash_strobe
 - flash_fault


## 闂厜 LED 鐨?V4L2 灏佽


LED 瀛愮郴缁熼┍鍔ㄤ篃鍙互浠?VideoForLinux2 瀛愮郴缁熺殑灞傞潰杩涜鎺у埗銆備负浜嗗惎鐢ㄦ鍔熻兘锛屽繀椤诲湪鍐呮牳閰嶇疆涓畾涔?`CONFIG_V4L2_FLASH_LED_CLASS` 绗﹀彿銆?

椹卞姩蹇呴』璋冪敤 v4l2_flash_init 鍑芥暟浠ュ湪 V4L2 瀛愮郴缁熶腑娉ㄥ唽銆傝鍑芥暟鎺ュ彈鍏釜鍙傛暟锛?

- dev:
	闂厜璁惧锛屼緥濡備竴涓?I2C 璁惧
- of_node:
	LED 鐨?of_node锛岃嫢涓庤澶囩殑鐩稿悓鍒欏彲涓?NULL
- fled_cdev:
	瑕佸皝瑁呯殑 LED 闂厜绫昏澶?
- iled_cdev:
	浠ｈ〃涓?fled_cdev 鍏宠仈鐨勬寚绀?LED 鐨?LED 闂厜绫昏澶囷紝鍙负 NULL
- ops:
	V4L2 鐗瑰畾鐨勬搷浣?

 - external_strobe_set
		瀹氫箟闂厜 LED strobe 鐨勬潵婧愨€斺€?
		V4L2_CID_FLASH_STROBE 鎺у埗鎴栧閮ㄦ潵婧愶紝閫氬父鏄?
		涓€涓紶鎰熷櫒锛岃繖鏍峰彲浠ヤ娇闂厜 strobe 鐨勫惎鍔ㄤ笌鏇濆厜鐨?
		鍚姩淇濇寔鍚屾锛?
 - intensity_to_led_brightness 涓?led_brightness_to_intensity
		浠ヨ澶囩壒瀹氱殑鏂瑰紡鎵ц
		enum led_brightness <-> V4L2 浜害鍊肩殑杞崲鈥斺€斿畠浠彲鐢ㄤ簬
		鍏锋湁闈炵嚎鎬?LED 鐢垫祦鍒诲害鐨勮澶囥€?
- config:
	V4L2 闂厜瀛愯澶囩殑閰嶇疆

 - dev_name
		濯掍綋瀹炰綋鐨勫悕绉帮紝鍦ㄧ郴缁熶腑鍞竴锛?
 - flash_faults
		LED 闂厜绫昏澶囧彲鎶ュ憡鐨勯棯鍏夋晠闅滅殑浣嶆帺鐮侊紱
		鐩稿簲鐨?LED_FAULT* 浣嶅畾涔夊彲鍦?<linux/led-class-flash.h> 涓壘鍒帮紝
 - torch_intensity
		闂厜妯″紡涓?LED 鐨勭害鏉燂紝浠ュ井瀹変负鍗曚綅锛?
 - indicator_intensity
		鎸囩ず LED 鐨勭害鏉燂紝浠ュ井瀹変负鍗曚綅锛?
 - has_external_strobe
		鍐冲畾闂厜 strobe 鏉ユ簮鏄惁鍙垏鎹㈠埌
		澶栭儴锛?

鍦ㄧЩ闄ゆ椂锛屽繀椤昏皟鐢?v4l2_flash_release 鍑芥暟锛屽畠鎺ュ彈涓€涓弬鏁扳€斺€斿嵆鍏堝墠鐢?v4l2_flash_init 杩斿洖鐨?struct v4l2_flash 鎸囬拡銆傝鍑芥暟鍙互瀹夊叏鍦颁互 NULL 鎴栭敊璇寚閽堜綔涓哄弬鏁拌皟鐢ㄣ€?

鏈夊叧 v4l2 闂厜灏佽鐨勭ず渚嬬敤娉曪紝璇峰弬闃?drivers/leds/leds-max77693.c銆?

涓€鏃︾敱鍒涘缓浜?Media controller 璁惧鐨勯┍鍔ㄦ敞鍐屼簡 V4L2 瀛愯澶囷紝璇ュ瓙璁惧鑺傜偣灏辫〃鐜板緱濡傚悓鍘熺敓 V4L2 闂厜 API 璁惧鐨勮妭鐐逛竴鏍枫€傝皟鐢ㄥ彧鏄璺敱鍒?LED 闂厜 API銆?

鎵撳紑 V4L2 闂厜瀛愯澶囦細浣?LED 瀛愮郴缁熺殑 sysfs 鎺ュ彛涓嶅彲鐢ㄣ€傚湪璇?V4L2 闂厜瀛愯澶囧叧闂悗锛屾帴鍙ｄ細閲嶆柊鍚敤銆?
