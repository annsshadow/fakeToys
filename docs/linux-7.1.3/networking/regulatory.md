
## Linux 鏃犵嚎鐩戠鏂囨。


鏈枃妗ｇ畝瑕佷粙缁?Linux 鏃犵嚎鐩戠鍩虹璁炬柦鐨勫伐浣滃師鐞嗐€?
鏇存柊鐨勪俊鎭彲鍦ㄩ」鐩綉椤佃幏鍙栵細

https://wireless.wiki.kernel.org/en/developers/Regulatory

### 鍦ㄧ敤鎴风┖闂寸淮鎶ょ洃绠″煙


鐢变簬鐩戠鍩熷叿鏈夊姩鎬佹€э紝鎴戜滑灏嗗畠浠繚鐣欏湪鐢ㄦ埛绌洪棿锛屽苟鎻愪緵涓€涓鏋讹紝渚涚敤鎴风┖闂?鍚戝唴鏍镐笂浼犱竴涓洃绠″煙锛屼綔涓烘墍鏈夋棤绾胯澶囬兘搴旈伒瀹堢殑涓ぎ鏍稿績鐩戠鍩熴€?
### 濡備綍灏嗙洃绠″煙鎻愪緵缁欏唴鏍?

褰撶洃绠″煙棣栨寤虹珛鏃讹紝鍐呮牳浼氳姹備竴涓寘鍚墍鏈夌洃绠¤鍒欑殑鏁版嵁搴撴枃浠?锛坮egulatory.db锛夈€傞殢鍚庡湪闇€瑕佹煡璇㈡煇涓浗瀹剁殑瑙勫垯鏃讹紝鍐呮牳浼氫娇鐢ㄨ繖涓暟鎹簱銆?
### 濡備綍灏嗙洃绠″煙鎻愪緵缁欏唴鏍革紙鏃?CRDA 鏂规锛?

鐢ㄦ埛绌洪棿閫氳繃鐢辩敤鎴风┖闂翠唬鐞嗘瀯寤虹洃绠″煙骞剁粡鐢?nl80211 鍙戦€侊紝浠庤€屽皢鍏舵彁渚涚粰鍐呮牳銆?鍐呮牳鍙細鎺ュ彈棰勬湡鐨勭洃绠″煙銆?
鐩墠鍙敤鐨勩€佽兘瀹屾垚姝や换鍔＄殑鐢ㄦ埛绌洪棿浠ｇ悊鏄?CRDA鈥斺€斾腑澶洃绠″煙浠ｇ悊锛坈entral
regulatory domain agent锛夈€傚叾鏂囨。瑙侊細

https://wireless.wiki.kernel.org/en/developers/Regulatory/CRDA

鏈川涓婏紝褰撳唴鏍哥煡閬撹嚜宸遍渶瑕佷竴涓柊鐩戠鍩熸椂锛屼細鍙戦€佷竴涓?udev 浜嬩欢銆傚彲浠ユ斁缃竴鏉?udev 瑙勫垯鏉ヨЕ鍙?crda锛屼负鐗瑰畾鐨?ISO/IEC 3166 alpha2 鍙戦€佺浉搴旂殑鐩戠鍩熴€?
涓嬮潰鏄竴涓彲浣跨敤鐨?udev 瑙勫垯绀轰緥锛?
# Example file, should be put in /etc/udev/rules.d/regulatory.rules
KERNEL=="regulatory*", ACTION=="change", SUBSYSTEM=="platform", RUN+="/sbin/crda"

alpha2 浣滀负鐜鍙橀噺 COUNTRY 浼犻€掋€?
### 璋佽姹傜洃绠″煙锛?

- 鐢ㄦ埛

鐢ㄦ埛鍙互浣跨敤 iw锛?
https://wireless.wiki.kernel.org/en/users/Documentation/iw

```
  # set regulatory domain to "Costa Rica"
  iw reg set CR

```
杩欎細璇锋眰鍐呮牳灏嗙洃绠″煙璁剧疆涓烘寚瀹氱殑 alpha2銆傚唴鏍搁殢鍚庝細閫氳繃鍙戦€?uevent锛岃姹?鐢ㄦ埛绌洪棿涓鸿鐢ㄦ埛鎸囧畾鐨?alpha2 鎻愪緵鐩戠鍩熴€?
- 鐢ㄤ簬鍥藉淇℃伅鍏冪礌锛圕ountry Information elements锛夌殑鏃犵嚎瀛愮郴缁?
鍐呮牳浼氬彂閫?uevent 閫氱煡鐢ㄦ埛绌洪棿闇€瑕佷竴涓柊鐩戠鍩熴€傞殢鐫€鍏堕泦鎴愮殑鍔犲叆锛屼細琛ュ厖鏇村
鍐呭銆?
- 椹卞姩

