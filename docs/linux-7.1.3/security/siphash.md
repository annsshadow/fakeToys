## SipHash 鈥斺€?涓€绉嶇煭杈撳叆 PRF


:Author: Written by Jason A. Donenfeld <jason@zx2c4.com>

SipHash 鏄竴绉嶅姞瀵嗗畨鍏ㄧ殑 PRF锛堝甫瀵嗛挜鐨勫搱甯屽嚱鏁帮級锛岄拡瀵圭煭杈撳叆琛ㄧ幇鏋佷匠锛?
鐢辨寰楀悕銆傚畠鐢卞瘑鐮佸瀹?Daniel J. Bernstein 鍜?Jean-Philippe Aumasson 璁捐锛?
鏃ㄥ湪鏇夸唬鏌愪簺鍦烘櫙涓嬪 `jhash`銆乣md5_transform`銆乣sha1_transform` 绛夊嚱鏁扮殑浣跨敤銆?

SipHash 鎺ュ彈涓€涓敱闅忔満鐢熸垚鐨勬暟瀛楀～鍏呰€屾垚鐨勫瘑閽ワ紝浠ュ強杈撳叆缂撳啿鍖烘垨鑻ュ共杈撳叆鏁存暟锛?
骞惰緭鍑轰竴涓笌闅忔満鏁版棤娉曞尯鍒嗙殑鏁存暟銆備綘鍙互灏嗚鏁存暟鐢ㄤ綔瀹夊叏搴忓垪鍙枫€佸畨鍏?
cookie 鐨勪竴閮ㄥ垎锛屾垨缁忔帺鐮佸鐞嗗悗鐢ㄤ簬鍝堝笇琛ㄣ€?

## 鐢熸垚瀵嗛挜


瀵嗛挜搴斿綋濮嬬粓鐢卞姞瀵嗗畨鍏ㄧ殑鏉ユ簮鐢熸垚
```

	siphash_key_t key;
	get_random_bytes(&key, sizeof(key));

```
濡傛灉浣犱笉鏄粠杩欓噷娲剧敓瀵嗛挜锛岄偅灏卞仛閿欎簡銆?

## 浣跨敤杩欎簺鍑芥暟


璇ュ嚱鏁扮殑鍙樹綋鏈変袱绉嶏細涓€绉嶆帴鍙楁暣鏁板垪琛紝鍙︿竴绉?
```

	u64 siphash(const void *data, size_t len, const siphash_key_t *key);

```
```

	u64 siphash_1u64(u64, const siphash_key_t *key);
	u64 siphash_2u64(u64, u64, const siphash_key_t *key);
	u64 siphash_3u64(u64, u64, u64, const siphash_key_t *key);
	u64 siphash_4u64(u64, u64, u64, u64, const siphash_key_t *key);
	u64 siphash_1u32(u32, const siphash_key_t *key);
	u64 siphash_2u32(u32, u32, const siphash_key_t *key);
	u64 siphash_3u32(u32, u32, u32, const siphash_key_t *key);
	u64 siphash_4u32(u32, u32, u32, u32, const siphash_key_t *key);

```
濡傛灉浣犲悜閫氱敤 siphash 鍑芥暟浼犲叆闀垮害鎭掑畾鐨勫唴瀹癸紝缂栬瘧鍣ㄤ細鍦ㄧ紪璇戞湡杩涜甯搁噺鎶樺彔锛?
骞惰嚜鍔ㄩ€夋嫨鍏朵腑涓€涓粡杩囦紭鍖栫殑鍑芥暟銆?


```

	struct some_hashtable {
		DECLARE_HASHTABLE(hashtable, 8);
		siphash_key_t key;
	};

	void init_hashtable(struct some_hashtable *table)
	{
		get_random_bytes(&table->key, sizeof(table->key));
	}

	static inline hlist_head *some_hashtable_bucket(struct some_hashtable *table, struct interesting_input *input)
	{
		return &table->hashtable[siphash(input, sizeof(*input), &table->key) & (HASH_SIZE(table->hashtable) - 1)];
	}

```
鐒跺悗浣犲彲浠ュ儚寰€甯镐竴鏍烽亶鍘嗚繑鍥炵殑鍝堝笇妗躲€?

## 瀹夊叏鎬?


