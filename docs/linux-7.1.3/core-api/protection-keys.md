
## 鍐呭瓨淇濇姢閿紙Memory Protection Keys锛?


鍐呭瓨淇濇姢閿彁渚涗簡涓€绉嶅己鍒跺熀浜庨〉鐨勪繚鎶ゆ満鍒讹紝浣嗕笉闇€瑕佸湪搴旂敤绋嬪簭鏇存敼淇濇姢鍩熸椂淇敼椤佃〃銆?

Pkeys Userspace锛圥KU锛夋槸涓€椤瑰彲浠ュ湪浠ヤ笅骞冲彴鎵惧埌鐨勭壒鎬э細
        - Intel 鏈嶅姟鍣?CPU锛孲kylake 鍙婃洿鏅?
        - Intel 瀹㈡埛绔?CPU锛孴iger Lake锛堢 11 浠ｉ叿鐫匡級鍙婃洿鏅?
        - 鏈潵鐨?AMD CPU
        - 瀹炵幇 Permission Overlay Extension锛團EAT_S1POE锛夌殑 arm64 CPU

## x86_64


淇濇姢閿殑宸ヤ綔鍘熺悊鏄皢姣忎釜椤佃〃椤逛腑鍏堝墠淇濈暀鐨?4 涓瘮鐗逛笓鐢ㄤ簬涓€涓€滀繚鎶ら敭锛坧rotection key锛夆€濓紝浠庤€屽緱鍒?16 涓彲鑳界殑閿€?

姣忎釜閿殑淇濇姢鐢辨瘡 CPU 鐢ㄦ埛鍙闂瘎瀛樺櫒锛圥KRU锛夊畾涔夈€傛瘡涓?PKRU 鏄竴涓?32 浣嶅瘎瀛樺櫒锛屼负 16 涓敭鍚勫瓨鍌ㄤ袱浣嶏紙璁块棶绂佹 Access Disable 鍜屽啓鍏ョ姝?Write Disable锛夈€?

浣滀负 CPU 瀵勫瓨鍣紝PKRU 澶╃敓鏄嚎绋嬪眬閮ㄧ殑锛屽彲鑳戒娇姣忎釜绾跨▼鎷ユ湁涓庡叾浠栫嚎绋嬩笉鍚岀殑淇濇姢闆嗗悎銆?

鏈変袱鏉℃寚浠わ紙RDPKRU/WRPKRU锛夌敤浜庤鍐欒瀵勫瓨鍣ㄣ€傚嵆浣?PAE PTE 涓悊璁轰笂瀛樺湪绌洪棿锛岃鐗规€т篃浠呭湪 64 浣嶆ā寮忎笅鍙敤銆傝繖浜涙潈闄愪粎瀵规暟鎹闂己鍒舵墽琛岋紝瀵规寚浠よ鍙栨病鏈夊奖鍝嶃€?

## arm64


淇濇姢閿湪姣忎釜椤佃〃椤逛腑浣跨敤 3 涓瘮鐗规潵缂栫爜涓€涓€滀繚鎶ら敭绱㈠紩锛坧rotection key index锛夆€濓紝浠庤€屽緱鍒?8 涓彲鑳界殑閿€?

姣忎釜閿殑淇濇姢鐢辨瘡 CPU 鐢ㄦ埛鍙啓绯荤粺瀵勫瓨鍣紙POR_EL0锛夊畾涔夈€傝繖鏄竴涓?64 浣嶅瘎瀛樺櫒锛屼负姣忎釜淇濇姢閿储寮曠紪鐮佽銆佸啓鍜屾墽琛岃鐩栨潈闄愩€?

浣滀负 CPU 瀵勫瓨鍣紝POR_EL0 澶╃敓鏄嚎绋嬪眬閮ㄧ殑锛屽彲鑳戒娇姣忎釜绾跨▼鎷ユ湁涓庡叾浠栫嚎绋嬩笉鍚岀殑淇濇姢闆嗗悎銆?

涓?x86_64 涓嶅悓锛屼繚鎶ら敭鏉冮檺涔熼€傜敤浜庢寚浠よ鍙栥€?

## 绯荤粺璋冪敤


```

	int pkey_alloc(unsigned long flags, unsigned long init_access_rights)
	int pkey_free(int pkey);
	int pkey_mprotect(unsigned long start, size_t len,
			  unsigned long prot, int pkey);

```
鍦ㄤ娇鐢?pkey 涔嬪墠锛屽繀椤诲厛鐢?pkey_alloc() 鍒嗛厤瀹冦€傚簲鐢ㄧ▼搴忕洿鎺ュ啓鍏ユ灦鏋勭浉鍏崇殑 CPU 瀵勫瓨鍣紝浠ユ洿鏀圭敱璇ラ敭瑕嗙洊鐨勫唴瀛樿闂潈闄愩€傚湪鏈緥涓紝杩欎竴鎿嶄綔琚竴涓悕涓?pkey_set() 鐨?C 鍑芥暟灏佽銆?
```

	int real_prot = PROT_READ|PROT_WRITE;
	pkey = pkey_alloc(0, PKEY_DISABLE_WRITE);
	ptr = mmap(NULL, PAGE_SIZE, PROT_NONE, MAP_ANONYMOUS|MAP_PRIVATE, -1, 0);
	ret = pkey_mprotect(ptr, PAGE_SIZE, real_prot, pkey);
	... 搴旂敤绋嬪簭鍦ㄦ杩愯

```
鐜板湪锛屽鏋滃簲鐢ㄧ▼搴忛渶瑕佹洿鏂?'ptr' 澶勭殑鏁版嵁锛屽畠鍙互
```

	pkey_set(pkey, 0); // 娓呴櫎 PKEY_DISABLE_WRITE
	*ptr = foo; // 璧嬪€?
	pkey_set(pkey, PKEY_DISABLE_WRITE); // 閲嶆柊璁剧疆 PKEY_DISABLE_WRITE

```
褰撻噴鏀惧唴瀛樻椂锛岀敱浜?
```

	munmap(ptr, PAGE_SIZE);
	pkey_free(pkey);

```
瀹冧篃浼氶噴鏀捐 pkey銆傜ず渚嬪疄鐜板彲鍦?tools/testing/selftests/mm/pkey-{arm64,powerpc,x86}.h 涓壘鍒般€?

## 琛屼负


鍐呮牳璇曞浘浣夸繚鎶ら敭涓?
```

	mprotect(ptr, size, PROT_NONE);
	something(ptr);

```
淇濇寔涓€鑷淬€傛棤璁?something() 鏄 'ptr' 鐨勭洿鎺ヨ闂?
```

	*ptr = foo;

```
杩樻槸鍐呮牳浠ｈ〃搴旂敤绋嬪簭杩涜璁块棶
```

	read(fd, ptr, 1);

```
鍦ㄨ繖涓ょ鎯呭喌涓嬪唴鏍搁兘浼氬彂閫?SIGSEGV锛屼絾褰撹繚鍙嶄繚鎶ら敭鏃?si_code 浼氳璁句负 SEGV_PKERR锛岃€屽綋杩濆弽鏅€?mprotect() 鏉冮檺鏃跺垯涓?SEGV_ACCERR銆?

娉ㄦ剰锛屾潵鑷?kthread锛堝 io_uring锛夌殑鍐呮牳璁块棶灏嗕娇鐢ㄤ繚鎶ら敭瀵勫瓨鍣ㄧ殑榛樿鍊硷紝鍥犳涓庣敤鎴风┖闂寸殑瀵勫瓨鍣ㄥ€兼垨 mprotect() 涓嶄竴鑷淬€?
