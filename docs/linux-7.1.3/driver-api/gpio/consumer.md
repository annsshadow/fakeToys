## GPIO 鎻忚堪绗︽秷璐硅€呮帴鍙?


鏈枃妗ｆ弿杩?GPIO 妗嗘灦鐨勬秷璐硅€咃紙consumer锛夋帴鍙ｃ€?


## GPIO 娑堣垂鑰呮寚鍗?


鏃犳硶鍦ㄦ爣鍑?GPIO 璋冪敤缂哄け鐨勬儏鍐典笅宸ヤ綔鐨勯┍鍔紝搴斿綋鍏锋湁渚濊禆锛坉epend on锛塆PIOLIB 鎴?
閫夋嫨锛坰elect锛塆PIOLIB 鐨?Kconfig 鏉＄洰銆傚厑璁搁┍鍔ㄤ娇鐢?GPIO 鐨勫嚱鏁伴兘澹版槑鍦?
<linux/gpio/consumer.h> 澶存枃浠朵腑锛?

```
	#include <linux/gpio/consumer.h>
```

鍦?GPIOLIB 琚鐢ㄧ殑鎯呭喌涓嬶紝澶存枃浠朵腑涓烘墍鏈夊嚱鏁版彁渚涗簡闈欐€佸唴鑱旀々锛坰tub锛夊嚱鏁般€傝皟鐢?
杩欎簺妗╁嚱鏁版椂浼氬彂鍑鸿鍛娿€傝繖浜涙々鍑芥暟鐢ㄤ簬涓ょ鐢ㄤ緥锛?

- 绠€鍗曠殑缂栬瘧瑕嗙洊锛坈ompile coverage锛夛紝渚嬪浣跨敤 COMPILE_TEST鈥斺€斿綋鍓嶅钩鍙版槸鍚﹀惎鐢ㄦ垨
  閫夋嫨 GPIOLIB 骞朵笉閲嶈锛屽洜涓烘垜浠湰鏉ュ氨涓嶆墦绠楄繍琛岃绯荤粺銆?

- 鐪熸鍙€夌殑 GPIOLIB 鏀寔鈥斺€旈┍鍔ㄥ湪鏌愪簺缂栬瘧鏃堕厤缃笅鐨勬煇浜涚郴缁熶腑骞朵笉鐪熸浣跨敤
  GPIO锛屼絾鍦ㄥ叾瀹冪紪璇戞椂閰嶇疆涓嬩細浣跨敤銆傚湪杩欑鎯呭喌涓嬶紝娑堣垂鑰呭繀椤荤‘淇濅笉璋冪敤杩欎簺鍑芥暟锛?
  鍚﹀垯鐢ㄦ埛浼氶亣鍒板彲鑳戒护浜轰笉瀹夌殑鎺у埗鍙拌鍛娿€傚皢鐪熸鍙€夌殑 GPIOLIB 浣跨敤涓庡
  `[devm_]gpiod_get_optional()` 鐨勮皟鐢ㄧ粨鍚堣捣鏉ユ槸涓€涓?*绯熺硶鐨勪富鎰?*锛屽苟浼氬鑷村鎬?
  鐨勯敊璇秷鎭€傝瀵瑰彲閫夌殑 GPIOLIB 浣跨敤鏅€氱殑 getter 鍑芥暟锛氳繖鏍峰仛鏃跺簲褰撻鏈熼渶瑕佷竴浜?
  鎵嬪姩缂栧啓鐨勯敊璇鐞嗙殑浠ｇ爜銆?

鎵€鏈変娇鐢ㄥ熀浜庢弿杩扮鐨?GPIO 鎺ュ彛鐨勫嚱鏁伴兘浠?`gpiod_` 涓哄墠缂€銆俙gpio_` 鍓嶇紑鐢ㄤ簬閬楃暀
锛坙egacy锛夋帴鍙ｃ€傚唴鏍镐腑鍏跺畠鍑芥暟涓嶅簲浣跨敤杩欎簺鍓嶇紑銆傚己鐑堜笉寤鸿浣跨敤閬楃暀鍑芥暟锛屾柊浠ｇ爜搴斿綋
浠呬娇鐢?<linux/gpio/consumer.h> 鍜屾弿杩扮銆?


## 鑾峰彇涓庨噴鏀?GPIO


鍦ㄥ熀浜庢弿杩扮鐨勬帴鍙ｄ腑锛孏PIO 閫氳繃涓嶉€忔槑锛坥paque锛夈€佷笉鍙吉閫狅紙non-forgeable锛夌殑鍙ユ焺
锛坔andler锛夋潵鏍囪瘑锛岃鍙ユ焺蹇呴』閫氳繃瀵规煇涓?gpiod_get() 鍑芥暟鐨勮皟鐢ㄦ潵鑾峰彇銆備笌璁稿鍏跺畠
鍐呮牳瀛愮郴缁熶竴鏍凤紝gpiod_get() 鎺ユ敹灏嗕娇鐢ㄨ GPIO 鐨勮澶囷紝浠ュ強鎵€璇锋眰 GPIO 搴斿綋
瀵瑰簲鐨勫姛鑳斤細

```
	struct gpio_desc *gpiod_get(struct device *dev, const char *con_id,
				    enum gpiod_flags flags)
