## 褰卞瓙鍙橀噺锛圫hadow Variables锛?

褰卞瓙鍙橀噺锛圫hadow Variables锛夋槸涓€绉嶈 livepatch 妯″潡灏嗛澶栫殑鈥滃奖瀛愨€濇暟鎹笌宸叉湁鏁版嵁缁撴瀯鍏宠仈璧锋潵鐨勭畝鍗曟柟寮忋€傚奖瀛愭暟鎹嫭绔嬩簬鐖舵暟鎹粨鏋勫垎閰嶏紝鐖舵暟鎹粨鏋勪繚鎸佷笉鍙樸€傛湰鏂囨。鎻忚堪鐨勫奖瀛愬彉閲?API 鐢ㄤ簬灏嗗奖瀛愬彉閲忓垎閰?娣诲姞浠ュ強绉婚櫎/閲婃斁鍒板叾鐖跺璞°€?
璇ュ疄鐜板紩鍏ヤ簡涓€涓叏灞€鐨勫唴鏍告€佸搱甯岃〃锛屽皢鎸囧悜鐖跺璞＄殑鎸囬拡涓庡奖瀛愭暟鎹殑鏁板瓧鏍囪瘑绗﹀叧鑱旇捣鏉ャ€傝鏁板瓧鏍囪瘑绗︽槸涓€涓畝鍗曠殑鏋氫妇锛屽彲鐢ㄤ簬鎻忚堪褰卞瓙鍙橀噺鐨勭増鏈€佺被鍒垨绫诲瀷绛夈€傛洿鍏蜂綋鍦拌锛岀埗鎸囬拡浣滀负鍝堝笇琛ㄧ殑閿紝鑰屾暟瀛?id 闅忓悗鐢ㄤ簬杩囨护鍝堝笇琛ㄦ煡璇€傚涓奖瀛愬彉閲忓彲浠ラ檮鍔犲埌鍚屼竴涓埗瀵硅薄锛屼絾瀹冧滑鐨勬暟瀛楁爣璇嗙灏嗗畠浠尯鍒嗗紑鏉ャ€?

## 1. 绠€鐭?API 姒傝


锛堝畬鏁寸殑 API 浣跨敤 docbook 璇存槑瑙?livepatch/shadow.c銆傦級

涓€涓搱甯岃〃寮曠敤浜嗘墍鏈夊奖瀛愬彉閲忋€傝繖浜涘紩鐢ㄩ€氳繃 <obj, id> 瀵规潵瀛樺偍鍜屾绱€?
- `klp_shadow` 鍙橀噺鏁版嵁缁撴瀯鍚屾椂灏佽浜嗚窡韪厓鏁版嵁鍜屽奖瀛愭暟鎹細

  - 鍏冩暟鎹?
    - obj - 鎸囧悜鐖跺璞＄殑鎸囬拡
    - id - 鏁版嵁鏍囪瘑绗?
  - data[] - 褰卞瓙鏁版嵁鐨勫瓨鍌ㄧ┖闂?
闇€瑕佹敞鎰忕殑鏄紝`klp_shadow_alloc()` 鍜?`klp_shadow_get_or_alloc()` 榛樿浼氬皢鍙橀噺娓呴浂銆傚綋闇€瑕佷竴涓潪闆跺€兼椂锛屽畠浠篃鍏佽璋冪敤涓€涓嚜瀹氫箟鐨勬瀯閫犲嚱鏁般€傝皟鐢ㄨ€呭簲褰撴彁渚涙墍闇€鐨勪换浣曚簰鏂ヤ繚鎶ゃ€?
娉ㄦ剰锛屾瀯閫犲嚱鏁板湪 klp_shadow_lock 鑷棆閿佷笅璋冪敤銆傚畠鍏佽鎵ц閭ｄ簺鍦ㄥ垎閰嶆柊鍙橀噺鏃跺彧鑳藉仛涓€娆＄殑鎿嶄綔銆?
- klp_shadow_get() - 妫€绱竴涓奖瀛愬彉閲忔暟鎹寚閽?  - 鍦ㄥ搱甯岃〃涓悳绱?<obj, id> 瀵?
- klp_shadow_alloc() - 鍒嗛厤骞舵坊鍔犱竴涓柊鐨勫奖瀛愬彉閲?  - 鍦ㄥ搱甯岃〃涓悳绱?<obj, id> 瀵?
  - 濡傛灉瀛樺湪

    - 璀﹀憡骞惰繑鍥?NULL

  - 濡傛灉 <obj, id> 灏氫笉瀛樺湪

    - 鍒嗛厤涓€涓柊鐨勫奖瀛愬彉閲?    - 濡傛灉鎻愪緵浜嗚嚜瀹氫箟鏋勯€犲嚱鏁板拰鏁版嵁锛屽垯浣跨敤瀹冧滑鍒濆鍖栬鍙橀噺
    - 灏?<obj, id> 娣诲姞鍒板叏灞€鍝堝笇琛?
