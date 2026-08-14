
## Kerberos V 瀵嗙爜瀛?API


  - 姒傝堪锛圤verview锛夈€?    - 灏忓瀷缂撳啿鍖猴紙Small Buffer锛夈€?  - 缂栫爜绫诲瀷锛圗ncoding Type锛夈€?  - 瀵嗛挜娲剧敓锛圞ey Derivation锛夈€?    - PRF+ 璁＄畻銆?    - Kc銆並e 鍜?Ki 娲剧敓銆?  - 瀵嗙爜鍑芥暟锛圕rypto Functions锛夈€?    - 鍑嗗鍑芥暟锛圥reparation Functions锛夈€?    - 鍔犲瘑妯″紡锛圗ncryption Mode锛夈€?    - 鏍￠獙鍜屾ā寮忥紙Checksum Mode锛夈€?  - krb5enc AEAD 绠楁硶

## 姒傝堪


姝?API 鎻愪緵 Kerberos 5 椋庢牸鐨勫瘑鐮佸锛岀敤浜庡瘑閽ユ淳鐢熴€佸姞瀵嗗拰鏍￠獙鍜岋紝鍙緵缃戠粶鏂囦欢绯荤粺浣跨敤锛屽苟鍙敤浜庡疄鐜?GSSAPI 鎵€闇€鐨勫簳灞傚姞瀵嗐€?
```

	KRB5_ENCTYPE_AES128_CTS_HMAC_SHA1_96
	KRB5_ENCTYPE_AES256_CTS_HMAC_SHA1_96
	KRB5_ENCTYPE_AES128_CTS_HMAC_SHA256_128
	KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192
	KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC
	KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC

	KRB5_CKSUMTYPE_HMAC_SHA1_96_AES128
	KRB5_CKSUMTYPE_HMAC_SHA1_96_AES256
	KRB5_CKSUMTYPE_CMAC_CAMELLIA128
	KRB5_CKSUMTYPE_CMAC_CAMELLIA256
	KRB5_CKSUMTYPE_HMAC_SHA256_128_AES128
	KRB5_CKSUMTYPE_HMAC_SHA384_192_AES256

```
```

	#include <crypto/krb5.h>

```
### 灏忓瀷缂撳啿鍖猴紙Small Buffer锛?

涓轰簡浼犻€掕濡傚瘑閽ヤ箣绫荤殑灏忓潡鏁版嵁锛屼娇鐢ㄥ涓嬬紦鍐插尯缁撴瀯
```

	struct krb5_buffer {
		unsigned int	len;
		void		*data;
	};

```
## 缂栫爜绫诲瀷锛圗ncoding Type锛?

```

	struct krb5_enctype {
		int		etype;
		int		ctype;
		const char	*name;
		u16		key_bytes;
		u16		key_len;
		u16		Kc_len;
		u16		Ke_len;
		u16		Ki_len;
		u16		prf_len;
		u16		block_len;
		u16		conf_len;
		u16		cksum_len;
		...
	};

```
API 浣跨敤鑰呮劅鍏磋叮鐨勫瓧娈靛涓嬶細

  - `etype` 鍜?`ctype` 鍒嗗埆鎸囩ず璇ョ紪鐮佺被鍨嬬敤浜庡姞瀵嗗拰鏍￠獙鍜岀殑鍗忚缂栧彿銆傚畠浠繚瀛?`KRB5_ENCTYPE_**` 鍜?`KRB5_CKSUMTYPE_**` 甯搁噺銆?
  - `name` 鏄缂栫爜鐨勬寮忓悕绉般€?
  - `key_len` 鍜?`key_bytes` 鏄緭鍏ュ瘑閽ラ暱搴﹀拰娲剧敓瀵嗛挜闀垮害銆傦紙鎴戣涓哄畠浠彧鍦?DES 鏃朵笉鍚岋紝鑰岃繖閲屼笉鏀寔 DES锛夈€?
  - `Kc_len`銆乣Ke_len` 鍜?`Ki_len` 鏄淳鐢熺殑 Kc銆並e 鍜?Ki 瀵嗛挜鐨勫ぇ灏忋€侹c 鐢ㄤ簬鏍￠獙鍜屾ā寮忥紱Ke 鍜?Ki 鐢ㄤ簬鍔犲瘑妯″紡銆?
  - `prf_len` 鏄?PRF+ 鍑芥暟璁＄畻鐨勭粨鏋滃ぇ灏忋€?
  - `block_len`銆乣conf_len` 鍜?`cksum_len` 鍒嗗埆鏄姞瀵嗗潡闀垮害銆佹贩娣嗘暟锛坈onfounder锛夐暱搴﹀拰鏍￠獙鍜岄暱搴︺€備笁鑰呴兘鐢ㄤ簬鍔犲瘑妯″紡锛屼絾鍙湁鏍￠獙鍜岄暱搴︾敤浜庢牎楠屽拰妯″紡銆?
