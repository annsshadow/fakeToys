
## 鍔犲瘑寮曟搸锛圕rypto Engine锛?

### 姒傝堪


鍔犲瘑寮曟搸锛圕E锛堿PI 鏄竴涓姞瀵嗚姹傞槦鍒楃鐞嗗櫒銆?
### 瑕佹眰


浣犲繀椤诲湪浣犵殑杞崲涓婁笅鏂?your_tfm_ctx 鐨勮捣濮嬪鏀剧疆缁撴瀯浣?crypto_engine锛?
```

	struct your_tfm_ctx {
		struct crypto_engine engine;
		...
	};

```
鍔犲瘑寮曟搸鍙互 crypto_async_request 鐨勫舰寮忕鐞嗗紓姝ヨ姹傘€傚畠鏃犳硶鐭ユ檽
搴曞眰璇锋眰绫诲瀷锛屽洜姝ゅ彧鑳借闂浆鎹㈢粨鏋勪綋銆傛棤娉曚娇鐢?container_of 璁块棶
涓婁笅鏂囥€傛澶栵紝寮曟搸瀵逛綘鐨勭粨鏋勪綋 "`struct your_tfm_ctx`" 涓€鏃犳墍鐭ャ€?寮曟搸鍋囧畾锛堣姹傦級灏嗗凡鐭ョ殑鎴愬憳 `struct crypto_engine` 鏀惧湪璧峰浣嶇疆銆?
### 鎿嶄綔椤哄簭


浣犻渶瑕侀€氳繃 `crypto_engine_alloc_init()` 鑾峰彇涓€涓?struct crypto_engine銆?閫氳繃 `crypto_engine_start()` 鍚姩瀹冦€傚畬鎴愬伐浣滃悗锛屼娇鐢?`crypto_engine_stop()`
鍏抽棴寮曟搸锛屽苟浣跨敤 `crypto_engine_exit()` 閿€姣佸紩鎿庛€?
鍦ㄤ紶杈撲换浣曡姹備箣鍓嶏紝浣犲繀椤婚€氳繃鎻愪緵浠ヤ笅鍑芥暟鏉ュ～鍏呬笂涓嬫枃 enginectx锛?
- `prepare_cipher_request`/`prepare_hash_request`锛氬湪姣忔瀵瑰簲鐨?  璇锋眰鎵ц鍓嶈璋冪敤銆傚鏋滈渶瑕佹煇浜涘鐞嗘垨鍏跺畠鍑嗗宸ヤ綔锛屽湪姝ゅ瀹屾垚銆?
- `unprepare_cipher_request`/`unprepare_hash_request`锛氬湪姣忔
  璇锋眰澶勭悊鍚庤璋冪敤銆傛竻鐞?/ 鎾ら攢鍦?prepare 鍑芥暟涓畬鎴愮殑宸ヤ綔銆?
- `cipher_one_request`/`hash_one_request`锛氶€氳繃鎵ц鎿嶄綔鏉ュ鐞嗗綋鍓嶈姹傘€?
娉ㄦ剰锛岃繖浜涘嚱鏁拌闂笌鏀跺埌鐨勮姹傜浉鍏宠仈鐨?crypto_async_request 缁撴瀯浣撱€?浣犲彲浠ラ€氳繃濡備笅鏂瑰紡鍙栧洖鍘熷璇锋眰锛?
```

	container_of(areq, struct yourrequesttype_request, base);

```
褰撲綘鐨勯┍鍔ㄦ敹鍒颁竴涓?crypto_request 鏃讹紝浣犲繀椤婚€氳繃浠ヤ笅涔嬩竴灏嗗叾
浼犺緭缁欏姞瀵嗗紩鎿庯細

- crypto_transfer_aead_request_to_engine()

- crypto_transfer_akcipher_request_to_engine()

- crypto_transfer_hash_request_to_engine()

- crypto_transfer_kpp_request_to_engine()

- crypto_transfer_skcipher_request_to_engine()

鍦ㄨ姹傚鐞嗙粨鏉熸椂锛岄渶瑕佽皟鐢ㄤ互涓嬪嚱鏁颁箣涓€锛?
- crypto_finalize_aead_request()

- crypto_finalize_akcipher_request()

- crypto_finalize_hash_request()

- crypto_finalize_kpp_request()

- crypto_finalize_skcipher_request()