```

濡傛灉鏌愪釜鍔熻兘鏄€氳繃灏嗗涓?GPIO 涓€璧蜂娇鐢ㄦ潵瀹炵幇鐨勶紙渚嬪涓€涓畝鍗曠殑 LED锛夛紝鍒欎娇鐢細

```
	struct gpio_desc *gpiod_get_index(struct device *dev,
					  const char *con_id, unsigned int idx,
					  enum gpiod_flags flags)
```

鍏充簬鍦?DeviceTree 鎯呭喌涓?con_id 鍙傛暟鐨勬洿璇︾粏鎻忚堪锛岃鍙傝
Documentation/driver-api/gpio/board.rst

flags 鍙傛暟鐢ㄤ簬鍙€夊湴鎸囧畾 GPIO 鐨勬柟鍚戝拰鍒濆鍊笺€傚叾鍊煎彲浠ヤ负锛?

- GPIOD_ASIS 鎴?0锛氬畬鍏ㄤ笉鍒濆鍖栬 GPIO銆傛柟鍚戝繀椤荤◢鍚庨€氳繃鏌愪釜涓撶敤鍑芥暟璁剧疆銆?
- GPIOD_IN锛氬皢璇?GPIO 鍒濆鍖栦负杈撳叆銆?
- GPIOD_OUT_LOW锛氬皢璇?GPIO 鍒濆鍖栦负杈撳嚭锛屽€间负 0銆?
- GPIOD_OUT_HIGH锛氬皢璇?GPIO 鍒濆鍖栦负杈撳嚭锛屽€间负 1銆?
- GPIOD_OUT_LOW_OPEN_DRAIN锛氫笌 GPIOD_OUT_LOW 鐩稿悓锛屼絾杩樺己鍒惰绾胯矾浠ュ紑婕?
  锛坥pen drain锛夋柟寮忕數姘斾娇鐢ㄣ€?
- GPIOD_OUT_HIGH_OPEN_DRAIN锛氫笌 GPIOD_OUT_HIGH 鐩稿悓锛屼絾杩樺己鍒惰绾胯矾浠ュ紑婕忔柟寮?
  鐢垫皵浣跨敤銆?

娉ㄦ剰锛屽垵濮嬪€兼槸**閫昏緫**锛坙ogical锛夊€硷紝鐗╃悊绾胯矾鐢靛钩鍙栧喅浜庤绾胯矾琚厤缃负浣庢湁鏁?
锛坅ctive low锛夎繕鏄珮鏈夋晥锛坅ctive high锛夛紙瑙?active_low_semantics锛夈€?

鏈€鍚庝袱涓爣蹇楃敤浜庡紑婕忔槸蹇呴渶鐨勪娇鐢ㄥ満鏅紝渚嬪 I2C锛氬鏋滆绾胯矾鍦ㄦ槧灏勶紙瑙?board.rst锛?
涓皻鏈閰嶇疆涓哄紑婕忥紝閭ｄ箞鏃犺濡備綍閮戒細寮哄埗浣跨敤寮€婕忥紝骞舵墦鍗颁竴鏉¤鍛婏紝鎻愮ず闇€瑕佹洿鏂版澘绾?
閰嶇疆浠ュ尮閰嶈浣跨敤鍦烘櫙銆?

涓や釜鍑芥暟閮借繑鍥炰竴涓湁鏁堢殑 GPIO 鎻忚堪绗︼紝鎴栬€呬竴涓彲鐢?IS_ERR() 妫€鏌ョ殑閿欒鐮侊紙瀹冧滑
姘歌繙涓嶄細杩斿洖 NULL 鎸囬拡锛夈€傚綋涓斾粎褰撴病鏈?GPIO 琚垎閰嶇粰璇ヨ澶?鍔熻兘/绱㈠紩涓夊厓缁勬椂锛?
浼氳繑鍥?-ENOENT锛涘叾瀹冮敊璇爜鐢ㄤ簬宸茬粡鍒嗛厤浜?GPIO 浣嗚幏鍙栧畠鏃跺彂鐢熼敊璇殑鎯呭喌銆傝繖瀵逛簬鍖哄垎
鏅€氶敊璇拰鍙€?GPIO 鍙傛暟缂哄け GPIO 寰堟湁鐢ㄣ€傚浜?GPIO 鍙€夌殑甯歌妯″紡锛屽彲浠ヤ娇鐢?
gpiod_get_optional() 鍜?gpiod_get_index_optional() 鍑芥暟銆傝繖浜涘嚱鏁板湪娌℃湁 GPIO 鏃?
杩斿洖 NULL锛?

```
	struct gpio_desc *gpiod_get_optional(struct device *dev,
					     const char *con_id,
					     enum gpiod_flags flags)

	struct gpio_desc *gpiod_get_index_optional(struct device *dev,
						   const char *con_id,
						   unsigned int index,
						   enum gpiod_flags flags)
