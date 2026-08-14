
## SHA-3 绠楁硶闆嗗悎锛圫HA-3 Algorithm Collection锛?


## 姒傝堪

SHA-3 绯诲垪绠楁硶鐢?NIST FIPS-202 [^1^]_ 瑙勮寖瀹氫箟锛屽寘鍚熀浜?Keccak 娴风坏锛坰ponge锛夊嚱鏁扮殑鍏绠楁硶銆傚畠浠箣闂寸殑宸紓鍦ㄤ簬锛?

- "rate"锛堥€熺巼锛屽嵆姣忔璋冪敤 Keccak 鍑芥暟鏃惰鏂版暟鎹洿鏂扮殑鐘舵€佺紦鍐插尯澶у皬锛岀被浼间簬"鍧楀ぇ灏?锛夛紱
- 杩藉姞鍒拌緭鍏ユ暟鎹箣鍚庣殑鍩熷垎绂诲悗缂€锛坉omain separation suffix锛夛紱
- 浠ュ強浠庢湯灏炬彁鍙栫殑杈撳嚭鏁版嵁閲忋€?

Keccak 娴风坏鍑芥暟琚璁′负鍙浠绘剰闀垮害鐨勮緭鍑鸿繘琛屾彁鍙栵紙杩欐鏄儴鍒嗙畻娉曟墍闇€瑕佺殑锛夈€?

鎻愪緵鍥涚鎽樿绠楁硶锛?

- SHA3-224
- SHA3-256
- SHA3-384
- SHA3-512

姝ゅ锛岃繕鎻愪緵涓ょ鍙墿灞曡緭鍑哄嚱鏁帮紙XOF锛夛細

- SHAKE128
- SHAKE256

SHA-3 搴?API 鏀寔涓婅堪鍏绠楁硶銆傚叾涓洓绉嶆憳瑕佺畻娉曟敮鎸?`crypto_shash` 涓?`crypto_ahash` 涓ょ被 API銆?

鏈枃妗ｆ弿杩?SHA-3 搴?API銆?


## Digests锛堟憳瑕侊級

```
	void sha3_224(const u8 *in, size_t in_len, u8 out[SHA3_224_DIGEST_SIZE]);
	void sha3_256(const u8 *in, size_t in_len, u8 out[SHA3_256_DIGEST_SIZE]);
	void sha3_384(const u8 *in, size_t in_len, u8 out[SHA3_384_DIGEST_SIZE]);
	void sha3_512(const u8 *in, size_t in_len, u8 out[SHA3_512_DIGEST_SIZE]);
```

濡傛灉鐢ㄦ埛闇€瑕佷互澧為噺锛坕ncremental锛夋柟寮忎紶鍏ユ暟鎹紝鍙娇鐢ㄥ閲?API锛?

```
	struct sha3_ctx { ... };
```

```
	void sha3_224_init(struct sha3_ctx *ctx);
	void sha3_256_init(struct sha3_ctx *ctx);
	void sha3_384_init(struct sha3_ctx *ctx);
	void sha3_512_init(struct sha3_ctx *ctx);
```

```
	void sha3_update(struct sha3_ctx *ctx, const u8 *in, size_t in_len);
```

```
	void sha3_final(struct sha3_ctx *ctx, u8 *out);
```

`sha3_final` 浼氭竻闆讹紙zeroize锛変笂涓嬫枃銆傛憳瑕侀暱搴︾敱鎵€璋冪敤鐨勫垵濮嬪寲鍑芥暟鍐冲畾銆?


## Extendable-Output 鍑芥暟锛堝彲鎵╁睍杈撳嚭鍑芥暟锛?

```
	void shake128(const u8 *in, size_t in_len, u8 *out, size_t out_len);
	void shake256(const u8 *in, size_t in_len, u8 *out, size_t out_len);
```

濡傛灉鐢ㄦ埛闇€瑕佷互澧為噺鏂瑰紡鎻愪緵杈撳叆鏁版嵁 / 鎺ユ敹杈撳嚭鏁版嵁锛屽彲浣跨敤澧為噺 API锛?

```
	struct shake_ctx { ... };
```

```
	void shake128_init(struct shake_ctx *ctx);
	void shake256_init(struct shake_ctx *ctx);
```

```
	void shake_update(struct shake_ctx *ctx, const u8 *in, size_t in_len);
```

```
	void shake_squeeze(struct shake_ctx *ctx, u8 *out, size_t out_len);
```

`shake_squeeze` 閫氳繃鍛婄煡瑕佹彁鍙栫殑鏁版嵁閲忔潵宸ヤ綔銆傛敞鎰忥細鎵ц澶氭 squeeze 鏃讹紝杈撳嚭浼氳繛缁湴鎺掑竷鍦ㄧ紦鍐插尯涓紝杩欎笌鍦ㄥ崟涓紦鍐插尯涓婃墽琛屼竴娆°€佹彁鍙栫浉鍚屾€婚噺鐨勫崟娆?squeeze 寰楀埌鐨勭粨鏋滃畬鍏ㄧ浉鍚屻€備竴鏃﹀紑濮?squeeze锛屽氨涓嶈兘鍐嶈拷鍔犳洿澶氳緭鍏ユ暟鎹€?

```
	void shake_zeroize_ctx(struct shake_ctx *ctx);
```


## Testing锛堟祴璇曪級

娴嬭瘯 SHA-3 浠ｇ爜锛屽彲浣跨敤 `sha3_kunit`锛堝搴旈厤缃」 `CONFIG_CRYPTO_LIB_SHA3_KUNIT_TEST`锛夈€?

鐢变簬 SHA-3 绠楁硶宸茶幏 FIPS 鎵瑰噯锛屽綋鍐呮牳浠?FIPS 妯″紡鍚姩鏃讹紝SHA-3 搴撲細鎵ц涓€娆＄畝鍗曠殑鑷娴嬶紙self-test锛夛紝杩欑函绮规槸涓轰簡婊¤冻 FIPS 鍚堣瑕佹眰銆傚父瑙勬祴璇曞垯鐢卞唴鏍稿紑鍙戣€呬笌闆嗘垚鑰呬娇鐢ㄦ洿涓哄叏闈㈢殑 KUnit 娴嬭瘯濂椾欢鏉ュ畬鎴愩€?


## References锛堝弬鑰冭祫鏂欙級


## API 鍑芥暟 鍙傝€?