```

	const struct krb5_enctype *crypto_krb5_find_enctype(u32 enctype);

```
## 瀵嗛挜娲剧敓锛圞ey Derivation锛?

涓€鏃﹀簲鐢ㄧ▼搴忛€夊畾浜嗗姞瀵嗙被鍨嬶紝灏卞彲浠ヤ粠浼犺緭瀵嗛挜锛坱ransport key锛夋淳鐢熷嚭鐢ㄤ簬瀹為檯鍔犲瘑鐨勫瘑閽ャ€?
### PRF+ 璁＄畻


涓轰簡杈呭姪瀵嗛挜娲剧敓锛屾彁渚涗竴涓嚱鏁版潵璁＄畻 Kerberos GSSAPI 鐨?PRF+
```

	int crypto_krb5_calc_PRFplus(const struct krb5_enctype *krb5,
				     const struct krb5_buffer *K,
				     unsigned int L,
				     const struct krb5_buffer *S,
				     struct krb5_buffer *result,
				     gfp_t gfp);

```
杩欏彲鐢ㄤ簬浠庢簮瀵嗛挜鍔犱笂棰濆鐨勬暟鎹淳鐢熶紶杈撳瘑閽ワ紝浠ラ檺鍒跺叾鐢ㄩ€斻€?
## 瀵嗙爜鍑芥暟锛圕rypto Functions锛?

瀵嗛挜娲剧敓瀹屾垚鍚庯紝灏卞彲浠ュ鏁版嵁鎵ц鍔犲瘑鎿嶄綔銆傝皟鐢ㄦ柟鍦ㄤ负浼犺緭鍑嗗娑堟伅鏃讹紝蹇呴』鍦ㄧ紦鍐插尯涓暀鍑虹敤浜庡瓨鏀炬贩娣嗘暟锛堝闇€瑕侊級鍜屾牎楠屽拰鐨勭┖闅欍€備竴涓灇涓?```

	enum krb5_crypto_mode {
		KRB5_CHECKSUM_MODE,
		KRB5_ENCRYPT_MODE,
	};

	size_t crypto_krb5_how_much_buffer(const struct krb5_enctype *krb5,
					   enum krb5_crypto_mode mode,
					   size_t data_size, size_t *_offset);

	size_t crypto_krb5_how_much_data(const struct krb5_enctype *krb5,
					 enum krb5_crypto_mode mode,
					 size_t *_buffer_size, size_t *_offset);

```
鎵€鏈夎繖浜涘嚱鏁伴兘鎺ュ彈缂栫爜绫诲瀷浠ュ強鍔犲瘑妯″紡鐨勬寚绀猴紙浠呮牎楠屽拰鎴栧畬鏁村姞瀵嗭級銆?
绗竴涓嚱鏁拌繑鍥炲绾崇粰瀹氭暟鎹噺鎵€闇€鐨勭紦鍐插尯澶у皬锛涚浜屼釜鍑芥暟杩斿洖鐗瑰畾澶у皬鐨勭紦鍐插尯鑳藉绾冲灏戞暟鎹紝骞剁浉搴斿湴涓嬭皟鎵€闇€缂撳啿鍖虹殑澶у皬銆傚湪杩欎袱绉嶆儏鍐典笅锛岃繕浼氳繑鍥炴暟鎹湪缂撳啿鍖轰腑鐨勫亸绉汇€?
褰撴敹鍒颁竴鏉℃秷鎭椂锛屾暟鎹殑浣嶇疆鍜屽ぇ灏忕敱
```

	int crypto_krb5_where_is_the_data(const struct krb5_enctype *krb5,
					  enum krb5_crypto_mode mode,
					  size_t *_offset, size_t *_len);

```
璋冪敤鏂瑰悜鍑芥暟鎻愪緵娑堟伅鐨勫亸绉诲拰闀垮害锛屽嚱鏁伴殢鍚庝慨鏀硅繖浜涘€间互鎸囩ず鍖呭惈鏁版嵁鐨勫尯鍩燂紙鍔犱笂浠讳綍濉厖锛夈€傛湁澶氬皯濉厖鐢辫皟鐢ㄦ柟鍐冲畾銆傚鏋滈暱搴﹀お灏忥紝鎴栬€呮ā寮忎负
```

	int crypto_krb5_check_data_len(const struct krb5_enctype *krb5,
				       enum krb5_crypto_mode mode,
				       size_t len, size_t min_content);

```
鍒欐彁渚涗竴涓嚱鏁版潵浠呭仛鍩烘湰妫€鏌ワ紝纭瑙ｅ瘑/楠岃瘉鍚庣殑娑堟伅鍏锋湁瓒冲鐨勬渶灏忔湁鏁堣浇鑽枫€?
### 鍑嗗鍑芥暟锛圥reparation Functions锛?