```

娉ㄦ剰锛実pio_get*_optional() 鍑芥暟锛堝強鍏舵墭绠″彉浣擄級涓?gpiolib API 鐨勫叾浣欓儴鍒嗕笉鍚岋紝鍦?
gpiolib 鏀寔琚鐢ㄦ椂涔熶細杩斿洖 NULL銆傝繖瀵归┍鍔ㄤ綔鑰呭緢鏈夊府鍔╋紝鍥犱负浠栦滑涓嶉渶瑕佺壒鍒?
-ENOSYS 杩斿洖鐮併€備笉杩囩郴缁熼泦鎴愬憳搴斿綋灏忓績锛屽湪闇€瑕?gpiolib 鐨勭郴缁熶笂鍚敤瀹冦€?

```
	struct gpio_descs *gpiod_get_array(struct device *dev,
					   const char *con_id,
					   enum gpiod_flags flags)
```

璇ュ嚱鏁拌繑鍥炰竴涓寘鍚弿杩扮鏁扮粍鐨?struct gpio_descs銆傚畠杩樺寘鍚竴涓寚鍚?gpiolib
绉佹湁缁撴瀯鐨勬寚閽堬紝璇ョ粨鏋勶細

```
	struct gpio_descs {
		struct gpio_array *info;
		unsigned int ndescs;
		struct gpio_desc *desc[];
	}
```

濡傛灉娌℃湁鍒嗛厤浠讳綍 GPIO锛屼互涓嬪嚱鏁拌繑鍥?NULL 鑰岄潪 -ENOENT锛?

```
	struct gpio_descs *gpiod_get_array_optional(struct device *dev,
						    const char *con_id,
						    enum gpiod_flags flags)
```

```
	struct gpio_desc *devm_gpiod_get(struct device *dev, const char *con_id,
					 enum gpiod_flags flags)

	struct gpio_desc *devm_gpiod_get_index(struct device *dev,
					       const char *con_id,
					       unsigned int idx,
					       enum gpiod_flags flags)

	struct gpio_desc *devm_gpiod_get_optional(struct device *dev,
						  const char *con_id,
						  enum gpiod_flags flags)

	struct gpio_desc *devm_gpiod_get_index_optional(struct device *dev,
							const char *con_id,
							unsigned int index,
							enum gpiod_flags flags)

	struct gpio_descs *devm_gpiod_get_array(struct device *dev,
						const char *con_id,
						enum gpiod_flags flags)

	struct gpio_descs *devm_gpiod_get_array_optional(struct device *dev,
							 const char *con_id,
							 enum gpiod_flags flags)
```

```
	void gpiod_put(struct gpio_desc *desc)
```

```
	void gpiod_put_array(struct gpio_descs *descs)
```

鍦ㄨ皟鐢ㄨ繖浜涘嚱鏁颁箣鍚庝娇鐢ㄦ弿杩扮鏄弗鏍肩姝㈢殑銆備篃涓嶅厑璁镐粠閫氳繃 gpiod_get_array() 鑾峰彇鐨?
鏁扮粍涓崟鐙噴鏀炬弿杩扮锛堜娇鐢?gpiod_put()锛夈€?

```
	void devm_gpiod_put(struct device *dev, struct gpio_desc *desc)

	void devm_gpiod_put_array(struct device *dev, struct gpio_descs *descs)
```

## 浣跨敤 GPIO


### 璁剧疆鏂瑰悜


椹卞姩瀵?GPIO 瑕佸仛鐨勭涓€浠朵簨鏄缃叾鏂瑰悜銆傚鏋滄病鏈夌粰瀹氫换浣曟柟鍚戣缃爣蹇楃粰
gpiod_get*()锛屽垯鐢变互涓嬪嚱鏁板畬鎴愶細

```
	int gpiod_direction_input(struct gpio_desc *desc)
	int gpiod_direction_output(struct gpio_desc *desc, int value)
