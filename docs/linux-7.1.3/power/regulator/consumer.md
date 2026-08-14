## 璋冭妭鍣ㄦ秷璐硅€呴┍鍔ㄦ帴鍙?

鏈枃鎻忚堪浜嗛潰鍚戞秷璐硅€呰澶囬┍鍔ㄧ殑璋冭妭鍣紙regulator锛夋帴鍙ｃ€傛湳璇鏄庤鍙傞槄 overview.txt銆?

## 1. 娑堣垂鑰呰皟鑺傚櫒璁块棶锛堥潤鎬佷笌鍔ㄦ€侀┍鍔級


```
	regulator = regulator_get(dev, "Vcc");

```
娑堣垂鑰呬紶鍏ュ叾 struct device 鎸囬拡鍜岀數婧?ID銆傛牳蹇冮殢鍚庨€氳繃鏌ヨ鏈哄櫒鐗瑰畾鐨勬煡鎵捐〃鏉ユ壘鍒版纭殑璋冭妭鍣ㄣ€傚鏋滄煡鎵炬垚鍔燂紝璇ヨ皟鐢ㄥ皢杩斿洖涓€涓寚鍚戜负璇ユ秷璐硅€呬緵鐢电殑 struct regulator 鐨勬寚閽堛€?
```
	regulator_put(regulator);

```
娑堣垂鑰呭彲鑳界敱澶氫釜璋冭妭鍣ㄤ緵鐢碉紝渚嬪甯︽湁濡備笅浠ｇ爜鐨勭紪瑙ｇ爜鍣ㄦ秷璐硅€咃細
```
	struct regulator_bulk_data supplies[2];

	supplies[0].supply = "Vcc"; /* digital core */
	supplies[1].supply = "Avdd"; /* analog */

	ret = regulator_bulk_get(dev, ARRAY_SIZE(supplies), supplies);

	// convenience helper to call regulator_put() on multiple regulators
	regulator_bulk_free(ARRAY_SIZE(supplies), supplies);


```
璋冭妭鍣ㄨ闂嚱鏁?regulator_get() 鍜?regulator_put() 閫氬父浼氬垎鍒湪浣犵殑璁惧椹卞姩鐨?probe() 鍜?remove() 涓皟鐢ㄣ€?

## 2. 璋冭妭鍣ㄨ緭鍑哄惎鐢ㄤ笌绂佺敤锛堥潤鎬佷笌鍔ㄦ€侀┍鍔級


```
	int regulator_enable(regulator);

```
娉ㄦ剰锛?  鍦ㄨ皟鐢?regulator_enable() 涔嬪墠锛岀數婧愬彲鑳藉凡缁忚鍚敤銆傚鏋滄秷璐硅€呭叡浜璋冭妭鍣紝鎴栬€呰璋冭妭鍣ㄤ箣鍓嶅凡琚紩瀵煎姞杞界▼搴忔垨鍐呮牳鏉跨骇鍒濆鍖栦唬鐮佸惎鐢紝灏变細鍙戠敓杩欑鎯呭喌銆?
```
	int regulator_is_enabled(regulator);

```
褰撹皟鑺傚櫒琚惎鐢ㄦ椂锛岃繖灏嗚繑鍥炲ぇ浜?0 鐨勫€笺€?
```
	int regulator_bulk_enable(int num_consumers,
				  struct regulator_bulk_data *consumers);


```
```
	int regulator_disable(regulator);

```
```
	int regulator_bulk_disable(int num_consumers,
			 	   struct regulator_bulk_data *consumers);

```
娉ㄦ剰锛?  濡傛灉涓庡叾浠栨秷璐硅€呭叡浜紝杩欏彲鑳戒笉浼氱鐢ㄧ數婧愩€傚彧鏈夊綋鍚敤寮曠敤璁℃暟涓洪浂鏃讹紝璋冭妭鍣ㄦ墠浼氳绂佺敤銆?
```
	int regulator_force_disable(regulator);

```
```
	int regulator_bulk_force_disable(int num_consumers,
			 		 struct regulator_bulk_data *consumers);

```
娉ㄦ剰锛?  杩欏皢绔嬪嵆涓斿己鍒跺叧闂皟鑺傚櫒杈撳嚭銆傛墍鏈夋秷璐硅€呴兘浼氳鏂數銆?
## 3. 璋冭妭鍣ㄧ數鍘嬫帶鍒朵笌鐘舵€侊紙鍔ㄦ€侀┍鍔級