鎻愪緵涓や釜鍑芥暟鏉ュ垎閰嶅苟鍑嗗涓€涓緵浣跨敤鐨勫姞瀵嗗璞?```

	struct crypto_aead *
	crypto_krb5_prepare_encryption(const struct krb5_enctype *krb5,
				       const struct krb5_buffer *TK,
				       u32 usage, gfp_t gfp);
	struct crypto_shash *
	crypto_krb5_prepare_checksum(const struct krb5_enctype *krb5,
				     const struct krb5_buffer *TK,
				     u32 usage, gfp_t gfp);

```
杩欎袱涓嚱鏁伴兘鎺ュ彈缂栫爜绫诲瀷銆佷紶杈撳瘑閽ヤ互鍙婄敤浜庢淳鐢熺浉搴斿瓙瀵嗛挜鐨?usage 鍊笺€傚畠浠垱寤轰竴涓悎閫傜殑鍔犲瘑瀵硅薄鈥斺€旂敤浜庡姞瀵嗙殑 AEAD 妯℃澘鍜岀敤浜庢牎楠屽拰鐨勫悓姝ュ搱甯屸€斺€斿湪鍏朵笂璁剧疆瀵嗛挜骞惰繘琛岄厤缃€傝皟鐢ㄦ柟搴斿皢杩欎簺鍙ユ焺浼犻€掔粰涓嬮潰鐨勫姩浣滃嚱鏁般€?
### 鍔犲瘑妯″紡锛圗ncryption Mode锛?

```

	ssize_t crypto_krb5_encrypt(const struct krb5_enctype *krb5,
				    struct crypto_aead *aead,
				    struct scatterlist *sg, unsigned int nr_sg,
				    size_t sg_len,
				    size_t data_offset, size_t data_len,
				    bool preconfounded);
	int crypto_krb5_decrypt(const struct krb5_enctype *krb5,
				struct crypto_aead *aead,
				struct scatterlist *sg, unsigned int nr_sg,
				size_t *_offset, size_t *_len);

```
鍦ㄨ繖涓ょ鎯呭喌涓嬶紝杈撳叆鍜岃緭鍑虹紦鍐插尯鐢卞悓涓€涓?scatterlist 鎸囩ず銆?
瀵逛簬鍔犲瘑鍑芥暟锛岃緭鍑虹紦鍐插尯鍙兘姣旀墍闇€鏇村ぇ锛堣繑鍥炵敓鎴愮殑杈撳嚭閲忥級锛屽苟鎸囧嚭鏁版嵁鐨勪綅缃拰澶у皬锛堝繀椤讳笌缂栫爜鍖归厤锛夈€傚鏋滄湭璁剧疆娣锋穯鏁帮紝鍑芥暟浼氭彃鍏ヤ竴涓€?
瀵逛簬瑙ｅ瘑鍑芥暟锛屾彁渚涚紦鍐插尯涓秷鎭殑鍋忕Щ鍜岄暱搴︼紝杩欎簺鍊间細琚敹缂╀互閫傚簲鏁版嵁銆傝В瀵嗗嚱鏁颁細楠岃瘉娑堟伅鍐呯殑浠讳綍鏍￠獙鍜岋紝濡傛灉涓嶅尮閰嶅垯鎶ラ敊銆?
### 鏍￠獙鍜屾ā寮忥紙Checksum Mode锛?