```

杩斿洖鍊间负闆惰〃绀烘垚鍔燂紝鍚﹀垯涓鸿礋鐨?errno銆傚簲褰撴鏌ヨ杩斿洖鍊硷紝鍥犱负 get/set 璋冪敤涓嶄細杩斿洖
閿欒锛屼笖鍙兘鍙戠敓閿欒閰嶇疆銆備綘閫氬父搴斿綋鍦ㄤ换鍔★紙task锛変笂涓嬫枃涓彂鍑鸿繖浜涜皟鐢ㄣ€傜劧鑰岋紝瀵逛簬
鑷棆閿佸畨鍏ㄧ殑锛坰pinlock-safe锛塆PIO锛屽湪浠诲姟鍚敤涔嬪墠銆佷綔涓烘棭鏈熸澘绾у垵濮嬪寲鐨勪竴閮ㄥ垎浣跨敤
瀹冧滑鏄病闂鐨勩€?

瀵逛簬杈撳嚭 GPIO锛屾墍鎻愪緵鐨勫€兼垚涓哄垵濮嬭緭鍑哄€笺€傝繖鏈夊姪浜庨伩鍏嶇郴缁熷惎鍔ㄦ湡闂寸殑淇″彿姣涘埡
锛坓litch锛夈€?

```
	int gpiod_get_direction(const struct gpio_desc *desc)
```

璇ュ嚱鏁拌繑鍥?0 琛ㄧず杈撳嚭锛? 琛ㄧず杈撳叆锛涘嚭閿欐椂杩斿洖閿欒鐮併€?

璇锋敞鎰忥紝GPIO 娌℃湁榛樿鏂瑰悜銆傚洜姝わ紝**鍦ㄤ笉鍏堣缃叾鏂瑰悜鐨勬儏鍐典笅浣跨敤 GPIO 鏄潪娉曡涓猴紝
骞跺皢瀵艰嚧鏈畾涔夌殑琛屼负锛?*


### 鑷棆閿佸畨鍏ㄧ殑 GPIO 璁块棶


澶у鏁?GPIO 鎺у埗鍣ㄥ彲浠ラ€氳繃鍐呭瓨璇?鍐欐寚浠よ闂€傝繖浜涗笉闇€瑕佺潯鐪狅紝骞朵笖鍙互瀹夊叏鍦颁粠纭?
锛堥潪绾跨▼鍖栵級IRQ 澶勭悊绋嬪簭鍙婄被浼间笂涓嬫枃涓繘琛屻€?

```
	int gpiod_get_value(const struct gpio_desc *desc);
	void gpiod_set_value(struct gpio_desc *desc, int value);
```

杩欎簺鍊兼槸甯冨皵鍊硷紝闆惰〃绀洪潪婵€娲伙紙inactive锛夛紝闈為浂琛ㄧず婵€娲伙紙active锛夈€傝鍙栬緭鍑哄紩鑴氱殑
鍊兼椂锛岃繑鍥炵殑鍊煎簲褰撴槸寮曡剼涓婃墍鐪嬪埌鐨勫€笺€傜敱浜庡寘鎷紑婕忎俊鍙凤紙open-drain signaling锛夊拰
杈撳嚭寤惰繜鍦ㄥ唴鐨勫悇绉嶉棶棰橈紝杩欏苟涓嶆€绘槸涓庢寚瀹氱殑杈撳嚭鍊肩浉鍖归厤銆?

get/set 璋冪敤涓嶄細杩斿洖閿欒锛屽洜涓?鈥滄棤鏁?GPIO鈥?搴斿綋鏃╁凡鐢?gpiod_direction_*() 鎶ュ憡銆?
鐒惰€岃娉ㄦ剰锛屽苟闈炴墍鏈夊钩鍙伴兘鑳借鍙栬緭鍑哄紩鑴氱殑鍊硷紱閭ｄ簺涓嶈兘璇诲彇鐨勫钩鍙板簲褰撳缁堣繑鍥為浂銆?
姝ゅ锛屽杩欎簺鍦ㄦ病鏈夌潯鐪犵殑鎯呭喌涓嬫棤娉曞畨鍏ㄨ闂殑 GPIO锛堣涓嬫枃锛変娇鐢ㄨ繖浜涜皟鐢ㄦ槸涓€绉嶉敊璇€?


### 鍙兘鐫＄湢鐨?GPIO 璁块棶


鏈変簺 GPIO 鎺у埗鍣ㄥ繀椤讳娇鐢ㄥ熀浜庢秷鎭殑鎬荤嚎锛堝 I2C 鎴?SPI锛夋潵璁块棶銆傝鍙栨垨鍐欏叆杩欎簺 GPIO
鍊肩殑鍛戒护闇€瑕佺瓑寰呮帓鍒伴槦鍒楀ご閮ㄤ互鍙戦€佸懡浠ゅ苟鑾峰彇鍏跺搷搴斻€傝繖闇€瑕佺潯鐪狅紝鑰岃繖鏄棤娉曞湪 IRQ
澶勭悊绋嬪簭鍐呴儴瀹屾垚鐨勩€?

鏀寔姝ょ被 GPIO 鐨勫钩鍙伴€氳繃浠ヤ笅鏂瑰紡灏嗗畠浠笌鍏跺畠 GPIO 鍖哄垎寮€鏉ワ細

```
	int gpiod_cansleep(const struct gpio_desc *desc)