涓€浜涙秷璐硅€呴┍鍔ㄩ渶瑕佽兘澶熷姩鎬佸湴鏀瑰彉鍏朵緵鐢电數鍘嬩互鍖归厤绯荤粺宸ヤ綔鐐广€備緥濡傦紝CPUfreq 椹卞姩鍙互闅忛鐜囦竴璧疯皟鑺傜數鍘嬩互鑺傜渷鍔熻€楋紝SD 椹卞姩鍙兘闇€瑕侀€夋嫨姝ｇ‘鐨勫崱鐢靛帇绛夈€?
```
	int regulator_set_voltage(regulator, min_uV, max_uV);

```
鍏朵腑 min_uV 鍜?max_uV 鏄互寰紡涓哄崟浣嶇殑銆佸彲鎺ュ彈鐨勬渶灏忓拰鏈€澶х數鍘嬨€?
娉ㄦ剰锛氳繖鍙互鍦ㄨ皟鑺傚櫒鍚敤鎴栫鐢ㄦ椂璋冪敤銆傚鏋滃湪鍚敤鏃惰皟鐢紝鐢靛帇浼氱珛鍗虫敼鍙橈紱鍚﹀垯鐢靛帇閰嶇疆浼氬彂鐢熷彉鍖栵紝骞跺湪璋冭妭鍣ㄤ笅娆″惎鐢ㄦ椂瀹為檯璁剧疆鐢靛帇銆?
```
	int regulator_get_voltage(regulator);

```
娉ㄦ剰锛?  get_voltage() 鏃犺璋冭妭鍣ㄥ惎鐢ㄨ繕鏄鐢ㄩ兘浼氳繑鍥為厤缃殑杈撳嚭鐢靛帇锛屼笉搴斾娇鐢ㄥ畠鏉ュ垽鏂皟鑺傚櫒鐨勮緭鍑虹姸鎬併€備笉杩囧畠鍙互涓?is_enabled() 閰嶅悎浣跨敤锛屼互纭畾璋冭妭鍣ㄧ殑瀹為檯杈撳嚭鐢靛帇銆?

## 4. 璋冭妭鍣ㄧ數娴侀檺鍒舵帶鍒朵笌鐘舵€侊紙鍔ㄦ€侀┍鍔級


涓€浜涙秷璐硅€呴┍鍔ㄩ渶瑕佽兘澶熷姩鎬佸湴鏀瑰彉鍏朵緵鐢电數娴侀檺鍒朵互鍖归厤绯荤粺宸ヤ綔鐐广€備緥濡傦紝LCD 鑳屽厜椹卞姩鍙互鏀瑰彉鐢垫祦闄愬埗鏉ヨ皟鑺傝儗鍏変寒搴︼紝USB 椹卞姩鍦ㄤ緵鐢垫椂鍙兘鎯虫妸闄愬埗璁句负 500mA銆?
```
	int regulator_set_current_limit(regulator, min_uA, max_uA);

```
鍏朵腑 min_uA 鍜?max_uA 鏄互寰畨涓哄崟浣嶇殑銆佸彲鎺ュ彈鐨勬渶灏忓拰鏈€澶х數娴侀檺鍒躲€?
娉ㄦ剰锛?  杩欏彲浠ュ湪璋冭妭鍣ㄥ惎鐢ㄦ垨绂佺敤鏃惰皟鐢ㄣ€傚鏋滃湪鍚敤鏃惰皟鐢紝鐢垫祦闄愬埗浼氱珛鍗虫敼鍙橈紱鍚﹀垯鐢垫祦闄愬埗閰嶇疆浼氬彂鐢熷彉鍖栵紝骞跺湪璋冭妭鍣ㄤ笅娆″惎鐢ㄦ椂瀹為檯璁剧疆鐢垫祦闄愬埗銆?
```
	int regulator_get_current_limit(regulator);

```
娉ㄦ剰锛?  get_current_limit() 鏃犺璋冭妭鍣ㄥ惎鐢ㄨ繕鏄鐢ㄩ兘浼氳繑鍥炵數娴侀檺鍒讹紝涓嶅簲浣跨敤瀹冩潵鍒ゆ柇璋冭妭鍣ㄧ殑鐢垫祦璐熻浇銆?