濡傛灉椹卞姩纭畾闇€瑕佽缃壒瀹氱殑鐩戠鍩燂紝瀹冧滑鍙互浣跨敤 regulatory_hint() 閫氱煡鏃犵嚎鏍稿績銆?瀹冧滑鏈変袱涓€夋嫨鈥斺€旇涔堟彁渚涗竴涓?alpha2锛屼互渚?crda 鑳借繑鍥炶鍥藉鐨勭洃绠″煙锛涜涔?鏍规嵁鑷韩鍐呴儴鐨勫畾鍒剁煡璇嗘瀯寤鸿嚜宸辩殑鐩戠鍩燂紝浣挎棤绾挎牳蹇冭兘澶熼伒瀹堝畠銆?
**澶у鏁?*椹卞姩渚濊禆绗竴绉嶆満鍒讹紝鍗虫彁渚涘甫 alpha2 鐨勭洃绠℃彁绀恒€傚浜庤繖浜涢┍鍔紝鍙互
浣跨敤涓€椤归澶栫殑妫€鏌ワ紝浠ュ熀浜庡畾鍒剁殑 EEPROM 鐩戠鏁版嵁纭繚鍚堣銆傞┍鍔ㄥ彲浠ラ€氳繃鍦ㄥ叾
struct wiphy 涓婃敞鍐屼竴涓?reg_notifier() 鍥炶皟鏉ヤ娇鐢ㄨ繖椤规鏌ャ€傚綋鏍稿績鐨勭洃绠″煙鍙戠敓
鍙樻洿鏃朵細璋冪敤姝ら€氱煡鍑芥暟銆傞┍鍔ㄥ彲浠ュ埄鐢ㄥ畠瀹℃煡鎵€鍋氱殑鏇存敼锛屽苟瀹℃煡鏄皝鍋氬嚭鐨勬洿鏀?锛堥┍鍔ㄣ€佺敤鎴枫€佸浗瀹?IE锛夛紝鐒跺悗鏍规嵁鍏跺唴閮?EEPROM 鏁版嵁鍐冲畾鍏佽浠€涔堛€傚笇鏈涘叿澶囧叏鐞?婕父鑳藉姏鐨勮澶囬┍鍔ㄥ簲浣跨敤姝ゅ洖璋冦€傞殢鐫€鍏舵敮鎸佺殑鍚敤锛屾湰鏂囨。浼氳ˉ鍏呮洿澶氬叧浜庡叏鐞?婕父鐨勫唴瀹广€?
鎻愪緵鑷韩鍐呯疆鐩戠鍩熺殑璁惧椹卞姩涓嶉渶瑕佸洖璋冿紝鍥犱负鐢卞畠浠敞鍐岀殑淇￠亾鏄敮涓€琚厑璁哥殑
淇￠亾锛屽洜姝?*棰濆**鐨勪俊閬撴棤娉曡鍚敤銆?
### 绀轰緥浠ｇ爜鈥斺€旈┍鍔ㄦ彁绀?alpha2锛?

鏈ず渚嬫潵鑷?zd1211rw 璁惧椹卞姩銆傛偍鍙互鍏堝缓绔嬭澶?EEPROM 鍥藉/鐩戠鍩熺殑鏄犲皠