```

```
	int gpiod_get_value_cansleep(const struct gpio_desc *desc)
	void gpiod_set_value_cansleep(struct gpio_desc *desc, int value)
```

璁块棶姝ょ被 GPIO 闇€瑕佷竴涓彲鑳界潯鐪犵殑涓婁笅鏂囷紝渚嬪绾跨▼鍖?IRQ 澶勭悊绋嬪簭锛屽苟涓斿繀椤讳娇鐢ㄨ繖浜?
璁块棶鍣紝鑰屼笉鏄笉甯?cansleep() 鍚庣紑鐨勮嚜鏃嬮攣瀹夊叏璁块棶鍣ㄣ€?

闄や簡杩欎簺璁块棶鍣ㄥ彲鑳界潯鐪犮€佷笖鑳藉伐浣滀簬鏃犳硶浠庣‖ IRQ 澶勭悊绋嬪簭璁块棶鐨?GPIO 涔嬪锛岃繖浜涜皟鐢?
鐨勮涓轰笌鑷棆閿佸畨鍏ㄧ殑璋冪敤鐩稿悓銆?


### 浣庢湁鏁堜笌寮€婕忚涔?


鐢变簬娑堣垂鑰呬笉搴斿叧蹇冪墿鐞嗙嚎璺數骞筹紝鎵€鏈夌殑 gpiod_set_value_xxx() 鎴?
gpiod_set_array_value_xxx() 鍑芥暟閮戒互**閫昏緫**锛坙ogical锛夊€艰繘琛屾搷浣溿€傜敱姝ゅ畠浠細鑰冭檻
浣庢湁鏁堬紙active low锛夊睘鎬с€傝繖鎰忓懗鐫€瀹冧滑浼氭鏌ヨ GPIO 鏄惁琚厤缃负浣庢湁鏁堬紝濡傛灉鏄紝鍒?
鍦ㄩ┍鍔ㄧ墿鐞嗙嚎璺數骞充箣鍓嶅浼犲叆鐨勫€艰繘琛岀浉搴斿鐞嗐€?

杩欏悓鏍烽€傜敤浜庡紑婕忔垨寮€婧愶紙open source锛夎緭鍑虹嚎璺細瀹冧滑涓嶄細涓诲姩椹卞姩杈撳嚭涓洪珮锛堝紑婕忥級鎴?
涓轰綆锛堝紑婧愶級锛岃€屽彧鏄皢鍏惰緭鍑哄垏鎹负楂橀樆锛坔igh impedance锛夊€笺€傛秷璐硅€呬笉搴旈渶瑕佸叧蹇冭繖鐐广€?
锛堢粏鑺傝闃呰 driver.rst 涓叧浜庡紑婕忕殑鍐呭銆傦級

鐢辨锛屾墍鏈?gpiod_set_(array)_value_xxx() 鍑芥暟灏嗗弬鏁?鈥渧alue鈥?瑙ｉ噴涓?鈥滄縺娲烩€濓紙鈥?鈥濓級
鎴?鈥滈潪婵€娲烩€濓紙鈥?鈥濓級銆傜墿鐞嗙嚎璺數骞冲皢琚浉搴斿湴椹卞姩銆?

渚嬪锛屽鏋滀负鏌愪釜涓撶敤 GPIO 璁剧疆浜嗕綆鏈夋晥灞炴€э紝鑰?gpiod_set_(array)_value_xxx() 浼犲叆
鈥滄縺娲烩€濓紙鈥?鈥濓級锛屽垯鐗╃悊绾胯矾鐢靛钩灏嗚椹卞姩涓轰綆銆?

```
  Function (example)                 line property          physical line
  gpiod_set_raw_value(desc, 0);      don't care             low
  gpiod_set_raw_value(desc, 1);      don't care             high
  gpiod_set_value(desc, 0);          default (active high)  low
  gpiod_set_value(desc, 1);          default (active high)  high
  gpiod_set_value(desc, 0);          active low             high
  gpiod_set_value(desc, 1);          active low             low
  gpiod_set_value(desc, 0);          open drain             low
  gpiod_set_value(desc, 1);          open drain             high impedance
  gpiod_set_value(desc, 0);          open source            high impedance
  gpiod_set_value(desc, 1);          open source            high