## 5. 璋冭妭鍣ㄥ伐浣滄ā寮忔帶鍒朵笌鐘舵€侊紙鍔ㄦ€侀┍鍔級


褰撴秷璐硅€呯殑宸ヤ綔鐘舵€佹敼鍙樻椂锛屼竴浜涙秷璐硅€呭彲浠ラ€氳繃灏嗕负鍏朵緵鐢电殑璋冭妭鍣ㄧ殑宸ヤ綔妯″紡鏀逛负鏇撮珮鏁堟潵杩涗竴姝ヨ妭鐪佺郴缁熷姛鑰椼€備緥濡傦紝娑堣垂鑰呴┍鍔ㄧ┖闂插悗闅忎箣娑堣€楁洿灏戠殑鐢垫祦銆?
璋冭妭鍣ㄧ殑宸ヤ綔妯″紡鍙互闂存帴鎴栫洿鎺ュ湴鏀瑰彉銆?
### 闂存帴宸ヤ綔妯″紡鎺у埗銆?
娑堣垂鑰呴┍鍔ㄥ彲浠ヨ姹傛敼鍙樹负鍏朵緵鐢电殑璋冭妭鍣ㄧ殑宸ヤ綔妯″紡
```
	int regulator_set_load(struct regulator *regulator, int load_uA);

```
杩欏皢浣挎牳蹇冮噸鏂拌绠楄皟鑺傚櫒涓婄殑鎬昏礋杞斤紙鍩轰簬鍏舵墍鏈夋秷璐硅€咃級锛屽苟鍦ㄥ繀瑕佹椂鍙婂厑璁哥殑鎯呭喌涓嬫敼鍙樺伐浣滄ā寮忥紝浠ユ渶浣冲尮閰嶅綋鍓嶅伐浣滆礋杞姐€?
load_uA 鍊煎彲浠ヤ粠娑堣垂鑰呯殑鏁版嵁鎵嬪唽涓‘瀹氥€備緥濡傦紝澶у鏁版暟鎹墜鍐岄兘鏈夎〃鏍兼樉绀哄湪鏌愪簺鎯呭喌涓嬬殑鐢垫祦娑堣€楁渶澶у€笺€?
澶у鏁版秷璐硅€呬細浣跨敤闂存帴宸ヤ綔妯″紡鎺у埗锛屽洜涓哄畠浠笉浜嗚В璋冭妭鍣紝涔熶笉鐭ラ亾璋冭妭鍣ㄦ槸鍚︿笌鍏朵粬娑堣垂鑰呭叡浜€?
### 鐩存帴宸ヤ綔妯″紡鎺у埗銆?

瀹氬埗鐨勬垨绱у瘑鑰﹀悎鐨勯┍鍔ㄥ彲鑳藉笇鏈涙牴鎹叾宸ヤ綔鐐圭洿鎺ユ帶鍒惰皟鑺傚櫒鐨勫伐浣滄ā寮忋€傝繖鍙互閫氳繃浠ヤ笅鏂瑰紡瀹炵幇锛?```
	int regulator_set_mode(struct regulator *regulator, unsigned int mode);
	unsigned int regulator_get_mode(struct regulator *regulator);

```
鐩存帴妯″紡鍙細琚偅浜?*浜嗚В**璇ヨ皟鑺傚櫒銆佷笖鏈笌鍏朵粬娑堣垂鑰呭叡浜璋冭妭鍣ㄧ殑娑堣垂鑰呬娇鐢ㄣ€?