- klp_shadow_get_or_alloc() - 鑾峰彇宸叉湁鐨勬垨鍒嗛厤涓€涓柊鐨勫奖瀛愬彉閲?  - 鍦ㄥ搱甯岃〃涓悳绱?<obj, id> 瀵?
  - 濡傛灉瀛樺湪

    - 杩斿洖宸叉湁鐨勫奖瀛愬彉閲?
  - 濡傛灉 <obj, id> 灏氫笉瀛樺湪

    - 鍒嗛厤涓€涓柊鐨勫奖瀛愬彉閲?    - 濡傛灉鎻愪緵浜嗚嚜瀹氫箟鏋勯€犲嚱鏁板拰鏁版嵁锛屽垯浣跨敤瀹冧滑鍒濆鍖栬鍙橀噺
    - 灏?<obj, id> 瀵规坊鍔犲埌鍏ㄥ眬鍝堝笇琛?
- klp_shadow_free() - 鍒嗙骞堕噴鏀句竴涓?<obj, id> 褰卞瓙鍙橀噺
  - 浠庡叏灞€鍝堝笇琛ㄤ腑鏌ユ壘骞剁Щ闄や竴涓?<obj, id> 寮曠敤

    - 濡傛灉鎵惧埌

      - 濡傛灉瀹氫箟浜嗘瀽鏋勫嚱鏁板垯璋冪敤瀹?      - 閲婃斁褰卞瓙鍙橀噺

- klp_shadow_free_all() - 鍒嗙骞堕噴鏀炬墍鏈?<_, id> 褰卞瓙鍙橀噺
  - 浠庡叏灞€鍝堝笇琛ㄤ腑鏌ユ壘骞剁Щ闄や换鎰?<_, id> 寮曠敤

    - 濡傛灉鎵惧埌

      - 濡傛灉瀹氫箟浜嗘瀽鏋勫嚱鏁板垯璋冪敤瀹?      - 閲婃斁褰卞瓙鍙橀噺


## 2. 浣跨敤鍦烘櫙


锛堝畬鏁寸殑鍙繍琛屾紨绀鸿鍙傝 samples/livepatch/ 涓殑褰卞瓙鍙橀噺 livepatch 妯″潡绀轰緥銆傦級

瀵逛簬浠ヤ笅浣跨敤鍦烘櫙绀轰緥锛岃鑰冭檻鎻愪氦 1d147bfa6429锛堚€渕ac80211: fix AP powersave TX vs. wakeup race鈥濓級锛屽畠鍚?net/mac80211/sta_info.h 娣诲姞浜嗕竴涓?**spinlock锛堣嚜鏃嬮攣锛?*锛歴truct sta_info銆傛瘡涓娇鐢ㄥ満鏅ず渚嬮兘鍙互瑙嗕负璇ヤ慨澶嶇殑涓€涓嫭绔?livepatch 瀹炵幇銆?

### 鍖归厤鐖跺璞＄殑鐢熷懡鍛ㄦ湡


濡傛灉鐖舵暟鎹粨鏋勯绻佸湴琚垱寤哄拰閿€姣侊紝鏈€绠€鍗曠殑鏂规硶鍙兘鏄皢瀹冧滑鐨勫奖瀛愬彉閲忕敓鍛藉懆鏈熷榻愬埌鐩稿悓鐨勫垎閰嶅拰閲婃斁鍑芥暟銆傚湪杩欑鎯呭喌涓嬶紝鐖舵暟鎹粨鏋勯€氬父浼氳鍒嗛厤銆佸垵濮嬪寲锛岀劧鍚庝互鏌愮鏂瑰紡娉ㄥ唽銆傚奖瀛愬彉閲忕殑鍒嗛厤鍜岃缃彲瑙嗕负鐖跺璞″垵濮嬪寲鐨勪竴閮ㄥ垎锛屽苟涓斿簲鍦ㄧ埗瀵硅薄鈥滀笂绾库€濓紙鍗冲姝?<obj, id> 瀵瑰彂鍑轰换浣曞奖瀛愬彉閲?get-API 璇锋眰锛変箣鍓嶅畬鎴愩€?
瀵逛簬鎻愪氦 1d147bfa6429锛屽綋鍒嗛厤涓€涓埗 sta_info 缁撴瀯鏃讹紝
```

  #define PS_LOCK 1
  struct sta_info *sta_info_alloc(struct ieee80211_sub_if_data *sdata,
				  const u8 *addr, gfp_t gfp)
  {
	struct sta_info *sta;
	spinlock_t *ps_lock;

	/* Parent structure is created */
	sta = kzalloc(sizeof(*sta) + hw->sta_data_size, gfp);

	/* Attach a corresponding shadow variable, then initialize it */
	ps_lock = klp_shadow_alloc(sta, PS_LOCK, sizeof(*ps_lock), gfp,
				   NULL, NULL);
	if (!ps_lock)
		goto shadow_fail;
	spin_lock_init(ps_lock);
	...

```
褰撻渶瑕佷竴涓?ps_lock 鏃讹紝鏌ヨ褰卞瓙鍙橀噺 API 鏉ユ绱竴涓?```

  void ieee80211_sta_ps_deliver_wakeup(struct sta_info *sta)
  {
	spinlock_t *ps_lock;

	/* sync with ieee80211_tx_h_unicast_ps_buf */
	ps_lock = klp_shadow_get(sta, PS_LOCK);
	if (ps_lock)
		spin_lock(ps_lock);
	...

```
褰撶埗 sta_info 缁撴瀯琚噴鏀炬椂锛屽厛閲婃斁褰卞瓙鍙橀噺
```

  void sta_info_free(struct ieee80211_local *local, struct sta_info *sta)
  {
	klp_shadow_free(sta, PS_LOCK, NULL);
	kfree(sta);
	...


```