```

鍙互浣跨敤 set_raw/get_raw 鍑芥暟鏉ヨ鐩栬繖浜涜涔夛紝浣嗗簲褰撳敖鍙兘閬垮厤锛屽挨鍏舵槸涓庣郴缁熸棤鍏崇殑
椹卞姩锛屽畠浠笉搴旈渶瑕佸叧蹇冨疄闄呯殑鐗╃悊绾胯矾鐢靛钩锛岃€屽簲鍏虫敞閫昏緫鍊笺€?


### 璁块棶鍘熷 GPIO 鍊?


瀛樺湪杩欐牱鐨勬秷璐硅€咃細瀹冧滑闇€瑕佺鐞?GPIO 绾胯矾鐨勯€昏緫鐘舵€侊紝鍗冲叾璁惧瀹為檯灏嗘帴鏀跺埌鐨勫€硷紝鏃犺
鍏朵笌璇?GPIO 绾胯矾涔嬮棿闅旂潃浠€涔堛€?

浠ヤ笅杩欑粍璋冪敤浼氬拷鐣?GPIO 鐨勪綆鏈夋晥鎴栧紑婕忓睘鎬э細

```
	int gpiod_get_raw_value(const struct gpio_desc *desc)
	void gpiod_set_raw_value(struct gpio_desc *desc, int value)
	int gpiod_get_raw_value_cansleep(const struct gpio_desc *desc)
	void gpiod_set_raw_value_cansleep(struct gpio_desc *desc, int value)
	int gpiod_direction_output_raw(struct gpio_desc *desc, int value)
```

GPIO 鐨勪綆鏈夋晥鐘舵€佷篃鍙互浣跨敤浠ヤ笅鍑芥暟鏌ヨ鍜屽垏鎹細

```
	int gpiod_is_active_low(const struct gpio_desc *desc)
	void gpiod_toggle_active_low(struct gpio_desc *desc)
```

璇锋敞鎰忥紝杩欎簺鍑芥暟搴斿綋浠呭湪闈炲父鑺傚埗鐨勬儏鍐典笅浣跨敤锛涢┍鍔ㄤ笉搴旈渶瑕佸叧蹇冪墿鐞嗙嚎璺數骞虫垨寮€婕?
璇箟銆?


### 浣跨敤鍗曟璋冪敤璁块棶澶氫釜 GPIO


```
	int gpiod_get_array_value(unsigned int array_size,
				  struct gpio_desc **desc_array,
				  struct gpio_array *array_info,
				  unsigned long *value_bitmap);
	int gpiod_get_raw_array_value(unsigned int array_size,
				      struct gpio_desc **desc_array,
				      struct gpio_array *array_info,
				      unsigned long *value_bitmap);
	int gpiod_get_array_value_cansleep(unsigned int array_size,
					   struct gpio_desc **desc_array,
					   struct gpio_array *array_info,
					   unsigned long *value_bitmap);
	int gpiod_get_raw_array_value_cansleep(unsigned int array_size,
					   struct gpio_desc **desc_array,
					   struct gpio_array *array_info,
					   unsigned long *value_bitmap);

	int gpiod_set_array_value(unsigned int array_size,
				  struct gpio_desc **desc_array,
				  struct gpio_array *array_info,
				  unsigned long *value_bitmap)
	int gpiod_set_raw_array_value(unsigned int array_size,
				      struct gpio_desc **desc_array,
				      struct gpio_array *array_info,
				      unsigned long *value_bitmap)
	int gpiod_set_array_value_cansleep(unsigned int array_size,
					   struct gpio_desc **desc_array,
					   struct gpio_array *array_info,
					   unsigned long *value_bitmap)
	int gpiod_set_raw_array_value_cansleep(unsigned int array_size,
					       struct gpio_desc **desc_array,
					       struct gpio_array *array_info,
					       unsigned long *value_bitmap)
