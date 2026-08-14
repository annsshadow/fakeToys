## 鏁板瓧绛惧悕楠岃瘉 API


:Author: Dmitry Kasatkin
:Date: 06.10.2011


   1. 绠€浠?   2. API
   3. 鐢ㄦ埛绌洪棿宸ュ叿


## 绠€浠?

鏁板瓧绛惧悕楠岃瘉 API 鎻愪緵浜嗕竴绉嶉獙璇佹暟瀛楃鍚嶇殑鏂规硶銆傜洰鍓嶆暟瀛楃鍚嶇敱 IMA/EVM
瀹屾暣鎬т繚鎶ゅ瓙绯荤粺浣跨敤銆?
鏁板瓧绛惧悕楠岃瘉閫氳繃浣跨敤 GnuPG 澶氱簿搴︽暣鏁帮紙MPI锛夊簱鐨勭簿绠€鍐呮牳绉绘鐗堝疄鐜般€傝鍐呮牳
绉绘鐗堟彁渚涗簡鍐呭瓨鍒嗛厤閿欒澶勭悊锛屽凡鎸夌収鍐呮牳缂栫爜椋庢牸杩涜浜嗛噸鏋勶紝骞朵慨澶嶄簡
checkpatch.pl 鎶ュ憡鐨勯敊璇笌璀﹀憡銆?
```

	struct pubkey_hdr {
		uint8_t		version;	/* key format version */
		time_t		timestamp;	/* key made, always 0 for now */
		uint8_t		algo;
		uint8_t		nmpi;
		char		mpi[0];
	} __packed;

	struct signature_hdr {
		uint8_t		version;	/* signature format version */
		time_t		timestamp;	/* signature made */
		uint8_t		algo;
		uint8_t		hash;
		uint8_t		keyid[8];
		uint8_t		nmpi;
		char		mpi[0];
	} __packed;

```
keyid 绛変簬瀵瑰瘑閽ユ暣浣撳唴瀹硅绠?SHA1[12-19] 鐨勭粨鏋溿€?绛惧悕澶磋鐢ㄤ綔鐢熸垚绛惧悕鐨勮緭鍏ャ€?杩欑鏂瑰紡纭繚浜嗗瘑閽ユ垨绛惧悕澶存棤娉曡鏇存敼銆?瀹冧繚鎶ゆ椂闂存埑涓嶈淇敼锛屽彲鐢ㄤ簬鍥炴粴淇濇姢銆?
## API


```

	digsig_verify() - 浣跨敤鍏挜杩涜鏁板瓧绛惧悕楠岃瘉


	/**
	* digsig_verify() - 浣跨敤鍏挜杩涜鏁板瓧绛惧悕楠岃瘉
	* @keyring:	鍦ㄥ叾涓悳绱㈠瘑閽ョ殑 keyring
	* @sig:	鏁板瓧绛惧悕
	* @sigen:	绛惧悕闀垮害
	* @data:	鏁版嵁
	* @datalen:	鏁版嵁闀垮害
	* @return:	鎴愬姛杩斿洖 0锛屽惁鍒欒繑鍥?-EINVAL
	*
	* 閽堝鏁板瓧绛惧悕楠岃瘉鏁版嵁瀹屾暣鎬с€?	* 鐩墠浠呮敮鎸?RSA銆?	* 閫氬父灏嗗唴瀹圭殑鍝堝笇浣滀负璇ュ嚱鏁扮殑鏁版嵁浣跨敤銆?	*
	*/
	int digsig_verify(struct key *keyring, const char *sig, int siglen,
			  const char *data, int datalen);

```
## 鐢ㄦ埛绌洪棿宸ュ叿


鐢ㄤ簬绛惧悕涓庡瘑閽ョ鐞嗙殑宸ュ叿 evm-utils 鎻愪緵浜嗙敓鎴愮鍚嶃€佸皢瀵嗛挜鍔犺浇鍒板唴鏍?keyring
鐨勫姛鑳姐€傚瘑閽ュ彲浠ユ槸 PEM 鏍煎紡锛屼篃鍙互杞崲涓哄唴鏍告牸寮忋€傚綋瀵嗛挜琚姞鍏ュ唴鏍?keyring
鏃讹紝keyid 瀹氫箟浜嗗瘑閽ョ殑鍚嶇О锛氬涓嬩緥涓殑 5D2B05FC633EE3E8銆?
```

	$ keyctl show
	Session Keyring
	-3 --alswrv      0     0  keyring: _ses
	603976250 --alswrv      0    -1   \_ keyring: _uid.0
	817777377 --alswrv      0     0       \_ user: kmk
	891974900 --alswrv      0     0       \_ encrypted: evm-key
	170323636 --alswrv      0     0       \_ keyring: _module
	548221616 --alswrv      0     0       \_ keyring: _ima
	128198054 --alswrv      0     0       \_ keyring: _evm

	$ keyctl list 128198054
	1 key in keyring:
	620789745: --alswrv     0     0 user: 5D2B05FC633EE3E8

```