### 鍦ㄩ€旂埗瀵硅薄


鏈夋椂锛屼笌鍏剁埗瀵硅薄涓€璧峰垎閰嶅奖瀛愬彉閲忓彲鑳戒笉鏂逛究鎴栦笉鍙銆傛垨鑰咃紝涓€涓?livepatch 淇鍙兘鍙渶瑕佷负鐖跺璞″疄渚嬬殑涓€涓瓙闆嗚缃奖瀛愬彉閲忋€傚湪杩欎簺鎯呭喌涓嬶紝鍙互浣跨敤 klp_shadow_get_or_alloc() 璋冪敤鏉ュ皢褰卞瓙鍙橀噺闄勫姞鍒板凡缁忓湪閫旂殑鐖跺璞′笂銆?
瀵逛簬鎻愪氦 1d147bfa6429锛屽垎閰嶅奖瀛愯嚜鏃嬮攣鐨勪竴涓悎閫備綅缃槸
```

  int ps_lock_shadow_ctor(void *obj, void *shadow_data, void *ctor_data)
  {
	spinlock_t *lock = shadow_data;

	spin_lock_init(lock);
	return 0;
  }

  #define PS_LOCK 1
  void ieee80211_sta_ps_deliver_wakeup(struct sta_info *sta)
  {
	spinlock_t *ps_lock;

	/* sync with ieee80211_tx_h_unicast_ps_buf */
	ps_lock = klp_shadow_get_or_alloc(sta, PS_LOCK,
			sizeof(*ps_lock), GFP_ATOMIC,
			ps_lock_shadow_ctor, NULL);

	if (ps_lock)
		spin_lock(ps_lock);
	...

```
杩欑鐢ㄦ硶浼氬湪闇€瑕佹椂鍒涘缓涓€涓奖瀛愬彉閲忥紝鍚﹀垯浼氫娇鐢ㄥ凡缁忎负璇?<obj, id> 瀵瑰垱寤虹殑閭ｄ釜銆?
涓庡墠闈㈢殑浣跨敤鍦烘櫙绫讳技锛屽奖瀛愯嚜鏃嬮攣闇€瑕佽娓呯悊銆傚奖瀛愬彉閲忓彲浠ュ湪鍏剁埗瀵硅薄琚噴鏀句箣鍓嶉噴鏀撅紝鐢氳嚦鍙互鍦ㄥ奖瀛愬彉閲忔湰韬笉鍐嶉渶瑕佹椂閲婃斁銆?

### 鍏朵粬浣跨敤鍦烘櫙


褰卞瓙鍙橀噺涔熷彲浠ョ敤浣滀竴涓爣蹇楋紝琛ㄦ槑鏌愪釜鏁版嵁缁撴瀯鏄敱鏂扮殑銆佺粡杩?livepatch 鐨勪唬鐮佸垎閰嶇殑銆傚湪杩欑鎯呭喌涓嬶紝褰卞瓙鍙橀噺鎸佹湁浣曠鏁版嵁鍊煎苟涓嶉噸瑕侊紝瀹冪殑瀛樺湪鏈韩灏辨殫绀轰簡濡備綍澶勭悊鐖跺璞°€?

## 3. 鍙傝€冭祫鏂?

- https://github.com/dynup/kpatch

  璇?livepatch 瀹炵幇鍩轰簬 kpatch 鐗堟湰鐨勫奖瀛愬彉閲忋€?
- http://files.mkgnu.net/files/dynamos/doc/papers/dynamos_eurosys_07.pdf

  銆奃ynamic and Adaptive Updates of Non-Quiescent Subsystems in Commodity Operating System Kernels銆嬶紙Kritis Makris銆並yung Dong Ryu锛?007锛夋彁鍑轰簡涓€绉嶇О涓衡€滃奖瀛愭暟鎹粨鏋勶紙shadow data structures锛夆€濈殑鏁版嵁绫诲瀷鏇存柊鎶€鏈€?