## 6. 璋冭妭鍣ㄤ簨浠?

璋冭妭鍣ㄥ彲浠ュ悜娑堣垂鑰呴€氱煡澶栭儴浜嬩欢銆傛秷璐硅€呭彲鑳藉湪璋冭妭鍣ㄥ浜庡帇鍔涙垨鏁呴殰鏉′欢涓嬫椂鏀跺埌浜嬩欢銆?
```
	int regulator_register_notifier(struct regulator *regulator,
					struct notifier_block *nb);

```
```
	int regulator_unregister_notifier(struct regulator *regulator,
					  struct notifier_block *nb);

```
璋冭妭鍣ㄤ娇鐢ㄥ唴鏍搁€氱煡锛坣otifier锛夋鏋跺悜鎰熷叴瓒ｅ畠浠殑娑堣垂鑰呭彂閫佷簨浠躲€?
## 7. 璋冭妭鍣ㄧ洿鎺ュ瘎瀛樺櫒璁块棶


鏌愪簺鐢垫簮绠＄悊纭欢鎴栧浐浠惰璁捐鎴愰渶瑕佸璋冭妭鍣ㄨ繘琛屽簳灞傜‖浠惰闂紝涓斾笉娑夊強鍐呮牳銆傝繖绫昏澶囩殑渚嬪瓙鏈夛細

- 甯︽湁鍘嬫帶鎸崱鍣ㄥ拰閫氳繃 I2C 鏀瑰彉渚涚數鐢靛帇浠ュ疄鐜版墍闇€杈撳嚭鏃堕挓棰戠巼鐨勬帶鍒堕€昏緫鐨勬椂閽熸簮
- 鑳藉鍦ㄨ繃鐑潯浠朵笅鍙戝嚭浠绘剰 I2C 浜嬪姟鏉ユ墽琛岀郴缁熸柇鐢电殑鐑鐞嗗浐浠?
瑕侀厤缃繖鏍风殑璁惧/鍥轰欢锛岄渶瑕佸皢璋冭妭鍣ㄧ殑 I2C 鍦板潃銆佸悇绉嶈皟鑺傚櫒瀵勫瓨鍣ㄥ湴鍧€绛夊弬鏁伴厤缃粰瀹冦€傝皟鑺傚櫒妗嗘灦鎻愪緵浠ヤ笅杈呭姪鍑芥暟鏉ユ煡璇㈣繖浜涚粏鑺傘€?
鎬荤嚎鐩稿叧鐨勭粏鑺傦紙濡?I2C 鍦板潃鎴栦紶杈撻€熺巼锛夌敱
```
	struct regmap *regulator_get_regmap(struct regulator *regulator);

```
瑕佽幏鍙栬皟鑺傚櫒鐢靛帇鐨勭‖浠跺瘎瀛樺櫒鍋忕Щ鍜屼綅鎺╃爜
```
	int regulator_get_hardware_vsel_register(struct regulator *regulator,
						 unsigned *vsel_reg,
						 unsigned *vsel_mask);

```
瑕佸皢璋冭妭鍣ㄦ鏋剁殑鐢靛帇閫夋嫨鍣ㄤ唬鐮侊紙鐢?regulator_list_voltage 浣跨敤锛夎浆鎹负鍙互
```
	int regulator_list_hardware_vsel(struct regulator *regulator,
					 unsigned selector);

```
瑕佽闂‖浠朵互鍚敤/绂佺敤璋冭妭鍣紝娑堣垂鑰呭繀椤讳娇鐢?regulator_get_exclusive()锛屽洜涓哄鏋滃瓨鍦ㄥ涓?```
	int regulator_hardware_enable(struct regulator *regulator, bool enable);

```