```

璇ユ暟缁勫彲浠ユ槸浠绘剰涓€缁?GPIO銆傚鏋滅浉搴旂殑鑺墖椹卞姩鏀寔锛岃繖浜涘嚱鏁颁細灏濊瘯鍚屾椂璁块棶灞炰簬鍚屼竴
bank 鎴栬姱鐗囩殑 GPIO銆傚湪杩欑鎯呭喌涓嬶紝鎬ц兘浼氭湁鏄捐憲鎻愬崌銆傚鏋滄棤娉曞悓鏃惰闂紝鍒?GPIO 灏嗚
椤哄簭璁块棶銆?

杩欎簺鍑芥暟鎺ュ彈鍥涗釜鍙傛暟锛?

 - array_size	- 鏁扮粍鍏冪礌鐨勬暟閲?
 - desc_array	- 涓€涓?GPIO 鎻忚堪绗︽暟缁?
 - array_info	- 浠?gpiod_get_array() 鑾峰彇鐨勫彲閫変俊鎭?
 - value_bitmap	- 鐢ㄤ簬瀛樺偍 GPIO 鍊肩殑浣嶅浘锛坓et锛夛紝鎴?
          瑕佸垎閰嶇粰 GPIO 鐨勫€肩殑浣嶅浘锛坰et锛?

鎻忚堪绗︽暟缁勫彲浠ヤ娇鐢?gpiod_get_array() 鍑芥暟鎴栧叾鏌愪釜鍙樹綋鑾峰彇銆傚鏋滆鍑芥暟杩斿洖鐨勬弿杩扮缁?
涓庢墍闇€鐨?GPIO 缁勭浉鍖归厤锛岄偅涔堝彧闇€浣跨敤浠ヤ笅鏂瑰紡鍗冲彲璁块棶杩欎簺 GPIO锛?

```
	struct gpio_descs *my_gpio_descs = gpiod_get_array(...);
	gpiod_set_array_value(my_gpio_descs->ndescs, my_gpio_descs->desc,
			      my_gpio_descs->info, my_gpio_value_bitmap);
```

涔熷彲浠ヨ闂畬鍏ㄤ换鎰忕殑鎻忚堪绗︽暟缁勩€傛弿杩扮鍙互浣跨敤 gpiod_get() 鍜?gpiod_get_array()
鐨勪换浣曠粍鍚堣幏鍙栥€備箣鍚庯紝鍦ㄥ皢璇ユ弿杩扮鏁扮粍浼犻€掔粰涓婅堪鍑芥暟涔嬩竴涔嬪墠锛屽繀椤绘墜鍔ㄨ缃畠銆傚湪杩欑
鎯呭喌涓嬶紝array_info 搴斿綋璁句负 NULL銆?

璇锋敞鎰忥紝涓轰簡鑾峰緱鏈€浣虫€ц兘锛屽睘浜庡悓涓€鑺墖鐨?GPIO 搴斿綋鍦ㄦ弿杩扮鏁扮粍涓繛缁帓鍒椼€?

濡傛灉鎻忚堪绗︾殑鏁扮粍绱㈠紩涓庡崟涓姱鐗囩殑纭欢寮曡剼鍙风浉鍖归厤锛屽垯鍙互瀹炵幇鏇村ソ鐨勬€ц兘銆傚鏋滀紶閫掔粰
get/set 鏁扮粍鍑芥暟鐨勬暟缁勪笌浠?gpiod_get_array() 鑾峰彇鐨勬暟缁勭浉鍖归厤锛屽苟涓斾篃浼犻€掍簡涓庤鏁扮粍
鍏宠仈鐨?array_info锛岄偅涔堝嚱鏁板彲鑳戒細閲囧彇蹇€熺殑浣嶅浘澶勭悊璺緞锛屽皢 value_bitmap 鍙傛暟鐩存帴
浼犻€掔粰璇ヨ姱鐗囩浉搴旂殑 .get/set_multiple() 鍥炶皟銆傝繖鏍峰彲浠ュ皢 GPIO bank 鐢ㄤ綔鏁版嵁 I/O 绔彛
鑰屼笉浼氭崯澶卞お澶氭€ц兘銆?

gpiod_get_array_value() 鍙婂叾鍙樹綋鐨勮繑鍥炲€间负 0 琛ㄧず鎴愬姛锛屼负璐熻〃绀哄嚭閿欍€傝娉ㄦ剰杩欎笌
gpiod_get_value() 鐨勫尯鍒紝鍚庤€呮垚鍔熸椂杩斿洖 0 鎴?1 浠ヤ紶杈?GPIO 鍊笺€傚浜庢暟缁勫嚱鏁帮紝GPIO
鍊煎瓨鍌ㄥ湪 value_array 涓紝鑰屼笉鏄綔涓鸿繑鍥炲€间紶鍥炪€?


### 鏄犲皠鍒?IRQ 鐨?GPIO


GPIO 绾胯矾缁忓父鍙互鐢ㄤ綔 IRQ銆備綘鍙互鑾峰彇 IRQ 鍙凤細

```
	int gpiod_to_irq(const struct gpio_desc *desc)