SipHash 鍏锋湁鏋侀珮鐨勫畨鍏ㄨ搴︼紝鍏跺瘑閽ヤ负 128 浣嶃€傚彧瑕佸瘑閽ヤ繚鎸佹満瀵嗭紝鏀诲嚮鑰呭氨涓嶅彲鑳?
鐚滃嚭鍑芥暟鐨勮緭鍑猴紝鍗充究鑳藉瑙傚療鍒板ぇ閲忚緭鍑猴紝鍥犱负 2^128 绉嶈緭鍑烘槸鐩稿綋鍙鐨勩€?

Linux 瀹炵幇浜?SipHash 鐨?"2-4" 鍙樹綋銆?

## 缁撴瀯浣撲紶閫掗櫡闃?


寰堝鏃跺€?XuY 绯诲垪鍑芥暟瀹归噺涓嶈冻锛屾鏃朵綘浼氬笇鏈涘悜 siphash 浼犲叆涓€涓鍏堝～鍏呭ソ鐨?
缁撴瀯浣撱€傝繖鏍峰仛鏃讹紝鍔″繀纭繚缁撴瀯浣撲腑娌℃湁濉厖绌烘礊銆傛渶绠€鍗曠殑鏂规硶鏄細鎸夊ぇ灏?
闄嶅簭鎺掑垪缁撴瀯浣撶殑鎴愬憳锛屽苟鍦ㄨ幏鍙栧ぇ灏忔椂浣跨敤 offsetofend() 鑰岄潪 sizeof()銆傚嚭浜?
鎬ц兘鑰冭檻锛屽鏋滃彲鑳界殑璇濓紝灏嗙粨鏋勪綋杩涜瀵归綈鏄釜涓嶉敊鐨勫仛娉?
```

	const struct {
		struct in6_addr saddr;
		u32 counter;
		u16 dport;
	} __aligned(SIPHASH_ALIGNMENT) combined = {
		.saddr = *(struct in6_addr *)saddr,
		.counter = counter,
		.dport = dport
	};
	u64 h = siphash(&combined, offsetofend(typeof(combined), dport), &secret);

```
## 璧勬簮


濡傛灉浣犳湁鍏磋叮娣卞叆浜嗚В锛岃闃呰 SipHash 璁烘枃锛?
https://131002.net/siphash/siphash.pdf

### 

## HalfSipHash 鈥斺€?SipHash 涓嶅畨鍏ㄧ殑灏忚〃寮?


:Author: Written by Jason A. Donenfeld <jason@zx2c4.com>

涓囦竴 SipHash 鐨勯€熷害鏃犳硶婊¤冻浣犵殑闇€姹傦紝浣犲彲鑳借兘澶熸壘鍒颁娇鐢?HalfSipHash 鐨勭悊鐢扁€斺€?
涓€绉嶄护浜轰笉瀹変絾鎴栬鏈夌敤鐨勫彲鑳姐€侶alfSipHash 灏?SipHash 鐨勮疆鏁颁粠 "2-4" 鍓婂噺鍒?
"1-3"锛屾洿鍙€曠殑鏄紝瀹冧娇鐢ㄥ鏄撹鏆村姏鐮磋В鐨?64 浣嶅瘑閽ワ紙杈撳嚭涓?32 浣嶏級锛岃€岄潪
SipHash 鐨?128 浣嶅瘑閽ャ€備笉杩囷紝杩欏彲鑳藉鏌愪簺楂樻€ц兘鐨?`jhash` 鐢ㄦ埛鏈夊惛寮曞姏銆?

HalfSipHash 鐨勬敮鎸侀€氳繃 "hsiphash" 绯诲垪鍑芥暟鎻愪緵銆?

   鍒囧嬁灏?hsiphash 鍑芥暟鐢ㄤ簬闄ゅ搱甯岃〃閿嚱鏁颁互澶栫殑浠讳綍鐢ㄩ€旓紝涓斿彧鏈夊湪浣犺兘澶熺粷瀵?
   纭畾鍏惰緭鍑烘案杩滀笉浼氫紶鍑哄唴鏍告椂鎵嶅彲浣跨敤銆傜浉姣?`jhash`锛屽畠浠呬綔涓虹紦瑙ｅ搱甯岃〃
   娉涙椽鎷掔粷鏈嶅姟鏀诲嚮鐨勪竴绉嶆墜娈垫墠鍏锋湁鏈夐檺鐨勪环鍊笺€?