鎻愪緵涓€瀵瑰嚱鏁版潵鐢熸垚娑堟伅鐨勬牎楠屽拰骞?```

	ssize_t crypto_krb5_get_mic(const struct krb5_enctype *krb5,
				    struct crypto_shash *shash,
				    const struct krb5_buffer *metadata,
				    struct scatterlist *sg, unsigned int nr_sg,
				    size_t sg_len,
				    size_t data_offset, size_t data_len);
	int crypto_krb5_verify_mic(const struct krb5_enctype *krb5,
				   struct crypto_shash *shash,
				   const struct krb5_buffer *metadata,
				   struct scatterlist *sg, unsigned int nr_sg,
				   size_t *_offset, size_t *_len);

```
鍦ㄨ繖涓ょ鎯呭喌涓嬶紝杈撳叆鍜岃緭鍑虹紦鍐插尯鐢卞悓涓€涓?scatterlist 鎸囩ず銆傚彲浠ヤ紶鍏ラ澶栫殑鍏冩暟鎹紝瀹冧細鍦ㄦ暟鎹箣鍓嶈鍔犲叆鍝堝笇銆?
瀵逛簬 get_mic 鍑芥暟锛岃緭鍑虹紦鍐插尯鍙兘姣旀墍闇€鏇村ぇ锛堣繑鍥炵敓鎴愮殑杈撳嚭閲忥級锛屽苟鎸囧嚭鏁版嵁鐨勪綅缃拰澶у皬锛堝繀椤讳笌缂栫爜鍖归厤锛夈€?
瀵逛簬楠岃瘉鍑芥暟锛屾彁渚涚紦鍐插尯涓秷鎭殑鍋忕Щ鍜岄暱搴︼紝杩欎簺鍊间細琚敹缂╀互閫傚簲鏁版嵁銆傚鏋滄牎楠屽拰涓嶅尮閰嶏紝灏嗚繑鍥為敊璇€?
## krb5enc AEAD 绠楁硶


鎻愪緵浜嗕竴涓悕涓?鈥渒rb5enc鈥?鐨勬ā鏉?AEAD 鍔犲瘑绠楁硶锛屽畠鍦ㄥ姞瀵嗘槑鏂囦箣鍓嶅厛瀵规槑鏂囧仛鍝堝笇锛堜笌 authenc 鐩稿弽锛夈€俙crypto_krb5_prepare_encryption()` 杩斿洖鐨勫彞鏌勫彲鑳芥槸鍏朵腑涔嬩竴锛屼絾姝?API 鐨勪娇鐢ㄨ€呮棤闇€鐩存帴涓庝箣浜や簰銆?
浣滀负鍙傝€冿紝鍏跺瘑閽ユ牸寮忎互鏍煎紡鍙风殑 BE32 寮€澶淬€傚彧鎻愪緵鏍煎紡 1锛屽叾鍚庤窡涓€涓?Ke 瀵嗛挜闀垮害鐨?BE32锛屽啀璺熶竴涓?Ki 瀵嗛挜闀垮害鐨?BE32锛岀劧鍚庢槸 Ke 瀵嗛挜鐨勫瓧鑺傦紝鍐嶆槸 Ki 瀵嗛挜鐨勫瓧鑺傘€?
浣跨敤鐗瑰畾椤哄簭鐨勫瓧鎰忓懗鐫€闈欐€佹祴璇曟暟鎹笉闇€瑕佸瓧鑺備氦鎹紙byteswapping锛夈€?