```
  static struct zd_reg_alpha2_map reg_alpha2_map[] = {
	{ ZD_REGDOMAIN_FCC, "US" },
	{ ZD_REGDOMAIN_IC, "CA" },
	{ ZD_REGDOMAIN_ETSI, "DE" }, /* Generic ETSI, use most restrictive */
	{ ZD_REGDOMAIN_JAPAN, "JP" },
	{ ZD_REGDOMAIN_JAPAN_ADD, "JP" },
	{ ZD_REGDOMAIN_SPAIN, "ES" },
	{ ZD_REGDOMAIN_FRANCE, "FR" },
  };

```
鐒跺悗鎮ㄥ彲浠ュ畾涔変竴涓緥绋嬶紝灏嗚鍙栧埌鐨?EEPROM 鍊兼槧灏勪负 alpha2锛?
```
  static int zd_reg2alpha2(u8 regdomain, char *alpha2)
  {
	unsigned int i;
	struct zd_reg_alpha2_map *reg_map;
		for (i = 0; i < ARRAY_SIZE(reg_alpha2_map); i++) {
			reg_map = &reg_alpha2_map[i];
			if (regdomain == reg_map->reg) {
			alpha2[0] = reg_map->alpha2[0];
			alpha2[1] = reg_map->alpha2[1];
			return 0;
		}
	}
	return 1;
  }

```
鏈€鍚庯紝濡傛灉鎵惧埌鍖归厤椤癸紝鎮ㄥ彲浠ュ悜鏍稿績鎻愮ず鎮ㄥ彂鐜扮殑 alpha2銆傛偍闇€瑕佸湪娉ㄥ唽 wiphy
涔嬪悗鎵ц姝ゆ搷浣溿€傞鏈熷湪鍒濆鍖栨湡闂村畬鎴愩€?
```
	r = zd_reg2alpha2(mac->regdomain, alpha2);
	if (!r)
		regulatory_hint(hw->wiphy, alpha2);

```
### 绀轰緥浠ｇ爜鈥斺€旈┍鍔ㄦ彁渚涘唴缃洃绠″煙锛?

[娉ㄦ剰锛氭 API 褰撳墠涓嶅彲鐢紝闇€瑕佹椂鍐嶆坊鍔燷

濡傛灉鎮ㄦ湁鍙粠椹卞姩鑾峰彇鐨勭洃绠′俊鎭紝骞朵笖**闇€瑕?*浣跨敤姝ゆ柟寮忥紝鎴戜滑鍏佽鎮ㄦ瀯寤轰竴涓洃绠?鍩熺粨鏋勫苟灏嗗叾浼犻€掔粰鏃犵嚎鏍稿績銆備负姝わ紝鎮ㄥ簲 kmalloc() 涓€涓冻浠ュ绾崇洃绠″煙缁撴瀯鐨勫唴瀛橈紝
鐒跺悗濉叆鎮ㄧ殑鏁版嵁銆傛渶鍚庡彧闇€浠ヨ鐩戠鍩熺粨鏋勪负鍙傛暟璋冪敤 regulatory_hint()銆?
涓嬮潰鏄竴涓畝鍗曠ず渚嬶紝鐩戠鍩熶娇鐢ㄦ爤缂撳瓨銆傛偍鐨勫疄鐜板彲鑳戒笉鍚岋紙渚嬪鏀逛负璇诲彇 EEPROM
缂撳瓨锛夈€?
```
  struct ieee80211_regdomain mydriver_jp_regdom = {
	.n_reg_rules = 3,
	.alpha2 =  "JP",
	//.alpha2 =  "99", /* If I have no alpha2 to map it to */
	.reg_rules = {
		/* IEEE 802.11b/g, channels 1..14 */
		REG_RULE(2412-10, 2484+10, 40, 6, 20, 0),
		/* IEEE 802.11a, channels 34..48 */
		REG_RULE(5170-10, 5240+10, 40, 6, 20,
			NL80211_RRF_NO_IR),
		/* IEEE 802.11a, channels 52..64 */
		REG_RULE(5260-10, 5320+10, 40, 6, 20,
			NL80211_RRF_NO_IR|
			NL80211_RRF_DFS),
	}
  };

```
```

	struct ieee80211_regdomain *rd;
	int size_of_regd;
	int num_rules = mydriver_jp_regdom.n_reg_rules;
	unsigned int i;

	size_of_regd = sizeof(struct ieee80211_regdomain) +
		(num_rules * sizeof(struct ieee80211_reg_rule));

	rd = kzalloc(size_of_regd, GFP_KERNEL);
	if (!rd)
		return -ENOMEM;

	memcpy(rd, &mydriver_jp_regdom, sizeof(struct ieee80211_regdomain));

	for (i=0; i < num_rules; i++)
		memcpy(&rd->reg_rules[i],
		       &mydriver_jp_regdom.reg_rules[i],
		       sizeof(struct ieee80211_reg_rule));
	regulatory_struct_hint(rd);

```
### 闈欐€佺紪璇戠殑鐩戠鏁版嵁搴?

褰撴煇涓暟鎹簱闇€瑕佸浐鍖栬繘鍐呮牳鏃讹紝鍙互鍦ㄦ瀯寤烘椂浣滀负涓€涓浐浠舵枃浠舵彁渚涳紝闅忓悗琚摼鎺ヨ繘
鍐呮牳銆?