鍦?64 浣嶅唴鏍镐腑锛宧siphash 鍑芥暟瀹為檯涓婂疄鐜扮殑鏄?SipHash-1-3锛圫ipHash 鐨勭缉鍑忚疆鏁?
鍙樹綋锛夛紝鑰岄潪 HalfSipHash-1-3銆傝繖鏄洜涓哄湪 64 浣嶄唬鐮佷腑锛孲ipHash-1-3 骞朵笉姣?
HalfSipHash-1-3 鎱紝鐢氳嚦鍙兘鏇村揩銆傛敞鎰忥紝杩?*骞朵笉**鎰忓懗鐫€鍦?64 浣嶅唴鏍镐腑
hsiphash 鍑芥暟涓?siphash 鍑芥暟鐩稿悓锛屾垨鎰忓懗鐫€瀹冧滑鏄畨鍏ㄧ殑锛沨siphash 鍑芥暟浠嶄娇鐢?
瀹夊叏鎬ц緝浣庣殑缂╁噺杞暟绠楁硶锛屽苟灏嗗叾杈撳嚭鎴柇涓?32 浣嶃€?

## 鐢熸垚 hsiphash 瀵嗛挜


瀵嗛挜搴斿綋濮嬬粓鐢卞姞瀵嗗畨鍏ㄧ殑鏉ユ簮鐢熸垚
```

	hsiphash_key_t key;
	get_random_bytes(&key, sizeof(key));

```
濡傛灉浣犱笉鏄粠杩欓噷娲剧敓瀵嗛挜锛岄偅灏卞仛閿欎簡銆?

## 浣跨敤 hsiphash 鍑芥暟


璇ュ嚱鏁扮殑鍙樹綋鏈変袱绉嶏細涓€绉嶆帴鍙楁暣鏁板垪琛紝鍙︿竴绉?
```

	u32 hsiphash(const void *data, size_t len, const hsiphash_key_t *key);

```
```

	u32 hsiphash_1u32(u32, const hsiphash_key_t *key);
	u32 hsiphash_2u32(u32, u32, const hsiphash_key_t *key);
	u32 hsiphash_3u32(u32, u32, u32, const hsiphash_key_t *key);
	u32 hsiphash_4u32(u32, u32, u32, u32, const hsiphash_key_t *key);

```
濡傛灉浣犲悜閫氱敤 hsiphash 鍑芥暟浼犲叆闀垮害鎭掑畾鐨勫唴瀹癸紝缂栬瘧鍣ㄤ細鍦ㄧ紪璇戞湡杩涜甯搁噺鎶樺彔锛?
骞惰嚜鍔ㄩ€夋嫨鍏朵腑涓€涓粡杩囦紭鍖栫殑鍑芥暟銆?

## 鍝堝笇琛ㄩ敭鍑芥暟鐢ㄦ硶



```

	struct some_hashtable {
		DECLARE_HASHTABLE(hashtable, 8);
		hsiphash_key_t key;
	};

	void init_hashtable(struct some_hashtable *table)
	{
		get_random_bytes(&table->key, sizeof(table->key));
	}

	static inline hlist_head *some_hashtable_bucket(struct some_hashtable *table, struct interesting_input *input)
	{
		return &table->hashtable[hsiphash(input, sizeof(*input), &table->key) & (HASH_SIZE(table->hashtable) - 1)];
	}

```
鐒跺悗浣犲彲浠ュ儚寰€甯镐竴鏍烽亶鍘嗚繑鍥炵殑鍝堝笇妗躲€?

## 鎬ц兘


hsiphash() 澶х害姣?jhash() 鎱?3 鍊嶃€傚浜庤澶氭浛浠ｅ満鏅€岃█锛岃繖涓嶄細鎴愪负闂锛屽洜涓?
鍝堝笇琛ㄦ煡鎵惧苟闈炵摱棰堛€傛€讳綋鑰岃█锛屼负浜?hsiphash() 鐨勫畨鍏ㄦ€у拰鎶?DoS 鑳藉姏鑰屽仛鍑鸿繖绉?
鐗虹壊鎴栬鏄€煎緱鐨勩€?