```

瀹冧細杩斿洖涓€涓?IRQ 鍙凤紝鎴栬€呭綋鏄犲皠鏃犳硶瀹屾垚鏃惰繑鍥炶礋鐨?errno 鐮侊紙鏈€鍙兘鏄洜涓鸿鐗瑰畾 GPIO
鏃犳硶鐢ㄤ綔 IRQ锛夈€備娇鐢ㄦ湭閫氳繃 gpiod_direction_input() 璁剧疆涓鸿緭鍏ョ殑 GPIO锛屾垨浣跨敤骞堕潪
鍘熸湰鏉ヨ嚜 gpiod_to_irq() 鐨?IRQ 鍙凤紝閮芥槸鏈鏌ョ殑閿欒銆俫piod_to_irq() 涓嶅厑璁哥潯鐪犮€?

gpiod_to_irq() 杩斿洖鐨勯潪閿欒鍊煎彲浠ヤ紶閫掔粰 request_irq() 鎴?free_irq()銆傚畠浠€氬父浼氳
鏉跨骇鐗瑰畾鐨勫垵濮嬪寲浠ｇ爜瀛樺叆骞冲彴璁惧鐨?IRQ 璧勬簮涓€傝娉ㄦ剰锛孖RQ 瑙﹀彂閫夐」锛堝
IRQF_TRIGGER_FALLING锛変互鍙婄郴缁熷敜閱掞紙wakeup锛夎兘鍔涢兘灞炰簬 IRQ 鎺ュ彛鐨勪竴閮ㄥ垎銆?


## GPIO 涓?ACPI


鍦?ACPI 绯荤粺涓婏紝GPIO 鐢辫澶?_CRS 閰嶇疆瀵硅薄鎵€鍒楀嚭鐨?GpioIo()/GpioInt() 璧勬簮鎻忚堪銆傝繖浜?
璧勬簮涓嶄负 GPIO 鎻愪緵杩炴帴 ID锛堝悕绉帮級锛屽洜姝ら渶瑕佷负姝や娇鐢ㄤ竴涓澶栫殑鏈哄埗銆?

绗﹀悎 ACPI 5.1 鎴栨洿鏂扮増鏈殑绯荤粺鍙互鎻愪緵 _DSD 閰嶇疆瀵硅薄锛岄櫎鍏跺畠鐢ㄩ€斿锛屽畠鍙敤浜庝负 _CRS
涓敱 GpioIo()/GpioInt() 璧勬簮鎻忚堪鐨勭壒瀹?GPIO 鎻愪緵杩炴帴 ID銆傚鏋滄槸杩欑鎯呭喌锛屽畠灏嗙敱 GPIO
瀛愮郴缁熻嚜鍔ㄥ鐞嗐€傜劧鑰岋紝濡傛灉 _DSD 涓嶅瓨鍦紝鍒?GpioIo()/GpioInt() 璧勬簮涓?GPIO 杩炴帴 ID
涔嬮棿鐨勬槧灏勯渶瑕佺敱璁惧椹卞姩鎻愪緵銆?

缁嗚妭璇峰弬闃?Documentation/firmware-guide/acpi/gpio-properties.rst


## 涓庨仐鐣?GPIO 瀛愮郴缁熶氦浜?


璁稿鍐呮牳瀛愮郴缁熷拰椹卞姩浠嶄娇鐢ㄩ仐鐣欑殑鍩轰簬鏁存暟鐨勬帴鍙ｅ鐞?GPIO銆傚己鐑堝缓璁皢杩欎簺鏇存柊涓烘柊鐨?
gpiod 鎺ュ彛銆傚浜庨渶瑕佸悓鏃朵娇鐢ㄤ袱绉嶆帴鍙ｇ殑鎯呭喌锛屼互涓嬩袱涓嚱鏁板厑璁稿皢 GPIO 鎻忚堪绗﹁浆鎹负
GPIO 鏁存暟鍛藉悕绌洪棿锛?

```
	int desc_to_gpio(const struct gpio_desc *desc)
	struct gpio_desc *gpio_to_desc(unsigned gpio)
```

鍙 GPIO 鎻忚堪绗?`desc` 鏈閲婃斁锛宒esc_to_gpio() 杩斿洖鐨?GPIO 鍙峰氨鍙互瀹夊叏鍦扮敤浣?
gpio\_*() 鍑芥暟鐨勫弬鏁般€傚悓鏍凤紝浼犻€掔粰 gpio_to_desc() 鐨?GPIO 鍙峰繀椤婚鍏堥€氳繃渚嬪
gpio_request_one() 姝ｇ‘鑾峰彇锛屽苟涓旇繑鍥炵殑 GPIO 鎻忚堪绗︿粎鍦ㄨ GPIO 鍙烽€氳繃 gpio_free()
閲婃斁涔嬪墠琚涓烘湁鏁堛€?

鐢ㄤ竴涓?API 閲婃斁鐢卞彟涓€涓?API 鑾峰彇鐨?GPIO 鏄绂佹鐨勶紝骞朵笖鏄湭妫€鏌ョ殑閿欒銆?
