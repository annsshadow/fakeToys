## eCryptfs 鏂囦欢绯荤粺鐨勫姞瀵嗗瘑閽?


ECryptfs 鏄竴涓爢鍙犲紡鏂囦欢绯荤粺锛屽畠浣跨敤闅忔満鐢熸垚鐨勬枃浠跺姞瀵嗗瘑閽ワ紙FEK锛夊姣忎釜鏂囦欢杩涜閫忔槑鍔犲瘑涓庤В瀵嗐€?

姣忎釜 FEK 鍙堢敱涓€涓枃浠跺姞瀵嗗瘑閽ュ姞瀵嗗瘑閽ワ紙FEKEK锛夊姞瀵嗭紝鍙互鍦ㄥ唴鏍哥┖闂磋繘琛岋紝涔熷彲浠ョ敱鍚嶄负 'ecryptfsd' 鐨勭敤鎴风┖闂村畧鎶よ繘绋嬭繘琛屻€傚墠鑰呯殑鎯呭喌涓嬶紝鎿嶄綔鐢卞唴鏍?CryptoAPI 鐩存帴浣跨敤涓€涓敱鐢ㄦ埛杈撳叆鍙ｄ护娲剧敓鍑虹殑瀵嗛挜锛團EKEK锛夋墽琛岋紱鍚庤€呯殑鎯呭喌涓嬶紝FEK 鐢?'ecryptfsd' 鍊熷姪澶栭儴搴撹繘琛屽姞瀵嗭紝浠ユ敮鎸佸叕閽ュ瘑鐮佸銆丳KCS#11 浠ュ強鍩轰簬 TPM 鐨勫叾浠栨満鍒躲€?

eCryptfs 瀹氫箟鐨勬暟鎹粨鏋勶紝鐢ㄤ簬鍖呭惈 FEK 瑙ｅ瘑鎵€闇€鐨勪俊鎭紝绉颁负璁よ瘉浠ょ墝锛坅uthentication token锛夛紝鐩墠鍙互瀛樺偍鍦?'user' 绫诲瀷鐨勫唴鏍稿瘑閽ヤ腑锛岀敱闅?'ecryptfs-utils' 杞欢鍖呮彁渚涚殑鐢ㄦ埛绌洪棿宸ュ叿 'mount.ecryptfs' 鎻掑叆鍒扮敤鎴风壒瀹氫細璇濈殑瀵嗛挜鐜腑銆?

涓轰簡涓?eCryptfs 鏂囦欢绯荤粺閰嶅悎浣跨敤锛?encrypted' 瀵嗛挜绫诲瀷閫氳繃寮曞叆鏂扮殑 'ecryptfs' 鏍煎紡杩涜浜嗘墿灞曘€傛柊寮曞叆鏍煎紡鐨勫姞瀵嗗瘑閽ュ湪鍏惰浇鑽蜂腑瀛樺偍涓€涓璇佷护鐗岋紝鍏朵腑鐨?FEKEK 鐢卞唴鏍搁殢鏈虹敓鎴愶紝骞剁敱鐖朵富瀵嗛挜淇濇姢銆?

涓轰簡閬垮厤宸茬煡鏄庢枃鏀诲嚮锛岄€氳繃 'keyctl print' 鎴?'keyctl pipe' 鍛戒护鑾峰緱鐨?datablob 涓嶅寘鍚暣浣撹璇佷护鐗岋紙鍏跺唴瀹逛紬鎵€鍛ㄧ煡锛夛紝鑰屽彧鍖呭惈鍔犲瘑褰㈠紡鐨?FEKEK銆?

eCryptfs 鏂囦欢绯荤粺纭疄鍙互浠庝娇鐢ㄥ姞瀵嗗瘑閽ヤ腑鑾风泭锛屽洜涓烘墍闇€瀵嗛挜鍙互鐢辩鐞嗗憳瀹夊叏鍦扮敓鎴愶紝骞跺湪瑙ｅ皝涓€涓?'trusted' 瀵嗛挜涔嬪悗浜庡紩瀵兼椂鎻愪緵锛屼互渚垮湪鍙楁帶鐜涓墽琛屾寕杞姐€傚彟涓€涓紭鍔挎槸锛岃瀵嗛挜涓嶄細鏆撮湶浜庢伓鎰忚蒋浠剁殑濞佽儊涔嬩笅锛屽洜涓哄畠浠呭湪鍐呮牳灞備互鏄庢枃褰㈠紡鍙敤銆?

```

   keyctl add encrypted name "new ecryptfs key-type:master-key-name keylen" ring
   keyctl add encrypted name "load hex_blob" ring
   keyctl update keyid "update key-type:master-key-name"

```
```

	name:= '<16 涓崄鍏繘鍒跺瓧绗?'
	key-type:= 'trusted' | 'user'
	keylen:= 64


```

浣跨敤 eCryptfs 鏂囦欢绯荤粺鐨勫姞瀵嗗瘑閽ョず渚嬶細

鍒涘缓涓€涓暱搴︿负 64 瀛楄妭銆佹牸寮忎负 'ecryptfs' 鐨勫姞瀵嗗瘑閽?"1000100010001000"
```

    $ keyctl add encrypted 1000100010001000 "new ecryptfs user:test 64" @u
    19184530

    $ keyctl print 19184530
    ecryptfs user:test 64 490045d4bfe48c99f0d465fbbbb79e7500da954178e2de0697
    dd85091f5450a0511219e9f7cd70dcd498038181466f78ac8d4c19504fcc72402bfc41c2
    f253a41b7507ccaa4b2b03fff19a69d1cc0b16e71746473f023a95488b6edfd86f7fdd40
    9d292e4bacded1258880122dd553a661

    $ keyctl pipe 19184530 > ecryptfs.blob

```

浣跨敤鎵€鍒涘缓鐨勫姞瀵嗗瘑閽?"1000100010001000" 鎸傝浇 eCryptfs 鏂囦欢绯荤粺
```

    $ mount -i -t ecryptfs -oecryptfs_sig=1000100010001000,\
      ecryptfs_cipher=aes,ecryptfs_key_bytes=32 /secret /secret

```
